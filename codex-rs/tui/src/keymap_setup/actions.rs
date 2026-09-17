//! Catalog and accessors for keymap actions shown by `/keymap`.
//!
//! The descriptor table is the single UI-facing inventory of configurable
//! actions. Each descriptor ties together the config path segment, user-facing
//! context label, stable action name, and short description used by the picker
//! and action menu.
//!
//! Root-config accessors mirror the descriptor table, while runtime lookups
//! reuse the inventory owned by [`crate::keymap`]. A catalog action must remain
//! both writable in `TuiKeymap` and readable from the shared runtime inventory.

use std::collections::BTreeSet;
use std::sync::OnceLock;

use codex_config::types::KeybindingsSpec;
use codex_config::types::TuiKeymap;
use crossterm::event::KeyEvent;

use codex_i18n::current;
use codex_i18n::tr;

use crate::keymap::RuntimeKeymap;
use crate::keymap::bindings_for_action;

#[derive(Clone, Copy, Debug)]
pub(super) struct KeymapActionDescriptor {
    /// Config context segment, such as `composer` in `tui.keymap.composer.submit`.
    pub(super) context: &'static str,
    /// Human-readable group label shown in the picker.
    pub(super) context_label: &'static str,
    /// Config action segment, such as `submit` in `tui.keymap.composer.submit`.
    pub(super) action: &'static str,
    /// Short user-facing explanation of what the action does.
    pub(super) description: &'static str,
    /// Feature required before the action appears in `/keymap`.
    required_feature: Option<KeymapActionFeature>,
}

fn action(
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    description: &'static str,
) -> KeymapActionDescriptor {
    KeymapActionDescriptor {
        context,
        context_label,
        action,
        description,
        required_feature: None,
    }
}

fn gated_action(
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    description: &'static str,
    required_feature: KeymapActionFeature,
) -> KeymapActionDescriptor {
    KeymapActionDescriptor {
        context,
        context_label,
        action,
        description,
        required_feature: Some(required_feature),
    }
}

