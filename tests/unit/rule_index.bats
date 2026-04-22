#!/usr/bin/env bats
# Tests for intent claude rules index - deterministic JSON generation.

load "../lib/test_helper.bash"

CANON_INDEX="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/index.json"

# Back up the canon index.json before tests and restore on teardown, so
# running the test suite doesn't mutate committed content.
setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  if [ -f "$CANON_INDEX" ]; then
    cp "$CANON_INDEX" "$TEST_TEMP_DIR/index.backup.json"
  fi
}

teardown() {
  if [ -f "$TEST_TEMP_DIR/index.backup.json" ]; then
    cp "$TEST_TEMP_DIR/index.backup.json" "$CANON_INDEX"
  fi
  rm -rf "$TEST_TEMP_DIR"
}

@test "rules index regenerates index.json" {
  run run_intent claude rules index
  assert_success
  assert_output_contains "ok: indexed"
  assert_file_exists "$CANON_INDEX"
}

@test "rules index is deterministic - running twice produces byte-identical output" {
  run run_intent claude rules index
  assert_success
  cp "$CANON_INDEX" "$TEST_TEMP_DIR/run1.json"

  run run_intent claude rules index
  assert_success
  cp "$CANON_INDEX" "$TEST_TEMP_DIR/run2.json"

  run diff -q "$TEST_TEMP_DIR/run1.json" "$TEST_TEMP_DIR/run2.json"
  assert_success
}

@test "rules index output declares the v1 schema" {
  run run_intent claude rules index
  assert_success
  assert_file_contains "$CANON_INDEX" '"schema"'
  assert_file_contains "$CANON_INDEX" 'intent-rule-index/v1'
}

@test "rules index populates intent_version from VERSION" {
  run run_intent claude rules index
  assert_success
  assert_file_contains "$CANON_INDEX" '"intent_version"'
}

@test "rules index populates upstream_pin from attribution file" {
  run run_intent claude rules index
  assert_success
  # Upstream pin is the elixir-test-critic commit hash recorded at WP01
  assert_file_contains "$CANON_INDEX" '1d9aa40700dab7370b4abd338ce11b922e914b14'
}

@test "rules index includes the four agnostic rules after WP04" {
  # Canon had only _schema/ and _attribution/ during WP02; WP04 lands the
  # agnostic rule pack (Highlander, PFIC, Thin Coordinator, No Silent Errors).
  # The index should now report exactly those four canon rules and no others
  # until WP05/WP06 add language packs.
  run run_intent claude rules index
  assert_success
  assert_file_contains "$CANON_INDEX" '"rule_count": 4'
  assert_file_contains "$CANON_INDEX" '"IN-AG-HIGHLANDER-001"'
  assert_file_contains "$CANON_INDEX" '"IN-AG-PFIC-001"'
  assert_file_contains "$CANON_INDEX" '"IN-AG-THIN-COORD-001"'
  assert_file_contains "$CANON_INDEX" '"IN-AG-NO-SILENT-001"'
}

@test "rules index does not include ext rules (extensions are runtime-only)" {
  # Even when an ext fixture contributes a rule, the canon index.json must
  # not include it — extensions are discovered at runtime by list/show/validate,
  # never baked into the canon index.
  export INTENT_EXT_DIR="${INTENT_PROJECT_ROOT}/tests/fixtures/extensions"
  run run_intent claude rules index
  assert_success
  run grep "IN-AG-EXT-001" "$CANON_INDEX"
  assert_failure
}

@test "rules index sorts keys for stable diff" {
  run run_intent claude rules index
  assert_success
  # jq -S produces alphabetically sorted keys at the top level
  local first_key
  first_key=$(head -2 "$CANON_INDEX" | tail -1 | sed 's/[^a-zA-Z_]//g')
  [ "$first_key" = "intentversion" ] || [ "$first_key" = "intent_version" ]
}
