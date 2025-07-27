#!/usr/bin/env bats
# Tests for intent agents commands (v2.0.0)

load "../lib/test_helper.bash"

# Setup/teardown for agent tests
setup() {
  # Create temp dir outside of Intent project
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  
  # Create a mock .claude directory for testing
  mkdir -p "$HOME/.claude/agents"
  
  # Save any existing agents
  if [ -d "$HOME/.claude/agents.backup" ]; then
    rm -rf "$HOME/.claude/agents.backup"
  fi
  if [ -d "$HOME/.claude/agents" ] && [ "$(ls -A $HOME/.claude/agents 2>/dev/null)" ]; then
    cp -r "$HOME/.claude/agents" "$HOME/.claude/agents.backup"
  fi
  
  # Clean the agents directory for testing
  rm -f "$HOME/.claude/agents"/*.md 2>/dev/null || true
}

teardown() {
  # Restore backed up agents if they exist
  if [ -d "$HOME/.claude/agents.backup" ]; then
    rm -rf "$HOME/.claude/agents"
    mv "$HOME/.claude/agents.backup" "$HOME/.claude/agents"
  fi
  
  # Clean up test directory
  if [ -d "${TEST_TEMP_DIR}" ]; then
    cd "${INTENT_PROJECT_ROOT}" || exit 1
    rm -rf "${TEST_TEMP_DIR}"
  fi
  
  # Clean up test manifests
  rm -rf "$HOME/.intent/agents" 2>/dev/null || true
}

@test "agents command shows help when no subcommand given" {
  run run_intent agents
  assert_success
  assert_output_contains "Usage: intent agents <command>"
  assert_output_contains "list"
  assert_output_contains "install"
}

@test "agents list shows available agents" {
  run run_intent agents list
  assert_success
  assert_output_contains "Available Agents:"
  assert_output_contains "Global:"
  assert_output_contains "intent"
  assert_output_contains "elixir"
}

@test "agents list shows installation status" {
  # Initially nothing installed
  run run_intent agents list
  assert_success
  assert_output_contains "[NOT INSTALLED]"
  
  # Install an agent manually
  cp "${INTENT_HOME}/agents/intent/agent.md" "$HOME/.claude/agents/intent.md"
  
  # Check it shows as installed
  run run_intent agents list
  assert_success
  assert_output_contains "intent       - Intent-aware development assistant [INSTALLED]"
  assert_output_contains "elixir       - Elixir code doctor with Usage Rules [NOT INSTALLED]"
}

@test "agents install requires an agent name" {
  run run_intent agents install
  assert_failure
  assert_output_contains "Error: No agent specified"
  assert_output_contains "Usage: intent agents install"
}

@test "agents install installs a single agent" {
  run run_intent agents install intent --force
  assert_success
  assert_output_contains "Installing agent: intent"
  assert_output_contains "Installed successfully"
  assert_output_contains "Installed: 1"
  
  # Verify the file was created
  assert_file_exists "$HOME/.claude/agents/intent.md"
  
  # Verify it shows as installed
  run run_intent agents list
  assert_success
  assert_output_contains "intent       - Intent-aware development assistant [INSTALLED]"
}

@test "agents install handles non-existent agent" {
  run run_intent agents install nonexistent
  assert_failure  # Command fails when no agents installed
  assert_output_contains "Error: Agent 'nonexistent' not found"
  assert_output_contains "Failed: 1"
}

@test "agents install prompts before overwriting" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Try to install again, saying no to overwrite
  run bash -c "echo 'n' | ${INTENT_BIN_DIR}/intent agents install intent"
  assert_success
  assert_output_contains "Agent already exists"
  assert_output_contains "Skipped"
  assert_output_contains "Skipped: 1"
}

@test "agents install can overwrite when confirmed" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Modify the agent to test overwrite
  echo "# Modified" >> "$HOME/.claude/agents/intent.md"
  
  # Try to install again, saying yes to overwrite
  run bash -c "echo 'y' | ${INTENT_BIN_DIR}/intent agents install intent"
  assert_success
  assert_output_contains "Agent already exists"
  assert_output_contains "Installed successfully"
  assert_output_contains "Installed: 1"
  
  # Verify modification was overwritten
  run grep "# Modified" "$HOME/.claude/agents/intent.md"
  assert_failure
}

@test "agents install supports multiple agents" {
  run run_intent agents install intent elixir --force
  assert_success
  assert_output_contains "Installing agent: intent"
  assert_output_contains "Installing agent: elixir"
  assert_output_contains "Installed: 2"
  
  # Verify both files exist
  assert_file_exists "$HOME/.claude/agents/intent.md"
  assert_file_exists "$HOME/.claude/agents/elixir.md"
}

@test "agents install --all installs all available agents" {
  run run_intent agents install --all --force
  assert_success
  assert_output_contains "Installing agent: intent"
  assert_output_contains "Installing agent: elixir"
  assert_output_contains "Installed: 2"
  
  # Verify all agents are installed
  run run_intent agents list
  assert_success
  assert_output_contains "intent       - Intent-aware development assistant [INSTALLED]"
  assert_output_contains "elixir       - Elixir code doctor with Usage Rules [INSTALLED]"
}

@test "agents install creates manifest" {
  # Clean any existing manifest
  rm -rf "$HOME/.intent/agents" 2>/dev/null || true
  
  run run_intent agents install intent --force
  assert_success
  
  # Check manifest was created
  assert_file_exists "$HOME/.intent/agents/installed-agents.json"
  
  # Verify manifest content
  run cat "$HOME/.intent/agents/installed-agents.json"
  assert_success
  assert_output_contains '"name": "intent"'
  assert_output_contains '"source": "global"'
  assert_output_contains '"checksum":'
}

@test "agents install updates manifest on reinstall" {
  # Install once
  run run_intent agents install intent --force
  assert_success
  
  # Get original timestamp
  original_manifest=$(cat "$HOME/.intent/agents/installed-agents.json")
  
  # Wait a moment and reinstall
  sleep 1
  run bash -c "echo 'y' | ${INTENT_BIN_DIR}/intent agents install intent"
  assert_success
  
  # Verify manifest was updated
  new_manifest=$(cat "$HOME/.intent/agents/installed-agents.json")
  [ "$original_manifest" != "$new_manifest" ] || fail "Manifest should have been updated"
  
  # Should still only have one entry for intent
  count=$(jq '.installed | map(select(.name == "intent")) | length' "$HOME/.intent/agents/installed-agents.json")
  [ "$count" -eq 1 ] || fail "Should only have one entry for intent agent"
}

@test "agents handles missing Claude directory gracefully" {
  # Remove .claude directory
  rm -rf "$HOME/.claude"
  
  run run_intent agents list
  assert_success
  assert_output_contains "Note: Claude Code not detected"
  
  run run_intent agents install intent
  assert_failure
  assert_output_contains "Error: Claude Code not detected"
}

@test "agents creates .claude/agents directory if missing" {
  # Ensure .claude exists but not agents subdirectory
  mkdir -p "$HOME/.claude"
  rm -rf "$HOME/.claude/agents"
  
  run run_intent agents install intent --force
  assert_success
  assert_directory_exists "$HOME/.claude/agents"
  assert_file_exists "$HOME/.claude/agents/intent.md"
}

@test "agents command handles invalid subcommand" {
  run run_intent agents invalid
  assert_failure
  assert_output_contains "Error: Unknown command 'intent agents invalid'"
  assert_output_contains "Run 'intent agents help' for usage"
}

@test "agents works from within a project" {
  project_dir=$(create_test_project "Agent Test Project")
  cd "$project_dir"
  
  # Should work the same from within a project
  run run_intent agents list
  assert_success
  assert_output_contains "Available Agents:"
  
  run run_intent agents install intent --force
  assert_success
  assert_output_contains "Installed successfully"
}

# Sync command tests
@test "agents sync requires installed agents" {
  # Clean manifest
  rm -rf "$HOME/.intent/agents" 2>/dev/null || true
  
  run run_intent agents sync
  assert_success
  assert_output_contains "No installed agents found"
  assert_output_contains "Use 'intent agents install'"
}

@test "agents sync detects up-to-date agents" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Sync should find nothing to update
  run run_intent agents sync
  assert_success
  assert_output_contains "Checking agent: intent"
  assert_output_contains "Up to date"
  assert_output_contains "Skipped: 1"
}

@test "agents sync detects local modifications" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Modify the agent
  echo "# Test modification" >> "$HOME/.claude/agents/intent.md"
  
  # Sync should detect modification
  run bash -c "echo 'n' | ${INTENT_BIN_DIR}/intent agents sync"
  assert_success
  assert_output_contains "Warning: Agent has been modified locally"
  assert_output_contains "Overwrite local changes?"
  assert_output_contains "Skipped"
}

@test "agents sync can force overwrite modifications" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Modify the agent
  echo "# Test modification" >> "$HOME/.claude/agents/intent.md"
  
  # Force sync should overwrite
  run run_intent agents sync --force
  assert_success
  assert_output_contains "Warning: Agent has been modified locally"
  assert_output_contains "Overwriting local changes (--force)"
  assert_output_contains "Updated successfully"
  
  # Verify modification was removed
  run grep "# Test modification" "$HOME/.claude/agents/intent.md"
  assert_failure
}

@test "agents sync updates when source changes" {
  # Install an agent
  run run_intent agents install intent --force
  assert_success
  
  # Simulate source update by modifying the source file
  # (In real scenario, this would be from a git pull)
  echo "# Source update" >> "$INTENT_HOME/agents/intent/agent.md"
  
  # Sync should detect and update
  run run_intent agents sync
  assert_success
  assert_output_contains "Update available"
  assert_output_contains "Updated successfully"
  assert_output_contains "Updated: 1"
  
  # Verify update was applied
  run grep "# Source update" "$HOME/.claude/agents/intent.md"
  assert_success
  
  # Clean up source modification
  sed -i.bak '/# Source update/d' "$INTENT_HOME/agents/intent/agent.md"
  rm -f "$INTENT_HOME/agents/intent/agent.md.bak"
}

@test "agents sync handles missing Claude directory" {
  # Remove .claude directory
  rm -rf "$HOME/.claude"
  
  run run_intent agents sync
  assert_failure
  assert_output_contains "Error: Claude Code not detected"
}

@test "agents sync works with multiple agents" {
  # Install multiple agents
  run run_intent agents install intent elixir --force
  assert_success
  
  # Sync should check both
  run run_intent agents sync
  assert_success
  assert_output_contains "Checking agent: intent"
  assert_output_contains "Checking agent: elixir"
  assert_output_contains "Up to date"
}

@test "agents uninstall is not yet implemented" {
  run run_intent agents uninstall intent
  assert_failure
  assert_output_contains "Error: 'intent agents uninstall' not yet implemented"
}

@test "agents show is not yet implemented" {
  run run_intent agents show intent
  assert_failure
  assert_output_contains "Error: 'intent agents show' not yet implemented"
}

@test "agents status is not yet implemented" {
  run run_intent agents status
  assert_failure
  assert_output_contains "Error: 'intent agents status' not yet implemented"
}