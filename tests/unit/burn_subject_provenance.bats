#!/usr/bin/env bats
# burn.sh must name the binary it measured, and refuse rather than guess.
#
# burn.sh measures the DELTA between a run under the default INTENT_BIN and a
# run under INTENT_BIN=/usr/bin/false. The mutant arm pins its subject inline at
# the call site, so it is recoverable from the source forever. The default arm
# bound NOTHING and inherited whatever was ambient -- so the one arm whose
# subject actually varies was the one arm with no provenance, and the TSV
# recorded neither the value nor the fact that it had been inherited.
#
# The consequence is not "the baseline probably measured the wrong binary". It
# is that burn-baseline.tsv's subject is UNRECOVERABLE IN BOTH DIRECTIONS: the
# file was last written 2026-08-14, four days before v3 self-hosting, when
# tests/lib/test_helper.bash defaulted INTENT_BIN to the v2 SHELL script -- but
# INTENT_BIN is an environment variable, whoever ran it may have exported
# anything, and the artefact records neither the default nor an override. No
# re-reading settles it and no evidence exists either way.
#
# So burn.sh refuses instead of defaulting. A default here would be a THIRD home
# for a rule the estate already spells two ways (test_helper.bash resolves it to
# the v2 shell script, tests/conformance/run_v2_suite.bash to target/debug), and
# a third copy would hide that disagreement rather than resolve it.
#
# These tests never run the estate. The refusal arms exit before any bats
# invocation, and the header arm points WT at a scratch directory with no
# tests/ tree, so the file loop has nothing to iterate and only the header is
# emitted.

load "../lib/test_helper.bash"

# INTENT_BURN_SCRIPT redirects every test in this file at another copy of the
# script, so these guards can be mutation-tested -- pointed at a deliberately
# broken copy to confirm they go red -- without editing the shipped one. The
# idiom is INTENT_RELEASE_SCRIPT's, in tests/unit/release_script.bats.
BURN="${INTENT_BURN_SCRIPT:-${INTENT_HOME}/intent/st/ST0056/parity/tools/burn.sh}"

@test "burn.sh is syntactically valid" {
  run bash -n "$BURN"
  assert_success
}

@test "burn.sh refuses when INTENT_BIN is unset" {
  # The whole point. An inherited default silently decides which CLI is
  # measured, and nothing downstream can recover that decision.
  run env -u INTENT_BIN WT="$TEST_TEMP_DIR" bash "$BURN"

  [ "$status" -ne 0 ]
  [[ "$output" == *"INTENT_BIN"* ]]
  [[ "$output" == *"set INTENT_BIN explicitly"* ]]
}

@test "burn.sh refuses when INTENT_BIN is not executable" {
  run env INTENT_BIN="$TEST_TEMP_DIR/no-such-binary" WT="$TEST_TEMP_DIR" bash "$BURN"

  [ "$status" -eq 2 ]
  [[ "$output" == *"is not executable"* ]]
}

@test "burn.sh records the subject and its version on the TSV header" {
  # An empty WT means `find tests` matches nothing, so the loop body never runs
  # and this costs one --version call rather than a full estate burn.
  local fake="$TEST_TEMP_DIR/fake-intent"
  cat > "$fake" <<'SH'
#!/usr/bin/env bash
echo "intent 9.9.9 (cafebabe)"
SH
  chmod +x "$fake"

  run env INTENT_BIN="$fake" WT="$TEST_TEMP_DIR" bash "$BURN"

  [ "$status" -eq 0 ]
  [[ "$output" == *"SUBJECT=$fake"* ]]
  [[ "$output" == *"VERSION=intent 9.9.9 (cafebabe)"* ]]
}

@test "burn.sh asks the subject for its version rather than deriving it from the path" {
  # A path is not an identity. native/rust/target/release/intent is a shared
  # artefact peers rebuild, so the same path names a different binary on
  # different days -- which is why the decision is "the release binary at a
  # named commit" and not "the release binary".
  local fake="$TEST_TEMP_DIR/fake-intent"
  cat > "$fake" <<'SH'
#!/usr/bin/env bash
echo "intent 1.2.3 (deadbeef)"
SH
  chmod +x "$fake"

  run env INTENT_BIN="$fake" WT="$TEST_TEMP_DIR" bash "$BURN"

  [ "$status" -eq 0 ]
  # Same path as the test above, deliberately: only the binary's own answer
  # differs, so a header built from the path could not tell these two apart.
  [[ "$output" == *"VERSION=intent 1.2.3 (deadbeef)"* ]]
  [[ "$output" != *"9.9.9"* ]]
}

@test "the header carries the subject without disturbing the five data columns" {
  # Every consumer skips line 1 by construction -- gen_register.sh uses
  # `tail -n +2` and `NR>1`, lib_corpus.sh's corpus_diff uses `tail -n +2 |
  # cut -f1`, and coverage_map.sh looks rows up by `^<file>\t`, which no header
  # can match. Extra fields on line 1 are therefore free; extra fields on a DATA
  # row would not be, so this pins the header as the only place they live.
  local fake="$TEST_TEMP_DIR/fake-intent"
  printf '#!/usr/bin/env bash\necho "intent 9.9.9"\n' > "$fake"
  chmod +x "$fake"

  # Assert against the ARTEFACT, not against `run`'s capture. `run` merges
  # stderr into $output, and burn.sh announces its subject on stderr before the
  # header reaches stdout -- so $output's first line is the announcement, while
  # the TSV's first line is the header. The file is what consumers read, and
  # capturing stdout alone also proves the announcement stays out of it.
  local tsv="$TEST_TEMP_DIR/burn.tsv"
  run env INTENT_BIN="$fake" WT="$TEST_TEMP_DIR" bash -c "bash '$BURN' > '$tsv'"

  [ "$status" -eq 0 ]
  local header
  header="$(head -1 "$tsv")"
  [[ "$header" == "FILE"*"TESTS"*"DEFAULT_FAIL"*"BURN"*"STATUS"*"SUBJECT="* ]]
  # The announcement is on stderr and must not be in the TSV.
  run grep -c 'burn.sh: subject' "$tsv"
  [ "$output" -eq 0 ]
}
