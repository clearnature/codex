# i18n 前期技术验证计划

> 状态：**H1–H4 已执行**（结论见下方「执行结果」；§7 的范围边界有一处有意偏差，已注明）。
> 上游文档：[`i18n-design.md`](./i18n-design.md)（参考分析与嵌入点设计）。
> 基线：`feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。

## 执行结果（2026-09-16 实测）

| # | 假设 | 结论 | 证据（可复现命令 → 实测值） |
| --- | --- | --- | --- |
| **H1** | 「英文原文即 key」让 En 输出逐字节不变 | ✅ **成立** | `RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib -- --skip ide_context::ipc` → 4285 passed / 2 failed / **snapshot 类失败 0**。两条失败单独重跑均通过（并发时序型）。`ide_context::ipc` 10 条为环境型故显式 skip：其中 `fetch_ide_context_prefers_primary_socket` 单独跑仍 FAILED（panic 在 `tui/src/ide_context/ipc.rs:983`），另有一条永不返回。注：默认 2 MiB 测试线程栈会另致 `app::agents_overview` 一条 SIGABRT，故须带 `RUST_MIN_STACK`。 |
| **H2** | tui 文案可机械识别 | ✅ **成立**（带上界与已知残余） | `cd codex-rs && python3 ../scripts/i18n_scan.py` → 39,225 字面量 → 3,168 候选（8.1%），40 项抽样精确率约 85–90%；残余误判是喂模型的 `json!` payload 与内部 error-context。 |
| **H3** | 漂移可自动发现 | ✅ **成立** | `cargo run -p codex-i18n-check -- --root ..` → missing 0 / unused 0 / coverage 100.0%，且**退出码非零即代表真漂移**，可直接入 CI。 |
| **H4** | locale 解析与配置入口可落地 | ✅ **成立** | 五级链 `--lang` > `config.toml` 的 `locale` > `LC_ALL` > `LANG` > 系统 locale（`codex-i18n::resolve` / `resolve_from_process` + `sys-locale`）；`--lang` 实见于 `cargo run -q -p codex-exec -- --help`；`cargo test -p codex-i18n` → 30 passed。 |

**与 §7 的偏差（有意，且已记录）**：翻译已随 `i18n-design.md` §3.4 的铺开开始（首批 27 条，落在 footer）。这不是「H1 需要译文」——H1 只用机制即可成立——而是为了把「发布出去的语言真的到达渲染输出」变成一条可失败的测试：`tui/src/bottom_pane/footer.rs` 的 `the_rendered_hints_follow_the_language_they_are_given`。

## 铺开进度（滚动记录）

`i18n-design.md` §3.4 的铺开按轮推进，每轮以三条硬证据收口：`cargo test -p codex-i18n`（30 passed）、`just i18n-check`（missing 0 / unused 0 / coverage 100.0% / EXIT=0）、`RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib -- --skip ide_context::ipc`（**snapshot 类失败 0**）。剩余量用 `python3 scripts/i18n_todo.py --top 8` 量。

| 轮次 | 覆盖模块 | 字典条数 | 全量套件 |
| --- | --- | --- | --- |
| 28 | `resume_picker.rs`、`bottom_pane/app_link_view.rs` | 934 | 4285 passed / 2 failed / snapshot 0 |
| 29 | `chatwidget/slash_dispatch.rs`（32 对） | 965 | 4285 passed / 2 failed / snapshot 0 |
| 30 | `clipboard_copy.rs`（28 对） | 991 | 4285 passed / 2 failed / snapshot 0 |

**此表在 round 30 处停更。** 之后仍有推进（`2def58b6a` / `8b2a439be` / `e6847b6b8` 三个提交都改过 `dict_zh.rs`），
但没有逐轮记录。以下是 **2026-09-16 对现状的独立实测**（每行标注口径，可复跑）：

| 指标 | 实测值 | 口径 |
| --- | --- | --- |
| 字典条目 · 已提交（HEAD `e6847b6b8`） | 1226 | 用 `codex-i18n-check` 二进制指向 HEAD 版 `dict_zh.rs` 的副本 |
| 字典条目 · 工作区 | **1262** | `just i18n-check` |
| 漂移与覆盖 | rendered 1262；missing **0**、unused **0**、coverage **100.0%**、CJK 边界空格 **0**、`EXIT=0` | `just i18n-check` |
| 接入面 | **44 文件 / 1632 处 `tr`·`tr_with` 调用点** | 独立正则统计（`(?<![A-Za-z0-9_])tr(_with)?\(`） |
| 　`tui` | 42 文件 / 1548 点：`chatwidget` 11·420、`keymap_setup` 2·308、`bottom_pane` 7·248、`app` 9·179、`tui/src` 顶层 4·174、`history_cell` 5·89、`onboarding` 2·72、`status` 1·42、`ide_context` 1·16 | 同上 |
| 　`cli` / `exec` | 各 1 文件 · 47 / 37 点 | 同上 |
| 剩余候选 | **≈1831**：`<top>` 439、`chatwidget` 344、`app` 312、`bottom_pane` 256、`history_cell` 91、`pets` 85、`external_agent_config_migration` 70、`ide_context` 48 … | `python3 scripts/i18n_todo.py --top 15` |
| 完成度 | ≈ **40%**（1262 / H2 估的 3168 候选 = 39.8%，两口径吻合） | 上两行 |
| 未提交 | `i18n/src/dict_zh.rs` +124/-0（+36 条）；`exec/src/lib.rs` +188/-51 | `git diff --numstat` |
| 快照 | 877 个 `.snap`、**0 个待审 `.snap.new`** | `find codex-rs/tui/src -name '*.snap'` |

2026-09-16 复跑三条硬证据。第三条走 Bazel 侧，与本文的 cargo 口径**不完全等价**（Bazel：8 分片 + `flaky` 重试 +
`RUST_MIN_STACK=8388608`；本文：`RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib -- --skip ide_context::ipc`）：

- `cargo test -p codex-i18n` → **30 passed; 0 failed** —— 与本文记录一致。
- `just i18n-check` → 见上表，全绿。
- `bazel test //codex-rs/tui:tui-unit-tests //codex-rs/exec:exec-unit-tests` → `exec` PASSED；`tui` FLAKY
  （12 分片 4 次首试失败，重试后通过），**失败集合中没有任何快照测试**，故 **snapshot 类失败 0** 成立。
  本轮失败集与第 28/29/30 轮**均不重合**，再次印证「逐轮换人」判据：
  - `app::tests::background_exit_tests::exit_interrupts_before_requesting_shutdown` —— 第 28 轮已记过；
  - `ide_context::ipc::tests::fetch_ide_context_uses_unregistered_request_route` —— 本节已标为环境型；
  - `startup_draft::tests::startup_draft_does_not_turn_a_standalone_enter_into_a_newline_at_handoff` —— **新增**。
    断言 `pump.pending_paste_newline.is_some()`（lookahead 时间窗）。该模块未接入 i18n，本分支从未改动此文件，
    其最后改动是上游 `9587c9ef3`（2026-09-06）且基线中已存在 → 负载敏感型 flaky，与适配无关。

> **续记（2026-09-16，第 42–43 轮）**：字典 **1287 → 1326**。新接入三个文件：`core/src/config/mod.rs`
> （26 条配置校验消息，§3.4 步骤 5 开工；core 其余大头是喂模型的 `tools/*_spec.rs` / `guardian/prompt.rs`，不译）、
> `tui/chatwidget/permission_popups.rs`（23 条）、`tui/app/agents_overview.rs`（18 条）。
> `python3 scripts/i18n_todo.py` 实测 tui 口径 **已包 1394 / 剩 1788**。同一提交序列里还修掉一个 HEAD 上的红测试
> （`config_schema_matches_fixture` 生成物过期，见 §八.2）。

### 两条失败的归属（H1 的对抗自检）

连续三轮、同一命令下，失败集合**互不相同**：

- 第 28 轮：`app::tests::background_exit_tests::exit_interrupts_before_requesting_shutdown` + `app::tests::safety_buffering::agents_overview_acknowledges_inactive_steer_before_interrupt`
- 第 29 轮：`app::tests::safety_buffering::active_turn_interrupt_is_nonblocking_and_coalesces_repeated_requests` + `app_server_session::rollout_history::tests::cached_legacy_resume_revalidates_history_across_migration_settings`
- 第 30 轮：`app::tests::safety_buffering::agents_overview_acknowledges_inactive_steer_before_interrupt` + `app_server_session::rollout_history::tests::cached_legacy_resume_revalidates_history_across_migration_settings`

**测试集逐轮换人**本身就是判据：i18n 接入是确定性变换（En 分支返回 key 本身），不可能让失败集合在相同命令下漂移；这属于并发时序型 flaky。H1 的承重判据仍是 **snapshot 类失败 0**（877 个界面快照逐字节守住 En 输出）。

### 已知工具链限制（非本改动引入）

`just fmt` 在本机**退出码 1**：rustfmt 段通过，失败在最后一步 Bazel/Starlark 格式化 —— `[Errno 2] No such file or directory: 'dotslash'`。Rust 侧格式化改用 `cargo fmt --all`（同配置，退出码 0）。

## 一、为什么先做"前期验证"

`i18n-design.md` §3.3 给出了一个策略性赌注：**用「英文原文即 key」保住 877 个界面快照不动**。

这个判断目前是**推断，不是实测**。它一旦不成立，整个接入方式要重做——所以在写任何 codemod、
翻译任何一条文案之前，先用**最小可证伪实验**把它验掉。

## 二、参考：qwen-code 的中文国际化

`qwen-code`（`@qwen-code/qwen-code` 0.23.0，Node/TS monorepo）已有一套**生产级、CI 强制**的 i18n，
是与 codex 形态最接近的参考。

| 组成 | 位置 |
| --- | --- |
| 语言资源（9 种） | `packages/cli/src/i18n/locales/{en,zh,zh-TW,ru,de,ja,pt,fr,ca}.js`（各 140–177 KB） |
| 主 API | `packages/cli/src/i18n/index.ts` — `t(key, params?)`、`setLanguage`、`resolveLanguage`、`detectSystemLanguage` |
| 语言定义 | `packages/cli/src/i18n/languages.ts` — `SupportedLanguage` + `LanguageDefinition` |
| 强制翻译清单 | `packages/cli/src/i18n/mustTranslateKeys.ts` — `MUST_TRANSLATE_KEYS` |
| CI 检查 | `scripts/check-i18n.ts`（691 行），由 `.github/workflows/ci.yml:1172` 调用 |
| 用户入口 | `packages/cli/src/ui/commands/languageCommand.ts`（`/language` 命令） |

### 2.1 关键印证：key 就是英文原文

`locales/ca.js` 的实际内容：

```js
'↑ to manage attachments': '↑ per gestionar els adjunts',
'Attachments: ': 'Adjunts: ',
'Use {{symbol}} to specify files for context (e.g., {{example}}) to target ...':
  'Useu {{symbol}} per especificar fitxers de context ...',
```

**这与 `trha` 的 Haskell 实现（`packages/app/CLI/UI/I18n.hs`）是同一策略。**
两个互不相关的项目、两种语言，都选择了「英文原文即 key」——说明它是 CLI 国际化的**成熟模式**，
而非权宜之计。`index.ts` 的注释也写明设计目标：*"so English and untranslated tools are unaffected"*。

### 2.2 检查机制（本计划要参考的核心）

`check-i18n.ts` 的检查维度：

| 检查 | 实现 |
| --- | --- |
| 缺失 key | `missingKeys` |
| 代码里已不用的 key | `findUnusedKeys` |
| 只在 locale 里存在的 key | `findKeysOnlyInLocales` |
| 必须翻译的 key 是否漏翻 | 配合 `MUST_TRANSLATE_KEYS` |
| en 表自身 key/value 一致性 | `checkKeyValueConsistency` |
| 翻译覆盖率 | `countTranslatedKeys` |
| zh-TW 混入简体字／大陆词汇 | `findForbiddenZhTwPatterns` |
| **从源码反查实际使用的 key** | `extractUsedKeys(sourceDir)` |

## 三、四个可证伪假设

每个假设都写明**反证条件**，避免做成"跑一遍看起来没问题"的假验证。

| # | 假设 | 反证条件 | 成本 |
| --- | --- | --- | --- |
| **H1** | 「英文原文即 key」能让 En 输出**逐字节不变** | 接入后**任何一个快照**出现 diff | 半天 |
| **H2** | `tui` 文案形态**适合机械识别**（codemod 可行） | 扫描器无法区分「用户可见文案」与「内部字符串」，误判率高 | 1–2 小时 |
| **H3** | 漂移检测可行（新增／失效 key 能被自动发现） | 造不出 `extractUsedKeys` 的等价物 | 半天 |
| **H4** | locale 解析与配置入口能落地 | 现有 `sys-locale` / config 机制无法承载 | 2 小时 |

**执行顺序：H1 → H2 → H3 → H4。** H1 优先——它一旦失败，后续都没意义。

## 四、H1 详细步骤（承重墙）

### 步骤

1. 新增 `codex-rs/i18n` crate：
   - `Lang` 枚举（至少 `En`、`Zh`）
   - `tr(lang, key) -> &'static str`，其中 `En` 分支**直接返回 key 本身**
   - 空的 `dict_zh`（先不翻译任何东西）
2. `codex-rs/tui` 加对 `codex-i18n` 的依赖，**但不改任何调用点**
3. 跑 `just test -p codex-tui` → 877 个快照应**全绿**（基线：证明「仅加 crate」不破坏任何东西）
4. 在 `tui/src/bottom_pane/footer.rs` 选 **1–3 条**文案，包成 `tr(lang, "原文")`
5. 再跑 `just test -p codex-tui`

### 判据

| 结果 | 结论 |
| --- | --- |
| 快照全绿 | **H1 成立** —— 策略可用，可放心铺开 |
| 出现 diff | **H1 被证伪** —— 必须重新设计（例如接受批量 accept 快照，或改用别的接入方式） |

### 这个实验的设计要点

它**不需要**：写 codemod、翻译任何文案、改架构。
只需要：新建一个小 crate + 改几行 + 跑一次测试。**这是能验证承重墙的最便宜方式。**

## 五、H2 / H3 / H4 的方法

### H2：只读扫描器（先统计，不改代码）

遍历 `codex-rs/tui/src/**/*.rs`，提取字符串字面量，按规则分类：

- 疑似用户可见（含空格、像句子、长度阈值）
- 内部：`key`、URL、格式串、tracing 日志、测试断言、`snapshots`

输出统计 + 抽样人工复核。**准确率高才谈得上 codemod**；否则要退回「缩小覆盖范围」的方案。

### H3：漂移检测原型

移植 `check-i18n.ts` 的最小三项（缺失 / 失效 / 覆盖率）：
从 Rust 源码提取 `tr(lang, "…")` 的第一个参数作为「实际使用的 key」，与 `dict_zh` 对账。

### H4：locale 入口

复用已存在的 `sys-locale 0.3.2`（探测系统语言），并参考 qwen 的 `code` / `id` 分离设计。

**注意不要动** `core/src/unified_exec/process_manager.rs:93` —— 那里把子进程的 `LANG` / `LC_ALL`
固定为 `C.UTF-8` 是为了让 shell 输出稳定可解析，与界面本地化无关，改了会破坏工具输出解析。

## 六、从 qwen-code 可复用的清单

| qwen-code 的做法 | codex 的对应 |
| --- | --- |
| key = 英文原文 | 直接采用（H1 即在验证它） |
| `{{var}}` 插值 | Rust 用 `{var}` 或保持同格式 |
| `code` / `id` 分离（内部 code vs UI 标准名） | 采用（Rust `Lang` vs 配置里的 `zh-CN`） |
| `strictParity` 渐进式 key 对齐 | **采用** —— 上万条文案不可能一次达标 |
| `MUST_TRANSLATE_KEYS` 强制清单 | 采用（先标 slash 命令描述等高可见度文案） |
| `check-i18n.ts` 的检查维度 | 移植为 Rust 工具（至少三项：缺失 / 失效 / 覆盖率） |
| `detectSystemLanguage()`（靠 `Intl`） | 用 `sys-locale`（依赖已在 `Cargo.toml`） |
| 内置 + 用户 locales 双目录 | 可选，长期有价值（让用户自带语言包） |

## 七、范围边界（本阶段不做）

- **不翻译任何文案** —— H1 只需要机制，不需要译文
- **不写 codemod** —— H2 先只做统计
- **不碰 `core` 与协议层** —— `app-server-protocol` 是跨进程契约，字段不可动
- **不碰提示词资产** —— `core/*.md`、`core/templates/*` 是喂给模型的，属独立决策
- **不评估桌面端** —— 闭源，不在可控范围

## 八、未验证 / 风险

1. **H1 已实测成立**（2026-09-16，见「执行结果」）——不再是推断。残余风险转移到「铺开的广度」：接入面已从最初的 `bottom_pane/footer.rs` 扩到 **44 文件 / 1632 处调用点**（2026-09-16 实测，见「铺开进度」——含 `chatwidget` 11 文件·420 点、`app` 9·179、`bottom_pane` 7·248），**未接入**部分尚余 **≈1831** 个候选（`chatwidget` 344、`app` 312、`bottom_pane` 256 …），故本报告中的「快照零 diff」只保证**已接入部分**无损。
2. ~~**Bazel 侧成本未评估**~~ **已评估（2026-09-16）**：改动量是 **1 行** —— `codex-rs/i18n-check/BUILD.bazel`
   加 `crate_srcs = []`（纯 bin crate，否则默认的 `src/**/*.rs` glob 会把 `*_tests.rs` 一并编成 library）。
   **不需要**任何 `compile_data` / `build_script_data`：本分支没有新增 `include_str!` / `include_bytes!`。
   验证：`bazel build //codex-rs/cli:codex //codex-rs/exec:all` → 4512 actions / exit 0；
   `bazel test //codex-rs/i18n-check:i18n-check-tests` → 2/2 PASSED。细节见
   [`i18n-bazel-handoff.md`](./i18n-bazel-handoff.md)。
3. **`tui` 的文案量与扫描口径未定** —— [`../maps/references.md`](../maps/references.md) 里的
   12,681 条是启发式**上界**（含测试代码），实际可翻译量要靠 H2 的扫描结果确定。
4. **未评估构建时长影响** —— 新增 workspace crate 会增大构建规模。
5. **`unused 0` 不是纯精确匹配** —— `codex-i18n-check` 除静态调用点外，还会把「文件里出现过
   `codex_i18n`」且带 `label: "<字面量>"` 字段的字符串计为**已渲染**（本次报 bound keys **28** 条）。
   方向是保守的（宁可少报 unused），但读「unused 0」时要知道里面有这层启发式；key 是否真的到达
   渲染输出，仍要靠快照与 `codex-i18n` 的插值测试兜底。
6. **日期更正** —— 本文早先多处写「2026-09-17」，而实际测量与提交都是 **2026-09-16**（`git log` 时间戳 +
   本机系统日期一致）。已更正；台账里更早的几条流水也写成了 09-15/09-17，那批是未经核对的估值，
   按纪律**不追改历史**，以本文与提交时间戳为准。

7. **`i18n_todo` 对短串系统性低估** —— 扫描器把短于 8 字符的字面量归入 `internal:short`
   （`scripts/i18n_scan.py` 的 `MIN_CANDIDATE_LEN = 8`），所以 `--top` 榜上看不到**短 UI 片段**。
   实测锚点：`tui/src/keymap_setup/picker.rs` 的 `" close"`（6 字符）、`"Keymap"`（6）、`"Debug"`（5）
   都不在候选表里，却都是用户可见文案。处置：短片段**跟随所在界面一起处理**（本轮随 picker 一起包装），
   不做独立的短串扫荡（`--only short` 下 tui 253 条、keymap 205 条，绝大多数是内部标识符，噪声过大）；
   漏译判定以 `codex-i18n-check` 的 missing/unused 为准，最终兜底是 locale 快照。

### 9.1 补充类别（第 54–55 轮新增）

5. **协议 / 错误匹配串** —— 不译，且**译了就坏**。锚点：`tui/src/app_server_session.rs:209-222` 的
   `["historymode", "history mode", "excludeturns", "exclude turns", "thread/turns/list",
   "thread/items/list"]` 与 `:262` 的 `["dynamictools", "dynamic tool", "namespace", "inputschema"]`
   —— 它们用 `message.contains(field)` **匹配服务端返回的英文错误文本**来决定是否降级重试。
   翻译会静默破坏回退逻辑（不是排版问题，是逻辑错误）。
6. **配置键值转储** —— 不译。锚点：`tui/src/debug_config.rs:36,50,51,61,63,73,78,83,425,628`
   的 `  - network_proxy` / `    - HTTP_PROXY  = http://{addr}` / `  - enabled = {}` /
   `{key} = {value}` 等行：内容全是 **config.toml 的键名与值**，用户的动作是把键名贴回配置文件，
   译名反而不可用；这些行没有散文（真正的散文如 `(V1 only; ignored by V2)` 已单独包装成
   `  - max_depth = {0} (V1 only; ignored by V2)` 模板）。
7. **thiserror 属性宏**（**已知缺口，不是"已排除"**）—— 锚点：`tui/src/app_server_session.rs:393-396`
   的 `#[error("the selected permission profile cannot be safely represented by the legacy
   app-server sandbox policy; …")]`。`#[error(..)]` 里放不下 `tr()` 调用（属性宏要求字面量），
   要译只能把该 variant 改成手写 `Display`。**当前未译，登记为待办**，不要当成已排除。

## 十、locale 入口的两个真实缺陷（第 58 轮实测）

1. **启动顺序：`help = tr(..)` 永远是英文（已修）**。`--lang` 只有 clap 解析完之后才知道，而 clap 的
   `help = tr(current(), "…")` 是**解析期**求值的 —— 于是「解析后发布语言」这条路对 help 文案永远晚一步。
   修法：在 `cli/src/main.rs::main()` 里先预扫描 `std::env::args()`（`--lang zh` 与 `--lang=zh` 两种拼法），
   把语言发布一次，再做 `MultitoolCli::parse()`。
   *证据*：修复并重建后 `./target/debug/codex --lang zh login --with-api-key --help` 输出
   `从标准输入读取访问令牌（例如 \`printenv CODEX_ACCESS_TOKEN | codex login --with-access-token\`）`
   （CJK 行数 2）；同一命令不带 `--lang` 仍为英文。
   *诚实边界*：没有做「回退后重建再对比」的 A/B（一次重建约 4 分钟），「修复前是英文」由代码路径
   （`set_current` 在 `parse()` 之后、`current()` 默认 `En`）支持，不是实测对照。
2. **集成测试必须钉 locale（已处理）**。发布提前之后，OS locale 为中文的机器上跑 `cargo test -p codex-cli`
   会失败：CLI 自己渲染成 `错误：stdin不是终端`，而测试断言英文。按 `core/tests/common/test_codex_exec.rs`
   的既有先例，在 `cli/tests/*.rs` 的 spawn 辅助函数里钉 `.env("LC_ALL", "C")`（17 个文件）。
   *证据*：`cargo test -p codex-cli` 全部套件通过（13/275/5/4/1/3/2/2/1/2/3/7 …，0 failed）。

### 10.1 门禁：`just i18n-smoke`（第 59 轮）

`i18n-check` 是**静态对账**（字典 ↔ 渲染点），它证明不了「语言真的到达渲染面」——§十 的启动顺序缺陷就是
1800 条译文在生产路径上看不见的例子。补的门禁跑真实二进制：

```
just i18n-smoke      # cargo run -p codex-cli --bin codex -- --lang zh --help，断言 ≥5 行 CJK
```

*证据*：`just i18n-smoke` → `OK（26 行中文）`，EXIT=0；把阈值临时抬到 99999 用同一段代码 → 打印
`期望至少 99999 行中文帮助，实际 26 行` 且 EXIT=1（门禁不是空转）。CI 里已加到 `repo-checks.yml`
（`i18n-check` 之后）。

### 10.2 clap 的 `about` 与 doc comment（第 59 轮踩到的坑）

clap 从 **doc comment** 推导 `about` 时会**去掉句尾句点**；显式 `about =` 不会。所以把 doc comment 换成
`about = tr(current(), "…")` 时必须把键里的句尾句点一并去掉，否则英文帮助会多出一个 `.` ——
`cli/src/snapshots/…exec_server_help_documents_remote_options.snap` 立刻抓到（old 无句点 / new 有句点）。
处置：27 个 `about` 的英文键与字典键同时去掉句尾句点（值仍保留中文句号）。

## 十一、短键复用审计（第 63 轮）

**动机**：包装时的对抗自检指出一个风险 —— 同一个英文键可能出现在多个界面（如 `read {0}` 既是
审批摘要里的动词，也是宠物资源错误上下文），单键多义会让一处译文在另一处变味。

**方法（可复现）**：用正则收集全仓 `tr(current(), "…")` / `tr_with(current(), "…", …)` 的渲染点，
按 key 聚合到「文件集合」，再与字典键求交，输出「长度 ≤ 12 且渲染于 ≥2 文件」的清单。

**结果**：
* 多文件渲染的键 **60** 个；其中 ≤8 字符 **18** 个，≤12 字符 **32** 个。
* ≤8 字符的 18 个逐一复核（`read {0}`、`  Press `、` to save`、`Agents`、`Approval`、`Composer`、
  `Editor`、`Cancel`、`Plugin`、`Reason: `、`Running`、`Server: `、`Source`、`Status: `、`Working`、
  `disabled`、`item`、`items`）：**都是通用词，各处语义一致，无需拆分**。
* 两处值得记录的判定：
  * `read {0}` / `write {0}` → 「读取{0}」/「写入{0}」：在 `bottom_pane/approval_overlay.rs` 是
    **审批摘要动词**，在 `pets/*` 是**错误上下文**（`read /path: No such file`）。两处都读作
    「读取 X」，共用可接受。
  * `item` / `items` → 「条目」：`external_agent_config_migration/render.rs` 与
    `chatwidget/status_surfaces.rs` 共用；**刻意避开「项目」**，以免与 project 混淆。

**诚实边界**：本轮判定依据是**渲染点所在文件名**，没有逐行读完 60 个站点；`Back` / `Open` 这类
多义动词的残余风险仍在。要更严就得把 60 个站点做成表格逐条读——**未做**，登记为后续。

**可复用规则**：写新短键前先 `grep '("KEY"'` 查字典；若条目已存在但语义不同，**不要复用**，
改用更长、自带上下文的键（` to toggle; ` 这种带尾随分号的碎片键就是这么来的）。

## 九、翻译口径判据（"不译"的依据）

判据是**文本流向**，不是「用哪个宏产生」：同一段文案经 `.context(…)` 走到 UI 就译，进日志 / 协议 /
模型上下文就留英文。迄今结论分四类，逐类给锚点。

1. **模型面向（model-visible）** —— 不译。译了会改变模型看到的输入，而英文原文是 key 的语义基准。
   锚点：`tui/src/goal_files.rs` 的 `GOAL_FILE_PREFIX` / `GOAL_FILE_SUFFIX` / `pasted text file:` /
   `- [Image #n]:` / `image file:` / `Referenced image files:` / `Referenced image URLs:`（本文件里
   其余 8 条**用户可见**的错误文案已译，见同文件 `:43-249`）；`tui/src/app/side.rs` 的模型提示；
   `dynamic_tools.rs` 的工具规格。
2. **协议 / 机器契约** —— 不译。改字符串等于改对外契约。锚点：doctor 子系统的 `--json` 输出与
   `tracing::*` 的 message 字段、app-server 响应载荷、`ide_context/ipc.rs` 的 `#[error]` 与诊断串、
   `DEFAULT_TOKEN_BUDGET_REMINDER_MESSAGE_TEMPLATE`（被写进用户 config.toml）、子命令路径名。
3. **预览占位符（假数据）** —— 不译。锚点：`tui/src/bottom_pane/status_surface_preview.rs:44-79`
   的 `StatusSurfacePreviewItem::placeholder`，它返回的是**模拟真实值的样例**（`"Working"`、
   `"thread name"`、`"Context 0% left"`、`"0 window"`、`"5.2 credits"`、`"gpt-5.2-codex medium"`、
   `"Tasks 0/0"`）。这些不是给用户读的句子，而是让预览看起来像真数据的样本；译了会把「占位」变成
   「文案」，并使预览与真实值形态不一致（真值随 locale 变化会与样例对不上）。
   反例（**该译**）：同文件 `:255-292` 的 `rate_limit_preview_copy` 返回的是**说明文字**
   （`Remaining usage on the … limit (omitted when unavailable)`，7 条），属用户可见文案，已译；
   其中 primary / secondary 两条此前已由 `chatwidget/status_surfaces.rs` 覆盖，字典里已存在
   （`dict_zh.rs` 的 `主用量限额的剩余额度（不可用时省略）` / `次用量…`）。
4. **`unused 0` 与 `label:` 启发式** —— 口径见 §八.5；读该数字时必须知道里面有这层保守启发式。

> 纪律：每一批的「不译」都要在这里留一行锚点（文件:行 + 为什么），否则下次扫描会把同一批候选
> 重新捞出来，形成「反复判断同一件事」的噪音。

## 十二、"不译"锚点总表（第 78–84 轮判定）

按 §九 的口径逐条判定后**保持英文**的位置，附 `文件:行` 与依据。这些行**不是漏做**：判据见 §9.1
（按文本流向判定）、§9.1.5（匹配表 / 查找键）、glossary:37/39（不译清单）。任何一批新增判定必须
续在本表末尾，否则下一轮扫描会把同一批候选重新捞出来。

### 12.1 匹配 / 解析管线（译了会静默失效）

| 位置 | 依据 |
| --- | --- |
| `tui/src/chatwidget/turn_runtime.rs:15,17` | 安全拦截前缀用 `starts_with` 匹配服务端英文报错；译了检测直接失效（源码内已有中文注释） |
| `tui/src/chatwidget/warnings.rs:3,5` | `FALLBACK_MODEL_METADATA_WARNING_SUFFIX` 的后缀被 `fallback_model_metadata_slug()` 用反引号扫描 |
| `tui/src/chatwidget/permission_popups.rs:83` / `permissions_menu.rs:110` | `preset.description.replace(" (Identical to Agent mode)", "")` 后缀剥离 |
| `tui/src/app/session_start.rs:69` | `archived_prefix` 与 `archived_session_guidance()` 的 `starts_with` 配对 |
| `tui/src/app/config_update.rs:257,301` | `split_once(", add ")` / `rsplit_once(" as a trusted project in ")` 解析服务端 `disabledReason` |
| `tui/src/external_agent_config_migration/mod.rs:208,248` | `"Import …"` 归一化后又用 `strip_prefix("Import enabled plugins from ")` 二次匹配 |
| `tui/src/status/thread_usage.rs:23-38,210-235` | 档位表被 `.position(|v| v == display_name)` 与分组 `entry(key)` 当键使用 |
| `tui/src/markdown_render/local_links.rs:19` | 正则字面量 |
| `tui/src/keymap_setup/actions.rs:462-466` | `const fn label()` 的静态标签，供调试表按来源分组 |

### 12.2 喂给模型的文本（设计 §4 决策 3）

| 位置 | 依据 |
| --- | --- |
| `tui/src/dynamic_tools.rs:156-232` | 工具描述与参数模式（模型读） |
| `tui/src/dynamic_tools.rs:360-1178` | 工具参数校验/错误文案（回给模型）；`<codex_delegation>` 载荷 |
| `tui/src/goal_files.rs:21-125` | 追加到提示词的 "Read the Codex goal objective file at …" |
| `tui/src/task_mentions.rs:35,266` | `## My request for Codex:` / `## Referenced chats with Codex:` 提示词脚手架 |
| `tui/src/terminal_visualization_instructions.rs:4` | 追加到提示词的可视化规则 |
| `tui/src/git_action_directives.rs:109,127` | 指令文本 |
| `tui/src/chatwidget/plan_implementation.rs:27-31` | `PLAN_IMPLEMENTATION_CLEAR_CONTEXT_PREFIX`；同一 const 里 `"Implement the plan."` 亦然 |
| `tui/src/app/recap.rs:59-64` | 回顾生成提示词 |
| `tui/src/app/side.rs:56,70` | 侧会话边界文本，注入模型上下文 |
| `tui/src/app/thread_title.rs:246-362` | 标题生成提示词与 `<message role=…>` 模板 |
| `tui/src/bottom_pane/request_user_input/mod.rs:942` | `user_note: …` 回填进 `ToolRequestUserInputAnswer` |

### 12.3 诊断 / eyre 上下文 / 内部错误

`tui/src/app/{safety_buffering.rs:303, event_dispatch.rs:494, history_pagination.rs:30, resize_reflow.rs:533,
session_picker.rs:128, resume_config.rs:15, side.rs:762}`、`app/config_persistence.rs:28`（`{error_context} task failed`）、`app/config_update.rs:221,223`（远程项目信任解析的 eyre 上下文）、
`tui/src/{get_git_diff.rs:137,157,180, npm_registry.rs:34-66, named_session_lookup.rs:33,89,110,123,133,
terminal_probe.rs:101-124, windows_sandbox.rs:127,130,133, update_versions.rs:12, session_resume.rs:76,
startup_error.rs:6, wrapping.rs:392, app_server_connection.rs:20, dynamic_tools_mcp.rs:137,264,
session_queue_commands.rs:155}`、`tui/src/{notifications/bel.rs:29, notifications/osc9.rs:59, terminal_title.rs:96,
tui.rs:261,282,424,427,455}`（WinAPI/stdio 内部消息）。

### 12.4 `#[error(...)]`（thiserror 只接受字面量）

`tui/src/app_server_session.rs:397`、`tui/src/named_session_lookup.rs:25`、`tui/src/external_editor.rs:23,26,28`。
要翻译需先手写 `Display` 实现，属独立改动。

### 12.5 代码样本 / 配置转储 / 数据格式

`tui/src/theme_picker.rs:68-118`（Rust 代码样张）、`tui/src/debug_config.rs:53-655`（key=value 转储）、
`tui/src/app/history_ui.rs:422` 与 `tui/src/{clipboard_copy.rs:338, clipboard_paste.rs:219}`（内嵌 PowerShell）、
`tui/src/update_action.rs:56,65`（安装命令）、`tui/src/branch_summary.rs:378`（HTTP 头）、
`tui/src/resume_picker.rs:3524`（strftime）、`tui/src/inline_visualization.rs:239` 与
`tui/src/resume_picker_transcript_preview.rs:294,297`（重写后的 markdown 脚手架）、
`tui/src/inline_visualization/viewer.rs:18-82`（CSP/CSS/HTML）、
`tui/src/bottom_pane/multi_select_picker.rs:92`（分隔线）、`tui/src/status_indicator_widget.rs:78,83`（时长格式）、
`tui/src/history_cell/notices.rs:54` 与 `tui/src/update_prompt.rs:196`（emoji + U+200A）。

### 12.6 产品名 / 标识符（glossary:37,39）

`tui/src/status/card.rs:760`（OpenAI Codex）、`tui/src/status/helpers.rs:112-122`（Enterprise (Automation) /
Business Premium / Pro Lite / Edu Plus 等套餐名）、`tui/src/model_catalog.rs:12`（Luna Reserve）、
`tui/src/pets/catalog.rs:80`（Null Signal）、`tui/src/external_agent_config_migration/source.rs:41`（Claude Code）、
`tui/src/chatwidget/agent_status_feed.rs:161`（`MCP {server}/{tool}` 标识符）。

### 12.7 关键词表 / 调试转储 / 其他

`tui/src/pets/picker.rs:94`、`tui/src/keymap_setup/picker.rs:362`（搜索关键词 blob）、
`tui/src/keymap_setup/debug.rs:238`（`code={:?} modifiers={} kind={:?}` 转储）、
`tui/src/bottom_pane/status_surface_preview.rs:54-79`（预览样例值，见 §九.3）、
`tui/src/app/{app.rs:770,789}`（断言片段）。

### 12.9 第 85 轮补录（此前分散在 §九 之外的口径）

| 位置 | 依据 |
| --- | --- |
| `tui/src/ide_context/ipc.rs:49-393`（33 条） | IDE 上下文协议错误与 socket 诊断；IPC 层，不对用户渲染 |
| `tui/src/ide_context/prompt.rs`（11 条） | 注入模型的 IDE 上下文提示词 |
| `tui/src/ide_context/windows_pipe.rs:186-343` | Windows 命名管道内部错误（WinAPI） |
| `tui/src/tui/terminal_stderr.rs:80-291`（9 条） | 终端 stderr 抑制状态机的内部断言/状态标签 |
| `tui/src/tui/input_boundary.rs:38,92,109` | `io::Error` 诊断 |
| `tui/src/tui/keyboard_modes.rs:110,262,284,306` | `cmd.exe /c set TERM_PROGRAM` 参数与「legacy Windows API 未实现」内部错误 |
| `tui/src/streaming/controller.rs:411,444,463` | `tracing::trace!` 消息 |
| `tui/src/chatwidget/plugin_catalog.rs:1753-1760` | `Git · url@ref` / `npm · pkg@ver`：坐标标识符（`Git`/`npm` 为产品名） |
| `tui/src/chatwidget/goal_status.rs:87,100` | `N / M tokens`：唯一英文词 `tokens` 在 glossary:37 的不译清单内 |
| `tui/src/app_backtrack.rs:495-515` | 分支失败内部原因（`Result` 控制流） |
| `tui/src/app/app_server_events.rs:416,418,490` | app-server 工具错误文本与 eyre 上下文 |
| `tui/src/app/startup_prompts.rs:94,95` | `    {display_index}. {folder}` / `       {reason}`：列表脚手架，数据原样渲染 |
| `tui/src/app/agent_status_feed.rs:161` | `MCP {server}/{tool}` 标识符 |
| `tui/src/startup_draft.rs:70,219` | 启动取消/输入流关闭的内部原因 |
| `tui/src/bin/md-events.rs:7` | 调试用二进制（打印 markdown 事件）的诊断输出 |

### 12.10 第 85 轮补录（最后 18 条开放候选）

| 位置 | 依据 |
| --- | --- |
| `tui/src/app_server_session/fs.rs:119-131`（5 条） | TUI 侧 app-server 文件系统请求的 eyre 上下文（`{method} failed in TUI`） |
| `tui/src/app_server_session/history.rs:130,151,208` | 分页历史加载的 eyre 上下文 |
| `tui/src/tui.rs:261,282` | WinAPI 未实现提示（与 `terminal_title.rs` 同类） |
| `tui/src/tui.rs:424,427,455` | `stdin/stdout is not a terminal` 诊断与 `tracing` 消息 |
| `tui/src/chatwidget/tool_lifecycle.rs:227` | 工具结果缺失的内部 `Err(String)` |
| `tui/src/chatwidget/user_messages.rs:304` | `[Pasted Content N chars]` 占位符进的是**发给模型的用户消息** |
| `tui/src/external_agent_config_migration/flow.rs:335` | `tracing::warn!` 消息 |
| `tui/src/chatwidget/reset_credits.rs:44` | strftime 格式 |
| `tui/src/update_prompt.rs:200` | emoji + U+200A |

### 12.8 待人类裁决

`tui/src/app/transcript_export.rs:158-300`（导出 markdown 正文与标题，等 export-scaffolding 裁决）。
