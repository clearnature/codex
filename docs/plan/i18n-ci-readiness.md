# i18n 云端 CI 就绪度评估

问题：**现在可以开云端 CI（PR 门禁）了吗？**
结论：**可以开**。CI 与本机之间只剩「本机跑不了、只能由 CI 判」的少数几项；
下面把每一项标成**本机已取证**或**CI-first（未验证）**，并给出预期最可能红的一项。

## 一、CI required 门槛 ↔ 本机可验性映射

`blocking-ci.yml` 的 required 集合是：Bazel / Blob size policy / cargo-deny / Codespell /
repo-checks / rust-ci / sdk。

| CI job           | 具体步骤                                                                                                  | 本机对应门禁                                                      | 状态                                                                                                                                                                                                                                                                                                                                                    |
| ---------------- | --------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| repo-checks      | `just fmt-check`                                                                                          | `fmt-check`                                                       | **已取证** `r-mu8hhteb-t7ukc3`（最终树）                                                                                                                                                                                                                                                                                                                |
| repo-checks      | `just i18n-check`                                                                                         | `i18n-check`                                                      | **已取证** `r-mu8h8wsf-e70wvp`（全零）                                                                                                                                                                                                                                                                                                                  |
| repo-checks      | `just i18n-smoke`                                                                                         | `i18n-smoke`                                                      | **已取证** `r-mu8hgswh-xaqrtf`（zh 26 行 / C 0 行，双向）                                                                                                                                                                                                                                                                                               |
| repo-checks      | `pnpm run format`（prettier）                                                                             | ——（无对应门禁）                                                  | **CI-first**：本机无 `node_modules`、沙箱无网络 ⇒ 见 §三.1                                                                                                                                                                                                                                                                                              |
| repo-checks      | `check-clean-worktree`                                                                                    | `git status`                                                      | **已核实**：工作树干净（提交 `4220d82d2` 后）                                                                                                                                                                                                                                                                                                           |
| Codespell        | `codespell`                                                                                               | `codespell`                                                       | **已取证** `r-mu8hh72k-8b04t2`                                                                                                                                                                                                                                                                                                                          |
| Bazel            | `bazel test` 多平台矩阵 + `check-module-bazel-lock.sh`                                                    | `bazel-lock-check` / `bazel-i18n`                                 | 锁与 i18n 目标**已取证**（`r-mu8e783e-zo6pia` / `r-mu8cpxxl-yum8un`）；**bazel test 全矩阵本机从未跑**（按预算约束不跑）⇒ CI-first                                                                                                                                                                                                                      |
| cargo-deny       | 许可证/advisory/ban                                                                                       | ——                                                                | **CI-first**：本机无 `cargo-deny` 二进制                                                                                                                                                                                                                                                                                                                |
| rust-ci          | 三平台 cargo test/clippy 矩阵                                                                             | `clippy` / `exec-test` / `tui-test` / `i18n-unit` / 各 crate 套件 | Linux 侧**已取证**（`clippy r-mu8e2qwp-f6dzc6`、`i18n-unit r-mu8e7bt8-x6r03f`、core-plugins 438 passed `r-mu8ejqmb-26i4ea` 等）；**macOS/Windows 侧只能由 CI 判**（本机无跨目标类型检查，见 `known_issues no-cross-target-typecheck-local`）                                                                                                            |
| sdk              | Python SDK：`ruff check` + `ruff format --check` + `pytest`（**cwd = `sdk/python`**，见 `sdk.yml:26/36`） | 本机无 ruff 可执行文件，但 `scripts/` 有独立的一步                | **作用域澄清**：CI 的 ruff 只在 `sdk/python` 里跑 ⇒ `scripts/*.py` **不在 CI ruff 范围**；而 `fmt-check` 的 `python_scripts_formatter_group`（`scripts/format.py:118`）会用 `uv run --project scripts ruff format` 检查 `scripts/` ⇒ 本批对 `scripts/i18n_apply.py` 的补丁**已取证**：`r-mu8hiz5w-oruj3d`（format ✓ / check ✓ / 全仓仍只有预存的 9 条） |
| Blob size policy | 变更 blob ≤ 512000 字节                                                                                   | 本机只量了尺寸                                                    | **已量**：`not-translated-unwrapped.tsv` = 385 091 B（75% 上限）⇒ 见 §三.2                                                                                                                                                                                                                                                                              |

