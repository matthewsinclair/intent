#!/usr/bin/env bats
# Tests for the agnostic rule pack (WP04).
#
# These tests guard three invariants:
#   1. Presence — all six canonical agnostic rules exist at their expected
#      filesystem paths with a well-formed RULE.md.
#   2. Frontmatter — each rule declares `language: agnostic` and carries the
#      required schema fields.
#   3. `concretised_by:` invariant — see below. The pack has TWO KINDS of rule
#      and they discharge the anti-vagueness requirement differently.
#
# CONSCIOUS UPDATE, 2026-08-30 (dc). This file asserted FOUR while SIX rules
# were on disk, and the count assertion had been red since `red-control` landed
# on 2026-08-26 — four days, because the tripwire fired and the update it asks
# for was never done. It is done here for BOTH late arrivals at once.
#
# THE ROSTERS BELOW ARE THE TRIPWIRE, AND THEY ARE STRONGER THAN THE COUNT WAS.
# A count only asks "how many"; the rosters ask "which kind", so a seventh rule
# does not merely have to be counted, it has to be CLASSIFIED before this file
# goes green. That matters because the two kinds have opposite obligations, and
# the old per-rule-by-name checks meant a new rule was governed by nothing at
# all — `red-control` sat in this pack for four days with zero `concretised_by`
# entries and no test in the suite had an opinion about it.

load "../lib/test_helper.bash"

AGNOSTIC_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/rules/agnostic"

# Shared helper: counts non-empty lines emitted by `rules list`-style helpers.
count_nonempty_lines() {
  grep -cE '^[A-Za-z0-9]' || true
}

# ====================================================================
# Presence
# ====================================================================

# PATTERN rules govern a code shape and MUST cite >=2 `concretised_by:`
# language rules. PROCEDURAL rules govern an ACTION, have no language-specific
# spelling to point at, and carry NONE — they discharge the same obligation
# through `applies_when` naming situations. Schema: `_schema/rule-schema.md`,
# prose: `intent/docs/rules.md`.
AGNOSTIC_PATTERN="highlander pfic thin-coordinator no-silent-errors"
AGNOSTIC_PROCEDURAL="red-control fiat-close-is-the-humans-verb"
AGNOSTIC_ALL="$AGNOSTIC_PATTERN $AGNOSTIC_PROCEDURAL"

@test "agnostic pack contains all six canonical rules" {
  local rule
  for rule in $AGNOSTIC_ALL; do
    assert_file_exists "$AGNOSTIC_ROOT/$rule/RULE.md"
  done
}

@test "agnostic pack has no unexpected RULE.md files" {
  # Guards against a contributor adding a fifth agnostic rule without updating
  # MODULES.md and this test. New agnostic rules require a conscious update.
  local found
  found=$(find "$AGNOSTIC_ROOT" -name 'RULE.md' -type f | wc -l | tr -d ' ')
  local declared
  declared=$(echo $AGNOSTIC_ALL | wc -w | tr -d ' ')
  [ "$found" -eq "$declared" ]
}

