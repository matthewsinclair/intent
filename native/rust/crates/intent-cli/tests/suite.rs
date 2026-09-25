//! The crate's ONE integration-test target.
//!
//! Every `.rs` directly under `tests/` used to be its own autodiscovered target, and
//! so its own separately linked executable against the whole dependency graph. This
//! file is the single target they are now modules of, per hv's estate-wide ruling of
//! 2026-08-27. Laksa took it on 2026-08-27 and Lamplight partially; Intent, which made
//! the ruling, had not -- and had grown to hundreds of targets, most of them in that
//! same month.
//!
//! **THE FILES DID NOT MOVE, AND THAT IS THE WHOLE REASON THIS WAS CHEAP.** The obvious
//! consolidation relocates everything under `tests/suite/`, which breaks every
//! acceptance-test row citing a test by path -- and `intent at` has no verb that
//! retargets a row's file. `autotests = false` plus one `[[test]]` plus `#[path]` gets
//! the same single binary with every file exactly where it was, so no citation goes
//! stale because nothing it cites ever moves.
//!
//! **AND THE COST THAT IS REAL: these were separate PROCESSES and are now threads in
//! one.** Anything touching process-global state -- `set_current_dir`, `env::set_var`,
//! a fixed port, a shared socket -- stops failing cleanly and starts being flaky, which
//! is worse because it gets blamed elsewhere. Measured before the merge rather than
//! hoped for afterwards: across every file, only this one mutates process state
//! (`intent-cli/tests/dual_path_conformance.rs`, `set_current_dir` at :199), and it
//! keeps its own `[[test]]` target for that reason. No test spawns cargo, so the
//! inner-build deadlock cannot arise; the one fixed port is written to a file and
//! parsed, never bound; and every `intentd.sock` path is per-test under a tempdir.
//!
//! **A FILE ADDED UNDER `tests/` NO LONGER RUNS ON ITS OWN.** `autotests = false` sees
//! to that, so an undeclared file is silently never compiled. That inverted failure is
//! the trade, and it is guarded -- see `tests/no_orphan_suite_member.rs`.

#[path = "all_flag_reaches_every_unit.rs"]
mod all_flag_reaches_every_unit;
#[path = "at_new_takes_no_status.rs"]
mod at_new_takes_no_status;
#[path = "common/mod.rs"]
mod common;

