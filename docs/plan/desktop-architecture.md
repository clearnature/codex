# 桌面端架构设计

> 状态：**设计稿**，尚未实现。
> 基线：`feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。
> 关联：[`i18n-design.md`](./i18n-design.md)（i18n 嵌入点）、[`i18n-verification.md`](./i18n-verification.md)（验证计划）。

## 〇、先给结论

**桌面端最大的优势是「零侵入上游」**：它不修改 `codex-rs` 的任何源码，只通过
`app-server` 协议消费核心能力。因此上游的每次更新对桌面端**几乎无感**——这与 i18n 那条线
（必须改源码）是**完全不同的耦合度**。

核心结论三条：

1. **对接方式已确认存在**：`codex-app-server` 支持 `stdio://`、`unix://`、`ws://IP:PORT` 三种传输
   （见 §2.1），并已生成 **711 个 TypeScript 协议类型**（见 §2.3）可直接复用。
2. **建议照搬 `trha` 的宿主模式**：electron 主进程作为 host client，负责 spawn、握手、租约、优雅退出
   （见 §4.3）；`trha` 已把这些踩坑沉淀为可复用代码。
3. **i18n 与桌面端共享同一批核心文案**，两件事在 `core` 层汇合（见 §5）。

## 一、桌面端的定位

**做「codex 核心的一个 GUI 客户端」**，而不是重写一个 agent。

| 是 | 不是 |
| --- | --- |
| `app-server` 协议的前端 | 独立实现的 agent |
| 会话/审批/工具调用的图形化呈现 | 新的推理内核 |
| 自带 UI 的 i18n | 改 `codex-rs` 源码 |

理由：上游的核心能力（工具、沙箱、审批、MCP、扩展）已由 `core` + `app-server` 提供，
重写一遍既不可能也没必要。

## 二、codex 已提供的对接能力（实测）

### 2.1 传输：三种 + daemon 模式

`codex-rs/app-server-transport/src/transport/mod.rs:75`：

```rust
pub enum AppServerTransport {
    Stdio,
    UnixSocket { socket_path: AbsolutePathBuf },
    WebSocket { bind_address: SocketAddr },
    Off,
}
```

`--listen` 接受的 URL（错误消息里列出的完整集合）：
`stdio://`、`unix://`、`unix://PATH`、`ws://IP:PORT`、`off`

另有 daemon 能力：`app-server-daemon` crate + 控制 socket + 启动锁
（`app_server_control_socket_path()` / `app_server_startup_lock_path()`，位于 `codex_home` 下）。

### 2.2 鉴权

`app-server/src/main.rs` 有 `AppServerWebsocketAuthArgs`（`:58`），WebSocket 传输支持鉴权参数。

### 2.3 协议类型：711 个 TypeScript 文件

`codex-rs/app-server-protocol/schema/typescript/` 下有 **711 个 `.ts`**，是从 Rust 生成的协议绑定：

| 文件 | 含义 |
| --- | --- |
| `ClientRequest.ts` / `ClientNotification.ts` | 客户端 → 服务端 |
| `ApplyPatchApprovalParams.ts` / `...Response.ts` | 审批流 |
| `AuthMode.ts`、`AgentMessageInputContent.ts` | 基础类型 |
| `v2/Account*.ts`、`v2/ActivePermissionProfile.ts` | v2 协议子集 |

配套还有 `schema/json/` 下的 JSON Schema。

**这意味着桌面端可以直接获得类型安全的 JSON-RPC 客户端**，不必手工对照协议。

### 2.4 现有 SDK 的定位（重要区别）

`sdk/typescript` 是 `@openai/codex-sdk`，其 README 明确：

> The TypeScript SDK wraps the `codex` CLI from `@openai/codex`. **It spawns the CLI and
> exchanges JSONL events over stdin/stdout.**

它的 `exec.ts` 用 `spawn` 调用 **`codex exec`**（非交互模式），API 为
`new Codex()` → `startThread()` → `run()` / `runStreamed()`。

**结论：它是「把 agent 嵌入工作流」的封装，不是完整的 app-server 客户端。**
桌面端若要做会话管理、审批交互、多窗口，应走 app-server 协议；SDK 可作为快速原型或降级方案。

## 三、三条候选路径对比

