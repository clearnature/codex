# GitNexus 使用说明

本目录的项目地图、架构分析、技术栈三份文档的数据来自 **GitNexus 知识图谱**。
本文记录它的调用方式、图谱 schema、查询范式，以及在**本机**使用时的注意点。

上游：<https://github.com/abhigyanpatwari/GitNexus>（PolyForm Noncommercial 许可）
本机版本：**1.6.6**

## 一、本机位置与调用

**没有全局安装**，直接使用 `/data/work/GitNexus` 里的构建产物：

```bash
GX=/data/work/GitNexus/gitnexus/dist/cli/index.js
node $GX --help
node $GX doctor        # 环境诊断
```

> **不要用 `npx gitnexus@latest`。** 那会拉取最新版，版本漂移会让产出与
> 本目录文档记录的统计数字不一致。

`doctor` 在本机的输出（用于确认环境可用）：

| 项          | 状态                       |
| ----------- | -------------------------- |
| 图存储      | available                  |
| 全文搜索    | available                  |
| 向量索引    | available                  |
| native 模块 | `lbugjs.node` 已加载       |
| ONNX        | 1.25.1                     |
| 嵌入后端    | local（设备 auto、4 线程） |

## 二、索引本仓库

```bash
node $GX analyze /data/training/cli/codex --index-only --name codex-i18n
```

### `--index-only` 是关键

默认情况下 GitNexus 会往目标仓库**注入**内容（`AGENTS.md`、`CLAUDE.md`、
`.claude/skills/`）。`--index-only` 跳过全部文件注入，保证**目标仓库不被改动**。
本次索引后已验证 `AGENTS.md` / `CLAUDE.md` 未被修改。

### 耗时与规模参考

本仓库实测：3,792 个 `.rs` 文件、361 MB → **493.1 秒**。

### 常用参数

| 参数                   | 作用                                      |
| ---------------------- | ----------------------------------------- |
| `-f, --force`          | 即使已是最新也强制重建                    |
| `--index-only`         | 纯索引，跳过所有文件注入                  |
| `--name <alias>`       | 注册自定义别名（本仓库用了 `codex-i18n`） |
| `--workers <n>`        | 解析并发数（默认 `cores-1`，上限 16）     |
| `--max-file-size <kb>` | 跳过超大文件（默认 512，硬上限 32768）    |
| `--embeddings [limit]` | 生成向量嵌入（**默认关闭**，更慢）        |
| `--skip-git`           | 把给定路径当作索引根，不向上查找 git 根   |

## 三、图谱 schema

理解 schema 是有效查询的前提。以下是本仓库索引的实测结果。

### 节点标签

| 标签       |   数量 | 标签        |  数量 |
| ---------- | -----: | ----------- | ----: |
| `Function` | 50,611 | `Community` | 4,399 |
| `Property` | 24,696 | `Module`    | 4,089 |
| `Struct`   |  6,048 | `Impl`      | 2,549 |
| `File`     |  5,968 | `Enum`      | 1,948 |
| `Const`    |  4,436 | `Process`   |   300 |

其余：`Section` 788、`Folder` 692、`Variable` 524、`TypeAlias` 411、`Method` 399、
`Static` 151、`Trait` 145、`Class` 118、`Macro` 48。

### 边：只有一种类型，语义在属性里

**所有边都是 `CodeRelation`**（312,008 条）。区分语义要靠 `r.type` 属性：

| `r.type`       |    数量 |     | `r.type`            |  数量 |
| -------------- | ------: | --- | ------------------- | ----: |
| `CALLS`        | 109,590 |     | `HAS_METHOD`        | 3,793 |
| `ACCESSES`     |  70,245 |     | `METHOD_IMPLEMENTS` | 2,075 |
| `DEFINES`      |  54,055 |     | `STEP_IN_PROCESS`   | 1,554 |
| `MEMBER_OF`    |  32,510 |     | `IMPLEMENTS`        |   876 |
| `HAS_PROPERTY` |  22,598 |     | `USES`              |   378 |
| `CONTAINS`     |   7,424 |     | `EXTENDS`           |    12 |
| `IMPORTS`      |   6,898 |     |                     |       |

边的属性：`type`、`confidence`、`reason`、`step`。

### 关键节点类型的属性

| 标签        | 属性                                                                                                     |
| ----------- | -------------------------------------------------------------------------------------------------------- |
| `Process`   | `id`、`label`、`heuristicLabel`、`processType`、`stepCount`、`communities`、`entryPointId`、`terminalId` |
| `Community` | `id`、`label`、`heuristicLabel`、`keywords`、`description`、`enrichedBy`、`cohesion`、`symbolCount`      |

## 四、Cypher 方言的两个坑

LadybugDB 的 Cypher 与 Neo4j **不完全兼容**，实测踩到两处：

| 写法        | 结果                                                            |
| ----------- | --------------------------------------------------------------- |
| `labels(n)` | ✅ 可用                                                         |
| `type(r)`   | ❌ 报 `Catalog exception: function TYPE does not exist`         |
| `r.type`    | ✅ 用这个取边语义                                               |
| `label(r)`  | ⚠️ 返回的是节点/边标签（对边恒为 `CodeRelation`），不是语义类型 |

另一个坑：**本机已索引多个仓库**（`rtl-sdr`、`DeepSeek-Reasonix`、`putty`、
`claude-code`、`flash-attention`、`t0-gpu`、`codex-i18n`），
查询时**必须带 `-r codex-i18n`**，否则报
`Multiple repositories indexed. Specify which one with the "repo" parameter`。

