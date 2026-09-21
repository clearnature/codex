# `codex-plugins` 520 条的推进方案（第 626 轮实测）

> 实测口径：`python3 scripts/i18n_todo.py --root codex-rs/core-plugins/src` ⇒ **518 条 / 32 个文件**
> （先前口头说的 520 是粗数，以本条为准）。

## 一、先定界：这三类各怎么处理

| 类           | 判据（与 §9.1 同源）                                                                                                             | 处置                                                        |
| ------------ | -------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------- |
| **译**       | 消息**流向用户面**：`CodexErr::*`（含 `Fatal`）、CLI `plugin_cmd` 的 issue 列表、TUI 警告/事件、`#[error]` 类型被 `?` 抛到这些面 | 包 `tr`/`tr_with`（`thiserror` 属性走 §12.70 的属性级改法） |
| **登记**     | 只进日志/遥测/tracing、`--json` 机器契约（与 `doctor` 851 同族）、内部缓存键、喂模型载荷                                         | 记入 `not-translated-unwrapped.tsv`（按值/站点豁免 + 理由） |
| **不属文案** | 机器语法片段：git 参数（实测样本 `git sparse-checkout marketplace source`）、manifest 键名、命令片段、路径模板                   | 同上登记，理由写「机器语法/标识符」（§12.1/§12.7）          |

**关键判断**：这条流水线的**真实成本在「接收者分析」**（每条往上找调用链），不在改写（改写已是工具化）。

## 二、形态分布（518 条的实测形状）

| 形态                       | 条数 | 占比 | 处理提示                                                                                                |
| -------------------------- | ---: | ---: | ------------------------------------------------------------------------------------------------------- |
| `format!` / `.to_string()` |  343 |  66% | 多为错误消息构造 ⇒ 需看**所在函数**抛给谁；同函数内的一条分析可覆盖多条                                 |
| 「其它」（无宏包裹）       |  106 |  20% | 抽样多为 **token/标签/命令片段**（`install remote plugin`、`missing url`）与结构体字段 ⇒ 多数是**登记** |
| `#[error(...)]` 属性       |   68 |  13% | **按错误类型成组**（一个类型一次接收者分析）⇒ 效率最高的部分                                            |
| `.context(...)`            |    1 |    — | 直接译                                                                                                  |

**热点文件**（累计到 ~130 一批）：`startup_sync.rs` 80、`store.rs` 56、`remote_bundle.rs` 35、`loader.rs` 32、
`remote.rs` 29、`marketplace_upgrade/git.rs` 25、`marketplace.rs` 23、`remote/share/checkout.rs` 23、
`marketplace_policy.rs` 22、`plugin_bundle_archive.rs` 21、`manager.rs` 15、`npm_source.rs` 15 …
（长尾：另 20 个文件各 ≤14 条）

## 三、分批（按累计 ~110–140 条一批，共 4 批）

| 批     | 文件                                                                                                                  | 条数 | 备注                                                                    |
| ------ | --------------------------------------------------------------------------------------------------------------------- | ---: | ----------------------------------------------------------------------- |
| **B1** | `startup_sync.rs` + `store.rs`                                                                                        |  136 | 缓存/同步/持久化的内部诊断占多数 ⇒ 预期**登记多于译**；适合当**探针批** |
| **B2** | `remote_bundle.rs` + `loader.rs` + `remote.rs` + `remote/share/checkout.rs`                                           |  119 | 远程插件安装链 ⇒ 会有相当比例流到 CLI 的 issue 列表                     |
| **B3** | `marketplace_upgrade/git.rs` + `marketplace.rs` + `marketplace_policy.rs` + `plugin_bundle_archive.rs` + `manager.rs` |  106 | git/npm 参数片段多 ⇒ 登记为主                                           |
| **B4** | 其余 20 个文件（`npm_source.rs` 15 … 长尾）                                                                           |  157 | 逐个文件走同一流程                                                      |

## 四、每批的固定流程（复用已固化的工具链）

1. `python3 scripts/i18n_todo.py --root codex-rs/core-plugins/src --file <f> --dump-rows` 拉站点；
2. **接收者分析**：`thiserror` 按类型分组、`format!` 按函数分组，用 `grep` 找调用点；产出「译/登记」两栏；
3. 写 spec（`translate`/`register`/`extra_edits`/`extra_dict`）→ `scripts/i18n_apply.py --specs …`（plan 先看断言）→ `--apply`；
4. `cd codex-rs && just fmt` → `python3 scripts/i18n_dossier_lines.py --fix` → **逐条核对登记行命中源码**（值出现多次则人工判定）；
5. 门禁：`i18n-check` / `fmt-check` / `clippy` / 合并自检（**扩到 6 个 scope**：加 `core-plugins`）/ `just test -p codex-core-plugins`；
6. 有现成英文整句断言的文件，**必须**把那批的断言跑绿（逐字节不变的承重判据）；
7. 文档 §12.7x + 台账条目（requirements 全 `proves`）+ commit。

## 五、前置与风险

- **依赖**：`codex-rs/core-plugins/Cargo.toml` 目前**没有** `codex-i18n` ⇒ 需加 1 行
  `codex-i18n = { workspace = true }`（叶子 crate，无环；`BUILD.bazel` 由 `codex_rust_crate` 宏自动带依赖，无需手改）+ `Cargo.lock`；
  按 AGENTS.md 跑 `just bazel-lock-update` 与 `just bazel-lock-check`（plugin 批次实测：只有 `Cargo.lock` 变）。
- **可能触发范围外**：部分消息进 app-server / `--json` 契约 ⇒ **登记**（不要顺手译进机器契约）。
- **不可简化**：接收者分析不能按「文件看起来内部」跳过 —— core 最后的 10 条就是「看着内部、实际渲染给用户」的反例。

## 六、成本与建议

- **基准**（本会话实测）：每 30–50 站点 ≈ **8–12 次工具调用** + 门禁墙钟 5–10 min（clippy ~3 min、crate 测试 1–5 min）。
  ⇒ 518 条 ≈ **12–16 个子批 ≈ 3–5 个工作回合**。
- **建议**：先做 **B1 的探针子批（`store.rs` 56 条）**，产出**实测译/登记比与单批耗时**，再按实测值决定全量推进节奏
  （现在给的「登记多于译」是先验，不是结论）。

## 七、探针批次实测（store.rs 56 条，已完成）

探针批次选 `codex-rs/core-plugins/src/store.rs`（56 条，全 crate 最多的一处热点），
目的是**证伪或证实方案里的先验**，而不是先埋头改 518 条。

### 7.1 先验被证伪：这 56 条**全是「译」**，不是「登记多于译」

判据是接收方（谁渲染它），不是类型名。`PluginStoreError` 的两条链路都到底：
`PluginInstallError::Store(#[from])`（`manager.rs:3644`、`remote_bundle.rs:135`）→
① CLI `plugin_cmd` 的 install/uninstall 路径打成 stderr；
② app-server `request_processors/plugins.rs:1972` 的 `plugin_install_error` → `JSONRPCErrorError.message`
（JSON-RPC 的 **message** 字段是给人看的文本，不是机器键——机器键是同文件
`manager.rs:3735` 的 `store_io`/`store_invalid` 遥测 token，那些不在候选里）。
⇒ 56 条全部走 `tr`/`tr_with`，登记行 0 条。

### 7.2 三个实测陷阱（都已在工具或词典里落防）

1. **命名占位符的重编号顺序**（会静默错位）。工具的 `key` 重编号是
   **先命名占位符、后空占位符**（`{err}`→`{0}`、`{rollback_err}`→`{1}`、两个 `{}`→`{2}`/`{3}`），
   不是源码出现顺序。`store.rs:728-732` 那条同时含两类占位符：按源码顺序给 `args`
   占位符个数断言照样通过，渲染却会串位。**给 `args` 与写 `zh` 都必须按重编号后的 `{N}` 顺序。**
2. **`kind:"to_string"` 要求 `.to_string()` 紧邻字面量**（`text.find(".to_string()") == aj+1`）。
   跨行写法
   （字面量一行、`.to_string()` 下一行）会断言失败——这类站点用 `kind:"arg"`：
   替换只动字面量，原有的 `.to_string()` 自然保留，语义等价且无邻接约束（本次 15 条全走这条）。
3. **CJK/Latin 边界空格**是 `i18n-check` 的硬门禁（`has_mixed_boundary_space`：CJK 与 ASCII
   字母/数字之间不许有空格）。第一版 12 条译文写成「远程插件安装元数据 schema 版本」这种
   带空格形式，`i18n-check` 直接红（`[spacing] … : 12`）；改成「…元数据schema版本」后归零。
   仓库既有惯例同向（如「仅V1；V2忽略」）。

### 7.3 探针批次的成本实测

| 项         | 实测                                                                          |
| ---------- | ----------------------------------------------------------------------------- |
| 站点数     | 56（13 `format` + 43 字面量）                                                 |
| 新增词条   | 51（5 组重复值共用词条）                                                      |
| 登记行     | 0                                                                             |
| 工具调用   | 规格生成/plan/apply 各 1 次 + 1 次类型修正                                    |
| 编译安全网 | `cargo check -p codex-core-plugins --all-targets` EXIT=0（args 类型当场暴露） |

⇒ 按此速率，其余约 462 条 ≈ 9 个同规模批次；`core-plugins` 属**本机可编译**的 crate，
所以类型错误有编译器兜底（与平台门控批次不同）。

### 7.4 依赖变更的连带义务

`core-plugins/Cargo.toml` 新增 `codex-i18n = { workspace = true }` ⇒ 按 AGENTS.md 必须
`just bazel-lock-update` 并检查 `MODULE.bazel.lock` 漂移（`bazel-lock-check`）。
`BUILD.bazel` 经 `codex_rust_crate` 宏自动处理，无需手改。

## 八、第二批：startup_sync.rs 80 条全部「登记」（不译）

`core-plugins/src/startup_sync.rs` 是全 crate 最大的热点（80 条），但**一条都不译**——
这不是省的，是判据要求的。

### 8.1 判据（`docs/plan/i18n-design.md:186` 明文）

该文的甄别规则是**按调用点分类**，不是按「像不像一句话」：进
`add_error_message` / `add_info_message` / `add_warning_message` /
`add_to_history(new_error_event(…))`、`SelectionViewParams` 标题条目、直接渲染进
`Line`/`Span` 的字面量 ⇒ **要译**；`tracing::*` 日志、`.wrap_err("…")` 错误链上下文、
遥测属性名与值、喂模型的提示词、内部 id 与配置键 ⇒ **不译**。

### 8.2 本文件的文本流向（逐链查到终点）

- 唯一生产调用点：`manager.rs:3253`，在 `std::thread::Builder::new().name("plugins-curated-repo-sync")`
  起的 **detached 后台线程**里；`Err(err)` 的归宿是 `manager.rs:3276` 的
  `warn!("failed to sync curated plugins repo: {err}")`（`Ok` 分支的刷新失败同样是 `warn!`）。
- 文件自身的 `warn!` 调用点（`:119` `:134` `:144` `:404` `:417` `:441` `:452` `:463` `:476`）直接是日志参数。
- 其余 80 条是 `Result<_, String>` / `.context(…)` 链上的中间串，最终都汇进上面那条 `warn!`。
- 反向核对：`tui/src` 与 `core/src` 中不存在把 curated plugins 同步错误渲染成
  `Line`/`Span`/`add_*_message` 的调用点（grep 无命中）。

⇒ 80 条全部登记，登记行 80 条（`codex-rs/i18n/not-translated-unwrapped.tsv`，逐站点 `值 / path:line / 理由`），
词条与代码改动各 0。`i18n_todo.py --file …/startup_sync.rs` 复核：**0 candidates / 80 exempted**；
crate 总量 **462 → 374**。

