---
feature: i18n-completion-record-refresh
status: delivered
updated: 2026-10-04
branch: feat/i18n
commits: ff2fd98c7..867d16921
---

# i18n 完成度记录刷新（docs 与实测对齐）

## Report

**What was built** — 把 `docs/plan/i18n-verification.md` 的完成度记录与机器实测对齐：
① 修复 `not-translated-unwrapped.tsv` 中 `# Codex conversation\n` 哨兵的两条站点形态登记
（行号随工作区改动 +25 漂移 `:240/:299` → `:265/:324`，登记曾静默失效、`tui` 伪剩 2 条候选）；
② 用同口径命令重测全部八个依赖 `codex-i18n` 的 scope，合计 **7123/7123 = 100%、剩余 0**；
③ 文档三处落账：顶部新增「2026-10-04 最新实测」权威现状表（旧 2026-09-16 表标注为历史快照原文保留）、
§12.65 普查表尾加指针、追加 `### 12.73` 记录根因/修复/八 scope 普查/与 §12.72 的账本差异。
历史快照数字一律未改写。设计中「六 scope」在交付时扩为「八 scope」（补测 `core-plugins` 531 与
`codex-mcp` 119，均为既有在范围工作，是超集不是偏离）。

**Verification** —（全部 2026-10-04 实跑）

- 八 scope `python3 scripts/i18n_todo.py --root codex-rs/<crate>/src` → 全部 `unwrapped candidates : 0`
  （1281/1164/3229/75/16/708/531/119，合计 7123 = docs 表内数字）→ **PASS**
- `i18n_todo --root codex-rs/tui/src --dump-rows` → 0 行；`--audit-rows` silent no-op 0 → **PASS**
- `just i18n-check` → 4212 词条、missing/unused/spacing/nested/duplicate/placeholder/asset 全 0、
  coverage 4193/4202（99.8%）、EXIT=0 → **PASS**
- `npx prettier --check docs/plan/i18n-verification.md docs/compose/spec/...` → PASS；`codespell`（2 md + tsv）→ 0
- `find codex-rs -name '*.snap.new'` → 0（`.snap` 940）→ **PASS**
- 独立 reviewer 子代理三结论（spec 合规 / 正确性 / 一致性）全过、0 critical；其指出的
  §12.73 死行描述「HEAD 相对」问题已改写后提交
- 本任务无 Rust 源码改动 ⇒ cargo 测试不适用

**Journey log** —

1. 起步在多个历史完成度数字（40% / 98.5% / 全 0）间游走定位「2% 差距」，靠一轮定向提问收敛到「两处都刷新」；教训：先问清引用对象再搜证。
2. 「tui 剩 2」的根因是站点形态登记行号漂移（计划 §51.2 教训原样再现）；`--audit-rows` 结构性查不出漂移，判据只能是 `i18n_todo` 候选数对照。
3. 该哨兵值含真实换行且以 `#` 开头（TSV 行首 `#` 判注释）⇒ 值形态永不可用、站点形态必随行号烂；修完必须 `--dump-rows` 复核（已做，0 行）。
4. TSV 文件同时承载用户未提交批次的改动，无法干净拆分单独提交 ⇒ TSV 两行修复留在工作区，随用户批次一起入库（否则 §12.73 的普查数字在该 commit 上不可复跑）。
5. reviewer 交叉验证了表内算术（7123 求和、4193/4202=99.8%、行号对源码）并抓到一处 HEAD 相对表述——文档里描述行号内容时必须写明参照版本。

## [S1] Problem

`docs/plan/i18n-verification.md` 里两处完成度记录与机器实测不一致：

1. 顶部「2026-09-16 对现状的独立实测」表记 **完成度 ≈40%**（1262/3168），早已过期；
   同文件 §12.65 普查表记 **5660/5749 = 98.5%、剩 89**（第 588 轮快照），也已被 §12.68–§12.71 的
   「五 scope 归 0」超越。用户判断 docs 完成度与前面分析仍有约 2 个百分点差距。
