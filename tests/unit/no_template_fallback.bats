#!/usr/bin/env bats
# Issue 0022: `intent st new` and `intent wp new` carried heredoc fallbacks that
# wrote a "minimal" info.md whenever the templates could not be found. That is
# a second copy of generated content, which project rule 6 forbids for the
# reason both copies then demonstrated:
#
#   - the WP fallback wrote a `## Acceptance Criteria` section with checkboxes,
#     a form the template retired when ST0044 made acceptance.md the single
#     home -- so the shadow copy actively instructed users to do the thing the
#     real template forbids;
#   - the ST fallback wrote info.md and nothing else, so a thread born from it
#     silently lacked acceptance.md and every gate that reads it.
#
# Neither drift was noticed, because nothing keeps a shadow copy honest. A
# missing template is a broken install, so both now fail loudly and name the
# path instead of quietly substituting divergent content (No Silent Errors).

load "../lib/test_helper.bash"

# An install with bin/ but no lib/templates -- exactly the state the fallbacks
# existed to paper over.
broken_install() {
  local root="$BATS_TEST_TMPDIR/broken-install"
  rm -rf "$root"
  mkdir -p "$root/lib/templates"
  cp -R "${INTENT_PROJECT_ROOT}/bin" "$root/bin"
  echo "$root"
}

@test "st new fails loudly when the steel-thread templates are missing" {
  local project="$(create_test_project "no-template-st")"
  local broken="$(broken_install)"
  cd "$project" || exit 1

  run env INTENT_HOME="$broken" "$broken/bin/intent" st new "a thread"

  [ "$status" -ne 0 ]
  [[ "$output" == *"templates not found"* ]]
  # The path is named: a broken install is actionable only if you know where it
  # looked.
  [[ "$output" == *"lib/templates/prj/st"* ]]
}

@test "st new leaves nothing behind when it refuses" {
  local project="$(create_test_project "no-template-st-clean")"
  local broken="$(broken_install)"
  cd "$project" || exit 1

  run env INTENT_HOME="$broken" "$broken/bin/intent" st new "a thread"

  [ "$status" -ne 0 ]
  # The error claims "nothing was created". A verifier of results may not state
  # conclusions it has not checked, so check it.
  run bash -c "find '$project/intent/st' -name 'ST[0-9][0-9][0-9][0-9]' -type d | wc -l | tr -d ' '"
  assert_output "0"
}

@test "wp new fails loudly when the work-package template is missing" {
  local project="$(create_test_project "no-template-wp")"
  local broken="$(broken_install)"
  cd "$project" || exit 1
  run run_intent st new "host thread"
  assert_success

  run env INTENT_HOME="$broken" "$broken/bin/intent" wp new ST0001 "a package"

  [ "$status" -ne 0 ]
  [[ "$output" == *"template not found"* ]]
  [[ "$output" == *"lib/templates/prj/st/WP/info.md"* ]]
}

@test "wp new leaves no work-package directory behind when it refuses" {
  local project="$(create_test_project "no-template-wp-clean")"
  local broken="$(broken_install)"
  cd "$project" || exit 1
  run run_intent st new "host thread"
  assert_success

  run env INTENT_HOME="$broken" "$broken/bin/intent" wp new ST0001 "a package"

  [ "$status" -ne 0 ]
  [ ! -d "$project/intent/st/NOT-STARTED/ST0001/WP/01" ]
}

@test "a healthy install still creates all five steel-thread documents" {
  # The refusal must not have been bought by breaking the working path.
  local project="$(create_test_project "template-healthy")"
  cd "$project" || exit 1

  run run_intent st new "a thread"
  assert_success

  [ -f "intent/st/NOT-STARTED/ST0001/info.md" ]
  [ -f "intent/st/NOT-STARTED/ST0001/acceptance.md" ]
  [ -f "intent/st/NOT-STARTED/ST0001/design.md" ]
  [ -f "intent/st/NOT-STARTED/ST0001/impl.md" ]
  [ -f "intent/st/NOT-STARTED/ST0001/tasks.md" ]
}

@test "a healthy wp carries the template's Acceptance pointer, not the retired checkbox form" {
  local project="$(create_test_project "template-healthy-wp")"
  cd "$project" || exit 1
  run run_intent st new "host thread"
  assert_success
  run run_intent wp new ST0001 "a package"
  assert_success

  local wp="intent/st/NOT-STARTED/ST0001/WP/01/info.md"
  grep -q '^## Acceptance$' "$wp" || fail "WP info.md lost the template's '## Acceptance' section"
  ! grep -q '^## Acceptance Criteria' "$wp" || fail "WP info.md carries the retired checkbox form (issue 0022)"
}

@test "no info.md-shaped heredoc remains in the ST or WP creators" {
  # Mechanical: the rule is 'single template source', so grep for a second copy
  # rather than reading for one (the 0011 lesson).
  run grep -c "<< *'TEMPLATE'" "${INTENT_PROJECT_ROOT}/bin/intent_st" "${INTENT_PROJECT_ROOT}/bin/intent_wp"
  [[ "$output" != *":1"* ]] || fail "a TEMPLATE heredoc is back in a creator: $output"
}
