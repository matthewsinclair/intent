#!/usr/bin/env bats
# intent/st/ST0056/parity/tools/stale_at_check.sh -- a to-write AT row whose
# cited file exists is reported, and the rows are read from the thread canon,
# never from a rendered acceptance.md (ST0096 AC-03.1). A thread's acceptance
# view is an aggregate view that leaves git, so the check must read the same
# rows whether the views are tracked, untracked or absent.
#
# Every case runs over a throwaway project holding the tool and its citation
# library at their own paths and a thread canon the case writes, so the verdict
# is about what each case plants. Every expected count is read from the planted
# canon with jq, never written down.

load "../lib/test_helper.bash"

TOOLS_REL="intent/st/ST0056/parity/tools"

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/stale-at-XXXXXX")" && pwd)"
  PROJ="$TEST_TEMP_DIR/proj"
  mkdir -p "$PROJ/$TOOLS_REL" "$PROJ/intent/.config" "$PROJ/intent/.canon/st" "$PROJ/tests"
  printf '{}\n' >"$PROJ/intent/.config/config.json"
  cp "${INTENT_PROJECT_ROOT}/$TOOLS_REL/stale_at_check.sh" "${INTENT_PROJECT_ROOT}/$TOOLS_REL/lib_citation.sh" "$PROJ/$TOOLS_REL/"
  printf 'present\n' >"$PROJ/tests/present.bats"
}

teardown() {
  if [ -n "${TEST_TEMP_DIR:-}" ]; then
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# thread <id> <status> <tests-json> -- writes a thread's canon.
thread() {
  printf '{"status":"%s","tests":%s}\n' "$2" "$3" >"$PROJ/intent/.canon/st/$1.json"
}

# at <id> <status> <file> -- one AT row as the canon carries it.
at() {
  printf '{"id":"%s","kind":"test","status":"%s","file":"%s"}' "$1" "$2" "$3"
}

# live_rows / live_cited -- the population the planted canon holds, over the
# threads that are neither not-started nor cancelled.
live_rows() {
  jq -s '[.[] | select(.status != "not-started" and .status != "cancelled") | .tests[]] | length' "$PROJ"/intent/.canon/st/*.json
}
live_cited() {
  jq -s '[.[] | select(.status != "not-started" and .status != "cancelled") | .tests[] | select(.status == "to-write" and .file != "")] | length' "$PROJ"/intent/.canon/st/*.json
}

# skipped -- how many planted threads are not-started or cancelled.
skipped() {
  jq -s '[.[] | select(.status == "not-started" or .status == "cancelled")] | length' "$PROJ"/intent/.canon/st/*.json
}

check() {
  run bash "$PROJ/$TOOLS_REL/stale_at_check.sh" "$@"
}

@test "with no acceptance.md on disk the rows are read from canon" {
  thread ST0001 wip "[$(at AT-01.1 to-write tests/absent.bats),$(at AT-01.2 green tests/present.bats),$(at AT-01.3 to-write '')]"
  [ -z "$(find "$PROJ/intent" -name acceptance.md)" ]
  check
  assert_success
  assert_output_contains "ok: examined $(live_cited) to-write row(s) with a citation, of $(live_rows) AT row(s)"
}

@test "a to-write row whose cited file exists is reported with its thread" {
  thread ST0001 wip "[$(at AT-01.1 to-write tests/present.bats),$(at AT-01.2 to-write tests/absent.bats)]"
  check
  assert_success
  assert_output_contains "stale: ST0001/AT-01.1"
  assert_output_contains "cites tests/present.bats -- the file EXISTS"
  refute_output_contains "AT-01.2"
}

@test "a citation carrying a line is checked for its file" {
  thread ST0001 wip "[$(at AT-01.1 to-write tests/present.bats:12)]"
  check
  assert_success
  assert_output_contains "stale: ST0001/AT-01.1"
}

@test "a rendered view that disagrees with canon is not read" {
  thread ST0001 wip "[$(at AT-01.1 green tests/present.bats)]"
  mkdir -p "$PROJ/intent/st/ST0001"
  printf -- '- AT-01.1 `tests/present.bats` -- covers -- status: to-write\n' >"$PROJ/intent/st/ST0001/acceptance.md"
  check
  assert_success
  assert_output_contains "examined $(live_cited) to-write row(s)"
  refute_output_contains "stale:"
}

@test "a not-started or cancelled thread is skipped and named on the count line" {
  thread ST0001 wip "[$(at AT-01.1 to-write tests/absent.bats)]"
  thread ST0002 not-started "[$(at AT-01.1 to-write tests/present.bats)]"
  thread ST0003 cancelled "[$(at AT-01.1 to-write tests/present.bats)]"
  check
  assert_success
  assert_output_contains "of $(live_rows) AT row(s) (skipped $(skipped) not-started/cancelled thread(s): ST0002(not-started) ST0003(cancelled))"
  refute_output_contains "stale:"
}

@test "a named thread is read alone, and an unknown one refuses" {
  thread ST0001 wip "[$(at AT-01.1 to-write tests/present.bats)]"
  thread ST0002 wip "[$(at AT-01.1 to-write tests/absent.bats)]"
  check ST0002
  assert_success
  refute_output_contains "stale:"
  check ST0009
  [ "$status" -eq 1 ]
  assert_output_contains "no thread canon for ST0009"
}

@test "a row with no status refuses rather than reporting clean" {
  thread ST0001 wip '[{"id":"AT-01.1","kind":"test","state":"to-write","file":"tests/present.bats"}]'
  check
  [ "$status" -eq 2 ]
  assert_output_contains "ST0001/AT-01.1: its canon row has no status word"
}

@test "a status that is present but not a status word refuses" {
  thread ST0001 wip "[$(at AT-01.1 Green tests/present.bats)]"
  check
  [ "$status" -eq 2 ]
  assert_output_contains "ST0001/AT-01.1: its canon row has no status word"
}

@test "no live thread refuses, and so does a canon jq cannot read" {
  thread ST0001 cancelled "[$(at AT-01.1 to-write tests/absent.bats)]"
  check
  [ "$status" -eq 1 ]
  assert_output_contains "no live thread canon"
  printf '{"status":"wip","tests":[' >"$PROJ/intent/.canon/st/ST0002.json"
  check
  [ "$status" -eq 2 ]
  assert_output_contains "could not be read as JSON"
}