| | A. stdio 子进程 | B. WebSocket | C. Unix socket |
| --- | --- | --- | --- |
| 启动方式 | electron 主进程 `spawn` app-server | spawn 或连常驻 daemon | spawn 或连常驻 daemon |
| 端口/鉴权 | 不需要 | 需要端口 + `AppServerWebsocketAuthArgs` | 文件系统权限 |
| 跨平台 | ✅ 全平台 | ✅ 全平台 | ❌ **Windows 不可用** |
| 多客户端共享 | ❌ 独占 | ✅ | ✅（Unix 内） |
| 进程生命周期 | 由 electron 管理（简单、明确） | 松耦合，但需自己管崩溃重启 | 同 B |
| 与 trha 的相似度 | 高（trha 即 spawn + 握手） | 中 | 低 |

**建议：以 A（stdio）起步，架构上为 B 留出切换余地。**

理由：A 的进程模型最简单、最贴近 `trha` 已验证的模式；把传输选择收敛成一个配置项后，
将来切 WebSocket 不需要改动上层（协议与类型绑定完全一致）。

## 四、推荐架构

### 4.1 分层

```
┌──────────────────────────────────────────────────────────┐
│ 渲染进程 (renderer)                                       │
│   UI 组件 + 自有 i18n（见 §5）                            │
│   不直接接触协议：只经 preload 暴露的窄接口                │
└──────────────────────────────────────────────────────────┘
              ▲ IPC (contextBridge, 窄接口)
              ▼
┌──────────────────────────────────────────────────────────┐
│ 主进程 (main)                                             │
│   ├── HostClient：spawn app-server + 握手 + 租约          │
│   ├── 协议客户端：JSON-RPC over stdio                     │
│   │     └── 类型来自 app-server-protocol 的 711 个 .ts     │
│   └── 状态转发：把协议事件推给渲染进程                     │
└──────────────────────────────────────────────────────────┘
              ▲ stdio（JSON-RPC）
              ▼
┌──────────────────────────────────────────────────────────┐
│ codex-app-server（子进程，未修改的上游二进制）             │
│   → codex-core → 工具 / 沙箱 / 审批 / MCP / 扩展          │
└──────────────────────────────────────────────────────────┘
```

**关键约束（安全基线）**：渲染进程**不直连**协议，只通过 preload 暴露的窄接口。
这与 `trha` 的 `desktop-port-contract.md`（"组件只经 `port/` 访问后端，禁止直接 `fetch`"）
是同一原则，也与 codex 自己的 `webPreferences` 最保守配置一致
（`contextIsolation: true` + `nodeIntegration: false` + `sandbox: true`）。

### 4.2 技术栈

| 项 | 选择 | 依据 |
| --- | --- | --- |
| 外壳 | **electron** | `trha`、`studio` 均用 electron；`Reasonix` 的 tauri 已废弃（`internal/appidentity/identity_windows_test.go:128` 的 `"legacy tauri desktop"`） |
| electron 版本 | **44.x**，精确锁定 | 与 `Reasonix`（44.2.0）对齐，见 `trha/docs/plans/electron-44-upgrade.md` |
| 打包 | `electron-builder` 26.x | `trha` 已升级到位 |
| 协议客户端 | `app-server-protocol` 生成的 TS 类型 | §2.3 |

### 4.3 直接复用 trha 的宿主模式

`trha` 的 `packages/app/Desktop/src/main/index.ts`（215 行）已经解决了同类问题，
桌面端应照搬它的结构而非重新发明：

| trha 的做法 | 用途 |
| --- | --- |
| `startScheduler()` + `handshake.ts` 的 `parseHandshake` / `isSupported` | **握手校验**：版本/名称不符则拒绝启动 |
| stdout **只解析首行**，其余交给 stderr | 避免日志污染协议通道 |
| stdin 作为 **租约**：退出时 `end()`，让子进程自行 drain 退出 | 跨平台优雅退出（不依赖信号） |
| `schedulerRuntime()` | 打包产物里用自带 node runtime，开发时用系统 node |
| 5s 握手超时 + 明确失败 | 避免"卡住但看不出来" |

这套模式与 codex 的 `app-server` 正好契合（app-server 也是长驻子进程 + 结构化通道）。

## 五、与 i18n 的协同

两条线在 `core` 层汇合：

```
core 产生的文案（错误 / 审批提示 / 工具输出）
   ├──→ tui 渲染          ← i18n 改造在这条线生效
   └──→ app-server 协议   ← 桌面端收到的也是同一批文本
```

