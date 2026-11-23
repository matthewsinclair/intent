#!/usr/bin/env bats
# The commit gate's canon readers read the release format (thread@4.0, ST0096
# WP-05) as well as the flat one: stale_at_check.sh and absent_at_check.sh
# through lib_at_canon.sh, declared_kind_check.sh, and canon_commit_check.sh.
# In the release format a thread file carries no AT row and no attachment:
# each is its own file under intent/.canon/records/<thread>/, and the thread's
# own `realised` takes the place of .intentfiles. The 3.5.0 rehearsal's step 3
# found all four reading nothing in a clone `intent upgrade` had moved, so
# Intent's own commit gate refused every commit after the upgrade.
#
# Each case runs in a throwaway project made by the binary under test, under a
# sandboxed HOME, so every record a tool reads is one that build wrote, in the
# format it writes; no case spells the format. Every arm is red on its base,
# where each tool read only the thread file and refused on finding nothing.

load "../lib/test_helper.bash"

TOOLS_REL="intent/st/ST0056/parity/tools"
TOOLS="${INTENT_PROJECT_ROOT}/$TOOLS_REL"
SCHEMA_REL="schema/thread.schema.json"

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/release-format-XXXXXX")" && pwd)"
  export HOME="$TEST_TEMP_DIR/home"
  PROJ="$TEST_TEMP_DIR/proj"
  mkdir -p "$HOME" "$PROJ"
  unset_user_dirs
  cd "$PROJ" || exit 1
  "$INTENT_BIN" init "Release format" >/dev/null 2>&1 || fail "intent init did not make the fixture project"
  THREAD="$(mint st new "Release checklist")" || fail "st new minted no thread"
  # The tools sit in the project, as they do in Intent's own tree, and read the
  # model's vocabulary from its schema there.
  mkdir -p "$PROJ/$TOOLS_REL" "$PROJ/schema" "$PROJ/tests" "$PROJ/native/rust/target/release"
  local f
  for f in stale_at_check.sh absent_at_check.sh lib_at_canon.sh lib_citation.sh declared_kind_check.sh \
    lib_read_only.sh runner_roster_check.sh canon_commit_check.sh; do
    cp "$TOOLS/$f" "$PROJ/$TOOLS_REL/"
  done
  cp "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" "$PROJ/$SCHEMA_REL"
  ln -sfn "$INTENT_BIN" "$PROJ/native/rust/target/release/intent"
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# verb <verb>... -- run a verb that must succeed.
verb() {
  local out
  out="$("$INTENT_BIN" "$@" 2>&1)" || fail "intent $* refused: $out"
}

# row <AT> <file> -- an AT row on $THREAD citing <file>, covering a criterion of
# its own, at the status `at new` gives it.
row() {
  local ac="AC-${1#AT-}"
  verb ac new "$THREAD" "$ac" --kind test --text "The release checklist holds $1"
  verb at new "$THREAD" "$1" --covers "$ac" --file "$2"
}

