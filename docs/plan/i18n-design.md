# i18n 架构设计：参考分析与嵌入点

> **状态**：§3.1 的 crate、§3.3 的「英文原文即 key」策略、§3.4 的步 1–2（建 crate + footer 最小切片）与 §3.5/§4 的 locale 入口**已实现并验证**；§3.4 步 3–6 的铺开进行中。基线 `feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。
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
└── i18n/                    ← 已建（`codex-i18n`）
    ├── Cargo.toml           ← name = "codex-i18n"
    └── src/
        ├── lib.rs           ← pub fn tr(lang, key) -> &'static str
        ├── lang.rs          ← enum Lang { Zh, En } + parse_lang()（剥 POSIX 的 .codeset / @modifier）
        ├── dict_zh.rs       ← 英文原文 → 中文映射表（ENTRIES，字面量对，承重形状）
        ├── resolution.rs    ← locale 五级链的纯函数 + Env + sys-locale 探测（本 crate 唯一触外部之处）
        ├── current.rs       ← 进程内当前语言 current()/set_current()（对齐 qwen 的 setLanguage）
        └── interpolate.rs   ← tr_with / substitute / placeholders（{0} 位置占位）
```

渲染路径（`tr` / `current`）保持纯查表；只有 `resolution` 读环境与操作系统，且其优先级规则本身是纯函数、可穷尽测试。

依赖方向（**只加边，不动现有结构**）：

```
        ┌─────────────┐
        │ codex-i18n  │  ← 纯函数、无 IO、无 workspace 内部依赖
        └─────────────┘
           ▲   ▲   ▲   ▲   ▲
           │   │   │   │   └──────── codex-mcp   ┐ §3.4 步 5–6 铺开后
           │   │   │   └──────────── codex-core  ┘ 新增的两条边
           │   │   └──────────────── codex-exec
           │   └──────────────────── codex-cli
           └──────────────────────── codex-tui
```

`codex-i18n` 不依赖任何其他 workspace crate（唯一外部依赖是探系统 locale 的 `sys-locale`），
因此不会与「`tui` 不依赖 `core`」这条既有边界冲突。

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

### 3.6 三类特例与处置约定（铺开中实测得出）

| 特例 | 现象 | 约定 | 先例 / 状态 |
| --- | --- | --- | --- |
| **`const` 表** | 英文原文存在 `const` 结构里，而 `const` 不能调用 `tr` | **把表改成函数**（`fn …() -> [T; N]`，`T: Copy` 时按值返回零成本；表大可加 `OnceLock` 缓存，仍返回 `&'static [..]`） | `plugin_catalog::remote_marketplace_sections`、`keymap_setup::keymap_actions`、`compaction::compaction_header` ✅ 已做 |
| **静态提升失效** | 把字面量放进 `&[…]` / `&{…}` 字面量、再赋给 `&'static` 字段时，包 `tr` 会报 **E0716 temporary value dropped while borrowed** | **把字段改成拥有式容器**（`&'static [T]` → `Vec<T>`），构造处 `&[` → `vec![`，遍历处 `in x.field` → `in &x.field` | `history_cell::SafetyAccessBlockCell.actions` ✅ 已做 |
| **`match` 模式位置** | 字面量出现在 `match` 的**模式**里（如 `"workspace with network access" => …`），包 `tr` 会报 **E0532 expected a pattern** | **不要包**：模式是拿来做比对的，不是拿来渲染的——即使能编译，zh 下也会破坏匹配。只包**表达式位置**的字面量 | `status/card.rs` 的权限/沙箱匹配 ✅ 已避开（该文件其余标签已接入） |
| **碎片拼句** | `format!("{advanced_label} {verb} usage limits faster")`，其中 `verb` 来自英文动词表 | **先重构再翻译**：直接包 `tr_with` 会得到半中半英的句子，比不译更糟。**若拼出的句子在中文里语序恰好相同**（如 `"… Space to {action}; Enter details."`），可以逐片翻译——但必须**连同变量取值点一起接入 `tr`**（`let action = if enabled { tr(current(), "disable") } else { tr(current(), "enable") }`），否则变量仍是英文 | `model_popups.rs`（待重构）、`plugin_catalog.rs` 的 `toggle_action` ✅ 已做 |
| **平台 `cfg` 块** | `#[cfg(target_os = "windows")]` 内的改动在 Linux 上**不参与类型检查** | 跨平台正确性只能靠多平台 CI；本机 `cargo check` 通过**不算验证** | `windows_sandbox_prompts.rs` ⚠️ 未验证 |