2. 实测（2026-10-04，`scripts/i18n_todo.py`）显示 `tui` 反而**剩 2 条候选**：
   `tui/src/app/transcript_export.rs:265/:324` 的 `# Codex conversation\n` 哨兵。
   根因是**登记行号漂移**：`not-translated-unwrapped.tsv` 里该哨兵的两条站点形态登记仍指向
   旧行 `:240/:299`（工作区未提交改动使整文件下移 25 行），站点形态按 `path:line` 精确匹配 ⇒
   登记静默失效（计划 §51.2 已记过同一教训；`--audit-rows` 查不出漂移）。

## [S2] Design

- **登记修复（事实层）**：把 `codex-rs/i18n/not-translated-unwrapped.tsv` 中
  `transcript_export.rs:240` → `:265`、`:299` → `:324` 两行改到当前行号；值形态不可用
  （值含真实换行且以 `#` 开头），站点形态是唯一选项，修完用 `--dump-rows` 复核 tui 为空。
  判决沿用既有裁定（dict_zh.rs 注释「哨兵不译」/ §12.69 / §12.1 匹配键），不产生新裁定。
  TSV 中 `:241`/`:300` 两行指向非字面量行（`for cell` / `Ok(markdown)`），早已不匹配任何候选，
  属历史死行：本轮不动，仅在新普查小节如实记录其存在，避免范围蔓延。
- **完成度重测（同回合机器实测，口径写进文档）**：
  - 六 scope 普查：`cli` / `core` / `tui` / `exec` / `plugin` / `app-server` 各跑
    `python3 scripts/i18n_todo.py --root codex-rs/<crate>/src`，记 candidates / wrapped / remaining；
  - 字典与漂移：`just i18n-check`（词条数、rendered、coverage、全零项）；
  - 接入面：独立正则 `(?<![A-Za-z0-9_])tr(_with)?\(` 统计文件数与调用点数（与旧表同口径）；
  - 快照：`find codex-rs -name '*.snap' | wc -l` 与 `*.snap.new` 计数。
- **文档刷新（两处，历史快照不改写）**：
  1. 顶部现状表：在其上方新增「2026-10-04 最新实测」表（同列口径），旧表标注
     「⚠ 历史快照，完成度数据已被上方最新实测取代」——保留历史、指向最新；
  2. 追加 `### 12.73` 普查小节：六 scope 新普查表 + 登记漂移修复记录 + 证据命令，
     并在 §12.65 表尾加一行「⚠ 第 588 轮历史快照，最新见 §12.73」。
- **验收判据**：六 scope `i18n_todo` remaining 全为 0；`--dump-rows`（tui）为空；
  `i18n-check` 全零且 EXIT=0；docs 两处显示本轮实测数字且带口径与日期。

## [S3] Out of Scope

- 工作区其余未提交改动（app-server 批 K+L+…、dict_zh.rs、transcript_export.rs 的翻译、
  glossary 重排）——不提交、不回退、不改。
- `:241`/`:300` 两条历史死行的删除（留待后续，理由记录在 §12.73）。
- §12.65 及其它历史小节的数字**不改写**（历史快照），只加指针。
- `core-plugins` 范围裁决、`i18n-locale-zh` 门禁登记等 §12.72 未闭合三处。
- 平台受限 43 处 CI 复查（`i18n-platform-ci-checklist.md` 已有清单）。

## Tasks

- [x] T1: 修 TSV 哨兵登记行号（:240/:299 → :265/:324） — acceptance: `i18n_todo --root codex-rs/tui/src` 剩余 0，`--dump-rows` 为空 (covers: S2)
- [x] T2: 采集六 scope 普查 + `just i18n-check` + 接入面 + 快照计数 — acceptance: 每项数字带可复跑命令，落入口稿 (covers: S2; depends: T1)
- [x] T3: 刷新 i18n-verification.md 两处（顶部新增最新实测表并标注旧表为历史；追加 §12.73 并在 §12.65 加指针） — acceptance: docs 显示 2026-10-04 实测完成度，历史快照原文保留 (covers: S2; depends: T2)
- [x] T4: 验证 + review + finalize — acceptance: 六 scope 全零、i18n-check 全零、codespell 0（本任务文件）、reviewer 三结论通过 (covers: S2; depends: T3)
