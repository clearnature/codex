# i18n 前期技术验证计划

> 状态：**H1–H4 已执行**（结论见下方「执行结果」；§7 的范围边界有一处有意偏差，已注明）。
> 上游文档：[`i18n-design.md`](./i18n-design.md)（参考分析与嵌入点设计）。
> 基线：`feat/i18n` @ `rust-v0.154.0`（commit `6b9826e3`）。

## 执行结果（2026-09-17 实测）

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

1. **H1 已实测成立**（2026-09-17，见「执行结果」）——不再是推断。残余风险转移到「铺开的广度」：目前接入面只覆盖 `bottom_pane/footer.rs`，`chatwidget`（705 候选）与 `app`（476 候选）尚未接入，故本报告中的「快照零 diff」只保证**已接入部分**无损。
2. **Bazel 侧成本未评估** —— `AGENTS.md` 要求新增 crate 时同步更新 `BUILD.bazel`
   （`compile_data` / `build_script_data` 等），本轮未衡量改动量。
3. **`tui` 的文案量与扫描口径未定** —— [`../maps/references.md`](../maps/references.md) 里的
   12,681 条是启发式**上界**（含测试代码），实际可翻译量要靠 H2 的扫描结果确定。
4. **未评估构建时长影响** —— 新增 workspace crate 会增大构建规模。
