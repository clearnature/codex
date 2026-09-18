//! English source text -> Simplified Chinese.
//!
//! Entries live in [`ENTRIES`] as `(english, translation)` pairs. That shape is
//! load-bearing: `codex-i18n-check` reconciles the keys rendered across the
//! workspace against this array to detect drift, so entries have to stay plain
//! string-literal pairs rather than being built up programmatically.
//!
//! The dictionary grows with the rollout described in
//! `docs/plan/i18n-design.md` §3.4. What is in it today is the first vertical
//! slice: the footer (`tui/src/bottom_pane/footer.rs`). Those entries exist so
//! that "the published language actually reaches rendered output" is a test
//! rather than an inference from the wiring.
//!
//! Conventions for every entry:
//!
//! * the key is the English source text **verbatim**, including the leading
//!   space these hints carry because they are appended after a key binding. A
//!   key that does not match the source character for character is never looked
//!   up, so the leading space is not cosmetic;
//! * a missing entry degrades to the English key (see [`crate::tr`]), never to a
//!   placeholder, so a partial dictionary stays usable;
//! * a translated template keeps exactly the placeholders its English key uses
//!   (`{0}`, `{1}`, ...). `interpolate_tests::interpolated_translations_keep_their_placeholders`
//!   walks this array and fails if that is violated.
//!
//! Deliberately not translated: key bindings and slash commands that are
//! *literal* keystrokes the user must type (`shift+tab`, `/keymap`, `/goal`),
//! and the `Side `-prefixed agent labels, whose prefix is matched structurally.

use std::collections::HashMap;
use std::sync::LazyLock;