### 8.3 顺带修掉的工具缺陷（会咬第二次）

`scripts/i18n_apply.py` 的 `_used_imports()` 以 `used = {"current"}` 起步，于是**纯登记**
（`translate: {}`）的 spec 也会往源文件插 `use codex_i18n::current;` —— 未用 import，
`clippy` 会红。本次实测踩到并已修（只有存在翻译站点时才加 `current`），
复核：同一 spec 的 plan 输出「将按需插入的 import： []」，误插的那一行已 `git checkout` 回退。

### 8.4 判据复核（对抗自检）

最可能被反驳的是「80 条里有一条其实到人眼」。逐条排除的依据是**调用点唯一性**：
全仓 `sync_openai_plugins_repo(` 只有 `manager.rs:3253` 与测试；中间串不离开本文件与 manager
的错误链。若将来有人在 UI 侧消费这条错误（例如插件面板显示同步失败原因），
**这批登记行必须重判**——判据是「谁渲染它」，不是「当初怎么登的」。

### 8.5 对抗自检抓到的一处真缺陷：按值登记会**连带豁免**别人的站点

§8.4 写的「若将来有人在 UI 侧消费这条错误，这批登记行必须重判」当场就应验了，而且**不是将来**。

**症状**：本批登记 80 行后，crate 候选总数是 462 → **374**，即 **−88**，比登记的 80 条多出 8 条。

**归因**（先量后猜）：`i18n_todo.py --fanout` 报「17 rows exempt >1 unwrapped site (41 sites)」，
其中 10 行的值同时出现在 `marketplace_upgrade/git.rs`；实测该文件从「25 candidates / 0 exempted」
变成「17 candidates / **8 exempted**」——**登记行的值是按值匹配的**，于是 startup_sync 的
`failed to run {context}: {err}` 这一行，把 `git.rs:193` 同值站点一并静默了。

**为什么这 8 条不能静默**：`marketplace_upgrade.rs:178` 把 git 助手的 `Err(String)` 推进
`outcome.errors`（`ConfiguredMarketplaceUpgradeError`），而 `cli/src/marketplace_cmd.rs` 会把
`errors` 渲染成用户可见输出（含 `--json`）。⇒ 它们是**用户面**，要译；与只看日志的
startup_sync 同值但**不同接收者**。

**修法**：同值不同接收者时，登记行必须用**站点式**（值列留空，只写 `path:line`）——
仓里既有 60 行就是这么写的（例如值跨行写不进 TSV 的那些）。本批把 10 行
（8 个值）改成站点式。

**复核**（回执 `r-mu8h8plx-xdis7p`）：`git.rs` 25 candidates / 0 exempted；
`startup_sync.rs` 0 candidates / 80 exempted；crate **382**（= 374 + 8）✓ 差额完全对上。

**收录为判据**：新增登记行前，先跑 `--fanout`；**值会在别的文件出现时，一律用站点式**，
否则就是把一个没做过的裁决顺手做掉了（"静默放行"）。

### 8.6 下一批（顺延）

`marketplace_upgrade/git.rs`（25 candidates）成为下一个高优先批次：它的 8 条模板错误
**终点在 CLI 用户面**（`marketplace_upgrade.rs:178` → `marketplace_cmd.rs` 的 errors 渲染），
按判据是**译**，而不是像 startup_sync 那样登记——**同一个值，两个文件，两个裁决**。

## 九、第三批：marketplace_upgrade/git.rs 25 条全部「译」

这是 §8.5 那 8 条「被误登记」站点的正主，也是**同一个值两种裁决**的活样本：
`startup_sync.rs` 的 `failed to run {context}: {err}` 只进日志（登记），
`git.rs` 的同值站点进 CLI（译）。

### 9.1 接收方判据（逐链接到终点）

- 两个入口：`git_remote_revision`（`marketplace_upgrade.rs:241` 调用）、
  `clone_git_source`（`marketplace_upgrade.rs:274` 调用）。
- 两者都在 `upgrade_configured_git_marketplace` 内，其 `Err(err)` 在
  `marketplace_upgrade.rs:145-151` 被推进 `ConfiguredMarketplaceUpgradeError{ message: err }`
  ⇒ `outcome.errors` ⇒ `cli/src/marketplace_cmd.rs` 渲染成人类可读输出（并有 `--json` 形态）。
- 内部全部是 `?` 链（`run_git_command_with_timeout` / `ensure_git_success` 返回 `Result<_, String>`），
  本文件**没有** `tracing::warn!` ⇒ 25 条**无一例外**到达用户面。
  ⇒ 25 条全译、登记 0 条。

### 9.2 字形与占位符（两个易错点）

- 13 条 `{context}` 实参（`git ls-remote 市场来源` 类）译成「命令名 + 中文宾语」形式，
  且**不留 CJK/Latin 边界空格**（`git ls-remote市场来源`）——`i18n-check` 的 `[spacing]` 是硬门禁。
- 8 条模板同时含命名占位符与空占位符，工具的重编号是「先命名后空」，于是键变成
  `{0} timed out after {2}s: {1}` / `{0} failed with status {2}: {1}` 这种**乱序**形态，
  `zh` 必须按同一套索引写（`{0}在{2}秒后超时：{1}`）。若按源码顺序给 `args`，
  占位符个数断言照样通过、渲染却会串位（§7.2 的同一个坑）。
- 模板译法沿用词典既有风格：`("{0} failed with status {1}", "{0}失败，状态码{1}")`。

### 9.3 收尾对账（沿用 §8.5 的护栏）

- `git.rs`：25 candidates → **0**；crate：382 → **357**，差额**正好 25**（无越权/漏账）。
- 门禁：`i18n-check` `r-mu8hrc1w-omwgo8`（3449 词条 / missing 0 / spacing 0 / dup 0）、
  `clippy` `r-mu8hviva-ux41eu`（46 crate 真编）、`cargo check -p codex-core-plugins --all-targets` EXIT=0、
  `just test -p codex-core-plugins` **438 passed** `r-mu8hwzn5-aoqa7x`。

### 9.4 对抗自检

最可能错的是「25 条全到用户面」这个全称判断。反例候选与排除：
① 有没有 `warn!` 分支？——本文件 grep 无 `warn!`；
② 有没有别的调用点（例如某个不经 `outcome.errors` 的后台刷新）？——全仓 `clone_git_source(`/`git_remote_revision(`
只有 `marketplace_upgrade.rs:241/274` 两处（另有 `marketplace_add/install.rs` 里**同名但不同模块**的函数，不在本文件链上）；
③ 有没有测试断言这些英文串（若有，默认 En 下仍会绿，等于没测到）？——crate 438 测试全绿，未新增语言相关断言，
**这一条只做到「没红」，不等于「zh 下确实出现中文」**（与 §12.72 的 `receiver-these-strings` 同一缺口）。

## 十、第四批：remote_bundle.rs 35 条全译（属性级路线首次规模化）

### 10.1 接收方

`RemotePluginBundleInstallError` 被 `RemotePluginOperationErrorKind::Bundle(#[error("install remote plugin bundle: {0}")])`
包住（`remote_mutations.rs:67`），而 `RemotePluginOperationErrorKind` 在
**app-server `request_processors/plugins.rs:1585`** 被逐变体消费（映射成 JSON-RPC 错误）⇒
与 store / marketplace 同族：**用户面 ⇒ 35 条全译、登记 0**。

### 10.2 两种形态：12 条属性 + 23 条调用点

- **12 条 `#[error("…")]`**：走 §12.70 的**属性级最小改法**
  —— `#[error("{}", tr_with(current(), "<重编号后的键>", &[<字段表达式>]))]`，保留 `#[derive(Debug, Error)]`。
  这是该路线**第一次规模化使用**（core 那次只有 10 条、且未涉及 u64/StatusCode 这类字段），因此在本批摸清了两个要点：
  1. `args` 的**字段类型**要按字段写：`String` 用 `.as_str()`；`PluginIdError`/`RouteAwareRequestError`/`HttpError`
     用 `&source.to_string()`；`StatusCode` 用 `&status.to_string()`；`u64` 用 `&max_bytes.to_string()`；
  2. 键与译文要**手写**（工具不重编号 `extra_edits`），所以键必须写成位置式 `{0}`/`{1}`，
     并与 `extra_dict` 里的 `zh` 索引一致 —— 否则渲染会串位。
- **23 条调用点**：`format!`（13）/ 裸字面量（9）/ `.to_string()`（4，其中 2 条同时出现在两处 ⇒ 值去重）。
- **2 条片段**（`:356` `\n[响应体在{0}字节后被截断]`、`:360` `\n[读取响应体失败：{0}]`）：
  它们被 `body.push_str(...)` 追加到响应体上，最终进入 `DownloadStatus { body }` 的**同一条用户可见消息**，
  所以是**可独立翻译的括号注记**（§3.6 的「碎片拼句」在这里不适用：它们不与正文构成一个被拆开的句子）。

### 10.3 收尾对账与门禁

- `remote_bundle.rs`：35 candidates → **0**；crate：357 → **322**（差额**正好 35**）。
- `cargo check -p codex-core-plugins --all-targets` EXIT=0（属性路线的类型当场验）；
  `i18n-check` `r-mu8yjdf1-p1b40v`（3482 词条 / missing 0 / **spacing 0** / dup 0）；
  `clippy` `r-mu8yn9fe-kfyc6k`（46 crate / 2m54s，`uninlined-format-args` 未对 `"{}", expr` 形态报错，与 core 批次一致）；
  `just test -p codex-core-plugins` **438 passed** `r-mu8yp9mj-bnt04b`；`fmt-check` `r-mu8yq0r8-x6hm8r`。

### 10.4 对抗自检

最可能错的仍是「35 条全到用户面」这个全称判断。本批的反例候选与排除：
① 有没有 `warn!` 分支？——本文件 grep 无 `warn!`；
② 有没有第二条消费路径（比如只进内部诊断）？——`RemotePluginOperationErrorKind` 的全仓消费点集中在
app-server `plugins.rs:1581-1598`；`Bundle(...)` 那一支被显式列出；
③ 有没有把**机器键**当文案译了？——`:274/:296` 的 `JoinError`、`:436/:540/:588` 的 `serde` 错误都只作为 `{N}` 插值，
没有拿翻译后的文本做匹配或控制流。
**未验证**：这 35 条在 zh 下**确实渲染出中文**——与 §12.72 的 `receiver-these-strings` 同一缺口（本机无 CLI/app-server 级断言）。

## 十一、第五批：loader.rs 32 条 = 译 9 + 登记 23（首例「登记多于译」）

这是 core-plugins 里第一个**日志占多数**的文件：32 条候选里 23 条只进 `tracing`，9 条到用户面。

### 11.1 四处判决（都是查出来的，不是按文件名猜的）