## 五、查询命令

### `cypher` —— 最灵活

```bash
# 节点标签分布
node $GX cypher -r codex-i18n \
  "MATCH (n) RETURN labels(n) AS l, count(*) AS c ORDER BY c DESC"

# 边语义分布（注意用 r.type，不是 type(r)）
node $GX cypher -r codex-i18n \
  "MATCH ()-[r]->() RETURN r.type AS t, count(*) AS c ORDER BY c DESC"

# 执行流清单
node $GX cypher -r codex-i18n \
  "MATCH (p:Process) RETURN p.label AS label, p.stepCount AS steps ORDER BY steps DESC LIMIT 20"

# 功能聚类
node $GX cypher -r codex-i18n \
  "MATCH (c:Community) RETURN c.label AS label, c.symbolCount AS syms ORDER BY syms DESC LIMIT 20"
```

输出是 JSON，其中 `markdown` 字段已是渲染好的表格，`row_count` 是行数。

### 其余查询命令

| 命令                         | 用途                                                                                |
| ---------------------------- | ----------------------------------------------------------------------------------- |
| `context -r <repo> <symbol>` | 符号 360° 视图：调用者、被调用者、所属流程（`-u <uid>` 可消歧、`--content` 带源码） |
| `query -r <repo> "<概念>"`   | 语义搜索执行流（`-g <goal>`、`-c <context>` 提升排序）                              |
| `impact -r <repo> <target>`  | 影响面分析：改这个符号会影响什么                                                    |

## 六、其余子命令

| 命令              | 用途                                                | 注意                                                                                       |
| ----------------- | --------------------------------------------------- | ------------------------------------------------------------------------------------------ |
| `status`          | 当前仓库的索引状态                                  | 会显示「已是最新」或落后                                                                   |
| `list`            | 列出所有已索引仓库                                  | 本机 7 个                                                                                  |
| `doctor`          | 运行平台能力诊断                                    | —                                                                                          |
| `analyze`         | 索引仓库                                            | 见第二节                                                                                   |
| `index [path...]` | 把已有 `.gitnexus/` 注册到全局注册表（不重新分析）  | —                                                                                          |
| `serve`           | 启动本地 HTTP 服务，供 Web UI 连接                  | —                                                                                          |
| `mcp`             | 启动 MCP server（stdio），供 AI agent 接入          | —                                                                                          |
| `setup`           | 为 Cursor / Claude Code / OpenCode / Codex 配置 MCP | —                                                                                          |
| `wiki [path]`     | 从图谱生成仓库 Wiki                                 | **会调用 LLM，产生费用**，需 `--provider` / `--model` / `--api-key`；支持 `--lang chinese` |
| `clean`           | 删除当前仓库的 GitNexus 索引                        | —                                                                                          |
| `remove <target>` | 删除已注册仓库的索引（可在仓库外执行）              | —                                                                                          |
| `augment`         | 用图谱上下文增强搜索（供 hooks 使用）               | —                                                                                          |

> **`wiki` 是本仓库唯一会调用外部 LLM 的命令。** 本次生成 `docs/maps/` 的文档时
> **没有**使用它——所有文档均由人工撰写，数据来自 `cypher` / `context` 查询与源码核对。

## 七、索引产物与磁盘占用

| 项              | 值                                                                                                 |
| --------------- | -------------------------------------------------------------------------------------------------- |
| 位置            | `<repo>/.gitnexus/`                                                                                |
| 本仓库实测      | **1.4 GB**（其中 `lbug` 数据库 789 MB）                                                            |
| 是否被 git 跟踪 | **否**——它自带 `.gitignore`（内容为 `*`），且 `analyze` 会往 `.git/info/exclude` 写入 `.gitnexus/` |
| 删除方式        | `node $GX clean`                                                                                   |

正因为有双重忽略，**不需要**往仓库的 `.gitignore` 里添加规则。

## 八、使用注意

1. **不要目测估计，要数。** 图谱里没有现成的「crate 依赖计数」查询，
   `Cargo.toml` 的依赖数要用 `grep -c` 统计。本次撰写 `architecture.md` 时曾按
   目测写入 5 个数字，交付前 grep 复核发现全部偏差（47→48、50→49、56→60、
   70+→73、31→26），已修正。**凡是要写进文档的数字，都实测一次。**
2. **`Community.keywords` 为空**表示聚类未做语义增强（需要 `--embeddings` 或 LLM）。
   此时聚类标签实际上来自目录名，不代表语义分区。
3. **图谱是提交点快照。** 本次索引提交是 `85f4d67`；代码再变动后需重新 `analyze`
   才会反映新状态（`status` 会提示）。
4. **索引有文件大小上限**，本仓库跳过了 2 个 >512KB 的 schema JSON
   （`codex-rs/app-server-protocol/schema/json/*.schemas.json`），
   统计数字不含它们的内部结构。
5. **一次 `analyze` 约 8 分钟**（本仓库规模），期间 worker 峰值 CPU 231%、
   RSS 约 7.4 GB。请预留资源，不要并发跑构建。

## 九、本次索引结果备查

| 项          | 值                         |
| ----------- | -------------------------- |
| 仓库        | `/data/training/cli/codex` |
| 别名        | `codex-i18n`               |
| 索引提交    | `85f4d67`                  |
| 索引时间    | 北京时间 2026-09-15 06:59  |
| 耗时        | 493.1 秒                   |
| 节点 / 边   | 108,320 / 312,008          |
| 聚类 / 流程 | 4,777 / 300                |
