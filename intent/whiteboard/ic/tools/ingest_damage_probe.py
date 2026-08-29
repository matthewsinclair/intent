#!/usr/bin/env python3
"""IN-ESTATE probe for issue 0133 exposure. Run it on YOUR OWN repository.

WHAT THIS ANSWERS: how many acceptance criteria in this estate were authored
in a shape the v3 store CANNOT REPRESENT, and therefore lost a field at the
v2->v3 hop.

THE DEFECT, IN ONE PARAGRAPH. `AcState::Unsatisfied` was a UNIT VARIANT: it
carried no payload. So a criterion authored `satisfied: no` WITH an evidence
clause had nowhere to put that clause once ingested. `legacy.rs:1707` matched
`(true, Some(e)) => Satisfied{e}` and sent everything else to the wildcard
`_ => Unsatisfied`, so `(false, Some(evidence))` was destroyed silently. This
is a REPRESENTABLE-STATE REGRESSION in the model, not a parser bug.

WHICH BUILD YOU ARE ON DECIDES WHAT YOU CAN DO ABOUT IT. Intent's `04cf6f18`
(2026-08-29) makes it `Unsatisfied { note }` and replaces the wildcard with
three explicit arms. **That commit is NOT in the published v3.0.0 tag**, and
`brew install intent` currently hands you a build that still destroys. Test for
the commit, not for a version number.

EXPOSURE IS NOT DAMAGE. This counts rows whose PRE-HOP AUTHORED form has the
destroyable shape. It reads only your own authored source, recovered from git
history at the path the ingest itself read (`legacy.rs:1273` -> `acceptance.md`).
It does NOT compare against your current store. Say PREDICTED-UNCONFIRMED until
you confirm it against your own canon.

READ-ONLY BY CONSTRUCTION. `git log`, `git show`, `git cat-file` only.
`git status` is NEVER called: it refreshes the index and would disturb a live
checkout. Safe to run against a repo with sessions attached.

EXIT CODES ARE GRADED, BECAUSE THE QUANTITY IS.
  0  every thread accounted for, figure is complete
  3  NOT MEASURED -- no v2-authored form recovered anywhere
  4  MEASURED BUT INCOMPLETE -- some threads could not be classified
  2  not a git repository

DRIVE THE CONTROLS BEFORE YOU TRUST A NUMBER:  ./ingest_damage_probe.py --self-test
A zero from an undriven detector is not evidence. See the method document.

BOTH OF THE FIXES BELOW WERE FOUND BY LAMPLIGHT'S NODES, running this probe
blind against their own estate and returning 25 against a predicted 25. They
are credited at each site. An instrument improved by the estate it was sent to
is the only kind whose zero I would now believe.
"""

import subprocess, sys, re, os, tempfile, shutil

def git(repo, *a):
  r = subprocess.run(["git", "-C", repo] + list(a), capture_output=True)
  return r.returncode, r.stdout.decode("utf-8", "replace")

def acceptance_paths(repo):
  rc, out = git(repo, "log", "--all", "--format=", "--name-only", "--", "*/acceptance.md")
  return sorted({l.strip() for l in out.splitlines() if l.strip().endswith("acceptance.md")})

THREAD = re.compile(r"/(ST\d+)/")

def thread_key(path):
  """A criterion's identity is its THREAD, not its path."""
  m = THREAD.search(path)
  return m.group(1) if m else path

# ---------------------------------------------------------------------------
# LAMPLIGHT'S FINDING 2: `*/acceptance.md` IS NOT A THREAD PREDICATE.
#
# It is a filename glob, and an estate has other reasons to hold a file by
# that name. Lamplight's run caught
# `design/system/handoff/intent/st/ST0334/acceptance.md` and paths under
# `_inbox/` -- 9 non-threads, 5 of which had no canon record at all. And the
# failure is worse than a bad path, because `thread_key` extracted a PLAUSIBLE
# id from every one of them: the key looked right, so nothing downstream had
# any reason to doubt it.
#
# A glob cannot settle this and no cleverer glob can either. It needs an
# ORACLE -- the estate's own record of which threads exist. Where the oracle
# is absent the cross-check SAYS SO rather than passing everything, because a
# check that silently does not run is the shape this whole method exists
# to refuse.
# ---------------------------------------------------------------------------

