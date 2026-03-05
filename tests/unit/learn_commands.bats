#!/usr/bin/env bats
# Tests for intent learn command (v2.6.0)

load "../lib/test_helper.bash"

# ============================================================
# Help and Routing
# ============================================================

@test "learn shows usage when no args given" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn
  assert_success
  assert_output_contains "Usage: intent learn"
}

@test "learn help shows usage" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn help
  assert_success
  assert_output_contains "Usage: intent learn"
  assert_output_contains "--category"
  assert_output_contains "--list"
}

@test "learn --help shows usage" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --help
  assert_success
  assert_output_contains "Usage: intent learn"
}

@test "learn unknown option shows error" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --bogus
  assert_failure
  assert_output_contains "Unknown option: --bogus"
}

# ============================================================
# Help System Integration
# ============================================================

@test "intent help shows learn command" {
  run run_intent help
  assert_success
  assert_output_contains "learn"
}

@test "intent help learn shows learn help file" {
  run run_intent help learn
  assert_success
  assert_output_contains "intent learn"
  assert_output_contains "--category"
}

# ============================================================
# Adding Learnings
# ============================================================

@test "learn adds default footgun learning" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn "Never use Map.get on structs"
  assert_success
  assert_output_contains "Added footgun learning"

  # Check file was created
  [ -f ".intent/learnings.md" ]

  # Check content
  grep -q "Never use Map.get on structs" ".intent/learnings.md"
}

@test "learn adds learning with worked category" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --category worked "Ash bulk actions work great"
  assert_success
  assert_output_contains "Added worked learning"

  # Check it's under the right section
  grep -q "Ash bulk actions work great" ".intent/learnings.md"
}

@test "learn adds learning with failed category" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --category failed "Ecto.Multi nesting deadlocks"
  assert_success
  assert_output_contains "Added failed learning"

  grep -q "Ecto.Multi nesting deadlocks" ".intent/learnings.md"
}

@test "learn rejects invalid category" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --category bogus "Some learning"
  assert_failure
  assert_output_contains "Unknown category: bogus"
}

@test "learn --category without argument shows error" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --category
  assert_failure
  assert_output_contains "Missing argument for --category"
}

@test "learn without description shows error" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --category worked
  assert_failure
  assert_output_contains "Missing description"
}

@test "learn creates learnings file if missing" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  [ ! -f ".intent/learnings.md" ]

  run run_intent learn "First learning"
  assert_success

  [ -f ".intent/learnings.md" ]
  assert_output_contains "Created .intent/learnings.md"
}

@test "learn appends multiple learnings to same category" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn "First footgun"
  assert_success

  run run_intent learn "Second footgun"
  assert_success

  grep -c "footgun" ".intent/learnings.md" | grep -q "2"
}

@test "learn adds date prefix to entries" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn "Dated entry"
  assert_success

  local today
  today=$(date +%Y-%m-%d)
  grep -q "$today: Dated entry" ".intent/learnings.md"
}

# ============================================================
# Listing Learnings
# ============================================================

@test "learn --list shows learnings" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn "Test footgun"
  assert_success

  run run_intent learn --list
  assert_success
  assert_output_contains "Test footgun"
  assert_output_contains "Footguns"
}

@test "learn --list with no file shows message" {
  project_dir=$(create_test_project "Learn Test")
  cd "$project_dir"

  run run_intent learn --list
  assert_success
  assert_output_contains "No learnings file found"
}

# ============================================================
# Project Context
# ============================================================

@test "learn requires project context" {
  run run_intent learn "Some learning"
  assert_failure
  assert_output_contains "Intent project"
}
