#!/usr/bin/env bats
# The commit gate's parity scripts ask the binary's id door, read-only, in place
# of an id pattern (ST0095 AC-12.1, hv decision 79): rulings_check.sh,
# declared_kind_check.sh and view_skew_check.sh, through lib_read_only.sh.
#
# Each case runs in a throwaway project made by the binary under test, under a
# sandboxed HOME, with that binary linked as the project's own pair, so the
# thread and issue it cites are ones that build minted, in the family it mints.
# The arms that cite a hashed thread or issue are red on their base, where each
# script matched a four-digit ordinal; the arms named as regressions are not,
# and say so.

load "../lib/test_helper.bash"

TOOLS="${INTENT_PROJECT_ROOT}/intent/st/ST0056/parity/tools"

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/gated-id-door-XXXXXX")" && pwd)"
  export HOME="$TEST_TEMP_DIR/home"
  PROJ="$TEST_TEMP_DIR/proj"
  mkdir -p "$HOME" "$PROJ"
  unset_user_dirs
  cd "$PROJ" || exit 1
  "$INTENT_BIN" init "Gated id door" >/dev/null 2>&1 || fail "intent init did not make the fixture project"
  THREAD="$("$INTENT_BIN" st new "Release checklist" 2>&1 | awk '$1 == "created:" { print $2; exit }')"
  [ -n "$THREAD" ] || fail "st new printed no created: line"
  ISSUE="$("$INTENT_BIN" issues add "A loose end" 2>&1 | awk '$1 == "created:" { print $2; exit }')"
  [ -n "$ISSUE" ] || fail "issues add printed no created: line"
  # The arms below claim hashed records, and the door, not a pattern, says so: a
  # hashed id is shown by a short form that is not its stored id.
  "$INTENT_BIN" id --format json "$THREAD" "$ISSUE" |
    jq -e 'all(.answers[]; .answer == "id" and .shown != .id)' >/dev/null ||
    fail "the build under test did not mint hashed ids for $THREAD and $ISSUE"
  pair "$INTENT_BIN"
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

# pair <binary> -- link <binary> as the fixture project's own pair, where the
# gated scripts look for the tree's build.
pair() {
  mkdir -p "$PROJ/native/rust/target/release"
  ln -sfn "$1" "$PROJ/native/rust/target/release/intent"
}

# old_pair -- a pair from before `intent id --read-only`: this binary refuses an
# unknown flag with clap's line on stderr, no report, and exit 1.
old_pair() {
  mkdir -p "$TEST_TEMP_DIR/old"
  printf '#!/bin/bash\necho "error: unexpected argument %s found" >&2\nexit 1\n' "'--read-only'" >"$TEST_TEMP_DIR/old/intent"
  chmod +x "$TEST_TEMP_DIR/old/intent"
  pair "$TEST_TEMP_DIR/old/intent"
}

# The scripts promise bash 3.2, so they run under /bin/bash where that is the
# system bash, and under the bash on PATH elsewhere.
sh_for_gate() {
  if [ -x /bin/bash ]; then echo /bin/bash; else echo bash; fi
}

# rulings <record> -- rulings_check.sh over a one-ruling table whose record is
# <record>, in the fixture project.
rulings() {
  mkdir -p "$TEST_TEMP_DIR/st"
  printf '{"invariants":[{"id":"INV-T","target":{"rulings":[{"state":"ratified","authority":"vc","date":"2026-09-14","record":"%s"}]}}],"families":[],"new_surface":[]}\n' "$1" >"$TEST_TEMP_DIR/table.json"
  RULINGS_N="$(jq '[.invariants[].target.rulings[]] | length' "$TEST_TEMP_DIR/table.json")"
  run env REPO_ROOT="$PROJ" ST_DIR="$TEST_TEMP_DIR/st" TABLE="$TEST_TEMP_DIR/table.json" \
    "$(sh_for_gate)" "$TOOLS/rulings_check.sh"
}

# declared_kind <path> -- declared_kind_check.sh over one AT row citing <path>,
# with the real roster beside it, in the fixture project.
declared_kind() {
  mkdir -p "$PROJ/intent/st/ST0056/parity/tools"
  cp "$TOOLS/runner_roster_check.sh" "$PROJ/intent/st/ST0056/parity/tools/"
  printf 'T\tAT-01.1\tgreen\t%s\n' "$1" >"$TEST_TEMP_DIR/cites.tsv"
  CITES_N="$(grep -c . "$TEST_TEMP_DIR/cites.tsv")"
  run env ROOT="$PROJ" "$(sh_for_gate)" "$TOOLS/declared_kind_check.sh" --cites "$TEST_TEMP_DIR/cites.tsv"
}

# view_skew <path>... -- view_skew_check.sh on a change carrying <path>s.
view_skew() {
  run env ROOT="$PROJ" "$(sh_for_gate)" "$TOOLS/view_skew_check.sh" --changed "$@"
}

