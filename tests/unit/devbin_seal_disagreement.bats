#!/usr/bin/env bats
# The runner REFUSES a verdict when a gate's return code and its seal disagree.
#
# WHAT WENT WRONG, MEASURED 2026-08-17. The bats leg sealed a non-empty `.errors`
# (one failure of 1311) at 04:05:33; the Rust leg sealed empty at 04:06:47; the
# run was reported to a human as "100% green (rust and bats)" at 04:07, and they
# acted on it. `run_all` built its FAILED set from each gate's RETURN CODE alone,
# while every gate ALSO seals an `.errors` companion that is empty exactly when
# that gate was green. Two independent claims about one run, and nothing compared
# them.
#
# WHY IT WAS UNREPRESENTABLE RATHER THAN MERELY UNCHECKED. `record_seal` returned
# early when rc was 0, so a green gate never entered the ledger -- the row a false
# green would have to contradict was never written. The fix is not a check bolted
# onto the summary; it is recording the second source at all.
#
# WHY THESE ARE DRIVEN RATHER THAN REASONED ABOUT. A CORRECT gate cannot produce
# a disagreement, so this refusal has no natural fixture and would otherwise ship
# having never fired. That is the same trap as a mutation that fails to apply: an
# arm that has never run reports exactly what a working arm reports. The two arms
# that matter -- C and E below -- are therefore built by hand, and the ledger they
# read is written by the REAL `record_seal` rather than by a printf in this file,
# so writer and reader are paired by construction instead of by two copies of a
# format agreeing.

load "../lib/test_helper.bash"

DEVBIN_LIB="${INTENT_PROJECT_ROOT}/bin/.devbin/lib"

# Deliberately NOT a `setup()`. Defining one here would silently replace
# test_helper.bash's, every existing test would still pass because they use
# absolute paths, and TEST_TEMP_DIR would quietly stop being created.
load_devbin() {
  # shellcheck disable=SC1090
  . "${DEVBIN_LIB}/helpers"
  # shellcheck disable=SC1090
  . "${DEVBIN_LIB}/runlog"
}

# The three states an `.errors` companion can be in. Named, because the middle
# one is the whole point: non-empty is NOT the same as "recorded a failure".
seal_green() { : >"$1"; }
seal_failed() { printf 'not ok 532 something\n' >"$1"; }
seal_inflight() { inflight_marker >"$1"; }

# Build a ledger through the REAL writer, so a format change breaks this file
# rather than sliding past it.
ledger_with() { # ledger_with <ledger> [rc label seal]...
  local ledger="$1"
  shift
  : >"$ledger"
  DEVBIN_SEAL_LEDGER="$ledger"
  while [ "$#" -ge 3 ]; do
    record_seal "$1" "$2" "$3"
    shift 3
  done
  unset DEVBIN_SEAL_LEDGER
}

# --------------------------------------------------------------------
# The change that makes a disagreement representable at all
# --------------------------------------------------------------------

@test "a GREEN gate is recorded in the ledger, which is what makes a false green contradictable" {
  load_devbin
  local ledger="${TEST_TEMP_DIR}/ledger" seal="${TEST_TEMP_DIR}/green.errors"
  seal_green "$seal"
  ledger_with "$ledger" 0 green-gate "$seal"

  # Before this change `record_seal` returned early on rc=0 and this file was
  # empty -- so there was nothing for a seal to disagree WITH.
  run cat "$ledger"
  assert_success
  assert_output_contains "green-gate"
  # rc leads the row: the summary filters on it, and a reader who has to reach
  # the end of the line to learn which claim it is looking at stops looking.
  [ "${lines[0]:0:2}" = "0	" ]
}

# --------------------------------------------------------------------
# A -- the steady state is SILENCE, so it is asserted first
# --------------------------------------------------------------------

@test "A BASELINE: an all-green run produces no disagreement" {
  load_devbin
  local ledger="${TEST_TEMP_DIR}/ledger"
  local a="${TEST_TEMP_DIR}/a.errors" b="${TEST_TEMP_DIR}/b.errors"
  seal_green "$a"
  seal_green "$b"
  ledger_with "$ledger" 0 shell "$a" 0 rust "$b"

  run seal_disagreements "$ledger"
  assert_success
  assert_output ""
}

# --------------------------------------------------------------------
# D -- a gate run directly, with no `all` above it, is untouched
# --------------------------------------------------------------------

@test "D BASELINE: no ledger at all is not a disagreement" {
  load_devbin
  run seal_disagreements ""
  assert_success
  assert_output ""

  run seal_disagreements "${TEST_TEMP_DIR}/never-written"
  assert_success
  assert_output ""
}

