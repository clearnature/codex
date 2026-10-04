# i18n 收尾批：工作区提交 + 分支推送 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use compose:subagent (recommended) or compose:execute to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 把工作区已完成的 44 个未提交文件（§51 app-server 批 156→0、tui transcript_export 接 tr、TSV 登记漂移修复、两份计划文档）跑完全量门禁后单提交入库，并推送 `feat/i18n` 上积压的提交，最后确认台账遗留项登记完整。

**Architecture:** 不写任何新代码——这是对既有工作区状态的「验证 → 单提交 → 推送」三段收尾。所有门禁先跑通再暂存；因为 `dict_zh.rs` 的 +344 行是单个连续 hunk（无法按 scope 拆分暂存），且拆开会让任一中间提交的 `i18n-check` 出现 unused/missing，故采用**单提交**（44 文件一次入库）。推送为 fast-forward，不改写历史。

**Tech Stack:** Rust workspace（codex-rs）、`codex-i18n-check`、`scripts/i18n_todo.py`、just、cargo/nextest、prettier、codespell、git。

**无独立 spec**——范围来自 2026-10-04 用户在 Question 工具中的选择「全收尾（推荐）：验证并提交 §51 未提交批次 + 推送 5 个提交 + 登记平台 CI 遗留项」。背景事实（本回合已实测复核）：断机任务 `i18n-completion-record-refresh` 已收尾（spec `status: delivered`，提交 `867d16921` + `8e2a73705` 已入库，stash 为空）；八 scope 普查 remaining 全 0；`just i18n-check` EXIT=0。

## Global Constraints

- 工作目录固定为当前 `feat/i18n` 检出（`/data/training/cli/codex`），**不建 worktree**；只有任务相关的 44 个已修改文件可入库，不新建/删除任何源码文件。
- **禁止** force push、改写历史、`git commit --amend` 已有提交；推送只允许 fast-forward。
- 本计划**不产生**新翻译、新裁定、新代码改动；唯一允许的代码字节变化只能来自 `just fmt` 的格式化（且触发时必须复核 TSV 登记，见 Task 1 Step 6 的复核链）。
- 任何改动 Rust 行号的操作之后，必须复跑 `i18n_todo --dump-rows` 与八 scope 普查——站点形态登记按精确 `path:line` 包配，静默失效是已知事故模式（§12.73、§51.2）。
- 测试命令遵循仓库 AGENTS.md：包级测试用 `just test -p <pkg>`；tui 全量 lib 测试用台账既有口径（含 `--skip ide_context::ipc` 环境型跳过，nextest 不支持该过滤，故此条不用 `just test`）。
- 提交信息用中文、按仓库现有 `i18n(...): … —— …` 风格，正文列门禁证据。
- 已知的非阻断事实（不要当失败处理）：tui 全量 lib 测试有 2 条并发时序型 flaky（单独重跑即过）；`ide_context::ipc` 10 条为环境型、必须 skip；coverage 99.8% 的差 9 条是 `not-translated.tsv` 声明不译键。

---

### Task 1: i18n 一致性门禁（提交前证据）

**Covers:** 无 spec 节（纯验证任务）——对应用户选择的「验证 §51 未提交批次」。

**Files:**

- 只读验证，不修改任何文件（若 Step 6 触发 prettier 修复，仅允许改 `docs/plan/i18n-core-plugins-plan.md` / `docs/plan/i18n-glossary.md` / 本计划文件的空白格式）。

**Interfaces:**

- Consumes: 工作区现有 44 个已修改文件（§51 批次 + transcript_export + TSV + 两份 docs）。
- Produces: 全绿的 i18n 门禁证据，供 Task 3 提交信息引用；若任何一步失败，先按各步的「失败处理」修复再继续，**不得带病进入 Task 3**。

- [ ] **Step 1: 基线状态确认**

Run:

```bash
git branch --show-current          # 期望: feat/i18n
git stash list                     # 期望: 空（断机任务的 stash@{0} 已清空）
git status --porcelain | wc -l     # 期望: 44
git status --porcelain | grep -c '^??' || true   # 期望: 0（无未跟踪文件混入）
```