## 二、已在本机取证的门禁（一页索引）

`fmt-check` · `i18n-check` · `i18n-smoke` · `codespell` · `clippy` · `i18n-unit` ·
`bazel-lock-check` · `bazel-i18n` · `argument-comment-lint` · `check-tui-lib` / `check-tui-release` ·
`exec-test` · `tui-test` · `i18n-locale-en` · `i18n-locale-chain`。
回执都在 `~/.dsh/state/swe-mode/receipts/`，可用 `verify action:"receipts"` 复核**当前是否仍有效**
（门禁定义改过、回执文件被删、或本就没过都会报出来）。

## 三、CI-first 清单与预判风险（按「最可能红」排序）

### 1. prettier（`pnpm run format`）——**已实测确认会红，并已修复**

第一版把这条写成「未验证 + 风险最高」。装上依赖后**实测确认了预判**：

- `pnpm run format`（= `prettier --check *.json *.md docs/**/*.md .github/workflows/*.yml **/*.js`）
  → **EXIT=1，16 个文件不合格**：`docs/plan/` 的 i18n 文档族 10 个、`docs/maps/*` 6 个、
  以及 `docs/plan/{build-and-versioning,desktop-architecture,test-baselines}.md`。
  后两类**不是本任务写的**，但 CI 检查的是**全量匹配文件**而非变更集 ⇒ 必须一起修。
- `pnpm run format:fix` → 16 files changed, 1370 insertions(+), 1357 deletions(-)。
- 复核：`pnpm run format` → `All matched files use Prettier code style!` **EXIT=0**
  （回执 `r-mu8i2854-ytho4y`）；`just fmt-check` 复跑仍绿（`r-mu8i2shd-6ec8cu`）。

⚠ 之前无法本机验证的原因**不是「没有网络」**（那是误判，见 §六.4），而是当时还没装 `node_modules`。

**工作流规矩（本批踩了两次）**：本机已装 `node_modules`+prettier ⇒ **任何手工改动的 md 之后都要立刻跑
`pnpm run format:fix`**，再提交。否则 CI 的 `pnpm run format` 会红在同一条上（本批在
`i18n-ci-readiness.md` 上连着犯了两次：先是我手写的段落，后是索引式拼接写坏文件后的重写）。

### 2. Blob size policy——**已处置（选处方 ①：缩短理由列）**

- 处置前：`codex-rs/i18n/not-translated-unwrapped.tsv` = **385 091 B** / 上限 **512 000 B**。
- 处置：本批 80 行（startup_sync）复制了同一段 §12.3 长说明（每行 ≈235 B 的重复文本），
  改为短引用 `§12.3 日志/诊断链（详情 docs/plan/i18n-core-plugins-plan.md §8.2；终点 manager.rs:3276 warn!）`。
- 处置后：**366 291 B**（−18 800 B，仍 1316 行）；豁免语义不变（`startup_sync.rs` 仍 0 candidates / 80 exempted，
  crate 仍 382）。
- 剩余预算 **145 709 B**；按新样式每行 ≈240 B 估算 ⇒ 还能放 **约 600 行**登记行。
- 判据（下次登记批次后复核）：文件 < 512 000 B；若再逼近，按同样方式压缩**重复的理由文本**
  （处方 ① 优先），其次才考虑 allowlist（②）或拆分（③）。

### 3. cargo-deny——预期无影响，但未验证