| 站点的去向                                                            | 查证结果                                                                                                                                                                                                                                                               | 判决                                                                                                       |
| --------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| `:197` `:358` `:613` `:643` `:1571` `:1688` `:1715`                   | 紧邻的 `warn!(...)` 的**参数本体**（`:193` `:354` `:609` `:639` `:1567` `:1684` `:1711`）                                                                                                                                                                              | 登记                                                                                                       |
| `:809` `:821` `:822`                                                  | 作为字符串参数传给 `configured_plugin_ids(…, invalid_plugin_key_message)`（`loader.rs:784`）与 `configured_plugins_from_codex_home(…, read_error_message, parse_error_message)`（`:750`），**两者内部就是 `warn!("{invalid_plugin_key_message}")`（`:793` / `:760`）** | 登记                                                                                                       |
| `:392` `:402` `:437` `:459` `:477` `:486` `:597` `:668` `:678` `:696` | 都在 cache refresh 链上（`Result<_, String>`）；终点的调用方 **`manager.rs:2988` 是 `warn!("failed to prepare non-curated plugin cache refresh: {err}")`**，而 app-server `plugins.rs:588` 调 `refresh_non_curated_plugin_cache_for_context` **只取 bool**、丢弃 error | 登记                                                                                                       |
| `:870` `:892` `:897`                                                  | 写入 `LoadedPlugin.error`；全仓消费点只有 `loader.rs:120-122` 的 `warn!`（app-server 的 `marketplace_load_errors` 另有来源，不吃这个字段）                                                                                                                             | 登记                                                                                                       |
| **`:1260` `:1270`**                                                   | `hook_load_warnings` 会经 **app-server `catalog_processor.rs:615/625` 的 `hooks/list` 返回**，且有测试 `hooks_list_shows_plugin_hook_load_warnings`（`app-server/tests/suite/v2/hooks_list.rs:1090`）**证明它被展示**                                                  | **译**                                                                                                     |
| **`:1761` `:1770` `:1785` `:1789`**                                   | `materialize_marketplace_plugin_source*` 的错误经 `manager.rs:2620` 的 `.map_err(MarketplaceError::InvalidPlugin)` 进 marketplace 用户面                                                                                                                               | **译**                                                                                                     |
| **`:1865` `:1904` `:1910`**                                           | 同上的 git 助手（`run_git*`）只在 materialize 的 git 检出流程里被调用                                                                                                                                                                                                  | **译**（注意：同一助手也被 cache refresh 调到 ⇒ **同值两接收者 ⇒ 走到 UI 就译**，见 `i18n-design.md:387`） |

### 11.2 两处实测到的坑（都当场纠正）

1. **§7.2 的坑又踩了一次**：`:1260/:1270/:1761/:1770/:1904` 的字面量里**同时有空占位符和命名占位符**（`{}: {err}`），
   工具的键是「先命名后空」⇒ 键变成 `… {1}: {0}`；我按**源码顺序**给 `args` ⇒ 渲染会串位。
   修法：`args` 按重编号顺序给（`[&err.to_string(), &path.display().to_string()]`），`zh` 用同一套索引（`…{1}失败：{0}`）。
   ⇒ 已给 `scripts/i18n_apply.py` **加了一道断言**：`zh` 的占位符索引集合必须等于键的索引集合。
   **能力边界（诚实标注）**：它挡「缺项/越界」，**挡不住「位置互换」**（索引集合相同、位置对调时两者都通过）；
   位置正确性目前只能靠人读 plan 输出。
2. **站点式登记行不随行号漂移自动重链**：本批工具在文件顶部插了 2 行 import ⇒ 全文件行号 +2 ⇒
   `i18n_dossier_lines.py --fix` **只重链了 21 条按值行，2 条站点式行仍是旧行号**（于是 census 少了 30 而不是 32）。
   靠 §8.5 的**对账规矩**（登记/翻译站点数必须等于 census 差额）当场发现，手工把 `892→894`、`897→899` 后归零。
   ⇒ 规矩升级：**每次 `just fmt` / 插 import 之后，用差额对账而不是只看「跑过了」**。

### 11.3 收尾对账与门禁

- `loader.rs`：32 candidates → **0**；crate：322 → **290**（差额**正好 32** = 9 译 + 23 登记）；23 条登记行**逐条命中源码**。
- `cargo check -p codex-core-plugins --all-targets` EXIT=0；`i18n-check` `r-mu8z79aj-b343q5`（3490 词条 / missing 0 / spacing 0 / dup 0）；
  `clippy` `r-mu9dk1t8-11gbl6`；`just test -p codex-core-plugins` **438 passed** `r-mu9dlui6-29j9ma`。

## 十二、第六批：remote.rs 29 条全译（18 属性 + 11 调用点）

### 12.1 接收方（两类都在用户面）

- `RemotePluginCatalogError` 经 `RemotePluginOperationErrorKind::Catalog { context, source }`
  被 app-server `request_processors/plugins.rs:1581` 消费 ⇒ 用户面；
- `validate_remote_plugin_id` 直接返回 `Result<(), JSONRPCErrorError>`（`:369`，`:374` 那条）⇒ 用户面。
  ⇒ 29 条全译、登记 0。

### 12.2 属性路线的三种形状（本批一次覆盖）

| 形状                   | 例                                                                                                      | args 写法                                                          |
| ---------------------- | ------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------ |
| unit 变体              | `AuthRequired`（`:385`）、`MissingUploadEtag`（`:466`）                                                 | `&[]`                                                              |
| 元组变体 + `#[source]` | `AuthToken(#[source] io::Error)`（`:393`）、`InvalidBaseUrl`（`:417`）、`ArchiveJoin`（`:458`）         | **`&_0.to_string()`** —— thiserror 为元组字段生成的绑定名就是 `_0` |
| 具名字段变体           | `Request{url, #[source] source}`（`:396`）、`ArchiveTooLarge{bytes: usize, max_bytes: usize}`（`:462`） | `url.as_str()` / `&source.to_string()` / `&bytes.to_string()`      |

类型清单（都由 `cargo check` 当场验，18.72s EXIT=0）：`String`→`.as_str()`；`io::Error`/`serde_json::Error`/`url::ParseError`/`RouteAwareRequestError`/`JoinError`→`&x.to_string()`；
`StatusCode`/`bool`/`usize`→`&x.to_string()`；`PathBuf`→`&x.display().to_string()`；`&'static str`（`remote_plugin_canonical_marketplace_name` 的返回值）→**直接用**、不要 `.as_str()`。

### 12.3 门禁当场抓到的两处（都是我的错）

1. **`[spacing]` 1**：`"远程插件 \`{0}\` 不可用于 plugin/share/checkout"`——`于`与`p`之间那个空格正好落在 CJK↔拉丁边界上。
改成`不可用于plugin/share/checkout`后归零。⇒ 判据：**译文里凡是「中文 + 英文标识符」相接处，默认不留空格**（英文缩写内部如`API key` 的空格不受影响）。
2. **`[duplicate]` 1**：`chatgpt authentication required for remote plugin catalog` **早先批次（cli 的 plugin_cmd 批次）已经在词典里**，
   我这次又写了一条 ⇒ 两条同键，旧条目成为「dead」。这暴露了属性路线的盲区：走 `extra_dict` 时**工具不知道键是否已存在**
   （调用点路线由工具自己 `setdefault` 就不会重复）。处置：删掉旧的那条（保留本批措辞）。
   **但删的时候我只删了 `zh` 那一行，留下了 `("key",)` 这种单元素元组** ⇒ `cargo check -p codex-i18n` 报 `E0308`。
   ⇒ 规矩：**手工改词典后必须 `cargo check -p codex-i18n`**（`i18n-check` 也会编译到它，但用 cargo 直接验更快、错误更直白）。

### 12.4 一条查过并排除的疑虑（对抗自检的正向结果）

翻译前我担心 `cli/src/plugin_cmd.rs:866` 那处同值是**匹配键**（若如此，把消息译成中文会让那个匹配失效 ⇒ 行为改变）。
查证结果：那处**早已被 `tr(current(), …)` 包住**（所以 cli 的候选普查才是 0），不存在控制流依赖 ⇒ 疑虑排除、可以放心译。
⇒ 判据复述：**同值出现在别处时，先去那处看它是不是匹配键**；是匹配键则不能译（§9.1），是渲染点则可译。

### 12.5 收尾对账与门禁

- `remote.rs`：29 candidates → **0**；crate：290 → **261**（差额**正好 29**）；core-plugins 的 **103** 条登记行逐条命中源码（0 漂移）。
- `cargo check -p codex-core-plugins --all-targets` EXIT=0；`i18n-check` `r-mu9dvggk-6kazjo`（3515 词条 / missing 0 / **spacing 0** / **duplicate 0**）；
  `clippy` `r-mu9e04c9-wgt5hx`（46 crate / 3m29s）；`just test -p codex-core-plugins` **438 passed** `r-mu9e2b12-iblw84`。

## 十三、第七批：marketplace.rs 23 条（译 21 + 登记 2）

### 13.1 分簇与接收方

| 簇                                                          | 站点                                                                                                     | 去向                                                                                        | 判决           |
| ----------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------- | -------------- |
| `MarketplaceError` 的 `#[error]` 属性                       | `:213` `:216` `:219` `:226` `:233`                                                                       | 该错误在 `marketplace_cmd` / `manager.rs:2620`（`MarketplaceError::InvalidPlugin`）进用户面 | 译（属性路线） |
| `InvalidMarketplaceFile { message: … }` 的 **message 实参** | `:316` `:351` `:658` `:675` `:686` `:704` `:714` `:728` `:753` `:770` `:815` `:843` `:865` `:876` `:894` | 被 `"invalid marketplace file \`{path}\`: {message}"` 嵌进**用户可见**那条 ⇒ 中间串也到人眼 | 译             |
| `MarketplaceError::io(context, …)` 的 context               | `:497`                                                                                                   | 同 store 批次：context 是用户可见消息的一半                                                 | 译             |
| `warn!` 参数本体                                            | `:383`（warn! 在 `:379`）、`:585`（`:581`）                                                              | 只进日志                                                                                    | 登记           |

类型清单（`cargo check` 当场验）：`PathBuf`→`&x.display().to_string()`；`String`→`.as_str()`；`&str`（`url`/`package`/`field` 经 `.trim()` 后）→**直接用**。

### 13.2 抓到一个**工具缺陷**（这是本批最大的收获）

`i18n-check` 报 `[duplicate] 1`：`marketplace root does not contain a supported manifest` 在词典里有**两条**。
根因不在我的键盘上，而在工具里：`i18n_apply.py` 判断「键是否已存在」用的是 `f'("{k}"' in dict` ——
**只认单行形态 `("k", "v"),`**；而工具自己写入长条目时会**折行**成

```
    (
        "k",
        "v",
    ),
```

⇒ 折行条目对它「不存在」⇒ 同一个键被反复写入（**这也解释了上一批 chatgpt 那条重复的成因**）。

处置：把查重改成 `re.search(r'"<key>"\s*,', dict)`（单行/折行都认），并做三形态自检（单行命中 / 折行命中 / 不存在不命中）；
随后**整元组**删掉本批重复写入的那一条（上一批的教训：只删 `zh` 行会留下单元素元组 `E0308`），
`cargo check -p codex-i18n` EXIT=0 复核。⇒ 复跑 `i18n-check` 全零。

### 13.3 两条流程规矩的落地效果

1. **上批新增的「写 `extra_dict` 前先 grep 查重」当场生效**：`plugin \`{0}\` was not found in marketplace \`{1}\``
   已在词典中 ⇒ 跳过，不重复添加（少一条重复）。
2. **对账规矩**（census 差额 == 站点数）：261 → **238**（差 **23** = 16 译 + 5 属性 + 2 登记）✓。

### 13.4 门禁

`i18n-check` 首跑红 `r-mu9e7kbc-t2057j`（duplicate 1）→ 修工具 + 删重复后 `r-mu9e9odt-nda7s2` 全零（3533 词条 / spacing 0 / **duplicate 0**）；
`clippy` `r-mu9ecvg6-rsc7ff`（46 crate / 2m22s）；`just test -p codex-core-plugins` **438 passed** `r-mu9edprf-paul0z`。
另注：同值还出现在 `cli/src/plugin_cmd.rs:1108`（早已 `tr(...)` 包住）与 **测试** `cli/tests/plugin_cli.rs:533`（断言英文原文，默认 En 下无影响）。

## 十四、第八批：remote/share/checkout.rs 23 条全译（0 登记）

### 14.1 接收方

本文件**所有**相关函数都返回 `Result<_, RemotePluginCatalogError>`（`:42` `:174` `:191` `:246` `:262` `:346` `:412`），
而该错误经 `RemotePluginOperationErrorKind::Catalog` 在 app-server 用户面被消费（§12.1 已证）⇒ **23 条全译**。

