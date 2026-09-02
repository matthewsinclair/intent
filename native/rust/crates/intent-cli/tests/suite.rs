//! The crate's ONE integration-test target.
//!
//! Every `.rs` directly under `tests/` used to be its own autodiscovered target, and
//! so its own separately linked executable against the whole dependency graph. This
//! file is the single target they are now modules of, per hv's estate-wide ruling of
//! 2026-08-27. Laksa took it on 2026-08-27 and Lamplight partially; Intent, which made
//! the ruling, had not -- and had grown to 257 targets, 201 of them in that same month.
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
//! hoped for afterwards: across all 257 files exactly ONE mutates process state
//! (`intent-cli/tests/dual_path_conformance.rs`, `set_current_dir` at :199), and it
//! keeps its own `[[test]]` target for that reason. No test spawns cargo, so the
//! inner-build deadlock cannot arise; the one fixed port is written to a file and
//! parsed, never bound; and every `intentd.sock` path is per-test under a tempdir.
//!
//! **A FILE ADDED UNDER `tests/` NO LONGER RUNS ON ITS OWN.** `autotests = false` sees
//! to that, so an undeclared file is silently never compiled. That inverted failure is
//! the trade, and it is guarded -- see `tests/no_orphan_suite_member.rs`.

#[path = "common/mod.rs"]
mod common;

#[path = "a_narrowed_render_names_its_scope.rs"]
mod a_narrowed_render_names_its_scope;
#[path = "ac_new_refuses_what_it_used_to_replace.rs"]
mod ac_new_refuses_what_it_used_to_replace;
#[path = "acceptance_surface.rs"]
mod acceptance_surface;
#[path = "agents_sync_parity.rs"]
mod agents_sync_parity;
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
#[path = "critic_refuses_an_empty_library_end_to_end.rs"]
mod critic_refuses_an_empty_library_end_to_end;
#[path = "critic_surface.rs"]
mod critic_surface;
#[path = "daemon_and_local_agree.rs"]
mod daemon_and_local_agree;
#[path = "daemon_lifecycle.rs"]
mod daemon_lifecycle;
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
#[path = "edit_answers_about_the_entity_you_named.rs"]
mod edit_answers_about_the_entity_you_named;
#[path = "embedded_init.rs"]
mod embedded_init;
#[path = "error_literal_shape.rs"]
mod error_literal_shape;
#[path = "events_are_queryable.rs"]
mod events_are_queryable;
#[path = "every_skill_has_a_live_caller.rs"]
mod every_skill_has_a_live_caller;
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
#[path = "one_dispatch_home.rs"]
mod one_dispatch_home;
#[path = "organize_default_declaration.rs"]
mod organize_default_declaration;
#[path = "organize_default_force_applies.rs"]
mod organize_default_force_applies;
#[path = "plugin_surface.rs"]
mod plugin_surface;
#[path = "prefix_resolution.rs"]
mod prefix_resolution;
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
#[path = "spelling_notes_name_their_issue.rs"]
mod spelling_notes_name_their_issue;
#[path = "st_dehydrate_round_trips_with_hydrate.rs"]
mod st_dehydrate_round_trips_with_hydrate;
#[path = "st_edit_opens_or_prints.rs"]
mod st_edit_opens_or_prints;
#[path = "st_list_shows_the_title.rs"]
mod st_list_shows_the_title;
#[path = "sync_to_store_does_not_contradict_itself.rs"]
mod sync_to_store_does_not_contradict_itself;
#[path = "table_driven_tests_fixture_their_home.rs"]
mod table_driven_tests_fixture_their_home;
#[path = "the_binary_under_test_is_the_one_cargo_built.rs"]
mod the_binary_under_test_is_the_one_cargo_built;
#[path = "the_canon_set_is_derived.rs"]
mod the_canon_set_is_derived;
#[path = "the_daemon_harness_can_restart.rs"]
mod the_daemon_harness_can_restart;
#[path = "the_daemon_takes_the_backup_itself.rs"]
mod the_daemon_takes_the_backup_itself;
#[path = "the_web_face_answers_on_the_published_port.rs"]
mod the_web_face_answers_on_the_published_port;
#[path = "twin_spellings_agree.rs"]
mod twin_spellings_agree;
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
