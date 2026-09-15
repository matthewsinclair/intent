#!/usr/bin/env python3
"""Class (i) / issue 0124 conservation check, SOURCE-BASED.

Compares each v2-authored acceptance row against the v3 canon record it
became, and reports authored prose that reaches NO field of the canon record.

The comparison source is the pre-hop `acceptance.md` recovered from git
history. That is the ingest's OWN input path (legacy.rs:1273), so it exists
for any git-tracked estate whether or not a copy was preserved by hand.

CONSERVATION, NOT LENGTH. Issue 0126 established that a length delta nets a
loss against a gain and reports clean, so this asks a different question:
is every authored SEGMENT still findable somewhere in the record? That is
blind to ordering and to duplication -- 0126 needs its own instrument -- but
it cannot be fooled by a compensating gain.
"""

import json, re, sys, subprocess

def norm(s):
  return re.sub(r"\s+", " ", s or "").strip()

def parse_row(row):
  """`- AC-01.2 (non-test) TEXT -- key: value -- key: value` -> id, segments."""
  m = re.match(r"^-\s+((?:AC|AT)-[\w.]+)\s+(.*)$", row.strip())
  if not m:
    return None
  rid, rest = m.group(1), m.group(2)
  # a leading (non-test)/(test) marker becomes canon STRUCTURE (`kind`), not text
  rest = re.sub(r"^\((?:non-test|test)\)\s*", "", rest)
  parts = [p.strip() for p in rest.split(" -- ")]
  return rid, parts

def canon_strings(rec):
  """Every string the canon record holds, flattened."""
  out = []
  def walk(v):
    if isinstance(v, str):
      out.append(v)
    elif isinstance(v, dict):
      for x in v.values(): walk(x)
    elif isinstance(v, list):
      for x in v: walk(x)
  walk(rec)
  return norm(" || ".join(out))

def segment_present(seg, hay):
  """Is this authored segment accounted for in the canon record?"""
  s = norm(seg)
  if not s:
    return True
  # a `key: value` segment is conserved if its VALUE survives; the key becomes structure
  m = re.match(r"^([a-z][\w ]{0,20}):\s*(.*)$", s)
  body = norm(m.group(2)) if m else s
  if not body:
    return True
  if body in hay:
    return True
  # kind/state markers become structure, not text
  if body.lower() in ("yes", "no", "yes (computed)", "(non-test)", "(test)", "green", "red"):
    return True
  # `covers AC-01.1, AC-02.1` carries no colon and becomes a structural LINK, not prose
  if re.match(r"^covers\s+(?:(?:AC|AT)-[\w.]+)(?:\s*,\s*(?:AC|AT)-[\w.]+)*$", body, re.I):
    return True
  # `satisfied:`/`status:` carry a STATE TOKEN that canon normalises into state.is.
  # Bounded at 30 chars deliberately: a long value is prose wearing a structural
  # key, and must still surface rather than be suppressed by the key alone.
  if m and m.group(1).lower() in ("satisfied", "status") and len(body) <= 30:
    return True
  # tolerate the trailing-punctuation and backtick normalisation the render does
  b = body.strip(" .`\"'")
  return bool(b) and b in hay

def check(stid, v2_text, canon_path):
  d = json.load(open(canon_path))
  by_id = {c["id"]: c for c in (d.get("criteria") or [])}
  by_id.update({t["id"]: t for t in (d.get("tests") or []) if isinstance(t, dict) and "id" in t})
  rows = [l for l in v2_text.splitlines() if re.match(r"^-\s+(AC|AT)-", l.strip())]
  findings, matched = [], 0
  for row in rows:
    p = parse_row(row)
    if not p:
      continue
    rid, segs = p
    rec = by_id.get(rid)
    if rec is None:
      findings.append({"st": stid, "id": rid, "class": "ROW-ABSENT",
                       "detail": "authored row has no canon record"})
      continue
    matched += 1
    hay = canon_strings(rec)
    for seg in segs:
      if not segment_present(seg, hay):
        findings.append({"st": stid, "id": rid, "class": "SEGMENT-LOST",
                         "detail": norm(seg)[:220]})
  return len(rows), matched, findings

if __name__ == "__main__":
  sys.path.insert(0, "/private/tmp/claude-501/-Users-matts-Devel-prj-Intent/c3439256-4fb7-4499-8444-95d1f0d52bd7/scratchpad")
  from findv2 import last_v2
  tot_rows = tot_match = 0
  allf = []
  for stid in sys.argv[1:]:
    c, p, t = last_v2(stid)
    if c is None:
      print("%s  UNMEASURED -- no v2 source in history" % stid); continue
    n, m, f = check(stid, t, "intent/.canon/st/%s.json" % stid)
    tot_rows += n; tot_match += m; allf += f
    print("%s  blob=%s rows=%d matched=%d findings=%d" % (stid, c[:8], n, m, len(f)))
  print("\nTOTAL authored rows %d, matched to canon %d, findings %d" % (tot_rows, tot_match, len(allf)))
  for f in allf[:40]:
    print("  %s/%s  %s  %s" % (f["st"], f["id"], f["class"], f["detail"]))
  json.dump(allf, open("/private/tmp/claude-501/-Users-matts-Devel-prj-Intent/c3439256-4fb7-4499-8444-95d1f0d52bd7/scratchpad/findings.json", "w"), indent=1)
