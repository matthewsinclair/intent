#!/usr/bin/env bats
#
# config_undefined -- `intent config` is DECLARED and UNIMPLEMENTED, and this
# file is what makes that ruling checkable instead of merely asserted.
#
# WHY IT EXISTS, IN AC-06.1's OWN WORDS: "`intent config` lands a conformance
# test BEFORE its behaviour is designed, or the `undefined` ruling on it is
# unverifiable by construction." The ordering is the whole clause. A test
# written AFTER someone designs `config` would be written against that design
# and could never have caught the design arriving unannounced -- so writing it
# late does not satisfy the criterion late, it fails the criterion.
#
# WHAT IS BEING PINNED IS AN ABSENCE, WHICH IS THE UNUSUAL PART. Every arm below
# asserts that `config` does NOTHING and says so at rc=2. The day someone wires
# it, this file goes red, and that redness is the point: it forces the change to
# be deliberate and reviewed rather than noticed later by whoever first depended
# on the old behaviour.
#
# rc=2 IS A CONTRACT, NOT A MAGIC NUMBER. `guide.rs` states it: 0 is success, 1
# means the command RAN and the answer is no, and "2 means this build cannot
# answer the question at all, and it NEVER carries a verdict about your work"
# -- which is why the shipped pre-commit gate fails open on it. `config` takes
# `unwired_whole_family` -> `refuse_unwired` -> `Failure::Unavailable`, one home
# for both absences precisely so the exit code cannot drift between them.
#
# THE `set` ARM IS THE ONE THAT MATTERS AND IT CARRIES BOTH CONTROLS. An
# unimplemented verb that quietly wrote would be far worse than one that
# refuses, so the arm asserts the config file is byte-identical across the call.
# That assertion is worthless unless the file provably EXISTS first (an absent
# file is trivially "unchanged"), so the fixture is asserted before it is used
# -- a control that cannot tell SAFE from NEVER TRIED is not a control.
#
# STDOUT IS ASSERTED EMPTY ON EVERY REFUSAL, separately from the exit code.
# A consumer doing `value=$(intent config get k)` gets the empty string either
# way; what stops it silently becoming a wrong VALUE is that nothing is printed
# on the success channel at all. The diagnosis belongs on stderr and is checked
# there.

bats_require_minimum_version 1.5.0

load "../lib/test_helper"

# The invariant substring of the refusal, declared once in `render.rs` as
# `UNWIRED_PHRASE` and imported by `guide.rs` rather than re-typed. Repeated
# here because a bats file cannot import a Rust const -- if this drifts, the
# Rust-side test that compares the emitter against the guide text goes red
# first, which is the ordering that makes this copy safe to keep.
UNWIRED_PHRASE="is a known command that is not implemented yet"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-config-XXXXXX)"
  export HOME="$TEST_TEMP_DIR/h"
  mkdir -p "$HOME"
  cd "$TEST_TEMP_DIR" || exit 1
}

teardown() {
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

@test "config: the bare family refuses at rc=2 as an unbuilt command" {
  run "$INTENT_BIN" config
  [ "$status" -eq 2 ]
  [[ "$output" == *"$UNWIRED_PHRASE"* ]]
}

@test "config: the refusal names the whole-family remedy, not a per-verb one" {
  run "$INTENT_BIN" config
  # `unwired_whole_family` and `unwired` differ ONLY in their remedy, and which
  # one fires says whether the family has a dispatch arm at all. Asserting the
  # remedy is how this file can tell "config is unbuilt" from "config exists
  # but this verb is missing" -- two states that would otherwise read alike.
  [[ "$output" == *"nothing in this build provides it"* ]]
}

@test "config get: refuses at rc=2 and prints nothing on stdout" {
  run --separate-stderr "$INTENT_BIN" config get project_name
  [ "$status" -eq 2 ]
  # The success channel is empty, so `value=$(intent config get k)` cannot pick
  # up a diagnosis and treat it as a value.
  [ -z "$output" ]
  [[ "$stderr" == *"$UNWIRED_PHRASE"* ]]
}

@test "config set: refuses at rc=2 and does not write the config file" {
  run "$INTENT_BIN" init probe
  [ "$status" -eq 0 ]

  local cfg="$TEST_TEMP_DIR/intent/.config/config.json"
  # POSITIVE CONTROL ON THE FIXTURE. Without this the byte-identity assertion
  # below passes trivially when the file does not exist, which is the shape of
  # a control that proves nothing.
  [ -f "$cfg" ]
  local before
  before="$(shasum -a 256 "$cfg" | cut -d' ' -f1)"

  run --separate-stderr "$INTENT_BIN" config set project_name HIJACKED
  [ "$status" -eq 2 ]
  [ -z "$output" ]
  [[ "$stderr" == *"$UNWIRED_PHRASE"* ]]

  local after
  after="$(shasum -a 256 "$cfg" | cut -d' ' -f1)"
  [ "$before" = "$after" ]
}

@test "config set: the byte-identity check can actually detect a write" {
  # NEGATIVE CONTROL for the arm above, in its own test so its failure is
  # legible. If this ever passes while the file is rewritten, the comparison
  # used above is broken and every no-mutation claim resting on it is vacuous.
  run "$INTENT_BIN" init probe
  [ "$status" -eq 0 ]

  local cfg="$TEST_TEMP_DIR/intent/.config/config.json"
  [ -f "$cfg" ]
  local before
  before="$(shasum -a 256 "$cfg" | cut -d' ' -f1)"

  printf '{"deliberately":"rewritten"}' > "$cfg"
  local after
  after="$(shasum -a 256 "$cfg" | cut -d' ' -f1)"
  [ "$before" != "$after" ]
}

@test "config --help: renders at rc=0, and that is NOT evidence of an arm" {
  # THE TRAP THIS ARM EXISTS TO NAME. `--help` renders for a DECLARED command
  # whether or not a dispatch arm exists -- it produced three wrong readings in
  # three hands in one evening on this thread. So help rendering is asserted
  # HERE, beside the rc=2 arms above, to keep the two facts adjacent: the
  # family is declared, the family is unbuilt, and neither implies the other.
  run "$INTENT_BIN" config --help
  [ "$status" -eq 0 ]
  [[ "$output" == *"get"* ]]
  [[ "$output" == *"set"* ]]
  # And the rendered help must not claim the thing works.
  [[ "$output" != *"$UNWIRED_PHRASE"* ]]
}