Expected: 四项全部符合；若 stash 非空或出现未跟踪文件，**停下报告**，不要继续。

- [ ] **Step 2: `just i18n-check` 漂移门禁**

Run: `just i18n-check`

Expected（尾部关键行 + 退出码）:

```text
dictionary        : …/codex-rs/i18n/src/dict_zh.rs (4212 entries)
rendered keys     : 4202
[missing] rendered but not in the dictionary: 0
[unused] in the dictionary but rendered nowhere: 0
[coverage] translated 4193/4202 rendered keys (99.8%)
[spacing] … 0   [nested] … 0   [duplicate] … 0   [placeholder] … 0   [asset] … 0
EXIT=0
```

失败处理: 任何 missing/unused 非 0 说明提交内容自相矛盾（代码与词典不同步）——对照 `git diff codex-rs/i18n/src/dict_zh.rs` 定位缺哪侧，**不要**改注册表来凑数；报告后中止。

- [ ] **Step 3: 八 scope 普查归零**

Run:

```bash
for c in tui cli core app-server core-plugins codex-mcp exec plugin; do
  echo "== $c"
  python3 scripts/i18n_todo.py --root codex-rs/$c/src --top 1 | tail -5
done
```

Expected: 每个 scope 打印 `wrapped so far : N` 且 N 依次为 3229 / 1281 / 1164 / 708 / 531 / 119 / 75 / 16（合计 7123），其后的 `module  remaining` 表**为空**（无任何剩余候选行）。

失败处理: 某 scope 出现剩余候选 ⇒ 多半是行号漂移（fmt/编辑使站点形态登记失效）。用该候选的 `path:line` 对照 `codex-rs/i18n/not-translated-unwrapped.tsv` 找到失效行并修正行号（参照 §12.73 的 `:240→:265` 修法），修后重跑本步直到全零，再重跑 Step 4/5。

- [ ] **Step 4: 登记复核（dump-rows / audit-rows）**

Run:

```bash
python3 scripts/i18n_todo.py --root codex-rs/tui/src --dump-rows
python3 scripts/i18n_todo.py --root codex-rs/app-server/src --dump-rows
python3 scripts/i18n_todo.py --root codex-rs/tui/src --audit-rows
python3 scripts/i18n_todo.py --root codex-rs/app-server/src --audit-rows
```

Expected: 四条命令**均无输出**（0 个待复核站点、0 个 silent no-op），退出码 0。

失败处理: dump-rows 吐出站点 ⇒ 该登记已漂移，按 Step 3 失败处理修行号后重跑；audit-rows 报行 ⇒ 该行指向的不再是候选，人工核对该登记是否已多余（已译则删行），改完回到 Step 3。

- [ ] **Step 5: 快照与拼写**

Run:

```bash
find codex-rs -name '*.snap.new' | wc -l    # 期望: 0
find codex-rs -name '*.snap' | wc -l         # 期望: 940
codespell                                   # 期望: 无输出, EXIT=0
```

Expected: 快照零待审；codespell 干净（配置读取仓库根 `.codespellrc`）。

失败处理: 出现 `.snap.new` 说明有人跑测试生成了待审快照——逐个 `git diff` 查看；本任务不应改 En 输出，任何快照差异都必须先弄清来源再报告，**禁止** `cargo insta accept`。

- [ ] **Step 6: 计划文档 prettier 格式**

Run:

```bash
npx prettier --check docs/plan/i18n-core-plugins-plan.md docs/plan/i18n-glossary.md docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md
```

Expected: PASS（exit 0，无文件被点名）。

失败处理: 仅对被点名的文件运行 `npx prettier --write <该文件>`，然后**回到 Step 3** 重跑普查与 Step 4——prettier 只改 Markdown，不会动 Rust 行号，但要确认它没顺手改到别的文件（`git status --porcelain | wc -l` 仍为 44；若本计划文件是新增的则为 45，见 Task 3 Step 1）。

---

### Task 2: 编译与测试门禁

