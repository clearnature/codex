//! Traditional Chinese dictionary -- GENERATED from `dict_zh.rs` by
//! `scripts/i18n_gen_hant.py` (OpenCC `s2twp`, Taiwan vocabulary). Do not edit
//! by hand: keys must stay in strict parity with `dict_zh.rs`, which
//! `codex-i18n-check` enforces. Regenerate after any `dict_zh.rs` change.

use std::collections::HashMap;
use std::sync::LazyLock;

/// English source text -> Traditional Chinese (Taiwan vocabulary).
pub(crate) static ENTRIES: &[(&str, &str)] = &[
    ("      Importing: none", "      正在匯入：無"),
    ("      • +{0} more marketplaces", "      • 還有{0}個市場"),
    ("     - <none>", "     - <無>"),
    ("     reason: {0}", "     原因：{0}"),
    ("     {0}: <empty>", "     {0}: <空>"),
    ("    See the ", "    參見 "),
    ("    note: ", "    備註："),
    ("    • Auth: ", "    • 認證："),
    ("    • Resource templates: ", "    • 資源模板："),
    ("    • Resource templates: (none)", "    • 資源模板：（無）"),
    ("    • Resources: ", "    • 資源："),
    ("    • Resources: (none)", "    • 資源：（無）"),
    ("    • Tools: ", "    • 工具："),
    ("    • Tools: (none)", "    • 工具：（無）"),
    (
        "  Use /mcp verbose for tools and resources.",
        "  使用/mcp verbose檢視工具與資源。",
    ),
    (
        "  - max_depth = {0} (V1 only; ignored by V2)",
        "  - max_depth = {0}（僅V1；V2忽略）",
    ),
    ("   Less ", "   較少 "),
    (
        "   No token activity in the last 12 months",
        "   最近12個月沒有Token活動",
    ),
    (
        "   Token activity history unavailable",
        "   Token活動歷史不可用",
    ),
    (
        "   Widen terminal to show activity graph",
        "   拉寬終端以顯示活動圖",
    ),
    ("   last 12 months", "   最近12個月"),
    ("  - {0}: {1} (source: {2})", "  - {0}: {1}（來源：{2}）"),
    (
        "  1. Open this link in your browser and sign in",
        "  1. 在瀏覽器中開啟此連結並登入",
    ),
    (
        "  2. Enter this one-time code after you are signed in (expires in 15 minutes)",
        "  2. 登入後輸入這個一次性驗證碼（15分鐘內有效）",
    ),
    ("  <none>", "  <無>"),
    (
        "  No experimental features available for now",
        "  目前沒有可用的實驗性特性",
    ),
    ("  Billed tokens", "  計費Token"),
    (
        "  Continue only if you started this login in Codex. If a website or another person gave you this code, cancel.",
        "  只有當你自己在本機Codex中發起登入時才繼續。如果是網站或他人給了你這個驗證碼，請取消。",
    ),
    (
        "  Choose which local AI server to use for your session.",
        "  選擇本次會話要使用的本地AI伺服器。",
    ),
    ("  Models", "  模型"),
    ("  No memory settings available", "  沒有可用的記憶設定"),
    (
        "  Press Enter to select • Ctrl+C to exit",
        "  按Enter選擇 • 按Ctrl+C退出",
    ),
    ("  Reasoning", "  推理"),
    ("  Rename › ", "  重新命名 › "),
    ("  Requesting a one-time code...", "  正在請求一次性驗證碼…"),
    ("  Search › ", "  搜尋 › "),
    (
        "  To get started, describe a task or try one of these commands:",
        "  開始使用：描述一個任務，或試試以下命令：",
    ),
    ("  no match", "  無匹配"),
    ("  searching", "  搜尋中"),
    ("  {0}. {1} ({2})", "  {0}. {1}（{2}）"),
    ("  • No MCP servers configured.", "  • 未配置MCP伺服器。"),
    ("  • No MCP tools available.", "  • 沒有可用的MCP工具。"),
    (
        "  • No background terminals running.",
        "  • 沒有正在執行的後臺終端。",
    ),
    ("  • No sub-agents running.", "  • 沒有執行中的子代理。"),
    ("  ● Running  ○ Not Running", "  ● 執行中  ○ 未執行"),
    (" (+ {0} cached)", "（+{0}快取）"),
    (" (current)", "（當前）"),
    (" (default)", "（預設）"),
    (" (interrupted)", "（已中斷）"),
    (" (press ", "（按 "),
    (" (read-only)", "（只讀）"),
    (" (reasoning {0})", "（推理{0}）"),
    (" (unanswered)", "（未回答）"),
    (
        " - choose what Codex is allowed to do",
        " - 選擇允許Codex做什麼",
    ),
    (
        " - choose what model and reasoning effort to use",
        " - 選擇模型與推理強度",
    ),
    (
        " - create an AGENTS.md file with instructions for Codex",
        " - 建立AGENTS.md檔案，寫入給Codex的說明",
    ),
    (
        " - review any changes and find issues",
        " - 審查改動並找出問題",
    ),
    (
        " - show current session configuration",
        " - 顯示當前會話配置",
    ),
    (" Token activity", " Token活動"),
    (" close", " 關閉"),
    (" custom · ", " 自定義 · "),
    (" edit last queued message", " 編輯最後一條排隊訊息"),
    (" edit shortcut · ", " 編輯快捷鍵 · "),
    (" for agents", " 切換智慧體"),
    (" for shortcuts", " 檢視快捷鍵"),
    (" group · ", " 分組 · "),
    (
        " history unavailable | PgUp to retry ",
        " 歷史不可用 | 按PgUp重試 ",
    ),
    (" loading older history... ", " 正在載入更早的歷史… "),
    (" navigate  ", " 導航  "),
    (" or ", " 或 "),
    (
        " partial history | PgUp for earlier ",
        " 部分歷史 | 按PgUp檢視更早 ",
    ),
    (" start inspector · ", " 啟動檢查器 · "),
    (
        " the request for codex network access to ",
        " Codex請求訪問以下網路的 ",
    ),
    (" start of history ", " 歷史起點 "),
    (" the request to run ", " 請求執行以下內容的 "),
    (" this session", " 本次會話"),
    (" to change", " 切換"),
    (" to configure them.", " 進行配置。"),
    (" to confirm", " 確認"),
    (" to confirm and close", " 確認並關閉"),
    (" to confirm or ", " 確認，或"),
    (" to continue and create a sandbox...", " 以繼續並建立沙箱…"),
    (" to continue, ", " 繼續，"),
    (" to customize", " 自定義"),
    (" to interrupt and send immediately)", " 可中斷並立即傳送）"),
    (" to move, ", " 移動，"),
    (" to move, press ", " 移動，按 "),
    (" to queue message", " 排隊傳送訊息"),
    (" to queue", " 排隊"),
    (" to review hooks; ", " 審查鉤子；"),
    (" to save or select", " 儲存或選擇"),
    (" to select, ", " 選擇，"),
    (" to submit message", " 傳送訊息"),
    (" to interrupt", " 中斷"),
    (" to exit", " 退出"),
    (" again to quit", " 再按一次退出"),
    (" again to edit previous message", " 再按一次編輯上一條訊息"),
    (" to edit previous message", " 編輯上一條訊息"),
    (" for agents (empty prompt)", " 切換智慧體（空提示）"),
    (
        "App server did not report $CODEX_HOME; cannot materialize goal files",
        "應用伺服器未報告$CODEX_HOME；無法生成目標檔案",
    ),
    (
        "Could not create goal attachment directory {0}",
        "無法建立目標附件目錄{0}",
    ),
    (" to toggle", " 切換"),
    (" to toggle, ", " 切換，"),
    (" to toggle; ", " 切換；"),
    (" to trust all; ", " 全部信任；"),
    (" unbound · ", " 未繫結 · "),
    (
        "'/archive' is unavailable in side conversations. Press Ctrl+C to return to the main thread first.",
        "側會話中不可用'/archive'。請先按Ctrl+C返回主執行緒。",
    ),
    (
        "'/delete' is unavailable in side conversations. Press Ctrl+C to return to the main thread first.",
        "側會話中不可用'/delete'。請先按Ctrl+C返回主執行緒。",
    ),
    (" {0} or ", " 或{0} "),
    (" {0}/{1} answered", " 已答 {0}/{1}"),
    (" · exit {0}", " · 退出碼 {0}"),
    ("({0} cached)", "（{0} 已快取）"),
    ("**Tip:** {0}", "**提示：** {0}"),
    (
        ", OpenAI's command-line coding agent",
        "，OpenAI的命令列編碼代理",
    ),
    ("*New* Build faster with Codex.", "*新* 用Codex更快構建。"),
    (
        "*New* Build faster with the **Desktop app**. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true",
        "*新* 用**桌面應用**更快構建。執行'codex app'，或訪問https://chatgpt.com/codex?app-landing-page=true",
    ),
    (
        "*New* For a limited time, Codex is included in your plan for free – let’s build together.",
        "*新* 限時福利：你的套餐已免費包含Codex — 一起構建吧。",
    ),
    (
        "*New* Use **/fast** to enable our fastest inference with increased plan usage.",
        "*新* 用 **/fast** 開啟最快推理（套餐用量會增加）。",
    ),
    ("+{0} more", "還有{0}個"),
    ("- (best {0}d)", "-（最長{0}天）"),
    ("1 MCP server", "1個MCP伺服器"),
    ("1 action.", "1個操作。"),
    (
        "Actions without an active shortcut.",
        "沒有生效快捷鍵的操作。",
    ),
    (
        "Bring over supported setup from another coding agent.",
        "從另一個編碼代理遷移受支援的配置。",
    ),
    ("1 app", "1個應用"),
    (
        "1 hook needs review before it can run.",
        "有1個鉤子在執行前需要審查。",
    ),
    ("1 hook is new or changed.", "有1個鉤子是新增或已變更。"),
    ("1 {0} ago", "1{0}前"),
    ("<< Code review finished >>", "<< 程式碼審查已完成 >>"),
    ("<audio content>", "<音訊內容>"),
    ("<image content>", "<影象內容>"),
    ("<unknown embedded resource>", "<未知嵌入式資源>"),
    ("<unspecified>", "<未指定>"),
    (
        ">> Code review started: {0} <<",
        ">> 程式碼審查已開始：{0} <<",
    ),
    ("A balanced stack for deep work", "深度工作用的平衡堆疊"),
    (
        "A steady rock when the diff gets large",
        "diff變大時的穩如磐石",
    ),
    (
        "A tidy duck for calm workspace days",
        "平靜工作日裡的整潔小鴨",
    ),
    ("A recap is already being generated.", "已在生成回顧。"),
    ("A tiny blue-screen gremlin", "藍色畫面小精靈"),
    (
        "Active background terminals block /cd.",
        "有活動的後臺終端，/cd被阻止。",
    ),
    (
        "Add Codex files alongside your existing project files",
        "在你的現有專案檔案旁新增Codex檔案",
    ),
    ("Acknowledge findings and continue", "確認發現並繼續"),
    ("Add a short status model", "新增簡短的狀態模型"),
    ("Advanced Reasoning", "高階推理"),
    ("Agent command center", "代理指揮中心"),
    ("Agent errored", "代理出錯"),
    ("Agent resume failed", "代理恢復失敗"),
    (
        "Agent session {0} is unavailable: {1}",
        "代理會話{0}不可用：{1}",
    ),
    ("Agent spawn failed", "代理啟動失敗"),
    (
        "Ask a workspace admin to confirm plugin access.",
        "請聯絡工作區管理員確認外掛訪問許可權。",
    ),
    (
        "Ask a workspace admin to enable Codex plugins or plugin sharing.",
        "請聯絡工作區管理員啟用Codex外掛或外掛共享。",
    ),
    (
        "Ask the sharer or a workspace admin to confirm plugin access.",
        "請聯絡共享者或工作區管理員確認外掛訪問許可權。",
    ),
    (
        "Apply the latest diff produced by Codex agent as a `git apply` to your local working tree",
        "把Codex代理生成的最新diff以 `git apply` 應用到本地工作樹。",
    ),
    (
        "Agent thread {0} could not be resumed live. Replaying saved transcript.",
        "代理執行緒{0}無法實時恢復。正在回放已儲存的記錄。",
    ),
    (
        "Agent thread {0} is already active.",
        "代理執行緒{0}已經處於活動狀態。",
    ),
    (
        "Agent thread {0} is closed. Replaying saved transcript.",
        "代理執行緒{0}已關閉。正在回放已儲存的記錄。",
    ),
    (
        "An experimental feature save is still in progress. Retry after it finishes.",
        "實驗性特性儲存仍在進行中。請在完成後重試。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: `ctrl-z` is reserved for suspending the terminal on Unix. Choose a different chord and retry.",
        "`{0}` = `{1}` 有歧義：`ctrl-z` 在Unix上保留用於掛起終端。請換一組和絃後重試。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: its prefix shadows `{2}`. Unbind or remap the existing shortcut before using it as a chord prefix.",
        "`{0}` = `{1}` 有歧義：其字首遮蔽了 `{2}`。請先解除或改綁已有快捷鍵，再把它用作戰和絃字首。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: plain `esc` is reserved for cancelling a pending chord.",
        "`{0}` = `{1}` 有歧義：單獨的 `esc` 保留用於取消未完成的和絃。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: the chord uses the key reserved by `{2}`. Choose a different chord and retry.",
        "`{0}` = `{1}` 有歧義：該和絃使用了 `{2}` 保留的按鍵。請換一組和絃後重試。",
    ),
    (
        "Ambiguous `{0}` = `{1}`: the same chord is already assigned to `{2}`. Choose a unique chord and retry.",
        "`{0}` = `{1}` 有歧義：同一和絃已分配給 `{2}`。請使用不重複的和絃後重試。",
    ),
    ("All models → {0}", "所有模型 → {0}"),
    ("Alpha", "Alpha"),
    ("Always use current directory", "始終使用當前目錄"),
    (
        "Always use current directory ({0})",
        "始終使用當前目錄（{0}）",
    ),
    ("Always use session directory", "始終使用會話目錄"),
    (
        "Ambiguous `tui.keymap.{2}` bindings: `{0}` shadows `{1}` with the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{2}` 的繫結有歧義：`{0}` 以同一個按鍵遮蔽了 `{1}`。請在`~/.codex/config.toml`中設定不重複的按鍵後重試。支援的動作與示例見Codex快捷鍵文件。",
    ),
    (
        "Ambiguous `tui.keymap.{2}` bindings: `{0}` and `{1}` use the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{2}`的繫結有歧義：`{0}`與`{1}`使用了同一個按鍵。請在`~/.codex/config.toml`中設定不重複的按鍵後重試。支援的動作與示例見Codex快捷鍵文件。",
    ),
    (
        "Ambiguous approval overlay keymap bindings: `{0}` and `{1}` use the same key. Set unique keys in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "審批浮層的快捷鍵繫結有歧義：`{0}`與`{1}`使用了同一個按鍵。請在`~/.codex/config.toml`中設定不重複的按鍵後重試。支援的動作與示例見Codex快捷鍵文件。",
    ),
    ("Answer the questions to continue.", "回答問題以繼續。"),
    ("Approvals reviewer: {0}", "審批覆核者：{0}"),
    (
        "Archive a saved session by id or session name",
        "按id或會話名歸檔已儲存的會話。",
    ),
    (
        "Attestation generation is not available in TUI.",
        "TUI中不支援生成證明。",
    ),
    ("Ask Codex to do anything", "讓Codex做任何事"),
    ("Ask a follow-up question", "繼續追問"),
    ("Assigned actions:", "已繫結的操作："),
    ("Auto-reviewer ", "自動審查器 "),
    ("Background terminals", "後臺終端"),
    ("Beta", "Beta"),
    ("Blocked", "已阻塞"),
    (
        "Choose a pet to wake in the terminal.",
        "選擇要在終端中喚醒的寵物。",
    ),
    (
        "Cannot continue into the new worktree because source configuration changed.",
        "源配置已變更，無法繼續進入新的工作樹。",
    ),
    (
        "Cannot continue into the new worktree while the source session changed or is unavailable.",
        "源會話已變更或不可用，無法繼續進入新的工作樹。",
    ),
    (
        "Cannot create a worktree from an explicitly untrusted source.",
        "無法從顯式不受信任的來源建立工作樹。",
    ),
    (
        "Cannot load the new worktree configuration: {0}",
        "無法載入新的工作樹配置：{0}",
    ),
    (
        "Check that you are signed in to the correct workspace and still have access.",
        "請確認你已登入正確的工作區且仍擁有訪問許可權。",
    ),
    (
        "Browse all agent sessions on the shared local app-server daemon",
        "瀏覽共享本地app-server守護程序上的所有代理會話。",
    ),
    ("Booting MCP server:", "正在啟動MCP伺服器："),
    ("Cancel task", "取消任務"),
    ("Cannot change directories: {0}", "無法切換目錄：{0}"),
    (
        "Cannot fork into this worktree because developer instructions differ. Start a new conversation instead. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "開發者指令不同，無法派生到該工作樹。請改為開始新會話。已在{0}建立未使用的檢出；請從源倉庫執行 `git worktree remove <checkout-path>` 將其刪除。",
    ),
    (
        "Cannot dispatch `{0}`: the keymap action inventory exceeds {1} internal tokens.",
        "無法分派 `{0}`：快捷鍵動作清單超過{1}個內部標記。",
    ),
    (
        "Cannot dispatch unknown keymap action `{0}`.",
        "無法分派未知的快捷鍵動作 `{0}`。",
    ),
    ("Cannot load {0}: {1}", "無法載入{0}：{1}"),
    (
        "Cannot register managed worktree ownership: {0}",
        "無法註冊託管工作樹歸屬：{0}",
    ),
    (
        "Cannot open external editor: set $VISUAL or $EDITOR before starting Codex.",
        "無法開啟外部編輯器：請在啟動Codex前設定$VISUAL或$EDITOR。",
    ),
    (
        "Checked features are configured on. Some experimental features take effect only in new tasks or after restarting the Codex server.",
        "已勾選的特性將配置為開啟。部分實驗性特性僅在新建任務或重啟Codex伺服器後生效。",
    ),
    (
        "Choose how Codex uses and creates memories. Changes are saved to config.toml",
        "選擇Codex如何使用與建立記憶。更改會儲存到config.toml",
    ),
    (
        "Changes were saved, but the configured values differ from your selections. A higher-priority setting may override them.",
        "更改已儲存，但配置值與你的選擇不一致。可能有更高優先順序的設定覆蓋了它們。",
    ),
    (
        "Cannot safely retry a turn whose input exceeds the bounded history page.",
        "該輪次的輸入超出有界歷史頁範圍，無法安全重試。",
    ),
    ("Chat paused as a precaution", "聊天已出於安全考慮暫停"),
    ("Chat sessions", "聊天會話"),
    ("Chat sessions ({0})", "聊天會話（{0}）"),
    ("Chat stopped as a precaution", "聊天已出於安全考慮停止"),
    (
        "Choose a communication style for Codex.",
        "選擇Codex的溝通風格。",
    ),
    ("Choose a Markdown filename", "選擇Markdown檔名"),
    ("Choose an action", "選擇操作"),
    (
        "Choose how you'd like Codex to proceed.",
        "選擇你希望Codex如何進行。",
    ),
    ("Choose items to import.", "選擇要匯入的條目。"),
    (
        "Choose what happens to the current task.",
        "選擇如何處理當前任務。",
    ),
    ("Choose what to import", "選擇要匯入的內容"),
    (
        "Codex may add files to your current project folder.",
        "Codex可能會向當前專案資料夾新增檔案。",
    ),
    (
        "Clear local memory files and summaries. Existing threads stay intact.",
        "清除本地記憶檔案與摘要。現有執行緒不受影響。",
    ),
    ("Choose working directory to ", "選擇工作目錄以"),
    ("Closed", "已關閉"),
    ("Closed an agent", "已關閉一個代理"),
    (
        "Codex CLI\n\nIf no subcommand is specified, options will be forwarded to the interactive CLI",
        "Codex CLI\n\n未指定子命令時，選項會被轉發給互動式CLI。",
    ),
    (
        "Codex is currently experiencing high load.",
        "Codex當前負載較高。",
    ),
    (
        "Codex just got an upgrade. Introducing {0}.",
        "Codex剛剛升級。隆重推出{0}。",
    ),
    ("Compacted context", "已壓縮上下文"),
    ("Completed", "已完成"),
    ("Completed ", "已完成 "),
    ("Completed `{0}`", "已完成 `{0}`"),
    (
        "Config layer stack (lowest precedence first):",
        "配置層棧（優先順序從低到高）：",
    ),
    (
        "Concise, task-focused, and direct.",
        "簡潔、聚焦任務、直接。",
    ),
    ("Config key: ", "配置鍵："),
    ("Connected to your IDE.", "已連線到你的IDE。"),
    ("Contacted", "已聯絡"),
    ("Context {0}% left", "剩餘上下文 {0}%"),
    ("Context {0}% used", "已用上下文 {0}%"),
    ("Continue planning with the model.", "繼續與模型一起規劃。"),
    ("Continue with Luna Reserve", "使用Luna Reserve繼續"),
    (
        "Continue without trusting (hooks won't run)",
        "不信任並繼續（鉤子不會執行）",
    ),
    ("Conversation history is not saved.", "會話歷史未儲存。"),
    (
        "Conversation interrupted - tell the model what to do differently. Something went wrong? Hit `/feedback` to report the issue.",
        "對話已中斷 — 請告訴模型該如何調整。遇到問題？可用`/feedback`反饋。",
    ),
    ("Copied conversation to clipboard", "已複製會話到剪貼簿"),
    (
        "Could not generate a recap. Please try again.",
        "無法生成回顧。請重試。",
    ),
    (
        "Copy the complete Markdown transcript",
        "複製完整的Markdown轉錄",
    ),
    ("Could not read goal image {0}", "無法讀取目標圖片{0}"),
    (
        "Could not read goal objective file {0}",
        "無法讀取目標內容檔案{0}",
    ),
    ("Could not restore session: {0}", "無法恢復會話：{0}"),
    ("Could not write goal file {0}", "無法寫入目標檔案{0}"),
    (
        "Goal objective file reference is too long: {0} characters. Limit: {1}",
        "目標內容檔案引用過長：{0}個字元。上限：{1}",
    ),
    (
        "Goal objective file {0} is not valid UTF-8",
        "目標內容檔案{0}不是有效的UTF-8",
    ),
    (
        "Every configurable action currently has a shortcut.",
        "當前每個可配置操作都已繫結快捷鍵。",
    ),
    (
        "Creating a worktree requires an idle primary session without queued input.",
        "建立工作樹需要主會話空閒且沒有排隊輸入。",
    ),
    (
        "Couldn't set up your sandbox with Administrator permissions",
        "無法以管理員許可權完成沙箱設定",
    ),
    (
        "Current = your current working directory",
        "當前 = 你當前的工作目錄",
    ),
    ("Credits", "額度"),
    ("Current project", "當前專案"),
    (
        "Custom .tmTheme files can be added to the {0} directory.",
        "可以把自定義.tmTheme檔案放到{0}目錄中。",
    ),
    ("Current project: ", "當前專案："),
    ("Custom permissions", "自定義許可權"),
    ("Customize selection", "自定義選擇"),
    ("Debugging tools", "除錯工具。"),
    (
        "Diagnose local Codex installation, config, auth, and runtime health",
        "診斷本地Codex安裝、配置、認證與執行時健康狀況。",
    ),
    (
        "Delete local memory files and rollout summaries.",
        "刪除本地記憶檔案與rollout摘要。",
    ),
    ("Default mode unavailable", "預設模式不可用"),
    ("Detected: ", "識別到："),
    ("Disable terminal pets", "禁用終端寵物"),
    (
        "Disconnected from this task. Any running work continues.",
        "已與此任務斷開連線。正在執行的工作會繼續。",
    ),
    (
        "Disconnected from this task. The current turn was stopped.",
        "已與此任務斷開連線。當前輪次已停止。",
    ),
    (
        "Disconnected from this task. Work may still be running.",
        "已與此任務斷開連線。工作可能仍在執行。",
    ),
    ("Discovery was interrupted", "發現過程被中斷"),
    ("Discuss a code change", "討論程式碼改動"),
    (
        "Discuss a code change (Recommended)",
        "討論程式碼改動（推薦）",
    ),
    (
        "Enable this app to use it for the current request.",
        "啟用此應用以用於當前請求。",
    ),
    ("Dismiss and keep waiting", "關閉並繼續等待"),
    (
        "Earlier messages are available — scroll up to load them",
        "可以載入更早的訊息 — 向上滾動以載入",
    ),
    (
        "Earlier messages unavailable — scroll up to retry",
        "更早的訊息不可用 — 向上滾動以重試",
    ),
    (
        "Do you trust the contents of this directory? Working with untrusted \n                 contents comes with higher risk of prompt injection. Trusting the \n                 directory allows project-local config, hooks, and exec policies to load.",
        "你信任此目錄的內容嗎？處理不受信任的內容會帶來更高的提示詞注入風險。信任該目錄後，專案本地的配置、鉤子與執行策略才會被載入。",
    ),
    ("Each column = 1 week · tallest ", "每列=1周 · 最高 "),
    ("Enable or disable skills.", "啟用或禁用技能。"),
    ("Enable {0}?", "啟用{0}？"),
    ("Entered review mode", "已進入審查模式"),
    ("Enterprise-managed config value", "企業託管配置值"),
    ("Error", "錯誤"),
    ("Error adding directories: {0}", "新增目錄出錯：{0}"),
    ("Error loading configuration: {0}", "載入配置出錯：{0}"),
    ("Execpolicy tooling", "Execpolicy工具。"),
    (
        "Exit Codex and leave the task running",
        "退出Codex並讓任務繼續執行",
    ),
    ("Exited review mode", "已退出審查模式"),
    (
        "Failed to archive current thread: {0}",
        "歸檔當前執行緒失敗：{0}",
    ),
    (
        "Failed to attach to fresh app-server thread: {0}",
        "附加到新的app-server執行緒失敗：{0}",
    ),
    (
        "Failed to attach to resumed app-server thread: {0}",
        "附加到已恢復的app-server執行緒失敗：{0}",
    ),
    (
        "External current time is not available in TUI.",
        "TUI中不支援外部當前時間。",
    ),
    (
        "Experimental feature discovery exceeded 10 pages",
        "實驗性特性發現超過10頁",
    ),
    (
        "Experimental feature page exceeds requested limit",
        "實驗性特性頁面超過請求的上限",
    ),
    (
        "Experimental feature pagination repeated a cursor",
        "實驗性特性分頁重複了同一個遊標",
    ),
    ("Experimental feature request failed", "實驗性特性請求失敗"),
    ("Experimental features", "實驗性特性"),
    (
        "Experimental features are unavailable until startup completes.",
        "啟動完成前無法使用實驗性特性。",
    ),
    ("Export conversation", "匯出對話"),
    ("Extra high", "極高"),
    ("Failed to change: {0}", "切換失敗：{0}"),
    (
        "Failed to name the worktree session: {0}",
        "無法命名工作樹會話：{0}",
    ),
    (
        "Failed to delete current thread: {0}",
        "刪除當前執行緒失敗：{0}",
    ),
    (
        "Failed to load background task settings: {0}",
        "載入後臺任務設定失敗：{0}",
    ),
    ("Failed to name the new session: {0}", "命名新會話失敗：{0}"),
    (
        "Failed to open browser for {0}: {1}",
        "無法為{0}開啟瀏覽器：{1}",
    ),
    ("Failed to open editor: {0}", "開啟編輯器失敗：{0}"),
    (
        "Failed to read new session defaults: {0}",
        "讀取新會話預設值失敗：{0}",
    ),
    (
        "Failed to open this session in the Desktop app: {0}. Install or launch the Desktop app and try again.",
        "無法在桌面應用中開啟此會話：{0}。請安裝或啟動桌面應用後重試。",
    ),
    (
        "Failed to refresh shortcuts: {0}",
        "重新整理快捷鍵失敗：{0}",
    ),
    ("Failed to rename task: {0}", "重新命名任務失敗：{0}"),
    (
        "Failed to save approvals reviewer: {0}",
        "儲存審批覆核者失敗：{0}",
    ),
    (
        "Failed to start the background server: {0}",
        "啟動後臺伺服器失敗：{0}",
    ),
    (
        "Failed to update app config for {0}: {1}",
        "更新{0}的應用配置失敗：{1}",
    ),
    (
        "Failed to resume session from {0}: {1}",
        "從{0}恢復會話失敗：{1}",
    ),
    (
        "Failed to start a fresh session through the app server: {0}",
        "透過app server啟動新會話失敗：{0}",
    ),
    (
        "Failed to switch side conversation: {0}",
        "切換側會話失敗：{0}",
    ),
    (
        "Failed to view thread open elsewhere: {0}",
        "檢視在其他位置開啟的執行緒失敗：{0}",
    ),
    (
        "Failed to save experimental features. Reopen /experimental to check configured values before retrying.",
        "儲存實驗性特性失敗。請重新開啟/experimental確認已配置的值後再重試。",
    ),
    (
        "Failed to retry with a faster model: original turn is unavailable.",
        "用更快的模型重試失敗：原始輪次不可用。",
    ),
    (
        "Failed to retry with a faster model: {0}",
        "用更快的模型重試失敗：{0}",
    ),
    ("Failed to save {0} setting: {1}", "儲存{0}設定失敗：{1}"),
    ("Fast off", "Fast關"),
    ("Fast on", "Fast開"),
    ("Feature discovery was interrupted", "特性發現被中斷"),
    (
        "Features were saved, but readback was interrupted",
        "特性已儲存，但回讀被中斷",
    ),
    (
        "Features were saved, but configured values could not be refreshed: {0}",
        "特性已儲存，但無法重新整理已配置的值：{0}",
    ),
    ("Finished waiting", "等待結束"),
    (
        "Fork a previous interactive session (picker by default; use --last to fork the most recent)",
        "派生之前的互動會話（預設彈出選擇器；用--last派生最近一個）。",
    ),
    (
        "For demanding work using multiple agents · highest usage",
        "適合使用多個代理的高強度工作 · 用量最高",
    ),
    (
        "For difficult problems when quality matters more than speed · higher usage",
        "適合質量優先於速度的難題 · 用量較高",
    ),
    ("Fresh thread with this plan", "用此計劃新建執行緒。"),
    (
        "Start a fresh thread (current context: {0})",
        "新建執行緒（當前上下文：{0}）",
    ),
    (
        "Future messages will include your current IDE selection and open tabs.",
        "後續訊息會包含你當前的IDE選中內容與開啟的標籤頁。",
    ),
    ("Generate memories", "生成記憶"),
    (
        "Generate memories from the following threads. Current thread included.",
        "從以下執行緒生成記憶。包含當前執行緒。",
    ),
    ("Generate shell completion scripts", "生成shell補全指令碼。"),
    ("Generated an image", "已生成影象"),
    (
        "Giving this request a little extra thought",
        "正在為這個請求多想一會兒",
    ),
    (
        "Goal budget reached - the turn was stopped.",
        "已達目標預算 — 本輪已停止。",
    ),
    ("Goal objective must not be empty.", "目標內容不能為空。"),
    (
        "Ignored invalid status line {0}: {1}.",
        "已忽略無效的狀態行{0}：{1}。",
    ),
    (
        "Ignored invalid terminal title {0}: {1}.",
        "已忽略無效的終端標題{0}：{1}。",
    ),
    (
        "Hang tight, this may take a few minutes",
        "請稍候，這可能需要幾分鐘",
    ),
    (
        "Hooks can run outside the sandbox after you trust them.",
        "信任後，鉤子可以在沙箱之外執行。",
    ),
    ("Hooks need review", "鉤子需要審查"),
    ("Hot path energy for fast iteration", "快速迭代的熱路徑能量"),
    (
        "If you'd rather not wait, retry with a faster model. It may be less capable of handling complex requests.",
        "如果你不想等待，可以用更快的模型重試。它處理複雜請求的能力可能較弱。",
    ),
    ("IDE context could not be enabled.", "無法啟用IDE上下文。"),
    ("IDE context is off.", "IDE上下文已關閉。"),
    ("IDE context is on.", "IDE上下文已開啟。"),
    (
        "IDE context was skipped for this message.",
        "本條訊息已跳過IDE上下文。",
    ),
    ("Implement this plan?", "要實施此計劃嗎？"),
    ("Import selected", "匯入所選項"),
    ("Import setup", "匯入配置"),
    ("Import skills from ", "從以下位置匯入技能 "),
    ("Inference: {0} {1} ({2})", "推理：{0} {1}（{2}）"),
    (
        "Input disabled until setup completes.",
        "設定完成前無法輸入。",
    ),
    ("Inspect feature flags", "檢視特性開關。"),
    ("Inspect keypresses", "檢查按鍵"),
    (
        "Inspect keypresses from your terminal.",
        "檢查終端發來的按鍵。",
    ),
    (
        "Inspect or migrate legacy local sessions to paginated thread history",
        "檢查舊版本地會話，或將其遷移為分頁執行緒歷史。",
    ),
    (
        "Install this app in your browser, then return here.",
        "在瀏覽器中安裝此應用，然後返回此處。",
    ),
    ("Interacted with ", "已與 "),
    ("Interacted with `{0}`", "已與 `{0}` 互動"),
    ("Interacted with background terminal", "已與後臺終端互動"),
    (
        "Interacted with background terminal: {0}",
        "已與後臺終端互動：{0}",
    ),
    ("Interrupted", "已中斷"),
    ("Interrupted ", "已中斷 "),
    ("Interrupted `{0}`", "已中斷 `{0}`"),
    (
        "Job: running/completed/failed/expired; Run/Experiment: succeeded/failed/unknown (Recommended when triaging long-running background work and status transitions)",
        "Job：running/completed/failed/expired；Run/Experiment：succeeded/failed/unknown（排查長時間後臺任務與狀態變化時推薦）",
    ),
    (
        "Invalid `{0}` = `{1}`. Use a single key such as `ctrl-a` or a two-stroke chord such as `ctrl-x ctrl-t`.",
        "`{0}` = `{1}` 無效。請使用單個按鍵（如 `ctrl-a`）或兩段和絃（如 `ctrl-x ctrl-t`）。",
    ),
    (
        "Invalid `{0}` = `{1}`: `backspace` is reserved for editing task input.",
        "`{0}` = `{1}` 無效：`backspace` 保留用於編輯任務輸入。",
    ),
    (
        "Invalid `{0}` = `{1}`: a chord prefix outside Vim must use ctrl, alt, or a non-character key so ordinary text input is not intercepted.",
        "`{0}` = `{1}` 無效：Vim之外的和絃字首必須使用ctrl、alt或非字元鍵，以免攔截普通文字輸入。",
    ),
    (
        "Invalid `{0}` = `{1}`: a ctrl-alt character prefix may be AltGr text input on Windows. Choose a different chord and retry.",
        "`{0}` = `{1}` 無效：ctrl-alt字元字首在Windows上可能是AltGr文字輸入。請換一組和絃後重試。",
    ),
    (
        "Invalid `{0}` = `{1}`. Use values like `ctrl-a`, `shift-enter`, or `page-down`. See the Codex keymap documentation for supported actions and examples.",
        "`{0}` = `{1}` 無效。請使用`ctrl-a`、`shift-enter`或`page-down`這類取值。支援的動作與示例見Codex快捷鍵文件。",
    ),
    (
        "Join the OpenAI community Discord: http://discord.gg/openai",
        "加入OpenAI社群Discord：http://discord.gg/openai",
    ),
    ("Keep waiting", "繼續等待"),
    ("Keep {0} disabled.", "保持禁用{0}。"),
    ("Keymap", "快捷鍵"),
    (
        "No configurable actions are available in this group.",
        "此分組沒有可配置的操作。",
    ),
    (
        "Launch the Desktop app (opens the app installer if missing)",
        "啟動桌面應用（缺失時開啟安裝程式）。",
    ),
    ("Keypress Inspector", "按鍵檢查器"),
    ("LM Studio", "LM Studio"),
    ("Last 30 days of chats", "最近30天的聊天"),
    ("Last message", "最後一條訊息"),
    ("Latest activity", "最新活動"),
    ("Layer value", "層值"),
    (
        "Legacy command approval requests are not available in TUI yet.",
        "TUI中尚不支援舊版命令審批請求。",
    ),
    (
        "Legacy patch approval requests are not available in TUI yet.",
        "TUI中尚不支援舊版補丁審批請求。",
    ),
    (
        "Learn more <https://developers.openai.com/codex/windows>",
        "瞭解更多 <https://developers.openai.com/codex/windows>",
    ),
    ("Learn more: ", "瞭解更多： "),
    ("List skills", "列出技能"),
    ("Loading MCP inventory", "正在載入MCP清單"),
    ("Loading MCP inventory...", "正在載入MCP清單…"),
    ("Loading earlier messages...", "正在載入更早的訊息…"),
    ("Loading preview...", "正在載入預覽…"),
    ("Loading server experiments…", "正在載入伺服器實驗…"),
    ("Loading {0} plugins.", "正在載入{0}外掛。"),
    ("Loading {0} plugins...", "正在載入{0}外掛…"),
    (
        "Local LM Studio server (default port 1234)",
        "本地LM Studio伺服器（預設埠1234）",
    ),
    (
        "Local Ollama server (Responses API, default port 11434)",
        "本地Ollama伺服器（Responses API，預設埠11434）",
    ),
    ("Local tools: {0} {1} ({2})", "本地工具：{0} {1}（{2}）"),
    ("Longest task", "最長任務"),
    ("M Studio", "M Studio"),
    ("MCP Tools", "MCP工具"),
    ("MCP server", "MCP伺服器"),
    ("MCP servers", "MCP伺服器"),
    ("MCP {0} on {1}", "在{1}上呼叫MCP {0}"),
    ("MDM value", "MDM值"),
    (
        "Managed worktrees are only supported for local sessions.",
        "託管工作樹僅支援本地會話。",
    ),
    ("Manage Codex plugins", "管理Codex外掛。"),
    (
        "Manage external MCP servers for Codex",
        "管理Codex的外部MCP伺服器。",
    ),
    ("Manage login", "管理登入。"),
    (
        "Message too long; limit {0} characters",
        "訊息過長；上限{0}個字元",
    ),
    (
        "Move up/down to live preview themes",
        "上下移動即可實時預覽主題",
    ),
    (
        "Messages to be submitted after next tool call",
        "將在下次工具呼叫後提交的訊息",
    ),
    (
        "Messages to be submitted at end of turn",
        "將在本輪結束時提交的訊息",
    ),
    ("Monthly credit limit", "每月額度限額"),
    ("Needs attention", "需要關注"),
    ("Needs input", "需要輸入"),
    ("New chat", "新建聊天"),
    ("New task", "新建任務"),
    (
        "No action is required. Codex will keep waiting, and this menu will close when the response is ready.",
        "無需任何操作。Codex會繼續等待，響應就緒後此選單會自動關閉。",
    ),
    (
        "No active session found matching '{0}'.",
        "找不到與'{0}'匹配的活動會話。",
    ),
    ("No agents completed yet", "還沒有代理完成"),
    ("No approved plan available", "沒有已批准的計劃"),
    ("No changes", "無變更"),
    ("No personality instructions.", "無個性指令。"),
    ("No pet will be shown.", "不會顯示寵物。"),
    ("No questions", "沒有問題"),
    ("No recent activity yet.", "還沒有最近活動。"),
    (
        "No saved session found with ID {0}. Run `codex {1}` without an ID to choose from existing sessions.",
        "找不到ID為{0}的已儲存會話。可執行 `codex {1}`（不帶ID）從現有會話中選擇。",
    ),
    ("No server experiments available.", "沒有可用的伺服器實驗。"),
    ("No shortcuts in this group", "此分組沒有快捷鍵"),
    ("No skills available.", "沒有可用的技能。"),
    ("No transcript content available", "沒有可顯示的轉錄內容"),
    ("No unbound shortcuts", "沒有未繫結的快捷鍵"),
    (
        "Open a live inspector that shows the detected key, config key, and matching actions.",
        "開啟實時檢查器，顯示識別到的按鍵、配置鍵名與匹配的操作。",
    ),
    (
        "Press Enter to start. Then press any key to inspect it; Ctrl+C exits.",
        "按Enter開始。之後按任意鍵即可檢查；Ctrl+C退出。",
    ),
    (
        "See the key Codex detects and any shortcuts assigned to it.",
        "檢視Codex識別到的按鍵及其繫結的快捷鍵。",
    ),
    ("No, quit", "不，退出"),
    ("No, stay in Plan mode", "不，留在計劃模式"),
    ("None of the above", "以上都不是"),
    ("Not found", "未找到"),
    (
        "Note: You’re in a subdirectory of a Git project. Trusting will apply to the repository root: {0}",
        "注意：你位於某個Git專案的子目錄中。信任將應用到倉庫根目錄：{0}",
    ),
    (
        "OSS provider selection was cancelled by user",
        "使用者取消了開源提供方選擇",
    ),
    ("Objective: {0}", "目標：{0}"),
    ("Ollama (Chat)", "Ollama（Chat）"),
    ("Ollama (Responses)", "Ollama（Responses）"),
    (
        "Opened this session in the Desktop app.",
        "已在桌面應用中開啟此會話。",
    ),
    ("Open task to review.", "開啟任務以檢視。"),
    ("Opened {0} in your browser.", "已在瀏覽器中開啟{0}。"),
    ("Option 1", "選項1"),
    ("Option 2", "選項2"),
    ("Option 3", "選項3"),
    ("Or run {0} and select {1}.", "或執行{0}並選擇{1}。"),
    ("Other (write an answer)", "其他（自行作答）"),
    (
        "Paste an image with Ctrl+V to attach it to your next message.",
        "按Ctrl+V貼上圖片，附加到下一條訊息。",
    ),
    ("Pending init", "等待初始化"),
    (
        "Pets are disabled in Zellij. Terminal images don’t stay reliably pane-local in Zellij. Run Codex outside Zellij to use pets.",
        "Zellij中已禁用寵物。終端影象在Zellij中無法可靠地侷限於單個窗格。請在Zellij之外執行Codex以使用寵物。",
    ),
    (
        "Pets are disabled in tmux. Terminal images don’t stay pane-local in tmux and can corrupt scrollback or move between panes. Run Codex outside tmux to use pets.",
        "tmux中已禁用寵物。終端影象在tmux中無法侷限於單個窗格，可能破壞回滾緩衝或在窗格間移動。請在tmux之外執行Codex以使用寵物。",
    ),
    (
        "Pets aren’t available in this terminal. Terminal pets need image support, and this terminal environment doesn’t expose a supported image protocol. Try a terminal with Kitty graphics or Sixel support, or run Codex outside tmux.",
        "此終端不支援寵物。終端寵物需要影象支援，而當前終端環境未提供受支援的影象協議。請改用支援Kitty圖形或Sixel的終端，或在tmux之外執行Codex。",
    ),
    (
        "Pets require iTerm2 3.6 or newer. Upgrade iTerm2 to use terminal pets.",
        "寵物需要iTerm2 3.6或更高版本。請升級iTerm2以使用終端寵物。",
    ),
    (
        "Plugin sharing is disabled for this Codex session. Enable plugin sharing to load shared plugins.",
        "本次Codex會話已禁用外掛共享。請啟用外掛共享以載入共享外掛。",
    ),
    (
        "Permission profile cannot be preserved by /cd.",
        "/cd無法保留許可權配置檔案。",
    ),
    (
        "Permanently delete a saved session by id or session name",
        "按id或會話名永久刪除已儲存的會話。",
    ),
    (
        "Personality selection is disabled until startup completes.",
        "啟動完成前無法選擇個性。",
    ),
    ("Plugin · {0}", "外掛 · {0}"),
    (
        "Press any key to see what Codex receives. Esc is inspected; Ctrl+C closes.",
        "按任意鍵檢視Codex收到的內容。Esc會被檢查；Ctrl+C關閉。",
    ),
    ("Preparing device code login", "正在準備裝置碼登入"),
    (
        "Press Tab to queue a message when a task is running; otherwise it sends immediately (except `!`).",
        "任務執行時按Tab可排隊一條訊息；否則會立即傳送（`!`除外）。",
    ),
    ("Press enter to continue", "按Enter繼續"),
    ("Preview unavailable", "預覽不可用"),
    (
        "Queue a message for an existing session",
        "為現有會話排隊一條訊息。",
    ),
    ("Projects ({0})", "專案（{0}）"),
    ("Question requested", "有新的提問請求"),
    ("Question {0}/{1}", "問題 {0}/{1}"),
    ("Questions {0}/{1} answered", "問題 已答 {0}/{1}"),
    ("Queued follow-up inputs", "排隊中的後續輸入"),
    (
        "Queued message {0} for thread {1}.",
        "已為執行緒{1}排隊訊息{0}。",
    ),
    ("Quiet signal from the void", "來自虛空的安靜訊號"),
    ("Raw event: ", "原始事件："),
    (
        "Raw output mode off: rich transcript rendering restored.",
        "原始輸出模式已關閉：已恢復富文字轉錄渲染。",
    ),
    (
        "Raw output mode on: transcript text is shown for clean terminal selection.",
        "原始輸出模式已開啟：顯示純文字轉錄，便於在終端中乾淨地選取。",
    ),
    ("Ready", "就緒"),
    (
        "Reasoning is already at the highest level ({0}).",
        "推理強度已是最高檔（{0}）。",
    ),
    (
        "Reasoning is already at the lowest level ({0}).",
        "推理強度已是最低檔（{0}）。",
    ),
    (
        "Reasoning shortcuts are disabled until startup completes.",
        "啟動完成前無法使用推理快捷鍵。",
    ),
    (
        "Reasoning shortcuts are unavailable for {0}.",
        "{0}不支援推理快捷鍵。",
    ),
    ("Recent chat sessions", "最近的聊天會話"),
    ("Reconnect: {0}", "重新連線：{0}"),
    (
        "Requested directory or permissions not applied.",
        "請求的目錄或許可權未應用。",
    ),
    (
        "Remove stored authentication credentials",
        "刪除已儲存的認證憑據。",
    ),
    (
        "Resume a previous interactive session (picker by default; use --last to continue the most recent)",
        "恢復之前的互動會話（預設彈出選擇器；用--last繼續最近一個）。",
    ),
    (
        "Removed custom shortcut for `{0}.{1}`.",
        "已刪除 `{0}.{1}` 的自定義快捷鍵。",
    ),
    (
        "Reconnected. No input was resent. Review uncertain submissions before retrying; recovered queues remain paused.",
        "已重新連線。沒有重發任何輸入。重試前請先檢查不確定的提交；恢復的佇列仍處於暫停狀態。",
    ),
    (
        "Reconnecting — agent list is stale",
        "正在重新連線 — 代理列表可能已過期",
    ),
    ("Refactor", "重構"),
    ("Remote ID {0}", "遠端ID {0}"),
    ("Rename › ", "重新命名 › "),
    ("Reset all memories", "重置所有記憶"),
    ("Reset all memories?", "重置所有記憶？"),
    (
        "Respond to the MCP server request to continue.",
        "響應MCP伺服器請求以繼續。",
    ),
    (
        "Respond to the tool suggestion to continue.",
        "響應工具建議以繼續。",
    ),
    ("Responses API inference: {0}", "Responses API推理：{0}"),
    ("Responses API overhead: {0}", "Responses API開銷：{0}"),
    ("Resume another chat", "恢復另一個聊天"),
    ("Resumed an agent", "已恢復一個代理"),
    ("Retry with a faster model", "用更快的模型重試"),
    ("Return to memory settings.", "返回記憶設定。"),
    ("Review a diff", "審查diff"),
    ("Review findings", "檢視發現"),
    ("Review hooks", "檢視鉤子"),
    ("Review selection", "檢視選擇"),
    ("Review the diff", "審查該diff"),
    (
        "Run /review to get a code review of your current changes.",
        "執行/review對當前改動做程式碼審查。",
    ),
    ("Run Codex non-interactively", "以非互動方式執行Codex。"),
    (
        "Run a code review non-interactively",
        "以非互動方式執行程式碼審查。",
    ),
    (
        "Run commands within a Codex-provided sandbox",
        "在Codex提供的沙箱中執行命令。",
    ),
    (
        "Run `codex app` to open the Desktop app (it installs on macOS if needed).",
        "執行 `codex app` 開啟桌面應用（必要時會在macOS上自動安裝）。",
    ),
    ("Run in background", "在後臺執行"),
    ("Run targeted tests", "執行針對性測試"),
    ("Run tests", "執行測試"),
    ("Running", "執行中"),
    (
        "Save on the server for new threads. This thread is unchanged.",
        "在伺服器上儲存以用於新執行緒。當前執行緒不受影響。",
    ),
    (
        "Saving experimental features timed out; the write may still finish. Reopen /experimental to check.",
        "儲存實驗性特性超時；寫入可能仍會完成。請重新開啟/experimental確認。",
    ),
    ("Save conversation", "儲存對話"),
    (
        "Save the complete conversation as Markdown",
        "將完整對話儲存為Markdown",
    ),
    ("Save to file", "儲存到檔案"),
    ("Saving experimental features…", "正在儲存實驗性特性…"),
    (
        "Saving was interrupted. Reopen /experimental to check configured values.",
        "儲存被中斷。請重新開啟/experimental檢查已配置的值。",
    ),
    (
        "Saving… Closing this popup will not cancel the write.",
        "正在儲存…關閉此彈窗不會取消寫入。",
    ),
    ("Search › ", "搜尋 › "),
    (
        "See the Codex keymap documentation for supported actions and examples.",
        "支援的操作與示例見Codex快捷鍵文件。",
    ),
    ("Select Personality", "選擇個性"),
    ("Select Pet", "選擇寵物"),
    ("Select Syntax Theme", "選擇語法主題"),
    ("Select an open-source provider", "選擇開源提供方"),
    ("Select provider?", "選擇提供方？"),
    ("Selected {0} of {1} {2}.", "已選擇{0}/{1}個{2}。"),
    (
        "Selections retained. Save to retry, or cancel to close.",
        "已保留選擇。儲存以重試，或取消以關閉。",
    ),
    ("Sent input to", "已傳送輸入到"),
    ("Sent input to an agent", "已向代理傳送輸入"),
    (
        "Session = latest cwd recorded in the {0} session",
        "會話 = 記錄在{0}次會話中的最新cwd",
    ),
    ("Session ID: {0}", "會話ID：{0}"),
    ("Session archived: {0}", "會話已歸檔：{0}"),
    ("Session runtime:", "會話執行時："),
    (
        "Set up the Codex agent sandbox to protect your files and control network access. Learn more <https://developers.openai.com/codex/windows>",
        "設定Codex代理沙箱，以保護你的檔案並控制網路訪問。瞭解更多 <https://developers.openai.com/codex/windows>",
    ),
    ("Setting up sandbox...", "正在設定沙箱…"),
    (
        "Settings, instructions, integrations, agents, commands, and skills",
        "設定、指令、整合、代理、命令與技能",
    ),
    ("Shell mode", "Shell模式"),
    ("Ship it", "釋出"),
    ("Shutdown", "已停止"),
    (
        "Sign in to ChatGPT, then try loading this section again.",
        "請先登入ChatGPT，然後重新載入此分割槽。",
    ),
    (
        "Sign in with ChatGPT auth; API key auth cannot load remote plugin catalogs.",
        "請用ChatGPT賬號登入；API key認證無法載入遠端外掛目錄。",
    ),
    ("Shutting down...", "正在關閉…"),
    ("Slash commands", "斜槓命令"),
    ("Small green shoots for new ideas", "新想法的嫩綠幼苗"),
    ("Spawned", "已啟動"),
    ("Spawned an agent", "已啟動一個代理"),
    (
        "Start a fresh idea with /new; the previous session stays in history.",
        "用/new開啟新話題；上一次會話仍保留在歷史中。",
    ),
    ("Started", "已啟動"),
    ("Started ", "已啟動 "),
    ("Started `{0}`", "已啟動 `{0}`"),
    (
        "Stop the current turn: run {0}, select this task, and {1}.",
        "停止當前輪次：執行{0}，選擇此任務，然後{1}。",
    ),
    (
        "Stop the current task and exit Codex",
        "停止當前任務並退出Codex",
    ),
    (
        "Stop the current task and stay in Codex",
        "停止當前任務並留在Codex",
    ),
    (
        "Still waiting? If nothing changes when you press a key, your terminal is not sending that key to Codex. Only received keys can be assigned as shortcuts.",
        "還在等待？如果按鍵後沒有任何變化，說明你的終端沒有把該按鍵傳送給Codex。只有接收到的按鍵才能被指定為快捷鍵。",
    ),
    ("Starting MCP servers", "正在啟動MCP伺服器"),
    (
        "Startup did not finish binding a thread to this worktree: {0}\nThe checkout was kept. Inspect it and confirm no session is using it.\nTo remove it, run `git worktree remove <checkout-path>` from the source repository,\nreplacing <checkout-path> with the path above. Do not use --force.",
        "啟動未能把執行緒繫結到該工作樹：{0}\n已保留檢出。請檢查並確認沒有會話在用。\n如需刪除，請在源倉庫執行 `git worktree remove <checkout-path>`，\n把 <checkout-path> 替換為上面的路徑。不要使用 --force。",
    ),
    ("Stop and retry", "停止並重試"),
    ("Stop this attempt and retry?", "停止本次嘗試並重試？"),
    (
        "Stopping all background terminals.",
        "正在停止所有後臺終端。",
    ),
    ("Stream", "流"),
    ("Streams", "流"),
    ("Sub-agents running", "執行中的子代理"),
    (
        "Switch to the matching workspace or ask the sharer for access.",
        "請切換到匹配的工作區，或向共享者申請訪問許可權。",
    ),
    (
        "Submit with {0} unanswered {1}.",
        "仍有{0}個未回答的{1}，仍要提交。",
    ),
    (
        "Switch to Default and start coding.",
        "切換到預設模式並開始編碼。",
    ),
    (
        "Switch models or reasoning effort quickly with /model.",
        "用/model快速切換模型或推理強度。",
    ),
    ("T R A N S C R I P T", "轉錄"),
    ("TBT: {0}", "TBT：{0}"),
    ("TTFT: {0}", "TTFT：{0}"),
    ("Task encountered an error.", "任務遇到錯誤。"),
    ("Task is still running", "任務仍在執行"),
    ("Tasks {0}/{1}", "任務 {0}/{1}"),
    ("Terminal pets disabled", "終端寵物已禁用"),
    (
        "The new worktree is not trusted; run Codex there.",
        "新的工作樹尚未受信任；請在那裡執行Codex。",
    ),
    (
        "The Desktop app is only available on macOS and Windows",
        "桌面應用僅在macOS和Windows上可用",
    ),
    (
        "The initial thread may have been created, but its ID was not received. Nothing was retried. Your prompt is editable; inspect your tasks before relaunching.",
        "初始執行緒可能已建立，但沒有收到它的ID。沒有重試任何操作。你的提示詞仍可編輯；重新啟動前請先檢查任務。",
    ),
    ("The original Codex companion", "最初的Codex夥伴"),
    (
        "The rename target disappeared. Unsubmitted title: {0}",
        "重新命名目標已消失。未提交的標題：{0}",
    ),
    (
        "There is no conversation history to recap.",
        "沒有可供回顧的會話歷史。",
    ),
    (
        "The server did not advertise experimental feature `{0}`",
        "伺服器未宣告實驗性特性 `{0}`",
    ),
    ("Thinking", "思考中"),
    (
        "This directory is not trusted; run Codex there.",
        "此目錄尚未受信任；請在那裡執行Codex。",
    ),
    (
        "This clears local memory files and rollout summaries for the current Codex home.",
        "這將清除當前Codex主目錄的本地記憶檔案與rollout摘要。",
    ),
    (
        "This will stop the current attempt and retry in a new thread. Any file changes or other actions already taken will remain.",
        "這會停止當前嘗試並在新執行緒中重試。已產生的檔案變更或其他操作會保留。",
    ),
    (
        "Tip: Codex can only inspect keys your terminal sends.",
        "提示：Codex只能檢查終端實際傳送的按鍵。",
    ),
    (
        "This conversation is unavailable. Its cached transcript and draft remain here; input is paused. Open the agent picker or return to the parent to continue.",
        "此對話當前不可用。快取的轉錄與草稿仍保留在此；輸入已暫停。請開啟代理選擇器或返回父級以繼續。",
    ),
    ("Thread usage", "執行緒用量"),
    (
        "Tip: press $ to open this list directly.",
        "提示：按$直接開啟此列表。",
    ),
    ("Time: {0}.", "耗時：{0}。"),
    ("To continue this session, run:", "要繼續此會話，請執行："),
    ("Token usage so far:", "目前Token用量："),
    ("Token usage:", "Token用量："),
    (
        "Token usage: total={0} input={1}{2} output={3}{4}",
        "Token用量：總計={0} 輸入={1}{2} 輸出={3}{4}",
    ),
    ("Tokens: {0}/{1}.", "Token：{0}/{1}。"),
    ("Tool {0}", "工具{0}"),
    (
        "Try again later; local plugin functionality is still available.",
        "請稍後重試；本地外掛功能仍可使用。",
    ),
    (
        "Turn skills on or off. Your changes are saved automatically.",
        "開啟或關閉技能。更改會自動儲存。",
    ),
    ("Tools & setup", "工具與配置"),
    ("Trust all and continue", "全部信任並繼續"),
    ("Trusting hooks...", "正在信任鉤子…"),
    ("Try new model", "試用新模型"),
    ("Try setting up admin sandbox again", "重試管理員沙箱設定"),
    (
        "Try the **Desktop app** on Linux: install it from https://learn.chatgpt.com/docs/linux/linux-app and run 'chatgpt'.",
        "在Linux上試試**桌面應用**：從https://learn.chatgpt.com/docs/linux/linux-app安裝後執行'chatgpt'。",
    ),
    (
        "Try the **Desktop app**. Run 'codex app' or visit https://chatgpt.com/codex?app-landing-page=true",
        "試試**桌面應用**。執行'codex app'，或訪問https://chatgpt.com/codex?app-landing-page=true",
    ),
    (
        "Type / to open the command popup; Tab autocompletes slash commands.",
        "輸入/開啟命令彈窗；Tab可補全斜槓命令。",
    ),
    ("Type a filename and press Enter", "輸入檔名後按Enter"),
    ("Type to filter pets...", "輸入以篩選寵物…"),
    ("Type to filter themes...", "輸入以篩選主題…"),
    ("Type to search shortcuts", "輸入以搜尋快捷鍵"),
    (
        "Unarchive a saved session by id or session name",
        "按id或會話名取消歸檔已儲存的會話。",
    ),
    ("Type to search skills", "輸入以搜尋技能"),
    ("Type your answer", "輸入你的回答"),
    ("Unbound ({0})", "未繫結（{0}）"),
    (
        "Update Codex, then try opening the shared plugin again.",
        "請更新Codex，然後重試開啟該共享外掛。",
    ),
    (
        "Update Codex to the latest version",
        "把Codex更新到最新版本。",
    ),
    ("Updated {0} file(s)", "已更新{0}個檔案"),
    (
        "Usage limit reached. You've reached your usage limit. Increase your limits to continue using codex.",
        "已達用量限額。你的用量限額已用盡。要提高限額才能繼續使用codex。",
    ),
    ("Usage: /ide [on|off|status]", "用法：/ide [on|off|status]"),
    ("Use ", "用 "),
    (
        "Your existing setup will not be changed.",
        "你現有的配置不會被修改。",
    ),
    (
        "Use Codex with non-admin sandbox",
        "在非管理員沙箱中使用Codex",
    ),
    (
        "Use /compact when the conversation gets long to summarize history and free up context.",
        "對話變長時用/compact彙總歷史並釋放上下文。",
    ),
    (
        "Use /copy or press Ctrl+O to copy the latest agent response as Markdown.",
        "用/copy或按Ctrl+O把最新回覆複製為Markdown。",
    ),
    (
        "Use /feedback to send logs to the maintainers when something looks off.",
        "發現異常時用/feedback把日誌發給維護者。",
    ),
    (
        "Use /fork to branch the current chat into a new thread.",
        "用/fork把當前聊天派生成新執行緒。",
    ),
    (
        "Use /init to create an AGENTS.md with project-specific guidance.",
        "用/init建立帶專案專屬指引的AGENTS.md。",
    ),
    (
        "Use /mcp to list configured MCP tools.",
        "用/mcp列出已配置的MCP工具。",
    ),
    (
        "Use /permissions to control when Codex asks for confirmation.",
        "用/permissions控制Codex何時請求確認。",
    ),
    (
        "Use /personality to customize how Codex communicates.",
        "用/personality定製Codex的溝通風格。",
    ),
    (
        "Use /rename to rename your threads for easier thread resuming.",
        "用/rename重新命名執行緒，便於後續恢復。",
    ),
    (
        "Use /side to start a side conversation in a temporary fork without polluting the main thread.",
        "用/side在臨時分叉中開啟側會話，不汙染主執行緒。",
    ),
    (
        "Use /skills to list available skills or ask Codex to use one.",
        "用/skills列出可用技能，或讓Codex使用某個技能。",
    ),
    (
        "Use /status to see the current model, approvals, and token usage.",
        "用/status檢視當前模型、審批設定與Token用量。",
    ),
    (
        "Use /statusline to configure which items appear in the status line.",
        "用/statusline配置狀態行顯示哪些項。",
    ),
    ("Use Detailed Hint A (Recommended)", "使用詳細提示A（推薦）"),
    ("Use Detailed Hint B", "使用詳細提示B"),
    ("Use Detailed Hint C", "使用詳細提示C"),
    ("Use current directory ({0})", "使用當前目錄（{0}）"),
    ("Use existing model", "沿用現有模型"),
    ("Use memories", "使用記憶"),
    (
        "Use memories in the following threads. Applied at next thread.",
        "在以下執行緒中使用記憶。在下一個執行緒生效。",
    ),
    (
        "Use non-admin sandbox (higher risk if prompt injected)",
        "使用非管理員沙箱（提示詞注入時風險更高）",
    ),
    ("Use session directory ({0})", "使用會話目錄（{0}）"),
    (
        "Use the OpenAI docs MCP for API questions; enable it with `codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp`.",
        "API問題可用OpenAI文件MCP；用`codex mcp add openaiDeveloperDocs --url https://developers.openai.com/mcp`啟用。",
    ),
    ("Viewed {0}", "已檢視{0}"),
    (
        "Viewing sub-agent — direct input is disabled",
        "正在檢視子代理 — 已禁用直接輸入",
    ),
    ("Vim mode disabled.", "Vim模式已關閉。"),
    ("Vim mode enabled.", "Vim模式已開啟。"),
    (
        "Visit the Codex community forum: https://community.openai.com/c/codex/37",
        "訪問Codex社群論壇：https://community.openai.com/c/codex/37",
    ),
    ("Waited for an agent", "等待了一個代理"),
    ("Waited for background terminal", "已等待後臺終端"),
    ("Waited for background terminal: {0}", "已等待後臺終端：{0}"),
    ("Waiting for", "正在等待"),
    ("Waiting for a keypress...", "等待按鍵…"),
    ("Waiting for agents", "正在等待代理"),
    ("Waiting for approval.", "正在等待批准。"),
    ("Waiting for your response.", "正在等待你的回覆。"),
    ("Waiting for {0} agents", "正在等待{0}個代理"),
    (
        "We couldn't complete the world-writable scan, so protections cannot be verified. ",
        "無法完成全域性可寫掃描，因此無法驗證防護。 ",
    ),
    (
        "We couldn’t confirm the agent was acting safely and following your instructions. To continue working, start or resume another chat.",
        "我們無法確認代理的行動是否安全、是否遵循你的指令。要繼續工作，請新建或恢復另一個聊天。",
    ),
    (
        "We couldn’t confirm the agent was interpreting your instructions correctly. Review what we detected before deciding to continue.",
        "我們無法確認代理是否正確理解了你的指令。請先檢視我們檢測到的內容，再決定是否繼續。",
    ),
    (
        "Warm, collaborative, and helpful.",
        "溫和、樂於協作、有幫助。",
    ),
    (
        "We recommend switching from {0} to {1}.",
        "我們建議從{0}切換到{1}。",
    ),
    ("Web search: {0}", "網路搜尋：{0}"),
    (
        "WebSocket: {0} events send ({1})",
        "WebSocket：傳送{0}個事件（{1}）",
    ),
    ("WebSocket timing: {0}", "WebSocket計時：{0}"),
    ("Welcome to ", "歡迎使用 "),
    (
        "When the composer is empty, press Esc to step back and edit your last message; Enter confirms.",
        "輸入框為空時，按Esc回退編輯上一條訊息；Enter確認。",
    ),
    ("Worked for {0}", "耗時 {0}"),
    ("Working directory changed to: {0}", "工作目錄已切換到：{0}"),
    ("Yes, clear context and implement", "是，清空上下文並實施"),
    ("Yes, continue", "是，繼續"),
    ("Yes, enable", "是，啟用"),
    ("Yes, implement this plan", "是，實施此計劃"),
    (
        "You can still use Codex in a non-admin sandbox. It carries greater risk if prompt injected.",
        "你仍可在非管理員沙箱中使用Codex。提示詞注入時風險更高。",
    ),
    (
        "Your organization requires the default Codex agent sandbox to continue. Set it up to protect your files and control network access.",
        "你的組織要求繼續使用預設的Codex代理沙箱。請完成設定以保護你的檔案並控制網路訪問。",
    ),
    (
        "Your organization requires the default sandbox before Codex can continue.",
        "你的組織要求先使用預設沙箱，Codex才能繼續。",
    ),
    (
        "Your message will be sent again using {0}, which may be less capable on complex tasks.",
        "你的訊息將改用{0}重新傳送，它在複雜任務上的能力可能較弱。",
    ),
    ("You are in ", "你當前位於 "),
    (
        "Your included usage is exhausted. Choose an option below to continue.",
        "你的包含用量已用盡。請從下方選擇一個選項繼續。",
    ),
    (
        "You're out of credits. Your workspace is out of credits. Add credits to continue using Codex.",
        "你的額度已用盡。你的工作區額度已用盡。請充值額度以繼續使用Codex。",
    ),
    (
        "You can continue using {0} if you prefer.",
        "如果你願意，也可以繼續使用{0}。",
    ),
    (
        "You can resume a previous conversation by running `codex resume`",
        "可執行`codex resume`恢復之前的對話",
    ),
    (
        "You can run any shell command from Codex using `!` (e.g. `!ls`)",
        "在Codex裡可用`!`執行任意shell命令（如`!ls`）",
    ),
    ("[ ! ] Action Required", "[ ! ] 需要操作"),
    ("[ . ] Action Required", "[ . ] 需要操作"),
    (
        "a user-configured MCP server already owns the codex_tui namespace",
        "使用者配置的MCP伺服器已佔用codex_tui名稱空間",
    ),
    (
        "animation {0} fallback {1} does not exist",
        "動畫{0}的回退{1}不存在",
    ),
    (
        "animation {0} fps must be finite and between 0 and {1}, got {2}",
        "動畫{0}的fps必須是有限值且介於0與{1}之間，實際為{2}",
    ),
    (
        "animation {0} references sprite index {1}, but pet has {2} frames",
        "動畫{0}引用了精靈索引{1}，但寵物只有{2}幀",
    ),
    (
        "[EXPERIMENTAL] Browse tasks from Codex Cloud and apply changes locally",
        "[實驗性]瀏覽Codex Cloud的任務並在本地應用更改。",
    ),
    (
        "[EXPERIMENTAL] Run the standalone exec-server service",
        "[實驗性]執行獨立的exec-server服務。",
    ),
    (
        "[experimental] Manage the app-server daemon with remote control enabled",
        "[實驗性]在啟用遠端控制的情況下管理app-server守護程序。",
    ),
    (
        "[experimental] Run the app server or related tooling",
        "[實驗性]執行app server或相關工具。",
    ),
    ("[Pasted Content {0} chars]", "[貼上內容 {0} 字元]"),
    (
        "[tui.keymap] in ~/.codex/config.toml lets you rebind supported shortcuts.",
        "~/.codex/config.toml中的[tui.keymap]可重新繫結受支援的快捷鍵。",
    ),
    ("[{0}] {1} · modified", "[{0}] {1} · 已修改"),
    ("[{0}] {1} · new", "[{0}] {1} · 新增"),
    (
        "[… {0} lines] ctrl + a view all",
        "[… {0} 行] ctrl + a檢視全部",
    ),
    ("\n\nStartup warnings:\n{0}", "\n\n啟動警告：\n{0}"),
    (
        "`tui.resume_cwd = \"current\"` requires `--cd` when using a remote workspace",
        "在遠端工作區中使用 `tui.resume_cwd = \"current\"` 需要 `--cd`",
    ),
    (
        "`--worktree` is only supported for local sessions",
        "`--worktree`僅支援本地會話",
    ),
    (
        "`--worktree` cannot create a checkout from an explicitly untrusted source",
        "`--worktree`無法從未經信任的來源建立檢出",
    ),
    (
        "`--worktree` requires the worktrees feature; enable it with `--enable worktrees`",
        "`--worktree`需要worktrees特性；請用`--enable worktrees`啟用",
    ),
    ("`open {0}` exited with {1}", "`open {0}` 以{1}退出"),
    ("a patch touching ", "涉及以下內容的補丁 "),
    ("agent tool: {0} · {1}", "代理工具：{0} · {1}"),
    ("answer: ******", "回答：******"),
    ("answer: {0}", "回答：{0}"),
    ("app server unavailable", "app-server不可用"),
    (
        "app-server session could not be restored",
        "無法恢復app-server會話",
    ),
    ("apply_patch touching {0}", "apply_patch涉及{0}"),
    ("apply_patch touching {0} files", "apply_patch涉及{0}個檔案"),
    ("authentication required", "需要認證"),
    (
        "base64 payload is not valid UTF-8",
        "base64載荷不是有效的UTF-8",
    ),
    ("auto-resolves in {0}", "{0}後自動處理"),
    ("back", "返回"),
    ("call", "次呼叫"),
    ("calls", "次呼叫"),
    ("cancelled", "已取消"),
    (
        "cannot queue through an embedded app server while a local app-server daemon is running; remove configuration overrides or use --remote",
        "本地app-server守護程序執行時，無法透過嵌入式應用伺服器排隊；請移除配置覆蓋，或使用--remote",
    ),
    ("canonicalize {0}", "規範化{0}"),
    ("chat session", "聊天會話"),
    (
        "clipboard image paste is unsupported on Android",
        "Android上不支援從剪貼簿貼上圖片",
    ),
    ("codex could access {0}", "Codex可能會訪問{0}"),
    (
        "codex could call MCP tool {0}.{1}",
        "Codex可能會呼叫MCP工具{0}.{1}",
    ),
    ("codex could request permissions", "Codex可能會請求許可權"),
    ("codex could {0}", "Codex可能會{0}"),
    ("codex to access {0}", "Codex將要訪問{0}"),
    (
        "codex to call MCP tool {0}.{1}",
        "Codex將要呼叫MCP工具{0}.{1}",
    ),
    ("codex to request permissions", "Codex將要請求許可權"),
    ("codex to {0}", "Codex將要{0}"),
    ("connected", "已連線"),
    ("context compacted", "上下文已壓縮"),
    ("tokens used", "已用token"),
    (" succeeded", " 執行成功"),
    (" exited ", " 退出碼 "),
    (" declined", " 已拒絕"),
    (" in progress", " 進行中"),
    ("started", "已開始"),
    ("web search:", "網頁搜尋："),
    ("apply patch", "應用補丁"),
    ("warning:", "警告："),
    ("ERROR:", "錯誤："),
    ("model rerouted:", "模型已改道："),
    ("turn interrupted", "回合已中斷"),
    (
        "Heads up: Long threads and multiple compactions can cause the model to be less accurate. Start a new thread when possible to keep threads small and targeted.",
        "注意：長會話與多次壓縮會讓模型準確性下降。條件允許時請新建會話，讓會話保持小而聚焦。",
    ),
    (
        "Stop hook requested continuation without a prompt; ignoring the block.",
        "Stop鉤子請求繼續但未提供提示詞；已忽略該阻斷。",
    ),
    (
        "Invalid image in your last message. Please remove it and try again.",
        "上一條訊息中的圖片無效。請移除後重試。",
    ),
    (
        "Falling back from WebSockets to HTTPS transport. {0}",
        "正在從WebSockets回退到HTTPS傳輸。{0}",
    ),
    (
        "Failed to save the conversation transcript; Codex will continue retrying. Error: {0}",
        "儲存會話轉錄失敗；Codex將繼續重試。錯誤：{0}",
    ),
    (
        "Model metadata for `{0}` not found. Defaulting to fallback metadata; this can degrade performance and cause issues.",
        "未找到模型 `{0}` 的後設資料。將使用回退後設資料；這可能降低效能並引發問題。",
    ),
    (
        "Automatic approval review rejected too many approval requests for this turn ({0} consecutive, {1} in the last {2} reviews); interrupting the turn.",
        "本輪自動審批審查拒絕了過多審批請求（連續 {0} 次，最近 {2} 次審查中有 {1} 次）；正在中斷該回合。",
    ),
    (
        "GitHub MCP does not support OAuth. Log in by adding a personal access token (https://github.com/settings/personal-access-tokens) to your environment and config.toml:\n[mcp_servers.{0}]\nbearer_token_env_var = CODEX_GITHUB_PERSONAL_ACCESS_TOKEN",
        "GitHub MCP不支援OAuth。請在環境變數與config.toml中加入個人訪問令牌（https://github.com/settings/personal-access-tokens）後登入：\n[mcp_servers.{0}]\nbearer_token_env_var = CODEX_GITHUB_PERSONAL_ACCESS_TOKEN",
    ),
    (
        "Use your client's MCP OAuth sign-in flow.",
        "請使用你客戶端的MCP OAuth登入流程。",
    ),
    ("Run `codex mcp login {0}`.", "執行 `codex mcp login {0}`。"),
    ("requires OAuth reauthentication", "需要重新進行OAuth認證"),
    ("is not logged in", "尚未登入"),
    ("The {0} MCP server {1}. {2}", "{0} MCP伺服器{1}。{2}"),
    (
        "MCP client for `{0}` timed out after {1} seconds. Add or adjust `startup_timeout_sec` in your config.toml:\n[mcp_servers.{2}]\nstartup_timeout_sec = XX",
        "`{0}` 的MCP客戶端在 {1} 秒後超時。請在config.toml中新增或調整 `startup_timeout_sec`：\n[mcp_servers.{2}]\nstartup_timeout_sec = XX",
    ),
    (
        "MCP client for `{0}` failed to start: {1}",
        "`{0}` 的MCP客戶端啟動失敗：{1}",
    ),
    ("Approve app tool call?", "批准應用工具呼叫？"),
    ("Cancel this tool call.", "取消該工具呼叫。"),
    ("this app", "此應用"),
    ("the {0} MCP server", "{0} MCP伺服器"),
    ("Allow {0} to run tool \"{1}\"?", "允許{0}執行工具“{1}”嗎？"),
    ("Install MCP servers?", "安裝MCP伺服器？"),
    (
        "The following MCP servers are required by the selected skills but are not installed yet: {0}. Install them now?",
        "所選技能需要以下MCP伺服器，但它們尚未安裝：{0}。現在安裝嗎？",
    ),
    (
        "Install and enable the missing MCP servers in your global config.",
        "在全域性配置中安裝並啟用缺失的MCP伺服器。",
    ),
    (
        "Skip installation for now and do not show again for these MCP servers in this session.",
        "暫時跳過安裝，並在本次會話中不再為這些MCP伺服器提示。",
    ),
    ("Failed to initialize session: {0}", "會話初始化失敗：{0}"),
    (
        "required MCP server `{0}` was not initialized",
        "必需的MCP伺服器 `{0}` 未能初始化",
    ),
    (
        "Optionally, add details in notes (tab).",
        "可在備註中補充細節（tab）。",
    ),
    ("could not encode image: {0}", "無法編碼影象：{0}"),
    ("could not write {0}: {1}", "無法寫入{0}：{1}"),
    ("create {0}", "建立{0}"),
    (
        "ctrl+c clear input, then quit · actions paused until the list is refreshed",
        "ctrl+c清空輸入後再退出 · 列表重新整理前操作暫停",
    ),
    (
        "ctrl + p / ctrl + n change question",
        "ctrl+p/ctrl+n切換問題",
    ),
    ("creator account {0}", "建立者賬號 {0}"),
    ("creator {0}", "建立者 {0}"),
    ("creator {0} ({1})", "建立者 {0}（{1}）"),
    ("customize shortcuts with ", "自定義快捷鍵："),
    ("IDE context", "IDE上下文"),
    (" for commands", " 檢視命令"),
    (" for shell commands", " 檢視Shell命令"),
    (" for newline", " 換行"),
    (" for file paths", " 檔案路徑"),
    (" to paste images", " 貼上圖片"),
    (" to edit in external editor", " 用外部編輯器編輯"),
    (" search history", " 搜尋歷史"),
    (" to view transcript", " 檢視記錄"),
    (" to change mode", " 切換模式"),
    (" reasoning down", " 降低推理強度"),
    (" reasoning up", " 提高推理強度"),
    ("day", "天"),
    ("deny read {0}", "禁止讀取 {0}"),
    ("directory:", "目錄："),
    ("directory: {0}", "目錄：{0}"),
    ("disabled", "已禁用"),
    ("download pet asset from {0}", "從{0}下載寵物資源"),
    ("editor command is empty", "編輯器命令為空"),
    ("editor directory has no parent", "編輯器目錄沒有父目錄"),
    (
        "editor directory must not be writable",
        "編輯器目錄必須不可寫",
    ),
    (
        "editor directory must not contain symbolic links",
        "編輯器目錄不能包含符號連結",
    ),
    ("editor exited with status {0}", "編輯器以{0}退出"),
    ("embedded resource: {0}", "嵌入式資源：{0}"),
    ("enabled", "已啟用"),
    ("event", "事件"),
    ("events", "事件"),
    ("failed", "失敗"),
    ("failed to archive session", "無法歸檔會話"),
    (
        "failed to connect to remote app server",
        "無法連線到遠端app-server",
    ),
    (
        "failed to bind managed worktree thread",
        "無法繫結託管工作樹執行緒",
    ),
    ("failed to delete session", "無法刪除會話"),
    (
        "failed to encode thread/inject_items payload",
        "無法編碼thread/inject_items載荷",
    ),
    (
        "failed to list loaded threads from app server",
        "無法從應用伺服器列出已載入的執行緒",
    ),
    (
        "failed to serialize Auto Review denial event",
        "無法序列化Auto Review拒絕事件",
    ),
    ("failed to invoke `open`: {0}", "呼叫 `open` 失敗：{0}"),
    (
        "failed to launch the Desktop app through PowerShell with {0}",
        "透過PowerShell啟動桌面應用失敗，返回{0}",
    ),
    (
        "failed to launch the Desktop app through PowerShell: {0}",
        "透過PowerShell啟動桌面應用失敗：{0}",
    ),
    ("failed to load skills on startup", "啟動時載入技能失敗"),
    (
        "failed to localize requested filesystem paths: {0}",
        "本地化所請求的檔案系統路徑失敗：{0}",
    ),
    (
        "failed to reject app-server request: {0}",
        "拒絕應用伺服器請求失敗：{0}",
    ),
    (
        "failed to serialize MCP elicitation response: {0}",
        "序列化MCP elicitation響應失敗：{0}",
    ),
    (
        "failed to serialize command execution approval response: {0}",
        "序列化命令執行審批響應失敗：{0}",
    ),
    (
        "failed to serialize file change approval response: {0}",
        "序列化檔案變更審批響應失敗：{0}",
    ),
    (
        "failed to serialize permissions approval response: {0}",
        "序列化許可權審批響應失敗：{0}",
    ),
    (
        "failed to serialize request_user_input response: {0}",
        "序列化request_user_input響應失敗：{0}",
    ),
    ("failed to queue session message", "無法排隊會話訊息"),
    ("failed to resolve CODEX_HOME", "無法解析CODEX_HOME"),
    (
        "failed to start embedded app server",
        "無法啟動嵌入式app-server",
    ),
    (
        "failed to restore terminal. Run `reset` or restart your terminal to recover: {0}",
        "恢復終端失敗。請執行 `reset` 或重啟終端以恢復：{0}",
    ),
    ("failed to unarchive session", "無法取消歸檔會話"),
    (
        "file changes: {0} · {1} changes",
        "檔案變更：{0} · {1} 處變更",
    ),
    (
        "failed to write config.toml: {0}",
        "寫入config.toml失敗：{0}",
    ),
    ("forked_from_id is invalid: {0}", "forked_from_id無效：{0}"),
    (
        "frame path has no valid file stem",
        "幀路徑沒有有效的檔名主幹",
    ),
    ("glob `{0}`", "通配 `{0}`"),
    ("group", "分組"),
    ("hook prompt: ", "鉤子提示："),
    ("hour", "小時"),
    ("image: {0}", "影象：{0}"),
    (
        "in-process sessions have no connection to restore",
        "程序內會話沒有可恢復的連線",
    ),
    ("install {0}", "安裝{0}"),
    (
        "interrupted turn {0} is missing from the source thread",
        "被中斷的輪次{0}不在源執行緒中",
    ),
    (
        "interrupted turn {0} is no longer the latest turn",
        "被中斷的輪次{0}已不是最新輪次",
    ),
    (
        "interrupted turn {0} is still in progress",
        "被中斷的輪次{0}仍在進行中",
    ),
    ("interrupted with {0} unanswered", "已中斷，還有{0}個未回答"),
    (
        "invalid remote address `{0}`; expected `ws://host:port`, `wss://host:port`, `unix://`, or `unix://PATH`",
        "遠端地址 `{0}` 無效；應為 `ws://host:port`、`wss://host:port`、`unix://` 或 `unix://PATH`",
    ),
    ("invalid keymap configuration: {0}", "快捷鍵配置無效：{0}"),
    ("invalid visualization fragment", "視覺化片段無效"),
    ("io error: {0}", "IO錯誤：{0}"),
    ("item", "條目"),
    ("items", "條目"),
    (
        "managed MCP requirements do not permit the TUI task-tools server",
        "託管MCP要求不允許TUI task-tools伺服器",
    ),
    ("join pet load task", "等待寵物載入任務"),
    (
        "join pet spritesheet cache validation task",
        "等待寵物精靈表快取校驗任務",
    ),
    (
        "join pet spritesheet install task",
        "等待寵物精靈表安裝任務",
    ),
    ("link: {0}", "連結：{0}"),
    ("local app-server daemon", "本地app-server守護程序"),
    ("local {0}", "本地 {0}"),
    ("main prompt", "主提示"),
    ("mcp tool: {0}/{1} · {2}", "MCP工具：{0}/{1} · {2}"),
    ("minute", "分"),
    ("model: {0}{1}", "模型：{0}{1}"),
    ("network access to {0}", "對{0}的網路訪問"),
    ("new task", "新建任務"),
    ("next question", "下一題"),
    ("no image on clipboard: {0}", "剪貼簿中沒有圖片：{0}"),
    ("no matches", "無匹配項"),
    ("not started", "未啟動"),
    ("note: {0}", "備註：{0}"),
    ("now", "剛剛"),
    ("open", "開啟"),
    ("option {0}/{1}", "選項 {0}/{1}"),
    ("parse pet asset download URL {0}", "解析寵物資源下載URL{0}"),
    (
        "pet asset download from {0} exceeded {1} bytes",
        "從{0}下載的寵物資源超過{1}位元組",
    ),
    (
        "pet frame index does not fit usize",
        "寵物幀索引無法放入usize",
    ),
    (
        "pet frame index exceeds expected frame count",
        "寵物幀索引超出預期的幀數",
    ),
    ("permission request", "許可權請求"),
    ("permission request: {0}", "許可權請求：{0}"),
    ("permissions:", "許可權："),
    ("pet frame index overflow", "寵物幀索引溢位"),
    ("pet frame x offset overflow", "寵物幀x偏移溢位"),
    ("pet frame y offset overflow", "寵物幀y偏移溢位"),
    (
        "pet image asset unavailable: {0}",
        "寵物影象資源不可用：{0}",
    ),
    (
        "pet spritesheet path should include an assets directory",
        "寵物精靈表路徑應包含assets目錄",
    ),
    (
        "plugin list cwd must be absolute",
        "外掛列表的cwd必須是絕對路徑",
    ),
    ("press {0}", "按{0}"),
    ("prev question", "上一題"),
    ("question", "問題"),
    ("questions", "問題"),
    ("queued messages", "排隊訊息"),
    ("raw output", "原始輸出"),
    (
        "read pet asset download from {0}",
        "讀取從{0}下載的寵物資源",
    ),
    ("read {0}", "讀取{0}"),
    ("reasoning {0}", "推理 {0}"),
    ("remote app server", "遠端應用伺服器"),
    ("remote {0}", "遠端 {0}"),
    ("remove {0}", "刪除{0}"),
    (
        "sixel RGBA buffer length overflow",
        "sixel RGBA緩衝區長度溢位",
    ),
    (
        "sixel byte index does not fit usize",
        "sixel位元組索引無法放入usize",
    ),
    ("rename", "重新命名"),
    ("resume", "恢復"),
    ("review finished: ", "審查已完成："),
    ("review started: ", "審查已開始："),
    ("search", "搜尋"),
    ("second", "秒"),
    ("send input to terminal {0}: {1}", "向終端{0}傳送輸入：{1}"),
    ("sixel byte index overflow", "sixel位元組索引溢位"),
    (
        "sixel image dimensions must be non-zero",
        "sixel影象尺寸必須非零",
    ),
    (
        "sixel pixel count does not fit usize",
        "sixel畫素計數無法放入usize",
    ),
    ("sixel pixel count overflow", "sixel畫素計數溢位"),
    ("sixel pixel index overflow", "sixel畫素索引溢位"),
    ("slash command", "斜槓命令"),
    ("starting", "正在啟動"),
    ("status: {0}{1}", "狀態：{0}{1}"),
    ("stop", "停止"),
    ("summaries off", "摘要關閉"),
    ("summaries {0}", "摘要 {0}"),
    ("tab or esc to clear notes", "tab或esc清除備註"),
    ("tab to add notes", "tab新增備註"),
    (
        "temporary structured response exceeds {0} bytes",
        "臨時結構化響應超過{0}位元組",
    ),
    (
        "temporary structured thread did not preserve permission profile {0}",
        "臨時結構化執行緒未保留許可權配置檔案{0}",
    ),
    (
        "temporary structured thread did not start with read-only permissions",
        "臨時結構化執行緒未以只讀許可權啟動",
    ),
    (
        "temporary structured thread start timed out",
        "臨時結構化執行緒啟動超時",
    ),
    (
        "temporary structured turn completed without a response",
        "臨時結構化輪次未返回響應即完成",
    ),
    (
        "temporary structured turn ended with status {0}",
        "臨時結構化輪次以狀態{0}結束",
    ),
    (
        "temporary structured turn notification channel closed",
        "臨時結構化輪次的通知通道已關閉",
    ),
    ("terminal image write failed: {0}", "終端影象寫入失敗：{0}"),
    (
        "the selected thread is no longer available for prompt editing",
        "所選執行緒已不可用於編輯提示詞",
    ),
    (
        "the {0} does not support thread/queue/add; update or restart the {0}",
        "{0}不支援thread/queue/add；請更新或重啟{0}",
    ),
    ("thread id `{0}` is invalid: {1}", "執行緒ID`{0}`無效：{1}"),
    (
        "thread usage request timed out in TUI",
        "執行緒用量請求在TUI中超時",
    ),
    ("to continue working", "繼續工作"),
    ("to edit message", "編輯訊息"),
    ("to edit next", "編輯下一條"),
    ("to edit prev", "編輯上一條"),
    ("to go back", "返回"),
    ("to jump", "跳轉"),
    ("to page", "翻頁"),
    ("to scroll", "滾動"),
    ("tool", "工具"),
    ("tool: {0} · {1}", "工具：{0} · {1}"),
    ("tools", "工具"),
    (
        "tui.keymap.agents.{0}: ctrl-z is reserved for suspend",
        "tui.keymap.agents.{0}：ctrl-z保留用於掛起",
    ),
    (
        "tui.keymap.agents.{0}: printable keys and backspace are reserved for task input",
        "tui.keymap.agents.{0}：可列印字元鍵與backspace保留用於任務輸入",
    ),
    (
        "tui.keymap.chat.{0}: printable keys are reserved for text input",
        "tui.keymap.chat.{0}：可列印字元鍵保留用於文字輸入",
    ),
    (
        "tui.keymap.global.open_agents: AltGr characters are reserved for text input",
        "tui.keymap.global.open_agents：AltGr字元保留用於文字輸入",
    ),
    (
        "tui.keymap.global.open_agents: ctrl-z is reserved for suspend",
        "tui.keymap.global.open_agents：ctrl-z保留用於掛起",
    ),
    ("unknown", "未知"),
    (
        "unknown protocol {0}; expected auto, kitty, or sixel",
        "未知協議{0}；應為auto、kitty或sixel",
    ),
    (
        "unsupported pet asset download URL scheme {0}",
        "不支援的寵物資源下載URL協議{0}",
    ),
    ("unsupported - {0}", "不支援 - {0}"),
    ("use the configured stop shortcut", "使用已配置的停止快捷鍵"),
    (
        "visualization fragment has no file name",
        "視覺化片段沒有檔名",
    ),
    (
        "visualization viewer cache is unavailable",
        "視覺化檢視器快取不可用",
    ),
    (
        "visualization viewer directory must not contain symbolic links",
        "視覺化檢視器目錄不能包含符號連結",
    ),
    ("waiting for next key", "等待下一個按鍵"),
    ("web search: ", "網路搜尋："),
    (
        "workload identity must be configured on the remote app-server host",
        "必須在遠端app-server主機上配置工作負載身份",
    ),
    ("write {0}", "寫入{0}"),
    (
        "{0} A checkout was retained at {1}; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.",
        "{0}已在{1}保留一份檢出；如不再需要，請在源倉庫中執行 `git worktree remove <checkout-path>` 將其刪除。",
    ),
    (
        "{0}   {1} working   {2} ready",
        "{0}   {1} 工作中   {2} 就緒",
    ),
    ("{0} (custom)", "{0}（自定義）"),
    ("{0} (iapi)", "{0}（iapi）"),
    ("{0} (service)", "{0}（service）"),
    ("{0} ({1} skill)", "{0}（{1}個技能）"),
    ("{0} ({1} unanswered)", "{0}（{1}個未回答）"),
    (
        "{0} Learn more about {1} at ",
        "{0} 瞭解{1}的更多資訊，請訪問 ",
    ),
    ("{0} MCP servers", "{0}個MCP伺服器"),
    ("{0} actions.", "{0}個操作。"),
    ("{0} apps", "{0}個應用"),
    (
        "{0} are disabled in this TUI session.",
        "本次TUI會話中{0}已禁用。",
    ),
    ("{0} credits", "{0} 額度"),
    ("{0} events received ({1})", "接收{0}個事件（{1}）"),
    ("{0} hooks", "{0}鉤子"),
    (
        "{0} hooks need review before they can run.",
        "有{0}個鉤子在執行前需要審查。",
    ),
    (
        "{0} hooks are new or changed.",
        "有{0}個鉤子是新增或已變更。",
    ),
    ("{0} input", "輸入 {0}"),
    ("{0} limit", "{0}限額"),
    ("{0} need input", "{0} 需要輸入"),
    ("{0} of {1}", "{0}/{1}"),
    ("{0} of {1} credits used", "已用{0}/{1}額度"),
    ("{0} on {1}", "{0}，於 {1}"),
    ("{0} output", "輸出 {0}"),
    ("{0} questions requested", "請求了{0}個問題"),
    (
        "{0} setting saved on the server for new threads. This thread is unchanged. Project or task settings may override it.",
        "{0}設定已儲存到伺服器，用於新執行緒。當前執行緒不受影響。專案或任務設定可能覆蓋它。",
    ),
    (
        "{0} setting was saved but is overridden: {1}",
        "{0}設定已儲存但被覆蓋：{1}",
    ),
    (
        "{0} skills enabled, {1} skills disabled",
        "已啟用{0}個技能，已禁用{1}個技能",
    ),
    ("{0} submit", "{0} 提交"),
    ("{0} to interrupt", "{0} 中斷"),
    ("{0} unavailable", "{0}不可用"),
    ("{0} window", "{0} 視窗"),
    ("{0} {1} limit", "{0} {1}限額"),
    ("{0} {1}s ago", "{0}{1}前"),
    ("{0}% context left", "剩餘上下文 {0}%"),
    ("{0} used", "已用 {0}"),
    ("100% context left", "剩餘上下文100%"),
    ("Configure Terminal Title", "配置終端標題"),
    (
        "Select which items to display in the terminal title.",
        "選擇在終端標題中顯示哪些專案。",
    ),
    ("Codex app name", "Codex應用名"),
    (
        "Project name (falls back to current directory name)",
        "專案名（回退為當前目錄名）",
    ),
    ("Current working directory", "當前工作目錄"),
    (
        "Spinner while working, action-required message while blocked.",
        "工作時顯示轉圈，被阻塞時顯示需要操作的訊息。",
    ),
    (
        "Compact session run-state text (Ready, Working, Thinking)",
        "簡潔的會話執行狀態文字（就緒、工作中、思考中）",
    ),
    (
        "Current thread name (omitted when unnamed)",
        "當前執行緒名（未命名時省略）",
    ),
    (
        "Current thread title, or thread identifier when unnamed",
        "當前執行緒標題，未命名時用執行緒標識",
    ),
    (
        "Current Git branch (omitted when unavailable)",
        "當前Git分支（不可用時省略）",
    ),
    (
        "Percentage of context window remaining (omitted when unknown)",
        "剩餘上下文視窗百分比（未知時省略）",
    ),
    (
        "Percentage of context window used (omitted when unknown)",
        "已用上下文視窗百分比（未知時省略）",
    ),
    (
        "Remaining usage on the primary usage limit (omitted when unavailable)",
        "主用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the secondary usage limit (omitted when unavailable)",
        "次用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the 5-hour usage limit (omitted when unavailable)",
        "5小時用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the annual usage limit (omitted when unavailable)",
        "每年用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the daily usage limit (omitted when unavailable)",
        "每日用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the monthly usage limit (omitted when unavailable)",
        "每月用量限額的剩餘額度（不可用時省略）",
    ),
    (
        "Remaining usage on the weekly usage limit (omitted when unavailable)",
        "每週用量限額的剩餘額度（不可用時省略）",
    ),
    ("Codex application version", "Codex應用版本"),
    (
        "Total tokens used in session (omitted when zero)",
        "本次會話已用token總數（為0時省略）",
    ),
    (
        "Total input tokens used in session",
        "本次會話輸入token總數",
    ),
    (
        "Total output tokens used in session",
        "本次會話輸出token總數",
    ),
    (
        "Estimated current-thread credits (Enterprise workspaces only; omitted when unavailable)",
        "當前執行緒的預估額度消耗（僅企業工作區；不可用時省略）",
    ),
    (
        "Estimated current-thread cost (Enterprise workspaces only; omitted when unavailable)",
        "當前執行緒的預估費用（僅企業工作區；不可用時省略）",
    ),
    (
        "Current thread identifier (omitted until thread starts)",
        "當前執行緒標識（執行緒開始前省略）",
    ),
    (
        "Whether Fast mode is currently active",
        "Fast模式當前是否啟用",
    ),
    ("Current model name", "當前模型名"),
    (
        "Current model name with reasoning level",
        "當前模型名及推理強度",
    ),
    ("Current reasoning level", "當前推理強度"),
    (
        "Latest task progress from update_plan (omitted until available)",
        "update_plan的最新任務進度（尚不可用時省略）",
    ),
    (
        "Project name (omitted when unavailable)",
        "專案名（不可用時省略）",
    ),
    (
        "Current machine hostname (omitted when unavailable)",
        "當前機器主機名（不可用時省略）",
    ),
    (
        "Open pull request number for the current branch (omitted when unavailable)",
        "當前分支的待處理PR編號（不可用時省略）",
    ),
    (
        "Committed branch changes against the default branch (omitted when unavailable)",
        "相對預設分支的已提交改動（不可用時省略）",
    ),
    (
        "Active permission profile or sandbox mode",
        "當前許可權配置或沙箱模式",
    ),
    ("Active command approval mode", "當前命令審批模式"),
    (
        "Total context window size in tokens (omitted when unknown)",
        "上下文視窗總token數（未知時省略）",
    ),
    (
        "Estimated current-thread cost in USD (Enterprise workspaces only; omitted when unavailable)",
        "當前執行緒的預估費用（美元，僅企業工作區；不可用時省略）",
    ),
    (
        "Whether raw scrollback mode is active",
        "原始回滾模式是否啟用",
    ),
    (
        "Workspace notification headline (Enterprise workspaces only; omitted when unavailable)",
        "工作區通知標題（僅企業工作區；不可用時省略）",
    ),
    ("Use theme colors", "使用主題顏色"),
    (
        "Apply colors from the active /theme",
        "應用當前 /theme的顏色",
    ),
    ("Configure Status Line", "配置狀態列"),
    (
        "Select which items to display in the status line.",
        "選擇在狀態列中顯示哪些專案。",
    ),
    (
        "Would you like to run the following command?",
        "要執行下面的命令嗎？",
    ),
    (
        "Would you like to send input to the existing terminal?",
        "要把輸入傳送到已有終端嗎？",
    ),
    (
        "Would you like to send input to terminal {0}?",
        "要把輸入傳送到終端 {0} 嗎？",
    ),
    (
        "Do you want to approve network access to \"{0}\"?",
        "要批准訪問網路「{0}」嗎？",
    ),
    (
        "Would you like to grant these permissions?",
        "要授予這些許可權嗎？",
    ),
    (
        "Would you like to make the following edits?",
        "要進行下面的編輯嗎？",
    ),
    ("{0} needs your approval.", "{0} 需要你的批准。"),
    (
        "You did not grant additional permissions",
        "你未授予額外許可權",
    ),
    (
        "You granted additional permissions with strict auto review",
        "你以嚴格自動審查方式授予了額外許可權",
    ),
    (
        "You granted additional permissions for this session",
        "你在本次會話中授予了額外許可權",
    ),
    ("You granted additional permissions", "你授予了額外許可權"),
    ("Yes, proceed", "是，繼續"),
    (
        "Yes, and don't ask again for these files",
        "是，且這些檔案不再詢問",
    ),
    (
        "Yes, grant these permissions for this turn",
        "是，本輪授予這些許可權",
    ),
    (
        "Yes, grant these permissions for this session",
        "是，本次會話授予這些許可權",
    ),
    (
        "Yes, grant for this turn with strict auto review",
        "是，本輪以嚴格自動審查方式授予",
    ),
    ("Yes, provide the requested info", "是，提供所請求的資訊"),
    ("No, continue without running it", "不，不執行就繼續"),
    ("No, continue without permissions", "不，不授予許可權繼續"),
    ("No, but continue without it", "不，但不使用它繼續"),
    (
        "No, and tell Codex what to do differently",
        "不，並告訴Codex該怎麼做",
    ),
    ("Cancel this request", "取消本次請求"),
    ("to confirm", "確認"),
    ("to cancel", "取消"),
    (" to open thread", " 開啟執行緒"),
    ("Thread: ", "執行緒："),
    ("Environment: ", "環境："),
    ("Reason: ", "原因："),
    ("Permission rule: ", "許可權規則："),
    ("Input: ", "輸入："),
    ("Server: ", "伺服器："),
    ("Yes, just this once", "是，僅此一次"),
    (
        "Yes, and don't ask again for commands that start with `{0}`",
        "是，且不再詢問以 `{0}` 開頭的命令",
    ),
    (
        "Yes, and allow this host for this conversation",
        "是，本次對話允許該主機",
    ),
    (
        "Yes, and allow these permissions for this session",
        "是，本次會話允許這些許可權",
    ),
    (
        "Yes, and don't ask again for this command in this session",
        "是，本次會話不再詢問該命令",
    ),
    (
        "Yes, and allow this host in the future",
        "是，以後允許該主機",
    ),
    (
        "No, and block this host in the future",
        "不，以後阻止該主機",
    ),
    ("Usage", "用量"),
    (
        "View account usage or redeem an earned reset.",
        "檢視賬戶用量，或兌換已獲得的重置。",
    ),
    ("Show usage", "檢視用量"),
    (
        "View recent account token usage.",
        "檢視最近的賬戶token用量。",
    ),
    ("Redeem usage limit reset", "兌換用量限額重置"),
    ("Check reset availability.", "檢查重置可用性。"),
    (
        "No usage limit resets available.",
        "沒有可用的用量限額重置。",
    ),
    ("You have {0} {1} available.", "你有 {0} 個{1}可用。"),
    ("{0} {1} available.", "{0} 個{1}可用。"),
    ("Usage limit resets", "用量限額重置"),
    ("Checking your available resets...", "正在檢查可用的重置…"),
    ("Loading...", "載入中…"),
    (
        "You don't have any usage limit resets available.",
        "你沒有可用的用量限額重置。",
    ),
    (
        "Couldn't load usage limit resets. Please try again.",
        "無法載入用量限額重置，請重試。",
    ),
    ("Cancel", "取消"),
    ("Use this reset?", "使用這次重置嗎？"),
    ("Yes, use reset", "是，使用重置"),
    ("No, go back", "不，返回"),
    ("Choose a different reset.", "選擇另一個重置。"),
    ("Plugins are disabled.", "外掛功能已禁用。"),
    (
        "Enable the plugins feature to use /plugins.",
        "啟用外掛功能後才能使用 /plugins。",
    ),
    ("Add marketplace", "新增市場源"),
    (
        "owner/repo, git URL, or local marketplace path",
        "owner/repo、Git URL或本地市場源路徑",
    ),
    (
        "Examples: owner/repo, git URL, ./marketplace",
        "示例：owner/repo、Git URL、./marketplace",
    ),
    ("Installed {0} plugin.", "已安裝外掛 {0}。"),
    (
        "No additional app authentication is required.",
        "無需額外的應用認證。",
    ),
    (
        "{0} app(s) still need authentication: {1}",
        "仍有 {0} 個應用需要認證：{1}",
    ),
    ("Collaboration modes are disabled.", "協作模式已禁用。"),
    (
        "Enable collaboration modes to use /plan.",
        "啟用協作模式後才能使用 /plan。",
    ),
    ("Plan mode unavailable right now.", "計劃模式當前不可用。"),
    ("Archive this session?", "歸檔本次會話？"),
    ("No, don't archive", "不，不歸檔"),
    ("Return to the current session", "返回當前會話"),
    ("Yes, archive and exit", "是，歸檔並退出"),
    ("Archive this session now", "立即歸檔本次會話"),
    ("Delete this session?", "刪除本次會話？"),
    (
        "Cannot be undone. Subagent threads will also be deleted.",
        "無法撤銷。子代理執行緒也會被刪除。",
    ),
    ("No, keep this session", "不，保留本次會話"),
    ("Yes, delete and exit", "是，刪除並退出"),
    (
        "Permanently delete this session now",
        "立即永久刪除本次會話",
    ),
    (
        "Session is still starting; try /app again in a moment.",
        "會話仍在啟動中，請稍後再試 /app。",
    ),
    (
        "Where should the new conversation run?",
        "新對話應在哪裡執行？",
    ),
    (
        "Where should the forked conversation run?",
        "分叉的對話應在哪裡執行？",
    ),
    ("Current checkout", "當前檢出"),
    (
        "Keep using the current working directory",
        "繼續使用當前工作目錄",
    ),
    ("New worktree", "新建工作樹"),
    ("Create an isolated managed checkout", "建立隔離的受管檢出"),
    (
        "Enable worktrees in /experimental to create a worktree.",
        "在 /experimental中啟用工作樹後才能建立工作樹。",
    ),
    (
        "Managed worktrees require a local Git repository.",
        "受管工作樹需要本地Git倉庫。",
    ),
    ("Worktrees", "工作樹"),
    ("Continue current conversation", "繼續當前對話"),
    (
        "Preserve this conversation in the new checkout",
        "在新檢出中保留本次對話",
    ),
    ("Start new conversation", "開始新對話"),
    (
        "Open a fresh conversation in the new checkout",
        "在新檢出中開啟全新對話",
    ),
    ("Browse worktrees", "瀏覽工作樹"),
    (
        "Resume an owner thread or copy a working directory",
        "恢復歸屬執行緒，或複製工作目錄",
    ),
    ("Managed worktrees", "受管工作樹"),
    ("Loading worktrees…", "正在載入工作樹…"),
    (
        "Cannot list managed worktrees: {0}",
        "無法列出受管工作樹：{0}",
    ),
    (
        "No worktrees in this repository's configured pool",
        "該倉庫的配置池中沒有工作樹",
    ),
    (
        "Select a worktree to resume its owner or copy its working directory",
        "選擇一個工作樹以恢復其歸屬執行緒或複製其工作目錄",
    ),
    ("No owner metadata", "無歸屬後設資料"),
    ("Owner: {0}", "歸屬：{0}"),
    ("Worktree working directory", "工作樹工作目錄"),
    ("Resume owner thread", "恢復歸屬執行緒"),
    ("Copy working directory", "複製工作目錄"),
    ("Path is not valid UTF-8", "路徑不是有效的UTF-8"),
    ("Worktree", "工作樹"),
    (
        "Warning: OpenAI base URL is overridden to {0}. Selecting models may not be supported or work properly.",
        "警告：OpenAI base URL已被覆蓋為 {0}。選擇模型可能不受支援或無法正常工作。",
    ),
    (
        "Choose a specific model and reasoning level (current: {0})",
        "選擇具體模型與推理強度（當前：{0}）",
    ),
    (
        "Model selection is disabled until startup completes.",
        "啟動完成前無法選擇模型。",
    ),
    (
        "Models are being updated; please try /model again in a moment.",
        "模型正在更新，請稍後再試 /model。",
    ),
    ("Select Model", "選擇模型"),
    ("All models", "全部模型"),
    (
        "Pick a quick auto mode or browse all models.",
        "選擇一種自動模式，或瀏覽全部模型。",
    ),
    ("Select Model and Effort", "選擇模型與推理強度"),
    (
        "Access legacy models by running codex -m <model_name> or in your config.toml",
        "執行codex -m <model_name> 或在config.toml中訪問舊版模型",
    ),
    (
        "No additional models are available right now.",
        "當前沒有其他可用模型。",
    ),
    ("no reasoning", "無推理"),
    ("the selected reasoning", "所選推理強度"),
    ("{0} reasoning", "{0} 推理"),
    ("Always use {0} in Plan mode.", "在計劃模式下始終使用 {0}。"),
    (
        "user-chosen Plan override ({0})",
        "使用者選擇的計劃模式覆蓋（{0}）",
    ),
    ("built-in Plan default ({0})", "內建計劃模式預設值（{0}）"),
    (
        "built-in Plan default (no reasoning)",
        "內建計劃模式預設值（無推理）",
    ),
    ("built-in Plan default", "內建計劃模式預設值"),
    (
        "Set the global default reasoning level and the Plan mode override. This replaces the current {0}.",
        "設定全域性預設推理強度與計劃模式覆蓋。這會替換當前的 {0}。",
    ),
    ("Choose where to apply {0}.", "選擇將 {0} 應用到何處。"),
    (
        "⚠ {0} reasoning effort can quickly consume Plus plan rate limits.",
        "⚠ {0} 推理強度會很快消耗Plus套餐的用量限額。",
    ),
    ("More reasoning…", "更多推理強度…"),
    ("Select Reasoning Level for {0}", "為 {0} 選擇推理強度"),
    ("Minimal", "最小"),
    ("Low", "低"),
    ("Medium", "中"),
    ("High", "高"),
    ("Max", "最高"),
    ("Ultra", "終極"),
    ("Persistent", "持久"),
    (
        "Ultra reasoning may proactively use multiple agents. This session is configured for {0} concurrent threads with up to {1} subagents which can increase usage quickly. Consider setting features.multi_agent_v2.max_concurrent_threads_per_session below 8.",
        "終極推理可能會主動使用多個代理。本次會話配置為 {0} 個併發執行緒、最多 {1} 個子代理，這會很快增加用量。建議把features.multi_agent_v2.max_concurrent_threads_per_session設為小於8。",
    ),
    ("Full Access mode", "完全訪問模式"),
    ("Agent mode", "代理模式"),
    ("Read-Only mode", "只讀模式"),
    (
        "The Windows sandbox cannot guarantee protection in {0}.",
        "Windows沙箱在{0}下無法保證保護。",
    ),
    (
        "The Windows sandbox cannot protect writes to folders that are writable by Everyone.",
        "Windows沙箱無法保護Everyone可寫的資料夾。",
    ),
    ("and {0} more", "以及另外 {0} 項"),
    ("Continue", "繼續"),
    ("Apply {0} for this session", "本次會話應用{0}"),
    ("Continue and don't warn again", "繼續且不再警告"),
    ("Enable {0} and remember this choice", "啟用{0}並記住此選擇"),
    (
        "Set up default sandbox (requires Administrator permissions)",
        "設定預設沙箱（需要管理員許可權）",
    ),
    ("Plugins", "外掛"),
    ("OpenAI Curated", "OpenAI精選"),
    ("Workspace", "工作區"),
    ("Shared with me", "與我共享"),
    ("Shared with me (link)", "與我共享（連結）"),
    ("Local", "本地"),
    ("No workspace plugins available", "沒有可用的工作區外掛"),
    (
        "No workspace directory plugins are available.",
        "沒有可用的工作區目錄外掛。",
    ),
    ("No shared plugins available", "沒有可用的共享外掛"),
    (
        "No plugins have been shared with you.",
        "還沒有人向你共享外掛。",
    ),
    ("Loading available plugins...", "正在載入可用外掛…"),
    ("Loading plugins...", "正在載入外掛…"),
    ("Adding marketplace...", "正在新增市場源…"),
    ("Remove {0} marketplace?", "要移除市場源 {0} 嗎？"),
    (" select", " 選擇"),
    ("esc close", "Esc關閉"),
    ("Remove marketplace", "移除市場源"),
    ("Back to plugins", "返回外掛"),
    ("Keep this marketplace installed.", "保留已安裝的市場源。"),
    ("Removing {0}...", "正在移除 {0}…"),
    ("{0} unavailable.", "{0} 不可用。"),
    (
        "Local plugin functionality is still available.",
        "本地外掛功能仍然可用。",
    ),
    ("All Plugins", "全部外掛"),
    ("Add Marketplace", "新增市場源"),
    ("Edit goal", "編輯目標"),
    (
        "Type a goal objective and press Enter",
        "輸入目標內容後按Enter",
    ),
    ("Resume paused goal?", "恢復已暫停的目標？"),
    ("Goal: {0}", "目標：{0}"),
    ("Resume goal", "恢復目標"),
    (
        "Mark it active and continue when idle",
        "標記為活躍，空閒時繼續",
    ),
    ("Leave paused", "保持暫停"),
    (
        "Keep it paused; use /goal resume later",
        "保持暫停；稍後用 /goal resume",
    ),
    ("Status: ", "狀態："),
    ("Objective: ", "目標："),
    ("Time used: ", "已用時間："),
    ("Tokens used: ", "已用token："),
    ("Token budget: ", "token預算："),
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
    ("active", "進行中"),
    ("paused", "已暫停"),
    ("stalled", "停滯"),
    ("usage limited", "用量受限"),
    ("limited by budget", "預算受限"),
    ("complete", "已完成"),
    (
        "Ctrl+L is disabled while a task is in progress.",
        "任務進行中時Ctrl+L已禁用。",
    ),
    (
        "Copied last message to clipboard",
        "已複製最後一條訊息到剪貼簿",
    ),
    ("No agent response to copy", "沒有可複製的代理回覆"),
    ("Whole status", "整個狀態列"),
    ("Whole response", "整條回覆"),
    ("Code block", "程式碼塊"),
    ("Blockquote", "引用塊"),
    ("Copy to clipboard", "複製到剪貼簿"),
    ("Copied {0} to clipboard", "已複製{0}到剪貼簿"),
    ("Type a name and press Enter", "輸入名稱後按Enter"),
    ("Thread name cannot be empty.", "執行緒名不能為空。"),
    ("Generating a title suggestion…", "正在生成標題建議…"),
    ("Suggested from this conversation", "根據本次對話生成"),
    ("Compacting context", "正在壓縮上下文"),
    ("Making room to continue.", "騰出空間以繼續。"),
    ("Working", "處理中"),
    ("Context compacted", "上下文已壓縮"),
    ("Context compacted · {0}", "上下文已壓縮 · {0}"),
    (
        "Steer messages aren't supported during /review. Press Ctrl+C now to cancel the review.",
        "在 /review期間不支援引導訊息。現在按Ctrl+C可取消審查。",
    ),
    ("Copy failed: {0}", "複製失敗：{0}"),
    ("Update Model Permissions", "更新模型許可權"),
    ("Disabled by requirements.", "已被要求禁用。"),
    ("Current permission profile.", "當前許可權配置。"),
    ("Configured permission profile.", "已配置的許可權配置。"),
    ("Not available on this server.", "此伺服器上不可用。"),
    (
        "No permission profiles returned by the server.",
        "伺服器未返回任何許可權配置。",
    ),
    (
        "Internal error: missing the 'read-only' approval preset.",
        "內部錯誤：缺少 'read-only' 審批預設。",
    ),
    (
        "Internal error: missing the 'full-access' approval preset.",
        "內部錯誤：缺少 'full-access' 審批預設。",
    ),
    (
        "Marketplace {0} is already added.",
        "市場源 {0} 已新增過了。",
    ),
    ("Added marketplace {0}.", "已新增市場源 {0}。"),
    ("Marketplace root: {0}", "市場源根目錄：{0}"),
    ("Removed marketplace {0}.", "已移除市場源 {0}。"),
    (
        "Removed marketplace config for {0}.",
        "已移除 {0} 的市場源配置。",
    ),
    (
        "No configured Git marketplaces to upgrade.",
        "沒有可升級的已配置Git市場源。",
    ),
    (
        "Only configured Git marketplaces can be upgraded.",
        "只能升級已配置的Git市場源。",
    ),
    (
        "Marketplace {0} is already up to date.",
        "市場源 {0} 已是最新。",
    ),
    (
        "Checked {0} marketplaces; all are already up to date.",
        "已檢查 {0} 個市場源，全部都是最新。",
    ),
    ("Checked: {0}", "已檢查：{0}"),
    ("marketplace", "市場源"),
    ("marketplaces", "市場源"),
    ("Upgraded {0} {1}.", "已升級 {0} 個{1}。"),
    (
        "Side conversations are ephemeral and cannot be renamed.",
        "側邊對話是臨時的，不能重新命名。",
    ),
    (
        "'/side' is unavailable until the main thread is ready.",
        "主執行緒就緒之前無法使用 '/side'。",
    ),
    (
        "'/side' is unavailable until the current conversation has started. Send a message first, then try /side again.",
        "當前對話開始之前無法使用 '/side'。請先傳送一條訊息，然後再試 /side。",
    ),
    (
        "A side conversation is already open. Press ctrl + c to return before starting another.",
        "已經有一個側邊對話處於開啟狀態。請先按ctrl + c返回再新建。",
    ),
    ("main needs input", "主執行緒需要輸入"),
    ("parent needs input", "父執行緒需要輸入"),
    ("main needs approval", "主執行緒需要審批"),
    ("parent needs approval", "父執行緒需要審批"),
    ("main failed", "主執行緒失敗"),
    ("parent failed", "父執行緒失敗"),
    ("main interrupted", "主執行緒已中斷"),
    ("parent interrupted", "父執行緒已中斷"),
    ("main closed", "主執行緒已關閉"),
    ("parent closed", "父執行緒已關閉"),
    ("main finished", "主執行緒已完成"),
    ("parent finished", "父執行緒已完成"),
    ("Failed to add marketplace: {0}", "新增市場失敗：{0}"),
    ("Failed to remove marketplace: {0}", "移除市場失敗：{0}"),
    ("Failed to upgrade marketplace: {0}", "升級市場失敗：{0}"),
    ("Failed to install plugin: {0}", "安裝外掛失敗：{0}"),
    ("Failed to uninstall plugin: {0}", "解除安裝外掛失敗：{0}"),
    (
        "Failed to update plugin config: {0}",
        "更新外掛配置失敗：{0}",
    ),
    ("Failed to update hook config: {0}", "更新鉤子配置失敗：{0}"),
    ("Failed to trust hook: {0}", "信任鉤子失敗：{0}"),
    ("Failed to trust hooks: {0}", "信任鉤子失敗：{0}"),
    ("Failed to upload feedback: {0}", "上傳反饋失敗：{0}"),
    ("Failed to load MCP inventory: {0}", "載入MCP清單失敗：{0}"),
    ("Permissions updated to {0}", "許可權已更新為 {0}"),
    (
        "Wait for the task to connect before selecting permissions.",
        "請等待任務連線後再選擇許可權。",
    ),
    (
        "Wait for the current turn to finish before changing permissions.",
        "請等待當前回合結束後再更改許可權。",
    ),
    (
        "Failed to rebuild config for cwd {0}",
        "為工作目錄 {0} 重建配置失敗",
    ),
    (
        "Failed to set permission profile `{0}`: {1}",
        "設定許可權配置 `{0}` 失敗：{1}",
    ),
    ("A worktree is already being created.", "已在建立工作樹。"),
    (
        "Changing directories is not supported for remote workspaces or remote execution environments.",
        "遠端工作區或遠端執行環境不支援切換目錄。",
    ),
    ("Not a directory: {0}", "不是目錄：{0}"),
    ("Cannot access directory {0}: {1}", "無法訪問目錄 {0}：{1}"),
    (
        "No saved chat found matching '{0}'.",
        "沒有找到與 '{0}' 匹配的已儲存對話。",
    ),
    (
        "Failed to name the forked session: {0}",
        "為分叉會話命名失敗：{0}",
    ),
    (
        "Failed to attach to forked app-server thread: {0}",
        "附加到分叉的app-server執行緒失敗：{0}",
    ),
    (
        "Failed to fork current session through the app server: {0}",
        "透過app server分叉當前會話失敗：{0}",
    ),
    (
        "A thread must contain at least one turn before it can be forked.",
        "執行緒至少要包含一個回合才能分叉。",
    ),
    (
        "Wait for permissions to update before editing this prompt.",
        "請等待許可權更新後再編輯此提示。",
    ),
    ("Export failed: {0}", "匯出失敗：{0}"),
    ("Failed to interrupt task: {0}", "中斷任務失敗：{0}"),
    ("Failed to pause task goal: {0}", "暫停任務目標失敗：{0}"),
    ("Logout failed: {0}", "退出登入失敗：{0}"),
    (
        "Goals need a saved session. This session is temporary.\nRun `codex` to start a saved session, or `codex resume` / `/resume` to reopen one.",
        "目標功能需要已儲存的會話。當前會話是臨時的。\n請執行 `codex` 開啟已儲存的會話，或用 `codex resume` / `/resume` 重新開啟一個。",
    ),
    ("Goal {0}", "目標{0}"),
    ("Goal cleared", "目標已清除"),
    ("No goal is currently set.", "當前沒有設定目標。"),
    ("No goal to clear", "沒有可清除的目標"),
    (
        "This thread does not currently have a goal.",
        "此執行緒當前沒有目標。",
    ),
    ("Replace current goal", "替換當前目標"),
    (
        "Set the new objective and start it now",
        "設定新目標並立即開始",
    ),
    ("Keep the current goal", "保留當前目標"),
    ("Replace goal?", "替換目標？"),
    ("New objective: {0}", "新目標：{0}"),
    ("Create a goal before editing it.", "請先建立目標再編輯。"),
    (
        "This task cannot be safely replaced.",
        "無法安全替換此任務。",
    ),
    (
        "Wait for permissions to update before changing directories.",
        "請等待許可權更新後再切換目錄。",
    ),
    (
        "Changing directories with a named profile is not supported.",
        "不支援在指定配置檔案的情況下切換目錄。",
    ),
    ("MCP inventory is still loading.", "MCP清單仍在載入中。"),
    (
        "Cannot change: another agent is running.",
        "無法切換：另一個代理正在執行。",
    ),
    (
        "Cannot continue into the new worktree while the session is offline or blocked by a policy warning. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "會話離線或被策略警告阻止時，無法繼續進入新工作樹。已在 {0} 建立了一個未使用的檢出；請在源倉庫中執行 `git worktree remove <checkout-path>` 將其移除。",
    ),
    (
        "The source conversation changed while creating the worktree. An unused checkout was created at {0}; remove it with `git worktree remove <checkout-path>` from the source repository.",
        "建立工作樹期間源對話發生了變化。已在 {0} 建立了一個未使用的檢出；請在源倉庫中執行 `git worktree remove <checkout-path>` 將其移除。",
    ),
    (
        "Could not start a session in the managed worktree. A checkout was retained at {0}; remove it with `git worktree remove <checkout-path>` from the source repository if it is no longer needed.",
        "無法在受管工作樹中啟動會話。檢出保留在 {0}；如果不再需要，請在源倉庫中執行 `git worktree remove <checkout-path>` 將其移除。",
    ),
    ("No agents available yet.", "暫無可用代理。"),
    ("Subagents", "子代理"),
    (
        "Agent thread {0} is not yet available for replay or live attach.",
        "代理執行緒 {0} 尚不能用於回放或實時附加。",
    ),
    (
        "Agent thread {0} is no longer available.",
        "代理執行緒 {0} 已不可用。",
    ),
    (
        "Failed to attach to agent thread {0}: {1}",
        "附加到代理執行緒 {0} 失敗：{1}",
    ),
    ("Main [default]", "主執行緒 [預設]"),
    ("Agent ({0})", "代理 ({0})"),
    ("Already viewing {0}.", "已在檢視{0}。"),
    ("No active thread is available.", "沒有可用的活動執行緒。"),
    (
        "This conversation is read-only or unavailable; no operation was sent.",
        "此對話為只讀或不可用；未傳送任何操作。",
    ),
    (
        "Not available in TUI yet for thread {0}.",
        "TUI中暫不支援執行緒 {0} 的此操作。",
    ),
    ("Global", "全域性"),
    ("Chat", "對話"),
    ("Composer", "輸入框"),
    ("Editor", "編輯器"),
    (
        "Open the shared agent-session overview.",
        "開啟共享的代理會話總覽。",
    ),
    ("Open the transcript overlay.", "開啟記錄浮層。"),
    (
        "Open the current draft in an external editor.",
        "用外部編輯器開啟當前草稿。",
    ),
    (
        "Copy the last agent response to the clipboard.",
        "複製最後一條代理回覆到剪貼簿。",
    ),
    ("Clear the terminal UI.", "清空終端介面。"),
    ("Turn Vim composer mode on or off.", "開關Vim輸入模式。"),
    ("Turn Fast mode on or off.", "開關Fast模式。"),
    ("Toggle raw scrollback mode.", "切換原始回滾模式。"),
    (
        "Switch between a side conversation and its parent.",
        "在側邊對話與其父對話之間切換。",
    ),
    ("Interrupt the active turn.", "中斷當前回合。"),
    ("Decrease reasoning effort.", "降低推理強度。"),
    ("Increase reasoning effort.", "提高推理強度。"),
    (
        "Switch to the previous available permission mode.",
        "切換到上一個可用的許可權模式。",
    ),
    (
        "Switch to the next available permission mode.",
        "切換到下一個可用的許可權模式。",
    ),
    (
        "Move up through questions, then edit the last queued message.",
        "在問題間向上移動，然後編輯最後一條排隊訊息。",
    ),
    (
        "Move back through questions toward the composer.",
        "在問題間向後返回到輸入框。",
    ),
    ("Skip the focused question.", "跳過當前聚焦的問題。"),
    ("Submit the current composer draft.", "提交當前輸入內容。"),
    (
        "Queue the draft while a task is running.",
        "任務執行時把草稿加入佇列。",
    ),
    (
        "Show or hide the composer shortcut overlay.",
        "顯示或隱藏輸入快捷鍵浮層。",
    ),
    (
        "Open history search or move to the previous match.",
        "開啟歷史搜尋，或移動到上一個匹配項。",
    ),
    (
        "Move to the next history search match.",
        "移動到下一個歷史搜尋匹配項。",
    ),
    ("Insert a newline in the editor.", "在編輯器中插入換行。"),
    ("Move the cursor left.", "游標左移。"),
    ("Vim normal", "Vim普通模式"),
    ("Move the cursor right.", "游標右移。"),
    ("Move the cursor up.", "游標上移。"),
    ("Move the cursor down.", "游標下移。"),
    (
        "Move to the beginning of the previous word.",
        "移動到上一個詞的開頭。",
    ),
    (
        "Move to the end of the next word.",
        "移動到下一個詞的結尾。",
    ),
    ("Move to the beginning of the line.", "移動到行首。"),
    ("Move to the end of the line.", "移動到行尾。"),
    ("Delete one grapheme to the left.", "刪除左側一個字形。"),
    ("Delete one grapheme to the right.", "刪除右側一個字形。"),
    ("Delete the previous word.", "刪除上一個詞。"),
    ("Delete the next word.", "刪除下一個詞。"),
    ("Delete from cursor to line start.", "從游標刪除到行首。"),
    ("Delete the current line.", "刪除當前行。"),
    ("Delete from cursor to line end.", "從游標刪除到行尾。"),
    ("Paste the kill buffer.", "貼上刪除緩衝區的內容。"),
    ("Enter insert mode at the cursor.", "在游標處進入插入模式。"),
    (
        "Enter insert mode after the cursor.",
        "在游標之後進入插入模式。",
    ),
    ("Enter insert mode at end of line.", "在行尾進入插入模式。"),
    (
        "Enter insert mode at the first non-blank character.",
        "在第一個非空白字元處進入插入模式。",
    ),
    (
        "Open a new line below and enter insert mode.",
        "在下方新建一行並進入插入模式。",
    ),
    (
        "Open a new line above and enter insert mode.",
        "在上方新建一行並進入插入模式。",
    ),
    (
        "Enter replace mode and overwrite characters.",
        "進入替換模式並覆蓋字元。",
    ),
    ("Move left in Vim normal mode.", "在Vim普通模式下左移。"),
    ("Move right in Vim normal mode.", "在Vim普通模式下右移。"),
    (
        "Move up or recall older history in Vim normal mode.",
        "在Vim普通模式下上移或回溯更早的歷史。",
    ),
    (
        "Move down or recall newer history in Vim normal mode.",
        "在Vim普通模式下下移或回溯更新的歷史。",
    ),
    (
        "Move to the start of the next word.",
        "移動到下一個詞的開頭。",
    ),
    (
        "Move to the start of the previous word.",
        "移動到上一個詞的開頭。",
    ),
    (
        "Move to the end of the current or next word.",
        "移動到當前詞或下一個詞的結尾。",
    ),
    ("Move to the start of the line.", "移動到行首。"),
    (
        "Find the next character on the current line.",
        "在當前行查詢下一個字元。",
    ),
    (
        "Find the previous character on the current line.",
        "在當前行查詢上一個字元。",
    ),
    (
        "Stop before the next character on the current line.",
        "停在當前行下一個字元之前。",
    ),
    (
        "Stop after the previous character on the current line.",
        "停在當前行上一個字元之後。",
    ),
    ("Jump to the first buffer line.", "跳到緩衝區第一行。"),
    ("Jump to the last buffer line.", "跳到緩衝區最後一行。"),
    (
        "Delete the character under the cursor.",
        "刪除游標下的字元。",
    ),
    (
        "Replace the character under the cursor.",
        "替換游標下的字元。",
    ),
    ("Repeat the last complete edit.", "重複上一次完整編輯。"),
    (
        "Delete the character under the cursor and enter insert mode.",
        "刪除游標下的字元並進入插入模式。",
    ),
    ("Delete from cursor to end of line.", "從游標刪除到行尾。"),
    (
        "Change from cursor to end of line and enter insert mode.",
        "從游標修改到行尾並進入插入模式。",
    ),
    ("Yank the entire line.", "複製整行。"),
    ("Paste after the cursor.", "貼上到游標之後。"),
    (
        "Begin a delete operator and wait for a motion.",
        "開始刪除運算子並等待動作。",
    ),
    (
        "Begin a yank operator and wait for a motion.",
        "開始複製運算子並等待動作。",
    ),
    (
        "Begin a change operator and wait for a motion or text object.",
        "開始修改運算子並等待動作或文字物件。",
    ),
    ("Vim search", "Vim搜尋"),
    ("Vim operator", "Vim運算子"),
    ("Undo the last complete edit.", "撤銷上一次完整編輯。"),
    ("Redo the last undone edit.", "重做上一次撤銷的編輯。"),
    ("Cancel a pending Vim operator.", "取消待處理的Vim運算子。"),
    (
        "Search forward in the active buffer.",
        "在當前緩衝區向前搜尋。",
    ),
    (
        "Search backward in the active buffer.",
        "在當前緩衝區向後搜尋。",
    ),
    ("Repeat the accepted search.", "重複已接受的搜尋。"),
    (
        "Repeat the search in the opposite direction.",
        "朝相反方向重複搜尋。",
    ),
    (
        "Repeat delete operator to delete the whole line.",
        "重複刪除運算子以刪除整行。",
    ),
    (
        "Repeat yank operator to yank the whole line.",
        "重複複製運算子以複製整行。",
    ),
    ("Operator motion left.", "運算子動作：左移。"),
    ("Operator motion right.", "運算子動作：右移。"),
    ("Operator motion up.", "運算子動作：上移。"),
    ("Operator motion down.", "運算子動作：下移。"),
    (
        "Operator motion to start of next word.",
        "運算子動作：移到下一個詞的開頭。",
    ),
    (
        "Operator motion to start of previous word.",
        "運算子動作：移到上一個詞的開頭。",
    ),
    (
        "Operator motion to end of word.",
        "運算子動作：移到詞的結尾。",
    ),
    ("Operator motion to line start.", "運算子動作：移到行首。"),
    ("Operator motion to line end.", "運算子動作：移到行尾。"),
    (
        "Operator motion to the next character on the current line.",
        "運算子動作：移到當前行的下一個字元。",
    ),
    (
        "Operator motion to the previous character on the current line.",
        "運算子動作：移到當前行的上一個字元。",
    ),
    (
        "Operator motion to the first buffer line.",
        "運算子動作：移到緩衝區第一行。",
    ),
    (
        "Operator motion to the last buffer line.",
        "運算子動作：移到緩衝區最後一行。",
    ),
    ("Vim text object", "Vim文字物件"),
    ("Pager", "分頁檢視"),
    ("List", "列表"),
    ("Select an inner text object.", "選擇內部文字物件。"),
    ("Select an around text object.", "選擇環繞文字物件。"),
    ("Cancel the pending operator.", "取消待處理的運算子。"),
    ("Target the current word.", "定位到當前詞。"),
    ("Target the current WORD.", "定位到當前WORD。"),
    ("Target enclosing parentheses.", "定位到外層圓括號。"),
    ("Target enclosing brackets.", "定位到外層方括號。"),
    ("Target enclosing braces.", "定位到外層花括號。"),
    ("Target enclosing double quotes.", "定位到外層雙引號。"),
    ("Target enclosing single quotes.", "定位到外層單引號。"),
    ("Target enclosing backticks.", "定位到外層反引號。"),
    ("Cancel the pending text object.", "取消待處理的文字物件。"),
    ("Scroll up by one row.", "上移一行。"),
    ("Scroll down by one row.", "下移一行。"),
    ("Scroll up by one page.", "上翻一頁。"),
    ("Scroll down by one page.", "下翻一頁。"),
    ("Scroll up by half a page.", "上翻半頁。"),
    ("Scroll down by half a page.", "下翻半頁。"),
    ("Jump to the beginning.", "跳到開頭。"),
    ("Jump to the end.", "跳到結尾。"),
    ("Close the pager overlay.", "關閉分頁浮層。"),
    ("Close the transcript overlay.", "關閉記錄浮層。"),
    ("Move list selection up.", "列表選中項上移。"),
    ("Move list selection down.", "列表選中項下移。"),
    ("Agents", "代理"),
    ("Approval", "審批"),
    (
        "Move horizontally left in list pickers.",
        "在列表選擇中左移。",
    ),
    (
        "Move horizontally right in list pickers.",
        "在列表選擇中右移。",
    ),
    (
        "Move list selection up by one page.",
        "列表選中項上移一頁。",
    ),
    (
        "Move list selection down by one page.",
        "列表選中項下移一頁。",
    ),
    ("Jump to the first list item.", "跳到列表第一項。"),
    ("Jump to the last list item.", "跳到列表最後一項。"),
    ("Accept the current list selection.", "接受當前列表選擇。"),
    ("Cancel and close selection views.", "取消並關閉選擇檢視。"),
    ("Open the session resume picker.", "開啟會話恢復選擇器。"),
    ("Search the available agent tasks.", "搜尋可用的代理任務。"),
    (
        "Start composing a new agent task.",
        "開始編寫新的代理任務。",
    ),
    ("Rename the selected task.", "重新命名所選任務。"),
    ("Stop the selected running task.", "停止所選的執行中任務。"),
    (
        "Group tasks by status or project.",
        "按狀態或專案分組任務。",
    ),
    ("Open approval details fullscreen.", "全屏開啟審批詳情。"),
    (
        "Open the approval source thread when available.",
        "在可用時開啟審批來源執行緒。",
    ),
    ("Approve the primary option.", "批准主要選項。"),
    (
        "Approve for the session when available.",
        "在可用時批准本次會話。",
    ),
    (
        "Approve with an exec-policy prefix when available.",
        "在可用時按exec-policy字首批准。",
    ),
    (
        "Choose the explicit deny option when available.",
        "在可用時選擇明確的拒絕選項。",
    ),
    (
        "Decline and provide corrective guidance.",
        "拒絕並給出糾正建議。",
    ),
    ("Cancel an elicitation request.", "取消一次資訊請求。"),
    ("Running hooks", "正在執行鉤子"),
    ("Running hook", "正在執行鉤子"),
    ("{0}% used", "已用{0}%"),
    ("{0}d (best {1}d)", "{0}天（最長{1}天）"),
    ("{0}d ago", "{0}天前"),
    ("{0}d {1}h {2}m", "{0}天{1}小時{2}分"),
    ("{0}h", "{0}小時"),
    ("{0}h ago", "{0}小時前"),
    ("{0}h {1}m", "{0}小時{1}分"),
    ("{0}m", "{0}分"),
    ("{0}m ago", "{0}分前"),
    ("{0}m {1}s", "{0}分{1}秒"),
    ("{0}s", "{0}秒"),
    ("{0}s ago", "{0}秒前"),
    ("• Copied conversation to clipboard", "• 已複製會話到剪貼簿"),
    ("• Saved conversation to ", "• 會話已儲存到"),
    ("• Waited for background terminal", "• 已等待後臺終端"),
    ("←/→ to navigate questions", "←/→ 切換問題"),
    ("↳ Hook · ", "↳ 鉤子 · "),
    ("Hook completed", "鉤子已完成"),
    ("Hook failed", "鉤子失敗"),
    ("Blocked by hook", "被鉤子阻止"),
    ("Hook stopped", "鉤子已停止"),
    ("Hook running", "鉤子執行中"),
    ("Conversation recap", "對話回顧"),
    ("Update available!", "有可用更新！"),
    ("See full release notes:", "檢視完整發布說明："),
    ("Run {0} to update.", "執行 {0} 進行更新。"),
    ("This content can't be shown", "此內容無法顯示"),
    ("This content can’t be shown", "此內容無法顯示"),
    ("Trusted Access", "受信任訪問"),
    ("Learn more", "瞭解更多"),
    ("Apply for Daybreak", "申請Daybreak"),
    (
        "We take extra caution with requests involving biological research and applications that could pose safety risks. Eligible researchers can apply for Trusted Access.",
        "對於涉及生物研究以及可能帶來安全風險的應用的請求，我們會格外謹慎。符合條件的研究人員可以申請受信任訪問。",
    ),
    (
        "We take extra care with some cybersecurity requests. If you’re doing authorized security work, apply for Daybreak to get broader access.",
        "我們對某些網路安全請求會格外謹慎。如果你從事獲授權的安全工作，可以申請Daybreak以獲得更寬的訪問許可權。",
    ),
    (
        "Daybreak isn’t available for Astra. Some cybersecurity requests may still be limited.",
        "Daybreak不適用於Astra。部分網路安全請求仍可能受到限制。",
    ),
    (
        "We take extra care with some cybersecurity requests.",
        "我們對某些網路安全請求會格外謹慎。",
    ),
    ("approved", "已批准"),
    ("did not approve", "未批准"),
    ("persisted", "已持久化"),
    ("denied", "已拒絕"),
    ("Request ", "請求 "),
    ("Review ", "審查 "),
    ("Unknown", "未知"),
    ("Unsupported", "不支援"),
    ("Not logged in", "未登入"),
    ("Bearer token", "Bearer令牌"),
    ("tool result (image output)", "工具結果（圖片輸出）"),
    ("MCP docs", "MCP文件"),
    ("Error: {0}", "錯誤：{0}"),
    ("Tip: {0}", "提示：{0}"),
    ("model changed:", "模型已更改："),
    ("requested: {0}", "請求：{0}"),
    ("used: {0}", "實際使用：{0}"),
    ("OpenAI Codex", "OpenAI Codex"),
    ("OpenAI Codex (v{0})", "OpenAI Codex (v{0})"),
    ("YOLO mode", "YOLO模式"),
    ("permissions: YOLO mode", "許可權：YOLO模式"),
    (" codex to run ", " 讓codex執行 "),
    (" this time", "（僅此次）"),
    (" this request", "（本次請求）"),
    (" codex network access to ", " 讓codex訪問網路 "),
    (" every time this session", "（本次會話內每次都允許）"),
    (" and saved that rule", "，並儲存了該規則"),
    (" Codex network access to ", " Codex訪問網路 "),
    (" for codex to run ", " 由codex執行 "),
    ("not available for this account", "此賬戶不可用"),
    (
        "limits may be stale - run /status again shortly.",
        "限額資料可能已過期 - 請稍後重新執行 /status。",
    ),
    (
        "limits may be stale - start new turn to refresh.",
        "限額資料可能已過期 - 開始新回合以重新整理。",
    ),
    (
        "refresh requested; run /status again shortly.",
        "已請求重新整理；請稍後重新執行 /status。",
    ),
    ("data not available yet", "資料尚不可用"),
    ("read-only with network access", "只讀並允許網路訪問"),
    (
        "custom permissions with network access",
        "自定義許可權並允許網路訪問",
    ),
    ("Read Only with network access", "只讀並允許網路訪問"),
    ("Read Only", "只讀"),
    ("Full Access", "完全訪問"),
    (" used / ", " 已用 / "),
    ("{0} ({1})", "{0}（{1}）"),
    ("Workspace{0} ({1})", "工作區{0}（{1}）"),
    (
        "Workspace with network access{0} ({1})",
        "工作區（允許網路訪問）{0}（{1}）",
    ),
    ("No Sandbox ({0})", "無沙箱（{0}）"),
    ("Profile {0} ({1}, {2})", "配置檔 {0}（{1}，{2}）"),
    ("Read Only ({0})", "只讀（{0}）"),
    ("Custom ({0}, {1})", "自定義（{0}，{1}）"),
    ("{0}% left", "剩餘 {0}%"),
    ("(resets {0})", "（{0} 重置）"),
    ("Approve for me", "替我審批"),
    ("Ask for approval", "請求批准"),
    (
        "API key configured (run codex login to use ChatGPT)",
        "已配置API key（執行codex login以使用ChatGPT）",
    ),
    ("Model provider", "模型提供方"),
    ("Thread name", "執行緒名"),
    ("Forked from", "分叉自"),
    ("Collaboration mode", "協作模式"),
    ("Token usage", "token用量"),
    ("Context window", "上下文視窗"),
    (" for up-to-date", " 以獲取最新"),
    ("information on rate limits and credits", "限額與額度資訊"),
    (
        "Lifecycle hooks from config and enabled plugins.",
        "來自配置與已啟用外掛的生命週期鉤子。",
    ),
    (
        "Turn hooks on or off. Your changes are saved automatically.",
        "開啟或關閉鉤子。改動會自動儲存。",
    ),
    ("Installed", "已安裝"),
    ("Active", "已啟用"),
    ("Review", "審查"),
    ("Description", "描述"),
    ("Issues", "問題"),
    (
        "No hooks installed for this event.",
        "此事件未安裝任何鉤子。",
    ),
    ("Matcher", "匹配器"),
    ("Source", "來源"),
    ("Command", "命令"),
    ("MCP Server", "MCP伺服器"),
    ("MCP Tool", "MCP工具"),
    ("Handler", "處理方式"),
    ("Prompt", "提示詞"),
    ("Agent", "代理"),
    ("Timeout", "超時"),
    ("Context", "上下文"),
    ("unlimited", "無限制"),
    ("limit: {0} approximate tokens", "上限：約 {0} 個token"),
    (" to go back", " 返回"),
    (
        "Managed hooks are always on; press ",
        "受管鉤子始終開啟；按 ",
    ),
    ("Press ", "按 "),
    (
        "Sign in with ChatGPT to use Codex as part of your paid plan",
        "使用ChatGPT登入，以透過付費方案使用Codex",
    ),
    (
        "or connect an API key for usage-based billing",
        "或連線API key以按用量計費",
    ),
    ("ChatGPT login is disabled", "ChatGPT登入已被禁用"),
    (
        "Usage included with Plus, Pro, Business, and Enterprise plans",
        "Plus、Pro、Business和Enterprise方案已包含用量",
    ),
    (
        "Sign in from another device with a one-time code",
        "在另一臺裝置上用一次性驗證碼登入",
    ),
    ("Sign in with ChatGPT", "使用ChatGPT登入"),
    ("Sign in with Device Code", "使用裝置碼登入"),
    ("Use an OpenAI API key", "使用OpenAI API key"),
    ("Provide your own API key", "使用你自己的API key"),
    ("Pay for what you use", "按用量付費"),
    ("Use Amazon Bedrock", "使用Amazon Bedrock"),
    ("Connect using your AWS credentials", "使用你的AWS憑據連線"),
    ("Finish signing in via your browser", "在瀏覽器中完成登入"),
    ("Codex docs", "Codex文件"),
    ("API key login is disabled.", "API key登入已被禁用。"),
    ("Set up Amazon Bedrock", "設定Amazon Bedrock"),
    ("AWS access key ID", "AWS訪問金鑰ID"),
    ("AWS secret access key", "AWS秘密訪問金鑰"),
    ("AWS session token (optional)", "AWS會話令牌（可選）"),
    ("Continue with {0}", "繼續使用 {0}"),
    ("Use your existing AWS credentials", "使用你現有的AWS憑據"),
    (
        "Use your existing Amazon Bedrock API key",
        "使用你現有的Amazon Bedrock API key",
    ),
    ("Continue with detected credentials", "使用檢測到的憑據繼續"),
    ("Choose another sign-in method", "選擇其他登入方式"),
    ("Other AWS sign-in methods", "其他AWS登入方式"),
    (
        "Use another profile, access keys, or environment variables",
        "使用其他配置檔、訪問金鑰或環境變數",
    ),
    ("AWS profile", "AWS配置檔"),
    ("Use AWS SSO or a named profile", "使用AWS SSO或命名配置檔"),
    ("AWS access keys", "AWS訪問金鑰"),
    (
        "Enter an access key ID and secret access key",
        "輸入訪問金鑰ID與秘密訪問金鑰",
    ),
    ("Environment variables", "環境變數"),
    ("send logs to maintainers", "把日誌發給維護者"),
    (
        "start a new chat during a conversation",
        "在對話中開始新對話",
    ),
    (
        "create an AGENTS.md file with instructions for Codex",
        "建立帶Codex指令的AGENTS.md檔案",
    ),
    (
        "summarize conversation to prevent hitting the context limit",
        "總結對話以避免觸及上下文上限",
    ),
    ("summarize the current conversation now", "立即總結當前對話"),
    (
        "review my current changes and find issues",
        "審查當前改動並找出問題",
    ),
    ("rename the current thread", "重新命名當前執行緒"),
    ("resume a saved chat", "恢復已儲存的對話"),
    ("archive this session and exit", "歸檔本次會話並退出"),
    (
        "permanently delete this session and exit",
        "永久刪除本次會話並退出",
    ),
    (
        "clear the terminal and start a new chat",
        "清空終端並開始新對話",
    ),
    ("fork the current chat", "分叉當前對話"),
    (
        "start or continue a conversation in a new worktree",
        "在新工作樹中開始或繼續對話",
    ),
    (
        "continue this session in the Desktop app",
        "在桌面應用中繼續本次會話",
    ),
    ("exit Codex", "退出Codex"),
    (
        "copy the last response or part of it",
        "複製最後一條回覆或其一部分",
    ),
    (
        "export the conversation as markdown",
        "把對話匯出為markdown",
    ),
    (
        "toggle raw scrollback mode for copy-friendly terminal selection",
        "切換原始回滾模式，便於終端複製選擇",
    ),
    (
        "show git diff (including untracked files)",
        "顯示git diff（含未跟蹤檔案）",
    ),
    ("mention a file", "提及一個檔案"),
    (
        "use skills to improve how Codex performs specific tasks",
        "用技能改進Codex完成特定任務的方式",
    ),
    (
        "import setup, this project, and recent chats from Claude Code",
        "從Claude Code匯入設定、本專案與近期對話",
    ),
    ("view and manage lifecycle hooks", "檢視與管理生命週期鉤子"),
    (
        "show current session configuration and token usage",
        "顯示當前會話配置與token用量",
    ),
    ("change the current working directory", "切換當前工作目錄"),
    ("show the current working directory", "顯示當前工作目錄"),
    (
        "view account usage or use a usage limit reset",
        "檢視賬戶用量或使用一次限額重置",
    ),
    (
        "show config layers and requirement sources for debugging",
        "顯示配置層與要求來源，用於除錯",
    ),
    (
        "configure which items appear in the terminal title",
        "配置終端標題中顯示的專案",
    ),
    (
        "configure which items appear in the status line",
        "配置狀態列中顯示的專案",
    ),
    ("choose a syntax highlighting theme", "選擇語法高亮主題"),
    ("switch the interface language", "切換介面語言"),
    ("Select Language", "選擇語言"),
    ("Choose the interface language.", "選擇介面語言。"),
    ("Type to filter languages...", "輸入以過濾語言..."),
    ("Persistence", "持久化"),
    (
        "Keep the new interface language after restart?",
        "重啟後是否保留新介面語言？",
    ),
    ("Session only", "僅本次會話"),
    ("Apply until you quit Codex.", "生效至退出Codex。"),
    ("Write to config.toml", "寫入config.toml"),
    (
        "Save the choice so it persists across restarts.",
        "儲存該選擇，重啟後依然生效。",
    ),
    (
        "Interface language changed (session only).",
        "介面語言已切換（僅本次會話）。",
    ),
    (
        "Interface language changed and saved to config.toml.",
        "介面語言已切換，並已寫入config.toml。",
    ),
    (
        "Unrecognized language; pick one from the list.",
        "無法識別的語言；請從列表中選擇。",
    ),
    (
        "Cannot use both --config and --session. Choose one persistence flag.",
        "不能同時使用--config與--session；請選擇一個持久化旗標。",
    ),
    ("Available languages: {0}", "可用語言：{0}"),
    ("LLM output language: {0}", "LLM輸出語言：{0}"),
    (
        "LLM output language is not set; the model follows your input language.",
        "未設定LLM輸出語言；模型將跟隨你的輸入語言。",
    ),
    ("LLM output language cleared.", "LLM輸出語言已清除。"),
    (
        "LLM output language set to {0}; applies to new sessions.",
        "LLM輸出語言已設為{0}；對新會話生效。",
    ),
    ("Failed to update AGENTS.md: {0}", "更新AGENTS.md失敗：{0}"),
    (
        "Failed to write config.toml: {0}",
        "寫入config.toml失敗：{0}",
    ),
    ("choose or hide the terminal pet", "選擇或隱藏終端寵物"),
    ("list background terminals", "列出後臺終端"),
    ("stop all background terminals", "停止所有後臺終端"),
    ("DO NOT USE", "請勿使用"),
    (
        "choose what model and reasoning effort to use",
        "選擇使用的模型與推理強度",
    ),
    (
        "choose a communication style for Codex",
        "為Codex選擇溝通風格",
    ),
    ("switch to Plan mode", "切換到計劃模式"),
    (
        "set or view the goal for a long-running task",
        "設定或檢視長任務的目標",
    ),
    (
        "view and switch between all active agent sessions",
        "檢視並在所有活動代理會話間切換",
    ),
    (
        "switch between this session's subagents",
        "在本次會話的子代理間切換",
    ),
    ("choose what Codex is allowed to do", "選擇允許Codex做什麼"),
    ("remap TUI shortcuts", "重新對映TUI快捷鍵"),
    ("toggle Vim mode for the composer", "開關輸入框的Vim模式"),
    ("set up elevated agent sandbox", "設定提權代理沙箱"),
    ("toggle experimental features", "開關實驗性功能"),
    (
        "approve one retry of a recent auto-review denial",
        "批准對最近一次自動審查拒絕的一次重試",
    ),
    (
        "configure memory use and generation",
        "配置記憶的使用與生成",
    ),
    (
        "list configured MCP tools; use /mcp verbose for details",
        "列出已配置的MCP工具；用 /mcp verbose檢視詳情",
    ),
    ("manage apps", "管理應用"),
    ("browse plugins", "瀏覽外掛"),
    ("log out of Codex", "退出Codex登入"),
    ("print the rollout file path", "列印rollout檔案路徑"),
    ("test approval request", "測試審批請求"),
    ("Removing marketplace...", "正在移除市場源…"),
    ("Loading plugin details...", "正在載入外掛詳情…"),
    (
        "This updates when plugin details load.",
        "外掛詳情載入後會更新。",
    ),
    ("Installing plugin...", "正在安裝外掛…"),
    ("Uninstalling plugin...", "正在解除安裝外掛…"),
    (
        "This updates when the plugin removal completes.",
        "外掛移除完成後會更新。",
    ),
    ("Failed to load plugins.", "載入外掛失敗。"),
    ("Plugin marketplace unavailable", "外掛市場源不可用"),
    ("Failed to add marketplace.", "新增市場源失敗。"),
    ("Marketplace add failed", "市場源新增失敗"),
    ("Try again", "重試"),
    (
        "Review the confirmation prompt again.",
        "請重新檢視確認提示。",
    ),
    ("No marketplace plugins available", "沒有可用的市場源外掛"),
    (
        "Browse plugins from available marketplaces.",
        "從可用市場源瀏覽外掛。",
    ),
    ("Installed plugins.", "已安裝的外掛。"),
    ("No installed plugins", "沒有已安裝的外掛"),
    ("No installed plugins.", "沒有已安裝的外掛。"),
    (
        "Loading OpenAI Curated plugins...",
        "正在載入OpenAI精選外掛…",
    ),
    ("OpenAI Curated unavailable", "OpenAI精選不可用"),
    (
        "No OpenAI Curated plugins available",
        "沒有可用的OpenAI精選外掛",
    ),
    (
        "No OpenAI Curated plugins available.",
        "沒有可用的OpenAI精選外掛。",
    ),
    ("OpenAI Curated marketplace.", "OpenAI精選市場源。"),
    (
        "Select the plugins you want to use and press Enter to install or view details.",
        "選擇要使用的外掛，按Enter安裝或檢視詳情。",
    ),
    (
        "No plugins available in this marketplace",
        "此市場源中沒有可用外掛",
    ),
    (
        "No plugins available in this marketplace.",
        "此市場源中沒有可用外掛。",
    ),
    ("Type to search plugins", "輸入以搜尋外掛"),
    (
        "Press Enter to enter a marketplace source.",
        "按Enter輸入市場源。",
    ),
    ("Installed by admin", "由管理員安裝"),
    (
        "Data shared with this app is subject to the app's ",
        "與此應用共享的資料受該應用的",
    ),
    ("terms of service", "服務條款"),
    (" and ", " 和 "),
    ("privacy policy", "隱私政策"),
    (". ", "。 "),
    ("Marketplace removal failed", "市場源移除失敗"),
    (
        "Failed to remove the selected marketplace.",
        "移除所選市場源失敗。",
    ),
    ("Uninstall plugin", "解除安裝外掛"),
    ("Remove this plugin now.", "立即移除此外掛。"),
    ("Install plugin", "安裝外掛"),
    ("Install this plugin now.", "立即安裝此外掛。"),
    ("MCP Servers", "MCP伺服器"),
    ("plugin details are unavailable", "外掛詳情不可用"),
    ("Upgrading marketplaces...", "正在升級市場源…"),
    ("Upgrading {0} marketplace...", "正在升級市場源 {0}…"),
    ("Loading details for {0}...", "正在載入 {0} 的詳情…"),
    ("Installed ({0})", "已安裝（{0}）"),
    ("{0} installed successfully.", "{0} 安裝成功。"),
    (
        "Installed {0} of {1} {2} plugins.",
        "已安裝 {0}/{1} 個{2}外掛。",
    ),
    (
        "{0}   Space to {1}; Enter view details.",
        "{0}   Space可{1}；Enter檢視詳情。",
    ),
    ("{0}   Space to {1}.", "{0}   Space可{1}。"),
    (
        "{0}   Press Enter to view plugin details.",
        "{0}   Enter檢視外掛詳情。",
    ),
    (
        "{0}   Plugin details are unavailable.",
        "{0}   外掛詳情不可用。",
    ),
    (
        "{0}   Press Enter to install or view plugin details.",
        "{0}   Enter安裝或檢視外掛詳情。",
    ),
    (
        "{0}   Remote plugin details are not available yet.",
        "{0}   遠端外掛詳情尚不可用。",
    ),
    ("disable", "禁用"),
    ("enable", "啟用"),
    ("No changes detected.", "未檢測到改動。"),
    (
        "That Windows sandbox option is disallowed by requirements.",
        "該Windows沙箱選項被要求禁止。",
    ),
    ("Sandbox ready", "沙箱已就緒"),
    (
        "Codex can now safely edit files and execute commands in your computer",
        "Codex現在可以在你的電腦上安全地編輯檔案並執行命令",
    ),
    ("Service tier cleared", "服務層級已清除"),
    ("To continue this session, run ", "要繼續本次會話，請執行 "),
    ("Failed to start turn: {0}", "啟動回合失敗：{0}"),
    ("Failed to save default model: {0}", "儲存預設模型失敗：{0}"),
    (
        "Model changed to {0} {1} for this conversation",
        "本次對話的模型已更改為 {0} {1}",
    ),
    (
        "Failed to prepare Windows sandbox for the selected permission profile: {0}",
        "為所選許可權配置準備Windows沙箱失敗：{0}",
    ),
    (
        "Granting sandbox read access to {0} ...",
        "正在授予 {0} 的沙箱讀許可權…",
    ),
    (
        "Sandbox read access granted for {0}",
        "已授予 {0} 的沙箱讀許可權",
    ),
    (
        "Failed to enable the Windows sandbox feature: {0}",
        "啟用Windows沙箱功能失敗：{0}",
    ),
    (
        "Cyber models default to \"Approve for me\" for safety reasons.",
        "出於安全考慮，網路安全類模型預設使用“替我審批”。",
    ),
    ("Personality set to {0}", "溝通風格已設定為 {0}"),
    (
        "Failed to save default personality: {0}",
        "儲存預設溝通風格失敗：{0}",
    ),
    ("Service tier set to {0}", "服務層級已設定為 {0}"),
    ("App", "應用"),
    ("Vim", "Vim"),
    ("Navigation", "導航"),
    (" select · ", " 選擇 · "),
    ("Custom root override", "自定義根級覆蓋"),
    ("Default keymap", "預設鍵位表"),
    ("Edit Shortcut", "編輯快捷鍵"),
    ("Current ", "當前 "),
    (
        "Capture a key for this unbound action.",
        "為此未繫結操作捕獲一個按鍵。",
    ),
    ("Replace binding", "替換繫結"),
    ("Add alternate binding", "新增備用繫結"),
    ("Replace one binding...", "替換其中一個繫結…"),
    (
        "Choose which existing binding to replace.",
        "選擇要替換的現有繫結。",
    ),
    ("Replace all bindings", "替換全部繫結"),
    ("Set key chord", "設定組合鍵"),
    ("Replace with key chord", "用組合鍵替換"),
    ("Add alternate key chord", "新增備用組合鍵"),
    ("Remove custom binding", "移除自定義繫結"),
    (
        "Restore the default keymap binding.",
        "恢復預設鍵位表繫結。",
    ),
    ("Back to shortcuts", "返回快捷鍵列表"),
    ("Return to the shortcut list.", "返回快捷鍵列表。"),
    (" override.", " 覆蓋。"),
    ("Choose the binding to replace.", "選擇要替換的繫結。"),
    ("Shortcut Conflict", "快捷鍵衝突"),
    ("Pick another key", "另選一個按鍵"),
    (
        "Return to key capture for this action.",
        "返回為此操作捕獲按鍵。",
    ),
    ("Leave keymap unchanged.", "保持鍵位表不變。"),
    (
        "Only ctrl, alt, and shift modifiers can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能儲存ctrl、alt和shift修飾鍵。",
    ),
    (
        "Only printable ASCII keys can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能儲存可列印ASCII按鍵。",
    ),
    (
        "That key is not supported by `tui.keymap`.",
        "`tui.keymap` 不支援該按鍵。",
    ),
    (
        "Capture a replacement key for `{0}`.",
        "為 `{0}` 捕獲替換按鍵。",
    ),
    (
        "Keep `{0}` and add another key.",
        "保留 `{0}` 並新增另一個按鍵。",
    ),
    ("Replace `{0}` with one key.", "用一個按鍵替換 `{0}`。"),
    (
        "Replace `{0}` with a two-stroke key chord.",
        "用兩段組合鍵替換 `{0}`。",
    ),
    (
        "Keep `{0}` and add a two-stroke key chord.",
        "保留 `{0}` 並新增兩段組合鍵。",
    ),
    (
        "Replace `{0}` with another key.",
        "用另一個按鍵替換 `{0}`。",
    ),
    ("{0} (key chord)", "{0}（組合鍵）"),
    ("{0}.{1} cannot use `{2}`.", "{0}.{1} 不能使用 `{2}`。"),
    (
        "No change: `{0}.{1}` already uses `{2}`.",
        "未更改：`{0}.{1}` 已使用 `{2}`。",
    ),
    (
        "`{0}.{1}` no longer uses `{2}`. Reopen /keymap and choose a binding again.",
        "`{0}.{1}` 已不再使用 `{2}`。請重新開啟 /keymap並再次選擇繫結。",
    ),
    (
        "Remapped `{0}.{1}` to `{2}`.",
        "已將 `{0}.{1}` 重新對映為 `{2}`。",
    ),
    ("Added `{0}` to `{1}.{2}`.", "已為 `{1}.{2}` 新增 `{0}`。"),
    (
        "Replaced `{0}` with `{1}` for `{2}.{3}`.",
        "已為 `{2}.{3}` 將 `{0}` 替換為 `{1}`。",
    ),
    (
        "Unknown keymap action `{0}.{1}`. Reopen /keymap and choose an action.",
        "未知的鍵位表操作 `{0}.{1}`。請重新開啟 /keymap並選擇一個操作。",
    ),
    (
        "Only function keys F1 through F{0} can be stored in `tui.keymap`.",
        "`tui.keymap` 中只能儲存F1到F{0} 的功能鍵。",
    ),
    ("Global and chat-level shortcuts.", "全域性與對話級快捷鍵。"),
    (
        "Composer submission and queue shortcuts.",
        "輸入框提交與排隊快捷鍵。",
    ),
    (
        "Inline editor movement and editing shortcuts.",
        "內嵌編輯器的移動與編輯快捷鍵。",
    ),
    (
        "Vim normal-mode and operator shortcuts.",
        "Vim普通模式與運算子快捷鍵。",
    ),
    (
        "Pager and selection-list navigation shortcuts.",
        "分頁檢視與選擇列表的導航快捷鍵。",
    ),
    ("Shared agents dashboard shortcuts.", "共享代理面板快捷鍵。"),
    ("Approval prompt shortcuts.", "審批提示快捷鍵。"),
    ("All configurable shortcuts.", "全部可配置快捷鍵。"),
    ("No shortcuts available", "沒有可用的快捷鍵"),
    (
        "No configurable shortcuts are available.",
        "沒有可配置的快捷鍵。",
    ),
    ("Frequently customized shortcuts.", "常被自定義的快捷鍵。"),
    ("No common shortcuts", "沒有常用快捷鍵"),
    (
        "No common shortcut actions are available.",
        "沒有可用的常用快捷鍵操作。",
    ),
    ("Root-level shortcut overrides.", "根級快捷鍵覆蓋。"),
    ("No customized shortcuts", "沒有已自定義的快捷鍵"),
    (
        "{0} actions, {1} customized, {2} unbound.",
        "{0} 個操作，{1} 個已自定義，{2} 個未繫結。",
    ),
    ("Customized ({0})", "已自定義（{0}）"),
    ("All", "全部"),
    ("Common", "常用"),
    ("Custom", "自定義"),
    ("Default", "預設"),
    ("Custom global", "全域性自定義"),
    ("Debug", "除錯"),
    ("Resume a previous session", "恢復之前的會話"),
    ("Fork a previous session", "分叉之前的會話"),
    (
        "No transcript available for this session",
        "此會話沒有可用的記錄",
    ),
    (
        "Failed to read session metadata from selected session",
        "從所選會話讀取會話後設資料失敗",
    ),
    ("Could not load transcript preview", "無法載入記錄預覽"),
    ("(no message yet)", "（還沒有訊息）"),
    ("Type to search", "輸入以搜尋"),
    ("Filter: ", "篩選："),
    ("start new", "新建"),
    ("clear search", "清除搜尋"),
    ("dense view", "緊湊檢視"),
    ("comfortable view", "寬鬆檢視"),
    ("focus sort/filter", "聚焦排序/篩選"),
    ("change option", "切換選項"),
    ("Loading transcript…", "正在載入記錄…"),
    ("Loading older sessions…", "正在載入更早的會話…"),
    ("↓ loading more", "↓ 正在載入更多"),
    ("no branch", "無分支"),
    ("Loading recent transcript...", "正在載入最近的記錄…"),
    ("No transcript preview available", "沒有可用的記錄預覽"),
    ("Searching…", "正在搜尋…"),
    ("No results for your search", "沒有匹配的搜尋結果"),
    ("Loading sessions…", "正在載入會話…"),
    ("No sessions yet", "還沒有會話"),
    (
        "Failed to read session metadata from {0}",
        "從 {0} 讀取會話後設資料失敗",
    ),
    ("Search: {0}", "搜尋：{0}"),
    (
        "Sign in to this app in your browser, then return here.",
        "請在瀏覽器中登入此應用，然後回到這裡。",
    ),
    ("Action required", "需要操作"),
    ("Server: {0}", "伺服器：{0}"),
    (
        "Complete the requested action in your browser, then return here.",
        "請在瀏覽器中完成所需操作，然後回到這裡。",
    ),
    ("Open sign-in URL", "開啟登入連結"),
    ("I already signed in", "我已登入"),
    ("Open link", "開啟連結"),
    ("I finished", "我已完成"),
    ("Manage on ChatGPT", "在ChatGPT中管理"),
    ("Disable app", "禁用應用"),
    ("Enable app", "啟用應用"),
    ("Install on ChatGPT", "在ChatGPT中安裝"),
    ("I already Installed it", "我已安裝"),
    (
        "Use $ to insert this app into the prompt.",
        "用 $ 將此應用插入提示。",
    ),
    (
        "Newly installed apps can take a few minutes to appear in /apps.",
        "新安裝的應用可能需要幾分鐘才會出現在 /apps中。",
    ),
    (
        "After installed, use $ to insert this app into the prompt.",
        "安裝後，用 $ 將此應用插入提示。",
    ),
    ("Finish App Sign In", "完成應用登入"),
    ("Finish Authentication", "完成認證"),
    ("Finish in Browser", "在瀏覽器中完成"),
    ("Finish App Setup", "完成應用設定"),
    (
        "Sign in to the app on ChatGPT in the browser window that just opened.",
        "在剛開啟的瀏覽器視窗中登入ChatGPT上的該應用。",
    ),
    (
        "Complete authentication in the browser window that just opened.",
        "在剛開啟的瀏覽器視窗中完成認證。",
    ),
    (
        "Then return here and select \"I already signed in\".",
        "然後回到這裡並選擇“我已登入”。",
    ),
    (
        "Complete the requested action in the browser window that just opened.",
        "在剛開啟的瀏覽器視窗中完成所需操作。",
    ),
    (
        "Then return here and select \"I finished\".",
        "然後回到這裡並選擇“我已完成”。",
    ),
    (
        "Complete app setup on ChatGPT in the browser window that just opened.",
        "在剛開啟的瀏覽器視窗中完成ChatGPT上的應用設定。",
    ),
    (
        "Sign in there if needed, then return here and select \"I already Installed it\".",
        "如需要請在那裡登入，然後回到這裡並選擇“我已安裝”。",
    ),
    ("Sign-in URL:", "登入連結："),
    ("Setup URL:", "設定連結："),
    (" to move", " 移動"),
    (" to select", " 選擇"),
    (" to close", " 關閉"),
    ("No actions", "沒有可用操作"),
    ("Side starting...", "側會話啟動中…"),
    (
        "Press Ctrl+C to return to the main thread first.",
        "請先按Ctrl+C返回主執行緒。",
    ),
    (
        "Example: /goal improve benchmark coverage",
        "示例：/goal improve benchmark coverage",
    ),
    ("Usage: /raw [on|off]", "用法：/raw [on|off]"),
    (
        "Sign in with ChatGPT to use /usage.",
        "請登入ChatGPT後使用/usage。",
    ),
    (
        "'/{0}' is unavailable in side conversations. {1}",
        "側會話中不可用'/{0}'。{1}",
    ),
    (
        "'/{0}' is unavailable before the session starts.",
        "會話開始前不可用'/{0}'。",
    ),
    (
        "'/{0}' is disabled while a task is in progress.",
        "任務進行中已禁用'/{0}'。",
    ),
    (
        "'/{0}' is unavailable while code review is running.",
        "程式碼審查進行中不可用'/{0}'。",
    ),
    (
        "Are you sure? This will archive the current session and exit Codex",
        "確定嗎？這將歸檔當前會話並退出Codex",
    ),
    (
        "Session is still starting; try /recap again in a moment.",
        "會話仍在啟動中；請稍後重試/recap。",
    ),
    (
        "Internal error: missing the 'auto' approval preset.",
        "內部錯誤：缺少'auto'審批預設。",
    ),
    (
        "Usage: /sandbox-add-read-dir <absolute-directory-path>",
        "用法：/sandbox-add-read-dir <絕對目錄路徑>",
    ),
    (
        "`/diff` — _not inside a git repository_",
        "`/diff` — _不在git倉庫內_",
    ),
    ("Failed to compute diff: {0}", "計算diff失敗：{0}"),
    (
        "Failed to compute diff: workspace command runner unavailable",
        "計算diff失敗：工作區命令執行器不可用",
    ),
    ("Current working directory: {0}", "當前工作目錄：{0}"),
    ("Memory maintenance", "記憶維護"),
    ("Current rollout path: {0}", "當前rollout路徑：{0}"),
    (
        "Rollout path is not available yet.",
        "rollout路徑尚不可用。",
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
        "會話開始後才能修改目標。",
    ),
    (
        "The session must start before you can set a goal.",
        "會話開始後才能設定目標。",
    ),
    (
        "Unrecognized command '/{0}'. Type \"/\" for a list of supported commands.",
        "無法識別的命令'/{0}'。輸入\"/\"檢視支援的命令列表。",
    ),
    (
        "terminal clipboard copy failed over SSH: {0}",
        "SSH環境下終端剪貼簿寫入失敗：{0}",
    ),
    (
        "OSC 52 clipboard copy failed over SSH: {0}",
        "SSH環境下OSC 52剪貼簿寫入失敗：{0}",
    ),
    (
        "native clipboard: {0}; WSL fallback: {1}; terminal fallback: {2}",
        "原生剪貼簿：{0}；WSL回退：{1}；終端回退：{2}",
    ),
    (
        "native clipboard: {0}; WSL fallback: {1}; OSC 52 fallback: {2}",
        "原生剪貼簿：{0}；WSL回退：{1}；OSC 52回退：{2}",
    ),
    (
        "native clipboard: {0}; terminal fallback: {1}",
        "原生剪貼簿：{0}；終端回退：{1}",
    ),
    (
        "native clipboard: {0}; OSC 52 fallback: {1}",
        "原生剪貼簿：{0}；OSC 52回退：{1}",
    ),
    (
        "tmux clipboard: {0}; OSC 52 fallback: {1}",
        "tmux剪貼簿：{0}；OSC 52回退：{1}",
    ),
    ("stderr suppression lock poisoned", "stderr抑制鎖已中毒"),
    ("clipboard unavailable: {0}", "剪貼簿不可用：{0}"),
    (
        "failed to set clipboard text: {0}",
        "設定剪貼簿文字失敗：{0}",
    ),
    (
        "native clipboard unavailable on Android",
        "Android上原生剪貼簿不可用",
    ),
    (
        "failed to spawn powershell.exe: {0}",
        "啟動powershell.exe失敗：{0}",
    ),
    (
        "failed to open powershell.exe stdin",
        "開啟powershell.exe標準輸入失敗",
    ),
    (
        "failed to write to powershell.exe: {0}",
        "寫入powershell.exe失敗：{0}",
    ),
    (
        "failed to wait for powershell.exe: {0}",
        "等待powershell.exe失敗：{0}",
    ),
    (
        "powershell.exe exited with status {0}",
        "powershell.exe退出，狀態碼{0}",
    ),
    ("powershell.exe failed: {0}", "powershell.exe失敗：{0}"),
    (
        "WSL clipboard fallback unavailable on this platform",
        "此平臺不支援WSL剪貼簿回退",
    ),
    ("failed to spawn tmux: {0}", "啟動tmux失敗：{0}"),
    ("failed to open tmux stdin", "開啟tmux標準輸入失敗"),
    ("failed to write to tmux: {0}", "寫入tmux失敗：{0}"),
    ("failed to wait for tmux: {0}", "等待tmux失敗：{0}"),
    ("tmux exited with status {0}", "tmux退出，狀態碼{0}"),
    ("tmux failed: {0}", "tmux失敗：{0}"),
    (
        "tmux clipboard forwarding is disabled",
        "tmux剪貼簿轉發已關閉",
    ),
    (
        "tmux clipboard forwarding is unavailable: missing Ms capability",
        "tmux剪貼簿轉發不可用：缺少Ms能力",
    ),
    ("tmux output was not UTF-8: {0}", "tmux輸出不是UTF-8：{0}"),
    ("failed to write OSC 52: {0}", "寫入OSC 52失敗：{0}"),
    ("failed to flush OSC 52: {0}", "重新整理OSC 52失敗：{0}"),
    (
        "OSC 52 payload too large ({0} bytes; max {1})",
        "OSC 52載荷過大（{0}位元組；上限{1}）",
    ),
    ("• Feedback uploaded.", "• 反饋已上傳。"),
    (
        "• Feedback recorded (no logs).",
        "• 反饋已記錄（不含日誌）。",
    ),
    (
        "{0} You can share this in #codex-feedback:",
        "{0} 可在#codex-feedback中分享：",
    ),
    (
        "{0} Please open an issue using the following URL:",
        "{0} 請透過以下連結提交issue：",
    ),
    ("{0} Thanks for the feedback!", "{0} 感謝你的反饋！"),
    ("  Sentry Feedback ID: ", "  Sentry反饋ID："),
    ("  Sentry URL: ", "  Sentry連結："),
    (
        "  Or mention your thread ID ",
        "  或在已有的issue中提及你的執行緒ID ",
    ),
    (" in an existing issue.", "，可附在已有issue中。"),
    ("  Thread ID: ", "  執行緒ID："),
    ("How was this?", "這次體驗如何？"),
    ("bug", "缺陷"),
    (
        "Crash, error message, hang, or broken UI/behavior.",
        "崩潰、報錯、卡死，或介面/行為異常。",
    ),
    ("bad result", "結果不佳"),
    (
        "Output was off-target, incorrect, incomplete, or unhelpful.",
        "輸出偏題、錯誤、不完整或沒有幫助。",
    ),
    ("good result", "結果良好"),
    (
        "Helpful, correct, high‑quality, or delightful result worth celebrating.",
        "有幫助、正確、高質量或令人驚喜的結果，值得慶祝。",
    ),
    ("safety check", "安全檢查"),
    (
        "Benign usage blocked due to safety checks or refusals.",
        "正常使用被安全檢查或拒答攔截。",
    ),
    ("other", "其他"),
    (
        "Slowness, feature suggestion, UX feedback, or anything else.",
        "速度慢、功能建議、互動反饋，或其他任何問題。",
    ),
    ("Sending feedback is disabled", "反饋傳送已禁用"),
    (
        "This action is disabled by configuration.",
        "此操作已被配置禁用。",
    ),
    ("Close", "關閉"),
    ("Upload logs?", "上傳日誌？"),
    ("The following files will be sent:", "將傳送以下檔案："),
    ("Connectivity diagnostics", "連線診斷"),
    ("Yes", "是"),
    (
        "Share the current Codex session logs and diagnostics with the team for troubleshooting.",
        "將當前Codex會話日誌與診斷資訊分享給團隊以便排查。",
    ),
    ("No", "否"),
    (
        "Failed to rebuild config for permission profile {0}",
        "為許可權配置檔{0}重建配置失敗",
    ),
    (
        "Permission selection requested: {0}",
        "已請求許可權選擇：{0}",
    ),
    (
        "Named profiles require a newer app server.",
        "命名配置檔需要更新的app server。",
    ),
    ("Failed to select permissions: {0}", "選擇許可權失敗：{0}"),
    (
        "Wait for permissions to update before changing permissions.",
        "請等待許可權更新完成後再更改許可權。",
    ),
    (
        "{0} were saved, but Codex could not refresh the effective config: {1}",
        "{0}已儲存，但Codex無法重新整理生效配置：{1}",
    ),
    (
        "Failed to carry forward approval policy override: {0}",
        "沿用審批策略覆蓋失敗：{0}",
    ),
    (
        "Failed to carry forward approvals reviewer: {0}",
        "沿用審批審查者失敗：{0}",
    ),
    (
        "Failed to carry forward permission profile override: {0}",
        "沿用許可權配置檔覆蓋失敗：{0}",
    ),
    ("{0}: {1}", "{0}：{1}"),
    (
        "{0}: unsupported active permission profile `{1}`",
        "{0}：不支援的生效許可權配置檔`{1}`",
    ),
    (
        "Failed to update experimental feature `{0}`: {1}",
        "更新實驗特性`{0}`失敗：{1}",
    ),
    ("Failed to enable Approve for me", "啟用「替我批准」失敗"),
    (
        "Failed to update experimental features: {0}",
        "更新實驗特性失敗：{0}",
    ),
    (
        "Experimental feature changes were saved but not applied: {0}",
        "實驗特性更改已儲存但未生效：{0}",
    ),
    (
        "Failed to enable Approve for me: {0}",
        "啟用「替我批准」失敗：{0}",
    ),
    (
        "Failed to save memory settings: {0}",
        "儲存記憶設定失敗：{0}",
    ),
    (
        "Memory setting changes were saved but not applied: {0}",
        "記憶設定更改已儲存但未生效：{0}",
    ),
    (
        "Saved memory settings, but failed to update the current thread: {0}",
        "記憶設定已儲存，但更新當前執行緒失敗：{0}",
    ),
    ("Failed to reset memories: {0}", "重置記憶失敗：{0}"),
    ("Reset local memories.", "已重置本地記憶。"),
    ("default", "預設"),
    ("None", "無"),
    ("Friendly", "友好"),
    ("Pragmatic", "務實"),
    (
        "Failed to refresh overridden Approve for me settings: {0}",
        "重新整理被覆蓋的「替我批准」設定失敗：{0}",
    ),
    (
        "Failed to refresh overridden Approve for me settings",
        "重新整理被覆蓋的「替我批准」設定",
    ),
    (
        "Windows sandbox changes were saved but not applied: {0}",
        "Windows沙箱更改已儲存但未生效：{0}",
    ),
    ("Windows sandbox changes", "Windows沙箱更改"),
    (
        "the effective config is overridden by a higher-priority layer",
        "生效配置被更高優先順序的層覆蓋",
    ),
    ("Failed to set approval policy", "設定審批策略失敗"),
    ("Failed to set permission profile", "設定許可權配置檔失敗"),
    (
        "  Choose how you want to use Codex.",
        "  選擇你想如何使用Codex。",
    ),
    (
        "  API key login is disabled by this workspace. Sign in with ChatGPT to continue.",
        "  此工作區已禁用API key登入。請使用ChatGPT登入以繼續。",
    ),
    ("  Press ", "  按 "),
    (" to continue", " 繼續"),
    (
        "  If the link doesn't open automatically, open the following link to authenticate:",
        "  如果連結沒有自動開啟，請開啟以下連結完成認證：",
    ),
    (
        "  On a remote or headless machine? Press ",
        "  遠端或無頭環境？按 ",
    ),
    (" and choose ", " 並選擇 "),
    (" to cancel", " 取消"),
    ("  For more details see the ", "  詳情請見 "),
    (
        "  Uses your plan's rate limits and ",
        "  使用你套餐的速率限制和 ",
    ),
    ("training data preferences", "訓練資料偏好"),
    (
        "✓ Signed in with your ChatGPT account",
        "✓ 已使用你的ChatGPT賬戶登入",
    ),
    ("  Before you start:", "  開始之前："),
    (
        "  Decide how much autonomy you want to grant Codex",
        "  決定給Codex多大的自主權",
    ),
    ("  Codex can make mistakes", "  Codex可能會出錯"),
    (
        "  Review the code it writes and commands it runs",
        "  請審查它編寫的程式碼與執行的命令",
    ),
    (
        "  Powered by your ChatGPT account",
        "  由你的ChatGPT賬戶提供支援",
    ),
    ("■ Copy failed: ", "■ 複製失敗"),
    ("■ Export failed: ", "■ 匯出失敗："),
    ("⚠ Consumes usage limits faster", "⚠ 更快消耗用量限額"),
    ("✓ API key configured", "✓ API key已配置"),
    (
        "  Codex will use usage-based billing with your API key.",
        "  Codex將使用你的API key按用量計費。",
    ),
    (
        "Use your own OpenAI API key for usage-based billing",
        "使用你自己的OpenAI API key按用量計費",
    ),
    (
        "  Paste or type your API key below. It will be stored locally in auth.json.",
        "  在下方貼上或輸入你的API key。它將儲存在本地的auth.json中。",
    ),
    (
        "  Detected OPENAI_API_KEY environment variable.",
        "  檢測到OPENAI_API_KEY環境變數。",
    ),
    (
        "  Paste a different key if you prefer to use another account.",
        "  如果想使用其他賬戶，請貼上另一個key。",
    ),
    ("Paste or type your API key", "貼上或輸入你的API key"),
    ("API key", "API金鑰"),
    (" to save", " 儲存"),
    ("API key cannot be empty", "API key不能為空"),
    (
        "Unexpected account/login/start response: {0}",
        "意外的account/login/start響應：{0}",
    ),
    ("Failed to save API key: {0}", "儲存API key失敗：{0}"),
    ("✓ Amazon Bedrock configured", "✓ 已配置Amazon Bedrock"),
    (".", "。"),
    ("Managed", "受管"),
    ("Trusted", "已信任"),
    ("New hook - review required", "新鉤子 - 需要審查"),
    (
        "Modified since last trusted - review required",
        "自上次信任後已修改 - 需要審查",
    ),
    ("Before a tool executes", "工具執行之前"),
    ("When permission is requested", "請求許可權時"),
    ("After a tool executes", "工具執行之後"),
    ("Before context compaction", "上下文壓縮之前"),
    ("After context compaction", "上下文壓縮之後"),
    ("When a new session starts", "新會話開始時"),
    ("Right before a session ends", "會話結束之前"),
    ("When the user submits a prompt", "使用者提交提示詞時"),
    ("When a subagent is created", "建立子代理時"),
    (
        "Right before a subagent ends its turn",
        "子代理結束本輪之前",
    ),
    ("Right before Codex ends its turn", "Codex結束本輪之前"),
    (
        "Right before an interrupted turn is aborted",
        "被中斷的本輪終止之前",
    ),
    ("Hook {0}", "鉤子{0}"),
    ("Plugin - {0}", "外掛 - {0}"),
    ("Plugin", "外掛"),
    ("Admin config", "管理員配置"),
    ("User config", "使用者配置"),
    ("Project config", "專案配置"),
    ("Session flags", "會話標誌"),
    ("Cloud-managed config", "雲端管理的配置"),
    ("Unknown source", "未知來源"),
    ("Mode", "模式"),
    ("Async", "非同步"),
    ("Sync", "同步"),
    (
        "This updates when OpenAI Curated plugins finish loading.",
        "OpenAI精選外掛載入完成後會更新。",
    ),
    (
        "This updates when workspace plugins finish loading.",
        "工作區外掛載入完成後會更新。",
    ),
    (
        "This updates when shared plugins finish loading.",
        "共享外掛載入完成後會更新。",
    ),
    (
        "This updates when the marketplace list is ready.",
        "市場源列表就緒後會更新。",
    ),
    (
        "This updates when marketplace installation completes.",
        "市場源安裝完成後會更新。",
    ),
    (
        "This removes the configured marketplace from Codex.",
        "這會從Codex中移除已配置的市場源。",
    ),
    (
        "Remove this marketplace from the available plugin list.",
        "從可用外掛列表中移除此市場源。",
    ),
    (
        "This updates when marketplace removal completes.",
        "市場源移除完成後會更新。",
    ),
    (
        "This updates when marketplace upgrade completes.",
        "市場源升級完成後會更新。",
    ),
    (
        "This updates when plugin installation completes.",
        "外掛安裝完成後會更新。",
    ),
    (
        "Failed to add marketplace from the provided source.",
        "無法從提供的來源新增市場源。",
    ),
    ("Enter a marketplace source.", "輸入市場源。"),
    ("Return to the plugin list.", "返回外掛列表。"),
    ("Failed to remove marketplace.", "移除市場源失敗。"),
    ("Failed to load plugin details.", "載入外掛詳情失敗。"),
    ("Plugin detail unavailable", "外掛詳情不可用"),
    (
        "No plugins are available in the discovered marketplaces.",
        "已發現的市場源中沒有可用外掛。",
    ),
    (
        "Add a marketplace from a Git repo or local root.",
        "從Git倉庫或本地根目錄新增市場源。",
    ),
    (
        "Enter a source to make its plugins available in this menu.",
        "輸入來源，使其外掛在此選單中可用。",
    ),
    (
        "Enter owner/repo, a Git URL, or a local marketplace path.",
        "輸入owner/repo、Git URL或本地市場源路徑。",
    ),
    (
        "This plugin is installed by your workspace admin.",
        "此外掛由你的工作區管理員安裝。",
    ),
    (
        "This remote plugin did not provide an uninstall identity.",
        "此遠端外掛未提供解除安裝標識。",
    ),
    (
        "This plugin is disabled by your workspace admin.",
        "此外掛已被你的工作區管理員禁用。",
    ),
    (
        "This plugin is not installable from this marketplace.",
        "此外掛無法從此市場源安裝。",
    ),
    (
        "This plugin did not provide an install location.",
        "此外掛未提供安裝位置。",
    ),
    ("Skills", "技能"),
    (
        "ctrl + u upgrade · ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + u升級 · ctrl + r移除 · space切換 · ←/→標籤頁 · enter詳情 · esc關閉",
    ),
    (
        "ctrl + r remove · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + r移除 · space切換 · ←/→標籤頁 · enter詳情 · esc關閉",
    ),
    (
        "ctrl + u upgrade · space toggle · ←/→ tabs · enter details · esc close",
        "ctrl + u升級 · space切換 · ←/→標籤頁 · enter詳情 · esc關閉",
    ),
    (
        "space enable/disable · ←/→ select marketplace · enter view details · esc close",
        "space啟用/禁用 · ←/→選擇市場源 · enter檢視詳情 · esc關閉",
    ),
    ("Press esc to close.", "按esc關閉。"),
    ("Disabled by admin", "已被管理員禁用"),
    ("Enabled by Admin", "已由管理員啟用"),
    ("Disabled", "已禁用"),
    ("Not installable", "不可安裝"),
    ("Can be installed", "可安裝"),
    ("Version", "版本"),
    ("Sharing", "共享"),
    ("Auth on install", "安裝時認證"),
    ("Auth on use", "使用時認證"),
    ("Listed", "已列出"),
    ("Workspace link", "工作區連結"),
    ("Private", "私有"),
    ("No explicit principals", "無顯式主體"),
    (
        "Local plugin functionality is already available.",
        "本地外掛功能已可用。",
    ),
    ("This section loaded successfully.", "此分割槽已成功載入。"),
    ("Admin assigned", "管理員指定"),
    ("Available", "可用"),
    ("No plugin skills.", "沒有外掛技能。"),
    ("No plugin apps.", "沒有外掛應用。"),
    ("No plugin hooks.", "沒有外掛鉤子。"),
    ("No plugin MCP servers.", "沒有外掛MCP伺服器。"),
    ("D I F F", "差異"),
    ("P A T C H", "補丁"),
    ("E X E C", "執行"),
    ("P E R M I S S I O N S", "許可權"),
    ("E L I C I T A T I O N", "詢問"),
    (
        "Background server started. Run `codex agents` in another terminal; this session remains unchanged.",
        "後臺服務已啟動。請在另一個終端執行`codex agents`；本會話保持不變。",
    ),
    (
        "Wait for the current task to finish before running /recap.",
        "請等待當前任務完成後再執行/recap。",
    ),
    (
        "A thread must start before it can be archived.",
        "執行緒啟動後才能歸檔。",
    ),
    (
        "A thread must start before it can be deleted.",
        "執行緒啟動後才能刪除。",
    ),
    (
        "Failed to save default service tier: {0}",
        "儲存預設服務層級失敗：{0}",
    ),
    (
        "Failed to save Agent mode warning preference: {0}",
        "儲存Agent模式警告偏好失敗：{0}",
    ),
    (
        "Failed to save rate limit reminder preference: {0}",
        "儲存速率限制提醒偏好失敗：{0}",
    ),
    (
        "Failed to save Plan mode reasoning effort: {0}",
        "儲存Plan模式推理強度失敗：{0}",
    ),
    (
        "Failed to save model migration prompt preference: {0}",
        "儲存模型遷移提示偏好失敗：{0}",
    ),
    (
        "Failed to update skill config for {0}: {1}",
        "更新技能配置失敗（{0}）：{1}",
    ),
    (
        "Failed to save status line settings: {0}",
        "儲存狀態列設定失敗：{0}",
    ),
    (
        "Failed to save terminal title items: {0}",
        "儲存終端標題項失敗：{0}",
    ),
    (
        "Open this project in VS Code or Cursor with the Codex extension active.",
        "請在已啟用Codex擴充套件的VS Code或Cursor中開啟此專案。",
    ),
    (
        "The IDE extension did not provide context.",
        "IDE擴充套件未提供上下文。",
    ),
    (
        "Codex will keep trying on future messages.",
        "Codex會在後續訊息中繼續嘗試。",
    ),
    ("{0} Try /ide again.", "{0} 請重試/ide。"),
    (
        "The selected IDE context is too large. Clear any large selection in your IDE and try /ide again.",
        "所選IDE上下文過大。請在IDE中清除較大的選區後重試/ide。",
    ),
    (
        "Codex could not request IDE context. Try /ide again.",
        "Codex無法請求IDE上下文。請重試/ide。",
    ),
    (
        "Codex could not read IDE context. Try /ide again.",
        "Codex無法讀取IDE上下文。請重試/ide。",
    ),
    (
        "The selected IDE context is too large. Clear any large selection in your IDE.",
        "所選IDE上下文過大。請在IDE中清除較大的選區。",
    ),
    (
        "Codex timed out waiting for IDE context. It will keep trying on future messages.",
        "Codex等待IDE上下文超時。它會在後續訊息中繼續嘗試。",
    ),
    (
        "The IDE connection changed while Codex was requesting context.",
        "在Codex請求上下文期間IDE連線發生了變化。",
    ),
    (
        "The IDE extension did not answer in time.",
        "IDE擴充套件未及時響應。",
    ),
    (
        "The connected IDE extension is not compatible with this IDE context request.",
        "所連線的IDE擴充套件與此IDE上下文請求不相容。",
    ),
    (
        "The connected IDE client does not support IDE context requests.",
        "所連線的IDE客戶端不支援IDE上下文請求。",
    ),
    (
        "Codex lost the IDE connection while requesting context.",
        "Codex在請求上下文期間丟失了IDE連線。",
    ),
    (
        "Codex received an unexpected IDE context response.",
        "Codex收到了意外的IDE上下文響應。",
    ),
    (
        "Codex could not read IDE context.",
        "Codex無法讀取IDE上下文。",
    ),
    ("default model and reasoning effort", "預設模型與推理強度"),
    ("default service tier", "預設服務層級"),
    ("Plan mode reasoning effort", "Plan模式推理強度"),
    (
        "Saved {0}, but a higher-priority configuration layer overrides the saved value.",
        "已儲存{0}，但更高優先順序的配置層覆蓋了儲存的值。",
    ),
    (
        "Read the API key from stdin (e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`)",
        "從標準輸入讀取API key（例如 `printenv OPENAI_API_KEY | codex login --with-api-key`）",
    ),
    (
        "Read the access token from stdin (e.g. `printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`)",
        "從標準輸入讀取訪問令牌（例如 `printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`）",
    ),
    (
        "(deprecated) Previously accepted the API key directly; now exits with guidance to use --with-api-key",
        "（已棄用）過去可直接接受API key；現在會退出並提示使用--with-api-key",
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
        "直接exec-server傳輸不支援轉發",
    ),
    ("PATH is not set", "未設定PATH"),
    (
        "--force requires a session UUID; names must be confirmed interactively",
        "--force需要會話UUID；名稱必須在互動模式下確認",
    ),
    (
        "`codex agents` received conflicting remote server endpoints",
        "`codex agents`收到了衝突的遠端服務端地址",
    ),
    (
        "`codex agents` does not accept an initial prompt or images",
        "`codex agents`不接受初始提示詞或圖片",
    ),
    (
        "`codex agents` is unavailable while workload identity is active",
        "工作負載身份啟用時`codex agents`不可用",
    ),
    (
        "`codex agents` requires `--remote` on this platform",
        "此平臺上`codex agents`需要`--remote`",
    ),
    (
        "`codex sandbox` is not supported on this operating system",
        "此作業系統不支援`codex sandbox`",
    ),
    (
        "Choose one login credential source: --with-api-key or --with-access-token.",
        "請選擇一種登入憑據來源：--with-api-key或--with-access-token。",
    ),
    (
        "Codex executable path is not configured",
        "未配置Codex可執行檔案路徑",
    ),
    (
        "--environment-id is required when --remote is set",
        "設定--remote時必須提供--environment-id",
    ),
    (
        "CODEX_ACCESS_TOKEN is required when --use-agent-identity-auth is set",
        "設定--use-agent-identity-auth時必須提供CODEX_ACCESS_TOKEN",
    ),
    (
        "Agent Identity authentication is unavailable",
        "Agent Identity認證不可用",
    ),
    (
        "CODEX_ACCESS_TOKEN did not provide permitted Agent Identity authentication",
        "CODEX_ACCESS_TOKEN未透過Agent Identity認證",
    ),
    (
        "remote exec-server registration URL must include a host",
        "遠端exec-server註冊URL必須包含主機名",
    ),
    (
        "`codex fork --worktree` requires an explicit session ID",
        "`codex fork --worktree`需要顯式的會話ID",
    ),
    (
        "`--worktree` cannot resume an existing session; use `codex exec fork --worktree`",
        "`--worktree`無法恢復已有會話；請使用`codex exec fork --worktree`",
    ),
    (
        "`--worktree` is not supported for code review",
        "程式碼審查不支援`--worktree`",
    ),
    ("Continue anyway? [y/N]: ", "仍要繼續？[y/N]："),
    ("stdin is not a terminal", "stdin不是終端"),
    ("stdout is not a terminal", "stdout不是終端"),
    (
        "`--remote-auth-token-env` requires `--remote`.",
        "`--remote-auth-token-env`需要`--remote`。",
    ),
    (
        "`--remote-auth-token-env` requires a `wss://` or loopback `ws://` remote.",
        "`--remote-auth-token-env`需要`wss://`或迴環`ws://`遠端地址。",
    ),
    (
        "failed to resolve socket path `{0}`: {1}",
        "解析socket路徑`{0}`失敗：{1}",
    ),
    ("ERROR: {0}", "錯誤：{0}"),
    ("Updating Codex via `{0}`...", "正在透過`{0}`更新Codex…"),
    ("`{0}` failed with status {1}", "`{0}`失敗，狀態碼{1}"),
    (
        "Could not find an absolute update command `{0}` on PATH. Please update manually: https://developers.openai.com/codex/cli/",
        "在PATH中找不到絕對路徑的更新命令`{0}`。請手動更新：https://developers.openai.com/codex/cli/",
    ),
    (
        "could not find update command `{0}` on PATH",
        "在PATH中找不到更新命令`{0}`",
    ),
    (
        "Could not detect the Codex installation method. Please update manually: https://developers.openai.com/codex/cli/",
        "無法識別Codex的安裝方式。請手動更新：https://developers.openai.com/codex/cli/",
    ),
    ("Unknown feature flag: {0}", "未知的特性開關：{0}"),
    (
        "invalid remote exec-server registration URL: {0}",
        "無效的遠端exec-server註冊URL：{0}",
    ),
    (
        "Enabled feature `{0}` in config.toml.",
        "已在config.toml中啟用特性`{0}`。",
    ),
    (
        "Disabled feature `{0}` in config.toml.",
        "已在config.toml中禁用特性`{0}`。",
    ),
    ("Cleared memory state from {0}.", "已清除{0}的記憶狀態。"),
    ("No memories db found at {0}.", "在{0}未找到記憶資料庫。"),
    (
        "`--strict-config` is not supported for `codex {0}`",
        "`codex {0}`不支援`--strict-config`",
    ),
    (
        "warning: failed to load updater network configuration: {0}",
        "警告：載入更新器網路配置失敗：{0}",
    ),
    (
        "environment variable `{0}` is not set",
        "環境變數`{0}`未設定",
    ),
    ("environment variable `{0}` is empty", "環境變數`{0}`為空"),
    (
        "\n🎉 Update ran successfully! Please restart Codex.",
        "\n🎉 更新成功！請重啟Codex。",
    ),
    (
        "--worktree cannot be combined with --ignore-user-config",
        "--worktree不能與--ignore-user-config同時使用",
    ),
    (
        "--worktree cannot be combined with --ephemeral",
        "--worktree不能與--ephemeral同時使用",
    ),
    (
        "--worktree is not supported with `codex exec resume`",
        "`codex exec resume`不支援--worktree",
    ),
    (
        "--worktree is not supported with `codex exec review`",
        "`codex exec review`不支援--worktree",
    ),
    (
        "--worktree requires local execution",
        "--worktree需要本地執行",
    ),
    (
        "--worktree requires the worktrees feature; enable it with --enable worktrees",
        "--worktree需要worktrees特性；請用--enable worktrees啟用",
    ),
    (
        "--worktree requires a source that is not explicitly untrusted",
        "--worktree要求來源未被明確標記為不受信任",
    ),
    (
        "Could not create otel exporter: panicked during initialization",
        "無法建立otel匯出器：初始化時發生panic",
    ),
    (
        "Forking with images requires a prompt",
        "帶圖片分叉需要提供提示詞",
    ),
    (
        "Forking with output options requires a prompt",
        "帶輸出選項分叉需要提供提示詞",
    ),
    ("Ephemeral forks require a prompt", "臨時分叉需要提供提示詞"),
    (
        "Not inside a trusted directory and --skip-git-repo-check was not specified.",
        "不在受信任目錄內，且未指定--skip-git-repo-check。",
    ),
    ("Reading prompt from stdin...", "正在從標準輸入讀取提示詞…"),
    (
        "Reading additional input from stdin...",
        "正在從標準輸入讀取附加輸入…",
    ),
    (
        "No prompt provided via stdin.",
        "未透過標準輸入提供提示詞。",
    ),
    ("Review prompt cannot be empty", "審查提示詞不能為空"),
    (
        "Specify --uncommitted, --base, --commit, or provide custom review instructions",
        "請指定--uncommitted、--base、--commit，或提供自定義審查說明",
    ),
    (
        "--worktree requires a source that is not explicitly untrusted; unused checkout at {0} remains. Remove it manually with `git worktree remove` when safe",
        "--worktree要求來源未被明確標記為不受信任；{0}處仍留有未使用的檢出。請在安全時用`git worktree remove`手動移除",
    ),
    (
        "No default OSS provider configured. Use --local-provider=provider or set oss_provider to one of: {0}, {1} in config.toml",
        "未配置預設OSS提供方。請使用--local-provider=provider，或在config.toml中把oss_provider設為{0}、{1}之一",
    ),
    ("Error loading rules:\n{0}", "載入規則失敗：\n{0}"),
    (
        "Error loading config.toml:\n{0}",
        "載入config.toml失敗：\n{0}",
    ),
    (
        "Failed to read output schema file {0}: {1}",
        "讀取輸出schema檔案{0}失敗：{1}",
    ),
    (
        "Output schema file {0} is not valid JSON: {1}",
        "輸出schema檔案{0}不是合法JSON：{1}",
    ),
    ("Error parsing -c overrides: {0}", "解析-c覆蓋項失敗：{0}"),
    ("Error finding codex home: {0}", "查詢codex主目錄失敗：{0}"),
    (
        "Could not create otel exporter: {0}",
        "無法建立otel匯出器：{0}",
    ),
    ("Error loading config.toml: {0}", "載入config.toml失敗：{0}"),
    ("OSS setup failed: {0}", "OSS設定失敗：{0}"),
    (
        "failed to initialize in-process app-server client: {0}",
        "初始化程序內app-server客戶端失敗：{0}",
    ),
    ("Session not found: {0}", "未找到會話：{0}"),
    (
        "in-process app-server shutdown failed: {0}",
        "程序內app-server關閉失敗：{0}",
    ),
    (
        "input is not valid UTF-8 (invalid byte at offset {0}). Convert it to UTF-8 and retry (e.g., `iconv -f <ENC> -t UTF-8 prompt.txt`).",
        "輸入不是合法的UTF-8（偏移{0}處的位元組無效）。請轉換為UTF-8後重試（例如`iconv -f <ENC> -t UTF-8 prompt.txt`）。",
    ),
    (
        "input looked like {0} but could not be decoded. Convert it to UTF-8 and retry.",
        "輸入看起來是{0}但無法解碼。請轉換為UTF-8後重試。",
    ),
    (
        "input appears to be {0}. Convert it to UTF-8 and retry.",
        "輸入似乎是{0}。請轉換為UTF-8後重試。",
    ),
    (
        "No prompt provided. Either specify one as an argument or pipe the prompt into stdin.",
        "未提供提示詞。請將其作為引數傳入，或透過標準輸入管道輸入。",
    ),
    (
        "Failed to read prompt from stdin: {0}",
        "從標準輸入讀取提示詞失敗：{0}",
    ),
    (
        "features.token_budget.reminder_threshold_tokens must be positive",
        "features.token_budget.reminder_threshold_tokens必須為正數",
    ),
    (
        "features.token_budget.reminder_message_template must not be empty",
        "features.token_budget.reminder_message_template不能為空",
    ),
    (
        "features.token_budget.auto_compact_fallback_buffer_tokens is required when auto_compact_fallback_prompt is set",
        "設定auto_compact_fallback_prompt時必須提供features.token_budget.auto_compact_fallback_buffer_tokens",
    ),
    (
        "features.token_budget.auto_compact_fallback_buffer_tokens must be positive",
        "features.token_budget.auto_compact_fallback_buffer_tokens必須為正數",
    ),
    (
        "projects table missing after initialization",
        "初始化後缺少projects表",
    ),
    (
        "features.rollout_budget.limit_tokens is required when rollout_budget is enabled",
        "啟用rollout_budget時必須提供features.rollout_budget.limit_tokens",
    ),
    (
        "features.rollout_budget.limit_tokens must be positive",
        "features.rollout_budget.limit_tokens必須為正數",
    ),
    (
        "features.rollout_budget.reminder_at_remaining_tokens is required when rollout_budget is enabled",
        "啟用rollout_budget時必須提供features.rollout_budget.reminder_at_remaining_tokens",
    ),
    (
        "features.rollout_budget.reminder_at_remaining_tokens must contain only positive values below limit_tokens",
        "features.rollout_budget.reminder_at_remaining_tokens只能包含小於limit_tokens的正數值",
    ),
    (
        "`experimental_thread_store_endpoint` is no longer supported; remove it from config.toml",
        "不再支援`experimental_thread_store_endpoint`；請從config.toml中移除",
    ),
    (
        "`--dangerously-bypass-hook-trust` is enabled. Enabled hooks may run without review for this invocation.",
        "已啟用`--dangerously-bypass-hook-trust`。本次呼叫中已啟用的鉤子可能不經審查就執行。",
    ),
    (
        "`sandbox_mode` and `permission_profile` overrides cannot both be set",
        "`sandbox_mode`與`permission_profile`覆蓋不能同時設定",
    ),
    (
        "`sandbox_mode` and `default_permissions` overrides cannot both be set",
        "`sandbox_mode`與`default_permissions`覆蓋不能同時設定",
    ),
    (
        "`permission_profile` and `default_permissions` overrides cannot both be set",
        "`permission_profile`與`default_permissions`覆蓋不能同時設定",
    ),
    (
        "config defines `[permissions]` profiles but does not set `default_permissions`",
        "配置定義了`[permissions]`配置檔，但未設定`default_permissions`",
    ),
    (
        "features.multi_agent_v2.max_concurrent_threads_per_session must be at least 1",
        "features.multi_agent_v2.max_concurrent_threads_per_session至少為1",
    ),
    (
        "features.multi_agent_v2.min_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms",
        "features.multi_agent_v2.min_wait_timeout_ms不能大於features.multi_agent_v2.max_wait_timeout_ms",
    ),
    (
        "features.multi_agent_v2.default_wait_timeout_ms must be at least features.multi_agent_v2.min_wait_timeout_ms",
        "features.multi_agent_v2.default_wait_timeout_ms不能小於features.multi_agent_v2.min_wait_timeout_ms",
    ),
    (
        "features.multi_agent_v2.default_wait_timeout_ms must be at most features.multi_agent_v2.max_wait_timeout_ms",
        "features.multi_agent_v2.default_wait_timeout_ms不能大於features.multi_agent_v2.max_wait_timeout_ms",
    ),
    (
        "agents.max_concurrent_threads_per_session must be at least 1",
        "agents.max_concurrent_threads_per_session至少為1",
    ),
    (
        "thread_unload_delay_secs is too large",
        "thread_unload_delay_secs過大",
    ),
    (
        "`approval_policy = \"never\"` cannot be used because requirements do not allow `sandbox_mode = \"danger-full-access\"`; Codex would fall back to read-only permissions with approvals disabled. Choose an `approval_policy` based on what you need, such as `on-request`, or choose an allowed sandbox mode.",
        "`approval_policy = \"never\"`不可用，因為requirements不允許`sandbox_mode = \"danger-full-access\"`；Codex將退化為禁用審批的只讀許可權。請根據需求選擇`approval_policy`（例如`on-request`），或選擇允許的沙箱模式。",
    ),
    (
        "goals.max_goal_token_budget exceeds the maximum supported token budget",
        "goals.max_goal_token_budget超過支援的最大token預算",
    ),
    (
        "requirements.toml default_permissions requires allowed_permission_profiles",
        "requirements.toml在設定default_permissions時需要allowed_permission_profiles",
    ),
    (
        "requirements.toml default_permissions must be set unless allowed_permission_profiles allows both `:workspace` and `:read-only`",
        "requirements.toml的default_permissions必須設定，除非allowed_permission_profiles同時允許`:workspace`和`:read-only`",
    ),
    ("{0} (non-admin sandbox)", "{0}（非管理員沙箱）"),
    (
        "The non-admin sandbox protects your files and prevents network access under most circumstances. However, it carries greater risk if prompt injected. To upgrade to the default sandbox, run ",
        "非管理員沙箱在多數情況下會保護你的檔案並阻止網路訪問。但若遭遇提示詞注入，風險更高。要升級到預設沙箱，請執行 ",
    ),
    (
        "No recent auto-review denials in this thread.",
        "此執行緒中沒有最近的自動審查拒絕記錄。",
    ),
    (
        "Denials are recorded after auto-review rejects an action.",
        "自動審查拒絕某個操作後會記錄拒絕項。",
    ),
    ("That thread is no longer available.", "該執行緒已不可用。"),
    ("Auto-review Denials", "自動審查拒絕記錄"),
    (
        "Select a denied action to approve.",
        "選擇一條被拒絕的操作進行批准。",
    ),
    (
        "That auto-review denial is no longer available.",
        "該自動審查拒絕記錄已不可用。",
    ),
    (
        "Approval recorded for one retry of the selected auto-review denial.",
        "已記錄批准，所選自動審查拒絕項可重試一次。",
    ),
    (
        "The model will see the approval context; the retry still goes through auto-review.",
        "模型會看到批准上下文；重試仍會經過自動審查。",
    ),
    ("Enable full access?", "啟用完全訪問？"),
    (
        "We strongly recommend selecting \"Approve for me\" instead, and customizing the reviewer policy for your use case.",
        "我們強烈建議改為選擇「替我批准」，並按你的使用場景自定義審查策略。",
    ),
    (
        "We strongly recommend selecting \"Ask for approval\" instead.",
        "我們強烈建議改為選擇「請求批准」。",
    ),
    (
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval.",
        "Codex以完全訪問執行時，可以不經你批准就編輯電腦上的任何檔案並執行帶網路的命令。",
    ),
    (
        "Cyber models carry a higher risk of dangerous actions.",
        "網路安全模型帶來危險操作的風險更高。",
    ),
    (
        " Ensure proper safeguards are in place before granting full access. ",
        " 在授予完全訪問前，請確保已設定適當的防護措施。 ",
    ),
    (
        "When Codex runs with full access, it can edit any file on your computer and run commands with network, without your approval. ",
        "Codex以完全訪問執行時，可以不經你批准就編輯電腦上的任何檔案並執行帶網路的命令。 ",
    ),
    (
        "Exercise caution when enabling full access. This significantly increases the risk of data loss, leaks, or unexpected behavior.",
        "啟用完全訪問時請謹慎。這會顯著增加資料丟失、洩露或意外行為的風險。",
    ),
    ("Yes, continue anyway", "是，仍然繼續"),
    (
        "Apply full access for this session",
        "為本次會話應用完全訪問",
    ),
    (
        "Go back without enabling full access",
        "返回且不啟用完全訪問",
    ),
    ("Shared agents unavailable", "共享代理不可用"),
    (
        "The agents dashboard is unavailable while workload identity is active.",
        "工作負載身份啟用時，代理面板不可用。",
    ),
    (
        "This session isn’t connected to a shared background server.",
        "本會話未連線到共享後臺服務。",
    ),
    (
        "Connect to a remote background server to use the agents dashboard.",
        "請連線到遠端後臺服務以使用代理面板。",
    ),
    (
        "Starting a background server will not interrupt or move this session.",
        "啟動後臺服務不會中斷或遷移本會話。",
    ),
    ("Start background server", "啟動後臺服務"),
    (
        "Open `codex agents` in another terminal afterward.",
        "之後在另一個終端中執行`codex agents`。",
    ),
    ("Return to this session", "返回本會話"),
    (
        "Cannot resume task without preserving the selected permissions.",
        "不能在未保留所選許可權的情況下恢復任務。",
    ),
    (
        "Permission profile has different settings.",
        "許可權配置檔的設定不同。",
    ),
    ("Failed to load shared agents: {0}", "載入共享代理失敗：{0}"),
    ("Failed to load task settings: {0}", "載入任務設定失敗：{0}"),
    ("Failed to attach to task: {0}", "附加到任務失敗：{0}"),
    (
        "Failed to load project settings: {0}",
        "載入專案設定失敗：{0}",
    ),
    (
        "Failed to start background task: {0}",
        "啟動後臺任務失敗：{0}",
    ),
    ("Failed to send task message: {0}", "傳送任務訊息失敗：{0}"),
    (
        "Failed to stop background task: {0}",
        "停止後臺任務失敗：{0}",
    ),
    ("daemon process exited with {0}", "守護程序退出，狀態{0}"),
    ("Bundled apps remain installed.", "捆綁應用仍保持安裝。"),
    ("Already installed in this session.", "本次會話中已安裝。"),
    (
        "Install the required Apps in ChatGPT to continue:",
        "請先在ChatGPT中安裝所需應用以繼續：",
    ),
    (
        "Open the ChatGPT app management page",
        "開啟ChatGPT應用管理頁",
    ),
    (
        "Open the app page in your browser.",
        "在瀏覽器中開啟該應用頁面。",
    ),
    ("ChatGPT apps link unavailable", "ChatGPT應用連結不可用"),
    (
        "This app did not provide an install/manage URL.",
        "該應用未提供安裝/管理連結。",
    ),
    ("This app is already installed.", "該應用已安裝。"),
    ("Advance to the next app.", "繼續到下一個應用。"),
    (
        "Trust your confirmation and continue to the next app.",
        "信任你的確認並繼續到下一個應用。",
    ),
    (
        "Continue without waiting for refresh to complete.",
        "不再等待重新整理完成，直接繼續。",
    ),
    ("Skip remaining app setup", "跳過剩餘應用設定"),
    (
        "Stop this follow-up flow for this plugin.",
        "停止此外掛的後續流程。",
    ),
    (
        "Abandon remaining required app setup.",
        "放棄剩餘必需的應用設定。",
    ),
    (
        "The plugin may not be usable until required apps are installed.",
        "在安裝所需應用前，該外掛可能無法使用。",
    ),
    (
        "You can now continue managing plugins from /plugins.",
        "現在可以繼續透過/plugins管理外掛。",
    ),
    ("Uninstalled {0} plugin.", "已解除安裝外掛{0}。"),
    ("{0} plugin installed.", "已安裝外掛{0}。"),
    ("App setup {0}/{1}: {2}", "應用設定{0}/{1}：{2}"),
    (
        "Failed to update plugin config for {0}: {1}",
        "更新外掛配置失敗（{0}）：{1}",
    ),
    (
        "Skipped remaining app setup for {0} plugin.",
        "已跳過外掛{0}的剩餘應用設定。",
    ),
    (
        "Completed app setup flow for {0} plugin.",
        "已完成外掛{0}的應用設定流程。",
    ),
    (
        "  Checking for existing AWS credentials...",
        "  正在檢查現有的AWS憑據…",
    ),
    (
        "  Setting up Amazon Bedrock...",
        "  正在設定Amazon Bedrock…",
    ),
    ("  Choose an AWS profile.", "  請選擇AWS配置檔。"),
    (
        "  AWS credentials detected in your environment.",
        "  在你的環境中檢測到AWS憑據。",
    ),
    ("  No AWS credentials found.", "  未找到AWS憑據。"),
    (
        "  Choose how you authenticate with AWS.",
        "  請選擇你與AWS的認證方式。",
    ),
    (
        "  Enter the name of your AWS profile.",
        "  請輸入AWS配置檔名稱。",
    ),
    ("  AWS profile: ", "  AWS配置檔："),
    (
        "  Enter your Amazon Bedrock API key.",
        "  請輸入你的Amazon Bedrock API key。",
    ),
    ("  Bedrock API key: ", "  Bedrock API key："),
    (
        "  Enter the AWS Region to use with Amazon Bedrock.",
        "  請輸入與Amazon Bedrock一起使用的AWS區域。",
    ),
    ("  AWS Region: ", "  AWS區域："),
    ("  Enter your AWS access keys.", "  請輸入你的AWS訪問金鑰。"),
    (
        "  Configure AWS credentials in your environment, then restart Codex.",
        "  請在你的環境中配置AWS憑據，然後重啟Codex。",
    ),
    ("  Setup guide: ", "  設定指南："),
    ("Enter a Bedrock API key", "輸入Bedrock API key"),
    ("  AWS profile detected: {0}", "  檢測到AWS配置檔：{0}"),
    ("  Region: {0}", "  區域：{0}"),
    ("shift+tab to cycle", "shift+tab切換"),
    ("Plan mode{0}", "Plan模式{0}"),
    ("Pursuing goal ({0})", "正在推進目標（{0}）"),
    ("Pursuing goal", "正在推進目標"),
    ("Goal paused (/goal resume)", "目標已暫停（/goal resume）"),
    ("Goal stalled (/goal resume)", "目標已停滯（/goal resume）"),
    (
        "Goal hit usage limits (/goal resume)",
        "目標已達用量上限（/goal resume）",
    ),
    ("Goal unmet ({0})", "目標未滿足（{0}）"),
    ("Goal abandoned", "目標已放棄"),
    ("Goal achieved ({0})", "目標已達成（{0}）"),
    ("Goal achieved", "目標已達成"),
    ("reverse-i-search: ", "反向增量搜尋："),
    ("{0} for side", "{0} 進入側邊會話"),
    ("from main thread", "來自主執行緒"),
    ("from parent thread ({0})", "來自父執行緒（{0}）"),
    ("{0} to switch", "{0} 切換"),
    ("ctrl + c to close", "ctrl + c關閉"),
    ("Side {0}", "側邊會話 {0}"),
    (
        "Wait for permissions to update before forking.",
        "請等待許可權更新完成後再分叉。",
    ),
    (
        "Failed to close side conversation {0}; it is still open: {1}",
        "關閉側邊會話{0}失敗；它仍處於開啟狀態：{1}",
    ),
    (
        "Failed to start side conversation: {0}",
        "啟動側邊會話失敗：{0}",
    ),
    (
        "Failed to prepare side conversation {0}: {1}",
        "準備側邊會話{0}失敗：{1}",
    ),
    (
        "Failed to switch into side conversation {0}: {1}",
        "切換到側邊會話{0}失敗：{1}",
    ),
    (
        "Failed to switch into side conversation {0}.",
        "切換到側邊會話{0}失敗。",
    ),
    ("missing spritesheet {0}", "缺少精靈圖{0}"),
    ("CODEX_HOME is not available", "CODEX_HOME不可用"),
    ("unknown pet {0}", "未知的寵物{0}"),
    ("pet path {0}", "寵物路徑{0}"),
    (
        "pet json path has no containing directory",
        "寵物json路徑沒有所在目錄",
    ),
    ("resolve {0}", "解析路徑{0}"),
    (
        "missing pet.json or avatar.json in {0}",
        "在{0}中缺少pet.json或avatar.json",
    ),
    ("parse {0}", "解析{0}"),
    (
        "spritesheet path must stay inside {0}",
        "精靈圖路徑必須位於{0}內",
    ),
    (
        "pet frame dimensions and grid counts must be non-zero",
        "寵物幀尺寸與網格數量必須為非零",
    ),
    ("pet frame grid width overflow", "寵物幀網格寬度溢位"),
    ("pet frame grid height overflow", "寵物幀網格高度溢位"),
    ("pet frame count overflow", "寵物幀數量溢位"),
    (
        "pet frame count does not fit usize",
        "寵物幀數量超出usize範圍",
    ),
    (
        "pet frame count {0} exceeds maximum {1}",
        "寵物幀數量{0}超過上限{1}",
    ),
    ("HOME is not set", "未設定HOME"),
    (
        "animation {0} must include at least one frame",
        "動畫{0}必須至少包含一幀",
    ),
    ("secondary usage", "次級用量"),
    (
        "Heads up, you have less than {0}% of your {1} limit left. Run /status for a breakdown.",
        "提醒：你的{1}限額僅剩不到{0}%。執行/status檢視明細。",
    ),
    (
        "Uses fewer credits for upcoming turns.",
        "後續輪次消耗更少額度。",
    ),
    ("Switch to {0}", "切換到{0}"),
    ("Keep current model", "保持當前模型"),
    (
        "Keep current model (never show again)",
        "保持當前模型（不再提示）",
    ),
    (
        "Hide future rate limit reminders about switching models.",
        "不再顯示關於切換模型的速率限制提醒。",
    ),
    ("Approaching rate limits", "接近速率限制"),
    (
        "Switch to {0} for lower credit usage?",
        "切換到{0}以降低額度消耗？",
    ),
    (
        "Your workspace is out of credits. Ask your workspace owner to add more. Notify owner?",
        "你的工作區額度已用盡。請讓工作區所有者補充額度。要通知所有者嗎？",
    ),
    ("Usage limit reached", "已達用量上限"),
    (
        "Request a limit increase from your owner to continue using codex. Request increase?",
        "請向所有者申請提高限額以繼續使用codex。要申請提高嗎？",
    ),
    ("Workspace owner notified.", "已通知工作區所有者。"),
    (
        "Workspace owner was already notified recently.",
        "最近已通知過工作區所有者。",
    ),
    (
        "Could not notify your workspace owner. Please try again.",
        "無法通知工作區所有者。請重試。",
    ),
    ("Limit increase requested.", "已申請提高限額。"),
    (
        "A limit increase was already requested recently.",
        "最近已申請過提高限額。",
    ),
    (
        "Could not request a limit increase. Please try again.",
        "無法申請提高限額。請重試。",
    ),
    ("Saved conversation to {0}", "會話已儲存到{0}"),
    ("could not load conversation: {0}", "無法載入會話：{0}"),
    (
        "could not load conversation history: {0}",
        "無法載入會話歷史：{0}",
    ),
    ("could not create {0}: {1}", "無法建立{0}：{1}"),
    (
        "No active conversation to export.",
        "沒有可匯出的活動會話。",
    ),
    (
        "No conversation content to export.",
        "沒有可匯出的會話內容。",
    ),
    ("could not determine the home directory", "無法確定主目錄"),
    ("Archived", "已歸檔"),
    ("Deleted", "已刪除"),
    ("Unarchived", "已取消歸檔"),
    ("{0} session {1} ({2}).", "{0}會話 {1}（{2}）。"),
    ("{0} session {1}.", "{0}會話 {1}。"),
    ("Permanently delete session {0}?", "確定永久刪除會話{0}？"),
    ("Continue? [y/N]: ", "繼續？[y/N]："),
    ("failed to parse -c overrides: {0}", "解析-c覆蓋項失敗：{0}"),
    (
        "app server returned invalid session id `{0}`",
        "app server返回了無效的會話id `{0}`",
    ),
    ("failed to find Codex home", "找不到Codex主目錄"),
    (
        "failed to resolve local runtime paths",
        "解析本地執行時路徑失敗",
    ),
    (
        "failed to discover execution environments",
        "發現執行環境失敗",
    ),
    ("failed to resolve config cwd", "解析配置工作目錄失敗"),
    ("failed to load config.toml", "載入config.toml失敗"),
    ("failed to load configuration", "載入配置失敗"),
    (
        "failed to initialize environment manager",
        "初始化環境管理器失敗",
    ),
    (
        "failed to initialize state database",
        "初始化狀態資料庫失敗",
    ),
    ("Delete cancelled.", "已取消刪除。"),
    (
        "cannot confirm session deletion without an interactive terminal; rerun with --force and a session UUID",
        "沒有互動式終端時無法確認刪除會話；請加--force並提供會話UUID重試",
    ),
    (
        "This cannot be undone. Subagent threads will also be deleted.",
        "此操作無法撤銷。子代理執行緒也會被刪除。",
    ),
    (
        "Usage: /goal [<objective>|clear|edit|pause|resume]",
        "用法：/goal [<objective>|clear|edit|pause|resume]",
    ),
    ("Not available in TUI yet.", "TUI中尚不可用。"),
    (
        "This sub-agent is controlled by its parent. Direct input is disabled.",
        "此子代理由其父級控制。已禁用直接輸入。",
    ),
    ("Run the tool and continue.", "執行該工具並繼續。"),
    ("Allow this request and continue.", "允許該請求並繼續。"),
    (
        "Run the tool and remember this choice for this session.",
        "執行該工具，並在本次會話中記住此選擇。",
    ),
    (
        "Allow this request and remember this choice for this session.",
        "允許該請求，並在本次會話中記住此選擇。",
    ),
    ("Allow for this session", "本次會話內允許"),
    (
        "Run the tool and remember this choice for future tool calls.",
        "執行該工具，並在今後的工具呼叫中記住此選擇。",
    ),
    (
        "Allow this request and remember this choice for future requests.",
        "允許該請求，並在今後的請求中記住此選擇。",
    ),
    ("Always allow", "始終允許"),
    ("Cancel this tool call", "取消此次工具呼叫"),
    ("Decline this request and continue.", "拒絕該請求並繼續。"),
    ("←/→ to navigate fields", "←/→ 切換欄位"),
    (
        "ctrl + p / ctrl + n change field",
        "ctrl + p / ctrl + n切換欄位",
    ),
    ("esc to cancel", "esc取消"),
    (
        "Answer required fields before submitting.",
        "提交前請先填寫必填欄位。",
    ),
    ("No options", "沒有選項"),
    ("No fields", "沒有欄位"),
    ("{0} to submit", "{0} 提交"),
    ("{0} to submit answer", "{0} 提交答案"),
    ("{0} to submit all", "{0} 全部提交"),
    ("Field {0}/{1}", "欄位 {0}/{1}"),
    ("{0} ({1} required unanswered)", "{0}（{1} 個必填項未填寫）"),
    ("Allow", "允許"),
    ("Deny", "拒絕"),
    ("Confirm", "確認"),
    ("True", "真"),
    ("False", "假"),
    ("Apps are disabled.", "應用已禁用。"),
    (
        "Enable the apps feature to use $ or /apps.",
        "請啟用應用特性以使用 $ 或 /apps。",
    ),
    ("No apps available.", "沒有可用的應用。"),
    (
        "Loading installed and available apps...",
        "正在載入已安裝和可用的應用…",
    ),
    ("Loading apps...", "正在載入應用…"),
    (
        "This updates when the full list is ready.",
        "完整列表就緒後會更新。",
    ),
    ("Failed to load apps.", "載入應用失敗。"),
    ("App directory unavailable", "應用目錄不可用"),
    (
        "The app directory request failed. Retry, or press Esc to continue.",
        "應用目錄請求失敗。請重試，或按Esc繼續。",
    ),
    (
        "Reload installed and available apps.",
        "重新載入已安裝和可用的應用。",
    ),
    (
        "Use $ to insert an installed app into your prompt.",
        "使用 $ 把已安裝的應用插入提示詞。",
    ),
    ("Manage this app in your browser.", "在瀏覽器中管理此應用。"),
    (
        "Install this app in your browser, then reload Codex.",
        "在瀏覽器中安裝此應用，然後重新載入Codex。",
    ),
    ("Type to search apps", "輸入以搜尋應用"),
    ("Installed · Disabled", "已安裝 · 已停用"),
    (
        "Installed {0} of {1} available apps.",
        "已安裝 {0}/{1} 個可用應用。",
    ),
    (
        "{0}. Press Enter to open the app page to install, manage, or enable/disable this app.",
        "{0}。按Enter開啟應用頁面以安裝、管理或啟用/停用此應用。",
    ),
    (
        "{0}. Press Enter to open the app page to install this app.",
        "{0}。按Enter開啟應用頁面以安裝此應用。",
    ),
    ("{0}. App link unavailable.", "{0}。應用連結不可用。"),
    (
        "Your usage does not need a reset right now.",
        "你的用量目前無需重置。",
    ),
    (
        "That reset is no longer available. Refresh to see your current resets.",
        "該重置項已不可用。請重新整理以檢視當前的重置項。",
    ),
    (
        "No usage limit resets are available.",
        "沒有可用的用量限額重置項。",
    ),
    ("Usage reset.", "用量已重置。"),
    ("usage limit reset", "用量限額重置項"),
    ("usage limit resets", "用量限額重置項"),
    (
        "Usage reset. You have {0} {1} left.",
        "用量已重置。你還剩{0}個{1}。",
    ),
    (
        "You have {0} {1} available. Run /usage to use one.",
        "你有{0}個{1}可用。執行/usage使用一個。",
    ),
    (
        "No compatible setup was found to import.",
        "未找到可匯入的相容設定。",
    ),
    (
        "Import from other apps is unavailable in remote sessions. Start Codex locally and run /import.",
        "遠端會話中不支援從其他應用匯入。請在本地啟動Codex並執行/import。",
    ),
    (
        "Import from other apps is unavailable while Codex is connected to the local app-server daemon. Stop the daemon, restart Codex, and run /import.",
        "當Codex連線到本地app-server守護程序時，不支援從其他應用匯入。請停止守護程序、重啟Codex並執行/import。",
    ),
    (", +{0} more", "，另有{0}項"),
    ("{0} failed", "{0}項失敗"),
    ("{0} imported", "已匯入{0}項"),
    ("Import failed: {0}", "匯入失敗：{0}"),
    (
        "{0} additional items remain. After it finishes, run /import again to review them.",
        "還剩{0}個額外專案。完成後請再次執行/import檢視。",
    ),
    ("Import started.", "匯入已開始。"),
    (
        " You can keep working while it finishes.",
        " 期間可繼續工作。",
    ),
    (
        "Imported setup will apply to new chats.",
        "匯入的設定將應用於新會話。",
    ),
    ("Import finished: ", "匯入完成："),
    ("Results by type:", "按型別統計："),
    (
        "Run /import again to check for additional items.",
        "再次執行/import以檢查更多專案。",
    ),
    (
        "1 additional item remains. After it finishes, run /import again to review it.",
        "還剩1個額外專案。完成後請再次執行/import檢視。",
    ),
    (
        "Selected import source is no longer available.",
        "所選匯入來源已不可用。",
    ),
    (" scroll disclosure", " 滾動展開"),
    ("to submit", "提交"),
    (
        "Your data may be used to improve our models and products",
        "你的資料可能被用於改進我們的模型和產品",
    ),
    (
        "Your feedback can be used to improve ChatGPT. ",
        "你的反饋可用於改進ChatGPT。",
    ),
    ("Tell us more (bad result)", "告訴我們更多（結果不佳）"),
    (
        "(optional) Write a short description to help us further",
        "（可選）寫一段簡短描述，幫助我們進一步瞭解",
    ),
    ("Tell us more (good result)", "告訴我們更多（結果良好）"),
    ("Tell us more (bug)", "告訴我們更多（缺陷）"),
    ("Tell us more (safety check)", "告訴我們更多（安全檢查）"),
    (
        "(optional) Share what was refused and why it should have been allowed",
        "（可選）說明被拒絕的內容，以及為何本應被允許",
    ),
    ("Tell us more (other)", "告訴我們更多（其他）"),
    (
        "By submitting feedback, you agree that OpenAI can use your feedback for safety purposes and internal model training, as explained in more detail ",
        "提交反饋即表示你同意OpenAI將你的反饋用於安全目的和內部模型訓練，詳見",
    ),
    ("Action: ", "操作："),
    (
        "Agent thread {0} closed. Failed to switch back to main thread {1}.",
        "智慧體執行緒{0}已關閉。無法切回主執行緒{1}。",
    ),
    (
        "Agent thread {0} closed. Switched back to main thread.",
        "智慧體執行緒{0}已關閉。已切回主執行緒。",
    ),
    ("Current: ", "當前："),
    ("Failed to interrupt turn: {0}", "無法中斷該輪次：{0}"),
    (
        "Failed to resolve app-server request for thread {0}: {1}",
        "無法處理執行緒{0}的app-server請求：{1}",
    ),
    (
        "First key: {0}. Press the second key. Esc cancels.",
        "第一個鍵：{0}。請按第二個鍵。按Esc取消。",
    ),
    (
        "Press the first key, then the second. Esc cancels.",
        "請依次按第一個鍵和第二個鍵。按Esc取消。",
    ),
    (
        "Press the new key now. Esc cancels.",
        "請現在按下新鍵。按Esc取消。",
    ),
    ("Remap Shortcut", "重新對映快捷鍵"),
    ("failed to load local auth: {0}", "載入本地認證失敗：{0}"),
    ("failed to refresh skills", "重新整理技能失敗"),
    (
        "local ChatGPT auth is missing chatgpt account id",
        "本地ChatGPT認證缺少chatgpt賬戶ID",
    ),
    (
        "local ChatGPT auth is missing token data",
        "本地ChatGPT認證缺少令牌資料",
    ),
    (
        "local ChatGPT auth must use one of workspace(s) {0}, but found {1}",
        "本地ChatGPT認證必須使用工作區{0}中的一個，但發現{1}",
    ),
    (
        "local auth is not a ChatGPT login",
        "本地認證不是ChatGPT登入",
    ),
    ("no local auth available", "沒有可用的本地認證"),
    ("Run ", "執行"),
    (" to update.", "以完成更新。"),
    ("See ", "參見"),
    (" for installation options.", "，瞭解安裝選項。"),
    (
        "See https://github.com/openai/codex for installation options.",
        "參見https://github.com/openai/codex瞭解安裝選項。",
    ),
    ("Generating conversation recap", "正在生成對話回顧"),
    ("Generating conversation recap...", "正在生成對話回顧…"),
    ("✘ Failed to apply patch", "✘ 應用補丁失敗"),
    ("Image generation failed", "影象生成失敗"),
    ("Generated Image:", "生成的影象："),
    ("Saved to: ", "儲存到："),
    ("Proposed Plan", "建議計劃"),
    ("Updated Plan", "已更新計劃"),
    ("(no steps provided)", "（未提供步驟）"),
    ("Searched the web", "已搜尋網路"),
    ("Searching the web", "正在搜尋網路"),
    ("⚠ {0} MCP startup issue", "⚠ {0} 個MCP啟動問題"),
    ("⚠ {0} MCP startup issues", "⚠ {0} 個MCP啟動問題"),
    ("⚠ {0} startup issue", "⚠ {0} 個啟動問題"),
    ("⚠ {0} startup issues", "⚠ {0} 個啟動問題"),
    ("{0} MCP", "{0} 個MCP"),
    ("{0} needs sign-in", "{0} 個需要登入"),
    ("{0} need sign-in", "{0} 個需要登入"),
    (" · {0} for details", " · 詳情見{0}"),
    ("workspace with network access", "工作區並允許網路訪問"),
    ("… +{0} lines", "… 另有{0}行"),
    ("… +{0} lines ({1})", "… 另有{0}行（{1}）"),
    ("ctrl + t to view transcript", "按ctrl + t檢視記錄"),
    (
        "Interacted with `{0}`, sent `{1}`",
        "與`{0}`互動，傳送了`{1}`",
    ),
    ("Waited for `{0}`", "等待`{0}`"),
    ("(no output)", "（無輸出）"),
    (
        "Open {0} visualization in the browser",
        "在瀏覽器中開啟{0}視覺化",
    ),
    (
        "_Visualization unavailable on this device._",
        "_此裝置不支援視覺化。_",
    ),
    ("    answer: ", "    回答："),
    ("Failed to disable pets: {0}", "禁用桌寵失敗：{0}"),
    ("Failed to save pet selection: {0}", "儲存桌寵選擇失敗：{0}"),
    ("Failed to load pet: {0}", "載入桌寵失敗：{0}"),
    (
        "Failed to load configured pet: {0}",
        "載入已配置的桌寵失敗：{0}",
    ),
    ("Loading Pet", "正在載入桌寵"),
    ("Preparing the terminal pet.", "正在準備終端桌寵。"),
    ("Loading selected pet...", "正在載入所選桌寵…"),
    ("Approval requested: {0}", "請求審批：{0}"),
    ("Codex wants to edit {0}", "Codex想要編輯{0}"),
    ("{0} files", "{0} 個檔案"),
    ("Approval requested by {0}", "由{0}請求審批"),
    ("Plan mode prompt: {0}", "計劃模式提示：{0}"),
    ("Agent turn complete", "智慧體輪次完成"),
    (
        "This conversation is open in another app",
        "此對話已在另一個應用中開啟",
    ),
    (" to Retry", "重試"),
    (
        "Close it there and press R to continue here.",
        "在那裡關閉它，然後按R在此繼續。",
    ),
    ("{0} unanswered question", "{0} 個未回答的問題"),
    ("{0} unanswered questions", "{0} 個未回答的問題"),
    ("No choices", "無選項"),
    (
        "Selected session does not have a thread ID.",
        "所選會話沒有執行緒ID。",
    ),
    (
        "Use /archive to archive the current session and exit.",
        "使用/archive歸檔當前會話並退出。",
    ),
    ("Failed to archive session: {0}", "歸檔會話失敗：{0}"),
    (
        "Failed to restore archived session: {0}",
        "恢復已歸檔會話失敗：{0}",
    ),
    (
        "Ambiguous `tui.keymap.{0}` bindings: `{1}` uses a key reserved by `{2}`. Set a different key in `~/.codex/config.toml` and retry. See the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap.{0}` 的繫結有歧義：`{1}` 使用了由 `{2}` 保留的按鍵。請在 `~/.codex/config.toml` 中設定其他按鍵後重試。支援的按鍵與示例見Codex鍵位文件。",
    ),
    (
        "Conflicting `{0}` and `{1}` bindings",
        "`{0}` 與 `{1}` 的繫結衝突",
    ),
    ("Changes write the root ", "更改會寫入根"),
    (
        "Invalid `tui.keymap` configuration: {0}",
        "`tui.keymap` 配置無效：{0}",
    ),
    ("Choose an import source", "選擇匯入來源"),
    (
        "Select the app whose setup you want to import.",
        "選擇要匯入其配置的應用。",
    ),
    (
        "Could not check for importable setup: {0}",
        "無法檢查可匯入的配置：{0}",
    ),
    ("Waiting for background terminal", "正在等待後臺終端"),
    (
        "This thread is open elsewhere. Close it there and retry resume to continue.",
        "此執行緒已在別處開啟。請在那裡關閉它，然後重試恢復以繼續。",
    ),
    (
        "Cannot switch collaboration mode while a turn is running.",
        "輪次執行期間無法切換協作模式。",
    ),
    (
        "Model interrupted to submit steer instructions.",
        "模型被中斷以提交引導指令。",
    ),
    (
        "Thread model is unavailable. Wait for the thread to finish syncing or choose a model before sending input.",
        "執行緒模型不可用。請等待執行緒同步完成，或先選擇模型再傳送輸入。",
    ),
    ("Rename thread", "重新命名執行緒"),
    ("Name thread", "命名執行緒"),
    ("Loading permission profiles…", "正在載入許可權配置…"),
    ("Failed to load hooks: {0}", "載入鉤子失敗：{0}"),
    ("quit", "退出"),
    ("Reconnecting to app-server…", "正在重新連線app-server…"),
    (
        "Connection lost. Attempting to reconnect…",
        "連線已斷開。正在嘗試重新連線…",
    ),
    (
        "Reconnect failed — check the endpoint, then relaunch",
        "重新連線失敗——請檢查端點，然後重新啟動",
    ),
    (
        "Automatic reconnect could not restore this session. Your draft is still editable. Copy it before quitting with Ctrl+C, then reconnect with the same command.",
        "自動重連無法恢復此會話。你的草稿仍可編輯。請在按Ctrl+C退出前複製它，然後用同一條命令重新連線。",
    ),
    (
        "Luna model settings are unavailable; please try /model again in a moment.",
        "Luna模型設定不可用；請稍後重試/model。",
    ),
    (
        "Other models return when ordinary usage is available again.",
        "常規用量恢復後即可選擇其他模型。",
    ),
    (
        "MCP startup interrupted. The following servers were not initialized: {0}",
        "MCP啟動被中斷。以下伺服器未完成初始化：{0}",
    ),
    ("failed: {0}", "失敗：{0}"),
    ("MCP startup incomplete ({0})", "MCP啟動未完成（{0}）"),
    (
        "No other permission modes are available.",
        "沒有其他可用的許可權模式。",
    ),
    ("Updated roots: {0}", "已更新的根目錄：{0}"),
    ("Failed to upgrade {0} {1}: {2}", "升級{0}個{1}失敗：{2}"),
    (
        "This request requires additional safety checks, some tool calls might take extra time",
        "此請求需要額外的安全檢查，部分工具呼叫可能耗時更長",
    ),
    (
        "You've reached your workspace credit limit",
        "你已達到工作區額度上限",
    ),
    ("Expires {0}", "有效期至{0}"),
    ("Full reset", "完整重置"),
    ("Reset your current usage limits.", "重置當前用量上限。"),
    ("Thread forked from ", "執行緒分叉自 "),
    (
        "You’re continuing from this point in a new conversation",
        "你將從新對話中的這一點繼續",
    ),
    (
        "Model {0} does not support image inputs. Remove images or switch models.",
        "模型{0}不支援影象輸入。請移除影象或切換模型。",
    ),
    ("Model changed to {0}", "模型已切換為{0}"),
    (
        "Current model ({0}) doesn't support personalities. Try /model to pick a different model.",
        "當前模型（{0}）不支援個性化。請嘗試/model選擇其他模型。",
    ),
    ("I've installed it", "我已安裝"),
    ("Press a number to choose", "按數字選擇"),
    (
        "Press a number to choose · esc to dismiss · type to continue",
        "按數字選擇 · esc關閉 · 輸入以繼續",
    ),
    ("esc to dismiss · type to continue", "esc關閉 · 輸入以繼續"),
    ("Destination: ", "目標位置："),
    ("unavailable", "不可用"),
    (" insert · ", " 插入 · "),
    (" close · ", " 關閉 · "),
    (" switch search modes", "切換搜尋模式"),
    ("All Results", "全部結果"),
    ("Filesystem Only", "僅檔案系統"),
    (" (disabled)", "（已禁用）"),
    (" to insert or ", "插入或"),
    ("Vim: Normal", "Vim: 普通模式"),
    ("Vim: Insert", "Vim: 插入模式"),
    ("Vim: Replace", "Vim: 替換模式"),
    (" to answer", "回答"),
    (
        "Answer too long; limit {0} characters",
        "回答過長；上限{0}個字元",
    ),
    ("{0} (if available)", "{0}（如可用）"),
    ("Approval needed in {0}", "執行緒{0}需要審批"),
    (" to switch threads", "切換執行緒"),
    ("{0} question", "{0} 個問題"),
    ("{0} questions", "{0} 個問題"),
    ("{0} (disabled: {1})", "{0}（已禁用：{1}）"),
    ("disabled: {0}", "已禁用：{0}"),
    (
        "{0} background terminal running · /ps to view · /stop to close",
        "{0} 個後臺終端執行中 · /ps檢視 · /stop關閉",
    ),
    (
        "{0} background terminals running · /ps to view · /stop to close",
        "{0} 個後臺終端執行中 · /ps檢視 · /stop關閉",
    ),
    ("skills", "技能"),
    (
        "failed to localize guardian filesystem paths: {0}",
        "無法本地化guardian檔案系統路徑：{0}",
    ),
    (
        "Select an agent to watch. {0} previous, {1} next.",
        "選擇要檢視的智慧體。上一個：{0}，下一個：{1}。",
    ),
    ("Describe a new task", "描述一個新任務"),
    ("Unsent task: {0}", "未傳送的任務：{0}"),
    ("tasks · dispatch paused", "任務 · 排程已暫停"),
    ("  current", "  當前"),
    ("Task details", "任務詳情"),
    ("No prompt available.", "沒有可用的提示詞。"),
    (
        "Earlier messages are available — press {0} to view the full transcript",
        "更早的訊息可用——按{0}檢視完整記錄",
    ),
    (
        "Failed to start TUI session picker: {0}",
        "啟動TUI會話選擇器失敗：{0}",
    ),
    (
        "Failed to open session picker: {0}",
        "開啟會話選擇器失敗：{0}",
    ),
    ("Unable to resume session", "無法恢復會話"),
    ("Return to command center", "返回命令中心"),
    (
        "Permission overrides are not supported when resuming a remote task.",
        "恢復遠端任務時不支援許可權覆蓋。",
    ),
    (
        "Permission overrides are not supported when forking a remote task.",
        "分叉遠端任務時不支援許可權覆蓋。",
    ),
    (
        "Reconnect failed — agent list is stale; relaunch to retry",
        "重新連線失敗——智慧體列表已過期；請重新啟動後重試",
    ),
    ("Experimental feature changes", "實驗性功能變更"),
    ("Memory setting changes", "記憶設定變更"),
    (
        "Skipped loading {0} skill(s) due to invalid SKILL.md files.",
        "由於SKILL.md檔案無效，已跳過載入{0}個技能。",
    ),
    (
        "Project-local config, hooks, and exec policies are disabled in the following folders until the project is trusted, but skills still load.\n",
        "在專案被信任之前，以下資料夾中的專案級配置、鉤子和執行策略已禁用，但技能仍會載入。\n",
    ),
    (
        "The server returned duplicate permission profiles.",
        "伺服器返回了重複的許可權配置。",
    ),
    (
        "Permission discovery exceeded its pagination limit. Try /permissions again.",
        "許可權發現超出分頁上限。請重試/permissions。",
    ),
    (
        "Permission discovery timed out. Try /permissions again.",
        "許可權發現超時。請重試/permissions。",
    ),
    (
        "This server does not support permission discovery. Upgrade the Codex server to use this menu.",
        "此伺服器不支援許可權發現。請升級Codex伺服器以使用此選單。",
    ),
    ("Failed to load permissions: {0}", "載入許可權失敗：{0}"),
    ("Release notes: ", "版本說明："),
    ("Update now (runs `{0}`)", "立即更新（執行`{0}`）"),
    ("Skip until next version", "跳過直到下個版本"),
    ("Skip", "跳過"),
    ("  Resuming session…", "  正在恢復會話…"),
    ("  Forking session…", "  正在分叉會話…"),
    ("This conversation is archived", "此對話已歸檔"),
    ("Unarchive and {0}", "取消歸檔並{0}"),
    (" to continue or ", "繼續，或"),
    ("fork", "分叉"),
    ("resumed", "已恢復"),
    ("forked", "已分叉"),
    ("Failed to unarchive session {0}", "取消歸檔會話失敗：{0}"),
    (
        "Failed to {0} session from {1}: {2}",
        "從{1}{0}會話失敗：{2}",
    ),
    (" to interrupt)", " 中斷）"),
    ("{0} and {1}", "{0} 和 {1}"),
    (
        "Ignoring --add-dir ({0}) because the effective permissions do not allow additional writable roots. Switch to workspace-write or danger-full-access to allow them.",
        "忽略--add-dir（{0}）：當前生效的許可權不允許額外的可寫根目錄。請切換到workspace-write或danger-full-access以允許它們。",
    ),
    (
        "Sent by Codex from task {0}\n{1}",
        "由Codex從任務{0}傳送\n{1}",
    ),
    (
        "Failed to branch before the selected prompt: {0}",
        "無法在所選提示詞之前分叉：{0}",
    ),
    (
        "No active or archived session found matching '{0}'.",
        "沒有找到匹配{0}的活躍或已歸檔會話。",
    ),
    ("active or archived", "活躍或已歸檔"),
    (
        "No {0} session found matching '{1}'.",
        "沒有找到匹配{1}的{0}會話。",
    ),
    (
        "Permanently delete session '{0}' ({1})?",
        "永久刪除會話{0}（{1}）？",
    ),
    ("Continuation request (quoted)", "繼續請求（引用）"),
    ("What we detected", "我們檢測到的內容"),
    (
        "Couldn’t continue this chat. Review its latest status before trying again.",
        "無法繼續此對話。請先檢視其最新狀態後再試。",
    ),
    ("Automatically switched to", "已自動切換到"),
    (" due to usage limits.", "，因為已達用量上限。"),
    ("Automatically switched back to", "已自動切回"),
    (
        " because ordinary usage is available again.",
        "，因為常規用量已恢復。",
    ),
    (
        "this app server does not support confirmed permission changes; use /permissions",
        "此app server不支援經確認的許可權變更；請使用/permissions",
    ),
    ("Failed to update permissions: {0}", "更新許可權失敗：{0}"),
    ("   Loading...", "   正在載入…"),
    ("   Token activity unavailable", "   Token活動不可用"),
    ("Session ID", "會話ID"),
    ("{0} {1}% left", "{0} 剩餘{1}%"),
    ("Reviewing approval request", "正在稽核審批請求"),
    ("Reviewing {0} approval requests", "正在稽核{0}個審批請求"),
    (
        "The session must start before you can change its working directory.",
        "會話必須先啟動，才能更改其工作目錄。",
    ),
    (
        "Changing directories requires an idle primary session without queued input.",
        "切換目錄需要主會話空閒且沒有排隊的輸入。",
    ),
    (
        "Custom theme \"{0}\" at {1} could not be loaded (invalid .tmTheme format). Falling back to the default theme.",
        "位於{1}的自定義主題“{0}”無法載入（.tmTheme格式無效）。已回退到預設主題。",
    ),
    (
        "Theme \"{0}\" not found. Using the default theme. To use a custom theme, place a .tmTheme file at {1}.",
        "未找到主題“{0}”。正在使用預設主題。若要使用自定義主題，請將.tmTheme檔案放在{1}。",
    ),
    (
        "Invalid `tui.keymap` configuration: {0}\nFix the config and retry.\nSee the Codex keymap documentation for supported actions and examples.",
        "`tui.keymap`配置無效：{0}\n請修正配置後重試。\n支援的按鍵與示例見Codex鍵位文件。",
    ),
    ("Failed to {0} thread goal: {1}", "執行緒目標{0}失敗：{1}"),
    ("read", "讀取"),
    ("replace", "替換"),
    ("update", "更新"),
    ("clear", "清除"),
    ("Go back", "返回"),
    ("Failed to set trust for {0}: {1}", "無法為{0}設定信任：{1}"),
    ("Compose new task", "撰寫新任務"),
    ("neither VISUAL nor EDITOR is set", "未設定VISUAL或EDITOR"),
    ("failed to parse editor command", "解析編輯器命令失敗"),
    (
        "Multiple sessions match '{0}' (including {1} and {2}); use a session UUID to disambiguate.",
        "有多個會話匹配{0}（包括{1}和{2}）；請使用會話UUID以消除歧義。",
    ),
    (
        "Cannot verify a unique session label across server pages; matching session UUID: {0}. Use it only if this is the session you want.",
        "無法跨伺服器分頁確認唯一的會話標籤；匹配的會話UUID：{0}。僅當你確實想要該會話時才使用它。",
    ),
    (
        "the selected permission profile cannot be safely represented by the legacy app-server sandbox policy; select a named or legacy-compatible permission profile",
        "所選許可權配置無法由舊的app-server沙箱策略安全表示；請選擇具名或相容舊版的許可權配置",
    ),
    (
        "failed to initialize sqlite local db at {0}: {1}",
        "無法在{0}初始化sqlite本地資料庫：{1}",
    ),
    ("Retry", "重試"),
    ("Back", "返回"),
    ("Action", "操作"),
    ("Hooks", "鉤子"),
    ("Apps", "應用"),
    ("Auth", "認證"),
    ("Quit", "退出"),
    ("Resetting your usage...", "正在重置你的用量…"),
    ("Using a reset...", "正在使用一次重置…"),
    (
        "Couldn't reset usage. Please try again.",
        "無法重置用量，請重試。",
    ),
    (
        "Usage reset. Checking your remaining resets...",
        "用量已重置。正在檢查剩餘的重置次數…",
    ),
    ("Refreshing...", "重新整理中…"),
    ("timed out", "超時"),
    ("canceled", "已取消"),
    ("You ", "你 "),
    (" files", " 個檔案"),
    (" for ", "："),
    (" before ", "，"),
    (" before codex could run ", "，codex未能執行"),
    (" before codex could access ", "，codex未能訪問"),
    (" before codex could apply ", "，codex未能應用"),
    (" for codex to apply ", "，讓codex應用"),
    (
        " before this request could be approved",
        "，本次請求未獲批准",
    ),
    (
        " codex to always run commands that start with ",
        "，讓codex始終執行以此開頭的命令：",
    ),
    ("Installing {0}...", "正在安裝{0}…"),
    ("Uninstalling {0}...", "正在解除安裝{0}…"),
    (
        "Installed {0} of {1} available plugins.",
        "已安裝{0}/{1}個可用外掛。",
    ),
    ("Showing {0} installed plugins.", "顯示{0}個已安裝的外掛。"),
    (
        "Installed {0} of {1} OpenAI Curated plugins.",
        "已安裝{0}/{1}個OpenAI精選外掛。",
    ),
    ("1 principal: {0}", "1位主體：{0}"),
    ("{0} principals", "{0}位主體"),
    (" to view hooks; ", " 檢視鉤子；"),
    (" to trust; ", " 信任；"),
    (
        "include current selection, open files, and other context from your IDE",
        "包含當前選中內容、開啟的檔案以及來自IDE的其他上下文",
    ),
    (
        "start a side conversation in an ephemeral fork",
        "在臨時分叉中開始一段旁支對話",
    ),
    (
        "let sandbox read a directory: /sandbox-add-read-dir <absolute_path>",
        "讓沙箱讀取一個目錄：/sandbox-add-read-dir <absolute_path>",
    ),
    (
        "Configure AWS credentials in your environment, then return here.",
        "在你的環境中配置AWS憑據，然後回到這裡。",
    ),
    ("Bedrock API key", "Bedrock API key"),
    ("{0} consumes usage limits faster", "{0}更快地消耗用量限額"),
    ("{0} consume usage limits faster", "{0}更快地消耗用量限額"),
    (
        "{0} is available under /model → {1} → More reasoning…",
        "{0}可在「/model → {1} → More reasoning…」中使用",
    ),
    (
        "{0} are available under /model → {1} → More reasoning…",
        "{0}可在「/model → {1} → More reasoning…」中使用",
    ),
    ("{0} code", "{0}程式碼"),
    ("      Importing: {0}", "      正在匯入：{0}"),
    (
        "Search scanned first {0} sessions; more may exist",
        "已掃描前{0}個會話；可能還有更多",
    ),
    (
        " Consider removing write access for Everyone from the following folders:",
        " 建議取消Everyone對這些資料夾的寫許可權：",
    ),
    (
        "No root-level keymap overrides have been configured.",
        "尚未配置根級快捷鍵覆蓋。",
    ),
    (
        "Capture two consecutive keys for this action.",
        "為此操作連續按兩個鍵。",
    ),
    (
        "Failed to determine working directory for resume: {0}",
        "確定恢復會話的工作目錄失敗：{0}",
    ),
    (
        "Failed to rebuild configuration for resume: {0}",
        "為恢復會話重建配置失敗：{0}",
    ),
    ("Review against a base branch", "對基準分支做審查"),
    ("(PR Style)", "（PR風格）"),
    ("Review uncommitted changes", "審查未提交的改動"),
    ("Review a commit", "審查某個提交"),
    ("Custom review instructions", "自定義審查說明"),
    ("Select a review preset", "選擇審查預設"),
    ("Select a base branch", "選擇基準分支"),
    ("Type to search branches", "輸入以搜尋分支"),
    ("Select a commit to review", "選擇要審查的提交"),
    ("Type to search commits", "輸入以搜尋提交"),
    ("Type instructions and press Enter", "輸入說明後按回車"),
    (
        "features.token_budget.reminder_message_template must not exceed {0} bytes",
        "features.token_budget.reminder_message_template不得超過{0}位元組",
    ),
    (
        "features.token_budget.guidance_message must not exceed {0} bytes",
        "features.token_budget.guidance_message不得超過{0}位元組",
    ),
    (
        "features.token_budget.auto_compact_fallback_prompt must not exceed {0} bytes",
        "features.token_budget.auto_compact_fallback_prompt不得超過{0}位元組",
    ),
    (
        "model_catalog_json path `{0}` must contain at least one model",
        "model_catalog_json路徑`{0}`必須至少包含一個模型",
    ),
    ("{0} must be at least {1}", "{0}至少為{1}"),
    ("{0} must be at most {1}", "{0}最多為{1}"),
    ("{0} must not be empty", "{0}不能為空"),
    (
        "{0} must not have leading or trailing whitespace",
        "{0}不能有前導或尾隨空白",
    ),
    (
        "{0} must match ^[a-zA-Z0-9_-]+$",
        "{0}必須匹配^[a-zA-Z0-9_-]+$",
    ),
    ("{0} must be at most {1} characters", "{0}最多為{1}個字元"),
    (
        "{0} uses a reserved namespace: {1}",
        "{0}使用了保留名稱空間：{1}",
    ),
    (
        "Configured value for `{0}` is disallowed by requirements; falling back to required value {1}. Details: {2}",
        "`{0}`的配置值被requirements禁止，將回退到要求值{1}。詳情：{2}",
    ),
    (
        "configured value for `{0}` is disallowed by requirements ({1}); fallback to a requirement-compliant value also failed ({2})",
        "`{0}`的配置值被requirements禁止（{1}）；回退到符合要求的值也失敗（{2}）",
    ),
    (
        "mcp_servers.{0} uses unsupported `bearer_token`; set `bearer_token_env_var`.",
        "mcp_servers.{0}使用了不支援的`bearer_token`；請改用`bearer_token_env_var`。",
    ),
    (
        "features.rollout_budget.{0} must be finite and non-negative",
        "features.rollout_budget.{0}必須為有限非負數",
    ),
    (
        "legacy `profile = \"{0}\"` config is no longer supported; use `--profile {1}` with `{2}.config.toml` instead",
        "舊式`profile = \"{0}\"`配置已不再支援；請改用`--profile {1}`配合`{2}.config.toml`",
    ),
    ("Model provider `{0}` not found", "未找到模型提供方`{0}`"),
    (
        "requirements.toml permissions profile `{0}` conflicts with a config-defined profile of the same name",
        "requirements.toml的許可權配置`{0}`與配置中定義的同名配置衝突",
    ),
    (
        "Configured value for `permission_profile` is disallowed by requirements; falling back from `{0}` to required value `{1}`.",
        "`permission_profile`的配置值被requirements禁止，將從`{0}`回退到要求值`{1}`。",
    ),
    (
        "requirements.toml allowed_permission_profiles refers to undefined profile `{0}`",
        "requirements.toml的allowed_permission_profiles引用了未定義的配置`{0}`",
    ),
    (
        "requirements.toml default_permissions `{0}` must be allowed by allowed_permission_profiles",
        "requirements.toml的default_permissions`{0}`必須被allowed_permission_profiles允許",
    ),
    ("`{0}` requires approval: {1}", "`{0}`需要審批：{1}"),
    ("`{0}` requires approval by policy", "`{0}`根據策略需要審批"),
    ("`{0}` rejected: {1}", "`{0}`被拒絕：{1}"),
    (
        "`{0}` rejected: policy forbids commands starting with `{1}`",
        "`{0}`被拒絕：策略禁止以`{1}`開頭的命令",
    ),
    (
        "`{0}` rejected: blocked by policy",
        "`{0}`被拒絕：被策略阻止",
    ),
    (
        "rm -f style commands are not permitted. Use a safer approach",
        "不允許使用rm -f風格的命令。請改用更安全的方式",
    ),
    ("blocked by policy", "被策略阻止"),
    (
        "approval required by policy, but AskForApproval is set to Never",
        "策略要求審批，但AskForApproval被設為Never",
    ),
    (
        "approval required by policy, but AskForApproval::Granular.sandbox_approval is false",
        "策略要求審批，但AskForApproval::Granular.sandbox_approval為false",
    ),
    (
        "approval required by policy rule, but AskForApproval::Granular.rules is false",
        "策略規則要求審批，但AskForApproval::Granular.rules為false",
    ),
    (
        "network approval was cancelled before a decision was returned",
        "網路審批在返回決定前被取消",
    ),
    (
        "Network request disconnected after {0} ms, before approval could complete",
        "網路請求在審批完成前斷開（已等待{0}毫秒）",
    ),
    (
        "Network access to \"{0}\" was blocked by policy.",
        "對\"{0}\"的網路訪問被策略阻止。",
    ),
    (
        "{0} is not in the allowed_domains",
        "{0}不在allowed_domains中",
    ),
    (
        "Error while requesting approval: {0}",
        "請求審批時出錯：{0}",
    ),
    ("Error while requesting approval", "請求審批時出錯"),
    (
        "Failed to apply network policy amendment: {0}",
        "應用網路策略修改失敗：{0}",
    ),
    (
        "failed to resolve environment network policy: {0}",
        "解析環境網路策略失敗：{0}",
    ),
    (
        "failed to build environment network policy: {0}",
        "構建環境網路策略失敗：{0}",
    ),
    (
        "failed to build execution-scoped network proxy: {0}",
        "構建執行域網路代理失敗：{0}",
    ),
    (
        "failed to create execution-scoped network proxy: {0}",
        "建立執行域網路代理失敗：{0}",
    ),
    (
        "environment network policy requires its configured controller proxy",
        "環境網路策略需要其已配置的控制器代理",
    ),
    (
        "permissions profile `{0}` uses a reserved built-in profile prefix",
        "許可權配置`{0}`使用了保留的內建配置字首",
    ),
    (
        "default_permissions requires a `[permissions]` table",
        "default_permissions需要`[permissions]`表",
    ),
    (
        "Filesystem glob `{0}` uses `read` or `write` access, which is not fully supported by this platform's sandboxing. Use an exact path or trailing `/**` subtree rule instead. `deny` globs are supported.",
        "檔案系統通配`{0}`使用了`read`或`write`訪問，本平臺沙箱對此支援不完整。請改用精確路徑或尾部`/**`的子樹規則。`deny`通配是受支援的。",
    ),
    (
        "Filesystem deny-read glob `{0}` uses `**`. Non-macOS sandboxing does not support unbounded `**` natively; set `glob_scan_max_depth` in this filesystem profile to cap Linux glob expansion and silence this warning, or enumerate explicit depths such as `*.env`, `*/*.env`, and `*/*/*.env`.",
        "檔案系統deny-read通配`{0}`使用了`**`。非macOS沙箱不原生支援無界的`**`；請在該檔案系統配置裡設定`glob_scan_max_depth`以限制Linux通配展開並消除此警告，或顯式列出深度，例如`*.env`、`*/*.env`、`*/*/*.env`。",
    ),
    (
        "default_permissions refers to unknown built-in profile `{0}`",
        "default_permissions引用了未知的內建配置`{0}`",
    ),
    (
        "filesystem path `{0}` does not support nested entries",
        "檔案系統路徑`{0}`不支援巢狀條目",
    ),
    (
        "filesystem glob subpath `{0}` only supports `deny` access",
        "檔案系統通配子路徑`{0}`只支援`deny`訪問",
    ),
    (
        "filesystem glob path `{0}` only supports `deny` access; use an exact path or trailing `/**` for `{1}` subtree access",
        "檔案系統通配路徑`{0}`只支援`deny`訪問；如需`{1}`子樹訪問，請改用精確路徑或尾部`/**`",
    ),
    (
        "filesystem path `{0}` must be absolute, use `~/...`, or start with `:`",
        "檔案系統路徑`{0}`必須是絕對路徑、使用`~/...`，或以`:`開頭",
    ),
    (
        "filesystem subpath `{0}` must be a descendant path without `.` or `..` components",
        "檔案系統子路徑`{0}`必須是不含`.`或`..`成分的後代路徑",
    ),
    (
        "Permissions profile `{0}` does not define any recognized filesystem entries for this version of Codex. Filesystem access will remain restricted. Upgrade Codex if this profile expects filesystem permissions.",
        "許可權配置`{0}`未定義本版Codex可識別的任何檔案系統條目。檔案系統訪問將保持受限。若該配置本應包含檔案系統許可權，請升級Codex。",
    ),
    (
        "Configured filesystem path `{0}` with nested entry `{1}` is not recognized by this version of Codex and will be ignored. Upgrade Codex if this path is required.",
        "配置的檔案系統路徑`{0}`（含巢狀條目`{1}`）不被本版Codex識別，將被忽略。若確實需要該路徑，請升級Codex。",
    ),
    (
        "Configured filesystem path `{0}` is not recognized by this version of Codex and will be ignored. Upgrade Codex if this path is required.",
        "配置的檔案系統路徑`{0}`不被本版Codex識別，將被忽略。若確實需要該路徑，請升級Codex。",
    ),
    (
        "glob_scan_max_depth must be at least 1",
        "glob_scan_max_depth至少為1",
    ),
    (
        "turn environment working directory exceeds the maximum size",
        "回合環境的工作目錄超出最大長度",
    ),
    (
        "duplicate turn environment id `{0}`",
        "重複的回合環境id`{0}`",
    ),
    ("unknown turn environment id `{0}`", "未知的回合環境id`{0}`"),
    (
        "ephemeral thread does not support metadata updates: {0}",
        "臨時執行緒不支援後設資料更新：{0}",
    ),
    (
        "ephemeral thread does not support section moves: {0}",
        "臨時執行緒不支援區段移動：{0}",
    ),
    (
        "failed to load thread-spawn descendants: {0}",
        "載入執行緒派生後代失敗：{0}",
    ),
    (
        "internal sessions require an internal session source",
        "內部會話需要內部會話來源",
    ),
    (
        "failed to read subagent fork source {0}: {1}",
        "讀取子代理分叉源{0}失敗：{1}",
    ),
    (
        "thread {0} is not a recorded multi-agent v2 child",
        "執行緒{0}不是已記錄的多代理v2子執行緒",
    ),
    (
        "cannot resume multi-agent v2 child {0}: parent {1} is not loaded; resume the parent first",
        "無法恢復多代理v2子執行緒{0}：父執行緒{1}未載入；請先恢復父執行緒",
    ),
    (
        "failed to read stored thread {0}: invalid thread-store request: {1}",
        "讀取已儲存執行緒{0}失敗：無效的執行緒儲存請求：{1}",
    ),
    (
        "failed to read stored thread {0}: {1}",
        "讀取已儲存執行緒{0}失敗：{1}",
    ),
    (
        "failed to load model context for thread {0}: {1}",
        "載入執行緒{0}的模型上下文失敗：{1}",
    ),
    (
        "reserved thread ID cannot be used when resuming a thread",
        "恢復執行緒時不能使用保留的執行緒ID",
    ),
    (
        "thread {0} is already running with a different rollout path",
        "執行緒{0}已在以不同的rollout路徑執行",
    ),
    (
        "thread {0} did not include persisted history",
        "執行緒{0}未包含已持久化的歷史",
    ),
    (
        "failed to read thread by rollout path: {0}",
        "按rollout路徑讀取執行緒失敗：{0}",
    ),
    (
        "thread metadata update is not supported by this store: {0}",
        "此儲存不支援執行緒後設資料更新：{0}",
    ),
    (
        "failed to update thread metadata {0}: {1}",
        "更新執行緒後設資料{0}失敗：{1}",
    ),
    (
        "invalid stored agent path: {0}",
        "無效的已儲存代理路徑：{0}",
    ),
    (
        "multi-agent v2 child {0} is not owned by its loaded parent",
        "多代理v2子執行緒{0}不屬於其已載入的父執行緒",
    ),
    (
        "cannot resume multi-agent v2 child {0} with the current parent settings",
        "無法使用當前父執行緒設定恢復多代理v2子執行緒{0}",
    ),
    (
        "cannot resume multi-agent v2 child {0}: parent ownership is unavailable; resume the parent first",
        "無法恢復多代理v2子執行緒{0}：父執行緒所有權不可用；請先恢復父執行緒",
    ),
    (
        "cannot resume multi-agent v2 child {0}: recorded parent ownership is inconsistent",
        "無法恢復多代理v2子執行緒{0}：記錄的父執行緒所有權不一致",
    ),
    (
        "cannot resume multi-agent v2 child {0}: parent execution policy has changed; retry through the parent",
        "無法恢復多代理v2子執行緒{0}：父執行緒執行策略已變更；請透過父執行緒重試",
    ),
    (
        "cannot resume multi-agent v2 child {0}: cached environment {1} {2}",
        "無法恢復多代理v2子執行緒{0}：快取環境{1}：{2}",
    ),
    (
        "approval_policy is invalid: {0}",
        "approval_policy無效：{0}",
    ),
    (
        "permission_profile is invalid: {0}",
        "permission_profile無效：{0}",
    ),
    (
        "permissions cannot be intersected safely: {0}",
        "無法安全求交許可權：{0}",
    ),
    (
        "parent thread history unavailable for fork: {0}",
        "分叉時父執行緒歷史不可用：{0}",
    ),
    (
        "no longer matches a ready parent environment",
        "不再匹配任何就緒的父環境",
    ),
    ("configuration is not ready", "配置尚未就緒"),
    (
        "configuration differs from the current parent",
        "配置與當前父執行緒不同",
    ),
    (
        "permissions changed on a remote executor",
        "許可權在遠端執行器上發生了變更",
    ),
    (
        "working directory is not a local absolute path",
        "工作目錄不是本地絕對路徑",
    ),
    (
        "workspace roots are not local absolute paths",
        "工作區根目錄不是本地絕對路徑",
    ),
    (
        "spawn_agent fork requires a parent spawn call id",
        "spawn_agent分叉需要父spawn呼叫ID",
    ),
    (
        "spawn_agent fork requires a fork mode",
        "spawn_agent分叉需要分叉模式",
    ),
    (
        "spawn_agent fork requires a thread-spawn session source",
        "spawn_agent分叉需要執行緒派生會話來源",
    ),
    ("conversation is not running", "會話未在執行"),
    (
        "AVAS realtime calls require realtime v1 or v3",
        "AVAS實時呼叫需要realtime v1或v3",
    ),
    (
        "existing realtime calls do not support session configuration options",
        "已存在的實時呼叫不支援會話配置選項",
    ),
    (
        "AVAS realtime calls require conversational realtime",
        "AVAS實時呼叫需要會話式實時模式",
    ),
    (
        "{0} must not exceed {1} estimated tokens",
        "{0}不得超過{1}個估計token",
    ),
    (
        "initial realtime items require realtime v3",
        "初始實時條目需要realtime v3",
    ),
    (
        "initial realtime items must contain no more than {0} items",
        "初始實時條目不得超過{0}條",
    ),
    (
        "each initial realtime item must not exceed {0} estimated tokens",
        "每條初始實時條目不得超過{0}個估計token",
    ),
    (
        "initial realtime items must not exceed {0} estimated tokens in total",
        "初始實時條目總計不得超過{0}個估計token",
    ),
    (
        "text realtime output modality requires realtime v2",
        "文字實時輸出模態需要realtime v2",
    ),
    (
        "realtime voice `{0}` is not supported for {1}; supported voices: {2}",
        "實時語音`{0}`不支援{1}；支援的語音：{2}",
    ),
    (
        "realtime conversation requires API key auth",
        "實時會話需要API key認證",
    ),
    (
        "invalid realtime api key header: {0}",
        "無效的實時API key請求頭：{0}",
    ),
    (
        "failed to serialize request_permissions response: {0}",
        "序列化request_permissions響應失敗：{0}",
    ),
    (
        "{0} handler received unsupported payload",
        "{0}處理器收到不支援的負載",
    ),
    (
        "failed to serialize {0} response: {1}",
        "序列化{0}響應失敗：{1}",
    ),
    ("failed to sleep: {0}", "休眠失敗：{0}"),
    (
        "Automatic approval review timed out while evaluating the requested approval.",
        "自動審批覆核在評估所請求的批准前超時。",
    ),
    ("guardian review failed", "guardian複核失敗"),
    (
        "Automatic approval review failed: {0}",
        "自動審批覆核失敗：{0}",
    ),
    (
        "Automatic approval review {0} (risk: {1}, authorization: {2}): {3}",
        "自動審批覆核{0}（風險：{1}，授權：{2}）：{3}",
    ),
    (
        "Auto-reviewer denied the action without a specific rationale.",
        "自動複核器拒絕了該操作，但未給出具體理由。",
    ),
    (
        "This action was rejected due to unacceptable risk.\nReason: {0}\n{1}",
        "該操作因不可接受的風險被拒絕。\n理由：{0}\n{1}",
    ),
    (
        "guardian review session could not preserve REPL developer policy: {0}",
        "guardian複核會話無法保留REPL開發者策略：{0}",
    ),
    (
        "guardian review completed without an assessment payload",
        "guardian複核完成但沒有評估負載",
    ),
    ("failed to load rules: {0}", "載入規則失敗：{0}"),
    (
        "Configured service tier `{0}` is not advertised as supported for model `{1}` and will be omitted from requests.",
        "配置的服務層級`{0}`未宣告為模型`{1}`所支援，將從請求中省略。",
    ),
    (
        "Session persistence is disabled; cannot {0}.",
        "會話持久化已禁用；無法{0}。",
    ),
    (
        "This session was recorded with model `{0}` but is resuming with `{1}`. Consider switching back to `{0}` as it may affect Codex performance.",
        "本會話記錄時使用的模型是`{0}`，但正以`{1}`恢復。建議切回`{0}`，否則可能影響Codex效能。",
    ),
    (
        "managed network proxy refresh semaphore closed",
        "託管網路代理重新整理訊號量已關閉",
    ),
    (
        "failed to update runtime allowlist: {0}",
        "更新執行時允許列表失敗：{0}",
    ),
    (
        "failed to update runtime denylist: {0}",
        "更新執行時拒絕列表失敗：{0}",
    ),
    (
        "failed to persist network policy amendment to execpolicy: {0}",
        "將網路策略修訂持久化到execpolicy失敗：{0}",
    ),
    (
        "network policy amendment host '{0}' does not match approved host '{1}'",
        "網路策略修訂主機'{0}'與已批准主機'{1}'不一致",
    ),
    (
        "Your account was flagged for potentially high-risk cyber activity and this request was routed to gpt-5.2 as a fallback. To regain access to gpt-5.3-codex, apply for trusted access: {0} or learn more: {1}",
        "你的賬號因可能存在高風險網路活動被標記，本次請求已回退路由到gpt-5.2。要恢復對gpt-5.3-codex的訪問，請申請可信訪問：{0}，或瞭解詳情：{1}",
    ),
    (
        "turn suspension requires the owning root thread",
        "掛起回合需要擁有該執行緒的根執行緒",
    ),
    ("thread session has stopped", "執行緒會話已停止"),
    ("thread suspension reply was lost", "執行緒掛起回覆丟失"),
    ("items must not be empty", "條目不得為空"),
    (
        "out-of-band elicitation count overflowed",
        "帶外徵詢計數溢位",
    ),
    (
        "out-of-band elicitation count is already zero",
        "帶外徵詢計數已為零",
    ),
    ("load history", "載入歷史"),
    ("read thread", "讀取執行緒"),
    ("update thread metadata", "更新執行緒後設資料"),
    ("append rollout items", "追加rollout條目"),
    (
        "Memory consolidation was rejected by a Stop hook.",
        "記憶整合被Stop鉤子拒絕。",
    ),
    (
        "stream closed before response.completed",
        "流在response.completed之前關閉",
    ),
    ("agent path `{0}` already exists", "代理路徑`{0}`已存在"),
    (
        "failed to prepare network proxy for environment `{0}`: {1}",
        "為環境`{0}`準備網路代理失敗：{1}",
    ),
    (
        "Auto-review returned a low-risk allow decision.",
        "自動複核返回了低風險的允許決定。",
    ),
    (
        "automatic approval review could not prepare the action",
        "自動審批覆核無法準備該操作",
    ),
    (
        "response.completed usage.codex_rollout_budget_units must be finite and non-negative",
        "response.completed的usage.codex_rollout_budget_units必須是有限的非負值",
    ),
    ("thread {0} is already running", "執行緒{0}已在執行"),
    (
        "automatic approval review could not complete",
        "自動審批覆核無法完成",
    ),
    ("'{0}' in {1}", "在{1}中查詢'{0}'"),
    (
        "Auto-review returned a deny decision without a rationale.",
        "自動複核作出了拒絕決定，但未給出理由。",
    ),
    ("User cancelled execution", "使用者取消了執行"),
    (
        "this terminal cannot enforce the current environment-owned network restrictions; start a new terminal",
        "本終端無法強制執行當前由環境擁有的網路限制；請新建終端",
    ),
    (
        "this terminal cannot enforce the current denied-read restrictions; start a new terminal",
        "本終端無法強制執行當前的禁止讀取限制；請新建終端",
    ),
    (
        "This terminal was launched outside the sandbox, bypassing any managed network proxy.",
        "本終端在沙箱外啟動，繞過了託管網路代理。",
    ),
    (
        "This terminal runs without a filesystem sandbox.",
        "本終端在沒有檔案系統沙箱的情況下執行。",
    ),
    (
        "This terminal uses the current permissions.",
        "本終端使用當前許可權。",
    ),
    (
        "This terminal retains additional permissions.",
        "本終端保留附加許可權。",
    ),
    (
        "This terminal retains sandbox or network settings that differ from the current permissions.",
        "本終端保留了與當前許可權不同的沙箱或網路設定。",
    ),
    (
        "cannot access the terminal's original environment; select it before retrying",
        "無法訪問終端的原始環境；請先選擇它再重試",
    ),
    (
        "terminal input contains a NUL byte and cannot be reviewed safely",
        "終端輸入包含NUL位元組，無法安全複核",
    ),
    (
        "Send input to an existing terminal. {0}",
        "向已有終端傳送輸入。{0}",
    ),
    (
        " It also has an internal plugin metrics write grant.",
        "它還擁有內部外掛指標寫入授權。",
    ),
    (
        " The cwd is its launch directory; the terminal's current directory and state may have changed.",
        "其cwd是啟動目錄；終端的當前目錄與狀態可能已改變。",
    ),
    (" Retained grants: {0}.", "保留的授權：{0}。"),
    ("Falling back to direct tools", "回退到直接工具"),
    ("Code mode will fail closed", "Code mode將失敗關閉"),
    (
        "Code Mode is unavailable because {0}. {1}; enable `features.code_mode_host` and install `codex-code-mode-host`.",
        "Code Mode不可用，原因：{0}。{1}；請啟用`features.code_mode_host`並安裝`codex-code-mode-host`。",
    ),
    ("command args are empty", "命令引數為空"),
    ("invalid exec cwd: {0}", "無效的exec cwd：{0}"),
    ("invalid sandbox cwd: {0}", "無效的沙箱cwd：{0}"),
    (
        "managed Windows proxy route is missing its restricting SID",
        "託管的Windows代理路由缺少其限制SID",
    ),
    (
        "windows sandbox: failed to resolve codex_home: {0}",
        "windows沙箱：無法解析codex_home：{0}",
    ),
    ("windows sandbox: {0}", "windows沙箱：{0}"),
    (
        "windows sandbox join error: {0}",
        "windows沙箱join錯誤：{0}",
    ),
    (
        "stdout pipe was unexpectedly not available",
        "stdout管道意外不可用",
    ),
    (
        "stderr pipe was unexpectedly not available",
        "stderr管道意外不可用",
    ),
    (
        "Failed to apply execpolicy amendment: {0}",
        "應用execpolicy修訂失敗：{0}",
    ),
    ("num_turns must be >= 1", "num_turns必須>=1"),
    (
        "Cannot rollback while a turn is in progress.",
        "回合進行中無法回滾。",
    ),
    ("rollback thread", "回滾執行緒"),
    (
        "thread rollback requires persisted thread history",
        "執行緒回滾需要已持久化的執行緒歷史",
    ),
    (
        "failed to flush thread persistence for rollback replay: {0}",
        "為回滾重放重新整理執行緒持久化失敗：{0}",
    ),
    (
        "failed to load thread history for rollback replay: {0}",
        "為回滾重放載入執行緒歷史失敗：{0}",
    ),
    (
        "Rolled the thread back, but failed to save the rollback marker. Codex will continue retrying. Error: {0}",
        "執行緒已回滾，但儲存回滾標記失敗。Codex將繼續重試。錯誤：{0}",
    ),
    ("update thread memory mode", "更新執行緒記憶模式"),
    (
        "Failed to shutdown thread persistence",
        "關閉執行緒持久化失敗",
    ),
    (
        "remote network policy decision timeout exceeds protocol limit",
        "遠端網路策略決策超時超過協議限制",
    ),
    (
        "selected exec-server does not support executor-local network proxy launches",
        "所選的exec-server不支援執行器本地網路代理啟動",
    ),
    (
        "failed to query exec-server capabilities: {0}",
        "查詢exec-server能力失敗：{0}",
    ),
    ("missing command line for PTY", "缺少PTY的命令列"),
    (
        "unified_exec zsh-fork is not supported for remote environments",
        "遠端環境不支援unified_exec zsh-fork",
    ),
    (
        "attachment-owned network policy requires a remote executor",
        "由附件擁有的網路策略需要遠端執行器",
    ),
    (
        "environment network restrictions must use network_policy",
        "環境網路限制必須使用network_policy",
    ),
    (
        "environment command policy cannot contain allow rules",
        "環境命令策略不能包含允許規則",
    ),
    (
        "invalid environment network policy: {0}",
        "無效的環境網路策略：{0}",
    ),
    (
        "environment readiness contains more than {0} selected capability roots",
        "環境就緒資訊包含超過{0}個選定的能力根",
    ),
    (
        "selected capability roots must have unique non-empty IDs and belong to environment `{0}`",
        "選定的能力根必須有唯一且非空的ID，並屬於環境`{0}`",
    ),
    (
        "environment `{0}` is not selected on this thread with the requested workspace",
        "本執行緒未選擇與環境`{0}`匹配的工作區",
    ),
    (
        "attachment-owned network policy cannot be bypassed by sandbox escalation",
        "由附件擁有的網路策略不能透過沙箱提權繞過",
    ),
    (
        "could not prepare approval action: {0}",
        "無法準備審批操作：{0}",
    ),
    (
        "Network access to \"{0}\" is blocked by policy.",
        "對\"{0}\"的網路訪問被策略阻止。",
    ),
    (
        "command failed; retry without sandbox?",
        "命令失敗；是否不用沙箱重試？",
    ),
    (
        "remote exec-server does not support inherited file descriptors",
        "遠端exec-server不支援繼承的檔案描述符",
    ),
    (
        "MCP tool call requires approval, but approval policy is never",
        "MCP工具呼叫需要審批，但審批策略是never",
    ),
    ("user rejected MCP tool call", "使用者拒絕了MCP工具呼叫"),
    (
        "cannot approve intercepted execution without an active turn",
        "沒有活動回合時無法批准被攔截的執行",
    ),
    ("Execution forbidden by policy", "策略禁止執行"),
    ("User denied execution", "使用者拒絕了執行"),
    (
        "unexpected shell command format for zsh-fork execution",
        "zsh-fork執行遇到意外的shell命令格式",
    ),
    (
        "network proxy constraints are invalid: {0}",
        "網路代理約束無效：{0}",
    ),
    (
        "failed to build network proxy: {0}",
        "構建網路代理失敗：{0}",
    ),
    ("failed to run network proxy: {0}", "執行網路代理失敗：{0}"),
    (
        "wait_for_environment handler received unsupported payload",
        "wait_for_environment處理器收到不支援的負載",
    ),
    (
        "Network access was blocked by policy.",
        "網路訪問被策略阻止。",
    ),
    (
        "domain is explicitly denied by policy and cannot be approved from this prompt",
        "該域名被策略明確拒絕，無法在此提示中批准",
    ),
    (
        "domain is not on the allowlist for the current sandbox mode",
        "該域名不在當前沙箱模式的允許列表中",
    ),
    (
        "local/private network addresses are blocked by the sandbox policy",
        "本地/私有網路地址被沙箱策略阻止",
    ),
    (
        "request method is blocked by the current network mode",
        "請求方法被當前網路模式阻止",
    ),
    ("network proxy is disabled", "網路代理已禁用"),
    ("request is blocked by network policy", "請求被網路策略阻止"),
    (
        "Network access to \"{0}\" was blocked: {1}.",
        "對\"{0}\"的網路訪問被阻止：{1}。",
    ),
    ("{0} is not supported yet", "{0}尚不支援"),
    (
        "Codex cannot access session files at {0} (permission denied). If sessions were created using sudo, fix ownership: sudo chown -R $(whoami) {1}",
        "Codex無法訪問位於{0}的會話檔案（許可權不足）。若這些會話是用sudo建立的，請修復所有權：sudo chown -R $(whoami) {1}",
    ),
    (
        "Session storage missing at {0}. Create the directory or choose a different Codex home.",
        "會話儲存缺失：{0}。請建立該目錄，或改用其他Codex home。",
    ),
    (
        "Session storage path {0} is blocked by an existing file. Remove or rename it so Codex can create sessions.",
        "會話儲存路徑{0}被一個已存在的檔案佔用。請移除或重新命名它，以便Codex建立會話。",
    ),
    (
        "Session data under {0} looks corrupt or unreadable. Clearing the sessions directory may help (this will remove saved threads).",
        "{0}下的會話資料看起來已損壞或不可讀。清空sessions目錄可能有幫助（這會刪除已儲存的執行緒）。",
    ),
    (
        "Session storage path {0} has an unexpected type. Ensure it is a directory Codex can use for session files.",
        "會話儲存路徑{0}的型別不符合預期。請確保它是Codex可用於會話檔案的目錄。",
    ),
    ("{0} (underlying error: {1})", "{0}（底層錯誤：{1}）"),
    (
        "tool {0} invoked with incompatible payload",
        "工具{0}的呼叫負載型別不匹配",
    ),
    ("suspend an unfinished root turn", "暫停未完成的根回合"),
    (
        "flush before root turn suspension failed: {0}",
        "根回合掛起前刷寫失敗：{0}",
    ),
    (
        "accepted root turn suspension had no running turn",
        "已接受的根回合掛起沒有正在執行的回合",
    ),
    (
        "accepted root turn suspension had no running task",
        "已接受的根回合掛起沒有正在執行的任務",
    ),
    (
        "flush after root turn suspension failed: {0}",
        "根回合掛起後刷寫失敗：{0}",
    ),
    (
        "close suspended root turn writer failed: {0}",
        "關閉已掛起根回合的寫入器失敗：{0}",
    ),
    (
        "lastTurnId '{0}' was not found in the source thread",
        "在源執行緒中找不到lastTurnId '{0}'",
    ),
    (
        "lastTurnId '{0}' is not a persisted canonical turn in the source thread",
        "lastTurnId '{0}'不是源執行緒中已持久化的規範回合",
    ),
    (
        "lastTurnId '{0}' identifies an in-progress turn",
        "lastTurnId '{0}'指向一個進行中的回合",
    ),
    (
        "beforeTurnId '{0}' is not a persisted canonical turn in the source thread",
        "beforeTurnId '{0}'不是源執行緒中已持久化的規範回合",
    ),
    (
        "beforeTurnId '{0}' was not found in the source thread",
        "在源執行緒中找不到beforeTurnId '{0}'",
    ),
    (
        "failed to build websocket prewarm client setup: {0}",
        "構建websocket預熱客戶端失敗：{0}",
    ),
    ("websocket connection is unavailable", "websocket連線不可用"),
    (
        "remote compaction v2 stream closed before response.completed",
        "遠端壓縮v2的流在response.completed之前關閉",
    ),
    (
        "failed to read effective config for selected permission profile: {0}",
        "讀取所選許可權檔位的有效配置失敗：{0}",
    ),
    ("Reconnecting... waiting for network", "正在重連…等待網路"),
    ("Reconnecting... {0}/{1}", "正在重連…{0}/{1}"),
    (
        "after_agent hook '{0}' failed and aborted turn completion: {1}",
        "after_agent鉤子'{0}'失敗，已中止回合完成：{1}",
    ),
    ("rejected by configuration", "被配置拒絕"),
    (
        "automatic approval review denied the action",
        "自動審批覆核拒絕了該操作",
    ),
    (
        "automatic approval review was cancelled",
        "自動審批覆核已取消",
    ),
    ("turn input was not submitted: {0}", "回合輸入未提交：{0}"),
    (
        "live agent path `{0}` not found",
        "找不到活動agent路徑`{0}`",
    ),
    (
        "Ignoring `{0}` in `model_providers.{1}` because managed residency is required.",
        "已忽略`{0}`（位於`model_providers.{1}`）：託管駐留是強制要求。",
    ),
    (
        "Configured value for `{0}` is overridden by the required value {1} from {2}.",
        "`{0}`的配置值被來自{2}的必需值{1}覆蓋。",
    ),
    (
        "Environment value for `$CODEX_SQLITE_HOME` is overridden by the required `sqlite_home` value {0} from {1}.",
        "環境變數`$CODEX_SQLITE_HOME`被來自{1}的必需`sqlite_home`值{0}覆蓋。",
    ),
    (
        "Configured values under `{0}` are overridden by requirements from {1}.",
        "`{0}`下的配置值被來自{1}的要求覆蓋。",
    ),
    ("failed to read current time: {0}", "讀取當前時間失敗：{0}"),
    (
        "failed to persist stale thread-spawn edge status for {0}: {1}",
        "儲存過期的執行緒派生邊狀態失敗（{0}）：{1}",
    ),
    ("no available agent nicknames", "沒有可用的agent暱稱"),
    (
        "Review was interrupted. Please re-run /review and wait for it to complete.",
        "審查被中斷。請重新執行 `/review` 並等待其完成。",
    ),
    (
        "shell is unavailable in this session",
        "本會話中shell不可用",
    ),
    (
        "shell working directory is not native to the Codex host",
        "shell工作目錄不屬於Codex宿主的原生路徑",
    ),
    (
        "automatic approval review cannot access the terminal's environment; select it before retrying",
        "自動審批覆核無法訪問該終端環境；請先選擇它再重試",
    ),
    (
        "only user input or standalone function-call outputs can start or steer a turn",
        "只有使用者輸入或獨立的函式呼叫輸出才能啟動或引導回合",
    ),
    (
        "only user input can steer a turn",
        "只有使用者輸入才能引導回合",
    ),
    (
        "invalid thread settings override: {0}",
        "無效的執行緒設定覆蓋：{0}",
    ),
    ("failed to submit turn input: {0}", "提交回合輸入失敗：{0}"),
    (
        "Code Mode is enabled in configuration, but model `{0}` does not advertise Code Mode support. This may degrade model performance. Disable `features.code_mode` and `features.code_mode_only`, or select a model whose metadata enables Code Mode.",
        "配置中已啟用Code Mode，但模型`{0}`未宣告支援Code Mode。這可能降低模型效能。請禁用`features.code_mode`與`features.code_mode_only`，或選擇後設資料啟用了Code Mode的模型。",
    ),
    (
        "configured permission profile with valid network policy ({0})",
        "配置了有效網路策略的許可權檔位（{0}）",
    ),
    (
        "zsh fork feature enabled, but no packaged zsh fork is available for this install",
        "已啟用zsh fork特性，但本次安裝沒有可用的打包zsh fork",
    ),
    ("Error running remote compact task", "執行遠端壓縮任務出錯"),
    (
        "writing outside of the project; rejected by user approval settings",
        "寫入專案之外；已被使用者審批設定拒絕",
    ),
    (
        "writing is blocked by read-only sandbox; rejected by user approval settings",
        "寫入被只讀沙箱阻止；已被使用者審批設定拒絕",
    ),
    ("empty patch", "空補丁"),
    (
        "features.current_time_reminder.clock_source is external, but no external current-time provider is available",
        "特性current_time_reminder.clock_source設為external，但沒有可用的外部當前時間提供者",
    ),
    ("exec command rejected by user", "使用者拒絕了exec命令"),
    ("patch rejected by user", "使用者拒絕了補丁"),
    (
        "zsh fork feature enabled, but packaged zsh fork `{0}` is not usable",
        "已啟用zsh fork特性，但打包的zsh fork`{0}`不可用",
    ),
    (
        "Ignoring invalid `{0}` config: {1}",
        "忽略無效的`{0}`配置：{1}",
    ),
    (
        "Using legacy `features` requirement `{0}` from {1}; prefer canonical feature key `{2}`",
        "使用了來自{1}的舊版`features`要求`{0}`；建議改用規範特性鍵`{2}`",
    ),
    (
        "Ignoring unknown `features` requirement `{0}` from {1}",
        "忽略來自{1}的未知`features`要求`{0}`",
    ),
    ("failed to persist config at {0}", "儲存配置失敗（{0}）"),
    ("[read-only, workspace-write]", "[只讀, 工作區可寫]"),
    (
        "elevated Windows sandbox setup is only supported on Windows",
        "提升許可權的Windows沙箱設定僅在Windows上受支援",
    ),
    (
        "legacy Windows sandbox setup is only supported on Windows",
        "舊版Windows沙箱設定僅在Windows上受支援",
    ),
    (
        "Windows sandbox read-root refresh is only supported on Windows",
        "Windows沙箱讀根重新整理僅在Windows上受支援",
    ),
    (
        "windows sandbox setup task failed: {0}",
        "Windows沙箱設定任務失敗：{0}",
    ),
    (
        "failed to persist windows sandbox mode: {0}",
        "儲存Windows沙箱模式失敗：{0}",
    ),
    (
        "failed to load AGENTS.md instructions for environment `{0}`: {1}",
        "無法為環境`{0}`載入AGENTS.md指令：{1}",
    ),
    ("unknown environment shell `{0}`", "未知的環境shell`{0}`"),
    (
        "approval policy disallowed sandbox approval prompt",
        "審批策略不允許沙箱審批提示",
    ),
    (
        "output_token_limit exceeds the TOML integer range",
        "output_token_limit超出TOML整數範圍",
    ),
    (
        "Warning: no last agent message; wrote empty content to {0}",
        "警告：沒有最後的agent訊息；已向{0}寫入空內容",
    ),
    (
        "Failed to write last message file {0}: {1}",
        "寫入最後訊息檔案{0}失敗：{1}",
    ),
    ("session id", "會話id"),
    ("session id `{0}` is invalid: {1}", "會話id`{0}`無效：{1}"),
    (
        "forked-from thread id is invalid: {0}",
        "派生來源執行緒id無效：{0}",
    ),
    ("parent thread id is invalid: {0}", "父執行緒id無效：{0}"),
    (
        "in-process app-server event stream lagged; dropped {0} events",
        "程序內app-server事件流滯後；丟棄了{0}個事件",
    ),
    (
        "failed to encode mcp elicitation response: {0}",
        "編碼mcp elicitation響應失敗：{0}",
    ),
    (
        "failed to resolve `{0}` server request: {1}",
        "處理服務端請求`{0}`失敗：{1}",
    ),
    (
        "failed to reject `{0}` server request: {1}",
        "拒絕服務端請求`{0}`失敗：{1}",
    ),
    (
        "command execution approval is not supported in exec mode for thread `{0}`",
        "exec模式下不支援命令執行審批（執行緒`{0}`）",
    ),
    (
        "file change approval is not supported in exec mode for thread `{0}`",
        "exec模式下不支援檔案變更審批（執行緒`{0}`）",
    ),
    (
        "request_user_input is not supported in exec mode for thread `{0}`",
        "exec模式下不支援request_user_input（執行緒`{0}`）",
    ),
    (
        "dynamic tool calls are not supported in exec mode for thread `{0}`",
        "exec模式下不支援動態工具呼叫（執行緒`{0}`）",
    ),
    (
        "chatgpt auth token refresh is not supported in exec mode",
        "exec模式下不支援chatgpt認證令牌重新整理",
    ),
    (
        "attestation generation is not supported in exec mode",
        "exec模式下不支援生成證明",
    ),
    (
        "external current time is not supported in exec mode",
        "exec模式下不支援外部當前時間",
    ),
    (
        "apply_patch approval is not supported in exec mode for thread `{0}`",
        "exec模式下不支援apply_patch審批（執行緒`{0}`）",
    ),
    (
        "exec command approval is not supported in exec mode for thread `{0}`",
        "exec模式下不支援exec命令審批（執行緒`{0}`）",
    ),
    (
        "permissions approval is not supported in exec mode for thread `{0}`",
        "exec模式下不支援許可權審批（執行緒`{0}`）",
    ),
    (
        "failed to initialize local thread metadata",
        "初始化本地執行緒後設資料失敗",
    ),
    (
        "one or more rollout migrations failed",
        "一個或多個rollout遷移失敗",
    ),
    ("Scanning local rollouts...", "正在掃描本地rollout…"),
    (
        "{0} eligible  •  {1} already paginated",
        "{0}個符合條件  •  {1}個已分頁",
    ),
    (
        "{0} migrated  •  {1} already paginated",
        "{0}個已遷移  •  {1}個已分頁",
    ),
    (
        "{0} rollouts  {1}/{2} ({3}%)  •  {4}  •  {5} skipped  •  {6} failed  •  {7}",
        "{0}個rollout  {1}/{2} ({3}%)  •  {4}  •  已跳過{5}  •  失敗{6}  •  用時{7}",
    ),
    ("Scan complete", "掃描完成"),
    ("Migration complete", "遷移完成"),
    ("{0} in {1}.", "{0}，用時{1}。"),
    (
        "Scanned {0} rollout(s): {1} eligible, {2} already paginated, {3} skipped ({4} empty, {5} busy), {6} failed.",
        "已掃描{0}個rollout：{1}個符合條件，{2}個已分頁，{3}個已跳過（{4}個為空，{5}個使用中），{6}個失敗。",
    ),
    (
        "Scanned {0} rollout(s): {1} migrated, {2} already paginated, {3} skipped ({4} empty, {5} busy), {6} failed.",
        "已掃描{0}個rollout：{1}個已遷移，{2}個已分頁，{3}個已跳過（{4}個為空，{5}個使用中），{6}個失敗。",
    ),
    (
        "Disk used for thread storage: {0} -> {1}",
        "執行緒儲存佔用磁碟：{0} -> {1}",
    ),
    (
        "Run `codex migrate-rollouts --apply` to migrate eligible sessions.",
        "執行`codex migrate-rollouts --apply`以遷移符合條件的會話。",
    ),
    (
        "Codex couldn't start because its local database appears to be damaged.",
        "Codex無法啟動，因為其本地資料庫似乎已損壞。",
    ),
    (
        "Moving the damaged local database aside so Codex can rebuild it from saved data.",
        "正在把損壞的本地資料庫移開，以便Codex從已儲存的資料重建。",
    ),
    (
        "Codex rebuilt its local database.",
        "Codex已重建其本地資料庫。",
    ),
    (
        "Codex detected a damaged local database, moved it into a backup folder, and will continue startup with a fresh database.",
        "Codex檢測到本地資料庫損壞，已將其移入備份資料夾，並將以全新的資料庫繼續啟動。",
    ),
    ("Database path: {0}", "資料庫路徑：{0}"),
    ("Backup folder: {0}", "備份資料夾：{0}"),
    ("Backup folder: unavailable", "備份資料夾：不可用"),
    ("Press Enter to continue.", "按回車繼續。"),
    (
        "Continuing startup with a fresh local database...",
        "正在以全新的本地資料庫繼續啟動…",
    ),
    (
        "Run `codex doctor` to check your setup and get next-step guidance.",
        "執行`codex doctor`檢查你的配置並獲取下一步指引。",
    ),
    (
        "If this keeps happening, share the technical details below when asking for help.",
        "如果反覆出現，請在求助時提供以下技術細節。",
    ),
    (
        "Codex couldn't start because another Codex process is using its local data.",
        "Codex無法啟動，因為另一個Codex程序正在使用其本地資料。",
    ),
    (
        "Quit any other copies of Codex that may still be running, then try again.",
        "請退出其它可能仍在執行的Codex副本，然後重試。",
    ),
    ("Technical details:", "技術細節："),
    ("  Location: {0}", "  位置：{0}"),
    ("  Cause: {0}", "  原因：{0}"),
    (
        "OAuth provider rejected discovered scopes. Retrying without scopes…",
        "OAuth提供方拒絕了探測到的scope，正在不帶scope重試…",
    ),
    ("command is required", "必須提供命令"),
    (
        "OAuth client IDs require a streamable HTTP MCP server",
        "OAuth客戶端ID需要streamable HTTP MCP伺服器",
    ),
    (
        "failed to load MCP servers from {0}",
        "無法從{0}載入MCP伺服器",
    ),
    (
        "failed to write MCP servers to {0}",
        "無法將MCP伺服器寫入{0}",
    ),
    (
        "Added global MCP server '{0}'.",
        "已新增全域性MCP伺服器'{0}'。",
    ),
    ("OAuth callback URL: {0}", "OAuth回撥URL：{0}"),
    (
        "Detected OAuth support. Starting OAuth flow…",
        "檢測到OAuth支援，正在啟動OAuth流程…",
    ),
    ("Successfully logged in.", "登入成功。"),
    (
        "MCP server may or may not require login. Run `codex mcp login {0}` to login.",
        "該MCP伺服器可能需要登入。執行`codex mcp login {0}`登入。",
    ),
    (
        "Removed global MCP server '{0}'.",
        "已移除全域性MCP伺服器'{0}'。",
    ),
    (
        "No MCP server named '{0}' found.",
        "未找到名為'{0}'的MCP伺服器。",
    ),
    (
        "OAuth login is only supported for streamable HTTP servers.",
        "OAuth登入僅支援streamable HTTP伺服器。",
    ),
    (
        "Successfully logged in to MCP server '{0}'.",
        "已成功登入MCP伺服器'{0}'。",
    ),
    (
        "No MCP server named '{0}' found in configuration.",
        "配置中未找到名為'{0}'的MCP伺服器。",
    ),
    (
        "OAuth logout is only supported for streamable_http transports.",
        "OAuth登出僅支援streamable_http傳輸方式。",
    ),
    (
        "Removed OAuth credentials for '{0}'.",
        "已移除'{0}'的OAuth憑據。",
    ),
    (
        "No OAuth credentials stored for '{0}'.",
        "未儲存'{0}'的OAuth憑據。",
    ),
    (
        "failed to delete OAuth credentials: {0}",
        "刪除OAuth憑據失敗：{0}",
    ),
    (
        "No MCP servers configured yet. Try `codex mcp add my-tool -- my-command`.",
        "尚未配置任何MCP伺服器。試試`codex mcp add my-tool -- my-command`。",
    ),
    (
        "environment entries must be in KEY=VALUE form",
        "環境變數條目必須為KEY=VALUE形式",
    ),
    (
        "invalid server name '{0}' (use letters, numbers, '-', '_', ':', '@', '/', '.')",
        "無效的伺服器名'{0}'（請使用字母、數字、'-'、'_'、':'、'@'、'/'、'.'）",
    ),
    (
        "Added plugin `{0}` from marketplace `{1}`.",
        "已從市場源`{1}`新增外掛`{0}`。",
    ),
    ("Installed plugin root: {0}", "已安裝外掛根目錄：{0}"),
    ("failed to list marketplace plugins", "無法列出市場源外掛"),
    (
        "No plugins found in marketplace `{0}`.",
        "市場源`{0}`中沒有找到外掛。",
    ),
    ("No marketplace plugins found.", "沒有找到市場源外掛。"),
    ("Marketplace `{0}`", "市場源`{0}`"),
    ("Remote catalog", "遠端目錄"),
    ("remote plugins are not enabled", "未啟用遠端外掛"),
    (
        "Removed plugin `{0}` from marketplace `{1}`.",
        "已從市場源`{1}`移除外掛`{0}`。",
    ),
    (
        "plugin id `{0}` belongs to marketplace `{1}`, but --marketplace specified `{2}`",
        "外掛ID`{0}`屬於市場源`{1}`，但--marketplace指定了`{2}`",
    ),
    (
        "plugin requires --marketplace unless passed as <plugin>@<marketplace>",
        "外掛需要--marketplace，除非以<plugin>@<marketplace>形式傳入",
    ),
    (
        "Warning: failed to list remote marketplace plugins: {0}",
        "警告：無法列出遠端市場源外掛：{0}",
    ),
    (
        "failed to list remote marketplace plugins",
        "無法列出遠端市場源外掛",
    ),
    (
        "plugin `{0}` was not found in remote marketplace `{1}`",
        "遠端市場源`{1}`中沒有找到外掛`{0}`",
    ),
    (
        "plugin `{0}` matched multiple remote plugins",
        "外掛`{0}`匹配到多個遠端外掛",
    ),
    (
        "plugin `{0}` was not found in marketplace `{1}`",
        "市場源`{1}`中沒有找到外掛`{0}`",
    ),
    (
        "plugin `{0}` in marketplace `{1}` matched multiple marketplace roots",
        "市場源`{1}`中的外掛`{0}`匹配到多個市場源根目錄",
    ),
    ("- `{0}` at {1}: {2}", "- `{0}` 位於 {1}：{2}"),
    (
        "failed to load configured marketplace snapshot(s):\n{0}",
        "無法載入已配置的市場源快照：\n{0}",
    ),
    (
        "configured marketplace entry must be a table",
        "配置的市場源條目必須是表",
    ),
    (
        "configured local marketplace source is missing or empty",
        "配置的本地市場源來源缺失或為空",
    ),
    (
        "marketplace root does not contain a supported manifest",
        "市場源根目錄不包含受支援的清單檔案",
    ),
    (
        "Warning: failed to resolve login log directory: {0}",
        "警告：無法解析登入日誌目錄：{0}",
    ),
    (
        "Warning: failed to create login log directory {1}: {0}",
        "警告：無法建立登入日誌目錄{0}：{1}",
    ),
    (
        "Warning: failed to open login log file {1}: {0}",
        "警告：無法開啟登入日誌檔案{0}：{1}",
    ),
    (
        "Warning: failed to initialize login log file {1}: {0}",
        "警告：無法初始化登入日誌檔案{0}：{1}",
    ),
    (
        "Starting local login server on http://localhost:{0}.\nIf your browser did not open, navigate to this URL to authenticate:\n\n{1}\n\nOn a remote or headless machine? Use `codex login --device-auth` instead.",
        "正在http://localhost:{0}啟動本地登入伺服器。\n如果瀏覽器沒有自動開啟，請訪問此URL完成認證：\n\n{1}\n\n在遠端或無頭機器上？請改用`codex login --device-auth`。",
    ),
    ("Error logging in: {0}", "登入出錯：{0}"),
    (
        "Error logging in with access token: {0}",
        "使用訪問令牌登入出錯：{0}",
    ),
    (
        "--with-api-key expects the API key on stdin. Try piping it, e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`.",
        "--with-api-key需要從標準輸入讀取API金鑰。試試用管道傳入，例如`printenv OPENAI_API_KEY | codex login --with-api-key`",
    ),
    (
        "Reading API key from stdin...",
        "正在從標準輸入讀取API金鑰…",
    ),
    (
        "No API key provided via stdin.",
        "沒有透過標準輸入提供API金鑰。",
    ),
    (
        "--with-access-token expects the access token on stdin. Try piping it, e.g. `printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`.",
        "--with-access-token需要從標準輸入讀取訪問令牌。試試用管道傳入，例如`printenv CODEX_ACCESS_TOKEN | codex login --with-access-token`",
    ),
    (
        "Reading access token from stdin...",
        "正在從標準輸入讀取訪問令牌…",
    ),
    (
        "No access token provided via stdin.",
        "沒有透過標準輸入提供訪問令牌。",
    ),
    ("Failed to read stdin: {0}", "讀取標準輸入失敗：{0}"),
    (
        "Error logging in with device code: {0}",
        "使用裝置碼登入出錯：{0}",
    ),
    (
        "Device code login is not enabled; falling back to browser login.",
        "未啟用裝置碼登入；回退到瀏覽器登入。",
    ),
    (
        "Logged in using workload identity",
        "已使用工作負載身份登入",
    ),
    ("Error checking login status: {0}", "檢查登入狀態出錯：{0}"),
    (
        "Logged in using an API key - {0}",
        "已使用API金鑰登入 - {0}",
    ),
    (
        "Unexpected error retrieving API key: {0}",
        "獲取API金鑰時出現意外錯誤：{0}",
    ),
    ("Logged in using ChatGPT", "已使用ChatGPT登入"),
    (
        "Logged in using personal access token",
        "已使用個人訪問令牌登入",
    ),
    (
        "Logged in using Amazon Bedrock API key",
        "已使用Amazon Bedrock API金鑰登入",
    ),
    (
        "Logged in using Amazon Bedrock AWS access keys",
        "已使用Amazon Bedrock AWS訪問金鑰登入",
    ),
    ("Error logging out: {0}", "登出出錯：{0}"),
    (
        "Error clearing Amazon Bedrock configuration after logout: {0}",
        "登出後清除Amazon Bedrock配置出錯：{0}",
    ),
    ("Successfully logged out", "已成功登出"),
    (
        "ChatGPT login is disabled. Use API key login instead.",
        "ChatGPT登入已被禁用。請改用API金鑰登入。",
    ),
    (
        "API key login is disabled. Use ChatGPT login instead.",
        "API金鑰登入已被禁用。請改用ChatGPT登入。",
    ),
    (
        "Access token login is disabled. Use API key login instead.",
        "訪問令牌登入已被禁用。請改用API金鑰登入。",
    ),
    ("Successfully logged in", "登入成功"),
    (
        "`codex agents` cannot apply local provider or additional-directory overrides to a remote server",
        "`codex agents` 無法對遠端伺服器應用本地provider或附加目錄覆蓋項",
    ),
    (
        "The --api-key flag is no longer supported. Pipe the key instead, e.g. `printenv OPENAI_API_KEY | codex login --with-api-key`.",
        "不再支援--api-key引數。請改用管道傳入金鑰，例如`printenv OPENAI_API_KEY | codex login --with-api-key`。",
    ),
    (
        "--profile only applies to runtime commands and `codex mcp`: `codex`, `codex exec`, `codex review`, `codex resume`, `codex queue`, `codex archive`, `codex delete`, `codex unarchive`, `codex fork`, `codex mcp`, `codex sandbox`, and `codex debug prompt-input`.",
        "--profile只適用於執行時命令與`codex mcp`：`codex`、`codex exec`、`codex review`、`codex resume`、`codex queue`、`codex archive`、`codex delete`、`codex unarchive`、`codex fork`、`codex mcp`、`codex sandbox` 和 `codex debug prompt-input`。",
    ),
    (
        "remote exec-server registration requires ChatGPT authentication or API key authentication; run `codex login` or set CODEX_API_KEY",
        "遠端exec-server註冊需要ChatGPT認證或API金鑰認證；請執行`codex login`或設定CODEX_API_KEY",
    ),
    (
        "remote exec-server registration requires ChatGPT authentication or API key authentication; Agent Identity auth requires --use-agent-identity-auth",
        "遠端exec-server註冊需要ChatGPT認證或API金鑰認證；請執行`codex login`或設定CODEX_API_KEY",
    ),
    (
        "remote exec-server API-key authentication is restricted to HTTPS openai.com and openai.org hosts and subdomains or loopback hosts",
        "遠端exec-server的API金鑰認證僅限HTTPS的openai.com與openai.org主機及子域，或迴環主機",
    ),
    (
        "Under-development features enabled: {0}. Under-development features are incomplete and may behave unpredictably. To suppress this warning, set `suppress_unstable_features_warning = true` in {1}.",
        "已啟用開發中特性：{0}。開發中特性尚不完整，行為可能不可預測。要抑制此警告，請在{1}中設定`suppress_unstable_features_warning = true`。",
    ),
    (
        " Cleared memory directories under {0}.",
        " 已清除位於{0}的記憶體目錄。",
    ),
    (
        "`--remote {0}` is only supported for interactive TUI commands, not `codex {1}`",
        "`--remote {0}`僅支援互動式TUI命令，不支援`codex {1}`",
    ),
    (
        "`--remote-auth-token-env` is only supported for interactive TUI commands, not `codex {0}`",
        "`--remote-auth-token-env`僅支援互動式TUI命令，不支援`codex {0}`",
    ),
    (
        "`--worktree` supports new interactive sessions, `codex fork`, `codex exec`, and `codex exec fork`",
        "`--worktree`支援新建互動式會話、`codex fork`、`codex exec` 和 `codex exec fork`",
    ),
    (
        "TERM is set to \"dumb\". Refusing to start the interactive TUI because no terminal is available for a confirmation prompt (stdin/stderr is not a TTY). Run in a supported terminal or unset TERM.",
        "TERM被設為\"dumb\"。由於沒有可用於確認提示的終端（stdin/stderr不是TTY），拒絕啟動互動式TUI。請在受支援的終端中執行，或取消設定TERM。",
    ),
    (
        "WARNING: TERM is set to \"dumb\". Codex's interactive TUI may not work in this terminal.",
        "警告：TERM被設為\"dumb\"。Codex的互動式TUI在此終端中可能無法正常工作。",
    ),
    (
        "failed to move damaged Codex local database files into a backup folder automatically: {0}",
        "無法自動把損壞的Codex本地資料庫檔案移動到備份資料夾：{0}",
    ),
    (
        "Marketplace `{0}` is already added from {1}.",
        "市場源`{0}`已從{1}新增過。",
    ),
    (
        "Added marketplace `{0}` from {1}.",
        "已從{1}新增市場源`{0}`。",
    ),
    (
        "Installed marketplace root: {0}",
        "已安裝的市場源根目錄：{0}",
    ),
    ("failed to list plugin marketplaces", "無法列出外掛市場源"),
    (
        "failed to load marketplace(s):\n{0}",
        "無法載入市場源：\n{0}",
    ),
    ("No plugin marketplaces in scope.", "範圍內沒有外掛市場源。"),
    ("Removed marketplace `{0}`.", "已移除市場源`{0}`。"),
    (
        "Removed installed marketplace root: {0}",
        "已移除已安裝的市場源根目錄：{0}",
    ),
    (
        "Failed to upgrade marketplace `{0}`: {1}",
        "升級市場源`{0}`失敗：{1}",
    ),
    ("{0} upgrade failure(s) occurred.", "發生{0}次升級失敗。"),
    (
        "Marketplace `{0}` is already up to date.",
        "市場源`{0}`已是最新。",
    ),
    (
        "All configured Git marketplaces are already up to date.",
        "所有已配置的Git市場源都已是最新。",
    ),
    (
        "Upgraded marketplace `{0}` to the latest configured revision.",
        "已把市場源`{0}`升級到最新配置的修訂版本。",
    ),
    ("Upgraded {0} marketplace(s).", "已升級{0}個市場源。"),
    (
        "Starting app-server with remote control enabled...",
        "正在啟動啟用遠端控制的app-server…",
    ),
    (
        "Starting app-server daemon with remote control enabled...",
        "正在啟動啟用遠端控制的app-server守護程序…",
    ),
    ("Stopping remote control...", "正在停止遠端控制…"),
    (
        "failed to flush remote-control progress message",
        "重新整理遠端控制進度訊息失敗",
    ),
    (
        "failed to create private app-server socket directory",
        "建立私有app-server套接字目錄失敗",
    ),
    (
        "private app-server socket path was not absolute",
        "私有app-server套接字路徑不是絕對路徑",
    ),
    ("failed to listen for Ctrl-C: {0}", "監聽Ctrl-C失敗：{0}"),
    (
        "foreground app-server task failed to join",
        "前臺app-server任務join失敗",
    ),
    (
        "foreground app-server exited with an error",
        "前臺app-server因錯誤退出",
    ),
    (
        "foreground app-server exited before remote control became ready",
        "遠端控制就緒之前，前臺app-server已退出",
    ),
    (
        "foreground app-server task failed before remote control became ready",
        "遠端控制就緒之前，前臺app-server任務失敗",
    ),
    (
        "This machine is available for remote control as {0}.",
        "本機可被作為{0}用於遠端控制。",
    ),
    (
        "Remote control is enabled on {0} and still connecting.",
        "遠端控制已在{0}上啟用，仍在連線中。",
    ),
    (
        "Remote control is enabled on {0} but the connection is errored.",
        "遠端控制已在{0}上啟用，但連線出錯。",
    ),
    (
        "Remote control is disabled on {0}.",
        "{0}上的遠端控制已被禁用。",
    ),
    ("Press Ctrl-C to stop.", "按Ctrl-C停止。"),
    ("Daemon used app-server:", "守護程序使用的app-server："),
    (
        "remote-control pairing response did not include a manual pairing code",
        "遠端控制配對響應中沒有包含手動配對碼",
    ),
    ("Pairing code: {0}", "配對碼：{0}"),
    ("Remote control stopped.", "遠端控制已停止。"),
    ("Remote control is not running.", "遠端控制未在執行。"),
    (
        "Remote control stop completed with status {0}.",
        "遠端控制停止完成，狀態為{0}。",
    ),
    (
        "Seatbelt sandbox is only available on macOS",
        "Seatbelt沙箱僅在macOS上可用",
    ),
    (
        "invalid --sandbox-state-json value: {0}",
        "無效的--sandbox-state-json值：{0}",
    ),
    (
        "sandbox state cwd is not native to this host",
        "沙箱狀態的工作目錄不屬於本機",
    ),
    (
        "--sandbox-state-disable-network cannot be applied to a disabled permission profile",
        "--sandbox-state-disable-network無法應用於已禁用的許可權配置",
    ),
    ("sandbox command must not be empty", "沙箱命令不能為空"),
    (
        "Windows sandbox is only available on Windows",
        "Windows沙箱僅在Windows上可用",
    ),
    (
        "Seatbelt command is missing its generated policy",
        "Seatbelt命令缺少其生成的策略",
    ),
    ("None found.", "未找到。"),
    ("windows sandbox failed: {0}", "windows沙箱失敗：{0}"),
    ("failed to parse --from", "無法解析--from"),
    ("failed to parse --to", "無法解析--to"),
    (
        "expected RFC3339 or unix seconds, got {0}",
        "期望RFC3339或Unix秒，實際為{0}",
    ),
    ("failed to fetch backfill logs", "無法獲取回填日誌"),
    ("failed to fetch new logs", "無法獲取新日誌"),
    ("failed to fetch max log id", "無法獲取最大日誌ID"),
    (
        "`codex sandbox setup` currently requires --elevated",
        "`codex sandbox setup`當前需要--elevated",
    ),
    (
        "failed to load target user's Codex config for sandbox provisioning",
        "無法載入目標使用者的Codex配置以進行沙箱配置",
    ),
    (
        "sandbox provisioning succeeded, but failed to persist elevated sandbox config: {0}",
        "沙箱配置成功，但持久化提權沙箱配置失敗：{0}",
    ),
    (
        "Windows elevated sandbox setup completed for {0} at {1}.",
        "已完成{0}的Windows提權沙箱設定（{1}）。",
    ),
    (
        "failed to determine current user from environment: {0}",
        "無法從環境確定當前使用者：{0}",
    ),
    (
        "--user or --current-user is required",
        "需要--user或--current-user",
    ),
    (
        "--codex-home is required with --user",
        "使用--user時需要--codex-home",
    ),
    ("failed to resolve current directory", "無法解析當前目錄"),
    ("failed to load bootstrap configuration", "無法載入引導配置"),
    (
        "failed to resolve cloud configuration authentication",
        "無法解析雲配置認證",
    ),
    (
        "failed to initialize cloud configuration authentication",
        "無法初始化雲配置認證",
    ),
    (
        "Could not listen for exec-server shutdown signal: {0}",
        "無法監聽exec-server關閉訊號：{0}",
    ),
    ("invalid path {0}: {1}", "無效路徑{0}：{1}"),
    (
        "`codex queue` does not support image attachments",
        "`codex queue` 不支援圖片附件",
    ),
    (
        "Opening workspace {0} in the Desktop app...",
        "正在桌面應用中開啟工作區{0}…",
    ),
    (
        "Desktop app not found; opening Windows installer...",
        "未找到桌面應用；正在開啟Windows安裝程式…",
    ),
    (
        "After installing the Desktop app, open workspace {0}.",
        "安裝桌面應用後，請開啟工作區{0}。",
    ),
    (
        "failed to invoke `powershell.exe`",
        "呼叫`powershell.exe`失敗",
    ),
    ("failed to open {0}", "開啟{0}失敗"),
    ("failed to open {0} with {1}", "開啟{0}失敗（{1}）"),
    ("Opening Desktop app at {0}...", "正在開啟桌面應用：{0}…"),
    (
        "Desktop app not found; downloading installer...",
        "未找到桌面應用；正在下載安裝程式…",
    ),
    (
        "failed to download/install Desktop app",
        "下載/安裝桌面應用失敗",
    ),
    (
        "Launching Desktop app from {0}...",
        "正在從{0}啟動桌面應用…",
    ),
    ("Opening workspace {0}...", "正在開啟工作區{0}…"),
    ("failed to invoke `open`", "呼叫`open`失敗"),
    (
        "`open -a {0} {1}` exited with {2}",
        "`open -a {0} {1}` 以 {2} 退出",
    ),
    (
        "failed to verify Desktop app signature",
        "校驗桌面應用簽名失敗",
    ),
    (
        "Desktop app at {2} failed OpenAI signature verification (team {0}, bundle {1}): {3}",
        "桌面應用位於{0}，未透過OpenAI簽名校驗（團隊{1}，bundle{2}）：{3}",
    ),
    ("failed to create temp dir", "建立臨時目錄失敗"),
    (
        "Mounting Desktop app installer...",
        "正在掛載桌面應用安裝程式…",
    ),
    ("Installer mounted at {0}.", "安裝程式已掛載到{0}。"),
    (
        "failed to locate Codex.app in mounted dmg",
        "在已掛載的dmg中找不到Codex.app",
    ),
    (
        "refusing to install an unverified Desktop app",
        "拒絕安裝未經驗證的桌面應用",
    ),
    (
        "warning: failed to detach dmg at {0}: {1}",
        "警告：無法解除安裝{0}處的dmg：{1}",
    ),
    (
        "Installing Desktop app into {0}...",
        "正在把桌面應用安裝到{0}…",
    ),
    (
        "failed to create applications dir {0}",
        "建立應用目錄{0}失敗",
    ),
    (
        "warning: failed to install Codex.app to {0}: {1}",
        "警告：無法把Codex.app安裝到{0}：{1}",
    ),
    (
        "failed to install Codex.app to any applications directory",
        "無法把Codex.app安裝到任何應用目錄",
    ),
    ("Downloading installer...", "正在下載安裝程式…"),
    ("failed to invoke `curl`", "呼叫`curl`失敗"),
    ("curl download failed with {0}", "curl下載失敗，狀態為{0}"),
    (
        "failed to invoke `hdiutil attach`",
        "呼叫`hdiutil attach`失敗",
    ),
    (
        "`hdiutil attach` failed with {0}: {1}",
        "`hdiutil attach` 以 {0} 失敗：{1}",
    ),
    (
        "failed to parse mount point from hdiutil output:\n{0}",
        "無法從hdiutil輸出解析掛載點：\n{0}",
    ),
    (
        "failed to invoke `hdiutil detach`",
        "呼叫`hdiutil detach`失敗",
    ),
    (
        "hdiutil detach failed with {0}",
        "hdiutil detach失敗，狀態為{0}",
    ),
    ("failed to read {0}", "讀取{0}失敗"),
    (
        "failed to read mount directory entry",
        "讀取掛載目錄條目失敗",
    ),
    ("no .app bundle found at {0}", "在{0}處沒有找到.app bundle"),
    ("failed to invoke `ditto`", "呼叫`ditto`失敗"),
    ("ditto copy failed with {0}", "ditto複製失敗，狀態為{0}"),
    (
        "Failed to create unified exec process: {0}",
        "建立統一執行程序失敗：{0}",
    ),
    ("Unified exec process failed: {0}", "統一執行程序失敗：{0}"),
    ("Unknown process id {0}", "未知的程序ID {0}"),
    ("stdin approval failed: {0}", "標準輸入審批失敗：{0}"),
    ("failed to write to stdin", "寫入標準輸入失敗"),
    (
        "stdin is closed for this session; rerun exec_command with tty=true to keep stdin open",
        "本會話的標準輸入已關閉；要保留標準輸入，請用tty=true重新執行exec_command",
    ),
    (
        "missing command line for unified exec request",
        "統一執行請求缺少命令列",
    ),
    ("Command denied by sandbox: {0}", "命令被沙箱拒絕：{0}"),
    ("{0} is not valid on {1}", "{0} 在 {1} 上無效"),
    (
        "This app tool requires a non-empty string link_id argument",
        "此應用工具要求link_id引數為非空字串",
    ),
    (
        "plugin resource path `{0}` is outside package root `{1}`",
        "外掛資源路徑`{0}`位於包根目錄`{1}`之外",
    ),
    (
        "invalid plugin key `{0}`; expected <plugin>@<marketplace>",
        "無效的外掛鍵`{0}`；應為<plugin>@<marketplace>",
    ),
    ("{0} in `{1}`", "{0}（位於`{1}`）"),
    ("invalid {0}: must not be empty", "{0}無效：不能為空"),
    (
        "invalid {0}: path traversal is not allowed",
        "{0}無效：不允許路徑穿越",
    ),
    (
        "invalid {0}: dots must separate non-empty name segments",
        "{0}無效：點號必須分隔非空的名稱段",
    ),
    (
        "ASCII letters, digits, `.`, `_`, and `-`",
        "ASCII字母、數字、`.`、`_`和`-`",
    ),
    (
        "ASCII letters, digits, `_`, and `-`",
        "ASCII字母、數字、`_`和`-`",
    ),
    ("invalid {0}: only {1} are allowed", "{0}無效：只允許{1}"),
    ("plugin name", "外掛名稱"),
    ("marketplace name", "市場源名稱"),
    (
        "failed to read remote plugin install metadata",
        "讀取遠端外掛安裝後設資料失敗",
    ),
    (
        "failed to parse remote plugin install metadata: {0}",
        "解析遠端外掛安裝後設資料失敗：{0}",
    ),
    (
        "unsupported remote plugin install metadata schema version: {0}",
        "不支援的遠端外掛安裝後設資料schema版本：{0}",
    ),
    (
        "invalid remote plugin install metadata: remote plugin id must not be blank",
        "無效的遠端外掛安裝後設資料：遠端外掛id不能為空",
    ),
    (
        "failed to resolve plugin cache root",
        "解析外掛快取根目錄失敗",
    ),
    (
        "failed to resolve plugin data root",
        "解析外掛資料根目錄失敗",
    ),
    ("failed to resolve Codex home", "解析Codex主目錄失敗"),
    (
        "cannot write remote identity for uninstalled plugin `{0}`",
        "不能為未安裝的外掛 `{0}` 寫入遠端標識",
    ),
    (
        "remote plugin install metadata path has no parent: {0}",
        "遠端外掛安裝後設資料路徑沒有父目錄：{0}",
    ),
    (
        "failed to serialize remote plugin install metadata: {0}",
        "序列化遠端外掛安裝後設資料失敗：{0}",
    ),
    (
        "failed to create temporary remote plugin install metadata",
        "建立臨時遠端外掛安裝後設資料失敗",
    ),
    (
        "failed to write remote plugin install metadata",
        "寫入遠端外掛安裝後設資料失敗",
    ),
    (
        "failed to flush remote plugin install metadata",
        "重新整理遠端外掛安裝後設資料失敗",
    ),
    (
        "failed to persist remote plugin install metadata",
        "持久化遠端外掛安裝後設資料失敗",
    ),
    (
        "plugin source path is not a directory: {0}",
        "外掛源路徑不是目錄：{0}",
    ),
    (
        "plugin.json name `{0}` does not match marketplace plugin name `{1}`",
        "plugin.json中的名稱 `{0}` 與市場外掛名稱 `{1}` 不匹配",
    ),
    (
        "failed to remove remote plugin install metadata",
        "刪除遠端外掛安裝後設資料失敗",
    ),
    (
        "invalid plugin version: must not be empty",
        "無效的外掛版本：不能為空",
    ),
    (
        "invalid plugin version: path traversal is not allowed",
        "無效的外掛版本：不允許路徑穿越",
    ),
    (
        "invalid plugin version: only ASCII letters, digits, `.`, `+`, `_`, and `-` are allowed",
        "無效的外掛版本：只允許ASCII字母、數字以及 `.`、`+`、`_`、`-`",
    ),
    ("missing or invalid plugin.json", "缺少或無效的plugin.json"),
    (
        "failed to parse plugin.json: {0}",
        "解析plugin.json失敗：{0}",
    ),
    ("missing plugin.json", "缺少plugin.json"),
    ("failed to read plugin.json", "讀取plugin.json失敗"),
    (
        "invalid plugin version in plugin.json: expected string",
        "plugin.json中的外掛版本無效：需要字串",
    ),
    (
        "invalid plugin version in plugin.json: must not be blank",
        "plugin.json中的外掛版本無效：不能為空",
    ),
    (
        "failed to remove existing plugin cache entry",
        "刪除已有的外掛快取條目失敗",
    ),
    (
        "plugin cache path has no parent: {0}",
        "外掛快取路徑沒有父目錄：{0}",
    ),
    (
        "failed to create plugin cache directory",
        "建立外掛快取目錄失敗",
    ),
    (
        "plugin cache path has no directory name: {0}",
        "外掛快取路徑沒有目錄名：{0}",
    ),
    (
        "failed to create temporary plugin cache directory",
        "建立臨時外掛快取目錄失敗",
    ),
    (
        "plugin manifest is outside the plugin source",
        "外掛清單位於外掛源之外",
    ),
    (
        "plugin manifest path has no parent",
        "外掛清單路徑沒有父目錄",
    ),
    (
        "failed to create plugin manifest directory",
        "建立外掛清單目錄失敗",
    ),
    (
        "failed to write fallback plugin manifest",
        "寫入回退外掛清單失敗",
    ),
    (
        "plugin manifest is missing after installation staging",
        "安裝暫存後缺少外掛清單",
    ),
    (
        "plugin manifest changed during installation staging",
        "外掛清單在安裝暫存期間發生變化",
    ),
    (
        "failed to read staged plugin.json",
        "讀取暫存的plugin.json失敗",
    ),
    (
        "plugin manifest contents changed during installation staging",
        "外掛清單內容在安裝暫存期間發生變化",
    ),
    (
        "failed to activate updated plugin cache version",
        "啟用更新後的外掛快取版本失敗",
    ),
    (
        "failed to create plugin cache backup directory",
        "建立外掛快取備份目錄失敗",
    ),
    (
        "failed to back up plugin cache entry",
        "備份外掛快取條目失敗",
    ),
    (
        "failed to activate updated plugin cache entry",
        "啟用更新後的外掛快取條目失敗",
    ),
    (
        "failed to activate updated plugin cache entry at {2}: {0}; failed to restore previous cache entry (left at {3}): {1}",
        "啟用位於 {2} 的更新後外掛快取條目失敗：{0}；恢復先前的快取條目失敗（留在 {3}）：{1}",
    ),
    (
        "failed to activate plugin cache entry",
        "啟用外掛快取條目失敗",
    ),
    (
        "failed to activate updated plugin cache version `{0}` while `{1}` remains active",
        "啟用更新後的外掛快取版本 `{0}` 失敗，而 `{1}` 仍處於活動狀態",
    ),
    (
        "failed to create plugin target directory",
        "建立外掛目標目錄失敗",
    ),
    (
        "failed to read plugin source directory",
        "讀取外掛源目錄失敗",
    ),
    ("failed to enumerate plugin source", "列舉外掛源失敗"),
    (
        "failed to inspect plugin source entry",
        "檢查外掛源條目失敗",
    ),
    ("failed to copy plugin file", "複製外掛檔案失敗"),
    ("git ls-remote marketplace source", "git ls-remote市場來源"),
    (
        "git ls-remote returned empty output for marketplace source",
        "git ls-remote對市場來源返回了空輸出",
    ),
    (
        "unexpected git ls-remote output for marketplace source: {0}",
        "git ls-remote對市場來源的輸出異常：{0}",
    ),
    (
        "git ls-remote returned empty revision for marketplace source",
        "git ls-remote對市場來源返回了空修訂版本",
    ),
    ("git clone marketplace source", "git clone市場來源"),
    ("git checkout marketplace ref", "git checkout市場引用"),
    (
        "git sparse-checkout marketplace source",
        "git sparse-checkout市場來源",
    ),
    (
        "git rev-parse marketplace revision",
        "git rev-parse市場修訂版本",
    ),
    (
        "git rev-parse returned empty revision for marketplace source",
        "git rev-parse對市場來源返回了空修訂版本",
    ),
    ("failed to run {0}: {1}", "執行{0}失敗：{1}"),
    ("failed to wait for {0}: {1}", "等待{0}失敗：{1}"),
    ("failed to poll {0}: {1}", "輪詢{0}失敗：{1}"),
    (
        "failed to wait for {0} after timeout: {1}",
        "超時後等待{0}失敗：{1}",
    ),
    ("{0} timed out after {1}s", "{0}在{1}秒後超時"),
    ("{0} timed out after {2}s: {1}", "{0}在{2}秒後超時：{1}"),
    ("{0} failed with status {1}", "{0}失敗，狀態碼{1}"),
    ("{0} failed with status {2}: {1}", "{0}失敗，狀態碼{2}：{1}"),
    (
        "failed to join remote plugin bundle install task: {0}",
        "加入遠端外掛bundle安裝任務失敗：{0}",
    ),
    (
        "failed to join remote plugin bundle extraction task: {0}",
        "加入遠端外掛bundle抽取任務失敗：{0}",
    ),
    (
        "\n[response body truncated after {0} bytes]",
        "\\\n[響應體在{0}位元組後被截斷]",
    ),
    (
        "\n[failed to read response body: {0}]",
        "\\\n[讀取響應體失敗：{0}]",
    ),
    (
        "failed to create remote plugin bundle staging directory",
        "建立遠端外掛bundle暫存目錄失敗",
    ),
    (
        "failed to create remote plugin bundle extraction directory",
        "建立遠端外掛bundle抽取目錄失敗",
    ),
    (
        "failed to resolve extracted remote plugin bundle root: {0}",
        "解析抽出的遠端外掛bundle根目錄失敗：{0}",
    ),
    (
        "plugin checkout destination already exists: {0}",
        "外掛檢出目標已存在：{0}",
    ),
    (
        "plugin checkout destination has no parent: {0}",
        "外掛檢出目標沒有父目錄：{0}",
    ),
    (
        "failed to create plugin checkout directory",
        "建立外掛檢出目錄失敗",
    ),
    (
        "remote plugin bundle did not contain a valid plugin.json",
        "遠端外掛bundle中沒有有效的plugin.json",
    ),
    (
        "plugin.json name `{0}` does not match remote plugin name `{1}`",
        "plugin.json名稱 `{0}` 與遠端外掛名稱 `{1}` 不匹配",
    ),
    (
        "failed to activate checked out plugin directory",
        "啟用已檢出的外掛目錄失敗",
    ),
    (
        "failed to read remote plugin manifest",
        "讀取遠端外掛清單失敗",
    ),
    (
        "failed to parse remote plugin manifest: {0}",
        "解析遠端外掛清單失敗：{0}",
    ),
    (
        "remote plugin manifest must be a JSON object",
        "遠端外掛清單必須是JSON物件",
    ),
    (
        "failed to write remote plugin manifest",
        "寫入遠端外掛清單失敗",
    ),
    (
        "failed to write remote plugin app manifest",
        "寫入遠端外掛應用清單失敗",
    ),
    (
        "remote plugin output path has no parent: {0}",
        "遠端外掛輸出路徑沒有父目錄：{0}",
    ),
    (
        "failed to serialize remote plugin JSON override: {0}",
        "序列化遠端外掛JSON覆蓋失敗：{0}",
    ),
    (
        "remote plugin bundle did not contain a standard plugin root with plugin.json",
        "遠端外掛bundle不包含帶plugin.json的標準外掛根目錄",
    ),
    (
        "backend did not return a release version for remote plugin `{0}`",
        "後端未為遠端外掛 `{0}` 返回釋出版本",
    ),
    (
        "backend returned an invalid release version for remote plugin `{0}`: {1}",
        "後端為遠端外掛 `{0}` 返回了無效的釋出版本：{1}",
    ),
    (
        "backend did not return a download URL for remote plugin `{0}`",
        "後端未為遠端外掛 `{0}` 返回下載URL",
    ),
    (
        "backend returned an invalid download URL for remote plugin `{0}`: {1}",
        "後端為遠端外掛 `{0}` 返回了無效的下載URL：{1}",
    ),
    (
        "backend returned an unsupported download URL scheme for remote plugin `{0}`: {1}",
        "後端為遠端外掛 `{0}` 返回了不支援的下載URL方案：{1}",
    ),
    (
        "backend returned an invalid local plugin id for remote plugin `{0}`: {1}",
        "後端為遠端外掛 `{0}` 返回了無效的本地外掛id：{1}",
    ),
    (
        "failed to send remote plugin bundle download request to {0}: {1}",
        "向{0}傳送遠端外掛bundle下載請求失敗：{1}",
    ),
    (
        "remote plugin bundle download from {0} failed with status {1}: {2}",
        "從{0}下載遠端外掛bundle失敗，狀態碼{1}：{2}",
    ),
    (
        "failed to read remote plugin bundle download response from {0}: {1}",
        "讀取從{0}下載的遠端外掛bundle響應失敗：{1}",
    ),
    (
        "remote plugin bundle download from {0} exceeded maximum size of {1} bytes",
        "從{0}下載的遠端外掛bundle超過最大大小{1}位元組",
    ),
    (
        "remote plugin bundle download from {0} redirected to unsupported URL {1}",
        "從{0}下載的遠端外掛bundle被重定向到不支援的URL {1}",
    ),
    (
        "remote plugin bundle extracted size would be {0} bytes, exceeding the maximum total size of {1} bytes",
        "遠端外掛bundle解壓後為{0}位元組，超過最大總大小{1}位元組",
    ),
    (
        "failed to read plugin hooks config {1}: {0}",
        "讀取外掛hooks配置{1}失敗：{0}",
    ),
    (
        "failed to parse plugin hooks config {1}: {0}",
        "解析外掛hooks配置{1}失敗：{0}",
    ),
    (
        "failed to create marketplace plugin source staging directory {1}: {0}",
        "建立市場外掛源暫存目錄{1}失敗：{0}",
    ),
    (
        "failed to create marketplace plugin source staging directory in {1}: {0}",
        "在{1}中建立市場外掛源暫存目錄失敗：{0}",
    ),
    (
        "failed to resolve materialized plugin source path: {0}",
        "解析已物化的外掛源路徑失敗：{0}",
    ),
    (
        "checked out Git SHA {0} does not match requested SHA {1}",
        "檢出的Git SHA{0}與請求的SHA{1}不匹配",
    ),
    ("failed to run git {1}: {0}", "執行git {1}失敗：{0}"),
    (
        "git {0} failed with status {1}\nstdout:\n{2}\nstderr:\n{3}",
        "git {0}失敗，狀態碼{1}\\\nstdout:\\\n{2}\\\nstderr:\\\n{3}",
    ),
    (
        "invalid remote plugin id: only ASCII letters, digits, `_`, `-`, and `~` are allowed",
        "無效的遠端外掛id：只允許ASCII字母、數字以及 `_`、`-`、`~`",
    ),
    (
        "workspace plugin `{0}` did not include discoverability",
        "工作區外掛 `{0}` 未包含可發現性資訊",
    ),
    (
        "invalid local plugin id for remote plugin `{1}`: {0}",
        "遠端外掛 `{1}` 的本地外掛id無效：{0}",
    ),
    (
        "failed to join remote plugin cache removal task: {0}",
        "加入遠端外掛快取刪除任務失敗：{0}",
    ),
    (
        "failed to resolve remote plugin cache root: {0}",
        "解析遠端外掛快取根目錄失敗：{0}",
    ),
    (
        "invalid remote plugin cache id for `{0}` in `{1}`: {2}",
        "無效的遠端外掛快取id（`{0}` / `{1}`）：{2}",
    ),
    (
        "failed to remove remote plugin cache entry {1}: {0}",
        "刪除遠端外掛快取條目{1}失敗：{0}",
    ),
    (
        "invalid remote plugin config id for `{2}` in `{0}`: {1}",
        "`{2}` 在 `{0}` 中的遠端外掛配置id無效：{1}",
    ),
    (
        "chatgpt authentication required for remote plugin catalog",
        "遠端外掛目錄需要ChatGPT身份驗證",
    ),
    (
        "chatgpt authentication required for remote plugin catalog; api key auth is not supported",
        "遠端外掛目錄需要ChatGPT身份驗證；不支援API key方式",
    ),
    (
        "failed to read auth token for remote plugin catalog: {0}",
        "讀取遠端外掛目錄的身份驗證令牌失敗：{0}",
    ),
    (
        "failed to send remote plugin catalog request to {0}: {1}",
        "向{0}傳送遠端外掛目錄請求失敗：{1}",
    ),
    (
        "remote plugin catalog request to {0} failed with status {1}: {2}",
        "向{0}傳送的遠端外掛目錄請求失敗，狀態碼{1}：{2}",
    ),
    (
        "failed to parse remote plugin catalog response from {0}: {1}",
        "解析來自{0}的遠端外掛目錄響應失敗：{1}",
    ),
    (
        "invalid remote plugin catalog base URL: {0}",
        "無效的遠端外掛目錄基礎URL：{0}",
    ),
    (
        "invalid remote plugin catalog base URL path",
        "無效的遠端外掛目錄基礎URL路徑",
    ),
    (
        "remote marketplace `{0}` is not supported",
        "不支援遠端市場 `{0}`",
    ),
    (
        "remote plugin mutation returned unexpected plugin id: expected `{0}`, got `{1}`",
        "遠端外掛變更返回了非預期的外掛id：期望 `{0}`，實際 `{1}`",
    ),
    (
        "remote plugin skill response returned unexpected skill name: expected `{0}`, got `{1}`",
        "遠端外掛技能響應返回了非預期的技能名：期望 `{0}`，實際 `{1}`",
    ),
    (
        "remote plugin mutation returned unexpected enabled state for `{0}`: expected {1}, got {2}",
        "遠端外掛變更返回了 `{0}` 的非預期啟用狀態：期望{1}，實際{2}",
    ),
    (
        "invalid plugin path `{0}`: {1}",
        "無效的外掛路徑 `{0}`：{1}",
    ),
    (
        "remote plugin `{0}` is not available for plugin/share/checkout",
        "遠端外掛 `{0}` 不可用於plugin/share/checkout",
    ),
    (
        "failed to archive plugin at `{0}`: {1}",
        "歸檔位於 `{0}` 的外掛失敗：{1}",
    ),
    (
        "failed to join plugin archive task: {0}",
        "加入外掛歸檔任務失敗：{0}",
    ),
    (
        "plugin archive would be {0} bytes, exceeding the maximum upload size of {1} bytes",
        "外掛歸檔將為{0}位元組，超過最大上傳大小{1}位元組",
    ),
    (
        "workspace plugin upload response did not include an etag",
        "工作區外掛上傳響應未包含etag",
    ),
    (
        "marketplace file is not in a supported location",
        "市場原始檔不在受支援的位置",
    ),
    ("failed to read marketplace file", "讀取市場原始檔失敗"),
    (
        "local plugin source path must not be empty",
        "本地外掛源路徑不能為空",
    ),
    (
        "local plugin source path must start with `./`",
        "本地外掛源路徑必須以 `./` 開頭",
    ),
    (
        "local plugin source path must stay within the marketplace root",
        "本地外掛源路徑必須位於市場源根目錄內",
    ),
    (
        "git plugin source path must not be empty",
        "git外掛源路徑不能為空",
    ),
    (
        "git plugin source path must stay within the repository root",
        "git外掛源路徑必須位於倉庫根目錄內",
    ),
    (
        "git plugin source url must not be empty",
        "git外掛源url不能為空",
    ),
    (
        "invalid git plugin source url: {0}",
        "無效的git外掛源url：{0}",
    ),
    (
        "relative git plugin source url must stay within the marketplace root",
        "相對git外掛源url必須位於市場源根目錄內",
    ),
    (
        "invalid npm plugin source package: {0}",
        "無效的npm外掛源包名：{0}",
    ),
    (
        "npm plugin source version must use the registry: {0}",
        "npm外掛源版本必須使用registry：{0}",
    ),
    (
        "invalid npm plugin source registry: {0}",
        "無效的npm外掛源registry：{0}",
    ),
    (
        "npm plugin source {0} must not be empty",
        "npm外掛源{0}不能為空",
    ),
    (
        "marketplace file `{0}` does not exist",
        "市場原始檔 `{0}` 不存在",
    ),
    (
        "invalid marketplace file `{0}`: {1}",
        "無效的市場原始檔 `{0}`：{1}",
    ),
    (
        "plugin `{0}` is not available for install in marketplace `{1}`",
        "外掛 `{0}` 在市場源 `{1}` 中不可安裝",
    ),
    ("plugins feature is disabled", "外掛功能已禁用"),
    (
        "remote plugin `{0}` returned invalid plugin name: {1}",
        "遠端外掛 `{0}` 返回了無效的外掛名：{1}",
    ),
    (
        "could not determine home directory for personal plugin marketplace",
        "無法確定個人外掛市場的主目錄",
    ),
    (
        "failed to resolve home directory for personal plugin marketplace: {0}",
        "解析個人外掛市場的主目錄失敗：{0}",
    ),
    (
        "failed to prepare remote plugin bundle checkout: {0}",
        "準備遠端外掛包檢出失敗：{0}",
    ),
    (
        "failed to check out remote plugin bundle: {0}",
        "檢出遠端外掛包失敗：{0}",
    ),
    (
        "failed to record plugin share local path mapping: {0}",
        "記錄外掛分享本地路徑對映失敗：{0}",
    ),
    (
        "failed to build checked out plugin id: {0}",
        "構建已檢出外掛id失敗：{0}",
    ),
    (
        "failed to load plugin share local path mapping: {0}",
        "載入外掛分享本地路徑對映失敗：{0}",
    ),
    (
        "cannot check out remote plugin `{0}` because the local plugin path already exists",
        "無法檢出遠端外掛 `{0}`，因為本地外掛路徑已存在",
    ),
    (
        "{0}; additionally failed to clean up checked out plugin path `{2}`: {1}",
        "{0}；另外清理已檢出的外掛路徑 `{2}` 失敗：{1}",
    ),
    (
        "personal marketplace file must contain a JSON object",
        "個人市場原始檔必須是JSON物件",
    ),
    ("marketplace name must be a string", "市場源名稱必須是字串"),
    ("marketplace name is invalid: {0}", "市場源名稱無效：{0}"),
    (
        "marketplace plugins must be an array",
        "市場源plugins必須是陣列",
    ),
    (
        "marketplace already contains plugin `{0}` with a different source path",
        "市場源已包含外掛 `{0}`，但源路徑不同",
    ),
    (
        "failed to update personal plugin marketplace: {0}",
        "更新個人外掛市場失敗：{0}",
    ),
    (
        "failed to parse personal marketplace file: {0}",
        "解析個人市場原始檔失敗：{0}",
    ),
    (
        "failed to read personal plugin marketplace: {0}",
        "讀取個人外掛市場失敗：{0}",
    ),
    (
        "local plugin path must be inside the home directory to be listed in the personal marketplace",
        "本地外掛路徑必須位於主目錄內，才能列出到個人市場源",
    ),
    (
        "local plugin path contains non-UTF-8 segments",
        "本地外掛路徑包含非UTF-8片段",
    ),
    (
        "local plugin path cannot be represented as a personal marketplace path",
        "本地外掛路徑無法表示為個人市場源路徑",
    ),
    (
        "local plugin path must not be the home directory",
        "本地外掛路徑不能是主目錄",
    ),
    ("path {0} has no parent directory", "路徑{0}沒有父目錄"),
    (
        "marketplace source `{1}` is not allowed by requirements from {0}",
        "市場源 `{1}` 不被來自{0}的要求允許",
    ),
    (
        "marketplace `{0}` is reserved and cannot be loaded from this source",
        "市場源 `{0}` 是保留名，不能從該來源載入",
    ),
    (
        "marketplace `{0}` must be added to config before plugins can be installed while marketplace source restrictions are enabled",
        "在市場源限制啟用時，必須先把市場源 `{0}` 加入配置才能安裝外掛",
    ),
    (
        "configured marketplace `{0}` does not have a usable root",
        "已配置的市場源 `{0}` 沒有可用的根目錄",
    ),
    (
        "marketplace path `{1}` does not match configured marketplace `{0}`",
        "市場源路徑 `{1}` 與已配置的市場源 `{0}` 不匹配",
    ),
    (
        "configured Git marketplace source is not a Git URL",
        "已配置的Git市場源不是Git URL",
    ),
    (
        "marketplace `{0}` is reserved and cannot be added from this source",
        "市場源 `{0}` 是保留名，不能從該來源新增",
    ),
    (
        "invalid marketplace allowed source `{0}` in {1}: {2}",
        "無效的市場源允許項 `{0}`（來自{1}）：{2}",
    ),
    ("missing source", "缺少source"),
    ("missing url", "缺少url"),
    ("ref must not be empty", "ref不能為空"),
    ("expected a Git URL", "需要Git URL"),
    ("missing host_pattern", "缺少host_pattern"),
    ("missing path", "缺少path"),
    ("local path must be absolute", "本地路徑必須是絕對路徑"),
    (
        "invalid config for marketplace `{0}`: {1}",
        "市場源 `{0}` 的配置無效：{1}",
    ),
    (
        "configured marketplace `{0}` is missing source_type",
        "已配置的市場源 `{0}` 缺少source_type",
    ),
    (
        "configured marketplace `{0}` is missing source",
        "已配置的市場源 `{0}` 缺少source",
    ),
    (
        "invalid source for marketplace `{0}`: {1}",
        "市場源 `{0}` 的來源無效：{1}",
    ),
    (
        "configured marketplace `{0}` source does not match source_type `git`",
        "已配置的市場源 `{0}` 的來源與source_type `git` 不匹配",
    ),
    (
        "marketplace manifest name `{0}` does not match managed marketplace `{1}`",
        "市場清單名稱 `{0}` 與受管市場源 `{1}` 不匹配",
    ),
    ("expected a plugin directory", "需要一個外掛目錄"),
    (
        "missing .codex-plugin/plugin.json or valid Agent Plugin manifest",
        "缺少.codex-plugin/plugin.json或有效的Agent Plugin清單",
    ),
    (
        "failed to compute plugin archive path for `{1}`: {0}",
        "計算外掛歸檔路徑失敗（`{1}`）：{0}",
    ),
    (
        "unsupported plugin archive entry type: {0}",
        "不支援的外掛歸檔條目型別：{0}",
    ),
    (
        "failed to create plugin bundle extraction directory",
        "建立外掛包解壓目錄失敗",
    ),
    ("failed to read plugin bundle tar", "讀取外掛包tar失敗"),
    (
        "failed to read plugin bundle tar entry",
        "讀取外掛包tar條目失敗",
    ),
    (
        "failed to read plugin bundle tar entry path",
        "讀取外掛包tar條目路徑失敗",
    ),
    (
        "failed to create plugin bundle directory",
        "建立外掛包目錄失敗",
    ),
    (
        "plugin bundle output path has no parent: {0}",
        "外掛包輸出路徑沒有父目錄：{0}",
    ),
    ("failed to unpack plugin bundle entry", "解壓外掛包條目失敗"),
    (
        "plugin bundle tar entry `{0}` is a link",
        "外掛包tar條目 `{0}` 是連結",
    ),
    (
        "plugin bundle tar entry `{0}` escapes extraction root",
        "外掛包tar條目 `{0}` 逃逸出解壓根目錄",
    ),
    (
        "plugin bundle tar entry has an empty path",
        "外掛包tar條目路徑為空",
    ),
    (
        "plugin archive would be {0} bytes, exceeding maximum size of {1} bytes",
        "外掛歸檔將為{0}位元組，超過最大大小{1}位元組",
    ),
    (
        "failed to archive plugin bundle: {0}",
        "歸檔外掛包失敗：{0}",
    ),
    (
        "plugin bundle extracted size would be {0} bytes, exceeding maximum total size of {1} bytes",
        "外掛包解壓後為{0}位元組，超過最大總大小{1}位元組",
    ),
    (
        "archive would be {0} bytes, exceeding maximum size of {1} bytes",
        "歸檔將為{0}位元組，超過最大大小{1}位元組",
    ),
    (
        "plugin bundle tar entry `{0}` has unsupported type {1}",
        "外掛包tar條目 `{0}` 型別不受支援：{1}",
    ),
    (
        "local curated marketplace sha is not available",
        "本地精選市場源sha不可用",
    ),
    (
        "installed plugin cache entry is missing for {0}",
        "已安裝外掛的快取條目缺失：{0}",
    ),
    (
        "failed to materialize plugin source: {0}",
        "物化外掛源失敗：{0}",
    ),
    (
        "path does not exist or is not a directory",
        "路徑不存在或不是目錄",
    ),
    (
        "marketplace `{0}` is not configured as a Git marketplace",
        "市場源 `{0}` 未配置為Git市場源",
    ),
    ("an npm plugin", "一個npm外掛"),
    ("a cross-repo plugin", "一個跨倉庫外掛"),
    (
        "This is {0}. Install it to view more detailed information. The source of the plugin is {1}.",
        "這是{0}。安裝後可檢視更詳細資訊。外掛來源為{1}。",
    ),
    (
        "failed to join plugin install task: {0}",
        "加入外掛安裝任務失敗：{0}",
    ),
    (
        "failed to join plugin uninstall task: {0}",
        "加入外掛解除安裝任務失敗：{0}",
    ),
    (
        "failed to extract npm plugin package: {0}",
        "解壓npm外掛包失敗：{0}",
    ),
    (
        "npm pack completed without creating plugin package directory {0}",
        "npm pack完成但未建立外掛包目錄{0}",
    ),
    ("failed to run npm pack: {0}", "執行npm pack失敗：{0}"),
    (
        "npm pack failed with status {0}\nstdout:\n{1}\nstderr:\n{2}",
        "npm pack失敗，狀態碼{0}\\\nstdout:\\\n{1}\\\nstderr:\\\n{2}",
    ),
    (
        "failed to read npm pack destination: {0}",
        "讀取npm pack目標目錄失敗：{0}",
    ),
    (
        "npm pack completed with {0} package archives; expected exactly one",
        "npm pack產生了{0}個包歸檔；期望恰好一個",
    ),
    (
        "failed to inspect npm package archive: {0}",
        "檢查npm包歸檔失敗：{0}",
    ),
    (
        "npm package archive is {0} bytes, exceeding maximum size of {1} bytes",
        "npm包歸檔為{0}位元組，超過最大大小{1}位元組",
    ),
    (
        "failed to read npm package archive: {0}",
        "讀取npm包歸檔失敗：{0}",
    ),
    (
        "failed to read npm plugin package metadata {1}: {0}",
        "讀取npm外掛包後設資料{1}失敗：{0}",
    ),
    (
        "failed to parse npm plugin package metadata {1}: {0}",
        "解析npm外掛包後設資料{1}失敗：{0}",
    ),
    (
        "npm plugin package name '{1}' does not match requested package '{0}'",
        "npm外掛包名 '{1}' 與請求的包 '{0}' 不匹配",
    ),
    (
        "chatgpt authentication required for remote plugin mutation",
        "遠端外掛變更需要ChatGPT身份驗證",
    ),
    (
        "chatgpt authentication required for remote plugin mutation; api key auth is not supported",
        "遠端外掛變更需要ChatGPT身份驗證；不支援API key方式",
    ),
    (
        "failed to read auth token for remote plugin mutation: {0}",
        "讀取遠端外掛變更的身份驗證令牌失敗：{0}",
    ),
    (
        "invalid chatgpt base url for remote plugin mutation: {0}",
        "遠端外掛變更的chatgpt基礎URL無效：{0}",
    ),
    (
        "chatgpt base url cannot be used for plugin mutation",
        "chatgpt基礎URL不能用於外掛變更",
    ),
    (
        "failed to send remote plugin mutation request to {0}: {1}",
        "向{0}傳送遠端外掛變更請求失敗：{1}",
    ),
    (
        "remote plugin mutation failed with status {0} from {1}: {2}",
        "遠端外掛變更失敗，狀態碼{0}（來自{1}）：{2}",
    ),
    (
        "failed to parse remote plugin mutation response from {0}: {1}",
        "解析來自{0}的遠端外掛變更響應失敗：{1}",
    ),
    (
        "invalid chatgpt base url for remote featured plugin request: {0}",
        "遠端特色外掛的chatgpt基礎URL無效：{0}",
    ),
    (
        "failed to send remote featured plugin request to {0}: {1}",
        "向{0}傳送遠端特色外掛請求失敗：{1}",
    ),
    (
        "remote featured plugin request to {0} failed with status {1}: {2}",
        "向{0}傳送的遠端特色外掛請求失敗，狀態碼{1}：{2}",
    ),
    (
        "failed to parse remote featured plugin response from {0}: {1}",
        "解析來自{0}的遠端特色外掛響應失敗：{1}",
    ),
    ("failed to add marketplace: {0}", "新增市場源失敗：{0}"),
    (
        "--sparse is only supported for git marketplace sources",
        "只對git市場源支援--sparse",
    ),
    (
        "failed to create marketplace install directory {1}: {0}",
        "建立市場源安裝目錄{1}失敗：{0}",
    ),
    (
        "failed to resolve installed marketplace root: {0}",
        "解析已安裝的市場源根目錄失敗：{0}",
    ),
    (
        "marketplace '{0}' is already added from a different source; remove it before adding this source",
        "市場源 '{0}' 已從不同來源新增；請先移除它再新增此來源",
    ),
    (
        "failed to create marketplace staging directory {1}: {0}",
        "建立市場源暫存目錄{1}失敗：{0}",
    ),
    (
        "failed to create temporary marketplace directory in {1}: {0}",
        "在{1}中建立臨時市場源目錄失敗：{0}",
    ),
    (
        "failed to install marketplace at {1}: {0}",
        "安裝市場源到{1}失敗：{0}",
    ),
    (
        "{0}; additionally failed to roll back installed marketplace at {2}: {1}",
        "{0}；另外回滾已安裝的市場源（{2}）失敗：{1}",
    ),
    (
        "failed to serialize activated marketplace metadata: {0}",
        "序列化已啟用的市場源後設資料失敗：{0}",
    ),
    (
        "failed to write activated marketplace metadata: {0}",
        "寫入已啟用的市場源後設資料失敗：{0}",
    ),
    (
        "failed to determine marketplace install parent for {0}",
        "無法確定市場源的安裝父目錄：{0}",
    ),
    (
        "failed to create marketplace install parent {1}: {0}",
        "建立市場源安裝父目錄{1}失敗：{0}",
    ),
    (
        "failed to create marketplace backup directory in {1}: {0}",
        "在{1}中建立市場源備份目錄失敗：{0}",
    ),
    (
        "failed to move previous marketplace root out of the way at {1}: {0}",
        "移開先前的市場源根目錄失敗（{1}）：{0}",
    ),
    (
        "failed to activate upgraded marketplace at {1}: {0}",
        "啟用升級後的市場源失敗（{1}）：{0}",
    ),
    (
        "failed to activate upgraded marketplace at {2}: {0}; failed to restore previous marketplace root (left at {3}): {1}",
        "啟用升級後的市場源（{2}）失敗：{0}；恢復先前的市場源根目錄失敗（留在 {3}）：{1}",
    ),
    (
        "{0}; failed to restore previous marketplace root at {2} (left at {3}): {1}",
        "{0}；恢復先前的市場源根目錄（{2}）失敗（留在 {3}）：{1}",
    ),
    (
        "{0}; failed to remove newly activated marketplace root at {2}: {1}",
        "{0}；刪除新啟用的市場源根目錄（{2}）失敗：{1}",
    ),
    (
        "installed marketplace `{0}` changed while auto-upgrade was in flight",
        "已安裝的市場源 `{0}` 在自動升級過程中發生變化",
    ),
    (
        "remote plugin install is not enabled for marketplace {0}",
        "市場源{0}未啟用遠端外掛安裝",
    ),
    (
        "read remote plugin details before install",
        "安裝前讀取遠端外掛詳情",
    ),
    (
        "invalid resolved plugin id `{1}`: {0}",
        "解析出的外掛id `{1}` 無效：{0}",
    ),
    (
        "failed to coordinate remote plugin install",
        "協調遠端外掛安裝失敗",
    ),
    ("install remote plugin", "安裝遠端外掛"),
    (
        "remote plugin uninstall is not enabled",
        "未啟用遠端外掛解除安裝",
    ),
    (
        "resolve remote plugin before uninstall",
        "解除安裝前解析遠端外掛",
    ),
    (
        "failed to coordinate remote plugin uninstall",
        "協調遠端外掛解除安裝失敗",
    ),
    ("uninstall remote plugin", "解除安裝遠端外掛"),
    ("install remote plugin bundle: {0}", "安裝遠端外掛包：{0}"),
    (
        "remote plugin {0} is disabled by admin",
        "遠端外掛 {0} 已被管理員禁用",
    ),
    (
        "remote plugin {0} is not available for install",
        "遠端外掛 {0} 不可安裝",
    ),
    (
        "invalid configured Git marketplace: {0}",
        "已配置的Git市場源無效：{0}",
    ),
    (
        "configured Git marketplace is missing source",
        "已配置的Git市場源缺少source",
    ),
    (
        "validated Git marketplace source resolved to a local path",
        "已驗證的Git市場源解析為本地路徑",
    ),
    (
        "failed to create marketplace upgrade staging directory {1}: {0}",
        "建立市場源升級暫存目錄{1}失敗：{0}",
    ),
    (
        "failed to create temporary marketplace upgrade directory in {1}: {0}",
        "在{1}中建立臨時市場源升級目錄失敗：{0}",
    ),
    (
        "failed to validate upgraded marketplace root: {0}",
        "校驗升級後的市場源根目錄失敗：{0}",
    ),
    (
        "upgraded marketplace name `{0}` does not match configured marketplace `{1}`",
        "升級後的市場源名稱 `{0}` 與已配置的市場源 `{1}` 不匹配",
    ),
    (
        "upgraded marketplace path is not absolute: {0}",
        "升級後的市場源路徑不是絕對路徑：{0}",
    ),
    (
        "failed to reload config while checking marketplace upgrade: {0}",
        "檢查市場源升級時重新載入配置失敗：{0}",
    ),
    (
        "configured marketplace `{0}` changed while auto-upgrade was in flight",
        "已配置的市場源 `{0}` 在自動升級過程中發生變化",
    ),
    (
        "configured marketplace `{0}` was removed or is no longer a Git marketplace",
        "已配置的市場源 `{0}` 已被移除或不再是Git市場源",
    ),
    (
        "remote installed plugin `{1}` has an invalid local cache id: {0}",
        "遠端已安裝外掛`{1}`的本地快取id無效：{0}",
    ),
    (
        "timed out waiting for another remote plugin cache mutation; retry",
        "等待另一個遠端外掛快取變更超時；請重試",
    ),
    (
        "remote plugin state changed during reconciliation; retry reconciliation",
        "協調過程中遠端外掛狀態發生變化；請重試協調",
    ),
    (
        "failed to resolve trusted Codex home: {0}",
        "解析受信任的Codex主目錄失敗：{0}",
    ),
    (
        "failed to create trusted Git repository root: {0}",
        "建立受信任的Git倉庫根目錄失敗：{0}",
    ),
    (
        "failed to resolve trusted Git repository root: {0}",
        "解析受信任的Git倉庫根目錄失敗：{0}",
    ),
    (
        "trusted Git repository root escapes Codex home",
        "受信任的Git倉庫根目錄超出了Codex主目錄範圍",
    ),
    (
        "failed to create trusted Git repository: {0}",
        "建立受信任的Git倉庫失敗：{0}",
    ),
    (
        "failed to initialize trusted Git repository: {0}",
        "初始化受信任的Git倉庫失敗：{0}",
    ),
    (
        "failed to initialize trusted Git repository HEAD: {0}",
        "初始化受信任的Git倉庫HEAD失敗：{0}",
    ),
    ("failed to remove marketplace: {0}", "移除市場源失敗：{0}"),
    (
        "marketplace `{0}` is configured in {1}; remove it from that configuration source instead",
        "市場源`{0}`配置在{1}中；請改為從該配置來源移除",
    ),
    (
        "failed to remove marketplace '{0}' from user config.toml: {1}",
        "從使用者config.toml中移除市場源'{0}'失敗：{1}",
    ),
    (
        "marketplace `{0}` does not match configured marketplace `{1}` exactly",
        "市場源`{0}`與已配置的市場源`{1}`不完全匹配",
    ),
    (
        "marketplace `{0}` is not configured or installed",
        "市場源`{0}`未配置或未安裝",
    ),
    (
        "failed to resolve installed marketplace root {1}: {0}",
        "解析已安裝市場源根目錄{1}失敗：{0}",
    ),
    (
        "failed to inspect installed marketplace root {1}: {0}",
        "檢查已安裝市場源根目錄{1}失敗：{0}",
    ),
    (
        "failed to remove installed marketplace root {1}: {0}",
        "移除已安裝市場源根目錄{1}失敗：{0}",
    ),
    (
        "marketplace name '{0}' cannot be used as an install directory",
        "市場源名稱'{0}'不能用作安裝目錄",
    ),
    (
        "failed to resolve marketplace install root {1}: {0}",
        "解析市場源安裝根目錄{0}失敗：{1}",
    ),
    (
        "marketplace destination has no parent",
        "市場源目標路徑沒有父目錄",
    ),
    (
        "failed to resolve marketplace destination parent {1}: {0}",
        "解析市場源目標父目錄{0}失敗：{1}",
    ),
    (
        "marketplace destination {0} is outside install root {1}",
        "市場源目標路徑{0}在安裝根目錄{1}之外",
    ),
    ("marketplace source must not be empty", "市場源不能為空"),
    (
        "--ref is only supported for git marketplace sources",
        "--ref僅支援git市場源",
    ),
    (
        "local marketplace source must be a directory, not a file",
        "本地市場源必須是目錄，不能是檔案",
    ),
    (
        "invalid marketplace source format; expected owner/repo, a git URL, or a local marketplace path",
        "市場源格式無效；應為owner/repo、git URL或本地市場源路徑",
    ),
    (
        "failed to read current working directory for local marketplace source: {0}",
        "讀取本地市場源的當前工作目錄失敗：{0}",
    ),
    (
        "failed to resolve local marketplace source path: {0}",
        "解析本地市場源路徑失敗：{0}",
    ),
    (
        "workspace plugin create response did not include a plugin id",
        "工作區外掛建立響應未包含外掛id",
    ),
    (
        "created workspace plugin `{0}` did not include share_principals",
        "建立的工作區外掛`{0}`未包含share_principals",
    ),
    (
        "workspace plugin share requires an account id",
        "工作區外掛分享需要account id",
    ),
    (
        "failed to add marketplace '{0}' to user config.toml: {1}",
        "向使用者config.toml新增市場源'{0}'失敗：{1}",
    ),
    (
        "failed to read user config {1}: {0}",
        "讀取使用者配置{0}失敗：{1}",
    ),
    (
        "failed to parse user config {1}: {0}",
        "解析使用者配置{0}失敗：{1}",
    ),
    ("Migrated source command `{0}`", "遷移自源命令`{0}`"),
    (
        "No command template body was found.",
        "未找到命令模板正文。",
    ),
    (
        "tool-suggest metadata cache loader closed",
        "tool-suggest後設資料快取載入器已關閉",
    ),
    (
        "recommended plugin `{0}` is missing remote plugin identity",
        "推薦外掛`{0}`缺少遠端外掛標識",
    ),
    (
        "shareTargets cannot include workspace principals; use discoverability UNLISTED for workspace link access",
        "shareTargets不能包含工作區主體；工作區連結訪問請改用discoverability UNLISTED",
    ),
    ("failed to reload config: {0}", "重新載入配置失敗：{0}"),
    ("list marketplace plugins", "列出市場源外掛"),
    (
        "failed to list marketplace plugins: {0}",
        "列出市場源外掛失敗：{0}",
    ),
    (
        "list OpenAI Curated remote plugin catalog",
        "列出OpenAI精選遠端外掛目錄",
    ),
    ("list remote plugin catalog", "列出遠端外掛目錄"),
    (
        "list installed and suggested marketplace plugins",
        "列出已安裝和建議的市場源外掛",
    ),
    (
        "failed to list installed and suggested plugins: {0}",
        "列出已安裝和建議的外掛失敗：{0}",
    ),
    ("read plugin details", "讀取外掛詳情"),
    (
        "remote plugin read is not enabled for marketplace {0}",
        "市場源{0}未啟用遠端外掛讀取",
    ),
    ("read remote plugin details", "讀取遠端外掛詳情"),
    (
        "remote plugin skill read is not enabled for marketplace {0}",
        "市場源{0}未啟用遠端外掛技能讀取",
    ),
    (
        "invalid remote plugin skill name: cannot be empty",
        "遠端外掛技能名稱無效：不能為空",
    ),
    ("read remote plugin skill details", "讀取遠端外掛技能詳情"),
    ("invalid remote plugin id", "遠端外掛id無效"),
    (
        "discoverability and shareTargets are only supported when creating a plugin share; use plugin/share/updateTargets to update share settings",
        "discoverability與shareTargets僅在建立外掛分享時支援；更新分享設定請使用plugin/share/updateTargets",
    ),
    (
        "discoverability LISTED is not supported for plugin/share/save; use UNLISTED or PRIVATE",
        "plugin/share/save不支援discoverability LISTED；請使用UNLISTED或PRIVATE",
    ),
    ("save remote plugin share", "儲存遠端外掛分享"),
    ("update remote plugin share targets", "更新遠端外掛分享目標"),
    ("list remote plugin shares", "列出遠端外掛分享"),
    ("checkout plugin share", "檢出外掛分享"),
    ("delete remote plugin share", "刪除遠端外掛分享"),
    ("install plugin", "安裝外掛"),
    (
        "failed to persist installed plugin config: {0}",
        "持久化已安裝外掛配置失敗：{0}",
    ),
    (
        "failed to enable remote plugin: {0}",
        "啟用遠端外掛失敗：{0}",
    ),
    ("failed to install plugin: {0}", "安裝外掛失敗：{0}"),
    (
        "failed to clear plugin config: {0}",
        "清除外掛配置失敗：{0}",
    ),
    (
        "failed to uninstall remote plugin: {0}",
        "解除安裝遠端外掛失敗：{0}",
    ),
    ("failed to uninstall plugin: {0}", "解除安裝外掛失敗：{0}"),
    ("failed to {0}: {1}", "{0}失敗：{1}"),
    (
        "MCP client startup timed out after {0}",
        "MCP客戶端啟動在{0}後超時",
    ),
    ("MCP startup cancelled", "MCP啟動已取消"),
    ("MCP startup failed: {0}", "MCP啟動失敗：{0}"),
    (
        "executor-owned MCP server `{0}` cannot use hosted ChatGPT authentication; configure executor-owned credentials instead",
        "executor擁有的MCP伺服器`{0}`不能使用託管的ChatGPT認證；請改用executor自有的憑據",
    ),
    (
        "non-local HTTP MCP server `{0}` did not resolve an execution environment",
        "非本地HTTP MCP伺服器`{0}`未能解析執行環境",
    ),
    (
        "Environment variable {0} for MCP server '{1}' is empty",
        "MCP伺服器'{1}'的環境變數{0}為空",
    ),
    (
        "Environment variable {0} for MCP server '{1}' is not set",
        "MCP伺服器'{1}'的環境變數{0}未設定",
    ),
    (
        "Environment variable {0} for MCP server '{1}' contains invalid Unicode",
        "MCP伺服器'{1}'的環境變數{0}包含無效Unicode",
    ),
    (
        "Invalid MCP server name '{0}': must match pattern {1}",
        "無效的MCP伺服器名'{0}'：必須匹配模式{1}",
    ),
    (
        "Reconnect {0} on ChatGPT to grant the permissions needed for this request.",
        "請在ChatGPT上重新連線{0}，以授予此請求所需的許可權",
    ),
    (
        "Reconnect {0} on ChatGPT to restore access for this request.",
        "請在ChatGPT上重新連線{0}，以恢復此請求的訪問許可權",
    ),
    (
        "Sign in to {0} on ChatGPT to use it in Codex.",
        "請在ChatGPT上登入{0}以在Codex中使用它",
    ),
    (
        "Sign in to {0} on ChatGPT to continue.",
        "請在ChatGPT上登入{0}以繼續",
    ),
    (
        "{0} is unavailable without sqlite state",
        "無sqlite狀態時{0}不可用",
    ),
    (
        "failed to stage {0} metadata: {1}",
        "暫存{0}後設資料失敗：{1}",
    ),
    (
        "{0} must match ^[a-zA-Z0-9_-]+$ to match Responses API: {1}",
        "{0}必須匹配^[a-zA-Z0-9_-]+$以符合Responses API引數要求：{1}",
    ),
    (
        "{0} must be at most {1} characters to match Responses API: {2}",
        "{0}最長不能超過{1}個字元以符合Responses API引數要求：{2}",
    ),
    (
        "dynamic tool name must not be empty",
        "動態工具名稱不能為空",
    ),
    (
        "dynamic tool name has leading/trailing whitespace: {0}",
        "動態工具名稱首尾有空白：{0}",
    ),
    (
        "dynamic tool name is reserved: {0}",
        "動態工具名稱被保留：{0}",
    ),
    (
        "duplicate dynamic tool name in namespace {0}: {1}",
        "名稱空間{0}中的動態工具名稱重複：{1}",
    ),
    ("duplicate dynamic tool name: {0}", "動態工具名稱重複：{0}"),
    ("unarchive", "取消歸檔"),
    ("truncate thread for fork", "為fork截斷執行緒"),
    (
        "thread/rollback is deprecated and will be removed soon",
        "thread/rollback已棄用，即將移除",
    ),
    ("thread not found: {0}", "未找到執行緒：{0}"),
    ("set thread name", "設定執行緒名稱"),
    ("set thread memory mode", "設定執行緒記憶體模式"),
    ("sectionId must not be empty", "sectionId不能為空"),
    ("revert", "回滾"),
    ("move thread in section", "在分割槽中移動執行緒"),
    ("invalid thread id: {0}", "無效的執行緒id：{0}"),
    ("invalid before thread id: {0}", "無效的before執行緒id：{0}"),
    ("inherit source thread name", "繼承源執行緒名稱"),
    ("failed to {0} session: {1}", "{0}會話失敗：{1}"),
    (
        "failed to set app server client info: {0}",
        "設定app server客戶端資訊失敗：{0}",
    ),
    (
        "failed to acquire thread list state permit: {0}",
        "獲取執行緒列表狀態許可失敗：{0}",
    ),
    (
        "dynamic tool namespace must not be empty",
        "動態工具名稱空間不能為空",
    ),
    (
        "dynamic tool namespace is reserved: {0}",
        "動態工具名稱空間被保留：{0}",
    ),
    (
        "dynamic tool namespace description must be at most 1024 characters",
        "動態工具名稱空間描述最長1024個字元",
    ),
    (
        "dynamic tool namespace collides with a reserved Responses API namespace: {0}",
        "動態工具名稱空間{0}與保留的Responses API名稱空間衝突",
    ),
    (
        "dynamic tool input schema is not supported for {0}: {1}",
        "動態工具{0}不支援該輸入schema：{1}",
    ),
    (
        "duplicate dynamic tool namespace: {0}",
        "動態工具名稱空間重複：{0}",
    ),
    (
        "deferred dynamic tool must include a namespace: {0}",
        "延遲載入的動態工具必須包含名稱空間：{0}",
    ),
    (
        "beforeThreadId requires a non-null sectionId",
        "beforeThreadId需要非空的sectionId",
    ),
    ("archive", "歸檔"),
    (
        "Full-history hydration is deprecated for paginated threads; use `excludeTurns: true`, then page with `thread/turns/list` and `thread/items/list`.",
        "分頁執行緒不再支援完整歷史回填；請使用`excludeTurns: true`，然後用`thread/turns/list`和`thread/items/list`翻頁。",
    ),
    (
        "Full-history hydration is deprecated for paginated threads; omit `includeTurns` or set it to `false`, then page with `thread/turns/list` and `thread/items/list`.",
        "分頁執行緒不再支援完整歷史回填；請省略`includeTurns`或設為`false`，然後用`thread/turns/list`和`thread/items/list`翻頁。",
    ),
    ("error creating thread: {0}", "建立執行緒出錯：{0}"),
    ("project not found: {0}", "未找到專案：{0}"),
    ("failed to read project: {0}", "讀取專案失敗：{0}"),
    ("projectId must not be empty", "projectId不能為空"),
    (
        "`permissions` cannot be combined with `sandbox`",
        "`permissions`不能與`sandbox`組合使用",
    ),
    ("dynamic tool namespace", "動態工具名稱空間"),
    ("dynamic tool name", "動態工具名稱"),
    (
        "invalid thread/list cwd filter `{0}`: {1}",
        "無效的thread/list cwd過濾器`{0}`：{1}",
    ),
    (
        "dynamic tool namespace has leading/trailing whitespace: {0}",
        "動態工具名稱空間首尾有空白：{0}",
    ),
    (
        "failed to restore thread settings after revert: {0}",
        "回滾後恢復執行緒設定失敗：{0}",
    ),
    (
        "thread {0} reloaded as {1} after revert",
        "執行緒{0}在回滾後重新載入為{1}",
    ),
    (
        "error reloading thread after revert: {0}",
        "回滾後重新載入執行緒出錯：{0}",
    ),
    (
        "thread {0} disappeared before revert",
        "執行緒{0}在回滾前消失",
    ),
    (
        "thread {0} listener stopped before draining shutdown events",
        "執行緒{0}監聽器在排空關閉事件前停止",
    ),
    (
        "timed out waiting for thread {0} listener to drain shutdown events",
        "等待執行緒{0}監聽器排空關閉事件超時",
    ),
    (
        "timed out shutting down thread {0} before revert",
        "回滾前關閉執行緒{0}超時",
    ),
    (
        "failed to shut down thread {0} before revert",
        "回滾前關閉執行緒{0}失敗",
    ),
    (
        "connection closed before thread {0} could be reverted",
        "連線線上程{0}可回滾前關閉",
    ),
    (
        "failed to read thread metadata: {0}",
        "讀取執行緒後設資料失敗：{0}",
    ),
    (
        "gitInfo.originUrl must be a valid Git remote",
        "gitInfo.originUrl必須是有效的Git遠端地址",
    ),
    (
        "gitInfo must include at least one field",
        "gitInfo必須至少包含一個欄位",
    ),
    (
        "thread metadata update must include at least one field",
        "執行緒後設資料更新必須至少包含一個欄位",
    ),
    (
        "failed to clear memory directories under {0}: {1}",
        "清除{0}下的記憶體目錄失敗：{1}",
    ),
    (
        "failed to clear memory rows in memories db: {0}",
        "清除memories資料庫中的記憶體行失敗：{0}",
    ),
    (
        "sqlite state db unavailable for memory reset",
        "記憶體重置需要sqlite狀態資料庫，當前不可用",
    ),
    (
        "failed to decrement out-of-band elicitation counter: {0}",
        "遞減帶外提示計數器失敗：{0}",
    ),
    (
        "failed to increment out-of-band elicitation counter: {0}",
        "遞增帶外提示計數器失敗：{0}",
    ),
    (
        "failed to list spawned descendants for thread id {0}: {1}",
        "列出執行緒{0}派生的子程序失敗：{1}",
    ),
    ("invalid session id: {0}", "無效的會話id：{0}"),
    ("thread name must not be empty", "執行緒名稱不能為空"),
    (
        "rollout path missing after reloading thread {0}",
        "重新載入執行緒{0}後缺少rollout路徑",
    ),
    (
        "rollback already in progress for this thread",
        "此執行緒已有回滾正在進行",
    ),
    (
        "parentThreadId and ancestorThreadId are mutually exclusive",
        "parentThreadId與ancestorThreadId互斥",
    ),
    (
        "paginated threads do not support thread/rollback",
        "分頁執行緒不支援thread/rollback",
    ),
    (
        "originator filtering is not supported by the local app-server",
        "本地app-server不支援發起方過濾",
    ),
    ("numTurns must be >= 1", "numTurns必須>= 1"),
    ("local environment is not configured", "未配置本地環境"),
    (
        "invalid background terminal process id: {0}",
        "無效的後臺終端程序id：{0}",
    ),
    (
        "invalid Guardian denial event: {0}",
        "無效的Guardian拒絕事件：{0}",
    ),
    (
        "failed to start shell command: {0}",
        "啟動shell命令失敗：{0}",
    ),
    ("failed to start rollback: {0}", "啟動回滾失敗：{0}"),
    ("failed to start compaction: {0}", "啟動壓縮失敗：{0}"),
    (
        "failed to clean background terminals: {0}",
        "清理後臺終端失敗：{0}",
    ),
    (
        "failed to approve Guardian denial: {0}",
        "批准Guardian拒絕事件失敗：{0}",
    ),
    (
        "dynamic tool namespace must contain at least one tool: {0}",
        "動態工具名稱空間{0}必須至少包含一個工具",
    ),
    ("command must not be empty", "命令不能為空"),
    (
        "thread {0} is closing; retry thread/resume after the thread is closed",
        "執行緒{0}正在關閉；請線上程關閉後重試thread/resume",
    ),
    (
        "thread store did not return history for thread {0}",
        "執行緒儲存未返回執行緒{0}的歷史",
    ),
    (
        "no rollout found for thread id {0}",
        "未找到執行緒{0}的rollout",
    ),
    (
        "no rollout found for conversation id {0}",
        "未找到會話{0}的rollout",
    ),
    ("invalid parent thread id: {0}", "無效的父執行緒id：{0}"),
    ("invalid cursor: {0}", "無效的遊標：{0}"),
    ("invalid ancestor thread id: {0}", "無效的祖先執行緒id：{0}"),
    (
        "failed to search thread occurrences: {0}",
        "搜尋執行緒出現失敗：{0}",
    ),
    ("failed to read thread: {0}", "讀取執行緒失敗：{0}"),
    (
        "failed to load full turn items for {0}: thread store returned a repeated cursor",
        "載入執行緒{0}的完整turn項失敗：執行緒儲存返回了重複的遊標",
    ),
    (
        "failed to load full thread turns for {0}: thread store returned a repeated cursor",
        "載入執行緒{0}的完整turn失敗：執行緒儲存返回了重複的遊標",
    ),
    ("failed to list thread items: {0}", "列出執行緒項失敗：{0}"),
    (
        "failed to list thread history: {0}",
        "列出執行緒歷史失敗：{0}",
    ),
    (
        "ephemeral threads do not support thread/turns/list",
        "臨時執行緒不支援thread/turns/list",
    ),
    (
        "cannot resume paginated thread {0} with stale path: requested {1}, current {2}; omit path and resume by thread id",
        "無法用過期路徑恢復分頁執行緒{0}：請求{1}，當前{2}；請省略路徑並按執行緒id恢復",
    ),
    (
        "session {0} is archived. Run `codex unarchive {0}` to unarchive it first.",
        "會話{0}已歸檔。請先執行`codex unarchive {0}`取消歸檔。",
    ),
    (
        "rollout path missing for thread {0}",
        "缺少執行緒{0}的rollout路徑",
    ),
    (
        "paginated_threads is not supported yet",
        "暫不支援paginated_threads",
    ),
    (
        "invalid `approval_policy` config override: {0}",
        "無效的`approval_policy`配置覆蓋：{0}",
    ),
    ("history must not be empty", "歷史不能為空"),
    (
        "failed to read thread from store: {0}",
        "從儲存讀取執行緒失敗：{0}",
    ),
    (
        "failed to prepare paginated fork: {0}",
        "準備分頁fork失敗：{0}",
    ),
    (
        "failed to enqueue running thread resume for thread {0}: thread listener command channel is closed",
        "為執行緒{0}排隊執行中恢復失敗：執行緒監聽器命令通道已關閉",
    ),
    (
        "failed to enqueue running thread resume for thread {0}: thread listener is not running",
        "為執行緒{0}排隊執行中恢復失敗：執行緒監聽器未執行",
    ),
    (
        "failed to build resume response for thread {0}: initial history missing",
        "為執行緒{0}構建恢復響應失敗：缺少初始歷史",
    ),
    ("error resuming thread: {0}", "恢復執行緒出錯：{0}"),
    (
        "ephemeral paginated thread/fork requires `excludeTurns: true`",
        "臨時分頁執行緒/fork需要`excludeTurns: true`",
    ),
    (
        "cannot resume running thread {0} with stale path: requested `{1}`, active `{2}`",
        "無法用過期路徑恢復正在執行的執行緒{0}：請求的`{1}`，當前的`{2}`",
    ),
    (
        "cannot resume thread {0} with history while it is already running",
        "執行緒{0}已在執行，無法帶歷史恢復",
    ),
    (
        "cannot resume an unloaded multi-agent v2 sub-agent through its parent; resume the parent first, or use thread/read to inspect it",
        "無法透過父執行緒恢復未載入的multi-agent v2子代理；請先恢復父執行緒，或用thread/read檢查它",
    ),
    (
        "`deferGoalContinuation` cannot be combined with `ephemeral`",
        "`deferGoalContinuation`不能與`ephemeral`組合使用",
    ),
    (
        "`beforeTurnId` cannot be combined with `lastTurnId`",
        "`beforeTurnId`不能與`lastTurnId`組合使用",
    ),
    (
        "failed to load conversation summary from {0}: {1}",
        "從{0}載入會話摘要失敗：{1}",
    ),
    (
        "failed to deserialize stored thread item {0}: {1}",
        "反序列化已儲存的執行緒項{0}失敗：{1}",
    ),
    (
        "thread {0} is not materialized yet; includeTurns is unavailable before first user message",
        "執行緒{0}尚未物化；在第一條使用者訊息之前includeTurns不可用",
    ),
    (
        "thread {0} is not materialized yet; thread/turns/list is unavailable before first user message",
        "執行緒{0}尚未物化；在第一條使用者訊息之前thread/turns/list不可用",
    ),
    (
        "rollout path queries are only supported with the local thread store",
        "rollout路徑查詢僅支援本地執行緒儲存",
    ),
    (
        "invalid cursor: anchor turn is no longer present",
        "無效的遊標：錨點turn已不存在",
    ),
    ("failed to serialize cursor: {0}", "序列化遊標失敗：{0}"),
    ("failed to load thread {0}: {1}", "載入執行緒{0}失敗：{1}"),
    (
        "failed to load thread history for thread {0}: {1}",
        "載入執行緒{0}的歷史失敗：{1}",
    ),
    (
        "failed to load conversation summary for {0}: {1}",
        "載入會話{0}摘要失敗：{1}",
    ),
    ("failed to list threads: {0}", "列出執行緒失敗：{0}"),
    (
        "failed to inherit source thread goal: {0}",
        "繼承源執行緒目標失敗：{0}",
    ),
    (
        "failed to flush source thread goal: {0}",
        "重新整理源執行緒目標失敗：{0}",
    ),
    ("error forking thread: {0}", "建立fork執行緒出錯：{0}"),
    (
        "user layer not found in updated layers",
        "在更新的配置層中未找到使用者層",
    ),
    ("invalid configuration", "無效的配置"),
    ("failed to serialize configuration", "序列化配置失敗"),
    (
        "failed to resolve user config path",
        "未能解析使用者配置路徑",
    ),
    (
        "failed to resolve config cwd to an absolute path",
        "未能將配置cwd解析為絕對路徑",
    ),
    ("failed to read configuration layers", "讀取配置層失敗"),
    ("failed to persist config.toml", "持久化config.toml失敗"),
    ("failed to deserialize configuration", "反序列化配置失敗"),
    ("failed to build config edits", "構建配置編輯失敗"),
    (
        "`{0}` is managed by requirements and cannot be changed",
        "`{0}`由requirements管理，不能更改",
    ),
    (
        "`profiles` contains legacy config profile tables and can no longer be written; use `--profile <name>` with `<name>.config.toml` instead",
        "`profiles`包含舊的配置profile表，不能再寫入；請改用`--profile <name>`配合`<name>.config.toml`",
    ),
    (
        "`profile` is a legacy config selector and can no longer be written; use `--profile <name>` with `<name>.config.toml` instead",
        "`profile`是舊的配置選擇器，不能再寫入；請改用`--profile <name>`配合`<name>.config.toml`",
    ),
    (
        "Only writes to the user config are allowed",
        "只允許寫入使用者配置",
    ),
    ("Invalid configuration: {0}", "無效的配置：{0}"),
    (
        "Configuration was modified since last read. Fetch latest version and retry.",
        "自上次讀取後配置已被修改。請獲取最新版本後重試。",
    ),
    (
        "Overridden by legacy managed configuration from MDM",
        "被MDM的舊式託管配置覆蓋",
    ),
    (
        "{0} permission selection missing thread snapshot",
        "{0}許可權選擇缺少執行緒快照",
    ),
    (
        "turn settings operation ended before replying",
        "turn設定操作在回覆前結束",
    ),
    ("sha must not be empty", "sha不能為空"),
    (
        "paginated threads do not support detached review",
        "分頁執行緒不支援分離式review",
    ),
    ("no active turn to interrupt", "沒有可中斷的活躍turn"),
    (
        "items[{0}] is not a valid response item: {1}",
        "items[{0}]不是有效的響應項：{1}",
    ),
    ("instructions must not be empty", "instructions不能為空"),
    ("input must not be empty", "input不能為空"),
    (
        "failed to update thread settings: {0}",
        "更新執行緒設定失敗：{0}",
    ),
    (
        "failed to submit turn settings: {0}",
        "提交turn設定失敗：{0}",
    ),
    (
        "failed to stop realtime conversation: {0}",
        "停止實時會話失敗：{0}",
    ),
    ("failed to steer turn: {0}", "轉向turn失敗：{0}"),
    ("failed to start review: {0}", "啟動review失敗：{0}"),
    (
        "failed to start realtime conversation: {0}",
        "啟動實時會話失敗：{0}",
    ),
    (
        "failed to start detached review: {0}",
        "啟動分離式review失敗：{0}",
    ),
    ("failed to interrupt {0}: {1}", "中斷{0}失敗：{1}"),
    (
        "failed to inject response items: {0}",
        "注入響應項失敗：{0}",
    ),
    (
        "failed to append realtime conversation text: {0}",
        "追加實時會話文字失敗：{0}",
    ),
    (
        "failed to append realtime conversation speech: {0}",
        "追加實時會話語音失敗：{0}",
    ),
    (
        "failed to append realtime conversation audio: {0}",
        "追加實時會話音訊失敗：{0}",
    ),
    ("expectedTurnId must not be empty", "expectedTurnId不能為空"),
    (
        "expected active turn id {0} but found {1}",
        "期望活躍turn id為{0}，實際為{1}",
    ),
    (
        "expected active turn id `{0}` but found `{1}`",
        "期望活躍turn id為`{0}`，實際為`{1}`",
    ),
    (
        "existingCall transport does not support {0}",
        "existingCall傳輸不支援{0}",
    ),
    ("cannot steer a review turn", "無法轉向review turn"),
    ("cannot steer a compact turn", "無法轉向compact turn"),
    ("branch must not be empty", "branch不能為空"),
    (
        "active turn uses a different output schema",
        "當前活躍turn使用了不同的輸出schema",
    ),
    (
        "`toolOutput` cannot be combined with nonempty `input`",
        "`toolOutput`不能與非空`input`組合使用",
    ),
    (
        "`toolOutput.name` must not be empty",
        "`toolOutput.name`不能為空",
    ),
    (
        "`permissions` cannot be combined with `sandboxPolicy`",
        "`permissions`不能與`sandboxPolicy`組合使用",
    ),
    (
        "Use thread/start followed by review/start with delivery \"inline\" for a separate review thread, or thread/fork followed by turn/start with your own review instructions.",
        "改用thread/start後接delivery為inline的review/start建立獨立review執行緒，或thread/fork後接自定義review指令的turn/start。",
    ),
    (
        "Input exceeds the maximum length of {0} characters.",
        "輸入超出最大長度{0}個字元。",
    ),
    ("workspace messages fetch timed out", "獲取工作區訊息超時"),
    (
        "token usage profile fetch timed out",
        "獲取token用量檔案超時",
    ),
    ("thread usage fetch timed out", "獲取執行緒用量超時"),
    ("logout failed: {0}", "登出失敗：{0}"),
    ("invalid login id: {0}", "無效的登入id：{0}"),
    (
        "invalid Codex open app URL: {0}",
        "無效的Codex開啟應用URL：{0}",
    ),
    (
        "failed to start login server: {0}",
        "啟動登入伺服器失敗：{0}",
    ),
    ("failed to set external auth: {0}", "設定外部認證失敗：{0}"),
    ("failed to save api key: {0}", "儲存API金鑰失敗：{0}"),
    (
        "failed to save Amazon Bedrock auth: {0}",
        "儲存Amazon Bedrock認證失敗：{0}",
    ),
    ("failed to request device code: {0}", "請求裝置碼失敗：{0}"),
    (
        "failed to parse workspace message timestamp `{0}`: {1}",
        "解析工作區訊息時間戳`{0}`失敗：{1}",
    ),
    (
        "failed to notify workspace owner: {0}",
        "通知工作區所有者失敗：{0}",
    ),
    (
        "failed to fetch workspace messages: {0}",
        "獲取工作區訊息失敗：{0}",
    ),
    (
        "failed to fetch token usage profile: {0}",
        "獲取token用量檔案失敗：{0}",
    ),
    (
        "failed to fetch thread usage: {0}",
        "獲取執行緒用量失敗：{0}",
    ),
    (
        "failed to fetch codex rate limits: {0}",
        "獲取codex速率限制失敗：{0}",
    ),
    (
        "failed to fetch codex rate limits: no snapshots returned",
        "獲取codex速率限制失敗：未返回快照",
    ),
    (
        "codex account authentication required to read workspace messages",
        "讀取工作區訊息需要codex賬號認證",
    ),
    (
        "codex account authentication required to read token usage",
        "讀取token用量需要codex賬號認證",
    ),
    (
        "codex account authentication required to read rate limits",
        "讀取速率限制需要codex賬號認證",
    ),
    (
        "codex account authentication required to notify workspace owner",
        "通知工作區所有者需要codex賬號認證",
    ),
    (
        "chatgpt authentication required to read workspace messages",
        "讀取工作區訊息需要chatgpt認證",
    ),
    (
        "chatgpt authentication required to read token usage",
        "讀取token用量需要chatgpt認證",
    ),
    (
        "chatgpt authentication required to read rate limits",
        "讀取速率限制需要chatgpt認證",
    ),
    (
        "chatgpt authentication required to notify workspace owner",
        "通知工作區所有者需要chatgpt認證",
    ),
    ("Login was not completed", "登入未完成"),
    ("Login timed out", "登入超時"),
    ("Login server error: {0}", "登入伺服器錯誤：{0}"),
    (
        "External auth must use one of workspace(s) {0}, but received {1}.",
        "外部認證必須使用工作區{0}之一，但收到{1}。",
    ),
    (
        "External auth is active. Use account/login/start (chatgptAuthTokens) to update it or account/logout to clear it.",
        "外部認證處於啟用狀態。請用account/login/start(chatgptAuthTokens)更新，或用account/logout清除。",
    ),
    (
        "External ChatGPT auth is disabled. Use API key login instead.",
        "外部ChatGPT認證已禁用。請改用API金鑰登入。",
    ),
    (
        "Configured external authentication is owned by the app-server host and cannot be changed through account RPCs.",
        "已配置的外部認證由app-server宿主擁有，不能透過account RPC更改。",
    ),
    (
        "Amazon Bedrock login is disabled. Use ChatGPT login instead.",
        "Amazon Bedrock登入已禁用。請改用ChatGPT登入。",
    ),
    (
        "Amazon Bedrock does not support region `{0}`",
        "Amazon Bedrock不支援區域`{0}`",
    ),
    (
        "Amazon Bedrock API key must not be empty.",
        "Amazon Bedrock API金鑰不能為空。",
    ),
    (
        "AWS access key ID and secret access key must not be empty.",
        "AWS訪問金鑰ID和私有訪問金鑰不能為空。",
    ),
    (
        "remote control is disabled by managed requirements",
        "遠端控制被託管要求禁用",
    ),
    (
        "remote code-mode host requires the code_mode_host feature to be enabled",
        "遠端code-mode宿主需要啟用code_mode_host特性",
    ),
    (
        "no transport configured; use --listen or enable remote control",
        "未配置傳輸；請使用--listen或啟用遠端控制",
    ),
    (
        "no transport configured; remote control disabled by managed requirements",
        "未配置傳輸；遠端控制被託管要求禁用",
    ),
    (
        "no transport configured; remote control disabled because sqlite state db is unavailable",
        "未配置傳輸；sqlite狀態資料庫不可用，遠端控制已禁用",
    ),
    (
        "invalid test user config path: {0}",
        "無效的測試使用者配置路徑：{0}",
    ),
    ("error parsing -c overrides: {0}", "解析-c覆蓋配置出錯：{0}"),
    ("error loading otel config: {0}", "載入otel配置出錯：{0}"),
    (
        "error loading default config after config error: {0}",
        "配置出錯後載入預設配置出錯：{0}",
    ),
    (
        "Invalid configuration; using defaults.",
        "無效的配置；正在使用預設值。",
    ),
    (
        "Error parsing rules; custom rules not applied.",
        "解析規則出錯；未應用自定義規則。",
    ),
    (
        "failed to initialize sqlite state runtime under {1}: {0}",
        "在{1}下初始化sqlite狀態執行時失敗：{0}",
    ),
    (
        "failed to initialize sqlite state runtime after moving damaged database file into a backup folder: {0}",
        "將受損資料庫檔案移入備份目錄後初始化sqlite狀態執行時失敗：{0}",
    ),
    (
        "Codex local database at {0} appears damaged. Moving it into a backup folder so the app server can rebuild it from saved data.",
        "{0}處的Codex本地資料庫已受損。正在將其移入備份目錄，以便app server從已存資料重建。",
    ),
    (
        "failed to move damaged sqlite state database files into a backup folder: {0}; original error: {1}",
        "將受損sqlite狀態資料庫檔案移入備份目錄失敗：{0}；原始錯誤：{1}",
    ),
    (
        "Moved damaged Codex local database file {0} to {1}",
        "已將受損的Codex本地資料庫檔案{0}移至{1}",
    ),
    (
        "Database path: {0}\nBackup folder: {1}",
        "資料庫路徑：{0}\n備份目錄：{1}",
    ),
    (
        "Project-local config, hooks, and exec policies are disabled in the following folders until the project is trusted, but skills still load.",
        "在專案被信任之前，以下目錄中的專案級配置、鉤子和執行策略已禁用，但技能仍會載入。",
    ),
    ("sortDirection requires sortKey", "sortDirection需要sortKey"),
    ("project name must not be empty", "專案名稱不能為空"),
    ("idempotencyKey must not be empty", "idempotencyKey不能為空"),
    (
        "idempotencyKey must be at most 512 bytes",
        "idempotencyKey最長512位元組",
    ),
    ("invalid project root: {0}", "無效的專案根目錄：{0}"),
    ("duplicate thread id: {0}", "執行緒id {0}重複"),
    (
        "stored project root is not absolute: {0}",
        "已存的專案根目錄不是絕對路徑：{0}",
    ),
    ("duplicate project root: {0}", "重複的專案根目錄：{0}"),
    (
        "duplicate resolved project root: {0}",
        "解析後的專案根目錄重複：{0}",
    ),
    (
        "direct app-server input is not allowed for unloaded spawned sub-agents",
        "不允許對未載入的派生子代理直接輸入app-server訊息",
    ),
    (
        "invalid queue pagination cursor: {0}",
        "無效的佇列分頁遊標：{0}",
    ),
    ("queued submission not found: {0}", "未找到排隊提交項：{0}"),
    (
        "resume the thread before starting a queued message",
        "啟動排隊訊息前請先恢復執行緒",
    ),
    (
        "thread already has an active or pending turn",
        "執行緒已有活躍或待處理的turn",
    ),
    (
        "Core declined to start queued user message: {0}",
        "Core拒絕啟動排隊使用者訊息：{0}",
    ),
    ("user message queue is unavailable", "使用者訊息佇列不可用"),
    (
        "ephemeral thread does not support queued submissions: {0}",
        "臨時執行緒不支援排隊提交：{0}",
    ),
    (
        "queued submission operation failed: {0}",
        "排隊提交操作失敗：{0}",
    ),
    (
        "queued submission does not contain user input",
        "排隊提交不包含使用者輸入",
    ),
    (
        "queued submission is missing its client message id",
        "排隊提交缺少客戶端訊息id",
    ),
    (
        "cursor {0} exceeds total models {1}",
        "遊標{0}超出模型總數{1}",
    ),
    (
        "cursor {0} exceeds total feature flags {1}",
        "遊標{0}超出特性開關總數{1}",
    ),
    (
        "failed to resolve permission profiles: {0}",
        "解析許可權檔案失敗：{0}",
    ),
    (
        "cursor {0} exceeds total permission profiles {1}",
        "遊標{0}超出許可權檔案總數{1}",
    ),
    (
        "failed to update skill settings: {0}",
        "更新技能設定失敗：{0}",
    ),
    (
        "failed to refresh MCP servers: {0}",
        "重新整理MCP伺服器失敗：{0}",
    ),
    (
        "failed to resolve MCP server runtime: {0}",
        "解析MCP伺服器執行時失敗：{0}",
    ),
    (
        "failed to resolve MCP OAuth callback: {0}",
        "解析MCP OAuth回撥失敗：{0}",
    ),
    (
        "failed to login to MCP server '{0}': {1}",
        "登入MCP伺服器'{0}'失敗：{1}",
    ),
    (
        "cursor {0} exceeds total MCP servers {1}",
        "遊標{0}超出MCP伺服器總數{1}",
    ),
    ("originCallId requires threadId", "originCallId需要threadId"),
    (
        "failed to deserialize MCP resource read response: {0}",
        "反序列化MCP資源讀取響應失敗：{0}",
    ),
    ("goals feature is disabled", "goals特性已禁用"),
    (
        "ephemeral thread does not support goals: {0}",
        "臨時執行緒不支援goals：{0}",
    ),
    (
        "failed to locate thread id {0}: {1}",
        "定位執行緒id {0}失敗：{1}",
    ),
    (
        "failed to read thread ownership: {0}",
        "讀取執行緒所有權失敗：{0}",
    ),
    (
        "thread metadata does not match requested id",
        "執行緒後設資料與請求的id不匹配",
    ),
    (
        "sqlite state db unavailable for thread goals",
        "執行緒goals需要sqlite狀態資料庫，當前不可用",
    ),
    (
        "failed to parse existing user config.toml",
        "解析已存在的使用者config.toml失敗",
    ),
    (
        "failed to create empty user config.toml",
        "建立空的使用者config.toml失敗",
    ),
    ("invalid value: {0}", "無效的值：{0}"),
    ("keyPath must not be empty", "keyPath不能為空"),
    (
        "unterminated escape in keyPath",
        "keyPath中的轉義序列未終止",
    ),
    ("keyPath segments must not be empty", "keyPath段不能為空"),
    (
        "unterminated quoted keyPath segment",
        "帶引號的keyPath段未終止",
    ),
    (
        "cannot set value on non-table parent",
        "無法在非表型別的父節點上設值",
    ),
    (
        "approval_policy = \"untrusted\" is no longer supported; remove this setting",
        "approval_policy = \"untrusted\" 已不再支援；請移除該設定",
    ),
    (
        "Overridden by packaged defaults: {0}",
        "被打包預設值覆蓋：{0}",
    ),
    (
        "Overridden by managed policy (MDM): {0}",
        "被託管策略(MDM)覆蓋：{0}",
    ),
    (
        "Overridden by managed config (system): {0}",
        "被系統託管配置覆蓋：{0}",
    ),
    (
        "Overridden by enterprise-managed config: {0}",
        "被企業託管配置覆蓋：{0}",
    ),
    ("Overridden by project config: {0}", "被專案配置覆蓋：{0}"),
    ("Overridden by session flags", "被會話標誌覆蓋"),
    ("Overridden by user config: {0}", "被使用者配置覆蓋：{0}"),
    (
        "Overridden by legacy managed_config.toml: {0}",
        "被舊式managed_config.toml覆蓋：{0}",
    ),
    (
        "failed to read user config.toml",
        "讀取使用者config.toml失敗",
    ),
    ("invalid quoted keyPath segment", "無效的帶引號keyPath段"),
    (
        "streaming command/exec is not supported with windows sandbox",
        "windows沙箱不支援流式command/exec",
    ),
    (
        "custom outputBytesCap is not supported with windows sandbox",
        "windows沙箱不支援自定義outputBytesCap",
    ),
    (
        "duplicate active command/exec process id: {0}",
        "重複的活躍command/exec程序id：{0}",
    ),
    ("exec failed: {0}", "exec失敗：{0}"),
    ("invalid command cwd: {0}", "無效的命令cwd：{0}"),
    ("failed to spawn command: {0}", "啟動命令失敗：{0}"),
    ("invalid deltaBase64: {0}", "無效的deltaBase64：{0}"),
    (
        "no active command/exec for process id: {0}",
        "程序id {0}沒有活躍的command/exec",
    ),
    (
        "stdin streaming is not enabled for this command/exec",
        "該command/exec未啟用stdin流式傳輸",
    ),
    ("stdin is already closed", "stdin已關閉"),
    ("failed to resize PTY: {0}", "調整PTY大小失敗：{0}"),
    (
        "in-process request response channel closed: {0}",
        "程序內請求響應通道已關閉：{0}",
    ),
    (
        "in-process app-server client queue is full",
        "程序內app-server客戶端佇列已滿",
    ),
    (
        "in-process app-server runtime is closed",
        "程序內app-server執行時已關閉",
    ),
    (
        "in-process initialize failed: {0}",
        "程序內initialize失敗：{0}",
    ),
    ("duplicate request id: {0}", "重複的請求id：{0}"),
    (
        "in-process app-server request queue is full",
        "程序內app-server請求佇列已滿",
    ),
    (
        "in-process app-server request processor is closed",
        "程序內app-server請求處理器已關閉",
    ),
    ("failed to serialize response: {0}", "序列化響應失敗：{0}"),
    (
        "in-process server request queue is full",
        "程序內服務端請求佇列已滿",
    ),
    (
        "in-process server request consumer is closed",
        "程序內服務端請求消費者已關閉",
    ),
    (
        "in-process app-server runtime is shutting down",
        "程序內app-server執行時正在關閉",
    ),
    ("processHandle must not be empty", "processHandle不能為空"),
    (
        "{0} timeoutMs must be non-negative, got {1}",
        "{0}的timeoutMs必須為非負數，實際收到{1}",
    ),
    (
        "duplicate active process handle: {0}",
        "重複的活躍程序控制代碼：{0}",
    ),
    ("failed to spawn process: {0}", "啟動程序失敗：{0}"),
    (
        "stdin streaming is not enabled for this process",
        "該程序未啟用stdin流式傳輸",
    ),
    (
        "process size rows and cols must be greater than 0",
        "程序size的rows和cols必須大於0",
    ),
    (
        "no active process for process handle {0}",
        "程序控制代碼{0}沒有活躍程序",
    ),
    ("process {0} is no longer running", "程序{0}已不再執行"),
    (
        "failed to record detected connector candidates: {0}",
        "記錄檢測到的聯結器候選項失敗：{0}",
    ),
    (
        "external agent memory import is disabled",
        "外部代理記憶匯入未啟用",
    ),
    (
        "memory import requires at least one selected memory",
        "記憶匯入至少需要選擇一條記憶",
    ),
    ("Import sessions", "匯入會話"),
    ("state database is unavailable", "狀態資料庫不可用"),
    (
        "failed to read import histories: {0}",
        "讀取匯入歷史失敗：{0}",
    ),
    (
        "failed to read imported connector candidates: {0}",
        "讀取已匯入的聯結器候選項失敗：{0}",
    ),
    (
        "failed to record import history: {0}",
        "記錄匯入歷史失敗：{0}",
    ),
    ("Validate session imports", "校驗會話匯入"),
    (
        "external agent session was not detected for import: {0}",
        "未檢測到可匯入的外部代理會話：{0}",
    ),
    (
        "app-server current-time provider is unavailable",
        "app-server當前時間提供者不可用",
    ),
    (
        "external sleep duration is outside the supported range",
        "外部sleep時長超出支援範圍",
    ),
    (
        "external sleep deadline is outside the supported range",
        "外部sleep截止時間超出支援範圍",
    ),
    (
        "timed out waiting for a client to subscribe to the thread after {0}s",
        "等待客戶端訂閱執行緒超時（{0}秒）",
    ),
    (
        "current-time request failed: code={0} message={1}",
        "當前時間請求失敗：code={0} message={1}",
    ),
    (
        "current-time request was canceled: {0}",
        "當前時間請求已取消：{0}",
    ),
    (
        "current-time request timed out after {0}s",
        "當前時間請求超時（{0}秒）",
    ),
    ("invalid current-time response", "無效的當前時間響應"),
    (
        "current-time response is outside the supported range",
        "當前時間響應超出支援範圍",
    ),
    (
        "expected exactly one client subscribed to the thread, found {0}",
        "期望執行緒恰好有1個已訂閱客戶端，實際為{0}個",
    ),
    (
        "failed to load accessible apps: {0}",
        "載入可訪問應用失敗：{0}",
    ),
    ("failed to list apps: {0}", "列出應用失敗：{0}"),
    ("failed to load app lists", "載入應用列表失敗"),
    (
        "timed out waiting for app lists after {0} seconds",
        "等待應用列表超時（{0}秒）",
    ),
    (
        "cursor {0} exceeds total apps {1}",
        "遊標{0}超出應用總數{1}",
    ),
    (
        "mcp tool: {0}/{1}({2}) · {3}",
        "mcp工具：{0}/{1}({2}) · {3}",
    ),
    ("structured result: {0}", "結構化結果：{0}"),
    ("error: {0}", "錯誤：{0}"),
    ("User", "使用者"),
    ("Assistant", "助手"),
    ("Plan", "計劃"),
    ("Reasoning", "推理"),
    ("Activity", "活動"),
    ("section name must not be empty", "分割槽名稱不能為空"),
    (
        "the built-in pinned section cannot be renamed",
        "內建置頂分割槽不可重新命名",
    ),
    ("thread section not found: {0}", "未找到執行緒分割槽：{0}"),
    (
        "the built-in pinned section cannot be deleted",
        "內建置頂分割槽不可刪除",
    ),
    (
        "section appearance {0} must not exceed {1} bytes",
        "分割槽外觀{0}不得超過{1}位元組",
    ),
    (
        "MCP event subscriptions are only supported for hosted apps",
        "MCP事件訂閱僅支援託管應用",
    ),
    (
        "MCP event subscription '{0}' already exists",
        "MCP事件訂閱'{0}'已存在",
    ),
    (
        "MCP event subscription limit of {0} reached",
        "已達到MCP事件訂閱上限{0}",
    ),
    (
        "MCP event subscription authentication changed during startup",
        "MCP事件訂閱啟動期間認證已變更",
    ),
    (
        "connection is not subscribed to thread '{0}'",
        "連線未訂閱執行緒'{0}'",
    ),
    (
        "MCP event stream ended before becoming active",
        "MCP事件流在啟用前已結束",
    ),
    ("MCP event stream startup timed out", "MCP事件流啟動超時"),
    (
        "failed to start MCP event stream for '{0}': {1}",
        "啟動MCP事件流'{0}'失敗：{1}",
    ),
    (
        "external agent session import permit could not be acquired",
        "無法獲取外部代理會話匯入許可",
    ),
    (
        "external agent session preparation task failed: {0}",
        "外部代理會話準備任務失敗：{0}",
    ),
    (
        "failed to prepare external agent session: {0}",
        "準備外部代理會話失敗：{0}",
    ),
    (
        "failed to load imported session config: {0}",
        "載入匯入的會話配置失敗：{0}",
    ),
    ("failed to import session: {0}", "匯入會話失敗：{0}"),
    (
        "failed to update imported session: {0}",
        "更新匯入的會話失敗：{0}",
    ),
    (
        "failed to persist imported session: {0}",
        "持久化匯入的會話失敗：{0}",
    ),
    (
        "failed to shutdown imported session: {0}",
        "關閉匯入的會話失敗：{0}",
    ),
    (
        "auth refresh request canceled: {0}",
        "認證重新整理請求已取消：{0}",
    ),
    (
        "auth refresh request failed: code={0}",
        "認證重新整理請求失敗：code={0}",
    ),
    (
        "auth refresh request timed out after {0}s",
        "認證重新整理請求在{0}秒後超時",
    ),
    ("invalid auth refresh response", "無效的認證重新整理響應"),
    (
        "auth refresh returned invalid credentials",
        "認證重新整理返回了無效憑據",
    ),
    ("external auth lock is poisoned", "外部認證鎖已中毒"),
    (
        "code-mode host URL must contain valid UTF-8",
        "code-mode主機URL必須是有效UTF-8",
    ),
    (
        "invalid code-mode host URL: {0}",
        "無效的code-mode主機URL：{0}",
    ),
    (
        "code-mode host URL must use http:// or https:// with a host",
        "code-mode主機URL必須使用http://或https://幷包含主機名",
    ),
    (
        "code-mode host URL must not contain a fragment",
        "code-mode主機URL不能包含片段",
    ),
    (
        "gRPC code-mode host URL must not contain credentials",
        "gRPC code-mode主機URL不能包含憑據",
    ),
    (
        "gRPC code-mode host URL must not contain a path or query",
        "gRPC code-mode主機URL不能包含路徑或查詢",
    ),
    (
        "`permissionProfile` cannot be combined with `sandboxPolicy`",
        "`permissionProfile`不能與`sandboxPolicy`同時使用",
    ),
    ("invalid permission profile: {0}", "無效的許可權配置：{0}"),
    ("invalid sandbox policy: {0}", "無效的沙箱策略：{0}"),
    (
        "failed to delete app-server state for {0}: {1}",
        "為執行緒{0}刪除app-server狀態失敗：{1}",
    ),
    (
        "thread is not persisted and cannot be deleted: {0}",
        "執行緒未持久化，無法刪除：{0}",
    ),
    (
        "failed to read app-server state for {0}: {1}",
        "為執行緒{0}讀取app-server狀態失敗：{1}",
    ),
    ("failed to delete thread: {0}", "刪除執行緒失敗：{0}"),
    (
        "failed to discover AWS profiles: {0}",
        "發現AWS配置檔案失敗：{0}",
    ),
    (
        "Codex-managed Bedrock credentials are already configured and take priority over AWS environment credentials. Run `codex logout` and try again.",
        "Codex託管的Bedrock憑據已配置，優先順序高於AWS環境憑據。請執行`codex logout`後重試。",
    ),
    (
        "AWS profile name must not be empty.",
        "AWS配置檔名不能為空。",
    ),
    (
        "failed to load credentials for AWS profile `{0}`: {1}",
        "載入AWS配置檔案`{0}`的憑據失敗：{1}",
    ),
    (
        "No AWS credentials found. Please Configure AWS credentials or complete AWS sign-in, then try again.",
        "未找到AWS憑據。請先配置AWS憑據或完成AWS登入，然後重試。",
    ),
    ("creditId must not be empty", "creditId不能為空"),
    (
        "rate limit reset consume timed out",
        "速率限制重置額度兌換超時",
    ),
    (
        "failed to consume rate limit reset: {0}",
        "兌換速率限制重置額度失敗：{0}",
    ),
    (
        "codex account authentication required for rate limit reset credits",
        "兌換速率限制重置額度需要codex賬戶認證",
    ),
    (
        "chatgpt authentication required for rate limit reset credits",
        "兌換速率限制重置額度需要chatgpt認證",
    ),
    (
        "Amazon Bedrock configuration cannot take effect: {0}",
        "Amazon Bedrock配置無法生效：{0}",
    ),
    (
        "failed to load configuration layers: {0}",
        "載入配置層失敗：{0}",
    ),
    (
        "failed to resolve user config path: {0}",
        "未能解析使用者配置路徑：{0}",
    ),
    (
        "Amazon Bedrock login cannot select `{0}` because {1} sets `model_provider` to {2}",
        "Amazon Bedrock登入無法選擇`{0}`，因為{1}把`model_provider`設定為{2}",
    ),
    (
        "failed to reload Amazon Bedrock configuration: {0}",
        "重新載入Amazon Bedrock配置失敗：{0}",
    ),
    (
        "thread {0} is closing; retry after the thread is closed",
        "執行緒{0}正在關閉，請等待其關閉後重試",
    ),
    (
        "sending feedback is disabled by configuration",
        "配置已禁用反饋傳送",
    ),
    (
        "Three feedback uploads are already in progress; try again after one finishes",
        "已有三個反饋上傳在進行中，請等其中一個完成後再試",
    ),
    ("failed to upload feedback: {0}", "上傳反饋失敗：{0}"),
    ("Invalid request: {0}", "請求無效：{0}"),
    (
        "`permissionProfile` is no longer supported for `{0}`; use `permissions` with a named profile id instead",
        "`permissionProfile`已不再支援`{0}`，請改用帶命名profileid的`permissions`",
    ),
    ("Not initialized", "尚未初始化"),
    (
        "MCP event subscription was not reserved before startup",
        "MCP事件訂閱未在啟動前預留",
    ),
    (
        "failed to read thread {0} after rollback: {1}",
        "回滾後讀取執行緒{0}失敗：{1}",
    ),
    (
        "thread {0} did not include persisted history after rollback",
        "執行緒{0}回滾後未包含持久化歷史",
    ),
    (
        "failed to localize granted filesystem paths: {0}",
        "本地化已授權的檔案系統路徑失敗：{0}",
    ),
    ("search remote plugin catalog", "搜尋遠端外掛目錄"),
    (
        "invalid remote plugin search result id: {0}",
        "無效的遠端外掛搜尋結果id：{0}",
    ),
    ("unknown environment id `{0}`", "未知的環境id`{0}`"),
    (
        "failed to get info for environment `{0}`: {1}",
        "獲取環境`{0}`的資訊失敗：{1}",
    ),
    ("Already initialized", "已經初始化過"),
    (
        "Invalid clientInfo.name: '{0}'. Must be a valid HTTP header value.",
        "clientInfo.name無效：'{0}'。必須是合法的HTTP頭值。",
    ),
    (
        "failed to reconcile remote installed plugins: {0}",
        "對賬遠端已安裝外掛失敗：{0}",
    ),
    (
        "plugin hook trust update was cancelled: {0}",
        "外掛鉤子信任更新已取消：{0}",
    ),
    (
        "failed to trust materialized plugin hooks: {0}",
        "信任已物化的外掛鉤子失敗：{0}",
    ),
    ("sessionId must not be empty", "sessionId不能為空"),
    (
        "failed to start fuzzy file search session: {0}",
        "啟動模糊檔案搜尋會話失敗：{0}",
    ),
    (
        "fuzzy file search session not found: {0}",
        "未找到模糊檔案搜尋會話：{0}",
    ),
    ("invalid cwd: {0}", "無效的工作目錄：{0}"),
    (
        "invalid cwd for environment `{0}`: path `{1}` does not use absolute POSIX or Windows path syntax",
        "環境`{0}`的工作目錄無效：路徑`{1}`不是絕對POSIX或Windows路徑語法",
    ),
    (
        "invalid runtime workspace root for environment `{0}`: path `{1}` does not use absolute POSIX or Windows path syntax",
        "環境`{0}`的執行時工作區根目錄無效：路徑`{1}`不是絕對POSIX或Windows路徑語法",
    ),
    (
        "failed to resolve feature override precedence: {0}",
        "解析功能開關覆蓋優先順序失敗：{0}",
    ),
    (
        "failed to update feature enablement",
        "更新功能啟用狀態失敗",
    ),
    ("failed to upgrade marketplaces: {0}", "升級市場源失敗：{0}"),
    (
        "failed to decode import item type {0}: {1}",
        "解碼匯入條目型別{0}失敗：{1}",
    ),
    ("watchId already exists: {0}", "watchId已存在：{0}"),
    (
        "client request resolved because the turn state was changed",
        "客戶端請求因輪次狀態變化而被解除",
    ),
    (
        "User verification is not available in this build.",
        "當前構建不支援使用者驗證。",
    ),
    (
        "failed to read app metadata: {0}",
        "讀取應用後設資料失敗：{0}",
    ),
    ("failed to load configuration: {0}", "載入配置失敗：{0}"),
    ("local filesystem is not configured", "未配置本地檔案系統"),
    (
        "remote control is unavailable for this app-server",
        "該app-server不支援遠端控制",
    ),
    (
        "failed to validate environment selections: {0}",
        "校驗環境選擇失敗：{0}",
    ),
    (
        "Windows sandbox service provisioning task failed: {0}",
        "Windows沙箱服務配置任務失敗：{0}",
    ),
    (
        "invalid Windows sandbox setup mode: {0}",
        "無效的Windows沙箱設定模式：{0}",
    ),
    (
        "failed to refresh installed connector runtime state: {0}",
        "重新整理已安裝聯結器的執行時狀態失敗：{0}",
    ),
    (
        "failed to compute git diff to remote for cwd: {0}",
        "為工作目錄{0}計算到遠端的git diff失敗",
    ),
    (
        "direct app-server input is not allowed for multi-agent v2 sub-agents",
        "不允許直接向multi-agent v2子代理下發app-server輸入",
    ),
    (
        "remote image URLs are not supported; use an inline data URL instead",
        "不支援遠端圖片URL，請改用內聯data URL",
    ),
];

/// English source text -> Traditional Chinese.
pub(crate) static DICT_ZH_HANT: LazyLock<HashMap<&'static str, &'static str>> =
    LazyLock::new(|| ENTRIES.iter().copied().collect());
