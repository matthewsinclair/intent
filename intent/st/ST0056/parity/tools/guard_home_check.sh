#!/usr/bin/env bash
# guard_home_check.sh -- a self-hosted Intent tree must run ITS OWN guards.
#
# WHY THIS EXISTS, and it closes an exposure whose premise was already false.
#
# hv's standing directive said that with `INTENT_HOME` pointing at the frozen
# `Intentv2`, this repo's guards resolve out of that checkout. ic measured that
# they do not: `pre-commit.sh` sets `GUARD_HOME="$INTENT_HOME_RESOLVED"` and then
# OVERRIDES it to the repo root when the repo is itself an Intent install --
# `lib/templates/hooks/pre-commit-guards.sh` and `VERSION` both present. That
# self-hosted branch wins here, so the directive's premise is falsified.
#
# hv asked for a MECHANISM rather than a variable. The mechanism already existed
# and nothing was watching it. That is the exposure: not that resolution is
# wrong today, but that the branch which makes it right is four lines that any
# refactor can drop, and NOTHING WOULD REPORT THE LOSS.
#
# WHAT THE LOSS LOOKS LIKE, AND WHY IT IS SILENT. Delete the branch and every
# self-hosted tree falls back to `$INTENT_HOME` -- here, the FROZEN v2 checkout.
# Behaviour would not change on the day it broke, because the four guard bodies
# are BYTE-IDENTICAL across both trees (driven 2026-08-22, all four). It would
# change later, when they drift, and by then the cause is a deleted branch
# nobody connected to a guard that started behaving differently.
#
# SO BYTE-IDENTITY IS THE WRONG CANARY, and it was the one offered. An ACTIVE
# tree and a FROZEN one are SUPPOSED to diverge -- this repo's guards are under
# development and v2's are frozen by definition -- so a byte-identity check goes
# red on the first legitimate guard edit and is cry-wolf by construction. The
# identity is not the property to protect; it is the reason the fallback is
# currently INVISIBLE. Resolution is the property, so resolution is what gates.
#
# IT GATES THE TRACKED TEMPLATE, NEVER THE INSTALLED COPY, AND THAT IS
# LOAD-BEARING. `pre-commit.intent` is gitignored by design (the installer owns
# it; tracking it would be a second home for canon), so a fresh clone has none.
# A check keyed to the installed copy would fail in every clone -- which is
# exactly the ARM C shape AC-01.5 spent two days on. The template is what ships
# to every consumer, so the template is what must carry the branch.
#
# WHAT IT DOES NOT DO. It does not run the hook, does not compare guard bodies
# across trees, and does not verify that the branch's CONDITION is correct for
# any particular install -- only that the override exists and assigns the repo
# root. A branch that is present and wrong is out of reach here, and saying so
# is cheaper than letting a green be read as more than it is.
set -euo pipefail

ROOT="${PROJECT_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
TEMPLATE="$ROOT/lib/templates/hooks/pre-commit.sh"
rc=0

echo "==> guard-home: a self-hosted tree runs its own guards, not \$INTENT_HOME's"

if [ ! -f "$TEMPLATE" ]; then
  echo "guard-home: NOT APPLICABLE -- no shipped hook template at lib/templates/hooks/pre-commit.sh."
  echo "guard-home:   This is not an Intent tool tree, so there is no template to protect."
  exit 0
fi

# The two halves are asserted SEPARATELY because they fail differently: a
# missing condition means the override can never fire, a missing assignment
# means it fires and does nothing. One combined grep would report either as the
# same fault and send the reader at the wrong half.
has_condition=$(grep -c 'lib/templates/hooks/pre-commit-guards.sh' "$TEMPLATE" || true)
has_assign=$(grep -c 'GUARD_HOME="\$_repo_root"' "$TEMPLATE" || true)

if [ "$has_condition" -eq 0 ]; then
  echo "guard-home: BLOCKED -- the shipped template no longer tests whether this repo IS an Intent install."
  echo "guard-home:   file: lib/templates/hooks/pre-commit.sh"
  echo "guard-home:   Without that test the self-hosted branch can never fire, so EVERY self-hosted"
  echo "guard-home:   tree falls back to \$INTENT_HOME -- for this repo, the FROZEN v2 checkout."
  echo "guard-home:   It would not misbehave today: the guard bodies are byte-identical across both"
  echo "guard-home:   trees, so the fallback is invisible until they drift, and then the cause is a"
  echo "guard-home:   deleted branch nobody connects to a guard that changed."
  rc=1
