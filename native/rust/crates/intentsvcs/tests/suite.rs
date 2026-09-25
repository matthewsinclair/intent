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

#[path = "canon_commit_guard_names_a_held_edit.rs"]
mod canon_commit_guard_names_a_held_edit;
#[path = "common/mod.rs"]
mod common;

#[path = "a_cold_warm_leaves_a_peers_write.rs"]
mod a_cold_warm_leaves_a_peers_write;
#[path = "a_create_refuses_a_child_id_that_is_taken.rs"]
mod a_create_refuses_a_child_id_that_is_taken;
#[path = "a_create_refuses_a_key_that_is_taken.rs"]
mod a_create_refuses_a_key_that_is_taken;
#[path = "a_facade_catches_up_with_a_peers_commit.rs"]
mod a_facade_catches_up_with_a_peers_commit;
#[path = "a_fresh_clone_takes_its_boards.rs"]
mod a_fresh_clone_takes_its_boards;
#[path = "a_mutation_does_not_duplicate_the_prose_index.rs"]
mod a_mutation_does_not_duplicate_the_prose_index;
#[path = "a_re_cite_keeps_what_it_was_not_given.rs"]
mod a_re_cite_keeps_what_it_was_not_given;
#[path = "a_resolved_reference_joins_a_written_one.rs"]
mod a_resolved_reference_joins_a_written_one;
#[path = "a_retire_reaches_the_consumer.rs"]
mod a_retire_reaches_the_consumer;
#[path = "a_scheduled_backup_is_the_same_call.rs"]
mod a_scheduled_backup_is_the_same_call;
#[path = "a_scoped_refresh_repairs_an_index_its_delete_damaged.rs"]
mod a_scoped_refresh_repairs_an_index_its_delete_damaged;
#[path = "a_search_names_what_it_could_not_answer.rs"]
mod a_search_names_what_it_could_not_answer;
#[path = "a_stale_store_does_not_overwrite_committed_canon.rs"]
mod a_stale_store_does_not_overwrite_committed_canon;
#[path = "a_store_fault_is_not_a_bad_query.rs"]
mod a_store_fault_is_not_a_bad_query;
#[path = "a_subkind_is_a_word_the_index_writes.rs"]
mod a_subkind_is_a_word_the_index_writes;
#[path = "a_sync_writes_no_render_the_store_has_moved_past.rs"]
mod a_sync_writes_no_render_the_store_has_moved_past;
#[path = "a_view_an_older_renderer_wrote_is_not_a_hand_edit.rs"]
mod a_view_an_older_renderer_wrote_is_not_a_hand_edit;
#[path = "a_work_packages_criteria_are_its_scoped_rows.rs"]
mod a_work_packages_criteria_are_its_scoped_rows;
#[path = "a_write_refuses_a_record_that_moved_under_it.rs"]
mod a_write_refuses_a_record_that_moved_under_it;
#[path = "a_write_waits_for_the_lock_rather_than_refusing.rs"]
mod a_write_waits_for_the_lock_rather_than_refusing;
#[path = "absent_manifest_is_not_unreadable.rs"]
mod absent_manifest_is_not_unreadable;
#[path = "ac_kind_state_invariant.rs"]
mod ac_kind_state_invariant;
#[path = "address_collections_resolve.rs"]
mod address_collections_resolve;
#[path = "address_empty_authority.rs"]
mod address_empty_authority;
#[path = "address_format_set.rs"]
mod address_format_set;
#[path = "address_md_is_the_hydrated_bytes.rs"]
mod address_md_is_the_hydrated_bytes;
#[path = "address_promotes_a_bare_id.rs"]
mod address_promotes_a_bare_id;
#[path = "address_resolution_single_home.rs"]
mod address_resolution_single_home;
#[path = "address_views_have_no_url.rs"]
mod address_views_have_no_url;
#[path = "an_issue_body_can_be_corrected.rs"]
mod an_issue_body_can_be_corrected;
#[path = "an_unreadable_index_table_is_named_and_the_canon_is_not_blamed.rs"]
mod an_unreadable_index_table_is_named_and_the_canon_is_not_blamed;
#[path = "an_unreadable_index_table_is_rebuilt_in_place.rs"]
mod an_unreadable_index_table_is_rebuilt_in_place;
#[path = "attachment_cap.rs"]
mod attachment_cap;
#[path = "attachment_carry.rs"]
mod attachment_carry;
#[path = "attachment_drift_detected.rs"]
mod attachment_drift_detected;
#[path = "attachment_form_by_content.rs"]
mod attachment_form_by_content;
#[path = "attachment_naming_gate.rs"]
mod attachment_naming_gate;
#[path = "attachment_path_must_name_a_place_in_the_thread.rs"]
mod attachment_path_must_name_a_place_in_the_thread;
#[path = "attachment_put_refuses_what_it_cannot_carry.rs"]
mod attachment_put_refuses_what_it_cannot_carry;
#[path = "authored_row_round_trip.rs"]
mod authored_row_round_trip;
#[path = "authored_sections_have_one_form.rs"]
mod authored_sections_have_one_form;
#[path = "backup_retention.rs"]
mod backup_retention;
#[path = "backup_snapshot.rs"]
mod backup_snapshot;
#[path = "canon_holds_a_settings_file_it_did_not_write.rs"]
mod canon_holds_a_settings_file_it_did_not_write;
#[path = "canon_preserves_what_it_did_not_write.rs"]
mod canon_preserves_what_it_did_not_write;
#[path = "canon_relocation.rs"]
mod canon_relocation;
#[path = "canon_relocation_roundtrip.rs"]
mod canon_relocation_roundtrip;
#[path = "canon_resolver_singularity.rs"]
mod canon_resolver_singularity;
#[path = "canon_round_trip.rs"]
mod canon_round_trip;
#[path = "canon_seeds_the_critic_config_as_the_template_holds_it.rs"]
mod canon_seeds_the_critic_config_as_the_template_holds_it;
#[path = "canon_seeds_the_mcp_declaration_once.rs"]
mod canon_seeds_the_mcp_declaration_once;
#[path = "canon_seeds_usage_rules_with_the_project_name.rs"]
mod canon_seeds_usage_rules_with_the_project_name;
#[path = "carrier_is_installed_beside_the_block.rs"]
mod carrier_is_installed_beside_the_block;
#[path = "close_and_verdict_verbs_enforce_what_they_promise.rs"]
mod close_and_verdict_verbs_enforce_what_they_promise;
#[path = "close_gate_parity.rs"]
mod close_gate_parity;
#[path = "contention_wait_is_chosen.rs"]
mod contention_wait_is_chosen;
#[path = "critic_refuses_an_empty_library.rs"]
mod critic_refuses_an_empty_library;
#[path = "daemon_address.rs"]
mod daemon_address;
#[path = "daemon_descriptor_hygiene.rs"]
mod daemon_descriptor_hygiene;
#[path = "daemon_health_splits_stale_from_absent.rs"]
mod daemon_health_splits_stale_from_absent;
#[path = "daemon_pid_is_only_read_under_the_lock.rs"]
mod daemon_pid_is_only_read_under_the_lock;
#[path = "daemon_probe_contract.rs"]
mod daemon_probe_contract;
#[path = "db_stamps_the_record.rs"]
mod db_stamps_the_record;
#[path = "declarable_languages_derive_from_one_list.rs"]
mod declarable_languages_derive_from_one_list;
#[path = "default_declaration_has_one_definition.rs"]
mod default_declaration_has_one_definition;
#[path = "dehydration_ship_gate.rs"]
mod dehydration_ship_gate;
#[path = "dep_graph_guard.rs"]
mod dep_graph_guard;
#[path = "dependency_rationale.rs"]
mod dependency_rationale;
#[path = "doctor_advisory.rs"]
mod doctor_advisory;
#[path = "init_keeps_the_store_out_of_git.rs"]
mod init_keeps_the_store_out_of_git;

