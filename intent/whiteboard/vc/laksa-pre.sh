#!/usr/bin/env bash
# Laksa ST0008: v2 status SUPERSEDED is not in the v2 vocabulary; the first landing recorded it as cancelled (the body says "Superseded by ST0012"); make the same mapping explicit in the source.
f=intent/st/COMPLETED/ST0008/info.md; grep -q '^status: SUPERSEDED' "$f" || { echo "ST0008 status line not as expected: $(grep -m1 '^status:' "$f")"; exit 1; }
sed -i '' 's/^status: SUPERSEDED$/status: Cancelled/' "$f" && echo "ST0008: status SUPERSEDED -> Cancelled (verblock keeps the word; canon at the first landing already read cancelled)"
# ST0089 AC-06.10: the row attributes a 2026-08-05 decision to hv ("hv's ruling") with no anchor, and Laksa's
# own hv-attribution guard (bin/hooks/guards, added after the first landing) refuses any commit that ADDS the
# line -- which a re-rendered view does. The guard's sanctioned admission is the word UNANCHORED on the line.
# Appended to the row's PROSE as a parenthetical (the row carries no satisfied: field), text otherwise verbatim.
f=intent/st/COMPLETED/ST0089/acceptance.md
grep -q 'AC-06.10 .*UNANCHORED' "$f" || perl -i -pe 'if (/^- AC-06\.10 /) { chomp; $_ .= " (hv attribution UNANCHORED: spoken in chat 2026-08-05, no inbox entry)\n" }' "$f"
echo "ST0089 AC-06.10: UNANCHORED admission on the row ($(grep -c 'AC-06.10 .*UNANCHORED' "$f") line)"
