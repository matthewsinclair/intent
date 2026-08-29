#!/usr/bin/env python3
"""IN-ESTATE probe for issue 0133 exposure. Run it on YOUR OWN repository.

WHAT THIS ANSWERS: how many acceptance criteria in this estate were authored
in a shape the v3 store CANNOT REPRESENT, and therefore lost a field at the
v2->v3 hop.

THE DEFECT, IN ONE PARAGRAPH. `AcState::Unsatisfied` is a UNIT VARIANT: it
carries no payload. So a criterion authored `satisfied: no` WITH an evidence
clause has nowhere to put that clause once ingested. `legacy.rs:1707` matches
`(true, Some(e)) => Satisfied{e}` and sends everything else to the wildcard
`_ => Unsatisfied`, so `(false, Some(evidence))` is destroyed silently. This
is a REPRESENTABLE-STATE REGRESSION in the model, not a parser bug -- v2 could
author the state and v3 cannot hold it.

EXPOSURE IS NOT DAMAGE. This counts rows whose PRE-HOP AUTHORED form has the
destroyable shape. It reads only your own authored source, recovered from git
history at the path the ingest itself read (`legacy.rs:1273` -> `acceptance.md`).
It does NOT compare against your current store, so a row counted here is one
that WOULD have lost evidence. Say PREDICTED-UNCONFIRMED until you confirm it
against your own canon. Confirming is a second, per-estate step.

READ-ONLY BY CONSTRUCTION. `git log`, `git show`, `git cat-file` only.
`git status` is NEVER called: it refreshes the index and would disturb a live
checkout. Safe to run against a repo with sessions attached.

DRIVE THE CONTROLS BEFORE YOU TRUST A NUMBER:  ./ingest_damage_probe.py --self-test
A zero from an undriven detector is not evidence. See the method document.
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

def newest_v2_blob(repo, paths, cap=25):
  """Newest v2-AUTHORED blob for one thread, across EVERY path it ever had.

  **PER-PATH IS THE WRONG UNIT AND IT SILENTLY INFLATES.** v2 kept threads in
  status-bucket directories (`st/NOT-STARTED/ST0052/`, `st/WIP/...`,
  `st/COMPLETED/...`) and estates collapsed those into a flat layout before
  hopping. A thread therefore has SEVERAL historical paths, each holding a
  frozen snapshot from whenever it left that bucket. Scanning per-path counts
  one criterion once per bucket it ever sat in, each at a stale verdict.

  Measured on Intent: `ST0052 AC-01.2` reads `satisfied: no` at the July
  `NOT-STARTED/` path and `satisfied: yes` at the post-collapse path. It was
  SATISFIED before the hop; the stale snapshot alone made it look exposed.

  So: gather every candidate commit across every path, order by COMMIT TIME,
  and take the newest that is present and v2-authored. One thread, one
  pre-hop authored form."""
  cands = []
  for p in paths:
    rc, out = git(repo, "log", "--all", "--format=%H %ct", "--", p)
    for line in out.splitlines()[:cap]:
      f = line.split()
      if len(f) == 2:
        cands.append((int(f[1]), f[0], p))
  cands.sort(reverse=True)
  for ts, c, p in cands:
    rc, _ = git(repo, "cat-file", "-e", "%s:%s" % (c, p))
    if rc != 0:
      continue                                   # absent here, not evidence
    rc, t = git(repo, "show", "%s:%s" % (c, p))
    if rc != 0 or "GENERATED VIEW" in t:
      continue                                   # a v3 view, not authored v2
    if " -- satisfied: " not in t and " -- status: " not in t:
      continue                                   # no v2 row markers
    return c, p, t
  return None, None, None

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
  exposed, immune, no_ev, scaffold, files = [], 0, 0, [], 0
  by_thread = {}
  for p in acceptance_paths(repo):
    by_thread.setdefault(thread_key(p), []).append(p)
  for key in sorted(by_thread):
    c, p, t = newest_v2_blob(repo, by_thread[key])
    if t is None:
      continue
    files += 1
    for line in t.splitlines():
      s_ = line.strip()
      if not AC.match(s_):
        continue
      m_ev = re.search(r" -- evidence:\s*(.*?)(?:\s+--\s|$)", s_)
      m_v = re.search(r" -- satisfied:\s*([a-z/]+)", s_)
      verdict = m_v.group(1) if m_v else None
      if not m_ev:
        no_ev += 1
      elif verdict != "no":
        immune += 1                              # keeps its evidence: Satisfied{e}
      elif is_scaffold(m_ev.group(1)):
        scaffold.append((p, s_[:90]))            # never an authored claim
      else:
        exposed.append((p, s_[:90]))             # THE DESTROYABLE SHAPE
  return files, exposed, immune, no_ev, scaffold, len(by_thread)

# ---------------------------------------------------------------- self-test

FIXTURE = """# Acceptance -- ST9999

