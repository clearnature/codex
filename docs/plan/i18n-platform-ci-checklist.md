# 平台受限改动的 CI 复查清单

本机（只有 linux target，且依赖的 C 构建挡住跨目标 check）无法编译这些文件，
因此它们**只能由 macOS/Windows CI 复核**。核对方式：对每一行，看 `file:line` 的调用形态与其实参表达式是否与源码声明类型相符；
逐站点依据见 `docs/plan/i18n-specs/`（这些规格就是实际执行过的输入）。

| 文件:行 | 种类 | 站点实参（`tr_with` 的第 3 参） | 文档 |
| --- | --- | --- | --- |
| `codex-rs/cli/src/debug_sandbox.rs:92` | `bail` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:218` | `anyhow` | `&err.to_string()` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:230` | `context` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:279` | `bail` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:305` | `context` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:336` | `bail` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:402` | `bail` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:466` | `eprintln` | `（无参）` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:501` | `eprintln` | `&err.to_string()` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:533` | `eprintln` | `&err.to_string()` | §12.66 |
| `codex-rs/cli/src/debug_sandbox.rs:400` | `register` | `Seatbelt(SBPL) 策略语法行：`policy.push_str("\n(deny file-ioctl (i` | §12.66 |
| `codex-rs/cli/src/desktop_app/windows.rs:17` | `eprintln` | `display_workspace.as_str()` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:22` | `eprintln` | `（无参）` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:29` | `eprintln` | `display_workspace.as_str()` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:43` | `context` | `（无参）` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:60` | `format` | `url` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:65` | `bail` | `url, &status.to_string()` | §12.67 |
| `codex-rs/cli/src/desktop_app/windows.rs:39` | `register` | `PowerShell 脚本文本（`Get-StartApps | Where-Object AppID -Like …`` | §12.67 |

合计 **18 处**（mac 33 译 + windows 6 译 + debug_sandbox 4 处平台分支 + 登记行）。

复核要点（按出错概率排序）：① `String` 用 `x.as_str()`、`&str` 用裸标识符、`Display` 表达式用 `&x.to_string()`（本仓 `-D needless-borrow` / `-D redundant-clone` / `-D uninlined-format-args` 都当错误）；
② `{0:?}` 这种 Debug 形参必须写成 `&format!("{_0:?}")`（内联格式参数）；③ 常量作实参（如 `std::env::consts::OS`）直接传 `&'static str`。