**为什么会有「静态提升失效」这一类**：`&[("a", "b")]` 能赋给 `&'static [(&'static str, &'static str)]`，靠的是 Rust 的 **rvalue static promotion**，而它的前提是内容为常量表达式。`tr(current(), "…")` 是函数调用 ⇒ 提升失效 ⇒ 数组成为短命临时值。修法只有「让容器拥有数据」这一条（`const` 化不可能，因为 `tr` 不是 `const fn`）；代价是一次 `Vec` 分配，发生在创建提示块/通知的时刻，不在热路径上。

**为什么 `const` 表不走「用法处翻译」**：那样 key 不会出现在任何调用点，`codex-i18n-check` 看不见它，工具就必须长出命名启发式才能不把"正在屏幕上的翻译"报成 unused。启发式也试过——把 bound-key 从 `label:` 扩到 `*_name` / `*_description`，**立刻产生 33 条假 `missing`**（尚未接线的字符串被当成缺口）。结论是**规则宁窄勿宽**：遇到表就改结构，而不是改量尺。

**哪些字符串算「用户可见」（甄别规则）**：进入 `add_error_message` / `add_info_message` / `add_warning_message` / `add_to_history(new_error_event(…))`、`SelectionViewParams` 的标题与条目、以及直接渲染进 `Line` / `Span` 的字面量 → **要译**；`tracing::*` 日志、`.wrap_err("…")` 错误链上下文、遥测属性名与值（如 `codex.thread.fork`）、喂模型的提示词资产、内部 id 与配置键 → **不译**。这条规则是用来替代「逐条拍脑袋」的：铺开时先按**调用点**分类，只有同时落进两边的（例如同一个 `format!` 结果既进日志又进 UI）才需要单独看。

**CI 落点**：`i18n-check` 已接入 `.github/workflows/repo-checks.yml`（与 `just fmt-check` 同一作业，该作业已有 Rust/`just` 环境），漂移即拦合并；`i18n-scan` 是只读统计、没有可判定的通过/失败条件，故不入 CI。

## 四、待决策（进入实现前需要拍板）

| # | 决策点 | 候选 |
| --- | --- | --- |
| 1 | **locale 入口** | ① `config.toml` 新增配置项 ② `--lang` 命令行参数 ③ 只读环境变量 `LANG` ④ 组合（配置项 > 参数 > 环境） |
| 2 | **翻译产物形式** | ① Rust 内嵌映射表（`dict_zh.rs`，与 trha CLI 一致） ② 外部文件（`.ftl` / `.po` / JSON），需处理嵌入二进制与打包 |
| 3 | **范围是否含提示词** | `core/*.md` 与 `core/templates/*` 是**喂给模型**的，翻译会改变模型行为 —— 建议**排除在 UI i18n 之外**，单独立项 |
| 4 | **是否对齐桌面端命名** | issue 里出现的 `[desktop] localeOverride` 在 `codex-rs` 中**不存在**（0 命中）；是否沿用这个名字取决于是否在意两侧配置长得一样 |
| 5 | **中文以外的语言** | 先只做 zh 还是同时留出多语言表结构（trha Desktop 的「key 类型从 zh 推导」模式可平移为 Rust 的 `match` 穷尽性） |

### 已定（2026-09-17）