@test "every agnostic rule on disk is classified pattern or procedural" {
  # The tripwire proper. A new rule must be named in exactly one roster above
  # before this file goes green — being merely COUNTED is not enough, because
  # the two kinds carry opposite `concretised_by:` obligations.
  local dir name
  for dir in "$AGNOSTIC_ROOT"/*/; do
    name=$(basename "$dir")
    [ -f "$dir/RULE.md" ] || continue
    echo " $AGNOSTIC_ALL " | grep -q " $name " || {
      echo "agnostic rule '$name' is on disk and in neither roster -- classify it" >&2
      return 1
    }
  done
}

# ====================================================================
# Frontmatter shape
# ====================================================================

@test "each agnostic rule declares language: agnostic" {
  local rule
  for rule in $AGNOSTIC_ALL; do
    assert_file_contains "$AGNOSTIC_ROOT/$rule/RULE.md" 'language: agnostic'
  done
}

@test "each agnostic rule has a canonical id" {
  assert_file_contains "$AGNOSTIC_ROOT/highlander/RULE.md" 'id: IN-AG-HIGHLANDER-001'
  assert_file_contains "$AGNOSTIC_ROOT/pfic/RULE.md" 'id: IN-AG-PFIC-001'
  assert_file_contains "$AGNOSTIC_ROOT/thin-coordinator/RULE.md" 'id: IN-AG-THIN-COORD-001'
  assert_file_contains "$AGNOSTIC_ROOT/no-silent-errors/RULE.md" 'id: IN-AG-NO-SILENT-001'
  assert_file_contains "$AGNOSTIC_ROOT/red-control/RULE.md" 'id: IN-AG-RED-CONTROL-001'
  assert_file_contains "$AGNOSTIC_ROOT/fiat-close-is-the-humans-verb/RULE.md" 'id: IN-AG-FIAT-001'
}

@test "each agnostic rule has severity critical" {
  local rule
  for rule in $AGNOSTIC_ALL; do
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

count_concretised_by() {
  awk '
    /^concretised_by:[[:space:]]*$/ { inside=1; next }
    inside == 1 && /^[[:space:]]+-[[:space:]]/ { print; next }
    inside == 1 && /^[A-Za-z]/ { inside=0 }
  ' "$1" | wc -l | tr -d ' '
}

@test "every PATTERN agnostic rule concretises in at least two language packs" {
  local rule
  for rule in $AGNOSTIC_PATTERN; do
    assert_concretised_by_at_least_two "$AGNOSTIC_ROOT/$rule/RULE.md"
  done
}

@test "every PROCEDURAL agnostic rule carries no concretised_by" {
  # The other half of the clause, and it did not exist before 2026-08-30. The
  # old tests named four rules one at a time, so a rule outside that list was
  # governed by NOTHING -- which is how `red-control` sat here for four days
  # with zero entries and nothing objected. Asserting the zero makes the
  # exception a contract rather than a gap: a procedural rule that grows a
  # `concretised_by:` has become a pattern rule and must move roster.
  local rule items
  for rule in $AGNOSTIC_PROCEDURAL; do
    items=$(count_concretised_by "$AGNOSTIC_ROOT/$rule/RULE.md")
    [ "$items" -eq 0 ] || {
      echo "procedural rule '$rule' carries $items concretised_by items, expected 0" >&2
      return 1
    }
  done
}

@test "every PROCEDURAL agnostic rule names situations in applies_when" {
  # What a procedural rule discharges the anti-vagueness requirement WITH.
  # Structural only: this asserts the entries exist, never that they are good.
  local rule items
  for rule in $AGNOSTIC_PROCEDURAL; do
    items=$(awk '
      /^applies_when:[[:space:]]*$/ { inside=1; next }
      inside == 1 && /^[[:space:]]+-[[:space:]]/ { print; next }
      inside == 1 && /^[A-Za-z]/ { inside=0 }
    ' "$AGNOSTIC_ROOT/$rule/RULE.md" | wc -l | tr -d ' ')
    [ "$items" -ge 2 ] || {
      echo "procedural rule '$rule' has $items applies_when entries, expected >=2" >&2
      return 1
    }
  done
}

# ====================================================================
# Cross-check: validator agrees every agnostic rule is well-formed
# ====================================================================

@test "all six agnostic rules pass intent claude rules validate" {
  local rule
  for rule in $AGNOSTIC_ALL; do
    run run_intent claude rules validate "$AGNOSTIC_ROOT/$rule/RULE.md"
    assert_success
    assert_output_contains "1 ok"
  done
}

@test "rules list reports all six agnostic rules" {
  run run_intent claude rules list --lang agnostic
  assert_success
  assert_output_contains "IN-AG-HIGHLANDER-001"
  assert_output_contains "IN-AG-PFIC-001"
  assert_output_contains "IN-AG-THIN-COORD-001"
  assert_output_contains "IN-AG-NO-SILENT-001"
  assert_output_contains "IN-AG-RED-CONTROL-001"
  assert_output_contains "IN-AG-FIAT-001"
}
