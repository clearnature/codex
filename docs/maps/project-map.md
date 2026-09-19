# 项目地图

## 顶层布局

| 目录 / 文件                                            | 作用                                               | 备注                                                              |
| ------------------------------------------------------ | -------------------------------------------------- | ----------------------------------------------------------------- |
| `codex-rs/`                                            | **核心**：Rust workspace                           | `Cargo.toml` 有 145 个成员条目；`rust-toolchain.toml` 固定工具链  |
| `codex-cli/`                                           | npm 分发包 `@openai/codex`                         | 仅启动壳，`bin/codex.js` 8,790 B                                  |
| `sdk/`                                                 | 语言 SDK                                           | `sdk/python/`、`sdk/typescript/`                                  |
| `docs/`                                                | 开发者文档                                         | `guides/`（指路牌）、`policies/`（贡献与法律）、`maps/`（本目录） |
| `bazel/` + `BUILD.bazel` + `MODULE.bazel` + `defs.bzl` | Bazel 构建定义                                     | `MODULE.bazel.lock` 达 1.65 MB                                    |
| `third_party/`                                         | 内嵌重型依赖                                       | `v8/`、`wezterm/`、`powershell/`、`wine/`、`voice/`               |
| `patches/`                                             | 26 个 Bazel / 原生依赖补丁                         | 以 `rules_rust_*`、`v8_*`、`*_windows_*` 为主                     |
| `scripts/`                                             | 维护脚本（Python / shell）                         | `codex_package/`、`mcp_conformance/`                              |
| `tools/`                                               | 构建与开发辅助                                     | —                                                                 |
| `.github/`                                             | CI、issue 模板、CLA 流程                           | `workflows/cla.yml`、`ISSUE_TEMPLATE/`                            |
| 根目录文件                                             | `AGENTS.md`、`justfile`、`package.json`、`LICENSE` | `AGENTS.md` 是仓库约定，动代码前必读                              |

## 文件规模

| 指标          |        数值 | 口径                                        |
| ------------- | ----------: | ------------------------------------------- |
| `.rs` 文件    |       3,792 | 仅 `codex-rs/`                              |
| `.rs` 文件    |       4,156 | 全仓库（差额来自 `third_party/` 等）        |
| Rust 代码行数 | ≈ 1,741,939 | 全仓 `.rs` 累计                             |
| 仓库体积      |      361 MB | 不含 `.gitnexus/` 索引                      |
| `.snap` 快照  |   877 / 940 | `tui/` / 全仓 — **i18n 改造的主要回归成本** |
| 提示词 `.md`  |    1,732 行 | `codex-rs/core/` 下                         |

## `codex-rs/` 成员分区

> 依据：`Cargo.toml` 的 145 个成员条目，按命名与实测依赖人工归类。
> 这是**分析结果**，不是官方分层声明。

### 入口与前端

`cli`（主二进制 `codex`）· `tui`（交互式界面）· `exec`（非交互 `codex exec`）· `arg0` · `install-context` · `build-info`

### 服务与协议

`app-server` · `app-server-client` · `app-server-daemon` · `app-server-transport` · `app-server-protocol` · `app-server-protocol-noop-macros` · `app-server-test-client` · `protocol` · `exec-server` · `exec-server-protocol`

### 核心

`core` · `core-api` · `core-plugins` · `config` · `config-schema` · `features` · `state` · `thread-store` · `rollout` · `rollout-trace` · `history` · `context-fragments` · `prompts` · `models-manager` · `model-provider` · `model-provider-info` · `responses-api-proxy`

### 工具与扩展

`tools` · `apply-patch` · `execpolicy` · `skills` · `plugin` · `connectors` · `rmcp-client` · `codex-mcp` · `hooks` · `collaboration-mode-templates` · `code-mode`（+ `-host` / `-protocol` / `-runtime`，基于 V8）

`ext/`（14 个扩展点）：`agent` · `connectors` · `extension-api` · `git-attribution` · `goal` · `guardian-v2` · `history-notes` · `image-generation` · `items` · `mcp` · `memories` · `queue` · `skills` · `web-search`

### 沙箱与安全

`sandboxing` · `linux-sandbox` · `mxc-sandbox` · `windows-sandbox-service` · `bwrap` · `secrets` · `keyring-store` · `network-proxy` · `process-hardening` · `shell-command` · `shell-escalation` · `guardian-context` · `workload-identity` · `aws-auth`

### 多智能体

`agent-graph-store` · `agent-identity` · `agent-roles` · `external-agent-migration` · `thread-manager-sample`

### 模型接入与实时通信

`lmstudio` · `ollama` · `realtime-webrtc` · `websocket-client` · `http-client`

### 存储、记忆与文件

`memories/read` · `memories/write` · `attachment-store` · `file-system` · `file-watcher` · `file-search` · `git-utils` · `worktree` · `codex-home`

### 终端与 IO

`terminal-detection` · `stdio-to-uds` · `uds` · `ansi-escape` · `utils/pty` · `utils/stream-parser`

### 可观测性与账号

`otel` · `otel-trace-websocket` · `analytics` · `diagnostics` · `feedback` · `login` · `cloud-config` · `cloud-tasks`（+ `-client` / `-mock-client`） · `backend-client` · `codex-backend-openapi-models` · `codex-api` · `codex-client` · `codex-experimental-api-macros`

### 语音

`voice-host` · `utils/audio`

### `utils/`（26 个基础设施小 crate）

`absolute-path` · `approval-presets` · `audio` · `cache` · `cargo-bin` · `cli` · `elapsed` · `fuzzy-match` · `git-discovery` · `home-dir` · `image` · `json-to-toml` · `oss` · `output-truncation` · `path-uri` · `path-utils` · `plugins` · `pty` · `readiness` · `redacted-string` · `rustls-provider` · `sandbox-summary` · `sleep-inhibitor` · `stream-parser` · `string` · `template`

### 其他

`async-utils` · `test-binary-support` · `v8-poc` · `file-watcher` · `network-proxy`

## 对 i18n 开发的含义

界面文案集中在**入口与前端**分区（`tui` 12,681 条、`cli` 1,929 条），
而提示词资产在 `core`（`core/*.md` 与 `core/templates/`）。
两者分属不同分区、且 `tui` 与 `core` 之间隔着协议边界——改造时不能按单一目录推进。
详见 [references.md](./references.md#i18n-落点)。