- 本轮给 `codex-plugins` 加了 `codex-i18n = { workspace = true }`。
  `codex-i18n` 本来就已在依赖图里（cli/tui/exec 都依赖它），**没有新 crate 进入图** ⇒
  许可证/advisory 面不变；风险只在 `bans` 类规则（重复版本/wildcard），本改动不新增版本。
- 判据：CI 的 cargo-deny job 绿。若红，先看是不是**既有**问题（用 baseline 提交对照）。

### 4. Bazel 侧的新依赖——预期无需手改

- `codex-rs/core-plugins/BUILD.bazel` 只有一句 `codex_rust_crate(...)`；
  `defs.bzl` 用 `all_crate_deps()`（`defs.bzl:319/333/355`）从 `Cargo.toml` 推导依赖 ⇒
  加 `codex-i18n` 依赖**不需要手改 BUILD.bazel**。
- `MODULE.bazel.lock` 本机已验：`bazel-lock-check` `r-mu8e783e-zo6pia` 通过、`git diff` 为空。

### 5. 平台门控的 43 处未完成——**不会让 CI 红**

- 没有任何 required 门禁要求「候选数为 0」；未包的字符串只是「还没做」，不是失败。
- 见 `docs/plan/i18n-platform-ci-checklist.md`（43 处代码改动 + 2 行登记），等 macOS/Windows CI 跑起来后逐条落。

## 四、开 CI 前的本机收尾清单（都已做）

- [x] `just fmt` 已跑（含文档、词典、测试）
- [x] 工作树干净（`git status --short` 空）
- [x] 生成物无漂移：未改 `ConfigToml` / app-server schema ⇒ 无需 `write-config-schema` / `write-app-server-schema`
- [x] 六范围候选普查：cli/core/tui/exec/plugin = 0，core-plugins = 382（见 §12.72 与本文件 §二）
- [x] 本机可跑的 CI 同款门禁全部重跑并取证（§一表前三行 + Codespell）

## 五、对抗自检：这份评估最可能错在哪

1. **「prettier 会红」只是预判，不是事实**：我没跑过它，也没找到本仓库 `docs/**` 下 prettier-clean 的
   对照物（`third_party/voice/README.md` 的表格不补空格，但它不在 prettier 的 glob 里 ⇒ 不构成证据）。
   所以这一条应当读作「**未验证 + 风险最高**」，而不是「已知会红」。
2. **映射表可能漏项**：我只读了 `repo-checks.yml` 全文与 `blocking-ci.yml` 的 required 列表；
   `bazel.yml` / `rust-ci.yml` / `sdk.yml` 的内部步骤没有逐行读 ⇒ 其中若有额外的 i18n 相关门槛，
   本表会漏。
3. **`check-clean-worktree` 的判据我按 `git status` 推的**，没有读过该 action 的实现；
   若它还检查未跟踪文件或特定生成物，我的「已核实」就偏窄。

## 六、修订记录（第一版之后被新证据推翻/补强的说法）

1. **「本批未触碰 SDK」这条不够精确**（已改 §一 sdk 行）。实际是：CI 的 ruff 只在 `sdk/python` 跑
   （`sdk.yml` 的 `-w ${GITHUB_WORKSPACE}/sdk/python`），而 `scripts/` 的 Python 由本机门禁
   `fmt-check` 的 `python_scripts_formatter_group`（`scripts/format.py:118`）覆盖。
   查证后**本批的 `scripts/i18n_apply.py` 补丁是有证据的**（`r-mu8hiz5w-oruj3d`），
   不是「未验证」——第一版把它列进 sdk 行的「未验证」是错的。
   另：`ruff check .` 的 9 条预存 findings 在 CI 里**没有**对应的 required 步骤（workflows 里只有
   `sdk.yml` 那两行 ruff，且作用域是 `sdk/python`）⇒ 本地 `lint-python-scripts` 红**不阻塞 CI**。