#[path = "doctor_checks.rs"]
mod doctor_checks;
#[path = "doctor_reads_the_search_index.rs"]
mod doctor_reads_the_search_index;
#[path = "doctor_reports_a_board_row_its_file_lacks.rs"]
mod doctor_reports_a_board_row_its_file_lacks;
#[path = "doctor_reports_a_root_file_behind_its_template.rs"]
mod doctor_reports_a_root_file_behind_its_template;
#[path = "doctor_scope.rs"]
mod doctor_scope;

#[path = "edit_prints_a_path_that_exists.rs"]
mod edit_prints_a_path_that_exists;
#[path = "egest_estate.rs"]
mod egest_estate;
#[path = "egest_refuses_to_empty_the_estate.rs"]
mod egest_refuses_to_empty_the_estate;
#[path = "elixir_references_resolve_through_the_compiler_tracer.rs"]
mod elixir_references_resolve_through_the_compiler_tracer;
#[path = "error_remedies.rs"]
mod error_remedies;
#[path = "event_log_envelopes.rs"]
mod event_log_envelopes;
#[path = "every_declared_form_resolves_to_an_entity.rs"]
mod every_declared_form_resolves_to_an_entity;
#[path = "every_key_the_runner_reads_is_declared.rs"]
mod every_key_the_runner_reads_is_declared;
#[path = "every_st_op_has_a_declared_list_answer.rs"]
mod every_st_op_has_a_declared_list_answer;
#[path = "explore_lands_where_the_address_names.rs"]
mod explore_lands_where_the_address_names;
#[path = "export_round_trip.rs"]
mod export_round_trip;
#[path = "facade_acceptance.rs"]
mod facade_acceptance;
#[path = "facade_dehydrate.rs"]
mod facade_dehydrate;
#[path = "facade_hydrate.rs"]
mod facade_hydrate;
#[path = "facade_st_wp.rs"]
mod facade_st_wp;
#[path = "fiat_close_is_stamped_by_the_database.rs"]
mod fiat_close_is_stamped_by_the_database;
#[path = "fiat_close_is_visible_on_every_surface.rs"]
mod fiat_close_is_visible_on_every_surface;
#[path = "fiat_state_serde.rs"]
mod fiat_state_serde;
#[path = "finding_remedies.rs"]
mod finding_remedies;
#[path = "flush_is_one_transaction.rs"]
mod flush_is_one_transaction;
#[path = "form_declares_layout_not_the_field_set.rs"]
mod form_declares_layout_not_the_field_set;
#[path = "gate_not_running_is_reported.rs"]
mod gate_not_running_is_reported;
#[path = "generated_views_are_not_formatted.rs"]
mod generated_views_are_not_formatted;
#[path = "generated_views_are_unaddressable.rs"]
mod generated_views_are_unaddressable;
#[path = "graphql_face_agrees.rs"]
mod graphql_face_agrees;
#[path = "graphql_reads_through_the_facade.rs"]
mod graphql_reads_through_the_facade;
#[path = "ignored_paths_corpus.rs"]
mod ignored_paths_corpus;
#[path = "info_round_trip.rs"]
mod info_round_trip;
#[path = "info_round_trip_reaches_the_store.rs"]
mod info_round_trip_reaches_the_store;
#[path = "ingest_refusal.rs"]
mod ingest_refusal;
#[path = "intentfiles_default_declaration.rs"]
mod intentfiles_default_declaration;
#[path = "intentfiles_grammar.rs"]
mod intentfiles_grammar;
#[path = "intentfiles_is_the_list.rs"]
mod intentfiles_is_the_list;
#[path = "intentfiles_names_artefacts_only.rs"]
mod intentfiles_names_artefacts_only;
#[path = "issue_estate.rs"]
mod issue_estate;
#[path = "legacy_at_citation_annotation.rs"]
mod legacy_at_citation_annotation;
#[path = "legacy_at_status_annotation.rs"]
mod legacy_at_status_annotation;
#[path = "legacy_at_unbalanced_bracket.rs"]
mod legacy_at_unbalanced_bracket;
#[path = "legacy_at_without_a_subject.rs"]
mod legacy_at_without_a_subject;
#[path = "legacy_bucket_attachments.rs"]
mod legacy_bucket_attachments;
#[path = "legacy_bucketed_residue.rs"]
mod legacy_bucketed_residue;
#[path = "legacy_citation_and_prose_covers.rs"]
mod legacy_citation_and_prose_covers;
#[path = "legacy_covers_plus.rs"]
mod legacy_covers_plus;
#[path = "legacy_covers_token_rule.rs"]
mod legacy_covers_token_rule;
#[path = "legacy_document_conservation.rs"]
mod legacy_document_conservation;
#[path = "legacy_leftovers.rs"]
mod legacy_leftovers;
#[path = "legacy_marker_and_descope.rs"]
mod legacy_marker_and_descope;
#[path = "legacy_row_accounting.rs"]
mod legacy_row_accounting;
#[path = "legacy_satisfied_verdict.rs"]
mod legacy_satisfied_verdict;
#[path = "legacy_scope_carry.rs"]
mod legacy_scope_carry;
#[path = "legacy_unread_field.rs"]
mod legacy_unread_field;
#[path = "legacy_vocabulary.rs"]
mod legacy_vocabulary;
#[path = "lifecycle_verbs_edit_the_list.rs"]
mod lifecycle_verbs_edit_the_list;
#[path = "mandatory_fields_reach_a_reader.rs"]
mod mandatory_fields_reach_a_reader;
#[path = "marker_has_one_home.rs"]
mod marker_has_one_home;
#[path = "migrate_hooks_continuity.rs"]
mod migrate_hooks_continuity;
#[path = "migrate_refusal.rs"]
mod migrate_refusal;
#[path = "migrate_v2_project.rs"]
mod migrate_v2_project;
#[path = "migration_dirty_check_reads_content.rs"]
mod migration_dirty_check_reads_content;
#[path = "migrator_determinism.rs"]
mod migrator_determinism;
#[path = "migrator_population_is_canon.rs"]
mod migrator_population_is_canon;
#[path = "model_laws.rs"]
mod model_laws;
#[path = "mutation_completeness.rs"]
mod mutation_completeness;
#[path = "mutation_create_splits_two_ways.rs"]
mod mutation_create_splits_two_ways;
#[path = "mutation_creates_criteria_and_tests.rs"]
mod mutation_creates_criteria_and_tests;
#[path = "mutation_every_writable_field.rs"]
mod mutation_every_writable_field;
#[path = "mutation_put_format_by_authorship.rs"]
mod mutation_put_format_by_authorship;
#[path = "mutation_roundtrip_complete.rs"]
mod mutation_roundtrip_complete;
#[path = "nav_is_the_shared_path_contract.rs"]
mod nav_is_the_shared_path_contract;
#[path = "no_function_takes_a_time.rs"]
mod no_function_takes_a_time;
#[path = "rekinding_a_criterion_re_enters_its_state.rs"]
mod rekinding_a_criterion_re_enters_its_state;
#[path = "rules_validate_reaches_an_ext_pack.rs"]
mod rules_validate_reaches_an_ext_pack;
#[path = "rust_references_resolve_through_rust_analyzer.rs"]
mod rust_references_resolve_through_rust_analyzer;
#[path = "search_answers_one_envelope.rs"]
mod search_answers_one_envelope;
#[path = "symbols_come_from_the_grammars_own_tags.rs"]
mod symbols_come_from_the_grammars_own_tags;
#[path = "sync_ingest_takes_only_what_the_store_did_not_write.rs"]
mod sync_ingest_takes_only_what_the_store_did_not_write;
#[path = "test_target_topology_guard.rs"]
mod test_target_topology_guard;
#[path = "the_source_tokeniser_is_measured.rs"]
mod the_source_tokeniser_is_measured;
#[path = "upgrade_dehydrates_undeclared_thread_views.rs"]
mod upgrade_dehydrates_undeclared_thread_views;

