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
