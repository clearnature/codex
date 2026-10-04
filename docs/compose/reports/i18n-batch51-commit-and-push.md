---
feature: i18n-batch51-commit-and-push
status: delivered
specs: []
plans:
  - docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md
branch: feat/i18n
commits: 762d8315a..448b5cc8e
---

# i18n 收尾批：工作区提交 + 分支推送 — Final Report

## What Was Built

对 `feat/i18n` 工作区积压的 44 个已完成文件（app-server 批 Q 156→0 的 39 个源文件、
`transcript_export.rs` 导出活动行接 `tr`、`not-translated-unwrapped.tsv` 哨兵行号漂移修复、
`dict_zh.rs` +344 行词典、两份计划文档）执行了全量门禁验证后单提交入库
（`762d8315a`），并把本地积压的全部 7 个提交 fast-forward 推送到 `origin/feat/i18n`
（`6e497323d..448b5cc8e`），随后确认台账仅剩的两类遗留已登记在案。

完成后状态：八 scope 普查 **7123/7123 = 100%（remaining 0）**、`just i18n-check`
EXIT=0（4212 词条 / missing·unused 等全 0 / coverage 99.8%）、`.snap.new` = 0、
工作区干净、本地与远端 0/0 一致。这是一个「验证 → 单提交 → 推送」的收尾任务，
未新增任何源码逻辑；唯一新文件是本任务的执行计划与本报告。

## Architecture

无新组件。流程为四任务串行门禁：

1. **i18n 一致性门禁**：`just i18n-check`、八 scope `scripts/i18n_todo.py --root …` 普查、
   `--dump-rows`/`--audit-rows` 登记复核、`.snap.new` 计数、`codespell`、docs `prettier --check`。
2. **编译与测试门禁**：`cargo check`/`cargo clippy --tests -- -D warnings -p codex-app-server`、
   tui lib 全量（台账口径 `RUST_MIN_STACK=16777216 … -- --skip ide_context::ipc`）、
   `just test -p codex-app-server`、`just fmt-check`。
3. **单提交入库**：44 文件一次提交 + 计划文档单独提交。
4. **推送与遗留确认**：fetch 核对 behind=0 后 `git push origin feat/i18n`；grep 台账
   §12.73 遗留句确认两类遗留登记存在。

### Design Decisions

- **单提交而非按 scope 拆分**：`dict_zh.rs` 的全部新增是单个连续 hunk，按 scope 拆开会让
  任一中间提交的 `i18n-check` 出现 unused/missing（代码与词典不同步），故 44 文件一次入库。
- **tui 测试用台账口径而非 `just test`**：nextest 不支持 `-- --skip ide_context::ipc`
  这类 libtest 过滤，而 `ide_context::ipc` 在本机是环境型挂起，必须跳过。
- **app-server 10 条失败经干净 HEAD 对照实验分类后放行**：临时 worktree（`--detach`）
  跑同批测试得到完全相同的 10 条失败 ⇒ 脏工作区新增失败 = 0；用户裁定「继续收尾，
  失败记遗留」。

## Usage

- 复跑 i18n 门禁：`just i18n-check`；逐 scope 普查
  `python3 scripts/i18n_todo.py --root codex-rs/<crate>/src --top 1`（八 scope：tui / cli /
  core / app-server / core-plugins / codex-mcp / exec / plugin）。
- 复跑测试门禁：`just test -p codex-app-server`；
  `cd codex-rs && RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib -- --skip ide_context::ipc`。
- 执行计划：`docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md`（20 个勾选步骤）。

## Verification

- **i18n**：`i18n-check` EXIT=0（dictionary 4212 / rendered 4202 / missing 0 / unused 0 /
  coverage 99.8%）；八 scope 普查 wrapped 合计 7123、remaining 全 0；tui 与 app-server
  `--dump-rows` 均 `# 0 rows`，`--audit-rows` silent no-op 0。
- **编译**：`cargo check -p codex-app-server --all-targets` 0 错 0 警；
  `cargo clippy --tests -p codex-app-server -- -D warnings` EXIT=0。
- **测试**：tui 定向 4/4 通过；tui 全量 4288 passed / 1 条已知时序 flaky（单跑通过）/
  snapshot 失败 0；app-server 1525 条中 1515 passed、10 failed——经干净 HEAD 对照
  worktree 复跑确认 10 条全部为 HEAD 既有（分类见 Journey Log），本批新增失败 0。
- **格式与拼写**：`just fmt-check` EXIT=0；`codespell` EXIT=0；三份 docs
  `prettier --check` 通过。
- **入库与推送**：`git status --porcelain` = 0；
  `git rev-list --left-right --count origin/feat/i18n...HEAD` = `0 0`；
  push 输出 `6e497323d..448b5cc8e`。
- **遗留登记**：`i18n-verification.md:2731-2732` 记有「平台受限 43 处待平台 CI（清单
  `i18n-platform-ci-checklist.md`，文件存在）」与「`i18n-locale-zh` 是否登记具名门禁待裁决」。

## Journey Log

- [lesson] 站点形态登记的行号漂移只能靠复跑普查暴露（`--audit-rows` 查不出）——
  本批 `:240/:299 → :265/:324` 即此模式，提交前 `dump-rows` 归零是硬门禁。
- [pivot] app-server 测试 10 条失败时没有猜测：用 `git worktree add --detach` 干净 HEAD
  对照复跑同批测试，10 条同样失败 ⇒ 失败与本批无关，避免了误杀或带病放行。
- [lesson] 失败分类：7 条 `command_exec` 是本机沙箱拒绝 fd 创建（环境型）；2 条超长输入
  是已提交批 C（`4cb2d9d02`，已在远端）把 `format!("{MAX_USER_INPUT_TEXT_CHARS}")`
  错写成硬编码 `100000` 的真 bug（应为 `tr_with` + `{0}`，待后续任务修复）；
  1 条 `thread_revert` 为 HEAD 既有。
- [lesson] 计划里预写的 ahead 计数（6）在执行时修正为 7——基准数（origin 领先量）必须
  在写入期望值前重新实测。

## Source Materials

| 文件                                                            | 角色             | 备注                 |
| --------------------------------------------------------------- | ---------------- | -------------------- |
| `docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md` | 执行计划         | 20 步全部执行完毕    |
| `docs/plan/i18n-verification.md` §12.73                         | 完成度权威现状表 | 遗留登记在 §12.73 尾 |
| `docs/plan/i18n-core-plugins-plan.md` §51                       | 批 Q 过程与判据  | 随收尾批提交         |
