#!/usr/bin/env bats
#
# release_pair_promotion -- `promote_pair`, the staging half of 0196's fix.
#
# WHY IT EXISTS. 0196 is a hazard with no failing test and no way to get one:
# the defect is that a FAILED build leaves the shared pair absent, and driving
# that end to end means deliberately breaking a release build four sessions and
# the human's shell are consuming. The fix moves the whole risky span into a
# staging dir precisely so it can be exercised without a window -- and the arm
# that has to be right is the promotion, which is the only code that touches the
# shared path at all.
#
# THE ARM THAT CARRIES THE FIX IS THE REFUSAL, NOT THE HAPPY PATH. A promotion
# that moves two files is the easy half; what 0196 asks for is that every exit
# BEFORE the promotion leaves the old pair working. So the refusal arms assert
# the destination is untouched, which is the property the issue was filed about.
#
# NO RUST TOOLCHAIN AND NO CARGO. `promote_pair` moves files and compares device
# numbers; it does not care what the files contain. Fixtures are text.

load "../lib/test_helper"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-promote-XXXXXX)"
  FROM="$TEST_TEMP_DIR/staging/release"
  TO="$TEST_TEMP_DIR/target/release"
  mkdir -p "$FROM" "$TO"

  # The destination starts with a WORKING pair, because that is the state 0196
  # is about: the shared artefact exists and is being consumed when the build
  # starts, and the question is whether it survives a failure.
  echo "OLD intent"  > "$TO/intent"
  echo "OLD intentd" > "$TO/intentd"

  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/lib/helpers"
  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/releasebuild.lib"
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

stage_pair() {
  echo "NEW intent"  > "$FROM/intent"
  echo "NEW intentd" > "$FROM/intentd"
}

@test "success: both binaries are promoted and the staging dir is left empty of them" {
  stage_pair
  run promote_pair "$FROM" "$TO"
  [ "$status" -eq 0 ]
  [ "$(cat "$TO/intent")"  = "NEW intent" ]
  [ "$(cat "$TO/intentd")" = "NEW intentd" ]
  [ ! -f "$FROM/intent" ]
  [ ! -f "$FROM/intentd" ]
}

@test "invariant: an absent staged binary refuses and the OLD shared pair is untouched" {
  # Only half a set staged -- the exact condition verify_pair exists to refuse,
  # arriving here means something upstream let it through.
  echo "NEW intent" > "$FROM/intent"
  run promote_pair "$FROM" "$TO"
  [ "$status" -ne 0 ]
  [[ "$output" == *"refusing to move half a set"* ]]
  # THE LOAD-BEARING ASSERTION: the shared pair a live session is invoking is
  # still the old one, still complete, still agreeing with itself.
  [ "$(cat "$TO/intent")"  = "OLD intent" ]
  [ "$(cat "$TO/intentd")" = "OLD intentd" ]
}

@test "invariant: a cross-filesystem destination refuses BEFORE the first rename" {
  stage_pair
  # A rename across filesystems fails EXDEV. Taken after the FIRST move had
  # already succeeded, that is the one path leaving the shared pair permanently
  # split -- so the check must fire before anything moves. Two filesystems are
  # not available in a unit test; the device read is stubbed instead, which is
  # the thing the code actually branches on.
  # `stat -f %d <path>` -- the path is the THIRD argument, not the second.
  stat() { if [ "$3" = "$TO" ]; then echo 99; else echo 1; fi; }
  run promote_pair "$FROM" "$TO"
  [ "$status" -ne 0 ]
  [[ "$output" == *"different filesystems"* ]]
  # Nothing moved, in EITHER direction.
  [ "$(cat "$TO/intent")"  = "OLD intent" ]
  [ "$(cat "$TO/intentd")" = "OLD intentd" ]
  [ -f "$FROM/intent" ]
  [ -f "$FROM/intentd" ]
}

@test "the shared release dir is never a cargo target: guarded_release_build stages first" {
  # The fix is only real if the SHARED path stopped being the build target. Read
  # off the function body rather than asserted in prose, because the prose is
  # what was true before the defect too.
  body="$(awk '/^guarded_release_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' \
    "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/releasebuild.lib")"
  [ -n "$body" ]
  # The clean and the build both run against the staged dir's parent.
  [[ "$body" == *'build_dir="$STAGING_RELEASE_DIR"'* ]]
  [[ "$body" == *'CARGO_TARGET_DIR="$(dirname "$build_dir")" cargo clean'* ]]
  [[ "$body" == *'CARGO_TARGET_DIR="$(dirname "$build_dir")" cargo build'* ]]
  # And RELEASE_DIR appears only as a promotion DESTINATION, never as a target.
  [[ "$body" == *'final_dir="$RELEASE_DIR"'* ]]
  [[ "$body" != *'CARGO_TARGET_DIR="$(dirname "$RELEASE_DIR")"'* ]]
}

@test "ordering: verification happens before promotion, not after" {
  # The defect 0196 names is a verification that ran after the artefact was
  # already overwritten. If promote_pair ever precedes verify_pair the fix is
  # inverted while every other arm here still passes.
  body="$(awk '/^guarded_release_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' \
    "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/releasebuild.lib")"
  verify_at="$(printf '%s\n' "$body" | grep -n 'verify_pair' | head -1 | cut -d: -f1)"
  promote_at="$(printf '%s\n' "$body" | grep -n 'promote_pair' | head -1 | cut -d: -f1)"
  [ -n "$verify_at" ]
  [ -n "$promote_at" ]
  [ "$verify_at" -lt "$promote_at" ]
}

@test "a private build is not promoted -- the redirect IS the artefact" {
  body="$(awk '/^guarded_release_build\(\) \{/ { inb = 1 } inb { print } inb && /^\}/ { exit }' \
    "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/releasebuild.lib")"
  # A dirty build must still land in the private dir and must NOT be moved into
  # the shared path -- promoting it would defeat the dirt verdict entirely.
  [[ "$body" == *'build_dir="$PRIVATE_RELEASE_DIR"'* ]]
  [[ "$body" == *'final_dir=""'* ]]
  [[ "$body" == *'[ -n "$final_dir" ] || return 0'* ]]
}