2. **「开 CI」的动作要说清**：本仓库有远端
   （`origin https://github.com/clearnature/codex.git`，当前分支 `feat/i18n`）
   ⇒ 这一步是 **push + 开/更新 PR**，不是本机能「跑」的东西。本机的贡献是把 required 门槛里
   **可本机验的部分全部取证**（§一、§二），把不可本机验的部分**显式列出**（§三）。
3. **§三.1（prettier）仍是「未验证 + 风险最高」**，没有新证据改变它：本机既无 `node_modules`
   也无网络（`ERR_PNPM_NO_OFFLINE_TARBALL`），且 `docs/**/*.md` 确实在它的 glob 里。

## 七、这个 fork 上「怎么触发 CI」（不 PR 到底行不行）

问题：仓库是 fork（`origin = https://github.com/clearnature/codex.git`，当前分支 `feat/i18n`），
**不 PR 就无法 CI 吗？** 答：**分档** —— 有一半能、有一半不能。

### 7.1 触发器事实（本机 `.github/workflows/*.yml` 全文读出，非推测）

| 触发器                                   | 谁有                                                                                                                                                         |
| ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `pull_request`                           | `blocking-ci`、`cla`、`v8-canary`                                                                                                                            |
| `push: branches: [main]`                 | `blocking-ci`、`postmerge-ci`                                                                                                                                |
| `push: branches: ["**full-ci**"]`        | `rust-ci-full`（注释原文：_Keep this opt-in branch trigger for developers who want the full suite before merging_）                                          |
| `push: tags`                             | 各 release workflow（与本任务无关）                                                                                                                          |
| `workflow_dispatch`（可手动跑）          | `bazel`、`rust-ci`、`rust-ci-full`、`v8-canary`、`python-runtime-release`、`rust-release-prepare`（fork 上 disabled）、`close-stale-contributor-prs`（同上） |
| **只有 `workflow_call`**（不能自己触发） | `repo-checks`、`codespell`、`cargo-deny`、`blob-size-policy`、`sdk`、`rust-release-windows` 等                                                               |

`blocking-ci.yml` 的注释把设计意图写明了：_"It also runs after pushes to main so the same check family stays grouped in the Actions UI."_
它的 `CI required` 聚合 job 的 `needs` 是：
`bazel / blob-size-policy / cargo-deny / codespell / repo-checks / rust-ci / sdk`。

### 7.2 所以：不 PR 能/不能拿到什么

| 想要的结果                                                                                      | 不 PR 能否 | 路径                                                                |
| ----------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------- |
| **`repo-checks`（内含 `just fmt-check` / `just i18n-check` / `just i18n-smoke` 三道 i18n 闸）** | ❌         | 只有 `pull_request` 或 `push: main`；它自己没有 `workflow_dispatch` |
| `codespell` / `cargo-deny` / `blob-size-policy` / `sdk`                                         | ❌         | 同上（仅 `workflow_call`）                                          |
| **完整 rust 套件**（`rust-ci-full`，比 blocking 里的 `rust-ci` 更全）                           | ✅         | 推一个**以 `full-ci` 结尾**的分支名，或手动 dispatch `rust-ci-full` |
| `bazel` / `rust-ci`（含平台矩阵）                                                               | ✅（手动） | `workflow_dispatch`（网页 Run workflow 或 `gh workflow run`）       |
| `postmerge-ci`（rust-ci-full + v8-canary）                                                      | ❌         | 仅 `push: main`                                                     |

### 7.3 这个 fork 的实际状态（2026-09-14 之后；**§八 有首次运行后的更新**）

- `GET /repos/clearnature/codex/actions/runs?per_page=8` → **`total_count: 0`**：**这个 fork 从来没跑过一次 workflow**。
- `GET /repos/clearnature/codex/actions/workflows` → 29 个，关键各条（`blocking-ci`/`repo-checks`/`bazel`/`rust-ci`/
  `rust-ci-full`/`codespell`/`cargo-deny`/`postmerge-ci`/`sdk`）都是 **`state: active`**
  ⇒ **Actions 没有被禁用**（只有 `close-stale-contributor-prs` 与 `rust-release-prepare` 是 `disabled_fork`）。