三种「被嵌入」的形态也一并算用户面：

- `invalid_marketplace_file(path, message)`（`:452`）的第 2 个实参被嵌进用户可见的 `invalid marketplace file …: {message}`；
- `write_json_atomically`（`:459`）返回的 `io::Error` 消息被调用方包进 `failed to update personal plugin marketplace: {err}`；
- 本文件**没有** `tracing::warn!` ⇒ 登记 0 条。

### 14.2 类型检查抓到一处「同名不同型」

`cargo check` 报 `E0658 str_as_str`（`:358`）：`:321` 那条我按 `:54`/`:209` 的经验给 `plugin_name` 加了 `.as_str()`，
但在**那个**函数里 `plugin_name` 是 `&str` 形参（`entry.get("name")… == Some(plugin_name)`）⇒ 对 `&str` 调 `.as_str()` 只存在于 nightly。
改成裸 `plugin_name` 后 `cargo check -p codex-core-plugins --all-targets` **EXIT=0**（4.60s）。
⇒ 判据：**同一个标识符在不同函数里可能是不同类型**；读**所在函数的签名**，不要按名字套经验。
（本批另一处对照：`:54` 的 `reason` 来自 `validate_plugin_segment(...) -> Result<(), String>` ⇒ `String` ⇒ `.as_str()` 正确。）

### 14.3 收尾对账与门禁

- `checkout.rs`：23 candidates → **0**（其余 1 条是早先批次已登记的站点）；crate：238 → **215**（差额**正好 23**）。
- `i18n-check` `r-mu9einb6-akc5it`（3556 词条 / missing 0 / spacing 0 / duplicate 0 / coverage 99.7%）；
  `clippy` `r-mu9ekxc8-sjgztl`（46 crate / 1m40s）；`just test -p codex-core-plugins` **438 passed** `r-mu9eljdw-1zh7gp`。

## 十五、第九批：marketplace_policy.rs 22 条全译（0 登记）

### 15.1 按「私有 helper → 公有入口 → 消费者」建链

本文件的候选分属 7 个函数，其中多数是**私有 helper**（外部没有调用点），所以判接收方要沿链往上走：

| 站点（所在函数）                                 | 链路                                                                                                                                                                        | 判决                                                                                |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | --- |
| `:100`（`validate_source`）                      | `validate_source` ← `validate_git_source` / `validate_install` / `validate_marketplace_source_for_add`                                                                      | 译                                                                                  |
| `:121 :135 :146 :150`（`validate_install`）      | `manager.rs:2500` `.map_err(                                                                                                                                                | message                                                                             | MarketplaceError::InvalidMarketplaceFile{ … message … })` ⇒ 用户面；`manager.rs:2050` 也构造同一错误 | 译  |
| `:167`（`validate_git_source`）                  | `marketplace_upgrade.rs:125` ⇒ `errors[].message`（用户面）；`manager.rs:718` 只用 `.is_ok()`（丢弃）                                                                       | 译（同值两接收者，走到 UI 就译）                                                    |
| `:340`（`validate_marketplace_name_for_add`）    | `marketplace_upgrade.rs:123` ⇒ `errors[].message`；`marketplace_add.rs`                                                                                                     | 译                                                                                  |
| `:352 :356 … :400`（`compile_allowed_source`）   | **`RestrictedMarketplacePolicy.allowed_sources: Result<Vec<…>, String>`**，而 `validate_source` 里 `allowed_sources.as_ref().map_err(Clone::clone)?` ⇒ 错误**冒泡到用户面** | **译**（我最初按「只在 `from_requirements` 里 collect」判成登记 —— 数据把它翻正了） |
| `:418 … :436`（`configured_marketplace_source`） | ← `validate_configured_marketplace` ← `validate_install`（用户面）/ 策略列举（日志）                                                                                        | 译                                                                                  |
| `:451`（`validate_expected_marketplace_name`）   | ← `validate_install`（`:117`）/ `validate_marketplace_name_for_add`（`:336`）                                                                                               | 译                                                                                  |

⚠ 对照：`plugin_cmd.rs:633/1045` 调 `allowed_configured_marketplace_names` 时**只把它当集合做 `contains` 过滤**（错误走 `:307` 的 `warn!`）——
所以「同一个模块的函数」也可能一条链到人眼、另一条链只到日志；**按站点查链路，不按文件判性质**。

### 15.2 clippy 抓到 `cargo check` 看不出的一类缺陷

首跑 `clippy` **红**：`marketplace_policy.rs:107`

```
error: redundant clone
   &source.display().to_string(),
   ^^^^^^^^^^^^^^^^^^^^ help: remove this
   = note: requested on the command line with `-D clippy::redundant-clone`
```

根因：`MarketplaceSource::display()` 返回的是 **`String`**（`marketplace_add/source.rs:206`），不是 `Path::display()` 那种适配器
⇒ `.to_string()` 成了克隆。改成 `source.display().as_str()` 后 `clippy` 绿 `r-mu9ewpvs-9fp6mg`。
⇒ 判据：**`cargo check` 管类型，`clippy` 管「能编译但有冗余」**（本仓 `redundant_clone` / `needless_borrow` 是 deny）——
两步都不能省；`.to_string()` 之前先确认那个 `display()`/访问器返回的到底是 `String` 还是 `&str`/`Display`。

### 15.3 收尾对账与门禁

- `marketplace_policy.rs`：22 candidates → **0**；crate：215 → **193**（差额**正好 22**）。
- `i18n-check` `r-mu9equtp-fiuxkg`（3577 词条 / missing 0 / spacing 0 / duplicate 0 / coverage 99.7%）；
  `clippy` `r-mu9ewpvs-9fp6mg`（2m37s）；`just test -p codex-core-plugins` **438 passed** `r-mu9eyegv-hqp1hj`。

## 十六、第十批：plugin_bundle_archive.rs 21 条全译（0 登记）

### 16.1 两个错误枚举 + 两个消费者

文件里有两个枚举：`PluginBundlePackError`（`:14`，`InvalidPluginPath` / `ArchiveTooLarge` / `Io`）与
`PluginBundleUnpackError`（`:29`，`ExtractedBundleTooLarge` / `Io{context,source}` / `InvalidBundle(String)`）。
消费者都把错误映射进**用户面**：

- `pack_plugin_bundle_tar_gz` → `remote/share.rs:482` `.map_err(|err| match err { … })`；
- `unpack_plugin_bundle_tar_gz` → `remote_bundle.rs:660` `.map_err(|err| match err { … })`（§10 已证该族到 app-server/CLI）。
  ⇒ 21 条全译、登记 0。

### 16.2 本批的新形态：`write!` 里的 Display 文案

`:312` 那条不在 `format!` 里，而是在

```rust
impl fmt::Display for ArchiveSizeLimitExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "archive would be {} bytes, exceeding maximum size of {} bytes", self.bytes, self.max_bytes)
    }
}
```

⇒ 用 `extra_edits` **整块改写**成 `write!(f, "{}", tr_with(current(), "archive would be {0} bytes, …", &[&self.bytes.to_string(), &self.max_bytes.to_string()]))`。
⚠ 必须把原来的两个 `self.bytes, self.max_bytes` 一起删掉 —— 只换字面量会留下「format 参数未使用」的编译错。

### 16.3 编译器抓到的两个真缺陷（都当场修）

1. **`current` 被同名局部绑定遮蔽**：`fn append_plugin_tree<W: Write>(archive, plugin_root, current: &Path)` 里的形参就叫 `current`
   ⇒ 工具插入的 `current()` 解析成了「调用一个 `&Path`」⇒ `E0618`（报了两处）。
   修法：该文件里所有插入调用改成**限定路径** `codex_i18n::current()`，并删掉因此不再使用的 `use codex_i18n::current;`。
   ⇒ 这是工具能力的**已知盲区**（它不知道作用域里有没有同名绑定）：已记 `known_issues i18n-apply-current-shadowed-by-local-binding`。
2. **规格里凭猜写了变量名**：`plugin bundle output path has no parent` 那条我在 spec 里写了 `path.display()`，
   而实际绑定是 **`output_path`** ⇒ `E0423 expected value, found built-in attribute 'path'`。
   ⇒ 修成 `output_path.display()`。教训与第十批同源：**实参名要看源码，不能按语义猜**（同批 `:105` 也印证过：我以为那里是 `entry_type`，其实是 `path`）。

### 16.4 工具的第二处局限：`{:?}` 不被计为占位符

`:199` 的字面量含 `{:?}`，而 `i18n_apply.py` 的占位符正则只认 `{}` / `{ident}` ⇒ 断言「占位符 1 个 vs 表表达式 2 个」直接拦下。
处置：该站点改用 `extra_edits` 整块改写（键手写为位置式 `… has unsupported type {1}`，值先用 `&format!("{entry_type:?}")` 预格式化）。
⇒ 判据：**字面量里出现格式说明（`{:?}` / `{:>5}` 等）时，走 `extra_edits`，别走 `translate`。**

### 16.5 门禁抓到的一处（我的）与收尾对账

- `[spacing] 1`：`"缺少 .codex-plugin/plugin.json 或有效的 Agent Plugin 清单"` —— `n` 与 `或` 之间有空格 ⇒
  改成 `缺少.codex-plugin/plugin.json或有效的Agent Plugin清单` 后归零（中文与英文标识符相接处不留空格）。
- `plugin_bundle_archive.rs`：21 candidates → **0**；crate：193 → **172**（差额**正好 21**）。
- `i18n-check` `r-mu9f742b-mg3177`（3596 词条 / spacing 0 / duplicate 0）；`clippy` `r-mu9fartj-yu8e2p`（46 crate / 2m44s）；
  `just test -p codex-core-plugins` **438 passed** `r-mu9fcju8-v5jl86`；`cargo check --all-targets` EXIT=0（修后 4.79s）。

## 十七、第十一批：manager.rs 15 条（译 9 + 登记 4 + 属性 2）

### 17.1 登记 4 条：都是 tracing 的消息本体

`:1562`（`tracing::info!`）、`:2138`（`tracing::warn!`）、`:2164`（`tracing::warn!`）、`:3355`（`warn!`）——
判据是「本行就是宏的消息参数，同调用内其余参数是结构化字段」（`error_type = %…` / `materialized_remote_plugin_count = …` 这类）。

### 17.2 译 9 条（含一条「值已在词典」的樱桃）

| 站点                            | 去向                                                                                                  | 备注                           |
| ------------------------------- | ----------------------------------------------------------------------------------------------------- | ------------------------------ |
| `:2203`                         | `PluginStoreError::Invalid`                                                                           | store 族 ⇒ 用户面（§7.1 已证） |
| `:2613` `:2625` `:2633` `:2650` | `MarketplaceError::InvalidPlugin`                                                                     | 用户面                         |
| `:2912`                         | `upgrade_configured_marketplaces_for_config_with_mode` → **app-server `marketplace_processor.rs:86`** | ✅ 见 17.3                     |
| `:3626` `:3628` `:3631`         | `remote_plugin_install_required_description`（`tool_suggest_metadata.rs:216` + `manager.rs:2586`）    | ✅ 见 17.4                     |

- **`:2650` 是计划好的樱桃**：该值（`missing or invalid plugin.json`）在 §7 的 store 批次已进词典 ⇒ 本批**只包站点、不新增词条**。
  工具也如实报出「字典已有（只包站点、不新增）1 条」——这是 §13.2 修的查重缺陷在生产里的第一次正收益。
- **`:2633` 与 loader.rs 那个登记行同值**（`path does not exist or is not a directory`）：这里到用户面 ⇒ **译**；loader 那边只进 `warn!` ⇒ **登记**（站点式行）。
  ⇒ 又一次印证「同值不同接收者，按站点判」。

