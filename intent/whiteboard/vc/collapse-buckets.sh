#!/usr/bin/env bash
# HALTED 2026-08-26 (ic): the delete arm removes old acceptance.md whose authored
# preamble canon never ingested. DO NOT --apply. Rewrite under: delete only when
# old is a line-subset of new, else keep as <name>.v2.<ext>; caller gates on rc.
echo "HALTED -- see header"; exit 9
# collapse-buckets.sh -- close the migration's documented bucket hole BY MOVE, never by prune.
#
# migrate.rs:47 names it: v3 `upgrade` writes fresh canon and regenerated views
# at the flat `intent/st/<ID>/` and "does not relocate a thread out of v2's
# COMPLETED/ / CANCELLED/ / NOT-STARTED/ buckets". So after hop 2 every bucketed
# thread has TWO homes, and the old one holds authored prose (design.md,
# impl.md, tasks.md, anything hand-written) that canon does NOT carry -- ic
# measured 68K of prose per thread against 16K of canon on Riffle. Pruning the
# bucket is therefore data loss. Intent closed the same hole on itself at
# 1af21f4e with 192 `git mv` renames and deletions ONLY of files the flat dir
# had regenerated; this script is that commit, mechanised, with the loss check
# that commit's author did by eye.
#
# RULE, per file under the bucket: a flat counterpart at the same relative path
# EXISTS -> the old copy is a superseded view, `git rm` it; NO counterpart ->
# it is authored, `git mv` it into the flat dir. Then the bucket dir is empty
# and goes. A bucketed thread with NO flat dir at all means hop 2 did not
# migrate it: STOP, that is a different defect and moving it would hide it.
#
# Usage: collapse-buckets.sh <project> [--apply]   (dry run without --apply)
set -uo pipefail
P="${1:?project dir}"; MODE="${2:-dry}"
cd "$P" || exit 1
[ -d intent/st ] || { echo "no intent/st"; exit 1; }
moved=0; removed=0; stop=0
for bucket in COMPLETED CANCELLED NOT-STARTED; do
  [ -d "intent/st/$bucket" ] || continue
  for old in intent/st/"$bucket"/ST*/; do
    [ -d "$old" ] || continue
    id=$(basename "$old"); new="intent/st/$id"
    if [ ! -d "$new" ]; then
      echo "STOP: $bucket/$id has NO flat counterpart -- hop 2 did not migrate it; not touching it"; stop=1; continue
    fi
    # walk every file under the old thread dir
    while IFS= read -r f; do
      rel="${f#"$old"}"
      if [ -e "$new/$rel" ]; then
        [ "$MODE" = "--apply" ] && git rm -q -- "$f"
        removed=$((removed+1)); [ "$MODE" = "--apply" ] || echo "rm   (superseded view) $f"
      else
        [ "$MODE" = "--apply" ] && { mkdir -p "$(dirname "$new/$rel")"; git mv -k -- "$f" "$new/$rel" 2>/dev/null || mv -- "$f" "$new/$rel"; git add -- "$new/$rel"; }
        moved=$((moved+1)); [ "$MODE" = "--apply" ] || echo "mv   (authored)        $f -> $new/$rel"
      fi
    done < <(find "$old" -type f | sort)
    if [ "$MODE" = "--apply" ]; then
      # loss check BEFORE the directory goes: nothing may remain only-in-old
      left=$(find "$old" -type f 2>/dev/null | wc -l | tr -d ' ')
      [ "$left" -eq 0 ] && rm -rf "$old" || { echo "STOP: $left file(s) still under $old after the walk"; stop=1; }
    fi
  done
  [ "$MODE" = "--apply" ] && [ -d "intent/st/$bucket" ] && [ -z "$(ls -A "intent/st/$bucket")" ] && rmdir "intent/st/$bucket"
done
echo "collapse ($MODE): moved=$moved (authored, into flat) removed=$removed (superseded views) stop=$stop"
[ "$stop" -eq 0 ]