**Covers:** 无 spec 节（纯验证任务）。

**Files:**

- 只读验证，不修改任何文件（clippy 禁止带 `--fix`；格式化只允许 Task 1 失败处理中声明的 docs prettier）。

**Interfaces:**

- Consumes: Task 1 全绿。
- Produces: 编译/测试全绿证据，供 Task 3 提交信息引用；任何一条不过都**不得进入 Task 3**。

- [ ] **Step 1: app-server 类型检查**

Run:

```bash
cd codex-rs && cargo check -p codex-app-server --all-targets
```

Expected: `Finished` 成功，**0 errors / 0 warnings**（§51.5 口径；出现任何 warning 即失败）。

失败处理: 报错说明 §51 批次与 HEAD 之后的合并状态不一致——贴出完整报错并中止，不要自行改代码。

- [ ] **Step 2: app-server clippy**

Run:

```bash
cd codex-rs && cargo clippy --tests -p codex-app-server -- -D warnings
```

Expected: `Finished` 成功，无 warning（`-D warnings` 使其等价于 CI 门禁）。

失败处理: 若告警是 §51.5 已清过的同类（unused import / dead code / useless_format），允许用 `just fix -p codex-app-server` 自动修复——修复后 Rust 行号可能变化 ⇒ **回到 Task 1 Step 3** 重跑普查+登记复核，再回到本步。

- [ ] **Step 3: tui 定向测试（transcript_export 相关）**

Run:

```bash
cd codex-rs && RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib transcript_export
```

Expected: 全部通过（`transcript_export_tests` 与 `chatwidget::transcript_export` 匹配项均 pass，0 failed）。

失败处理: 若断言的是被改动行的**英文**输出——En 下 `tr` 原样返回 key，理论上逐字节不变；任何失败都可能是真实回归，贴出断言差异并中止，不要改测试凑过。

- [ ] **Step 4: tui 全量 lib 测试（台账口径）**

Run:

```bash
cd codex-rs && RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib -- --skip ide_context::ipc
```

Expected: 约 **4285 passed**、**0 snapshot 类失败**；允许最多 2 条已知并发时序型 flaky（台账记载的已知集合）。

失败处理:

- flaky 判据: 单独重跑该测试即通过 ⇒ 记录测试名，视为通过（`cargo test -p codex-tui --lib <单测名>`）。
- snapshot 类失败 ⇒ **硬失败**：本任务不该改 En 渲染；查看 `.snap.new` 差异，报告后中止。
- 其他确定性失败 ⇒ 报告完整输出，中止，不改测试不改代码。

- [ ] **Step 5: app-server 测试**

Run: `just test -p codex-app-server`（nextest，`RUST_MIN_STACK` 由 justfile 注入，`--no-fail-fast`）。

Expected: 全部通过。此步是 §51 之后首次跑该套件，**是本计划最大不确定点**。

失败处理: 逐条读失败——(a) 若是 locale 断言/错误串断言与新 `tr` 包装相关，属于真实回归：报告失败详情并中止（不在本计划内修码）；(b) 若是环境型/超时型，单独重跑 `just test -p codex-app-server <单测过滤>` 确认；(c) 禁止通过改断言、改注册表、跳过测试来过关。

- [ ] **Step 6: 格式门禁**

Run:

```bash
just fmt-check        # 期望: EXIT=0, 无文件被点名
```

失败处理: 若被点名的是本任务 44 文件内的 Rust 文件 ⇒ `just fmt` 后 Rust 行号已变，**必须**回到 Task 1 Step 3 重跑普查 + Step 4 登记复核 + Step 2 的 i18n-check，然后回到本步复核；若是计划 Markdown ⇒ `npx prettier --write <文件>` 后复核本步。

---

### Task 3: 单提交入库（44 文件）

**Covers:** 无 spec 节——对应用户选择的「验证并提交 §51 未提交批次」。

**Files:**

