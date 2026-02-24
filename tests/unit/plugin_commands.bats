#!/usr/bin/env bats
# Tests for intent plugin command

load "../lib/test_helper.bash"

# ====================================================================
# intent plugin list (default)
# ====================================================================

@test "plugin with no args lists plugins" {
  run run_intent plugin
  assert_success
  assert_output_contains "Intent Plugins"
  assert_output_contains "claude"
  assert_output_contains "agents"
}

@test "plugin list shows both plugins" {
  run run_intent plugin list
  assert_success
  assert_output_contains "claude"
  assert_output_contains "agents"
}

@test "plugin list shows command syntax" {
  run run_intent plugin list
  assert_success
  assert_output_contains "intent claude subagents"
  assert_output_contains "intent claude skills"
  assert_output_contains "intent claude upgrade"
  assert_output_contains "intent agents"
}

@test "plugin list shows descriptions" {
  run run_intent plugin list
  assert_success
  assert_output_contains "Claude Code integration for Intent projects"
  assert_output_contains "AGENTS.md management for Intent projects"
}

# ====================================================================
# intent plugin show
# ====================================================================

@test "plugin show claude shows detailed info" {
  run run_intent plugin show claude
  assert_success
  assert_output_contains "Plugin: claude"
  assert_output_contains "Version:"
  assert_output_contains "Description:"
  assert_output_contains "Commands (3):"
  assert_output_contains "intent claude subagents"
  assert_output_contains "intent claude skills"
  assert_output_contains "intent claude upgrade"
}

@test "plugin show agents shows detailed info" {
  run run_intent plugin show agents
  assert_success
  assert_output_contains "Plugin: agents"
  assert_output_contains "Version:"
  assert_output_contains "Description:"
  assert_output_contains "Commands (1):"
  assert_output_contains "intent agents"
}

@test "plugin show missing plugin errors" {
  run run_intent plugin show nonexistent
  assert_failure
  assert_output_contains "Unknown plugin 'nonexistent'"
}

@test "plugin show with no name errors" {
  run run_intent plugin show
  assert_failure
  assert_output_contains "Usage: intent plugin show <name>"
}

# ====================================================================
# intent plugin help
# ====================================================================

@test "plugin help shows usage" {
  run run_intent plugin help
  assert_success
  assert_output_contains "Usage: intent plugin"
  assert_output_contains "list"
  assert_output_contains "show"
  assert_output_contains "help"
}

@test "plugin --help shows usage" {
  run run_intent plugin --help
  assert_success
  assert_output_contains "Usage: intent plugin"
}

# ====================================================================
# Error handling
# ====================================================================

@test "plugin invalid subcommand errors" {
  run run_intent plugin badcmd
  assert_failure
  assert_output_contains "Unknown plugin subcommand 'badcmd'"
}

# ====================================================================
# Global command (works without project context)
# ====================================================================

@test "plugin works from non-project directory" {
  cd /tmp
  run run_intent plugin
  assert_success
  assert_output_contains "Intent Plugins"
  assert_output_contains "claude"
  assert_output_contains "agents"
}
