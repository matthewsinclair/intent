#!/usr/bin/env bats
# Tests for diogenes subagent and in-elixir-testing skill (WP-11)

load "../lib/test_helper.bash"

# Setup/teardown for diogenes tests
setup() {
  # Create temp dir outside of Intent project
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Use a fake HOME so tests never touch real ~/.claude
  setup_fake_home
}

teardown() {
  # Restore real HOME
  teardown_fake_home

  # Clean up test directory
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
}

# ====================================================================
# Diogenes subagent: list
# ====================================================================

@test "claude subagents list includes diogenes" {
  run run_intent claude subagents list
  assert_success
  assert_output_contains "diogenes"
}

# ====================================================================
# Diogenes subagent: install
# ====================================================================

@test "claude subagents install diogenes file has correct content" {
  run run_intent claude subagents install diogenes --force
  assert_success

  # Verify key content in the installed file
  assert_file_contains "$HOME/.claude/agents/diogenes.md" "Aristotle"
  assert_file_contains "$HOME/.claude/agents/diogenes.md" "Diogenes"
  assert_file_contains "$HOME/.claude/agents/diogenes.md" "Test Architect"
}

# ====================================================================
# Diogenes subagent: show
# ====================================================================

@test "claude subagents show diogenes displays metadata" {
  run run_intent claude subagents show diogenes
  assert_success
  assert_output_contains "diogenes"
  assert_output_contains "Test Architect"
}

# ====================================================================
# Diogenes subagent: sync
# ====================================================================

@test "claude subagents sync detects diogenes as up-to-date" {
  # Install diogenes
  run run_intent claude subagents install diogenes --force
  assert_success

  # Sync should find nothing to update
  run run_intent claude subagents sync
  assert_success
  assert_output_contains "diogenes"
  assert_output_contains "up to date"
}

@test "claude subagents sync detects diogenes modification" {
  # Install diogenes
  run run_intent claude subagents install diogenes --force
  assert_success

  # Modify the installed file
  echo "# Modified locally" >> "$HOME/.claude/agents/diogenes.md"

  # Sync detects the modification and HOLDS it rather than overwriting it. A held
  # unit needs a decision, and that exits non-zero by design.
  run run_intent claude subagents sync
  [ "$status" -eq 1 ] || fail "a held unit should exit 1, got $status"
  assert_output_contains "diogenes"
  assert_output_contains "modified here since it was installed -- HELD"
  assert_output_contains "1 need a decision"
  refute_output_contains "up to date"
  assert_file_contains "$HOME/.claude/agents/diogenes.md" "# Modified locally"
}

# ====================================================================
# Diogenes subagent: uninstall
# ====================================================================

@test "claude subagents uninstall diogenes removes the file" {
  # Install first
  run run_intent claude subagents install diogenes --force
  assert_success
  assert_file_exists "$HOME/.claude/agents/diogenes.md"

  # Uninstall
  run run_intent claude subagents uninstall diogenes --force
  assert_success
  assert_output_contains "diogenes"
  assert_file_not_exists "$HOME/.claude/agents/diogenes.md"
}

# ====================================================================
# in-elixir-testing skill: list
# ====================================================================

@test "claude skills list includes in-elixir-testing" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "in-elixir-testing"
}

@test "claude skills list shows in-elixir-testing description" {
  run run_intent claude skills list -v
  assert_success
  assert_output_contains "in-elixir-testing"
  assert_output_contains "testing"
}

# ====================================================================
# in-elixir-testing skill: install
# ====================================================================

@test "claude skills install in-elixir-testing file has correct content" {
  run run_intent claude skills install in-elixir-testing --force
  assert_success

  # Verify key content in the installed file. After WP03, the skill points at
  # rules by ID rather than inlining rule prose, so the assertions pivot to
  # the rule IDs themselves.
  assert_file_contains "$HOME/.claude/skills/in-elixir-testing/SKILL.md" "Elixir Testing Essentials"
  assert_file_contains "$HOME/.claude/skills/in-elixir-testing/SKILL.md" "IN-EX-TEST-001"
  assert_file_contains "$HOME/.claude/skills/in-elixir-testing/SKILL.md" "IN-EX-TEST-005"
  assert_file_contains "$HOME/.claude/skills/in-elixir-testing/SKILL.md" "strong-assertions"
}

# ====================================================================
# in-elixir-testing skill: show
# ====================================================================

# ====================================================================
# in-elixir-testing skill: sync
# ====================================================================

@test "claude skills sync detects in-elixir-testing as up-to-date" {
  # Install the skill
  run run_intent claude skills install in-elixir-testing --force
  assert_success

  # Sync should find nothing to update
  run run_intent claude skills sync
  assert_success
  assert_output_contains "in-elixir-testing"
  assert_output_contains "up to date"
}

@test "claude skills sync detects in-elixir-testing modification" {
  # Install the skill
  run run_intent claude skills install in-elixir-testing --force
  assert_success

  # Modify the installed file
  echo "# Modified locally" >> "$HOME/.claude/skills/in-elixir-testing/SKILL.md"

  # Sync detects the modification and HOLDS it rather than overwriting it. A held
  # unit needs a decision, and that exits non-zero by design.
  run run_intent claude skills sync
  [ "$status" -eq 1 ] || fail "a held unit should exit 1, got $status"
  assert_output_contains "in-elixir-testing"
  assert_output_contains "modified here since it was installed -- HELD"
  assert_output_contains "1 need a decision"
  refute_output_contains "up to date"
  assert_file_contains "$HOME/.claude/skills/in-elixir-testing/SKILL.md" "# Modified locally"
}

# ====================================================================
# in-elixir-testing skill: uninstall
# ====================================================================

@test "claude skills uninstall in-elixir-testing removes the file" {
  # Install first
  run run_intent claude skills install in-elixir-testing --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/in-elixir-testing/SKILL.md"

  # Uninstall
  run run_intent claude skills uninstall in-elixir-testing --force
  assert_success
  assert_output_contains "in-elixir-testing"
  assert_file_not_exists "$HOME/.claude/skills/in-elixir-testing/SKILL.md"
}
