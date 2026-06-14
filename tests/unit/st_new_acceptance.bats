#!/usr/bin/env bats
# ST0044: acceptance.md as a first-class steel-thread doc, plus the WP-07 guard.
#
# WP-05 (red-first -- both tests failed until the production change landed):
# - AT-05.1: the info.md / WP info.md templates must reference acceptance.md
#   (the AC home) and must NOT restate ACs. The shipped WP template still
#   carried a "## Acceptance Criteria" / "- [ ]" section, so it failed before.
# - AT-05.2: st show <id> acceptance displays it, st show <id> all lists it,
#   and st edit <id> acceptance prints the file's absolute path (no editor).
#   show/edit hardcoded info|design|impl|tasks before, so `acceptance` errored.
#
# WP-07 (green-by-construction guard, NOT red-first):
# - AT-07.1: the two open steel threads adopting the AC/AT process (ST0043 and
#   ST0044) each carry an acceptance.md. The invariant already held when this
#   landed, so there was no honest red phase -- it stands guard against either
#   thread losing its contract.
# @test names are cited by AT-05.1 / AT-05.2 / AT-07.1 in intent/st/ST0044/acceptance.md.

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

@test "open STs ST0043 and ST0044 each have an acceptance.md" {
  # Green-by-construction guard (WP-07 dogfood). The two open steel threads that
  # adopt the AC/AT process must each carry an acceptance.md. The invariant
  # already holds as this lands, so there is no honest red phase -- this stands
  # guard against either thread losing its contract, it is not a red-first lap.
  # Globbed across both layouts so it survives st done relocating a thread from
  # st/ST0044/ to st/COMPLETED/ST0044/ -- the contract file travels with it.
  shopt -s nullglob
  for st in ST0043 ST0044; do
    hits=( "${INTENT_PROJECT_ROOT}"/intent/st/"${st}"/acceptance.md \
           "${INTENT_PROJECT_ROOT}"/intent/st/*/"${st}"/acceptance.md )
    [ "${#hits[@]}" -ge 1 ] || fail "expected an acceptance.md for open ST ${st}, found none"
  done
}