# package_view -- mint a package under $THREAD and print its view's path, which is
# the door's answer too, whichever form of the package's id the directory carries.
package_view() {
  local wp view
  wp="$("$INTENT_BIN" wp new "$THREAD" "First cut" 2>&1 | awk '$1 == "created:" { print $2; exit }')"
  [ -n "$wp" ] || fail "wp new printed no created: line"
  view="$("$INTENT_BIN" id --format json "$wp" | jq -r '.answers[0].path')"
  case "$view" in *.md) ;; *) view="$view/info.md" ;; esac
  [ "${view#intent/st/"$THREAD"/}" != "$view" ] || fail "the door realises $wp at $view, not under its thread"
  printf '%s' "$view"
}

store_sum() {
  # What the store says: the database's bytes, and how many bytes of frames its
  # write-ahead log holds (absent and empty are both none). A read-only open may
  # leave an empty log and a shared-memory index behind, which carry no write, as
  # cc's id_read_only_never_writes_the_store.rs judges it.
  local db="$PROJ/intent/.cache/intent.db" wal=0
  [ -f "$db" ] || return 0
  [ -f "$db-wal" ] && wal="$(wc -c <"$db-wal" | tr -d ' ')"
  printf '%s wal-bytes=%s\n' "$(shasum -a 256 "$db" | cut -d' ' -f1)" "$wal"
}

# --- rulings_check.sh ------------------------------------------------------

@test "rulings: a hashed thread's generated view resolves through the canon the door names" {
  [ ! -e "$PROJ/intent/st/$THREAD/acceptance.md" ] || rm "$PROJ/intent/st/$THREAD/acceptance.md"
  rulings "intent/st/$THREAD/acceptance.md"
  assert_success
  assert_output_contains "$RULINGS_N conform"
}

@test "rulings: a hashed thread's work package view resolves through its canon" {
  local view
  view="$(package_view)" || fail "no package view to cite"
  [ ! -e "$PROJ/$view" ] || rm "$PROJ/$view"
  rulings "$view"
  assert_success
  assert_output_contains "$RULINGS_N conform"
}

@test "rulings: a hashed thread's attachment resolves when its canon carries it" {
  printf '# Design\n' >"$TEST_TEMP_DIR/design.md"
  "$INTENT_BIN" st attach "$THREAD" design.md --from "$TEST_TEMP_DIR/design.md" >/dev/null 2>&1 || fail "st attach refused"
  [ ! -e "$PROJ/intent/st/$THREAD/design.md" ] || rm "$PROJ/intent/st/$THREAD/design.md"
  rulings "intent/st/$THREAD/design.md"
  assert_success
  assert_output_contains "$RULINGS_N conform"
}

@test "rulings: a hashed issue's record resolves by the door" {
  rulings "issue $ISSUE"
  assert_success
  assert_output_contains "$RULINGS_N conform"
}

@test "rulings: regression, no red on its base -- a directory the door does not answer as a thread dangles" {
  rulings "intent/st/ST9999/acceptance.md"
  [ "$status" -eq 1 ] || fail "expected exit 1, got $status: $output"
  assert_output_contains "$RULINGS_N dangling"
}

@test "rulings: regression, no red on its base -- an attachment its thread's canon does not carry dangles" {
  rulings "intent/st/$THREAD/notes.md"
  [ "$status" -eq 1 ] || fail "expected exit 1, got $status: $output"
  assert_output_contains "$RULINGS_N dangling"
}

@test "rulings: regression, no red on its base -- an issue the project does not hold dangles" {
  rulings "issue 9999"
  [ "$status" -eq 1 ] || fail "expected exit 1, got $status: $output"
  assert_output_contains "$RULINGS_N dangling"
}

@test "rulings: a pair without id --read-only refuses with its remedy, and never reads as dangling or clean" {
  old_pair
  rulings "issue $ISSUE"
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "bin/devbin build all"
  refute_output_contains "dangling"
}

@test "rulings: a tree with no shared release build is judged by its private one, and says so" {
  rm -rf "$PROJ/native/rust/target/release"
  mkdir -p "$PROJ/native/rust/target/private/release"
  ln -sfn "$INTENT_BIN" "$PROJ/native/rust/target/private/release/intent"
  rulings "issue $ISSUE"
  assert_success
  assert_output_contains "judged by its private one"
  assert_output_contains "$RULINGS_N conform"
}

@test "rulings: with no store, the door answers from the committed canon and no store is made" {
  # A project with no store, as a fresh clone is: the canon is all there is to
  # know, so the door answers from it in memory, in place, and stops for nothing.
  rm -rf "$PROJ/intent/.cache"
  rulings "issue $ISSUE"
  assert_success
  assert_output_contains "$RULINGS_N conform"
  refute_output_contains "runs on a copy of intent/"
  [ ! -e "$PROJ/intent/.cache/intent.db" ] || fail "the check made the project's store"
}

# --- declared_kind_check.sh ------------------------------------------------

@test "declared-kind: a rostered script in a hashed thread's parity tools is judged as an instrument" {
  mkdir -p "$PROJ/intent/st/$THREAD/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/$THREAD/parity/tools/"
  declared_kind "intent/st/$THREAD/parity/tools/rulings_check.sh"
  assert_success
  assert_output_contains "$CITES_N agreeing"
}

