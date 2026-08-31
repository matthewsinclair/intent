#!/usr/bin/env bash
# F1 -- a v2 estate CAPTURED from this repository's own history, migrated by the
# v3.0.0 keg, so that claims about v2-ingest behaviour can be DRIVEN.
#
# WHY CAPTURED AND NOT WRITTEN. The estate's own fixture README rejects a
# hand-authored seed as "one degree closer to the thing under test than a
# fixture may be". The ingest defects are about what the ingest does to AUTHORED
# PROSE, so prose written from the issue titles would be shaped by the very
# defect it is meant to detect. f7434f1c is a real 2.19.0 estate, 194 threads,
# 14 acceptance files, authored long before any of these issues were filed.
#
# WHICH BINARY, AND WHICH TREE. The keg reports 3.0.0 (80d8b2ca), the cut
# commit. THIS repository's store is schema 16 and the keg speaks 13, so it
# refuses this tree before reaching any behaviour -- every drive must happen in
# a tree the KEG initialised or migrated. Every claim names its tree, because
# "I ran the keg" does not say which store it met.
#
# 2.18.0 IS BELOW THE FLOOR. An earlier capture refused with "below the
# migration floor -- run install intent@2 && intent upgrade first". The capture
# commit must declare 2.19.0.
#
# WHAT IT CANNOT ANSWER. Two probes here do not discriminate and say so rather
# than reporting a number: ' -- ' occurs NATIVELY in v2 row syntax, so finding
# it in canon does not show the ingest inserted it; and 0071 is about the V2
# binary's own upgrade, which is not installed.

set -uo pipefail
K=/opt/homebrew/Cellar/intent/3.0.0_1/bin/intent
REPO=/Users/matts/Devel/prj/Intent; CAP=f7434f1c
RUN=$(mktemp -d); export HOME="$RUN/h"; mkdir -p "$HOME"
unset INTENT_HOME INTENT_ROOT INTENT_BIN INTENT_BIN_DIR 2>/dev/null
mkdir -p "$RUN/v2" && cd "$RUN/v2" && git init -q .
git --git-dir="$REPO/.git" archive "$CAP" intent/st intent/.config 2>/dev/null | tar -x
cp -R intent/st "$RUN/v2-source"          # the pre-ingest prose, for comparison
"$K" upgrade > "$RUN/up.out" 2>&1
echo "TREE: $RUN/v2   (captured $CAP, 2.19.0, migrated by the keg; store schema $(sqlite3 intent/.cache/intent.db 'pragma user_version' 2>/dev/null))"
echo

echo "== 0080 -- does migration leave Thread.slug empty? =="
"$K" st list --status all 2>&1 | head -4
echo "  canon slugs empty: $(jq -r '.slug // "NULL"' intent/.canon/st/*.json 2>/dev/null | grep -c '^NULL$') of $(find intent/.canon/st -name '*.json' 2>/dev/null | grep -c .)"
echo

echo "== 0103 -- wp list on a migrated (bucketed) thread whose WPs are in the store =="
t=$(jq -r 'select((.work_packages // []) | length > 0) | .id' intent/.canon/st/*.json 2>/dev/null | head -1)
echo "  thread with WPs in canon: ${t:-none found}"
if [ -n "$t" ]; then
  echo "  canon WP count: $(jq -r '(.work_packages // []) | length' "intent/.canon/st/$t.json")"
  echo "  -- wp list --"; "$K" wp list "$t" 2>&1 | head -5
fi
echo

echo "== 0140 -- an unsatisfied criterion carrying a note, writable only by migration =="
jq -r 'select(.criteria != null) | .criteria[]? | select((.state.is // "") == "unsatisfied") | "  \(.id)  note=\((.state.note // "none") | .[0:60])"' intent/.canon/st/*.json 2>/dev/null | head -4
echo "  unsatisfied criteria carrying a note: $(jq -r '.criteria[]? | select((.state.is//"")=="unsatisfied") | .state.note // empty' intent/.canon/st/*.json 2>/dev/null | grep -c . )"
echo

echo "== 0124/0126/0127/0129 -- did the ingest damage authored prose? =="
echo "  legacy.raw stubs in canon (0127's signature): $(grep -l 'legacy' intent/.canon/st/*.json 2>/dev/null | wc -l | tr -d ' ') file(s)"
echo "  canon rows whose text contains the ' -- ' delimiter (0129): $(jq -r '.tests[]?.prose // empty, .criteria[]?.text // empty' intent/.canon/st/*.json 2>/dev/null | grep -c ' -- ')"
echo "  -- a v2 source acceptance row against its canon row --"
src=$(find "$RUN/v2-source" -path '*ST0054/acceptance.md' 2>/dev/null | head -1)
[ -n "$src" ] && grep -m1 '^- AT-' "$src" | cut -c1-100 | sed 's/^/    v2:    /'
jq -r '.tests[0]? | "    canon: - \(.id) \(.prose // .file // "")"' intent/.canon/st/ST0054.json 2>/dev/null | cut -c1-100
