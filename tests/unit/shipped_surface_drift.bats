#!/usr/bin/env bats
# TRANSITIONAL GUARD -- DELETE THIS WHOLE FILE WHEN v2 IS END-OF-LIFE.
#
# hv's ruling, 2026-08-24, verbatim: "intent's devbin use is idiosyncratic for
# now, but will resolve at Intentv3. That is because Intent itself has a script
# that other projects use. That's going away with Intentv3 which moves to a
# native rust cli." Sized accordingly: a cheap guard for the remaining v2 life,
# not a permanent mechanism. The durable investment goes to v3-native
# equivalents.
#
# WHAT IT GUARDS. Every project on this machine resolves `intent` through
# $INTENT_HOME to the FROZEN v2-maintenance checkout, while development happens
# in the v3 tree. A fix landed in only one of them reaches nobody and presents
# as done. That happened three times in one day (2026-08-24): the Claude Code
# hook door, the git commit guards, and the `intent upgrade` verb. The
# both-checkouts landing discipline is mandatory until v3 ships; this test is
# what makes forgetting it visible.
#
# THE PROPERTY IS NOT "THE TWO TREES AGREE". hv's freeze ruling, 2026-08-24,
# is SCOPED: Intentv2 is frozen for FEATURES and live for SHIPPED-SURFACE
# DEFECTS. Convergence is therefore the goal for a defect fix and a defect for
# a feature. What this guard can actually assert is the property that survives
# both cases: EITHER IN BOTH TREES OR DECLARED. The declaration lists below are
# the mechanism, and they are split by which of the two kinds an entry is,
# because only one of them should ever shrink to zero.
#
# WHY IT IS CHEAP RATHER THAN CLEVER. Measured 2026-08-24 at 797ea1b7: of 247
# files in the shipped surface, 243 are byte-identical across the two trees, 2
# differ and 2 are absent -- and all four are declared below, with nothing left
# over. The surface is already in lockstep, so a plain comparison is nearly
# green and costs one `cmp` per file.
#
# SCOPE IS THE SHIPPED SURFACE ONLY -- what a consumer project receives.
#
# `intent/plugins/agents/` WAS MISSING FROM THIS WALK FOR TWO HOURS AFTER THIS
# FILE WAS WRITTEN, and the omission is worth recording because it is the same
# class the fence exists to catch. The surface was enumerated from the paths
# that had come up in the round -- `lib/templates` and `intent/plugins/claude`
# -- rather than from what `init` and `lang init` actually read. 20 files that
# seed every consumer project sat outside the guard. conflab-vc's finding that
# a false auth prohibition ships from `templates/elixir/RULES.md` is what
# surfaced the path, not the guard.
#
# A ROSTER ASSEMBLED FROM WHAT WAS RECENTLY DISCUSSED IS NOT A ROSTER OF THE
# SUBJECT.
# `bin/` is deliberately EXCLUDED: it carries v3-only shims (`intent3`,
# `intentd3`) and is where the two trees legitimately part.

load "../lib/test_helper.bash"

SURFACE_PATHS="lib/templates intent/plugins/claude intent/plugins/agents"

# THIS GUARD SKIPPED ITSELF INTO USELESSNESS FOR ITS FIRST DAY, AND THE SKIP
# WAS IN THE ORIGINAL DESIGN RATHER THAN IN A REGRESSION. It resolved the v2
# tree from a filesystem path only, so on a CI runner -- where no such
# directory exists -- all three tests skipped, INCLUDING THE POSITIVE CONTROL
# WHOSE ENTIRE JOB IS TO PROVE THE COMPARISON LOOKED AT SOMETHING. bats reports
# a skip as `ok`, so the runner printed "All tests passed!" and exited 0. A
# guard that only works where a human is already standing is the exact shape hv
# had already declined twice by name (direnv, and refreshing the frozen copy by
# hand). Nobody applied that criterion to the detector itself.
#
# TWO ROUTES TO THE SAME DIRECTORY, so the comparison below has one form.
#
#   Route 1, GROUND TRUTH: the live checkout. This is the tree $INTENT_HOME
#   actually resolves to, so it is what the fleet executes and the only source
#   that can answer "did this fix reach anybody".
#
#   Route 2, PROXY: `git archive` of the pushed v2-maintenance ref into a temp
#   dir. Needs no working tree, which is precisely what CI has. Validated
#   rather than assumed -- see the proxy-currency test below, which is the
#   thing that catches the ref going stale against the checkout.
#
# Route 1 wins where both exist. REMOTE-TRACKING REFS ONLY, never a bare local
# `v2-maintenance`: hv ruled the stale local branch deleted on 2026-08-24
# because it resolved silently to a two-thousand-commit-old revision, and
# accepting one here would reinstate the trap inside the guard.
_v2_ref() {
  local r
  for r in origin/v2-maintenance upstream/v2-maintenance local/v2-maintenance; do
    git -C "$INTENT_PROJECT_ROOT" rev-parse --verify --quiet "$r" >/dev/null 2>&1 && { echo "$r"; return 0; }
  done
  return 1
}

