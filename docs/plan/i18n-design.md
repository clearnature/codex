# i18n 架构设计：参考分析与嵌入点

> **状态**：设计稿，**尚未实现任何代码**。基线 `feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。
> 事实依据见 [../maps/references.md](../maps/references.md) 与 [../maps/architecture.md](../maps/architecture.md)。

---

## 一、参考分析

### 1.1 上游与社区：**没有源码级先例**

| 来源 | 结论 |
| --- | --- |
| 上游 issue | [#30025](https://github.com/openai/codex/issues/30025) / [#30060](https://github.com/openai/codex/issues/30060) / [#30421](https://github.com/openai/codex/issues/30421) 三个 i18n 请求，均 open，2026-06 起无推进 |
| 社区项目 | `xqnode/codex-zh-CN` 等全是**闭源桌面端的汉化补丁**，无一是 `codex-rs` 源码级实现 |
| 上游代码 | `codex-rs` 中 `i18n` / `l10n` 命中数为 **0**；无 locale 模块 |

**结论：架构必须从零设计，无可抄实现。**

### 1.2 内部参考项目（自有代码，模式已验证）

| 项目 | 语言 | TUI / CLI 的组织 | 桌面端 | UI i18n |
| --- | --- | --- | --- | --- |
| `/home/yanli/work/trha` | Haskell | 三个前端（`packages/app/{CLI,Desktop,Web}`）**共享 `trha-core`**；CLI 界面在 `packages/app/CLI/UI/` | `src/Presentation/Desktop/`（含 `IPC.hs`、`Preload.hs`）+ `packages/app/Desktop/`（electron renderer） | **有，两套** |
| `/home/yanli/work/DeepSeek-Reasonix-studio-v2.13.0` | Go | **TUI 与 CLI 同包**：`internal/cli/`（bubbletea v2 —— `model.go`、`statusline`、各类 picker 都在此） | `desktop/electron/`（`hostclient.js` 连 host），host 由 `cmd/reasonix-studio-host` 提供 | 未见 UI 层实现 |
| `/home/yanli/work/DeepSeek-Reasonix` | Go | 同上（`internal/cli/`） | `desktop/electron/`；**tauri 是已废弃路径**（`internal/appidentity/identity_windows_test.go:128` 里的 `"legacy tauri desktop"` 为证） | 未见 UI 层实现 |

**两种组织方式对比：**

- **trha / studio 模式**：TUI 与 CLI 共享同一层（trha 共享 core，studio 干脆同包）。
  → 文案本地化**只需做一处**。
- **codex 模式**：`tui`、`cli`、`exec` 是**三个分离的 crate**（`cli` 依赖 `tui`；`tui` 不依赖 `core`）。
  → 需要额外造一层共享，这正是第三节 `codex-i18n` crate 的动机。

### 1.3 已落地的两套 i18n 实现（在 `trha`）

| | CLI 侧 —— `packages/app/CLI/UI/I18n.hs`（239 行） | Desktop 侧 —— `packages/app/Desktop/src/renderer/ui/i18n.ts`（152 行） |
| --- | --- | --- |
| key 形式 | **英文原文本身即 key** | 命名空间 key（`app.newSession`、`settings.language`、`providers.*`） |
| 基准表 | En（原文即 key） | zh（类型源） |
| 查表 | `tr :: Lang -> Text -> Text` | `t(lang, key, vars)` |
| 语言类型 | `data Lang = Zh \| En` | `Lang`（来自 `state/prefs.ts`） |
| 缺失处理 | 回退返回 key 本身（= 英文） | **编译期报错**（key 类型从 zh 推导）+ 运行期回退 zh |
| 插值 | 无 | `{var}` |
| 模块性质 | **纯模块、无 IO**（注释明示便于单测） | 纯 store |
| locale 解析 | `parseLang`：`zh`/`zh-CN`/`zh-hans`/`中文`/`cn` → Zh；未知 → **En** | 未知 locale → 回退 zh |
| 测试 | `test/UI/I18nSpec.hs`：**遍历命令注册表**断言每条描述都有中文；断言命令名不被翻译 | `test/i18n.test.ts`：断言各 locale 的 key **无缺口**、插值、回退 |

CLI 侧模块注释原文（关键设计意图）：

> every user-facing string stays in English as its canonical key; rendering looks the key up through 't'. ... **This keeps `En` behaviour byte-identical to the pre-i18n shell** and makes missing translations degrade gracefully.

### 1.4 五个可迁移的模式

1. **英文原文即 key**（trha CLI）—— 见第三节 3.3，这是解决 codex 快照约束的关键。
2. **翻译层是纯函数、无 IO**（trha 两套）—— 便于单测与并行。
3. **覆盖率用测试兜底**（trha 两套）—— 遍历注册表/断言 key 无缺口，而不是靠人工检查。
4. **locale 容错解析 + 未知回退**（trha `parseLang`）—— `parseLang` 接受 6 种写法的中文标识，未知值绝不抛错。
5. **表现层各自实现、业务层共享**（trha 三前端 / studio 同包）—— i18n 属于表现层职责。

---

## 二、codex 的现状与硬约束

| 项 | 事实 | 来源 |
| --- | --- | --- |
| 文案分布 | `tui` 12,681 / `core` 8,112 / `cli` 1,929 / 协议层 2,891 | 启发式扫描（上界，含测试） |
| 三份前端 | `tui`（交互界面）、`cli`（入口 + doctor）、`exec`（非交互） | `Cargo.toml` 实测 |
| **`tui` 不依赖 `core`** | 走 `app-server-client` / `app-server-protocol` | 实测 `tui/Cargo.toml` |
| **877 个界面快照** | `chatwidget` 275、`bottom_pane` 225、`tui/src` 156……另有 146 个测试文件 | 实测 |
| 无任何 i18n 基础设施 | `.rs` 中 `i18n`/`l10n` 命中 0；全仓 `localeOverride` 命中 0 | 实测 |
| 可复用依赖 | `sys-locale 0.3.2`、`icu_locale_core 2.1` 已在 `Cargo.toml` | 实测 |
| 协议层不可动 | `app-server-protocol` 的字段名/枚举值被外部客户端依赖 | `AGENTS.md` 将其列为 external integration surface |

**最大的两个约束：**
- **约束 A**：877 个快照捕获了渲染出的英文文本 → 默认语言下输出**必须逐字节不变**。
- **约束 B**：`tui` 与 `core` 之间隔着协议边界 → 文案无法靠「改一处」全局生效。

---

## 三、嵌入点设计

### 3.1 新增共享 crate：`codex-rs/i18n`

```
codex-rs/
└── i18n/                    ← 新增（纯 crate，无 IO）
    ├── Cargo.toml           ← name = "codex-i18n"
    └── src/
        ├── lib.rs           ← pub fn tr(lang, key) -> &'static str
        ├── lang.rs          ← enum Lang { Zh, En } + parse_lang()
        └── dict_zh.rs       ← 英文原文 → 中文映射表
