#!/usr/bin/env bash
# ingest-flat.sh <project-dir> [--commit] -- Prolix's shape: a FLAT (bucketless) v2 tree whose prose sits in the closed
# threads' flat dirs but was never collected into canon. Attach each such file in place (`st attach <ID> <rel> --from`),
# then `sync --to-disk` (canon is written from the store; on some projects attach updates canon directly, on others only
# the store), then verify canon holds every attached file's bytes, then commit canon + the re-rendered views.
# dehydrate-closed.sh can then remove the non-WIP flat dirs (every file canon-held).
set -uo pipefail
P=${1:?project dir}; shift; cd "$P" || exit 1; N=$(basename "$P"); I=${VC_INTENT:-$HOME/.local/bin/intent}; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
COMMIT=0; [ "${1:-}" = --commit ] && COMMIT=1
echo "## $N $(date -u +%H:%M:%SZ) HEAD $(git log --oneline -1 | cut -c1-50); dirty $(git status --porcelain | wc -l | tr -d ' ')"; [ "$(git status --porcelain | wc -l | tr -d ' ')" -eq 0 ] || { echo "dirty -- refusing"; exit 2; }
n=0; att=0; bad=0; : > "$L/flat-$N.refused"; : > "$L/flat-$N.attached"
for d in intent/st/ST*/; do id=$(basename "$d"); [ -f "intent/.canon/st/$id.json" ] || continue
  while IFS= read -r -d '' f; do rel=${f#$d}; case "$rel" in info.md|acceptance.md|WP/[0-9][0-9]/info.md) continue;; esac
    jq -e --arg p "$rel" '.attachments[]? | select(.path == $p)' "intent/.canon/st/$id.json" >/dev/null 2>&1 && continue
    n=$((n+1)); [ $COMMIT -eq 1 ] || continue
    if $I st attach "$id" "$rel" --from "$f" > "$L/flat-$N.out" 2>&1; then att=$((att+1)); printf '%s\t%s\t%s\n' "$id" "$rel" "$f" >> "$L/flat-$N.attached"; else printf '%s -- %s\n' "$f" "$(head -1 "$L/flat-$N.out" | cut -c1-100)" >> "$L/flat-$N.refused"; bad=$((bad+1)); fi
  done < <(find "$d" -type f -print0)
done
echo "$N: uncollected files in flat dirs $n; attached $att; refused $bad"; [ -s "$L/flat-$N.refused" ] && head -5 "$L/flat-$N.refused" | cut -c1-140
[ $COMMIT -eq 1 ] || { echo "dry run"; exit 0; }
$I sync --to-disk > "$L/flat-$N.todisk" 2>&1; echo "sync --to-disk rc=$? :: $(tail -1 "$L/flat-$N.todisk" | cut -c1-100)"
ok=0; vbad=0; while IFS=$'\t' read -r id rel f; do jq -j --arg p "$rel" '.attachments[] | select(.path == $p) | .text' "intent/.canon/st/$id.json" > "$L/flat-$N.canon" 2>/dev/null; if cmp -s "$L/flat-$N.canon" "$f"; then ok=$((ok+1)); else vbad=$((vbad+1)); printf '%s -- canon bytes differ after sync\n' "$f" >> "$L/flat-$N.refused"; fi; done < "$L/flat-$N.attached"
echo "$N: byte-verified in canon $ok; failed $vbad; views re-rendered by sync: $(git status --porcelain -- intent/st | wc -l | tr -d ' ') path(s)"
git add -A -- intent/.canon intent/st intent/todo.md && git commit -q --only -m "intent: carry the uncollected prose of a flat v2 tree into the store -- st attach in place, per file, byte-verified

$n files under threads' flat dirs were never collected into canon (the collector
walks a flat path the migrator never wrote for them); each is attached on
$($I --version 2>&1 | head -1 | cut -c1-40), canon is written from the store, and
canon is checked to hold its bytes verbatim ($ok verified, $vbad failed, $bad refused,
all named in the run log). Views re-rendered by the same pair ride in this commit.

(C) hello@matthewsinclair.com" -- intent/.canon intent/st intent/todo.md > "$L/flat-$N.commit" 2>&1; echo "commit rc=$? $(git log --oneline -1 | cut -c1-60); staged left $(git diff --cached --name-only | wc -l | tr -d ' ')"
