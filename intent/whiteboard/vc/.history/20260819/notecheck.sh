#!/bin/bash
# The two fields the id/file/covers/status check did NOT cover: the AT `note`, and the AC set + `text`.
set -u
ID="$1"; OUT="$2"
CANON="intent/.canon/st/$ID.json"; VIEW="intent/st/$ID/acceptance.md"

# --- AT notes ---
jq -r '.tests[] | select((.note//"")!="") | [.id, .note] | @tsv' "$CANON" | sort > "$OUT/cnote.$ID.tsv"
grep '^- AT-' "$VIEW" | sed -nE 's/^- (AT-[0-9]+\.[0-9]+).* -- status: [a-z/-]+ -- (.*)$/\1\t\2/p' | sort > "$OUT/vnote.$ID.tsv"
echo "== $ID AT notes =="
echo "canon notes: $(grep -c . "$OUT/cnote.$ID.tsv")   view notes: $(grep -c . "$OUT/vnote.$ID.tsv")"
diff "$OUT/cnote.$ID.tsv" "$OUT/vnote.$ID.tsv" > "$OUT/dnote.$ID.txt"; echo "DIFF rc=$?  differing lines: $(grep -c '^[<>]' "$OUT/dnote.$ID.txt")"
head -4 "$OUT/dnote.$ID.txt" | cut -c1-200

# --- AC set + text ---
jq -r '.criteria[] | [.id, .text] | @tsv' "$CANON" | sort > "$OUT/cac.$ID.tsv"
grep '^- AC-' "$VIEW" | sed -E 's/^- (AC-[0-9]+\.[0-9]+) (.*) -- satisfied: .*$/\1\t\2/' | sort > "$OUT/vac.$ID.tsv"
echo "== $ID AC =="
echo "canon criteria: $(grep -c . "$OUT/cac.$ID.tsv")   view criteria: $(grep -c . "$OUT/vac.$ID.tsv")"
diff "$OUT/cac.$ID.tsv" "$OUT/vac.$ID.tsv" > "$OUT/dac.$ID.txt"; echo "DIFF rc=$?  differing lines: $(grep -c '^[<>]' "$OUT/dac.$ID.txt")"
head -4 "$OUT/dac.$ID.txt" | cut -c1-200
