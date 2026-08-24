#!/usr/bin/env bash
# upgrade_issue_loss.sh -- does `intent upgrade` destroy ISSUES?
#
# THE SYMMETRIC HALF. sync_issue_loss.sh tested vc's suspect and did not
# reproduce. Testing only the other node's candidate and stopping is how an
# investigation reaches a conclusion its author preferred. This tests MINE.
set -uo pipefail
REPO="${1:?usage: upgrade_issue_loss.sh <repo>}"
say() { printf '%s\n' "$*"; }
arm() {
  local label="$1" bin="$2"
  say ""; say "=================================================="; say "ARM: $label"
  [ -x "$bin" ] || { say "  SKIP"; return 0; }
  say "  marker: $(strings "$bin" 2>/dev/null | grep -o '\[intent-source-commit:[^]]*\]' | head -1)"
  local T; T="$(mktemp -d /tmp/upg-loss-XXXXXX)"
  (
    cd "$T" || exit 1
    "$bin" init "Upgrade Probe" >/tmp/_u1.log 2>&1 || { say "  init FAILED:"; sed 's/^/    | /' /tmp/_u1.log; exit 1; }
    "$bin" st new "Probe thread" >/dev/null 2>&1
    local i; for i in 1 2 3 4 5; do "$bin" issues add "probe issue $i" >/dev/null 2>&1; done
    local bi bt
    bi="$("$bin" issues list 2>/dev/null | grep -cE '^[0-9]{4} ' || true)"
    bt="$("$bin" st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}' || true)"
    say "  BEFORE  issues=$bi  threads=$bt"
    [ "$bi" -gt 0 ] || { say "  ABORT: probe made no issues -- an arm that cannot fail is not a test"; exit 2; }
    say "  running: intent upgrade"
    "$bin" upgrade 2>&1 | sed 's/^/    | /'
    local ai at
    ai="$("$bin" issues list 2>/dev/null | grep -cE '^[0-9]{4} ' || true)"
    at="$("$bin" st list --status all 2>/dev/null | grep -cE '^ST[0-9]{4}' || true)"
    say "  AFTER   issues=$ai  threads=$at"
    if [ "$ai" -lt "$bi" ]; then say "  >>> REPRODUCED: upgrade DESTROYED issues ($bi -> $ai)"
    else say "  >>> NOT reproduced: issues survived ($bi -> $ai)"; fi
    if [ "$at" -lt "$bt" ]; then say "  >>> threads ALSO lost -- does NOT match the live symptom"
    else say "  >>> threads intact -- MATCHES the live symptom"; fi
  )
  local rc=$?; rm -rf "$T"; return $rc
}
arm "release build" "$REPO/native/rust/target/release/intent"
arm "debug build"   "$REPO/native/rust/target/debug/intent"