### 17.3 两处「按签名写 args」的实证（上批教训的直接应用）

- `:2912` 的 `marketplace_name` 来自 `if let Some(marketplace_name) = marketplace_name`（原参是 `Option<&str>`）⇒ **直接用**，不能 `.as_str()`；
- `:2613` 的 `plugin_key` 来自 `plugin_id.as_key()`（`String`）⇒ `.as_str()`；`:3631` 的 `source_kind` 是 `tr(...)` 的返回值（`&'static str`）⇒ 直接用、只有 `source_description`（`parts.join(", ")` 的 `String`）要 `.as_str()`。
  ⇒ 本批 `cargo check` **首次即 EXIT=0**（28.24s）。

### 17.4 一处需要说明的判决：`:3631` 的描述串是「双重去向」

`remote_plugin_install_required_description` 的返回值有两条消费：

- `tool_suggest_metadata.rs:216` → 包进 `prompt_safe_plugin_description(...)` → `ToolSuggestMetadataFragment.description`（**偏向模型/提示**）；
- `manager.rs:2586` → `PluginDetail.description`（**插件详情 UI**）。
  按设计文档「判据是文本流向；同一段文案经 `.context` 走到 UI 就译」⇒ **判译**。
  ⚠ 这条是**判断**不是机器事实：若将来确认该描述只会进模型上下文而永不进 UI，应改为登记。

### 17.5 顺带做的一次风险预检：`current` 遮蔽

§16 的教训（插入的 `current()` 被同名绑定遮蔽）在本批**先查后用**：`manager.rs` 里确有 `|current| …` 闭包（`:1374` `:1889` `:1914`），
但都不覆盖本批的 15 个站点 ⇒ 无需限定路径，编译一次通过。

### 17.6 收尾对账与门禁

- `manager.rs`：15 candidates → **0**（该文件另有 5 条早先批次的登记行）；crate：172 → **157**（差额**正好 15**）。
- `i18n-check` `r-mu9fj6gd-l5uqe0`（3606 词条 / spacing 0 / duplicate 0）；`clippy` `r-mu9fnyzx-wqdcyw`（46 crate / 3m37s）；
  `just test -p codex-core-plugins` **438 passed** `r-mu9fpofn-6mk3rr`。

## 十八、第十二批：npm_source.rs 15 条全译（0 登记）

### 18.1 接收方（一条链）

`materialize_npm_plugin_source` 被 **`loader.rs:1816`** 调用 → 它在 `materialize_marketplace_plugin_source*` 里 →
错误在 `manager.rs:2620` 变成 `MarketplaceError::InvalidPlugin`（§17.2 已证用户面）
⇒ 本文件 15 条**全译**；文件内**没有 `tracing::warn!`** ⇒ 登记 0。

### 18.2 开工前预检生效（本批 `cargo check` 也一次过，15.74s）

- **预检① 遮蔽**：`grep -nE "\bcurrent\b"` → 本文件没有名为 `current` 的绑定 ⇒ 无需限定路径；
- **预检② 值是否已在词典**：按工具的**重编号规则**算出键再 grep，命中 **3 条**与本批重复的键（都来自 §11 的 loader.rs 批次）：
  `failed to create marketplace plugin source staging directory {1}: {0}`、
  `… in {1}: {0}`、`failed to resolve materialized plugin source path: {0}`
  ⇒ 只包站点、不新增词条；工具事后也如实报出「字典已有（只包站点、不新增）3 条」——
  **跨文件复用词条**在这里第一次成规模（同一段文案在两个文件里各有一个渲染点，词典只有一份）；
- **预检③ 签名**：`package`（参数 `&str`）直接用；`metadata.name`（`String`）用 `.as_str()`；
  `archives.len()` / `archive_size` / `output.status` / 两个 const 各自 `.to_string()`。

### 18.3 一处需要留意的形态：`\n` 与常量占位符

- `:110` 的 `"npm pack failed with status {}\nstdout:\n{}\nstderr:\n{}"`：三个空占位符 ⇒ 键 `{0}/{1}/{2}`，
  译文同样带 `\n`（词条里存的是**字面 `\n` 两字符**，与源码字面量的运行时含义一致）；
- `:142` 的 `"{archive_size}"` 与 `"{NPM_PLUGIN_SOURCE_MAX_ARCHIVE_BYTES}"` 都是**具名占位符**（后者是常量名），
  重编号后为 `{0}`/`{1}`，两处都按 `&x.to_string()` 预格式化。

### 18.4 收尾对账与门禁

- `npm_source.rs`：15 candidates → **0**；crate：157 → **142**（差额**正好 15**）。
- `i18n-check` `r-mu9fuzqn-e853l6`（3618 词条 / missing 0 / spacing 0 / duplicate 0 / **coverage 99.8%**）；
  `clippy` `r-mu9fxak2-v08i2t`（46 crate / 1m42s）；`just test -p codex-core-plugins` **438 passed** `r-mu9fxvs0-2nscy7`。

## 十九、第十三批：remote_legacy.rs 14 条全译（纯属性批次，也是最暴露工具缺陷的一批）

### 19.1 形态与接收方

14 条**全部**是 `#[error(...)]` 属性，分属两个枚举：`RemotePluginMutationError`（10 条）与 `RemotePluginFetchError`（4 条）。
三个形状都出现：unit（`AuthRequired` / `UnsupportedAuthMode` / `InvalidBaseUrlPath`）、
元组 + `#[source]`（`AuthToken(io::Error)` / `InvalidBaseUrl(url::ParseError)`）、
具名字段（`Request{url, source}` / `UnexpectedStatus{url, status, body}` / `Decode{url, source}` / `UnexpectedPluginId` / `UnexpectedEnabledState`）。
⇒ 全译、登记 0。

### 19.2 预检② 又抓到 2 条跨文件复用

`remote plugin mutation returned unexpected plugin id: …` 与 `… unexpected enabled state for …`
在 §12 的 remote.rs 批次已进词典 ⇒ 本批只新增 **12** 条（工具报「字典已有 2 条」）。

### 19.3 命名占位符的**顺序**必须逐条看

`:48` 的 `remote plugin mutation failed with status {status} from {url}: {body}` —— 出现顺序是 **status → url → body**，
键为 `{0}=status / {1}=url / {2}=body`；而 §12 的 remote.rs `:99` 是 `… from {url} failed with status {status}`（**url 在前**）。
两条文案很像，但 args 顺序相反 ⇒ 若照抄上批会串位。**这也是为什么 args 必须对着源码写、不能凭记忆套。**

### 19.4 本批暴露并修掉的两个**工具缺陷**（`_used_imports`）

这是本会话**第一个纯属性批次**（`translate: {}`，只有 `extra_edits`），于是把两处一直被掩盖的缺陷顶了出来：

1. **`current` 的判定位置错了**：它在 `translate` 循环之后、`extra_edits` 循环之前 —— 纯属性规格下 `used` 还是空集
   ⇒ 不插 `current` ⇒ 生成的 `current()` **无法解析**。
   修法：把判定移到**两个循环之后**，条件是 `"tr" in used or "tr_with" in used`。三形态自检：
   纯登记 ⇒ `[]`；纯属性 ⇒ `{current, tr_with}`；调用点 ⇒ `{current, tr}`。
2. **`"tr(" in body` 子串判定假阳性**：`as_str()` 里就含 `tr(`（…`s`,`t`,`r`,`(`）⇒ 多插一个没用的 `use codex_i18n::tr;`
   ⇒ `unused_imports` 警告（clippy 首跑就有这条警告，但 exit 0 —— **警告不会拦，只会污染**）。
   修法：改成词边界正则 `(?<![A-Za-z0-9_])tr\(`；自检覆盖「含 `as_str()` ⇒ 不得多出 `tr`」；并从 `remote_legacy.rs` 删掉误插的那行。

> 两个缺陷都属「工具自己生成的东西没人检」这一类 —— 本轮的做法是：**发现即修 + 三形态自检 + 清掉误插产物**，
> 而不是手工绕过（绕过会让下一个纯属性批次再踩一次）。

### 19.5 收尾对账与门禁

- `remote_legacy.rs`：14 candidates → **0**；crate：142 → **128**（差额**正好 14**）；
  `cargo check -p codex-core-plugins --all-targets` **EXIT=0**（7.51s，证明 import 修复有效）。
- `i18n-check` `r-mu9g24xz-8uggz2`（3630 词条 / spacing 0 / duplicate 0 / coverage 99.8%）；
  `clippy` **修前** `r-mu9g5bsu-pwwljj`（带 1 条 unused-import 警告）→ **修后** `r-mu9gaq8t-95z2os`（无该警告）；
  `just test -p codex-core-plugins` **438 passed** `r-mu9gcotm-3jzr2d`。

## 二十、第十四批：marketplace_add.rs 12 条全译 + 工具自带 `--self-test`

### 20.1 批次本体

12 条候选全在 `add_marketplace_sync_with_cloner`（返回 `Result<_, MarketplaceAddError>`），该错误进 CLI 的
`plugin marketplace add` 用户面 ⇒ **全译、登记 0**；文件内无 `warn!`。
预检② 无命中（全为新键）；12 站点含 3 组**文件内重复值**（`failed to resolve installed marketplace root: {err}` ×3、
`… is already added from a different source …` ×2）⇒ 实际只新增 **9** 条词条。
对账：`marketplace_add.rs` 12 → **0**；crate：128 → **116**（差额**正好 12**）。

### 20.2 收尾：把「工具自身的判定」变成可跑的断言（补上一轮标记的未完成项）

上一批（§19）修掉 `_used_imports` 的两个缺陷后，我如实记了一条**未完成**：那些自检只是临时脚本，**没有常驻判据**。
本批把它补上：`scripts/i18n_apply.py --self-test` 现在跑 **10 项断言**，覆盖三处曾经出错/易错的判定——

1. `_used_imports` 的四个形态（空 spec / 纯属性（含 `as_str()`）/ 调用点 / 裸 `tr(`）；
2. 占位符索引守护（一致通过、缺项拦、越界拦）；
3. 词典查重谓词（单行命中、折行命中、不存在不命中）。

**两条回执（双向）**：

- 正向：`python3 scripts/i18n_apply.py --self-test` → `self-test ✅ 10 项断言全过`，回执 `r-mu9ghdpi-2tb2mb`；
- **负向控制**：把词边界判定**临时改回旧的子串写法**（`"tr(" in body`），自检立刻红，且**红在正确用例**上
  （`纯属性 ⇒ current+tr_with：got={'tr','tr_with','current'} want={'tr_with','current'}`），
  回执 `r-mu9gh9w4-2lrg3t`（`expectFail` 签发为**通过**——证据是「它确实红了、而且红在正确的地方」）；
  命令内自带备份/还原，事后 `diff` 确认工具与备份一致。

> 两次失败的负向控制尝试也值得记：第一次锚串在 shell 转义里变了形（注入失败），第二次把 `if` 叠成了 `if if`（语法错）。
> 两次都被工具判为**未通过**（`expect` 片段没出现）——**这正是 `expectFail` 该有的行为**：它不接受「随便怎么红都算红」。
> 第三次改成「落文件的补丁脚本 + 只替换谓词部分」才成立。

### 20.3 门禁

`i18n-check` `r-mu9gj9ow-tfhs9e`（3639 词条 / spacing 0 / duplicate 0 / coverage 99.8%）；
`clippy` `r-mu9gmrch-1sywqw`（46 crate / 2m35s）；`just test -p codex-core-plugins` **438 passed** `r-mu9go5zw-o7s314`；
`cargo check -p codex-core-plugins --all-targets` EXIT=0（14.42s）。

## 二十一、第十五批：marketplace_upgrade/activation.rs 12 条全译（0 登记）

### 21.1 接收方