#[derive(Clone, Copy, Debug)]
enum KeymapActionFeature {
    FastMode,
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct KeymapActionFilter {
    pub(crate) fast_mode_enabled: bool,
}

impl KeymapActionDescriptor {
    pub(super) fn is_visible(self, filter: KeymapActionFilter) -> bool {
        match self.required_feature {
            None => true,
            Some(KeymapActionFeature::FastMode) => filter.fast_mode_enabled,
        }
    }
}

#[rustfmt::skip]
/// The keymap inventory, rendered in the current UI language.
///
/// A cached function rather than a `const`: the context labels and descriptions
/// have to pass through `tr`, and a `const` cannot call a non-`const` function
/// (see `docs/plan/i18n-design.md` §3.6). The language is fixed for the life of
/// the process, so computing the table once is enough, and returning a
/// `&'static [..]` slice keeps every call site working unchanged.
pub(super) fn keymap_actions() -> &'static [KeymapActionDescriptor] {
    static ACTIONS: OnceLock<Vec<KeymapActionDescriptor>> = OnceLock::new();
    ACTIONS.get_or_init(|| {
        vec![
    action("global", tr(current(), "Global"), "open_agents", tr(current(), "Open the shared agent-session overview.")),
    action("global", tr(current(), "Global"), "open_transcript", tr(current(), "Open the transcript overlay.")),
    action("global", tr(current(), "Global"), "open_external_editor", tr(current(), "Open the current draft in an external editor.")),
    action("global", tr(current(), "Global"), "copy", tr(current(), "Copy the last agent response to the clipboard.")),
    action("global", tr(current(), "Global"), "clear_terminal", tr(current(), "Clear the terminal UI.")),
    action("global", tr(current(), "Global"), "toggle_vim_mode", tr(current(), "Turn Vim composer mode on or off.")),
    gated_action("global", tr(current(), "Global"), "toggle_fast_mode", tr(current(), "Turn Fast mode on or off."), KeymapActionFeature::FastMode),
    action("global", tr(current(), "Global"), "toggle_raw_output", tr(current(), "Toggle raw scrollback mode.")),
    action("global", tr(current(), "Global"), "toggle_side_conversation", tr(current(), "Switch between a side conversation and its parent.")),
    action("chat", tr(current(), "Chat"), "interrupt_turn", tr(current(), "Interrupt the active turn.")),
    action("chat", tr(current(), "Chat"), "decrease_reasoning_effort", tr(current(), "Decrease reasoning effort.")),
    action("chat", tr(current(), "Chat"), "increase_reasoning_effort", tr(current(), "Increase reasoning effort.")),
    action("chat", tr(current(), "Chat"), "previous_permission_mode", tr(current(), "Switch to the previous available permission mode.")),
    action("chat", tr(current(), "Chat"), "next_permission_mode", tr(current(), "Switch to the next available permission mode.")),
    action("chat", tr(current(), "Chat"), "edit_queued_message", tr(current(), "Move up through questions, then edit the last queued message.")),
    action("chat", tr(current(), "Chat"), "prompt_stack_back", tr(current(), "Move back through questions toward the composer.")),
    action("chat", tr(current(), "Chat"), "skip_question", tr(current(), "Skip the focused question.")),
    action("composer", tr(current(), "Composer"), "submit", tr(current(), "Submit the current composer draft.")),
    action("composer", tr(current(), "Composer"), "queue", tr(current(), "Queue the draft while a task is running.")),
    action("composer", tr(current(), "Composer"), "toggle_shortcuts", tr(current(), "Show or hide the composer shortcut overlay.")),
    action("composer", tr(current(), "Composer"), "history_search_previous", tr(current(), "Open history search or move to the previous match.")),
    action("composer", tr(current(), "Composer"), "history_search_next", tr(current(), "Move to the next history search match.")),
    action("editor", tr(current(), "Editor"), "insert_newline", tr(current(), "Insert a newline in the editor.")),
    action("editor", tr(current(), "Editor"), "move_left", tr(current(), "Move the cursor left.")),
    action("editor", tr(current(), "Editor"), "move_right", tr(current(), "Move the cursor right.")),
    action("editor", tr(current(), "Editor"), "move_up", tr(current(), "Move the cursor up.")),
    action("editor", tr(current(), "Editor"), "move_down", tr(current(), "Move the cursor down.")),
    action("editor", tr(current(), "Editor"), "move_word_left", tr(current(), "Move to the beginning of the previous word.")),
    action("editor", tr(current(), "Editor"), "move_word_right", tr(current(), "Move to the end of the next word.")),
    action("editor", tr(current(), "Editor"), "move_line_start", tr(current(), "Move to the beginning of the line.")),
    action("editor", tr(current(), "Editor"), "move_line_end", tr(current(), "Move to the end of the line.")),
    action("editor", tr(current(), "Editor"), "delete_backward", tr(current(), "Delete one grapheme to the left.")),
    action("editor", tr(current(), "Editor"), "delete_forward", tr(current(), "Delete one grapheme to the right.")),
    action("editor", tr(current(), "Editor"), "delete_backward_word", tr(current(), "Delete the previous word.")),
    action("editor", tr(current(), "Editor"), "delete_forward_word", tr(current(), "Delete the next word.")),
    action("editor", tr(current(), "Editor"), "kill_line_start", tr(current(), "Delete from cursor to line start.")),
    action("editor", tr(current(), "Editor"), "kill_whole_line", tr(current(), "Delete the current line.")),
    action("editor", tr(current(), "Editor"), "kill_line_end", tr(current(), "Delete from cursor to line end.")),
    action("editor", tr(current(), "Editor"), "yank", tr(current(), "Paste the kill buffer.")),
    action("vim_normal", tr(current(), "Vim normal"), "enter_insert", tr(current(), "Enter insert mode at the cursor.")),
    action("vim_normal", tr(current(), "Vim normal"), "append_after_cursor", tr(current(), "Enter insert mode after the cursor.")),
    action("vim_normal", tr(current(), "Vim normal"), "append_line_end", tr(current(), "Enter insert mode at end of line.")),
    action("vim_normal", tr(current(), "Vim normal"), "insert_line_start", tr(current(), "Enter insert mode at the first non-blank character.")),
    action("vim_normal", tr(current(), "Vim normal"), "open_line_below", tr(current(), "Open a new line below and enter insert mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "open_line_above", tr(current(), "Open a new line above and enter insert mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "enter_replace_mode", tr(current(), "Enter replace mode and overwrite characters.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_left", tr(current(), "Move left in Vim normal mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_right", tr(current(), "Move right in Vim normal mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_up", tr(current(), "Move up or recall older history in Vim normal mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_down", tr(current(), "Move down or recall newer history in Vim normal mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_word_forward", tr(current(), "Move to the start of the next word.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_word_backward", tr(current(), "Move to the start of the previous word.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_word_end", tr(current(), "Move to the end of the current or next word.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_line_start", tr(current(), "Move to the start of the line.")),
    action("vim_normal", tr(current(), "Vim normal"), "move_line_end", tr(current(), "Move to the end of the line.")),
    action("vim_normal", tr(current(), "Vim normal"), "find_forward", tr(current(), "Find the next character on the current line.")),
    action("vim_normal", tr(current(), "Vim normal"), "find_backward", tr(current(), "Find the previous character on the current line.")),
    action("vim_normal", tr(current(), "Vim normal"), "till_forward", tr(current(), "Stop before the next character on the current line.")),
    action("vim_normal", tr(current(), "Vim normal"), "till_backward", tr(current(), "Stop after the previous character on the current line.")),
    action("vim_normal", tr(current(), "Vim normal"), "jump_top", tr(current(), "Jump to the first buffer line.")),
    action("vim_normal", tr(current(), "Vim normal"), "jump_bottom", tr(current(), "Jump to the last buffer line.")),
    action("vim_normal", tr(current(), "Vim normal"), "delete_char", tr(current(), "Delete the character under the cursor.")),
    action("vim_normal", tr(current(), "Vim normal"), "replace_char", tr(current(), "Replace the character under the cursor.")),
    action("vim_normal", tr(current(), "Vim normal"), "repeat_last_change", tr(current(), "Repeat the last complete edit.")),
    action("vim_normal", tr(current(), "Vim normal"), "substitute_char", tr(current(), "Delete the character under the cursor and enter insert mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "delete_to_line_end", tr(current(), "Delete from cursor to end of line.")),
    action("vim_normal", tr(current(), "Vim normal"), "change_to_line_end", tr(current(), "Change from cursor to end of line and enter insert mode.")),
    action("vim_normal", tr(current(), "Vim normal"), "yank_line", tr(current(), "Yank the entire line.")),
    action("vim_normal", tr(current(), "Vim normal"), "paste_after", tr(current(), "Paste after the cursor.")),
    action("vim_normal", tr(current(), "Vim normal"), "start_delete_operator", tr(current(), "Begin a delete operator and wait for a motion.")),
    action("vim_normal", tr(current(), "Vim normal"), "start_yank_operator", tr(current(), "Begin a yank operator and wait for a motion.")),
    action("vim_normal", tr(current(), "Vim normal"), "start_change_operator", tr(current(), "Begin a change operator and wait for a motion or text object.")),
    action("vim_normal", tr(current(), "Vim normal"), "undo", tr(current(), "Undo the last complete edit.")),
    action("vim_normal", tr(current(), "Vim normal"), "redo", tr(current(), "Redo the last undone edit.")),
    action("vim_normal", tr(current(), "Vim normal"), "cancel_operator", tr(current(), "Cancel a pending Vim operator.")),
    action("vim_search", tr(current(), "Vim search"), "forward", tr(current(), "Search forward in the active buffer.")),
    action("vim_search", tr(current(), "Vim search"), "backward", tr(current(), "Search backward in the active buffer.")),
    action("vim_search", tr(current(), "Vim search"), "next", tr(current(), "Repeat the accepted search.")),
    action("vim_search", tr(current(), "Vim search"), "previous", tr(current(), "Repeat the search in the opposite direction.")),
    action("vim_operator", tr(current(), "Vim operator"), "delete_line", tr(current(), "Repeat delete operator to delete the whole line.")),
    action("vim_operator", tr(current(), "Vim operator"), "yank_line", tr(current(), "Repeat yank operator to yank the whole line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_left", tr(current(), "Operator motion left.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_right", tr(current(), "Operator motion right.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_up", tr(current(), "Operator motion up.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_down", tr(current(), "Operator motion down.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_word_forward", tr(current(), "Operator motion to start of next word.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_word_backward", tr(current(), "Operator motion to start of previous word.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_word_end", tr(current(), "Operator motion to end of word.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_line_start", tr(current(), "Operator motion to line start.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_line_end", tr(current(), "Operator motion to line end.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_find_forward", tr(current(), "Operator motion to the next character on the current line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_find_backward", tr(current(), "Operator motion to the previous character on the current line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_till_forward", tr(current(), "Stop before the next character on the current line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_till_backward", tr(current(), "Stop after the previous character on the current line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_jump_top", tr(current(), "Operator motion to the first buffer line.")),
    action("vim_operator", tr(current(), "Vim operator"), "motion_jump_bottom", tr(current(), "Operator motion to the last buffer line.")),
    action("vim_operator", tr(current(), "Vim operator"), "select_inner_text_object", tr(current(), "Select an inner text object.")),
    action("vim_operator", tr(current(), "Vim operator"), "select_around_text_object", tr(current(), "Select an around text object.")),
    action("vim_operator", tr(current(), "Vim operator"), "cancel", tr(current(), "Cancel the pending operator.")),
    action("vim_text_object", tr(current(), "Vim text object"), "word", tr(current(), "Target the current word.")),
    action("vim_text_object", tr(current(), "Vim text object"), "big_word", tr(current(), "Target the current WORD.")),
    action("vim_text_object", tr(current(), "Vim text object"), "parentheses", tr(current(), "Target enclosing parentheses.")),
    action("vim_text_object", tr(current(), "Vim text object"), "brackets", tr(current(), "Target enclosing brackets.")),
    action("vim_text_object", tr(current(), "Vim text object"), "braces", tr(current(), "Target enclosing braces.")),
    action("vim_text_object", tr(current(), "Vim text object"), "double_quote", tr(current(), "Target enclosing double quotes.")),
    action("vim_text_object", tr(current(), "Vim text object"), "single_quote", tr(current(), "Target enclosing single quotes.")),
    action("vim_text_object", tr(current(), "Vim text object"), "backtick", tr(current(), "Target enclosing backticks.")),
    action("vim_text_object", tr(current(), "Vim text object"), "cancel", tr(current(), "Cancel the pending text object.")),
    action("pager", tr(current(), "Pager"), "scroll_up", tr(current(), "Scroll up by one row.")),
    action("pager", tr(current(), "Pager"), "scroll_down", tr(current(), "Scroll down by one row.")),
    action("pager", tr(current(), "Pager"), "page_up", tr(current(), "Scroll up by one page.")),
    action("pager", tr(current(), "Pager"), "page_down", tr(current(), "Scroll down by one page.")),
    action("pager", tr(current(), "Pager"), "half_page_up", tr(current(), "Scroll up by half a page.")),
    action("pager", tr(current(), "Pager"), "half_page_down", tr(current(), "Scroll down by half a page.")),
    action("pager", tr(current(), "Pager"), "jump_top", tr(current(), "Jump to the beginning.")),
    action("pager", tr(current(), "Pager"), "jump_bottom", tr(current(), "Jump to the end.")),
    action("pager", tr(current(), "Pager"), "close", tr(current(), "Close the pager overlay.")),
    action("pager", tr(current(), "Pager"), "close_transcript", tr(current(), "Close the transcript overlay.")),
    action("list", tr(current(), "List"), "move_up", tr(current(), "Move list selection up.")),
    action("list", tr(current(), "List"), "move_down", tr(current(), "Move list selection down.")),
    action("list", tr(current(), "List"), "move_left", tr(current(), "Move horizontally left in list pickers.")),
    action("list", tr(current(), "List"), "move_right", tr(current(), "Move horizontally right in list pickers.")),
    action("list", tr(current(), "List"), "page_up", tr(current(), "Move list selection up by one page.")),
    action("list", tr(current(), "List"), "page_down", tr(current(), "Move list selection down by one page.")),
    action("list", tr(current(), "List"), "jump_top", tr(current(), "Jump to the first list item.")),
    action("list", tr(current(), "List"), "jump_bottom", tr(current(), "Jump to the last list item.")),
    action("list", tr(current(), "List"), "accept", tr(current(), "Accept the current list selection.")),
    action("list", tr(current(), "List"), "cancel", tr(current(), "Cancel and close selection views.")),
    action("agents", tr(current(), "Agents"), "resume", tr(current(), "Open the session resume picker.")),
    action("agents", tr(current(), "Agents"), "search", tr(current(), "Search the available agent tasks.")),
    action("agents", tr(current(), "Agents"), "new_task", tr(current(), "Start composing a new agent task.")),
    action("agents", tr(current(), "Agents"), "rename", tr(current(), "Rename the selected task.")),
    action("agents", tr(current(), "Agents"), "stop", tr(current(), "Stop the selected running task.")),
    action("agents", tr(current(), "Agents"), "toggle_grouping", tr(current(), "Group tasks by status or project.")),
    action("approval", tr(current(), "Approval"), "open_fullscreen", tr(current(), "Open approval details fullscreen.")),
    action("approval", tr(current(), "Approval"), "open_thread", tr(current(), "Open the approval source thread when available.")),
    action("approval", tr(current(), "Approval"), "approve", tr(current(), "Approve the primary option.")),
    action("approval", tr(current(), "Approval"), "approve_for_session", tr(current(), "Approve for the session when available.")),
    action("approval", tr(current(), "Approval"), "approve_for_prefix", tr(current(), "Approve with an exec-policy prefix when available.")),
    action("approval", tr(current(), "Approval"), "deny", tr(current(), "Choose the explicit deny option when available.")),
    action("approval", tr(current(), "Approval"), "decline", tr(current(), "Decline and provide corrective guidance.")),
    action("approval", tr(current(), "Approval"), "cancel", tr(current(), "Cancel an elicitation request.")),
        ]
    })
}

/// Convert a stable action identifier into a display label.
///
/// This is intentionally presentation-only: the returned string must never be
/// parsed back into an action name, because underscores and casing are part of
/// the stable config contract.
pub(super) fn action_label(action: &str) -> String {
    action
        .split('_')
        .map(|word| {
            let mut chars = word.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            format!("{}{}", first.to_ascii_uppercase(), chars.as_str())
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[rustfmt::skip]
/// Return the mutable root-config binding slot for one catalog action.
///
/// The returned `Option<KeybindingsSpec>` distinguishes three states that the
/// editor must preserve: absent means use fallback/default resolution, `Some`
/// with one or more keys is a custom binding, and `Some(Many([]))` is an
/// explicit unbind.
pub(super) fn binding_slot<'a>(
    keymap: &'a mut TuiKeymap,
    context: &str,
    action: &str,
) -> Option<&'a mut Option<KeybindingsSpec>> {
    match (context, action) {
        ("global", "open_agents") => Some(&mut keymap.global.open_agents),
        ("global", "open_transcript") => Some(&mut keymap.global.open_transcript),
        ("global", "open_external_editor") => Some(&mut keymap.global.open_external_editor),
        ("global", "copy") => Some(&mut keymap.global.copy),
        ("global", "clear_terminal") => Some(&mut keymap.global.clear_terminal),
        ("global", "toggle_vim_mode") => Some(&mut keymap.global.toggle_vim_mode),
        ("global", "toggle_fast_mode") => Some(&mut keymap.global.toggle_fast_mode),
        ("global", "toggle_raw_output") => Some(&mut keymap.global.toggle_raw_output),
        ("global", "toggle_side_conversation") => Some(&mut keymap.global.toggle_side_conversation),
        ("chat", "interrupt_turn") => Some(&mut keymap.chat.interrupt_turn),
        ("chat", "decrease_reasoning_effort") => Some(&mut keymap.chat.decrease_reasoning_effort),
        ("chat", "increase_reasoning_effort") => Some(&mut keymap.chat.increase_reasoning_effort),
        ("chat", "previous_permission_mode") => Some(&mut keymap.chat.previous_permission_mode),
        ("chat", "next_permission_mode") => Some(&mut keymap.chat.next_permission_mode),
        ("chat", "edit_queued_message") => Some(&mut keymap.chat.edit_queued_message),
        ("chat", "prompt_stack_back") => Some(&mut keymap.chat.prompt_stack_back),
        ("chat", "skip_question") => Some(&mut keymap.chat.skip_question),
        ("composer", "submit") => Some(&mut keymap.composer.submit),
        ("composer", "queue") => Some(&mut keymap.composer.queue),
        ("composer", "toggle_shortcuts") => Some(&mut keymap.composer.toggle_shortcuts),
        ("composer", "history_search_previous") => Some(&mut keymap.composer.history_search_previous),
        ("composer", "history_search_next") => Some(&mut keymap.composer.history_search_next),
        ("editor", "insert_newline") => Some(&mut keymap.editor.insert_newline),
        ("editor", "move_left") => Some(&mut keymap.editor.move_left),
        ("editor", "move_right") => Some(&mut keymap.editor.move_right),
        ("editor", "move_up") => Some(&mut keymap.editor.move_up),
        ("editor", "move_down") => Some(&mut keymap.editor.move_down),
        ("editor", "move_word_left") => Some(&mut keymap.editor.move_word_left),
        ("editor", "move_word_right") => Some(&mut keymap.editor.move_word_right),
        ("editor", "move_line_start") => Some(&mut keymap.editor.move_line_start),
        ("editor", "move_line_end") => Some(&mut keymap.editor.move_line_end),
        ("editor", "delete_backward") => Some(&mut keymap.editor.delete_backward),
        ("editor", "delete_forward") => Some(&mut keymap.editor.delete_forward),
        ("editor", "delete_backward_word") => Some(&mut keymap.editor.delete_backward_word),
        ("editor", "delete_forward_word") => Some(&mut keymap.editor.delete_forward_word),
        ("editor", "kill_line_start") => Some(&mut keymap.editor.kill_line_start),
        ("editor", "kill_whole_line") => Some(&mut keymap.editor.kill_whole_line),
        ("editor", "kill_line_end") => Some(&mut keymap.editor.kill_line_end),
        ("editor", "yank") => Some(&mut keymap.editor.yank),
        ("vim_normal", "enter_insert") => Some(&mut keymap.vim_normal.enter_insert),
        ("vim_normal", "append_after_cursor") => Some(&mut keymap.vim_normal.append_after_cursor),
        ("vim_normal", "append_line_end") => Some(&mut keymap.vim_normal.append_line_end),
        ("vim_normal", "insert_line_start") => Some(&mut keymap.vim_normal.insert_line_start),
        ("vim_normal", "open_line_below") => Some(&mut keymap.vim_normal.open_line_below),
        ("vim_normal", "open_line_above") => Some(&mut keymap.vim_normal.open_line_above),
        ("vim_normal", "enter_replace_mode") => Some(&mut keymap.vim_normal.enter_replace_mode),
        ("vim_normal", "move_left") => Some(&mut keymap.vim_normal.move_left),
        ("vim_normal", "move_right") => Some(&mut keymap.vim_normal.move_right),
        ("vim_normal", "move_up") => Some(&mut keymap.vim_normal.move_up),
        ("vim_normal", "move_down") => Some(&mut keymap.vim_normal.move_down),
        ("vim_normal", "move_word_forward") => Some(&mut keymap.vim_normal.move_word_forward),
        ("vim_normal", "move_word_backward") => Some(&mut keymap.vim_normal.move_word_backward),
        ("vim_normal", "move_word_end") => Some(&mut keymap.vim_normal.move_word_end),
        ("vim_normal", "move_line_start") => Some(&mut keymap.vim_normal.move_line_start),
        ("vim_normal", "move_line_end") => Some(&mut keymap.vim_normal.move_line_end),
        ("vim_normal", "find_forward") => Some(&mut keymap.vim_normal.find_forward),
        ("vim_normal", "find_backward") => Some(&mut keymap.vim_normal.find_backward),
        ("vim_normal", "till_forward") => Some(&mut keymap.vim_normal.till_forward),
        ("vim_normal", "till_backward") => Some(&mut keymap.vim_normal.till_backward),
        ("vim_normal", "jump_top") => Some(&mut keymap.vim_normal.jump_top),
        ("vim_normal", "jump_bottom") => Some(&mut keymap.vim_normal.jump_bottom),
        ("vim_normal", "delete_char") => Some(&mut keymap.vim_normal.delete_char),
        ("vim_normal", "replace_char") => Some(&mut keymap.vim_normal.replace_char),
        ("vim_normal", "repeat_last_change") => Some(&mut keymap.vim_normal.repeat_last_change),
        ("vim_normal", "substitute_char") => Some(&mut keymap.vim_normal.substitute_char),
        ("vim_normal", "delete_to_line_end") => Some(&mut keymap.vim_normal.delete_to_line_end),
        ("vim_normal", "change_to_line_end") => Some(&mut keymap.vim_normal.change_to_line_end),
        ("vim_normal", "yank_line") => Some(&mut keymap.vim_normal.yank_line),
        ("vim_normal", "paste_after") => Some(&mut keymap.vim_normal.paste_after),
        ("vim_normal", "start_delete_operator") => Some(&mut keymap.vim_normal.start_delete_operator),
        ("vim_normal", "start_yank_operator") => Some(&mut keymap.vim_normal.start_yank_operator),
        ("vim_normal", "start_change_operator") => Some(&mut keymap.vim_normal.start_change_operator),
        ("vim_normal", "undo") => Some(&mut keymap.vim_normal.undo),
        ("vim_normal", "redo") => Some(&mut keymap.vim_normal.redo),
        ("vim_normal", "cancel_operator") => Some(&mut keymap.vim_normal.cancel_operator),
        ("vim_search", "forward") => Some(&mut keymap.vim_search.forward),
        ("vim_search", "backward") => Some(&mut keymap.vim_search.backward),
        ("vim_search", "next") => Some(&mut keymap.vim_search.next),
        ("vim_search", "previous") => Some(&mut keymap.vim_search.previous),
        ("vim_operator", "delete_line") => Some(&mut keymap.vim_operator.delete_line),
        ("vim_operator", "yank_line") => Some(&mut keymap.vim_operator.yank_line),
        ("vim_operator", "motion_left") => Some(&mut keymap.vim_operator.motion_left),
        ("vim_operator", "motion_right") => Some(&mut keymap.vim_operator.motion_right),
        ("vim_operator", "motion_up") => Some(&mut keymap.vim_operator.motion_up),
        ("vim_operator", "motion_down") => Some(&mut keymap.vim_operator.motion_down),
        ("vim_operator", "motion_word_forward") => Some(&mut keymap.vim_operator.motion_word_forward),
        ("vim_operator", "motion_word_backward") => Some(&mut keymap.vim_operator.motion_word_backward),
        ("vim_operator", "motion_word_end") => Some(&mut keymap.vim_operator.motion_word_end),
        ("vim_operator", "motion_line_start") => Some(&mut keymap.vim_operator.motion_line_start),
        ("vim_operator", "motion_line_end") => Some(&mut keymap.vim_operator.motion_line_end),
        ("vim_operator", "motion_find_forward") => Some(&mut keymap.vim_operator.motion_find_forward),
        ("vim_operator", "motion_find_backward") => Some(&mut keymap.vim_operator.motion_find_backward),
        ("vim_operator", "motion_till_forward") => Some(&mut keymap.vim_operator.motion_till_forward),
        ("vim_operator", "motion_till_backward") => Some(&mut keymap.vim_operator.motion_till_backward),
        ("vim_operator", "motion_jump_top") => Some(&mut keymap.vim_operator.motion_jump_top),
        ("vim_operator", "motion_jump_bottom") => Some(&mut keymap.vim_operator.motion_jump_bottom),
        ("vim_operator", "select_inner_text_object") => Some(&mut keymap.vim_operator.select_inner_text_object),
        ("vim_operator", "select_around_text_object") => Some(&mut keymap.vim_operator.select_around_text_object),
        ("vim_operator", "cancel") => Some(&mut keymap.vim_operator.cancel),
        ("vim_text_object", "word") => Some(&mut keymap.vim_text_object.word),
        ("vim_text_object", "big_word") => Some(&mut keymap.vim_text_object.big_word),
        ("vim_text_object", "parentheses") => Some(&mut keymap.vim_text_object.parentheses),
        ("vim_text_object", "brackets") => Some(&mut keymap.vim_text_object.brackets),
        ("vim_text_object", "braces") => Some(&mut keymap.vim_text_object.braces),
        ("vim_text_object", "double_quote") => Some(&mut keymap.vim_text_object.double_quote),
        ("vim_text_object", "single_quote") => Some(&mut keymap.vim_text_object.single_quote),
        ("vim_text_object", "backtick") => Some(&mut keymap.vim_text_object.backtick),
        ("vim_text_object", "cancel") => Some(&mut keymap.vim_text_object.cancel),
        ("pager", "scroll_up") => Some(&mut keymap.pager.scroll_up),
        ("pager", "scroll_down") => Some(&mut keymap.pager.scroll_down),
        ("pager", "page_up") => Some(&mut keymap.pager.page_up),
        ("pager", "page_down") => Some(&mut keymap.pager.page_down),
        ("pager", "half_page_up") => Some(&mut keymap.pager.half_page_up),
        ("pager", "half_page_down") => Some(&mut keymap.pager.half_page_down),
        ("pager", "jump_top") => Some(&mut keymap.pager.jump_top),
        ("pager", "jump_bottom") => Some(&mut keymap.pager.jump_bottom),
        ("pager", "close") => Some(&mut keymap.pager.close),
        ("pager", "close_transcript") => Some(&mut keymap.pager.close_transcript),
        ("list", "move_up") => Some(&mut keymap.list.move_up),
        ("list", "move_down") => Some(&mut keymap.list.move_down),
        ("list", "move_left") => Some(&mut keymap.list.move_left),
        ("list", "move_right") => Some(&mut keymap.list.move_right),
        ("list", "page_up") => Some(&mut keymap.list.page_up),
        ("list", "page_down") => Some(&mut keymap.list.page_down),
        ("list", "jump_top") => Some(&mut keymap.list.jump_top),
        ("list", "jump_bottom") => Some(&mut keymap.list.jump_bottom),
        ("list", "accept") => Some(&mut keymap.list.accept),
        ("list", "cancel") => Some(&mut keymap.list.cancel),
        ("agents", "resume") => Some(&mut keymap.agents.resume),
        ("agents", "search") => Some(&mut keymap.agents.search),
        ("agents", "new_task") => Some(&mut keymap.agents.new_task),
        ("agents", "rename") => Some(&mut keymap.agents.rename),
        ("agents", "stop") => Some(&mut keymap.agents.stop),
        ("agents", "toggle_grouping") => Some(&mut keymap.agents.toggle_grouping),
        ("approval", "open_fullscreen") => Some(&mut keymap.approval.open_fullscreen),
        ("approval", "open_thread") => Some(&mut keymap.approval.open_thread),
        ("approval", "approve") => Some(&mut keymap.approval.approve),
        ("approval", "approve_for_session") => Some(&mut keymap.approval.approve_for_session),
        ("approval", "approve_for_prefix") => Some(&mut keymap.approval.approve_for_prefix),
        ("approval", "deny") => Some(&mut keymap.approval.deny),
        ("approval", "decline") => Some(&mut keymap.approval.decline),
        ("approval", "cancel") => Some(&mut keymap.approval.cancel),
        _ => None,
    }
}

/// Format an action's active single-key and chord alternatives in config order.
///
/// Duplicate runtime variants that normalize to the same config spec are shown
/// once so compatibility defaults do not appear as separate user choices.
pub(super) fn format_action_binding_summary(
    runtime_keymap: &RuntimeKeymap,
    context: &str,
    action: &str,
) -> String {
    let specs = super::active_binding_specs(runtime_keymap, context, action).unwrap_or_else(|_| {
        bindings_for_action(runtime_keymap, context, action)
            .unwrap_or_default()
            .iter()
            .filter_map(|binding| super::binding_to_config_key_spec(*binding).ok())
            .collect()
    });
    let mut seen = BTreeSet::new();
    let specs = specs
        .into_iter()
        .filter(|spec| seen.insert(spec.clone()))
        .collect::<Vec<_>>();
    if specs.is_empty() {
        "unbound".to_string()
    } else {
        specs.join(", ")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum KeymapDebugBindingSource {
    Custom,
    CustomGlobal,
    Default,
}

impl KeymapDebugBindingSource {
    /// Not `const`: the label reaches a rendered `Line`
    /// (`debug.rs:148`), so it has to pass through `tr`, which a `const fn`
    /// cannot call (§3.6 "const tables become functions").
    pub(super) fn label(&self) -> &'static str {
        match self {
            Self::Custom => tr(current(), "Custom"),
            Self::CustomGlobal => tr(current(), "Custom global"),
            Self::Default => tr(current(), "Default"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct KeymapDebugActionMatch {
    pub(super) context: &'static str,
    pub(super) action: &'static str,
    pub(super) label: String,
    pub(super) description: &'static str,
    pub(super) source: KeymapDebugBindingSource,
}

pub(super) fn matching_actions_for_key_event(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    event: KeyEvent,
) -> Vec<KeymapDebugActionMatch> {
    keymap_actions()
        .iter()
        .filter_map(|descriptor| {
            let bindings =
                bindings_for_action(runtime_keymap, descriptor.context, descriptor.action)?;
            bindings
                .iter()
                .any(|binding| binding.is_press(event))
                .then(|| KeymapDebugActionMatch {
                    context: descriptor.context,
                    action: descriptor.action,
                    label: action_label(descriptor.action),
                    description: descriptor.description,
                    source: debug_binding_source(keymap_config, descriptor),
                })
        })
        .collect()
}

fn debug_binding_source(
    keymap_config: &TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> KeymapDebugBindingSource {
    let mut keymap_config = keymap_config.clone();
    let Some(slot) = binding_slot(&mut keymap_config, descriptor.context, descriptor.action) else {
        return KeymapDebugBindingSource::Default;
    };
    if slot.is_some() {
        return KeymapDebugBindingSource::Custom;
    }

    let Some(global_slot) = global_fallback_slot(&mut keymap_config, descriptor) else {
        return KeymapDebugBindingSource::Default;
    };
    if global_slot.is_some() {
        KeymapDebugBindingSource::CustomGlobal
    } else {
        KeymapDebugBindingSource::Default
    }
}

fn global_fallback_slot<'a>(
    keymap: &'a mut TuiKeymap,
    descriptor: &KeymapActionDescriptor,
) -> Option<&'a mut Option<KeybindingsSpec>> {
    if descriptor.context != "composer" {
        return None;
    }

    match descriptor.action {
        "submit" => Some(&mut keymap.global.submit),
        "queue" => Some(&mut keymap.global.queue),
        "toggle_shortcuts" => Some(&mut keymap.global.toggle_shortcuts),
        _ => None,
    }
}
