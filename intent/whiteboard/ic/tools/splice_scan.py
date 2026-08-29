#!/usr/bin/env python3
"""Detect issue 0126 (SPLICE) from a v3 canon store ALONE, with no v2 source.

The splice signature is a SELF-CONTAINED property of the stored field: one
authored span reaching canon twice by two routes, so it appears TWICE inside
a single field. That needs no comparison source, which is why this is the
only one of the four ingest-damage classes that can be swept on all 17
estates rather than only where a v2 bucket survives.

WHAT THIS DOES NOT DETECT, stated because a survey that reports one number
sends someone to fix one class believing they fixed all four:
  0124  prose discarded BETWEEN two fields -- needs the v2 source. UNMEASURABLE here.
  0127  the note FIELD absent -- an ABSENT field, not a short one. Not a dup.
  0129  an authored full stop rewritten to ' -- ' -- no repetition, invisible here.
  0126's HEAD LOSS and TAIL TRUNCATION -- the issue records both alongside the
        duplication. This finds the DUPLICATION only. A row can be spliced,
        lose its head, and be found here solely by the middle.
"""

import json, os, sys, glob, argparse, re

def looks_authored(span):
  """Reject a repeat that is not authored PROSE.

  Added because the planted control fired on a 120-char run of '-': two
  non-overlapping 60-char halves of one run are a repeated substring by the
  letter and are not a splice. Markdown rules and table delimiters are the
  same shape and occur for real. A true splice is authored prose, so require
  prose-like diversity rather than trusting the length alone.
  """
  words = [w for w in re.split(r"\W+", span) if len(w) >= 2]
  return len(set(span)) >= 8 and len(words) >= 4

def repeated_span(s, minlen):
  """Longest span occurring >=2x, non-overlapping. None if none >= minlen."""
  n = len(s)
  if n < minlen * 2:
    return None
  best = None
  seen = {}
  for i in range(n - minlen + 1):
    w = s[i:i + minlen]
    if w in seen:
      j = seen[w]
      if i - j < minlen:      # overlapping -- a run of one char, not a splice
        continue
      k = minlen
      while i + k < n and j + k < i and s[j + k] == s[i + k]:
        k += 1
      cand = s[j:j + k]
      if looks_authored(cand) and (best is None or k > best[0]):
        best = (k, j, i, cand)
    else:
      seen[w] = i
  return best

def prose_fields(path):
  """Yield (locator, fieldname, text) for every authored prose field."""
  try:
    d = json.load(open(path))
  except Exception:
    return
  if not isinstance(d, dict):
    return
  ident = d.get("id") or d.get("number") or os.path.basename(path)
  for c in d.get("criteria") or []:
    for f in ("text", "note", "raw"):
      v = c.get(f)
      if isinstance(v, str) and v:
        yield ("%s/%s" % (ident, c.get("id")), f, v)
  for w in d.get("wps") or []:
    for c in w.get("criteria") or []:
      for f in ("text", "note", "raw"):
        v = c.get(f)
        if isinstance(v, str) and v:
          yield ("%s/WP%s/%s" % (ident, w.get("seq"), c.get("id")), f, v)

def scan(root, minlen):
  hits, total = [], 0
  for path in sorted(glob.glob(os.path.join(root, "**", "*.json"), recursive=True)):
    for loc, field, text in prose_fields(path):
      total += 1
      r = repeated_span(text, minlen)
      if r:
        hits.append({"loc": loc, "field": field, "len": len(text),
                     "duplen": r[0], "at": (r[1], r[2]), "span": r[3]})
  return total, hits

if __name__ == "__main__":
  ap = argparse.ArgumentParser()
  ap.add_argument("roots", nargs="+")
  ap.add_argument("--minlen", type=int, default=40)
  ap.add_argument("--json", action="store_true")
  a = ap.parse_args()
  out = {}
  for root in a.roots:
    total, hits = scan(root, a.minlen)
    out[root] = {"fields_scanned": total, "hits": hits}
    if not a.json:
      print("%s: %d prose fields scanned, %d carrying a repeated span >=%d chars"
            % (root, total, len(hits), a.minlen))
      for h in hits:
        print("  %-28s %-5s field=%d dup=%d at %s" % (h["loc"], h["field"], h["len"], h["duplen"], h["at"]))
        print("      %r" % h["span"][:160])
  if a.json:
    print(json.dumps(out, indent=1))