12 条分属两个 `pub(super) fn`：`write_installed_marketplace_metadata`（`:83` `:85`）与
`activate_marketplace_root`（`:97` … `:193`），外加 `installed_marketplace_snapshot_changed_error`（`:204`）。
两者的调用点是 **`marketplace_upgrade.rs:291-292`** 的升级流程 ⇒ 错误经
`upgrade_configured_git_marketplace` → `errors[].message`（§9.1 已证用户面）⇒ **全译、登记 0**。
⚠ 注意与本文件**形态极像**的 `startup_sync.rs`（§8）是**登记**：差别只在**调用链的终点**，不在文案长相。

### 21.2 clippy 第三次抓到 `redundant_clone`（本批 2 处）——并升级了预检

首跑 `clippy` 红：`:189` 与 `:224` 的 `&err.to_string()`

```
error: redundant clone
   &err.to_string(),
   ^^^^^^^^^^^^ help: remove this
   note: this value is dropped without further use
```

根因：这两处的 `err` 来自 `let Err(err) = …` / `if let Err(err) = activation_result`，而它的类型是 **`String`**
（`after_activate() -> Result<(), String>`）⇒ `.to_string()` 成了克隆。改成 `err.as_str()` 后 clippy 绿 `r-mu9gz4zq-l9osvf`。

**预检升级（第 ⑤ 条）**：写 args 前，对每个 `{err}`/`{x}` 占位符**看它绑定的类型**：

- 绑定来自 `io::Error` / `serde_json::Error` / `url::ParseError` / `JoinError` / 自定义 Error ⇒ `&x.to_string()`；
- 绑定来自 `Result<_, String>`（`let Err(err) = …`、`if let Err(err) = …`）或本就是 `String` ⇒ **`x.as_str()`**；
- 绑定是 `&str` ⇒ **直接用**（不 `.as_str()`，那是 `E0658 str_as_str`）。
  ⇒ 这是本会话**第三次**栽在「同名变量不同类型的取值方式」上（§14 `plugin_name: &str` vs `String`；§15 `MarketplaceSource::display() -> String`；本批 `err: String`）。
  **结论**：这类判断**不该靠记忆**——`cargo check` + `clippy` 两道门禁就是判据，代价只是多一轮；本批如实走完了这一轮。

### 21.3 收尾对账与门禁

- `activation.rs`：12 candidates → **0**；crate：116 → **104**（差额**正好 12**）；12 站点含 1 组文件内重复值（`:130`/`:177`）⇒ 新增 **11** 条词条。
- `cargo check --all-targets` EXIT=0（14.70s）；`i18n-check` `r-mu9gu0b1-1x5pmz`（3650 词条 / spacing 0 / duplicate 0 / coverage 99.8%）；
  `clippy` 修前红 `r-mu9guoc0-2ahtpn` → 修后绿 `r-mu9gz4zq-l9osvf`；
  `just test -p codex-core-plugins` **438 passed** `r-mu9h10ch-wvz9xy`。

## 二十二、第十六批：remote_mutations.rs 12 条全译（9 调用点 + 3 属性）

### 22.1 三种形态，同一归宿

接收方：本文件的错误类型是 `RemotePluginOperationError`（其 `kind: Box<RemotePluginOperationErrorKind>`），
而该枚举在 **app-server `request_processors/plugins.rs:1581-1598`** 被逐变体消费（§10.1 已证）⇒ 三类站点**全译**：

| 形态                               | 站点                                                                                                                                                             | 做法                                       |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ |
| `#[error]` 属性                    | `:66` `Bundle`（元组 + `#[source]` 错误类型）⇒ `&_0.to_string()`；`:73` `DisabledByAdmin(String)`、`:75` `NotAvailable(String)` ⇒ **`_0.as_str()`**              | 属性级路线（`extra_edits` + `extra_dict`） |
| `context: "…"` 字面量              | `:119` `:153` `:190` `:241` `:264` `:281` —— 它们是 `Catalog{context, …}` / `Sync{context, …}` 的字段值，经 `#[error("{context}: {source}")]` 渲染进用户可见消息 | `arg`（字面量→`tr(current(), …)`）         |
| `InvalidRequest`/`Internal` 的消息 | `:104`（`marketplace_name: String` ⇒ `.as_str()`）、`:125`（`{err}` + `detail.summary.id` ⇒ `&err.to_string()` + `.as_str()`）、`:230`                           | `format`                                   |

### 22.2 预检⑤ 第一次「先想清楚再写」并奏效

§21 我把「取值方式看绑定类型」立为预检第⑤条。本批**开工前就用上了**：
`:73`/`:75` 的元组字段是 **`String`** ⇒ 直接写 `_0.as_str()`（而不是 `&_0.to_string()`）⇒ **clippy 首次即绿**
（对比 §21：那次先红后修两处 `redundant_clone`）。
⇒ 这说明**把事故写成预检条目是有回报的**；但仍要保留「门禁是判据」的姿态——预检只降低概率，`cargo check` + `clippy` 才是判定。

### 22.3 收尾对账与门禁

- `remote_mutations.rs`：12 candidates → **0**；crate：104 → **92**（差额**正好 12**）；12 条全部新增（无跨文件复用、无文件内重复）。
- `cargo check --all-targets` EXIT=0（16.97s）；`i18n-check` `r-mu9h5o4q-0hr1m7`（3662 词条 / spacing 0 / duplicate 0 / coverage 99.8%）；
  `clippy` `r-mu9h9dq0-cygiau`（46 crate / 2m44s，**首次即绿**）；`just test -p codex-core-plugins` **438 passed** `r-mu9hbd9w-fzmo1t`。

## 二十三、第十七批：marketplace_upgrade.rs 11 条全译

### 23.1 预检①（`current` 遮蔽）**第一次在开工前就生效**

本文件 `:304` 有一行

```rust
    let current = read_configured_git_marketplace(reload_config, &expected.name)?;
    match current { … }
```

—— 局部绑定就叫 `current`，于是**它作用域内的两个站点（`:308` `:312`）如果按常规插入 `current()`，会被遮蔽**（§16 在 `plugin_bundle_archive.rs` 踩过的 `E0618`）。

处置（开工前就分好，而不是事后修）：

- 这 2 条走 **`extra_edits`**，代码里直接写**限定路径** `codex_i18n::current()`；
- 其余 9 条走常规 `translate`（未限定）⇒ 因此工具插入的 `use codex_i18n::current;` **仍被使用**，不会产生 unused import（§19 的教训）。
- 结果：`cargo check -p codex-core-plugins --all-targets` **首次即 EXIT=0**（14.59s），`clippy` **首次即绿**（`r-mu9hixcc-scrzm0`）。

⚠ 作用域边界要**逐条看**：同文件 `:324` 在 `fn read_configured_git_marketplace` 里，`current` **不在**其作用域 ⇒ 那条用常规写法即可。
⇒ 预检① 的价值在本批兑现：**同一类缺陷（§16 遮蔽 / §19 import 位置 / §21 冗余克隆）在写成预检后，连续两轮没再消耗返工轮次**。

### 23.2 站点与取值

| 站点                                             | 形态                        | 取值                                                                                                                                                                 |
| ------------------------------------------------ | --------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `:203` `:260` `:269` `:284` `:287` `:298` `:324` | `format!`                   | 错误类型字段 `&err.to_string()`；`PathBuf`/`Path` 用 `&x.display().to_string()`；**`String` 字段用 `.as_str()`**（`:287` 的 `marketplace_name`、`marketplace.name`） |
| `:216` `:237`                                    | `.to_string()` 的裸字面量   | `arg`                                                                                                                                                                |
| `:308` `:312`                                    | `format!`（在遮蔽作用域内） | `extra_edits` + **限定路径**，`expected.name: String` ⇒ `.as_str()`                                                                                                  |

### 23.3 收尾对账与门禁

- `marketplace_upgrade.rs`：11 candidates → **0**（该文件另有 1 条早先批次的登记行）；crate：92 → **81**（差额**正好 11**）。
- `i18n-check` `r-mu9hfoz3-0a9ee5`（3673 词条 / spacing 0 / duplicate 0 / coverage 99.8%）；
  `clippy` `r-mu9hixcc-scrzm0`（46 crate / 2m24s，首次即绿）；`just test -p codex-core-plugins` **438 passed** `r-mu9hjpwv-jpnad7`。

## 二十四、第十八批：remote_installed_plugin_sync.rs 10 条（3 译 / 7 登记）

### 24.1 接收者判定（本批的核心是逐条追到终点，而不是看字面）

| 站点                                     | 形态                                                             | 终点                                                                                      | 判定     |
| ---------------------------------------- | ---------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | -------- |
| `:82` `LockTimeout` / `:85` `Superseded` | 属性级 `#[error("…")]` on `RemoteInstalledPluginBundleSyncError` | 该枚举经 `manager.rs:1663` `.await?` 传播（`RemotePlugin*Error` 属用户可见面）            | **译**   |
| `:159`                                   | `RemotePluginCatalogError::UnexpectedResponse(format!(…))`       | 同上                                                                                      | **译**   |
| `:242` `:265` `:311`                     | 字符串**就在** `tracing::warn!` 的消息位                         | `warn!`                                                                                   | **登记** |
| `:422` `:428` `:450` `:457`              | `remove_stale_remote_plugin_caches -> Result<(), String>`        | 唯一消费者是 `:326` `if let Err(err) = … { warn!(error = %err, …) }` ⇒ 字符串只进 tracing | **登记** |

⇒ 同一文件内「同一个 `format!` 家族」被分成两类，依据**不是**字符串长相，而是**调用点是不是日志消息位 / 错误的唯一出口是不是 warn!**。

### 24.2 取值与重编号

- `:159` 原文 ``format!("remote installed plugin `{}` has an invalid local cache id: {err}", installed_plugin.plugin.id)``
  ⇒ 命名占位符先编号：`{err}`→`{0}`、空 `{}`→`{1}`，即键 ``remote installed plugin `{1}` has an invalid local cache id: {0}``；
  `args = ["&err.to_string()", "installed_plugin.plugin.id.as_str()"]`（`remote.rs:233` 里 `id: String` ⇒ `.as_str()`）。
- 属性级两条用零参 `tr(current(), "…")`（无 `{N}` 时不必 `tr_with`），键写在 `extra_dict`。
- 中文空格规则按既有先例（`dict_zh.rs:2151` 的 `"线程ID\`{0}\`无效：{1}"`）：CJK 与 ASCII/反引号之间**不加空格**。

### 24.3 对账与门禁

- `remote_installed_plugin_sync.rs`：10 candidates → **0**；crate：81 → **71**（差额**正好 10**）。
- 7 条登记行经 `i18n_dossier_lines.py --fix` 重链后逐条命中源码（`:258 :281 :327 :438 :444 :450 :457`）。
- `i18n-check` `r-mu9hpr7x-xbusvr`（3676 词条 / spacing 0 / duplicate 0 / unused 0 / missing 0 / coverage 99.8%）；
  `clippy` `r-mu9htahh-fs15gi`（162.6s，首次即绿）；`just test -p codex-core-plugins` **438 passed 0 skipped**（`EXIT=0`）。

## 二十五、第十九批：git_policy.rs 8 条（7 译 / 1 登记）+ marketplace_remove.rs 8 条全译

### 25.1 「同一函数的错误字符串有两条链」怎么判（预检④ 的实际遭遇）

`git_policy.rs` 只有一个函数 `configure_trusted_git_repository(&mut Command, &Path) -> Result<(), String>`（`:56`），
它的 8 条候选**同一个出口**，但这个出口有**三个消费者**，终点各不相同：