#[path = "a_clone_without_its_boards_names_sync_to_store.rs"]
mod a_clone_without_its_boards_names_sync_to_store;
#[path = "a_close_names_what_the_next_organize_removes.rs"]
mod a_close_names_what_the_next_organize_removes;
#[path = "a_creating_verb_names_what_it_overwrote.rs"]
mod a_creating_verb_names_what_it_overwrote;
#[path = "a_doctor_acknowledgement_stays_visible_and_leaves_the_verdict.rs"]
mod a_doctor_acknowledgement_stays_visible_and_leaves_the_verdict;
#[path = "a_fresh_project_commits_clean.rs"]
mod a_fresh_project_commits_clean;
#[path = "a_long_socket_path_is_refused_before_start.rs"]
mod a_long_socket_path_is_refused_before_start;
#[path = "a_multi_line_board_item_stays_one_item.rs"]
mod a_multi_line_board_item_stays_one_item;
#[path = "a_narrowed_render_names_its_scope.rs"]
mod a_narrowed_render_names_its_scope;
#[path = "a_pull_is_reflected_by_the_next_verb.rs"]
mod a_pull_is_reflected_by_the_next_verb;
#[path = "a_pull_is_repaired_by_one_command.rs"]
mod a_pull_is_repaired_by_one_command;
#[path = "a_search_by_target_asks_one_question.rs"]
mod a_search_by_target_asks_one_question;
#[path = "a_skewed_views_printed_remedy_clears_it.rs"]
mod a_skewed_views_printed_remedy_clears_it;
#[path = "a_stale_render_names_the_verb_that_clears_it.rs"]
mod a_stale_render_names_the_verb_that_clears_it;
#[path = "a_stale_store_shows_on_a_default_doctor_run.rs"]
mod a_stale_store_shows_on_a_default_doctor_run;
#[path = "a_tool_description_comes_from_its_row.rs"]
mod a_tool_description_comes_from_its_row;
#[path = "a_write_keeps_a_cover_edit_it_could_carry.rs"]
mod a_write_keeps_a_cover_edit_it_could_carry;
#[path = "a_write_names_the_view_it_rewrote.rs"]
mod a_write_names_the_view_it_rewrote;
#[path = "ac_new_refuses_what_it_used_to_replace.rs"]
mod ac_new_refuses_what_it_used_to_replace;
#[path = "acceptance_surface.rs"]
mod acceptance_surface;
#[path = "agents_sync_parity.rs"]
mod agents_sync_parity;
#[path = "an_absent_rule_library_is_visible_at_the_process_surface.rs"]
mod an_absent_rule_library_is_visible_at_the_process_surface;
#[path = "an_address_where_a_thread_id_is_wanted_is_refused_whole.rs"]
mod an_address_where_a_thread_id_is_wanted_is_refused_whole;
#[path = "another_projects_address_is_refused_by_name.rs"]
mod another_projects_address_is_refused_by_name;
#[path = "bootstrap_door.rs"]
mod bootstrap_door;
#[path = "canon_keys_are_read.rs"]
mod canon_keys_are_read;
#[path = "canon_states_a_rule_once.rs"]
mod canon_states_a_rule_once;
#[path = "claude_cwi_door.rs"]
mod claude_cwi_door;
#[path = "cli_end_to_end.rs"]
mod cli_end_to_end;
#[path = "cli_routing.rs"]
mod cli_routing;
#[path = "cli_write_moves_only_what_changed.rs"]
mod cli_write_moves_only_what_changed;
#[path = "closing_verbs_take_a_stated_date.rs"]
mod closing_verbs_take_a_stated_date;
#[path = "command_rosters_are_derived_or_declared.rs"]
mod command_rosters_are_derived_or_declared;
#[path = "corpus_machine_independence.rs"]
mod corpus_machine_independence;
#[path = "critic_surface.rs"]
mod critic_surface;
#[path = "daemon_and_local_agree.rs"]
mod daemon_and_local_agree;
#[path = "daemon_lifecycle.rs"]
mod daemon_lifecycle;
#[path = "daemon_logs_prints_and_follows.rs"]
mod daemon_logs_prints_and_follows;
#[path = "daemon_run_execs.rs"]
mod daemon_run_execs;
#[path = "daemon_status_answers_a_machine.rs"]
mod daemon_status_answers_a_machine;
#[path = "declared_values_are_enforced.rs"]
mod declared_values_are_enforced;
#[path = "default_declaration_help.rs"]
mod default_declaration_help;
#[path = "dispatch_ssot.rs"]
mod dispatch_ssot;
#[path = "doctor_advisory_summary.rs"]
mod doctor_advisory_summary;
#[path = "edit_and_browse_reach_one_model.rs"]
mod edit_and_browse_reach_one_model;
#[path = "edit_answers_about_the_entity_you_named.rs"]
mod edit_answers_about_the_entity_you_named;
#[path = "embedded_init.rs"]
mod embedded_init;
#[path = "error_literal_shape.rs"]
mod error_literal_shape;
#[path = "events_are_queryable.rs"]
mod events_are_queryable;
#[path = "every_face_says_what_level_three_knows.rs"]
mod every_face_says_what_level_three_knows;
#[path = "every_skill_has_a_live_caller.rs"]
mod every_skill_has_a_live_caller;
#[path = "every_spawn_has_a_fixture_home.rs"]
mod every_spawn_has_a_fixture_home;
#[path = "exit_code_consumers.rs"]
mod exit_code_consumers;
#[path = "exit_codes.rs"]
mod exit_codes;
#[path = "export_command.rs"]
mod export_command;
#[path = "export_md_accepted.rs"]
mod export_md_accepted;
#[path = "flag_reachability.rs"]
mod flag_reachability;
#[path = "format_roster_is_honoured.rs"]
mod format_roster_is_honoured;
#[path = "graphql_escape_hatch.rs"]
mod graphql_escape_hatch;
#[path = "hook_compat.rs"]
mod hook_compat;
#[path = "info_exit_code.rs"]
mod info_exit_code;
#[path = "ingest_command.rs"]
mod ingest_command;
#[path = "init_from_empty_dir.rs"]
mod init_from_empty_dir;
#[path = "init_mints_an_identity.rs"]
mod init_mints_an_identity;
#[path = "init_over_existing_files.rs"]
mod init_over_existing_files;
#[path = "issue_declaration_and_organize.rs"]
mod issue_declaration_and_organize;
#[path = "issue_realised_form.rs"]
mod issue_realised_form;
#[path = "issues_add_body_door.rs"]
mod issues_add_body_door;
#[path = "issues_surface.rs"]
mod issues_surface;
#[path = "lang_surface.rs"]
mod lang_surface;
#[path = "literal_stdout_parity.rs"]
mod literal_stdout_parity;
#[path = "llm_serves_the_guide_and_the_rules.rs"]
mod llm_serves_the_guide_and_the_rules;
#[path = "mcp_bridge_restart.rs"]
mod mcp_bridge_restart;
#[path = "mcp_resources.rs"]
mod mcp_resources;
#[path = "mcp_stdio_serves.rs"]
mod mcp_stdio_serves;

