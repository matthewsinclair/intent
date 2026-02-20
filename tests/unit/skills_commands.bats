#!/usr/bin/env bats
# Tests for intent claude skills commands (v2.4.0)

load "../lib/test_helper.bash"

# Setup/teardown for skill tests
setup() {
  # Create temp dir outside of Intent project
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Use a fake HOME so tests never touch real ~/.claude
  REAL_HOME="$HOME"
  export HOME="$TEST_TEMP_DIR/fakehome"
  mkdir -p "$HOME"

  # Create a mock .claude directory for testing
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
# Help and basic routing
# ====================================================================

@test "claude skills command shows help when no subcommand given" {
  run run_intent claude skills
  assert_success
  assert_output_contains "Usage: intent claude skills <command>"
  assert_output_contains "list"
  assert_output_contains "install"
  assert_output_contains "sync"
  assert_output_contains "uninstall"
  assert_output_contains "show"
}

@test "claude skills help shows usage information" {
  run run_intent claude skills help
  assert_success
  assert_output_contains "Usage: intent claude skills <command>"
  assert_output_contains "Examples:"
}

@test "claude skills handles invalid subcommand" {
  run run_intent claude skills invalid
  assert_failure
  assert_output_contains "Error: Unknown command 'intent claude skills invalid'"
  assert_output_contains "Run 'intent claude skills help' for usage"
}

# ====================================================================
# List command
# ====================================================================

@test "claude skills list shows available skills" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "Available Skills:"
  assert_output_contains "intent-essentials"
  assert_output_contains "intent-elixir-essentials"
  assert_output_contains "intent-ash-ecto-essentials"
  assert_output_contains "intent-phoenix-liveview"
}

@test "claude skills list shows installation status" {
  # Initially nothing installed
  run run_intent claude skills list
  assert_success
  assert_output_contains "[NOT INSTALLED]"

  # Install a skill manually
  mkdir -p "$HOME/.claude/skills/intent-elixir-essentials"
  cp "${INTENT_HOME}/intent/plugins/claude/skills/intent-elixir-essentials/SKILL.md" "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"

  # Check it shows as installed
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-elixir-essentials"
  assert_output_contains "[INSTALLED]"
}

@test "claude skills list warns when Claude not detected" {
  rm -rf "$HOME/.claude"

  run run_intent claude skills list
  assert_success
  assert_output_contains "Note: Claude Code not detected"
}

# ====================================================================
# Install command
# ====================================================================

@test "claude skills install requires a skill name" {
  run run_intent claude skills install
  assert_failure
  assert_output_contains "Error: No skill specified"
  assert_output_contains "Usage: intent claude skills install"
}

@test "claude skills install installs a single skill" {
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success
  assert_output_contains "Installing skill: intent-elixir-essentials"
  assert_output_contains "Installed successfully"
  assert_output_contains "Installation complete:"
  assert_output_contains "Installed:"

  # Verify the file was created
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
}

@test "claude skills install handles non-existent skill" {
  run run_intent claude skills install nonexistent
  assert_failure
  assert_output_contains "Error: Skill 'nonexistent' not found"
  assert_output_contains "Failed: 1"
}

@test "claude skills install prompts before overwriting" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Try to install again, saying no to overwrite
  run bash -c "echo 'n' | ${INTENT_BIN_DIR}/intent claude skills install intent-elixir-essentials"
  assert_success
  assert_output_contains "Skill already exists"
  assert_output_contains "Skipped"
  assert_output_contains "Skipped: 1"
}

@test "claude skills install can overwrite when confirmed" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Modify the skill to test overwrite
  echo "# Modified" >> "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"

  # Try to install again, saying yes to overwrite
  run bash -c "echo 'y' | ${INTENT_BIN_DIR}/intent claude skills install intent-elixir-essentials"
  assert_success
  assert_output_contains "Skill already exists"
  assert_output_contains "Installed successfully"

  # Verify modification was overwritten
  run grep "# Modified" "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
  assert_failure
}

@test "claude skills install supports multiple skills" {
  run run_intent claude skills install intent-elixir-essentials intent-ash-ecto-essentials --force
  assert_success
  assert_output_contains "Installing skill: intent-elixir-essentials"
  assert_output_contains "Installing skill: intent-ash-ecto-essentials"
  assert_output_contains "Installation complete:"
  assert_output_contains "Installed:"

  # Verify both files exist
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-ash-ecto-essentials/SKILL.md"
}

@test "claude skills install --all installs all available skills" {
  run run_intent claude skills install --all --force
  assert_success
  assert_output_contains "Installing skill: intent-essentials"
  assert_output_contains "Installing skill: intent-ash-ecto-essentials"
  assert_output_contains "Installing skill: intent-elixir-essentials"
  assert_output_contains "Installing skill: intent-phoenix-liveview"
  assert_output_contains "Installation complete:"

  # Verify all skills are installed
  assert_file_exists "$HOME/.claude/skills/intent-essentials/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-ash-ecto-essentials/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-phoenix-liveview/SKILL.md"
}

