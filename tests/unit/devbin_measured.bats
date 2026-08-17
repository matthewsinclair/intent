#!/usr/bin/env bats
# `int measured` answers the one question a seal cannot: WHICH TREE this verdict
# describes.
#
# WHY IT EXISTS (issue 0049). A seal records which tests failed. Nothing anywhere
# records what they ran against, so a run over a dirty tree and a run over a
# clean checkout produce artefacts identical in form and incomparable in meaning.
# Measured 2026-08-17: two red legs, three sessions establishing neither was a
# regression, and nothing having misbehaved -- every artefact accurate about what
# it measured, none able to say what that was.
#
# THE CONTROL THAT MATTERS HERE IS THE SCOPE TOTAL, not any of the states below.
# The first run of this command reported 164,142 files moved, because the prune
# list used `$ROOT/target` and cargo's output lives at `native/rust/target` -- so
# build artefacts were never excluded. That is the wrong-maximum shape: an
# alarming number reads as a finding and nobody re-checks an instrument that has
# just found something big. Two tests below pin the scope: that a pruned
# directory is genuinely excluded, and that the movement count is reported
# against the population it came from.

load "../lib/test_helper.bash"

MEASURED="${INTENT_MEASURED_SCRIPT:-${INTENT_PROJECT_ROOT}/bin/.devbin/cmd/measured}"
REAL_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib"

# A git checkout with one committed source file and one run's artefacts.
#
# The run is stamped in 2020 so that "now" is unambiguously after it: every file
# the fixture writes without an explicit mtime counts as movement, and anything
# needing to predate the run gets an explicit `touch -t`.
measured_fixture() {
  MROOT="${TEST_TEMP_DIR}/measured"
  rm -rf "$MROOT"
  mkdir -p "$MROOT/src"
  printf 'fn main() {}\n' >"$MROOT/src/main.rs"
  # `tmp/` is ignored, as it is in the real project. Without this the run
  # artefacts below are untracked files and the "clean tree" fixture is dirty --
  # which is how the first version of this file failed, and correctly: the
  # command was right and the fixture was lying.
  printf 'tmp/\ntarget/\n' >"$MROOT/.gitignore"
  git -C "$MROOT" init -q
  git -C "$MROOT" config user.email dc@example.com
  git -C "$MROOT" config user.name dc
  git -C "$MROOT" add -A
  git -C "$MROOT" commit -qm "the fixture commit"
  # Everything committed predates the run.
  touch -t 201912310000 "$MROOT/src/main.rs" "$MROOT/.gitignore"
  mkdir -p "$MROOT/tmp/test"
  printf 'gate output\n' >"$MROOT/tmp/test/20200101-1200.RUST.out"
  : >"$MROOT/tmp/test/20200101-1200.RUST.errors"
  ln -sfn 20200101-1200.RUST.out "$MROOT/tmp/test/LATEST_RUST.out"
  ln -sfn 20200101-1200.RUST.errors "$MROOT/tmp/test/LATEST_RUST.errors"
}

SEAL="tmp/test/20200101-1200.RUST.errors"

run_measured() {
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" "$@"
}

# The same invocation without bats' `run`, for when the OUTPUT is the input to
# another check rather than the thing being asserted.
measured_raw() {
  env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" "$@"
}

OUT="tmp/test/20200101-1200.RUST.out"

# stamp_log <referent-line> -- put a referent at the top of the fixture's run log.
stamp_log() {
  local body
  body="$(cat "$MROOT/$OUT")"
  printf '%s\n%s\n' "$1" "$body" >"$MROOT/$OUT"
}

@test "int measured is syntactically valid" {
  run bash -n "$MEASURED"
  assert_success
}

@test "it refuses to run standalone -- PROJECT_ROOT comes from the dispatcher" {
  run env -u PROJECT_ROOT DEVBIN_LIB="$REAL_LIB" bash "$MEASURED"
  [ "$status" -ne 0 ]
}