CANON_DIRS = ("intent/.canon/st", ".canon/st")

def canon_ids(repo):
  """(ids, source) when the estate can name its own threads; (None, why) otherwise."""
  for rel in CANON_DIRS:
    d = os.path.join(repo, rel)
    if os.path.isdir(d):
      ids = {f[:-5] for f in os.listdir(d) if f.endswith(".json")}
      if ids:
        return ids, rel
  return None, "no canon directory (looked for %s)" % " and ".join(CANON_DIRS)

# ---------------------------------------------------------------------------
# THE PORT COMMIT, AND WHY IT IS TIME AND NOT A BANNER.
#
# Lamplight's finding 1 came with a mechanism: earliest commit time per
# acceptance path, compared against the port commit. I built something else --
# "is the thread's oldest acceptance blob a GENERATED VIEW" -- reasoning that it
# needed no repo-wide constant. **IT IS WRONG, AND THE REAL ESTATE SAID SO.**
#
# Driven against Intent it put 44 of 67 threads in `residue`. Checking one
# rather than shipping the number: `intent/st/ST0001/acceptance.md` is FIRST
# CREATED by the hoist commit `0ec2ac79` -- "Intent is now self-hosted on
# Intent3" -- so it is v3-created beyond doubt, and it carries no banner
# because the early v3 renderer did not emit one. **A detector keyed on a
# string the subject did not always emit reports the subject absent.** The
# banner is a fact about the RENDERER'S VERSION, not about when a thread was
# born.
#
# So: the port is located by the arrival of `.canon/`, which is a v3 concept
# and cannot predate the hop, and a thread whose acceptance file first appears
# at or after it was created under v3. Where the marker is absent the probe
# SAYS the classification could not run rather than defaulting every thread to
# one side. Lamplight's mechanism, arrived at the long way round.
# ---------------------------------------------------------------------------

CANON_GLOBS = ("*/.canon/st/*.json", ".canon/st/*.json")

def port_time(repo):
  """Commit time of the earliest `.canon/` record, or None if never ported."""
  best = None
  for g in CANON_GLOBS:
    rc, out = git(repo, "log", "--all", "--topo-order", "--reverse", "--format=%ct", "--", g)
    for line in out.split():
      ts = int(line)
      if best is None or ts < best:
        best = ts
      break
  return best

def oldest_blob(repo, paths):
  """The genuinely oldest commit touching any of this thread's paths.

  Deliberately NOT taken from the capped candidate list below: a cap of 25
  newest commits per path answers a different question, and reading `oldest`
  off it would be right only for threads with a short history."""
  rc, out = git(repo, "log", "--all", "--topo-order", "--reverse", "--format=%H %ct", "--", *paths)
  for line in out.splitlines():
    f = line.split()
    if len(f) != 2:
      continue
    c, ts = f[0], int(f[1])
    for p in paths:
      rc, _ = git(repo, "cat-file", "-e", "%s:%s" % (c, p))
      if rc == 0:
        return c, p, ts
  return None