@test "claude skills install creates manifest" {
  # Clean any existing manifest
  rm -rf "$HOME/.intent/skills" 2>/dev/null || true

  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Check manifest was created
  assert_file_exists "$HOME/.intent/skills/installed-skills.json"

  # Verify manifest content
  run cat "$HOME/.intent/skills/installed-skills.json"
  assert_success
  assert_output_contains '"name": "intent-elixir-essentials"'
  assert_output_contains '"checksum":'
}

@test "claude skills install requires Claude directory" {
  rm -rf "$HOME/.claude"

  run run_intent claude skills install intent-elixir-essentials
  assert_failure
  assert_output_contains "Error: Claude Code not detected"
}

@test "claude skills install creates skills subdirectory" {
  # Ensure .claude exists but not skills subdirectory
  mkdir -p "$HOME/.claude"
  rm -rf "$HOME/.claude/skills"

  run run_intent claude skills install intent-elixir-essentials --force
  assert_success
  assert_directory_exists "$HOME/.claude/skills/intent-elixir-essentials"
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
}

# ====================================================================
# Sync command
# ====================================================================

@test "claude skills sync requires installed skills" {
  # Clean manifest
  rm -rf "$HOME/.intent/skills" 2>/dev/null || true

  run run_intent claude skills sync
  assert_success
  assert_output_contains "No installed skills found"
  assert_output_contains "Use 'intent claude skills install'"
}

@test "claude skills sync detects up-to-date skills" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Sync should find nothing to update
  run run_intent claude skills sync
  assert_success
  assert_output_contains "Checking skill: intent-elixir-essentials"
  assert_output_contains "Up to date"
  assert_output_contains "Skipped: 1"
}

@test "claude skills sync detects local modifications" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Modify the skill
  echo "# Test modification" >> "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"

  # Sync should detect modification
  run bash -c "echo 'n' | ${INTENT_BIN_DIR}/intent claude skills sync"
  assert_success
  assert_output_contains "Warning: Skill has been modified locally"
  assert_output_contains "Overwrite local changes?"
  assert_output_contains "Skipped"
}

@test "claude skills sync can force overwrite modifications" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Modify the skill
  echo "# Test modification" >> "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"

  # Force sync should overwrite
  run run_intent claude skills sync --force
  assert_success
  assert_output_contains "Warning: Skill has been modified locally"
  assert_output_contains "Overwriting local changes (--force)"
  assert_output_contains "Updated successfully"

  # Verify modification was removed
  run grep "# Test modification" "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
  assert_failure
}

@test "claude skills sync handles missing Claude directory" {
  rm -rf "$HOME/.claude"

  run run_intent claude skills sync
  assert_failure
  assert_output_contains "Error: Claude Code not detected"
}

@test "claude skills sync works with multiple skills" {
  # Install multiple skills
  run run_intent claude skills install intent-elixir-essentials intent-ash-ecto-essentials --force
  assert_success

  # Sync should check both
  run run_intent claude skills sync
  assert_success
  assert_output_contains "Checking skill: intent-elixir-essentials"
  assert_output_contains "Checking skill: intent-ash-ecto-essentials"
  assert_output_contains "Up to date"
}

# ====================================================================
# Uninstall command
# ====================================================================

@test "claude skills uninstall requires a skill name" {
  run run_intent claude skills uninstall
  assert_failure
  assert_output_contains "Error: No skill specified"
  assert_output_contains "Usage: intent claude skills uninstall"
}

@test "claude skills uninstall removes a single skill" {
  # Install a skill first
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Uninstall with force
  run run_intent claude skills uninstall intent-elixir-essentials --force
  assert_success
  assert_output_contains "Uninstalling skill: intent-elixir-essentials"
  assert_output_contains "Removed successfully"
  assert_output_contains "Removed: 1"

  # Verify it's gone
  [ ! -d "$HOME/.claude/skills/intent-elixir-essentials" ] || fail "Skill directory should have been removed"
}

@test "claude skills uninstall prompts for confirmation" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Try to uninstall, saying no
  run bash -c "echo 'n' | ${INTENT_BIN_DIR}/intent claude skills uninstall intent-elixir-essentials"
  assert_success
  assert_output_contains "The following skills will be uninstalled:"
  assert_output_contains "- intent-elixir-essentials"
  assert_output_contains "Continue?"
  assert_output_contains "Cancelled"

  # Verify skill still exists
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
}

@test "claude skills uninstall handles non-existent skill" {
  run run_intent claude skills uninstall nonexistent --force
  assert_success
  assert_output_contains "Uninstalling skill: nonexistent"
  assert_output_contains "Skill not found"
  assert_output_contains "Skipped: 1"
}

@test "claude skills uninstall supports multiple skills" {
  # Install multiple skills
  run run_intent claude skills install intent-elixir-essentials intent-ash-ecto-essentials --force
  assert_success

  # Uninstall both
  run run_intent claude skills uninstall intent-elixir-essentials intent-ash-ecto-essentials --force
  assert_success
  assert_output_contains "Uninstalling skill: intent-elixir-essentials"
  assert_output_contains "Uninstalling skill: intent-ash-ecto-essentials"
  assert_output_contains "Removed: 2"

  # Verify both are gone
  [ ! -d "$HOME/.claude/skills/intent-elixir-essentials" ] || fail "Skill directory should have been removed"
  [ ! -d "$HOME/.claude/skills/intent-ash-ecto-essentials" ] || fail "Skill directory should have been removed"
}