# --------------------------------------------------------------------
# THE REFERENT: what tree is this, right now
# --------------------------------------------------------------------

@test "a CLEAN tree reports its commit and says a verdict taken now describes it" {
  measured_fixture
  run_measured
  assert_success
  assert_output_contains "clean"
  refute_output_contains "DESCRIBES NO COMMIT"
}

@test "a DIRTY tree says a verdict taken now describes NO COMMIT" {
  measured_fixture
  printf 'fn main() { }\n' >"$MROOT/src/main.rs"
  run_measured
  assert_success
  assert_output_contains "DESCRIBES NO COMMIT"
  # WITH ITS INDEX COLUMN. A bare "src/main.rs" was satisfied by the MOVEMENT
  # section further down, so blanking this listing entirely left the test green.
  assert_output_contains " M src/main.rs"
}

@test "an untracked file counts as dirty -- it can affect a run as much as an edit" {
  measured_fixture
  printf 'stray\n' >"$MROOT/src/stray.rs"
  run_measured
  assert_output_contains "DESCRIBES NO COMMIT"
  assert_output_contains "?? src/stray.rs"
}

@test "NOT a git checkout degrades rather than crashing" {
  # devbin runs anywhere; this command is the project's, but a project without a
  # VCS must get an honest "nothing here can name a commit" and a zero exit,
  # not a stack of git errors.
  MROOT="${TEST_TEMP_DIR}/novcs"
  rm -rf "$MROOT"
  mkdir -p "$MROOT/tmp/test"
  run_measured
  assert_success
  assert_output_contains "not a git checkout"
}

# --------------------------------------------------------------------
# SEAL STATES -- four, not two
# --------------------------------------------------------------------

@test "a zero-byte seal reports GREEN" {
  measured_fixture
  run_measured
  assert_output_contains "GREEN"
}

@test "a seal with failure content reports RED" {
  measured_fixture
  printf 'test foo FAILED\n' >"$MROOT/$SEAL"
  run_measured
  assert_output_contains "RED"
}

@test "a seal still holding the in-flight marker reports IN-FLIGHT, not RED" {
  # These send a reader to different places: one to a failing test, the other to
  # a killed or unsealed run. Collapsing them sends someone hunting for a failure
  # that does not exist.
  measured_fixture
  printf 'run in flight (or aborted before sealing) -- no completed verdict; see the .out log\n' \
    >"$MROOT/$SEAL"
  run_measured
  assert_output_contains "IN-FLIGHT"
}

@test "a missing seal reports ABSENT, which is a third thing again" {
  measured_fixture
  rm -f "$MROOT/$SEAL"
  run_measured
  assert_output_contains "ABSENT"
}

@test "an unrecognised marker degrades to RED, never to GREEN" {
  # The marker is matched as a prefix. If it is reworded upstream this must fail
  # toward the LOUD answer -- a silently-green misread is the one unrecoverable
  # direction for this whole subsystem.
  measured_fixture
  printf 'run in flight BUT REWORDED UPSTREAM\n' >"$MROOT/$SEAL"
  run_measured
  refute_output_contains "GREEN"
}

# --------------------------------------------------------------------
# MOVEMENT, and the scope it is measured over
# --------------------------------------------------------------------

@test "a file changed AFTER the run started is reported as movement" {
  measured_fixture
  printf 'changed\n' >"$MROOT/src/main.rs"
  run_measured
  assert_output_contains "have a newer mtime than this run"
  assert_output_contains "src/main.rs"
}

@test "a file untouched since BEFORE the run started is NOT movement" {
  measured_fixture
  run_measured
  assert_output_contains "has changed since it started"
  refute_output_contains "have a newer mtime than this run"
}

