#!/usr/bin/env bats
# ST0056: the BATS estate is the v3 conformance harness, so every test that
# means to exercise THE CLI must reach it through `$INTENT_BIN`.
#
# `tests/lib/test_helper.bash` defines `INTENT_BIN` (defaulting to this repo's
# `bin/intent`) and `run_intent` calls it. A test that spells the dispatcher's
# path directly instead runs v2's shell script no matter what `INTENT_BIN` is
# pointed at -- so under a v3 binary it silently keeps testing v2 and reports
# green. That is the worst available failure: not a red, but a green that means
# nothing.
#
# This is a REGRESSION guard, not a cleanup. The estate was swept to
# `$INTENT_BIN` at `87a315b`; a sixth spelling
# (`"${INTENT_PROJECT_ROOT}/bin/intent"`) appeared in a test written after that
# sweep, which is the proof that a one-shot rewrite does not stay rewritten.
# The needle is deliberately wider than the five forms the sweep found, because
# a guard scoped to what is already clean only certifies the status quo.
#
# Matches the DISPATCHER only: `/bin/intent` not followed by an identifier
# character. `bin/intent_helpers`, `bin/intent_st` and the other ~146 direct
# `intent_<sub>` call sites are a different, classified concern (they bypass the
# dispatcher by design and have no single-binary equivalent -- see the register)
# and are deliberately NOT caught here.

load "../lib/test_helper.bash"

# Files permitted to name the dispatcher path directly, each for a stated
# reason. Adding a row here is a decision; leave the reason with it.
#
#   no_template_fallback.bats -- builds a deliberately BROKEN copy of the
#     install in a scratch dir and runs THAT. It cannot go through $INTENT_BIN,
#     because the whole point is to invoke an install that is not this one.
#
#   intent_bin_retarget_guard.bats -- this file; it contains the needle as data.
ALLOWLIST_RE='tests/unit/(no_template_fallback|intent_bin_retarget_guard)\.bats'

# **A LINE-LEVEL EXEMPTION, NOT A FILE ONE, AND THE DIFFERENCE IS THE WHOLE
# REASON IT IS SEPARATE FROM `ALLOWLIST_RE` ABOVE.**
#
# This guard exists so a test cannot silently measure whatever `intent` is on
# PATH instead of the dispatcher under test. A line that resolves ANOTHER
# INSTALL BY NAME is not that mistake -- it is the opposite, a test being
# explicit about which of two binaries it means. `critic_arming_census` compares
# v2's and v3's exit codes for the same invocation, which it cannot do through
# `$INTENT_BIN` because `$INTENT_BIN` is precisely the one binary it already has.
#
# **ALLOWLISTING THE FILE WOULD HAVE BEEN THE CHEAPER FIX AND THE WRONG ONE**:
# it would retire the guard over every OTHER line in a 300-line file, including
# ones nobody has written yet. The exemption is keyed on `INTENT_V2_CHECKOUT`,
# the variable that makes the other install explicit, so a bare `bin/intent`
# anywhere in that same file still reds.
OTHER_INSTALL_RE='INTENT_V2_CHECKOUT'

# Emit "file:line:text" for every non-comment .bats line naming the dispatcher
# path directly, minus the allowlist. Printing the offenders (not just counting)
# is what makes a failure actionable.
offending_sites() {
  grep -rnE '/bin/intent([^_[:alnum:]]|$)' "${INTENT_PROJECT_ROOT}/tests" --include='*.bats' 2>/dev/null |
    grep -vE '^[^:]+:[0-9]+:[[:space:]]*#' |
    grep -vE "$ALLOWLIST_RE" |
    grep -vE "$OTHER_INSTALL_RE" |
    sed "s|^${INTENT_PROJECT_ROOT}/||"
}

@test "no .bats file invokes the dispatcher by a path that bypasses \$INTENT_BIN" {
  run offending_sites
  # grep exits 1 on no-match, so `run` succeeding is not the assertion -- the
  # empty output is.
  if [ -n "$output" ]; then
    echo "Tests naming bin/intent directly instead of \$INTENT_BIN:"
    echo "$output"
    echo
    echo "Use run_intent, or \"\$INTENT_BIN\" when you need env/flags around it."
    echo "If the file genuinely must run a different install, add it to"
    echo "ALLOWLIST_RE in this guard with the reason."
    return 1
  fi
}

@test "the guard's needle actually matches the form that regressed" {
  # Mutation-proofing in the file itself: if someone loosens the regex, this
  # goes red. The string below is verbatim what appeared in
  # ambient_project_root_guard.bats after the sweep and was not caught.
  regressed='  run env PROJECT_ROOT="$DECOY" "${INTENT_PROJECT_ROOT}/bin/intent" st list'
  echo "$regressed" | grep -qE '/bin/intent([^_[:alnum:]]|$)'
}

@test "the guard's needle does NOT match the intent_<sub> direct call sites" {
  # The complement. Without this, widening the needle to catch the dispatcher
  # would silently start failing on ~146 classified sites, and the fix would be
  # to weaken the guard rather than the estate.
  for line in \
    '  source "${INTENT_PROJECT_ROOT}/bin/intent_helpers"' \
    '  run "${INTENT_BIN_DIR}/intent_st" list' \
    '  run "$INTENT_HOME/bin/intent_migrations"'; do
    if echo "$line" | grep -qE '/bin/intent([^_[:alnum:]]|$)'; then
      echo "needle wrongly matched an intent_<sub> site: $line"
      return 1
    fi
  done
}

@test "the allowlist names only files that exist" {
  # A stale allowlist entry is a hole nobody can see: it suppresses a real
  # finding under a filename that has moved or gone.
  for f in no_template_fallback intent_bin_retarget_guard; do
    assert_file_exists "${INTENT_PROJECT_ROOT}/tests/unit/${f}.bats"
  done
}