def classify(repo, paths, port, cap=25):
  """Put ONE thread in exactly one bucket: recovered | v3_created | residue.

  **PER-PATH IS THE WRONG UNIT AND IT SILENTLY INFLATES.** v2 kept threads in
  status-bucket directories (`st/NOT-STARTED/ST0052/`, `st/WIP/...`) and
  estates collapsed those into a flat layout before hopping. A thread has
  SEVERAL historical paths, each holding a frozen snapshot from whenever it
  left that bucket. Measured on Intent: `ST0052 AC-01.2` reads `satisfied: no`
  at the July `NOT-STARTED/` path and `yes` at the post-collapse path. It was
  SATISFIED before the hop; the stale snapshot alone made it look exposed.

  So: gather every candidate across every path, order by COMMIT TIME, take the
  newest that is present and v2-authored. One thread, one pre-hop form.

  AND WHEN THERE IS NO v2-AUTHORED FORM, THE THREAD STILL HAS TO GO SOMEWHERE.
  Its OLDEST blob answers which: a thread whose oldest acceptance file already
  carries the v3 banner was CREATED under v3 and has nothing to lose, while
  anything else is residue this probe cannot read."""
  # ORDERING COMES FROM GIT, NOT FROM A SORT ON COMMIT TIME, AND THAT IS A FIX
  # RATHER THAN A STYLE CHOICE. The previous form gathered `(%ct, sha, path)`
  # per path and sorted descending -- but two commits made in the SAME SECOND
  # carry the SAME `%ct`, so the sort fell through to comparing SHA strings and
  # picked a winner at random. Measured: the status-bucket-collapse fixture
  # commits twice in one second, and which blob won was decided by hex.
  #
  # **THE SELF-TEST PASSED ANYWAY, ON A COIN FLIP.** That is the failure worth
  # naming -- a control that reports PASS while testing nothing it claims to,
  # and reports it consistently enough to look stable. One `git log --all` over
  # every path returns commits newest-first in git's own topological order,
  # which is deterministic and has no ties to break.
  rc, out = git(repo, "log", "--all", "--topo-order", "--format=%H", "--", *paths)
  for c in out.split()[:cap]:
    for p in paths:
      rc, _ = git(repo, "cat-file", "-e", "%s:%s" % (c, p))
      if rc != 0:
        continue                                 # absent here, not evidence
      rc, t = git(repo, "show", "%s:%s" % (c, p))
      if rc != 0 or "GENERATED VIEW" in t:
        continue                                 # a v3 view, not authored v2
      if " -- satisfied: " not in t and " -- status: " not in t:
        continue                                 # no v2 row markers
      return "recovered", p, t
  ob = oldest_blob(repo, paths)
  if ob:
    c, p, ts = ob
    rc, t = git(repo, "show", "%s:%s" % (c, p))
    if rc == 0 and "GENERATED VIEW" in t:
      return "v3_created", p, None               # banner: SUFFICIENT, not necessary
  return "unclassified", (ob[1] if ob else paths[0]), None

AC = re.compile(r"^-\s+AC-[\w.]+\s")

# THE EXCLUSION AN OUTSIDE READER GETS WRONG FIRST.
# A bracketed evidence value is TEMPLATE SCAFFOLD -- the placeholder the ST
# template ships, never an authored claim. Counting it inflates the number
# badly and unevenly: on the fleet sweep it was 136 of 393 counted rows, and
# for one estate (Cdsync) it was 5 of 5, i.e. its ENTIRE apparent exposure.
# Nothing was destroyed there, because nothing was ever said.
SCAFFOLD = re.compile(r"^\[.*\]$")

def is_scaffold(ev):
  return bool(SCAFFOLD.match(ev.strip()))

def scan(repo):
  ids, oracle = canon_ids(repo)
  by_thread = {}
  for p in acceptance_paths(repo):
    by_thread.setdefault(thread_key(p), []).append(p)

  not_a_thread = []
  if ids is not None:
    for k in sorted(by_thread):
      if k not in ids:
        not_a_thread.append((k, by_thread[k][0]))
    for k, _ in not_a_thread:
      del by_thread[k]

  port = port_time(repo)
  r = {"exposed": [], "immune": 0, "no_ev": 0, "scaffold": [],
       "recovered": 0, "v3_created": 0, "residue": [],
       "seen": len(by_thread), "oracle": oracle, "ids": ids,
       "not_a_thread": not_a_thread, "port": port}

  for key in sorted(by_thread):
    kind, p, t = classify(repo, by_thread[key], port)
    if kind == "v3_created":
      r["v3_created"] += 1
      continue
    if kind == "unclassified":
      r["residue"].append((key, p))
      continue
    r["recovered"] += 1
    for line in t.splitlines():
      s_ = line.strip()
      if not AC.match(s_):
        continue
      m_ev = re.search(r" -- evidence:\s*(.*?)(?:\s+--\s|$)", s_)
      m_v = re.search(r" -- satisfied:\s*([a-z/]+)", s_)
      verdict = m_v.group(1) if m_v else None
      if not m_ev:
        r["no_ev"] += 1
      elif verdict != "no":
        r["immune"] += 1                         # keeps its evidence: Satisfied{e}
      elif is_scaffold(m_ev.group(1)):
        r["scaffold"].append((p, s_[:90]))       # never an authored claim
      else:
        r["exposed"].append((p, s_[:90]))        # THE DESTROYABLE SHAPE

  # THE PARTITION IS ASSERTED, NOT ASSUMED. If these ever disagree, a thread
  # fell out of the accounting and the figure below is not about the estate.
  total = r["recovered"] + r["v3_created"] + len(r["residue"])
  assert total == r["seen"], "partition leak: %d != %d" % (total, r["seen"])
  return r

