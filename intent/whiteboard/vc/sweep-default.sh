#!/usr/bin/env bash
# sweep-default.sh [--commit] <project-dir>... -- the TOOL-DRIVEN half of the fleet sweep (hv, 2026-08-26):
#   bare `intent organize --default` writes intent/.intentfiles declaring the open set when ABSENT and touches
#   NOTHING else (AC-11.1); when PRESENT it changes not one byte and exits 0 (AC-11.2). `--default --force`
#   (the destructive half) is hv's own keystrokes at a tty and is NEVER run from here.
# Predicate per project, hv's Devbin canary shape: exactly ONE path changes and it is intent/.intentfiles.
# Refuses (never commits) on: a dirty tree before the run, rc != 0, any other path changing, a present file
# changing at all. Every number is read by command in the same run; nothing is transcribed.
set -uo pipefail
I=${VC_INTENT:-$HOME/Devel/prj/Intent/bin/intent3}; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
COMMIT=0; [ "${1:-}" = --commit ] && { COMMIT=1; shift; }
pair=$($I --version 2>&1 | head -1); echo "## sweep --default on [$pair] at $(date -u +%H:%M:%SZ); commit=$COMMIT"
fail=0
for P in "$@"; do N=$(basename "$P"); cd "$P" || { echo "$N: no such dir -- SKIP"; fail=1; continue; }
  v=$(jq -r '.intent_version // empty' intent/.config/config.json 2>/dev/null); case "$v" in 3.*) ;; *) echo "$N: not on v3 ($v) -- SKIP"; fail=1; continue;; esac
  dirty_before=$(git status --porcelain | wc -l | tr -d ' '); [ "$dirty_before" -eq 0 ] || { echo "$N: dirty before the run ($dirty_before paths) -- REFUSED (a sweep never commits into someone's work)"; fail=1; continue; }
  present_before=0; [ -f intent/.intentfiles ] && { present_before=1; cp intent/.intentfiles "$L/sweep-$N.before"; }
  $I organize --default > "$L/sweep-$N.out" 2>&1; rc=$?
  changed=$(git status --porcelain | awk '{print $2}'); n=$(printf '%s\n' "$changed" | grep -c .)
  if [ $rc -ne 0 ]; then echo "$N: organize --default rc=$rc -- REFUSED: $(tail -2 "$L/sweep-$N.out" | tr '\n' ' ' | cut -c1-160)"; fail=1; continue; fi
  if [ $present_before -eq 1 ]; then
    if [ "$n" -eq 0 ] && cmp -s "$L/sweep-$N.before" intent/.intentfiles; then echo "$N: present, unchanged, rc 0 -- OK (AC-11.2; declares $(grep -c '^STEELTHREAD:' intent/.intentfiles) thread(s))"; else echo "$N: present file CHANGED or other paths moved ($n) -- REFUSED"; git status --porcelain | head -3; fail=1; fi; continue; fi
  if [ "$n" -eq 1 ] && [ "$changed" = "intent/.intentfiles" ]; then
    decl=$(grep -c '^STEELTHREAD:' intent/.intentfiles); echo "$N: written, exactly one path, declares $decl open thread(s) -- $(grep -m1 -E '^(ok|wrote|created)' "$L/sweep-$N.out" | cut -c1-100)"
    if [ $COMMIT -eq 1 ]; then git add intent/.intentfiles && git commit -q --only intent/.intentfiles -m "intent: declare the open set -- organize --default on $pair

Bare --default writes intent/.intentfiles declaring exactly the threads whose
status is not Completed and not Cancelled ($decl here) and changes nothing else
(AC-11.1); the destructive half (--default --force, after a confirm) is hv's own
keystrokes and was not run. Exactly one path changed, read by command.

(C) hello@matthewsinclair.com" > "$L/sweep-$N.commit" 2>&1 && echo "$N: committed $(git log --oneline -1 | cut -c1-8); left dirty: $(git status --porcelain | wc -l | tr -d ' ')" || { echo "$N: commit FAILED"; tail -3 "$L/sweep-$N.commit"; fail=1; }; fi
  else echo "$N: predicate FAILED -- $n path(s) changed: $(printf '%s' "$changed" | tr '\n' ' ' | cut -c1-160) -- REFUSED, left as is for a human"; fail=1; fi
done
[ $fail -eq 0 ] && echo "SWEEP VERDICT: every project named met its predicate" || { echo "SWEEP VERDICT: at least one project REFUSED (above)"; exit 1; }
