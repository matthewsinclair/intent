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
  assert_output_contains "src/main.rs"
}

@test "an untracked file counts as dirty -- it can affect a run as much as an edit" {
  measured_fixture
  printf 'stray\n' >"$MROOT/src/stray.rs"
  run_measured
  assert_output_contains "DESCRIBES NO COMMIT"
  assert_output_contains "stray.rs"
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
  assert_output_contains "changed AFTER this run started"
  assert_output_contains "src/main.rs"
}

@test "a file untouched since BEFORE the run started is NOT movement" {
  measured_fixture
  run_measured
  assert_output_contains "has changed since it started"
  refute_output_contains "changed AFTER this run started"
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
