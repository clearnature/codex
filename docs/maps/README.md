# Codex 项目地图

> **定位**：面向本 fork 开发者的内部参考，不是面向用户的文档。
> Codex 的用户文档在 [developers.openai.com/codex](https://developers.openai.com/codex)，
> 仓库里对应的指路牌在 [`../guides/`](../guides/)。
> 根目录 `AGENTS.md` 要求不要把用户文档放进 `docs/`，本目录是开发者内部资料，不属该禁令范围。

## 本目录内容

| 文件 | 内容 | 主要依据 |
| --- | --- | --- |
| [project-map.md](./project-map.md) | 仓库布局、workspace 成员分区、文件规模 | 目录结构 + GitNexus 知识图谱 |
| [architecture.md](./architecture.md) | 分层架构、依赖方向、关键执行流 | 各 crate 的 `Cargo.toml` 实测依赖 + 图谱 |
| [tech-stack.md](./tech-stack.md) | 语言、构建系统、工具链、关键依赖 | 仓库配置 + 本机实测 |
| [references.md](./references.md) | 关键文件索引、命令速查、外部资料、i18n 落点 | 实测可用性 |
| [gitnexus.md](./gitnexus.md) | **GitNexus 使用说明**：调用方式、图谱 schema、查询范式与已知坑 | 本机实测 |

## 基线

本目录描述的是 `feat/i18n` 分支上的 **`rust-v0.154.0`** 代码：

| 项 | 值 |
| --- | --- |
| 分支 | `feat/i18n` |
| 基线 commit | `6b9826e3aa83b1a5947db50f4332cb9c65f1b340` |
| 对应上游 tag | `rust-v0.154.0`（北京时间 2026-09-10 06:35 发布） |
| fork 远端 | `clearnature/codex` |
| upstream | `openai/codex` |

> **分支约定**：`main` 用于镜像上游 `openai/codex`，开发发生在 `feat/i18n`。
> 本文档描述的内容若与 `main` 不一致，以 `feat/i18n` 为准。

## 一句话概览

Codex CLI 是一个 **Rust workspace 单体仓库**：核心逻辑全部在 `codex-rs/`，
`codex-cli/` 里的 npm 包只是启动壳；
TypeScript 用于**代码生成**的协议绑定，Python 提供独立的 SDK。

## 结构统计（GitNexus 知识图谱）

索引时间：北京时间 2026-09-15 06:59 · 索引提交 `85f4d67` · 耗时 493.1s

| 指标 | 数值 |
| --- | --- |
| 节点 | 108,320 |
| 边 | 312,008 |
| 聚类 | 4,777 |
| 执行流 | 300 |

**节点构成**（前 10 类）：

| 标签 | 数量 | | 标签 | 数量 |
| --- | ---: | --- | --- | ---: |
| `Function` | 50,611 | | `Community` | 4,399 |
| `Property` | 24,696 | | `Module` | 4,089 |
| `Struct` | 6,048 | | `Impl` | 2,549 |
| `File` | 5,968 | | `Enum` | 1,948 |
| `Const` | 4,436 | | `Process` | 300 |

**边构成**（全部关系类型）：

| 类型 | 数量 | 类型 | 数量 |
| --- | ---: | --- | ---: |
| `CALLS` | 109,590 | `HAS_METHOD` | 3,793 |
| `ACCESSES` | 70,245 | `METHOD_IMPLEMENTS` | 2,075 |
| `DEFINES` | 54,055 | `STEP_IN_PROCESS` | 1,554 |
| `MEMBER_OF` | 32,510 | `IMPLEMENTS` | 876 |
| `HAS_PROPERTY` | 22,598 | `USES` | 378 |
| `CONTAINS` | 7,424 | `EXTENDS` | 12 |
| `IMPORTS` | 6,898 | | |

## 复现方式

本目录的统计数字由本机 GitNexus 索引生成。**完整用法、图谱 schema、
Cypher 方言差异与已知坑见 [gitnexus.md](./gitnexus.md)**，最简形式：

```bash
GX=/data/work/GitNexus/gitnexus/dist/cli/index.js

# 索引：--index-only 避免往目标仓库注入 AGENTS.md / CLAUDE.md / skills
node $GX analyze /data/training/cli/codex --index-only --name codex-i18n

# 查询（本机索引了多个仓库，必须带 -r）
node $GX cypher -r codex-i18n "MATCH (n) RETURN labels(n) AS l, count(*) AS c ORDER BY c DESC"
```

> 图谱的 `Community.keywords` 为空数组，说明聚类未做语义增强
> （需要 `--embeddings` 或 LLM）。因此聚类标签实际来自目录名，
> 语义层面的分区请以 [project-map.md](./project-map.md) 的人工归类为准。