```

依赖方向（**只加边，不动现有结构**）：

```
        ┌─────────────┐
        │ codex-i18n  │  ← 纯函数、无 IO、无 workspace 内部依赖
        └─────────────┘
           ▲    ▲    ▲
           │    │    └──────── codex-exec
           │    └───────────── codex-cli
           └────────────────── codex-tui
```

`codex-i18n` 不依赖任何其他 workspace crate，因此不会与「`tui` 不依赖 `core`」这条既有边界冲突。

### 3.2 为什么不做在 `core` 或 `protocol`

| 候选位置 | 否决理由 |
| --- | --- |
| `core` | `tui` **不依赖** `core`（约束 B），放这里 `tui` 取不到 |
| `protocol` / `app-server-protocol` | 那是跨进程契约层，字段名不可动；且会推向外部客户端 |
| 各自复制一份 | 违背 1.4 的模式 5，且 codex 三前端**同语言**，没有 trha 那样的多语言约束 |

### 3.3 「英文原文即 key」如何化解约束 A（877 快照）

采用 trha CLI 的策略：

```rust
pub fn tr(lang: Lang, key: &'static str) -> &'static str {
    match lang {
        Lang::En => key,                                  // 原样返回
        Lang::Zh => DICT_ZH.get(key).copied().unwrap_or(key), // 缺失回退英文
    }
}
```

改造方式是**把英文原文包成 key**，而不是替换成新标识符：

```rust
// 改造前
writeln!(f, "Show this help")?;
// 改造后（En 下输出逐字节相同）
writeln!(f, "{}", tr(lang, "Show this help"))?;
```

**结果**：默认 `En` 下渲染输出与改造前完全一致 → 877 个快照**零改动**、可随时回归验证。
这是选择该策略的唯一理由，也是最需要先验证的一点。

### 3.4 分层接入顺序

| 步 | 动作 | 验证方式 |
| --- | --- | --- |
| 1 | 建 `codex-i18n` crate（`Lang`、`parse_lang`、`tr`、空字典） | `cargo test -p codex-i18n`；全仓 `just test` 快照全绿（此时未接入任何调用点） |
| 2 | 选**最小垂直切片**接入：`bottom_pane/footer.rs` 几行 | 快照仍全绿（证明 En 逐字节不变）；`--lang zh` 能看到中文 |
| 3 | 铺开 `tui`：`bottom_pane` → `chatwidget` → `app` | 分批，每批后跑 `just test -p codex-tui` |
| 4 | 接入 `cli`：帮助文本、`doctor` 输出 | `codex doctor` 在 En 下输出不变 |
| 5 | 接入 `exec` 的非交互输出 | — |
| 6 | `core` 的用户可见错误（**需逐条甄别**，跳过日志/遥测/喂模型的工具描述） | 单独一轮，逐文件评估 |

### 3.5 locale 的来源（待定，见第四节）

可复用的既有能力：`sys-locale`（探测系统语言）+ trha 的 `parse_lang` 容错策略
（接受 `zh` / `zh-CN` / `zh-hans` / `中文` / `cn`，未知一律回退 `En`）。

**不要动**的是 `codex-rs/core/src/unified_exec/process_manager.rs:93` ——
那里把子进程的 `LANG` / `LC_ALL` 固定为 `C.UTF-8` 是为了让 shell 输出稳定可解析，
与界面本地化无关，改了会破坏工具输出解析。

---

## 四、待决策（进入实现前需要拍板）

| # | 决策点 | 候选 |
| --- | --- | --- |
| 1 | **locale 入口** | ① `config.toml` 新增配置项 ② `--lang` 命令行参数 ③ 只读环境变量 `LANG` ④ 组合（配置项 > 参数 > 环境） |
| 2 | **翻译产物形式** | ① Rust 内嵌映射表（`dict_zh.rs`，与 trha CLI 一致） ② 外部文件（`.ftl` / `.po` / JSON），需处理嵌入二进制与打包 |
| 3 | **范围是否含提示词** | `core/*.md` 与 `core/templates/*` 是**喂给模型**的，翻译会改变模型行为 —— 建议**排除在 UI i18n 之外**，单独立项 |
| 4 | **是否对齐桌面端命名** | issue 里出现的 `[desktop] localeOverride` 在 `codex-rs` 中**不存在**（0 命中）；是否沿用这个名字取决于是否在意两侧配置长得一样 |
| 5 | **中文以外的语言** | 先只做 zh 还是同时留出多语言表结构（trha Desktop 的「key 类型从 zh 推导」模式可平移为 Rust 的 `match` 穷尽性） |

---

## 五、本设计的边界与未验证项

1. **桌面端不在范围内。** `codex` 桌面端是闭源二进制（仓库内无 desktop/electron/tauri，Release 产物均为 CLI 侧）。trha 的 `renderer/ui/i18n.ts` 模式**无法照搬**——我们没有那份源码。给桌面端上中文只能走补丁路线（社区已有，脆弱、绑版本）。
2. **本节所有 codex 侧数字均为启发式扫描的上界**（含测试代码），实际接入时以逐文件甄别为准。
3. **第 3.3 节的「快照零改动」是设计推断，尚未实测。** 实施第 1 步时必须先验证这一点——若 En 下输出有任何字节差异，整个策略需要重新评估。
4. **未评估构建成本。** 新增 crate 需同步更新 `BUILD.bazel`（`AGENTS.md` 明确要求：涉及编译期文件读取时要更新对应 `BUILD.bazel`），本轮未验证 Bazel 侧改动量。

---

## 附：参考文件索引（本机）

| 用途 | 路径 |
| --- | --- |
| trha CLI 的 i18n 实现 | `/home/yanli/work/trha/packages/app/CLI/UI/I18n.hs` |
| trha CLI 的 i18n 测试 | `/home/yanli/work/trha/packages/app/CLI/test/UI/I18nSpec.hs` |
| trha Desktop 的 i18n | `/home/yanli/work/trha/packages/app/Desktop/src/renderer/ui/i18n.ts` |
| trha Desktop 的 i18n 测试 | `/home/yanli/work/trha/packages/app/Desktop/test/i18n.test.ts` |
| trha 架构图 | `/home/yanli/work/trha/maps/system-overview.mmd` |
| studio 的 TUI/CLI 同包 | `/home/yanli/work/DeepSeek-Reasonix-studio-v2.13.0/internal/cli/` |
| studio 的 electron 前端 | `/home/yanli/work/DeepSeek-Reasonix-studio-v2.13.0/desktop/electron/` |
| Reasonix 的 legacy tauri 痕迹 | `/home/yanli/work/DeepSeek-Reasonix/internal/appidentity/identity_windows_test.go:128` |