fi

if [ "$has_assign" -eq 0 ]; then
  echo "guard-home: BLOCKED -- the shipped template no longer assigns GUARD_HOME to the repo root."
  echo "guard-home:   file: lib/templates/hooks/pre-commit.sh"
  echo "guard-home:   The condition may still be present, in which case the branch fires and does"
  echo "guard-home:   nothing -- which reads as working when the resolution is unchanged."
  rc=1
fi

if [ "$rc" -eq 0 ]; then
  echo "guard-home: the self-hosted override is present in the shipped template (condition + assignment)."
  # REPORTED, NEVER GATED. The installed copy is per-machine and gitignored, so
  # its absence is the normal state of a fresh clone and must not fail anything.
  #
  # THE CARRIER HAS TWO SHAPES AND THIS ARM USED TO MODEL ONE (issue 0113).
  #
  # It asked ONE question -- does the installed copy carry `GUARD_HOME="$_repo_root"`
  # -- and read a `no` as "predates this template". Since the shim carrier landed
  # that inference is false by construction: **a shim is a LOCATOR and names no
  # guard content at all**, so it can never carry the override, and the printed
  # remedy (`intent claude upgrade --apply`) reinstalls the shim. The NOTE became
  # a permanent alarm on a healthy tree -- measured on Intent's own, where the
  # verb HAD been run that morning and the NOTE printed on every commit after.
  #
  # A reader who obeys it loops; a reader who learns to skip it skips the arm on
  # the day it means something. Same family as 0105: a reporter reading the
  # CARRIER and expecting the GATE BODY's properties.
  #
  # SO THE QUESTION IS NOW "WHAT IS THIS CARRIER", not "does it carry the string".
  # Three states, and the predates-NOTE is reserved for the one that is really
  # unrecognised rather than being the fallthrough for everything unmodelled.
  installed="$ROOT/.githooks/pre-commit.intent"
  shim_template="$ROOT/lib/templates/hooks/pre-commit-shim.sh"
  if [ ! -e "$installed" ]; then
    echo "guard-home:   this machine has no installed gate copy yet (normal in a fresh clone)."
  elif grep -q 'GUARD_HOME="\$_repo_root"' "$installed"; then
    echo "guard-home:   this machine's installed copy carries the override directly (monolithic carrier)."
  elif [ -f "$shim_template" ] && cmp -s "$installed" "$shim_template"; then
    echo "guard-home:   this machine's installed copy IS the current shim -- byte-identical to"
    echo "guard-home:   lib/templates/hooks/pre-commit-shim.sh. IT CARRIES NO OVERRIDE BY CONSTRUCTION,"
    echo "guard-home:   and that is correct: a shim locates the gate, and the override lives in the"
    echo "guard-home:   gate body it resolves -- which is the template gated above."
  else
    echo "guard-home:   NOTE -- this machine's installed copy matches NEITHER the override the template"
    echo "guard-home:   carries NOR the current shim, so this check cannot say what it is."
    echo "guard-home:   Most likely it predates one of the two; it may also be a shim installed from a"
    echo "guard-home:   DIFFERENT Intent install, which this arm compares against THIS tree's template"
    echo "guard-home:   and cannot distinguish. Ask the carrier itself before reinstalling:"
    echo "guard-home:     bash .githooks/pre-commit.intent --where"
    echo "guard-home:   If that resolves to a real install at state OK, the gate runs and nothing is owed."
    echo "guard-home:   Otherwise: intent claude upgrade --apply -- and note its SCOPE before running it."
    echo "guard-home:   That verb applies the WHOLE canon set, not just this file: it rewrites CLAUDE.md,"
    echo "guard-home:   AGENTS.md and .claude/settings.json, and region-edits the pre-commit chain block."
    echo "guard-home:   (Wording matched from bin/.devbin/cmd/hooks, which is the authority for it --"
    echo "guard-home:    one fact about a verb, phrased two ways, is the drift this repo calls Highlander.)"
  fi
fi

exit $rc
