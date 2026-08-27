#!/usr/bin/env bash
# bucket-prose-gate.sh <project-dir> -- is every file under a v2 status bucket
# either MODELLED by hop 2 or byte-carried into canon? Exit 9 if any is neither.
#
# **THE ONE CHECK THAT COVERS BOTH DOORS, WHICH IS WHY IT LIVES HERE AND NOT IN
# EITHER CALLER.** `reconvert.sh` compares an attachment count before and after,
# which needs a committed store to compare against -- so it is blind on a FRESH
# migration, where there is no before. Conflab is exactly that case and carries
# 1120 bucket files (dc, 2026-08-27). This asks the question that needs no
# history: the prose is either in canon or it is not.
#
# The defect it exists for: `collect_attachments` walks `thread_dir(id)` =
# `intent/st/<ID>`, which is FLAT, while a v2 tree keeps its files at
# `intent/st/<BUCKET>/<ID>/`. The walk enumerates nothing, so nothing is
# refused, no finding is filed and the hop exits 0. Measured on arca_cli: 23
# carried, then 0, with hop 2 `ok`, at-accounting PASS, verifier 0 and doctor
# IMPROVING 19 -> 2 over the loss.
#
# MODELLED means hop 2 read the file into the thread itself -- `info.md`,
# `acceptance.md`, `WP/NN/info.md`. Those are correctly never attachments, and
# counting them as a gap overstates the loss by ~40% (Laksa: 519 the naive way,
# 187 the honest way).
set -uo pipefail
P=${1:?project dir}; cd "$P" || exit 1
# **`intent/history/` IS SCANNED TOO, AND LEAVING IT OUT MADE THIS GATE RETURN A
# FALSE GREEN ON ITS OWN MOTIVATING CASE.** Once the carry is applied to the
# worktree the prose is no longer under `intent/st/<BUCKET>/` -- it is under
# `intent/history/`, and a scan that looks only at buckets reports "no bucket
# dirs, nothing this gate can be wrong about" over 187 un-carried files (Laksa,
# 2026-08-27). The question is WHERE THE PROSE IS relative to canon, and it has
# two possible homes, so both are asked. A file that has reached `history` is
# committed and recoverable -- it is not lost -- but it is outside the SSOT, and
# whether that is acceptable is a ruling rather than something this gate decides.
# It reports and refuses; the human disposes.
roots=()
for b in COMPLETED NOT-STARTED CANCELLED WIP; do
  [ -d "intent/st/$b" ] && roots+=("intent/st/$b")
done
[ -d intent/history ] && roots+=("intent/history")
if [ "${#roots[@]}" -eq 0 ]; then
  echo "bucket-prose: no bucket dirs and no intent/history -- there is no thread prose outside canon to be wrong about"
  exit 0
fi
[ -d intent/.canon/st ] || { echo "bucket-prose: prose present and NO CANON -- unmigrated, so this gate cannot answer yet"; exit 0; }
modelled=0; carried=0; gap=0; gapfile=$(mktemp)
while IFS= read -r -d '' f; do
  id=$(printf '%s' "$f" | grep -oE 'ST[0-9]{4}' | head -1)
  [ -n "$id" ] || continue
  # The prefix differs by home (`intent/st/<BUCKET>/<ID>/` vs
  # `intent/history/.../<ID>/`), so the relative path is cut at the id itself.
  rel=${f#*"$id"/}
  case "$rel" in
    info.md|acceptance.md|WP/[0-9][0-9]/info.md) modelled=$((modelled+1)); continue;;
  esac
  if [ -f "intent/.canon/st/$id.json" ] &&
     jq -e --arg r "$rel" '(.attachments//[])|any(.path==$r)' "intent/.canon/st/$id.json" > /dev/null 2>&1; then
    carried=$((carried+1))
  else
    gap=$((gap+1)); printf '%s/%s\n' "$id" "$rel" >> "$gapfile"
  fi
done < <(find "${roots[@]}" -type f -print0)
echo "bucket-prose: modelled by hop 2 $modelled; carried into canon $carried; NOT in canon $gap"
if [ "$gap" -gt 0 ]; then
  echo "bucket-prose: these would live ONLY on disk, outside the SSOT:"
  sed 's|^|    |' "$gapfile" | head -8
  [ "$gap" -gt 8 ] && echo "    ... and $((gap-8)) more"
  echo "bucket-prose: NOT DONE -- carry them with:"
  echo "  VC_INTENT=<intent> bash ~/Devel/prj/Intent/intent/whiteboard/vc/ingest-buckets.sh $P --commit"
  rm -f "$gapfile"; exit 9
fi
rm -f "$gapfile"
echo "bucket-prose: every non-modelled bucket file is byte-carried into canon"
exit 0
