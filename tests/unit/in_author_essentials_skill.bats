#!/usr/bin/env bats
# Tests for the /in-author-essentials skill (ST0052 WP05). AT-05.1 -- covers
# AC-05.1: the skill exists with valid frontmatter, carries the authoring
# pipeline, references the nine IN-AU-* rule ids, and is renderer-safe.

load "../lib/test_helper.bash"

SKILL="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-author-essentials/SKILL.md"

@test "in-author-essentials skill exists" {
  [ -f "$SKILL" ]
}

@test "in-author-essentials declares description and chains_to frontmatter" {
  assert_file_contains "$SKILL" "description:"
  assert_file_contains "$SKILL" "chains_to:"
  assert_file_contains "$SKILL" "in-detrope"
}

@test "in-author-essentials carries the authoring pipeline steps" {
  assert_file_contains "$SKILL" "Outline"
  assert_file_contains "$SKILL" "Draft"
  assert_file_contains "$SKILL" "Mechanical detrope"
  assert_file_contains "$SKILL" "Revise"
  assert_file_contains "$SKILL" "Structural check"
}

@test "in-author-essentials references the prose base + author rules by ID" {
  local id
  # Prose base (shared, ST0053 WP01) + the author-specific style rule + craft tier.
  for id in IN-PR-STYLE-001 IN-PR-STYLE-002 IN-PR-STYLE-003 IN-PR-STYLE-004 \
            IN-AU-STYLE-003 \
            IN-AU-CRAFT-001 IN-AU-CRAFT-002 IN-AU-CRAFT-003 IN-AU-CRAFT-004; do
    assert_file_contains "$SKILL" "$id"
  done
}

@test "in-author-essentials wires the two-form detrope (mechanical default + /in-detrope handoff)" {
  assert_file_contains "$SKILL" "trope-catalog.md"
  assert_file_contains "$SKILL" "/in-detrope"
  assert_file_contains "$SKILL" 'subagent_type="critic-author"'
}

# Renderer-safety: no em dashes (list-display truncation bug) and no $N
# positional tokens (skill-renderer stripping). The global skill_renderer_trap
# guard covers $N across all skills; this pins both locally for the new skill.
@test "in-author-essentials is renderer-safe (no em dash, no positional tokens)" {
  run grep -nF "—" "$SKILL"
  [ "$status" -ne 0 ]
  run grep -nE '\$([0-9]|@|#|\*|\?|!|\{[0-9@#*]\})' "$SKILL"
  [ "$status" -ne 0 ]
}
