#!/usr/bin/env bats
# Tests that the `/in-session` bootstrap skill exists with the expected
# contract: references the universal skills, declares detection probes for
# every supported language, and is wired into CLAUDE.md as the post-compact
# trigger.

load "../lib/test_helper.bash"

SKILL="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-session/SKILL.md"
CLAUDE_MD="${INTENT_PROJECT_ROOT}/CLAUDE.md"

# ====================================================================
# File exists with frontmatter
# ====================================================================

@test "in-session skill exists" {
  [ -f "$SKILL" ]
}

@test "in-session declares description and chains_to frontmatter" {
  assert_file_contains "$SKILL" "description:"
  assert_file_contains "$SKILL" "chains_to:"
}

# ====================================================================
# Universal skills are always loaded
# ====================================================================

@test "in-session references the universal skills unconditionally" {
  assert_file_contains "$SKILL" "/in-essentials"
  assert_file_contains "$SKILL" "/in-standards"
}

# ====================================================================
# Language detection probes match in-review stage-2
# ====================================================================

@test "in-session declares mix.exs probe for Elixir" {
  assert_file_contains "$SKILL" "mix.exs"
  assert_file_contains "$SKILL" "/in-elixir-essentials"
  assert_file_contains "$SKILL" "/in-elixir-testing"
}

@test "in-session declares Cargo.toml probe for Rust" {
  assert_file_contains "$SKILL" "Cargo.toml"
}

@test "in-session declares Package.swift probe for Swift" {
  assert_file_contains "$SKILL" "Package.swift"
}

@test "in-session declares Lua probe" {
  assert_file_contains "$SKILL" ".luarc.json"
}

@test "in-session declares shell probe" {
  assert_file_contains "$SKILL" "shebang"
}

# ====================================================================
# Elixir dep-based fan-out
# ====================================================================

@test "in-session fans out to Ash and LiveView skills on mix.exs match" {
  assert_file_contains "$SKILL" "/in-ash-ecto-essentials"
  assert_file_contains "$SKILL" "/in-phoenix-liveview"
  assert_file_contains "$SKILL" ":ash"
  assert_file_contains "$SKILL" ":phoenix_live_view"
}

# ====================================================================
# CLAUDE.md wiring
# ====================================================================

@test "CLAUDE.md instructs post-compact invocation of /in-session" {
  assert_file_contains "$CLAUDE_MD" "/in-session"
  assert_file_contains "$CLAUDE_MD" "compact"
}

# ====================================================================
# Gate-release block uses the helper script, not inline awk
# ====================================================================

@test "in-session SKILL.md invokes release-gate.sh, not inline awk" {
  # The inline awk form had `awk '{print $1}'` silently mangled to
  # `awk '{print }'` by Claude Code's skill renderer. The fix moved the
  # logic to a script file. If anyone re-inlines the awk pipeline, this
  # test catches it.
  assert_file_contains "$SKILL" "release-gate.sh"
  run grep -E "awk '\{print \\\$1\}'" "$SKILL"
  [ "$status" -ne 0 ]
}

@test "in-session ships release-gate.sh in scripts/" {
  local script="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-session/scripts/release-gate.sh"
  [ -f "$script" ]
  [ -x "$script" ]
}