# release_format -- the fixture is in the release format, or the case proves
# nothing: the thread files carry no row and no attachment, and the records do.
release_format() {
  local kind="$1" n
  jq -e 'all(has("tests") or has("attachments") | not)' -s "$PROJ"/intent/.canon/st/*.json >/dev/null ||
    fail "a thread file carries rows or attachments, so this is not the release format"
  n="$(find "$PROJ/intent/.canon/records" -path "*/$kind/*" -name '*.json' | grep -c .)"
  [ "$n" -gt 0 ] || fail "no $kind record under intent/.canon/records"
  printf '%s' "$n"
}

sh_for_gate() {
  if [ -x /bin/bash ]; then echo /bin/bash; else echo bash; fi
}

tool() {
  local name="$1"
  shift
  run env ROOT="$PROJ" "$(sh_for_gate)" "$PROJ/$TOOLS_REL/$name.sh" "$@"
}

commit_fixture() {
  git init -q "$PROJ"
  git -C "$PROJ" add -A
  git -C "$PROJ" -c user.name=fixture -c user.email=fixture@example.invalid commit -q -m fixture ||
    fail "the fixture could not be committed"
}

@test "stale_at: a to-write row under records/<thread>/at/ whose cited file exists is reported" {
  verb st start "$THREAD"
  printf 'present\n' >"$PROJ/tests/present.bats"
  row AT-01.1 tests/present.bats
  local rows
  rows="$(release_format at)"
  tool stale_at_check
  assert_success
  assert_output_contains "stale: "
  assert_output_contains "/AT-01.1"
  assert_output_contains "of $rows AT row(s)"
}

@test "absent_at: a red row under records/<thread>/at/ whose cited file is not there is reported" {
  # `at red` refuses a verdict on a file that is not there, so the cited test
  # is written, the verdict set, and the test then removed.
  printf 'gone\n' >"$PROJ/tests/gone.bats"
  row AT-01.1 tests/gone.bats
  verb at red "$THREAD" AT-01.1 --note "written, and red"
  rm "$PROJ/tests/gone.bats"
  local rows
  rows="$(release_format at)"
  tool absent_at_check
  [ "$status" -eq 1 ] || fail "expected exit 1, got $status: $output"
  assert_output_contains "absent: "
  assert_output_contains "/AT-01.1"
  assert_output_contains "of $rows AT row(s)"
}

@test "declared-kind: a row under records/<thread>/at/ citing a rostered instrument is judged" {
  mkdir -p "$PROJ/intent/st/$THREAD/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/$THREAD/parity/tools/"
  row AT-01.1 "intent/st/$THREAD/parity/tools/rulings_check.sh"
  local rows
  rows="$(release_format at)"
  tool declared_kind_check
  assert_success
  assert_output_contains "$rows agreeing"
}

@test "canon-commit: an attachment of a thread that is not realised is checked against its record's text" {
  printf 'the design\n' >"$TEST_TEMP_DIR/design.md"
  verb st attach "$THREAD" design.md --from "$TEST_TEMP_DIR/design.md"
  local n
  n="$(release_format attachments)"
  commit_fixture
  tool canon_commit_check HEAD --exhaustive
  assert_success
  assert_output_contains "$n recorded attachment(s)"
  assert_output_contains "ADDS 0"
}

@test "canon-commit: a realised thread's attachment whose file differs from its record is an ADD this commit refuses" {
  verb st start "$THREAD"
  printf 'the design\n' >"$TEST_TEMP_DIR/design.md"
  verb st attach "$THREAD" design.md --from "$TEST_TEMP_DIR/design.md"
  verb organize --apply
  local n view
  n="$(release_format attachments)"
  view="$(find "$PROJ/intent/st" -name design.md | head -1)"
  [ -n "$view" ] || fail "organize --apply realised no design.md for a started thread"
  printf 'edited by hand\n' >"$view"
  commit_fixture
  tool canon_commit_check HEAD --exhaustive
  [ "$status" -eq 1 ] || fail "expected exit 1, got $status: $output"
  assert_output_contains "$n recorded attachment(s)"
  assert_output_contains "ADDS $n of $n attachment(s) examined"
  assert_output_contains "/design.md"
}

# Issue I06a2146: --history over a commit whose attachment records cannot be
# read refuses, not skips. HEAD reads, so the run reaches the walk; the commit
# before it holds an attachment record cut off mid-value. Without pipefail,
# atts_at's status 2 for that commit was lost in its pipe, so its thread counted
# no attachments and the walk skipped it as unmeasurable, exit 0.
@test "canon-commit: --history over a commit whose attachment records cannot be read refuses, not skips" {
  printf 'the design\n' >"$TEST_TEMP_DIR/design.md"
  verb st attach "$THREAD" design.md --from "$TEST_TEMP_DIR/design.md"
  release_format attachments >/dev/null
  local record held
  record="$(find "$PROJ/intent/.canon/records" -path '*/attachments/*.json' | head -1)"
  [ -n "$record" ] || fail "no attachment record to cut off"
  held="$(cat "$record")"
  printf '{ "path": ' >"$record"
  commit_fixture
  printf '%s\n' "$held" >"$record"
  git -C "$PROJ" add -A
  git -C "$PROJ" -c user.name=fixture -c user.email=fixture@example.invalid commit -q -m repaired ||
    fail "the repair could not be committed"
  tool canon_commit_check HEAD --history 2
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "CANNOT MEASURE"
  refute_output_contains "skipped as unmeasurable"
}

# Issue Ia6e3769: a row file jq cannot read refuses the check. jq printed the
# rows before it and stopped, and the check judged those alone and passed.
@test "declared-kind: a row file it cannot read refuses rather than judging the rows before it" {
  mkdir -p "$PROJ/intent/st/$THREAD/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/$THREAD/parity/tools/"
  row AT-01.1 "intent/st/$THREAD/parity/tools/rulings_check.sh"
  row AT-01.2 "intent/st/$THREAD/parity/tools/rulings_check.sh"
  release_format at >/dev/null
  local cut
  cut="$(find "$PROJ/intent/.canon/records" -path '*/at/AT-01.2.json')"
  [ -n "$cut" ] || fail "no AT-01.2 record to cut off"
  printf '{ "id": "AT-01.2",' >"$cut"
  tool declared_kind_check
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "could not be read"
}
