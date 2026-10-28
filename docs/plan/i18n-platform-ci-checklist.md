# 平台受限改动的 CI 复查清单

本机只有 `x86_64-unknown-linux-{gnu,musl}`，且跨目标 `cargo check` 被依赖的 C 构建挡住（见 known_issues `no-cross-target-typecheck-local`）
⇒ 下列站点**只能由 macOS/Windows CI 复核**。核对方式：看 `file:line` 的调用形态与实参表达式是否与源码声明类型相符；
逐站点依据就是 `docs/plan/i18n-specs/` 里**实际执行过**的规格。

**平台受限站点合计 45 个**（其中 **43 处是代码改动**：mac.rs 33 + windows.rs 6 + debug_sandbox 的 2 处 macOS 分支与 2 处 Windows 分支；另 2 个是登记行）。本表列出三个相关文件的全部 52 个站点以便对照 —— **注意**：`debug_sandbox.rs` 的另外 10 处在 Linux 编译路径上，已由 clippy/测试覆盖，不在「未验证」之列。

> **✅ 已闭合（2026-10-28）**：43 处平台受限站点已由平台 CI 实测复核。载体为个人版
> workflow `.github/workflows/macos-i18n-verify.yml`——标准 runner（macos-15 双
> target + windows-2022，个人 public 仓库免费）上跑 `cargo clippy -p codex-cli
> --all-targets --target <t> -- -D warnings`（脚本 `scripts/i18n-macos-verify.sh`），
> 纯 cargo，不依赖 Bazel / 自托管 runner / v8 预编译产物。终验运行
> [40667069857](https://github.com/clearnature/codex/actions/runs/40667069857)
> （HEAD `261d7a5cc`）三 job 全绿；cargo home + sccache 两层缓存按 `Cargo.lock`
> 哈希做 key。复核过程逐层（core → tui → cli lib → cli bin）抓出并修复 5 处
> Linux 路径永远编译不到的门控缺陷：
> ① `tui/src/app/history_ui.rs:377` 多余 to_string（`4ad2ddc2e`）；
> ② `core/src/windows_sandbox.rs:9` Windows 路径 unused import，import 加
> `not(windows)` cfg（`d514d54ef`）；
> ③ `tui/src/app/config_persistence.rs` ×3 `tr_with(...).to_string()` 冗余克隆
> （`2d6b29c8a`）；
> ④ `cli/src/debug_sandbox.rs:525` Windows 分支冗余克隆（err 已是 String，
> `2588586a7`）；
> ⑤ `cli/src/main.rs:1020/:1029` `&[&command]` 多余借用（`261d7a5cc`）。
> **测试回执（2026-10-28 · Ubuntu · 1.95.0 同源工具链）**：tui lib 全量
> `RUST_MIN_STACK=16777216 … --skip ide_context::ipc` → **4287 passed / 2 failed /
> snapshot 失败 0**（`.snap.new` = 0）；2 条失败（`safety_buffering::
> agents_overview_acknowledges_inactive_steer_before_interrupt`、`rollout_history::
> cached_legacy_resume_revalidates_history_across_migration_settings`）均在 §「两条失败的
> 归属」历史 flaky 集合内（round 28/29 已记录），单跑双双通过（2.33s）⇒ 负载敏感时序型，
> 与 i18n 无关。
> 第 1–6 轮失败日志留存于仓库 Actions 历史。下表保留备查。

| 文件:行                                      | 平台门控                              | 种类       | 站点实参（`tr_with` 第 3 参）                                                                                                            | 文档              |
| -------------------------------------------- | ------------------------------------- | ---------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ----------------- | ------ |
| `codex-rs/cli/src/debug_sandbox.rs:92`       | Linux 编译路径                        | `bail`     | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:218`      | Linux 编译路径                        | `anyhow`   | `&err.to_string()`                                                                                                                       | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:230`      | Linux 编译路径                        | `context`  | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:279`      | Linux 编译路径                        | `bail`     | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:305`      | Linux 编译路径                        | `context`  | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:336`      | Linux 编译路径                        | `bail`     | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:400`      | Linux 编译路径                        | `register` | `Seatbelt(SBPL) 策略语法行：`policy.push_str("\n(deny f…`                                                                                | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:402`      | **macOS 分支**                        | `bail`     | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:466`      | **macOS 分支**                        | `eprintln` | `（无参）`                                                                                                                               | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:501`      | **Windows 分支**                      | `eprintln` | `&err.to_string()`                                                                                                                       | §12.66            |
| `codex-rs/cli/src/debug_sandbox.rs:533`      | **Windows 分支**                      | `eprintln` | `&err.to_string()`                                                                                                                       | §12.66            |
| `codex-rs/cli/src/desktop_app/mac.rs:20`     | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&app_path.display().to_string()`                                                                                                        | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:26`     | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:37`     | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:39`     | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&installed_app.display().to_string()`                                                                                                   | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:105`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&workspace.display().to_string()`                                                                                                       | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:115`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:122`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&app_path.display().to_string(), url.as_str(), &status.to_string()`                                                                     | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:130`    | **macOS 专属**（整个文件 `#[cfg]`）   | `register` | `代码签名**要求串**（codesign `-R=` 的语法）：`identifier "…" …`                                                                         | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:138`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:145`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&app_path.display().to_string(), OPENAI_APPLE_TEAM_IDENTIFIER, CODEX_BUNDLE_IDENTIFIER, String::from_utf8_lossy(&output.stderr).trim()` | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:163`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:170`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:173`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&mount_point.display().to_string()`                                                                                                     | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:178`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:181`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:189`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&mount_point.display().to_string(), &err.to_string()`                                                                                   | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:200`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&applications_dir.display().to_string()`                                                                                                | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:205`    | **macOS 专属**（整个文件 `#[cfg]`）   | `format`   | `&applications_dir.display().to_string()`                                                                                                | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:219`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `&applications_dir.display().to_string(), &err.to_string()`                                                                              | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:226`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:236`    | **macOS 专属**（整个文件 `#[cfg]`）   | `eprintln` | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:248`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:253`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&status.to_string()`                                                                                                                    | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:264`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:268`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&output.status.to_string(), &String::from_utf8_lossy(&output.stderr)`                                                                   | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:277`    | **macOS 专属**（整个文件 `#[cfg]`）   | `format`   | `&stdout`                                                                                                                                | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:286`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:291`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&status.to_string()`                                                                                                                    | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:302`    | **macOS 专属**（整个文件 `#[cfg]`）   | `format`   | `&mount_point.display().to_string()`                                                                                                     | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:306`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:314`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&mount_point.display().to_string()`                                                                                                     | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:325`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:330`    | **macOS 专属**（整个文件 `#[cfg]`）   | `bail`     | `&status.to_string()`                                                                                                                    | §12.68            |
| `codex-rs/cli/src/desktop_app/mac.rs:334`    | **macOS 专属**（整个文件 `#[cfg]`）   | `context`  | `（无参）`                                                                                                                               | §12.68            |
| `codex-rs/cli/src/desktop_app/windows.rs:17` | **Windows 专属**（整个文件 `#[cfg]`） | `eprintln` | `display_workspace.as_str()`                                                                                                             | §12.67            |
| `codex-rs/cli/src/desktop_app/windows.rs:22` | **Windows 专属**（整个文件 `#[cfg]`） | `eprintln` | `（无参）`                                                                                                                               | §12.67            |
| `codex-rs/cli/src/desktop_app/windows.rs:29` | **Windows 专属**（整个文件 `#[cfg]`） | `eprintln` | `display_workspace.as_str()`                                                                                                             | §12.67            |
| `codex-rs/cli/src/desktop_app/windows.rs:39` | **Windows 专属**（整个文件 `#[cfg]`） | `register` | `PowerShell 脚本文本（`Get-StartApps                                                                                                     | Where-Object Ap…` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:43` | **Windows 专属**（整个文件 `#[cfg]`） | `context`  | `（无参）`                                                                                                                               | §12.67            |
| `codex-rs/cli/src/desktop_app/windows.rs:60` | **Windows 专属**（整个文件 `#[cfg]`） | `format`   | `url`                                                                                                                                    | §12.67            |
| `codex-rs/cli/src/desktop_app/windows.rs:65` | **Windows 专属**（整个文件 `#[cfg]`） | `bail`     | `url, &status.to_string()`                                                                                                               | §12.67            |

**复核要点（按出错概率排序）**：

1. `String` → `x.as_str()`；`&str` → 裸标识符；只有 `Display` 的表达式（`path.display()`）→ `&x.to_string()` —— 本仓 `-D needless-borrow` / `-D redundant-clone` / `-D uninlined-format-args` 都当**错误**；
2. `Debug` 形参（原 `{0:?}`）→ `&format!("{_0:?}")`（内联格式参数，否则撞 `uninlined_format_args`）；
3. 常量实参（`std::env::consts::OS`、`OPENAI_APPLE_TEAM_IDENTIFIER`）直接传 `&'static str`；
4. 语法已由 `fmt-check`（rustfmt 必须解析这些文件）覆盖，文本级由 `i18n-check` 覆盖（`scanned … 330`）——**未覆盖的是类型与运行**。