_v2_checkout() {
  local v2="${INTENT_V2_CHECKOUT:-$HOME/Devel/prj/Intentv2}"
  [ -d "$v2/lib/templates" ] || return 1
  echo "$v2"
}

# Extract the ref's shipped surface into a directory laid out identically to a
# checkout, so `$dir/$relpath` addresses a file either way.
_v2_from_ref() {
  local ref dest
  ref="$(_v2_ref)" || return 1
  dest="${BATS_TEST_TMPDIR:-$(mktemp -d)}/v2-surface"
  [ -d "$dest" ] && { echo "$dest"; return 0; }
  mkdir -p "$dest" || return 1
  git -C "$INTENT_PROJECT_ROOT" archive "$ref" $SURFACE_PATHS 2>/dev/null > "$dest/.surface.tar" || return 1
  tar -x -C "$dest" -f "$dest/.surface.tar" || return 1
  rm -f "$dest/.surface.tar"
  echo "$dest"
}

_v2_surface_dir() {
  _v2_checkout || _v2_from_ref
}

# SKIP LOCALLY, FAIL IN CI, AND THE ASYMMETRY IS THE WHOLE POINT. A guard that
# skips when its input is missing cannot tell "not applicable" from "broken",
# and those two need opposite responses. On a developer machine with neither a
# checkout nor a fetched ref, not-applicable is the honest reading. In CI the
# ref is always fetchable, so absence means the wiring broke -- and a silent
# skip there is the defect this rewrite exists to close.
#
# THE MESSAGES LIVE HERE AND THE `skip`/`fail` CALLS LIVE IN THE TEST BODIES,
# WHICH IS NOT A STYLE CHOICE. bats implements both by unwinding the shell they
# are called in, so `v2="$(_helper)"` -- the obvious factoring -- aborts the
# COMMAND SUBSTITUTION'S SUBSHELL and lets the test carry on with an empty
# path. The first draft of this rewrite did exactly that, and instead of
# skipping it compared the whole v3 surface against "" and reported all 247
# files as drifted: the maximum-noise outcome, in CI, unattended. Caught by the
# negative control rather than by review, because it looks correct.
V2_ABSENT_LOCALLY="no v2 checkout and no fetched v2-maintenance ref; nothing to compare"
V2_ABSENT_IN_CI="no v2 shipped surface available and this is CI, where one must be.
Neither a checkout (\$INTENT_V2_CHECKOUT or ~/Devel/prj/Intentv2) nor a
remote-tracking v2-maintenance ref could be resolved. In CI the ref is the
intended route: the workflow fetches it explicitly. If that step was removed
or renamed, restore it -- do not relax this into a skip, which is what made
this guard report green over nothing for its first day."

# DECLARED EXCEPTIONS, IN TWO KINDS THAT MUST NOT BE CONFLATED.
#
# PENDING: v3 work not yet carried back. These are BACKPORT QUESTIONS, they
# should shrink to zero, and a PENDING entry that has converged is stale and
# must be retired so the guard covers the file again.
PENDING_BACKPORT="lib/templates/hooks/pre-commit.sh intent/plugins/claude/bin/intent_claude_upgrade"
_is_pending_backport() {
  case "$1" in
    # cc 2026-08-21: a self-hosted Intent checkout resolves its guards from
    # itself rather than from $INTENT_HOME. cc's own comment says the mechanism
    # is correct in BOTH trees, which argues it belongs in v2.
    lib/templates/hooks/pre-commit.sh) return 0 ;;
    # cc ST0057 AC-01.5: the pre-commit block refuses a gate it cannot run.
    intent/plugins/claude/bin/intent_claude_upgrade) return 0 ;;
    *) return 1 ;;
  esac
}

