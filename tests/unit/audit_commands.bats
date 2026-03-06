#!/usr/bin/env bats
# Tests for intent audit commands (v2.6.0)

load "../lib/test_helper.bash"

# ============================================================
# Help and Routing
# ============================================================

@test "audit shows usage when no subcommand given" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit
  assert_success
  assert_output_contains "Usage: intent audit"
  assert_output_contains "quick"
  assert_output_contains "help"
}

@test "audit help shows usage" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit help
  assert_success
  assert_output_contains "Usage: intent audit"
  assert_output_contains "quick"
}

@test "audit --help shows usage" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit --help
  assert_success
  assert_output_contains "Usage: intent audit"
}

@test "audit unknown subcommand shows error" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit bogus
  assert_failure
  assert_output_contains "Unknown audit command: bogus"
}

# ============================================================
# Help System Integration
# ============================================================

@test "intent help shows audit command" {
  run run_intent help
  assert_success
  assert_output_contains "audit"
}

@test "intent help audit shows audit help file" {
  run run_intent help audit
  assert_success
  assert_output_contains "intent audit"
  assert_output_contains "quick"
}

# ============================================================
# Quick: Prerequisites
# ============================================================

@test "audit quick requires mix.exs" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit quick
  assert_failure
  assert_output_contains "No mix.exs found"
}

@test "audit quick with --checks-only copies templates" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  # Create a fake mix.exs
  echo 'defmodule Test.MixProject do end' > mix.exs

  run run_intent audit quick --checks-only
  assert_success
  assert_output_contains "check templates installed"

  # Verify templates were copied
  [ -d "credo_checks" ]
  [ -f "credo_checks/boolean_operators.ex" ]
  [ -f "credo_checks/missing_impl_annotation.ex" ]
  [ -f "credo_checks/debug_artifacts.ex" ]
  [ -f "credo_checks/map_get_on_struct.ex" ]
  [ -f "credo_checks/thick_coordinator.ex" ]
  [ -f "credo_checks/highlander_suspect.ex" ]
  [ -f "credo_checks/dependency_graph.ex" ]
}

@test "audit quick --checks-only force-copies on re-run" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  echo 'defmodule Test.MixProject do end' > mix.exs

  # First run installs
  run run_intent audit quick --checks-only
  assert_success
  assert_output_contains "check templates installed"

  # Second run also force-copies (ensures updates are applied)
  run run_intent audit quick --checks-only
  assert_success
  assert_output_contains "check templates installed"
}

# ============================================================
# Quick: Flag Parsing
# ============================================================

@test "audit quick --rule with invalid rule shows error" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  echo 'defmodule Test.MixProject do end' > mix.exs

  run run_intent audit quick --rule R99
  assert_failure
  assert_output_contains "Unknown rule: R99"
}

@test "audit quick --rule without argument shows error" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  echo 'defmodule Test.MixProject do end' > mix.exs

  run run_intent audit quick --rule
  assert_failure
  assert_output_contains "Missing argument for --rule"
}

@test "audit quick unknown option shows error" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  echo 'defmodule Test.MixProject do end' > mix.exs

  run run_intent audit quick --bogus
  assert_failure
  assert_output_contains "Unknown option: --bogus"
}

# ============================================================
# Template Verification
# ============================================================

@test "credo check templates exist in INTENT_HOME" {
  [ -d "$INTENT_HOME/lib/templates/credo_checks/elixir" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/boolean_operators.ex" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/missing_impl_annotation.ex" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/debug_artifacts.ex" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/map_get_on_struct.ex" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/thick_coordinator.ex" ]
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/highlander_suspect.ex" ]
}

@test "credo check templates contain valid module definitions" {
  for template in "$INTENT_HOME/lib/templates/credo_checks/elixir/"*.ex; do
    grep -q "defmodule Mix.Checks" "$template" || {
      echo "Template $(basename "$template") missing defmodule Mix.Checks"
      return 1
    }
    grep -q "use Credo.Check" "$template" || {
      echo "Template $(basename "$template") missing use Credo.Check"
      return 1
    }
  done
}

@test "credo check templates have 7 files" {
  local count
  count=$(ls -1 "$INTENT_HOME/lib/templates/credo_checks/elixir/"*.ex | wc -l | tr -d ' ')
  [ "$count" -eq 7 ]
}

@test "D11 dependency graph template exists" {
  [ -f "$INTENT_HOME/lib/templates/credo_checks/elixir/dependency_graph.ex" ]
  grep -q "defmodule Mix.Checks.DependencyGraph" "$INTENT_HOME/lib/templates/credo_checks/elixir/dependency_graph.ex"
  grep -q "EX4007" "$INTENT_HOME/lib/templates/credo_checks/elixir/dependency_graph.ex"
}

# ============================================================
# Rules Display
# ============================================================

@test "audit help lists all rules" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit help
  assert_success
  assert_output_contains "R2"
  assert_output_contains "R6"
  assert_output_contains "R7"
  assert_output_contains "R8"
  assert_output_contains "R11"
  assert_output_contains "R15"
  assert_output_contains "D11"
}

@test "audit requires project context" {
  # Running outside a project should fail
  run run_intent audit quick
  assert_failure
  assert_output_contains "Intent project"
}

# ============================================================
# Health Subcommand
# ============================================================

@test "audit health runs without errors in project" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  run run_intent audit health
  assert_success
  assert_output_contains "health:"
  assert_output_contains "ok: healthy"
}

@test "audit health shows SKIP when no lib/ directory" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  run run_intent audit health
  assert_success
  assert_output_contains "[skip]"
}

@test "audit health with --report saves file" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  # Need to init git for timestamp tracking
  git init -q .

  run run_intent audit health --report
  assert_success
  assert_output_contains "saved:"

  # Check report file exists
  [ -d "intent/audit" ]
  local report_file
  report_file=$(ls intent/audit/*-health.md 2>/dev/null | head -1)
  [ -n "$report_file" ]
}

@test "audit health with --diff works" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  git init -q .

  run run_intent audit health --diff
  assert_success
  assert_output_contains "diff"
}

@test "audit health saves timestamp" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  git init -q .
  git config user.email "test@test.com"
  git config user.name "Test"
  git add -A && git commit -q -m "init"

  run run_intent audit health
  assert_success

  [ -f ".intent/last-health-check" ]
}

@test "audit health unknown option shows error" {
  project_dir=$(create_test_project "Audit Health Test")
  cd "$project_dir"

  run run_intent audit health --bogus
  assert_failure
  assert_output_contains "Unknown option: --bogus"
}

@test "audit help shows health subcommand" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  run run_intent audit help
  assert_success
  assert_output_contains "health"
}
