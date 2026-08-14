#!/usr/bin/env bats
# Issue 0010: a steel thread could be closed with `## Objective` still holding
# the words the template shipped -- ie marked complete without anyone ever
# having written down what it was for -- and nothing said a word.
#
# The fix is one line of voice at the close, and it deliberately does NOT gate.
# Two things are guarded here: that the warning fires (and does not fire) in the
# right places, and that the placeholder constants have not drifted away from
# the generators that write them. That second half is the load-bearing one: if a
# template is reworded and the constants are not, the warning silently stops
# firing, which is the failure mode this whole release exists to remove.

load "../lib/test_helper.bash"

# The constants under guard, read from their one home.
load_placeholders() {
  # shellcheck disable=SC1090
  source "$INTENT_BIN_DIR/intent_helpers"
}

# Replace the shipped placeholder line with real prose. awk, not `sed -i`, to
# sidestep both the BSD/GNU -i split and escaping a bracketed literal.
set_objective() {
  local f="$1" text="$2"
  awk -v t="$text" '/^\[Clear statement of what this/ { print t; next } { print }' "$f" > "$f.tmp" \
    && mv "$f.tmp" "$f"
}

setup_thread() {
  project_dir=$(create_test_project "Objective Placeholder Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success
  st_dir="intent/st/NOT-STARTED/ST0001"
  write_exempt_acceptance "$st_dir"
}

# ---- the warning fires where it should ----

@test "st done warns when Objective is still the template placeholder" {
  setup_thread
  run run_intent st done ST0001
  assert_output_contains "'## Objective' is still the template placeholder"
  assert_output_contains "ST0001"
}

@test "st done still closes the thread despite the warning -- it is voice, not a gate" {
  setup_thread
  run run_intent st done ST0001
  assert_success
  assert_output_contains "Marked steel thread as complete"
  assert_file_exists "intent/st/COMPLETED/ST0001/info.md"
  run grep -c '^status: Completed' intent/st/COMPLETED/ST0001/info.md
  assert_output "1"
}

@test "st done names the file where it now lives, not where it was" {
  setup_thread
  run run_intent st done ST0001
  assert_output_contains "intent/st/COMPLETED/ST0001/info.md"
  refute_output_contains "NOT-STARTED/ST0001/info.md"
}

@test "wp done warns when the work package Objective is still the placeholder" {
  setup_thread
  run run_intent wp new ST0001 "First package"
  assert_success
  run run_intent wp done ST0001/01
  assert_success
  assert_output_contains "'## Objective' is still the template placeholder"
  assert_output_contains "ST0001/WP-01"
}

# ---- and stays quiet where it should ----

@test "st done is silent once the Objective says something" {
  setup_thread
  set_objective "$st_dir/info.md" "Prove the close-time warning only fires on an unedited objective."
  run run_intent st done ST0001
  assert_success
  refute_output_contains "'## Objective' is still the template placeholder"
}

@test "wp done is silent once the work package Objective says something" {
  setup_thread
  run run_intent wp new ST0001 "First package"
  assert_success
  set_objective "intent/st/NOT-STARTED/ST0001/WP/01/info.md" "Ship the enumerator."
  run run_intent wp done ST0001/01
  assert_success
  refute_output_contains "'## Objective' is still the template placeholder"
}

@test "the check is scoped to Objective -- an unedited Context section does not fire it" {
  setup_thread
  set_objective "$st_dir/info.md" "Written. The Context placeholder below is deliberately left alone."
  # Context still carries its own shipped placeholder; that is not this warning's
  # business. A sweep for any '[...]' anywhere fires on most real threads, and a
  # warning that fires on everything gets switched off.
  run grep -c 'Background information and context' "$st_dir/info.md"
  assert_output "1"
  run run_intent st done ST0001
  assert_success
  refute_output_contains "'## Objective' is still the template placeholder"
}

@test "a thread that QUOTES the placeholder outside Objective does not fire it" {
  # This is what the section scoping actually buys, and it is not hypothetical:
  # a thread about the templates themselves quotes the placeholder verbatim in
  # its prose. Reading the whole file instead of the Objective section would
  # warn about a thread whose objective is written -- so this is the test that
  # fails if the scoping is dropped for a simpler whole-file grep.
  setup_thread
  set_objective "$st_dir/info.md" "Reword the steel-thread template."
  cat >> "$st_dir/info.md" << 'EOF'

## Notes

The line we are replacing reads `[Clear statement of what this steel thread aims to accomplish]`.
EOF
  run run_intent st done ST0001
  assert_success
  refute_output_contains "'## Objective' is still the template placeholder"
}

# ---- drift guards: the constants vs every generator that writes them ----

@test "the ST placeholder constant still matches the steel-thread template" {
  load_placeholders
  [ -n "$ST_OBJECTIVE_PLACEHOLDER" ] || fail "ST_OBJECTIVE_PLACEHOLDER is unset"
  run grep -cF -- "$ST_OBJECTIVE_PLACEHOLDER" "$INTENT_HOME/lib/templates/prj/st/ST####/info.md"
  assert_output "1"
}

@test "the WP placeholder constant still matches the work-package template" {
  load_placeholders
  [ -n "$WP_OBJECTIVE_PLACEHOLDER" ] || fail "WP_OBJECTIVE_PLACEHOLDER is unset"
  run grep -cF -- "$WP_OBJECTIVE_PLACEHOLDER" "$INTENT_HOME/lib/templates/prj/st/WP/info.md"
  assert_output "1"
}

# These two used to assert that the constants still matched the no-template
# fallback heredocs, on the grounds that "the fallback is a second generator of
# the same document; if it drifts, threads born from it are invisible to the
# warning." That was the right worry and the wrong remedy: both fallbacks HAD
# drifted, and guarding a second generator only keeps two copies in step for as
# long as someone keeps running the guard. Issue 0022 deleted them, so the
# assertion inverts -- there is exactly one generator per document, and these
# pin that there is no longer a second one to drift.

@test "intent_st carries no second generator of the steel-thread Objective" {
  load_placeholders
  run grep -cF -- "$ST_OBJECTIVE_PLACEHOLDER" "$INTENT_BIN_DIR/intent_st"
  assert_output "0"
}

@test "intent_wp carries no second generator of the work-package Objective" {
  load_placeholders
  run grep -cF -- "$WP_OBJECTIVE_PLACEHOLDER" "$INTENT_BIN_DIR/intent_wp"
  assert_output "0"
}
