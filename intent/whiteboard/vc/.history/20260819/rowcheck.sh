#!/bin/bash
# Canon (intent/.canon/st/<ID>.json) vs the REGENERATED view (intent/st/<ID>/acceptance.md),
# every AT row, on id + file + covers + status. Pure read: no sync, no cargo, no writes.
# The view renders `n-a` as `n/a`; that is a rendering translation, normalised here, not a divergence.
set -u
ID="$1"; OUT="$2"
CANON="intent/.canon/st/$ID.json"; VIEW="intent/st/$ID/acceptance.md"

jq -r '.tests[] | [.id, (.file // ""), (.covers|join(",")), .status] | @tsv' "$CANON" | sort > "$OUT/canon.$ID.tsv"

# Per-field extraction, so an arbitrarily long note cannot disturb the parse.
grep '^- AT-' "$VIEW" | while IFS= read -r line; do
  id=$(printf '%s' "$line" | sed -E 's/^- (AT-[0-9]+\.[0-9]+).*/\1/')
  file=$(printf '%s' "$line" | sed -nE 's/^- AT-[0-9]+\.[0-9]+ `([^`]*)`.*/\1/p')
  covers=$(printf '%s' "$line" | sed -nE 's/^.* -- covers (.*) -- status: .*/\1/p')
  status=$(printf '%s' "$line" | sed -nE 's/^.* -- status: ([a-z/-]+).*/\1/p' | sed 's|n/a|n-a|')
  printf '%s\t%s\t%s\t%s\n' "$id" "$file" "$covers" "$status"
done | sort > "$OUT/view.$ID.tsv"

echo "== $ID =="
echo "canon rows: $(grep -c . "$OUT/canon.$ID.tsv")   view rows: $(grep -c . "$OUT/view.$ID.tsv")"
echo "view rows with an EMPTY status (parse failures, must be 0): $(awk -F'\t' '$4==""' "$OUT/view.$ID.tsv" | grep -c . )"
echo "view rows with an EMPTY covers (parse failures, must be 0): $(awk -F'\t' '$3==""' "$OUT/view.$ID.tsv" | grep -c . )"
diff "$OUT/canon.$ID.tsv" "$OUT/view.$ID.tsv" > "$OUT/diff.$ID.txt"; rc=$?
echo "DIFF rc=$rc   differing lines: $(grep -c '^[<>]' "$OUT/diff.$ID.txt")"
head -30 "$OUT/diff.$ID.txt"
