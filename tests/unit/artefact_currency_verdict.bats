#!/usr/bin/env bats
#
# artefact_currency_verdict -- every arm of `shared/currency.lib`'s matrix.
#
# WHY IT EXISTS, and it is not "the new lib should have tests" (ic). Both
# refusal messages were driven ONLY by an ad-hoc scratchpad rig that lives
# nowhere. AND THE CLEAN-BASE ARM WAS DOUBLY UNREACHABLE: no test drove it, and
# the live release pair carries a `dirty-` marker so every real invocation takes
# the FLOOR branch. That wording could be observed by neither USE nor TEST --
# correct by inspection only, until the day somebody builds from a clean tree
# and the refusal changes to a form nobody has read in anger.
#
# A FIX THAT ADDS AN ARM ADDS A THING TO DRIVE, and this estate's record is that
# the undriven arm is the one that is wrong when it finally fires. The overclaim
# sat in the error message of the file written to refuse overclaims, and the fix
# for it added a second arm nothing drove.
#
# SELF-CONTAINED BY CONSTRUCTION. Every arm builds its own git repo and its own
# planted artefacts. Nothing reads the live checkout, whose HEAD moves under a
# four-node board and whose release pair is dirty today and may not be tomorrow
# -- a fixture tracking either would stop testing the subject without saying so.
#
# THE ARTEFACTS ARE TEXT FILES. `artefact_source_commit` is `strings` and
# nothing else, so a text file carrying `[intent-source-commit:<sha>]` is a
# faithful subject and needs no Rust toolchain.

load "../lib/test_helper"

setup() {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-currency-XXXXXX)"
  REPO="$TEST_TEMP_DIR/repo"
  REL="$TEST_TEMP_DIR/release"
  mkdir -p "$REPO/native/rust/crates" "$REPO/native/rust/crates/x/tests" "$REL"

  git init -q "$REPO"
  git -C "$REPO" config user.email "test@example.com"
  git -C "$REPO" config user.name "test_user"
  echo "fn main() {}" > "$REPO/native/rust/crates/lib.rs"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "base"
  BASE="$(git -C "$REPO" rev-parse HEAD)"

  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/artefact.lib"
  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/currency.lib"
}

teardown() {
  [ -n "${TEST_TEMP_DIR:-}" ] && rm -rf "$TEST_TEMP_DIR"
}

plant() {
  printf 'padding [intent-source-commit:%s] padding\n' "$1" > "$REL/intent"
  printf 'padding [intent-source-commit:%s] padding\n' "${2:-$1}" > "$REL/intentd"
}

# Advance HEAD by a NON-TEST source file, so the range test has something to see.
touch_source() {
  echo "// changed" >> "$REPO/native/rust/crates/lib.rs"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "source change"
}

# Advance HEAD by a TEST file only. The range must NOT count this.
touch_test_only() {
  echo "// test" >> "$REPO/native/rust/crates/x/tests/t.rs"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "test change"
}

@test "positive control: a coherent clean pair at HEAD over a clean range is ok, SILENTLY" {
  plant "$BASE"
  run artefact_currency_verdict "$REL" "$REPO"
  [ "$status" -eq 0 ]
  [ "$output" = "ok" ]
}

# THE CONTROL THAT MAKES EVERY REFUSAL BELOW MEAN SOMETHING. Without an arm that
# can reach `ok`, a uniformly-refusing harness proves only that it refuses.

@test "control: a range of TEST-ONLY commits does not count as behind" {
  plant "$BASE"
  touch_test_only
  run artefact_currency_verdict "$REL" "$REPO"
  [ "$output" = "ok" ]
}

@test "clean base + changed source REFUSES, and does NOT hedge the count" {
  plant "$BASE"
  touch_source
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"behind HEAD"* ]]
  # THE ARM ic SHOWED WAS UNREACHABLE BY USE *AND* BY TEST. Over a CLEAN base the
  # committed range IS the distance, so hedging it would understate a known fact.
  [[ "$output" != *"at least"* ]]
  [[ "$output" != *"FLOOR"* ]]
}

@test "dirty base + changed source REFUSES, and states the count as a FLOOR" {
  plant "dirty-$BASE"
  touch_source
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  # Whatever was uncommitted at build time lies OUTSIDE the measured range, in
  # either direction, so the count is a lower bound and must read as one.
  [[ "$output" == *"at least"* ]]
  [[ "$output" == *"FLOOR"* ]]
}

@test "the two refusal wordings actually DIFFER -- the whole point of the fix" {
  plant "$BASE"; touch_source
  clean_out="$(artefact_currency_verdict "$REL" "$REPO")"
  plant "dirty-$BASE"
  dirty_out="$(artefact_currency_verdict "$REL" "$REPO")"
  # Both refuse. If they were ever collapsed back into one message this fails,
  # which is the regression the fix exists to prevent.
  [[ "$clean_out" == refuse:* ]]
  [[ "$dirty_out" == refuse:* ]]
  [ "$clean_out" != "$dirty_out" ]
}

