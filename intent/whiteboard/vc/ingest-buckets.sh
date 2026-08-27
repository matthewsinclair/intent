#!/usr/bin/env bash
# ingest-buckets.sh <project-dir> [--commit] -- WP-13 by the tool's own verbs (hv: everything is 3.0.1; no bucket is
# deleted until every file under it is in the store). The migrator carried rows and left the prose of bucketed
# threads in the v2 status dirs (COMPLETED/ NOT-STARTED/ CANCELLED/ WIP/). Per file: `intent st attach <ID> <rel>
# --from <file>` (canon holds the bytes verbatim -- proven on a clone), then canon text == file bytes is checked for
# EVERY carried file, `organize --apply` realises open threads' attachments, and a bucket thread dir is pruned ONLY
# when every file under it is either a byte-verified attachment or one of the three files hop 2 modelled
# (info.md, acceptance.md, WP/NN/info.md). Anything the pair refuses (non-md on the 2a pair) or fails to verify
# blocks its dir's prune and is named. The set is DERIVED (find -type f -print0), never a list of names.
set -uo pipefail
P=${1:?project dir}; shift; cd "$P" || exit 1; N=$(basename "$P"); I=${VC_INTENT:?set VC_INTENT to the intent binary to use -- gate it with `int cli --version` first, which carries the pair-coherence guard. NOT a copy parked outside the checkout: the binary walks up from the exe for lib/templates/ (vc, 2026-08-27)}; L=${VC_SCRATCH:-/tmp/vc-scratch}; mkdir -p "$L"
COMMIT=0; [ "${1:-}" = --commit ] && COMMIT=1
echo "## $N $(date -u +%H:%M:%SZ) HEAD $(git log --oneline -1 | cut -c1-50); pair $($I --version 2>&1 | head -1 | cut -c1-40); dirty $(git status --porcelain | wc -l | tr -d ' ')"
[ "$(git status --porcelain | wc -l | tr -d ' ')" -eq 0 ] || { echo "$N: dirty -- refusing"; exit 2; }
# **TWO SOURCE ROOTS, ONE PRUNABLE.** Once the carry has run, the prose is no
# longer under `intent/st/<BUCKET>/` -- it is under `intent/history/<ID>/`, and a
# scan that looks only at buckets ingests NOTHING on an estate whose files have
# already moved (Laksa, 2026-08-27: 187 uncarried, invisible to this script).
# `intent/history/` is read as a RECOVERY PATH ONLY. It is hv's monthly done
# record (`YYYYMM-done.md`) and no thread directory belongs in it -- 415 were put
# there by vc generalising a one-project instruction, and hv reversed all of them
# on 2026-08-27. It is scanned so a tree still carrying that mistake can have its
# prose carried, and never pruned, because pruning would delete done-logs.
# ARRAYS, NOT A SPACE-JOINED STRING (IN-SH-CODE-001). The string form is built by
# concatenation and taken apart again by word-splitting at every use, which reads
# as safe only because no bucket path has ever carried a space -- and seven
# Lamplight filenames DO, which is how a positional parse silently dropped them
# from this same estate's rename census today.
prunable=()
for b in intent/st/COMPLETED intent/st/NOT-STARTED intent/st/CANCELLED intent/st/WIP; do
  [ -d "$b" ] && prunable+=("$b")
done
roots=("${prunable[@]}")
[ -d intent/history ] && roots+=("intent/history")
[ "${#roots[@]}" -gt 0 ] || { echo "$N: no bucket dirs and no intent/history -- nothing to ingest"; exit 0; }
$I doctor > "$L/ingest-$N.doctor" 2>&1; drc=$?; echo "doctor rc=$drc (a refusal here means the store verbs will refuse: $(grep -m1 -E 'refused|unknown-file-shape' "$L/ingest-$N.doctor" | cut -c1-100))"
total=0; modelled=0; att_ok=0; att_refused=0; ver_ok=0; ver_bad=0; : > "$L/ingest-$N.refused"; : > "$L/ingest-$N.verified"; : > "$L/ingest-$N.attached"
while IFS= read -r -d '' f; do total=$((total+1)); id=$(printf '%s' "$f" | sed -E 's|.*/(ST[0-9]+)/.*|\1|'); rel=${f#*/$id/}
  case "$rel" in info.md|acceptance.md) modelled=$((modelled+1)); continue;; WP/[0-9][0-9]/info.md) modelled=$((modelled+1)); continue;; esac
  [ -f "intent/.canon/st/$id.json" ] || { printf '%s -- no canon thread\n' "$f" >> "$L/ingest-$N.refused"; att_refused=$((att_refused+1)); continue; }
  if [ $COMMIT -eq 1 ]; then $I st attach "$id" "$rel" --from "$f" > "$L/ingest-$N.attach.out" 2>&1 || { printf '%s -- attach refused: %s\n' "$f" "$(head -1 "$L/ingest-$N.attach.out" | cut -c1-100)" >> "$L/ingest-$N.refused"; att_refused=$((att_refused+1)); continue; }; att_ok=$((att_ok+1)); printf '%s\n' "$f" >> "$L/ingest-$N.attached"
    jq -j --arg p "$rel" '.attachments[] | select(.path == $p) | .text' "intent/.canon/st/$id.json" > "$L/ingest-$N.canon.txt" 2>/dev/null; if cmp -s "$L/ingest-$N.canon.txt" "$f"; then ver_ok=$((ver_ok+1)); printf '%s\n' "$f" >> "$L/ingest-$N.verified"; else ver_bad=$((ver_bad+1)); printf '%s -- canon bytes differ after attach\n' "$f" >> "$L/ingest-$N.refused"; fi
  fi
