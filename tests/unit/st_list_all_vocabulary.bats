#!/usr/bin/env bats
# Issue 0020: `intent st list --status all` did not show all threads. The `all`
# branch walked a hardcoded array of ten status LITERALS and collected rows by
# exact string match, so a presentation ordering doubled as a membership test:
# any status outside those ten was dropped with no diagnostic, no count, and no
# exit-code change, from a view that names itself `all`.
#
# Two gaps compounded it. canonical_status -- THE synonym table -- was bypassed
# on this path, so `COMPLETE` was not folded into `Completed`. And a genuinely
# unknown status (`SUPERSEDED`) had no group at all, so even correct
# normalisation would have left it unplaced.
#
# It matters because of issue 0019: `st sync --write` composes this exact view
# into intent/st/steel_threads.md, the committed canonical index whose own
# preamble says it holds every steel thread. A row discarded here is discarded
# from tracked project state, and the regeneration reports success.
#
# Measured on Laksa: 96 info.md on disk, 94 rows emitted, exit 0 throughout.

load "../lib/test_helper.bash"

# Four threads spanning the four cases: a vocabulary status, a second
# vocabulary status (to pin group ORDER, which the fix rewrote), a synonym the
# table knows under another name, and a status the vocabulary cannot place.
setup_vocabulary() {
  project_dir=$(create_test_project "ST List Vocabulary Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success
  run run_intent st new "Bravo"
  assert_success
  run run_intent st new "Charlie"
  assert_success
  run run_intent st new "Delta"
  assert_success
  set_status intent/st/NOT-STARTED/ST0001/info.md WIP
  # ST0002 keeps the template's "Not Started".
  set_status intent/st/NOT-STARTED/ST0003/info.md COMPLETE
  set_status intent/st/NOT-STARTED/ST0004/info.md SUPERSEDED
}

set_status() {
  local f="$1" s="$2"
  sed -i '' "s/^status: .*/status: $s/" "$f" 2>/dev/null || sed -i "s/^status: .*/status: $s/" "$f"
}

@test "st list --status all emits a row for every thread on disk" {
  setup_vocabulary
  # The census that found the bug: count info.md on disk, count rows emitted,
  # compare. Nothing else in the output distinguishes "correctly filtered" from
  # "silently dropped".
  run bash -c "cd '$project_dir' && '${INTENT_BIN_DIR}/intent' st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}'"
  assert_output "4"
}

@test "st list --status all exits 0 and still reports the anomaly" {
  setup_vocabulary
  # The warning reports; it must not block. `sync --write` composes this view,
  # so a non-zero exit here would break index regeneration on any estate that
  # has ever written a status outside the vocabulary.
  run run_intent st list --status all
  assert_success
}

@test "a status the synonym table knows under another name lands in its group" {
  setup_vocabulary
  # canonical_status folds COMPLETE -> Completed and always could; this path
  # bypassed it, which is what made the vocabulary brittle rather than the
  # normaliser redundant.
  run bash -c "cd '$project_dir' && '${INTENT_BIN_DIR}/intent' st list --status all 2>/dev/null | grep -c '^ST0003'"
  assert_output "1"
}

@test "a status outside the vocabulary is still shown" {
  setup_vocabulary
  # SUPERSEDED is nobody's synonym, so normalisation alone cannot rescue it.
  # `all` must mean all: a view that cannot classify a row still has to show it.
  run bash -c "cd '$project_dir' && '${INTENT_BIN_DIR}/intent' st list --status all 2>/dev/null | grep -c '^ST0004'"
  assert_output "1"
}

@test "an out-of-vocabulary status is named on stderr with its id" {
  setup_vocabulary
  # Issue 0007's precedent: report the row AND the anomaly. Neither substitutes
  # for the other -- showing it silently hides a data problem, warning without
  # showing it is the bug.
  run bash -c "cd '$project_dir' && '${INTENT_BIN_DIR}/intent' st list --status all 2>&1 >/dev/null"
  assert_output_contains "outside the vocabulary"
  assert_output_contains "ST0004: SUPERSEDED"
  # The placed threads are not denounced.
  [[ "$output" != *"ST0001"* ]] || fail "a vocabulary status was reported as out-of-vocabulary"
  [[ "$output" != *"ST0003"* ]] || fail "a synonym status was reported as out-of-vocabulary"
}

@test "sync --write carries every thread into the committed index" {
  setup_vocabulary
  # This is the consequence that makes the defect matter: since 0019 the index
  # is composed from this view, so the omission reached tracked project state
  # and stayed there, with the write reporting success.
  run run_intent st sync --write
  assert_success
  run grep -cE '^\| ST[0-9]{4}' intent/st/steel_threads.md
  assert_output "4"
}

@test "the presentation order survives collapsing the literal list" {
  setup_vocabulary
  # The fix collapsed ten literals to five canonical tokens. That is the same
  # ordering -- WIP, Not Started, On Hold, Completed, Cancelled -- and the
  # unplaced rows come last. Pinned, because a wrong collapse reorders every
  # `--status all` listing and every regenerated index silently.
  run bash -c "cd '$project_dir' && '${INTENT_BIN_DIR}/intent' st list --status all 2>/dev/null | grep -nE '^ST[0-9]{4}' | cut -d: -f2 | cut -d' ' -f1 | tr '\n' ' '"
  assert_output "ST0001 ST0002 ST0003 ST0004 "
}