@test "THE SCOPE CONTROL: a pruned directory is genuinely excluded" {
  # This is the test that would have caught the 164,142 reading. A file written
  # into a pruned build directory must not register as movement.
  measured_fixture
  mkdir -p "$MROOT/target/debug"
  printf 'artefact\n' >"$MROOT/target/debug/thing.o"
  run_measured
  refute_output_contains "thing.o"
}

@test "the movement count is reported AGAINST ITS POPULATION, never bare" {
  # "22 of 1581" is a fact. "164142" is an instrument reporting its own breakage.
  measured_fixture
  printf 'changed\n' >"$MROOT/src/main.rs"
  run_measured
  [[ "$output" =~ of\ [0-9]+\ in-scope\ file ]] ||
    fail "the movement count was printed without its population: $output"
}

@test "the scope total is reported and is not zero" {
  measured_fixture
  run_measured
  assert_output_contains "in scope"
  refute_output_contains "in scope  0 file"
}

# --------------------------------------------------------------------
# WHAT IT READS, and what it refuses to trust
# --------------------------------------------------------------------

@test "it names the DATED artefact, never a LATEST_* pointer" {
  # LATEST_* is re-pointed by every run in the tree, so with several sessions
  # working it is last-writer-wins and can name a peer's run. The dated files are
  # always exact.
  measured_fixture
  run_measured
  assert_output_contains "20200101-1200.RUST.errors"
  refute_output_contains "LATEST_RUST"
}

@test "a run named with the second-qualified form still parses its start time" {
  # The collision ladder produces three name shapes. A start time that fails to
  # parse silently disables the movement test for that run.
  measured_fixture
  mv "$MROOT/tmp/test/20200101-1200.RUST.out" "$MROOT/tmp/test/20200101-1200-42.RUST.out"
  mv "$MROOT/tmp/test/20200101-1200.RUST.errors" "$MROOT/tmp/test/20200101-1200-42.RUST.errors"
  run_measured
  assert_output_contains "2020-01-01 12:00:42"
  refute_output_contains "unparseable"
}

@test "a run named with the counter form still parses its start time" {
  measured_fixture
  mv "$MROOT/tmp/test/20200101-1200.RUST.out" "$MROOT/tmp/test/20200101-1200-42-3.RUST.out"
  mv "$MROOT/tmp/test/20200101-1200.RUST.errors" "$MROOT/tmp/test/20200101-1200-42-3.RUST.errors"
  run_measured
  assert_output_contains "2020-01-01 12:00:42"
  refute_output_contains "unparseable"
}

@test "with no runs at all it says so rather than printing an empty section" {
  MROOT="${TEST_TEMP_DIR}/noruns"
  rm -rf "$MROOT"
  mkdir -p "$MROOT"
  git -C "$MROOT" init -q
  run_measured
  assert_success
  assert_output_contains "nothing has been gated here yet"
}

@test "a named run reports only that run" {
  measured_fixture
  printf 'other output\n' >"$MROOT/tmp/test/20200101-1300.SHELL.out"
  : >"$MROOT/tmp/test/20200101-1300.SHELL.errors"
  run_measured "$MROOT/tmp/test/20200101-1200.RUST.out"
  assert_output_contains "20200101-1200.RUST.errors"
  refute_output_contains "SHELL"
}

@test "a named run may be given as its SEAL path -- the path a verdict line prints" {
  # `verdict:` prints the .errors path, so that is the string a human has in
  # hand. Requiring the .out would make them derive it.
  measured_fixture
  run_measured "$MROOT/tmp/test/20200101-1200.RUST.errors"
  assert_success
  assert_output_contains "20200101-1200.RUST.errors"
}

# --------------------------------------------------------------------
# THE LIMIT, stated in its own output
# --------------------------------------------------------------------

@test "it states that movement is NOT a was-it-clean-at-start test" {
  # The honest boundary. A run that BEGAN on a dirty tree and was never touched
  # again reports no movement, and only a referent recorded at run start could
  # say otherwise. A tool that let a reader believe otherwise would be worse than
  # no tool, because it would certify the exact thing it cannot see.
  measured_fixture
  run_measured
  assert_output_contains "NOT A WAS-IT-CLEAN-AT-START TEST"
}