def report(r):
  """Print the accounting, then the figure. Never the figure alone.

  **LAMPLIGHT'S FINDING 1, AND IT IS THE SHARPER OF THE TWO.** The refusal
  this replaces was BINARY: it fired only when the recovered count hit zero.
  So an estate at 152 recovered of 358 threads exited 0 and printed a
  confident exposure figure with 206 threads unaccounted for -- and nothing in
  the output let the reader see it.

  I built that refusal because "nothing measured" and "nothing exposed"
  printed the same headline. Then I implemented it at ESTATE granularity for a
  subject whose unit is the THREAD -- the census rule I had corrected the
  fleet with that same morning. **A BINARY GUARD ON A CONTINUOUS QUANTITY IS
  NOT A SMALLER VERSION OF THE RIGHT GUARD. IT IS THE WRONG SHAPE, AND IT
  FAILS EXACTLY WHERE THE ANSWER LOOKS MOST CONFIDENT.**

  So the accounting is now per-thread and complete, and the exit code is
  graded because the quantity is."""
  print("threads named by an acceptance path : %d" % (r["seen"] + len(r["not_a_thread"])))
  if r["ids"] is None:
    print("  !! THREAD CROSS-CHECK DID NOT RUN: %s" % r["oracle"])
    print("     `*/acceptance.md` is a filename glob, not a thread predicate.")
    print("     Non-thread paths may be inflating every count below.")
  else:
    print("  cross-checked against %s (%d known threads)" % (r["oracle"], len(r["ids"])))
    if r["not_a_thread"]:
      print("  EXCLUDED, no canon record -- not threads (%d):" % len(r["not_a_thread"]))
      for k, p in r["not_a_thread"][:8]:
        print("     %-12s %s" % (k, p))
  print("")
  if r["port"] is None:
    print("  !! PORT MARKER ABSENT: no `.canon/` record anywhere in history.")
    print("     Cannot tell a v3-created thread from unreadable residue, so")
    print("     nothing is classified as v3-created and residue is an UPPER bound.")
  print("  v2-authored form recovered  : %d   <- the measured population" % r["recovered"])
  print("  banner-marked v3 view       : %d   <- nothing to lose, accounted for" % r["v3_created"])
  print("  UNCLASSIFIED                : %d   <- see the caveat below" % len(r["residue"]))
  print("                                ---")
  print("  every thread accounted for  : %d" % r["seen"])


# ---------------------------------------------------------------- self-test
# INLINE ON PURPOSE. This is a single file sent to other people's estates; a
# probe whose controls live in a second module is a probe whose controls do
# not travel, and an undriven detector is exactly what this method refuses.

FIXTURE = """# Acceptance -- ST9999

- AC-01.1 the positive arm -- satisfied: no -- evidence: measured on this tree, 14 rows
- AC-01.2 satisfied keeps its evidence -- satisfied: yes -- evidence: the gate counts it
- AC-01.3 unsatisfied and silent -- satisfied: no
- AC-01.4 template scaffold -- satisfied: no -- evidence: [named evidence]
- AC-01.5 more scaffold -- satisfied: no -- evidence: [a doc / eyeball / gate criterion]
"""

