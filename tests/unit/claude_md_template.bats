#!/usr/bin/env bats
# Tests for lib/templates/llm/_CLAUDE.md (ST0035/WP-09).
#
# Scenarios: template is under the length budget, contains the required
# canon landmarks, uses the canonical [[PLACEHOLDER]] syntax, and is
# sed-substituted correctly by `intent init` on a scratch project.

load "../lib/test_helper.bash"

TEMPLATE="${INTENT_PROJECT_ROOT}/lib/templates/llm/_CLAUDE.md"

@test "template exists" {
  [ -f "$TEMPLATE" ]
}

@test "template is within the 100-line budget" {
  local n
  n=$(wc -l < "$TEMPLATE")
  [ "$n" -le 100 ] || fail "template exceeds 100 lines ($n)"
}

@test "template references AGENTS.md as the primary contract" {
  assert_file_contains "$TEMPLATE" "AGENTS.md"
  assert_file_contains "$TEMPLATE" "primary"
}

@test "template directs Claude to run /in-session" {
  assert_file_contains "$TEMPLATE" "/in-session"
  assert_file_contains "$TEMPLATE" "compact"
}

@test "template references persistent memory directory" {
  assert_file_contains "$TEMPLATE" "~/.claude/projects"
  assert_file_contains "$TEMPLATE" "memory"
}

@test "template references .claude/settings.json hooks" {
  assert_file_contains "$TEMPLATE" ".claude/settings.json"
  assert_file_contains "$TEMPLATE" "session-hook-architecture"
}

@test "template includes a file map with canon landmarks" {
  assert_file_contains "$TEMPLATE" "usage-rules.md"
  assert_file_contains "$TEMPLATE" "working-with-llms.md"
  assert_file_contains "$TEMPLATE" "intent/llm/MODULES.md"
  assert_file_contains "$TEMPLATE" "intent/llm/DECISION_TREE.md"
}

@test "template cross-references rule IDs without duplicating rule text" {
  assert_file_contains "$TEMPLATE" "IN-AG-HIGHLANDER-001"
  assert_file_contains "$TEMPLATE" "IN-AG-PFIC-001"
  assert_file_contains "$TEMPLATE" "IN-AG-THIN-COORD-001"
  assert_file_contains "$TEMPLATE" "IN-AG-NO-SILENT-001"
  assert_file_contains "$TEMPLATE" "intent/plugins/claude/rules/agnostic"
}

@test "template includes critic dispatch section" {
  assert_file_contains "$TEMPLATE" "critic-"
  assert_file_contains "$TEMPLATE" "Task(subagent_type"
  assert_file_contains "$TEMPLATE" "intent/docs/critics.md"
}

@test "template includes user-preservation markers" {
  assert_file_contains "$TEMPLATE" "user:start"
  assert_file_contains "$TEMPLATE" "user:end"
}

@test "template uses the four canonical placeholders" {
  assert_file_contains "$TEMPLATE" "[[PROJECT_NAME]]"
  assert_file_contains "$TEMPLATE" "[[INTENT_VERSION]]"
  assert_file_contains "$TEMPLATE" "[[AUTHOR]]"
  assert_file_contains "$TEMPLATE" "[[DATE]]"
}

@test "intent init on a scratch project substitutes all placeholders" {
  local project_dir="${TEST_TEMP_DIR}/scratch"
  mkdir -p "$project_dir"
  cd "$project_dir" || exit 1

  INTENT_AUTHOR="TestUser" run run_intent init "ScratchProj"
  assert_success
  assert_file_exists "$project_dir/CLAUDE.md"

  # No placeholder should remain in the generated CLAUDE.md.
  run grep -F "[[" "$project_dir/CLAUDE.md"
  [ "$status" -ne 0 ] || fail "unsubstituted placeholders remain in generated CLAUDE.md"

  # Project name + author substituted from CLI arg / env.
  assert_file_contains "$project_dir/CLAUDE.md" "ScratchProj"
  assert_file_contains "$project_dir/CLAUDE.md" "TestUser"
}