# --------------------------------------------------------------------
# THE REFERENT STAMP -- `--stamp` and `--exec`
#
# The stamp is what turns "moved" into a real was-it-clean-at-start test. It is
# printed into the run log, which is the same stream the extraction grammars
# parse, so its SHAPE is load-bearing and not cosmetic -- see the canary below.
# --------------------------------------------------------------------

@test "--stamp prints exactly one line, and it is the referent" {
  measured_fixture
  run_measured --stamp
  assert_success
  [ "$(printf '%s\n' "$output" | grep -c '^')" -eq 1 ] ||
    fail "expected one line, got: $output"
  [[ "$output" == devbin-referent:* ]] || fail "not a referent line: $output"
}

@test "--stamp on a CLEAN tree names the commit and says clean" {
  measured_fixture
  run_measured --stamp
  [[ "$output" =~ ^devbin-referent:\ commit=[0-9a-f]+\ tree=clean$ ]] ||
    fail "unexpected stamp: $output"
}

@test "--stamp on a DIRTY tree carries the count" {
  measured_fixture
  printf 'changed\n' >"$MROOT/src/main.rs"
  printf 'stray\n' >"$MROOT/src/stray.rs"
  run_measured --stamp
  # ANCHORED AT BOTH ENDS. A loose `tree=dirty:2$` was satisfied by the refusal
  # message, which quotes the offending line and therefore ends the same way --
  # so a mutant that made the stamp illegal passed this test.
  [[ "$output" =~ ^devbin-referent:\ commit=[0-9a-f]+\ tree=dirty:2$ ]] ||
    fail "expected a well-formed dirty:2 stamp, got: $output"
}

@test "--stamp outside a git checkout says so rather than inventing a commit" {
  MROOT="${TEST_TEMP_DIR}/novcs-stamp"
  rm -rf "$MROOT"
  mkdir -p "$MROOT"
  run_measured --stamp
  assert_success
  assert_output_contains "not-a-git-checkout"
  refute_output_contains "tree=clean"
}

@test "THE CANARY: the stamped line extracts NOTHING under all seven grammars" {
  # THE POINT OF THE WHOLE SHAPE. run_gate does `"$@" 2>&1 | tee "$log"` and
  # writes no header of its own, so this line becomes line 1 of every gate log
  # and every grammar sees it. Two of them match ANYWHERE in a line rather than
  # at the start -- credo opens a block on `[A-Z] `, xcodebuild on `: error: ` --
  # so a stamp that grew a branch name or a commit subject would start sealing
  # failures into green runs, silently. The grammars are SOURCED from devbin,
  # never copied: a canary against a copy tests the copy.
  measured_fixture
  local line
  line="$(measured_raw --stamp)"
  # THE FIXTURE MUST ENGAGE, or this whole test is a green over nothing: if
  # --stamp died, `line` is empty, every grammar extracts nothing from an empty
  # file, and the assertion below passes while testing air.
  [ -n "$line" ] || fail "--stamp produced no line -- this canary would pass on an empty string"
  local bad
  bad="$(
    . "${INTENT_PROJECT_ROOT}/bin/.devbin/lib/helpers"
    . "${INTENT_PROJECT_ROOT}/bin/.devbin/lib/runlog"
    printf '%s\n' "$line" >"$MROOT/canary.log"
    for g in $(grammar_names); do
      out="$(awk "$(grammar_for "$g")" "$MROOT/canary.log" 2>/dev/null | strip_ansi)"
      [ -n "$out" ] && printf '%s ' "$g"
    done
    true
  )"
  [ -z "$bad" ] || fail "these grammars extract a failure from the stamp: $bad -- line was: $line"
}

