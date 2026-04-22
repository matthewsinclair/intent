#!/usr/bin/env bats
# Tests for the agnostic rule pack (WP04).
#
# These tests guard three invariants:
#   1. Presence — all four canonical agnostic rules exist at their expected
#      filesystem paths with a well-formed RULE.md.
#   2. Frontmatter — each rule declares `language: agnostic` and carries the
#      required schema fields.
#   3. `concretised_by:` invariant — each agnostic rule lists at least two
#      language-specific rule IDs. This is what stops `rules/agnostic/` from
#      becoming a dumping ground for vague wisdom; every principle must have
#      at least two concrete language-side demonstrations.

load "../lib/test_helper.bash"

AGNOSTIC_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/agnostic"

# Shared helper: counts non-empty lines emitted by `rules list`-style helpers.
count_nonempty_lines() {
  grep -cE '^[A-Za-z0-9]' || true
}

# ====================================================================
# Presence
# ====================================================================

@test "agnostic pack contains all four canonical rules" {
  assert_file_exists "$AGNOSTIC_ROOT/highlander/RULE.md"
  assert_file_exists "$AGNOSTIC_ROOT/pfic/RULE.md"
  assert_file_exists "$AGNOSTIC_ROOT/thin-coordinator/RULE.md"
  assert_file_exists "$AGNOSTIC_ROOT/no-silent-errors/RULE.md"
}

@test "agnostic pack has no unexpected RULE.md files" {
  # Guards against a contributor adding a fifth agnostic rule without updating
  # MODULES.md and this test. New agnostic rules require a conscious update.
  local found
  found=$(find "$AGNOSTIC_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  [ "$found" -eq 4 ]
}

# ====================================================================
# Frontmatter shape
# ====================================================================

@test "each agnostic rule declares language: agnostic" {
  local rule
  for rule in highlander pfic thin-coordinator no-silent-errors; do
    assert_file_contains "$AGNOSTIC_ROOT/$rule/RULE.md" 'language: agnostic'
  done
}

@test "each agnostic rule has a canonical id" {
  assert_file_contains "$AGNOSTIC_ROOT/highlander/RULE.md" 'id: IN-AG-HIGHLANDER-001'
  assert_file_contains "$AGNOSTIC_ROOT/pfic/RULE.md" 'id: IN-AG-PFIC-001'
  assert_file_contains "$AGNOSTIC_ROOT/thin-coordinator/RULE.md" 'id: IN-AG-THIN-COORD-001'
  assert_file_contains "$AGNOSTIC_ROOT/no-silent-errors/RULE.md" 'id: IN-AG-NO-SILENT-001'
}

@test "each agnostic rule has severity critical" {
  local rule
  for rule in highlander pfic thin-coordinator no-silent-errors; do
    assert_file_contains "$AGNOSTIC_ROOT/$rule/RULE.md" 'severity: critical'
  done
}

# ====================================================================
# `concretised_by:` invariant
# ====================================================================
#
# Each agnostic rule must list at least two language-specific rule IDs under
# `concretised_by:`. The IDs themselves may be forward references to rules
# that land in WP05 or WP06 — the validator does not enforce resolution on
# `concretised_by:`, only on `references:`.

assert_concretised_by_at_least_two() {
  local rule_path="$1"
  # Extract the block form of concretised_by and count indented list items.
  # Block form looks like:
  #   concretised_by:
  #     - IN-EX-CODE-006
  #     - IN-EX-TEST-004
  local items
  items=$(awk '
    /^concretised_by:[[:space:]]*$/ { inside=1; next }
    inside == 1 && /^[[:space:]]+-[[:space:]]/ { print; next }
    inside == 1 && /^[A-Za-z]/ { inside=0 }
  ' "$rule_path" | wc -l | tr -d ' ')
  [ "$items" -ge 2 ] || {
    echo "expected >=2 concretised_by items in $rule_path, got $items" >&2
    return 1
  }
}

@test "highlander rule concretises in at least two language packs" {
  assert_concretised_by_at_least_two "$AGNOSTIC_ROOT/highlander/RULE.md"
}

@test "pfic rule concretises in at least two language packs" {
  assert_concretised_by_at_least_two "$AGNOSTIC_ROOT/pfic/RULE.md"
}

@test "thin-coordinator rule concretises in at least two language packs" {
  assert_concretised_by_at_least_two "$AGNOSTIC_ROOT/thin-coordinator/RULE.md"
}

@test "no-silent-errors rule concretises in at least two language packs" {
  assert_concretised_by_at_least_two "$AGNOSTIC_ROOT/no-silent-errors/RULE.md"
}

# ====================================================================
# Cross-check: validator agrees every agnostic rule is well-formed
# ====================================================================

@test "all four agnostic rules pass intent claude rules validate" {
  run run_intent claude rules validate "$AGNOSTIC_ROOT/highlander/RULE.md"
  assert_success
  assert_output_contains "1 ok"

  run run_intent claude rules validate "$AGNOSTIC_ROOT/pfic/RULE.md"
  assert_success
  assert_output_contains "1 ok"

  run run_intent claude rules validate "$AGNOSTIC_ROOT/thin-coordinator/RULE.md"
  assert_success
  assert_output_contains "1 ok"

  run run_intent claude rules validate "$AGNOSTIC_ROOT/no-silent-errors/RULE.md"
  assert_success
  assert_output_contains "1 ok"
}

@test "rules list reports all four agnostic rules" {
  run run_intent claude rules list --lang agnostic
  assert_success
  assert_output_contains "IN-AG-HIGHLANDER-001"
  assert_output_contains "IN-AG-PFIC-001"
  assert_output_contains "IN-AG-THIN-COORD-001"
  assert_output_contains "IN-AG-NO-SILENT-001"
}
