#!/usr/bin/env python3
"""Would a proposed declared field reproduce the MCP withhold list we already have?

A DESIGN PROBE, NOT AN INSTRUMENT, and the difference decides how to read a
green: every other file in this directory checks a claim the table MAKES, and
this one tests a field the table does NOT YET CARRY. It exists to be run before
`acts_upon` (or whatever it ends up being called) is declared, and to be either
promoted into that field's check or DELETED with the idea. A probe kept past its
question is a check nobody can interpret.

WHY IT EXISTS. vc ruled that the withheld-13 policy earns a declared field, and
attached the condition that the field ships with its consumer and its check in
ONE change. They also named the canary in as many words: the field must
reproduce the existing 13 EXACTLY, computed rather than restated -- and if the
derived set is not those 13, either the field is wrong or one of the 13 is, and
both are worth knowing BEFORE it ships.

THE POPULATION. 26 rows carry `exposed_on_mcp: false`. Thirteen are family roots
with no action of their own; the other thirteen are leaves that were deliberately
withheld, and every one of those thirteen is a `mutate`. This probe is about the
second thirteen -- the roots are withheld for a structural reason nobody disputes.

HYPOTHESIS 1 -- `acts_upon`: one modelled entity / the estate / the environment,
withholding `mutate AND acts_upon != entity`. **DISPROVED, and structurally
rather than by calibration.** It withholds 32 where the table withholds 13, and
the count is not the argument. `lang init` and `lang remove` act upon the
identical thing and sit on opposite sides of the partition, as do `agents init`
against `agents generate`/`sync`, and `claude upgrade`/`start` against the four
other `claude` verbs. Any function of one field returns one answer for rows that
share that field's value, so no classification of this property can reproduce
the partition. Three families independently, which rules out a single bad row.

HYPOTHESIS 2 -- RECOVERABILITY: can the surface put the estate back?
  reversible  another shipped verb undoes it (`st done` / `st reopen`)
  idempotent  re-running produces the same state (a regenerated view)
  one-way     neither, so nothing on this surface recovers the prior state
`one-way` reproduces the thirteen with ONE disagreement, reported rather than
absorbed -- see below. It is also the better property on its own terms: nobody
withheld `lang remove` because of what it touches, they withheld it because you
cannot get back what it deletes, and that survives any ruling about MCP.

THE ONE FREE PARAMETER, DECLARED RATHER THAN BURIED, because fitting a model by
relabelling until it matches is the exact trap here. `lang init` moved from
`one-way` to `idempotent`, which took the result from 14 to 13. The
justification is the row's OWN help text -- "Install per-language canon
(idempotent; multi-lang)" -- so the table declares it and this file did not
decide it. Everything else was classified before comparing against
`exposed_on_mcp`.

THE SURVIVING DISAGREEMENT, WHICH IS THE USEFUL OUTPUT. `ext new` is `one-way`
by the rule and EXPOSED in the table: the `ext` family ships `list`, `show`,
`validate` and `new`, and no verb that removes an extension. Its twin runs the
other way -- `backup` is WITHHELD and is the same shape, additive, destroying
nothing, its own help reading "Snapshot this machine's store for fast local
restore". **Two rows that create something new and destroy nothing, treated
oppositely.** No property explains that pair; only a ruling does, and either
ruling makes the field derive cleanly. (It was first classified `reversible`
here with the note "(none, but scoped to one ext)" -- a note admitting the rule
did not hold, which is why it was re-run honestly.)

Run: python3 intent/st/ST0056/parity/tools/withhold_probe.py
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, "..", "..", "..", "..", ".."))
TABLE = os.environ.get("TABLE", os.path.join(REPO, "surface", "dispatch-table.json"))


def die(msg):
  sys.stderr.write("error: %s\n" % msg)
  raise SystemExit(2)


# --- hypothesis 1: what does the verb act upon? -------------------------------
ACTS_UPON = {
  "entity": {
    "st new", "st start", "st done", "st cancel", "st triage", "st hold", "st resume",
    "st reopen", "st reinstate", "st sync", "wp new", "wp start", "wp done", "wp reopen",
    "wp unstart", "ac satisfy", "ac unsatisfy", "ac descope", "ac rescope", "ac withdraw",
    "ac reinstate", "at green", "at red", "at na", "issues add", "issues close",
    "issues open", "todo done", "todo notdone", "todo toggle", "ext new",
  },
  "estate": {
    "at lint", "todo", "todo list", "todo update", "config set", "agents generate",
    "agents sync", "agents init", "lang init", "lang sync", "lang remove", "claude prime",
    "claude ws", "learn", "fileindex", "sync", "st repair", "st bootstrap", "init",
    "bootstrap", "upgrade", "ingest", "backup",
  },
  "environment": {
    "daemon", "mcp", "claude start", "claude upgrade", "claude subagents", "claude skills",
    "claude rules", "claude hook", "llm usage_rules",
  },
}

# --- hypothesis 2: can the surface put the estate back? -----------------------
RECOVERABILITY = {
  "reversible": {
    "st new", "st start", "st done", "st cancel", "st triage", "st hold", "st resume",
    "st reopen", "st reinstate", "wp new", "wp start", "wp done", "wp reopen", "wp unstart",
    "ac satisfy", "ac unsatisfy", "ac descope", "ac rescope", "ac withdraw", "ac reinstate",
    "at green", "at red", "at na", "issues add", "issues close", "issues open",
    "todo done", "todo notdone", "todo toggle", "config set",
  },
  "idempotent": {
    "todo", "todo list", "todo update", "agents generate", "agents sync", "lang sync",
    "sync", "fileindex", "st sync", "at lint", "claude prime", "learn", "llm usage_rules",
    "claude ws", "claude subagents", "claude skills", "claude rules", "claude hook",
    # Declared idempotent by its OWN help text, not by this file. See the header.
    "lang init",
  },
  "one-way": {
    "st repair", "st bootstrap", "init", "bootstrap", "upgrade", "agents init",
    "claude upgrade", "claude start", "lang remove", "ingest", "backup", "daemon", "mcp",
    # Reported as a DISAGREEMENT, not absorbed: exposed in the table, one-way here.
    "ext new",
  },
}


def classify(sets, path):
  for name, members in sets.items():
    if path in members:
      return name
  return None


def run(label, sets, withhold_when, mutations, actual):
  unclassified = sorted(p for p in mutations if classify(sets, p) is None)
  if unclassified:
    # A partial classification makes every number below meaningless, so it
    # REFUSES rather than reporting a comparison over whatever it happened to
    # cover. Same rule the shell instruments here follow.
    die("%s leaves %d row(s) unclassified, so nothing can be derived: %s"
        % (label, len(unclassified), ", ".join(unclassified)))

  derived = {p for p in mutations if withhold_when(classify(sets, p))}
  over = sorted(derived - actual)
  under = sorted(actual - derived)
  print("%s\n  derived withhold: %d   actual withhold: %d" % (label, len(derived), len(actual)))
  print("  would withhold but IS exposed (%d): %s" % (len(over), ", ".join(over) or "none"))
  print("  IS withheld but would expose (%d): %s" % (len(under), ", ".join(under) or "none"))
  return over, under


def main():
  if not os.path.exists(TABLE):
    die("no dispatch table at %s" % TABLE)
  tbl = json.load(open(TABLE))

  # `.families[].entries[]` is NEVER the population -- it is 104 of 112 rows,
  # and the other 8 are the top-level `new_surface` array.
  rows = [e for f in tbl["families"] for e in f["entries"]] + tbl.get("new_surface", [])
  shipped = [e for e in rows
             if e.get("disposition") != "retire" and e.get("target", {}).get("state") != "retire"]
  mutations = [e for e in shipped if e.get("read_or_mutate") == "mutate"]
  if not mutations:
    die("no shipped mutations found -- an empty population compares equal to a correct one")

  paths = {e["path"] for e in mutations}
  actual = {e["path"] for e in mutations if e.get("exposed_on_mcp") is False}

  print("shipped mutations: %d   withheld from MCP: %d\n" % (len(paths), len(actual)))
  run("H1  acts_upon (withhold: not `entity`)", ACTS_UPON, lambda c: c != "entity", paths, actual)
  print()
  over, under = run("H2  recoverability (withhold: `one-way`)", RECOVERABILITY,
                    lambda c: c == "one-way", paths, actual)

  print("\nH1 is disproved structurally, not by its count. The rows below act upon")
  print("the identical thing and sit on opposite sides, so no function of one field")
  print("can separate them:")
  for fam in ("lang", "agents", "claude"):
    same = [(p, classify(ACTS_UPON, p), p in actual) for p in sorted(paths)
            if p.split(" ")[0] == fam]
    by = {}
    for p, c, withheld in same:
      by.setdefault(c, []).append((p, withheld))
    for c, items in sorted(by.items()):
      if len({w for _, w in items}) > 1:
        print("  %-8s %-12s %s" % (fam, c,
              ", ".join("%s=%s" % (p, "WITHHELD" if w else "exposed") for p, w in items)))

  print("\nH2's surviving disagreement is the output worth reading:")
  print("  %s is one-way by the rule and exposed in the table." % (", ".join(over) or "none"))
  print("  `backup` is the same shape -- additive, destroys nothing -- and is WITHHELD.")
  print("  Two rows that create and destroy nothing, treated oppositely. Only a ruling")
  print("  settles that; either ruling makes the field derive cleanly.")


if __name__ == "__main__":
  main()
