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

  # SOURCED IN THE ORDER THE HEADERS REQUIRE. `currency.lib` needs `artefact.lib`
  # for the marker parse and `sharedtarget.lib` for the build-input scope and the
  # git isolation, and it sources neither itself. Getting this order wrong here
  # would test a configuration no consumer has.
  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/artefact.lib"
  # shellcheck source=/dev/null
  . "${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/sharedtarget.lib"
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

# Advance HEAD by a build input OUTSIDE `native/rust`. This is the whole point of
# the 2026-09-05 widening: the marker's base commit is chosen over three
# pathspecs, so a range measured over one of them answers a narrower question
# than the one the base was picked to serve.
touch_surface_only() {
  mkdir -p "$REPO/surface"
  echo '{"populations":{}}' >> "$REPO/surface/dispatch-table.json"
  git -C "$REPO" add -A
  git -C "$REPO" commit -qm "surface change, no native/rust file touched"
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

@test "_build_inputs_changed returns rc=0 on a TEST-ONLY range -- grep's no-match is not a verdict" {
  touch_test_only
  # THIS SUBSHELL SOURCED `currency.lib` ALONE AND THE ARM CAUGHT IT. After the
  # 2026-09-05 widening the function needs `SHARED_TARGET_DIRT_SCOPES`, so a
  # lone source returns the `noscope` refusal rather than a count -- which is the
  # designed behaviour and exactly why it must not silently default. Sourcing
  # both here matches what every consumer does.
  run bash -c "
    set -uo pipefail
    . '${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/sharedtarget.lib'
    . '${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/shared/currency.lib'
    _build_inputs_changed '$REPO' '$BASE'
  "
  [ "$status" -eq 0 ]
  [ "$output" = "0" ]
}

# NO ARM FOR THE `nodiff` SENTINEL. Making `git diff` fail while git is present,
# the base reachable and the ancestry confirmed needs a corrupted object store,
# and every cheap way I tried trips an EARLIER guard instead -- so the arm would
# pass while driving a different refusal. AN ARM THAT CANNOT FAIL IS NOT A TEST,
# and one that fires on the wrong branch is worse. Recorded as UNDRIVEN.

# --------------------------------------------------------------------------
# THE 2026-09-05 ARMS. Both defects were LATENT on the live tree when they were
# found -- the scope one because both pathspecs returned the same count that
# day, the git one because the gate hands the check the same repo the hook
# belongs to. Neither could be caught by observing the live estate, so each arm
# below plants the case where the right answer and the wrong answer DIFFER.
# --------------------------------------------------------------------------

@test "a build input OUTSIDE native/rust puts the pair behind HEAD -- the old narrow scope called this clean" {
  plant "$BASE"
  touch_surface_only
  run artefact_currency_verdict "$REL" "$REPO"
  assert_success
  [[ "$output" == refuse:* ]] || fail "expected a refusal for a surface-only change, got: $output"
  [[ "$output" == *"surface"* ]] || fail "the refusal must name the scope it measured, got: $output"
}

@test "the scope in the message is DERIVED from the array, not typed beside it" {
  plant "$BASE"
  touch_source
  run artefact_currency_verdict "$REL" "$REPO"
  assert_success
  local phrase; phrase="$(artefact_currency_scope_phrase)"
  [[ "$output" == *"$phrase"* ]] || fail "message does not carry the derived phrase '$phrase': $output"
  # AND THE PHRASE IS NOT A CONSTANT: strip the array and it must change, or this
  # arm passes for a hardcoded string that happens to match.
  local saved=("${SHARED_TARGET_DIRT_SCOPES[@]}")
  SHARED_TARGET_DIRT_SCOPES=()
  local empty; empty="$(artefact_currency_scope_phrase)"
  SHARED_TARGET_DIRT_SCOPES=("${saved[@]}")
  [ "$empty" != "$phrase" ] || fail "the phrase did not move when the array was emptied -- it is not derived"
}

@test "an UNDECLARED scope REFUSES rather than falling back to a narrower default" {
  plant "$BASE"
  touch_source
  local saved=("${SHARED_TARGET_DIRT_SCOPES[@]}")
  SHARED_TARGET_DIRT_SCOPES=()
  run artefact_currency_verdict "$REL" "$REPO"
  SHARED_TARGET_DIRT_SCOPES=("${saved[@]}")
  assert_success
  [[ "$output" == refuse:* ]] || fail "an undeclared scope must refuse, got: $output"
  [[ "$output" == *"undeclared"* ]] || fail "the refusal must say WHY, got: $output"
}

@test "the verdict is about the tree it was HANDED, even with GIT_DIR set as a hook sets it" {
  # A SECOND REPO WHOSE HONEST ANSWER IS THE OPPOSITE. Without this the arm
  # cannot discriminate: if both repos gave the same verdict, GIT_DIR winning
  # and GIT_DIR losing would look identical.
  local other="$TEST_TEMP_DIR/other"
  mkdir -p "$other/native/rust/crates"
  git init -q "$other"
  git -C "$other" config user.email "test@example.com"
  git -C "$other" config user.name "test_user"
  echo "fn main() {}" > "$other/native/rust/crates/lib.rs"
  git -C "$other" add -A
  git -C "$other" commit -qm "base"
  local other_base; other_base="$(git -C "$other" rev-parse HEAD)"
  echo "// changed" >> "$other/native/rust/crates/lib.rs"
  git -C "$other" add -A
  git -C "$other" commit -qm "source change"

  # THIS repo is clean at BASE; the OTHER is one source commit behind.
  plant "$BASE"
  run artefact_currency_verdict "$REL" "$REPO"
  assert_success
  [ "$output" = "ok" ] || fail "control: the handed repo must be ok before GIT_DIR is involved, got: $output"

  GIT_DIR="$other/.git" run artefact_currency_verdict "$REL" "$REPO"
  assert_success
  [ "$output" = "ok" ] || fail "GIT_DIR won over the handed repo -- the verdict described the wrong tree: $output"

  # AND THE MIRROR, which is the direction that fails OPEN: a stale pair must not
  # read as ok because GIT_DIR points somewhere clean.
  local other_rel="$TEST_TEMP_DIR/other_release"
  mkdir -p "$other_rel"
  printf 'padding [intent-source-commit:%s] padding\n' "$other_base" > "$other_rel/intent"
  printf 'padding [intent-source-commit:%s] padding\n' "$other_base" > "$other_rel/intentd"
  GIT_DIR="$REPO/.git" run artefact_currency_verdict "$other_rel" "$other"
  assert_success
  [[ "$output" == refuse:* ]] || fail "a stale pair read as clean because GIT_DIR pointed at a clean tree: $output"
}