/// English source text -> Simplified Chinese.
///
/// `pub(crate)` rather than private because the interpolation tests walk it to
/// check that every entry keeps its English placeholders.
pub(crate) static ENTRIES: &[(&str, &str)] = &[
    // Footer hints (`tui/src/bottom_pane/footer.rs`). The leading space is part
    // of the key: it separates the hint from the key binding rendered before it.
    ("      Importing: none", "      正在导入：无"),
    ("      • +{0} more marketplaces", "      • 还有{0}个市场"),
    ("     - <none>", "     - <无>"),
    ("     reason: {0}", "     原因：{0}"),
    ("     {0}: <empty>", "     {0}: <空>"),
    ("    See the ", "    参见 "),
    ("    note: ", "    备注："),
    ("    • Auth: ", "    • 认证："),
    ("    • Resource templates: ", "    • 资源模板："),
    ("    • Resource templates: (none)", "    • 资源模板：（无）"),
    ("    • Resources: ", "    • 资源："),
    ("    • Resources: (none)", "    • 资源：（无）"),
    ("    • Tools: ", "    • 工具："),
    ("    • Tools: (none)", "    • 工具：（无）"),
    (
        "  Use /mcp verbose for tools and resources.",
        "  使用/mcp verbose查看工具与资源。",
    ),
    (
        "  - max_depth = {0} (V1 only; ignored by V2)",
        "  - max_depth = {0}（仅V1；V2忽略）",
    ),
    ("   Less ", "   较少 "),
    (
        "   No token activity in the last 12 months",
        "   最近12个月没有Token活动",
    ),
    (
        "   Token activity history unavailable",
        "   Token活动历史不可用",
    ),
    (
        "   Widen terminal to show activity graph",
        "   拉宽终端以显示活动图",
    ),
    ("   last 12 months", "   最近12个月"),
    ("  - {0}: {1} (source: {2})", "  - {0}: {1}（来源：{2}）"),
    (
        "  1. Open this link in your browser and sign in",
        "  1. 在浏览器中打开此链接并登录",
    ),
    (
        "  2. Enter this one-time code after you are signed in (expires in 15 minutes)",
        "  2. 登录后输入这个一次性验证码（15分钟内有效）",
    ),
    ("  <none>", "  <无>"),
    (
        "  No experimental features available for now",
        "  目前没有可用的实验性特性",
    ),
    ("  Billed tokens", "  计费Token"),
    (
        "  Continue only if you started this login in Codex. If a website or another person gave you this code, cancel.",
        "  只有当你自己在本机Codex中发起登录时才继续。如果是网站或他人给了你这个验证码，请取消。",
    ),
    (
        "  Choose which local AI server to use for your session.",
        "  选择本次会话要使用的本地AI服务器。",
    ),
    ("  Models", "  模型"),
    ("  No memory settings available", "  没有可用的记忆设置"),
    (
        "  Press Enter to select • Ctrl+C to exit",
        "  按Enter选择 • 按Ctrl+C退出",
    ),
    ("  Reasoning", "  推理"),
    ("  Rename › ", "  重命名 › "),
    ("  Requesting a one-time code...", "  正在请求一次性验证码…"),
    ("  Search › ", "  搜索 › "),
    (
        "  To get started, describe a task or try one of these commands:",
        "  开始使用：描述一个任务，或试试以下命令：",
    ),
    ("  no match", "  无匹配"),
    ("  searching", "  搜索中"),
    ("  {0}. {1} ({2})", "  {0}. {1}（{2}）"),
    ("  • No MCP servers configured.", "  • 未配置MCP服务器。"),
    ("  • No MCP tools available.", "  • 没有可用的MCP工具。"),
    (
        "  • No background terminals running.",
        "  • 没有正在运行的后台终端。",
    ),
    ("  • No sub-agents running.", "  • 没有运行中的子代理。"),
    ("  ● Running  ○ Not Running", "  ● 运行中  ○ 未运行"),
    (" (+ {0} cached)", "（+{0}缓存）"),
    (" (current)", "（当前）"),
    (" (default)", "（默认）"),
    (" (interrupted)", "（已中断）"),
    (" (press ", "（按 "),
    (" (read-only)", "（只读）"),
    (" (reasoning {0})", "（推理{0}）"),
    (" (unanswered)", "（未回答）"),
    (
        " - choose what Codex is allowed to do",
        " - 选择允许Codex做什么",
    ),
    (
        " - choose what model and reasoning effort to use",
        " - 选择模型与推理强度",
    ),
    (
        " - create an AGENTS.md file with instructions for Codex",
        " - 创建AGENTS.md文件，写入给Codex的说明",
    ),
    (
        " - review any changes and find issues",
        " - 审查改动并找出问题",
    ),
    (
        " - show current session configuration",
        " - 显示当前会话配置",
    ),
    (" Token activity", " Token活动"),
    (" close", " 关闭"),
    (" custom · ", " 自定义 · "),
    (" edit last queued message", " 编辑最后一条排队消息"),
    (" edit shortcut · ", " 编辑快捷键 · "),
    (" for agents", " 切换智能体"),
    (" for shortcuts", " 查看快捷键"),
    (" group · ", " 分组 · "),
    (
        " history unavailable | PgUp to retry ",
        " 历史不可用 | 按PgUp重试 ",
    ),
    (" loading older history... ", " 正在加载更早的历史… "),
    (" navigate  ", " 导航  "),
    (" or ", " 或 "),
    (
        " partial history | PgUp for earlier ",
        " 部分历史 | 按PgUp查看更早 ",
    ),
    (" start inspector · ", " 启动检查器 · "),
    (
        " the request for codex network access to ",
        " Codex请求访问以下网络的 ",
    ),
    (" start of history ", " 历史起点 "),
    (" the request to run ", " 请求运行以下内容的 "),
    (" this session", " 本次会话"),
    (" to change", " 切换"),
    (" to configure them.", " 进行配置。"),
    (" to confirm", " 确认"),
    (" to confirm and close", " 确认并关闭"),
    (" to confirm or ", " 确认，或"),
    (" to continue and create a sandbox...", " 以继续并创建沙箱…"),
    (" to continue, ", " 继续，"),
    (" to customize", " 自定义"),
    (" to interrupt and send immediately)", " 可中断并立即发送）"),
    (" to move, ", " 移动，"),
    (" to move, press ", " 移动，按 "),
    (" to queue message", " 排队发送消息"),
    (" to queue", " 排队"),
    (" to review hooks; ", " 审查钩子；"),
    (" to save or select", " 保存或选择"),
    (" to select, ", " 选择，"),
    (" to submit message", " 发送消息"),
    (" to interrupt", " 中断"),
    (" to exit", " 退出"),
    (" again to quit", " 再按一次退出"),
    (" again to edit previous message", " 再按一次编辑上一条消息"),
    (" to edit previous message", " 编辑上一条消息"),
    (" for agents (empty prompt)", " 切换智能体（空提示）"),
    (
        "App server did not report $CODEX_HOME; cannot materialize goal files",
        "应用服务器未报告$CODEX_HOME；无法生成目标文件",
    ),
    (
        "Could not create goal attachment directory {0}",
        "无法创建目标附件目录{0}",
    ),
    (" to toggle", " 切换"),
    (" to toggle, ", " 切换，"),
    (" to toggle; ", " 切换；"),
    (" to trust all; ", " 全部信任；"),
    (" unbound · ", " 未绑定 · "),
    (
        "'/archive' is unavailable in side conversations. Press Ctrl+C to return to the main thread first.",
        "侧会话中不可用'/archive'。请先按Ctrl+C返回主线程。",
    ),
    (
        "'/delete' is unavailable in side conversations. Press Ctrl+C to return to the main thread first.",
        "侧会话中不可用'/delete'。请先按Ctrl+C返回主线程。",
    ),
    (" {0} or ", " 或{0} "),
    (" {0}/{1} answered", " 已答 {0}/{1}"),
    (" · exit {0}", " · 退出码 {0}"),
    ("({0} cached)", "（{0} 已缓存）"),
    ("**Tip:** {0}", "**提示：** {0}"),
    (
        ", OpenAI's command-line coding agent",
        "，OpenAI的命令行编码代理",
    ),
    ("*New* Build faster with Codex.", "*新* 用Codex更快构建。"),
    (
        "*New* Build faster with the **Desktop app**. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true",
        "*新* 用**桌面应用**更快构建。运行'codex app'，或访问https://chatgpt.com/codex?app-landing-page=true",
    ),
    (
        "*New* For a limited time, Codex is included in your plan for free – let’s build together.",
        "*新* 限时福利：你的套餐已免费包含Codex — 一起构建吧。",
    ),
    (
        "*New* Use **/fast** to enable our fastest inference with increased plan usage.",
        "*新* 用 **/fast** 开启最快推理（套餐用量会增加）。",
    ),
    ("+{0} more", "还有{0}个"),
    ("- (best {0}d)", "-（最长{0}天）"),
    ("1 MCP server", "1个MCP服务器"),
    ("1 action.", "1个操作。"),
    (
        "Actions without an active shortcut.",
        "没有生效快捷键的操作。",
    ),
    (
        "Bring over supported setup from another coding agent.",
        "从另一个编码代理迁移受支持的配置。",
    ),
    ("1 app", "1个应用"),
    (
        "1 hook needs review before it can run.",
        "有1个钩子在运行前需要审查。",
    ),
    ("1 hook is new or changed.", "有1个钩子是新增或已变更。"),
    ("1 {0} ago", "1{0}前"),
    ("<< Code review finished >>", "<< 代码审查已完成 >>"),
    ("<audio content>", "<音频内容>"),
    ("<image content>", "<图像内容>"),
    ("<unknown embedded resource>", "<未知嵌入式资源>"),
    ("<unspecified>", "<未指定>"),
    (
        ">> Code review started: {0} <<",
        ">> 代码审查已开始：{0} <<",
    ),
    ("A balanced stack for deep work", "深度工作用的平衡堆栈"),
    (
        "A steady rock when the diff gets large",
        "diff变大时的稳如磐石",
    ),
    (
        "A tidy duck for calm workspace days",
        "平静工作日里的整洁小鸭",
    ),
    ("A recap is already being generated.", "已在生成回顾。"),
    ("A tiny blue-screen gremlin", "蓝屏小精灵"),
    (
        "Active background terminals block /cd.",
        "有活动的后台终端，/cd被阻止。",
    ),
    (
        "Add Codex files alongside your existing project files",
        "在你的现有项目文件旁添加Codex文件",
    ),
    ("Acknowledge findings and continue", "确认发现并继续"),
    ("Add a short status model", "添加简短的状态模型"),
    ("Advanced Reasoning", "高级推理"),
    ("Agent command center", "代理指挥中心"),
    ("Agent errored", "代理出错"),
    ("Agent resume failed", "代理恢复失败"),
    (
        "Agent session {0} is unavailable: {1}",
        "代理会话{0}不可用：{1}",
    ),
    ("Agent spawn failed", "代理启动失败"),
    (
        "Ask a workspace admin to confirm plugin access.",
        "请联系工作区管理员确认插件访问权限。",
    ),
    (
        "Ask a workspace admin to enable Codex plugins or plugin sharing.",
        "请联系工作区管理员启用Codex插件或插件共享。",
    ),
    (
        "Ask the sharer or a workspace admin to confirm plugin access.",
        "请联系共享者或工作区管理员确认插件访问权限。",
    ),
    (
        "Apply the latest diff produced by Codex agent as a `git apply` to your local working tree",
        "把Codex代理生成的最新diff以 `git apply` 应用到本地工作树。",
    ),
    (
        "Agent thread {0} could not be resumed live. Replaying saved transcript.",
        "代理线程{0}无法实时恢复。正在回放已保存的记录。",
    ),
    (
        "Agent thread {0} is already active.",
        "代理线程{0}已经处于活动状态。",
    ),
    (
        "Agent thread {0} is closed. Replaying saved transcript.",
        "代理线程{0}已关闭。正在回放已保存的记录。",
    ),
    (
        "An experimental feature save is still in progress. Retry after it finishes.",
        "实验性特性保存仍在进行中。请在完成后重试。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: `ctrl-z` is reserved for suspending the terminal on Unix. Choose a different chord and retry.",
        "`{0}` = `{1}` 有歧义：`ctrl-z` 在Unix上保留用于挂起终端。请换一组和弦后重试。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: its prefix shadows `{2}`. Unbind or remap the existing shortcut before using it as a chord prefix.",
        "`{0}` = `{1}` 有歧义：其前缀遮蔽了 `{2}`。请先解除或改绑已有快捷键，再把它用作战和弦前缀。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: plain `esc` is reserved for cancelling a pending chord.",
        "`{0}` = `{1}` 有歧义：单独的 `esc` 保留用于取消未完成的和弦。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: the chord uses the key reserved by `{2}`. Choose a different chord and retry.",
        "`{0}` = `{1}` 有歧义：该和弦使用了 `{2}` 保留的按键。请换一组和弦后重试。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: the same chord is already assigned to `{2}`. Choose a unique chord and retry.",
        "`{0}` = `{1}` 有歧义：同一和弦已分配给 `{2}`。请使用不重复的和弦后重试。",
    ),
    ("All models → {0}", "所有模型 → {0}"),
    ("Alpha", "Alpha"),
    ("Always use current directory", "始终使用当前目录"),
    (
        "Always use current directory ({0})",
        "始终使用当前目录（{0}）",
    ),
    ("Always use session directory", "始终使用会话目录"),
    (
        "Ambiguous `tui.keymap.{2}` bindings: `{0}` shadows `{1}` with the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{2}` 的绑定有歧义：`{0}` 以同一个按键遮蔽了 `{1}`。请在`~/.codex/config.toml`中设置不重复的按键后重试。支持的动作与示例见Codex快捷键文档。",
    ),
    (
        "Ambiguous `tui.keymap.{2}` bindings: `{0}` and `{1}` use the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{2}`的绑定有歧义：`{0}`与`{1}`使用了同一个按键。请在`~/.codex/config.toml`中设置不重复的按键后重试。支持的动作与示例见Codex快捷键文档。",
    ),
    (
        "Ambiguous approval overlay keymap bindings: `{0}` and `{1}` use the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "审批浮层的快捷键绑定有歧义：`{0}`与`{1}`使用了同一个按键。请在`~/.codex/config.toml`中设置不重复的按键后重试。支持的动作与示例见Codex快捷键文档。",
    ),
    ("Answer the questions to continue.", "回答问题以继续。"),
    ("Approvals reviewer: {0}", "审批复核者：{0}"),
    (
        "Archive a saved session by id or session name",
        "按id或会话名归档已保存的会话。",
    ),
    (
        "Attestation generation is not available in TUI.",
        "TUI中不支持生成证明。",
    ),
    ("Ask Codex to do anything", "让Codex做任何事"),
    ("Ask a follow-up question", "继续追问"),
    ("Assigned actions:", "已绑定的操作："),
    ("Auto-reviewer ", "自动审查器 "),
    ("Background terminals", "后台终端"),
    ("Beta", "Beta"),
    ("Blocked", "已阻塞"),
    (
        "Choose a pet to wake in the terminal.",
        "选择要在终端中唤醒的宠物。",
    ),
    (
        "Cannot continue into the new worktree because source configuration changed.",
        "源配置已变更，无法继续进入新的工作树。",
    ),
    (
        "Cannot continue into the new worktree while the source session changed or is unavailable.",
        "源会话已变更或不可用，无法继续进入新的工作树。",
    ),
    (
        "Cannot create a worktree from an explicitly untrusted source.",
        "无法从显式不受信任的来源创建工作树。",
    ),
    (
        "Cannot load the new worktree configuration: {0}",
        "无法加载新的工作树配置：{0}",
    ),
    (
        "Check that you are signed in to the correct workspace and still have access.",
        "请确认你已登录正确的工作区且仍拥有访问权限。",
    ),
    (
        "Browse all agent sessions on the shared local app-server daemon",
        "浏览共享本地app-server守护进程上的所有代理会话。",
    ),
    ("Booting MCP server:", "正在启动MCP服务器："),
    ("Cancel task", "取消任务"),
    ("Cannot change directories: {0}", "无法切换目录：{0}"),
    (
        "Cannot fork into this worktree because developer instructions differ. Start a new conversation instead. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "开发者指令不同，无法派生到该工作树。请改为开始新会话。已在{0}创建未使用的检出；请从源仓库运行 `git worktree remove <checkout-path>` 将其删除。",
    ),
    (
        "Cannot dispatch `{0}`: the keymap action inventory exceeds {1} internal tokens.",
        "无法分派 `{0}`：快捷键动作清单超过{1}个内部标记。",
    ),
    (
        "Cannot dispatch unknown keymap action `{0}`.",
        "无法分派未知的快捷键动作 `{0}`。",
    ),
    ("Cannot load {0}: {1}", "无法加载{0}：{1}"),
    (
        "Cannot register managed worktree ownership: {0}",
        "无法注册托管工作树归属：{0}",
    ),
    (
        "Cannot open external editor: set $VISUAL or $EDITOR before starting Codex.",
        "无法打开外部编辑器：请在启动Codex前设置$VISUAL或$EDITOR。",
    ),
    (
        "Checked features are configured on. Some experimental features take effect only in new tasks or after restarting the Codex server.",
        "已勾选的特性将配置为开启。部分实验性特性仅在新建任务或重启Codex服务器后生效。",
    ),
    (
        "Choose how Codex uses and creates memories. Changes are saved to config.toml",
        "选择Codex如何使用与创建记忆。更改会保存到config.toml",
    ),
    (
        "Changes were saved, but the configured values differ from your selections. A higher-priority setting may override them.",
        "更改已保存，但配置值与你的选择不一致。可能有更高优先级的设置覆盖了它们。",
    ),
    (
        "Cannot safely retry a turn whose input exceeds the bounded history page.",
        "该轮次的输入超出有界历史页范围，无法安全重试。",
    ),
    ("Chat paused as a precaution", "聊天已出于安全考虑暂停"),
    ("Chat sessions", "聊天会话"),
    ("Chat sessions ({0})", "聊天会话（{0}）"),
    ("Chat stopped as a precaution", "聊天已出于安全考虑停止"),
    (
        "Choose a communication style for Codex.",
        "选择Codex的沟通风格。",
    ),
    ("Choose a Markdown filename", "选择Markdown文件名"),
    ("Choose an action", "选择操作"),
    (
        "Choose how you'd like Codex to proceed.",
        "选择你希望Codex如何进行。",
    ),
    ("Choose items to import.", "选择要导入的条目。"),
    (
        "Choose what happens to the current task.",
        "选择如何处理当前任务。",
    ),
    ("Choose what to import", "选择要导入的内容"),
    (
        "Codex may add files to your current project folder.",
        "Codex可能会向当前项目文件夹添加文件。",
    ),
    (
        "Clear local memory files and summaries. Existing threads stay intact.",
        "清除本地记忆文件与摘要。现有线程不受影响。",
    ),
    ("Choose working directory to ", "选择工作目录以"),
    ("Closed", "已关闭"),
    ("Closed an agent", "已关闭一个代理"),
    (
        "Codex CLI\n\nIf no subcommand is specified, options will be forwarded to the interactive CLI",
        "Codex CLI\n\n未指定子命令时，选项会被转发给交互式CLI。",
    ),
    (
        "Codex is currently experiencing high load.",
        "Codex当前负载较高。",
    ),
    (
        "Codex just got an upgrade. Introducing {0}.",
        "Codex刚刚升级。隆重推出{0}。",
    ),
    ("Compacted context", "已压缩上下文"),
    ("Completed", "已完成"),
    ("Completed ", "已完成 "),
    ("Completed `{0}`", "已完成 `{0}`"),
    (
        "Config layer stack (lowest precedence first):",
        "配置层栈（优先级从低到高）：",
    ),
    (
        "Concise, task-focused, and direct.",
        "简洁、聚焦任务、直接。",
    ),
    ("Config key: ", "配置键："),
    ("Connected to your IDE.", "已连接到你的IDE。"),
    ("Contacted", "已联系"),
    ("Context {0}% left", "剩余上下文 {0}%"),
    ("Context {0}% used", "已用上下文 {0}%"),
    ("Continue planning with the model.", "继续与模型一起规划。"),
    ("Continue with Luna Reserve", "使用Luna Reserve继续"),
    (
        "Continue without trusting (hooks won't run)",
        "不信任并继续（钩子不会运行）",
    ),
    ("Conversation history is not saved.", "会话历史未保存。"),
    (
        "Conversation interrupted - tell the model what to do differently. Something went wrong? Hit `/feedback` to report the issue.",
        "对话已中断 — 请告诉模型该如何调整。遇到问题？可用`/feedback`反馈。",
    ),
    ("Copied conversation to clipboard", "已复制会话到剪贴板"),
    (
        "Could not generate a recap. Please try again.",
        "无法生成回顾。请重试。",
    ),
    (
        "Copy the complete Markdown transcript",
        "复制完整的Markdown转录",
    ),
    ("Could not read goal image {0}", "无法读取目标图片{0}"),
    (
        "Could not read goal objective file {0}",
        "无法读取目标内容文件{0}",
    ),
    ("Could not restore session: {0}", "无法恢复会话：{0}"),
    ("Could not write goal file {0}", "无法写入目标文件{0}"),
    (
        "Goal objective file reference is too long: {0} characters. Limit: {1}",
        "目标内容文件引用过长：{0}个字符。上限：{1}",
    ),
    (
        "Goal objective file {0} is not valid UTF-8",
        "目标内容文件{0}不是有效的UTF-8",
    ),
    (
        "Every configurable action currently has a shortcut.",
        "当前每个可配置操作都已绑定快捷键。",
    ),
    (
        "Creating a worktree requires an idle primary session without queued input.",
        "创建工作树需要主会话空闲且没有排队输入。",
    ),
    (
        "Couldn't set up your sandbox with Administrator permissions",
        "无法以管理员权限完成沙箱设置",
    ),
    (
        "Current = your current working directory",
        "当前 = 你当前的工作目录",
    ),
    ("Credits", "额度"),
    ("Current project", "当前项目"),
    (
        "Custom .tmTheme files can be added to the {0} directory.",
        "可以把自定义.tmTheme文件放到{0}目录中。",
    ),
    ("Current project: ", "当前项目："),
    ("Custom permissions", "自定义权限"),
    ("Customize selection", "自定义选择"),
    ("Debugging tools", "调试工具。"),
    (
        "Diagnose local Codex installation, config, auth, and runtime health",
        "诊断本地Codex安装、配置、认证与运行时健康状况。",
    ),
    (
        "Delete local memory files and rollout summaries.",
        "删除本地记忆文件与rollout摘要。",
    ),
    ("Default mode unavailable", "默认模式不可用"),
    ("Detected: ", "识别到："),
    ("Disable terminal pets", "禁用终端宠物"),
    (
        "Disconnected from this task. Any running work continues.",
        "已与此任务断开连接。正在运行的工作会继续。",
    ),
    (
        "Disconnected from this task. The current turn was stopped.",
        "已与此任务断开连接。当前轮次已停止。",
    ),
    (
        "Disconnected from this task. Work may still be running.",
        "已与此任务断开连接。工作可能仍在运行。",
    ),
    ("Discovery was interrupted", "发现过程被中断"),
    ("Discuss a code change", "讨论代码改动"),
    (
        "Discuss a code change (Recommended)",
        "讨论代码改动（推荐）",
    ),
    (
        "Enable this app to use it for the current request.",
        "启用此应用以用于当前请求。",
    ),
    ("Dismiss and keep waiting", "关闭并继续等待"),
    (
        "Earlier messages are available — scroll up to load them",
        "可以加载更早的消息 — 向上滚动以载入",
    ),
    (
        "Earlier messages unavailable — scroll up to retry",
        "更早的消息不可用 — 向上滚动以重试",
    ),
    (
        "Do you trust the contents of this directory? Working with untrusted \n                 contents comes with higher risk of prompt injection. Trusting the \n                 directory allows project-local config, hooks, and exec policies to load.",
        "你信任此目录的内容吗？处理不受信任的内容会带来更高的提示词注入风险。信任该目录后，项目本地的配置、钩子与执行策略才会被加载。",
    ),
    ("Each column = 1 week · tallest ", "每列=1周 · 最高 "),
    ("Enable or disable skills.", "启用或禁用技能。"),
    ("Enable {0}?", "启用{0}？"),
    ("Entered review mode", "已进入审查模式"),
    ("Enterprise-managed config value", "企业托管配置值"),
    ("Error", "错误"),
    ("Error adding directories: {0}", "添加目录出错：{0}"),
    ("Error loading configuration: {0}", "加载配置出错：{0}"),
    ("Execpolicy tooling", "Execpolicy工具。"),
    (
        "Exit Codex and leave the task running",
        "退出Codex并让任务继续运行",
    ),
    ("Exited review mode", "已退出审查模式"),
    (
        "Failed to archive current thread: {0}",
        "归档当前线程失败：{0}",
    ),
    (
        "Failed to attach to fresh app-server thread: {0}",
        "附加到新的app-server线程失败：{0}",
    ),
    (
        "Failed to attach to resumed app-server thread: {0}",
        "附加到已恢复的app-server线程失败：{0}",
    ),
    (
        "External current time is not available in TUI.",
        "TUI中不支持外部当前时间。",
    ),
    (
        "Experimental feature discovery exceeded 10 pages",
        "实验性特性发现超过10页",
    ),
    (
        "Experimental feature page exceeds requested limit",
        "实验性特性页面超过请求的上限",
    ),
    (
        "Experimental feature pagination repeated a cursor",
        "实验性特性分页重复了同一个游标",
    ),
    ("Experimental feature request failed", "实验性特性请求失败"),
    ("Experimental features", "实验性特性"),
    (
        "Experimental features are unavailable until startup completes.",
        "启动完成前无法使用实验性特性。",
    ),
    ("Export conversation", "导出对话"),
    ("Extra high", "极高"),
    ("Failed to change: {0}", "切换失败：{0}"),
    (
        "Failed to name the worktree session: {0}",
        "无法命名工作树会话：{0}",
    ),
    (
        "Failed to delete current thread: {0}",
        "删除当前线程失败：{0}",
    ),
    (
        "Failed to load background task settings: {0}",
        "加载后台任务设置失败：{0}",
    ),
    ("Failed to name the new session: {0}", "命名新会话失败：{0}"),
    (
        "Failed to open browser for {0}: {1}",
        "无法为{0}打开浏览器：{1}",
    ),
    ("Failed to open editor: {0}", "打开编辑器失败：{0}"),
    (
        "Failed to read new session defaults: {0}",
        "读取新会话默认值失败：{0}",
    ),
    (
        "Failed to open this session in the Desktop app: {0}. Install or launch the Desktop app and try again.",
        "无法在桌面应用中打开此会话：{0}。请安装或启动桌面应用后重试。",
    ),
    ("Failed to refresh shortcuts: {0}", "刷新快捷键失败：{0}"),
    ("Failed to rename task: {0}", "重命名任务失败：{0}"),
    (
        "Failed to save approvals reviewer: {0}",
        "保存审批复核者失败：{0}",
    ),
    (
        "Failed to start the background server: {0}",
        "启动后台服务器失败：{0}",
    ),
    (
        "Failed to update app config for {0}: {1}",
        "更新{0}的应用配置失败：{1}",
    ),
    (
        "Failed to resume session from {0}: {1}",
        "从{0}恢复会话失败：{1}",
    ),
    (
        "Failed to start a fresh session through the app server: {0}",
        "通过app server启动新会话失败：{0}",
    ),
    (
        "Failed to switch side conversation: {0}",
        "切换侧会话失败：{0}",
    ),
    (
        "Failed to view thread open elsewhere: {0}",
        "查看在其他位置打开的线程失败：{0}",
    ),
    (
        "Failed to save experimental features. Reopen /experimental to check configured values before retrying.",
        "保存实验性特性失败。请重新打开/experimental确认已配置的值后再重试。",
    ),
    (
        "Failed to retry with a faster model: original turn is unavailable.",
        "用更快的模型重试失败：原始轮次不可用。",
    ),
    (
        "Failed to retry with a faster model: {0}",
        "用更快的模型重试失败：{0}",
    ),
    ("Failed to save {0} setting: {1}", "保存{0}设置失败：{1}"),
    ("Fast off", "Fast关"),
    ("Fast on", "Fast开"),
    ("Feature discovery was interrupted", "特性发现被中断"),
    (
        "Features were saved, but readback was interrupted",
        "特性已保存，但回读被中断",
    ),
    (
        "Features were saved, but configured values could not be refreshed: {0}",
        "特性已保存，但无法刷新已配置的值：{0}",
    ),
    ("Finished waiting", "等待结束"),
    (
        "Fork a previous interactive session (picker by default; use --last to fork the most recent)",
        "派生之前的交互会话（默认弹出选择器；用--last派生最近一个）。",
    ),
    (
        "For demanding work using multiple agents · highest usage",
        "适合使用多个代理的高强度工作 · 用量最高",
    ),
    (
        "For difficult problems when quality matters more than speed · higher usage",
        "适合质量优先于速度的难题 · 用量较高",
    ),
    ("Fresh thread with this plan.", "用此计划新建线程。"),
    ("Fresh thread. Context: {0}.", "新建线程。上下文：{0}。"),
    (
        "Future messages will include your current IDE selection and open tabs.",
        "后续消息会包含你当前的IDE选中内容与打开的标签页。",
    ),
    ("Generate memories", "生成记忆"),
    (
        "Generate memories from the following threads. Current thread included.",
        "从以下线程生成记忆。包含当前线程。",
    ),
    ("Generate shell completion scripts", "生成shell补全脚本。"),
    ("Generated an image", "已生成图像"),
    (
        "Giving this request a little extra thought",
        "正在为这个请求多想一会儿",
    ),
    (
        "Goal budget reached - the turn was stopped.",
        "已达目标预算 — 本轮已停止。",
    ),
    ("Goal objective must not be empty.", "目标内容不能为空。"),
    (
        "Ignored invalid status line {0}: {1}.",
        "已忽略无效的状态行{0}：{1}。",
    ),
    (
        "Ignored invalid terminal title {0}: {1}.",
        "已忽略无效的终端标题{0}：{1}。",
    ),
    (
        "Hang tight, this may take a few minutes",
        "请稍候，这可能需要几分钟",
    ),
    (
        "Hooks can run outside the sandbox after you trust them.",
        "信任后，钩子可以在沙箱之外运行。",
    ),
    ("Hooks need review", "钩子需要审查"),
    ("Hot path energy for fast iteration", "快速迭代的热路径能量"),
    (
        "If you'd rather not wait, retry with a faster model. It may be less capable of handling complex requests.",
        "如果你不想等待，可以用更快的模型重试。它处理复杂请求的能力可能较弱。",
    ),
    ("IDE context could not be enabled.", "无法启用IDE上下文。"),
    ("IDE context is off.", "IDE上下文已关闭。"),
    ("IDE context is on.", "IDE上下文已开启。"),
    (
        "IDE context was skipped for this message.",
        "本条消息已跳过IDE上下文。",
    ),
    ("Implement this plan?", "要实施此计划吗？"),
    ("Import selected", "导入所选项"),
    ("Import setup", "导入配置"),
    ("Import skills from ", "从以下位置导入技能 "),
    ("Inference: {0} {1} ({2})", "推理：{0} {1}（{2}）"),
    (
        "Input disabled until setup completes.",
        "设置完成前无法输入。",
    ),
    ("Inspect feature flags", "查看特性开关。"),
    ("Inspect keypresses", "检查按键"),
    (
        "Inspect keypresses from your terminal.",
        "检查终端发来的按键。",
    ),
    (
        "Inspect or migrate legacy local sessions to paginated thread history",
        "检查旧版本地会话，或将其迁移为分页线程历史。",
    ),
    (
        "Install this app in your browser, then return here.",
        "在浏览器中安装此应用，然后返回此处。",
    ),
    ("Interacted with ", "已与 "),
    ("Interacted with `{0}`", "已与 `{0}` 交互"),
    ("Interacted with background terminal", "已与后台终端交互"),
    (
        "Interacted with background terminal: {0}",
        "已与后台终端交互：{0}",
    ),
    ("Interrupted", "已中断"),
    ("Interrupted ", "已中断 "),
    ("Interrupted `{0}`", "已中断 `{0}`"),
    (
        "Job: running/completed/failed/expired; Run/Experiment: succeeded/failed/unknown (Recommended when triaging long-running background work and status transitions)",
        "Job：running/completed/failed/expired；Run/Experiment：succeeded/failed/unknown（排查长时间后台任务与状态变化时推荐）",
    ),
    (
        "Invalid `{0}` = `{1}`. Use a single key such as `ctrl-a` or a two-stroke chord such as `ctrl-x ctrl-t`.",
        "`{0}` = `{1}` 无效。请使用单个按键（如 `ctrl-a`）或两段和弦（如 `ctrl-x ctrl-t`）。",
    ),
    (
        "Invalid `{0}` = `{1}`: `backspace` is reserved for editing task input.",
        "`{0}` = `{1}` 无效：`backspace` 保留用于编辑任务输入。",
    ),
    (
        "Invalid `{0}` = `{1}`: a chord prefix outside Vim must use ctrl, alt, or a non-character key so ordinary text input is not intercepted.",
        "`{0}` = `{1}` 无效：Vim之外的和弦前缀必须使用ctrl、alt或非字符键，以免拦截普通文本输入。",
    ),
    (
        "Invalid `{0}` = `{1}`: a ctrl-alt character prefix may be AltGr text input on Windows. Choose a different chord and retry.",
        "`{0}` = `{1}` 无效：ctrl-alt字符前缀在Windows上可能是AltGr文本输入。请换一组和弦后重试。",
    ),
    (
        "Invalid `{0}` = `{1}`. Use values like `ctrl-a`, `shift-enter`, or `page-down`. See the Codex keymap documentation for supported actions and examples.",
        "`{0}` = `{1}` 无效。请使用`ctrl-a`、`shift-enter`或`page-down`这类取值。支持的动作与示例见Codex快捷键文档。",
    ),
    (
        "Join the OpenAI community Discord: http://discord.gg/openai",
        "加入OpenAI社区Discord：http://discord.gg/openai",
    ),
    ("Keep waiting", "继续等待"),
    ("Keep {0} disabled.", "保持禁用{0}。"),
    ("Keymap", "快捷键"),
    (
        "No configurable actions are available in this group.",
        "此分组没有可配置的操作。",
    ),
    (
        "Launch the Desktop app (opens the app installer if missing)",
        "启动桌面应用（缺失时打开安装程序）。",
    ),
    ("Keypress Inspector", "按键检查器"),
    ("LM Studio", "LM Studio"),
    ("Last 30 days of chats", "最近30天的聊天"),
    ("Last message", "最后一条消息"),
    ("Latest activity", "最新活动"),
    ("Layer value", "层值"),
    (
        "Legacy command approval requests are not available in TUI yet.",
        "TUI中尚不支持旧版命令审批请求。",
    ),
    (
        "Legacy patch approval requests are not available in TUI yet.",
        "TUI中尚不支持旧版补丁审批请求。",
    ),
    (
        "Learn more <https://developers.openai.com/codex/windows>",
        "了解更多 <https://developers.openai.com/codex/windows>",
    ),
    ("Learn more: ", "了解更多： "),
    ("List skills", "列出技能"),
    ("Loading MCP inventory", "正在加载MCP清单"),
    ("Loading MCP inventory...", "正在加载MCP清单…"),
    ("Loading earlier messages...", "正在加载更早的消息…"),
    ("Loading preview...", "正在加载预览…"),
    ("Loading server experiments…", "正在加载服务器实验…"),
    ("Loading {0} plugins.", "正在加载{0}插件。"),
    ("Loading {0} plugins...", "正在加载{0}插件…"),
    (
        "Local LM Studio server (default port 1234)",
        "本地LM Studio服务器（默认端口1234）",
    ),
    (
        "Local Ollama server (Responses API, default port 11434)",
        "本地Ollama服务器（Responses API，默认端口11434）",
    ),
    ("Local tools: {0} {1} ({2})", "本地工具：{0} {1}（{2}）"),
    ("Longest task", "最长任务"),
    ("M Studio", "M Studio"),
    ("MCP Tools", "MCP工具"),
    ("MCP server", "MCP服务器"),
    ("MCP servers", "MCP服务器"),
    ("MCP {0} on {1}", "在{1}上调用MCP {0}"),
    ("MDM value", "MDM值"),
    (
        "Managed worktrees are only supported for local sessions.",
        "托管工作树仅支持本地会话。",
    ),
    ("Manage Codex plugins", "管理Codex插件。"),
    (
        "Manage external MCP servers for Codex",
        "管理Codex的外部MCP服务器。",
    ),
    ("Manage login", "管理登录。"),
    (
        "Message too long; limit {0} characters",
        "消息过长；上限{0}个字符",
    ),
    (
        "Move up/down to live preview themes",
        "上下移动即可实时预览主题",
    ),
    (
        "Messages to be submitted after next tool call",
        "将在下次工具调用后提交的消息",
    ),
    (
        "Messages to be submitted at end of turn",
        "将在本轮结束时提交的消息",
    ),
    ("Monthly credit limit", "每月额度限额"),
    ("Needs attention", "需要关注"),
    ("Needs input", "需要输入"),
    ("New chat", "新建聊天"),
    ("New task", "新建任务"),
    (
        "No action is required. Codex will keep waiting, and this menu will close when the response is ready.",
        "无需任何操作。Codex会继续等待，响应就绪后此菜单会自动关闭。",
    ),
    (
        "No active session found matching '{0}'.",
        "找不到与'{0}'匹配的活动会话。",
    ),
    ("No agents completed yet", "还没有代理完成"),
    ("No approved plan available", "没有已批准的计划"),
    ("No changes", "无变更"),
    ("No personality instructions.", "无个性指令。"),
    ("No pet will be shown.", "不会显示宠物。"),
    ("No questions", "没有问题"),
    ("No recent activity yet.", "还没有最近活动。"),
    (
        "No saved session found with ID {0}. Run `codex {1}` without an ID to choose from existing sessions.",
        "找不到ID为{0}的已保存会话。可运行 `codex {1}`（不带ID）从现有会话中选择。",
    ),
    ("No server experiments available.", "没有可用的服务器实验。"),
    ("No shortcuts in this group", "此分组没有快捷键"),
    ("No skills available.", "没有可用的技能。"),
    ("No transcript content available", "没有可显示的转录内容"),
    ("No unbound shortcuts", "没有未绑定的快捷键"),
    (
        "Open a live inspector that shows the detected key, config key, and matching actions.",
        "打开实时检查器，显示识别到的按键、配置键名与匹配的操作。",
    ),
    (
        "Press Enter to start. Then press any key to inspect it; Ctrl+C exits.",
        "按Enter开始。之后按任意键即可检查；Ctrl+C退出。",
    ),
    (
        "See the key Codex detects and any shortcuts assigned to it.",
        "查看Codex识别到的按键及其绑定的快捷键。",
    ),
    ("No, quit", "不，退出"),
    ("No, stay in Plan mode", "不，留在计划模式"),
    ("None of the above", "以上都不是"),
    ("Not found", "未找到"),
    (
        "Note: You’re in a subdirectory of a Git project. Trusting will apply to the repository root: {0}",
        "注意：你位于某个Git项目的子目录中。信任将应用到仓库根目录：{0}",
    ),
    (
        "OSS provider selection was cancelled by user",
        "用户取消了开源提供方选择",
    ),
    ("Objective: {0}", "目标：{0}"),
    ("Ollama (Chat)", "Ollama（Chat）"),
    ("Ollama (Responses)", "Ollama（Responses）"),
    (
        "Opened this session in the Desktop app.",
        "已在桌面应用中打开此会话。",
    ),
    ("Open task to review.", "打开任务以查看。"),
    ("Opened {0} in your browser.", "已在浏览器中打开{0}。"),
    ("Option 1", "选项1"),
    ("Option 2", "选项2"),
    ("Option 3", "选项3"),
    ("Or run {0} and select {1}.", "或运行{0}并选择{1}。"),
    ("Other (write an answer)", "其他（自行作答）"),
    (
        "Paste an image with Ctrl+V to attach it to your next message.",
        "按Ctrl+V粘贴图片，附加到下一条消息。",
    ),
    ("Pending init", "等待初始化"),
    (
        "Pets are disabled in Zellij. Terminal images don’t stay reliably pane-local in Zellij. Run Codex outside Zellij to use pets.",
        "Zellij中已禁用宠物。终端图像在Zellij中无法可靠地局限于单个窗格。请在Zellij之外运行Codex以使用宠物。",
    ),
    (
        "Pets are disabled in tmux. Terminal images don’t stay pane-local in tmux and can corrupt scrollback or move between panes. Run Codex outside tmux to use pets.",
        "tmux中已禁用宠物。终端图像在tmux中无法局限于单个窗格，可能破坏回滚缓冲或在窗格间移动。请在tmux之外运行Codex以使用宠物。",
    ),
    (
        "Pets aren’t available in this terminal. Terminal pets need image support, and this terminal environment doesn’t expose a supported image protocol. Try a terminal with Kitty graphics or Sixel support, or run Codex outside tmux.",
        "此终端不支持宠物。终端宠物需要图像支持，而当前终端环境未提供受支持的图像协议。请改用支持Kitty图形或Sixel的终端，或在tmux之外运行Codex。",
    ),
    (
        "Pets require iTerm2 3.6 or newer. Upgrade iTerm2 to use terminal pets.",
        "宠物需要iTerm2 3.6或更高版本。请升级iTerm2以使用终端宠物。",
    ),
    (
        "Plugin sharing is disabled for this Codex session. Enable plugin sharing to load shared plugins.",
        "本次Codex会话已禁用插件共享。请启用插件共享以加载共享插件。",
    ),
    (
        "Permission profile cannot be preserved by /cd.",
        "/cd无法保留权限配置文件。",
    ),
    (
        "Permanently delete a saved session by id or session name",
        "按id或会话名永久删除已保存的会话。",
    ),
    (
        "Personality selection is disabled until startup completes.",
        "启动完成前无法选择个性。",
    ),
    ("Plugin · {0}", "插件 · {0}"),
    (
        "Press any key to see what Codex receives. Esc is inspected; Ctrl+C closes.",
        "按任意键查看Codex收到的内容。Esc会被检查；Ctrl+C关闭。",
    ),
    ("Preparing device code login", "正在准备设备码登录"),
    (
        "Press Tab to queue a message when a task is running; otherwise it sends immediately (except `!`).",
        "任务运行时按Tab可排队一条消息；否则会立即发送（`!`除外）。",
    ),
    ("Press enter to continue", "按Enter继续"),
    ("Preview unavailable", "预览不可用"),
    (
        "Queue a message for an existing session",
        "为现有会话排队一条消息。",
    ),
    ("Projects ({0})", "项目（{0}）"),
    ("Question requested", "有新的提问请求"),
    ("Question {0}/{1}", "问题 {0}/{1}"),
    ("Questions {0}/{1} answered", "问题 已答 {0}/{1}"),
    ("Queued follow-up inputs", "排队中的后续输入"),
    (
        "Queued message {0} for thread {1}.",
        "已为线程{1}排队消息{0}。",
    ),
    ("Quiet signal from the void", "来自虚空的安静信号"),
    ("Raw event: ", "原始事件："),
    (
        "Raw output mode off: rich transcript rendering restored.",
        "原始输出模式已关闭：已恢复富文本转录渲染。",
    ),
    (
        "Raw output mode on: transcript text is shown for clean terminal selection.",
        "原始输出模式已开启：显示纯文本转录，便于在终端中干净地选取。",
    ),
    ("Ready", "就绪"),
    (
        "Reasoning is already at the highest level ({0}).",
        "推理强度已是最高档（{0}）。",
    ),
    (
        "Reasoning is already at the lowest level ({0}).",
        "推理强度已是最低档（{0}）。",
    ),
    (
        "Reasoning shortcuts are disabled until startup completes.",
        "启动完成前无法使用推理快捷键。",
    ),
    (
        "Reasoning shortcuts are unavailable for {0}.",
        "{0}不支持推理快捷键。",
    ),
    ("Recent chat sessions", "最近的聊天会话"),
    ("Reconnect: {0}", "重新连接：{0}"),
    (
        "Requested directory or permissions not applied.",
        "请求的目录或权限未应用。",
    ),
    (
        "Remove stored authentication credentials",
        "删除已存储的认证凭据。",
    ),
    (
        "Resume a previous interactive session (picker by default; use --last to continue the most recent)",
        "恢复之前的交互会话（默认弹出选择器；用--last继续最近一个）。",
    ),
    (
        "Removed custom shortcut for `{0}.{1}`.",
        "已删除 `{0}.{1}` 的自定义快捷键。",
    ),
    (
        "Reconnected. No input was resent. Review uncertain submissions before retrying; recovered queues remain paused.",
        "已重新连接。没有重发任何输入。重试前请先检查不确定的提交；恢复的队列仍处于暂停状态。",
    ),
    (
        "Reconnecting — agent list is stale",
        "正在重新连接 — 代理列表可能已过期",
    ),
    ("Refactor", "重构"),
    ("Remote ID {0}", "远端ID {0}"),
    ("Rename › ", "重命名 › "),
    ("Reset all memories", "重置所有记忆"),
    ("Reset all memories?", "重置所有记忆？"),
    (
        "Respond to the MCP server request to continue.",
        "响应MCP服务器请求以继续。",
    ),
    (
        "Respond to the tool suggestion to continue.",
        "响应工具建议以继续。",
    ),
    ("Responses API inference: {0}", "Responses API推理：{0}"),
    ("Responses API overhead: {0}", "Responses API开销：{0}"),
    ("Resume another chat", "恢复另一个聊天"),
    ("Resumed an agent", "已恢复一个代理"),
    ("Retry with a faster model", "用更快的模型重试"),
    ("Return to memory settings.", "返回记忆设置。"),
    ("Review a diff", "审查diff"),
    ("Review findings", "查看发现"),
    ("Review hooks", "查看钩子"),
    ("Review selection", "查看选择"),
    ("Review the diff", "审查该diff"),
    (
        "Run /review to get a code review of your current changes.",
        "运行/review对当前改动做代码审查。",
    ),
    ("Run Codex non-interactively", "以非交互方式运行Codex。"),
    (
        "Run a code review non-interactively",
        "以非交互方式运行代码审查。",
    ),
    (
        "Run commands within a Codex-provided sandbox",
        "在Codex提供的沙箱中运行命令。",
    ),
    (
        "Run `codex app` to open the Desktop app (it installs on macOS if needed).",
        "运行 `codex app` 打开桌面应用（必要时会在macOS上自动安装）。",
    ),
    ("Run in background", "在后台运行"),
    ("Run targeted tests", "运行针对性测试"),
    ("Run tests", "运行测试"),
    ("Running", "运行中"),
    (
        "Save on the server for new threads. This thread is unchanged.",
        "在服务器上保存以用于新线程。当前线程不受影响。",
    ),
    (
        "Saving experimental features timed out; the write may still finish. Reopen /experimental to check.",
        "保存实验性特性超时；写入可能仍会完成。请重新打开/experimental确认。",
    ),
    ("Save conversation", "保存对话"),
    (
        "Save the complete conversation as Markdown",
        "将完整对话保存为Markdown",
    ),
    ("Save to file", "保存到文件"),
    ("Saving experimental features…", "正在保存实验性特性…"),
    (
        "Saving was interrupted. Reopen /experimental to check configured values.",
        "保存被中断。请重新打开/experimental检查已配置的值。",
    ),
    (
        "Saving… Closing this popup will not cancel the write.",
        "正在保存…关闭此弹窗不会取消写入。",
    ),
    ("Search › ", "搜索 › "),
    (
        "See the Codex keymap documentation for supported actions and examples.",
        "支持的操作与示例见Codex快捷键文档。",
    ),
    ("Select Personality", "选择个性"),
    ("Select Pet", "选择宠物"),
    ("Select Syntax Theme", "选择语法主题"),
    ("Select an open-source provider", "选择开源提供方"),
    ("Select provider?", "选择提供方？"),
    ("Selected {0} of {1} {2}.", "已选择{0}/{1}个{2}。"),
    (
        "Selections retained. Save to retry, or cancel to close.",
        "已保留选择。保存以重试，或取消以关闭。",
    ),
    ("Sent input to", "已发送输入到"),
    ("Sent input to an agent", "已向代理发送输入"),
    (
        "Session = latest cwd recorded in the {0} session",
        "会话 = 记录在{0}次会话中的最新cwd",
    ),
    ("Session ID: {0}", "会话ID：{0}"),
    ("Session archived: {0}", "会话已归档：{0}"),
    ("Session runtime:", "会话运行时："),
    (
        "Set up the Codex agent sandbox to protect your files and control network access. Learn more <https://developers.openai.com/codex/windows>",
        "设置Codex代理沙箱，以保护你的文件并控制网络访问。了解更多 <https://developers.openai.com/codex/windows>",
    ),
    ("Setting up sandbox...", "正在设置沙箱…"),
    (
        "Settings, instructions, integrations, agents, commands, and skills",
        "设置、指令、集成、代理、命令与技能",
    ),
    ("Shell mode", "Shell模式"),
    ("Ship it", "发布"),
    ("Shutdown", "已停止"),
    (
        "Sign in to ChatGPT, then try loading this section again.",
        "请先登录ChatGPT，然后重新加载此分区。",
    ),
    (
        "Sign in with ChatGPT auth; API key auth cannot load remote plugin catalogs.",
        "请用ChatGPT账号登录；API key认证无法加载远程插件目录。",
    ),
    ("Shutting down...", "正在关闭…"),
    ("Slash commands", "斜杠命令"),
    ("Small green shoots for new ideas", "新想法的嫩绿幼苗"),
    ("Spawned", "已启动"),
    ("Spawned an agent", "已启动一个代理"),
    (
        "Start a fresh idea with /new; the previous session stays in history.",
        "用/new开启新话题；上一次会话仍保留在历史中。",
    ),
    ("Started", "已启动"),
    ("Started ", "已启动 "),
    ("Started `{0}`", "已启动 `{0}`"),
    (
        "Stop the current turn: run {0}, select this task, and {1}.",
        "停止当前轮次：运行{0}，选择此任务，然后{1}。",
    ),
    (
        "Stop the current task and exit Codex",
        "停止当前任务并退出Codex",
    ),
    (
        "Stop the current task and stay in Codex",
        "停止当前任务并留在Codex",
    ),
    (
        "Still waiting? If nothing changes when you press a key, your terminal is not sending that key to Codex. Only received keys can be assigned as shortcuts.",
        "还在等待？如果按键后没有任何变化，说明你的终端没有把该按键发送给Codex。只有接收到的按键才能被指定为快捷键。",
    ),
    ("Starting MCP servers", "正在启动MCP服务器"),
    (
        "Startup did not finish binding a thread to this worktree: {0}\nThe checkout was kept. Inspect it and confirm no session is using it.\nTo remove it, run `git worktree remove <checkout-path>` from the source repository,\nreplacing <checkout-path> with the path above. Do not use --force.",
        "启动未能把线程绑定到该工作树：{0}\n已保留检出。请检查并确认没有会话在用。\n如需删除，请在源仓库运行 `git worktree remove <checkout-path>`，\n把 <checkout-path> 替换为上面的路径。不要使用 --force。",
    ),
    ("Stop and retry", "停止并重试"),
    ("Stop this attempt and retry?", "停止本次尝试并重试？"),
    (
        "Stopping all background terminals.",
        "正在停止所有后台终端。",
    ),
    ("Stream", "流"),
    ("Streams", "流"),
    ("Sub-agents running", "运行中的子代理"),
    (
        "Switch to the matching workspace or ask the sharer for access.",
        "请切换到匹配的工作区，或向共享者申请访问权限。",
    ),
    (
        "Submit with {0} unanswered {1}.",
        "仍有{0}个未回答的{1}，仍要提交。",
    ),
    (
        "Switch to Default and start coding.",
        "切换到默认模式并开始编码。",
    ),
    (
        "Switch models or reasoning effort quickly with /model.",
        "用/model快速切换模型或推理强度。",
    ),
    ("T R A N S C R I P T", "转录"),
    ("TBT: {0}", "TBT：{0}"),
    ("TTFT: {0}", "TTFT：{0}"),
    ("Task encountered an error.", "任务遇到错误。"),
    ("Task is still running", "任务仍在运行"),
    ("Tasks {0}/{1}", "任务 {0}/{1}"),
    ("Terminal pets disabled", "终端宠物已禁用"),
    (
        "The new worktree is not trusted; run Codex there.",
        "新的工作树尚未受信任；请在那里运行Codex。",
    ),
    (
        "The Desktop app is only available on macOS and Windows",
        "桌面应用仅在macOS和Windows上可用",
    ),
    (
        "The initial thread may have been created, but its ID was not received. Nothing was retried. Your prompt is editable; inspect your tasks before relaunching.",
        "初始线程可能已创建，但没有收到它的ID。没有重试任何操作。你的提示词仍可编辑；重新启动前请先检查任务。",
    ),
    ("The original Codex companion", "最初的Codex伙伴"),
    (
        "The rename target disappeared. Unsubmitted title: {0}",
        "重命名目标已消失。未提交的标题：{0}",
    ),
    (
        "There is no conversation history to recap.",
        "没有可供回顾的会话历史。",
    ),
    (
        "The server did not advertise experimental feature `{0}`",
        "服务器未声明实验性特性 `{0}`",
    ),
    ("Thinking", "思考中"),
    (
        "This directory is not trusted; run Codex there.",
        "此目录尚未受信任；请在那里运行Codex。",
    ),
    (
        "This clears local memory files and rollout summaries for the current Codex home.",
        "这将清除当前Codex主目录的本地记忆文件与rollout摘要。",
    ),
    (
        "This will stop the current attempt and retry in a new thread. Any file changes or other actions already taken will remain.",
        "这会停止当前尝试并在新线程中重试。已产生的文件变更或其他操作会保留。",
    ),
    (
        "Tip: Codex can only inspect keys your terminal sends.",
        "提示：Codex只能检查终端实际发送的按键。",
    ),
    (
        "This conversation is unavailable. Its cached transcript and draft remain here; input is paused. Open the agent picker or return to the parent to continue.",
        "此对话当前不可用。缓存的转录与草稿仍保留在此；输入已暂停。请打开代理选择器或返回父级以继续。",
    ),
    ("Thread usage", "线程用量"),
    (
        "Tip: press $ to open this list directly.",
        "提示：按$直接打开此列表。",
    ),
    ("Time: {0}.", "耗时：{0}。"),
    ("To continue this session, run:", "要继续此会话，请运行："),
    ("Token usage so far:", "目前Token用量："),
    ("Token usage:", "Token用量："),
    (
        "Token usage: total={0} input={1}{2} output={3}{4}",
        "Token用量：总计={0} 输入={1}{2} 输出={3}{4}",
    ),
    ("Tokens: {0}/{1}.", "Token：{0}/{1}。"),
    ("Tool {0}", "工具{0}"),
    (
        "Try again later; local plugin functionality is still available.",
        "请稍后重试；本地插件功能仍可使用。",
    ),
    (
        "Turn skills on or off. Your changes are saved automatically.",
        "开启或关闭技能。更改会自动保存。",
    ),
    ("Tools & setup", "工具与配置"),
    ("Trust all and continue", "全部信任并继续"),
    ("Trusting hooks...", "正在信任钩子…"),
    ("Try new model", "试用新模型"),
    ("Try setting up admin sandbox again", "重试管理员沙箱设置"),
    (
        "Try the **Desktop app** on Linux: install it from https://learn.chatgpt.com/docs/linux/linux-app and run 'chatgpt'.",
        "在Linux上试试**桌面应用**：从https://learn.chatgpt.com/docs/linux/linux-app安装后运行'chatgpt'。",
    ),
    (
        "Try the **Desktop app**. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true",
        "试试**桌面应用**。运行'codex app'，或访问https://chatgpt.com/codex?app-landing-page=true",
    ),
    (
        "Type / to open the command popup; Tab autocompletes slash commands.",
        "输入/打开命令弹窗；Tab可补全斜杠命令。",
    ),
    ("Type a filename and press Enter", "输入文件名后按Enter"),
    ("Type to filter pets...", "输入以筛选宠物…"),
    ("Type to filter themes...", "输入以筛选主题…"),
    ("Type to search shortcuts", "输入以搜索快捷键"),
    (
        "Unarchive a saved session by id or session name",
        "按id或会话名取消归档已保存的会话。",
    ),
    ("Type to search skills", "输入以搜索技能"),
    ("Type your answer", "输入你的回答"),
    ("Unbound ({0})", "未绑定（{0}）"),
    (
        "Update Codex, then try opening the shared plugin again.",
        "请更新Codex，然后重试打开该共享插件。",
    ),
    (
        "Update Codex to the latest version",
        "把Codex更新到最新版本。",
    ),
    ("Updated {0} file(s)", "已更新{0}个文件"),
    (
        "Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.",
        "已达用量限额。你的用量限额已用尽。要提高限额才能继续使用codex。",
    ),
    ("Usage: /ide [on|off|status]", "用法：/ide [on|off|status]"),
    ("Use ", "用 "),
    (
        "Your existing setup will not be changed.",
        "你现有的配置不会被修改。",
    ),
    (
        "Use Codex with non-admin sandbox",
        "在非管理员沙箱中使用Codex",
    ),
    (
        "Use /compact when the conversation gets long to summarize history and free up context.",
        "对话变长时用/compact汇总历史并释放上下文。",
    ),
    (
        "Use /copy or press Ctrl+O to copy the latest agent response as Markdown.",
        "用/copy或按Ctrl+O把最新回复复制为Markdown。",
    ),
    (
        "Use /feedback to send logs to the maintainers when something looks off.",
        "发现异常时用/feedback把日志发给维护者。",
    ),
    (
        "Use /fork to branch the current chat into a new thread.",
        "用/fork把当前聊天派生成新线程。",
    ),
    (
        "Use /init to create an AGENTS.md with project-specific guidance.",
        "用/init创建带项目专属指引的AGENTS.md。",
    ),
    (
        "Use /mcp to list configured MCP tools.",
        "用/mcp列出已配置的MCP工具。",
    ),
    (
        "Use /permissions to control when Codex asks for confirmation.",
        "用/permissions控制Codex何时请求确认。",
    ),
    (
        "Use /personality to customize how Codex communicates.",
        "用/personality定制Codex的沟通风格。",
    ),
    (
        "Use /rename to rename your threads for easier thread resuming.",
        "用/rename重命名线程，便于后续恢复。",
    ),
    (
        "Use /side to start a side conversation in a temporary fork without polluting the main thread.",
        "用/side在临时分叉中开启侧会话，不污染主线程。",
    ),
    (
        "Use /skills to list available skills or ask Codex to use one.",
        "用/skills列出可用技能，或让Codex使用某个技能。",
    ),
    (
        "Use /status to see the current model, approvals, and token usage.",
        "用/status查看当前模型、审批设置与Token用量。",
    ),
    (
        "Use /statusline to configure which items appear in the status line.",
        "用/statusline配置状态行显示哪些项。",
    ),
    ("Use Detailed Hint A (Recommended)", "使用详细提示A（推荐）"),
    ("Use Detailed Hint B", "使用详细提示B"),
    ("Use Detailed Hint C", "使用详细提示C"),
    ("Use current directory ({0})", "使用当前目录（{0}）"),
    ("Use existing model", "沿用现有模型"),
    ("Use memories", "使用记忆"),
    (
        "Use memories in the following threads. Applied at next thread.",
        "在以下线程中使用记忆。在下一个线程生效。",
    ),
    (
        "Use non-admin sandbox (higher risk if prompt injected)",
        "使用非管理员沙箱（提示词注入时风险更高）",
    ),
    ("Use session directory ({0})", "使用会话目录（{0}）"),
    (
        "Use the OpenAI docs MCP for API questions; enable it with `codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp`.",
        "API问题可用OpenAI文档MCP；用`codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp`启用。",
    ),
    ("Viewed {0}", "已查看{0}"),
    (
        "Viewing sub-agent — direct input is disabled",
        "正在查看子代理 — 已禁用直接输入",
    ),
    ("Vim mode disabled.", "Vim模式已关闭。"),
    ("Vim mode enabled.", "Vim模式已开启。"),
    (
        "Visit the Codex community forum: https://community.openai.com/c/codex/37",
        "访问Codex社区论坛：https://community.openai.com/c/codex/37",
    ),
    ("Waited for an agent", "等待了一个代理"),
    ("Waited for background terminal", "已等待后台终端"),
    ("Waited for background terminal: {0}", "已等待后台终端：{0}"),
    ("Waiting for", "正在等待"),
    ("Waiting for a keypress...", "等待按键…"),
    ("Waiting for agents", "正在等待代理"),
    ("Waiting for approval.", "正在等待批准。"),
    ("Waiting for your response.", "正在等待你的回复。"),
    ("Waiting for {0} agents", "正在等待{0}个代理"),
    (
        "We couldn't complete the world-writable scan, so protections cannot be verified. ",
        "无法完成全局可写扫描，因此无法验证防护。 ",
    ),
    (
        "We couldn’t confirm the agent was acting safely and following your instructions. To continue working, start or resume another chat.",
        "我们无法确认代理的行动是否安全、是否遵循你的指令。要继续工作，请新建或恢复另一个聊天。",
    ),
    (
        "We couldn’t confirm the agent was interpreting your instructions correctly. Review what we detected before deciding to continue.",
        "我们无法确认代理是否正确理解了你的指令。请先查看我们检测到的内容，再决定是否继续。",
    ),
    (
        "Warm, collaborative, and helpful.",
        "温和、乐于协作、有帮助。",
    ),
    (
        "We recommend switching from {0} to {1}.",
        "我们建议从{0}切换到{1}。",
    ),
    ("Web search: {0}", "网络搜索：{0}"),
    (
        "WebSocket: {0} events send ({1})",
        "WebSocket：发送{0}个事件（{1}）",
    ),
    ("WebSocket timing: {0}", "WebSocket计时：{0}"),
    ("Welcome to ", "欢迎使用 "),
    (
        "When the composer is empty, press Esc to step back and edit your last message; Enter confirms.",
        "输入框为空时，按Esc回退编辑上一条消息；Enter确认。",
    ),
    ("Worked for {0}", "耗时 {0}"),
    ("Working directory changed to: {0}", "工作目录已切换到：{0}"),
    ("Yes, clear context and implement", "是，清空上下文并实施"),
    ("Yes, continue", "是，继续"),
    ("Yes, enable", "是，启用"),
    ("Yes, implement this plan", "是，实施此计划"),
    (
        "You can still use Codex in a non-admin sandbox. It carries greater risk if prompt injected.",
        "你仍可在非管理员沙箱中使用Codex。提示词注入时风险更高。",
    ),
    (
        "Your organization requires the default Codex agent sandbox to continue. Set it up to protect your files and control network access.",
        "你的组织要求继续使用默认的Codex代理沙箱。请完成设置以保护你的文件并控制网络访问。",
    ),
    (
        "Your organization requires the default sandbox before Codex can continue.",
        "你的组织要求先使用默认沙箱，Codex才能继续。",
    ),
    (
        "Your message will be sent again using {0}, which may be less capable on complex tasks.",
        "你的消息将改用{0}重新发送，它在复杂任务上的能力可能较弱。",
    ),
    ("You are in ", "你当前位于 "),
    (
        "Your included usage is exhausted. Choose an option below to continue.",
        "你的包含用量已用尽。请从下方选择一个选项继续。",
    ),
    (
        "You're out of credits. Your workspace is out of credits. Add credits to continue using Codex.",
        "你的额度已用尽。你的工作区额度已用尽。请充值额度以继续使用Codex。",
    ),
    (
        "You can continue using {0} if you prefer.",
        "如果你愿意，也可以继续使用{0}。",
    ),
    (
        "You can resume a previous conversation by running `codex resume`",
        "可运行`codex resume`恢复之前的对话",
    ),
    (
        "You can run any shell command from Codex using `!` (e.g. `!ls`)",
        "在Codex里可用`!`运行任意shell命令（如`!ls`）",
    ),
    ("[ ! ] Action Required", "[ ! ] 需要操作"),
    ("[ . ] Action Required", "[ . ] 需要操作"),
    (
        "a user-configured MCP server already owns the codex_tui namespace",
        "用户配置的MCP服务器已占用codex_tui命名空间",
    ),
    (
        "animation {0} fallback {1} does not exist",
        "动画{0}的回退{1}不存在",
    ),
    (
        "animation {0} fps must be finite and between 0 and {1}, got {2}",
        "动画{0}的fps必须是有限值且介于0与{1}之间，实际为{2}",
    ),
    (
        "animation {0} references sprite index {1}, but pet has {2} frames",
        "动画{0}引用了精灵索引{1}，但宠物只有{2}帧",
    ),
    (
        "[EXPERIMENTAL] Browse tasks from Codex Cloud and apply changes locally",
        "[实验性]浏览Codex Cloud的任务并在本地应用更改。",
    ),
    (
        "[EXPERIMENTAL] Run the standalone exec-server service",
        "[实验性]运行独立的exec-server服务。",
    ),
    (
        "[experimental] Manage the app-server daemon with remote control enabled",
        "[实验性]在启用远程控制的情况下管理app-server守护进程。",
    ),
    (
        "[experimental] Run the app server or related tooling",
        "[实验性]运行app server或相关工具。",
    ),
    ("[Pasted Content {0} chars]", "[粘贴内容 {0} 字符]"),
    (
        "[tui.keymap] in ~/.codex/config.toml lets you rebind supported shortcuts.",
        "~/.codex/config.toml中的[tui.keymap]可重新绑定受支持的快捷键。",
    ),
    ("[{0}] {1} · modified", "[{0}] {1} · 已修改"),
    ("[{0}] {1} · new", "[{0}] {1} · 新增"),
    (
        "[… {0} lines] ctrl + a view all",
        "[… {0} 行] ctrl + a查看全部",
    ),
    ("\n\nStartup warnings:\n{0}", "\n\n启动警告：\n{0}"),
    (
        "`tui.resume_cwd = \"current\"` requires `--cd` when using a remote workspace",
        "在远程工作区中使用 `tui.resume_cwd = \"current\"` 需要 `--cd`",
    ),
    (
        "`--worktree` is only supported for local sessions",
        "`--worktree`仅支持本地会话",
    ),
    (
        "`--worktree` cannot create a checkout from an explicitly untrusted source",
        "`--worktree`无法从未经信任的来源创建检出",
    ),
    (
        "`--worktree` requires the worktrees feature; enable it with `--enable worktrees`",
        "`--worktree`需要worktrees特性；请用`--enable worktrees`启用",
    ),
    ("`open {0}` exited with {1}", "`open {0}` 以{1}退出"),
    ("a patch touching ", "涉及以下内容的补丁 "),
    ("agent tool: {0} · {1}", "代理工具：{0} · {1}"),
    ("answer: ******", "回答：******"),
    ("answer: {0}", "回答：{0}"),
    ("app server unavailable", "app-server不可用"),
    (
        "app-server session could not be restored",
        "无法恢复app-server会话",
    ),
    ("apply_patch touching {0}", "apply_patch涉及{0}"),
    ("apply_patch touching {0} files", "apply_patch涉及{0}个文件"),
    ("authentication required", "需要认证"),
    (
        "base64 payload is not valid UTF-8",
        "base64载荷不是有效的UTF-8",
    ),
    ("auto-resolves in {0}", "{0}后自动处理"),
    ("back", "返回"),
    ("call", "次调用"),
    ("calls", "次调用"),
    ("cancelled", "已取消"),
    (
        "cannot queue through an embedded app server while a local app-server daemon is running; remove configuration overrides or use --remote",
        "本地app-server守护进程运行时，无法通过嵌入式应用服务器排队；请移除配置覆盖，或使用--remote",
    ),
    ("canonicalize {0}", "规范化{0}"),
    ("chat session", "聊天会话"),
    (
        "clipboard image paste is unsupported on Android",
        "Android上不支持从剪贴板粘贴图片",
    ),
    ("codex could access {0}", "Codex可能会访问{0}"),
    (
        "codex could call MCP tool {0}.{1}",
        "Codex可能会调用MCP工具{0}.{1}",
    ),
    ("codex could request permissions", "Codex可能会请求权限"),
    ("codex could {0}", "Codex可能会{0}"),
    ("codex to access {0}", "Codex将要访问{0}"),
    (
        "codex to call MCP tool {0}.{1}",
        "Codex将要调用MCP工具{0}.{1}",
    ),
    ("codex to request permissions", "Codex将要请求权限"),
    ("codex to {0}", "Codex将要{0}"),
    ("connected", "已连接"),
    ("context compacted", "上下文已压缩"),
    // `exec` 阶段 5：非交互输出的用量行（`event_processor_with_human_output.rs:397`）。
    // 键里没有内部空格，避免触发 [spacing] 的 CJK↔拉丁规则；命令状态四态（` succeeded` 等）
    // **不进字典**：它们带前导空格、且被 `format!` 拼在 `status` 后缀里，形状不适合作键，
    // 单列在 docs/plan/i18n-verification.md §12.18。
    ("tokens used", "已用token"),
    // exec 命令状态四态：键**含前导空格**（排版：状态跟在命令名之后），译文保留该空格。
    // 拼接处是 `format!("{}{duration_suffix}:", tr(current(), " succeeded"))` —— 英文态
    // 与原先的 `format!(" succeeded{suffix}:")` 逐字节相同（exec-test 78 passed 守住）。
    (" succeeded", " 执行成功"),
    (" exited ", " 退出码 "),
    (" declined", " 已拒绝"),
    (" in progress", " 进行中"),
    // exec 阶段 5：状态/标签行（`event_processor_with_human_output.rs`）。
    // `started`/`completed`/`declined`/`in_progress` 里只有进 eprintln! 的才进字典；
    // `EXEC_STATUS_*` 之类**匹配用**字符串（:167-190 的 match 手臂）保持英文。
    ("started", "已开始"),
    ("web search:", "网页搜索："),
    ("apply patch", "应用补丁"),
    ("warning:", "警告："),
    ("ERROR:", "错误："),
    ("model rerouted:", "模型已改道："),
    ("turn interrupted", "回合已中断"),
    // core 阶段 6 批 1：流向 UI 的 `WarningEvent` / `ErrorEvent` 文案
    // （`compact.rs`、`session/turn.rs`）。判据：`EventMsg::Warning/Error` 经事件流到
    // `add_warning_message`/`add_error_message` 渲染；`tracing::*`、`wrap_err`、
    // 喂模型的文本与内部 `Err(String)` **不在此列**。
    (
        "Heads up: Long threads and multiple compactions can cause the model to be less accurate. Start a new thread when possible to keep threads small and targeted.",
        "注意：长会话与多次压缩会让模型准确性下降。条件允许时请新建会话，让会话保持小而聚焦。",
    ),
    (
        "Stop hook requested continuation without a prompt; ignoring the block.",
        "Stop钩子请求继续但未提供提示词；已忽略该阻断。",
    ),
    (
        "Invalid image in your last message. Please remove it and try again.",
        "上一条消息中的图片无效。请移除后重试。",
    ),
    // core 阶段 6 批 2：插值形态（`tr_with`）。键是**位置占位** `{0}`，不是 Rust 的命名捕获——
    // 下一批改这些串时，调用点的字面量必须与这里的键逐字一致，否则 i18n-check 报 missing。
    (
        "Falling back from WebSockets to HTTPS transport. {0}",
        "正在从WebSockets回退到HTTPS传输。{0}",
    ),
    (
        "Failed to save the conversation transcript; Codex will continue retrying. Error: {0}",
        "保存会话转录失败；Codex将继续重试。错误：{0}",
    ),
    (
        "Model metadata for `{0}` not found. Defaulting to fallback metadata; this can degrade performance and cause issues.",
        "未找到模型 `{0}` 的元数据。将使用回退元数据；这可能降低性能并引发问题。",
    ),
    (
        "Automatic approval review rejected too many approval requests for this turn ({0} consecutive, {1} in the last {2} reviews); interrupting the turn.",
        "本轮自动审批审查拒绝了过多审批请求（连续 {0} 次，最近 {2} 次审查中有 {1} 次）；正在中断该回合。",
    ),
    // codex-mcp 启动失败文案（`connection_manager/startup.rs`）：经
    // `McpStartupStatus::Failed.error` → `EventMsg::McpStartupUpdate` →
    // tui `chatwidget/mcp_startup.rs:122` → `on_warning` → `new_warning_event` 渲染给用户。
    // ⚠ `\n[mcp_servers.…]` 是**可粘贴的配置样例**，译文里必须逐字节保留（含 `XX` 占位）。
    (
        "GitHub MCP does not support OAuth. Log in by adding a personal access token (https://github.com/settings/personal-access-tokens) to your environment and config.toml:\n[mcp_servers.{0}]\nbearer_token_env_var = CODEX_GITHUB_PERSONAL_ACCESS_TOKEN",
        "GitHub MCP不支持OAuth。请在环境变量与config.toml中加入个人访问令牌（https://github.com/settings/personal-access-tokens）后登录：\n[mcp_servers.{0}]\nbearer_token_env_var = CODEX_GITHUB_PERSONAL_ACCESS_TOKEN",
    ),
    (
        "Use your client's MCP OAuth sign-in flow.",
        "请使用你客户端的MCP OAuth登录流程。",
    ),
    ("Run `codex mcp login {0}`.", "运行 `codex mcp login {0}`。"),
    ("requires OAuth reauthentication", "需要重新进行OAuth认证"),
    ("is not logged in", "尚未登录"),
    ("The {0} MCP server {1}. {2}", "{0} MCP服务器{1}。{2}"),
    (
        "MCP client for `{0}` timed out after {1} seconds. Add or adjust `startup_timeout_sec` in your config.toml:\n[mcp_servers.{2}]\nstartup_timeout_sec = XX",
        "`{0}` 的MCP客户端在 {1} 秒后超时。请在config.toml中新增或调整 `startup_timeout_sec`：\n[mcp_servers.{2}]\nstartup_timeout_sec = XX",
    ),
    (
        "MCP client for `{0}` failed to start: {1}",
        "`{0}` 的MCP客户端启动失败：{1}",
    ),
    // core `RequestUserInput` 的审批提问（`mcp_tool_call.rs` / `mcp_skill_dependencies.rs`）：
    // header / question / 选项 description 经 app-server 转 `ToolRequestUserInputQuestion`
    // 后在 tui `bottom_pane/async_questions/` 直接渲染。
    // 注：`Run the tool and continue.` / `…for this session.` / `…for future tool calls.`
    // 三条**字典里早已存在**（本轮只接调用点，不重复加条目——第 87 轮加的 [duplicate] 列
    // 当场抓到了我重复添加，这是它第一次派上用场）。
    ("Approve app tool call?", "批准应用工具调用？"),
    ("Cancel this tool call.", "取消该工具调用。"),
    ("this app", "此应用"),
    ("the {0} MCP server", "{0} MCP服务器"),
    ("Allow {0} to run tool \"{1}\"?", "允许{0}运行工具“{1}”吗？"),
    ("Install MCP servers?", "安装MCP服务器？"),
    (
        "The following MCP servers are required by the selected skills but are not installed yet: {0}. Install them now?",
        "所选技能需要以下MCP服务器，但它们尚未安装：{0}。现在安装吗？",
    ),
    (
        "Install and enable the missing MCP servers in your global config.",
        "在全局配置中安装并启用缺失的MCP服务器。",
    ),
    (
        "Skip installation for now and do not show again for these MCP servers in this session.",
        "暂时跳过安装，并在本次会话中不再为这些MCP服务器提示。",
    ),
    // session 初始化失败家族（§12.23）：外层 `ERROR: {0}` 早已接入（cli/src/main.rs:922），
    // 这里补内层的两条；`{0}` 是 anyhow 链展开后的文本（保留原样，属底层错误信息）。
    ("Failed to initialize session: {0}", "会话初始化失败：{0}"),
    (
        "required MCP server `{0}` was not initialized",
        "必需的MCP服务器 `{0}` 未能初始化",
    ),
    // 只译**纯显示**的那些；同名的比对值（`label` 被 TUI 原样提交、
    // 再与英文 const 精确比对）**不译** —— 译了会静默破坏匹配。
    // 见 `mcp_tool_call.rs` 中 `MCP_TOOL_APPROVAL_ACCEPT*` 的注释与 §3.6。
    (
        "Optionally, add details in notes (tab).",
        "可在备注中补充细节（tab）。",
    ),
    ("could not encode image: {0}", "无法编码图像：{0}"),
    ("could not write {0}: {1}", "无法写入{0}：{1}"),
    ("create {0}", "创建{0}"),
    (
        "ctrl+c clear input, then quit · actions paused until the list is refreshed",
        "ctrl+c清空输入后再退出 · 列表刷新前操作暂停",
    ),
    (
        "ctrl + p / ctrl + n change question",
        "ctrl+p/ctrl+n切换问题",
    ),
    ("creator account {0}", "创建者账号 {0}"),
    ("creator {0}", "创建者 {0}"),
    ("creator {0} ({1})", "创建者 {0}（{1}）"),
    ("customize shortcuts with ", "自定义快捷键："),
    ("IDE context", "IDE上下文"),
    // Shortcut descriptors. These are the labels the `_` arm of the matcher in
    // `ShortcutDescriptor::line` renders through `tr`.
    (" for commands", " 查看命令"),
    (" for shell commands", " 查看Shell命令"),
    (" for newline", " 换行"),
    (" for file paths", " 文件路径"),
    (" to paste images", " 粘贴图片"),
    (" to edit in external editor", " 用外部编辑器编辑"),
    (" search history", " 搜索历史"),
    (" to view transcript", " 查看记录"),
    (" to change mode", " 切换模式"),
    (" reasoning down", " 降低推理强度"),
    (" reasoning up", " 提高推理强度"),
    // Context-window line. Templates: the placeholders are substituted after
    // the lookup, so the translation may move them but not drop them.
    ("day", "天"),
    ("deny read {0}", "禁止读取 {0}"),
    ("directory:", "目录："),
    ("directory: {0}", "目录：{0}"),
    ("disabled", "已禁用"),
    ("download pet asset from {0}", "从{0}下载宠物资源"),
    ("editor command is empty", "编辑器命令为空"),
    ("editor directory has no parent", "编辑器目录没有父目录"),
    (
        "editor directory must not be writable",
        "编辑器目录必须不可写",
    ),
    (
        "editor directory must not contain symbolic links",
        "编辑器目录不能包含符号链接",
    ),
    ("editor exited with status {0}", "编辑器以{0}退出"),
    ("embedded resource: {0}", "嵌入式资源：{0}"),
    ("enabled", "已启用"),
    ("event", "事件"),
    ("events", "事件"),
    ("failed", "失败"),
    ("failed to archive session", "无法归档会话"),
    (
        "failed to connect to remote app server",
        "无法连接到远程app-server",
    ),
    (
        "failed to bind managed worktree thread",
        "无法绑定托管工作树线程",
    ),
    ("failed to delete session", "无法删除会话"),
    (
        "failed to encode thread/inject_items payload",
        "无法编码thread/inject_items载荷",
    ),
    (
        "failed to list loaded threads from app server",
        "无法从应用服务器列出已加载的线程",
    ),
    (
        "failed to serialize Auto Review denial event",
        "无法序列化Auto Review拒绝事件",
    ),
    ("failed to invoke `open`: {0}", "调用 `open` 失败：{0}"),
    (
        "failed to launch the Desktop app through PowerShell with {0}",
        "通过PowerShell启动桌面应用失败，返回{0}",
    ),
    (
        "failed to launch the Desktop app through PowerShell: {0}",
        "通过PowerShell启动桌面应用失败：{0}",
    ),
    ("failed to load skills on startup", "启动时加载技能失败"),
    (
        "failed to localize requested filesystem paths: {0}",
        "本地化所请求的文件系统路径失败：{0}",
    ),
    (
        "failed to reject app-server request: {0}",
        "拒绝应用服务器请求失败：{0}",
    ),
    (
        "failed to serialize MCP elicitation response: {0}",
        "序列化MCP elicitation响应失败：{0}",
    ),
    (
        "failed to serialize command execution approval response: {0}",
        "序列化命令执行审批响应失败：{0}",
    ),
    (
        "failed to serialize file change approval response: {0}",
        "序列化文件变更审批响应失败：{0}",
    ),
    (
        "failed to serialize permissions approval response: {0}",
        "序列化权限审批响应失败：{0}",
    ),
    (
        "failed to serialize request_user_input response: {0}",
        "序列化request_user_input响应失败：{0}",
    ),
    ("failed to queue session message", "无法排队会话消息"),
    ("failed to resolve CODEX_HOME", "无法解析CODEX_HOME"),
    (
        "failed to start embedded app server",
        "无法启动嵌入式app-server",
    ),
    (
        "failed to restore terminal. Run `reset` or restart your terminal to recover: {0}",
        "恢复终端失败。请运行 `reset` 或重启终端以恢复：{0}",
    ),
    ("failed to unarchive session", "无法取消归档会话"),
    (
        "file changes: {0} · {1} changes",
        "文件变更：{0} · {1} 处变更",
    ),
    (
        "failed to write config.toml: {0}",
        "写入config.toml失败：{0}",
    ),
    ("forked_from_id is invalid: {0}", "forked_from_id无效：{0}"),
    (
        "frame path has no valid file stem",
        "帧路径没有有效的文件名主干",
    ),
    ("glob `{0}`", "通配 `{0}`"),
    ("group", "分组"),
    ("hook prompt: ", "钩子提示："),
    ("hour", "小时"),
    ("image: {0}", "图像：{0}"),
    (
        "in-process sessions have no connection to restore",
        "进程内会话没有可恢复的连接",
    ),
    ("install {0}", "安装{0}"),
    (
        "interrupted turn {0} is missing from the source thread",
        "被中断的轮次{0}不在源线程中",
    ),
    (
        "interrupted turn {0} is no longer the latest turn",
        "被中断的轮次{0}已不是最新轮次",
    ),
    (
        "interrupted turn {0} is still in progress",
        "被中断的轮次{0}仍在进行中",
    ),
    ("interrupted with {0} unanswered", "已中断，还有{0}个未回答"),
    (
        "invalid remote address `{0}`; expected `ws://host:port`, `wss://host:port`, `unix://`, or `unix://PATH`",
        "远程地址 `{0}` 无效；应为 `ws://host:port`、`wss://host:port`、`unix://` 或 `unix://PATH`",
    ),
    ("invalid keymap configuration: {0}", "快捷键配置无效：{0}"),
    ("invalid visualization fragment", "可视化片段无效"),
    ("io error: {0}", "IO错误：{0}"),
    ("item", "条目"),
    ("items", "条目"),
    (
        "managed MCP requirements do not permit the TUI task-tools server",
        "托管MCP要求不允许TUI task-tools服务器",
    ),
    ("join pet load task", "等待宠物加载任务"),
    (
        "join pet spritesheet cache validation task",
        "等待宠物精灵表缓存校验任务",
    ),
    (
        "join pet spritesheet install task",
        "等待宠物精灵表安装任务",
    ),
    ("link: {0}", "链接：{0}"),
    ("local app-server daemon", "本地app-server守护进程"),
    ("local {0}", "本地 {0}"),
    ("main prompt", "主提示"),
    ("mcp tool: {0}/{1} · {2}", "MCP工具：{0}/{1} · {2}"),
    ("minute", "分"),
    ("model: {0}{1}", "模型：{0}{1}"),
    ("network access to {0}", "对{0}的网络访问"),
    ("new task", "新建任务"),
    ("next question", "下一题"),
    ("no image on clipboard: {0}", "剪贴板中没有图片：{0}"),
    ("no matches", "无匹配项"),
    ("not started", "未启动"),
    ("note: {0}", "备注：{0}"),
    ("now", "刚刚"),
    ("open", "打开"),
    ("option {0}/{1}", "选项 {0}/{1}"),
    ("parse pet asset download URL {0}", "解析宠物资源下载URL{0}"),
    (
        "pet asset download from {0} exceeded {1} bytes",
        "从{0}下载的宠物资源超过{1}字节",
    ),
    (
        "pet frame index does not fit usize",
        "宠物帧索引无法放入usize",
    ),
    (
        "pet frame index exceeds expected frame count",
        "宠物帧索引超出预期的帧数",
    ),
    ("permission request", "权限请求"),
    ("permission request: {0}", "权限请求：{0}"),
    ("permissions:", "权限："),
    ("pet frame index overflow", "宠物帧索引溢出"),
    ("pet frame x offset overflow", "宠物帧x偏移溢出"),
    ("pet frame y offset overflow", "宠物帧y偏移溢出"),
    (
        "pet image asset unavailable: {0}",
        "宠物图像资源不可用：{0}",
    ),
    (
        "pet spritesheet path should include an assets directory",
        "宠物精灵表路径应包含assets目录",
    ),
    (
        "plugin list cwd must be absolute",
        "插件列表的cwd必须是绝对路径",
    ),
    ("press {0}", "按{0}"),
    ("prev question", "上一题"),
    ("question", "问题"),
    ("questions", "问题"),
    ("queued messages", "排队消息"),
    ("raw output", "原始输出"),
    (
        "read pet asset download from {0}",
        "读取从{0}下载的宠物资源",
    ),
    ("read {0}", "读取{0}"),
    ("reasoning {0}", "推理 {0}"),
    ("remote app server", "远端应用服务器"),
    ("remote {0}", "远端 {0}"),
    ("remove {0}", "删除{0}"),
    (
        "sixel RGBA buffer length overflow",
        "sixel RGBA缓冲区长度溢出",
    ),
    (
        "sixel byte index does not fit usize",
        "sixel字节索引无法放入usize",
    ),
    ("rename", "重命名"),
    ("resume", "恢复"),
    ("review finished: ", "审查已完成："),
    ("review started: ", "审查已开始："),
    ("search", "搜索"),
    ("second", "秒"),
    ("send input to terminal {0}: {1}", "向终端{0}发送输入：{1}"),
    ("sixel byte index overflow", "sixel字节索引溢出"),
    (
        "sixel image dimensions must be non-zero",
        "sixel图像尺寸必须非零",
    ),
    (
        "sixel pixel count does not fit usize",
        "sixel像素计数无法放入usize",
    ),
    ("sixel pixel count overflow", "sixel像素计数溢出"),
    ("sixel pixel index overflow", "sixel像素索引溢出"),
    ("slash command", "斜杠命令"),
    ("starting", "正在启动"),
    ("status: {0}{1}", "状态：{0}{1}"),
    ("stop", "停止"),
    ("summaries off", "摘要关闭"),
    ("summaries {0}", "摘要 {0}"),
    ("tab or esc to clear notes", "tab或esc清除备注"),
    ("tab to add notes", "tab添加备注"),
    (
        "temporary structured response exceeds {0} bytes",
        "临时结构化响应超过{0}字节",
    ),
    (
        "temporary structured thread did not preserve permission profile {0}",
        "临时结构化线程未保留权限配置文件{0}",
    ),
    (
        "temporary structured thread did not start with read-only permissions",
        "临时结构化线程未以只读权限启动",
    ),
    (
        "temporary structured thread start timed out",
        "临时结构化线程启动超时",
    ),
    (
        "temporary structured turn completed without a response",
        "临时结构化轮次未返回响应即完成",
    ),
    (
        "temporary structured turn ended with status {0}",
        "临时结构化轮次以状态{0}结束",
    ),
    (
        "temporary structured turn notification channel closed",
        "临时结构化轮次的通知通道已关闭",
    ),
    ("terminal image write failed: {0}", "终端图像写入失败：{0}"),
    (
        "the selected thread is no longer available for prompt editing",
        "所选线程已不可用于编辑提示词",
    ),
    (
        "the {0} does not support thread/queue/add; update or restart the {0}",
        "{0}不支持thread/queue/add；请更新或重启{0}",
    ),
    ("thread id `{0}` is invalid: {1}", "线程ID`{0}`无效：{1}"),
    (
        "thread usage request timed out in TUI",
        "线程用量请求在TUI中超时",
    ),
    ("to continue working", "继续工作"),
    ("to edit message", "编辑消息"),
    ("to edit next", "编辑下一条"),
    ("to edit prev", "编辑上一条"),
    ("to go back", "返回"),
    ("to jump", "跳转"),
    ("to page", "翻页"),
    ("to scroll", "滚动"),
    ("tool", "工具"),
    ("tool: {0} · {1}", "工具：{0} · {1}"),
    ("tools", "工具"),
    (
        "tui.keymap.agents.{0}: ctrl-z is reserved for suspend",
        "tui.keymap.agents.{0}：ctrl-z保留用于挂起",
    ),
    (
        "tui.keymap.agents.{0}: printable keys and backspace are reserved for task input",
        "tui.keymap.agents.{0}：可打印字符键与backspace保留用于任务输入",
    ),
    (
        "tui.keymap.chat.{0}: printable keys are reserved for text input",
        "tui.keymap.chat.{0}：可打印字符键保留用于文本输入",
    ),
    (
        "tui.keymap.global.open_agents: AltGr characters are reserved for text input",
        "tui.keymap.global.open_agents：AltGr字符保留用于文本输入",
    ),
    (
        "tui.keymap.global.open_agents: ctrl-z is reserved for suspend",
        "tui.keymap.global.open_agents：ctrl-z保留用于挂起",
    ),
    ("unknown", "未知"),
    (
        "unknown protocol {0}; expected auto, kitty, or sixel",
        "未知协议{0}；应为auto、kitty或sixel",
    ),
    (
        "unsupported pet asset download URL scheme {0}",
        "不支持的宠物资源下载URL协议{0}",
    ),
    ("unsupported - {0}", "不支持 - {0}"),
    ("use the configured stop shortcut", "使用已配置的停止快捷键"),
    (
        "visualization fragment has no file name",
        "可视化片段没有文件名",
    ),
    (
        "visualization viewer cache is unavailable",
        "可视化查看器缓存不可用",
    ),
    (
        "visualization viewer directory must not contain symbolic links",
        "可视化查看器目录不能包含符号链接",
    ),
    ("waiting for next key", "等待下一个按键"),
    ("web search: ", "网络搜索："),
    (
        "workload identity must be configured on the remote app-server host",
        "必须在远程app-server主机上配置工作负载身份",
    ),
    ("write {0}", "写入{0}"),
    (
        "{0} A checkout was retained at {1}; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.",
        "{0}已在{1}保留一份检出；如不再需要，请在源仓库中运行 `git worktree remove <checkout-path>` 将其删除。",
    ),
    (
        "{0}   {1} working   {2} ready",
        "{0}   {1} 工作中   {2} 就绪",
    ),
    ("{0} (custom)", "{0}（自定义）"),
    ("{0} (iapi)", "{0}（iapi）"),
    ("{0} (service)", "{0}（service）"),
    ("{0} ({1} skill)", "{0}（{1}个技能）"),
    ("{0} ({1} unanswered)", "{0}（{1}个未回答）"),
    (
        "{0} Learn more about {1} at ",
        "{0} 了解{1}的更多信息，请访问 ",
    ),
    ("{0} MCP servers", "{0}个MCP服务器"),
    ("{0} actions.", "{0}个操作。"),
    ("{0} apps", "{0}个应用"),
    (
        "{0} are disabled in this TUI session.",
        "本次TUI会话中{0}已禁用。",
    ),
    ("{0} credits", "{0} 额度"),
    ("{0} events received ({1})", "接收{0}个事件（{1}）"),
    ("{0} hooks", "{0}钩子"),
    (
        "{0} hooks need review before they can run.",
        "有{0}个钩子在运行前需要审查。",
    ),
    (
        "{0} hooks are new or changed.",
        "有{0}个钩子是新增或已变更。",
    ),
    ("{0} input", "输入 {0}"),
    ("{0} limit", "{0}限额"),
    ("{0} need input", "{0} 需要输入"),
    ("{0} of {1}", "{0}/{1}"),
    ("{0} of {1} credits used", "已用{0}/{1}额度"),
    ("{0} on {1}", "{0}，于 {1}"),
    ("{0} output", "输出 {0}"),
    ("{0} questions requested", "请求了{0}个问题"),
    (
        "{0} setting saved on the server for new threads. This thread is unchanged. Project or task settings may override it.",
        "{0}设置已保存到服务器，用于新线程。当前线程不受影响。项目或任务设置可能覆盖它。",
    ),
    (
        "{0} setting was saved but is overridden: {1}",
        "{0}设置已保存但被覆盖：{1}",
    ),
    (
        "{0} skills enabled, {1} skills disabled",
        "已启用{0}个技能，已禁用{1}个技能",
    ),
    ("{0} submit", "{0} 提交"),
    ("{0} to interrupt", "{0} 中断"),
    ("{0} unavailable", "{0}不可用"),
    ("{0} window", "{0} 窗口"),
    ("{0} {1} limit", "{0} {1}限额"),
    ("{0} {1}s ago", "{0}{1}前"),
    ("{0}% context left", "剩余上下文 {0}%"),
    ("{0} used", "已用 {0}"),
    ("100% context left", "剩余上下文100%"),
    // Terminal-title picker (`tui/src/bottom_pane/title_setup.rs`). The item
    // *names* come from `strum` and the filter strings are internal codes, so
    // only the descriptions and the two headings are translated. Product names
    // (Codex, Fast) and tool names (update_plan) stay as they are.
    ("Configure Terminal Title", "配置终端标题"),
    (
        "Select which items to display in the terminal title.",
        "选择在终端标题中显示哪些项目。",
    ),
    ("Codex app name", "Codex应用名"),
    (
        "Project name (falls back to current directory name)",
        "项目名（回退为当前目录名）",
    ),
    ("Current working directory", "当前工作目录"),
    (
        "Spinner while working, action-required message while blocked.",
        "工作时显示转圈，被阻塞时显示需要操作的消息。",
    ),
    (
        "Compact session run-state text (Ready, Working, Thinking)",
        "简洁的会话运行状态文本（就绪、工作中、思考中）",
    ),
    (
        "Current thread name (omitted when unnamed)",
        "当前线程名（未命名时省略）",
    ),
    (
        "Current thread title, or thread identifier when unnamed",
        "当前线程标题，未命名时用线程标识",
    ),
    (
        "Current Git branch (omitted when unavailable)",
        "当前Git分支（不可用时省略）",
    ),
    (
        "Percentage of context window remaining (omitted when unknown)",
        "剩余上下文窗口百分比（未知时省略）",
    ),
    (
        "Percentage of context window used (omitted when unknown)",
        "已用上下文窗口百分比（未知时省略）",
    ),
    (
        "Remaining usage on the primary usage limit (omitted when unavailable)",
        "主用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the secondary usage limit (omitted when unavailable)",
        "次用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the 5-hour usage limit (omitted when unavailable)",
        "5小时用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the annual usage limit (omitted when unavailable)",
        "每年用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the daily usage limit (omitted when unavailable)",
        "每日用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the monthly usage limit (omitted when unavailable)",
        "每月用量限额的剩余额度（不可用时省略）",
    ),
    (
        "Remaining usage on the weekly usage limit (omitted when unavailable)",
        "每周用量限额的剩余额度（不可用时省略）",
    ),
    ("Codex application version", "Codex应用版本"),
    (
        "Total tokens used in session (omitted when zero)",
        "本次会话已用token总数（为0时省略）",
    ),
    (
        "Total input tokens used in session",
        "本次会话输入token总数",
    ),
    (
        "Total output tokens used in session",
        "本次会话输出token总数",
    ),
    (
        "Estimated current-thread credits (Enterprise workspaces only; omitted when unavailable)",
        "当前线程的预估额度消耗（仅企业工作区；不可用时省略）",
    ),
    (
        "Estimated current-thread cost (Enterprise workspaces only; omitted when unavailable)",
        "当前线程的预估费用（仅企业工作区；不可用时省略）",
    ),
    (
        "Current thread identifier (omitted until thread starts)",
        "当前线程标识（线程开始前省略）",
    ),
    (
        "Whether Fast mode is currently active",
        "Fast模式当前是否启用",
    ),
    ("Current model name", "当前模型名"),
    (
        "Current model name with reasoning level",
        "当前模型名及推理强度",
    ),
    ("Current reasoning level", "当前推理强度"),
    (
        "Latest task progress from update_plan (omitted until available)",
        "update_plan的最新任务进度（尚不可用时省略）",
    ),
    // Status-line picker (`tui/src/bottom_pane/status_line_setup.rs`). Most of
    // its descriptions are the same English sentences as the terminal-title
    // picker's, and those are *not* repeated here: with the English original as
    // the key, one sentence is one entry shared by every screen that shows it.
    // Only the sentences unique to this picker appear below.
    (
        "Project name (omitted when unavailable)",
        "项目名（不可用时省略）",
    ),
    (
        "Current machine hostname (omitted when unavailable)",
        "当前机器主机名（不可用时省略）",
    ),
    (
        "Open pull request number for the current branch (omitted when unavailable)",
        "当前分支的待处理PR编号（不可用时省略）",
    ),
    (
        "Committed branch changes against the default branch (omitted when unavailable)",
        "相对默认分支的已提交改动（不可用时省略）",
    ),
    (
        "Active permission profile or sandbox mode",
        "当前权限配置或沙箱模式",
    ),
    ("Active command approval mode", "当前命令审批模式"),
    (
        "Total context window size in tokens (omitted when unknown)",
        "上下文窗口总token数（未知时省略）",
    ),
    (
        "Estimated current-thread cost in USD (Enterprise workspaces only; omitted when unavailable)",
        "当前线程的预估费用（美元，仅企业工作区；不可用时省略）",
    ),
    (
        "Whether raw scrollback mode is active",
        "原始回滚模式是否启用",
    ),
    (
        "Workspace notification headline (Enterprise workspaces only; omitted when unavailable)",
        "工作区通知标题（仅企业工作区；不可用时省略）",
    ),
    ("Use theme colors", "使用主题颜色"),
    (
        "Apply colors from the active /theme",
        "应用当前 /theme的颜色",
    ),
    ("Configure Status Line", "配置状态栏"),
    (
        "Select which items to display in the status line.",
        "选择在状态栏中显示哪些项目。",
    ),
    // Approval overlay (`tui/src/bottom_pane/approval_overlay.rs`). These are the
    // questions the user is actually asked before a command runs, so they are
    // the highest-visibility strings in the crate after the composer itself.
    // The two templates keep their `{0}`, which is substituted after the lookup.
    (
        "Would you like to run the following command?",
        "要运行下面的命令吗？",
    ),
    (
        "Would you like to send input to the existing terminal?",
        "要把输入发送到已有终端吗？",
    ),
    (
        "Would you like to send input to terminal {0}?",
        "要把输入发送到终端 {0} 吗？",
    ),
    (
        "Do you want to approve network access to \"{0}\"?",
        "要批准访问网络「{0}」吗？",
    ),
    (
        "Would you like to grant these permissions?",
        "要授予这些权限吗？",
    ),
    (
        "Would you like to make the following edits?",
        "要进行下面的编辑吗？",
    ),
    ("{0} needs your approval.", "{0} 需要你的批准。"),
    (
        "You did not grant additional permissions",
        "你未授予额外权限",
    ),
    (
        "You granted additional permissions with strict auto review",
        "你以严格自动审查方式授予了额外权限",
    ),
    (
        "You granted additional permissions for this session",
        "你在本次会话中授予了额外权限",
    ),
    ("You granted additional permissions", "你授予了额外权限"),
    // Approval option buttons. These reach `tr` through a `label` field rather
    // than as a literal at the call site (`tr(current(), self.label)`), which is
    // why `codex-i18n-check` reports them under "bound keys".
    ("Yes, proceed", "是，继续"),
    (
        "Yes, and don't ask again for these files",
        "是，且这些文件不再询问",
    ),
    (
        "Yes, grant these permissions for this turn",
        "是，本轮授予这些权限",
    ),
    (
        "Yes, grant these permissions for this session",
        "是，本次会话授予这些权限",
    ),
    (
        "Yes, grant for this turn with strict auto review",
        "是，本轮以严格自动审查方式授予",
    ),
    ("Yes, provide the requested info", "是，提供所请求的信息"),
    ("No, continue without running it", "不，不运行就继续"),
    ("No, continue without permissions", "不，不授予权限继续"),
    ("No, but continue without it", "不，但不使用它继续"),
    (
        "No, and tell Codex what to do differently",
        "不，并告诉Codex该怎么做",
    ),
    ("Cancel this request", "取消本次请求"),
    // The rest of the approval overlay: its footer hints, the field labels of
    // the request header, and the options that depend on which decision the
    // agent offered. `to confirm` / `to cancel` follow a key binding rather than
    // standing alone, so the translation is the bare verb; the header labels
    // keep their colon (the key has a trailing space, which Chinese drops).
    ("to confirm", "确认"),
    ("to cancel", "取消"),
    (" to open thread", " 打开线程"),
    ("Thread: ", "线程："),
    ("Environment: ", "环境："),
    ("Reason: ", "原因："),
    ("Permission rule: ", "权限规则："),
    ("Input: ", "输入："),
    ("Server: ", "服务器："),
    ("Yes, just this once", "是，仅此一次"),
    (
        "Yes, and don't ask again for commands that start with `{0}`",
        "是，且不再询问以 `{0}` 开头的命令",
    ),
    (
        "Yes, and allow this host for this conversation",
        "是，本次对话允许该主机",
    ),
    (
        "Yes, and allow these permissions for this session",
        "是，本次会话允许这些权限",
    ),
    (
        "Yes, and don't ask again for this command in this session",
        "是，本次会话不再询问该命令",
    ),
    (
        "Yes, and allow this host in the future",
        "是，以后允许该主机",
    ),
    (
        "No, and block this host in the future",
        "不，以后阻止该主机",
    ),
    // Usage menu and rate-limit-reset flow
    // (`tui/src/chatwidget/usage.rs`), the first slice of the chatwidget step.
    // Two templates: the first counts resets, the second is the header of the
    // picker. Both keep their placeholders.
    ("Usage", "用量"),
    (
        "View account usage or redeem an earned reset.",
        "查看账户用量，或兑换已获得的重置。",
    ),
    ("Show usage", "查看用量"),
    (
        "View recent account token usage.",
        "查看最近的账户token用量。",
    ),
    ("Redeem usage limit reset", "兑换用量限额重置"),
    ("Check reset availability.", "检查重置可用性。"),
    (
        "No usage limit resets available.",
        "没有可用的用量限额重置。",
    ),
    ("You have {0} {1} available.", "你有 {0} 个{1}可用。"),
    ("{0} {1} available.", "{0} 个{1}可用。"),
    ("Usage limit resets", "用量限额重置"),
    ("Checking your available resets...", "正在检查可用的重置…"),
    ("Loading...", "加载中…"),
    (
        "You don't have any usage limit resets available.",
        "你没有可用的用量限额重置。",
    ),
    (
        "Couldn't load usage limit resets. Please try again.",
        "无法加载用量限额重置，请重试。",
    ),
    ("Cancel", "取消"),
    ("Use this reset?", "使用这次重置吗？"),
    ("Yes, use reset", "是，使用重置"),
    ("No, go back", "不，返回"),
    ("Choose a different reset.", "选择另一个重置。"),
    // Plugins (`tui/src/chatwidget/plugins.rs`), the second slice of the
    // chatwidget step. Two templates: the install confirmation and the count of
    // apps that still need authentication.
    ("Plugins are disabled.", "插件功能已禁用。"),
    (
        "Enable the plugins feature to use /plugins.",
        "启用插件功能后才能使用 /plugins。",
    ),
    ("Add marketplace", "添加市场源"),
    (
        "owner/repo, git URL, or local marketplace path",
        "owner/repo、Git URL或本地市场源路径",
    ),
    (
        "Examples: owner/repo, git URL, ./marketplace",
        "示例：owner/repo、Git URL、./marketplace",
    ),
    ("Installed {0} plugin.", "已安装插件 {0}。"),
    (
        "No additional app authentication is required.",
        "无需额外的应用认证。",
    ),
    (
        "{0} app(s) still need authentication: {1}",
        "仍有 {0} 个应用需要认证：{1}",
    ),
    // Slash-command dispatch (`tui/src/chatwidget/slash_dispatch.rs`): the
    // disabled-feature notices and the archive/delete session confirmations.
    // The `const` usage strings at the top of that file are wrapped where they
    // are used, because a `const` cannot call `tr`; their text stays the key.
    ("Collaboration modes are disabled.", "协作模式已禁用。"),
    (
        "Enable collaboration modes to use /plan.",
        "启用协作模式后才能使用 /plan。",
    ),
    ("Plan mode unavailable right now.", "计划模式当前不可用。"),
    ("Archive this session?", "归档本次会话？"),
    ("No, don't archive", "不，不归档"),
    ("Return to the current session", "返回当前会话"),
    ("Yes, archive and exit", "是，归档并退出"),
    ("Archive this session now", "立即归档本次会话"),
    ("Delete this session?", "删除本次会话？"),
    (
        "Cannot be undone. Subagent threads will also be deleted.",
        "无法撤销。子代理线程也会被删除。",
    ),
    ("No, keep this session", "不，保留本次会话"),
    ("Yes, delete and exit", "是，删除并退出"),
    (
        "Permanently delete this session now",
        "立即永久删除本次会话",
    ),
    (
        "Session is still starting; try /app again in a moment.",
        "会话仍在启动中，请稍后再试 /app。",
    ),
    // Managed worktrees (`tui/src/chatwidget/worktree_picker.rs`). "worktree"
    // is a Git term, so it is translated consistently as 工作树 and the compound
    // "managed worktree" as 受管工作树 everywhere in this file.
    (
        "Where should the new conversation run?",
        "新对话应在哪里运行？",
    ),
    (
        "Where should the forked conversation run?",
        "分叉的对话应在哪里运行？",
    ),
    ("Current checkout", "当前检出"),
    (
        "Keep using the current working directory",
        "继续使用当前工作目录",
    ),
    ("New worktree", "新建工作树"),
    ("Create an isolated managed checkout", "创建隔离的受管检出"),
    (
        "Enable worktrees in /experimental to create a worktree.",
        "在 /experimental中启用工作树后才能创建工作树。",
    ),
    (
        "Managed worktrees require a local Git repository.",
        "受管工作树需要本地Git仓库。",
    ),
    ("Worktrees", "工作树"),
    ("Continue current conversation", "继续当前对话"),
    (
        "Preserve this conversation in the new checkout",
        "在新检出中保留本次对话",
    ),
    ("Start new conversation", "开始新对话"),
    (
        "Open a fresh conversation in the new checkout",
        "在新检出中开启全新对话",
    ),
    ("Browse worktrees", "浏览工作树"),
    (
        "Resume an owner thread or copy a working directory",
        "恢复归属线程，或复制工作目录",
    ),
    ("Managed worktrees", "受管工作树"),
    ("Loading worktrees…", "正在加载工作树…"),
    (
        "Cannot list managed worktrees: {0}",
        "无法列出受管工作树：{0}",
    ),
    (
        "No worktrees in this repository's configured pool",
        "该仓库的配置池中没有工作树",
    ),
    (
        "Select a worktree to resume its owner or copy its working directory",
        "选择一个工作树以恢复其归属线程或复制其工作目录",
    ),
    ("No owner metadata", "无归属元数据"),
    ("Owner: {0}", "归属：{0}"),
    ("Worktree working directory", "工作树工作目录"),
    ("Resume owner thread", "恢复归属线程"),
    ("Copy working directory", "复制工作目录"),
    ("Path is not valid UTF-8", "路径不是有效的UTF-8"),
    ("Worktree", "工作树"),
    // Model selection and reasoning popups (`tui/src/chatwidget/model_popups.rs`).
    // Product names (OpenAI, Codex) and config keys (`config.toml`) stay as they
    // are; "Plan mode" is a mode name and is translated as 计划模式 throughout.
    // Every template keeps its `{0}`.
    (
        "Warning: OpenAI base URL is overridden to {0}. Selecting models may not be supported or work properly.",
        "警告：OpenAI base URL已被覆盖为 {0}。选择模型可能不受支持或无法正常工作。",
    ),
    (
        "Choose a specific model and reasoning level (current: {0})",
        "选择具体模型与推理强度（当前：{0}）",
    ),
    (
        "Model selection is disabled until startup completes.",
        "启动完成前无法选择模型。",
    ),
    (
        "Models are being updated; please try /model again in a moment.",
        "模型正在更新，请稍后再试 /model。",
    ),
    ("Select Model", "选择模型"),
    ("All models", "全部模型"),
    (
        "Pick a quick auto mode or browse all models.",
        "选择一种自动模式，或浏览全部模型。",
    ),
    ("Select Model and Effort", "选择模型与推理强度"),
    (
        "Access legacy models by running codex -m <model_name> or in your config.toml",
        "运行codex -m <model_name> 或在config.toml中访问旧版模型",
    ),
    (
        "No additional models are available right now.",
        "当前没有其他可用模型。",
    ),
    ("no reasoning", "无推理"),
    ("the selected reasoning", "所选推理强度"),
    ("{0} reasoning", "{0} 推理"),
    ("Always use {0} in Plan mode.", "在计划模式下始终使用 {0}。"),
    (
        "user-chosen Plan override ({0})",
        "用户选择的计划模式覆盖（{0}）",
    ),
    ("built-in Plan default ({0})", "内置计划模式默认值（{0}）"),
    (
        "built-in Plan default (no reasoning)",
        "内置计划模式默认值（无推理）",
    ),
    ("built-in Plan default", "内置计划模式默认值"),
    (
        "Set the global default reasoning level and the Plan mode override. This replaces the current {0}.",
        "设置全局默认推理强度与计划模式覆盖。这会替换当前的 {0}。",
    ),
    ("Choose where to apply {0}.", "选择将 {0} 应用到何处。"),
    (
        "⚠ {0} reasoning effort can quickly consume Plus plan rate limits.",
        "⚠ {0} 推理强度会很快消耗Plus套餐的用量限额。",
    ),
    ("More reasoning…", "更多推理强度…"),
    ("Select Reasoning Level for {0}", "为 {0} 选择推理强度"),
    (
        "Ultra reasoning may proactively use multiple agents. This session is configured for {0} concurrent threads with up to {1} subagents which can increase usage quickly. Consider setting features.multi_agent_v2.max_concurrent_threads_per_session below 8.",
        "Ultra推理可能会主动使用多个代理。本次会话配置为 {0} 个并发线程、最多 {1} 个子代理，这会很快增加用量。建议把features.multi_agent_v2.max_concurrent_threads_per_session设为小于8。",
    ),
    // Windows sandbox prompts
    // (`tui/src/chatwidget/windows_sandbox_prompts.rs`). The mode names are the
    // labels of `SandboxMode`, so they read as bare nouns in Chinese ("应用完全访问模式"),
    // which is why the sentence templates below put no space after the mode.
    ("Full Access mode", "完全访问模式"),
    ("Agent mode", "代理模式"),
    ("Read-Only mode", "只读模式"),
    (
        "The Windows sandbox cannot guarantee protection in {0}.",
        "Windows沙箱在{0}下无法保证保护。",
    ),
    (
        "The Windows sandbox cannot protect writes to folders that are writable by Everyone.",
        "Windows沙箱无法保护Everyone可写的文件夹。",
    ),
    ("and {0} more", "以及另外 {0} 项"),
    ("Continue", "继续"),
    ("Apply {0} for this session", "本次会话应用{0}"),
    ("Continue and don't warn again", "继续且不再警告"),
    ("Enable {0} and remember this choice", "启用{0}并记住此选择"),
    (
        "Set up default sandbox (requires Administrator permissions)",
        "设置默认沙箱（需要管理员权限）",
    ),
    // Plugins catalogue (`tui/src/chatwidget/plugin_catalog.rs`). This file holds
    // a `const` table of marketplace sections: a `const` cannot call `tr`, so the
    // table keeps its English text and the *use sites* render it through
    // `tr(current(), section.field)` -- the same shape as the footer's labels.
    ("Plugins", "插件"),
    ("OpenAI Curated", "OpenAI精选"),
    ("Workspace", "工作区"),
    ("Shared with me", "与我共享"),
    ("Shared with me (link)", "与我共享（链接）"),
    ("Local", "本地"),
    ("No workspace plugins available", "没有可用的工作区插件"),
    (
        "No workspace directory plugins are available.",
        "没有可用的工作区目录插件。",
    ),
    ("No shared plugins available", "没有可用的共享插件"),
    (
        "No plugins have been shared with you.",
        "还没有人向你共享插件。",
    ),
    ("Loading available plugins...", "正在加载可用插件…"),
    ("Loading plugins...", "正在加载插件…"),
    ("Adding marketplace...", "正在添加市场源…"),
    ("Remove {0} marketplace?", "要移除市场源 {0} 吗？"),
    (" select", " 选择"),
    ("esc close", "Esc关闭"),
    ("Remove marketplace", "移除市场源"),
    ("Back to plugins", "返回插件"),
    ("Keep this marketplace installed.", "保留已安装的市场源。"),
    ("Removing {0}...", "正在移除 {0}…"),
    ("{0} unavailable.", "{0} 不可用。"),
    (
        "Local plugin functionality is still available.",
        "本地插件功能仍然可用。",
    ),
    // Plugin tab labels. These live in `const` tables (`tui/src/chatwidget/plugins.rs`)
    // and are rendered through a variable at the use site, which is why the
    // drift check reports them under "bound keys" rather than as call sites.
    ("All Plugins", "全部插件"),
    ("Add Marketplace", "添加市场源"),
    // Goal menu and goal status (`tui/src/chatwidget/goal_menu.rs`). Slash
    // commands (`/goal resume`) stay verbatim -- they are things the user types.
    ("Edit goal", "编辑目标"),
    (
        "Type a goal objective and press Enter",
        "输入目标内容后按Enter",
    ),
    ("Resume paused goal?", "恢复已暂停的目标？"),
    ("Goal: {0}", "目标：{0}"),
    ("Resume goal", "恢复目标"),
    (
        "Mark it active and continue when idle",
        "标记为活跃，空闲时继续",
    ),
    ("Leave paused", "保持暂停"),
    (
        "Keep it paused; use /goal resume later",
        "保持暂停；稍后用 /goal resume",
    ),
    ("Status: ", "状态："),
    ("Objective: ", "目标："),
    ("Time used: ", "已用时间："),
    ("Tokens used: ", "已用token："),
    ("Token budget: ", "token预算："),
    (
        "Commands: /goal edit, /goal pause, /goal clear",
        "命令：/goal edit、/goal pause、/goal clear",
    ),
    (
        "Commands: /goal edit, /goal resume, /goal clear",
        "命令：/goal edit、/goal resume、/goal clear",
    ),
    (
        "Commands: /goal edit, /goal clear",
        "命令：/goal edit、/goal clear",
    ),
    ("active", "进行中"),
    ("paused", "已暂停"),
    ("stalled", "停滞"),
    ("usage limited", "用量受限"),
    ("limited by budget", "预算受限"),
    ("complete", "已完成"),
    // Interaction helpers (`tui/src/chatwidget/interaction.rs`): clipboard copy,
    // Ctrl+L, and thread renaming. `tracing::` log lines in that file are
    // deliberately left alone -- they are diagnostics, not UI.
    (
        "Ctrl+L is disabled while a task is in progress.",
        "任务进行中时Ctrl+L已禁用。",
    ),
    (
        "Copied last message to clipboard",
        "已复制最后一条消息到剪贴板",
    ),
    ("No agent response to copy", "没有可复制的代理回复"),
    ("Whole status", "整个状态栏"),
    ("Whole response", "整条回复"),
    ("Code block", "代码块"),
    ("Blockquote", "引用块"),
    ("Copy to clipboard", "复制到剪贴板"),
    ("Copied {0} to clipboard", "已复制{0}到剪贴板"),
    ("Type a name and press Enter", "输入名称后按Enter"),
    ("Thread name cannot be empty.", "线程名不能为空。"),
    ("Generating a title suggestion…", "正在生成标题建议…"),
    ("Suggested from this conversation", "根据本次对话生成"),
    // Compaction, permissions menu, and the two remaining interaction messages.
    // `compaction.rs` used to export its two strings as `const`s; they are now
    // functions so the text can pass through `tr` (see §3.6 of the design doc),
    // and the three call sites were updated with them.
    ("Compacting context", "正在压缩上下文"),
    ("Making room to continue.", "腾出空间以继续。"),
    ("Working", "处理中"),
    ("Context compacted", "上下文已压缩"),
    ("Context compacted · {0}", "上下文已压缩 · {0}"),
    (
        "Steer messages aren't supported during /review. Press Ctrl+C now to cancel the review.",
        "在 /review期间不支持引导消息。现在按Ctrl+C可取消审查。",
    ),
    ("Copy failed: {0}", "复制失败：{0}"),
    ("Update Model Permissions", "更新模型权限"),
    ("Disabled by requirements.", "已被要求禁用。"),
    ("Current permission profile.", "当前权限配置。"),
    ("Configured permission profile.", "已配置的权限配置。"),
    ("Not available on this server.", "此服务器上不可用。"),
    (
        "No permission profiles returned by the server.",
        "服务器未返回任何权限配置。",
    ),
    (
        "Internal error: missing the 'read-only' approval preset.",
        "内部错误：缺少 'read-only' 审批预设。",
    ),
    (
        "Internal error: missing the 'full-access' approval preset.",
        "内部错误：缺少 'full-access' 审批预设。",
    ),
    // Marketplace operations (`tui/src/chatwidget/plugins.rs`). `marketplace` and
    // `marketplaces` are two keys with one translation -- Chinese has no plural,
    // so the count template below reads "已升级 2 个市场源" for both.
    (
        "Marketplace {0} is already added.",
        "市场源 {0} 已添加过了。",
    ),
    ("Added marketplace {0}.", "已添加市场源 {0}。"),
    ("Marketplace root: {0}", "市场源根目录：{0}"),
    ("Removed marketplace {0}.", "已移除市场源 {0}。"),
    (
        "Removed marketplace config for {0}.",
        "已移除 {0} 的市场源配置。",
    ),
    (
        "No configured Git marketplaces to upgrade.",
        "没有可升级的已配置Git市场源。",
    ),
    (
        "Only configured Git marketplaces can be upgraded.",
        "只能升级已配置的Git市场源。",
    ),
    (
        "Marketplace {0} is already up to date.",
        "市场源 {0} 已是最新。",
    ),
    (
        "Checked {0} marketplaces; all are already up to date.",
        "已检查 {0} 个市场源，全部都是最新。",
    ),
    ("Checked: {0}", "已检查：{0}"),
    ("marketplace", "市场源"),
    ("marketplaces", "市场源"),
    ("Upgraded {0} {1}.", "已升级 {0} 个{1}。"),
    // Side conversations (`tui/src/app/side.rs`), the first slice of the `app`
    // step. `SIDE_BOUNDARY_PROMPT` in that file is deliberately *not* here: it is
    // fed to the model, and translating it would change what the model sees
    // (design §4, decision 3).
    (
        "Side conversations are ephemeral and cannot be renamed.",
        "侧边对话是临时的，不能重命名。",
    ),
    (
        "'/side' is unavailable until the main thread is ready.",
        "主线程就绪之前无法使用 '/side'。",
    ),
    (
        "'/side' is unavailable until the current conversation has started. Send a message first, then try /side again.",
        "当前对话开始之前无法使用 '/side'。请先发送一条消息，然后再试 /side。",
    ),
    (
        "A side conversation is already open. Press ctrl + c to return before starting another.",
        "已经有一个侧边对话处于打开状态。请先按ctrl + c返回再新建。",
    ),
    ("main needs input", "主线程需要输入"),
    ("parent needs input", "父线程需要输入"),
    ("main needs approval", "主线程需要审批"),
    ("parent needs approval", "父线程需要审批"),
    ("main failed", "主线程失败"),
    ("parent failed", "父线程失败"),
    ("main interrupted", "主线程已中断"),
    ("parent interrupted", "父线程已中断"),
    ("main closed", "主线程已关闭"),
    ("parent closed", "父线程已关闭"),
    ("main finished", "主线程已完成"),
    ("parent finished", "父线程已完成"),
    // Background request failures (`tui/src/app/background_requests.rs`). Only the
    // messages the user actually sees are here: the `.wrap_err("… failed in TUI")`
    // contexts next to them are error-chain diagnostics and stay English.
    ("Failed to add marketplace: {0}", "添加市场失败：{0}"),
    ("Failed to remove marketplace: {0}", "移除市场失败：{0}"),
    ("Failed to upgrade marketplace: {0}", "升级市场失败：{0}"),
    ("Failed to install plugin: {0}", "安装插件失败：{0}"),
    ("Failed to uninstall plugin: {0}", "卸载插件失败：{0}"),
    (
        "Failed to update plugin config: {0}",
        "更新插件配置失败：{0}",
    ),
    ("Failed to update hook config: {0}", "更新钩子配置失败：{0}"),
    ("Failed to trust hook: {0}", "信任钩子失败：{0}"),
    ("Failed to trust hooks: {0}", "信任钩子失败：{0}"),
    ("Failed to upload feedback: {0}", "上传反馈失败：{0}"),
    ("Failed to load MCP inventory: {0}", "加载MCP清单失败：{0}"),
    // Permission/config changes (`tui/src/app/config_persistence.rs`). The
    // lowercase `wrap_err("failed to set …")` contexts in that file are
    // diagnostics rather than user-facing messages and stay English.
    ("Permissions updated to {0}", "权限已更新为 {0}"),
    (
        "Wait for the task to connect before selecting permissions.",
        "请等待任务连接后再选择权限。",
    ),
    (
        "Wait for the current turn to finish before changing permissions.",
        "请等待当前回合结束后再更改权限。",
    ),
    (
        "Failed to rebuild config for cwd {0}",
        "为工作目录 {0} 重建配置失败",
    ),
    (
        "Failed to set permission profile `{0}`: {1}",
        "设置权限配置 `{0}` 失败：{1}",
    ),
    // Event dispatch (`tui/src/app/event_dispatch.rs`): the messages handed to
    // `add_error_message` / `add_info_message` / `add_warning_message`, i.e. the
    // ones the user actually reads. The `wrap_err("…")` and telemetry strings in
    // the same file stay English (see §3.6 of the design doc).
    ("A worktree is already being created.", "已在创建工作树。"),
    (
        "Changing directories is not supported for remote workspaces or remote execution environments.",
        "远程工作区或远程执行环境不支持切换目录。",
    ),
    ("Not a directory: {0}", "不是目录：{0}"),
    ("Cannot access directory {0}: {1}", "无法访问目录 {0}：{1}"),
    (
        "No saved chat found matching '{0}'.",
        "没有找到与 '{0}' 匹配的已保存对话。",
    ),
    (
        "Failed to name the forked session: {0}",
        "为分叉会话命名失败：{0}",
    ),
    (
        "Failed to attach to forked app-server thread: {0}",
        "附加到分叉的app-server线程失败：{0}",
    ),
    (
        "Failed to fork current session through the app server: {0}",
        "通过app server分叉当前会话失败：{0}",
    ),
    (
        "A thread must contain at least one turn before it can be forked.",
        "线程至少要包含一个回合才能分叉。",
    ),
    (
        "Wait for permissions to update before editing this prompt.",
        "请等待权限更新后再编辑此提示。",
    ),
    ("Export failed: {0}", "导出失败：{0}"),
    ("Failed to interrupt task: {0}", "中断任务失败：{0}"),
    ("Failed to pause task goal: {0}", "暂停任务目标失败：{0}"),
    ("Logout failed: {0}", "退出登录失败：{0}"),
    // Thread goal actions (`tui/src/app/thread_goal_actions.rs`). The first entry
    // is the `concat!` const that became a function; its key carries the newline
    // the two concatenated parts produced, so the escape below is load-bearing.
    (
        "Goals need a saved session. This session is temporary.\nRun `codex` to start a saved session, or `codex resume` / `/resume` to reopen one.",
        "目标功能需要已保存的会话。当前会话是临时的。\n请运行 `codex` 开启已保存的会话，或用 `codex resume` / `/resume` 重新打开一个。",
    ),
    ("Goal {0}", "目标{0}"),
    ("Goal cleared", "目标已清除"),
    ("No goal is currently set.", "当前没有设置目标。"),
    ("No goal to clear", "没有可清除的目标"),
    (
        "This thread does not currently have a goal.",
        "此线程当前没有目标。",
    ),
    ("Replace current goal", "替换当前目标"),
    (
        "Set the new objective and start it now",
        "设置新目标并立即开始",
    ),
    ("Keep the current goal", "保留当前目标"),
    ("Replace goal?", "替换目标？"),
    ("New objective: {0}", "新目标：{0}"),
    ("Create a goal before editing it.", "请先创建目标再编辑。"),
    // Working-directory changes (`tui/src/app/working_directory.rs`). The three
    // worktree messages keep the literal `git worktree remove <checkout-path>`
    // command as written -- the user has to type it.
    (
        "This task cannot be safely replaced.",
        "无法安全替换此任务。",
    ),
    (
        "Wait for permissions to update before changing directories.",
        "请等待权限更新后再切换目录。",
    ),
    (
        "Changing directories with a named profile is not supported.",
        "不支持在指定配置文件的情况下切换目录。",
    ),
    ("MCP inventory is still loading.", "MCP清单仍在加载中。"),
    (
        "Cannot change: another agent is running.",
        "无法切换：另一个代理正在运行。",
    ),
    (
        "Cannot continue into the new worktree while the session is offline or blocked by a policy warning. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "会话离线或被策略警告阻止时，无法继续进入新工作树。已在 {0} 创建了一个未使用的检出；请在源仓库中运行 `git worktree remove <checkout-path>` 将其移除。",
    ),
    (
        "The source conversation changed while creating the worktree. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "创建工作树期间源对话发生了变化。已在 {0} 创建了一个未使用的检出；请在源仓库中运行 `git worktree remove <checkout-path>` 将其移除。",
    ),
    (
        "Could not start a session in the managed worktree. A checkout was retained at {0}; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.",
        "无法在受管工作树中启动会话。检出保留在 {0}；如果不再需要，请在源仓库中运行 `git worktree remove <checkout-path>` 将其移除。",
    ),
    // Session lifecycle (`tui/src/app/session_lifecycle.rs`). The `.contains("…")`
    // checks next to these match server error text rather than rendering it.
    ("No agents available yet.", "暂无可用代理。"),
    ("Subagents", "子代理"),
    (
        "Agent thread {0} is not yet available for replay or live attach.",
        "代理线程 {0} 尚不能用于回放或实时附加。",
    ),
    (
        "Agent thread {0} is no longer available.",
        "代理线程 {0} 已不可用。",
    ),
    (
        "Failed to attach to agent thread {0}: {1}",
        "附加到代理线程 {0} 失败：{1}",
    ),
    // Thread routing (`tui/src/app/thread_routing.rs`). The `message: format!(…)`
    // values that are sent *to the app-server* (rejections, serialization
    // failures) are protocol payloads, not UI, and stay English.
    ("Main [default]", "主线程 [默认]"),
    ("Agent ({0})", "代理 ({0})"),
    ("Already viewing {0}.", "已在查看{0}。"),
    ("No active thread is available.", "没有可用的活动线程。"),
    (
        "This conversation is read-only or unavailable; no operation was sent.",
        "此对话为只读或不可用；未发送任何操作。",
    ),
    (
        "Not available in TUI yet for thread {0}.",
        "TUI中暂不支持线程 {0} 的此操作。",
    ),
    // Keymap inventory, first batch (`tui/src/keymap_setup/actions.rs`). The table
    // is built by `keymap_actions()` at runtime because a `const` cannot call
    // `tr` (design §3.6); the remaining entries are wrapped in later batches.
    // Context labels:
    ("Global", "全局"),
    ("Chat", "对话"),
    ("Composer", "输入框"),
    ("Editor", "编辑器"),
    // Descriptions:
    (
        "Open the shared agent-session overview.",
        "打开共享的代理会话总览。",
    ),
    ("Open the transcript overlay.", "打开记录浮层。"),
    (
        "Open the current draft in an external editor.",
        "用外部编辑器打开当前草稿。",
    ),
    (
        "Copy the last agent response to the clipboard.",
        "复制最后一条代理回复到剪贴板。",
    ),
    ("Clear the terminal UI.", "清空终端界面。"),
    ("Turn Vim composer mode on or off.", "开关Vim输入模式。"),
    ("Turn Fast mode on or off.", "开关Fast模式。"),
    ("Toggle raw scrollback mode.", "切换原始回滚模式。"),
    (
        "Switch between a side conversation and its parent.",
        "在侧边对话与其父对话之间切换。",
    ),
    ("Interrupt the active turn.", "中断当前回合。"),
    ("Decrease reasoning effort.", "降低推理强度。"),
    ("Increase reasoning effort.", "提高推理强度。"),
    (
        "Switch to the previous available permission mode.",
        "切换到上一个可用的权限模式。",
    ),
    (
        "Switch to the next available permission mode.",
        "切换到下一个可用的权限模式。",
    ),
    (
        "Move up through questions, then edit the last queued message.",
        "在问题间向上移动，然后编辑最后一条排队消息。",
    ),
    (
        "Move back through questions toward the composer.",
        "在问题间向后返回到输入框。",
    ),
    ("Skip the focused question.", "跳过当前聚焦的问题。"),
    ("Submit the current composer draft.", "提交当前输入内容。"),
    (
        "Queue the draft while a task is running.",
        "任务运行时把草稿加入队列。",
    ),
    (
        "Show or hide the composer shortcut overlay.",
        "显示或隐藏输入快捷键浮层。",
    ),
    (
        "Open history search or move to the previous match.",
        "打开历史搜索，或移动到上一个匹配项。",
    ),
    (
        "Move to the next history search match.",
        "移动到下一个历史搜索匹配项。",
    ),
    ("Insert a newline in the editor.", "在编辑器中插入换行。"),
    ("Move the cursor left.", "光标左移。"),
    // Keymap inventory, second batch (editor + Vim normal groups).
    ("Vim normal", "Vim普通模式"),
    ("Move the cursor right.", "光标右移。"),
    ("Move the cursor up.", "光标上移。"),
    ("Move the cursor down.", "光标下移。"),
    (
        "Move to the beginning of the previous word.",
        "移动到上一个词的开头。",
    ),
    (
        "Move to the end of the next word.",
        "移动到下一个词的结尾。",
    ),
    ("Move to the beginning of the line.", "移动到行首。"),
    ("Move to the end of the line.", "移动到行尾。"),
    ("Delete one grapheme to the left.", "删除左侧一个字形。"),
    ("Delete one grapheme to the right.", "删除右侧一个字形。"),
    ("Delete the previous word.", "删除上一个词。"),
    ("Delete the next word.", "删除下一个词。"),
    ("Delete from cursor to line start.", "从光标删除到行首。"),
    ("Delete the current line.", "删除当前行。"),
    ("Delete from cursor to line end.", "从光标删除到行尾。"),
    ("Paste the kill buffer.", "粘贴删除缓冲区的内容。"),
    ("Enter insert mode at the cursor.", "在光标处进入插入模式。"),
    (
        "Enter insert mode after the cursor.",
        "在光标之后进入插入模式。",
    ),
    ("Enter insert mode at end of line.", "在行尾进入插入模式。"),
    (
        "Enter insert mode at the first non-blank character.",
        "在第一个非空白字符处进入插入模式。",
    ),
    (
        "Open a new line below and enter insert mode.",
        "在下方新建一行并进入插入模式。",
    ),
    (
        "Open a new line above and enter insert mode.",
        "在上方新建一行并进入插入模式。",
    ),
    (
        "Enter replace mode and overwrite characters.",
        "进入替换模式并覆盖字符。",
    ),
    ("Move left in Vim normal mode.", "在Vim普通模式下左移。"),
    ("Move right in Vim normal mode.", "在Vim普通模式下右移。"),
    // Keymap inventory, third batch (Vim normal mode, continued).
    (
        "Move up or recall older history in Vim normal mode.",
        "在Vim普通模式下上移或回溯更早的历史。",
    ),
    (
        "Move down or recall newer history in Vim normal mode.",
        "在Vim普通模式下下移或回溯更新的历史。",
    ),
    (
        "Move to the start of the next word.",
        "移动到下一个词的开头。",
    ),
    (
        "Move to the start of the previous word.",
        "移动到上一个词的开头。",
    ),
    (
        "Move to the end of the current or next word.",
        "移动到当前词或下一个词的结尾。",
    ),
    ("Move to the start of the line.", "移动到行首。"),
    (
        "Find the next character on the current line.",
        "在当前行查找下一个字符。",
    ),
    (
        "Find the previous character on the current line.",
        "在当前行查找上一个字符。",
    ),
    (
        "Stop before the next character on the current line.",
        "停在当前行下一个字符之前。",
    ),
    (
        "Stop after the previous character on the current line.",
        "停在当前行上一个字符之后。",
    ),
    ("Jump to the first buffer line.", "跳到缓冲区第一行。"),
    ("Jump to the last buffer line.", "跳到缓冲区最后一行。"),
    (
        "Delete the character under the cursor.",
        "删除光标下的字符。",
    ),
    (
        "Replace the character under the cursor.",
        "替换光标下的字符。",
    ),
    ("Repeat the last complete edit.", "重复上一次完整编辑。"),
    (
        "Delete the character under the cursor and enter insert mode.",
        "删除光标下的字符并进入插入模式。",
    ),
    ("Delete from cursor to end of line.", "从光标删除到行尾。"),
    (
        "Change from cursor to end of line and enter insert mode.",
        "从光标修改到行尾并进入插入模式。",
    ),
    ("Yank the entire line.", "复制整行。"),
    ("Paste after the cursor.", "粘贴到光标之后。"),
    (
        "Begin a delete operator and wait for a motion.",
        "开始删除操作符并等待动作。",
    ),
    (
        "Begin a yank operator and wait for a motion.",
        "开始复制操作符并等待动作。",
    ),
    (
        "Begin a change operator and wait for a motion or text object.",
        "开始修改操作符并等待动作或文本对象。",
    ),
    // Keymap inventory, fourth batch (Vim search + Vim operator groups).
    ("Vim search", "Vim搜索"),
    ("Vim operator", "Vim操作符"),
    ("Undo the last complete edit.", "撤销上一次完整编辑。"),
    ("Redo the last undone edit.", "重做上一次撤销的编辑。"),
    ("Cancel a pending Vim operator.", "取消待处理的Vim操作符。"),
    (
        "Search forward in the active buffer.",
        "在当前缓冲区向前搜索。",
    ),
    (
        "Search backward in the active buffer.",
        "在当前缓冲区向后搜索。",
    ),
    ("Repeat the accepted search.", "重复已接受的搜索。"),
    (
        "Repeat the search in the opposite direction.",
        "朝相反方向重复搜索。",
    ),
    (
        "Repeat delete operator to delete the whole line.",
        "重复删除操作符以删除整行。",
    ),
    (
        "Repeat yank operator to yank the whole line.",
        "重复复制操作符以复制整行。",
    ),
    ("Operator motion left.", "操作符动作：左移。"),
    ("Operator motion right.", "操作符动作：右移。"),
    ("Operator motion up.", "操作符动作：上移。"),
    ("Operator motion down.", "操作符动作：下移。"),
    (
        "Operator motion to start of next word.",
        "操作符动作：移到下一个词的开头。",
    ),
    (
        "Operator motion to start of previous word.",
        "操作符动作：移到上一个词的开头。",
    ),
    (
        "Operator motion to end of word.",
        "操作符动作：移到词的结尾。",
    ),
    ("Operator motion to line start.", "操作符动作：移到行首。"),
    ("Operator motion to line end.", "操作符动作：移到行尾。"),
    (
        "Operator motion to the next character on the current line.",
        "操作符动作：移到当前行的下一个字符。",
    ),
    (
        "Operator motion to the previous character on the current line.",
        "操作符动作：移到当前行的上一个字符。",
    ),
    (
        "Operator motion to the first buffer line.",
        "操作符动作：移到缓冲区第一行。",
    ),
    (
        "Operator motion to the last buffer line.",
        "操作符动作：移到缓冲区最后一行。",
    ),
    // Keymap inventory, fifth batch (Vim text objects, pager, list).
    ("Vim text object", "Vim文本对象"),
    ("Pager", "分页视图"),
    ("List", "列表"),
    ("Select an inner text object.", "选择内部文本对象。"),
    ("Select an around text object.", "选择环绕文本对象。"),
    ("Cancel the pending operator.", "取消待处理的操作符。"),
    ("Target the current word.", "定位到当前词。"),
    ("Target the current WORD.", "定位到当前WORD。"),
    ("Target enclosing parentheses.", "定位到外层圆括号。"),
    ("Target enclosing brackets.", "定位到外层方括号。"),
    ("Target enclosing braces.", "定位到外层花括号。"),
    ("Target enclosing double quotes.", "定位到外层双引号。"),
    ("Target enclosing single quotes.", "定位到外层单引号。"),
    ("Target enclosing backticks.", "定位到外层反引号。"),
    ("Cancel the pending text object.", "取消待处理的文本对象。"),
    ("Scroll up by one row.", "上移一行。"),
    ("Scroll down by one row.", "下移一行。"),
    ("Scroll up by one page.", "上翻一页。"),
    ("Scroll down by one page.", "下翻一页。"),
    ("Scroll up by half a page.", "上翻半页。"),
    ("Scroll down by half a page.", "下翻半页。"),
    ("Jump to the beginning.", "跳到开头。"),
    ("Jump to the end.", "跳到结尾。"),
    ("Close the pager overlay.", "关闭分页浮层。"),
    ("Close the transcript overlay.", "关闭记录浮层。"),
    ("Move list selection up.", "列表选中项上移。"),
    ("Move list selection down.", "列表选中项下移。"),
    // Keymap inventory, sixth batch — this completes the 142-entry table.
    ("Agents", "代理"),
    ("Approval", "审批"),
    (
        "Move horizontally left in list pickers.",
        "在列表选择中左移。",
    ),
    (
        "Move horizontally right in list pickers.",
        "在列表选择中右移。",
    ),
    (
        "Move list selection up by one page.",
        "列表选中项上移一页。",
    ),
    (
        "Move list selection down by one page.",
        "列表选中项下移一页。",
    ),
    ("Jump to the first list item.", "跳到列表第一项。"),
    ("Jump to the last list item.", "跳到列表最后一项。"),
    ("Accept the current list selection.", "接受当前列表选择。"),
    ("Cancel and close selection views.", "取消并关闭选择视图。"),
    ("Open the session resume picker.", "打开会话恢复选择器。"),
    ("Search the available agent tasks.", "搜索可用的代理任务。"),
    (
        "Start composing a new agent task.",
        "开始编写新的代理任务。",
    ),
    ("Rename the selected task.", "重命名所选任务。"),
    ("Stop the selected running task.", "停止所选的运行中任务。"),
    (
        "Group tasks by status or project.",
        "按状态或项目分组任务。",
    ),
    ("Open approval details fullscreen.", "全屏打开审批详情。"),
    (
        "Open the approval source thread when available.",
        "在可用时打开审批来源线程。",
    ),
    ("Approve the primary option.", "批准主要选项。"),
    (
        "Approve for the session when available.",
        "在可用时批准本次会话。",
    ),
    (
        "Approve with an exec-policy prefix when available.",
        "在可用时按exec-policy前缀批准。",
    ),
    (
        "Choose the explicit deny option when available.",
        "在可用时选择明确的拒绝选项。",
    ),
    (
        "Decline and provide corrective guidance.",
        "拒绝并给出纠正建议。",
    ),
    ("Cancel an elicitation request.", "取消一次信息请求。"),
    // History cells: hook runs (`tui/src/history_cell/hook_cell.rs`) and notices
    // (`tui/src/history_cell/notices.rs`). "hook" is a Codex concept and stays as
    // 钩子; the provenance prefix keeps its arrow and separator spacing.
    ("Running hooks", "正在运行钩子"),
    ("Running hook", "正在运行钩子"),
    ("{0}% used", "已用{0}%"),
    ("{0}d (best {1}d)", "{0}天（最长{1}天）"),
    ("{0}d ago", "{0}天前"),
    ("{0}d {1}h {2}m", "{0}天{1}小时{2}分"),
    ("{0}h", "{0}小时"),
    ("{0}h ago", "{0}小时前"),
    ("{0}h {1}m", "{0}小时{1}分"),
    ("{0}m", "{0}分"),
    ("{0}m ago", "{0}分前"),
    ("{0}m {1}s", "{0}分{1}秒"),
    ("{0}s", "{0}秒"),
    ("{0}s ago", "{0}秒前"),
    ("• Copied conversation to clipboard", "• 已复制会话到剪贴板"),
    ("• Saved conversation to ", "• 会话已保存到"),
    ("• Waited for background terminal", "• 已等待后台终端"),
    ("←/→ to navigate questions", "←/→ 切换问题"),
    ("↳ Hook · ", "↳ 钩子 · "),
    ("Hook completed", "钩子已完成"),
    ("Hook failed", "钩子失败"),
    ("Blocked by hook", "被钩子阻止"),
    ("Hook stopped", "钩子已停止"),
    ("Hook running", "钩子运行中"),
    ("Conversation recap", "对话回顾"),
    ("Update available!", "有可用更新！"),
    ("See full release notes:", "查看完整发布说明："),
    ("Run {0} to update.", "运行 {0} 进行更新。"),
    ("This content can't be shown", "此内容无法显示"),
    ("This content can’t be shown", "此内容无法显示"),
    ("Trusted Access", "受信任访问"),
    ("Learn more", "了解更多"),
    ("Apply for Daybreak", "申请Daybreak"),
    (
        "We take extra caution with requests involving biological research and applications that could pose safety risks. Eligible researchers can apply for Trusted Access.",
        "对于涉及生物研究以及可能带来安全风险的应用的请求，我们会格外谨慎。符合条件的研究人员可以申请受信任访问。",
    ),
    (
        "We take extra care with some cybersecurity requests. If you’re doing authorized security work, apply for Daybreak to get broader access.",
        "我们对某些网络安全请求会格外谨慎。如果你从事获授权的安全工作，可以申请Daybreak以获得更宽的访问权限。",
    ),
    (
        "Daybreak isn’t available for Astra. Some cybersecurity requests may still be limited.",
        "Daybreak不适用于Astra。部分网络安全请求仍可能受到限制。",
    ),
    (
        "We take extra care with some cybersecurity requests.",
        "我们对某些网络安全请求会格外谨慎。",
    ),
    // History cells, second batch: approvals, MCP runs, session info
    // (`tui/src/history_cell/{approvals,mcp,session}.rs`). These are short spans
    // glued into lines, so the translations stay equally short and keep any
    // leading/trailing space the English has.
    ("approved", "已批准"),
    ("did not approve", "未批准"),
    ("persisted", "已持久化"),
    ("denied", "已拒绝"),
    ("Request ", "请求 "),
    ("Review ", "审查 "),
    ("Unknown", "未知"),
    ("Unsupported", "不支持"),
    ("Not logged in", "未登录"),
    ("Bearer token", "Bearer令牌"),
    ("tool result (image output)", "工具结果（图片输出）"),
    ("MCP docs", "MCP文档"),
    ("Error: {0}", "错误：{0}"),
    ("Tip: {0}", "提示：{0}"),
    ("model changed:", "模型已更改："),
    ("requested: {0}", "请求：{0}"),
    ("used: {0}", "实际使用：{0}"),
    ("OpenAI Codex", "OpenAI Codex"),
    ("OpenAI Codex (v{0})", "OpenAI Codex (v{0})"),
    ("YOLO mode", "YOLO模式"),
    ("permissions: YOLO mode", "权限：YOLO模式"),
    // Sentence *fragments* that get glued around a key binding, a command or a
    // value (`tui/src/history_cell/approvals.rs`). They are translated as glue
    // rather than as whole sentences -- Chinese keeps the same value order here,
    // which is what makes fragment translation viable at all; where it does not
    // (see the verb table in `model_popups.rs`), the sentence must be rebuilt
    // first. Leading/trailing spaces are part of the key.
    (" codex to run ", " 让codex运行 "),
    (" this time", "（仅此次）"),
    (" this request", "（本次请求）"),
    (" codex network access to ", " 让codex访问网络 "),
    (" every time this session", "（本次会话内每次都允许）"),
    (" and saved that rule", "，并保存了该规则"),
    (" Codex network access to ", " Codex访问网络 "),
    (" for codex to run ", " 由codex运行 "),
    // Status card (`tui/src/status/card.rs`). "workspace with network access" is
    // deliberately absent: in that file it is a *match pattern*, and translating
    // a pattern would break the match under zh (the pattern is compared, not
    // rendered).
    ("not available for this account", "此账户不可用"),
    (
        "limits may be stale - run /status again shortly.",
        "限额数据可能已过期 - 请稍后重新运行 /status。",
    ),
    (
        "limits may be stale - start new turn to refresh.",
        "限额数据可能已过期 - 开始新回合以刷新。",
    ),
    (
        "refresh requested; run /status again shortly.",
        "已请求刷新；请稍后重新运行 /status。",
    ),
    ("data not available yet", "数据尚不可用"),
    ("read-only with network access", "只读并允许网络访问"),
    (
        "custom permissions with network access",
        "自定义权限并允许网络访问",
    ),
    ("Read Only with network access", "只读并允许网络访问"),
    ("Read Only", "只读"),
    ("Full Access", "完全访问"),
    (" used / ", " 已用 / "),
    // Status card, second batch: the permission/sandbox labels and the row
    // labels. The templates were rewritten from Rust's named captures
    // (`{approval}`) to positional placeholders (`{1}`) because `tr_with` takes
    // positional arguments -- the translation therefore uses `{0}`-style slots
    // too, and the parity test keeps them aligned.
    ("{0} ({1})", "{0}（{1}）"),
    ("Workspace{0} ({1})", "工作区{0}（{1}）"),
    (
        "Workspace with network access{0} ({1})",
        "工作区（允许网络访问）{0}（{1}）",
    ),
    ("No Sandbox ({0})", "无沙箱（{0}）"),
    ("Profile {0} ({1}, {2})", "配置档 {0}（{1}，{2}）"),
    ("Read Only ({0})", "只读（{0}）"),
    ("Custom ({0}, {1})", "自定义（{0}，{1}）"),
    ("{0}% left", "剩余 {0}%"),
    ("(resets {0})", "（{0} 重置）"),
    ("Approve for me", "替我审批"),
    ("Ask for approval", "请求批准"),
    (
        "API key configured (run codex login to use ChatGPT)",
        "已配置API key（运行codex login以使用ChatGPT）",
    ),
    ("Model provider", "模型提供方"),
    ("Thread name", "线程名"),
    ("Forked from", "分叉自"),
    ("Collaboration mode", "协作模式"),
    ("Token usage", "token用量"),
    ("Context window", "上下文窗口"),
    (" for up-to-date", " 以获取最新"),
    ("information on rate limits and credits", "限额与额度信息"),
    // Hooks browser (`tui/src/bottom_pane/hooks_browser_view.rs`). Column headers,
    // detail-row labels and the footer hints; "Press " is glue in front of a key
    // binding, so it carries its trailing space.
    (
        "Lifecycle hooks from config and enabled plugins.",
        "来自配置与已启用插件的生命周期钩子。",
    ),
    (
        "Turn hooks on or off. Your changes are saved automatically.",
        "开启或关闭钩子。改动会自动保存。",
    ),
    ("Installed", "已安装"),
    ("Active", "已启用"),
    ("Review", "审查"),
    ("Description", "描述"),
    ("Issues", "问题"),
    (
        "No hooks installed for this event.",
        "此事件未安装任何钩子。",
    ),
    ("Matcher", "匹配器"),
    ("Source", "来源"),
    ("Command", "命令"),
    ("MCP Server", "MCP服务器"),
    ("MCP Tool", "MCP工具"),
    ("Handler", "处理方式"),
    ("Prompt", "提示词"),
    ("Agent", "代理"),
    ("Timeout", "超时"),
    ("Context", "上下文"),
    ("unlimited", "无限制"),
    ("limit: {0} approximate tokens", "上限：约 {0} 个token"),
    (" to go back", " 返回"),
    (
        "Managed hooks are always on; press ",
        "受管钩子始终开启；按 ",
    ),
    ("Press ", "按 "),
    // Onboarding: ChatGPT/API-key sign-in (`tui/src/onboarding/auth.rs`) and the
    // Amazon Bedrock flow (`tui/src/onboarding/bedrock.rs`). Product and protocol
    // names (ChatGPT, Codex, AWS, Amazon Bedrock, SSO, API key) stay as they are;
    // "AWS profile" uses 配置档 so it stays distinct from the permission profile
    // (权限配置) in the glossary.
    (
        "Sign in with ChatGPT to use Codex as part of your paid plan",
        "使用ChatGPT登录，以通过付费方案使用Codex",
    ),
    (
        "or connect an API key for usage-based billing",
        "或连接API key以按用量计费",
    ),
    ("ChatGPT login is disabled", "ChatGPT登录已被禁用"),
    (
        "Usage included with Plus, Pro, Business, and Enterprise plans",
        "Plus、Pro、Business和Enterprise方案已包含用量",
    ),
    (
        "Sign in from another device with a one-time code",
        "在另一台设备上用一次性验证码登录",
    ),
    ("Sign in with ChatGPT", "使用ChatGPT登录"),
    ("Sign in with Device Code", "使用设备码登录"),
    ("Use an OpenAI API key", "使用OpenAI API key"),
    ("Provide your own API key", "使用你自己的API key"),
    ("Pay for what you use", "按用量付费"),
    ("Use Amazon Bedrock", "使用Amazon Bedrock"),
    ("Connect using your AWS credentials", "使用你的AWS凭据连接"),
    ("Finish signing in via your browser", "在浏览器中完成登录"),
    ("Codex docs", "Codex文档"),
    ("API key login is disabled.", "API key登录已被禁用。"),
    ("Set up Amazon Bedrock", "设置Amazon Bedrock"),
    ("AWS access key ID", "AWS访问密钥ID"),
    ("AWS secret access key", "AWS秘密访问密钥"),
    ("AWS session token (optional)", "AWS会话令牌（可选）"),
    ("Continue with {0}", "继续使用 {0}"),
    ("Use your existing AWS credentials", "使用你现有的AWS凭据"),
    (
        "Use your existing Amazon Bedrock API key",
        "使用你现有的Amazon Bedrock API key",
    ),
    ("Continue with detected credentials", "使用检测到的凭据继续"),
    ("Choose another sign-in method", "选择其他登录方式"),
    ("Other AWS sign-in methods", "其他AWS登录方式"),
    (
        "Use another profile, access keys, or environment variables",
        "使用其他配置档、访问密钥或环境变量",
    ),
    ("AWS profile", "AWS配置档"),
    ("Use AWS SSO or a named profile", "使用AWS SSO或命名配置档"),
    ("AWS access keys", "AWS访问密钥"),
    (
        "Enter an access key ID and secret access key",
        "输入访问密钥ID与秘密访问密钥",
    ),
    ("Environment variables", "环境变量"),
    // Slash-command descriptions (`tui/src/slash_command.rs`). These are the
    // highest-visibility strings in the TUI -- they fill the `/` popup -- and the
    // design doc §6 explicitly asks to start `MUST_TRANSLATE_KEYS` with them. The
    // two `DO NOT USE` debug commands are included on purpose: the popup shows
    // them, so leaving them English would be the only English left in that list.
    ("send logs to maintainers", "把日志发给维护者"),
    (
        "start a new chat during a conversation",
        "在对话中开始新对话",
    ),
    (
        "create an AGENTS.md file with instructions for Codex",
        "创建带Codex指令的AGENTS.md文件",
    ),
    (
        "summarize conversation to prevent hitting the context limit",
        "总结对话以避免触及上下文上限",
    ),
    ("summarize the current conversation now", "立即总结当前对话"),
    (
        "review my current changes and find issues",
        "审查当前改动并找出问题",
    ),
    ("rename the current thread", "重命名当前线程"),
    ("resume a saved chat", "恢复已保存的对话"),
    ("archive this session and exit", "归档本次会话并退出"),
    (
        "permanently delete this session and exit",
        "永久删除本次会话并退出",
    ),
    (
        "clear the terminal and start a new chat",
        "清空终端并开始新对话",
    ),
    ("fork the current chat", "分叉当前对话"),
    (
        "start or continue a conversation in a new worktree",
        "在新工作树中开始或继续对话",
    ),
    (
        "continue this session in the Desktop app",
        "在桌面应用中继续本次会话",
    ),
    ("exit Codex", "退出Codex"),
    (
        "copy the last response or part of it",
        "复制最后一条回复或其一部分",
    ),
    (
        "export the conversation as markdown",
        "把对话导出为markdown",
    ),
    (
        "toggle raw scrollback mode for copy-friendly terminal selection",
        "切换原始回滚模式，便于终端复制选择",
    ),
    (
        "show git diff (including untracked files)",
        "显示git diff（含未跟踪文件）",
    ),
    ("mention a file", "提及一个文件"),
    (
        "use skills to improve how Codex performs specific tasks",
        "用技能改进Codex完成特定任务的方式",
    ),
    (
        "import setup, this project, and recent chats from Claude Code",
        "从Claude Code导入设置、本项目与近期对话",
    ),
    ("view and manage lifecycle hooks", "查看与管理生命周期钩子"),
    (
        "show current session configuration and token usage",
        "显示当前会话配置与token用量",
    ),
    ("change the current working directory", "切换当前工作目录"),
    ("show the current working directory", "显示当前工作目录"),
    (
        "view account usage or use a usage limit reset",
        "查看账户用量或使用一次限额重置",
    ),
    (
        "show config layers and requirement sources for debugging",
        "显示配置层与要求来源，用于调试",
    ),
    (
        "configure which items appear in the terminal title",
        "配置终端标题中显示的项目",
    ),
    (
        "configure which items appear in the status line",
        "配置状态栏中显示的项目",
    ),
    ("choose a syntax highlighting theme", "选择语法高亮主题"),
    ("choose or hide the terminal pet", "选择或隐藏终端宠物"),
    ("list background terminals", "列出后台终端"),
    ("stop all background terminals", "停止所有后台终端"),
    ("DO NOT USE", "请勿使用"),
    (
        "choose what model and reasoning effort to use",
        "选择使用的模型与推理强度",
    ),
    (
        "choose a communication style for Codex",
        "为Codex选择沟通风格",
    ),
    ("switch to Plan mode", "切换到计划模式"),
    (
        "set or view the goal for a long-running task",
        "设置或查看长任务的目标",
    ),
    (
        "view and switch between all active agent sessions",
        "查看并在所有活动代理会话间切换",
    ),
    (
        "switch between this session's subagents",
        "在本次会话的子代理间切换",
    ),
    ("choose what Codex is allowed to do", "选择允许Codex做什么"),
    ("remap TUI shortcuts", "重新映射TUI快捷键"),
    ("toggle Vim mode for the composer", "开关输入框的Vim模式"),
    ("set up elevated agent sandbox", "设置提权代理沙箱"),
    ("toggle experimental features", "开关实验性功能"),
    (
        "approve one retry of a recent auto-review denial",
        "批准对最近一次自动审查拒绝的一次重试",
    ),
    (
        "configure memory use and generation",
        "配置记忆的使用与生成",
    ),
    (
        "list configured MCP tools; use /mcp verbose for details",
        "列出已配置的MCP工具；用 /mcp verbose查看详情",
    ),
    ("manage apps", "管理应用"),
    ("browse plugins", "浏览插件"),
    ("log out of Codex", "退出Codex登录"),
    ("print the rollout file path", "打印rollout文件路径"),
    ("test approval request", "测试审批请求"),
    // Plugins catalogue, second batch (`tui/src/chatwidget/plugin_catalog.rs`):
    // the loading/empty/error copy of the marketplace and tab views. The
    // `OPENAI_CURATED_LOADING_DESCRIPTION` const became a function (§3.6).
    ("Removing marketplace...", "正在移除市场源…"),
    ("Loading plugin details...", "正在加载插件详情…"),
    (
        "This updates when plugin details load.",
        "插件详情加载后会更新。",
    ),
    ("Installing plugin...", "正在安装插件…"),
    ("Uninstalling plugin...", "正在卸载插件…"),
    (
        "This updates when the plugin removal completes.",
        "插件移除完成后会更新。",
    ),
    ("Failed to load plugins.", "加载插件失败。"),
    ("Plugin marketplace unavailable", "插件市场源不可用"),
    ("Failed to add marketplace.", "添加市场源失败。"),
    ("Marketplace add failed", "市场源添加失败"),
    ("Try again", "重试"),
    (
        "Review the confirmation prompt again.",
        "请重新查看确认提示。",
    ),
    ("No marketplace plugins available", "没有可用的市场源插件"),
    (
        "Browse plugins from available marketplaces.",
        "从可用市场源浏览插件。",
    ),
    ("Installed plugins.", "已安装的插件。"),
    ("No installed plugins", "没有已安装的插件"),
    ("No installed plugins.", "没有已安装的插件。"),
    (
        "Loading OpenAI Curated plugins...",
        "正在加载OpenAI精选插件…",
    ),
    ("OpenAI Curated unavailable", "OpenAI精选不可用"),
    (
        "No OpenAI Curated plugins available",
        "没有可用的OpenAI精选插件",
    ),
    (
        "No OpenAI Curated plugins available.",
        "没有可用的OpenAI精选插件。",
    ),
    ("OpenAI Curated marketplace.", "OpenAI精选市场源。"),
    (
        "Select the plugins you want to use and press Enter to install or view details.",
        "选择要使用的插件，按Enter安装或查看详情。",
    ),
    (
        "No plugins available in this marketplace",
        "此市场源中没有可用插件",
    ),
    (
        "No plugins available in this marketplace.",
        "此市场源中没有可用插件。",
    ),
    ("Type to search plugins", "输入以搜索插件"),
    (
        "Press Enter to enter a marketplace source.",
        "按Enter输入市场源。",
    ),
    ("Installed by admin", "由管理员安装"),
    // A sentence the app assembles from links and glue
    // (`plugin_catalog.rs`): every piece including " and " and ". " has to be
    // translated, otherwise the zh sentence has English floating inside it --
    // the fragment rule from §3.6, applied piece by piece because here the
    // value order happens to match Chinese.
    (
        "Data shared with this app is subject to the app's ",
        "与此应用共享的数据受该应用的",
    ),
    ("terms of service", "服务条款"),
    (" and ", " 和 "),
    ("privacy policy", "隐私政策"),
    (". ", "。 "),
    // Plugins catalogue, third batch: detail/action copy and the templates whose
    // named captures became positional placeholders. `toggle_action` is itself
    // translated ("disable"/"enable"), which is what keeps the assembled
    // sentence from mixing languages.
    ("Marketplace removal failed", "市场源移除失败"),
    (
        "Failed to remove the selected marketplace.",
        "移除所选市场源失败。",
    ),
    ("Uninstall plugin", "卸载插件"),
    ("Remove this plugin now.", "立即移除此插件。"),
    ("Install plugin", "安装插件"),
    ("Install this plugin now.", "立即安装此插件。"),
    ("MCP Servers", "MCP服务器"),
    ("plugin details are unavailable", "插件详情不可用"),
    ("Upgrading marketplaces...", "正在升级市场源…"),
    ("Upgrading {0} marketplace...", "正在升级市场源 {0}…"),
    ("Loading details for {0}...", "正在加载 {0} 的详情…"),
    ("Installed ({0})", "已安装（{0}）"),
    ("{0} installed successfully.", "{0} 安装成功。"),
    (
        "Installed {0} of {1} {2} plugins.",
        "已安装 {0}/{1} 个{2}插件。",
    ),
    (
        "{0}   Space to {1}; Enter view details.",
        "{0}   Space可{1}；Enter查看详情。",
    ),
    ("{0}   Space to {1}.", "{0}   Space可{1}。"),
    (
        "{0}   Press Enter to view plugin details.",
        "{0}   Enter查看插件详情。",
    ),
    (
        "{0}   Plugin details are unavailable.",
        "{0}   插件详情不可用。",
    ),
    (
        "{0}   Press Enter to install or view plugin details.",
        "{0}   Enter安装或查看插件详情。",
    ),
    (
        "{0}   Remote plugin details are not available yet.",
        "{0}   远程插件详情尚不可用。",
    ),
    ("disable", "禁用"),
    ("enable", "启用"),
    // Event dispatch, second batch (`tui/src/app/event_dispatch.rs`): the
    // placeholder-free notices; the `format!` templates in the same file are
    // converted site by site in a later batch.
    ("No changes detected.", "未检测到改动。"),
    (
        "That Windows sandbox option is disallowed by requirements.",
        "该Windows沙箱选项被要求禁止。",
    ),
    ("Sandbox ready", "沙箱已就绪"),
    (
        "Codex can now safely edit files and execute commands in your computer",
        "Codex现在可以在你的电脑上安全地编辑文件并执行命令",
    ),
    ("Service tier cleared", "服务层级已清除"),
    ("To continue this session, run ", "要继续本次会话，请运行 "),
    // Event dispatch, third batch: the templates that needed the whole
    // `format!` call rewritten to positional placeholders. The internal config
    // keys in the same file ("default model and reasoning effort",
    // "default service tier") are deliberately not translated -- they are labels
    // for a persistence call, not UI.
    ("Failed to start turn: {0}", "启动回合失败：{0}"),
    ("Failed to save default model: {0}", "保存默认模型失败：{0}"),
    (
        "Model changed to {0} {1} for this conversation",
        "本次对话的模型已更改为 {0} {1}",
    ),
    (
        "Failed to prepare Windows sandbox for the selected permission profile: {0}",
        "为所选权限配置准备Windows沙箱失败：{0}",
    ),
    (
        "Granting sandbox read access to {0} ...",
        "正在授予 {0} 的沙箱读权限…",
    ),
    (
        "Sandbox read access granted for {0}",
        "已授予 {0} 的沙箱读权限",
    ),
    (
        "Failed to enable the Windows sandbox feature: {0}",
        "启用Windows沙箱功能失败：{0}",
    ),
    (
        "Cyber models default to \"Approve for me\" for safety reasons.",
        "出于安全考虑，网络安全类模型默认使用“替我审批”。",
    ),
    ("Personality set to {0}", "沟通风格已设置为 {0}"),
    (
        "Failed to save default personality: {0}",
        "保存默认沟通风格失败：{0}",
    ),
    ("Service tier set to {0}", "服务层级已设置为 {0}"),
    // Keymap picker (`tui/src/keymap_setup/picker.rs`). The per-context tab table
    // was a `const` and became a cached function (§3.6); its labels reuse the
    // glossary's terms (Vim stays Vim, Pager is 分页视图).
    ("App", "应用"),
    ("Vim", "Vim"),
    ("Navigation", "导航"),
    // Keymap editor (`tui/src/keymap_setup.rs`): the action menu, the key-capture
    // prompts and the conflict/validation errors. `tui.keymap` and `/keymap` are a
    // config path and a command, so they stay verbatim inside the translation.
    (" select · ", " 选择 · "),
    ("Custom root override", "自定义根级覆盖"),
    ("Default keymap", "默认键位表"),
    ("Edit Shortcut", "编辑快捷键"),
    ("Current ", "当前 "),
    (
        "Capture a key for this unbound action.",
        "为此未绑定操作捕获一个按键。",
    ),
    ("Replace binding", "替换绑定"),
    ("Add alternate binding", "添加备用绑定"),
    ("Replace one binding...", "替换其中一个绑定…"),
    (
        "Choose which existing binding to replace.",
        "选择要替换的现有绑定。",
    ),
    ("Replace all bindings", "替换全部绑定"),
    ("Set key chord", "设置组合键"),
    ("Replace with key chord", "用组合键替换"),
    ("Add alternate key chord", "添加备用组合键"),
    ("Remove custom binding", "移除自定义绑定"),
    (
        "Restore the default keymap binding.",
        "恢复默认键位表绑定。",
    ),
    ("Back to shortcuts", "返回快捷键列表"),
    ("Return to the shortcut list.", "返回快捷键列表。"),
    (" override.", " 覆盖。"),
    ("Choose the binding to replace.", "选择要替换的绑定。"),
    ("Shortcut Conflict", "快捷键冲突"),
    ("Pick another key", "另选一个按键"),
    (
        "Return to key capture for this action.",
        "返回为此操作捕获按键。",
    ),
    ("Leave keymap unchanged.", "保持键位表不变。"),
    (
        "Only ctrl, alt, and shift modifiers can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能存储ctrl、alt和shift修饰键。",
    ),
    (
        "Only printable ASCII keys can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能存储可打印ASCII按键。",
    ),
    (
        "That key is not supported by `tui.keymap`.",
        "`tui.keymap` 不支持该按键。",
    ),
    (
        "Capture a replacement key for `{0}`.",
        "为 `{0}` 捕获替换按键。",
    ),
    (
        "Keep `{0}` and add another key.",
        "保留 `{0}` 并添加另一个按键。",
    ),
    ("Replace `{0}` with one key.", "用一个按键替换 `{0}`。"),
    (
        "Replace `{0}` with a two-stroke key chord.",
        "用两段组合键替换 `{0}`。",
    ),
    (
        "Keep `{0}` and add a two-stroke key chord.",
        "保留 `{0}` 并添加两段组合键。",
    ),
    (
        "Replace `{0}` with another key.",
        "用另一个按键替换 `{0}`。",
    ),
    ("{0} (key chord)", "{0}（组合键）"),
    ("{0}.{1} cannot use `{2}`.", "{0}.{1} 不能使用 `{2}`。"),
    (
        "No change: `{0}.{1}` already uses `{2}`.",
        "未更改：`{0}.{1}` 已使用 `{2}`。",
    ),
    (
        "`{0}.{1}` no longer uses `{2}`. Reopen /keymap and choose a binding again.",
        "`{0}.{1}` 已不再使用 `{2}`。请重新打开 /keymap并再次选择绑定。",
    ),
    (
        "Remapped `{0}.{1}` to `{2}`.",
        "已将 `{0}.{1}` 重新映射为 `{2}`。",
    ),
    ("Added `{0}` to `{1}.{2}`.", "已为 `{1}.{2}` 添加 `{0}`。"),
    (
        "Replaced `{0}` with `{1}` for `{2}.{3}`.",
        "已为 `{2}.{3}` 将 `{0}` 替换为 `{1}`。",
    ),
    (
        "Unknown keymap action `{0}.{1}`. Reopen /keymap and choose an action.",
        "未知的键位表操作 `{0}.{1}`。请重新打开 /keymap并选择一个操作。",
    ),
    (
        "Only function keys F1 through F{0} can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能存储F1到F{0} 的功能键。",
    ),
    ("Global and chat-level shortcuts.", "全局与对话级快捷键。"),
    (
        "Composer submission and queue shortcuts.",
        "输入框提交与排队快捷键。",
    ),
    (
        "Inline editor movement and editing shortcuts.",
        "内嵌编辑器的移动与编辑快捷键。",
    ),
    (
        "Vim normal-mode and operator shortcuts.",
        "Vim普通模式与操作符快捷键。",
    ),
    (
        "Pager and selection-list navigation shortcuts.",
        "分页视图与选择列表的导航快捷键。",
    ),
    ("Shared agents dashboard shortcuts.", "共享代理面板快捷键。"),
    ("Approval prompt shortcuts.", "审批提示快捷键。"),
    ("All configurable shortcuts.", "全部可配置快捷键。"),
    ("No shortcuts available", "没有可用的快捷键"),
    (
        "No configurable shortcuts are available.",
        "没有可配置的快捷键。",
    ),
    ("Frequently customized shortcuts.", "常被自定义的快捷键。"),
    ("No common shortcuts", "没有常用快捷键"),
    (
        "No common shortcut actions are available.",
        "没有可用的常用快捷键操作。",
    ),
    ("Root-level shortcut overrides.", "根级快捷键覆盖。"),
    ("No customized shortcuts", "没有已自定义的快捷键"),
    (
        "{0} actions, {1} customized, {2} unbound.",
        "{0} 个操作，{1} 个已自定义，{2} 个未绑定。",
    ),
    ("Customized ({0})", "已自定义（{0}）"),
    // Tab labels that are single words the picker builds directly (not from the
    // context table): All / Common / Debug.
    ("All", "全部"),
    ("Common", "常用"),
    // `keymap_setup/actions.rs` 的调试视图来源标签（经 `debug.rs:148` 进 Line 渲染）。
    // 三条都短于扫描器的 MIN_CANDIDATE_LEN=8，属 §十二·附 的「短标签盲区」——
    // 只有 `Custom global`（13 字符）会出现在候选里，另两条只能靠人工核对发现。
    ("Custom", "自定义"),
    ("Default", "默认"),
    ("Custom global", "全局自定义"),
    ("Debug", "调试"),
    // Session picker (`tui/src/resume_picker.rs`). The `eyre!` error contexts in
    // that file ("invalid keymap configuration…", "failed to write config.toml…")
    // are diagnostics and stay English; these are the strings that render.
    ("Resume a previous session", "恢复之前的会话"),
    ("Fork a previous session", "分叉之前的会话"),
    (
        "No transcript available for this session",
        "此会话没有可用的记录",
    ),
    (
        "Failed to read session metadata from selected session",
        "从所选会话读取会话元数据失败",
    ),
    ("Could not load transcript preview", "无法加载记录预览"),
    ("(no message yet)", "（还没有消息）"),
    ("Type to search", "输入以搜索"),
    ("Filter: ", "筛选："),
    ("start new", "新建"),
    ("clear search", "清除搜索"),
    ("dense view", "紧凑视图"),
    ("comfortable view", "宽松视图"),
    ("focus sort/filter", "聚焦排序/筛选"),
    ("change option", "切换选项"),
    ("Loading transcript…", "正在加载记录…"),
    ("Loading older sessions…", "正在加载更早的会话…"),
    ("↓ loading more", "↓ 正在加载更多"),
    ("no branch", "无分支"),
    ("Loading recent transcript...", "正在加载最近的记录…"),
    ("No transcript preview available", "没有可用的记录预览"),
    ("Searching…", "正在搜索…"),
    ("No results for your search", "没有匹配的搜索结果"),
    ("Loading sessions…", "正在加载会话…"),
    ("No sessions yet", "还没有会话"),
    (
        "Failed to read session metadata from {0}",
        "从 {0} 读取会话元数据失败",
    ),
    ("Search: {0}", "搜索：{0}"),
    // App link view (`tui/src/bottom_pane/app_link_view.rs`). `$`, `/apps` and
    // ChatGPT keep their spelling; the action labels must match the button text
    // they refer to, so the two sets are translated together.
    (
        "Sign in to this app in your browser, then return here.",
        "请在浏览器中登录此应用，然后回到这里。",
    ),
    ("Action required", "需要操作"),
    ("Server: {0}", "服务器：{0}"),
    (
        "Complete the requested action in your browser, then return here.",
        "请在浏览器中完成所需操作，然后回到这里。",
    ),
    ("Open sign-in URL", "打开登录链接"),
    ("I already signed in", "我已登录"),
    ("Open link", "打开链接"),
    ("I finished", "我已完成"),
    ("Manage on ChatGPT", "在ChatGPT中管理"),
    ("Disable app", "禁用应用"),
    ("Enable app", "启用应用"),
    ("Install on ChatGPT", "在ChatGPT中安装"),
    ("I already Installed it", "我已安装"),
    (
        "Use $ to insert this app into the prompt.",
        "用 $ 将此应用插入提示。",
    ),
    (
        "Newly installed apps can take a few minutes to appear in /apps.",
        "新安装的应用可能需要几分钟才会出现在 /apps中。",
    ),
    (
        "After installed, use $ to insert this app into the prompt.",
        "安装后，用 $ 将此应用插入提示。",
    ),
    ("Finish App Sign In", "完成应用登录"),
    ("Finish Authentication", "完成认证"),
    ("Finish in Browser", "在浏览器中完成"),
    ("Finish App Setup", "完成应用设置"),
    (
        "Sign in to the app on ChatGPT in the browser window that just opened.",
        "在刚打开的浏览器窗口中登录ChatGPT上的该应用。",
    ),
    (
        "Complete authentication in the browser window that just opened.",
        "在刚打开的浏览器窗口中完成认证。",
    ),
    (
        "Then return here and select \"I already signed in\".",
        "然后回到这里并选择“我已登录”。",
    ),
    (
        "Complete the requested action in the browser window that just opened.",
        "在刚打开的浏览器窗口中完成所需操作。",
    ),
    (
        "Then return here and select \"I finished\".",
        "然后回到这里并选择“我已完成”。",
    ),
    (
        "Complete app setup on ChatGPT in the browser window that just opened.",
        "在刚打开的浏览器窗口中完成ChatGPT上的应用设置。",
    ),
    (
        "Sign in there if needed, then return here and select \"I already Installed it\".",
        "如需要请在那里登录，然后回到这里并选择“我已安装”。",
    ),
    ("Sign-in URL:", "登录链接："),
    ("Setup URL:", "设置链接："),
    (" to move", " 移动"),
    (" to select", " 选择"),
    (" to close", " 关闭"),
    ("No actions", "没有可用操作"),
    ("Side starting...", "侧会话启动中…"),
    (
        "Press Ctrl+C to return to the main thread first.",
        "请先按Ctrl+C返回主线程。",
    ),
    (
        "Example: /goal improve benchmark coverage",
        "示例：/goal improve benchmark coverage",
    ),
    ("Usage: /raw [on|off]", "用法：/raw [on|off]"),
    (
        "Sign in with ChatGPT to use /usage.",
        "请登录ChatGPT后使用/usage。",
    ),
    (
        "'/{0}' is unavailable in side conversations. {1}",
        "侧会话中不可用'/{0}'。{1}",
    ),
    (
        "'/{0}' is unavailable before the session starts.",
        "会话开始前不可用'/{0}'。",
    ),
    (
        "'/{0}' is disabled while a task is in progress.",
        "任务进行中已禁用'/{0}'。",
    ),
    (
        "'/{0}' is unavailable while code review is running.",
        "代码审查进行中不可用'/{0}'。",
    ),
    (
        "Are you sure? This will archive the current session and exit Codex",
        "确定吗？这将归档当前会话并退出Codex",
    ),
    (
        "Session is still starting; try /recap again in a moment.",
        "会话仍在启动中；请稍后重试/recap。",
    ),
    (
        "Internal error: missing the 'auto' approval preset.",
        "内部错误：缺少'auto'审批预设。",
    ),
    (
        "Usage: /sandbox-add-read-dir <absolute-directory-path>",
        "用法：/sandbox-add-read-dir <绝对目录路径>",
    ),
    (
        "`/diff` — _not inside a git repository_",
        "`/diff` — _不在git仓库内_",
    ),
    ("Failed to compute diff: {0}", "计算diff失败：{0}"),
    (
        "Failed to compute diff: workspace command runner unavailable",
        "计算diff失败：工作区命令运行器不可用",
    ),
    ("Current working directory: {0}", "当前工作目录：{0}"),
    ("Memory maintenance", "记忆维护"),
    ("Current rollout path: {0}", "当前rollout路径：{0}"),
    (
        "Rollout path is not available yet.",
        "rollout路径尚不可用。",
    ),
    ("Usage: /pwd", "用法：/pwd"),
    (
        "Usage: /usage [daily|weekly|cumulative]",
        "用法：/usage [daily|weekly|cumulative]",
    ),
    ("Usage: /mcp [verbose]", "用法：/mcp [verbose]"),
    ("Usage: /keymap [debug]", "用法：/keymap [debug]"),
    (
        "The session must start before you can change a goal.",
        "会话开始后才能修改目标。",
    ),
    (
        "The session must start before you can set a goal.",
        "会话开始后才能设置目标。",
    ),
    (
        "Unrecognized command '/{0}'. Type \"/\" for a list of supported commands.",
        "无法识别的命令'/{0}'。输入\"/\"查看支持的命令列表。",
    ),
    (
        "terminal clipboard copy failed over SSH: {0}",
        "SSH环境下终端剪贴板写入失败：{0}",
    ),
    (
        "OSC 52 clipboard copy failed over SSH: {0}",
        "SSH环境下OSC 52剪贴板写入失败：{0}",
    ),
    (
        "native clipboard: {0}; WSL fallback: {1}; terminal fallback: {2}",
        "原生剪贴板：{0}；WSL回退：{1}；终端回退：{2}",
    ),
    (
        "native clipboard: {0}; WSL fallback: {1}; OSC 52 fallback: {2}",
        "原生剪贴板：{0}；WSL回退：{1}；OSC 52回退：{2}",
    ),
    (
        "native clipboard: {0}; terminal fallback: {1}",
        "原生剪贴板：{0}；终端回退：{1}",
    ),
    (
        "native clipboard: {0}; OSC 52 fallback: {1}",
        "原生剪贴板：{0}；OSC 52回退：{1}",
    ),
    (
        "tmux clipboard: {0}; OSC 52 fallback: {1}",
        "tmux剪贴板：{0}；OSC 52回退：{1}",
    ),
    ("stderr suppression lock poisoned", "stderr抑制锁已中毒"),
    ("clipboard unavailable: {0}", "剪贴板不可用：{0}"),
    (
        "failed to set clipboard text: {0}",
        "设置剪贴板文本失败：{0}",
    ),
    (
        "native clipboard unavailable on Android",
        "Android上原生剪贴板不可用",
    ),
    (
        "failed to spawn powershell.exe: {0}",
        "启动powershell.exe失败：{0}",
    ),
    (
        "failed to open powershell.exe stdin",
        "打开powershell.exe标准输入失败",
    ),
    (
        "failed to write to powershell.exe: {0}",
        "写入powershell.exe失败：{0}",
    ),
    (
        "failed to wait for powershell.exe: {0}",
        "等待powershell.exe失败：{0}",
    ),
    (
        "powershell.exe exited with status {0}",
        "powershell.exe退出，状态码{0}",
    ),
    ("powershell.exe failed: {0}", "powershell.exe失败：{0}"),
    (
        "WSL clipboard fallback unavailable on this platform",
        "此平台不支持WSL剪贴板回退",
    ),
    ("failed to spawn tmux: {0}", "启动tmux失败：{0}"),
    ("failed to open tmux stdin", "打开tmux标准输入失败"),
    ("failed to write to tmux: {0}", "写入tmux失败：{0}"),
    ("failed to wait for tmux: {0}", "等待tmux失败：{0}"),
    ("tmux exited with status {0}", "tmux退出，状态码{0}"),
    ("tmux failed: {0}", "tmux失败：{0}"),
    (
        "tmux clipboard forwarding is disabled",
        "tmux剪贴板转发已关闭",
    ),
    (
        "tmux clipboard forwarding is unavailable: missing Ms capability",
        "tmux剪贴板转发不可用：缺少Ms能力",
    ),
    ("tmux output was not UTF-8: {0}", "tmux输出不是UTF-8：{0}"),
    ("failed to write OSC 52: {0}", "写入OSC 52失败：{0}"),
    ("failed to flush OSC 52: {0}", "刷新OSC 52失败：{0}"),
    (
        "OSC 52 payload too large ({0} bytes; max {1})",
        "OSC 52载荷过大（{0}字节；上限{1}）",
    ),
    ("• Feedback uploaded.", "• 反馈已上传。"),
    (
        "• Feedback recorded (no logs).",
        "• 反馈已记录（不含日志）。",
    ),
    (
        "{0} You can share this in #codex-feedback:",
        "{0} 可在#codex-feedback中分享：",
    ),
    (
        "{0} Please open an issue using the following URL:",
        "{0} 请通过以下链接提交issue：",
    ),
    ("{0} Thanks for the feedback!", "{0} 感谢你的反馈！"),
    ("  Sentry Feedback ID: ", "  Sentry反馈ID："),
    ("  Sentry URL: ", "  Sentry链接："),
    (
        "  Or mention your thread ID ",
        "  或在已有的issue中提及你的线程ID ",
    ),
    (" in an existing issue.", "，可附在已有issue中。"),
    ("  Thread ID: ", "  线程ID："),
    ("How was this?", "这次体验如何？"),
    ("bug", "缺陷"),
    (
        "Crash, error message, hang, or broken UI/behavior.",
        "崩溃、报错、卡死，或界面/行为异常。",
    ),
    ("bad result", "结果不佳"),
    (
        "Output was off-target, incorrect, incomplete, or unhelpful.",
        "输出偏题、错误、不完整或没有帮助。",
    ),
    ("good result", "结果良好"),
    (
        "Helpful, correct, high‑quality, or delightful result worth celebrating.",
        "有帮助、正确、高质量或令人惊喜的结果，值得庆祝。",
    ),
    ("safety check", "安全检查"),
    (
        "Benign usage blocked due to safety checks or refusals.",
        "正常使用被安全检查或拒答拦截。",
    ),
    ("other", "其他"),
    (
        "Slowness, feature suggestion, UX feedback, or anything else.",
        "速度慢、功能建议、交互反馈，或其他任何问题。",
    ),
    ("Sending feedback is disabled", "反馈发送已禁用"),
    (
        "This action is disabled by configuration.",
        "此操作已被配置禁用。",
    ),
    ("Close", "关闭"),
    ("Upload logs?", "上传日志？"),
    ("The following files will be sent:", "将发送以下文件："),
    ("Connectivity diagnostics", "连接诊断"),
    ("Yes", "是"),
    (
        "Share the current Codex session logs and diagnostics with the team for troubleshooting.",
        "将当前Codex会话日志与诊断信息分享给团队以便排查。",
    ),
    ("No", "否"),
    (
        "Failed to rebuild config for permission profile {0}",
        "为权限配置档{0}重建配置失败",
    ),
    ("Permission selection requested: {0}", "已请求权限选择：{0}"),
    (
        "Named profiles require a newer app server.",
        "命名配置档需要更新的app server。",
    ),
    ("Failed to select permissions: {0}", "选择权限失败：{0}"),
    (
        "Wait for permissions to update before changing permissions.",
        "请等待权限更新完成后再更改权限。",
    ),
    (
        "{0} were saved, but Codex could not refresh the effective config: {1}",
        "{0}已保存，但Codex无法刷新生效配置：{1}",
    ),
    (
        "Failed to carry forward approval policy override: {0}",
        "沿用审批策略覆盖失败：{0}",
    ),
    (
        "Failed to carry forward approvals reviewer: {0}",
        "沿用审批审查者失败：{0}",
    ),
    (
        "Failed to carry forward permission profile override: {0}",
        "沿用权限配置档覆盖失败：{0}",
    ),
    ("{0}: {1}", "{0}：{1}"),
    (
        "{0}: unsupported active permission profile `{1}`",
        "{0}：不支持的生效权限配置档`{1}`",
    ),
    (
        "Failed to update experimental feature `{0}`: {1}",
        "更新实验特性`{0}`失败：{1}",
    ),
    ("Failed to enable Approve for me", "启用「替我批准」失败"),
    (
        "Failed to update experimental features: {0}",
        "更新实验特性失败：{0}",
    ),
    (
        "Experimental feature changes were saved but not applied: {0}",
        "实验特性更改已保存但未生效：{0}",
    ),
    (
        "Failed to enable Approve for me: {0}",
        "启用「替我批准」失败：{0}",
    ),
    (
        "Failed to save memory settings: {0}",
        "保存记忆设置失败：{0}",
    ),
    (
        "Memory setting changes were saved but not applied: {0}",
        "记忆设置更改已保存但未生效：{0}",
    ),
    (
        "Saved memory settings, but failed to update the current thread: {0}",
        "记忆设置已保存，但更新当前线程失败：{0}",
    ),
    ("Failed to reset memories: {0}", "重置记忆失败：{0}"),
    ("Reset local memories.", "已重置本地记忆。"),
    ("default", "默认"),
    ("None", "无"),
    ("Friendly", "友好"),
    ("Pragmatic", "务实"),
    (
        "Failed to refresh overridden Approve for me settings: {0}",
        "刷新被覆盖的「替我批准」设置失败：{0}",
    ),
    (
        "Failed to refresh overridden Approve for me settings",
        "刷新被覆盖的「替我批准」设置",
    ),
    (
        "Windows sandbox changes were saved but not applied: {0}",
        "Windows沙箱更改已保存但未生效：{0}",
    ),
    ("Windows sandbox changes", "Windows沙箱更改"),
    (
        "the effective config is overridden by a higher-priority layer",
        "生效配置被更高优先级的层覆盖",
    ),
    ("Failed to set approval policy", "设置审批策略失败"),
    ("Failed to set permission profile", "设置权限配置档失败"),
    (
        "  Choose how you want to use Codex.",
        "  选择你想如何使用Codex。",
    ),
    (
        "  API key login is disabled by this workspace. Sign in with ChatGPT to continue.",
        "  此工作区已禁用API key登录。请使用ChatGPT登录以继续。",
    ),
    ("  Press ", "  按 "),
    (" to continue", " 继续"),
    (
        "  If the link doesn't open automatically, open the following link to authenticate:",
        "  如果链接没有自动打开，请打开以下链接完成认证：",
    ),
    (
        "  On a remote or headless machine? Press ",
        "  远程或无头环境？按 ",
    ),
    (" and choose ", " 并选择 "),
    (" to cancel", " 取消"),
    ("  For more details see the ", "  详情请见 "),
    (
        "  Uses your plan's rate limits and ",
        "  使用你套餐的速率限制和 ",
    ),
    ("training data preferences", "训练数据偏好"),
    (
        "✓ Signed in with your ChatGPT account",
        "✓ 已使用你的ChatGPT账户登录",
    ),
    ("  Before you start:", "  开始之前："),
    (
        "  Decide how much autonomy you want to grant Codex",
        "  决定给Codex多大的自主权",
    ),
    ("  Codex can make mistakes", "  Codex可能会出错"),
    (
        "  Review the code it writes and commands it runs",
        "  请审查它编写的代码与执行的命令",
    ),
    (
        "  Powered by your ChatGPT account",
        "  由你的ChatGPT账户提供支持",
    ),
    ("■ Copy failed: ", "■ 复制失败"),
    ("■ Export failed: ", "■ 导出失败："),
    ("⚠ Consumes usage limits faster", "⚠ 更快消耗用量限额"),
    ("✓ API key configured", "✓ API key已配置"),
    (
        "  Codex will use usage-based billing with your API key.",
        "  Codex将使用你的API key按用量计费。",
    ),
    (
        "Use your own OpenAI API key for usage-based billing",
        "使用你自己的OpenAI API key按用量计费",
    ),
    (
        "  Paste or type your API key below. It will be stored locally in auth.json.",
        "  在下方粘贴或输入你的API key。它将保存在本地的auth.json中。",
    ),
    (
        "  Detected OPENAI_API_KEY environment variable.",
        "  检测到OPENAI_API_KEY环境变量。",
    ),
    (
        "  Paste a different key if you prefer to use another account.",
        "  如果想使用其他账户，请粘贴另一个key。",
    ),
    ("Paste or type your API key", "粘贴或输入你的API key"),
    ("API key", "API密钥"),
    (" to save", " 保存"),
    ("API key cannot be empty", "API key不能为空"),
    (
        "Unexpected account/login/start response: {0}",
        "意外的account/login/start响应：{0}",
    ),
    ("Failed to save API key: {0}", "保存API key失败：{0}"),
    ("✓ Amazon Bedrock configured", "✓ 已配置Amazon Bedrock"),
    (".", "。"),
    ("Managed", "受管"),
    ("Trusted", "已信任"),
    ("New hook - review required", "新钩子 - 需要审查"),
    (
        "Modified since last trusted - review required",
        "自上次信任后已修改 - 需要审查",
    ),
    ("Before a tool executes", "工具执行之前"),
    ("When permission is requested", "请求权限时"),
    ("After a tool executes", "工具执行之后"),
    ("Before context compaction", "上下文压缩之前"),
    ("After context compaction", "上下文压缩之后"),
    ("When a new session starts", "新会话开始时"),
    ("Right before a session ends", "会话结束之前"),
    ("When the user submits a prompt", "用户提交提示词时"),
    ("When a subagent is created", "创建子代理时"),
    (
        "Right before a subagent ends its turn",
        "子代理结束本轮之前",
    ),
    ("Right before Codex ends its turn", "Codex结束本轮之前"),
    (
        "Right before an interrupted turn is aborted",
        "被中断的本轮终止之前",
    ),
    ("Hook {0}", "钩子{0}"),
    ("Plugin - {0}", "插件 - {0}"),
    ("Plugin", "插件"),
    ("Admin config", "管理员配置"),
    ("User config", "用户配置"),
    ("Project config", "项目配置"),
    ("Session flags", "会话标志"),
    ("Cloud-managed config", "云端管理的配置"),
    ("Unknown source", "未知来源"),
    ("Mode", "模式"),
    ("Async", "异步"),
    ("Sync", "同步"),
    (
        "This updates when OpenAI Curated plugins finish loading.",
        "OpenAI精选插件加载完成后会更新。",
    ),
    (
        "This updates when workspace plugins finish loading.",
        "工作区插件加载完成后会更新。",
    ),
    (
        "This updates when shared plugins finish loading.",
        "共享插件加载完成后会更新。",
    ),
    (
        "This updates when the marketplace list is ready.",
        "市场源列表就绪后会更新。",
    ),
    (
        "This updates when marketplace installation completes.",
        "市场源安装完成后会更新。",
    ),
    (
        "This removes the configured marketplace from Codex.",
        "这会从Codex中移除已配置的市场源。",
    ),
    (
        "Remove this marketplace from the available plugin list.",
        "从可用插件列表中移除此市场源。",
    ),
    (
        "This updates when marketplace removal completes.",
        "市场源移除完成后会更新。",
    ),
    (
        "This updates when marketplace upgrade completes.",
        "市场源升级完成后会更新。",
    ),
    (
        "This updates when plugin installation completes.",
        "插件安装完成后会更新。",
    ),
    (
        "Failed to add marketplace from the provided source.",
        "无法从提供的来源添加市场源。",
    ),
    ("Enter a marketplace source.", "输入市场源。"),
    ("Return to the plugin list.", "返回插件列表。"),
    ("Failed to remove marketplace.", "移除市场源失败。"),
    ("Failed to load plugin details.", "加载插件详情失败。"),
    ("Plugin detail unavailable", "插件详情不可用"),
    (
        "No plugins are available in the discovered marketplaces.",
        "已发现的市场源中没有可用插件。",
    ),
    (
        "Add a marketplace from a Git repo or local root.",
        "从Git仓库或本地根目录添加市场源。",
    ),
    (
        "Enter a source to make its plugins available in this menu.",
        "输入来源，使其插件在此菜单中可用。",
    ),
    (
        "Enter owner/repo, a Git URL, or a local marketplace path.",
        "输入owner/repo、Git URL或本地市场源路径。",
    ),
    (
        "This plugin is installed by your workspace admin.",
        "此插件由你的工作区管理员安装。",
    ),
    (
        "This remote plugin did not provide an uninstall identity.",
        "此远程插件未提供卸载标识。",
    ),
    (
        "This plugin is disabled by your workspace admin.",
        "此插件已被你的工作区管理员禁用。",
    ),
    (
        "This plugin is not installable from this marketplace.",
        "此插件无法从此市场源安装。",
    ),
    (
        "This plugin did not provide an install location.",
        "此插件未提供安装位置。",
    ),
    ("Skills", "技能"),
    (
        "ctrl + u upgrade · ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + u升级 · ctrl + r移除 · space切换 · ←/→标签页 · enter详情 · esc关闭",
    ),
    (
        "ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + r移除 · space切换 · ←/→标签页 · enter详情 · esc关闭",
    ),
    (
        "ctrl + u upgrade · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + u升级 · space切换 · ←/→标签页 · enter详情 · esc关闭",
    ),
    (
        "space enable/disable · ←/→ select marketplace · enter view details · esc close",
        "space启用/禁用 · ←/→选择市场源 · enter查看详情 · esc关闭",
    ),
    ("Press esc to close.", "按esc关闭。"),
    ("Disabled by admin", "已被管理员禁用"),
    ("Enabled by Admin", "已由管理员启用"),
    ("Disabled", "已禁用"),
    ("Not installable", "不可安装"),
    ("Can be installed", "可安装"),
    ("Version", "版本"),
    ("Sharing", "共享"),
    ("Auth on install", "安装时认证"),
    ("Auth on use", "使用时认证"),
    ("Listed", "已列出"),
    ("Workspace link", "工作区链接"),
    ("Private", "私有"),
    ("No explicit principals", "无显式主体"),
    (
        "Local plugin functionality is already available.",
        "本地插件功能已可用。",
    ),
    ("This section loaded successfully.", "此分区已成功加载。"),
    ("Admin assigned", "管理员指定"),
    ("Available", "可用"),
    ("No plugin skills.", "没有插件技能。"),
    ("No plugin apps.", "没有插件应用。"),
    ("No plugin hooks.", "没有插件钩子。"),
    ("No plugin MCP servers.", "没有插件MCP服务器。"),
    ("D I F F", "差异"),
    ("P A T C H", "补丁"),
    ("E X E C", "执行"),
    ("P E R M I S S I O N S", "权限"),
    ("E L I C I T A T I O N", "询问"),
    (
        "Background server started. Run `codex agents` in another terminal; this session remains unchanged.",
        "后台服务已启动。请在另一个终端运行`codex agents`；本会话保持不变。",
    ),
    (
        "Wait for the current task to finish before running /recap.",
        "请等待当前任务完成后再运行/recap。",
    ),
    (
        "A thread must start before it can be archived.",
        "线程启动后才能归档。",
    ),
    (
        "A thread must start before it can be deleted.",
        "线程启动后才能删除。",
    ),
    (
        "Failed to save default service tier: {0}",
        "保存默认服务层级失败：{0}",
    ),
    (
        "Failed to save Agent mode warning preference: {0}",
        "保存Agent模式警告偏好失败：{0}",
    ),
    (
        "Failed to save rate limit reminder preference: {0}",
        "保存速率限制提醒偏好失败：{0}",
    ),
    (
        "Failed to save Plan mode reasoning effort: {0}",
        "保存Plan模式推理强度失败：{0}",
    ),
    (
        "Failed to save model migration prompt preference: {0}",
        "保存模型迁移提示偏好失败：{0}",
    ),
    (
        "Failed to update skill config for {0}: {1}",
        "更新技能配置失败（{0}）：{1}",
    ),
    (
        "Failed to save status line settings: {0}",
        "保存状态栏设置失败：{0}",
    ),
    (
        "Failed to save terminal title items: {0}",
        "保存终端标题项失败：{0}",
    ),
    (
        "Open this project in VS Code or Cursor with the Codex extension active.",
        "请在已启用Codex扩展的VS Code或Cursor中打开此项目。",
    ),
    (
        "The IDE extension did not provide context.",
        "IDE扩展未提供上下文。",
    ),
    (
        "Codex will keep trying on future messages.",
        "Codex会在后续消息中继续尝试。",
    ),
    ("{0} Try /ide again.", "{0} 请重试/ide。"),
    (
        "The selected IDE context is too large. Clear any large selection in your IDE and try /ide again.",
        "所选IDE上下文过大。请在IDE中清除较大的选区后重试/ide。",
    ),
    (
        "Codex could not request IDE context. Try /ide again.",
        "Codex无法请求IDE上下文。请重试/ide。",
    ),
    (
        "Codex could not read IDE context. Try /ide again.",
        "Codex无法读取IDE上下文。请重试/ide。",
    ),
    (
        "The selected IDE context is too large. Clear any large selection in your IDE.",
        "所选IDE上下文过大。请在IDE中清除较大的选区。",
    ),
    (
        "Codex timed out waiting for IDE context. It will keep trying on future messages.",
        "Codex等待IDE上下文超时。它会在后续消息中继续尝试。",
    ),
    (
        "The IDE connection changed while Codex was requesting context.",
        "在Codex请求上下文期间IDE连接发生了变化。",
    ),
    (
        "The IDE extension did not answer in time.",
        "IDE扩展未及时响应。",
    ),
    (
        "The connected IDE extension is not compatible with this IDE context request.",
        "所连接的IDE扩展与此IDE上下文请求不兼容。",
    ),
    (
        "The connected IDE client does not support IDE context requests.",
        "所连接的IDE客户端不支持IDE上下文请求。",
    ),
    (
        "Codex lost the IDE connection while requesting context.",
        "Codex在请求上下文期间丢失了IDE连接。",
    ),
    (
        "Codex received an unexpected IDE context response.",
        "Codex收到了意外的IDE上下文响应。",
    ),
    (
        "Codex could not read IDE context.",
        "Codex无法读取IDE上下文。",
    ),
    ("default model and reasoning effort", "默认模型与推理强度"),
    ("default service tier", "默认服务层级"),
    ("Plan mode reasoning effort", "Plan模式推理强度"),
    (
        "Saved {0}, but a higher-priority configuration layer overrides the saved value.",
        "已保存{0}，但更高优先级的配置层覆盖了保存的值。",
    ),
    (
        "Read the API key from stdin (e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`)",
        "从标准输入读取API key（例如 `printenv OPENAI_API_KEY | codex login --with-api-key`）",
    ),
    (
        "Read the access token from stdin (e.g. `printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`)",
        "从标准输入读取访问令牌（例如 `printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`）",
    ),
    (
        "(deprecated) Previously accepted the API key directly; now exits with guidance to use --with-api-key",
        "（已弃用）过去可直接接受API key；现在会退出并提示使用--with-api-key",
    ),
    (
        "--aws-sigv4 requires --remote-transport direct",
        "--aws-sigv4需要--remote-transport direct",
    ),
    (
        "--remote-transport direct requires --aws-sigv4",
        "--remote-transport direct需要--aws-sigv4",
    ),
    (
        "direct exec-server transport does not support forwarding",
        "直接exec-server传输不支持转发",
    ),
    ("PATH is not set", "未设置PATH"),
    (
        "--force requires a session UUID; names must be confirmed interactively",
        "--force需要会话UUID；名称必须在交互模式下确认",
    ),
    (
        "`codex agents` received conflicting remote server endpoints",
        "`codex agents`收到了冲突的远程服务端地址",
    ),
    (
        "`codex agents` does not accept an initial prompt or images",
        "`codex agents`不接受初始提示词或图片",
    ),
    (
        "`codex agents` is unavailable while workload identity is active",
        "工作负载身份启用时`codex agents`不可用",
    ),
    (
        "`codex agents` requires `--remote` on this platform",
        "此平台上`codex agents`需要`--remote`",
    ),
    (
        "`codex sandbox` is not supported on this operating system",
        "此操作系统不支持`codex sandbox`",
    ),
    (
        "Choose one login credential source: --with-api-key or --with-access-token.",
        "请选择一种登录凭据来源：--with-api-key或--with-access-token。",
    ),
    (
        "Codex executable path is not configured",
        "未配置Codex可执行文件路径",
    ),
    (
        "--environment-id is required when --remote is set",
        "设置--remote时必须提供--environment-id",
    ),
    (
        "CODEX_ACCESS_TOKEN is required when --use-agent-identity-auth is set",
        "设置--use-agent-identity-auth时必须提供CODEX_ACCESS_TOKEN",
    ),
    (
        "Agent Identity authentication is unavailable",
        "Agent Identity认证不可用",
    ),
    (
        "CODEX_ACCESS_TOKEN did not provide permitted Agent Identity authentication",
        "CODEX_ACCESS_TOKEN未通过Agent Identity认证",
    ),
    (
        "remote exec-server registration URL must include a host",
        "远程exec-server注册URL必须包含主机名",
    ),
    (
        "`codex fork --worktree` requires an explicit session ID",
        "`codex fork --worktree`需要显式的会话ID",
    ),
    (
        "`--worktree` cannot resume an existing session; use `codex exec fork --worktree`",
        "`--worktree`无法恢复已有会话；请使用`codex exec fork --worktree`",
    ),
    (
        "`--worktree` is not supported for code review",
        "代码审查不支持`--worktree`",
    ),
    ("Continue anyway? [y/N]: ", "仍要继续？[y/N]："),
    ("stdin is not a terminal", "stdin不是终端"),
    ("stdout is not a terminal", "stdout不是终端"),
    (
        "`--remote-auth-token-env` requires `--remote`.",
        "`--remote-auth-token-env`需要`--remote`。",
    ),
    (
        "`--remote-auth-token-env` requires a `wss://` or loopback `ws://` remote.",
        "`--remote-auth-token-env`需要`wss://`或回环`ws://`远程地址。",
    ),
    (
        "failed to resolve socket path `{0}`: {1}",
        "解析socket路径`{0}`失败：{1}",
    ),
    ("ERROR: {0}", "错误：{0}"),
    ("Updating Codex via `{0}`...", "正在通过`{0}`更新Codex…"),
    ("`{0}` failed with status {1}", "`{0}`失败，状态码{1}"),
    (
        "Could not find an absolute update command `{0}` on PATH. Please update manually: https://developers.openai.com/codex/cli/",
        "在PATH中找不到绝对路径的更新命令`{0}`。请手动更新：https://developers.openai.com/codex/cli/",
    ),
    (
        "could not find update command `{0}` on PATH",
        "在PATH中找不到更新命令`{0}`",
    ),
    (
        "Could not detect the Codex installation method. Please update manually: https://developers.openai.com/codex/cli/",
        "无法识别Codex的安装方式。请手动更新：https://developers.openai.com/codex/cli/",
    ),
    ("Unknown feature flag: {0}", "未知的特性开关：{0}"),
    (
        "invalid remote exec-server registration URL: {0}",
        "无效的远程exec-server注册URL：{0}",
    ),
    (
        "Enabled feature `{0}` in config.toml.",
        "已在config.toml中启用特性`{0}`。",
    ),
    (
        "Disabled feature `{0}` in config.toml.",
        "已在config.toml中禁用特性`{0}`。",
    ),
    ("Cleared memory state from {0}.", "已清除{0}的记忆状态。"),
    ("No memories db found at {0}.", "在{0}未找到记忆数据库。"),
    (
        "`--strict-config` is not supported for `codex {0}`",
        "`codex {0}`不支持`--strict-config`",
    ),
    (
        "warning: failed to load updater network configuration: {0}",
        "警告：加载更新器网络配置失败：{0}",
    ),
    (
        "environment variable `{0}` is not set",
        "环境变量`{0}`未设置",
    ),
    ("environment variable `{0}` is empty", "环境变量`{0}`为空"),
    (
        "\n🎉 Update ran successfully! Please restart Codex.",
        "\n🎉 更新成功！请重启Codex。",
    ),
    (
        "--worktree cannot be combined with --ignore-user-config",
        "--worktree不能与--ignore-user-config同时使用",
    ),
    (
        "--worktree cannot be combined with --ephemeral",
        "--worktree不能与--ephemeral同时使用",
    ),
    (
        "--worktree is not supported with `codex exec resume`",
        "`codex exec resume`不支持--worktree",
    ),
    (
        "--worktree is not supported with `codex exec review`",
        "`codex exec review`不支持--worktree",
    ),
    (
        "--worktree requires local execution",
        "--worktree需要本地执行",
    ),
    (
        "--worktree requires the worktrees feature; enable it with --enable worktrees",
        "--worktree需要worktrees特性；请用--enable worktrees启用",
    ),
    (
        "--worktree requires a source that is not explicitly untrusted",
        "--worktree要求来源未被明确标记为不受信任",
    ),
    (
        "Could not create otel exporter: panicked during initialization",
        "无法创建otel导出器：初始化时发生panic",
    ),
    (
        "Forking with images requires a prompt",
        "带图片分叉需要提供提示词",
    ),
    (
        "Forking with output options requires a prompt",
        "带输出选项分叉需要提供提示词",
    ),
    ("Ephemeral forks require a prompt", "临时分叉需要提供提示词"),
    (
        "Not inside a trusted directory and --skip-git-repo-check was not specified.",
        "不在受信任目录内，且未指定--skip-git-repo-check。",
    ),
    ("Reading prompt from stdin...", "正在从标准输入读取提示词…"),
    (
        "Reading additional input from stdin...",
        "正在从标准输入读取附加输入…",
    ),
    (
        "No prompt provided via stdin.",
        "未通过标准输入提供提示词。",
    ),
    ("Review prompt cannot be empty", "审查提示词不能为空"),
    (
        "Specify --uncommitted, --base, --commit, or provide custom review instructions",
        "请指定--uncommitted、--base、--commit，或提供自定义审查说明",
    ),
    (
        "--worktree requires a source that is not explicitly untrusted; unused checkout at {0} remains. Remove it manually with `git worktree remove` when safe",
        "--worktree要求来源未被明确标记为不受信任；{0}处仍留有未使用的检出。请在安全时用`git worktree remove`手动移除",
    ),
    (
        "No default OSS provider configured. Use --local-provider=provider or set oss_provider to one of: {0}, {1} in config.toml",
        "未配置默认OSS提供方。请使用--local-provider=provider，或在config.toml中把oss_provider设为{0}、{1}之一",
    ),
    (
        "Error loading rules:
{0}",
        "加载规则失败：
{0}",
    ),
    (
        "Error loading config.toml:
{0}",
        "加载config.toml失败：
{0}",
    ),
    (
        "Failed to read output schema file {0}: {1}",
        "读取输出schema文件{0}失败：{1}",
    ),
    (
        "Output schema file {0} is not valid JSON: {1}",
        "输出schema文件{0}不是合法JSON：{1}",
    ),
    ("Error parsing -c overrides: {0}", "解析-c覆盖项失败：{0}"),
    ("Error finding codex home: {0}", "查找codex主目录失败：{0}"),
    (
        "Could not create otel exporter: {0}",
        "无法创建otel导出器：{0}",
    ),
    ("Error loading config.toml: {0}", "加载config.toml失败：{0}"),
    ("OSS setup failed: {0}", "OSS设置失败：{0}"),
    (
        "failed to initialize in-process app-server client: {0}",
        "初始化进程内app-server客户端失败：{0}",
    ),
    ("Session not found: {0}", "未找到会话：{0}"),
    (
        "in-process app-server shutdown failed: {0}",
        "进程内app-server关闭失败：{0}",
    ),
    (
        "input is not valid UTF-8 (invalid byte at offset {0}). Convert it to UTF-8 and retry (e.g., `iconv -f <ENC> -t UTF-8 prompt.txt`).",
        "输入不是合法的UTF-8（偏移{0}处的字节无效）。请转换为UTF-8后重试（例如`iconv -f <ENC> -t UTF-8 prompt.txt`）。",
    ),
    (
        "input looked like {0} but could not be decoded. Convert it to UTF-8 and retry.",
        "输入看起来是{0}但无法解码。请转换为UTF-8后重试。",
    ),
    (
        "input appears to be {0}. Convert it to UTF-8 and retry.",
        "输入似乎是{0}。请转换为UTF-8后重试。",
    ),
    (
        "No prompt provided. Either specify one as an argument or pipe the prompt into stdin.",
        "未提供提示词。请将其作为参数传入，或通过标准输入管道输入。",
    ),
    (
        "Failed to read prompt from stdin: {0}",
        "从标准输入读取提示词失败：{0}",
    ),
    (
        "features.token_budget.reminder_threshold_tokens must be positive",
        "features.token_budget.reminder_threshold_tokens必须为正数",
    ),
    (
        "features.token_budget.reminder_message_template must not be empty",
        "features.token_budget.reminder_message_template不能为空",
    ),
    (
        "features.token_budget.auto_compact_fallback_buffer_tokens is required when auto_compact_fallback_prompt is set",
        "设置auto_compact_fallback_prompt时必须提供features.token_budget.auto_compact_fallback_buffer_tokens",
    ),
    (
        "features.token_budget.auto_compact_fallback_buffer_tokens must be positive",
        "features.token_budget.auto_compact_fallback_buffer_tokens必须为正数",
    ),
    (
        "projects table missing after initialization",
        "初始化后缺少projects表",
    ),
    (
        "features.rollout_budget.limit_tokens is required when rollout_budget is enabled",
        "启用rollout_budget时必须提供features.rollout_budget.limit_tokens",
    ),
    (
        "features.rollout_budget.limit_tokens must be positive",
        "features.rollout_budget.limit_tokens必须为正数",
    ),
    (
        "features.rollout_budget.reminder_at_remaining_tokens is required when rollout_budget is enabled",
        "启用rollout_budget时必须提供features.rollout_budget.reminder_at_remaining_tokens",
    ),
    (
        "features.rollout_budget.reminder_at_remaining_tokens must contain only positive values below limit_tokens",
        "features.rollout_budget.reminder_at_remaining_tokens只能包含小于limit_tokens的正数值",
    ),
    (
        "`experimental_thread_store_endpoint` is no longer supported; remove it from config.toml",
        "不再支持`experimental_thread_store_endpoint`；请从config.toml中移除",
    ),
    (
        "`--dangerously-bypass-hook-trust` is enabled. Enabled hooks may run without review for this invocation.",
        "已启用`--dangerously-bypass-hook-trust`。本次调用中已启用的钩子可能不经审查就运行。",
    ),
    (
        "`sandbox_mode` and `permission_profile` overrides cannot both be set",
        "`sandbox_mode`与`permission_profile`覆盖不能同时设置",
    ),
    (
        "`sandbox_mode` and `default_permissions` overrides cannot both be set",
        "`sandbox_mode`与`default_permissions`覆盖不能同时设置",
    ),
    (
        "`permission_profile` and `default_permissions` overrides cannot both be set",
        "`permission_profile`与`default_permissions`覆盖不能同时设置",
    ),
    (
        "config defines `[permissions]` profiles but does not set `default_permissions`",
        "配置定义了`[permissions]`配置档，但未设置`default_permissions`",
    ),
    (
        "features.multi_agent_v2.max_concurrent_threads_per_session must be at least 1",
        "features.multi_agent_v2.max_concurrent_threads_per_session至少为1",
    ),
    (
        "features.multi_agent_v2.min_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms",
        "features.multi_agent_v2.min_wait_timeout_ms不能大于features.multi_agent_v2.max_wait_timeout_ms",
    ),
    (
        "features.multi_agent_v2.default_wait_timeout_ms must be at least features.multi_agent_v2.min_wait_timeout_ms",
        "features.multi_agent_v2.default_wait_timeout_ms不能小于features.multi_agent_v2.min_wait_timeout_ms",
    ),
    (
        "features.multi_agent_v2.default_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms",
        "features.multi_agent_v2.default_wait_timeout_ms不能大于features.multi_agent_v2.max_wait_timeout_ms",
    ),
    (
        "agents.max_concurrent_threads_per_session must be at least 1",
        "agents.max_concurrent_threads_per_session至少为1",
    ),
    (
        "thread_unload_delay_secs is too large",
        "thread_unload_delay_secs过大",
    ),
    (
        "`approval_policy = \"never\"` cannot be used because requirements do not allow `sandbox_mode = \"danger-full-access\"`; Codex would fall back to read-only permissions with approvals disabled. Choose an `approval_policy` based on what you need, such as `on-request`, or choose an allowed sandbox mode.",
        "`approval_policy = \"never\"`不可用，因为requirements不允许`sandbox_mode = \"danger-full-access\"`；Codex将退化为禁用审批的只读权限。请根据需求选择`approval_policy`（例如`on-request`），或选择允许的沙箱模式。",
    ),
    (
        "goals.max_goal_token_budget exceeds the maximum supported token budget",
        "goals.max_goal_token_budget超过支持的最大token预算",
    ),
    (
        "requirements.toml default_permissions requires allowed_permission_profiles",
        "requirements.toml在设置default_permissions时需要allowed_permission_profiles",
    ),
    (
        "requirements.toml default_permissions must be set unless allowed_permission_profiles allows both `:workspace` and `:read-only`",
        "requirements.toml的default_permissions必须设置，除非allowed_permission_profiles同时允许`:workspace`和`:read-only`",
    ),
    ("{0} (non-admin sandbox)", "{0}（非管理员沙箱）"),
    (
        "The non-admin sandbox protects your files and prevents network access under most circumstances. However, it carries greater risk if prompt injected. To upgrade to the default sandbox, run ",
        "非管理员沙箱在多数情况下会保护你的文件并阻止网络访问。但若遭遇提示词注入，风险更高。要升级到默认沙箱，请运行 ",
    ),
    (
        "No recent auto-review denials in this thread.",
        "此线程中没有最近的自动审查拒绝记录。",
    ),
    (
        "Denials are recorded after auto-review rejects an action.",
        "自动审查拒绝某个操作后会记录拒绝项。",
    ),
    ("That thread is no longer available.", "该线程已不可用。"),
    ("Auto-review Denials", "自动审查拒绝记录"),
    (
        "Select a denied action to approve.",
        "选择一条被拒绝的操作进行批准。",
    ),
    (
        "That auto-review denial is no longer available.",
        "该自动审查拒绝记录已不可用。",
    ),
    (
        "Approval recorded for one retry of the selected auto-review denial.",
        "已记录批准，所选自动审查拒绝项可重试一次。",
    ),
    (
        "The model will see the approval context; the retry still goes through auto-review.",
        "模型会看到批准上下文；重试仍会经过自动审查。",
    ),
    ("Enable full access?", "启用完全访问？"),
    (
        "We strongly recommend selecting \"Approve for me\" instead, and customizing the reviewer policy for your use case.",
        "我们强烈建议改为选择「替我批准」，并按你的使用场景自定义审查策略。",
    ),
    (
        "We strongly recommend selecting \"Ask for approval\" instead.",
        "我们强烈建议改为选择「请求批准」。",
    ),
    (
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval.",
        "Codex以完全访问运行时，可以不经你批准就编辑电脑上的任何文件并执行带网络的命令。",
    ),
    (
        "Cyber models carry a higher risk of dangerous actions.",
        "网络安全模型带来危险操作的风险更高。",
    ),
    (
        " Ensure proper safeguards are in place before granting full access. ",
        " 在授予完全访问前，请确保已设置适当的防护措施。 ",
    ),
    (
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval. ",
        "Codex以完全访问运行时，可以不经你批准就编辑电脑上的任何文件并执行带网络的命令。 ",
    ),
    (
        "Exercise caution when enabling full access. This significantly increases the risk of data loss, leaks, or unexpected behavior.",
        "启用完全访问时请谨慎。这会显著增加数据丢失、泄露或意外行为的风险。",
    ),
    ("Yes, continue anyway", "是，仍然继续"),
    (
        "Apply full access for this session",
        "为本次会话应用完全访问",
    ),
    (
        "Go back without enabling full access",
        "返回且不启用完全访问",
    ),
    ("Shared agents unavailable", "共享代理不可用"),
    (
        "The agents dashboard is unavailable while workload identity is active.",
        "工作负载身份启用时，代理面板不可用。",
    ),
    (
        "This session isn’t connected to a shared background server.",
        "本会话未连接到共享后台服务。",
    ),
    (
        "Connect to a remote background server to use the agents dashboard.",
        "请连接到远程后台服务以使用代理面板。",
    ),
    (
        "Starting a background server will not interrupt or move this session.",
        "启动后台服务不会中断或迁移本会话。",
    ),
    ("Start background server", "启动后台服务"),
    (
        "Open `codex agents` in another terminal afterward.",
        "之后在另一个终端中运行`codex agents`。",
    ),
    ("Return to this session", "返回本会话"),
    (
        "Cannot resume task without preserving the selected permissions.",
        "不能在未保留所选权限的情况下恢复任务。",
    ),
    (
        "Permission profile has different settings.",
        "权限配置档的设置不同。",
    ),
    ("Failed to load shared agents: {0}", "加载共享代理失败：{0}"),
    ("Failed to load task settings: {0}", "加载任务设置失败：{0}"),
    ("Failed to attach to task: {0}", "附加到任务失败：{0}"),
    (
        "Failed to load project settings: {0}",
        "加载项目设置失败：{0}",
    ),
    (
        "Failed to start background task: {0}",
        "启动后台任务失败：{0}",
    ),
    ("Failed to send task message: {0}", "发送任务消息失败：{0}"),
    (
        "Failed to stop background task: {0}",
        "停止后台任务失败：{0}",
    ),
    ("daemon process exited with {0}", "守护进程退出，状态{0}"),
    ("Bundled apps remain installed.", "捆绑应用仍保持安装。"),
    ("Already installed in this session.", "本次会话中已安装。"),
    (
        "Install the required Apps in ChatGPT to continue:",
        "请先在ChatGPT中安装所需应用以继续：",
    ),
    (
        "Open the ChatGPT app management page",
        "打开ChatGPT应用管理页",
    ),
    (
        "Open the app page in your browser.",
        "在浏览器中打开该应用页面。",
    ),
    ("ChatGPT apps link unavailable", "ChatGPT应用链接不可用"),
    (
        "This app did not provide an install/manage URL.",
        "该应用未提供安装/管理链接。",
    ),
    ("This app is already installed.", "该应用已安装。"),
    ("Advance to the next app.", "继续到下一个应用。"),
    (
        "Trust your confirmation and continue to the next app.",
        "信任你的确认并继续到下一个应用。",
    ),
    (
        "Continue without waiting for refresh to complete.",
        "不再等待刷新完成，直接继续。",
    ),
    ("Skip remaining app setup", "跳过剩余应用设置"),
    (
        "Stop this follow-up flow for this plugin.",
        "停止此插件的后续流程。",
    ),
    (
        "Abandon remaining required app setup.",
        "放弃剩余必需的应用设置。",
    ),
    (
        "The plugin may not be usable until required apps are installed.",
        "在安装所需应用前，该插件可能无法使用。",
    ),
    (
        "You can now continue managing plugins from /plugins.",
        "现在可以继续通过/plugins管理插件。",
    ),
    ("Uninstalled {0} plugin.", "已卸载插件{0}。"),
    ("{0} plugin installed.", "已安装插件{0}。"),
    ("App setup {0}/{1}: {2}", "应用设置{0}/{1}：{2}"),
    (
        "Failed to update plugin config for {0}: {1}",
        "更新插件配置失败（{0}）：{1}",
    ),
    (
        "Skipped remaining app setup for {0} plugin.",
        "已跳过插件{0}的剩余应用设置。",
    ),
    (
        "Completed app setup flow for {0} plugin.",
        "已完成插件{0}的应用设置流程。",
    ),
    (
        "  Checking for existing AWS credentials...",
        "  正在检查现有的AWS凭据…",
    ),
    (
        "  Setting up Amazon Bedrock...",
        "  正在设置Amazon Bedrock…",
    ),
    ("  Choose an AWS profile.", "  请选择AWS配置档。"),
    (
        "  AWS credentials detected in your environment.",
        "  在你的环境中检测到AWS凭据。",
    ),
    ("  No AWS credentials found.", "  未找到AWS凭据。"),
    (
        "  Choose how you authenticate with AWS.",
        "  请选择你与AWS的认证方式。",
    ),
    (
        "  Enter the name of your AWS profile.",
        "  请输入AWS配置档名称。",
    ),
    ("  AWS profile: ", "  AWS配置档："),
    (
        "  Enter your Amazon Bedrock API key.",
        "  请输入你的Amazon Bedrock API key。",
    ),
    ("  Bedrock API key: ", "  Bedrock API key："),
    (
        "  Enter the AWS Region to use with Amazon Bedrock.",
        "  请输入与Amazon Bedrock一起使用的AWS区域。",
    ),
    ("  AWS Region: ", "  AWS区域："),
    ("  Enter your AWS access keys.", "  请输入你的AWS访问密钥。"),
    (
        "  Configure AWS credentials in your environment, then restart Codex.",
        "  请在你的环境中配置AWS凭据，然后重启Codex。",
    ),
    ("  Setup guide: ", "  设置指南："),
    ("Enter a Bedrock API key", "输入Bedrock API key"),
    ("  AWS profile detected: {0}", "  检测到AWS配置档：{0}"),
    ("  Region: {0}", "  区域：{0}"),
    ("shift+tab to cycle", "shift+tab切换"),
    ("Plan mode{0}", "Plan模式{0}"),
    ("Pursuing goal ({0})", "正在推进目标（{0}）"),
    ("Pursuing goal", "正在推进目标"),
    ("Goal paused (/goal resume)", "目标已暂停（/goal resume）"),
    ("Goal stalled (/goal resume)", "目标已停滞（/goal resume）"),
    (
        "Goal hit usage limits (/goal resume)",
        "目标已达用量上限（/goal resume）",
    ),
    ("Goal unmet ({0})", "目标未满足（{0}）"),
    ("Goal abandoned", "目标已放弃"),
    ("Goal achieved ({0})", "目标已达成（{0}）"),
    ("Goal achieved", "目标已达成"),
    ("reverse-i-search: ", "反向增量搜索："),
    ("{0} for side", "{0} 进入侧边会话"),
    ("from main thread", "来自主线程"),
    ("from parent thread ({0})", "来自父线程（{0}）"),
    ("{0} to switch", "{0} 切换"),
    ("ctrl + c to close", "ctrl + c关闭"),
    ("Side {0}", "侧边会话 {0}"),
    (
        "Wait for permissions to update before forking.",
        "请等待权限更新完成后再分叉。",
    ),
    (
        "Failed to close side conversation {0}; it is still open: {1}",
        "关闭侧边会话{0}失败；它仍处于打开状态：{1}",
    ),
    (
        "Failed to start side conversation: {0}",
        "启动侧边会话失败：{0}",
    ),
    (
        "Failed to prepare side conversation {0}: {1}",
        "准备侧边会话{0}失败：{1}",
    ),
    (
        "Failed to switch into side conversation {0}: {1}",
        "切换到侧边会话{0}失败：{1}",
    ),
    (
        "Failed to switch into side conversation {0}.",
        "切换到侧边会话{0}失败。",
    ),
    ("missing spritesheet {0}", "缺少精灵图{0}"),
    ("CODEX_HOME is not available", "CODEX_HOME不可用"),
    ("unknown pet {0}", "未知的宠物{0}"),
    ("pet path {0}", "宠物路径{0}"),
    (
        "pet json path has no containing directory",
        "宠物json路径没有所在目录",
    ),
    ("resolve {0}", "解析路径{0}"),
    (
        "missing pet.json or avatar.json in {0}",
        "在{0}中缺少pet.json或avatar.json",
    ),
    ("parse {0}", "解析{0}"),
    (
        "spritesheet path must stay inside {0}",
        "精灵图路径必须位于{0}内",
    ),
    (
        "pet frame dimensions and grid counts must be non-zero",
        "宠物帧尺寸与网格数量必须为非零",
    ),
    ("pet frame grid width overflow", "宠物帧网格宽度溢出"),
    ("pet frame grid height overflow", "宠物帧网格高度溢出"),
    ("pet frame count overflow", "宠物帧数量溢出"),
    (
        "pet frame count does not fit usize",
        "宠物帧数量超出usize范围",
    ),
    (
        "pet frame count {0} exceeds maximum {1}",
        "宠物帧数量{0}超过上限{1}",
    ),
    ("HOME is not set", "未设置HOME"),
    (
        "animation {0} must include at least one frame",
        "动画{0}必须至少包含一帧",
    ),
    ("secondary usage", "次级用量"),
    (
        "Heads up, you have less than {0}% of your {1} limit left. Run /status for a breakdown.",
        "提醒：你的{1}限额仅剩不到{0}%。运行/status查看明细。",
    ),
    (
        "Uses fewer credits for upcoming turns.",
        "后续轮次消耗更少额度。",
    ),
    ("Switch to {0}", "切换到{0}"),
    ("Keep current model", "保持当前模型"),
    (
        "Keep current model (never show again)",
        "保持当前模型（不再提示）",
    ),
    (
        "Hide future rate limit reminders about switching models.",
        "不再显示关于切换模型的速率限制提醒。",
    ),
    ("Approaching rate limits", "接近速率限制"),
    (
        "Switch to {0} for lower credit usage?",
        "切换到{0}以降低额度消耗？",
    ),
    (
        "Your workspace is out of credits. Ask your workspace owner to add more. Notify owner?",
        "你的工作区额度已用尽。请让工作区所有者补充额度。要通知所有者吗？",
    ),
    ("Usage limit reached", "已达用量上限"),
    (
        "Request a limit increase from your owner to continue using codex. Request increase?",
        "请向所有者申请提高限额以继续使用codex。要申请提高吗？",
    ),
    ("Workspace owner notified.", "已通知工作区所有者。"),
    (
        "Workspace owner was already notified recently.",
        "最近已通知过工作区所有者。",
    ),
    (
        "Could not notify your workspace owner. Please try again.",
        "无法通知工作区所有者。请重试。",
    ),
    ("Limit increase requested.", "已申请提高限额。"),
    (
        "A limit increase was already requested recently.",
        "最近已申请过提高限额。",
    ),
    (
        "Could not request a limit increase. Please try again.",
        "无法申请提高限额。请重试。",
    ),
    ("Saved conversation to {0}", "会话已保存到{0}"),
    ("could not load conversation: {0}", "无法加载会话：{0}"),
    (
        "could not load conversation history: {0}",
        "无法加载会话历史：{0}",
    ),
    ("could not create {0}: {1}", "无法创建{0}：{1}"),
    (
        "No active conversation to export.",
        "没有可导出的活动会话。",
    ),
    (
        "No conversation content to export.",
        "没有可导出的会话内容。",
    ),
    ("could not determine the home directory", "无法确定主目录"),
    ("Archived", "已归档"),
    ("Deleted", "已删除"),
    ("Unarchived", "已取消归档"),
    ("{0} session {1} ({2}).", "{0}会话 {1}（{2}）。"),
    ("{0} session {1}.", "{0}会话 {1}。"),
    ("Permanently delete session {0}?", "确定永久删除会话{0}？"),
    ("Continue? [y/N]: ", "继续？[y/N]："),
    ("failed to parse -c overrides: {0}", "解析-c覆盖项失败：{0}"),
    (
        "app server returned invalid session id `{0}`",
        "app server返回了无效的会话id `{0}`",
    ),
    ("failed to find Codex home", "找不到Codex主目录"),
    (
        "failed to resolve local runtime paths",
        "解析本地运行时路径失败",
    ),
    (
        "failed to discover execution environments",
        "发现执行环境失败",
    ),
    ("failed to resolve config cwd", "解析配置工作目录失败"),
    ("failed to load config.toml", "加载config.toml失败"),
    ("failed to load configuration", "加载配置失败"),
    (
        "failed to initialize environment manager",
        "初始化环境管理器失败",
    ),
    (
        "failed to initialize state database",
        "初始化状态数据库失败",
    ),
    ("Delete cancelled.", "已取消删除。"),
    (
        "cannot confirm session deletion without an interactive terminal; rerun with --force and a session UUID",
        "没有交互式终端时无法确认删除会话；请加--force并提供会话UUID重试",
    ),
    (
        "This cannot be undone. Subagent threads will also be deleted.",
        "此操作无法撤销。子代理线程也会被删除。",
    ),
    (
        "Usage: /goal [<objective>|clear|edit|pause|resume]",
        "用法：/goal [<objective>|clear|edit|pause|resume]",
    ),
    ("Not available in TUI yet.", "TUI中尚不可用。"),
    (
        "This sub-agent is controlled by its parent. Direct input is disabled.",
        "此子代理由其父级控制。已禁用直接输入。",
    ),
    ("Run the tool and continue.", "运行该工具并继续。"),
    ("Allow this request and continue.", "允许该请求并继续。"),
    (
        "Run the tool and remember this choice for this session.",
        "运行该工具，并在本次会话中记住此选择。",
    ),
    (
        "Allow this request and remember this choice for this session.",
        "允许该请求，并在本次会话中记住此选择。",
    ),
    ("Allow for this session", "本次会话内允许"),
    (
        "Run the tool and remember this choice for future tool calls.",
        "运行该工具，并在今后的工具调用中记住此选择。",
    ),
    (
        "Allow this request and remember this choice for future requests.",
        "允许该请求，并在今后的请求中记住此选择。",
    ),
    ("Always allow", "始终允许"),
    ("Cancel this tool call", "取消此次工具调用"),
    ("Decline this request and continue.", "拒绝该请求并继续。"),
    ("←/→ to navigate fields", "←/→ 切换字段"),
    (
        "ctrl + p / ctrl + n change field",
        "ctrl + p / ctrl + n切换字段",
    ),
    ("esc to cancel", "esc取消"),
    (
        "Answer required fields before submitting.",
        "提交前请先填写必填字段。",
    ),
    ("No options", "没有选项"),
    ("No fields", "没有字段"),
    ("{0} to submit", "{0} 提交"),
    ("{0} to submit answer", "{0} 提交答案"),
    ("{0} to submit all", "{0} 全部提交"),
    ("Field {0}/{1}", "字段 {0}/{1}"),
    ("{0} ({1} required unanswered)", "{0}（{1} 个必填项未填写）"),
    ("Allow", "允许"),
    ("Deny", "拒绝"),
    ("Confirm", "确认"),
    ("True", "真"),
    ("False", "假"),
    ("Apps are disabled.", "应用已禁用。"),
    (
        "Enable the apps feature to use $ or /apps.",
        "请启用应用特性以使用 $ 或 /apps。",
    ),
    ("No apps available.", "没有可用的应用。"),
    (
        "Loading installed and available apps...",
        "正在加载已安装和可用的应用…",
    ),
    ("Loading apps...", "正在加载应用…"),
    (
        "This updates when the full list is ready.",
        "完整列表就绪后会更新。",
    ),
    ("Failed to load apps.", "加载应用失败。"),
    ("App directory unavailable", "应用目录不可用"),
    (
        "The app directory request failed. Retry, or press Esc to continue.",
        "应用目录请求失败。请重试，或按Esc继续。",
    ),
    (
        "Reload installed and available apps.",
        "重新加载已安装和可用的应用。",
    ),
    (
        "Use $ to insert an installed app into your prompt.",
        "使用 $ 把已安装的应用插入提示词。",
    ),
    ("Manage this app in your browser.", "在浏览器中管理此应用。"),
    (
        "Install this app in your browser, then reload Codex.",
        "在浏览器中安装此应用，然后重新加载Codex。",
    ),
    ("Type to search apps", "输入以搜索应用"),
    ("Installed · Disabled", "已安装 · 已停用"),
    (
        "Installed {0} of {1} available apps.",
        "已安装 {0}/{1} 个可用应用。",
    ),
    (
        "{0}. Press Enter to open the app page to install, manage, or enable/disable this app.",
        "{0}。按Enter打开应用页面以安装、管理或启用/停用此应用。",
    ),
    (
        "{0}. Press Enter to open the app page to install this app.",
        "{0}。按Enter打开应用页面以安装此应用。",
    ),
    ("{0}. App link unavailable.", "{0}。应用链接不可用。"),
    (
        "Your usage does not need a reset right now.",
        "你的用量目前无需重置。",
    ),
    (
        "That reset is no longer available. Refresh to see your current resets.",
        "该重置项已不可用。请刷新以查看当前的重置项。",
    ),
    (
        "No usage limit resets are available.",
        "没有可用的用量限额重置项。",
    ),
    ("Usage reset.", "用量已重置。"),
    ("usage limit reset", "用量限额重置项"),
    ("usage limit resets", "用量限额重置项"),
    (
        "Usage reset. You have {0} {1} left.",
        "用量已重置。你还剩{0}个{1}。",
    ),
    (
        "You have {0} {1} available. Run /usage to use one.",
        "你有{0}个{1}可用。运行/usage使用一个。",
    ),
    (
        "No compatible setup was found to import.",
        "未找到可导入的兼容设置。",
    ),
    (
        "Import from other apps is unavailable in remote sessions. Start Codex locally and run /import.",
        "远程会话中不支持从其他应用导入。请在本地启动Codex并运行/import。",
    ),
    (
        "Import from other apps is unavailable while Codex is connected to the local app-server daemon. Stop the daemon, restart Codex, and run /import.",
        "当Codex连接到本地app-server守护进程时，不支持从其他应用导入。请停止守护进程、重启Codex并运行/import。",
    ),
    (", +{0} more", "，另有{0}项"),
    ("{0} failed", "{0}项失败"),
    ("{0} imported", "已导入{0}项"),
    ("Import failed: {0}", "导入失败：{0}"),
    (
        "{0} additional items remain. After it finishes, run /import again to review them.",
        "还剩{0}个额外项目。完成后请再次运行/import查看。",
    ),
    ("Import started.", "导入已开始。"),
    (
        " You can keep working while it finishes.",
        " 期间可继续工作。",
    ),
    (
        "Imported setup will apply to new chats.",
        "导入的设置将应用于新会话。",
    ),
    ("Import finished: ", "导入完成："),
    ("Results by type:", "按类型统计："),
    (
        "Run /import again to check for additional items.",
        "再次运行/import以检查更多项目。",
    ),
    (
        "1 additional item remains. After it finishes, run /import again to review it.",
        "还剩1个额外项目。完成后请再次运行/import查看。",
    ),
    (
        "Selected import source is no longer available.",
        "所选导入来源已不可用。",
    ),
    (" scroll disclosure", " 滚动展开"),
    ("to submit", "提交"),
    (
        "Your data may be used to improve our models and products",
        "你的数据可能被用于改进我们的模型和产品",
    ),
    (
        "Your feedback can be used to improve ChatGPT. ",
        "你的反馈可用于改进ChatGPT。",
    ),
    ("Tell us more (bad result)", "告诉我们更多（结果不佳）"),
    (
        "(optional) Write a short description to help us further",
        "（可选）写一段简短描述，帮助我们进一步了解",
    ),
    ("Tell us more (good result)", "告诉我们更多（结果良好）"),
    ("Tell us more (bug)", "告诉我们更多（缺陷）"),
    ("Tell us more (safety check)", "告诉我们更多（安全检查）"),
    (
        "(optional) Share what was refused and why it should have been allowed",
        "（可选）说明被拒绝的内容，以及为何本应被允许",
    ),
    ("Tell us more (other)", "告诉我们更多（其他）"),
    (
        "By submitting feedback, you agree that OpenAI can use your feedback for safety purposes and internal model training, as explained in more detail ",
        "提交反馈即表示你同意OpenAI将你的反馈用于安全目的和内部模型训练，详见",
    ),
    // Thread routing / key capture / local ChatGPT auth: the thread-switch status
    // lines (`tui/src/app/thread_routing.rs`), the key-capture prompts
    // (`tui/src/keymap_setup/capture.rs`) and the local auth diagnostics that
    // surface in the login flow (`tui/src/local_chatgpt_auth.rs`). Layout prefixes
    // such as "Action: " keep their trailing space in the key; the translation
    // drops it because CJK punctuation carries the separation.
    ("Action: ", "操作："),
    (
        "Agent thread {0} closed. Failed to switch back to main thread {1}.",
        "智能体线程{0}已关闭。无法切回主线程{1}。",
    ),
    (
        "Agent thread {0} closed. Switched back to main thread.",
        "智能体线程{0}已关闭。已切回主线程。",
    ),
    ("Current: ", "当前："),
    ("Failed to interrupt turn: {0}", "无法中断该轮次：{0}"),
    (
        "Failed to resolve app-server request for thread {0}: {1}",
        "无法处理线程{0}的app-server请求：{1}",
    ),
    (
        "First key: {0}. Press the second key. Esc cancels.",
        "第一个键：{0}。请按第二个键。按Esc取消。",
    ),
    (
        "Press the first key, then the second. Esc cancels.",
        "请依次按第一个键和第二个键。按Esc取消。",
    ),
    (
        "Press the new key now. Esc cancels.",
        "请现在按下新键。按Esc取消。",
    ),
    ("Remap Shortcut", "重新映射快捷键"),
    ("failed to load local auth: {0}", "加载本地认证失败：{0}"),
    ("failed to refresh skills", "刷新技能失败"),
    (
        "local ChatGPT auth is missing chatgpt account id",
        "本地ChatGPT认证缺少chatgpt账户ID",
    ),
    (
        "local ChatGPT auth is missing token data",
        "本地ChatGPT认证缺少令牌数据",
    ),
    (
        "local ChatGPT auth must use one of workspace(s) {0}, but found {1}",
        "本地ChatGPT认证必须使用工作区{0}中的一个，但发现{1}",
    ),
    (
        "local auth is not a ChatGPT login",
        "本地认证不是ChatGPT登录",
    ),
    ("no local auth available", "没有可用的本地认证"),
    // Transient history cells: the update-available notice, the patch/image
    // result headings, the plan cells, the web-search header and the startup
    // warning summary. The startup summary interpolates an "MCP " source marker
    // and inflects `issue{plural}` in English; Chinese needs no plural, so the
    // four English shapes map onto the same two Chinese values. The `(a; b)`
    // scaffolding and the trailing spaces in the fragment keys are layout and
    // deliberately stay in Rust (`history_cell/startup_warnings.rs`).
    ("Run ", "运行"),
    (" to update.", "以完成更新。"),
    ("See ", "参见"),
    (" for installation options.", "，了解安装选项。"),
    (
        "See https://github.com/openai/codex for installation options.",
        "参见https://github.com/openai/codex了解安装选项。",
    ),
    ("Generating conversation recap", "正在生成对话回顾"),
    ("Generating conversation recap...", "正在生成对话回顾…"),
    ("✘ Failed to apply patch", "✘ 应用补丁失败"),
    ("Image generation failed", "图像生成失败"),
    ("Generated Image:", "生成的图像："),
    ("Saved to: ", "保存到："),
    ("Proposed Plan", "建议计划"),
    ("Updated Plan", "已更新计划"),
    ("(no steps provided)", "（未提供步骤）"),
    ("Searched the web", "已搜索网络"),
    ("Searching the web", "正在搜索网络"),
    ("⚠ {0} MCP startup issue", "⚠ {0} 个MCP启动问题"),
    ("⚠ {0} MCP startup issues", "⚠ {0} 个MCP启动问题"),
    ("⚠ {0} startup issue", "⚠ {0} 个启动问题"),
    ("⚠ {0} startup issues", "⚠ {0} 个启动问题"),
    ("{0} MCP", "{0} 个MCP"),
    ("{0} needs sign-in", "{0} 个需要登录"),
    ("{0} need sign-in", "{0} 个需要登录"),
    (" · {0} for details", " · 详情见{0}"),
    ("workspace with network access", "工作区并允许网络访问"),
    // Exec cell (`tui/src/exec_cell/{render,live_output}.rs`), the transcript
    // shortcut hint (`tui/src/ui_consts.rs`, a function now: `tr` has to resolve at
    // render time) and the inline-visualization fallbacks
    // (`tui/src/inline_visualization.rs`). The `… +N lines` marker keeps its
    // leading glyph; the `(no output)` wrapper and the two-space hard break in the
    // rewritten markdown are scaffolding and stay in Rust.
    ("… +{0} lines", "… 另有{0}行"),
    ("… +{0} lines ({1})", "… 另有{0}行（{1}）"),
    ("ctrl + t to view transcript", "按ctrl + t查看记录"),
    (
        "Interacted with `{0}`, sent `{1}`",
        "与`{0}`交互，发送了`{1}`",
    ),
    ("Waited for `{0}`", "等待`{0}`"),
    ("(no output)", "（无输出）"),
    (
        "Open {0} visualization in the browser",
        "在浏览器中打开{0}可视化",
    ),
    (
        "_Visualization unavailable on this device._",
        "_此设备不支持可视化。_",
    ),
    ("    answer: ", "    回答："),
    // Terminal pet picker + notifications + the external-writer notice + the
    // request-user-input confirmation + the archive flow of the resume picker.
    // `pet` is rendered as 桌宠 (the ASCII terminal pet); the `R` / `/archive` key
    // and command names stay verbatim, and the plural forms collapse in Chinese.
    // The OS-notification bodies (`chatwidget/notifications.rs::display`) are
    // user-visible text, not protocol `type_name` strings.
    ("Failed to disable pets: {0}", "禁用桌宠失败：{0}"),
    ("Failed to save pet selection: {0}", "保存桌宠选择失败：{0}"),
    ("Failed to load pet: {0}", "加载桌宠失败：{0}"),
    (
        "Failed to load configured pet: {0}",
        "加载已配置的桌宠失败：{0}",
    ),
    ("Loading Pet", "正在加载桌宠"),
    ("Preparing the terminal pet.", "正在准备终端桌宠。"),
    ("Loading selected pet...", "正在加载所选桌宠…"),
    ("Approval requested: {0}", "请求审批：{0}"),
    ("Codex wants to edit {0}", "Codex想要编辑{0}"),
    ("{0} files", "{0} 个文件"),
    ("Approval requested by {0}", "由{0}请求审批"),
    ("Plan mode prompt: {0}", "计划模式提示：{0}"),
    ("Agent turn complete", "智能体轮次完成"),
    (
        "This conversation is open in another app",
        "此对话已在另一个应用中打开",
    ),
    (" to Retry", "重试"),
    (
        "Close it there and press R to continue here.",
        "在那里关闭它，然后按R在此继续。",
    ),
    ("{0} unanswered question", "{0} 个未回答的问题"),
    ("{0} unanswered questions", "{0} 个未回答的问题"),
    ("No choices", "无选项"),
    (
        "Selected session does not have a thread ID.",
        "所选会话没有线程ID。",
    ),
    (
        "Use /archive to archive the current session and exit.",
        "使用/archive归档当前会话并退出。",
    ),
    ("Failed to archive session: {0}", "归档会话失败：{0}"),
    (
        "Failed to restore archived session: {0}",
        "恢复已归档会话失败：{0}",
    ),
    // Keymap validation messages (`tui/src/keymap.rs`, `keymap/vim_search.rs`,
    // `keymap_setup.rs`, `chatwidget/keymap_picker.rs`) and the external-agent
    // config import screen (`external_agent_config_migration/*`). Config paths,
    // action ids and the `Press <Enter>` keycap keep their spelling; `Import ...`
    // strings in `external_agent_config_migration/mod.rs` are deliberately NOT
    // here: they normalize then re-match app-server descriptions.
    (
        "Ambiguous `tui.keymap.{0}` bindings: `{1}` uses a key reserved by `{2}`. Set a different key in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{0}` 的绑定有歧义：`{1}` 使用了由 `{2}` 保留的按键。请在 `~/.codex/config.toml` 中设置其他按键后重试。支持的按键与示例见Codex键位文档。",
    ),
    (
        "Conflicting `{0}` and `{1}` bindings",
        "`{0}` 与 `{1}` 的绑定冲突",
    ),
    ("Changes write the root ", "更改会写入根"),
    (
        "Invalid `tui.keymap` configuration: {0}",
        "`tui.keymap` 配置无效：{0}",
    ),
    ("Choose an import source", "选择导入来源"),
    (
        "Select the app whose setup you want to import.",
        "选择要导入其配置的应用。",
    ),
    (
        "Could not check for importable setup: {0}",
        "无法检查可导入的配置：{0}",
    ),
    // Chatwidget lifecycle surfaces: background-terminal status, external-writer
    // guards, the interrupted-turn notice, the thread name/rename titles, the
    // permission-discovery popup, hooks load errors and the reconnect banner.
    // `app-server`, `Ctrl+C` and the `ctrl+c` keycap stay verbatim.
    ("Waiting for background terminal", "正在等待后台终端"),
    (
        "This thread is open elsewhere. Close it there and retry resume to continue.",
        "此线程已在别处打开。请在那里关闭它，然后重试恢复以继续。",
    ),
    (
        "Cannot switch collaboration mode while a turn is running.",
        "轮次运行期间无法切换协作模式。",
    ),
    (
        "Model interrupted to submit steer instructions.",
        "模型被中断以提交引导指令。",
    ),
    (
        "Thread model is unavailable. Wait for the thread to finish syncing or choose a model before sending input.",
        "线程模型不可用。请等待线程同步完成，或先选择模型再发送输入。",
    ),
    ("Rename thread", "重命名线程"),
    ("Name thread", "命名线程"),
    ("Loading permission profiles…", "正在加载权限配置…"),
    ("Failed to load hooks: {0}", "加载钩子失败：{0}"),
    ("quit", "退出"),
    ("Reconnecting to app-server…", "正在重新连接app-server…"),
    (
        "Connection lost. Attempting to reconnect…",
        "连接已断开。正在尝试重新连接…",
    ),
    (
        "Reconnect failed — check the endpoint, then relaunch",
        "重新连接失败——请检查端点，然后重新启动",
    ),
    (
        "Automatic reconnect could not restore this session. Your draft is still editable. Copy it before quitting with Ctrl+C, then reconnect with the same command.",
        "自动重连无法恢复此会话。你的草稿仍可编辑。请在按Ctrl+C退出前复制它，然后用同一条命令重新连接。",
    ),
    // Chatwidget reservemodel/limits/status surfaces: the Luna reserve-model
    // notices, MCP startup warnings, permission-mode guard, plugin upgrade results,
    // the strict-review warning, credit-limit nudges, the reset-credit entries,
    // fork/continue history lines and the model-switch messages. English plural
    // forms become separate keys with one Chinese value.
    (
        "Luna model settings are unavailable; please try /model again in a moment.",
        "Luna模型设置不可用；请稍后重试/model。",
    ),
    (
        "Other models return when ordinary usage is available again.",
        "常规用量恢复后即可选择其他模型。",
    ),
    (
        "MCP startup interrupted. The following servers were not initialized: {0}",
        "MCP启动被中断。以下服务器未完成初始化：{0}",
    ),
    ("failed: {0}", "失败：{0}"),
    ("MCP startup incomplete ({0})", "MCP启动未完成（{0}）"),
    (
        "No other permission modes are available.",
        "没有其他可用的权限模式。",
    ),
    ("Updated roots: {0}", "已更新的根目录：{0}"),
    ("Failed to upgrade {0} {1}: {2}", "升级{0}个{1}失败：{2}"),
    (
        "This request requires additional safety checks, some tool calls might take extra time",
        "此请求需要额外的安全检查，部分工具调用可能耗时更长",
    ),
    (
        "You've reached your workspace credit limit",
        "你已达到工作区额度上限",
    ),
    ("Expires {0}", "有效期至{0}"),
    ("Full reset", "完整重置"),
    ("Reset your current usage limits.", "重置当前用量上限。"),
    ("Thread forked from ", "线程分叉自 "),
    (
        "You’re continuing from this point in a new conversation",
        "你将从新对话中的这一点继续",
    ),
    (
        "Model {0} does not support image inputs. Remove images or switch models.",
        "模型{0}不支持图像输入。请移除图像或切换模型。",
    ),
    ("Model changed to {0}", "模型已切换为{0}"),
    (
        "Current model ({0}) doesn't support personalities. Try /model to pick a different model.",
        "当前模型（{0}）不支持个性化。请尝试/model选择其他模型。",
    ),
    ("I've installed it", "我已安装"),
    // Bottom-pane surfaces: terminal-title action banner, banner hints, patch
    // approval headers, popup empty states, mentions footer/search modes, thread
    // approval hints, question counters, disabled-row rendering, Vim mode
    // indicator and the unified-exec background-terminal footer. Plural English
    // shapes pair with a single Chinese value; `esc`, `/ps`, `/stop`, `Vim` and the
    // ` - ` label join stay verbatim.
    ("Press a number to choose", "按数字选择"),
    (
        "Press a number to choose · esc to dismiss · type to continue",
        "按数字选择 · esc关闭 · 输入以继续",
    ),
    ("esc to dismiss · type to continue", "esc关闭 · 输入以继续"),
    ("Destination: ", "目标位置："),
    ("unavailable", "不可用"),
    (" insert · ", " 插入 · "),
    (" close · ", " 关闭 · "),
    (" switch search modes", "切换搜索模式"),
    ("All Results", "全部结果"),
    ("Filesystem Only", "仅文件系统"),
    (" (disabled)", "（已禁用）"),
    (" to insert or ", "插入或"),
    ("Vim: Normal", "Vim: 普通模式"),
    ("Vim: Insert", "Vim: 插入模式"),
    ("Vim: Replace", "Vim: 替换模式"),
    (" to answer", "回答"),
    (
        "Answer too long; limit {0} characters",
        "回答过长；上限{0}个字符",
    ),
    ("{0} (if available)", "{0}（如可用）"),
    ("Approval needed in {0}", "线程{0}需要审批"),
    (" to switch threads", "切换线程"),
    ("{0} question", "{0} 个问题"),
    ("{0} questions", "{0} 个问题"),
    ("{0} (disabled: {1})", "{0}（已禁用：{1}）"),
    ("disabled: {0}", "已禁用：{0}"),
    (
        "{0} background terminal running · /ps to view · /stop to close",
        "{0} 个后台终端运行中 · /ps查看 · /stop关闭",
    ),
    (
        "{0} background terminals running · /ps to view · /stop to close",
        "{0} 个后台终端运行中 · /ps查看 · /stop关闭",
    ),
    ("skills", "技能"),
    // Guardian path-localization error (`chatwidget/protocol_requests.rs`): the
    // conversion target had to be named explicitly (`GuardianAssessmentAction`) for
    // the `map_err` closure to keep a pinned type.
    (
        "failed to localize guardian filesystem paths: {0}",
        "无法本地化guardian文件系统路径：{0}",
    ),
    // App-level surfaces: the agent picker subtitle, the agents-overview composer
    // and list, the transcript notice, session-picker errors and buttons, startup
    // permission-override refusals, the stale-agent-list reconnect banner and the
    // config-change notices. Keycap placeholders keep their position.
    (
        "Select an agent to watch. {0} previous, {1} next.",
        "选择要查看的智能体。上一个：{0}，下一个：{1}。",
    ),
    ("Describe a new task", "描述一个新任务"),
    ("Unsent task: {0}", "未发送的任务：{0}"),
    ("tasks · dispatch paused", "任务 · 调度已暂停"),
    ("  current", "  当前"),
    ("Task details", "任务详情"),
    ("No prompt available.", "没有可用的提示词。"),
    (
        "Earlier messages are available — press {0} to view the full transcript",
        "更早的消息可用——按{0}查看完整记录",
    ),
    (
        "Failed to start TUI session picker: {0}",
        "启动TUI会话选择器失败：{0}",
    ),
    (
        "Failed to open session picker: {0}",
        "打开会话选择器失败：{0}",
    ),
    ("Unable to resume session", "无法恢复会话"),
    ("Return to command center", "返回命令中心"),
    (
        "Permission overrides are not supported when resuming a remote task.",
        "恢复远程任务时不支持权限覆盖。",
    ),
    (
        "Permission overrides are not supported when forking a remote task.",
        "分叉远程任务时不支持权限覆盖。",
    ),
    (
        "Reconnect failed — agent list is stale; relaunch to retry",
        "重新连接失败——智能体列表已过期；请重新启动后重试",
    ),
    ("Experimental feature changes", "实验性功能变更"),
    ("Memory setting changes", "记忆设置变更"),
    // Startup warnings that DO reach the user: `project_config_warning` and
    // `skill_load_warning_messages` feed StartupWarningsCell / add_warning_message
    // (app/startup.rs:183, app/working_directory.rs:531, app/thread_routing.rs:1807).
    // The folder list (`    N. <folder>`) and the per-folder reasons stay verbatim.
    (
        "Skipped loading {0} skill(s) due to invalid SKILL.md files.",
        "由于SKILL.md文件无效，已跳过加载{0}个技能。",
    ),
    (
        "Project-local config, hooks, and exec policies are disabled in the following folders until the project is trusted, but skills still load.",
        "在项目被信任之前，以下文件夹中的项目级配置、钩子和执行策略已禁用，但技能仍会加载。",
    ),
    // Permission discovery menu (`tui/src/permission_discovery.rs`), the update
    // prompt (`tui/src/update_prompt.rs`) and the startup draft header
    // (`tui/src/startup_draft.rs`). `/permissions` and the release-notes URL stay
    // verbatim; the leading spaces in the session-action labels are layout.
    (
        "The server returned duplicate permission profiles.",
        "服务器返回了重复的权限配置。",
    ),
    (
        "Permission discovery exceeded its pagination limit. Try /permissions again.",
        "权限发现超出分页上限。请重试/permissions。",
    ),
    (
        "Permission discovery timed out. Try /permissions again.",
        "权限发现超时。请重试/permissions。",
    ),
    (
        "This server does not support permission discovery. Upgrade the Codex server to use this menu.",
        "此服务器不支持权限发现。请升级Codex服务器以使用此菜单。",
    ),
    ("Failed to load permissions: {0}", "加载权限失败：{0}"),
    ("Release notes: ", "版本说明："),
    ("Update now (runs `{0}`)", "立即更新（运行`{0}`）"),
    ("Skip until next version", "跳过直到下个版本"),
    ("Skip", "跳过"),
    ("  Resuming session…", "  正在恢复会话…"),
    ("  Forking session…", "  正在分叉会话…"),
    // Unarchive prompt, session start/archive commands, the status-indicator
    // interrupt hint, the proper-join helper, --add-dir advice, the fatal-error
    // prefix, the cross-thread transcript header and the branch failure. The verbs
    // interpolated into those sentences (`resume`/`fork`, participles) are translated
    // at their definitions so the rendered sentence is not half English.
    ("This conversation is archived", "此对话已归档"),
    ("Unarchive and {0}", "取消归档并{0}"),
    (" to continue or ", "继续，或"),
    ("fork", "分叉"),
    ("resumed", "已恢复"),
    ("forked", "已分叉"),
    ("Failed to unarchive session {0}", "取消归档会话失败：{0}"),
    (
        "Failed to {0} session from {1}: {2}",
        "从{1}{0}会话失败：{2}",
    ),
    (" to interrupt)", " 中断）"),
    ("{0} and {1}", "{0} 和 {1}"),
    (
        "Ignoring --add-dir ({0}) because the effective permissions do not allow additional writable roots. Switch to workspace-write or danger-full-access to allow them.",
        "忽略--add-dir（{0}）：当前生效的权限不允许额外的可写根目录。请切换到workspace-write或danger-full-access以允许它们。",
    ),
    (
        "Sent by Codex from task {0}\n{1}",
        "由Codex从任务{0}发送\n{1}",
    ),
    (
        "Failed to branch before the selected prompt: {0}",
        "无法在所选提示词之前分叉：{0}",
    ),
    (
        "No active or archived session found matching '{0}'.",
        "没有找到匹配{0}的活跃或已归档会话。",
    ),
    ("active or archived", "活跃或已归档"),
    (
        "No {0} session found matching '{1}'.",
        "没有找到匹配{1}的{0}会话。",
    ),
    (
        "Permanently delete session '{0}' ({1})?",
        "永久删除会话{0}（{1}）？",
    ),
    // Misalignment review overlay, the automatic model-switch banner and the
    // permission-shortcut result messages. The banner is assembled as
    // `{prefix} {model}{suffix}` in Rust, so the space after the prefix stays a
    // format-string detail; `app server` and `/permissions` keep their spelling.
    ("Continuation request (quoted)", "继续请求（引用）"),
    ("What we detected", "我们检测到的内容"),
    (
        "Couldn’t continue this chat. Review its latest status before trying again.",
        "无法继续此对话。请先查看其最新状态后再试。",
    ),
    ("Automatically switched to", "已自动切换到"),
    (" due to usage limits.", "，因为已达用量上限。"),
    ("Automatically switched back to", "已自动切回"),
    (
        " because ordinary usage is available again.",
        "，因为常规用量已恢复。",
    ),
    (
        "this app server does not support confirmed permission changes; use /permissions",
        "此app server不支持经确认的权限变更；请使用/permissions",
    ),
    ("Failed to update permissions: {0}", "更新权限失败：{0}"),
    // Token-activity panel, status copy targets, rate-limit lines, the review
    // indicator, working-directory guards, theme fallback warnings, the startup
    // keymap error and thread-goal failures. The thread-goal template takes the
    // verb as {0}, so the callers pass tr(current(), "read"/"replace"/...) too.
    ("   Loading...", "   正在加载…"),
    ("   Token activity unavailable", "   Token活动不可用"),
    ("Session ID", "会话ID"),
    ("{0} {1}% left", "{0} 剩余{1}%"),
    ("Reviewing approval request", "正在审核审批请求"),
    ("Reviewing {0} approval requests", "正在审核{0}个审批请求"),
    (
        "The session must start before you can change its working directory.",
        "会话必须先启动，才能更改其工作目录。",
    ),
    (
        "Changing directories requires an idle primary session without queued input.",
        "切换目录需要主会话空闲且没有排队的输入。",
    ),
    (
        "Custom theme \"{0}\" at {1} could not be loaded (invalid .tmTheme format). Falling back to the default theme.",
        "位于{1}的自定义主题“{0}”无法加载（.tmTheme格式无效）。已回退到默认主题。",
    ),
    (
        "Theme \"{0}\" not found. Using the default theme. To use a custom theme, place a .tmTheme file at {1}.",
        "未找到主题“{0}”。正在使用默认主题。若要使用自定义主题，请将.tmTheme文件放在{1}。",
    ),
    (
        "Invalid `tui.keymap` configuration: {0}\nFix the config and retry.\nSee the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap`配置无效：{0}\n请修正配置后重试。\n支持的按键与示例见Codex键位文档。",
    ),
    ("Failed to {0} thread goal: {1}", "线程目标{0}失败：{1}"),
    ("read", "读取"),
    ("replace", "替换"),
    ("update", "更新"),
    ("clear", "清除"),
    // Memories reset confirmation rows, the shared disabled-row suffix, the
    // onboarding trust error and the public composer placeholder.
    ("Go back", "返回"),
    ("Failed to set trust for {0}: {1}", "无法为{0}设置信任：{1}"),
    ("Compose new task", "撰写新任务"),
    // Errors that used to be thiserror `#[error("...")]` attributes. The attribute
    // only accepts literals, so these three types implement Display by hand
    // (`external_editor.rs`, `named_session_lookup.rs`, `app_server_session.rs`).
    // English output is unchanged: tr(En, key) returns the key verbatim.
    ("neither VISUAL nor EDITOR is set", "未设置VISUAL或EDITOR"),
    ("failed to parse editor command", "解析编辑器命令失败"),
    (
        "Multiple sessions match '{0}' (including {1} and {2}); use a session UUID to disambiguate.",
        "有多个会话匹配{0}（包括{1}和{2}）；请使用会话UUID以消除歧义。",
    ),
    (
        "Cannot verify a unique session label across server pages; matching session UUID: {0}. Use it only if this is the session you want.",
        "无法跨服务器分页确认唯一的会话标签；匹配的会话UUID：{0}。仅当你确实想要该会话时才使用它。",
    ),
    (
        "the selected permission profile cannot be safely represented by the legacy app-server sandbox policy; select a named or legacy-compatible permission profile",
        "所选权限配置无法由旧的app-server沙箱策略安全表示；请选择具名或兼容旧版的权限配置",
    ),
    // Startup failure message (`tui/src/startup_error.rs`): the type used to
    // carry a thiserror `#[error("...")]` attribute, now a hand-written Display so
    // it can be translated. It reaches the user through lib.rs's io::Error::other.
    (
        "failed to initialize sqlite local db at {0}: {1}",
        "无法在{0}初始化sqlite本地数据库：{1}",
    ),
    // Short labels (2-7 characters) that the shape-based scanner cannot flag:
    // selection-row names, popup title and confirm buttons. `Cancel`/`Close`/`Retry`
    // also appear elsewhere; the dictionary key is shared, so the value must stay
    // consistent with those sites.
    ("Retry", "重试"),
    ("Back", "返回"),
    ("Action", "操作"),
    ("Hooks", "钩子"),
    ("Apps", "应用"),
    ("Auth", "认证"),
    ("Quit", "退出"),
    // Rate-limit reset popups (`chatwidget/usage.rs`). These sit one line away
    // from a `tr(..)` call, so the lenient "is this literal wrapped" lookback in
    // `scripts/i18n_todo.py` never listed them; the `--precise` rule did.
    ("Resetting your usage...", "正在重置你的用量…"),
    ("Using a reset...", "正在使用一次重置…"),
    (
        "Couldn't reset usage. Please try again.",
        "无法重置用量，请重试。",
    ),
    (
        "Usage reset. Checking your remaining resets...",
        "用量已重置。正在检查剩余的重置次数…",
    ),
    ("Refreshing...", "刷新中…"),
    // Approval history cells (`history_cell/approvals.rs`): sentence fragments
    // assembled as styled spans. The Chinese keeps the same order, so each
    // fragment stands on its own (design §3.6). The single-token ones
    // (`canceled`, `You `, ` files`) are below the scanner's candidate length,
    // so only the `--precise` sweep listed them.
    ("timed out", "超时"),
    ("canceled", "已取消"),
    ("You ", "你 "),
    (" files", " 个文件"),
    (" for ", "："),
    (" before ", "，"),
    (" before codex could run ", "，codex未能运行"),
    (" before codex could access ", "，codex未能访问"),
    (" before codex could apply ", "，codex未能应用"),
    (" for codex to apply ", "，让codex应用"),
    (
        " before this request could be approved",
        "，本次请求未获批准",
    ),
    (
        " codex to always run commands that start with ",
        "，让codex始终运行以此开头的命令：",
    ),
];

/// English source text -> Simplified Chinese.
pub(crate) static DICT_ZH: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| ENTRIES.iter().copied().collect());