@test "THE CANARY CAN FAIL: a poisoned line IS caught by the same check" {
  # Without this the test above is a green that proves nothing. `[R] ` is
  # credo's own issue tag; if the canary cannot see it, it cannot see anything.
  measured_fixture
  local bad
  bad="$(
    . "${INTENT_PROJECT_ROOT}/bin/.devbin/lib/helpers"
    . "${INTENT_PROJECT_ROOT}/bin/.devbin/lib/runlog"
    printf '%s\n' 'devbin-referent: [R] commit=deadbeef tree=clean' >"$MROOT/poison.log"
    for g in $(grammar_names); do
      out="$(awk "$(grammar_for "$g")" "$MROOT/poison.log" 2>/dev/null | strip_ansi)"
      [ -n "$out" ] && printf '%s ' "$g"
    done
    true
  )"
  [ -n "$bad" ] || fail "the poisoned line was not caught -- the canary above is blind"
}

@test "--exec propagates the child's exit code, green" {
  measured_fixture
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" --exec true
  assert_success
}

@test "--exec propagates the child's exit code, red" {
  # A WRAPPER THAT SWALLOWED THIS WOULD REINTRODUCE THE WHOLE CLASS IT EXISTS TO
  # CLOSE: run_gate reads the command's status to decide whether to seal empty,
  # so a dropped non-zero here writes a GREEN certificate over a failed run.
  measured_fixture
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" --exec sh -c 'exit 42'
  [ "$status" -eq 42 ] || fail "expected 42, got $status"
}

@test "--exec stamps BEFORE the child runs, so the stamp survives a child that dies" {
  # CHECKED AS LINE 1 AND WITH THE CHILD'S STATUS, not as a substring: the
  # refusal path prints the offending line too, so a bare
  # `assert_output_contains "devbin-referent:"` passed even when the stamp had
  # been refused and the child never ran.
  measured_fixture
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" --exec sh -c 'exit 1'
  [ "$status" -eq 1 ] || fail "expected the child's status 1, got $status"
  [ "$(printf '%s\n' "$output" | head -1 | cut -c1-16)" = "devbin-referent:" ] ||
    fail "the stamp is not the first line: $output"
}

@test "--exec puts the stamp first, ahead of the child's own output" {
  measured_fixture
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" --exec echo child-output
  [ "$(printf '%s\n' "$output" | head -1 | cut -c1-16)" = "devbin-referent:" ] ||
    fail "the stamp is not the first line: $output"
}

@test "--exec with no command REFUSES rather than exiting zero" {
  measured_fixture
  run env PROJECT_ROOT="$MROOT" DEVBIN_LIB="$REAL_LIB" bash "$MEASURED" --exec
  [ "$status" -ne 0 ] || fail "an --exec with nothing to run exited zero"
}

# --------------------------------------------------------------------
# READING THE STAMP BACK
# --------------------------------------------------------------------

@test "a run that started CLEAN reports the commit its verdict describes" {
  measured_fixture
  stamp_log 'devbin-referent: commit=abc1234 tree=clean'
  run_measured
  assert_output_contains "started CLEAN at abc1234"
}

@test "a run that started DIRTY says its verdict never described a commit" {
  measured_fixture
  stamp_log 'devbin-referent: commit=abc1234 tree=dirty:7'
  run_measured
  assert_output_contains "NEVER DESCRIBED A COMMIT"
  assert_output_contains "7 uncommitted"
}

@test "AN UNSTAMPED RUN READS AS NONE RECORDED, never as clean" {
  # The four-states-not-two lesson, one level up. Half the gates here are
  # builtins with no `run:` line to wrap, so they record nothing -- and printing
  # nothing for them would let an unstamped run look exactly like a run that
  # started clean, which is the defect this command exists to close.
  measured_fixture
  run_measured
  # THE PER-RUN SENTENCE, not the bare phrase. The footer explains NONE RECORDED
  # on every run, so a mutant deleting the per-run line left this green.
  assert_output_contains "NONE RECORDED -- this run does not name a tree"
  refute_output_contains "started CLEAN"
}

