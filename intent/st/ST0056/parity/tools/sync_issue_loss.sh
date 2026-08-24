#!/usr/bin/env bash
# sync_issue_loss.sh -- does a THREAD-SCOPED `sync --to-store <ID>` destroy ISSUES?
#
# Driven for vc, 2026-08-24, after the live store was found holding 0 issues
# against 47 in canon. Two candidate writers: dc's escaped `upgrade` (13:44:15Z)
# and vc's `sync --to-store ST0057` (14:07:21Z). This settles the MECHANISM,
# which is what archaeology on timestamps cannot do.
#
# WRITTEN AS A SCRIPT SO IT IS NOT AUTHOR-LOCKED. ic's finding today: a proof only
# its author can reproduce is not yet a proof the estate holds. vc can run this.
#
# TWO ARMS, BECAUSE THE BINARY CHOICE MAY ITSELF BE THE ANSWER. The release and
# debug binaries differ across intentsvcs/src/{facade,project,skills}.rs -- exactly
# where sync logic lives -- so a result from one says nothing about the other.
#
# TOUCHES NOTHING LIVE. Everything happens under a mktemp dir, and every verb is
# run with cwd INSIDE it -- the containment defect this estate found in
# cmd/hosting today was precisely a clone whose verbs ran from the live cwd.
set -uo pipefail

REPO="${1:?usage: sync_issue_loss.sh <path-to-Intent-checkout>}"
say() { printf '%s\n' "$*"; }
arm() {
  local label="$1" bin="$2"
  say ""
  say "=================================================================="
  say "ARM: $label"
  say "  binary: $bin"
  if [ ! -x "$bin" ]; then say "  SKIP -- not executable"; return 0; fi
  say "  marker: $("$REPO"/bin/.devbin/cmd/shared/artefact.lib >/dev/null 2>&1; \
    strings "$bin" 2>/dev/null | grep -o '\[intent-source-commit:[^]]*\]' | head -1)"

  local T; T="$(mktemp -d /tmp/sync-loss-XXXXXX)"
  (
    cd "$T" || exit 1
    # init takes the project name POSITIONALLY. The first version of this script
    # passed --name/--author and suppressed stderr, so both arms reported
    # "init FAILED" with the reason discarded -- the error you hide is the one
    # you need. Never 2>&1 >/dev/null a step whose failure you must diagnose.
    if ! "$bin" init "SyncLoss Probe" >/tmp/_init.log 2>&1; then
      say "  init FAILED:"; sed 's/^/    | /' /tmp/_init.log; exit 1
    fi

    "$bin" st new "Probe thread" >/dev/null 2>&1 || say "  (st new rc=$?)"
    local tid; tid="$("$bin" st list --status all 2>/dev/null | grep -oE 'ST[0-9]{4}' | head -1)"
    say "  thread created: ${tid:-<none>}"

    local i
    for i in 1 2 3; do
      "$bin" issues add "probe issue $i" >/tmp/_iss.log 2>&1 || { say "  issues add $i FAILED:"; sed 's/^/    | /' /tmp/_iss.log; }
    done

    local before_i before_t
    before_i="$("$bin" issues list 2>/dev/null | grep -cE '^[0-9]{4} ' || true)"
    before_t="$("$bin" st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}' || true)"
    say "  BEFORE  issues=$before_i  threads=$before_t"

    if [ "$before_i" -eq 0 ]; then
      say "  ABORT: the probe never created issues, so this arm cannot observe loss."
      say "  (an arm that cannot fail is not a test -- reporting rather than passing)"
      exit 2
    fi

    say "  running: $(basename "$bin") sync --to-store ${tid:-ST0001}"
    "$bin" sync --to-store "${tid:-ST0001}" 2>&1 | sed 's/^/    | /'

    local after_i after_t
    after_i="$("$bin" issues list 2>/dev/null | grep -cE '^[0-9]{4} ' || true)"
    after_t="$("$bin" st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}' || true)"
    say "  AFTER   issues=$after_i  threads=$after_t"

    if [ "$after_i" -lt "$before_i" ]; then
      say "  >>> REPRODUCED: a thread-scoped --to-store DESTROYED issues ($before_i -> $after_i)"
    else
      say "  >>> NOT reproduced: issues survived ($before_i -> $after_i)"
    fi
    if [ "$after_t" -lt "$before_t" ]; then
      say "  >>> threads ALSO lost ($before_t -> $after_t) -- does NOT match the live symptom"
    else
      say "  >>> threads intact ($before_t -> $after_t) -- MATCHES the live symptom"
    fi
  )
  local rc=$?
  rm -rf "$T"
  return $rc
}

say "sync_issue_loss.sh -- repo: $REPO"
arm "release build" "$REPO/native/rust/target/release/intent"
arm "debug build"   "$REPO/native/rust/target/debug/intent"
say ""
say "Both arms complete."
