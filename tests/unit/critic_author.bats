#!/usr/bin/env bats
# Tests for the critic-author subagent (ST0052 WP03). AT-03.1 -- covers AC-03.1
# (the subagent exists, declares a read-only tool loadout, and is registered).
# Also backstops the non-test ACs: AC-03.2 (two-tier review/craft-check contract)
# and AC-03.3 (two-form detrope wiring -- mechanical STYLE-005 default, CRAFT-003
# /in-detrope handoff, never invoked).

load "../lib/test_helper.bash"

CRITIC_DIR="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/critic-author"
AGENT_MD="$CRITIC_DIR/agent.md"
METADATA="$CRITIC_DIR/metadata.json"
MANIFEST="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/.manifest/global-agents.json"

# ====================================================================
# Presence + registration (AC-03.1)
# ====================================================================

@test "critic-author: directory has agent.md and metadata.json" {
  [ -d "$CRITIC_DIR" ]
  assert_file_exists "$AGENT_MD"
  assert_file_exists "$METADATA"
}

@test "critic-author: agent.md frontmatter declares name critic-author" {
  assert_file_contains "$AGENT_MD" 'name: critic-author'
}

@test "critic-author: registered in global-agents.json" {
  jq -r '.agents[].name' "$MANIFEST" | grep -qx "critic-author"
}

@test "critic-author: metadata.json is valid JSON naming critic-author" {
  run jq -r '.name' "$METADATA"
  assert_success
  assert_output "critic-author"
}

# ====================================================================
# Read-only tool loadout (AC-03.1: no Write/Edit -- critics report only)
# ====================================================================

@test "critic-author: agent.md grants Read, Grep, Glob, Bash" {
  run grep -E '^tools:' "$AGENT_MD"
  assert_success
  assert_output_contains "Read"
  assert_output_contains "Grep"
  assert_output_contains "Glob"
  assert_output_contains "Bash"
}

@test "critic-author: agent.md grants neither Write nor Edit" {
  run grep -E '^tools:' "$AGENT_MD"
  assert_success
  refute_output_contains "Write"
  refute_output_contains "Edit"
}

@test "critic-author: metadata.json tools are read-only" {
  run jq -r '.tools | index("Write") // empty, index("Edit") // empty' "$METADATA"
  assert_success
  assert_output ""
}

# ====================================================================
# Two-tier contract (AC-03.2): review (style) + craft-check (craft)
# ====================================================================

@test "critic-author: agent.md defines both mode verbs" {
  assert_file_contains "$AGENT_MD" 'review'
  assert_file_contains "$AGENT_MD" 'craft-check'
}

@test "critic-author: agent.md names both tiers (style mechanical, craft judgment)" {
  assert_file_contains "$AGENT_MD" 'category: style'
  assert_file_contains "$AGENT_MD" 'category: craft'
}

# ====================================================================
# Two-form detrope (AC-03.3): mechanical default + /in-detrope handoff
# ====================================================================

@test "critic-author: agent.md wires the mechanical trope pass (STYLE-005)" {
  assert_file_contains "$AGENT_MD" 'IN-AU-STYLE-005'
  assert_file_contains "$AGENT_MD" 'trope-catalog.md'
}

@test "critic-author: agent.md emits the CRAFT-003 /in-detrope handoff, not an invocation" {
  assert_file_contains "$AGENT_MD" 'IN-AU-CRAFT-003'
  assert_file_contains "$AGENT_MD" '/in-detrope'
  # The critic must state it does NOT run the skill itself (handoff only).
  run grep -iE 'does NOT (run|invoke).*(skill|/in-detrope)|never invokes' "$AGENT_MD"
  assert_success
}