# V3-ONLY BY RULING: deliberately never carried back. These are NOT backport
# questions and must not be measured as though convergence were the goal --
# hv froze Intentv2, so convergence here would be a defect, not progress.
#
# THIS CATEGORY EXISTS BECAUSE THE GUARD CAUGHT ITS OWN AUTHOR. Commit
# 5eb2a857 restored the agnostic RULES/ARCHITECTURE templates to v3 under hv's
# v3-only ruling, and this test failed on the next run naming both files. That
# is the guard working: a single-tree landing became visible immediately
# instead of silently reaching nobody. The fix was to declare the intent, not
# to widen the walk.
V3_ONLY_BY_RULING="lib/templates/llm/_RULES.md lib/templates/llm/_ARCHITECTURE.md"
_is_v3_only_by_ruling() {
  case "$1" in
    # hv 2026-08-24: v3 restores the agnostic pair, rewritten. Intentv2 frozen.
    lib/templates/llm/_RULES.md) return 0 ;;
    lib/templates/llm/_ARCHITECTURE.md) return 0 ;;
    *) return 1 ;;
  esac
}

_is_exception() {
  _is_pending_backport "$1" || _is_v3_only_by_ruling "$1"
}

@test "transitional: the shipped surface has not diverged between v3 canon and v2-maintenance" {
  local v2
  if ! v2="$(_v2_surface_dir)"; then
    if [ -n "${CI:-}" ]; then fail "$V2_ABSENT_IN_CI"; else skip "$V2_ABSENT_LOCALLY"; fi
  fi
  cd "$INTENT_PROJECT_ROOT" || exit 1

  local drifted="" missing="" f
  while IFS= read -r f; do
    _is_exception "$f" && continue
    if [ ! -e "$v2/$f" ]; then
      missing="$missing
  ABSENT in v2: $f"
    elif ! cmp -s "$f" "$v2/$f"; then
      drifted="$drifted
  DIFFERS: $f"
    fi
  done < <(find $SURFACE_PATHS -type f ! -name '.DS_Store' 2>/dev/null | sort)

  [ -z "$drifted$missing" ] || fail "shipped surface has drifted between the two checkouts:$drifted$missing

Compared against: $v2

Both trees must receive the same shipped-surface change. The v2 tree is what
every project on this machine actually executes, so a fix landed only in v3
reaches nobody while looking done.

If a difference is DELIBERATE, declare it -- _is_pending_backport if it is a
backport question, _is_v3_only_by_ruling if hv has ruled it v3-only -- and cite
the ruling in the comment beside it."
}

# POSITIVE CONTROL. Without this the test above passes when the find produces
# nothing at all -- a guard comparing an empty set is green and useless. BOTH
# SIDES ARE CHECKED, because the v2 side now has a route that can fail
# half-way: a `git archive` that produced an empty directory would report every
# file ABSENT, which is loud but names the wrong cause.
@test "positive control: the drift comparison actually walks a non-empty surface on both sides" {
  local v2
  if ! v2="$(_v2_surface_dir)"; then
    if [ -n "${CI:-}" ]; then fail "$V2_ABSENT_IN_CI"; else skip "$V2_ABSENT_LOCALLY"; fi
  fi
  cd "$INTENT_PROJECT_ROOT" || exit 1

  local n m
  n="$(find $SURFACE_PATHS -type f ! -name '.DS_Store' 2>/dev/null | wc -l | tr -d '[:space:]')"
  [ "${n:-0}" -gt 100 ] || fail "v3 shipped-surface walk found only ${n:-0} files; the comparison above is not covering anything"

  m="$(find "$v2" -type f ! -name '.DS_Store' 2>/dev/null | wc -l | tr -d '[:space:]')"
  [ "${m:-0}" -gt 100 ] || fail "v2 shipped surface at $v2 yielded only ${m:-0} files; the comparison above would report the whole surface ABSENT and blame the wrong tree"
}

