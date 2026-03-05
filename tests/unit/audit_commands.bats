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
  assert_output_contains "Installed check:"
  assert_output_contains "Check templates installed"

  # Verify templates were copied
  [ -d "lib/mix/checks" ]
  [ -f "lib/mix/checks/boolean_operators.ex" ]
  [ -f "lib/mix/checks/missing_impl_annotation.ex" ]
  [ -f "lib/mix/checks/debug_artifacts.ex" ]
  [ -f "lib/mix/checks/map_get_on_struct.ex" ]
  [ -f "lib/mix/checks/thick_coordinator.ex" ]
  [ -f "lib/mix/checks/highlander_suspect.ex" ]
}

@test "audit quick --checks-only is idempotent" {
  project_dir=$(create_test_project "Audit Test")
  cd "$project_dir"

  echo 'defmodule Test.MixProject do end' > mix.exs

  # First run installs
  run run_intent audit quick --checks-only
  assert_success
  assert_output_contains "Installed check:"

  # Second run should not reinstall
  run run_intent audit quick --checks-only
  assert_success
  refute_output_contains "Installed check:"
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

@test "credo check templates have 6 files" {
  local count
  count=$(ls -1 "$INTENT_HOME/lib/templates/credo_checks/elixir/"*.ex | wc -l | tr -d ' ')
  [ "$count" -eq 6 ]
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
}

@test "audit requires project context" {
  # Running outside a project should fail
  run run_intent audit quick
  assert_failure
  assert_output_contains "Intent project"
}
