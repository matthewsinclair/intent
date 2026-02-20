#!/usr/bin/env bats
# Tests for diogenes subagent and intent-elixir-testing skill (WP-11)

load "../lib/test_helper.bash"

# Setup/teardown for diogenes tests
setup() {
  # Create temp dir outside of Intent project
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Use a fake HOME so tests never touch real ~/.claude
  REAL_HOME="$HOME"
  export HOME="$TEST_TEMP_DIR/fakehome"
  mkdir -p "$HOME"

  # Create mock .claude directory for testing
  mkdir -p "$HOME/.claude/agents"
  mkdir -p "$HOME/.claude/skills"
}

teardown() {
  # Restore real HOME
  export HOME="$REAL_HOME"

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

@test "claude subagents list shows diogenes description" {
  run run_intent claude subagents list -v
  assert_success
  assert_output_contains "diogenes"
  assert_output_contains "Test Architect"
}

# ====================================================================
# Diogenes subagent: install
# ====================================================================

@test "claude subagents install diogenes installs successfully" {
  run run_intent claude subagents install diogenes --force
  assert_success
  assert_output_contains "Installing agent: diogenes"
  assert_output_contains "Installed successfully"

  # Verify the file was created
  assert_file_exists "$HOME/.claude/agents/diogenes.md"
}

@test "claude subagents install diogenes creates manifest entry" {
  # Clean any existing manifest
  rm -rf "$HOME/.intent/agents" 2>/dev/null || true

  run run_intent claude subagents install diogenes --force
  assert_success

  # Check manifest was created
  assert_file_exists "$HOME/.intent/agents/installed-agents.json"

  # Verify manifest content
  run cat "$HOME/.intent/agents/installed-agents.json"
  assert_success
  assert_output_contains '"name": "diogenes"'
  assert_output_contains '"checksum":'
}

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
  assert_output_contains "Up to date"
}

@test "claude subagents sync detects diogenes modification" {
  # Install diogenes
  run run_intent claude subagents install diogenes --force
  assert_success

  # Modify the installed file
  echo "# Modified locally" >> "$HOME/.claude/agents/diogenes.md"

  # Sync should detect the modification
  run run_intent claude subagents sync
  assert_success
  assert_output_contains "diogenes"
  # Should detect it is not up to date (either "modified" or "Updated")
  refute_output_contains "Up to date"
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
# intent-elixir-testing skill: list
# ====================================================================

@test "claude skills list includes intent-elixir-testing" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-elixir-testing"
}

@test "claude skills list shows intent-elixir-testing description" {
  run run_intent claude skills list -v
  assert_success
  assert_output_contains "intent-elixir-testing"
  assert_output_contains "testing"
}

# ====================================================================
# intent-elixir-testing skill: install
# ====================================================================

@test "claude skills install intent-elixir-testing installs successfully" {
  run run_intent claude skills install intent-elixir-testing --force
  assert_success
  assert_output_contains "Installing skill: intent-elixir-testing"
  assert_output_contains "Installed successfully"

  # Verify the file was created
  assert_file_exists "$HOME/.claude/skills/intent-elixir-testing/SKILL.md"
}

@test "claude skills install intent-elixir-testing creates manifest entry" {
  # Clean any existing manifest
  rm -rf "$HOME/.intent/skills" 2>/dev/null || true

  run run_intent claude skills install intent-elixir-testing --force
  assert_success

  # Check manifest was created
  assert_file_exists "$HOME/.intent/skills/installed-skills.json"

  # Verify manifest content
  run cat "$HOME/.intent/skills/installed-skills.json"
  assert_success
  assert_output_contains '"name": "intent-elixir-testing"'
  assert_output_contains '"checksum":'
}

@test "claude skills install intent-elixir-testing file has correct content" {
  run run_intent claude skills install intent-elixir-testing --force
  assert_success

  # Verify key content in the installed file
  assert_file_contains "$HOME/.claude/skills/intent-elixir-testing/SKILL.md" "Elixir Testing Essentials"
  assert_file_contains "$HOME/.claude/skills/intent-elixir-testing/SKILL.md" "No control flow in test bodies"
  assert_file_contains "$HOME/.claude/skills/intent-elixir-testing/SKILL.md" "Strong assertions"
}

@test "claude skills install --all includes intent-elixir-testing" {
  run run_intent claude skills install --all --force
  assert_success
  assert_output_contains "Installing skill: intent-elixir-testing"
  assert_output_contains "Installed successfully"

  # Verify it was installed
  assert_file_exists "$HOME/.claude/skills/intent-elixir-testing/SKILL.md"
}

# ====================================================================
# intent-elixir-testing skill: show
# ====================================================================

@test "claude skills show intent-elixir-testing displays content" {
  run run_intent claude skills show intent-elixir-testing
  assert_success
  assert_output_contains "intent-elixir-testing"
  assert_output_contains "testing"
}

# ====================================================================
# intent-elixir-testing skill: sync
# ====================================================================

@test "claude skills sync detects intent-elixir-testing as up-to-date" {
  # Install the skill
  run run_intent claude skills install intent-elixir-testing --force
  assert_success

  # Sync should find nothing to update
  run run_intent claude skills sync
  assert_success
  assert_output_contains "intent-elixir-testing"
  assert_output_contains "Up to date"
}

@test "claude skills sync detects intent-elixir-testing modification" {
  # Install the skill
  run run_intent claude skills install intent-elixir-testing --force
  assert_success

  # Modify the installed file
  echo "# Modified locally" >> "$HOME/.claude/skills/intent-elixir-testing/SKILL.md"

  # Sync should detect the modification
  run run_intent claude skills sync
  assert_success
  assert_output_contains "intent-elixir-testing"
  refute_output_contains "Up to date"
}

# ====================================================================
# intent-elixir-testing skill: uninstall
# ====================================================================

@test "claude skills uninstall intent-elixir-testing removes the file" {
  # Install first
  run run_intent claude skills install intent-elixir-testing --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/intent-elixir-testing/SKILL.md"

  # Uninstall
  run run_intent claude skills uninstall intent-elixir-testing --force
  assert_success
  assert_output_contains "intent-elixir-testing"
  assert_file_not_exists "$HOME/.claude/skills/intent-elixir-testing/SKILL.md"
}