# --------------------------------------------------------------------
# B -- a real failure stays a real failure, and does NOT also refuse
# --------------------------------------------------------------------

@test "B: a genuinely failing leg is FAILED and is NOT reported as a disagreement" {
  # The two sets must be disjoint. If a real failure also tripped the refusal,
  # the refusal would fire on every red run and be trained out of a reader within
  # a week -- which is how a guard stops being read.
  load_devbin
  local ledger="${TEST_TEMP_DIR}/ledger" seal="${TEST_TEMP_DIR}/red.errors"
  seal_failed "$seal"
  ledger_with "$ledger" 1 shell "$seal"

  run seal_failures "$ledger"
  assert_success
  assert_output_contains "shell"
  assert_output_contains "red.errors"

  run seal_disagreements "$ledger"
  assert_success
  assert_output ""
}

# --------------------------------------------------------------------
# C and E -- the refusals. Neither can be produced by a correct gate.
# --------------------------------------------------------------------

@test "C: a gate that returned 0 while its seal records a FAILURE is REFUSED" {
  # The defect itself, and the one that cost a human a wasted morning: the leg
  # sealed a failure, returned 0, and the run was reported green.
  load_devbin
  local ledger="${TEST_TEMP_DIR}/ledger" seal="${TEST_TEMP_DIR}/lying.errors"
  seal_failed "$seal"
  ledger_with "$ledger" 0 shell "$seal"

  run seal_disagreements "$ledger"
  assert_success
  assert_output_contains "shell"
  assert_output_contains "RECORDS A FAILURE"
  assert_output_contains "lying.errors"
  # It must not send the reader to the OTHER cause.
  refute_output_contains "WITHOUT SEALING"
}

@test "E: a gate that returned 0 without ever sealing is REFUSED, and says so DIFFERENTLY" {
  # `open_run_log` SEEDS the file, so a never-sealed run leaves a non-empty
  # `.errors` holding the in-flight marker. Same defect class, different place to
  # go look -- and this is the arm most likely to fire in real life, because a
  # killed or timed-out run produces it. Reporting it as "its seal records a
  # failure" would send the reader hunting for a failing test that does not exist.
  load_devbin
  local ledger="${TEST_TEMP_DIR}/ledger" seal="${TEST_TEMP_DIR}/unsealed.errors"
  seal_inflight "$seal"
  ledger_with "$ledger" 0 rust "$seal"

  run seal_disagreements "$ledger"
  assert_success
  assert_output_contains "rust"
  assert_output_contains "WITHOUT SEALING A VERDICT"
  refute_output_contains "RECORDS A FAILURE"
}

@test "C and E are told apart by the marker's ONE home, not by a copy of its wording" {
  # The discriminator is `seal_is_inflight`, which compares against
  # `inflight_marker` -- the same function `open_run_log` writes with. A copy of
  # that string in the reader would drift in the silent direction: reword the
  # marker and every UNSEALED run starts being reported as a sealed failure.
  load_devbin
  local seal="${TEST_TEMP_DIR}/probe.errors"

  seal_inflight "$seal"
  run seal_is_inflight "$seal"
  assert_success

  seal_failed "$seal"
  run seal_is_inflight "$seal"
  assert_failure

  # And the writer really does use it, so the two ends cannot drift apart.
  run grep -cE '^ *inflight_marker >"\$errors"$' "${DEVBIN_LIB}/runlog"
  assert_success
  assert_output "1"
}

# --------------------------------------------------------------------
# Structural: one home for the ledger format
# --------------------------------------------------------------------

@test "the ledger format is parsed beside the writer, never a second time in resolve" {
  # The reader used to live in run_all while the writer lived in runlog, so one
  # tab-separated format had two homes in two files -- and adding the rc field is
  # exactly the change that turns that into a silent mis-parse at one end.
  run grep -cE 'IFS="\$\(printf .\\t.\)" read' "${DEVBIN_LIB}/resolve"
  assert_failure

  run grep -cE '^(seal_failures|seal_disagreements)\(\) \{' "${DEVBIN_LIB}/runlog"
  assert_success
  assert_output "2"
}

@test "run_all reports a disagreement even when another gate genuinely failed" {
  # The shape this arrived in was `if failed / elif disagreed`, which hides a
  # false green behind an unrelated real failure -- and the false green is the
  # more dangerous of the two, because it is the one nobody goes looking for.
  run grep -cE '^  elif \[ -n "\$disagreed" \]' "${DEVBIN_LIB}/resolve"
  assert_failure

  run grep -cE '^  if \[ -z "\$failed" \] && \[ -z "\$disagreed" \]; then' "${DEVBIN_LIB}/resolve"
  assert_success
  assert_output "1"
}
