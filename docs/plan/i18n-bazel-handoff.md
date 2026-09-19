# i18n 分支 · Bazel 侧完整性与工具链交接单

> **给下一个会话（AI 或人）的交接。** 本文只记录 2026-09-16 这次会话在 `feat/i18n` 上做的**验证与修复**，
> 不重复 i18n 本身的设计——那是上游文档的事。
> 基线：`feat/i18n` @ `e6847b6b8`（与 `main` 的 merge-base：`b1205c12d`）。
> 上游文档：[`i18n-design.md`](./i18n-design.md)、[`i18n-verification.md`](./i18n-verification.md)、[`i18n-glossary.md`](./i18n-glossary.md)。

> **更新（2026-09-16，本单提交时）**：本文撰写时列为「未提交」的三个 i18n 文件
> （`exec/src/lib.rs`、`i18n/src/dict_zh.rs`、`core/tests/common/test_codex_exec.rs`）已由另一会话
> 提交为 **`4be5fdc8c`**（`i18n(exec): translate the exec CLI messages and pin the test locale`），
> 本文的 `i18n-check/BUILD.bazel` + 本文件 + `i18n-verification.md` 改动随同一提交落地。
> 另：`exec` 集成测试因本机 `LANG=zh_CN.UTF-8` 在 locale 链生效后出现 6 条英文断言失败，
> 已在 `TestCodexExecBuilder::cmd` 钉 `LC_ALL=C` 修好（`--test all` 78 passed / 0 failed）。

## 一、结论先行

| 问题                         | 结论                                                                                                       |
| ---------------------------- | ---------------------------------------------------------------------------------------------------------- |
| Bazel 侧缺文件吗？           | **不缺**。构建图完整，`@crates` 从 Cargo metadata 正确解析出新增的两个 crate，`MODULE.bazel.lock` 无漂移。 |
| 那有没有问题？               | **有一个真的编译失败**：`codex-rs/i18n-check/BUILD.bazel` 缺 `crate_srcs = []`。已修并验证。               |
| dotslash 是 Bazel 的依赖吗？ | **不是**。`bazel build` / `bazel test` 全程不需要它；它只影响 `just fmt` 一类工具链。本机已装好。          |

## 二、交付物：1 个文件改动（**未提交**）

| 文件                              | 改动                                | 状态                          |
| --------------------------------- | ----------------------------------- | ----------------------------- |
| `codex-rs/i18n-check/BUILD.bazel` | 加 `crate_srcs = []` + 2 行说明注释 | 未提交，已过 `just fmt-check` |

```bazel
codex_rust_crate(
    name = "i18n-check",
    crate_name = "codex_i18n_check",
    # Binary-only crate: without this the default `src/**/*.rs` glob builds a
    # library out of the `*_tests.rs` modules that sit beside main.rs.
    crate_srcs = [],
)
```

### 根因（完整因果链，别再重新推导）

`defs.bzl` 里 `codex_rust_crate` 的库源码默认值是：

```python
lib_srcs = native.glob(["src/**/*.rs"], exclude = binaries.values(), allow_empty = True)
```

`i18n-check` 是**纯 bin crate**（`Cargo.toml` 有 `[[bin]] path = "src/main.rs"`，无 `src/lib.rs`）。
`binaries.values()` 只排掉 `src/main.rs`，于是 glob 剩下 `src/main_tests.rs` → 非空 → Bazel 建了一个
**虚假的 `rust_library`**，拿 `main_tests.rs` 当库根。该库只拿到 normal deps（`pretty_assertions` 是
dev-dependency），且 `crate::extract_*` 指的是 `main.rs` 的条目 —— 实测 5 个 `E0432` 全部出自 `main_tests.rs`：

```
ERROR: codex-rs/i18n-check/BUILD.bazel:3:17: Compiling Rust rlib //codex-rs/i18n-check:i18n-check failed
error[E0432]: unresolved import `pretty_assertions`                --> src/main_tests.rs:1:5
error[E0432]: unresolved import `crate::extract_label_literals`    --> src/main_tests.rs:3:5
error[E0432]: unresolved import `crate::extract_tr_calls`          --> src/main_tests.rs:4:5
error[E0432]: unresolved import `crate::read_dictionary_pairs`     --> src/main_tests.rs:5:5
error[E0432]: unresolved import `crate::spacing_violations`        --> src/main_tests.rs:6:5
```

### 先例与边界（**不要**顺手去"修"别的 crate）

