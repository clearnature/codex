# i18n-165：合并上游 rust-v0.165.0 作战手册

> 状态：**未开始实际对账**（首试已中止，现场数据完备）。
> 分支 `i18n-165` = `feat/i18n` @ `f9fd6c1dee`（tag `i18n-0.154.0`）。
> 目标：merge `rust-v0.165.0`（`360319d8d`，1834 提交 / 6287 文件）。
> 工具链两侧已同钉 **1.99.1**（rust-toolchain.toml + MODULE.bazel，跳钉已完成）。

## 实测冲突画像（2026-10-29 首试）

- **冲突文件 260 个 / 冲突块 693 个**。
- 两侧 tr 密度：**ours 侧 1283 个 tr 调用，上游侧 0 个** ⇒ 99% 的冲突是
  「我们给上游字符串套了 tr 包装 vs 上游改了这些字符串的周边/内容」的同型冲突。
- 目录分布（文件数）：tui/chatwidget 32、tui/src 32、tui/app 27、bottom_pane 25、
  app-server/request_processors 14、history_cell 12、core/src 10、core/session 8、
  core-plugins 7、cli 6、app-server 6、onboarding 5 ……（其余零散）。
- 骨架文件冲突：`Cargo.toml`、`Cargo.lock`、`MODULE.bazel`、`.github/workflows/repo-checks.yml`。
- 上游侧偶有 import 结构变化（如 catalog_processor 新增 PathUri 系列 import）——
  不能盲选侧，需按块确认。

## 冲突块分类与解法（抽样验证）

| 形态 | 判别 | 解法 |
| --- | --- | --- |
| A. tr 包装 vs 上游字符串微调（~85%） | ours 侧 = `tr(current(), X)`，theirs 侧 = 同位置新字面量 | 取 theirs 新字面量，套 `tr(current(), 新字面量)`；X 需进 dict_zh.rs |
| B. 上游改了代码结构（~10%） | theirs 侧有新调用/新变量，不只是字符串 | 手工：上游结构 + tr 包装新文案 |
| C. 上游删除了该代码（~3%） | theirs 侧为空或整段移除 | 取删除；词典对应键转 unused（i18n-check 会报，需清理） |
| D. 骨架/配置（Cargo.lock、MODULE.bazel、repo-checks） | 非 .rs 或构建文件 | Cargo.lock：取 theirs 后由 cargo 重算；MODULE.bazel：已同钉 1.99.1，取 theirs；repo-checks：保留我们的个人化禁用（blocking/postmerge 已禁 push） |

## 分批执行策略（每批一个会话可完成）

1. **批 0（骨架）**：Cargo.toml/lock、MODULE.bazel、repo-checks.yml、其他非 .rs 冲突。
   Cargo.lock 取 theirs + `cargo check` 让其自洽；MODULE.bazel 取 theirs；repo-checks 手工对齐。
2. **批 1-N（按目录）**：tui（chatwidget/app/bottom_pane/history_cell 分小批）、
   app-server/request_processors、core、core-plugins、cli、其余。每文件：
   theirs 骨架 + tr 重放；完成后 `cargo check -p <crate>` 必须过再进下一文件。
3. **收尾对账**：`just i18n-check`（missing=新文案已包但未译→补 dict_zh.rs+重跑
   gen_hant；unused=上游删掉的键→清理词典）；`python3 scripts/i18n_todo.py` 扫上游
   新增未包文案；`i18n_dossier_lines.py --fix`；tui 套件；bazel 三 crate；云端四 job。
4. **完成后**：tag `i18n-0.165.0`，feat/i18n fast-forward 到 i18n-165。

## 已知注意点

- clippy 用 **1.99.1** 工具链（1.99 新 lint 的修复已全部在 i18n-165 里，
  merge 后新增代码若踩同类 lint 参照本分支既有修法：try_update/as_chunks/
  mem::take/double_must_use allow/手写 From/return Err(eyre!(..))）。
- 词典风格：CJK↔Latin **不加空格**（spacing 门禁）。
- dict_zh.rs 改后：`python3 scripts/i18n_gen_hant.py` 重生成繁体 + dossier_lines --fix。
- 测试写 `set_current` 相关断言时遵守「保存 previous/断言/还原，无 await 让渡」。
- main 已在 165：merge 完成后可对照 main 排查遗漏（`git diff main i18n-165 -- codex-rs` 应只剩 i18n 层与个人设施差异）。

## 现场数据

- 首试 merge 于 2026-10-29 中止（`git merge --abort`），分支已回干净态。
- 冲突统计命令（可复跑）：见本文件上方「实测冲突画像」的统计脚本模式
  （遍历 `git status --porcelain | grep ^UU`，awk 统计两侧 tr 密度）。
