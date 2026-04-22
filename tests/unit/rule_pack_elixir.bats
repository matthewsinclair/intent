#!/usr/bin/env bats
# Tests for the Elixir rule pack (WP05).
#
# Guards three invariants:
#   1. Presence — every rule catalogued in WP05 exists at the expected path
#      with a well-formed RULE.md. No silent deletions.
#   2. ID uniqueness — each Elixir rule declares its canonical IN-EX-* id.
#      The validator enforces project-wide uniqueness; this file pins the
#      authoritative list so a sloppy rename cannot quietly re-point an ID.
#   3. Validator agreement — each rule passes `intent claude rules validate`.

load "../lib/test_helper.bash"

ELIXIR_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/elixir"

# ====================================================================
# Canonical rule list — update this when WP05 adds rules
# ====================================================================
#
# The list is deliberately explicit (not derived from the filesystem) so a
# contributor adding a rule has to update this test and so has to reckon
# with the invariants the test guards.

elixir_rules() {
  cat <<'EOF'
code/pattern-match-over-conditionals|IN-EX-CODE-001
code/tagged-tuple-returns|IN-EX-CODE-002
code/impl-true-on-callbacks|IN-EX-CODE-003
code/with-for-railway|IN-EX-CODE-004
code/no-silent-failures|IN-EX-CODE-005
code/module-highlander|IN-EX-CODE-006
test/strong-assertions|IN-EX-TEST-001
test/no-process-sleep|IN-EX-TEST-002
test/async-by-default|IN-EX-TEST-003
test/start-supervised|IN-EX-TEST-004
test/no-control-flow-in-tests|IN-EX-TEST-005
test/real-code-over-mocks|IN-EX-TEST-006
test/test-highlander-shared-setup|IN-EX-TEST-007
ash/code-interfaces-only|IN-EX-ASH-001
ash/actor-on-query|IN-EX-ASH-002
phoenix/thin-controllers|IN-EX-PHX-001
lv/two-phase-mount|IN-EX-LV-001
lv/streams-for-lists|IN-EX-LV-002
lv/thin-liveviews|IN-EX-LV-003
EOF
}

# ====================================================================
# Presence
# ====================================================================

@test "elixir pack: every catalogued rule exists" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_exists "$ELIXIR_ROOT/$slug/RULE.md"
  done < <(elixir_rules)
}

@test "elixir pack has the expected total rule count" {
  local expected actual
  expected=$(elixir_rules | grep -c '|' || true)
  actual=$(find "$ELIXIR_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$expected" = "$actual" ] || {
    echo "elixir rule count drift: expected $expected (test catalog), found $actual (filesystem)" >&2
    return 1
  }
}

# ====================================================================
# ID assignment
# ====================================================================

@test "elixir pack: each rule declares its canonical id" {
  while IFS='|' read -r slug id; do
    [ -z "$slug" ] && continue
    assert_file_contains "$ELIXIR_ROOT/$slug/RULE.md" "id: $id"
  done < <(elixir_rules)
}

@test "elixir pack: each rule declares language: elixir" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    assert_file_contains "$ELIXIR_ROOT/$slug/RULE.md" 'language: elixir'
  done < <(elixir_rules)
}

# ====================================================================
# Validator agreement
# ====================================================================

@test "elixir pack: each rule passes intent claude rules validate" {
  while IFS='|' read -r slug _; do
    [ -z "$slug" ] && continue
    run run_intent claude rules validate "$ELIXIR_ROOT/$slug/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done < <(elixir_rules)
}

@test "elixir pack: rules list reports every elixir id" {
  run run_intent claude rules list --lang elixir
  assert_success
  while IFS='|' read -r _ id; do
    [ -z "$id" ] && continue
    assert_output_contains "$id"
  done < <(elixir_rules)
}
