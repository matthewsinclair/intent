#!/usr/bin/env bats
# WP02 guard (ST0053): the CO (content) language code is admitted by the rule-id
# validator, and `content` is registered in the canon-enumeration LANG_SUBDIRS.
# A well-formed IN-CO-* id validates; a malformed one still fails. Guards the
# enum/regex widening across rule-schema.md, id-scheme.md, index-generator.md,
# and intent_claude_rules -- the same 5-site plumbing as PR/AU.

load "../lib/test_helper.bash"

VALID_FIXTURE="${INTENT_PROJECT_ROOT}/tests/fixtures/rules/valid/RULE.md"

_co_rule() {
  local id="$1" lang="$2" dest="$3"
  sed -e "s/^id: .*/id: ${id}/" -e "s/^language: .*/language: ${lang}/" \
    "$VALID_FIXTURE" > "$dest"
}

# The id is one no canon rule declares: v3 validates a file against canon's ids,
# and IN-CO-STYLE-001 is a real rule, so it is refused as a duplicate before its
# shape is ever the question.
@test "rules validate accepts a well-formed IN-CO content id" {
  local rule="$TEST_TEMP_DIR/co-valid.md"
  _co_rule "IN-CO-STYLE-901" "content" "$rule"
  run run_intent claude rules validate "$rule"
  assert_success
  assert_output_contains "1 ok"
  refute_output_contains "does not match"
}

@test "rules validate rejects a malformed IN-CO id (bad zero-padding)" {
  local rule="$TEST_TEMP_DIR/co-badpad.md"
  _co_rule "IN-CO-STYLE-1" "content" "$rule"
  run run_intent claude rules validate "$rule"
  assert_failure
  assert_output_contains "does not match"
}

@test "rules validate rejects a lowercase co language code" {
  local rule="$TEST_TEMP_DIR/co-lower.md"
  _co_rule "IN-co-STYLE-001" "content" "$rule"
  run run_intent claude rules validate "$rule"
  assert_failure
  assert_output_contains "does not match"
}

# Discovery site: validating by path is not enough -- the canon enumerator only
# walks the languages it registers, so `content` must be registered or the
# pack is invisible to `list` / `index`. Asked of the enumeration itself:
# its v3 home is `intentsvcs::rules::LANGUAGES` (the v2 `LANG_SUBDIRS` in
# rules_lib.sh went with the v2 shell).
@test "content is registered in the canon enumeration" {
  run run_intent claude rules list --lang content
  assert_success
  assert_output_contains "IN-CO-"
}
