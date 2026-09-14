#!/usr/bin/env bats
# intent/st/ST0056/parity/tools/rulings_check.sh -- a ruling whose record names a
# thread file resolves through that thread's canon once the thread is
# dehydrated (issue 0347).
#
# Dehydration takes a completed thread's files off the disk and keeps all of them
# in its canon, so reading such a record as dangling kept threads realised for
# this script's sake. Every case runs over a throwaway root with a one-ruling
# table, so the verdict is about the record and nothing the live table carries.

load "../lib/test_helper.bash"

setup() {
  CHECK="${INTENT_PROJECT_ROOT}/intent/st/ST0056/parity/tools/rulings_check.sh"
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/rulings-XXXXXX")" && pwd)"
  mkdir -p "$TEST_TEMP_DIR/intent/.canon/st" "$TEST_TEMP_DIR/st"
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# One ratified ruling whose record is $1, checked over the throwaway root.
check_record() {
  printf '{"invariants":[{"id":"INV-T","target":{"rulings":[{"state":"ratified","authority":"vc","date":"2026-09-14","record":"%s"}]}}],"families":[],"new_surface":[]}\n' "$1" >"$TEST_TEMP_DIR/table.json"
  run env REPO_ROOT="$TEST_TEMP_DIR" ST_DIR="$TEST_TEMP_DIR/st" TABLE="$TEST_TEMP_DIR/table.json" \
    ISSUES_DIR="$TEST_TEMP_DIR/intent/issues" ISSUES_CANON="$TEST_TEMP_DIR/intent/.canon/issues" \
    bash "$CHECK"
}

@test "a dehydrated thread's generated view resolves through its canon" {
  printf '{"attachments":[]}\n' >"$TEST_TEMP_DIR/intent/.canon/st/ST0001.json"
  check_record "intent/st/ST0001/acceptance.md"
  echo "$output"
  [ "$status" -eq 0 ]
  [[ "$output" == *"1 conform"* ]]
}

@test "a dehydrated thread's attachment resolves when its canon lists it" {
  printf '{"attachments":[{"path":"design.md","text":"x","bytes":1,"sha256":"0"}]}\n' >"$TEST_TEMP_DIR/intent/.canon/st/ST0001.json"
  check_record "intent/st/ST0001/design.md"
  echo "$output"
  [ "$status" -eq 0 ]
  [[ "$output" == *"1 conform"* ]]
}

@test "a thread file with no canon behind it is still dangling" {
  check_record "intent/st/ST0002/acceptance.md"
  echo "$output"
  [ "$status" -eq 1 ]
  [[ "$output" == *"1 dangling"* ]]
}

@test "an attachment its thread's canon does not list is still dangling" {
  printf '{"attachments":[]}\n' >"$TEST_TEMP_DIR/intent/.canon/st/ST0001.json"
  check_record "intent/st/ST0001/notes.md"
  echo "$output"
  [ "$status" -eq 1 ]
  [[ "$output" == *"1 dangling"* ]]
}
