#!/usr/bin/env bash
# lamplight-triage.sh [--commit] -- set the four ex-_inbox threads to Triage via canon on a CURRENT pair (hv: "land as Triage").
# Deferred from the migration commit (4ffd0fe1f): `sync --to-store` was refused estate-wide by 23 unknown-file-shape
# files under the 2a classifier. Run only when bin/intent3's currency wrapper accepts the pair (it refuses otherwise).
# The committed canon already reads status "triage" for the four (the jq edit landed before the refused sync);
# this makes store and views agree with it, verifies, and commits ONLY the four threads' paths.
set -uo pipefail
P=~/Devel/prj/Lamplight; cd "$P" || exit 1; I=~/Devel/prj/Intent/bin/intent3; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
COMMIT=0; [ "${1:-}" = --commit ] && COMMIT=1
T="ST0339 ST0340 ST0341 ST0342"
echo "## Lamplight triage $(date -u +%H:%M:%SZ): HEAD $(git log --oneline -1 | cut -c1-50); pair $($I --version 2>&1 | head -1)"
$I --version >/dev/null 2>&1 || { echo "pair refuses (behind HEAD) -- not running"; exit 2; }
dirty_before=$(git status --porcelain | grep -vE '^.. (mix.lock|intent/whiteboard/|apps/|intent/.canon/issues/0005.json)' | wc -l | tr -d ' '); [ "$dirty_before" -eq 0 ] || { echo "tree dirty outside the known peer-owned paths ($dirty_before) -- refusing"; git status --porcelain | head -5; exit 3; }
for st in $T; do echo "$st canon: $(jq -r .status intent/.canon/st/$st.json)  view: $(grep -m1 '^status:' intent/st/$st/info.md)"; done
$I sync --to-store $T > "$L/triage-store.out" 2>&1; rc1=$?; $I sync --to-disk $T > "$L/triage-disk.out" 2>&1; rc2=$?
echo "sync --to-store rc=$rc1 :: $(tail -1 "$L/triage-store.out" | cut -c1-140)"; echo "sync --to-disk rc=$rc2 :: $(tail -1 "$L/triage-disk.out" | cut -c1-140)"
[ $rc1 -eq 0 ] && [ $rc2 -eq 0 ] || { echo "REFUSED -- the four stay Not Started; recorded"; grep -E 'refused|residue|remedy' "$L/triage-store.out" "$L/triage-disk.out" | head -6 | cut -c1-160; exit 4; }
ok=0; for st in $T; do grep -q '^status: Triage' intent/st/$st/info.md && ok=$((ok+1)); done; echo "views reading Triage: $ok of 4"
changed=$(git status --porcelain | grep -vE '^.. (mix.lock|intent/whiteboard/|apps/|intent/.canon/issues/0005.json)' | awk '{print $2}'); echo "changed paths: $(printf '%s\n' "$changed" | grep -c .)"; printf '%s\n' "$changed" | grep -vE "^intent/(st/(ST0339|ST0340|ST0341|ST0342)/|\.canon/st/(ST0339|ST0340|ST0341|ST0342)\.json|todo\.md|st/steel_threads\.md)" | sed 's/^/UNEXPECTED: /' | head -5
[ "$ok" -eq 4 ] || { echo "NOT DONE: views do not all read Triage"; exit 5; }
if [ $COMMIT -eq 1 ]; then paths=$(printf '%s\n' "$changed" | grep -E "^intent/(st/(ST0339|ST0340|ST0341|ST0342)/|\.canon/st/(ST0339|ST0340|ST0341|ST0342)\.json|todo\.md|st/steel_threads\.md)"); git add $paths && git commit -q --only $paths -m "intent: the four ex-_inbox threads are Triage (hv, 2026-08-26, via lamplight/vc)

ST0339-ST0342 came in flat from intent/st/_inbox on the migration commit as Not
Started because sync --to-store was refused estate-wide by unknown-file-shape
files under that pair's classifier; on $($I --version 2>&1 | head -1) the store
and the views now agree with the committed canon (status: triage).

(C) hello@matthewsinclair.com" > "$L/triage-commit.out" 2>&1 && echo "committed $(git log --oneline -1 | cut -c1-8); staged left: $(git diff --cached --name-only | wc -l | tr -d ' ')" || { echo "commit FAILED"; tail -3 "$L/triage-commit.out"; exit 6; }; fi
