#!/usr/bin/env python3
"""Find near-duplicate entries in a whiteboard board by lexical overlap.

WHY THIS EXISTS. A node's `wip.md` grows by appending, and the appending author
is the one person who cannot see that a new entry restates an old one -- reading
the board is what would notice, and the board has grown past the point where its
author reads it. Measured on ic's board at fold 26: entries 16 and 22 shared 21
distinct five-word phrases and were THE SAME INCIDENT written up an hour apart,
on a board its author had read a hundred times. Two further pairs the same way.
None of the three was available to attention; all three were available to a
count.

WHAT IT FINDS, AND WHAT IT CANNOT. It compares WORDS. Two entries describing one
class in different vocabulary will not be found, so a clean report is evidence
about phrasing and NOT evidence that the board has no duplicates -- read a zero
as "no lexical duplicates", never as "no duplicates". It also cannot tell a
duplicate from a deliberate restatement; it hands you pairs to judge, not a
verdict.

USAGE
  board_overlap.py <file.md> [--min N] [--gram N] [--from-heading TEXT]

  --min           report pairs sharing at least N distinct n-grams (default 5)
  --gram          n-gram width in words (default 5)
  --from-heading  only consider entries after the first heading containing TEXT,
                  eg --from-heading Watch-outs

An entry is a line beginning `- **` or `**` -- the board convention of one
bolded claim per bullet, one line per entry, never manually wrapped.
"""

import argparse
import collections
import re
import sys


def entries(path, from_heading):
  """Return [(index, text)] for the board's entry lines, 1-based."""
  out = []
  started = from_heading is None
  for raw in open(path, encoding="utf-8"):
    line = raw.rstrip("\n")
    if not started:
      if line.startswith("#") and from_heading.lower() in line.lower():
        started = True
      continue
    if line.startswith("- **") or line.startswith("**"):
      out.append((len(out) + 1, line))
  return out


def grams(text, width):
  words = re.sub(r"[^a-z0-9 ]", " ", text.lower()).split()
  return {" ".join(words[i:i + width]) for i in range(len(words) - width + 1)}


def main():
  ap = argparse.ArgumentParser(add_help=True)
  ap.add_argument("file")
  ap.add_argument("--min", type=int, default=5)
  ap.add_argument("--gram", type=int, default=5)
  ap.add_argument("--from-heading", default=None)
  args = ap.parse_args()

  rows = entries(args.file, args.from_heading)
  if not rows:
    print(f"board_overlap: no entries found in {args.file}"
          + (f" after a heading containing '{args.from_heading}'" if args.from_heading else ""))
    print("  An entry is a line starting `- **` or `**`. If this board uses another")
    print("  shape, this tool has not examined it -- that is an empty SUBJECT, not a clean report.")
    return 2

  index = collections.defaultdict(set)
  for num, text in rows:
    for g in grams(text, args.gram):
      index[g].add(num)

  shared = collections.Counter()
  for owners in index.values():
    if len(owners) > 1:
      ordered = sorted(owners)
      for i in range(len(ordered)):
        for j in range(i + 1, len(ordered)):
          shared[(ordered[i], ordered[j])] += 1

  hits = [(c, a, b) for (a, b), c in shared.items() if c >= args.min]
  hits.sort(reverse=True)

  by_num = dict(rows)
  print(f"board_overlap: {len(rows)} entries, {args.gram}-grams, reporting pairs sharing >= {args.min}")
  if not hits:
    print("  no lexical duplicate pairs at this threshold.")
    print("  THIS IS NOT 'NO DUPLICATES'. Two entries stating one class in different words")
    print("  are invisible to this tool; it compares phrasing, not meaning.")
    return 0

  for count, a, b in hits:
    print(f"\n  [{a}] <-> [{b}]  --  {count} shared {args.gram}-grams")
    for n in (a, b):
      print(f"    [{n}] {by_num[n][:170]}")
  print(f"\n  {len(hits)} pair(s) to judge. The tool does not know which of a pair to keep.")
  return 0


if __name__ == "__main__":
  sys.exit(main())