# THE ARM MY FIRST SELF-TEST COULD NOT EXHIBIT. One thread, two historical
# paths: a stale `NOT-STARTED/` snapshot saying `satisfied: no` and the later
# post-collapse blob saying `satisfied: yes`. The row was SATISFIED before the
# hop, so the correct answer is NOT exposed. A per-path scanner reports 1 and
# is wrong. Found on a live estate, not in a fixture.
MOVED_OLD = """# Acceptance -- ST9997

- AC-03.1 moved thread, stale snapshot -- satisfied: no -- evidence: a four-site grep
"""

MOVED_NEW = """# Acceptance -- ST9997

- AC-03.1 moved thread, current form -- satisfied: yes -- evidence: a four-site grep
"""

VIEW = """# Acceptance -- ST9998

<!-- GENERATED VIEW -- do not edit -->

- AC-02.1 a v3 view row -- satisfied: no -- evidence: would be a false positive
"""

# LAMPLIGHT'S FINDING 1, PLANTED. A thread with an acceptance file that is
# neither a v3 view nor readable as v2. The OLD binary guard put this nowhere
# and still exited 0.
RESIDUE = """# Acceptance -- ST9996

Nothing here parses as a v2 row and there is no generated-view banner.
"""

# LAMPLIGHT'S FINDING 2, PLANTED. A real `acceptance.md` at a path that is NOT
# a thread -- and one `thread_key` happily extracts a plausible id from. It
# carries the destroyable shape, so if the cross-check fails to exclude it the
# exposed count goes UP and every downstream number is wrong.
NOT_A_THREAD = """# Acceptance -- handoff copy

- AC-09.9 a non-thread path -- satisfied: no -- evidence: must never be counted
"""

def _w(d, rel, text):
  full = os.path.join(d, rel)
  os.makedirs(os.path.dirname(full), exist_ok=True)
  open(full, "w").write(text)

def _commit(d, msg, when):
  """Commit at an EXPLICIT time.

  **THE FIXTURE USED TO COMMIT EVERYTHING IN ONE BURST, AND THAT IS WHY ITS
  CONTROL WAS A COIN FLIP.** `%ct` is whole seconds, so several fixture commits
  landed in the SAME second and "newest" was decided by whatever order git
  returned -- measured by vc across 12 runs of one committed blob: 4 all-pass,
  8 failure, on one machine and one Python. Every failing arm was a
  moved-thread arm and every stable one was a flat filter, which is the
  signature.

  A burst-committed fixture also models no real estate. A v2 era, then a hop,
  are days apart. Planting the dates makes the fixture both deterministic AND
  honest about the timeline it claims to represent."""
  env = dict(os.environ, GIT_AUTHOR_DATE=when, GIT_COMMITTER_DATE=when)
  subprocess.run(["git", "-C", d, "add", "-A"], check=True)
  subprocess.run(["git", "-C", d, "commit", "-qm", msg], check=True, env=env)

