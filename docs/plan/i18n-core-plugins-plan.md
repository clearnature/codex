# `codex-plugins` 520 条的推进方案（第 626 轮实测）

> 实测口径：`python3 scripts/i18n_todo.py --root codex-rs/core-plugins/src` ⇒ **518 条 / 32 个文件**
> （先前口头说的 520 是粗数，以本条为准）。

## 一、先定界：这三类各怎么处理

| 类 | 判据（与 §9.1 同源） | 处置 |
| --- | --- | --- |
| **译** | 消息**流向用户面**：`CodexErr::*`（含 `Fatal`）、CLI `plugin_cmd` 的 issue 列表、TUI 警告/事件、`#[error]` 类型被 `?` 抛到这些面 | 包 `tr`/`tr_with`（`thiserror` 属性走 §12.70 的属性级改法） |
| **登记** | 只进日志/遥测/tracing、`--json` 机器契约（与 `doctor` 851 同族）、内部缓存键、喂模型载荷 | 记入 `not-translated-unwrapped.tsv`（按值/站点豁免 + 理由） |
| **不属文案** | 机器语法片段：git 参数（实测样本 `git sparse-checkout marketplace source`）、manifest 键名、命令片段、路径模板 | 同上登记，理由写「机器语法/标识符」（§12.1/§12.7） |

**关键判断**：这条流水线的**真实成本在「接收者分析」**（每条往上找调用链），不在改写（改写已是工具化）。

## 二、形态分布（518 条的实测形状）

| 形态 | 条数 | 占比 | 处理提示 |
| --- | ---: | ---: | --- |
| `format!` / `.to_string()` | 343 | 66% | 多为错误消息构造 ⇒ 需看**所在函数**抛给谁；同函数内的一条分析可覆盖多条 |
| 「其它」（无宏包裹） | 106 | 20% | 抽样多为 **token/标签/命令片段**（`install remote plugin`、`missing url`）与结构体字段 ⇒ 多数是**登记** |
| `#[error(...)]` 属性 | 68 | 13% | **按错误类型成组**（一个类型一次接收者分析）⇒ 效率最高的部分 |
| `.context(...)` | 1 | — | 直接译 |

**热点文件**（累计到 ~130 一批）：`startup_sync.rs` 80、`store.rs` 56、`remote_bundle.rs` 35、`loader.rs` 32、
`remote.rs` 29、`marketplace_upgrade/git.rs` 25、`marketplace.rs` 23、`remote/share/checkout.rs` 23、
`marketplace_policy.rs` 22、`plugin_bundle_archive.rs` 21、`manager.rs` 15、`npm_source.rs` 15 …
（长尾：另 20 个文件各 ≤14 条）

## 三、分批（按累计 ~110–140 条一批，共 4 批）

| 批 | 文件 | 条数 | 备注 |
| --- | --- | ---: | --- |
| **B1** | `startup_sync.rs` + `store.rs` | 136 | 缓存/同步/持久化的内部诊断占多数 ⇒ 预期**登记多于译**；适合当**探针批** |
| **B2** | `remote_bundle.rs` + `loader.rs` + `remote.rs` + `remote/share/checkout.rs` | 119 | 远程插件安装链 ⇒ 会有相当比例流到 CLI 的 issue 列表 |
| **B3** | `marketplace_upgrade/git.rs` + `marketplace.rs` + `marketplace_policy.rs` + `plugin_bundle_archive.rs` + `manager.rs` | 106 | git/npm 参数片段多 ⇒ 登记为主 |
| **B4** | 其余 20 个文件（`npm_source.rs` 15 … 长尾） | 157 | 逐个文件走同一流程 |

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
