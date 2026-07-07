#!/usr/bin/env bats
# Tests for the /in-content-essentials skill (ST0053 WP05). AT-05.1 -- covers
# AC-05.1: the skill exists with valid frontmatter, carries the content
# pipeline, references the prose base + content rule ids, and is renderer-safe.

load "../lib/test_helper.bash"

SKILL="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-content-essentials/SKILL.md"

@test "in-content-essentials skill exists" {
  [ -f "$SKILL" ]
}

@test "in-content-essentials declares description and chains_to frontmatter" {
  assert_file_contains "$SKILL" "description:"
  assert_file_contains "$SKILL" "chains_to:"
  assert_file_contains "$SKILL" "in-detrope"
}

@test "in-content-essentials carries the content pipeline steps" {
  assert_file_contains "$SKILL" "Draft"
  assert_file_contains "$SKILL" "Mechanical detrope"
  assert_file_contains "$SKILL" "Revise"
  assert_file_contains "$SKILL" "Structural check"
  assert_file_contains "$SKILL" "CTA"
}

@test "in-content-essentials references the prose base + content rules by ID" {
  local id
  # Prose base (shared) + content-specific style + craft tiers.
  for id in IN-PR-STYLE-001 IN-PR-STYLE-002 IN-PR-STYLE-003 IN-PR-STYLE-004 \
            IN-CO-STYLE-001 IN-CO-STYLE-002 IN-CO-STYLE-003 \
            IN-CO-CRAFT-001 IN-CO-CRAFT-002 IN-CO-CRAFT-003; do
    assert_file_contains "$SKILL" "$id"
  done
}

@test "in-content-essentials dispatches critic-prose for the mechanical pass" {
  assert_file_contains "$SKILL" "trope-catalog.md"
  assert_file_contains "$SKILL" "/in-detrope"
  assert_file_contains "$SKILL" 'subagent_type="critic-prose"'
}

# Renderer-safety: no em dashes (list-display truncation bug) and no $N
# positional tokens (skill-renderer stripping).
@test "in-content-essentials is renderer-safe (no em dash, no positional tokens)" {
  run grep -nF "—" "$SKILL"
  [ "$status" -ne 0 ]
  run grep -nE '\$([0-9]|@|#|\*|\?|!|\{[0-9@#*]\})' "$SKILL"
  [ "$status" -ne 0 ]
}
