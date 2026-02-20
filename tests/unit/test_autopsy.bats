#!/usr/bin/env bats
# Tests for intent-autopsy skill and full directory install (ST0021)

load "../lib/test_helper.bash"

# Setup/teardown for autopsy tests
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
# Skill source existence
# ====================================================================

@test "intent-autopsy SKILL.md exists in source" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/SKILL.md"
}

@test "intent-autopsy scripts directory exists" {
  assert_directory_exists "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/scripts"
}

@test "intent-autopsy autopsy.exs exists" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/scripts/autopsy.exs"
}

@test "intent-autopsy banned-words.txt exists" {
  assert_file_exists "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/scripts/banned-words.txt"
}

@test "intent-autopsy SKILL.md has frontmatter with description" {
  run head -3 "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/SKILL.md"
  assert_success
  assert_output_contains "---"
  assert_output_contains "description:"
}

# ====================================================================
# Skill list includes intent-autopsy
# ====================================================================

@test "claude skills list shows intent-autopsy" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-autopsy"
}

@test "claude skills list shows intent-autopsy as NOT INSTALLED" {
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-autopsy"
  assert_output_contains "[NOT INSTALLED]"
}

# ====================================================================
# Full directory install
# ====================================================================

@test "claude skills install copies SKILL.md for intent-autopsy" {
  run run_intent claude skills install intent-autopsy --force
  assert_success
  assert_output_contains "Installing skill: intent-autopsy"
  assert_output_contains "Installed successfully"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/SKILL.md"
}

@test "claude skills install copies scripts directory for intent-autopsy" {
  run run_intent claude skills install intent-autopsy --force
  assert_success
  assert_directory_exists "$HOME/.claude/skills/intent-autopsy/scripts"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/banned-words.txt"
}

@test "claude skills install --all includes intent-autopsy with scripts" {
  run run_intent claude skills install --all --force
  assert_success
  assert_output_contains "Installing skill: intent-autopsy"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/banned-words.txt"
}

@test "existing skills still install correctly with directory copy" {
  run run_intent claude skills install intent-essentials --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/intent-essentials/SKILL.md"
}

@test "intent-autopsy shows as INSTALLED after install" {
  run run_intent claude skills install intent-autopsy --force
  assert_success

  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-autopsy"
  assert_output_contains "[INSTALLED]"
}

# ====================================================================
# Sync with full directory
# ====================================================================

@test "claude skills sync updates intent-autopsy scripts" {
  # Install first
  run run_intent claude skills install intent-autopsy --force
  assert_success

  # Modify the installed script to simulate a change
  echo "# modified" >> "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"

  # Modify source SKILL.md checksum by touching the installed one
  echo "# checksum change" >> "$HOME/.claude/skills/intent-autopsy/SKILL.md"

  # Sync should detect change and update
  run run_intent claude skills sync --force
  assert_success
  assert_output_contains "intent-autopsy"

  # The modification to autopsy.exs should be overwritten
  run grep "# modified" "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"
  assert_failure
}

# ====================================================================
# Show command
# ====================================================================

@test "claude skills show displays intent-autopsy info" {
  run run_intent claude skills show intent-autopsy
  assert_success
  assert_output_contains "Skill: intent-autopsy"
  assert_output_contains "Description:"
  assert_output_contains "Content:"
}

@test "claude skills show intent-autopsy extracts description from frontmatter" {
  run run_intent claude skills show intent-autopsy
  assert_success
  assert_output_contains "Session forensics"
}

# ====================================================================
# Uninstall
# ====================================================================

@test "claude skills uninstall removes intent-autopsy completely" {
  # Install first
  run run_intent claude skills install intent-autopsy --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"

  # Uninstall
  run run_intent claude skills uninstall intent-autopsy --force
  assert_success
  assert_output_contains "Removed successfully"

  # Verify entire directory is gone
  [ ! -d "$HOME/.claude/skills/intent-autopsy" ] || fail "Skill directory should be removed"
}

# ====================================================================
# Script basic execution
# ====================================================================

@test "autopsy.exs --help shows usage" {
  skip_if_no_elixir
  run elixir "${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/scripts/autopsy.exs" --help
  assert_success
  assert_output_contains "autopsy.exs"
  assert_output_contains "Usage:"
  assert_output_contains "--days"
}

@test "banned-words.txt has expected format" {
  local bw="${INTENT_HOME}/intent/plugins/claude/skills/intent-autopsy/scripts/banned-words.txt"

  # Should have pattern|label lines
  run grep "|" "$bw"
  assert_success

  # Should have ai_ism entries
  run grep "ai_ism" "$bw"
  assert_success

  # Should have deferral entries
  run grep "deferral" "$bw"
  assert_success

  # Should have comments
  run grep "^#" "$bw"
  assert_success
}

# ====================================================================
# Full lifecycle
# ====================================================================

@test "intent-autopsy full lifecycle: install, sync, uninstall" {
  # Install
  run run_intent claude skills install intent-autopsy --force
  assert_success
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/SKILL.md"
  assert_file_exists "$HOME/.claude/skills/intent-autopsy/scripts/autopsy.exs"

  # Sync (should be up to date)
  run run_intent claude skills sync
  assert_success
  assert_output_contains "Up to date"

  # Uninstall
  run run_intent claude skills uninstall intent-autopsy --force
  assert_success
  [ ! -d "$HOME/.claude/skills/intent-autopsy" ] || fail "Skill directory should be removed"

  # List should show not installed
  run run_intent claude skills list
  assert_success
  assert_output_contains "intent-autopsy"
  assert_output_contains "[NOT INSTALLED]"
}

# ====================================================================
# Helpers
# ====================================================================

skip_if_no_elixir() {
  if ! command -v elixir >/dev/null 2>&1; then
    skip "Elixir not installed"
  fi
}
