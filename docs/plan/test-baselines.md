# 测试基线：成功集 / 失败集 / 状态哈希

本文是**本仓库的测试流程标准**（面向在本机反复跑测试的人与 agent），配套工具是
`scripts/test_baseline.py`，数据落在 `scripts/test-baseline.json`。

## 1. 要解决的问题

「这次跑了 432 passed / 2 failed」不是可复现的信息：

- 下一次跑可能不一样（顺序相关、时序相关、机器状态）；
- 没人能说清某个失败是**这次改动引入的**，还是**本来就在**。

这两种误判的代价都不小：把既有失败算到自己头上会浪费一整轮测试；反过来把真回归当成
"大概是 flaky" 放过去更糟。

所以本仓库把**失败集当作一等数据**：每个 scope 记录它的成功集哈希、失败集与状态哈希，
每次跑都拿这三样与记录比对。

## 2. 三个哈希

| 字段                 | 含义                                                                           |
| -------------------- | ------------------------------------------------------------------------------ |
| `pass_hash`          | 通过的测试名（排序后）的 sha1                                                  |
| `fail` / `fail_hash` | **失败集**：失败测试名（排序后）与它的 sha1                                    |
| `state_hash`         | `sha1(command + pass_hash + fail_hash)` —— 「**这个 scope 在这个状态**」的身份 |

**报告里引用哈希，而不是转述"测试是绿的"**。例如：`core-session` 的
`state_hash=b28e4bd9cdd1…` 就精确表示「`cargo test -p codex-core --lib session` 下
432 通过 + 那两条已知失败」这一状态；任何人重跑都能核对。

## 3. 判定规则（工具的硬规则）

`--check` **只把一件事当回归**：某个测试现在失败，而它既不在记录的失败集里，也没有被声明为 `flaky`。

| 观察                                   | 处置                                                      | 是否红           |
| -------------------------------------- | --------------------------------------------------------- | ---------------- |
| 失败集与记录逐名相同、`pass_hash` 相同 | 打印 `identical success set and failure set`              | 否               |
| 记录的失败现在通过了                   | 提示 `was failing, now passing (re-record with --update)` | 否               |
| 通过集变了（新增/删除测试）            | 打印计数变化与新 `pass_hash`                              | 否               |
| 出现**未记录**的失败                   | `NEW FAILURES: …`                                         | **是（exit 1）** |
| 失败在 `flaky` 名单里                  | `known flaky, tolerated: <名字> -- <说明>`                | 否               |

**`flaky` 不是"忽略名单"，是"需要证据的声明"**：声明时必须写 `flaky_notes`（观测次数、
红/绿分布、单独运行结果、为什么不归因本次改动），并且**不得**用它掩盖集合稳定的失败
（失败集稳定 = 真问题）。

## 4. 每批的标准动作

1. **开工前**（可选但推荐）：`python3 scripts/test_baseline.py --list` 看一眼涉及 scope 的 `state_hash`。
2. **收尾**：`python3 scripts/test_baseline.py --all --check`
   - 绿 ⇒ 在交付里写下相关 scope 的 `state_hash`（这就是「本批未引入失败」的证据形态）；
   - 红 ⇒ 只处理被点名的 `NEW FAILURES`，见下条。
3. **归因**（红的时候）：先判「是不是我的」——把本批改动 `git checkout HEAD~1 -- <改动文件>`
   （或在提交前 `git stash`），**在同一条命令、同一过滤范围**上重跑，比较失败集合的**名字**：
   - 名字逐字相同 ⇒ 与本次改动无关（预先存在）；把它记录进基线，别算进本批；
   - 出现新名字 ⇒ 是自己的改动，去修。
     ⚠ 只跑失败的那一条会得出相反结论（顺序相关失败单独跑会通过）。
4. **重录基线**：`--update`（只在"失败集变化已被解释"之后做；`--update` 会覆盖记录）。
5. **不要**"重试到绿"。同一命令两次失败集合不同 ⇒ 记为 flaky 并声明（第 3 节），
   或者如实标注"未验证"。

## 5. 现有 scope