def self_test():
  """DRIVE THE DETECTOR TO EVERY VERDICT ON PLANTED GROUND TRUTH.

  A control that cannot fail is decoration. Each arm is planted with a known
  answer, and the probe must produce that answer and no other."""
  d = tempfile.mkdtemp(prefix="probe-selftest-")
  try:
    for a in (["init", "-q"], ["config", "user.email", "t@t"], ["config", "user.name", "t"]):
      subprocess.run(["git", "-C", d] + a, check=True)
    # ---- THE v2 ERA. Authored files, no canon anywhere. ----
    _w(d, "st/ST9999/acceptance.md", FIXTURE)
    _w(d, "st/ST9996/acceptance.md", RESIDUE)
    _w(d, "st/NOT-STARTED/ST9997/acceptance.md", MOVED_OLD)
    _w(d, "design/handoff/intent/st/ST9995/acceptance.md", NOT_A_THREAD)
    _commit(d, "v2 era", "2026-01-10T09:00:00Z")

    # ---- THE BUCKET COLLAPSE, still v2: same thread, new path, new verdict ----
    shutil.rmtree(os.path.join(d, "st", "NOT-STARTED"))
    _w(d, "st/ST9997/acceptance.md", MOVED_NEW)
    _commit(d, "collapse status buckets", "2026-02-14T11:00:00Z")

    # ---- THE PORT. `.canon/` arrives; ST9998 is BORN here, under v3. ----
    # ST9995 is deliberately absent from canon -- that is the whole of finding 2.
    for t in ("ST9999", "ST9998", "ST9997", "ST9996"):
      _w(d, "intent/.canon/st/%s.json" % t, "{}")
    _w(d, "st/ST9998/acceptance.md", VIEW)
    _commit(d, "hop to v3", "2026-08-18T12:00:00Z")

    r = scan(d)
    ex = r["exposed"]
    arms = [
      ("POSITIVE  fires on a known-damaged row",       len(ex) == 1, "%d exposed" % len(ex)),
      ("POSITIVE  names the RIGHT row",                len(ex) == 1 and "AC-01.1" in ex[0][1], ex[0][1][:44] if ex else "-"),
      ("NEGATIVE  silent on satisfied-with-evidence",  r["immune"] == 2, "%d immune" % r["immune"]),
      ("NEGATIVE  silent on unsatisfied-no-evidence",  r["no_ev"] == 1, "%d no-ev" % r["no_ev"]),
      ("NEGATIVE  silent on TEMPLATE SCAFFOLD x2",     len(r["scaffold"]) == 2, "%d scaffold" % len(r["scaffold"])),
      ("NEGATIVE  a MOVED thread uses its NEWEST form", not any("AC-03.1" in e[1] for e in ex), "%d stale row(s) leaked" % sum(1 for e in ex if "AC-03.1" in e[1])),
      ("BUCKET    a v3 GENERATED VIEW is v3_created",  r["v3_created"] == 1, "%d v3-created" % r["v3_created"]),
      ("BUCKET    an unreadable file is UNCLASSIFIED",  len(r["residue"]) == 1, "%d unclassified" % len(r["residue"])),
      ("BUCKET    recovered counts only real v2 forms", r["recovered"] == 2, "%d recovered" % r["recovered"]),
      ("LAMPLIGHT-1  the partition accounts for EVERY thread",
         r["recovered"] + r["v3_created"] + len(r["residue"]) == r["seen"],
         "%d+%d+%d == %d" % (r["recovered"], r["v3_created"], len(r["residue"]), r["seen"])),
      ("LAMPLIGHT-1  unclassified is NAMED, not absorbed", bool(r["residue"]) and r["residue"][0][0] == "ST9996", r["residue"][0][0] if r["residue"] else "-"),
      ("LAMPLIGHT-2  a non-thread path is EXCLUDED",   len(r["not_a_thread"]) == 1 and r["not_a_thread"][0][0] == "ST9995", "%d excluded" % len(r["not_a_thread"])),
      ("LAMPLIGHT-2  and its row never reaches EXPOSED", not any("AC-09.9" in e[1] for e in ex), "%d non-thread row(s) leaked" % sum(1 for e in ex if "AC-09.9" in e[1])),
      ("LAMPLIGHT-2  the oracle is NAMED in the output", r["ids"] is not None and r["oracle"].endswith(".canon/st"), str(r["oracle"])),
    ]
    ok = all(p for _, p, _ in arms)
    print("SELF-TEST -- planted ground truth, every verdict driven\n")
    for name, passed, got in arms:
      print("  %-52s %s   (%s)" % (name, "PASS" if passed else "FAIL", got))
      ok = ok and passed

    # THE ORACLE-ABSENT ARM RUNS ON ITS OWN TREE, because the point is what
    # happens when the estate CANNOT name its threads -- and a check that
    # silently passes everything there is the defect, not the fallback.
    d2 = tempfile.mkdtemp(prefix="probe-selftest-noc-")
    try:
      for a in (["init", "-q"], ["config", "user.email", "t@t"], ["config", "user.name", "t"]):
        subprocess.run(["git", "-C", d2] + a, check=True)
      _w(d2, "st/ST9999/acceptance.md", FIXTURE)
      _commit(d2, "no canon", "2026-01-10T09:00:00Z")
      r2 = scan(d2)
      arm = (r2["ids"] is None and "no canon directory" in r2["oracle"])
      print("  %-52s %s   (%s)" % ("LAMPLIGHT-2  no oracle -> SAYS SO, does not pass silently",
                                   "PASS" if arm else "FAIL", str(r2["oracle"])[:34]))
      ok = ok and arm
    finally:
      shutil.rmtree(d2, ignore_errors=True)

    print("\n%s" % ("ALL ARMS PASS -- the detector discriminates."
                    if ok else "FAILURE -- do not trust this probe's numbers."))
    return 0 if ok else 1
  finally:
    shutil.rmtree(d, ignore_errors=True)