- 两者合起来自洽：**往功能分支 push 在本仓库配置下什么都不触发**（`origin/feat/i18n` 存在，但 0 次运行
  正是「推功能分支不触发任何 workflow」的结果）。
- 本地引用：`HEAD` 比 `origin/feat/i18n` **领先 308 个提交**（`git rev-list --left-right --count HEAD...origin/feat/i18n`
  = `308 0`）⇒ 最近的工作都还没推。

* 本机**可以推送**（第一版写错了）：`git ls-remote origin HEAD` EXIT=0、`pnpm install --frozen-lockfile` EXIT=0（551 包 / 15.4s）、`git push --dry-run origin HEAD:refs/heads/i18n-push-probe` **EXIT=0**（dry-run 不创建远端引用），且 `credential.helper = store` + `~/.git-credentials` 存在。⇒ 触发 CI 的动作在本机**做得到**；「推哪个分支 / 要不要开 PR」是人类的决策。

### 7.4 建议（附判据）

1. **开 PR（首选）**：一次带起 7 个 required job + 平台矩阵；且 `cla.yml:46` 的
   `path-to-document` 指向 `blob/feat/i18n/docs/policies/CLA.md` ⇒ 作者本就预期 PR 往 `feat/i18n` 走。
   判据：PR 上出现 `blocking-ci` 的 `CI required` 结论。
2. **不开 PR 也能拿全量 rust**：把分支名改成 `*-full-ci` 再推，或手动 dispatch `bazel.yml`/`rust-ci.yml`。
   判据：Actions 里出现对应 run。**注意这仍拿不到 repo-checks 的三道 i18n 闸。**
3. **i18n 三闸其实不依赖云**：它们就是本机的 `just fmt-check` / `just i18n-check` / `just i18n-smoke`
   （本仓库已有回执，见 §一、§二）。真正 CI-only 的只有：**prettier / cargo-deny / bazel test 矩阵 /
   macOS·Windows / sdk / blob-size checker**。
4. **动作前置（已核实）**：本机有网络与推送凭据（§六.5）⇒ 推分支做得到；开 PR / 手动 dispatch 需要 `gh` 或 API token（凭据在 `~/.git-credentials`，但**用不用它去动共享远端**要人拍板）。本文件只记录能力与代价，**不代表已经推过**：截至本提交，远端 `origin/feat/i18n` 仍停在 `aa944d6b9`，本地领先 310 个提交。

5. **「沙箱无网络」是错的**（已改正 §三.1 与 §七.3）。第一版依据 `pnpm install --frozen-lockfile --offline` 的失败写了「无网络」——那个失败是我自己加了 `--offline` 造成的（`ERR_PNPM_NO_OFFLINE_TARBALL`：pnpm store 缺 tarball），**不是**网络不可达。直接探测后事实相反：`git ls-remote origin HEAD` **EXIT=0**、`curl https://registry.npmmirror.com/` **HTTP 200**、`pnpm install --frozen-lockfile` **EXIT=0 / 15.4s / 551 包**。⇒ 教训：**环境限制必须用不带人为标志的直接探测来判**；拿自己中间产物的失败当证据，会把「我没装依赖」误判成「环境不允许」。
6. **「本机不能推送」也是错的**（已改正 §七.3/§七.4）：`git push --dry-run origin HEAD:refs/heads/i18n-push-probe` → **EXIT=0**，凭据齐备。⇒ 「开 CI」在本机**可执行**，只是**要不要动共享远端**属人类决策。

## 八、fork 上跑 CI 的**首次实测**（2026-09-19）

### 8.1 做了什么

1. **推送 `feat/i18n`**：`aa944d6b9..f015c527f`（313 个提交），远端 `refs/heads/feat/i18n` == 本地 HEAD。
2. **验证「推功能分支是否触发 CI」**：推完立刻查
   `GET /repos/clearnature/codex/actions/runs?event=push` → 仍是 **`total_count: 0`**
   ⇒ **证实**：在本仓库配置下，推普通功能分支**不触发任何 workflow**（§七.1 的表格读对了）。