| scope                 | 命令（`cwd=codex-rs`）                                                                       |
| --------------------- | -------------------------------------------------------------------------------------------- |
| `core-session`        | `cargo test -p codex-core --lib session`                                                     |
| `core-unified-exec`   | `… --lib unified_exec`                                                                       |
| `core-stdin-approval` | `… --lib stdin_approval`                                                                     |
| `core-guardian`       | `… --lib guardian`                                                                           |
| `core-approvals`      | `… --lib approvals`                                                                          |
| `core-thread-manager` | `… --lib thread_manager`                                                                     |
| `core-tools-handlers` | `… --lib handlers`                                                                           |
| `core-exec`           | `… --lib exec`                                                                               |
| `core-code-mode`      | `… --lib code_mode`                                                                          |
| `tui-lib`             | `cargo test -p codex-tui --lib -- --skip ide_context::ipc`（877 快照 + 全 lib 测试；约 60s） |

全部带 `RUST_MIN_STACK=16777216`（默认 2 MiB 线程栈会让部分测试 SIGABRT）。
加 scope 就在 `SCOPES` 表里加一行；可选 `skip` 字段会追加成 `-- --skip <名字>`（`tui-lib` 用它排掉
本机环境相关的 `ide_context::ipc`，与 `tui-test` 门禁命令逐字一致 —— **命令不一致的 scope 不能互相代替**）。
跨 crate 的其余范围（`just test -p codex-exec` 等）尚未纳入。

## 6. 已知记录（截至建立基线时）

- `core-session` 失败集固定为 2 条：`session::tests::user_shell_commands_do_not_inherit_managed_network_proxy`
  （单独跑也失败 ⇒ 独立预先存在）与
  `session::turn::tests::post_sampling_token_estimate_is_disabled_by_always_on_sinks`
  （单独跑通过、整批失败 ⇒ 顺序/全局 tracing 状态相关）。
- `core-stdin-approval` 声明 1 条 flaky：`unified_exec::tests::stdin_approval_preserves_the_reviewed_terminal`
  （4 次同过滤观测：1 次 7/1、3 次 8/0；单独运行 2 次均通过；耗时约 6s ⇒ 时序相关）。
- `tui-lib` 的失败集**不稳定**：同一命令同一二进制**七次**观测得到七种组合 ——

  | #   | 结果     | 失败集合                                                                                                                                                                            |
  | --- | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
  | 1   | 4285P/2F | `agents_overview_acknowledges_inactive_steer_before_interrupt`、`cached_legacy_resume_revalidates_history_across_migration_settings`                                                |
  | 2   | 4285P/2F | `agents_overview…`、`startup_draft_preserves_non_bracketed_multiline_pastes_without_submitting`（门禁回执 `r-mu7g6yu2-9s9yb6`）                                                     |
  | 3   | 4284P/3F | `agents_overview…`、`cached_legacy_resume…`、`startup_draft_…`                                                                                                                      |
  | 4   | 4284P/3F | `agents_overview…`、`cached_legacy_resume…`、`background_exit_tests::exit_interrupts_before_requesting_shutdown`（回执 `r-mu7gb1vo-a7j2s3`）                                        |
  | 5   | 4286P/1F | 只有 `agents_overview…`                                                                                                                                                             |
  | 6   | 4284P/3F | 同 #4（另有 1 条未解析，见 §8.5）                                                                                                                                                   |
  | 7   | 4285P/2F | `background_exit_tests::exit_interrupts…`、`safety_buffering::active_turn_interrupt_is_nonblocking_and_coalesces_repeated_requests`、`cached_legacy_resume…`（第 5 条名字首次出现） |

  因此该 scope 的 `fail` 集记为**空**，五条名字都进 `flaky` 并各带 `flaky_notes`：
  五条全部做过 `--exact` 单独复跑，各 3/3 通过（回执 `r-mu7g7zxc-hcn57x`、`r-mu7gtx5c-8gi9yi`）
  ⇒ 是并行/时序相关，不是确定性失败。`agents_overview…` 七次里失败五次却单独跑 3/3 通过，
  是这批里最稳定的「不稳定项」。
  **判据**：失败集合是否稳定，而不是「有没有失败」——同一命令两次失败集合不同即判 flaky，
  处置是**如实标注**，不是重试到绿、更不是改快照或 `#[ignore]`。

- 该 scope 的声明是**承重的**：删掉四条（当时）声明后 `--check` 报
  `NEW FAILURES: agents_overview…` 且 exit 1（负向控制回执 `r-mu7gwyhv-kg7qp1`）。
  ⚠ 这个控制是**第三次**才有效的 —— 前两次「无效」各自暴露了一个真问题，见 §8.5。

