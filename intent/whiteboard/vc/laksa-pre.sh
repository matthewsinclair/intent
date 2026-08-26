#!/usr/bin/env bash
# Laksa ST0008: v2 status SUPERSEDED is not in the v2 vocabulary; the first landing recorded it as cancelled (the body says "Superseded by ST0012"); make the same mapping explicit in the source.
f=intent/st/COMPLETED/ST0008/info.md; grep -q '^status: SUPERSEDED' "$f" || { echo "ST0008 status line not as expected: $(grep -m1 '^status:' "$f")"; exit 1; }
sed -i '' 's/^status: SUPERSEDED$/status: Cancelled/' "$f" && echo "ST0008: status SUPERSEDED -> Cancelled (verblock keeps the word; canon at the first landing already read cancelled)"
