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
# Languages-in-use is read from config (ST0037), NOT detected via
# filesystem probes. The probe table at lines 39-43 was a regression
# against design intent; v2.11.0 replaced it with a config read.
# ====================================================================

@test "in-session reads languages from intent/.config/config.json" {
  assert_file_contains "$SKILL" "intent/.config/config.json"
  assert_file_contains "$SKILL" "languages"
}

@test "in-session does not probe filesystem markers for language detection" {
  # These markers indicate filesystem-based detection. The new flow reads
  # the languages array from config; markers should not appear as detection
  # signals (mentions in the language reference table are fine).
  run grep -E "(mix\.exs|Cargo\.toml|Package\.swift|\.luarc\.json) (exists|→|->)" "$SKILL"
  [ "$status" -ne 0 ]
}

@test "in-session names Elixir as the only language with essentials skills" {
  # The language reference table names Elixir as the only language with a
  # per-language essentials skill set; rust/swift/lua/shell are listed but
  # described as covered by their rule packs + critic-<lang> only.
  assert_file_contains "$SKILL" "/in-elixir-essentials"
  assert_file_contains "$SKILL" "/in-elixir-testing"
}

@test "in-session does not reference phantom rust/swift/lua/shell essentials skills" {
  # /in-rust-essentials, /in-swift-essentials, /in-lua-essentials, and
  # /in-shell-essentials were promised in the v2.10.x SKILL.md ("ships in
  # WPNN") but never authored. ST0037 stripped the dead refs.
  run grep -E "/in-(rust|swift|lua|shell)-essentials" "$SKILL"
  [ "$status" -ne 0 ]
}

@test "in-session points at rule packs for non-Elixir languages" {
  assert_file_contains "$SKILL" "intent/plugins/claude/rules/rust"
  assert_file_contains "$SKILL" "intent/plugins/claude/rules/swift"
  assert_file_contains "$SKILL" "intent/plugins/claude/rules/lua"
  assert_file_contains "$SKILL" "intent/plugins/claude/rules/shell"
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
