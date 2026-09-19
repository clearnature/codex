# 参考资料

## 关键文件索引

| 文件                                                                                | 作用                                                                 |
| ----------------------------------------------------------------------------------- | -------------------------------------------------------------------- |
| [`AGENTS.md`](../../AGENTS.md)                                                      | **仓库约定，动代码前必读**（含「不要把用户文档放进 `docs/`」等约束） |
| [`justfile`](../../justfile)                                                        | 任务入口，默认工作目录 `codex-rs`                                    |
| [`codex-rs/Cargo.toml`](../../codex-rs/Cargo.toml)                                  | workspace 定义（145 个成员）、edition、共享依赖版本                  |
| [`codex-rs/rust-toolchain.toml`](../../codex-rs/rust-toolchain.toml)                | 固定 Rust 1.95.0                                                     |
| [`package.json`](../../package.json)                                                | 仓库维护脚本 + prettier glob                                         |
| [`MODULE.bazel`](../../MODULE.bazel) / [`defs.bzl`](../../defs.bzl)                 | Bazel 构建定义                                                       |
| [`codex-rs/app-server-protocol/schema/`](../../codex-rs/app-server-protocol/schema) | 协议 schema（JSON + 生成的 TypeScript 绑定）                         |
| [`../guides/`](../guides/)                                                          | 指向官方用户文档的指路牌                                             |
| [`../policies/`](../policies/)                                                      | 贡献指南、CLA、许可                                                  |

## 命令速查

### 构建与测试（在仓库根执行）

```bash
just fmt                        # 格式化
just fix -p codex-tui           # 单 crate 的 clippy --fix
just test -p codex-tui          # 单 crate 测试（nextest）
just test                       # 全量测试
cargo build                     # 纯 Cargo 构建
cargo run --bin codex -- "prompt"
```

### 本地发布构建（本 fork 新增脚本）

```bash
# glibc release，版本号由脚本从最近的 rust-v* tag 派生（工作区保持 0.0.0）
scripts/build-release-local.sh
scripts/build-release-local.sh --version 0.154.0

# musl release（官方 Linux 发行目标，静态链接）
scripts/setup-musl-toolchain.sh
set -a; . /tmp/codex-musl-env-x86_64-unknown-linux-musl.sh; set +a
scripts/build-release-local.sh x86_64-unknown-linux-musl
```

产物在 `codex-rs/target/<target>/release/`（`codex`、`codex-code-mode-host`、
`codex-responses-api-proxy`、`bwrap`）。原理、环境依赖与踩坑记录见
[`../plan/build-and-versioning.md`](../plan/build-and-versioning.md)。

### 运行时诊断（已安装的 codex CLI）

```bash
codex doctor                    # 环境、认证、运行时健康检查
codex debug models              # 打印生效的模型目录（JSON）
codex debug prompt-input        # 打印实际送给模型的输入
codex --version
```

`codex debug models` 是验证模型/catalog 配置的权威手段——它输出的是 Codex
**解析并规范化后**的目录，与该文件在磁盘上的原文可能不同（例如
`shell_type: "shell_command"` 会被渲染为 `"unified_exec"`，若干 `null` 字段不回显）。

### GitNexus 图谱查询

```bash
GX=/data/work/GitNexus/gitnexus/dist/cli/index.js

node $GX status
node $GX cypher -r codex-i18n "MATCH (n) RETURN labels(n) AS l, count(*) AS c ORDER BY c DESC"
node $GX cypher -r codex-i18n "MATCH ()-[r]->() RETURN r.type AS t, count(*) AS c ORDER BY c DESC"
node $GX context -r codex-i18n <symbol>
node $GX query   -r codex-i18n "<概念>"
node $GX impact  -r codex-i18n <symbol>
```

> GitNexus 的边统一为 `CodeRelation`，语义靠 `r.type` 属性区分；
> Cypher 方言**不支持** `type(r)` 与 `label(n)` 之外的 `labels()` 混用写法——
> 已验证可用的是 `labels(n)`（节点）与 `r.type`（边）。

## 外部资料

| 资源                         | 地址                                                           |
| ---------------------------- | -------------------------------------------------------------- |
| 官方用户文档                 | https://developers.openai.com/codex                            |
| 上游仓库                     | https://github.com/openai/codex                                |
| 本 fork                      | https://github.com/clearnature/codex                           |
| Releases（含 DotSlash 文件） | https://github.com/openai/codex/releases                       |
| DotSlash                     | https://dotslash-cli.com/                                      |
| `RUST_LOG` 配置              | https://docs.rs/env_logger/latest/env_logger/#enabling-logging |
| Rust API 文档                | https://docs.rs/codex-*（各 crate 独立发布时）                 |

## i18n 落点

本次开发的直接目标。以下数字来自启发式扫描（统计含空格、像句子的字符串字面量，
含测试代码，属**上界**）：

| 层                | 位置                                | 疑似条数 | 是否本地化                                 |
| ----------------- | ----------------------------------- | -------: | ------------------------------------------ |
| TUI 渲染          | `codex-rs/tui/src`                  |   12,681 | 要                                         |
| CLI 入口 / doctor | `codex-rs/cli/src`                  |    1,929 | 要                                         |
| core 用户可见消息 | `codex-rs/core/src`                 |    8,112 | **部分**（含日志、遥测、喂模型的工具描述） |
| 提示词资产        | `codex-rs/core/*.md` + `templates/` | 1,732 行 | **不要**（喂模型，翻译会改变行为）         |
| 协议层            | `app-server*/` + `protocol/`        |    2,891 | 只翻内容，**不翻字段名**                   |

TUI 内最密集的目录：`tui/src`（2,960）、`bottom_pane`（1,776）、`app`（1,611）、
`chatwidget`（755）、`history_cell`（690）。

### 三个硬约束

1. **877 个界面快照**（`tui/src/chatwidget/snapshots` 275、`bottom_pane/snapshots` 225、
   `tui/src/snapshots` 156、`history_cell/snapshots` 57……；另有 146 个测试文件）。
   因此**默认 locale（en）的输出必须与现状逐字节一致**，否则快照会成批失败。
2. **文案没有集中常量**：整个 `tui` 只有 `version.rs` 一个 `pub const ...: &str`，
   文案内联散落在 632 个文件的渲染函数里。
3. **`tui` 与 `core` 隔着协议边界**（见 [architecture.md](./architecture.md)），
   两侧文案要分别处理。

### 尚未存在的东西

- 仓库内 **0 个** `.rs` 文件包含 `i18n` / `l10n`
- 全仓 **0 处** `localeOverride` / `locale_override`——issue 里提到的
  `[desktop] localeOverride` 只被闭源桌面端读取，CLI 不认
- 上游提交历史里**没有** i18n / 本地化相关的工作

## 上游需求背景

| Issue                                                  | 标题                                                                                   | 状态 |
| ------------------------------------------------------ | -------------------------------------------------------------------------------------- | ---- |
| [#30025](https://github.com/openai/codex/issues/30025) | `[Feature Request] Add i18n/L10n support for Chinese (Simplified) and other languages` | open |
| [#30060](https://github.com/openai/codex/issues/30060) | `Add Chinese UI Localization Support`                                                  | open |
| [#30421](https://github.com/openai/codex/issues/30421) | `[Feature Request] Support Chinese (zh-CN) UI localization / i18n`                     | open |

三个请求均自 2026-06 起挂 open，评论 1–3 条，无官方推进。
社区现有方案（`xqnode/codex-zh-CN` 等）都是给**闭源桌面端**打汉化补丁，
没有一个是在 `codex-rs` 里做源码级 i18n——本分支要做的是第一个。
