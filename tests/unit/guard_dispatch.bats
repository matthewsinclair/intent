#!/usr/bin/env bats
# Every shipped guard is reachable from the shipped hook.
#
# **THESE TWO TESTS LIVED IN `whiteboard_header_guard.bats` AND BOTH FACTS IN
# THAT SENTENCE WERE WRONG BY 2026-08-20.** They were in a file named for ONE
# guard while asserting something about ALL of them, and their population was
# the glob `whiteboard-*-guard.sh` while the roster had generalised to any
# guard at all. Measured at the move: the glob matched 2, the roster carried 4,
# and the two it could not see -- `canon-ignore-guard.sh` and
# `append-only-guard.sh` -- are **exactly the two that had never run in this
# repository.** A test written to catch SHIPPED BUT NEVER INVOKED was
# structurally blind to the only two instances of it (cc, measured).
#
# So the file is named for the property rather than for one of its subjects,
# and the population is every `*-guard.sh` that ships. **A checker whose
# population is narrower than its claim is the same defect as a roster that
# names what it does not run** -- one level up, and it is why repointing the
# path at the new roster would have been necessary and not sufficient.
#
# THE ORIGINAL REASON STILL STANDS AND IS WHY THIS IS LOAD-BEARING: the same
# three checks once lived in two homes, one home followed a tree move and the
# other did not, and CI was green throughout. **A guard that ships and is never
# invoked is worse than an absent one** -- it is in MODULES.md, it has its own
# tests, and it enforces nothing. Neither the guard's tests nor the hook's would
# notice.
#
# WHAT THIS CHECKS AND WHAT IT DOES NOT. It checks the two structural links of
# the chain: the shipped hook dispatches the runner, and the runner's roster
# names every shipped guard. It does NOT execute them -- `pre_commit_hook.bats`
# drives the whole chain end to end through a real `git commit`, including the
# three absences, and duplicating that here would be a second home for it.

load "../lib/test_helper.bash"

setup() {
  HOOK="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit.sh"
  RUNNER="${INTENT_PROJECT_ROOT}/lib/templates/hooks/pre-commit-guards.sh"
  HOOKS_DIR="${INTENT_PROJECT_ROOT}/lib/templates/hooks"
}

@test "the shipped hook dispatches the guard runner" {
  # LINK ONE. The hook is the file COPIED into a consumer's .git/hooks/, so if
  # it does not reach the runner nothing else here matters.
  grep -qF 'pre-commit-guards.sh' "$HOOK"
  grep -qE '^[[:space:]]*bash "\$GUARD_RUNNER"' "$HOOK"
}

@test "the shipped hook names no guard and holds no roster" {
  # THE PROPERTY SHAPE 3 EXISTS FOR, asserted rather than remembered. The
  # roster went stale for six days precisely because it travelled inside this
  # copied file; a future edit that puts a guard name back here re-buys that,
  # and would otherwise pass every test in this file.
  local g name found=""
  for g in "$HOOKS_DIR"/*-guard.sh; do
    [ -f "$g" ] || continue
    name="$(basename "$g")"
    grep -qF "$name" "$HOOK" && found="${found} ${name}"
  done
  if [ -n "$found" ]; then
    echo "the COPIED hook names guards, so the roster can go stale again:${found}"
    return 1
  fi
}

@test "every shipped guard is named in the runner's roster" {
  # LINK TWO, over EVERY guard rather than the whiteboard ones. The `'<name>|'
  # form matches a roster ENTRY and not a mention in prose: this file is heavily
  # commented and several guards are discussed by name in it, so a bare
  # substring test would pass on a guard that had been described and removed.
  local g name missing=""
  for g in "$HOOKS_DIR"/*-guard.sh; do
    [ -f "$g" ] || continue
    name="$(basename "$g")"
    grep -qF "|${name}|" "$RUNNER" || missing="${missing} ${name}"
  done
  if [ -n "$missing" ]; then
    echo "shipped but named in no roster entry:${missing}"
    return 1
  fi
}

@test "the population is not empty and covers all four shipped guards" {
  # THE VACUITY CONTROL, and it is not ceremony: the loops above pass trivially
  # if the glob matches nothing, which is exactly what a directory rename would
  # produce. Naming the count AND the two that used to be invisible means a
  # future narrowing of the glob fails here rather than going quietly green.
  local count
  count="$(find "$HOOKS_DIR" -maxdepth 1 -name '*-guard.sh' | wc -l | tr -d ' ')"
  [ "$count" -ge 4 ]
  grep -qF '|whiteboard-clock-guard.sh|' "$RUNNER"
  grep -qF '|whiteboard-header-guard.sh|' "$RUNNER"
  grep -qF '|canon-ignore-guard.sh|' "$RUNNER"
  grep -qF '|append-only-guard.sh|' "$RUNNER"
}

@test "the runner runs every guard before deciding" {
  # Stopping at the first refusal costs a node one commit attempt per defect,
  # and a board with a bad stamp AND an escaped value is one editing session.
  # The aggregate is what makes the roster safe to grow.
  #
  # Moved with the roster: this asserted `WB_BLOCKED` in the hook, which is
  # where the aggregation used to happen.
  grep -qF 'BLOCKED=1' "$RUNNER"
  grep -qF 'exit "$BLOCKED"' "$RUNNER"
  # And the hook must still turn the runner's verdict into a refusal, or the
  # aggregate is computed and discarded.
  grep -qF 'bash "$GUARD_RUNNER" || exit 1' "$HOOK"
}

@test "the runner reports on stdout that it ran" {
  # SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING (cc, measured on
  # the first commit after the core.hooksPath redirect). Every other message in
  # the runner goes to stderr and only on a problem, so a passing run printed
  # nothing -- and so does a runner nothing dispatched. This is the one line
  # that tells them apart, and it is asserted because it is the only observable
  # difference a normal commit produces.
  grep -qF 'guards: %d ran' "$RUNNER"
}
