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

| 字段 | 含义 |
| --- | --- |
| `pass_hash` | 通过的测试名（排序后）的 sha1 |
| `fail` / `fail_hash` | **失败集**：失败测试名（排序后）与它的 sha1 |
| `state_hash` | `sha1(command + pass_hash + fail_hash)` —— 「**这个 scope 在这个状态**」的身份 |

**报告里引用哈希，而不是转述"测试是绿的"**。例如：`core-session` 的
`state_hash=b28e4bd9cdd1…` 就精确表示「`cargo test -p codex-core --lib session` 下
432 通过 + 那两条已知失败」这一状态；任何人重跑都能核对。

## 3. 判定规则（工具的硬规则）

`--check` **只把一件事当回归**：某个测试现在失败，而它既不在记录的失败集里，也没有被声明为 `flaky`。

| 观察 | 处置 | 是否红 |
| --- | --- | --- |
| 失败集与记录逐名相同、`pass_hash` 相同 | 打印 `identical success set and failure set` | 否 |
| 记录的失败现在通过了 | 提示 `was failing, now passing (re-record with --update)` | 否 |
| 通过集变了（新增/删除测试） | 打印计数变化与新 `pass_hash` | 否 |
| 出现**未记录**的失败 | `NEW FAILURES: …` | **是（exit 1）** |
| 失败在 `flaky` 名单里 | `known flaky, tolerated: <名字> -- <说明>` | 否 |

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

| scope | 命令（`cwd=codex-rs`） |
| --- | --- |
| `core-session` | `cargo test -p codex-core --lib session` |
| `core-unified-exec` | `… --lib unified_exec` |
| `core-stdin-approval` | `… --lib stdin_approval` |
| `core-guardian` | `… --lib guardian` |
| `core-approvals` | `… --lib approvals` |
| `core-thread-manager` | `… --lib thread_manager` |
| `core-tools-handlers` | `… --lib handlers` |
| `core-exec` | `… --lib exec` |

全部带 `RUST_MIN_STACK=16777216`（默认 2 MiB 线程栈会让部分测试 SIGABRT）。
加 scope 就在 `SCOPES` 表里加一行；跨 crate 的 scope（`just test -p codex-tui`）尚未纳入
（它的运行时长更适合放后台/CI）。

## 6. 已知记录（截至建立基线时）

- `core-session` 失败集固定为 2 条：`session::tests::user_shell_commands_do_not_inherit_managed_network_proxy`
  （单独跑也失败 ⇒ 独立预先存在）与
  `session::turn::tests::post_sampling_token_estimate_is_disabled_by_always_on_sinks`
  （单独跑通过、整批失败 ⇒ 顺序/全局 tracing 状态相关）。
- `core-stdin-approval` 声明 1 条 flaky：`unified_exec::tests::stdin_approval_preserves_the_reviewed_terminal`
  （4 次同过滤观测：1 次 7/1、3 次 8/0；单独运行 2 次均通过；耗时约 6s ⇒ 时序相关）。

## 7. 数据是"本机"的

`test-baseline.json` 记录的是**本机**的观测（依赖编译产物、环境变量、并发度）。在别的机器/CI 上
应从该环境重新 `--update` 建立，而不是照搬；跨机器比较失败集时先确认命令与 `cwd` 一致
（`state_hash` 已把 command 纳入，因此两台机器上同名 scope 的 `state_hash` 不同是**预期**行为）。