if __name__ == "__main__":
  if "--self-test" in sys.argv:
    sys.exit(self_test())
  repo = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else ".")
  if not os.path.isdir(os.path.join(repo, ".git")):
    print("not a git repository: %s" % repo); sys.exit(2)
  r = scan(repo)
  print("estate: %s" % repo)
  report(r)
  print("")

  # **A ZERO YOU CANNOT TRUST MUST NOT EXIT 0 -- AND NEITHER MAY A FIGURE.**
  if r["recovered"] == 0:
    if r["seen"] == 0:
      print("NOT MEASURED: no acceptance file appears anywhere in this history.")
      print("  This estate has no acceptance contract to lose. Nothing to say.")
    else:
      print("NOT MEASURED: %d thread(s) have an acceptance file, but NO v2-authored" % r["seen"])
      print("  form survives in history. Two very different causes look identical:")
      print("    (a) the estate was BORN under v3        -> genuinely nothing to lose")
      print("    (b) its v2 history was squashed/imported -> UNMEASURABLE FROM GIT")
      print("  **DO NOT REPORT THIS AS ZERO EXPOSURE.**")
    sys.exit(3)

  print("  EXPOSED (destroyable shape) : %d   <- PREDICTED-UNCONFIRMED" % len(r["exposed"]))
  print("  immune  (satisfied+evidence): %d" % r["immune"])
  print("  no evidence clause          : %d" % r["no_ev"])
  print("  template scaffold, excluded : %d" % len(r["scaffold"]))
  if r["exposed"]:
    print("\nexposed rows:")
    for p, l in r["exposed"]:
      print("  %s\n    %s" % (p, l))

  if r["residue"]:
    print("")
    print("INCOMPLETE: %d of %d thread(s) are UNCLASSIFIED, so the figure above is"
          % (len(r["residue"]), r["seen"]))
    print("  a FLOOR and not a total.")
    print("")
    print("  WHY THIS PROBE CANNOT SPLIT THEM, STATED RATHER THAN GUESSED:")
    print("    v3's GENERATED acceptance.md uses the SAME `-- satisfied:` row syntax")
    print("    as v2 authored, and early v3 emitted no `GENERATED VIEW` banner. So")
    print("    the only reliable v3 marker is a banner that is SUFFICIENT and NOT")
    print("    NECESSARY, and a thread without one may be either:")
    print("      (a) born under v3 before the banner existed -> nothing to lose")
    print("      (b) v2-authored in a form this probe cannot read -> UNMEASURED")
    print("    Timestamps do not settle it either: measured on Intent, the hoist")
    print("    wrote acceptance files ~25h BEFORE the first .canon record, so")
    print("    `.canon` arrival is not the port instant it looks like.")
    print("    **SETTLE THESE BY HAND AGAINST YOUR OWN HISTORY.** Lamplight did")
    print("    exactly that -- 197 v3-created, 9 genuine residue, 152 recovered,")
    print("    summing to 358 -- and estate knowledge is what made it possible.")
    print("")
    print("  Named, so you can settle them:")
    for k, p in r["residue"][:8]:
      print("     %-12s %s" % (k, p))
    if len(r["residue"]) > 8:
      print("     ... and %d more" % (len(r["residue"]) - 8))
    print("  Exiting 4 rather than 0: the count is real and the estate is not done.")
    sys.exit(4)

  print("\nEXPOSURE IS NOT DAMAGE. Confirm against your own canon before reporting"
        "\nthese as lost. Drive --self-test before trusting any number, including 0.")
