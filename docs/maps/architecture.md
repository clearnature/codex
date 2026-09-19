# 架构分析

> 依据：各 crate 的 `Cargo.toml` **实测依赖**（非官方声明）+ GitNexus 知识图谱（108,320 节点 / 312,008 边）。

## 分层

```
┌─────────────────────────────────────────────────────────────┐
│ 入口聚合层                                                   │
│   cli  —— 主二进制 codex，依赖 48 个 workspace crate         │
│    ├── tui          交互式终端界面                           │
│    ├── exec         非交互模式（codex exec）                 │
│    └── app-server   协议服务端（供 IDE / 外部客户端）         │
└─────────────────────────────────────────────────────────────┘
        │                                   │
        │  tui 走 app-server-client          │  exec / app-server
        │  + app-server-protocol             │  直接依赖 core
        │  （**不直接依赖 core**）            │
        ▼                                   ▼
┌─────────────────────────────────────────────────────────────┐
│ 核心层   core —— 依赖 73 个 workspace crate                  │
│   protocol · config · tools · state · rollout · history      │
│   execpolicy · sandboxing · shell-command · skills · mcp      │
│   model-provider · prompts · thread-store · …                │
└─────────────────────────────────────────────────────────────┘
        │
        ▼
┌─────────────────────────────────────────────────────────────┐
│ 能力 / 扩展层                                                │
│   ext/*（14 个扩展点）· utils/*（26 个基础设施 crate）        │
│   沙箱实现（linux / mxc / windows / bwrap）· 模型接入          │
│   code-mode（V8）· 可观测性 · 云与账号 · 语音                  │
└─────────────────────────────────────────────────────────────┘
```

## 实测依赖关系

| crate        | 依赖的 workspace crate 数 | 直接依赖 `core` | 角色                                                |
| ------------ | ------------------------: | :-------------: | --------------------------------------------------- |
| `cli`        |                        48 |       ✅        | 聚合入口，同时依赖 `tui`、`exec`、`app-server`      |
| `tui`        |                        49 |     **❌**      | 经 `app-server-client` / `app-server-protocol` 通信 |
| `exec`       |                        22 |       ✅        | 非交互模式，直接构建 core                           |
| `app-server` |                        60 |       ✅        | 外部客户端协议服务端                                |
| `core`       |                        73 |        —        | 依赖面最广，是事实上的中枢                          |

### 最重要的一条：TUI 与 core 是客户端-服务端关系

`tui` 的 `Cargo.toml` 里**没有** `codex-core`，它依赖的是
`codex-app-server-client`、`codex-app-server-protocol`、`codex-protocol` 等。
真正直接依赖 `core` 的是 `exec` 和 `app-server`。

这条事实有两个直接后果：

1. **界面文案与核心消息隔着协议边界**。本地化时，TUI 侧的字符串和 core 侧产生、
   经协议传上来的字符串是两套东西，不能指望改一处而全局生效。
2. **`app-server-protocol` 是双方共享的契约层**。它的 JSON 字段名、枚举值（如
   `"error"`、`"modified"`）被外部客户端依赖，**不可翻译**；只有 payload 里的
   文本内容可以本地化。该 crate 还生成 TypeScript 绑定（626 个 `.ts`）。

## 关键执行流

GitNexus 从调用图中提取了 **300 条执行流**（`Process` 节点，通过 `STEP_IN_PROCESS`
边串联，共 1,554 步）。按步数排序的代表性流程：

| 流程                                                                              | 类型            | 步数 |
| --------------------------------------------------------------------------------- | --------------- | ---: |
| `Start_inner → JSONRPCErrorError`                                                 | cross_community |   10 |
| `Run_main → As_path`                                                              | cross_community |    9 |
| `Run_main → All_layers_low_to_high`                                               | cross_community |    9 |
| `Handle_key_event → New`                                                          | cross_community |    9 |
| `Run_turn → Auth_cached`                                                          | cross_community |    9 |
| `Handle_history_search_key → Text`                                                | cross_community |    8 |
| `Spawn_windows_sandbox_session_elevated_for_permission_profile → Native`          | cross_community |    8 |
| `Multi_agent_v2_interrupt_agent_accepts_task_name_target → Verify_layer_ordering` | cross_community |    8 |
| `Apply_bespoke_event_handling → PendingTurnStartState`                            | cross_community |    7 |

从命名可读出的骨架：

- `Run_main` → CLI 主入口
- `Run_turn` → 一次对话轮次的驱动
- `Handle_key_event` / `Handle_history_search_key` → TUI 输入处理
- `Start_inner` → `app-server` 启动路径
- `Spawn_*_sandbox_session_*` → 沙箱会话创建（含 Windows 提权路径）

> 这些流程全部标记为 `cross_community`，说明主要调用链都**跨越了代码聚类边界**——
> 即仓库的分层是调用链意义上的，而非目录意义上的。

## 调用图规模

| 关系                                           |             数量 | 含义       |
| ---------------------------------------------- | ---------------: | ---------- |
| `CALLS`                                        |          109,590 | 函数调用   |
| `ACCESSES`                                     |           70,245 | 符号访问   |
| `DEFINES`                                      |           54,055 | 定义关系   |
| `MEMBER_OF`                                    |           32,510 | 成员归属   |
| `CONTAINS`                                     |            7,424 | 结构包含   |
| `IMPORTS`                                      |            6,898 | 模块导入   |
| `IMPLEMENTS` / `METHOD_IMPLEMENTS` / `EXTENDS` | 876 / 2,075 / 12 | 继承与实现 |

节点以函数为主（`Function` 50,611，占 46.7%），其次属性与结构体——
符合 Rust 大型工程的特征。

## 数据流视角

```
用户输入
  → tui（按键/粘贴/斜杠命令）
  → app-server-client ──协议──▶ app-server / core
  → core：组装上下文（prompts + config + tools）
  → model-provider ──HTTP──▶ 模型 API
  → core：解析工具调用
  → execpolicy 判定 → sandboxing 执行（linux / mxc / windows / bwrap）
  → 结果经协议回到 tui 渲染
```

沙箱与策略是**独立于模型**的一层：`execpolicy` 负责判定，`sandboxing` 及其
平台实现负责执行判决，`shell-escalation` 处理升级请求。这层改动与本次 i18n 无关，
但它的报错信息是用户可见文案的一部分。

## 与 i18n 相关的架构结论

1. 入口层（`tui` + `cli`）承载绝大部分用户可见文案；
2. `core` 产生跨协议的用户可见消息与错误，需甄别（同层还有日志、遥测、工具描述）；
3. `app-server-protocol` 是契约层，字段不可动；
4. `core/*.md` 与 `core/templates/*` 是**喂给模型**的提示词，翻译它们属于独立决策；
5. 沙箱/策略层的报错也是文案来源，容易被遗漏。