@test "dirty base + clean range WARNS and the warning is stated as uncleartable" {
  plant "dirty-$BASE"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == warn:* ]]
  [[ "$output" == *"never be cleared"* ]]
}

@test "a disagreeing pair REFUSES and names BOTH binaries" {
  plant "$BASE" "0000000000000000000000000000000000000000"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"DIFFERENT trees"* ]]
  [[ "$output" == *"intent "* ]]
  [[ "$output" == *"intentd "* ]]
}

@test "no marker at all REFUSES -- cannot say is not a pass" {
  printf 'no marker here\n' > "$REL/intent"
  printf 'no marker here\n' > "$REL/intentd"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"cannot say what it is"* ]]
}

@test "a missing binary REFUSES rather than reporting on the one that is present" {
  plant "$BASE"
  rm -f "$REL/intentd"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"no intentd binary"* ]]
}

@test "a base that is not a commit here REFUSES" {
  plant "deadbeefdeadbeefdeadbeefdeadbeefdeadbeef"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"not a commit in this repository"* ]]
}

@test "a base that is real but NOT an ancestor REFUSES for that reason specifically" {
  # The precondition is built rather than assumed: an orphan branch shares no
  # history with HEAD. A stale ref pointing at a BRANCH POINT is an ancestor and
  # would fall through to the source-changed rule -- refusing for the wrong
  # reason, which scores green by verdict class and tests nothing. That is the
  # exact trap this estate's local `v2-maintenance` ref laid for two nodes.
  # `commit-tree` with NO -p makes a parentless commit: real, reachable by sha,
  # and sharing no history with HEAD. It touches neither the working tree nor any
  # branch, so it cannot perturb the other arms -- the orphan-CHECKOUT version
  # failed here twice, which is the right way for a harness bug to present.
  ORPHAN="$(git -C "$REPO" commit-tree "$(git -C "$REPO" rev-parse HEAD^{tree})" -m orphan </dev/null)"

  run git -C "$REPO" merge-base --is-ancestor "$ORPHAN" HEAD
  [ "$status" -ne 0 ]

  plant "$ORPHAN"
  run artefact_currency_verdict "$REL" "$REPO"
  [[ "$output" == refuse:* ]]
  [[ "$output" == *"not an ancestor of HEAD"* ]]
}

@test "a tree that cannot answer WARNS -- the environment is not the artefact's fault" {
  plant "$BASE"
  run artefact_currency_verdict "$REL" "$TEST_TEMP_DIR"
  [[ "$output" == warn:* ]]
  [[ "$output" == *"undecidable"* ]]
}

@test "the verdict function never exits nonzero -- the CALLER decides what a verdict costs" {
  # A library that kills its host cannot serve a reporter and an actor both.
  printf 'no marker\n' > "$REL/intent"
  printf 'no marker\n' > "$REL/intentd"
  run artefact_currency_verdict "$REL" "$REPO"
  [ "$status" -eq 0 ]
  [[ "$output" == refuse:* ]]
}

# ---------------------------------------------------------------------------
# THE HARNESS RAN UNDER DIFFERENT SHELL OPTIONS THAN THE ONLY PRODUCTION CALLER,
# WHICH IS WHY EVERY ARM ABOVE PASSED OVER A FUNCTION RETURNING rc=1 ON ITS
# HEALTHIEST ANSWER. bats sets neither `errexit` nor `pipefail`; `bin/intent3`
# sets both. These arms drive the production options explicitly.

@test "the verdict survives set -euo pipefail WITHOUT a command substitution to hide behind" {
  plant "$BASE"
  touch_test_only
  # NOT `v=$(...)`: a command substitution DISARMS errexit in the subshell, which
  # is the accident that kept `bin/intent3` alive. Call it where errexit is live.
  run bash -c "
    set -euo pipefail
    . '${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/artefact.lib'
    . '${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/currency.lib'
    artefact_currency_verdict '$REL' '$REPO'
    printf ' REACHED-THE-END'
  "
  [ "$status" -eq 0 ]
  [[ "$output" == *"REACHED-THE-END"* ]]
}

@test "_rust_source_changed returns rc=0 on a TEST-ONLY range -- grep's no-match is not a verdict" {
  touch_test_only
  run bash -c "
    set -uo pipefail
    . '${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/currency.lib'
    _rust_source_changed '$REPO' '$BASE'
  "
  [ "$status" -eq 0 ]
  [ "$output" = "0" ]
}

# NO ARM FOR THE `nodiff` SENTINEL. Making `git diff` fail while git is present,
# the base reachable and the ancestry confirmed needs a corrupted object store,
# and every cheap way I tried trips an EARLIER guard instead -- so the arm would
# pass while driving a different refusal. AN ARM THAT CANNOT FAIL IS NOT A TEST,
# and one that fires on the wrong branch is worse. Recorded as UNDRIVEN.