@test "claude skills uninstall --all removes all managed skills" {
  # Install multiple skills
  run run_intent claude skills install intent-elixir-essentials intent-ash-ecto-essentials --force
  assert_success

  # Uninstall all
  run run_intent claude skills uninstall --all --force
  assert_success
  assert_output_contains "Uninstalling skill: intent-elixir-essentials"
  assert_output_contains "Uninstalling skill: intent-ash-ecto-essentials"
  assert_output_contains "Removed: 2"

  # Verify all are gone
  run run_intent claude skills list
  assert_success
  assert_output_contains "[NOT INSTALLED]"
}

@test "claude skills uninstall updates manifest" {
  # Install a skill
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  # Verify manifest has the skill
  run jq '.installed[].name' "$HOME/.intent/skills/installed-skills.json"
  assert_success
  assert_output_contains "intent-elixir-essentials"

  # Uninstall
  run run_intent claude skills uninstall intent-elixir-essentials --force
  assert_success

  # Verify manifest no longer has the skill
  run jq '.installed[].name' "$HOME/.intent/skills/installed-skills.json"
  assert_success
  refute_output_contains "intent-elixir-essentials"
}

@test "claude skills uninstall --all handles empty manifest" {
  rm -rf "$HOME/.intent/skills" 2>/dev/null || true

  run run_intent claude skills uninstall --all
  assert_success
  assert_output_contains "No installed skills found"
}

# ====================================================================
# Show command
# ====================================================================

@test "claude skills show requires a skill name" {
  run run_intent claude skills show
  assert_failure
  assert_output_contains "Error: Skill name required"
  assert_output_contains "Usage: intent claude skills show"
}

@test "claude skills show displays skill information" {
  run run_intent claude skills show intent-elixir-essentials
  assert_success
  assert_output_contains "Skill: intent-elixir-essentials"
  assert_output_contains "Content:"
}

@test "claude skills show extracts description from frontmatter" {
  run run_intent claude skills show intent-elixir-essentials
  assert_success
  assert_output_contains "Title: Elixir Essentials"
  assert_output_contains "Description: Elixir coding rules"
}

@test "claude skills show indicates installation status" {
  # Check when not installed
  run run_intent claude skills show intent-elixir-essentials
  assert_success
  assert_output_contains "Status: NOT INSTALLED"
  assert_output_contains "To install: intent claude skills install intent-elixir-essentials"

  # Install and check again
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success

  run run_intent claude skills show intent-elixir-essentials
  assert_success
  assert_output_contains "Status: INSTALLED"
  assert_output_contains "Location: $HOME/.claude/skills/intent-elixir-essentials/SKILL.md"
}

@test "claude skills show handles non-existent skill" {
  run run_intent claude skills show nonexistent
  assert_failure
  assert_output_contains "Error: Skill 'nonexistent' not found"
}

@test "claude skills show works for all available skills" {
  run run_intent claude skills show intent-elixir-essentials
  assert_success
  assert_output_contains "Skill: intent-elixir-essentials"

  run run_intent claude skills show intent-ash-ecto-essentials
  assert_success
  assert_output_contains "Skill: intent-ash-ecto-essentials"

  run run_intent claude skills show intent-phoenix-liveview
  assert_success
  assert_output_contains "Skill: intent-phoenix-liveview"
}

# ====================================================================
# Integration tests
# ====================================================================

@test "claude skills full lifecycle: install, sync, uninstall" {
  # Install
  run run_intent claude skills install intent-elixir-essentials --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/intent-elixir-essentials/SKILL.md"

  # Sync (should be up to date)
  run run_intent claude skills sync
  assert_success
  assert_output_contains "Up to date"

  # Uninstall
  run run_intent claude skills uninstall intent-elixir-essentials --force
  assert_success
  [ ! -d "$HOME/.claude/skills/intent-elixir-essentials" ] || fail "Skill directory should have been removed"

  # List should show not installed
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-elixir-essentials"
  assert_output_contains "[NOT INSTALLED]"
}

@test "claude skills manifest tracks multiple installs correctly" {
  # Install all skills
  run run_intent claude skills install --all --force
  assert_success

  # Record count after install-all
  local count_before
  count_before=$(jq '.installed | length' "$HOME/.intent/skills/installed-skills.json")
  [ "$count_before" -gt 0 ] || fail "Expected at least 1 installed skill"

  # Uninstall one
  run run_intent claude skills uninstall intent-ash-ecto-essentials --force
  assert_success

  # Manifest should have one fewer entry
  local count_after
  count_after=$(jq '.installed | length' "$HOME/.intent/skills/installed-skills.json")
  [ "$count_after" -eq $((count_before - 1)) ] || fail "Expected $((count_before - 1)) installed skills after uninstall, got $count_after"
}