# THE PROXY IS VALIDATED, NOT ASSUMED -- and this is the test that keeps CI's
# answer honest. CI compares against the pushed REF; the fleet executes the
# CHECKOUT. Those are two different objects and only one of them is ground
# truth. Measured 2026-08-24: the checkout sat 2 commits ahead of the pushed
# ref, and the guard's answer survived only because both commits were confined
# to `bin/.devbin/`, which this walk excludes anyway. That is luck, not a
# property -- nothing pushes that branch.
#
# WITHOUT THIS TEST the failure is silent in the worst direction: a v2
# shipped-surface defect fix that is committed but not pushed makes CI compare
# v3 against a ref that lacks it, and CI reddens naming DRIFT -- blaming the
# tree that is correct. This runs only where both routes exist, which is a
# developer machine, which is where the push is owed.
@test "the pushed v2-maintenance ref still stands in for the live v2 checkout" {
  local co ref_dir
  co="$(_v2_checkout)" || skip "no live v2 checkout; nothing to validate the proxy against"
  ref_dir="$(_v2_from_ref)" || skip "no fetched v2-maintenance ref; the proxy route is not in play here"

  local stale="" f rel
  while IFS= read -r f; do
    rel="${f#$ref_dir/}"
    if [ ! -e "$co/$rel" ]; then
      stale="$stale
  in ref, absent from checkout: $rel"
    elif ! cmp -s "$f" "$co/$rel"; then
      stale="$stale
  ref differs from checkout: $rel"
    fi
  done < <(find "$ref_dir" -type f ! -name '.DS_Store' 2>/dev/null | sort)

  [ -z "$stale" ] || fail "the pushed v2-maintenance ref no longer matches the live v2 checkout:$stale

CI compares the v3 surface against the REF; the fleet executes the CHECKOUT.
While these disagree, a green in CI is a statement about the wrong tree, and a
red in CI will name DRIFT while blaming the tree that is correct.

Push the v2 checkout's v2-maintenance branch. Do not silence this by widening
the drift exceptions -- the exception lists declare v3-vs-v2 intent and have
nothing to say about a ref lagging its own checkout."
}

# The exception list must not quietly become the mechanism.
#
# THE TWO LISTS GET SEPARATE CAPS, AND SHARING ONE WAS A LATENT LANDMINE. A
# single `count <= 6` treated both kinds as the same overflow risk, but under
# hv's freeze ruling they move in opposite directions: PENDING is a debt that
# must reach zero, while V3-ONLY growth is the ruling working as intended. A
# shared cap therefore fires on legitimate v3 divergence -- and now that this
# guard runs unattended in CI, it would fire there first, which is how a guard
# trains people to ignore it.
@test "the declared-exception list stays small and every entry still exists" {
  local v2
  if ! v2="$(_v2_surface_dir)"; then
    if [ -n "${CI:-}" ]; then fail "$V2_ABSENT_IN_CI"; else skip "$V2_ABSENT_LOCALLY"; fi
  fi
  cd "$INTENT_PROJECT_ROOT" || exit 1

  local f pend_n=0 ruled_n=0
  for f in $PENDING_BACKPORT; do
    pend_n=$((pend_n + 1))
    [ -f "$f" ] || fail "declared exception no longer exists: $f -- remove it from _is_pending_backport"
  done
  for f in $V3_ONLY_BY_RULING; do
    ruled_n=$((ruled_n + 1))
    [ -f "$f" ] || fail "declared exception no longer exists: $f -- remove it from _is_v3_only_by_ruling"
  done

  # PENDING is a debt. It should shrink; growth means the both-trees discipline
  # is being deferred rather than kept.
  [ "$pend_n" -le 4 ] || fail "pending-backport list has grown to $pend_n; the both-trees discipline is being deferred, not satisfied"

  # V3-ONLY is expected to grow while the freeze holds. The cap is a
  # CHECKPOINT, not a limit -- raising it is hv's call and the act of raising
  # it is what makes the growth visible. It is deliberately not derived from
  # anything, and saying so is better than implying it was.
  [ "$ruled_n" -le 12 ] || fail "v3-only-by-ruling list has grown to $ruled_n. That may be correct under the freeze -- raise this checkpoint deliberately, with hv, rather than as a side effect"

  # ONLY THE PENDING LIST IS CHASED FOR CONVERGENCE. A v3-only-by-ruling entry
  # converging would mean somebody carried it into the frozen tree, which is
  # the opposite of progress -- so it is not measured here at all.
  for f in $PENDING_BACKPORT; do
    if [ -e "$v2/$f" ] && cmp -s "$f" "$v2/$f"; then
      fail "$f no longer differs -- it converged. Remove it from _is_pending_backport so the guard covers it again."
    fi
  done
}