## 7. 数据是"本机"的

`test-baseline.json` 记录的是**本机**的观测（依赖编译产物、环境变量、并发度）。在别的机器/CI 上
应从该环境重新 `--update` 建立，而不是照搬；跨机器比较失败集时先确认命令与 `cwd` 一致
（`state_hash` 已把 command 纳入，因此两台机器上同名 scope 的 `state_hash` 不同是**预期**行为）。

## 8. 来自实战的补则（8.1–8.5）

### 8.1 flaky 要在**每个**匹配它的 scope 里声明

cargo 的过滤是**测试路径子串匹配**，所以同一个测试可能同时属于多个 scope。实测：
`unified_exec::tests::stdin_approval_preserves_the_reviewed_terminal` 同时匹配 `stdin_approval` 与
`unified_exec` 两个 scope —— 只在 `core-stdin-approval` 里声明 flaky 时，`core-unified-exec`
把它报成 `NEW FAILURES`（**工具行为正确**，是声明漏了）。补声明的成本很低，漏声明的代价是一轮假回归。

### 8.2 平台 `cfg` 下的改动：本机验不了就如实标，别冒充

改动落在 `#[cfg(target_os = "…")]` 而本机不是那个平台时，**本机 `cargo check` 根本不编译那段代码**——
它通过了也不代表那段代码编得过。本机的实测限制：对 `codex-core` 做
`cargo check --target x86_64-pc-windows-msvc` 会在**依赖构建期**失败
（`blake3`/`ring` 的 cc-rs 需要 `ml64.exe`/`lib.exe`，见 known_issues `no-windows-cross-check-linux`，回执 `r-mu7caoa2-p3z0qp`）。

处置：把这类改动单列成「**未验证（平台限制）**」并在交付里写清需哪个平台的 CI 复核；
台账里用 `needs_review`（验收要求里那条记 `incomplete`），**不要**因为 Linux 编译过了就写 `done`。

### 8.3 别让管道吞掉退出码

跑本节这些命令时（尤其 `--check`）**不要**把输出接进 `| tail`/`| head`/`| grep` 收尾，
或者显式带上被检命令的退出码：

```bash
cmd 2>&1 | grep …; rc=${PIPESTATUS[0]}; exit $rc      # 或者 bash: set -o pipefail
```

实测两次假绿：后台 job 报 `exit code: 0` 而 cargo 实际 101（0 来自 `| tail`）；末尾接 `echo`
让 `expectFail` 的负向控制判成「没红」。把「输出里有期待的片段」当**内容**证据、
退出码当**状态**证据，两个都要（known_issues `pipe-masks-exit-code`，模式层）。

### 8.4 一次改动的多个站点可能落在**不同报表**里 —— 控制要断言「看得见它的那个」

`i18n_todo` 的四个口径覆盖**不同的桶**：候选报表只看 `candidates` 桶；`--suspect` 看
`internal:assert`/`internal:log`；`--precise` 报「被宽松规则藏起来的邻居」；`--traps` 看匹配键。
同一个批次的站点**可能分散在多个桶**里（实测：`tools/runtimes/unified_exec.rs` 这批 8 个站点里，
两条 `missing command line for PTY` 在候选桶，而 `failed to query exec-server capabilities: {err}`
因为上方有日志形态的调用被判进 `internal:log`）。

所以负向控制不能只盯一个报表数数：**先确认该站点落哪个桶，再断言那个报表**（或断言多个报表的并集里
出现它）。本轮实测教训：我按「候选报表应报 3 条」断言，实际候选侧只报 2 条 + `--suspect` 报 1 条 ⇒
控制被判成"没红"，而这**不是**工具失灵，是断言盯错了报表。

### 8.5 报表**看不见**的失败 = 没有失败（`tui-lib` 的解析器 bug）

`tui-lib` 这条 scope 第一次接入时，`--check` 对它是**空洞通过**的：

- 症状：摘要行说 `failed=3`，而解析出来的失败集是空的 —— `fail_hash` 等于
  `sha1("") == da39a3ee5e6b4b0d3255bfef95601890afd80709`。**空哈希就是这次的自证**。