| 消费者                                | 链                                                                                                           | 终点                                                                                                           |
| ------------------------------------- | ------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `startup_sync.rs:612`                 | `git_ls_remote_head_sha` → `sync_openai_plugins_repo_via_git`（`:185`）→ `:106` → `sync_openai_plugins_repo` | `manager.rs:3265` 线程里 `warn!("failed to sync curated plugins repo: {err}")` ⇒ **日志**（§8.2 已登记的先例） |
| `loader.rs:1907`                      | `run_git_output -> Result<String, String>` → `run_git` → `clone_git_plugin_source`（`:1830`）                | 装插件路径（同类已按 §12.3 处理）                                                                              |
| `marketplace_upgrade/git.rs:28` `:75` | `.then(                                                                                                      |                                                                                                                | configure_trusted_git_repository(...)).transpose()?`→`git_remote_revision`/`clone_git_source`→`marketplace_upgrade.rs:254` `:289` `?`→`upgrade_configured_git_marketplace`的`Err(err)` | `marketplace_upgrade.rs:150` **原样** `errors.push(ConfiguredMarketplaceUpgradeError { message: err })` ⇒ **`outcome.errors`，用户可见** |

**口径**：一个错误字符串只要**存在一条可达用户可见的链**，就判**译**（用户确实会看到它）。
这与「按最窄面登记」相反，理由是本模式的接收者判据是「**UI 是否会渲染它**」——
存在渲染路径即为是；把所有可达面都列举出来（上表）是为了让裁决可复核，而不是靠印象挑一条。
⚠ 注意 `git.rs` **已**是已译文件（第三批 25 条），即同一批 git 失败在升级路径上本来就是中文——
若此处登记，会出现「同一次失败在两条链上一条中文一条英文」的自相矛盾。

### 25.2 唯一登记的一条：写入 `HEAD` 文件的载荷

`:81` `std::fs::write(repository.path().join("HEAD"), "ref: refs/heads/main\n")`
—— 它是**写进 git 内部引用文件的内容**（git 的格式），不是任何 UI 文本，翻译它会**弄坏仓库**。
判据落在「文件载荷 / 协议常量」这一类（§12.6），与它长得像不像句子无关。

### 25.3 marketplace_remove.rs：两个消费者都是用户可见面

`remove_marketplace`（`:36`）的 `Result<_, MarketplaceRemoveError>` 在两处被渲染：
`app-server/src/request_processors/marketplace_processor.rs:56`（JSON-RPC 错误 → 客户端）与
`cli/src/marketplace_cmd.rs:434`（CLI 输出）⇒ 8 条**全译**。
取值：`marketplace_name` 是 `let marketplace_name = request.marketplace_name;`（`:54`，`String`）⇒ `.as_str()`；
`source` 是 `format_config_layer_source(...)`（`String`）⇒ `.as_str()`；`configured_name` 是 `&String` ⇒ `.as_str()`；
io 错误 ⇒ `&err.to_string()`；`root: &Path` ⇒ `&root.display().to_string()`。

### 25.4 对账与门禁

- `git_policy.rs` 8 candidates → 0（余 1 条即 `:81` 登记者）；`marketplace_remove.rs` 8 → 0；
  crate：71 → **55**（差额**正好 16**）。
- `i18n-check` `r-mu9hzlvs-i9xfoe`（3691 词条 / spacing 0 / duplicate 0 / unused 0 / nested 0 / coverage 99.8%）；
  `clippy` `r-mu9i39np-d4ujid`（168.4s，首次即绿）。

## 二十六、第二十批：provider.rs 8 条全登记 + marketplace_add/install.rs 7 条全译

### 26.1 provider.rs：**公开导出**不等于用户可见

`ExecutorPluginProviderError` 在 `lib.rs:103` 是 `pub use`，8 条变体又都是「像 UI 文案」的整句——
但它的**唯一出口**是 `ext/mcp/src/executor_plugin.rs:81-89`：

```rust
let plugin = match self.plugin_provider.resolve_bound(selected_root).await {
    Ok(plugin) => plugin,
    Err(err) => {
        tracing::warn!(selected_root = selected_root.id, error = %err,
                       "failed to resolve selected executor plugin");
        return None;
    }
};
```

⇒ 个体的 8 条错误**只作为 `error = %err` 字段**进入那条 `warn!`，随后 `return None`（调用方拿不到错误）。
**8 条全登记**。判据是「错误的**下游**有没有渲染它的地方」，不是「它是不是 `pub`」。

### 26.2 install.rs：`MarketplaceAddError` 两个消费者都渲染

`app-server/src/request_processors/marketplace_processor.rs:138-139`（`InvalidRequest(message) => invalid_request(message)` /
`Internal(message) => internal_error(message)`）与 `cli/src/marketplace_cmd.rs:162` ⇒ **全译**。
取值：`marketplace_name: &str` 裸传；io 错误 `&err.to_string()`；`Path` 用 `&x.display().to_string()`；
`:124` 的 `args.join(" ")` ⇒ `&args.join(" ")`。

### 26.3 值复用（第 6 次）与 `[duplicate]` 事故

- `:124` 的 `"failed to run git {}: {err}"` 经工具重编号后键为 **`failed to run git {1}: {0}`**——
  **字典里已有**（早先批次写入）⇒ 工具自动「只包站点、不新增词条」，**索引一致**（既有键也是 `{1}`=git 参数、`{0}`=错误）。
- `:133` 是 4 个空 `{}` 的多行 `\n` 拼接（`git {} failed with status {}\nstdout:\n{}\nstderr:\n{}`），
  用 `extra_edits` 手写键 `{0}..{3}` + `extra_dict`。

⚠ **本批踩到一次 `[duplicate]`（`r-mu9i92v1-l9poil` EXIT=1）**：
`extra_dict` 写入的键在 **Rust 源码里是真实换行**，而字典里**早有一条**同位的键把换行写成**字面 `\\n`**
（`dict_zh.rs:8872`）。两条键语义相同、转义不同 ⇒ `HashMap` 后写者覆盖前者，既有条目变 `dead`。
**处置**：删掉 `extra_dict` 那条，**保留既有条目**（既有条目同时决定了渲染出来的转义形态，改它会改变用户可见输出）
⇒ `r-mu9ia0ds-tbke43` EXIT=0、duplicate 0。已登记 `known_issues: i18n-extra-dict-blind-to-escaped-equivalent`。
⇒ **流程修正**：`apply` 之后**先跑 `i18n-check`**（它 7 秒就能报 `[duplicate]`），再去跑分钟级的 `cargo check`/`clippy`。

### 26.4 对账与门禁

- `provider.rs` 8 → 0（8 条登记）；`marketplace_add/install.rs` 7 → 0；crate：55 → **40**。
- `i18n-check` `r-mu9ia0ds-tbke43`（3696 词条 / duplicate 0 / spacing 0 / unused 0 / coverage 99.8%）；
  `clippy` `r-mu9idbyv-csqihs`（145.2s，首次即绿）。

## 二十七、第二十一批：marketplace_add/source.rs 7 条全译 + remote/share.rs 7 条（4 译 / 3 登记）

### 27.1 source.rs：全部是 `MarketplaceAddError` ⇒ 与 §26.2 同链，全译

`parse_marketplace_source`（`:18`）与 `resolve_local_source_path`（`:149`）的 7 条都返回 `MarketplaceAddError`
（app-server `marketplace_processor.rs:138-139` 与 CLI `marketplace_cmd.rs:162` 渲染）⇒ **全译**。
其中 5 条是**无占位符**的纯句（`:25 :35 :41 :62 :78`）⇒ `kind:"arg"`；`:157 :165` 是 io 错误 ⇒ `kind:"format"` + `&err.to_string()`。
`--ref` / `--sparse` 两条键以 `--` 开头，用 `grep -Fc -e` 才不会被当成选项（预检② 的操作细节）。

### 27.2 share.rs：**公开导出的错误 ≠ 它的每个字段都该单独译**

| 站点                  | 形态                                                                                              | 判定                                                                                                                                                                                                     |
| --------------------- | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `:189` `:246` `:344`  | `RemotePluginCatalogError::UnexpectedResponse(format!(…))` / 变体字段                             | **译**（与 §24.1 同链：该枚举在 app-server 与 CLI 都是用户可见面）                                                                                                                                       |
| `:231`                | `UnexpectedResponse(format!("failed to load plugin share local path mapping: {err}"))`            | **译**（此条是 `list_remote_plugin_shares` 的 `?` 出口）                                                                                                                                                 |
| `:270`                | `io::Error::new(InvalidData, format!("invalid remote plugin id in share local path mapping: …"))` | **登记**：它是一个 `io::Error`，经 `local_paths` 冒泡到 app-server `plugins.rs:149`，被那里的 `internal_error(format!("failed to load plugin share local path mapping: {err}"))` 当作 **`err` 字段**渲染 |
| `:432`（`:439` 同值） | `RemotePluginCatalogError::Request/UnexpectedStatus { url: "workspace plugin upload URL" }`       | **登记**：它是 **`url` 字段**（占位标识），渲染模板是 `remote.rs:407-411` 的变体属性，不是这句话本身                                                                                                     |
| `:469`                | `InvalidArchive { reason: "plugin path must end in a valid UTF-8 directory name" }`               | **登记**：同上，`reason` 字段由变体属性模板渲染                                                                                                                                                          |

⚠ 两条口径细节：

1. **`:231` 与 app-server 的 `plugins.rs:154` 是两组不同的字符串**——`:231` 的键带 `{err}`（`RemotePluginCatalogError` 链），
   `plugins.rs:154` 那条键是 app-server 自己的（仍在候选清单里，全仓 app-server 还有上百条）。**不要**因为「看起来同一句话」就以为重复。
2. 本批 `:270` 判登记**不是**「它不重要」，而是「它是别人模板里的 `err` 字段」——被渲染的是**外层模板**，
   而那层模板属于 **app-server 批次**（`i18n_todo --root codex-rs/app-server` 有它）。
   ⇒ 顺带确认了一条**跨 crate 的边界**：`core-plugins` 的批次不替 app-server 的模板收口。

### 27.3 对账与门禁

- `marketplace_add/source.rs` 7 → 0（余 1 条登记者）；`remote/share.rs` 7 → 0（余 3 条登记者）；
  crate：40 → **26**（差额**正好 14**）。
- `i18n-check` `r-mu9iiuhq-j39h0f`（3705 词条 / spacing 0 / duplicate 0 / unused 0 / nested 0 / coverage 99.8%）；
  `clippy` `r-mu9im248-7kqk9i`（147.8s，首次即绿）。

## 二十八、第二十二批：余下 26 条一次收紧（5 译 / 21 登记）——**crate 降到 1**

### 28.1 manifest 系（12 条）全是 serde 自定义消息 ⇒ 登记

| 文件                       | 站点                           | 出口                                                                                                                                                                                                                                                   |
| -------------------------- | ------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `manifest.rs`              | `:498 :517 :530 :549`          | `warn_invalid_default_prompt`（`:556`）= `tracing::warn!("ignoring {field}: {message}")` —— 本行即 **`message` 字段**                                                                                                                                  |
| `agent_plugin_manifest.rs` | `:72 :115 :123 :133 :138 :145` | 这些消息是 **`serde_json::Error` 的自定义文本**，出口是 `manifest.rs:193` 的 `warn!("failed to parse plugin manifest: {err}")`，或 `provider.rs:225` 的 `ExecutorPluginProviderError`（而它的唯一出口又是 `ext/mcp executor_plugin.rs:83` 的 `warn!`） |

⚠ 口径：**「像不像 UI 文案」不能判**。`:145`（"invalid Agent Plugins name `{}`; use lowercase letters..."）读起来像给用户的提示，
但它是 serde 反序列化失败时拼进 `serde_json::Error` 的文本——**没有任何渲染方**（两条链都止于 `warn!`）。
反过来 `provider.rs` 的 8 条（§26.1）同样像 UI 文案而实际全登记。⇒ 判据一律是**终点**。

