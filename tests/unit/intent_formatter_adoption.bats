#!/usr/bin/env bats
# INTENT'S OWN ADOPTION of the canon staged-blob format guard (issue 0505, the
# first commit of the guards adoption pass).
#
# WHAT THESE ARMS ARE FOR, AND IT IS NOT THE GUARD'S BEHAVIOUR. That is covered
# by `staged_format_guard.bats`, twelve arms against canon. These arms hold the
# ADOPTION: that this project declares the formatters it used to hand-wire, that
# it no longer carries a second implementation of the same check, and that its
# declaration is spelled in the vocabulary the guard actually knows.
#
# THE HIGHLANDER ARM IS ARM 2 AND IT IS THE LOAD-BEARING ONE. The failure this
# whole pass exists to prevent is an estate hand-wiring a formatter beside the
# canon guard, which is how the fleet ended up with ten divergent copies and the
# re-staging defect of 0498. A comment may NAME a formatter -- the retirement
# note does, at length -- so the arm reads executable lines only.

load "../lib/test_helper.bash"

HOOK="${INTENT_PROJECT_ROOT}/.githooks/pre-commit"
CONFIG="${INTENT_PROJECT_ROOT}/intent/.config/config.json"
ROSTER="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit-guards.sh"

# Executable lines only: everything that is not blank and not a comment.
hook_code() { grep -vE '^[[:space:]]*(#|$)' "$HOOK"; }

@test "Intent declares exactly the three formatters the retired block checked" {
  run jq -r '.formatters // empty | sort | join(",")' "$CONFIG"
  [ "$status" -eq 0 ]
  [ "$output" = "elixir,markdown,rust" ]
}

@test "the hook carries NO second implementation of the format check" {
  run bash -c "grep -vE '^[[:space:]]*(#|\$)' '$HOOK' | grep -nE 'prettier|rustfmt|mix format|formatter_refused'"
  [ "$status" -ne 0 ]
  [ -z "$output" ]

  # Positive control: the same search DOES find the tools in a file that has
  # them, so an empty result above is an absence and not a broken search.
  run bash -c "grep -vE '^[[:space:]]*(#|\$)' '${INTENT_PROJECT_ROOT}/lib/templates/hooks/staged-format-guard.sh' | grep -cE 'prettier|rustfmt'"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

@test "every declared formatter is one the canon guard knows" {
  local declared
  declared="$(jq -r '.formatters[]' "$CONFIG")"
  for f in $declared; do
    run bash -c "grep -cE '\"?${f}\"?\\)' '${INTENT_PROJECT_ROOT}/lib/templates/hooks/staged-format-guard.sh'"
    [ "$status" -eq 0 ]
    [ "$output" -gt 0 ]
  done
}

@test "the roster dispatches the guard, so the declaration is reachable" {
  run grep -c 'staged-format-guard.sh' "$ROSTER"
  [ "$status" -eq 0 ]
  [ "$output" -gt 0 ]
}

@test "the hook still chains the canon gate and still runs the repo-local gates" {
  run bash -c "grep -vE '^[[:space:]]*(#|\$)' '$HOOK' | grep -c 'pre-commit.intent'"
  [ "$output" -gt 0 ]
  run bash -c "grep -vE '^[[:space:]]*(#|\$)' '$HOOK' | grep -c 'bin/int precommit'"
  [ "$output" -gt 0 ]
}