| # | 结论 | 落地位置 |
| --- | --- | --- |
| 1 | **候选 ④，且是五级链**：`--lang` > `config.toml` 的 `locale` > `LC_ALL` > `LANG` > 系统 locale（`sys-locale`）。按「是否出现」而非「能否识别」决定优先级：`--lang=en` 压过中文配置，未知 locale 一律回退 En、不报错。 | `i18n/src/resolution.rs`（纯函数 `resolve` + `Env` 数据化）、`i18n/src/current.rs`（进程内 `current()`/`set_current`）；旗标在 `utils/cli/src/shared_options.rs` 的 `SharedCliOptions.lang`，tui/exec 启动时 publish |
| 2 | **候选 ①**：Rust 内嵌映射表，`dict_zh.rs` 的 `ENTRIES` 为 `(英文, 中文)` 字面量对——形状是承重的，`codex-i18n-check` 靠它做对账。 | `i18n/src/dict_zh.rs` |
| 3 | **维持排除**：`core/*.md`、`core/templates/*` 与喂模型的工具描述不进 UI i18n。 | — |
| 4 | **定名 `locale`**（**人类裁决**，2026-09-17；稳定坐标＝台账流水的已闭环决策条目 `legacy-j-1b977695ad`【已裁决并关闭】）。理由：Codex 桌面端**不开源**，本项目的桌面支持是**自研**且已列在架构规划里（接入方案已预留），没有向它对齐命名的理由；先把 i18n 工作按计划完成，桌面适配留到自研桌面落地时再谈。曾经同时接受桌面端的 `localeOverride`（`#[serde(alias)]`），**现已移除**：`localeOverride` 不再被读（`ConfigToml` 容忍未知键，所以契约是「不读该值」，不是「解析报错」，测试按此断言）。⚠ 记录一处不一致：本行在裁决正式落盘**之前**就写有「人类裁决」字样，与当时台账里该决策的 `open` 状态相矛盾；两种可能（文档写早了 / 台账挂久了）无法从记录中判定，现以本次明确裁决为准、两者对齐。 | `config/src/config_toml.rs`、`core/src/config/mod.rs` |
| 5 | 维持只做 zh；`Lang` 是穷尽枚举，加语言必须过 `match`，翻译缺失回退英文。 | `i18n/src/lang.rs` |
| — | **插值**（§6 的「`{{var}}` 插值」一行）：实现为**位置占位** `{0}`/`{1}` + `tr_with(lang, key, args)`，因为 Rust 的 `format!` 要求格式串在编译期已知，而这里必须用查表之后的字符串。英文路径就是往英文原文里代入（`format!` 的旧行为不变），未知下标原样保留而不 panic。 | `i18n/src/interpolate.rs`（`tr_with` / `substitute` / `placeholders`），占位符对齐由 `interpolate_tests` 遍历字典守住 |

---

## 五、本设计的边界与未验证项

1. **桌面端不在范围内。** `codex` 桌面端是闭源二进制（仓库内无 desktop/electron/tauri，Release 产物均为 CLI 侧）。trha 的 `renderer/ui/i18n.ts` 模式**无法照搬**——我们没有那份源码。给桌面端上中文只能走补丁路线（社区已有，脆弱、绑版本）。
2. **本节所有 codex 侧数字均为启发式扫描的上界**（含测试代码），实际接入时以逐文件甄别为准。
3. **第 3.3 节的「快照零改动」已实测成立**（2026-09-17）：`codex-tui` 全量回归 4285 passed / snapshot 类失败 **0**，见 [`i18n-verification.md`](./i18n-verification.md) 的「执行结果」表。
4. **构建成本：已补验（第 118 轮核对）。** `codex-rs/i18n/BUILD.bazel` 与 `codex-rs/i18n-check/BUILD.bazel`
   均已存在；Bazel 侧门禁 `bazel-i18n`（`bazel test //codex-rs/i18n:all //codex-rs/i18n-check:all`）
   实测 **2 tests pass / EXIT=0**，回执 `r-mu5hy0u9-431oj3`（`~/.dsh/state/swe-mode/receipts/`）。
   原记「未评估」作废。

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
