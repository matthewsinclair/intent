#!/usr/bin/env python3
"""Fleet exposure scan for issue 0133 (conflab-vc's convention predictor).

EXPOSURE, not damage. An estate's exposure is the count of non-test criteria
whose PRE-HOP AUTHORED row carries an evidence clause AND `satisfied: no`.
Those are the rows the ingest destroys: `AcState::Unsatisfied` is a unit
variant, so a criterion authored unsatisfied-with-evidence has nowhere to put
it. Rows authored `satisfied: yes` keep their evidence and are the immune arm.

This needs no damage assessment and no comparison of outcomes -- it reads only
each estate's own authored source, recovered from git history at the path the
ingest itself read (legacy.rs:1273).

READ-ONLY BY CONSTRUCTION. Only `git log`, `git show` and `git cat-file` are
used. `git status` is deliberately never called: it refreshes the index and
would disturb a live checkout, and every estate here has a session attached.
"""

import subprocess, sys, re, os, collections

def git(repo, *a):
  r = subprocess.run(["git", "-C", repo] + list(a), capture_output=True)
  return r.returncode, r.stdout.decode("utf-8", "replace")

def acceptance_paths(repo):
  rc, out = git(repo, "log", "--all", "--format=", "--name-only", "--", "*/acceptance.md")
  return sorted({l.strip() for l in out.splitlines() if l.strip().endswith("acceptance.md")})

def last_v2_blob(repo, path, cap=25):
  """Newest blob at `path` that is v2-AUTHORED: present, carrying v2 row
  markers, and lacking the v3 generated-view banner. Absent is not clean."""
  rc, out = git(repo, "log", "--all", "--format=%H", "--", path)
  for c in out.split()[:cap]:
    rc, _ = git(repo, "cat-file", "-e", "%s:%s" % (c, path))
    if rc != 0:
      continue                                   # absent here, not evidence
    rc, t = git(repo, "show", "%s:%s" % (c, path))
    if rc != 0 or "GENERATED VIEW" in t:
      continue
    if " -- satisfied: " not in t and " -- status: " not in t:
      continue
    return c, t
  return None, None

AC = re.compile(r"^-\s+AC-[\w.]+\s")

def scan(repo):
  exposed, immune, no_ev, files = [], 0, 0, 0
  for p in acceptance_paths(repo):
    c, t = last_v2_blob(repo, p)
    if t is None:
      continue
    files += 1
    for line in t.splitlines():
      if not AC.match(line.strip()):
        continue
      has_ev = " -- evidence: " in line
      m = re.search(r" -- satisfied:\s*([a-z/]+)", line)
      verdict = m.group(1) if m else None
      if not has_ev:
        no_ev += 1
      elif verdict == "no":
        exposed.append((p, line.strip()[:90]))
      else:
        immune += 1
  return files, exposed, immune, no_ev

if __name__ == "__main__":
  print("%-13s %6s %9s %8s %9s" % ("estate", "files", "EXPOSED", "immune", "no-ev"))
  print("-" * 50)
  tot = 0
  detail = {}
  for name in sys.argv[1:]:
    repo = os.path.expanduser("~/Devel/prj/%s" % name)
    if not os.path.isdir(os.path.join(repo, ".git")):
      print("%-13s  NO GIT" % name); continue
    files, exposed, immune, no_ev = scan(repo)
    tot += len(exposed)
    detail[name] = exposed
    print("%-13s %6d %9d %8d %9d" % (name, files, len(exposed), immune, no_ev))
  print("-" * 50)
  print("FLEET EXPOSED TOTAL: %d" % tot)
  for k, v in detail.items():
    if v:
      print("\n%s -- %d exposed row(s):" % (k, len(v)))
      for p, l in v[:6]:
        print("   %s" % p)
        print("     %s" % l)
