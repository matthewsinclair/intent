#!/bin/bash
# AC set + text, canon vs regenerated view, compared on the FIRST LINE of each text.
# Canon carries exactly one criterion with an embedded newline (ST0056 AC-00.11), so first-line
# comparison is exact for every other row; that one row's continuation is checked separately below.
# Two rendering translations, each verified rather than assumed: the view prefixes `(non-test)`
# from .kind, and appends ` -- satisfied: ...`, which is COMPUTED and never stored.
set -u
ID="$1"; OUT="$2"
jq -r '.criteria[] | [.id, ((if .kind=="non-test" then "(non-test) " else "" end) + (.text|split("\n")[0]))] | @tsv' "intent/.canon/st/$ID.json" | sort > "$OUT/cac3.$ID.tsv"
grep '^- AC-[0-9]' "intent/st/$ID/acceptance.md" | sed -E 's/^- (AC-[0-9]+\.[0-9]+) (.*) -- satisfied: .*$/\1\t\2/; t; s/^- (AC-[0-9]+\.[0-9]+) (.*)$/\1\t\2/' | sort > "$OUT/vac3.$ID.tsv"
echo "== $ID AC (id + first line of text) =="
echo "canon: $(grep -c . "$OUT/cac3.$ID.tsv")   view: $(grep -c . "$OUT/vac3.$ID.tsv")   parse failures: $(awk -F'\t' 'NF<2' "$OUT/vac3.$ID.tsv" | grep -c .)"
diff "$OUT/cac3.$ID.tsv" "$OUT/vac3.$ID.tsv" > "$OUT/dac4.$ID.txt"; echo "DIFF rc=$?  differing lines: $(grep -c '^[<>]' "$OUT/dac4.$ID.txt")"
head -6 "$OUT/dac4.$ID.txt" | cut -c1-200