### 28.2 生成物与文件载荷（2 条登记）

- `command_migration.rs:412`：`format!` 出来的是 **`SKILL.md` 的内容**（`:171` `fs::write(target_dir.join("SKILL.md"), rendered)`）。
  它是**落盘产物**——翻译会改变别人仓库里生成出来的 skill 文本，属 §12.6「文件载荷」（与 `HEAD` 那条同类）。
  但同文件的 `:375`（`Migrated source command \`{source_name}\``）与 `:407`（`"No command template body was found."`）
是**同一份文档里的人读文本**……**本批判为译**：它们进的是**给模型/用户看的 skill 正文**，
与该仓库既有的 `SKILL.md`文案处理一致；而`:412` 那条是**整份文件的模板骨架**（含 YAML frontmatter 与标题结构），
翻译它会改掉文件结构约定（`name:`/`description:` 字段名等），风险与收益不对称 ⇒ 登记。

### 28.3 io/日志系（7 条登记）

`remote/share/local_paths.rs` `:51 :68 :112` 全是 `io::Error::new(..)` / `io::Error::other(..)`；
`discoverable.rs:87` 是 `anyhow::context`——它的唯一终点是 `core/src/session/turn.rs:1643`
`warn!("failed to load discoverable tool suggestions: {err:#}")` 后 `None`。均登记。

### 28.4 唯一的译文件：`tool_suggest_metadata.rs` 3 条 + `metadata.rs` 5 条

- `tool_suggest_metadata.rs` `:140 :224 :227` 返回 `Result<_, MarketplaceError>` ⇒ `InvalidPlugin(message)`
  ⇒ app-server `plugins.rs:2020 marketplace_error(...)` 渲染 ⇒ **译**。
- `marketplace_add/metadata.rs` `:45 :61 :68 :104 :111` 全是 `MarketplaceAddError` ⇒ **译**。

### 28.5 本批的两次返工（都是**我自己违反预检**，记下来）

1. **猜变量名**：`marketplace_add/metadata.rs` 的 4 处我写了 `&path.display().to_string()`，
   真实绑定是 **`config_path`**（`:59` `let config_path = codex_home.join(CONFIG_TOML_FILE);`）
   ⇒ `E0423 expected value, found built-in attribute 'path'` × 4。
   **这正是纪律里「不许在没读过文件的情况下猜参数」的同型错误**——我在 spec 里手写参数表达式时没回读源码。
2. **取值按绑定类型（预检⑤）**：`metadata.rs:49` 的 `marketplace_name` 与 `command_migration.rs:367` 的 `source_name`
   都是 **`&str`**，我写了 `.as_str()` ⇒ `E0658 str_as_str`（与 §17 的 `plugin_name` 完全同型）。
   **教训**：预检⑤ 我此前只对「工具自动生成 args」的站点做了，**手写 `extra_edits`/手写 spec 参数时漏了**。
   ⇒ 已把预检⑤ 扩为「**任何**参数表达式（工具生成或手写）都要按绑定类型核一遍」。

### 28.6 对账与门禁

- crate：26 → **1**（差额**正好 25**；余下 1 条是 `tool_suggest_metadata.rs:227` 的**同值站点**，
  该值在别处已登记 —— 需要单独包一层，属下一批）。
- `i18n-check` `r-mu9iw59a-0arf1x`（3711 词条 / spacing 0 / duplicate 0 / unused 0 / nested 0 / coverage 99.8%）；
  `clippy` `r-mu9j0fp5-bpk5sw`（198.1s，首次即绿）。

### 28.7 一次**期望错**的处置（有反例证据 + journal decision）

`just test -p codex-core-plugins` 首轮 **437 passed / 1 failed**：
`marketplace_add::metadata::tests::installed_marketplace_root_for_source_propagates_config_read_errors`。
断言写死 `contains("failed to read user config {}:")`（「错误消息紧跟冒号」= 改动前的插值顺序），
而中文语序把路径后置（`读取用户配置{0}失败：{1}`）。

**归类为期望错，不是代码错**，反例证据（负向控制，`r-mu9j3lcy-lx6mmy` 输出哈希 `7103c3d255f6`）：
改动前该测试红的输出是
`unexpected error: failed to read user config Is a directory (os error 21): /tmp/.tmpy9WZgE/config.toml`
—— 即**英文侧渲染仍然是 `failed to read user config <path>: <os error>`**（键 `failed to read user config {1}: {0}`
与源码 key 逐字相同、`tr_with` 按同一索引替换），**英文输出逐字节未变**，行为正确。

改法（`metadata.rs:236`）：拆成 `starts_with("failed to read user config ")` + `contains(": <config_path>")` 两条，
**仍然断言到具体路径**（未放宽为「只要不 panic」）。已落 `worklog journal kind:"decision"` `j-mu9j4xmz-ay6p`。
改后 `just test -p codex-core-plugins` **438 passed / 0 failed**（EXIT=0）。

## 二十九、第二十三批：crate 候选**清零**

### 29.1 最后一条：`recommended_plugin_install.rs:16`

```
let remote_plugin_id = plugin.remote_plugin_id.clone().ok_or_else(|| {
    RemotePluginCatalogError::UnexpectedResponse(format!(
        "recommended plugin `{}` is missing remote plugin identity", plugin.id))
})?;
```

`RemotePluginCatalogError::UnexpectedResponse` ⇒ 与 §24.1/§28.4 同链（app-server 与 CLI 都渲染）⇒ **译**，
键 `recommended plugin \`{0}\` is missing remote plugin identity`，args `&[plugin.id.as_str()]`（`DiscoverableTool::Plugin(plugin)`里`id: String`）。

⚠ **定位这一条花了两次**：`i18n_todo --file tool_suggest_metadata.rs` 报的「剩 1 条」是**行号漂移**——
那个文件的 `:226 :229` 早已被包好（`tr(current(), ..)`），候选清单里的行是可执行文件在**上次生成时的行号**。
⇒ **操作细节**：候选用 `--dump-rows`（带当前值）核对真身，别只按 `--file` 的计数下行。

### 29.2 core-plugins 全 crate 收官账

| 阶段                 |      候选 | 本批译 | 本批登记 |
| -------------------- | --------: | -----: | -------: |
| 起点（§7 开工）      |       518 |      — |        — |
| 本会话逐批（§7–§22） |   518 → 1 |      — |        — |
| 第二十三批           | 1 → **0** |      1 |        0 |

- 全 crate：`unwrapped candidates 0`；`wrapped so far 512 / all candidates 531`（差值 19 = 已声明的 `test-fixture-keys.tsv` 与 `not-translated.tsv` 例外）。
- 词典 **3712** 条；`coverage 99.8%`（未译的 9 条是**已声明**的 `not-translated.tsv` 例外）。
- 本次会话累计：**译 ~340 / 登记 ~130**（逐批差额见各节 `对账` 行）。

### 29.3 门禁

`i18n-check` `r-mu9j8258-g2161y`（3712 词条 / missing 0 / unused 0 / duplicate 0 / spacing 0 / nested 0 / placeholder 0 / asset 0）；
`clippy` `r-mu9je9vo-m7jub6`（284.9s，首次即绿）；`just test -p codex-core-plugins` **438 passed / 0 failed**（EXIT=0）。

## 三十、app-server 首批：plugins.rs 22 条全译（新 crate 接入）

### 30.1 新 crate 接入 i18n 必须先加依赖（新限制，已登记）

`cargo check -p codex-app-server` 首跑 **EXIT=101**：`unresolved import \`codex_i18n\``×3 —— 该 crate 的`Cargo.toml`里**没有**`codex-i18n`。处置：在 `[dependencies]`按字母序插入`codex-i18n = { workspace = true }`（放在 `codex-home` 之后）。**`BUILD.bazel`不用动**——`codex_rust_crate`宏从`Cargo.toml` 推导 deps（`core-plugins/BUILD.bazel`仅 15 行、无 deps 列表）。
已登记`known_issues: i18n-new-crate-needs-cargo-dep`（含误判防护）。

### 30.2 app-server 的字符串为什么**全译**

这一层的字符串有四种形态，**四种都落在用户可见面**：

| 形态                                                                                    | 例子                                                     | 渲染点                                                               |
| --------------------------------------------------------------------------------------- | -------------------------------------------------------- | -------------------------------------------------------------------- |
| `invalid_request("…")`                                                                  | `:321 :1219 :1259 :1263 :1268 :1319`                     | `JSONRPCErrorError.message` → 客户端                                 |
| `internal_error(format!("…: {err}"))`                                                   | `:154 :546 :670 :957`                                    | 同上                                                                 |
| `invalid_request(format!("…{var}"))`                                                    | `:1160 :1213`                                            | 同上                                                                 |
| `*_error_to_jsonrpc(err, "action")` / `marketplace_error(err, "action")` 的**动作标签** | `:666 :698 :752 :762 :954 :1039 :1173 :1234 :1289 :1333` | `plugins.rs:2347` `let message = format!("{context}: {err}")` → 同上 |

⇒ 动作标签**不是**日志，它是**用户可见消息的前缀**（`read plugin details: <error>`）⇒ 也译。

### 30.3 本批第三次同型返工：`String` 裸传（预检⑤）

`:1160 :1213` 我写了 `&[remote_marketplace_name]`，但它是 **`String`** ⇒ `E0308 expected \`&str\`, found \`String\``×2。
修法`&[&remote_marketplace_name]`。**这已是本会话第三次同型**（§17 `plugin_name`的`E0658`、§28.5 的 `E0658`、本节 `E0308`）
—— 共同点都是**手写 spec 参数时没回读绑定类型**。⇒ 结论：预检⑤ 必须**逐站点读源码**，不能凭「它看起来像字符串」。

### 30.4 对账与门禁

- `plugins.rs`：42 → **20**（本批 22 条）；crate：705 candidates / wrapped 19 → **41**。
- `i18n-check` `r-mu9jwr99-x14d2h`（3731 词条 / 全零 / coverage 99.8%）；
  `clippy` `r-mu9k193c-bq8prq`（203.9s，首次即绿）；`cargo check -p codex-app-server --all-targets` EXIT=0。

## 三十一、工具修复：`i18n_todo.py` 采纳 `out_of_scope_crates`（第 774 轮）

### 31.1 缺陷与根因

`scripts/i18n_todo.py` 的 `load_scope_exclusions()` 只读 `excluded`（path_prefix 形态），
**从不读**同一文件里的姊妹键 `out_of_scope_crates`（整 crate 形态），尽管函数的 docstring
自己就写着「A ruling that lives only in the ledger/docs is invisible to this counter」。

后果：`codex-rs/app-server`（裁定为范围外，依据 `docs/plan/i18n-design.md` §3.1 依赖图
只列 tui/cli/exec/core/mcp 五条边；其错误串属 JSON-RPC 协议契约）一直排在
「busiest files」**榜首**（644 条），诱导连续两批在范围外工作
（commit `89458ccfa` 译了 `app-server/src/request_processors/plugins.rs` 44 处）。
已登记 `known_issues: i18n-counter-blind-to-out-of-scope-crates`（high）。

### 31.2 修复

`load_scope_exclusions()` 现在把 `out_of_scope_crates` 的键也作为前缀返回：

- `excluded`（doctor：`codex-rs/cli/src/doctor`）+ `out_of_scope_crates`（app-server：`codex-rs/app-server`）
- 修后：`app-server` 视角 `unwrapped candidates: 0`（644 全被排除）；`codex-mcp` 不受影响（仍 107，它是计划内五条边之一）
- 修后总账：`excluded by scope ruling: 2186 candidates`（doctor 851 + app-server 644，去重后 1335 从剩余消失）

⚠ **不改变裁定本身**：app-server 误做的 44 处保留原地，其去留仍是待裁决 `j-muajv7wb-3pvn`。
