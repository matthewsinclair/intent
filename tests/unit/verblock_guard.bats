#!/usr/bin/env bats
# lib/templates/hooks/verblock-guard.sh -- a verblock is the version and one
# short line, never history (IN-PR-STYLE-005, issue 0653).
#
# ONE RED AND ONE GREEN ARM PER REFUSAL, and each arm is built so that the
# properties the guard rests on are what would turn it:
#
#   a second version stamp ....... arm 1 red, arm 2 green
#   a value over 100 characters .. arm 3 red, arm 4 green
#
# THE STAGED BLOB DECIDES, NOT THE WORKTREE, in both directions: arm 1 stages a
# refused verblock and leaves a passing one in the worktree, arm 2 the reverse,
# so a guard that read the worktree would flip both. Arm 2 also commits a
# record whose verblock breaks both checks and never restages it, which is the
# going-forward-only property: a guard that read history would refuse it. Arm 2
# stages a line that names a date, and a date is not a stamp. Arm 4's value is
# exactly 100 characters and more than 100 bytes, so a guard counting bytes
# would refuse it; the arm asserts the byte count first, so the fixture cannot
# silently stop discriminating.
#
# A BLOB LARGER THAN A PIPE BUFFER, arm 5: git runs a hook with SIGPIPE
# ignored, so the arm does too, and there a guard that piped the blob into a
# reader stopping at the end of the frontmatter printed `write error: Broken
# pipe`. It stages one such file with a verblock and one with no frontmatter,
# which the reader leaves at line 1, and asserts both sizes first, for the same
# reason as arm 4.

load "../lib/test_helper.bash"

GUARD="${INTENT_PROJECT_ROOT}/lib/templates/hooks/verblock-guard.sh"

# A scratch repository with one commit, so HEAD exists and a staged file reads
# as added.
scratch_repo() {
  local repo="$TEST_TEMP_DIR/repo"
  mkdir -p "$repo"
  cd "$repo" || return 1
  git init -q -b main >/dev/null
  git config user.email "test@example.com"
  git config user.name "Test"
  printf 'x\n' > README.txt
  git add README.txt
  git commit -q -m "init"
}

# A Markdown file at $1 whose frontmatter carries the verblock value $2.
doc() {
  printf -- '---\nverblock: "%s"\n---\n\n# Doc\n' "$2" > "$1"
}

@test "a second version stamp in the staged verblock is refused, whatever the worktree holds" {
  scratch_repo
  doc notes.md "19 Sep 2026:v0.4: citations re-synced. 15 Sep 2026:v0.3: citations moved"
  git add notes.md
  doc notes.md "19 Sep 2026:v0.4: citations re-synced"
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  assert_output_contains "notes.md: its verblock carries 2 version stamps (v0.4, v0.3)"
  assert_output_contains "IN-PR-STYLE-005"
}

@test "one version stamp passes, a date is not a stamp, and a committed record is never read" {
  scratch_repo
  doc record.md "16 Sep 2026:v0.2: a fold's whole account of the session, kept here as history. 15 Sep 2026:v0.1: the first version"
  git add record.md
  git commit -q -m "a record"
  doc notes.md "16 Oct 2026:v0.28: vc - added the 2026-10-16 entry"
  git add notes.md
  doc notes.md "16 Oct 2026:v0.28: vc - added the entry. 15 Oct 2026:v0.27: vc - the one before"
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  assert_output_contains "ok -- 1 verblock(s) in 1 staged Markdown file(s)"
}

@test "a verblock longer than 100 characters is refused, and the refusal names the elision at 97" {
  scratch_repo
  local value="2026-10-26:v1.0: dc - " i
  for i in $(seq 1 79); do value="${value}x"; done
  doc long.md "$value"
  git add long.md
  run bash "$GUARD"
  [ "$status" -eq 1 ]
  assert_output_contains "long.md: its verblock is 101 characters"
  assert_output_contains "elided at 97 and ends in ..."
}

@test "a verblock of exactly 100 characters passes, counted in characters and not bytes" {
  scratch_repo
  local value="2026-10-26:v1.0: dc - " dash i
  dash="$(printf '\342\200\224')"
  for i in $(seq 1 26); do value="${value}ab${dash}"; done
  [ "$(printf '%s' "$value" | LC_ALL=C wc -c | tr -d ' ')" -gt 100 ]
  doc exact.md "$value"
  git add exact.md
  run bash "$GUARD"
  [ "$status" -eq 0 ]
  assert_output_contains "ok -- 1 verblock(s) in 1 staged Markdown file(s)"
}

@test "a staged Markdown file larger than a pipe buffer prints no broken pipe, with SIGPIPE ignored as git runs a hook" {
  scratch_repo
  local i
  doc big.md "2026-10-26:v1.0: dc - one line"
  for i in $(seq 1 4096); do printf 'Body line %s, past the frontmatter the guard reads.\n' "$i"; done >> big.md
  for i in $(seq 1 4096); do printf 'Line %s of a document with no frontmatter.\n' "$i"; done > plain.md
  [ "$(LC_ALL=C wc -c < big.md | tr -d ' ')" -gt 65536 ]
  [ "$(LC_ALL=C wc -c < plain.md | tr -d ' ')" -gt 65536 ]
  git add big.md plain.md
  run bash -c 'trap "" PIPE; bash "$1"' _ "$GUARD"
  [ "$status" -eq 0 ]
  refute_output_contains "Broken pipe"
  assert_output_contains "ok -- 1 verblock(s) in 2 staged Markdown file(s)"
}