@test "declared-kind: a script under a thread-shaped directory the door does not answer as a thread is unjudged, where its base judged it by its shape" {
  mkdir -p "$PROJ/intent/st/ST9999/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/ST9999/parity/tools/"
  declared_kind "intent/st/ST9999/parity/tools/rulings_check.sh"
  assert_success
  assert_output_contains "$CITES_N unjudged"
}

@test "declared-kind: a pair without id --read-only refuses with its remedy" {
  mkdir -p "$PROJ/intent/st/$THREAD/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/$THREAD/parity/tools/"
  old_pair
  declared_kind "intent/st/$THREAD/parity/tools/rulings_check.sh"
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "bin/devbin build all"
}

@test "declared-kind: the door leaves the project's store byte-identical" {
  mkdir -p "$PROJ/intent/st/$THREAD/parity/tools"
  cp "$TOOLS/rulings_check.sh" "$PROJ/intent/st/$THREAD/parity/tools/"
  local before
  before="$(store_sum)"
  [ -n "$before" ] || fail "the fixture project has no store to compare"
  declared_kind "intent/st/$THREAD/parity/tools/rulings_check.sh"
  assert_success
  # Agreeing is counted only when the door answered the directory as a thread,
  # so this shows the door was asked as well as that the store held.
  assert_output_contains "$CITES_N agreeing"
  [ "$(store_sum)" = "$before" ] || fail "the store changed"
}

# --- view_skew_check.sh ----------------------------------------------------

@test "view-skew: a hashed thread's views in the change are named as not checked by this tool" {
  view_skew "intent/st/$THREAD/info.md" "intent/st/$THREAD/acceptance.md"
  assert_success
  assert_output_contains "NOT CHECKED"
  assert_output_contains "intent/st/$THREAD/info.md"
  assert_output_contains "intent/st/$THREAD/acceptance.md"
}

@test "view-skew: a hashed package's view in the change is named as not checked by this tool" {
  local view
  view="$(package_view)" || fail "no package view to cite"
  view_skew "$view"
  assert_success
  assert_output_contains "NOT CHECKED"
  assert_output_contains "$view"
}

@test "view-skew: a thread-shaped view the door does not answer as a thread is named with its answer, where its base listed it as a thread view" {
  view_skew "intent/st/ST9999/info.md"
  assert_success
  refute_output_contains "NOT CHECKED"
  assert_output_contains "NOT A THREAD VIEW"
  assert_output_contains "intent/st/ST9999/info.md -- the door answers"
}

@test "view-skew: a pair without id --read-only refuses with its remedy" {
  old_pair
  view_skew "intent/st/$THREAD/info.md"
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "bin/devbin build all"
}

# --- lib_read_only.sh's copy -----------------------------------------------

# copy <root> -- ro_copy over <root> under the callers' own options, then list
# what the copy holds, sorted. The copy route runs where the pair would migrate
# the store, which only an older store reaches from a script, so ro_copy is
# driven directly.
copy() {
  mkdir -p "$TEST_TEMP_DIR/work"
  run "$(sh_for_gate)" -c 'set -uo pipefail; . "$1/lib_read_only.sh"; RO_ROOT="$2"; RO_TAG=t; RO_WORK="$3"; ro_copy; cd "$3/copy" && find intent \( -type f -o -type l \) | LC_ALL=C sort' _ "$TOOLS" "$1" "$TEST_TEMP_DIR/work"
}

@test "the copy carries what git sees under intent/, and no ignored .DS_Store, store or backup" {
  git -C "$PROJ" init -q
  printf '.DS_Store\n' >"$PROJ/.gitignore"
  git -C "$PROJ" add -A
  git -C "$PROJ" -c user.name=t -c user.email=t@t.invalid -c commit.gpgsign=false commit -q -m fixture
  printf 'untracked, not ignored\n' >"$PROJ/intent/new-note.md"
  printf '\0\1\2' >"$PROJ/intent/.DS_Store"
  printf '\0\1\2' >"$PROJ/intent/st/.DS_Store"
  mkdir -p "$PROJ/intent/.backup" && printf 'old\n' >"$PROJ/intent/.backup/store.bak"
  local expected
  expected="$(cd "$PROJ" && git ls-files --cached --others --exclude-standard -- intent ':!intent/.cache' ':!intent/.backup' | LC_ALL=C sort)"
  [ -n "$expected" ] || fail "the fixture holds nothing under intent/ to copy"
  copy "$PROJ"
  assert_success
  [ "$output" = "$expected" ] || fail "the copy holds:
$output
git sees:
$expected"
  refute_output_contains ".DS_Store"
  refute_output_contains "intent/.cache/"
  refute_output_contains "intent/.backup/"
}

@test "the copy refuses, never reads as clean, where git cannot list the files" {
  # setup's project is not a git work tree until an arm makes it one.
  copy "$PROJ"
  [ "$status" -eq 2 ] || fail "expected exit 2, got $status: $output"
  assert_output_contains "git could not list the files under intent/"
}
