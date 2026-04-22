#!/usr/bin/env bats
# Tests that the refactored `in-*` skills reference Intent rules by ID
# rather than inlining rule prose.
#
# After WP03, each Elixir-related skill becomes a thin pointer file: it
# enumerates the rule IDs that apply to its domain and delegates the prose
# to the `RULE.md` files themselves. This test guards that contract.

load "../lib/test_helper.bash"

SKILL_ROOT="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills"

# ====================================================================
# in-elixir-essentials → Elixir code rules
# ====================================================================

@test "in-elixir-essentials references every Elixir code rule by ID" {
  local skill="$SKILL_ROOT/in-elixir-essentials/SKILL.md"
  local id
  for id in IN-EX-CODE-001 IN-EX-CODE-002 IN-EX-CODE-003 IN-EX-CODE-004 IN-EX-CODE-005 IN-EX-CODE-006; do
    assert_file_contains "$skill" "$id"
  done
}

# ====================================================================
# in-elixir-testing → Elixir test rules
# ====================================================================

@test "in-elixir-testing references every Elixir test rule by ID" {
  local skill="$SKILL_ROOT/in-elixir-testing/SKILL.md"
  local id
  for id in IN-EX-TEST-001 IN-EX-TEST-002 IN-EX-TEST-003 IN-EX-TEST-004 IN-EX-TEST-005 IN-EX-TEST-006 IN-EX-TEST-007; do
    assert_file_contains "$skill" "$id"
  done
}

# ====================================================================
# in-ash-ecto-essentials → Ash rules
# ====================================================================

@test "in-ash-ecto-essentials references every Ash rule by ID" {
  local skill="$SKILL_ROOT/in-ash-ecto-essentials/SKILL.md"
  local id
  for id in IN-EX-ASH-001 IN-EX-ASH-002; do
    assert_file_contains "$skill" "$id"
  done
}

# ====================================================================
# in-phoenix-liveview → LiveView + Phoenix rules (+ shared code rules)
# ====================================================================

@test "in-phoenix-liveview references every LiveView and Phoenix rule by ID" {
  local skill="$SKILL_ROOT/in-phoenix-liveview/SKILL.md"
  local id
  for id in IN-EX-LV-001 IN-EX-LV-002 IN-EX-LV-003 IN-EX-PHX-001; do
    assert_file_contains "$skill" "$id"
  done
}

# ====================================================================
# in-standards → agnostic rules
# ====================================================================

@test "in-standards references every agnostic rule by ID" {
  local skill="$SKILL_ROOT/in-standards/SKILL.md"
  local id
  for id in IN-AG-HIGHLANDER-001 IN-AG-PFIC-001 IN-AG-THIN-COORD-001 IN-AG-NO-SILENT-001; do
    assert_file_contains "$skill" "$id"
  done
}