**含义**：
1. 桌面端 UI（我们自己的代码）→ **自带 i18n，完全可控**；可直接移植 `trha`
   `Desktop/src/renderer/ui/i18n.ts` 的模式（扁平 key 表 + `t()` 插值 + 缺 key 编译报错）。
2. 核心产生的文案 → **依赖 i18n 那条线的进展**。若只做 `tui` 不做 `core`，
   桌面端会出现「中文界面 + 英文错误消息」的割裂体验。
3. 反过来说，桌面端是 i18n 的**额外验证场**——中文是否覆盖到位，GUI 上一眼可见。

## 六、与上游的隔离（本设计的核心价值）

| 项 | 状态 |
| --- | --- |
| 修改 `codex-rs` 源码 | **零处** |
| 依赖的上游接口 | 仅 `app-server` 协议（`AGENTS.md` 将其列为 external integration surface，上游会保持兼容） |
| 上游更新时的影响 | 需要的是「重新构建/获取新版 `codex-app-server` 二进制」，而非 rebase 冲突 |
| 唯一的同步点 | 协议版本变化（`v2/` 命名空间的存在说明上游对协议做了版本化） |

**这与 i18n 那条线形成鲜明对比**：i18n 必须侵入源码（TUI 的中文宽度要求必须在构造文本时翻译），
需要 codemod + 漂移检测来维持；桌面端则是干净的加法。

## 七、里程碑建议

| 阶段 | 目标 | 验证方式 |
| --- | --- | --- |
| **D0** | 用 `stdio://` spawn `codex-app-server`，完成一次 ping/echo 往返 | 能打印出一次真实响应 |
| **D1** | 用生成的 TS 类型调通 `ClientRequest` 的最小方法（如 thread 创建） | 类型检查通过 + 实机拿到 thread id |
| **D2** | electron 壳 + preload 窄接口 + 一次流式事件的端到端渲染 | 界面上能看到流式输出 |
| **D3** | 会话管理 / 审批交互 / 工具调用呈现 | 手动走通一次真实任务 |
| **D4** | UI i18n 接入（移植 trha 模式） | 中英切换 |
| **D5** | 打包分发（electron-builder） | 产出可安装包 |

**D0 是最便宜的可行性验证**，建议优先——它只验证「能不能连上、协议是否可用」，
不涉及 UI、不涉及 electron。

## 八、待决策（进入实现前需要拍板）

| # | 决策点 | 候选 |
| --- | --- | --- |
| 1 | **传输方式** | `stdio://`（简单）／ `ws://`（灵活、跨平台多客户端）／ daemon 常驻 |
| 2 | **进程归属** | 随 electron 启动子进程（简单）／ 独立常驻 daemon（多客户端共享） |
| 3 | **前端框架** | React + Vite（`trha` 路线）／ Next（`studio` 的 `frontend-next` 路线） |
| 4 | **SDK 用法** | 直接用 app-server 协议（能力全）／ 先用 `@openai/codex-sdk` 快速验证 |
| 5 | **桌面端目录位置** | 本仓库内（如 `desktop/`）／ 独立仓库（与上游隔离更彻底） |

第 5 点尤其重要：**放本仓库**便于与 i18n 协同、共享 CI；**放独立仓库**则 fork 更干净，
但需要额外维护协议同步。建议先在本仓库内做（`desktop/` 或 `packages/desktop/`），
待稳定后再考虑拆分。

## 九、未验证 / 风险

1. **app-server 的实际启动与握手流程未实测**（本设计基于源码阅读，未运行过 `codex-app-server`）。
   D0 就是为验证这一点而设。
2. **WebSocket 模式下实际端口的获取方式未确认**（`ws://127.0.0.1:0` 为系统随机分配，客户端如何得知需实测）。
3. **协议方法的完整清单未枚举**（711 个类型文件是**类型**，不等于方法清单；需要从
   `ClientRequest` 联合类型里导出实际可用的 RPC 方法）。
4. **`app-server` 二进制如何随桌面端分发未定**（内嵌进安装包 vs 依赖用户已装的 `codex`）。
5. **多窗口 / 多会话的并发模型未设计**（取决于 §8 的传输与进程归属决策）。
6. **审批交互（`ApplyPatchApproval*`）的 UI 设计未展开**——这是 GUI 相对 TUI 的最大增值点，
   也是工作量最大的一块。
