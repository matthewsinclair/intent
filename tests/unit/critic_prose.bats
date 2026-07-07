#!/usr/bin/env bats
# Tests for the critic-prose subagent (ST0052 WP03, renamed + parameterised
# ST0053 WP03). AT-03.1 -- covers AC-03.1 (the subagent exists as critic-prose,
# declares a read-only tool loadout, and is registered) and AC-03.2 (it loads
# the IN-PR-* base plus the declared discipline, parameterised by language, and
# preserves the two-form detrope with the mechanical pass at IN-PR-STYLE-004).

load "../lib/test_helper.bash"

CRITIC_DIR="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/critic-prose"
AGENT_MD="$CRITIC_DIR/agent.md"
METADATA="$CRITIC_DIR/metadata.json"
MANIFEST="${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/.manifest/global-agents.json"

# ====================================================================
# Rename + registration (AC-03.1): critic-prose, no critic-author
# ====================================================================

@test "critic-prose: directory has agent.md and metadata.json" {
  [ -d "$CRITIC_DIR" ]
  assert_file_exists "$AGENT_MD"
  assert_file_exists "$METADATA"
}

@test "critic-prose: agent.md frontmatter declares name critic-prose" {
  assert_file_contains "$AGENT_MD" 'name: critic-prose'
}

@test "critic-prose: registered in global-agents.json" {
  jq -r '.agents[].name' "$MANIFEST" | grep -qx "critic-prose"
}

@test "critic-prose: metadata.json is valid JSON naming critic-prose" {
  run jq -r '.name' "$METADATA"
  assert_success
  assert_output "critic-prose"
}

@test "critic-prose: the old critic-author is fully retired (no dir, no manifest entry)" {
  [ ! -d "${INTENT_PROJECT_ROOT}/intent/plugins/claude/subagents/critic-author" ]
  run bash -c "jq -r '.agents[].name' '$MANIFEST' | grep -x 'critic-author'"
  assert_failure
}

# ====================================================================
# Read-only tool loadout (AC-03.1: no Write/Edit -- critics report only)
# ====================================================================

@test "critic-prose: agent.md grants Read, Grep, Glob, Bash" {
  run grep -E '^tools:' "$AGENT_MD"
  assert_success
  assert_output_contains "Read"
  assert_output_contains "Grep"
  assert_output_contains "Glob"
  assert_output_contains "Bash"
}

@test "critic-prose: agent.md grants neither Write nor Edit" {
  run grep -E '^tools:' "$AGENT_MD"
  assert_success
  refute_output_contains "Write"
  refute_output_contains "Edit"
}

@test "critic-prose: metadata.json tools are read-only" {
  run jq -r '.tools | index("Write") // empty, index("Edit") // empty' "$METADATA"
  assert_success
  assert_output ""
}

# ====================================================================
# Two-tier contract (AC-03.2): review (style) + craft-check (craft)
# ====================================================================

@test "critic-prose: agent.md defines both mode verbs" {
  assert_file_contains "$AGENT_MD" 'review'
  assert_file_contains "$AGENT_MD" 'craft-check'
}

@test "critic-prose: agent.md names both tiers (style mechanical, craft judgment)" {
  assert_file_contains "$AGENT_MD" 'category: style'
  assert_file_contains "$AGENT_MD" 'category: craft'
}

# ====================================================================
# Parameterisation (AC-03.2): the IN-PR-* base + the declared discipline
# ====================================================================

@test "critic-prose: agent.md loads the shared prose base" {
  assert_file_contains "$AGENT_MD" 'rules list --lang prose'
}

@test "critic-prose: agent.md serves both disciplines (author + content)" {
  assert_file_contains "$AGENT_MD" 'list --lang author'
  assert_file_contains "$AGENT_MD" 'list --lang content'
}

@test "critic-prose: agent.md resolves the discipline from declared languages" {
  # Reads config languages to decide author vs content vs both.
  assert_file_contains "$AGENT_MD" 'languages'
  run grep -iE 'config.json|declared language' "$AGENT_MD"
  assert_success
}

# ====================================================================
# Two-form detrope (AC-03.2): mechanical default at IN-PR-STYLE-004 +
# /in-detrope handoff, never invoked
# ====================================================================

@test "critic-prose: agent.md wires the mechanical trope pass at IN-PR-STYLE-004" {
  assert_file_contains "$AGENT_MD" 'IN-PR-STYLE-004'
  assert_file_contains "$AGENT_MD" 'trope-catalog.md'
}

@test "critic-prose: agent.md emits the CRAFT-003 /in-detrope handoff, not an invocation" {
  assert_file_contains "$AGENT_MD" 'IN-AU-CRAFT-003'
  assert_file_contains "$AGENT_MD" '/in-detrope'
  run grep -iE 'does NOT (run|invoke).*(skill|/in-detrope)|never invokes' "$AGENT_MD"
  assert_success
}
