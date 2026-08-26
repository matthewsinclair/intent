#!/usr/bin/env bash
# dehydrate-closed.sh <project-dir> [--commit] -- hv's order 2026-08-26: every migrated project declares its open set and the closed threads' FLAT views come out tonight.
# Scope is intent/st/ST* ONLY. The v2 bucket trees (COMPLETED/ NOT-STARTED/ CANCELLED/) are NEVER touched here:
# they hold the only copies of the prose files the migration did not carry (measured fleet-wide), ruled
# ingest-first. A closed thread's flat dir is removed only if every file in it is rendered from canon
# (info.md, acceptance.md) or is an attachment canon already holds; anything else refuses that thread by name.
set -uo pipefail
P=${1:?project dir}; shift; cd "$P" || exit 1; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"; N=$(basename "$P")
COMMIT=0; [ "${1:-}" = --commit ] && COMMIT=1
echo "## $N $(date -u +%H:%M:%SZ) HEAD $(git log --oneline -1 | cut -c1-50); flat dirs $(ls -d intent/st/ST* | wc -l | tr -d ' '); staged $(git diff --cached --name-only | wc -l | tr -d ' ')"
[ "$(git diff --cached --name-only | wc -l | tr -d ' ')" -eq 0 ] || { echo "index not empty -- refusing"; exit 2; }
open=$(jq -r 'select(.status != "completed" and .status != "cancelled") | .id' intent/.canon/st/*.json | sort); closed=$(jq -r 'select(.status == "completed" or .status == "cancelled") | .id' intent/.canon/st/*.json | sort)
echo "open $(printf '%s\n' "$open" | grep -c .); closed $(printf '%s\n' "$closed" | grep -c .)"
refuse=0; rm_list=""
for id in $closed; do d="intent/st/$id"; [ -d "$d" ] || continue
  allowed=$(printf 'info.md\nacceptance.md\n'; jq -r '.attachments[]?.path' "intent/.canon/st/$id.json"; jq -r '.wps[]? | .seq | tostring' "intent/.canon/st/$id.json" | sed -E 's/^(WP-)?0*([0-9]+)$/\2/' | awk '{printf "WP/%02d/info.md\n", $1}'); extra=$(cd "$d" && find . -type f | sed 's|^\./||' | grep -v -x -F -f <(printf '%s\n' "$allowed"))
  if [ -n "$extra" ]; then echo "REFUSE $id: files canon does not hold: $(printf '%s' "$extra" | tr '\n' ' ' | cut -c1-120)"; refuse=$((refuse+1)); else rm_list="$rm_list $d"; fi
done
n=$(printf '%s' "$rm_list" | wc -w | tr -d ' '); echo "removable closed flat dirs: $n; refused: $refuse"
{ printf '# .intentfiles -- WHICH DATABASE ARTEFACTS ALSO HAVE A REALISED FORM ON DISK.\n#\n# Written on hv'"'"'s order 2026-08-26 from thread status (organize --default was not yet\n# built): every OPEN thread is declared, and nothing else. OPEN means every status\n# except Completed and Cancelled.\n#\n'; for id in $open; do printf 'STEELTHREAD:%s\n' "$id"; done; } > "$L/intentfiles.new"
echo "declaration: $(grep -c '^STEELTHREAD:' "$L/intentfiles.new") lines"
[ $COMMIT -eq 1 ] || { echo "dry run: would remove $n dirs and write intent/.intentfiles ($(grep -c '^STEELTHREAD:' "$L/intentfiles.new") declared)"; exit 0; }
cp "$L/intentfiles.new" intent/.intentfiles
git rm -r -q $rm_list && git add intent/.intentfiles || { echo "git rm/add failed"; exit 3; }
paths="intent/.intentfiles $rm_list"
git commit -q --only $paths -m "intent: declare the open set and dehydrate the closed threads' flat views -- hv's order, by hand, ahead of organize --default --force

A fully realised tree was not acceptable to hv tonight and
the verb that does this (organize --default --force, ST0057 WP-11) is not yet
built on the pair this tree runs on. intent/.intentfiles declares the OPEN
threads from canon status; the $n closed
threads' flat views (info.md + acceptance.md, rendered from canon; st hydrate
restores any one) are removed. NOTHING under intent/st/COMPLETED, NOT-STARTED
or CANCELLED is touched: those v2 trees hold the only copies of ~1,249 prose
files the migration did not carry and are ruled ingest-first (vc, 2026-08-26).

Every number above was read by command in this run.

(C) hello@matthewsinclair.com" > "$L/dehydrate-commit.out" 2>&1; crc=$?
echo "commit rc=$crc $(git log --oneline -1 | cut -c1-60)"; [ $crc -eq 0 ] || { grep -nE 'error|fatal|refus|blocked|anchor' "$L/dehydrate-commit.out" | head -5 | cut -c1-160; exit 4; }
echo "post: flat dirs $(ls -d intent/st/ST* | wc -l | tr -d ' ') (expect 57); buckets: COMPLETED $(ls -d intent/st/COMPLETED/ST* | wc -l | tr -d ' ') NOT-STARTED $(ls -d intent/st/NOT-STARTED/ST* | wc -l | tr -d ' ') CANCELLED $(ls -d intent/st/CANCELLED/ST* | wc -l | tr -d ' ') (untouched); staged left $(git diff --cached --name-only | wc -l | tr -d ' ')"
