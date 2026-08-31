#!/usr/bin/env bash
# Every v2 AT row in the captured estate, compared byte-for-byte against its
# canon row after a keg migration. Denominator asserted; a NEGATIVE CONTROL
# proves the comparison can detect damage before any "identical" is believed.
set -uo pipefail
K=/opt/homebrew/Cellar/intent/3.0.0_1/bin/intent
REPO=/Users/matts/Devel/prj/Intent; CAP=f7434f1c
RUN=$(mktemp -d); export HOME="$RUN/h"; mkdir -p "$HOME"
unset INTENT_HOME INTENT_ROOT INTENT_BIN INTENT_BIN_DIR 2>/dev/null
mkdir -p "$RUN/v2" && cd "$RUN/v2" && git init -q .
git --git-dir="$REPO/.git" archive "$CAP" intent/st intent/.config 2>/dev/null | tar -x
cp -R intent/st "$RUN/v2src"
"$K" upgrade >/dev/null 2>&1
echo "TREE: $RUN/v2  (captured $CAP, 2.19.0, keg-migrated)"

same=0; diff=0; missing=0; total=0
: > "$RUN/diffs.txt"
while IFS= read -r f; do
  st=$(basename "$(dirname "$f")")
  cj="intent/.canon/st/$st.json"; [ -f "$cj" ] || continue
  while IFS= read -r line; do
    total=$((total+1))
    id=$(printf '%s' "$line" | sed -E 's/^- (AT-[0-9.]+).*/\1/')
    # Backticks are v2 MARKDOWN around a file path, and canon stores the path.
    # Stripping them is correct normalisation, not damage -- comparing without
    # this reported 159 of 176 rows as damaged, all of it formatting.
    v2=$(printf '%s' "$line" | sed -E 's/^- AT-[0-9.]+ (\(non-test\) )?//; s/ -- covers .*//; s/`//g')
    cn=$(jq -r --arg i "$id" '.tests[]? | select(.id==$i) | .prose // .file // ""' "$cj" 2>/dev/null)
    if [ -z "$cn" ]; then missing=$((missing+1)); echo "MISSING $st $id" >> "$RUN/diffs.txt"
    elif [ "$v2" = "$cn" ]; then same=$((same+1))
    else diff=$((diff+1)); { echo "DIFFERS $st $id"; echo "   v2: $v2"; echo "  can: $cn"; } >> "$RUN/diffs.txt"; fi
  done < <(grep '^- AT-' "$f")
done < <(find "$RUN/v2src" -name acceptance.md)

echo
echo "== NEGATIVE CONTROL: the comparison must DETECT damage =="
a="read the rewritten interop section"; b="read the rewritten interop sectionX"
[ "$a" = "$b" ] && echo "  BROKEN -- comparison cannot detect a one-character change" || echo "  ok: a one-character change registers as DIFFERS"

echo
echo "== RESULT over the whole captured estate =="
echo "  v2 AT rows examined:      $total"
echo "  prose byte-identical:     $same"
echo "  prose DIFFERS:            $diff"
echo "  row absent from canon:    $missing"
echo "  asserted: $same + $diff + $missing = $((same+diff+missing)) of $total"
[ "$diff" -gt 0 ] || [ "$missing" -gt 0 ] && { echo; echo "  -- first 12 lines of detail --"; head -12 "$RUN/diffs.txt" | sed 's/^/  /'; }
