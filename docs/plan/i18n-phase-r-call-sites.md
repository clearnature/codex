# Phase R 调用点重放作战图（批 2 主体）
基准：ours=f9fd6c1dee（merge 的实际 ours）；清单=our 侧显式引用 codex_i18n 而工作树无引用的文件。
数量：178 个（批 1 词典/基础文件已交付 f5a267000b）

## 分布
     27 src/bottom_pane
     24 src/chatwidget
     21 src/app
     12 src/request_processors
     10 src/history_cell
      7 src/session
      6 src/tools
      4 src/onboarding
      4 src/guardian
      3 src/keymap_setup
      3 src/external_agent_config_migration
      3 src/config
      3 src/agent
      2 src/unified_exec
      2 src/exec_cell
      1 src/worktree_startup.rs
      1 src/update_prompt.rs
      1 src/unarchive_prompt.rs
      1 src/ui_consts.rs
      1 src/tooltips.rs
      1 src/thread_transcript.rs
      1 src/thread_manager.rs
      1 src/theme_picker.rs
      1 src/tasks
      1 src/status_indicator_widget.rs
      1 src/status
      1 src/startup_orchestration.rs
      1 src/startup_hooks_review.rs
      1 src/startup_draft.rs
      1 src/session_start.rs
      1 src/session_queue_commands.rs
      1 src/session_archive_commands.rs
      1 src/rmcp_client.rs
      1 src/resume_picker.rs
      1 src/render
      1 src/remote_control_cmd.rs
      1 src/plugin_id.rs
      1 src/plugin_cmd.rs
      1 src/permission_discovery.rs
      1 src/pager_overlay.rs
      1 src/oss_selection.rs
      1 src/multi_agents.rs
      1 src/model_migration.rs
      1 src/mcp_cmd.rs
      1 src/login.rs
      1 src/lib.rs
      1 src/keymap_setup.rs
      1 src/keymap.rs
      1 src/keymap
      1 src/dict_zh_hant.rs
      1 src/cwd_prompt.rs
      1 src/connection_manager
      1 src/config_toml.rs
      1 src/config_manager_service.rs
      1 src/compact.rs
      1 src/cloud_config.rs
      1 src/clipboard_paste.rs
      1 src/clipboard_copy.rs
      1 src/bespoke_event_handling.rs
      1 src/app_server_session.rs
      1 src/app_backtrack.rs
      1 cli/src

