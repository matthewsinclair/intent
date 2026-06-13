#!/usr/bin/env bats
# ST0044 WP-05: acceptance.md is a first-class steel-thread doc.
#
# Red-first: both tests fail until the production change lands.
# - AT-05.1: the info.md / WP info.md templates must reference acceptance.md
#   (the AC home) and must NOT restate ACs. The shipped WP template still
#   carries a "## Acceptance Criteria" / "- [ ]" section, so it fails today.
# - AT-05.2: st show <id> acceptance displays it, st show <id> all lists it,
#   and st edit <id> acceptance prints the file's absolute path (no editor).
#   show/edit hardcode info|design|impl|tasks today, so `acceptance` errors.
# @test names are cited by AT-05.1 / AT-05.2 in intent/st/ST0044/acceptance.md.

load "../lib/test_helper.bash"

@test "info templates reference acceptance.md and restate no ACs" {
  project_dir=$(create_test_project "WP05 Template Refs"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Ref Thread"; assert_success
  run run_intent wp new ST0001 "Build it"; assert_success

  st_info="intent/st/NOT-STARTED/ST0001/info.md"
  wp_info="intent/st/NOT-STARTED/ST0001/WP/01/info.md"

  # Both info templates point at acceptance.md as the AC home (Highlander).
  assert_file_contains "$st_info" "acceptance.md"
  assert_file_contains "$wp_info" "acceptance.md"

  # No restated ACs: the WP template no longer ships hand-ticked AC checkboxes.
  run grep -qE '^- \[ \]' "$wp_info"
  assert_failure
}

@test "st show and edit know the acceptance file type" {
  project_dir=$(create_test_project "WP05 Show Edit"); cd "$project_dir"; export EDITOR=echo
  run run_intent st new "Doc Thread"; assert_success

  # show <id> acceptance displays acceptance.md.
  run run_intent st show ST0001 acceptance
  assert_success
  assert_output_contains "Acceptance Criteria"

  # show <id> all includes acceptance.md in the doc-set listing.
  run run_intent st show ST0001 all
  assert_output_contains "-- acceptance.md"

  # edit <id> acceptance prints the file's absolute path (no editor launch).
  run run_intent st edit ST0001 acceptance
  assert_success
  assert_output_contains "/ST0001/acceptance.md"
}
