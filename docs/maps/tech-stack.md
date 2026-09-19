# 技术栈

## 语言分布

| 语言                            |                              文件数 | 角色                                                                                                                                         |
| ------------------------------- | ----------------------------------: | -------------------------------------------------------------------------------------------------------------------------------------------- |
| **Rust**                        | 4,156（全仓）/ 3,792（`codex-rs/`） | **核心实现**，约 1,741,939 行                                                                                                                |
| TypeScript                      |                                 744 | 主要为**代码生成**产物：626 个在 `codex-rs/app-server-protocol/schema/typescript/v2`，93 个在其上层；仅约 24 个是 `sdk/typescript/` 手写 SDK |
| Python                          |                                 198 | `sdk/python/`（SDK）、`scripts/codex_package`、`scripts/mcp_conformance`、`.github/scripts`、`third_party/voice`                             |
| Bazel（`BUILD.bazel` / `.bzl`） |                                 198 | 构建定义                                                                                                                                     |
| SQL                             |                                  69 | 数据库迁移                                                                                                                                   |
| Shell / PowerShell              |                              36 / 7 | 脚本                                                                                                                                         |

另：1,068 个 `.snap`（insta 快照测试）。

## Rust 工程配置

| 项                | 值                              | 来源                                                    |
| ----------------- | ------------------------------- | ------------------------------------------------------- |
| edition           | `2024`                          | `codex-rs/Cargo.toml` 的 `[workspace.package]`          |
| toolchain         | `1.95.0`                        | `codex-rs/rust-toolchain.toml`                          |
| toolchain 组件    | `clippy`、`rustfmt`、`rust-src` | 同上                                                    |
| workspace version | `0.154.0`                       | `[workspace.package]`；**release tag 上已是真实版本号** |
| license           | `Apache-2.0`                    | `[workspace.package]`                                   |
| workspace 成员    | 145 个条目                      | `members = [...]`，含 `ext/*`、`utils/*` 嵌套路径       |

> **版本号：源码里读不到真实版本**（2026-09-16 实测）：
>
> - **`main` 与工作区**：`codex-rs/Cargo.toml` 的 `[workspace.package] version` 是
>   `0.0.0`（占位），各 crate 以 `version.workspace = true` 继承，`codex --version`
>   经 `env!("CARGO_PKG_VERSION")` 输出 `codex-cli 0.0.0`。
> - **发版 tag 上**：真实版本由发版工具链写进 tag 指向的一次性提交，例如
>   `rust-v0.154.0` 的 `6b9826e3a` —— 整个提交只改这一行，且**不在 `main` 上**。
>
> 本 fork 不改源码，而是用 `scripts/build-release-local.sh` 在**构建期**从 tag 派生
> 版本、临时写入 manifest、构建后 `trap` 还原：产物携带真实版本，工作区保持
> `0.0.0`（快照因此天然一致，同步上游也零冲突）。完整方案与证据见
> [`../plan/build-and-versioning.md`](../plan/build-and-versioning.md)。
>
> `Cargo.lock` 里这些 crate（150 处）已是 `0.0.0`，与工作区 manifest 一致，
> 不再有"首次 `cargo build` 自动改写 lock"的情况。
> （`codex-cli/package.json` 是 npm 包侧的 `0.0.0-dev` 占位；`CHANGELOG.md` 指向
> GitHub releases 页。）

## 构建系统

三重并存：

| 系统            | 用途                                      | 关键文件                                                                                                                   |
| --------------- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| **Bazel**（主） | 处理内嵌重型原生依赖（V8、wezterm、Wine） | `MODULE.bazel`（26 KB）、`defs.bzl`（32 KB）、`.bazelrc`（13 KB）、`MODULE.bazel.lock`（1.65 MB）、`patches/`（26 个补丁） |
| **Cargo**       | Rust workspace 日常构建与测试             | `codex-rs/Cargo.toml`                                                                                                      |
| **just**        | 任务入口（默认工作目录 `codex-rs`）       | `justfile`                                                                                                                 |

本地发布构建的入口不在 `justfile`，而在 `scripts/`（详见
[`../plan/build-and-versioning.md`](../plan/build-and-versioning.md)）：