#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;
#[path = "no_view_claims_to_be_truth.rs"]
mod no_view_claims_to_be_truth;
#[path = "one_clock.rs"]
mod one_clock;
#[path = "one_definition_is_one_symbol_row.rs"]
mod one_definition_is_one_symbol_row;
#[path = "op_roster_and_the_live_log.rs"]
mod op_roster_and_the_live_log;
#[path = "opaque_attachment_canon.rs"]
mod opaque_attachment_canon;
#[path = "openness.rs"]
mod openness;
#[path = "operator_id_spellings.rs"]
mod operator_id_spellings;
#[path = "organize_attachment_divergence.rs"]
mod organize_attachment_divergence;
#[path = "organize_dehydration_gate.rs"]
mod organize_dehydration_gate;
#[path = "organize_five_rows.rs"]
mod organize_five_rows;
#[path = "organize_idempotent_mtime.rs"]
mod organize_idempotent_mtime;
#[path = "organize_moment_of_act_digest.rs"]
mod organize_moment_of_act_digest;
#[path = "organize_preview_polarity.rs"]
mod organize_preview_polarity;
#[path = "organize_prunes_what_it_emptied.rs"]
mod organize_prunes_what_it_emptied;
#[path = "output_shape.rs"]
mod output_shape;
#[path = "pin_writes_to_the_list.rs"]
mod pin_writes_to_the_list;
#[path = "preamble_conservation.rs"]
mod preamble_conservation;
#[path = "prose_ingest_fts.rs"]
mod prose_ingest_fts;
#[path = "realisation_is_recorded.rs"]
mod realisation_is_recorded;
#[path = "record_timestamps.rs"]
mod record_timestamps;
#[path = "refused_ingest_blocks_egest.rs"]
mod refused_ingest_blocks_egest;
#[path = "related_links.rs"]
mod related_links;
#[path = "related_links_have_their_own_verbs.rs"]
mod related_links_have_their_own_verbs;
#[path = "remedy_coverage.rs"]
mod remedy_coverage;
#[path = "renumber_moves_an_id_and_what_names_it.rs"]
mod renumber_moves_an_id_and_what_names_it;
#[path = "replacing_the_prose_index_truncates_it.rs"]
mod replacing_the_prose_index_truncates_it;
#[path = "retired_st_prefix.rs"]
mod retired_st_prefix;
#[path = "root_files_generated.rs"]
mod root_files_generated;
#[path = "schema_faces_drift.rs"]
mod schema_faces_drift;