done < <(find "${roots[@]}" -type f -print0)
# canon is written from the store by sync --to-disk (on some projects st attach updates canon directly, on others only the store);
# verification reads CANON, so it runs after this, over every carried file.
[ $COMMIT -eq 1 ] && { $I sync --to-disk > "$L/ingest-$N.todisk" 2>&1; echo "sync --to-disk rc=$? :: $(tail -1 "$L/ingest-$N.todisk" | cut -c1-100)"; ver_ok=0; ver_bad=0; : > "$L/ingest-$N.verified"; while IFS= read -r f; do id=$(printf '%s' "$f" | sed -E 's|.*/(ST[0-9]+)/.*|\1|'); rel=${f#*/$id/}; jq -j --arg p "$rel" '.attachments[] | select(.path == $p) | .text' "intent/.canon/st/$id.json" > "$L/ingest-$N.canon.txt" 2>/dev/null; if cmp -s "$L/ingest-$N.canon.txt" "$f"; then ver_ok=$((ver_ok+1)); printf '%s\n' "$f" >> "$L/ingest-$N.verified"; else ver_bad=$((ver_bad+1)); printf '%s -- canon bytes differ after sync\n' "$f" >> "$L/ingest-$N.refused"; fi; done < "$L/ingest-$N.attached"; echo "$N: after sync --to-disk: byte-verified in canon $ver_ok; failed $ver_bad"; }
echo "$N: files under buckets/history $total; modelled (hop 2 read them) $modelled; to carry $((total-modelled)); attached $att_ok; refused $att_refused; byte-verified $ver_ok; verify-failed $ver_bad"
[ -s "$L/ingest-$N.refused" ] && { echo "--- refused/failed (first 8):"; head -8 "$L/ingest-$N.refused" | cut -c1-150; }
[ $COMMIT -eq 1 ] || { echo "dry run: nothing written"; exit 0; }
$I organize --apply > "$L/ingest-$N.organize" 2>&1; orc=$?; echo "organize --apply rc=$orc :: $(grep -m1 -E '^organize' "$L/ingest-$N.organize" | cut -c1-140)"
pruned=0; kept=0
if [ "${#prunable[@]}" -gt 0 ]; then while IFS= read -r d; do bad=0
  while IFS= read -r -d '' f; do rel=${f#$d/}; case "$rel" in info.md|acceptance.md|WP/[0-9][0-9]/info.md) continue;; esac; grep -q -x -F "$f" "$L/ingest-$N.verified" || { bad=$((bad+1)); }; done < <(find "$d" -type f -print0)
  if [ $bad -eq 0 ]; then git rm -r -q "$d" && pruned=$((pruned+1)); else kept=$((kept+1)); printf '%s -- %s file(s) not verified\n' "$d" "$bad" >> "$L/ingest-$N.kept"; fi
  done < <(find "${prunable[@]}" -mindepth 1 -maxdepth 1 -type d -name 'ST*' | sort)
fi
for b in "${prunable[@]}"; do [ -z "$(find "$b" -mindepth 1 2>/dev/null)" ] && rmdir "$b" 2>/dev/null; done
echo "$N: bucket thread dirs pruned $pruned; kept (not fully verified) $kept$( [ -s "$L/ingest-$N.kept" ] && echo ": $(head -3 "$L/ingest-$N.kept" | cut -c1-80 | tr '\n' ';')")"
git add -A -- intent/.canon intent/st intent/todo.md 2>/dev/null; changed=$(git diff --cached --name-only | wc -l | tr -d ' ')
git commit -q --only -m "intent: carry the v2 buckets' prose into the store and prune what is proven carried -- WP-13 by st attach, per file, byte-verified

The migrator carried rows and left the prose of bucketed threads under
intent/st/{COMPLETED,NOT-STARTED,CANCELLED,WIP}. Here every file under
those dirs that hop 2 did not model is carried by \`intent st attach\` on
$($I --version 2>&1 | head -1 | cut -c1-60), canon is checked to hold its
bytes verbatim, open threads' attachments are realised by organize --apply,
and a bucket thread dir is pruned only when every file under it is either
byte-verified in canon or one of the three files hop 2 read. Counts read by
command in this run: $total files under buckets, $modelled modelled,
$att_ok attached, $ver_ok byte-verified, $att_refused refused (named in the
run log), $pruned dirs pruned, $kept kept.

(C) hello@matthewsinclair.com" -- intent/.canon intent/st intent/todo.md > "$L/ingest-$N.commit" 2>&1; crc=$?; echo "commit rc=$crc $(git log --oneline -1 | cut -c1-60); paths $changed; staged left $(git diff --cached --name-only | wc -l | tr -d ' ')"; [ $crc -eq 0 ] || grep -nE 'error|fatal|refus|blocked|anchor' "$L/ingest-$N.commit" | head -4 | cut -c1-160
