#!/usr/bin/env bats
# WP01 guard (ST0053): the PR (prose) language code is admitted by the rule-id
# validator, and `prose` is registered in the canon-enumeration LANG_SUBDIRS.
# A well-formed IN-PR-* id validates; a malformed one still fails. Guards the
# enum/regex widening across rule-schema.md, id-scheme.md, index-generator.md,
# and intent_claude_rules against silent regression -- the same 5-site plumbing
# the AU code taught us (au_language_code_guard.bats).

load "../lib/test_helper.bash"

VALID_FIXTURE="${INTENT_PROJECT_ROOT}/tests/fixtures/rules/valid/RULE.md"

# Build a temp RULE.md from the valid fixture with a given id + language.
_pr_rule() {
  local id="$1" lang="$2" dest="$3"
  sed -e "s/^id: .*/id: ${id}/" -e "s/^language: .*/language: ${lang}/" \
    "$VALID_FIXTURE" > "$dest"
}

@test "rules validate accepts a well-formed IN-PR prose id" {
  local rule="$TEST_TEMP_DIR/pr-valid.md"
  _pr_rule "IN-PR-STYLE-001" "prose" "$rule"
  run run_intent claude rules validate "$rule"
  assert_success
  assert_output_contains "1 ok"
  refute_output_contains "does not match"
}

@test "rules validate rejects a malformed IN-PR id (bad zero-padding)" {
  local rule="$TEST_TEMP_DIR/pr-badpad.md"
  _pr_rule "IN-PR-STYLE-1" "prose" "$rule"
  run run_intent claude rules validate "$rule"
  assert_failure
  assert_output_contains "does not match"
}

@test "rules validate rejects a lowercase pr language code" {
  local rule="$TEST_TEMP_DIR/pr-lower.md"
  _pr_rule "IN-pr-STYLE-001" "prose" "$rule"
  run run_intent claude rules validate "$rule"
  assert_failure
  assert_output_contains "does not match"
}

# Discovery site: validating by path is not enough -- the canon enumerator only
# walks languages in LANG_SUBDIRS, so `prose` must be registered there or the
# base pack is invisible to `list` / `index`. (rules_lib.sh, not
# intent_claude_rules.)
@test "prose is registered in the canon-enumeration LANG_SUBDIRS default" {
  run grep -E 'LANG_SUBDIRS:=.*\bprose\b' "${INTENT_PROJECT_ROOT}/intent/plugins/claude/lib/rules_lib.sh"
  assert_success
}