- 根因：TUI 测试把 ANSI 转义序列渲染到与测试框架自己那行 `test <名字> ... FAILED` **同一行**上，
  而解析用的是锚定正则 `^test (\S+) \.\.\. (FAILED)$`（`line.strip()` 之后仍带转义前缀）⇒ 一条也匹配不上。
  摘要行没被污染，所以计数是对的、名字是全丢的。
- 修法：先剥 ANSI（`\x1b\[[0-9;?]*[ -/]*[@-~]`），再按 `\r`/`\n` 切分（TUI 用 `\r` 重画），
  每段取**最后一个** `test … ok|FAILED` 事件，而不是锚定整行。
- **发现方式**：负向控制（删掉 flaky 声明 ⇒ 应报 `NEW FAILURES`）**没红**。当时第一反应是
  「flaky 恰好全过」，但连删四次声明仍不红 ⇒ 才去查解析路径，撞上上面的空哈希。
  **控制没红时，先假设是尺子坏了，再假设是被测对象没问题** —— 这次的顺序反了，白跑两轮。
- 另一个同类坑：`set -e` + `out=$(cmd)` —— **赋值语句的退出码就是命令的退出码**，
  于是「命令非零 ⇒ 脚本立刻终止」，后续的断言与 `echo "exit=$rc"` 一行都不执行，
  看起来像"什么也没发生"。要捕获退出码就写 `rc=0; out=$(cmd) || rc=$?`。

配套的另一个教训：`--update` 曾用 `dict(observed)` 覆盖整条记录，**把 `flaky_notes`/`note` 手写注记
一起抹掉**（= 一个 scope 悄悄失去「为什么容忍这些失败」的证据）。现在是显式保留这些字段。

## 语言环境必须钉住（第 547 轮实测）

**规矩**：在本机跑 CLI/TUI 的 crate 测试时，语言环境必须显式钉住 —— `LANG=C LC_ALL=C just test -p codex-cli`。

**为什么**：`docs/plan/i18n-design.md:204` 的语言优先级是
`--lang > config.toml 的 locale > LC_ALL > LANG > 系统 locale`。本机 `LANG=zh_CN.UTF-8`，
未显式指定语言的进程渲染中文；而仓库里有一批测试直接断言英文文案、自身不钉语言。

**实测（双向控制，决定性）**：

| 环境                           | 命令                                                         | 结果                                      |
| ------------------------------ | ------------------------------------------------------------ | ----------------------------------------- |
| `LANG=zh_CN.UTF-8`（本机默认） | `just test -p codex-cli -E 'test(queue_) \| test(worktree)'` | exit 100：9 run / 4 passed / **5 failed** |
| `LANG=C LC_ALL=C`              | 同上                                                         | **exit 0：9 passed**                      |
| 本机默认                       | `just test -p codex-cli`                                     | 414 run / 409 passed / 5 failed           |

失败名单：`queue_rejects_local_daemon_that_does_not_support_queueing`、
`queue_does_not_fallback_from_unsupported_explicit_remote`、`queue_submits_message_to_remote_app_server`、
`queue_rejects_overrides_that_bypass_local_daemon`、`interactive_worktree_start_and_fork_bind_owner_before_turn`。
归档：`known_issues zh-locale-breaks-english-cli-tests`（high/environment）、`journal j-mu7zk7gi-6s3q`。

**对测试的要求（标准条目）**：

1. 断言**用户可见文案**的测试必须在**测试内部**钉住语言（`--lang en`，或把 `LANG`/`LC_ALL` 置为 `C`），
   不得依赖运行环境 —— 否则同一份代码在 zh 机器与 CI 上结论不同；
2. 不得用**会被翻译的字符串**做控制流判据（§9.1 匹配键类老坑的测试版）；
3. 只测一侧语言的断言是**半个断言**：文案行为必须成对验证（环境/缺省一侧 + 显式覆盖一侧），
   双向探针见 `i18n-verification.md` §12.59。

4. **负向控制的包装脚本必须传播失败**：`expectFail` 回执要求命令真的**非零退出** ——
   若脚本把观察到的失败吞掉（最后一条语句成功、`exit 0`），回执会被判为未通过。
   实测（第 565 轮）：包装脚本先 echo「OK(负向)」再 `exit 0` ⇒ `r-mu81l71v-lc020g` 判未通过；
   改成 `exit "$rc"` 后签成通过。
