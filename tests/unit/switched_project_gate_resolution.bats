#!/usr/bin/env bats
# The commit gate must still resolve INTENT_HOME in a project that has been
# switched to v3. It does today BY ACCIDENT, and this file is what converts the
# accident into a contract.
#
# THE CHAIN. `.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` ->
# `$INTENT_HOME/lib/templates/hooks/`. The resolution step is
# `lib/templates/hooks/pre-commit.sh:97`, which parses an `INTENT_HOME:` line
# out of `intent info`'s STDOUT with `sed`. On this machine `intent` is v2.
#
# WHY IT SHOULD BREAK AND DOES NOT. A v2 binary refuses a v3 project -- our own
# record has it at exit 2, and `pre-commit.sh:115` documents the prior instance
# where `intent info` emitted no such line, the sed yielded nothing, and the
# guards ran with no home. **It holds only because `info` is READ-ONLY and v2's
# version refusal is scoped to verbs that WRITE**: `st` in a 3.0.0-dev project
# says "would write to a project built by a newer Intent than this one
# understands", and `info` says nothing of the kind. Nobody designed that
# alignment. It is a property of where the refusal was drawn.
#
# AND THE FAILURE DIRECTION IS THE WORST AVAILABLE: IT FAILS OPEN. Narrow what
# `info` answers, or extend the version gate to cover read verbs, and the gate
# stops resolving -- silently, in every switched project, with no error and no
# missing file. The symptom is unguarded commits, discovered later or never.
# **An accidental invariant is one nobody knows they are breaking**, so the only
# available remedy is to make it say so at the moment it is broken.
#
# THIS EXECUTES THE HOOK'S OWN LINE, GREPPED OUT OF THE SHIPPED FILE AND
# `eval`ed WITH ITS INPUT VARIABLE BOUND -- it does not restate the expression.
# A re-implementation would agree with the hook exactly until one of them moved,
# and then this file would pass about a line the gate no longer runs, which is
# the defect it exists to catch, one level up. **The first version of this file
# did exactly that** and claimed otherwise in a comment; see `run_hook_resolution`.
# Same reason `intent-cli/tests/migrated_guards_still_refuse.rs` reads its line
# out of the file rather than restating it.

load "../lib/test_helper"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-switched-XXXXXX)"
  PROJ="$TEST_TEMP_DIR/switched"
  mkdir -p "$PROJ/intent/.config"
  # Pinned to a v3 string DELIBERATELY, not taken from INTENT_FIXTURE_VERSION.
  # The whole subject is a v3-declaring project driven by the v2 binary, so a
  # fixture that tracks VERSION would silently stop testing it at the cutover.
  cat > "$PROJ/intent/.config/config.json" <<JSON
{
  "intent_version": "3.0.0-dev",
  "project_name": "Switched Project",
  "author": "test_user"
}
JSON
}

teardown() {
  [ -d "${TEST_TEMP_DIR}" ] && rm -rf "${TEST_TEMP_DIR}"
  return 0
}

# The file under test. A variable, not a literal, ONLY so the coupling below can
# itself be mutation-tested against a doctored copy -- the default is the shipped
# file and nothing in normal use overrides it.
hook_file() {
  printf '%s' "${INTENT_TEST_HOOK_FILE:-${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh}"
}

# The gate's own resolution line, lifted from the shipped file.
hook_sed_line() {
  grep 'INTENT_HOME_RESOLVED=' "$(hook_file)" | grep 'sed -n' | head -1
}

# EXECUTE the gate's line against <output>, rather than a copy of it.
#
# THIS IS THE FIX FOR A REAL DEFECT IN THE FIRST VERSION OF THIS FILE (vc caught
# it, 2026-08-21). That version asserted with a hand-typed duplicate of the `sed`
# and claimed in a comment that the two "cannot drift". They could: arm 1 checks
# the line EXISTS, so a MODIFICATION to the expression left arm 1 green while the
# arms below went on testing the old expression -- **green, about a line nobody
# runs**, which is the exact failure this file exists to catch, one level up.
# The copy worked, and a working copy gives you nothing to notice.
#
# `eval` is deliberate and its input is a line grepped from a tracked file in
# this repo at test time -- the same trust as sourcing the hook, which is what
# the gate does with it anyway. The line reads `$wb_info_out` and writes
# `INTENT_HOME_RESOLVED`, so both are bound here to match its contract.
run_hook_resolution() {
  local wb_info_out="$1" line INTENT_HOME_RESOLVED=""
  line="$(hook_sed_line)"
  [ -n "$line" ] || return 1
  eval "$line"
  printf '%s' "$INTENT_HOME_RESOLVED"
}

@test "the gate's resolution line is present -- a PRESENCE check, not a content one" {
  run hook_sed_line
  [ "$status" -eq 0 ]
  [ -n "$output" ]
  # If this fails the gate was refactored and the rest of this file is asserting
  # about a line nobody runs. That is a real failure, not a maintenance chore.
}

@test "v2 intent info exits 0 in a project declaring 3.0.0-dev" {
  cd "$PROJ"
  run "$INTENT_BIN" info
  [ "$status" -eq 0 ]
}

@test "the gate can resolve INTENT_HOME from v2 info in a switched project" {
  cd "$PROJ"
  run "$INTENT_BIN" info
  [ "$status" -eq 0 ]
  resolved="$(run_hook_resolution "$output")"
  [ -n "$resolved" ]
  [ -d "$resolved" ]
}

@test "control: the same extraction yields NOTHING when info emits no such line" {
  # Without this arm the three tests above pass for a sed that matches anything,
  # and a green would say only that something was printed.
  resolved="$(run_hook_resolution "$(printf 'Intent: The Steel Thread Process\n  Version: 2.19.0\n')")"
  [ -z "$resolved" ]
}