| crate                                               | `src/` 内容                            | 是否受影响               | 原因                                                                    |
| --------------------------------------------------- | -------------------------------------- | ------------------------ | ----------------------------------------------------------------------- |
| `voice-host`                                        | 有 `main_tests.rs` 等多个 `*_tests.rs` | 已写 `crate_srcs = []`   | **本仓库既有惯例，照它做**                                              |
| `config-schema` / `thread-manager-sample` / `bwrap` | 只有 `main.rs`                         | 不受影响                 | glob 为空 → 不建库                                                      |
| `core/tests/common`                                 | `crate_srcs = glob(["*.rs"])`          | 不受影响，**是有意为之** | 它整个目录就是一个测试支持**库**（`core_test_support`），不是 bin crate |

**可复用的规则**：新增 crate 时，只要满足「纯 bin（无 `src/lib.rs`）」**且**「`src/` 下除 `main.rs` 外还有别的 `.rs`」，
就必须显式写 `crate_srcs = []`。

### 验证（修复后实测）

```
bazel build //codex-rs/i18n-check:all                          → Build completed successfully
bazel test  //codex-rs/i18n:all //codex-rs/i18n-check:all
  //codex-rs/i18n:i18n-unit-tests                              PASSED in 0.1s
  //codex-rs/i18n-check:codex-i18n-check-bin-unit-tests         PASSED in 0.1s
  Executed 2 out of 2 tests: 2 tests pass.
```

## 三、Bazel 侧其他检查项（都通过，无需再查）

| 检查                  | 命令                                                                       | 实测                                                                                                        |
| --------------------- | -------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| 构建图完整性          | `bazel build --nobuild //codex-rs/i18n:all //codex-rs/i18n-check:all`      | ✅ 231 packages / 38260 targets configured                                                                  |
| lockfile 漂移         | `bazel mod deps --lockfile_mode=error`（= `just bazel-lock-check` 的核心） | ✅ `EXIT=0`，`MODULE.bazel.lock` 无需更新                                                                   |
| 编译期文件读取        | 本分支是否新增 `include_str!` / `include_bytes!`                           | ✅ 无 → **不需要**补 `compile_data` / `build_script_data`                                                   |
| 消费方 BUILD 是否要改 | `tui` / `cli` / `exec` 各只加了 `codex-i18n = { workspace = true }`        | ✅ **不该改**：Bazel 的 crate 依赖从 Cargo metadata 自动解析，已验证解析成功                                |
| 整条链能否编出来      | `bazel build //codex-rs/cli:codex //codex-rs/exec:all`                     | ✅ 10 targets / 4512 actions / 452.028s / `exit 0`（`core` 486 files → `tui` 630 files → `cli:codex` 全过） |

## 四、环境变更：dotslash 已安装（下一会话须知道）

**本机 `dotslash` 之前不存在，现已装好**：

```
cargo install --locked dotslash     →  dotslash v0.5.7  →  /home/yanli/.cargo/bin/dotslash   (60s)
dotslash --version                  →  DotSlash 0.5.7
dotslash tools/buildifier --version →  buildifier version: 8.5.1-3-g0951e28
```

`tools/buildifier` 用的是 `size`/`hash`/`digest`/`path`/`providers` 那套 manifest schema，0.5.7 能正常解析。

**dotslash 的适用边界（已实测确认）**：

| 场景                                                         | 需要 dotslash？                                                                                           |
| ------------------------------------------------------------ | --------------------------------------------------------------------------------------------------------- |
| `bazel build` / `bazel test`                                 | ❌ 不需要（今天全程没有它也跑通了）                                                                       |
| `just fmt` / `just fmt-check`                                | ✅ 需要（`scripts/format.py:73` 显式调 `dotslash tools/buildifier`，注释说明是因为 Windows 不认 shebang） |
| `just argument-comment-lint`（**带参数**，走预编译 wrapper） | ✅ 需要（`tools/argument-comment-lint/wrapper_common.py:234-243`）                                        |
| `just argument-comment-lint`（不带参数，走 Bazel aspect）    | ❌ 不需要                                                                                                 |
| `just assemble-codex-package`                                | ✅ 需要（`scripts/codex_package` 的 `rg` / `codex-zsh` / `zstd` 三个 manifest）                           |

装之前它曾是 `just fmt` **唯一**缺的工具：`format.py` 的 5 个 formatter group（Just / Rust / Bazel-Starlark / Python SDK / Python scripts）
并行跑，其余四组所需的 `just`、`cargo`、`uv`、`scripts/.venv`、`sdk/python/.venv` 本机都已齐备 —— 这解释了为什么之前报错只出现 Bazel/Starlark 一组。
上游也把它当标准开发环境的一部分：`.devcontainer/devcontainer.json:14` 装了 dotslash feature；CI 侧 `.github/workflows/repo-checks.yml:60-61` 跑 `just fmt-check`。

**安装后验证**：`just fmt-check` → `EXIT=0`（五组全过，含上面那处 `i18n-check/BUILD.bazel` 的格式）。

## 五、当前工作区状态（4 个未提交文件）

