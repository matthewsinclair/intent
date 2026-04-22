#!/usr/bin/env bats
# Tests for intent claude rules validate against the archetype and each
# known error-class fixture under tests/fixtures/rules/.

load "../lib/test_helper.bash"

RULE_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures/rules"
# The WP01 archetype was promoted to a production rule under rules/elixir/test/
# when WP05 populated the Elixir rule pack. The path below is the canonical
# location; the test below guards that this one real rule keeps validating.
EXEMPLAR_RULE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir/test/strong-assertions/RULE.md"
EXT_FIXTURES="${INTENT_PROJECT_ROOT}/tests/fixtures/extensions"

# ====================================================================
# Happy path
# ====================================================================

@test "rules validate passes the valid fixture" {
  run run_intent claude rules validate "$RULE_FIXTURES/valid/RULE.md"
  assert_success
  assert_output_contains "1 ok"
  refute_output_contains "error:"
}

@test "rules validate passes the ext valid-ext fixture rule" {
  export INTENT_EXT_DIR="$EXT_FIXTURES"
  run run_intent claude rules validate IN-AG-EXT-001
  assert_success
  assert_output_contains "ext:valid-ext"
  assert_output_contains "1 ok"
}

# ====================================================================
# Error-class fixtures
# ====================================================================

@test "rules validate fails missing-frontmatter fixture" {
  run run_intent claude rules validate "$RULE_FIXTURES/missing-frontmatter/RULE.md"
  assert_failure
  assert_output_contains "no YAML frontmatter"
}

@test "rules validate fails bad-id fixture" {
  run run_intent claude rules validate "$RULE_FIXTURES/bad-id/RULE.md"
  assert_failure
  assert_output_contains "does not match"
}

@test "rules validate fails unresolved-reference fixture" {
  run run_intent claude rules validate "$RULE_FIXTURES/unresolved-reference/RULE.md"
  assert_failure
  assert_output_contains "does not resolve"
}

@test "rules validate warns on unknown-field fixture (warning, not error)" {
  run run_intent claude rules validate "$RULE_FIXTURES/unknown-field/RULE.md"
  assert_success
  assert_output_contains "warning: frontmatter key 'unknown_field_we_do_not_recognise'"
}

# ====================================================================
# Multi-file pass: duplicate-id detection
# ====================================================================

@test "rules validate detects duplicate ids across files" {
  # Construct a temporary ext directory holding both duplicate-id fixtures
  # under the canonical ext rule layout. `rules validate` with no argument
  # walks canon + ext, so the summary line reflects BOTH the canon rules
  # (expected to pass) and the two duplicate fixtures (expected to fail).
  local sandbox="$TEST_TEMP_DIR/ext-dup-ids"
  mkdir -p "$sandbox/ext-dup/rules/agnostic/a" "$sandbox/ext-dup/rules/agnostic/b"
  cp "$RULE_FIXTURES/duplicate-id-a/RULE.md" "$sandbox/ext-dup/rules/agnostic/a/RULE.md"
  cp "$RULE_FIXTURES/duplicate-id-b/RULE.md" "$sandbox/ext-dup/rules/agnostic/b/RULE.md"

  export INTENT_EXT_DIR="$sandbox"
  run run_intent claude rules validate
  assert_failure
  assert_output_contains "declared by more than one RULE.md"
  # Both duplicate fixtures must be flagged failed; their exact path + id
  # pair shows up in the body. The summary always reports 2 failures from
  # the fixtures regardless of how many canon rules pass alongside.
  assert_output_contains "2 failed"
}

# ====================================================================
# Exemplar: cross-reference resolves after WP04
# ====================================================================

@test "rules validate against strong-assertions exemplar passes" {
  # IN-EX-TEST-001 references IN-AG-HIGHLANDER-001. Prior to WP04 the
  # reference failed validation; after WP04 the agnostic pack defines
  # IN-AG-HIGHLANDER-001 so it resolves. This test also guards the
  # WP05 archetype-to-production promotion — the rule used to live
  # under _schema/archetype/ and was moved to its real home under
  # rules/elixir/test/ at WP05 start.
  run run_intent claude rules validate "$EXEMPLAR_RULE"
  assert_success
  assert_output_contains "IN-EX-TEST-001"
  assert_output_contains "1 ok"
  refute_output_contains "does not resolve"
}

# ====================================================================
# Error messages for bad inputs
# ====================================================================

@test "rules validate fails when given a non-existent id and non-existent path" {
  run run_intent claude rules validate "no-such-thing"
  assert_failure
  assert_output_contains "neither a readable file nor a known rule id"
}
