#!/usr/bin/env bats
# Tests for intent modules commands (v2.6.0)

load "../lib/test_helper.bash"

# ============================================================
# Help and Routing
# ============================================================

@test "modules shows usage when no subcommand given" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  run run_intent modules
  assert_success
  assert_output_contains "Usage: intent modules"
  assert_output_contains "check"
  assert_output_contains "find"
}

@test "modules help shows usage" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  run run_intent modules help
  assert_success
  assert_output_contains "Usage: intent modules"
}

@test "modules --help shows usage" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  run run_intent modules --help
  assert_success
  assert_output_contains "Usage: intent modules"
}

@test "modules unknown subcommand shows error" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  run run_intent modules bogus
  assert_failure
  assert_output_contains "Unknown modules command: bogus"
}

# ============================================================
# Help System Integration
# ============================================================

@test "intent help shows modules command" {
  run run_intent help
  assert_success
  assert_output_contains "modules"
}

@test "intent help modules shows modules help file" {
  run run_intent help modules
  assert_success
  assert_output_contains "intent modules"
  assert_output_contains "check"
  assert_output_contains "find"
}

# ============================================================
# Check: Clean Registry
# ============================================================

@test "modules check with matching registry reports clean" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  # Create a MODULES.md with one entry
  mkdir -p bin
  echo '#!/bin/bash' > bin/intent_foo
  chmod +x bin/intent_foo

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry

## Core

| Concern | THE Module     | Notes |
| ------- | -------------- | ----- |
| Foo     | `bin/intent_foo` | Test  |
EOF

  run run_intent modules check
  assert_success
  assert_output_contains "ok: registry matches filesystem"
}

# ============================================================
# Check: Unregistered Files
# ============================================================

@test "modules check detects unregistered files" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  # Create files but empty MODULES.md
  mkdir -p bin
  echo '#!/bin/bash' > bin/intent_foo
  chmod +x bin/intent_foo
  echo '#!/bin/bash' > bin/intent_bar
  chmod +x bin/intent_bar

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry

## Core

| Concern | THE Module       | Notes |
| ------- | ---------------- | ----- |
| Foo     | `bin/intent_foo` | Test  |
EOF

  run run_intent modules check
  assert_failure
  assert_output_contains "unregistered files"
  assert_output_contains "bin/intent_bar"
}

# ============================================================
# Check: Stale Entries
# ============================================================

@test "modules check detects stale entries" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry

## Core

| Concern | THE Module            | Notes |
| ------- | --------------------- | ----- |
| Ghost   | `bin/intent_ghost`    | Gone  |
EOF

  run run_intent modules check
  assert_failure
  assert_output_contains "stale registry entries"
  assert_output_contains "bin/intent_ghost"
}

# ============================================================
# Check: Exit Codes
# ============================================================

@test "modules check exits 0 when clean" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules check
  assert_success
}

@test "modules check exits 1 when issues found" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  mkdir -p bin
  echo '#!/bin/bash' > bin/intent_orphan
  chmod +x bin/intent_orphan

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules check
  assert_failure
  assert_output_contains "issue(s) found"
}

# ============================================================
# Find
# ============================================================

@test "modules find returns matching entries" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry

## Core

| Concern        | THE Module         | Notes          |
| -------------- | ------------------ | -------------- |
| Shared helpers | `bin/intent_helpers` | error(), etc |
EOF

  run run_intent modules find helpers
  assert_success
  assert_output_contains "helpers"
}

@test "modules find with no match reports no results" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules find nonexistent
  assert_failure
  assert_output_contains "no matches"
}

@test "modules find without argument shows error" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules find
  assert_failure
  assert_output_contains "Missing search term"
}

# ============================================================
# No MODULES.md
# ============================================================

@test "modules check errors when no MODULES.md" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  rm -f intent/llm/MODULES.md

  run run_intent modules check
  assert_failure
  assert_output_contains "No MODULES.md found"
}

@test "modules find errors when no MODULES.md" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  rm -f intent/llm/MODULES.md

  run run_intent modules find foo
  assert_failure
  assert_output_contains "No MODULES.md found"
}

# ============================================================
# Project Context Required
# ============================================================

@test "modules requires project context" {
  run run_intent modules check
  assert_failure
  assert_output_contains "Intent project"
}

# ============================================================
# Check: --register flag
# ============================================================

@test "modules check --register shows registration guidance" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  mkdir -p bin
  echo '#!/bin/bash' > bin/intent_unregistered
  chmod +x bin/intent_unregistered

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules check --register
  assert_failure
  assert_output_contains "hint: add these"
  assert_output_contains "bin/intent_unregistered"
}

@test "modules check unknown option shows error" {
  project_dir=$(create_test_project "Modules Test")
  cd "$project_dir"

  cat > intent/llm/MODULES.md << 'EOF'
# Module Registry
EOF

  run run_intent modules check --bogus
  assert_failure
  assert_output_contains "Unknown option: --bogus"
}