#[path = "mcp_surface.rs"]
mod mcp_surface;
#[path = "migrated_guards_still_refuse.rs"]
mod migrated_guards_still_refuse;
#[path = "modules_surface.rs"]
mod modules_surface;
#[path = "no_flag_is_read_through_a_swallow.rs"]
mod no_flag_is_read_through_a_swallow;
#[path = "no_intent_home.rs"]
mod no_intent_home;
#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;
#[path = "no_pm_state_in_output.rs"]
mod no_pm_state_in_output;
#[path = "no_removal_is_unannounced.rs"]
mod no_removal_is_unannounced;
#[path = "no_skill_names_an_unshipped_verb.rs"]
mod no_skill_names_an_unshipped_verb;
#[path = "one_daemon_predicate_across_both_trees.rs"]
mod one_daemon_predicate_across_both_trees;
#[path = "the_mcp_organize_door_answers_in_project_relative_paths.rs"]
mod the_mcp_organize_door_answers_in_project_relative_paths;

#[path = "one_dispatch_home.rs"]
mod one_dispatch_home;

#[path = "one_home_for_the_ext_base.rs"]
mod one_home_for_the_ext_base;
#[path = "organize_default_declaration.rs"]
mod organize_default_declaration;
#[path = "organize_default_force_applies.rs"]
mod organize_default_force_applies;
#[path = "organize_verbosity.rs"]
mod organize_verbosity;
#[path = "plugin_surface.rs"]
mod plugin_surface;
#[path = "prefix_resolution.rs"]
mod prefix_resolution;
#[path = "register_text_carries_no_maintainer_notes.rs"]
mod register_text_carries_no_maintainer_notes;
#[path = "remedies_are_reachable.rs"]
mod remedies_are_reachable;
#[path = "retired_commands.rs"]
mod retired_commands;
#[path = "retirement_is_enumerable.rs"]
mod retirement_is_enumerable;
#[path = "routing_is_opt_in.rs"]
mod routing_is_opt_in;
#[path = "schema_command.rs"]
mod schema_command;
#[path = "schema_versioning.rs"]
mod schema_versioning;
#[path = "search_surface.rs"]
mod search_surface;
#[path = "self_loop_population.rs"]
mod self_loop_population;
#[path = "self_loop_voice.rs"]
mod self_loop_voice;
#[path = "session_hook_lockout.rs"]
mod session_hook_lockout;
#[path = "skill_cross_references_resolve.rs"]
mod skill_cross_references_resolve;
#[path = "skills_list_names_what_the_roster_does_not.rs"]
mod skills_list_names_what_the_roster_does_not;
#[path = "skills_triage_covers_the_catalogue.rs"]
mod skills_triage_covers_the_catalogue;
#[path = "spelling_notes_name_their_issue.rs"]
mod spelling_notes_name_their_issue;
#[path = "st_dehydrate_round_trips_with_hydrate.rs"]
mod st_dehydrate_round_trips_with_hydrate;
#[path = "st_edit_opens_or_prints.rs"]
mod st_edit_opens_or_prints;
#[path = "st_list_names_its_project.rs"]
mod st_list_names_its_project;
#[path = "st_list_shows_the_title.rs"]
mod st_list_shows_the_title;
#[path = "st_show_prints_the_file_it_names.rs"]
mod st_show_prints_the_file_it_names;
#[path = "subagents_sync_takes_the_preview_skills_has.rs"]
mod subagents_sync_takes_the_preview_skills_has;
#[path = "surface_is_declared.rs"]
mod surface_is_declared;
#[path = "sync_plan_and_apply_from_the_command_line.rs"]
mod sync_plan_and_apply_from_the_command_line;
#[path = "sync_to_disk_runs_beside_a_watching_daemon.rs"]
mod sync_to_disk_runs_beside_a_watching_daemon;
#[path = "sync_to_store_does_not_contradict_itself.rs"]
mod sync_to_store_does_not_contradict_itself;
#[path = "the_binary_under_test_is_the_one_cargo_built.rs"]
mod the_binary_under_test_is_the_one_cargo_built;
#[path = "the_canon_set_is_derived.rs"]
mod the_canon_set_is_derived;
#[path = "the_daemon_harness_can_restart.rs"]
mod the_daemon_harness_can_restart;
#[path = "the_daemon_takes_the_backup_itself.rs"]
mod the_daemon_takes_the_backup_itself;
#[path = "the_event_log_travels.rs"]
mod the_event_log_travels;
#[path = "the_index_says_what_it_holds.rs"]
mod the_index_says_what_it_holds;
#[path = "the_search_pane_is_resident.rs"]
mod the_search_pane_is_resident;
#[path = "the_search_tool_and_json_are_one_envelope.rs"]
mod the_search_tool_and_json_are_one_envelope;
#[path = "the_sql_door_is_read_only.rs"]
mod the_sql_door_is_read_only;
#[path = "the_struck_doors_refuse_as_retired.rs"]
mod the_struck_doors_refuse_as_retired;
#[path = "the_structural_doors_answer_in_the_envelope.rs"]
mod the_structural_doors_answer_in_the_envelope;
#[path = "the_structural_doors_take_the_filters.rs"]
mod the_structural_doors_take_the_filters;
#[path = "the_web_face_answers_on_the_published_port.rs"]
mod the_web_face_answers_on_the_published_port;
#[path = "twin_spellings_agree.rs"]
mod twin_spellings_agree;
#[path = "two_clones_that_mint_one_id_are_repaired_by_renumber.rs"]
mod two_clones_that_mint_one_id_are_repaired_by_renumber;
#[path = "uninstall_names_what_it_removed.rs"]
mod uninstall_names_what_it_removed;
#[path = "unmigrated_surface.rs"]
mod unmigrated_surface;
#[path = "upgrade_command.rs"]
mod upgrade_command;
#[path = "verbosity_flags.rs"]
mod verbosity_flags;
#[path = "version_spellings_agree.rs"]
mod version_spellings_agree;
#[path = "view_single_writer.rs"]
mod view_single_writer;
#[path = "wb_edit_door.rs"]
mod wb_edit_door;
#[path = "wb_migrate_names_its_kept_copies.rs"]
mod wb_migrate_names_its_kept_copies;
#[path = "wb_migrate_reports_the_restamp.rs"]
mod wb_migrate_reports_the_restamp;
#[path = "wb_pickup_records_the_session_it_runs_in.rs"]
mod wb_pickup_records_the_session_it_runs_in;
#[path = "wb_reads_list_live_messages.rs"]
mod wb_reads_list_live_messages;
#[path = "wb_register_correct.rs"]
mod wb_register_correct;
