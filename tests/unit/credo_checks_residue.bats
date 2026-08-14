#!/usr/bin/env bats
# Tests for the `intent doctor` check 4e: report a leftover credo_checks/ from
# the retired `st zero` D5a deliverable (issue 0021).
#
# D5a copied six custom Credo checks into consumer trees and then TRIED to
# register them in .credo.exs -- skipping the registration entirely when
# `elixir` was off PATH, and reducing it to a printed warning when it failed,
# while the copy itself was unconditional. The usual outcome was a directory of
# checks no runner ever loaded, duplicating concerns the rule library and the
# critic pre-commit gate already enforce. Reported by Laksa, which compiled them
# into every build for five months and executed them zero times.
#
# Three states are three different stories with the same remedy, and conflating
# them is the failure this check exists to prevent:
#   never-wired -- inert, safe to delete;
#   wired       -- they DO run, and a pre-1.7-API check crashes `credo --strict`;
#   stale       -- files gone, .credo.exs still naming them.
#
# It reports and never acts (0018's rule), and it names BOTH ends, because
# deleting the directory without its elixirc_paths entries breaks the build
# (0017's rule: a report that names one end damages everything after it).

load "../lib/test_helper.bash"

wired_credo_exs() {
  printf '%%{\n  configs: [%%{name: "default", requires: ["credo_checks/"], checks: []}]\n}\n' > "$1/.credo.exs"
}

mix_with_elixirc_paths() {
  printf 'defmodule App.MixProject do\n  defp elixirc_paths(:dev), do: ["lib", "credo_checks"]\n  defp elixirc_paths(:test), do: ["lib", "credo_checks"]\nend\n' > "$1/mix.exs"
}

@test "doctor: check passes when no credo_checks/ exists" {
  local project="$(create_test_project "credo-clean")"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"checking: leftover credo_checks ok"* ]]
}

@test "doctor: an unwired credo_checks/ is reported as never having run" {
  local project="$(create_test_project "credo-unwired")"
  mkdir -p "$project/credo_checks"
  touch "$project/credo_checks/highlander_suspect.ex"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"have never run"* ]]
  [[ "$output" == *"git rm -r credo_checks/"* ]]
}

@test "doctor: a wired credo_checks/ is reported as live, with the Credo version caveat" {
  local project="$(create_test_project "credo-wired")"
  mkdir -p "$project/credo_checks"
  touch "$project/credo_checks/highlander_suspect.ex"
  wired_credo_exs "$project"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"DO run"* ]]
  [[ "$output" == *"pre-1.7 API"* ]]
  # Must NOT tell the user to delete a directory that is load-bearing.
  [[ "$output" != *"git rm -r credo_checks/"* ]] || fail "told the user to delete a wired credo_checks/"
}

@test "doctor: a stale .credo.exs registration is reported when the directory is gone" {
  local project="$(create_test_project "credo-stale")"
  wired_credo_exs "$project"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"stale registration"* ]]
}

@test "doctor: the mix.exs elixirc_paths end is named, with line numbers" {
  local project="$(create_test_project "credo-both-ends")"
  mkdir -p "$project/credo_checks"
  touch "$project/credo_checks/highlander_suspect.ex"
  mix_with_elixirc_paths "$project"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"elixirc_paths"* ]]
  # Both offending lines quoted, so the user can act without hunting.
  [[ "$output" == *"mix.exs:2:"* ]]
  [[ "$output" == *"mix.exs:3:"* ]]
}

@test "doctor: residue warns and never errors, so it cannot abort a release" {
  # bin/release gates on `intent doctor` and aborts on a non-zero exit. A
  # consumer carrying this residue must still be able to cut a release.
  local project="$(create_test_project "credo-exit-code")"
  mkdir -p "$project/credo_checks"
  touch "$project/credo_checks/highlander_suspect.ex"
  mix_with_elixirc_paths "$project"
  cd "$project" || exit 1

  run "${INTENT_BIN_DIR}/intent" doctor

  [ "$status" -eq 0 ]
  [[ "$output" == *"0 errors"* ]]
}