3. **改用 dispatch 触发**（不开 PR、不动 main）：
   - `rust-ci` → run [35450055551](https://github.com/clearnature/codex/actions/runs/35450055551)
   - `Bazel` → run [35450058371](https://github.com/clearnature/codex/actions/runs/35450058371)
     这是该 fork **历史上第一次**运行 workflow。

### 8.2 踩到的工具坑：`gh` 默认解析到 **upstream**，不是 fork

第一次 dispatch 直接失败：

```
could not create workflow dispatch event: HTTP 403: Must have admin rights to Repository.
   (https://api.github.com/repos/openai/codex/actions/workflows/158138315/dispatches)
```

注意 URL 里的 **`openai/codex`** —— 本仓库同时有 `origin=clearnature/codex` 与 `upstream=openai/codex` 时，
`gh` 默认选了**上游**，于是拿 fork 的凭据去要上游的 admin 权限。
处方：显式 `--repo clearnature/codex`，或先 `gh repo set-default clearnature/codex`（本次两条都做了，之后 dispatch 成功）。
`gh run list` 也会因此列出**上游**的运行（本次实测：改前列出的全是 openai/codex 的 Issue/CLA 运行）。

### 8.3 结果分类：红的是**基础设施**，不是我们的代码

大量 job 直接红在**启动阶段**，注解原文：

| 注解原文                                                                                                            | 含义                                                    | 归类     |
| ------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- | -------- |
| `Required runner group 'codex-runners' not found`                                                                   | 工作流要上游组织级**自建 runner 组**，fork 里没有       | 环境限制 |
| `The job was not started because recent account payments have failed or your spending limit needs to be increased.` | `macos-15-xlarge` 等**计费机型**起不来（账单/额度问题） | 环境限制 |
| fork 的 `gh secret list` / `gh variable list` 都是 **0 条**                                                         | 需要机密的 job（远程缓存/签名等）也无从满足             | 环境限制 |

**能真正跑起来的**（标准 `ubuntu-24.04`、无机密依赖）：

- `changed`（Detect changed areas）— **completed success**
- `general`（步骤里有 `Format / etc` = `just fmt-check` 一类）— in_progress
- `cargo_shear` — in_progress
- `argument_comment_lint_package`（Linux、按包安装工具链）— in_progress
- 一条 `Bazel test on ubuntu-24.04 for x86_64-unknown-linux-musl`（标准 runner）— in_progress

⇒ 恰好是**我们最关心的几条**（格式、argument-comment-lint、shear）能跑；平台矩阵与 Bazel 的多平台/自建 runner 在个人 fork 上**结构性跑不了**。

### 8.4 结论（对应「不 PR 能不能 CI」）

- **能拿到远端信号的**：`rust-ci` 与 `bazel` 的手动 dispatch（`gh workflow run … --repo clearnature/codex --ref <branch>`），
  或推一个 `*full-ci` 分支跑 `rust-ci-full`。
- **仍然拿不到 i18n 三闸（`just fmt-check` / `just i18n-check` / `just i18n-smoke`）**：
  它们在 `repo-checks` 里，而 `repo-checks` 只有 `workflow_call`、由 `blocking-ci` 在 `pull_request` / `push: main` 时带起。
  ⇒ **想在远端跑那三道，必须开 PR**（或推 main）。好消息：`repo-checks` 跑在 `ubuntu-latest`、不依赖上游机密，
  所以在个人 fork 上**它本身是能跑的**（不像 Bazel 的多平台矩阵）。
- **平台矩阵（macOS/Windows）在本 fork 上结构性不可用**：要么缺 `codex-runners` 组，要么计费机型起不来。
  ⇒ 43 处平台门控站点的验签仍只能靠**上游 CI** 或人工平台验证，本 fork 帮不上。
