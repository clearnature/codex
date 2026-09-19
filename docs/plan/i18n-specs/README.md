# i18n 批次规格（机器可读的裁定与类型依据）

这些 JSON 是 `scripts/i18n_apply.py --spec/--specs` 实际执行过的输入，**逐站点**记录了：

- `translate.<line>.kind` / `.args`：该站点调用的形态与**每个占位符对应的实参表达式**（类型依据可回溯到源码声明处）；
- `register.<line>`：不译的理由（含「哪些站点一并被豁免」）；
- `extra_edits` / `extra_dict`：非机械替换（如 `{:?}` 手工改造）与只经替换引入的词条。

用途：**平台受限批次**（`macos-desktop-app.json` 的 33 处、`windows-and-queue.json` 的 6 处）在本机不编译，
逐实参类型依据必须能被第三方复核；有了这份规格，复核者无需依赖会话记录。

| 文件 | 对应文档 | 内容 |
| --- | --- | --- |
| `macos-desktop-app.json` | §12.68 | `desktop_app/mac.rs` 34 站点（33 译 + 1 登记{codesign 要求串}） |
| `windows-and-queue.json` | §12.67 | `desktop_app/windows.rs` 7 + `queue_cmd.rs` 1（登记：PowerShell 脚本文本） |
| `linux-six-files.json` | §12.66 | `debug_sandbox` / `logs_client` / `sandbox_setup` / `cloud_config` / `exec_server_telemetry` / `lib` 共 34 站点 |
| `tui-last-three.json` | §12.69 | `history_ui.rs:418` 与 `transcript_export.rs:240/:299` 三条登记 |
