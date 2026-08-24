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
# WHY IT IS CHEAP RATHER THAN CLEVER. Measured at the time of writing: 224 of
# 226 files in the shipped surface are byte-identical across the two trees. The
# surface is already in lockstep, so a plain comparison is nearly green and
# costs one `cmp` per file.
#
# SCOPE IS THE SHIPPED SURFACE ONLY -- what a consumer project receives.
# `bin/` is deliberately EXCLUDED: it carries v3-only shims (`intent3`,
# `intentd3`) and is where the two trees legitimately part.

load "../lib/test_helper.bash"

# The v2 checkout is not present on every machine or in CI. Absent means skip,
# never fail: a guard that fails where it cannot apply gets deleted, not fixed.
_v2_root() {
  local v2="${INTENT_V2_CHECKOUT:-$HOME/Devel/prj/Intentv2}"
  [ -d "$v2/lib/templates" ] || return 1
  echo "$v2"
}

# DECLARED EXCEPTIONS: files allowed to differ, each with its reason.
#
# An exception is a BACKPORT QUESTION, not a permanent fork. Both entries here
# are v3 work by cc that has not been carried back to v2-maintenance, and cc's
# own comment on the first says the mechanism is correct in BOTH trees -- which
# argues it belongs in v2 too. Empty this list rather than growing it.
_is_exception() {
  case "$1" in
    # cc 2026-08-21: a self-hosted Intent checkout resolves its guards from
    # itself rather than from $INTENT_HOME. Not yet in v2-maintenance.
    lib/templates/hooks/pre-commit.sh) return 0 ;;
    # cc ST0057 AC-01.5: the pre-commit block refuses a gate it cannot run.
    # Not yet in v2-maintenance.
    intent/plugins/claude/bin/intent_claude_upgrade) return 0 ;;
    *) return 1 ;;
  esac
}

@test "transitional: the shipped surface has not diverged between v3 canon and v2-maintenance" {
  local v2; v2="$(_v2_root)" || skip "v2-maintenance checkout not present; nothing to compare"
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
  done < <(find lib/templates intent/plugins/claude -type f ! -name '.DS_Store' 2>/dev/null | sort)

  [ -z "$drifted$missing" ] || fail "shipped surface has drifted between the two checkouts:$drifted$missing

Both trees must receive the same shipped-surface change. The v2 checkout at
$v2 is what every project on this machine actually executes, so a fix landed
only in v3 reaches nobody while looking done.

If a difference is DELIBERATE, add it to _is_exception with its reason -- and
treat that as a backport question rather than a permanent fork."
}

# POSITIVE CONTROL. Without this the test above passes when the find produces
# nothing at all -- a guard comparing an empty set is green and useless.
@test "positive control: the drift comparison actually walks a non-empty surface" {
  local v2; v2="$(_v2_root)" || skip "v2-maintenance checkout not present"
  cd "$INTENT_PROJECT_ROOT" || exit 1

  local n
  n="$(find lib/templates intent/plugins/claude -type f ! -name '.DS_Store' 2>/dev/null | wc -l | tr -d '[:space:]')"
  [ "${n:-0}" -gt 100 ] || fail "shipped surface walk found only ${n:-0} files; the comparison above is not covering anything"
}

# The exception list must not quietly become the mechanism. Each entry is a
# file that SHOULD converge; if this list grows, the guard has stopped guarding.
@test "the declared-exception list stays small and every entry still exists" {
  local v2; v2="$(_v2_root)" || skip "v2-maintenance checkout not present"
  cd "$INTENT_PROJECT_ROOT" || exit 1

  local declared="lib/templates/hooks/pre-commit.sh intent/plugins/claude/bin/intent_claude_upgrade"
  local count=0 f
  for f in $declared; do
    count=$((count + 1))
    [ -f "$f" ] || fail "declared exception no longer exists: $f -- remove it from _is_exception"
  done
  [ "$count" -le 4 ] || fail "exception list has grown to $count; the guard is being routed around rather than satisfied"

  # An exception that has CONVERGED should be retired, not left standing.
  for f in $declared; do
    if [ -e "$v2/$f" ] && cmp -s "$f" "$v2/$f"; then
      fail "$f no longer differs -- it converged. Remove it from _is_exception so the guard covers it again."
    fi
  done
}