- AC-01.1 the positive arm -- satisfied: no -- evidence: measured on this tree, 14 rows
- AC-01.2 satisfied keeps its evidence -- satisfied: yes -- evidence: the gate counts it
- AC-01.3 unsatisfied and silent -- satisfied: no
- AC-01.4 template scaffold -- satisfied: no -- evidence: [named evidence]
- AC-01.5 more scaffold -- satisfied: no -- evidence: [a doc / eyeball / gate criterion]
"""

# THE ARM MY FIRST SELF-TEST COULD NOT EXHIBIT.
# One thread, two historical paths: a stale `NOT-STARTED/` snapshot saying
# `satisfied: no` and the later post-collapse blob saying `satisfied: yes`.
# The row was SATISFIED before the hop, so the correct answer is NOT exposed.
# A per-path scanner reports 1 here and is wrong. Found on a live estate, not
# in a fixture -- which is why this arm exists.
MOVED_OLD = """# Acceptance -- ST9997

- AC-03.1 moved thread, stale snapshot -- satisfied: no -- evidence: a four-site grep
"""

MOVED_NEW = """# Acceptance -- ST9997

- AC-03.1 moved thread, current form -- satisfied: yes -- evidence: a four-site grep
"""

VIEW = """# Acceptance -- ST9999

<!-- GENERATED VIEW -- do not edit -->