#[path = "a_pulled_board_is_taken_only_when_the_store_is_still.rs"]
mod a_pulled_board_is_taken_only_when_the_store_is_still;
#[path = "skills_sync.rs"]
mod skills_sync;
#[path = "status_gate_states_what_it_observed.rs"]
mod status_gate_states_what_it_observed;
#[path = "status_vocabulary.rs"]
mod status_vocabulary;
#[path = "store_rebuild.rs"]
mod store_rebuild;
#[path = "store_round_trip.rs"]
mod store_round_trip;
#[path = "store_schema_version.rs"]
mod store_schema_version;
#[path = "subagents_payload.rs"]
mod subagents_payload;
#[path = "symbols_answer_the_highlander_question.rs"]
mod symbols_answer_the_highlander_question;
#[path = "symbols_are_qualified_elixir_references.rs"]
mod symbols_are_qualified_elixir_references;
#[path = "symbols_are_qualified_references.rs"]
mod symbols_are_qualified_references;
#[path = "symbols_are_typed_definitions.rs"]
mod symbols_are_typed_definitions;
#[path = "sync_direction.rs"]
mod sync_direction;
#[path = "sync_overwrite_names_a_diverged_attachment.rs"]
mod sync_overwrite_names_a_diverged_attachment;
#[path = "sync_reports_uncommitted_attachment.rs"]
mod sync_reports_uncommitted_attachment;
#[path = "sync_scan.rs"]
mod sync_scan;
#[path = "sync_scope.rs"]
mod sync_scope;
#[path = "sync_to_disk_materialises_a_canon_authored_attachment.rs"]
mod sync_to_disk_materialises_a_canon_authored_attachment;
#[path = "sync_to_store_keeps_board_rows.rs"]
mod sync_to_store_keeps_board_rows;
#[path = "template_pins.rs"]
mod template_pins;
#[path = "text_realisation.rs"]
mod text_realisation;
#[path = "the_backup_cycle_has_one_home.rs"]
mod the_backup_cycle_has_one_home;
#[path = "the_editor_gets_the_model_bytes.rs"]
mod the_editor_gets_the_model_bytes;
#[path = "the_index_is_built_from_the_tree_and_read_from_the_store.rs"]
mod the_index_is_built_from_the_tree_and_read_from_the_store;
#[path = "the_index_scope_is_the_repository.rs"]
mod the_index_scope_is_the_repository;
#[path = "the_migrator_says_what_it_did_not_carry.rs"]
mod the_migrator_says_what_it_did_not_carry;
#[path = "the_semantic_tier_is_staged_and_its_seams_hold.rs"]
mod the_semantic_tier_is_staged_and_its_seams_hold;
#[path = "the_shell_page_reads_the_fields_the_wire_sends.rs"]
mod the_shell_page_reads_the_fields_the_wire_sends;
#[path = "thread_body_conservation.rs"]
mod thread_body_conservation;
#[path = "thread_prose_carried.rs"]
mod thread_prose_carried;
#[path = "todo_watermark.rs"]
mod todo_watermark;
#[path = "triples_are_the_shared_derivation.rs"]
mod triples_are_the_shared_derivation;
#[path = "unmigrated_project.rs"]
mod unmigrated_project;
#[path = "unparsed_state.rs"]
mod unparsed_state;
#[path = "unpin_removes_from_the_list.rs"]
mod unpin_removes_from_the_list;
#[path = "unsatisfied_note_serde.rs"]
mod unsatisfied_note_serde;
#[path = "view_determinism.rs"]
mod view_determinism;
#[path = "view_skew_check.rs"]
mod view_skew_check;
#[path = "wb_archive_frees_the_bound.rs"]
mod wb_archive_frees_the_bound;
#[path = "wb_edit_keeps_the_old_text_out_of_a_commit.rs"]
mod wb_edit_keeps_the_old_text_out_of_a_commit;
#[path = "wb_migrate_carries_a_board.rs"]
mod wb_migrate_carries_a_board;
#[path = "wb_pickup_states_the_session.rs"]
mod wb_pickup_states_the_session;
#[path = "wb_register_names_a_node.rs"]
mod wb_register_names_a_node;
#[path = "wb_views_are_generated.rs"]
mod wb_views_are_generated;
#[path = "wb_writes_are_recorded.rs"]
mod wb_writes_are_recorded;
#[path = "wbmigrate_reads_a_board.rs"]
mod wbmigrate_reads_a_board;
#[path = "wp_prose_roundtrip.rs"]
mod wp_prose_roundtrip;
#[path = "wp_scaffolding_drop.rs"]
mod wp_scaffolding_drop;
#[path = "write_moves_only_what_changed.rs"]
mod write_moves_only_what_changed;
#[path = "write_path_canon_always.rs"]
mod write_path_canon_always;
#[path = "write_refuses_to_empty_an_authored_body.rs"]
mod write_refuses_to_empty_an_authored_body;
#[path = "write_set_rollback.rs"]
mod write_set_rollback;