## 清单
codex-rs/app-server/src/bespoke_event_handling.rs
codex-rs/app-server/src/config_manager_service.rs
codex-rs/app-server/src/lib.rs
codex-rs/app-server/src/request_processors/account_processor/bedrock_setup.rs
codex-rs/app-server/src/request_processors/account_processor/rate_limit_resets.rs
codex-rs/app-server/src/request_processors/account_processor.rs
codex-rs/app-server/src/request_processors/catalog_processor.rs
codex-rs/app-server/src/request_processors/command_exec_processor.rs
codex-rs/app-server/src/request_processors/feedback_processor.rs
codex-rs/app-server/src/request_processors/mcp_processor.rs
codex-rs/app-server/src/request_processors/thread_delete.rs
codex-rs/app-server/src/request_processors/thread_lifecycle.rs
codex-rs/app-server/src/request_processors/thread_processor.rs
codex-rs/app-server/src/request_processors/turn_processor.rs
codex-rs/app-server/src/request_processors/windows_sandbox_processor.rs
codex-rs/cli/src/cloud_config.rs
codex-rs/cli/src/login.rs
codex-rs/cli/src/mcp_cmd.rs
codex-rs/cli/src/plugin_cmd.rs
codex-rs/cli/src/remote_control_cmd.rs
codex-rs/codex-mcp/src/connection_manager/startup.rs
codex-rs/codex-mcp/src/rmcp_client.rs
codex-rs/config/src/config_toml.rs
codex-rs/core/src/agent/control.rs
codex-rs/core/src/agent/control/spawn.rs
codex-rs/core/src/agent/registry.rs
codex-rs/core/src/compact.rs
codex-rs/core/src/config/mod.rs
codex-rs/core/src/config/network_proxy_spec.rs
codex-rs/core/src/config/permissions.rs
codex-rs/core/src/guardian/assessment.rs
codex-rs/core/src/guardian/decision.rs
codex-rs/core/src/guardian/review.rs
codex-rs/core/src/guardian/runtime.rs
codex-rs/core/src/session/environment.rs
codex-rs/core/src/session/handlers.rs
codex-rs/core/src/session/session.rs
codex-rs/core/src/session/thread_settings.rs
codex-rs/core/src/session/time_reminder.rs
codex-rs/core/src/session/turn.rs
codex-rs/core/src/session/world_state.rs
codex-rs/core/src/tasks/mod.rs
codex-rs/core/src/thread_manager.rs
codex-rs/core/src/tools/approvals.rs
codex-rs/core/src/tools/handlers/current_time.rs
codex-rs/core/src/tools/handlers/sleep.rs
codex-rs/core/src/tools/network_approval.rs
codex-rs/core/src/tools/orchestrator.rs
codex-rs/core/src/tools/runtimes/unified_exec.rs
codex-rs/core/src/unified_exec/process_manager.rs
codex-rs/core/src/unified_exec/stdin_approval.rs
codex-rs/i18n/src/dict_zh_hant.rs
codex-rs/plugin/src/plugin_id.rs
codex-rs/tui/src/app/agents_overview_details.rs
codex-rs/tui/src/app/agents_overview.rs
codex-rs/tui/src/app/agents_overview_view.rs
codex-rs/tui/src/app/app_server_requests.rs
codex-rs/tui/src/app/background_requests.rs
codex-rs/tui/src/app_backtrack.rs
codex-rs/tui/src/app/config_persistence.rs
codex-rs/tui/src/app/event_dispatch.rs
codex-rs/tui/src/app/exit_summary.rs
codex-rs/tui/src/app/experimental_features.rs
codex-rs/tui/src/app/input.rs
codex-rs/tui/src/app/managed_worktree_creation.rs
codex-rs/tui/src/app/permission_shortcuts.rs
codex-rs/tui/src/app/pets.rs
codex-rs/tui/src/app/recap.rs
codex-rs/tui/src/app/reconnect.rs
codex-rs/tui/src/app_server_session.rs
codex-rs/tui/src/app/session_lifecycle.rs
codex-rs/tui/src/app/side.rs
codex-rs/tui/src/app/startup.rs
codex-rs/tui/src/app/thread_routing.rs
codex-rs/tui/src/app/transcript_export.rs
codex-rs/tui/src/app/working_directory.rs
codex-rs/tui/src/bottom_pane/app_link_view.rs
codex-rs/tui/src/bottom_pane/approval_overlay.rs
codex-rs/tui/src/bottom_pane/async_questions/render.rs
codex-rs/tui/src/bottom_pane/async_questions/state.rs
codex-rs/tui/src/bottom_pane/chat_composer/history_search.rs
codex-rs/tui/src/bottom_pane/experimental_features_view.rs
codex-rs/tui/src/bottom_pane/feedback_note_view.rs
codex-rs/tui/src/bottom_pane/feedback_view.rs
codex-rs/tui/src/bottom_pane/file_search_popup.rs
codex-rs/tui/src/bottom_pane/footer.rs
codex-rs/tui/src/bottom_pane/hooks_browser_view.rs
codex-rs/tui/src/bottom_pane/list_selection_view.rs
codex-rs/tui/src/bottom_pane/mcp_server_elicitation.rs
codex-rs/tui/src/bottom_pane/memories_settings_view.rs
codex-rs/tui/src/bottom_pane/mentions_v2/footer.rs
codex-rs/tui/src/bottom_pane/mod.rs
codex-rs/tui/src/bottom_pane/multi_select_picker.rs
codex-rs/tui/src/bottom_pane/pending_input_preview.rs
codex-rs/tui/src/bottom_pane/popup_consts.rs
codex-rs/tui/src/bottom_pane/questions.rs
codex-rs/tui/src/bottom_pane/request_user_input/mod.rs
codex-rs/tui/src/bottom_pane/selection_popup_common.rs
codex-rs/tui/src/bottom_pane/selection_row_layout.rs
codex-rs/tui/src/bottom_pane/skill_popup.rs
codex-rs/tui/src/bottom_pane/skills_toggle_view.rs
codex-rs/tui/src/bottom_pane/status_line_setup.rs
codex-rs/tui/src/bottom_pane/title_setup.rs
codex-rs/tui/src/chatwidget/connectors.rs
codex-rs/tui/src/chatwidget/interaction.rs
codex-rs/tui/src/chatwidget/model_popups.rs
codex-rs/tui/src/chatwidget/permission_discovery.rs
codex-rs/tui/src/chatwidget/permission_popups.rs
codex-rs/tui/src/chatwidget/permissions_menu.rs
codex-rs/tui/src/chatwidget/plugin_catalog.rs
codex-rs/tui/src/chatwidget/plugins.rs
codex-rs/tui/src/chatwidget/rate_limits.rs
codex-rs/tui/src/chatwidget/reasoning_shortcuts.rs
codex-rs/tui/src/chatwidget/reconnect.rs
codex-rs/tui/src/chatwidget/rendering.rs
codex-rs/tui/src/chatwidget/reset_credits.rs
codex-rs/tui/src/chatwidget/review_popups.rs
codex-rs/tui/src/chatwidget/session_flow.rs
codex-rs/tui/src/chatwidget/settings_popups.rs
codex-rs/tui/src/chatwidget/skills.rs
codex-rs/tui/src/chatwidget/status_surfaces.rs
codex-rs/tui/src/chatwidget/tests/slash_commands.rs
codex-rs/tui/src/chatwidget/tokens/chart.rs
codex-rs/tui/src/chatwidget/turn_runtime.rs
codex-rs/tui/src/chatwidget/usage.rs
codex-rs/tui/src/chatwidget/windows_sandbox_prompts.rs
codex-rs/tui/src/chatwidget/worktree_picker.rs
codex-rs/tui/src/clipboard_copy.rs
codex-rs/tui/src/clipboard_paste.rs
codex-rs/tui/src/cwd_prompt.rs
codex-rs/tui/src/exec_cell/live_output.rs
codex-rs/tui/src/exec_cell/render.rs
codex-rs/tui/src/external_agent_config_migration/flow.rs
codex-rs/tui/src/external_agent_config_migration/render.rs
codex-rs/tui/src/external_agent_config_migration/source.rs
codex-rs/tui/src/history_cell/hook_cell.rs
codex-rs/tui/src/history_cell/mcp_result.rs
codex-rs/tui/src/history_cell/mcp.rs
codex-rs/tui/src/history_cell/notices.rs
codex-rs/tui/src/history_cell/patches.rs
codex-rs/tui/src/history_cell/plans.rs
codex-rs/tui/src/history_cell/request_user_input.rs
codex-rs/tui/src/history_cell/separators.rs
codex-rs/tui/src/history_cell/session.rs
codex-rs/tui/src/history_cell/startup_warnings.rs
codex-rs/tui/src/keymap/chords.rs
codex-rs/tui/src/keymap.rs
codex-rs/tui/src/keymap_setup/actions.rs
codex-rs/tui/src/keymap_setup/capture.rs
codex-rs/tui/src/keymap_setup/picker.rs
codex-rs/tui/src/keymap_setup.rs
codex-rs/tui/src/model_migration.rs
codex-rs/tui/src/multi_agents.rs
codex-rs/tui/src/onboarding/auth.rs
codex-rs/tui/src/onboarding/bedrock.rs
codex-rs/tui/src/onboarding/trust_directory.rs
codex-rs/tui/src/onboarding/welcome.rs
codex-rs/tui/src/oss_selection.rs
codex-rs/tui/src/pager_overlay.rs
codex-rs/tui/src/permission_discovery.rs
codex-rs/tui/src/render/highlight.rs
codex-rs/tui/src/resume_picker.rs
codex-rs/tui/src/session_archive_commands.rs
codex-rs/tui/src/session_queue_commands.rs
codex-rs/tui/src/session_start.rs
codex-rs/tui/src/startup_draft.rs
codex-rs/tui/src/startup_hooks_review.rs
codex-rs/tui/src/startup_orchestration.rs
codex-rs/tui/src/status/card.rs
codex-rs/tui/src/status_indicator_widget.rs
codex-rs/tui/src/theme_picker.rs
codex-rs/tui/src/thread_transcript.rs
codex-rs/tui/src/tooltips.rs
codex-rs/tui/src/ui_consts.rs
codex-rs/tui/src/unarchive_prompt.rs
codex-rs/tui/src/update_prompt.rs
codex-rs/tui/src/worktree_startup.rs
codex-rs/utils/cli/src/shared_options.rs