- AC-02.1 a v3 view row -- satisfied: no -- evidence: would be a false positive
"""

def self_test():
  """DRIVE THE DETECTOR TO BOTH VERDICTS ON PLANTED GROUND TRUTH.

  A control that cannot fail is decoration. Each arm below is planted with a
  known answer, and the probe must produce that answer and no other."""
  d = tempfile.mkdtemp(prefix="probe-selftest-")
  try:
    subprocess.run(["git", "-C", d, "init", "-q"], check=True)
    subprocess.run(["git", "-C", d, "config", "user.email", "t@t"], check=True)
    subprocess.run(["git", "-C", d, "config", "user.name", "t"], check=True)
    os.makedirs(os.path.join(d, "st", "ST9999"))
    os.makedirs(os.path.join(d, "st", "ST9998"))
    open(os.path.join(d, "st", "ST9999", "acceptance.md"), "w").write(FIXTURE)
    open(os.path.join(d, "st", "ST9998", "acceptance.md"), "w").write(VIEW)
    os.makedirs(os.path.join(d, "st", "NOT-STARTED", "ST9997"))
    open(os.path.join(d, "st", "NOT-STARTED", "ST9997", "acceptance.md"), "w").write(MOVED_OLD)
    subprocess.run(["git", "-C", d, "add", "-A"], check=True)
    subprocess.run(["git", "-C", d, "commit", "-qm", "fixture"], check=True)
    # the status-bucket collapse: same thread, new path, updated verdict
    shutil.rmtree(os.path.join(d, "st", "NOT-STARTED"))
    os.makedirs(os.path.join(d, "st", "ST9997"))
    open(os.path.join(d, "st", "ST9997", "acceptance.md"), "w").write(MOVED_NEW)
    subprocess.run(["git", "-C", d, "add", "-A"], check=True)
    subprocess.run(["git", "-C", d, "commit", "-qm", "collapse buckets"], check=True)

    files, exposed, immune, no_ev, scaffold, seen = scan(d)
    arms = [
      ("POSITIVE  fires on a known-damaged row",      len(exposed) == 1,  "%d exposed" % len(exposed)),
      ("POSITIVE  names the RIGHT row",               len(exposed) == 1 and "AC-01.1" in exposed[0][1], exposed[0][1][:46] if exposed else "-"),
      ("NEGATIVE  silent on satisfied-with-evidence", immune == 2,        "%d immune (ST9999 + moved ST9997)" % immune),
      ("NEGATIVE  silent on unsatisfied-no-evidence", no_ev == 1,         "%d no-ev" % no_ev),
      ("NEGATIVE  silent on TEMPLATE SCAFFOLD x2",    len(scaffold) == 2, "%d scaffold" % len(scaffold)),
      ("NEGATIVE  skips a v3 GENERATED VIEW entirely", files == 2,        "%d v2 thread(s)" % files),
      ("NEGATIVE  a MOVED thread uses its NEWEST form", not any("AC-03.1" in e[1] for e in exposed), "%d stale-path row(s) leaked" % sum(1 for e in exposed if "AC-03.1" in e[1])),
      ("NEGATIVE  a moved thread counted ONCE, not per-path", immune == 2 and len(exposed) == 1, "1 thread, 1 row, not 2"),
    ]
    ok = True
    print("SELF-TEST -- planted ground truth, both verdicts driven\n")
    for name, passed, got in arms:
      print("  %-46s %s   (%s)" % (name, "PASS" if passed else "FAIL", got))
      ok = ok and passed
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
  files, exposed, immune, no_ev, scaffold, seen = scan(repo)
  print("estate: %s" % repo)
  print("threads with an acceptance file in history: %d" % seen)
  print("threads with a V2-AUTHORED form recovered   : %d" % files)
  print("")

  # **A ZERO YOU CANNOT TRUST MUST NOT EXIT 0.**
  # "Nothing was measured" and "nothing is exposed" print the same headline
  # and mean opposite things. Separating them is the whole point: an estate
  # whose v2 history was squashed, or which was born under v3, has NO v2
  # source to compare against -- and a confident 0 there is the
  # zero-by-construction shape this method exists to refuse.
  if files == 0:
    if seen == 0:
      print("NOT MEASURED: no acceptance file appears anywhere in this history.")
      print("  This estate has no acceptance contract to lose. Nothing to say.")
    else:
      print("NOT MEASURED: %d thread(s) have an acceptance file, but NO v2-authored" % seen)
      print("  form survives in history -- every blob carries the v3 GENERATED VIEW")
      print("  banner. Two very different causes look identical here:")
      print("    (a) the estate was BORN under v3        -> genuinely nothing to lose")
      print("    (b) its v2 history was squashed/imported -> UNMEASURABLE FROM GIT")
      print("  **DO NOT REPORT THIS AS ZERO EXPOSURE.** Check whether this estate")
      print("  ever ran v2 before drawing either conclusion.")
    sys.exit(3)

  print("  EXPOSED (destroyable shape) : %d   <- PREDICTED-UNCONFIRMED" % len(exposed))
  print("  immune  (satisfied+evidence): %d" % immune)
  print("  no evidence clause          : %d" % no_ev)
  print("  template scaffold, excluded : %d" % len(scaffold))
  if exposed:
    print("\nexposed rows:")
    for p, l in exposed:
      print("  %s\n    %s" % (p, l))
  print("\nEXPOSURE IS NOT DAMAGE. Confirm against your own canon before reporting"
        "\nthese as lost. Drive --self-test before trusting any number, including 0.")
