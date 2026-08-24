#!/usr/bin/env bash
# v2_estate_issue_carry.sh -- does a FIRST migration still carry every issue?
#
# THE NEGATIVE CONTROL FOR intent#0070's FIX, AND IT IS THE ARM THAT MUST NEVER
# GO RED. upgrade_issue_loss.sh (dc) drives the positive arm: an ALREADY-MIGRATED
# v3 project, where issues were destroyed and must now survive. This drives the
# other population -- a REAL v2 estate built by 2.19.0 and migrated by v3, where
# issues were ALWAYS carried correctly and must stay that way.
#
# WHY BOTH ARMS EXIST RATHER THAN ONE. The fix adds a union in `migrate::plan`
# that tops `issues` up from committed canon. A first migration has no canon to
# top up from, so the union must add NOTHING there. A fix that made the
# already-migrated path work by changing what a first migration carries would
# pass dc's script and be wrong; only this arm can see that.
#
# IF THIS EVER REPORTS ANYTHING OTHER THAN "carried all N": the union has begun
# contributing on a path where there is no canon, or `legacy::scan` has stopped
# reaching the v2 issue estate. Either way the fix for intent#0070 has grown a
# second defect, and the counts here name which of the two it is -- MORE than
# the estate held means the union is double-counting, FEWER means the scan is
# short. Do not "repair" this by widening the assertion.
#
# TWO v3 BINARIES ON PURPOSE, exactly as dc's does. A build predating the fix is
# the baseline; the question this asks is not "is it N" but "is it the SAME as
# it was before the union existed".
set -uo pipefail
REPO="${1:?usage: v2_estate_issue_carry.sh <repo> [v2-binary]}"
V2="${2:-$(command -v intent)}"
say() { printf '%s\n' "$*"; }

[ -x "$V2" ] || { say "ABORT: no v2 binary -- this arm needs one to BUILD the estate"; exit 2; }
say "v2 builder: $V2 ($("$V2" --version 2>&1 | head -1))"

arm() {
  local label="$1" bin="$2"
  say ""; say "=================================================="; say "ARM: $label"
  [ -x "$bin" ] || { say "  SKIP -- no binary at $bin"; return 0; }
  say "  marker: $(strings "$bin" 2>/dev/null | grep -o '\[intent-source-commit:[^]]*\]' | head -1)"
  local T; T="$(mktemp -d /tmp/v2carry-XXXXXX)"
  (
    cd "$T" || exit 1
    "$V2" init "V2 Estate" >/tmp/_v2carry.log 2>&1 || {
      say "  ABORT: v2 init failed:"; sed 's/^/    | /' /tmp/_v2carry.log; exit 2; }
    "$V2" st new "Probe thread" >/dev/null 2>&1
    local i; for i in 1 2 3 4 5; do "$V2" issues add "probe issue $i" >/dev/null 2>&1; done

    # BEFORE IS COUNTED OFF THE v2 ESTATE ON DISK, NEVER THROUGH A v3 READ.
    # There is no v3 store yet, so `issues list` would answer zero here and
    # every arm would read as a gain from nothing -- an instrument that cannot
    # register the failure it exists to catch.
    local bi bt
    bi="$(find intent/issues -name '*.md' 2>/dev/null | wc -l | tr -d ' ')"
    bt="$(find intent/st -maxdepth 1 -type d -name 'ST[0-9]*' 2>/dev/null | wc -l | tr -d ' ')"
    say "  BEFORE (v2 estate on disk)  issues=$bi  threads=$bt"
    [ "$bi" -gt 0 ] || { say "  ABORT: the estate has no issues -- an arm that cannot fail is not a test"; exit 2; }

    say "  running: v3 intent upgrade"
    "$bin" upgrade 2>&1 | sed 's/^/    | /'

    # AFTER is read from the STORE through the v3 binary, never from `sync`
    # reporting AGREE -- dc's constraint, because a regression that asserts via
    # the agreement report inherits intent#0069 and the reporting defect then
    # hides this one.
    local ai at
    ai="$("$bin" issues list 2>/dev/null | grep -cE '^[0-9]{4} ' || true)"
    at="$("$bin" st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}' || true)"
    say "  AFTER  (v3 store)           issues=$ai  threads=$at"
    if [ "$ai" -eq "$bi" ]; then
      say "  >>> CONTROL HOLDS: carried all $bi issue(s)"
    elif [ "$ai" -gt "$bi" ]; then
      say "  >>> CONTROL BROKEN: $ai from an estate of $bi -- the union is contributing where there is NO canon"
    else
      say "  >>> CONTROL BROKEN: $ai of $bi -- the v2 issue estate is no longer reaching the store"
    fi
  )
  local rc=$?; rm -rf "$T"; return $rc
}

arm "v3 release build" "$REPO/native/rust/target/release/intent"
arm "v3 debug build"   "$REPO/native/rust/target/debug/intent"