| 脚本                              | 作用                                                         |
| --------------------------------- | ------------------------------------------------------------ |
| `scripts/build-release-local.sh`  | 官方发布流程 + 构建期版本注入（glibc / musl 均可）           |
| `scripts/setup-rusty-v8.sh`       | 取 Codex 自建 rusty_v8 产物并校验（本地 `cargo build` 必需） |
| `scripts/setup-musl-toolchain.sh` | musl 交叉工具链（Zig + 自编译 libcap + 环境变量）            |

`third_party/` 下内嵌：`v8/`、`wezterm/`、`powershell/`、`wine/`、`voice/`。
`patches/` 里 26 个补丁几乎全部服务于让这些依赖在 Bazel + 多平台上可构建
（`rules_rust_windows_msvc_*.patch`、`v8_bazel_rules.patch`、
`webrtc-sys_hermetic_darwin_sysroot.patch` 等）。

### 常用 justfile 目标（已核对存在）

| 命令                               | 实际执行                                   | 位置                    |
| ---------------------------------- | ------------------------------------------ | ----------------------- |
| `just fmt`                         | 格式化                                     | `justfile:50`           |
| `just fmt-check`                   | 格式检查                                   | `justfile:54`           |
| `just fix -p <crate>`              | `cargo clippy --fix --tests --allow-dirty` | `justfile:57`           |
| `just test`                        | `cargo nextest run --no-fail-fast`         | `justfile:87`           |
| `just test -p codex-tui`           | 同上，限单个 crate                         | `justfile:87`           |
| `just bazel-test` / `bazel-clippy` | Bazel 侧检查                               | `justfile:156` / `:161` |

`just test` 依赖 `cargo-nextest`（`justfile:83` 注释给出安装命令）。

## Node.js 与分发

| 项                               | 值                                              |
| -------------------------------- | ----------------------------------------------- |
| npm 包名                         | `@openai/codex`                                 |
| 入口                             | `codex-cli/bin/codex.js`（8,790 B，ESM 启动壳） |
| 包的 `engines.node`              | `>=16`（仅壳）                                  |
| 仓库 `package.json` 的 `engines` | `node >=22`、`pnpm >=10.34.5`（开发用）         |
| 包管理器                         | `pnpm@10.34.5`（锁定在 `packageManager` 字段）  |
| 根 `package.json` 角色           | 仓库维护工具（prettier），非发布包              |

真正的实现是 `codex-rs` 编译出的 native binary；`codex.js` 只负责转交。
本机安装的 `@openai/codex@0.154.0` 与该文件大小一致。

## 与本地化相关的依赖（已存在，可直接复用）

| 依赖              | 版本    | 说明                                  |
| ----------------- | ------- | ------------------------------------- |
| `sys-locale`      | `0.3.2` | 探测系统语言（`codex-rs/Cargo.toml`） |
| `icu_locale_core` | `2.1`   | ICU locale 核心                       |

**注意**：仓库里**没有任何** `i18n` / `l10n` 模块，`.rs` 文件中
`i18n`、`l10n` 命中数为 **0**。更反直觉的是，`unified_exec` 会**刻意**把子进程的
`LANG` / `LC_ALL` 固定为 `C.UTF-8`（`codex-rs/core/src/unified_exec/process_manager.rs:93`），
目的是让 shell 工具输出稳定可解析，与界面本地化无关——改这块会破坏输出稳定性。

## 本机实测环境

| 组件                | 版本                                    |
| ------------------- | --------------------------------------- |
| OS                  | Ubuntu 26.4.0 (resolute) / linux-x86_64 |
| Node                | v24.21.0                                |
| npm                 | 11.19.0                                 |
| cargo               | 1.96.0                                  |
| rustc               | 1.96.0                                  |
| codex CLI（已安装） | 0.154.0                                 |
| locale              | `zh_CN.UTF-8`                           |

> 本机 `rustc` 是 1.96.0，而仓库 `rust-toolchain.toml` 固定 1.95.0——
> 用 `rustup` 时会自动切换到 1.95.0。

## CI 环境

`.github/workflows/` 的 runner 矩阵以 `ubuntu-latest`（39 次）和
`ubuntu-24.04`（15 次）为主，另有 `macos-15-xlarge`（2 次）。
由于用的都是**当前版本** runner，无法据此验证官方声明的**最低**支持版本
（该声明写在 [../guides/install.md](../guides/install.md)，源码中无对应常量）。
