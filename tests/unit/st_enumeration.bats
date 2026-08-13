#!/usr/bin/env bats
# Issue 0011: `intent st` enumerated any STnnnn directory at any depth below
# intent/st, so anything placed there became live steel threads -- and since
# such areas characteristically hold COPIES, the result was duplicate ids in a
# namespace whose one guarantee is that an id names one thread.
#
# The rule now has one home (list_st_dirs, guarded in helpers.bats). These guard
# the surfaces that consume it, and the organize sweep's behaviour on collision.

load "../lib/test_helper.bash"

# A project with a live ST0001 in the base dir and a staged COPY of it under a
# deliberately underscore-prefixed staging area -- the shape from the report.
setup_shadowed() {
  project_dir=$(create_test_project "ST Enumeration Test")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success
  run run_intent st new "Beta"
  assert_success
  mv intent/st/NOT-STARTED/ST0001 intent/st/ST0001
  set_status intent/st/ST0001/info.md WIP
  mkdir -p intent/st/_inbox
  cp -R intent/st/ST0001 intent/st/_inbox/ST0001
}

set_status() {
  local f="$1" s="$2"
  sed -i '' "s/^status: .*/status: $s/" "$f" 2>/dev/null || sed -i "s/^status: .*/status: $s/" "$f"
}

@test "a staging area under intent/st does not become duplicate live threads" {
  setup_shadowed
  # Underscore-prefixing reads as the obvious way to mark a directory not-live,
  # and had no effect: the tree simply grew a second thread with the same id.
  run run_intent st list
  assert_success
  run bash -c "cd '$project_dir' && intent st list 2>/dev/null | grep -c ST0001"
  assert_output "1"
}

@test "st sync --write does not persist a duplicate id into the committed index" {
  setup_shadowed
  # sync shells out to `st list --markdown` and writes the result between the
  # index markers, so a duplicate row was persisted into the project's
  # canonical, committed index with nothing downstream re-checking it.
  run run_intent st sync --write
  assert_success
  run grep -c 'ST0001' intent/st/steel_threads.md
  assert_output "1"
}

@test "threads in every canonical bucket still resolve" {
  setup_shadowed
  # The recursion was deliberate and necessary -- it is how COMPLETED /
  # NOT-STARTED / CANCELLED threads are found -- so the fix must not lose them.
  run run_intent st show ST0002
  assert_success
  run bash -c "cd '$project_dir' && intent st list --status all 2>/dev/null | grep -c 'ST000[12]'"
  assert_output "2"
}

@test "organize names a collision, finishes the sweep, and exits non-zero" {
  project_dir=$(create_test_project "ST Organize Collision")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success
  run run_intent st new "Beta"
  assert_success

  # ST0001 is Completed and sits in the base dir, so organize wants to move it
  # to COMPLETED/ -- where a directory of the same id already sits.
  set_status intent/st/NOT-STARTED/ST0001/info.md Completed
  mv intent/st/NOT-STARTED/ST0001 intent/st/ST0001
  mkdir -p intent/st/COMPLETED/ST0001
  cp intent/st/ST0001/info.md intent/st/COMPLETED/ST0001/
  # ST0002 also needs organizing and is processed AFTER the collision.
  set_status intent/st/NOT-STARTED/ST0002/info.md Cancelled
  mv intent/st/NOT-STARTED/ST0002 intent/st/ST0002

  run run_intent st organize --write
  assert_failure

  # The collision is NAMED -- id and both paths. Previously this module's `set
  # -e` took the whole command down on the failing mv: raw mv stderr, no
  # intent-level message, and no indication that a sweep had stopped early.
  assert_output_contains "cannot move ST0001"
  assert_output_contains "already exists"
  assert_output_contains "left in place"

  # The sweep FINISHED: the thread after the collision was still organized.
  [ -d intent/st/CANCELLED/ST0002 ]
  # And nothing was destroyed -- mv refuses to merge non-empty directories.
  [ -f intent/st/ST0001/info.md ]
  [ -f intent/st/COMPLETED/ST0001/info.md ]
}

@test "organize still exits zero and moves cleanly when there is no collision" {
  project_dir=$(create_test_project "ST Organize Clean")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success
  set_status intent/st/NOT-STARTED/ST0001/info.md Completed
  mv intent/st/NOT-STARTED/ST0001 intent/st/ST0001

  run run_intent st organize --write
  assert_success
  assert_output_contains "Moved ST0001"
  [ -d intent/st/COMPLETED/ST0001 ]
}

@test "organize does not promote a staged copy into the live namespace" {
  setup_shadowed
  # The genuinely lossy case in the report was the one with NO collision: a
  # staged copy whose id has no live counterpart at the target bucket moved
  # cleanly, silently promoting a stale, untriaged copy into the live namespace
  # where it is thereafter indistinguishable from a real thread.
  set_status intent/st/_inbox/ST0001/info.md Completed
  run run_intent st organize --write
  assert_success
  [ ! -d intent/st/COMPLETED/ST0001 ]
  [ -d intent/st/_inbox/ST0001 ]
}

@test "doctor reports a duplicate id as a defect rather than an extra table row" {
  project_dir=$(create_test_project "ST Doctor Duplicate")
  cd "$project_dir"
  export EDITOR=echo
  run run_intent st new "Alpha"
  assert_success

  run run_intent doctor
  assert_success
  assert_output_contains "steel thread ids"

  # Bounding the enumerator stops a staging area manufacturing duplicates, but
  # the same id can still sit in two CANONICAL buckets -- which is what an
  # interrupted or collided organize leaves behind.
  mkdir -p intent/st/COMPLETED/ST0001
  cp intent/st/NOT-STARTED/ST0001/info.md intent/st/COMPLETED/ST0001/

  run run_intent doctor
  assert_failure
  assert_output_contains "duplicate steel thread id(s): ST0001"
  assert_output_contains "intent/st/COMPLETED/ST0001"
}