- Modify（入库，不改动内容）: `codex-rs/app-server/src/**`（39 个文件）、`codex-rs/i18n/src/dict_zh.rs`、`codex-rs/i18n/not-translated-unwrapped.tsv`、`codex-rs/tui/src/app/transcript_export.rs`、`docs/plan/i18n-core-plugins-plan.md`、`docs/plan/i18n-glossary.md`——合计恰 44 个已修改文件。
- Create（本计划文档，随 Task 1 Step 6 的格式检查）: `docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md`（45 号文件，单独提交）。

**Interfaces:**

- Consumes: Task 1 + Task 2 全绿。
- Produces: 两个新提交（① 44 文件收尾批；② 本计划文档），`git status --porcelain` 归零，为 Task 4 推送提供 6 个待推提交。

- [ ] **Step 1: 确认暂存范围恰好是 44 个任务文件**

Run:

```bash
git status --porcelain | grep -v '^ M' || true   # 期望: 只剩本计划文件 `?? docs/compose/plans/…`
git add codex-rs/app-server codex-rs/i18n codex-rs/tui/src/app/transcript_export.rs \
        docs/plan/i18n-core-plugins-plan.md docs/plan/i18n-glossary.md
git diff --cached --stat | tail -1               # 期望: 44 files changed, …
```

Expected: `--stat` 汇总行是 **44 files changed**（插入/删除行数与 `git diff --stat` 基线 `+1124/-258` 一致，±fmt/prettier 允许微差）。

失败处理: 数目不是 44 ⇒ `git reset` 后逐路径核对 `git status --porcelain`，找出多暂存/漏暂存的文件，修正后重跑。

- [ ] **Step 2: 提交 44 文件收尾批**

Run:

```bash
git commit -m "$(cat <<'EOF'
i18n: 收尾批 —— app-server 批 Q 156→0 + tui 导出活动行接 tr + 登记漂移修复（八 scope remaining 0）

- app-server：39 文件，判决 127 译 / 29 登记（批次过程与三条判据教训见
  docs/plan/i18n-core-plugins-plan.md §51），i18n_todo 156 → 0
- tui：transcript_export.rs 活动行/资源占位接 tr/tr_with；
  not-translated-unwrapped.tsv 哨兵站点 :240/:299 → :265/:324 漂移修复
  （i18n-verification.md §12.73），导出正文已登记行随站点改包同步移除
- docs：core-plugins 计划追加 §51 批记；glossary 术语表按 prettier 重排
- 门禁（提交前实跑）：just i18n-check EXIT=0（4212 词条，missing·unused·
  spacing·nested·duplicate·placeholder·asset 全 0，coverage 99.8%）；
  八 scope i18n_todo remaining 全 0（7123/7123）；dump-rows/audit-rows 无输出；
  cargo check/clippy -D warnings -p codex-app-server 0 告警；
  tui lib（台账口径）与 app-server nextest 通过；.snap.new=0；fmt-check/codespell 通过
EOF
)"
```

Expected: 提交成功，`git log -1 --stat` 显示 44 files changed。

- [ ] **Step 3: 提交本计划文档**

Run:

```bash
git add docs/compose/plans/2026-10-04-i18n-batch51-commit-and-push.md
git commit -m "docs(compose): 收尾批执行计划（i18n-batch51-commit-and-push，验证→单提交→推送）"
git status --porcelain | wc -l    # 期望: 0
git log --oneline -3              # 期望: 计划提交 → 收尾批 → 8e2a73705
```

Expected: 工作区完全干净（0 个改动/未跟踪文件）。

失败处理: 若 status 仍非 0，逐条 `git status --porcelain` 核对是否本任务范围外的文件（应停下报告，不提交、不清理）。

---

### Task 4: 推送 feat/i18n + 遗留项登记确认

**Covers:** 无 spec 节——对应用户选择的「推送 5 个提交 + 登记平台 CI 遗留项」。

**Files:**

- 只读：`docs/plan/i18n-verification.md`、`docs/plan/i18n-platform-ci-checklist.md`（确认遗留登记存在，不编辑）。
- 远端：`origin/feat/i18n` 仅 fast-forward。

**Interfaces:**

