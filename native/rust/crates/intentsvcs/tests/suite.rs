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

#[path = "a_create_refuses_a_child_id_that_is_taken.rs"]
mod a_create_refuses_a_child_id_that_is_taken;
#[path = "a_create_refuses_a_key_that_is_taken.rs"]
mod a_create_refuses_a_key_that_is_taken;
#[path = "a_mutation_does_not_duplicate_the_prose_index.rs"]
mod a_mutation_does_not_duplicate_the_prose_index;
#[path = "a_re_cite_keeps_what_it_was_not_given.rs"]
mod a_re_cite_keeps_what_it_was_not_given;
#[path = "a_scheduled_backup_is_the_same_call.rs"]
mod a_scheduled_backup_is_the_same_call;
#[path = "a_write_refuses_a_record_that_moved_under_it.rs"]
mod a_write_refuses_a_record_that_moved_under_it;
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
#[path = "attachment_put_refuses_what_it_cannot_carry.rs"]
mod attachment_put_refuses_what_it_cannot_carry;
#[path = "authored_row_round_trip.rs"]
mod authored_row_round_trip;
#[path = "backup_retention.rs"]
mod backup_retention;
#[path = "backup_snapshot.rs"]
mod backup_snapshot;
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
#[path = "carrier_is_installed_beside_the_block.rs"]
mod carrier_is_installed_beside_the_block;
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
#[path = "doctor_checks.rs"]
mod doctor_checks;
#[path = "edit_prints_a_path_that_exists.rs"]
mod edit_prints_a_path_that_exists;
#[path = "egest_estate.rs"]
mod egest_estate;
#[path = "egest_refuses_to_empty_the_estate.rs"]
mod egest_refuses_to_empty_the_estate;
#[path = "error_remedies.rs"]
mod error_remedies;
#[path = "event_log_envelopes.rs"]
mod event_log_envelopes;
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
#[path = "migrate_hooks_continuity.rs"]
mod migrate_hooks_continuity;
#[path = "migrate_refusal.rs"]
mod migrate_refusal;
#[path = "migrate_v2_project.rs"]
mod migrate_v2_project;
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
#[path = "no_orphan_suite_member.rs"]
mod no_orphan_suite_member;
#[path = "no_view_claims_to_be_truth.rs"]
mod no_view_claims_to_be_truth;
#[path = "one_clock.rs"]
mod one_clock;
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
#[path = "remedy_coverage.rs"]
mod remedy_coverage;
#[path = "replacing_the_prose_index_truncates_it.rs"]
mod replacing_the_prose_index_truncates_it;
#[path = "retired_st_prefix.rs"]
mod retired_st_prefix;
#[path = "root_files_generated.rs"]
mod root_files_generated;
#[path = "schema_faces_drift.rs"]
mod schema_faces_drift;
#[path = "skills_sync.rs"]
mod skills_sync;
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
#[path = "sync_direction.rs"]
mod sync_direction;
#[path = "sync_reports_uncommitted_attachment.rs"]
mod sync_reports_uncommitted_attachment;
#[path = "sync_scan.rs"]
mod sync_scan;
#[path = "sync_scope.rs"]
mod sync_scope;
#[path = "template_pins.rs"]
mod template_pins;
#[path = "text_realisation.rs"]
mod text_realisation;
#[path = "the_backup_cycle_has_one_home.rs"]
mod the_backup_cycle_has_one_home;
#[path = "the_editor_gets_the_model_bytes.rs"]
mod the_editor_gets_the_model_bytes;
#[path = "the_migrator_says_what_it_did_not_carry.rs"]
mod the_migrator_says_what_it_did_not_carry;
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