| 文件                                            | 归属                                 | 说明           |
| ----------------------------------------------- | ------------------------------------ | -------------- |
| `codex-rs/i18n-check/BUILD.bazel`               | 本次会话                             | 见 §二，待提交 |
| `codex-rs/exec/src/lib.rs`                      | **本次会话之前就有**                 | 未触碰         |
| `codex-rs/i18n/src/dict_zh.rs`                  | **本次会话之前就有**                 | 未触碰         |
| `codex-rs/core/tests/common/test_codex_exec.rs` | **构建窗口内出现的，非本次会话所写** | 见 §六         |

## 六、一个需要留意的观察：构建期间源文件被并发修改

`bazel build` 全量构建期间，Bazel 报了：

```
WARNING: //codex-rs/core/tests/common:common: Skipping uploading outputs because of
concurrent modifications with --guard_against_concurrent_changes enabled:
.../codex-rs/core/tests/common/test_codex_exec.rs was modified during execution
```

该文件在本次会话开始时的 `git status` 里**不存在**，构建跑完后出现（mtime `07:25`）。diff 是 i18n 相关的一处改动——
给测试子进程固定 locale：

```rust
+            // The CLI resolves its locale from the environment (`--lang` > config
+            // `locale` > `LC_ALL` > `LANG` > system locale). These tests assert on
+            // English output, so pin the locale rather than inheriting whatever the
+            // developer（或 CI 机器）has exported.
+            .env("LC_ALL", "C")
```

**不是本次会话所写**（本次会话只改过 `i18n-check/BUILD.bazel` 一个文件），推测是用户或并行会话在构建窗口内编辑的，故按原样保留。
Bazel 那条警告的机制是「先算哈希 → 执行 → 发现文件又变了 → 拒绝把这批产物上传缓存」（保守防污染），**不是构建错误**。

已确认当前树是干净的：

```
bazel build //codex-rs/cli:codex //codex-rs/exec:all   → Build completed successfully（5.6s，全缓存）
bazel build //codex-rs/core/tests/common:common        → up-to-date
```

第二条是关键判据：`test_codex_exec.rs` 确实在该 target 的 15 个 `srcs` 里（`crate_srcs = glob(["*.rs"])`），
既然 Bazel 报 up-to-date，说明**当前内容已经成功编译过**，不是"改动没进构建"。
依赖关系上，该 target 在 `//codex-rs/exec:all` 的闭包里（经 `//codex-rs/exec:exec-all-test-bin`），
但**不在** `//codex-rs/cli:codex` 的闭包里。

**注意**：那句注释把中文「（或 CI 机器）」嵌进了英文句子。功能无碍，但若这批改动要向上游 PR，评审大概率会挑此处。

## 七、待办（按优先级）

1. **提交 `codex-rs/i18n-check/BUILD.bazel`** —— 它属于「新增 i18n-check crate」那次提交的遗留，可 `--fixup` / 直接跟在 i18n 系列提交后。
2. **更新 [`i18n-verification.md`](./i18n-verification.md) §八 的第 2 条风险** —— 原文写「Bazel 侧成本未评估 …… 本轮未衡量改动量」，**现已结清**：答案是 1 行 `crate_srcs = []`，且不需要任何 `compile_data` / `build_script_data`。
3. **决定 `test_codex_exec.rs` 里那句中英混排注释**怎么处理（见 §六）。
4. **补跑 Bazel 侧尚未跑过的门禁**（非阻塞，但提交前值得）：
   - `bazel test //codex-rs/tui:all`（877 个界面快照在 Bazel 侧的对应验证；注意 `tui` 的 `test_tags = ["no-sandbox"]`）
   - `just bazel-clippy`
   - `just argument-comment-lint`（dotslash 现已具备，预编译路径也可用了）
   - 完整 `just bazel-test`（`bazel test --test_tag_filters=-argument-comment-lint //... --keep_going`）——**未跑，耗时长**
5. 可选：`just fmt`（**fix 模式**，会真正写文件）本次只跑了 `--check`，改动只有 1 个已合规的 BUILD 文件，故未执行。

## 八、可粘贴的复现命令

```bash
# 本次改动涉及的两个 crate：构建 + 单测
bazel build //codex-rs/i18n:all //codex-rs/i18n-check:all
bazel test  //codex-rs/i18n:all //codex-rs/i18n-check:all --test_output=errors

# 纯分析（不编译，最快的结构完整性检查）
bazel build --nobuild //codex-rs/i18n:all //codex-rs/i18n-check:all

# lockfile 漂移门禁
bazel mod deps --lockfile_mode=error ; echo "EXIT=$?"

# 整条 CLI 链在 Bazel 下能否编出来
bazel build //codex-rs/cli:codex //codex-rs/exec:all

# 格式化门禁（需要 dotslash，本机已装）
just fmt-check ; echo "EXIT=$?"
```
