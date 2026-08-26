#!/usr/bin/env bash
# ingest-flat.sh <project-dir> [--commit] -- Prolix's shape: a FLAT (bucketless) v2 tree whose prose sits in the closed
# threads' flat dirs but was never collected into canon. Attach each such file in place (`st attach <ID> <rel> --from`),
# verify canon holds the bytes, then dehydrate-closed.sh can remove the closed flat dirs (every file canon-held).
set -uo pipefail
P=${1:?project dir}; shift; cd "$P" || exit 1; N=$(basename "$P"); I=${VC_INTENT:-$HOME/.local/bin/intent}; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
COMMIT=0; [ "${1:-}" = --commit ] && COMMIT=1
echo "## $N $(date -u +%H:%M:%SZ) HEAD $(git log --oneline -1 | cut -c1-50); dirty $(git status --porcelain | wc -l | tr -d ' ')"; [ "$(git status --porcelain | wc -l | tr -d ' ')" -eq 0 ] || { echo "dirty -- refusing"; exit 2; }
n=0; ok=0; bad=0; : > "$L/flat-$N.refused"
for d in intent/st/ST*/; do id=$(basename "$d"); [ -f "intent/.canon/st/$id.json" ] || continue
  while IFS= read -r -d '' f; do rel=${f#$d}; case "$rel" in info.md|acceptance.md|WP/[0-9][0-9]/info.md) continue;; esac
    jq -e --arg p "$rel" '.attachments[] | select(.path == $p)' "intent/.canon/st/$id.json" >/dev/null 2>&1 && continue
    n=$((n+1)); [ $COMMIT -eq 1 ] || continue
    $I st attach "$id" "$rel" --from "$f" > "$L/flat-$N.out" 2>&1 || { printf '%s -- %s\n' "$f" "$(head -1 "$L/flat-$N.out" | cut -c1-100)" >> "$L/flat-$N.refused"; bad=$((bad+1)); continue; }
    jq -j --arg p "$rel" '.attachments[] | select(.path == $p) | .text' "intent/.canon/st/$id.json" > "$L/flat-$N.canon" 2>/dev/null; cmp -s "$L/flat-$N.canon" "$f" && ok=$((ok+1)) || { printf '%s -- canon bytes differ\n' "$f" >> "$L/flat-$N.refused"; bad=$((bad+1)); }
  done < <(find "$d" -type f -print0)
done
echo "$N: uncollected files in flat dirs $n; attached+verified $ok; refused/failed $bad"; [ -s "$L/flat-$N.refused" ] && head -5 "$L/flat-$N.refused" | cut -c1-140
[ $COMMIT -eq 1 ] || { echo "dry run"; exit 0; }
git add -A -- intent/.canon && git commit -q --only -m "intent: carry the uncollected prose of a flat v2 tree into the store -- st attach in place, per file, byte-verified

$n files under closed threads' flat dirs were never collected into canon; each is attached on $($I --version 2>&1 | head -1 | cut -c1-40) and canon is checked to hold its bytes verbatim ($ok verified, $bad refused and named in the run log).

(C) hello@matthewsinclair.com" -- intent/.canon > "$L/flat-$N.commit" 2>&1; echo "commit rc=$? $(git log --oneline -1 | cut -c1-60); staged left $(git diff --cached --name-only | wc -l | tr -d ' ')"
