#!/usr/bin/env bats
# Issue 0025: an inherited PROJECT_ROOT must not decide anything.
#
# `PROJECT_ROOT` is a generic name. A Makefile, direnv, CI, or devbin -- which
# exports it on EVERY invocation -- can all set it, and Intent used to read it
# as an answer: `plugin_get_manifest_path` wrote the project-local manifest
# whenever it was non-empty and the $HOME one otherwise, with nothing ever
# resolving a project. So `intent claude subagents install`, run from any
# directory, wrote into whatever tree the ambient value named, silently.
#
# These guard the property that closes it: Intent resolves the project from the
# filesystem and OVERWRITES anything inherited (`resolve_project_root`, plus the
# `unset PROJECT_ROOT` at the top of `bin/intent`).
#
# The decoy is deliberately a REAL, VALID Intent project. A decoy that could
# never have been selected proves nothing -- if the ambient value is honoured
# this is exactly the tree that would receive the write, so the test can fail
# for the right reason. Mutation-proven: with the `unset` removed and
# `resolve_project_root` reverted to reading the variable, tests 1 and 2 fail.

load "../lib/test_helper.bash"

setup() {
  # `cd ... && pwd` normalises: $TMPDIR carries a trailing slash on macOS, so
  # the raw mktemp path contains `//` and never string-matches what
  # find_project_root returns (which is normalised by its own `pwd`).
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/intent-ambient-XXXXXX")" && pwd)"
  export HOME="$TEST_TEMP_DIR/fakehome"
  # `.claude` is how the CLI detects Claude Code; without it the subagent verbs
  # refuse before they ever reach the path decision under test.
  mkdir -p "$HOME/.claude/agents"

  # A real Intent project, standing in for whatever a parent process might name.
  DECOY="$TEST_TEMP_DIR/decoy"
  mkdir -p "$DECOY/intent/.config"
  cp "${INTENT_PROJECT_ROOT}/intent/.config/config.json" "$DECOY/intent/.config/config.json"

  # Somewhere that is NOT a project, so resolution must come up empty and the
  # $HOME branch is the correct answer.
  OUTSIDE="$TEST_TEMP_DIR/outside"
  mkdir -p "$OUTSIDE"
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

@test "an ambient PROJECT_ROOT does not redirect the subagent manifest" {
  cd "$OUTSIDE" || return 1

  run env PROJECT_ROOT="$DECOY" "${INTENT_PROJECT_ROOT}/bin/intent" \
    claude subagents install intent --force
  assert_success

  # The manifest belongs in $HOME: the cwd is not a project, so no project-local
  # manifest is correct regardless of what the environment claimed.
  assert_file_exists "$HOME/.intent/agents/installed-agents.json"
  [ ! -e "$DECOY/intent/plugins/claude/subagents/.manifest/installed-agents.json" ]
}

@test "an ambient PROJECT_ROOT does not make a non-project look like a project" {
  cd "$OUTSIDE" || return 1

  # `st list` is a project command. Standing outside a project it must refuse,
  # whatever the environment says -- before the fix the dispatcher's own
  # emptiness check was satisfied by the inherited value.
  run env PROJECT_ROOT="$DECOY" "${INTENT_PROJECT_ROOT}/bin/intent" st list
  assert_failure
  echo "$output" | grep -qi "not in an intent project"
}

@test "resolve_project_root overwrites an inherited value rather than trusting it" {
  cd "$OUTSIDE" || return 1

  # The unit-level property, asserted directly on the helper so a future reader
  # of PROJECT_ROOT cannot reintroduce the bug by going around the CLI.
  run env PROJECT_ROOT="$DECOY" bash -c "
    source '${INTENT_PROJECT_ROOT}/bin/intent_helpers'
    cd '$OUTSIDE'
    resolve_project_root
    echo \"resolved=[\$PROJECT_ROOT]\"
  "
  assert_success
  assert_output_contains "resolved=[]"
}

@test "resolve_project_root finds the real project when standing in one" {
  cd "$DECOY" || return 1

  # The complement, so the guard is not merely asserting that everything is
  # empty: from inside a project it must resolve to that project.
  run env PROJECT_ROOT="/nonexistent/decoy" bash -c "
    source '${INTENT_PROJECT_ROOT}/bin/intent_helpers'
    cd '$DECOY'
    resolve_project_root
    echo \"resolved=[\$PROJECT_ROOT]\"
  "
  assert_success
  assert_output_contains "resolved=[$DECOY]"
}