- Consumes: Task 3 的 2 个新提交（本地领先 origin/feat/i18n 共 7 个提交：`791f7bba0`、`1326c04d0`、`ff2fd98c7`、`867d16921`、`8e2a73705` + 新收尾批 + 计划文档）。
- Produces: 远端与本地一致（ahead/behind = 0/0）；收尾报告中列明仅剩的两类遗留（平台 CI、i18n-locale-zh 裁定）。

- [ ] **Step 1: 推送前核对（只允许 fast-forward）**

Run:

```bash
git fetch origin feat/i18n
git rev-list --left-right --count origin/feat/i18n...HEAD   # 期望: 0<TAB>7（behind=0, ahead=7）
```

Expected: `behind` 为 0——远端没有任何本地没有的提交。

失败处理: behind > 0 ⇒ 远端有新提交（可能他人推过）：**停下报告**，禁止 merge/rebase/force，等待指示。

- [ ] **Step 2: 推送**

Run: `git push origin feat/i18n`

Expected: fast-forward 推送成功，无 rejected/non-fast-forward。

失败处理: 若被拒绝（权限/钩子/远端变更），原样贴出报错并**停下报告**；不追加 `--force`、不跳过钩子。

- [ ] **Step 3: 推送后核对**

Run:

```bash
git rev-list --left-right --count origin/feat/i18n...HEAD   # 期望: 0<TAB>0
```

Expected: 本地与远端完全一致。

- [ ] **Step 4: 遗留项登记确认（只读）**

Run:

```bash
grep -n "平台受限 43 处待平台 CI" docs/plan/i18n-verification.md
grep -n "i18n-locale-zh" docs/plan/i18n-verification.md
test -f docs/plan/i18n-platform-ci-checklist.md && echo checklist-exists
```

Expected: 前两条各命中 §12.73「遗留」句（平台 CI 43 处 + i18n-locale-zh 门禁待裁决），第三行输出 `checklist-exists`。

失败处理: 任一 grep 落空 ⇒ 台账遗留登记缺失，这是文档缺口：在 `i18n-verification.md` 的 §12.73 遗留句中补回缺失项（保持「平台受限 43 处待平台 CI（清单 `i18n-platform-ci-checklist.md`）；`i18n-locale-zh` 是否登记具名门禁待裁决」的既有措辞），补完后 `npx prettier --check docs/plan/i18n-verification.md` 必须通过，并追加一个 docs 提交：`docs(i18n): 补记 §12.73 遗留项（平台 CI 43 处 / i18n-locale-zh 裁定）`，然后回到 Step 2 把该提交一并推送。

- [ ] **Step 5: 收尾报告**

向用户输出最终状态（中文，简短）：

- 提交: 收尾批 44 文件 + 计划文档，共 2 个新提交；
- 推送: `origin/feat/i18n` 与本地一致（6 个提交已上远端）;
- 完成度: 八 scope 7123/7123 = 100%、i18n-check EXIT=0（4212 词条 / coverage 99.8%）、`.snap.new` 0；
- 仅剩遗留（外部依赖，不在本计划内）: 平台受限 43 处等 macOS/Windows CI（清单 `i18n-platform-ci-checklist.md`）；`i18n-locale-zh` 具名门禁待人类裁决。

---

## Self-Review 记录

1. **范围覆盖**: 用户选择「全收尾」的三要素——验证/提交（Task 1–3）、推送（Task 4 Step 1–3）、遗留登记确认（Task 4 Step 4–5）——各有对应任务；无 spec 节可对档（本计划无 spec，范围声明见头部）。✔
2. **占位符扫描**: 无 TBD/TODO/「类似 Task N」；所有命令给出完整可执行形态与期望输出；失败处理均为具体判据与具体命令。✔
3. **类型/名称一致性**: `just i18n-check`、`scripts/i18n_todo.py`、`just test -p …`、`just fmt-check` 在各任务中名称一致；提交信息引用的数字（4212/4202/99.8%/7123/44 文件）与本回合实测一致；Task 3 的 44/45 文件计数与 Task 1 Step 6、Task 4 ahead=6 的推演一致。✔