@test "the referent is read from LINE 1 ONLY, never from the log body" {
  # A narrow window that can miss beats a wide one that can lie: this project's
  # own tests of this feature print referent lines into logs, and any window
  # wider than the mechanism would let a run report a tree it never ran against.
  #
  # This test caught the code doing exactly that. The reader was `head -3` on
  # defensive instinct, the fixture log is one line, so this planted line landed
  # on line 2 -- inside the window -- and the report named `deadbee` as the
  # commit the verdict described. The window is now the mechanism: line 1.
  measured_fixture
  printf 'devbin-referent: commit=deadbee tree=clean\n' >>"$MROOT/$OUT"
  run_measured
  assert_output_contains "NONE RECORDED"
  refute_output_contains "deadbee"
}

@test "a stamp on line 2 is NOT a referent -- the window is the mechanism, not a guess" {
  # The same boundary from the other side, with the log padded so line 2 is
  # unambiguously the body rather than an artefact of a one-line fixture.
  measured_fixture
  {
    printf 'first line of real gate output\n'
    printf 'devbin-referent: commit=deadbee tree=clean\n'
    printf 'more output\n'
  } >"$MROOT/$OUT"
  run_measured
  assert_output_contains "NONE RECORDED"
  refute_output_contains "deadbee"
}

# --------------------------------------------------------------------
# MTIME IS NOT CONTENT (vc, 2026-08-17)
#
# `moved` was purely an mtime test, and it read as a change test. A suite that
# backs a file up and restores it -- tests/unit/rule_index.bats does exactly this
# to the LIVE canon index -- rewrites identical bytes and moves the mtime. On a
# real run, three of eleven files reported as moved were byte-identical to HEAD.
# That is this command's own defect class occurring inside it: an instrument
# reporting movement that is not evidence.
# --------------------------------------------------------------------

@test "a file TOUCHED but not changed is byte-identical, not a content change" {
  measured_fixture
  touch "$MROOT/src/main.rs"
  run_measured
  assert_output_contains "BYTE-IDENTICAL"
  assert_output_contains "src/main.rs"
  refute_output_contains "differ in CONTENT"
}

@test "a file whose CONTENT changed is reported as differing, not merely touched" {
  measured_fixture
  printf 'fn main() { /* changed */ }\n' >"$MROOT/src/main.rs"
  run_measured
  assert_output_contains "differ in CONTENT"
  assert_output_contains "src/main.rs"
}

@test "an untracked file always counts as content, because it is in no commit" {
  # For an untracked file an mtime is genuinely all the signal there is, so it
  # must never be filed under "byte-identical to the reference".
  measured_fixture
  printf 'stray\n' >"$MROOT/src/stray.rs"
  run_measured
  assert_output_contains "differ in CONTENT"
  assert_output_contains "src/stray.rs"
}

@test "the split names WHICH commit it compared against" {
  # A comparison that does not name its reference is the same defect one level
  # up: a measurement whose subject is unstated.
  measured_fixture
  touch "$MROOT/src/main.rs"
  run_measured
  assert_output_contains "compared against"
}

@test "with a referent recorded, the comparison uses it rather than HEAD" {
  measured_fixture
  local sha
  sha="$(git -C "$MROOT" rev-parse --short HEAD)"
  stamp_log "devbin-referent: commit=$sha tree=clean"
  touch "$MROOT/src/main.rs"
  run_measured
  assert_output_contains "the referent this run recorded"
  refute_output_contains "nearest question available"
}

@test "with NO referent it says so rather than implying it compared against the run" {
  measured_fixture
  touch "$MROOT/src/main.rs"
  run_measured
  assert_output_contains "nearest question available"
}
