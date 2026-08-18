#!/bin/bash
# runner_roster_check.sh -- every parity instrument declares whether anything runs it.
#
# Issue 0059: eleven instruments lived in this directory, three had an execution
# site, and eight had none. Each of those eight had been written, reasoned about
# at length in its own header, registered in MODULES.md, mutation-tested, and in
# three cases CITED BY A GREEN ACCEPTANCE ROW -- with every individual record
# correct, because an instrument's existence and an instrument's execution are
# recorded in different places and NOTHING JOINED THEM. This is that join.
#
# WHY THE JOIN IS A THIRD AXIS AND NOT A COLUMN ON AN EXISTING ONE. MODULES.md
# records what a tool IS, and in prose what its POSTURE is ("REPORTS, never
# gates") -- that is the script's own internal contract, and it is true whether
# or not anything ever calls the script. `precommit` records what the GATE runs.
# Those two can each be perfectly accurate while the answer to "does anything
# run this?" is nowhere. So this file adds the missing fact -- a DISPOSITION --
# and then measures it against the runner instead of trusting it.
#
# THE POPULATION HAD ALREADY MOVED BEFORE THE FIX WAS WRITTEN, WHICH IS THE
# ARGUMENT FOR THE REFUSAL BELOW. 0059 measured eleven instruments at 0f87fc2c
# and its table lists eleven. By the time the remedy was built there were TWELVE
# -- `ratified_in_check.sh` landed hours after the census, unwired, and no
# artefact in the repository noticed. A correct measurement of a population that
# has silently acquired a member is the failure this check has to survive, so it
# does not compare against a remembered count: it enumerates the directory every
# run and REFUSES on any file it has no row for. The roster fails on the day a
# tool is added, which is the only day anybody is in a position to classify it.
#
# WHY IT ASKS THE RUNNER RATHER THAN REIMPLEMENTING ITS RULE. `precommit`'s own
# header states the doctrine, and states it because it was learned the hard way:
# `int hooks` derived the guard roster by grepping the runner's source, anchored
# on a PATH SHAPE, and under-reported a three-guard gate as two the same day.
# `--list-guards` exists so a reader can ask. This asks.
#
# DECLARED AND INVOKED ARE MEASURED SEPARATELY, BECAUSE THE GUARD-0 ROT IS
# EXACTLY THEIR DISAGREEMENT. A guard can be named in `--list-guards` and
# implemented nowhere, or invoked in the body and named in no roster; both are
# real and both have happened in this file. One measurement cannot see either.
#
# Exit codes follow the family: 0 clean, 1 a finding, 2 cannot measure. This one
# GATES on its findings, unlike its report-only siblings, and the difference is
# principled rather than a mood. A report-only check is report-only because most
# of its hits are a legitimate mid-ladder state -- a command not wired yet, a
# row not ratified yet. There is no legitimate state in which a tool exists and
# its disposition is undeclared: the fix is one line in the roster below, and
# the whole point is to force it at the moment the tool arrives.

set -uo pipefail

die() { echo "error: $1" >&2; exit 2; }

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)" || die "cannot resolve my own directory"
# tools -> parity -> ST0056 -> st -> intent -> repo root. FIVE. The count is
# taken from guide_refs_check.sh's header rather than recounted here: the
# version that was one short resolved to `intent/` and died on every single
# invocation, including all four mutants, which the harness then scored as kills.
ROOT="$(cd "$HERE/../../../../.." && pwd)" || die "cannot resolve the repository root from $HERE"

# RUNNER is overridable for the mutation proof below, following the family's
# existing `TABLE=` and `BIN=` precedent. It exists because the calibration on
# line ~140 is a guard on MY OWN NEEDLE, and a guard that has never been fired
# is an unrun claim -- the failure this whole check is about, one level down.
RUNNER="${RUNNER:-$ROOT/bin/.devbin/cmd/precommit}"
DISPATCH="$ROOT/bin/int"

[ -d "$HERE" ] || die "no tools directory at $HERE"
[ -f "$RUNNER" ] || die "no repo-local runner at $RUNNER -- there is nothing to measure a disposition against"
[ -x "$DISPATCH" ] || die "no devbin dispatcher at $DISPATCH -- the runner only answers through it"

# THE ROSTER. One row per `*_check.sh`: <name> <gated|manual> <reason>.
#
# `gated`  -- the repo-local pre-commit gate runs it, bare, on every commit.
# `manual` -- it does not, and the reason says why. A reason is REQUIRED and is
#             checked for emptiness: "manual" with no reason is the unlabelled
#             state this check exists to end, wearing a label.
#
# Every cost below was MEASURED on 2026-08-17 at 55e540df, not estimated, per
# the runner's own rule that coverage is reported as measured and never as
# designed. Re-time before moving a row on cost grounds.
ROSTER='
class_vocab_check.sh       gated   57ms, two committed files, one-line verdict
corrected_check.sh         gated   90ms, static, reports and never gates
generator_inputs_check.sh  gated   211ms, index-only, whole-set by design
provenance_check.sh        gated   431ms, three greps over the stamped artefacts
ratified_in_check.sh       gated   178ms, static read of the dispatch table
residue_class_check.sh     gated   42ms and a single line, the cheapest here
runner_roster_check.sh     gated   782ms of which 263ms is asking the runner through the dispatcher, which is the price of not re-grepping its source; it is a *_check.sh and rosters itself
self_provenance_check.sh   gated   504ms over 27 blobs read from the INDEX; whole-set because the failure is staging one of two facts, so a path trigger would have to fire on the path that is not there
stale_at_check.sh          gated   70ms and a single line, reports presence only
view_skew_check.sh         gated   3077ms, the slowest gated one, path-triggered
canon_commit_check.sh      manual  new 2026-08-18, awaiting roster admission -- there is no narrow attachment-sync verb, so the only order a gate would permit (sync canon, then commit file and canon together) is closed to nodes that may not sync; revisit after ST0057 WP-08. Reads git only: no worktree, no binary, no clock. 2.1-2.3s narrowed / 9.5-9.7s --exhaustive, measured at f2a2675f on two machines with /usr/bin/time -p -- STATED WITH SUBJECT AND REVISION BECAUSE THE BARE FIGURES IN THE ROWS ABOVE CANNOT BE COMPARED AGAINST ANYTHING: measuring your own tool and comparing it to one of them is measured-against-recorded across unknown machines, trees and dates, which is how the first timing claim for THIS row went wrong by half. Driven four ways on real history (vacuous 2, adds 1, inherited-not-failed 0, clean 0) and caught an unplanted divergence on its first whole-tree run
conservation_check.sh      manual  takes a MIGRATED tree as an argument and no such tree exists until WP-10 lands, so there is no bare invocation for a gate to make; it refuses with exit 2 rather than passing when handed an unmigrated one, which is the behaviour a gate would have to bypass on every commit
drift_check.sh             manual  compares a STAMPED inventory against live canon, so gating it would block a dispatch-table edit until somebody re-runs a 27-family measurement sweep -- a measurement, not a fix
guide_refs_check.sh        manual  takes required prose-file arguments, so there is no bare invocation for a gate to make
same_end_state_check.sh    manual  takes three tree arguments, so there is no bare invocation for a gate to make; it refuses an absent, EMPTY or UNCHANGED subject rather than comparing nothing, refuses two subjects that are one directory rather than comparing a tree with itself, and reports a differing SQLite store (or its -wal/-shm sidecar) as NOT JUDGED BY THIS TOOL, naming the path and the reason, because comparing the content of a container needs the sqlite3 shell and that would make the verdict depend on the machine
implemented_check.sh       manual  invokes every declared row in a fresh throwaway project with a sandboxed HOME
surface_check.sh           manual  probes --help across 100+ paths, so every commit would pay for a full surface sweep
'

# ---------------------------------------------------------------------------
# Populations. Each is enumerated, never remembered, and an empty one refuses.
# ---------------------------------------------------------------------------

# 1. PRESENT -- what THIS COMMIT holds, read from the index and never from the
#    working tree.
#
#    **It globbed the worktree until 2026-08-17, and in a shared clone that
#    froze every node's commits on paths they had never touched** (found by dc,
#    who held the commit and diagnosed it rather than reaching for
#    `--no-verify`). Four sessions work this one checkout, so any peer's
#    untracked mid-work `*_check.sh` was an unrostered tool to this guard --
#    and the only way past it was to wait for its owner to land a roster row.
#    A guard that has to be waited out is one step from a guard that gets
#    bypassed.
#
#    **The purpose survives exactly, which is the thing to check before
#    changing a guard**: a tool that is added AND STAGED is in this commit's
#    index and is still caught on the day it arrives, which is the only day
#    anyone is in a position to classify it. What stops being caught is a file
#    that is not part of the project and is not the committer's business.
#    `git ls-files` honours `GIT_INDEX_FILE`, and git hands a hook a temporary
#    index during a partial commit, so under `--only` this reads HEAD plus the
#    committer's own named paths -- which is the population it should judge.
#    Verified both ways at this tree: worktree glob 15, index read 15, same
#    names; and 15 again from a HEAD-only index built with `read-tree`.
PRESENT="$(git -C "$HERE" ls-files -- "$HERE/*_check.sh" | sed 's|.*/||' | sort)"
[ -n "$PRESENT" ] || die "this commit holds no *_check.sh under $HERE -- an empty population and a clean roster compare equal, so this is a refusal and not a pass"

# 2. ROSTERED -- what this file declares.
ROSTERED="$(printf '%s\n' "$ROSTER" | awk 'NF { print $1 }' | sort)"
[ -n "$ROSTERED" ] || die "the roster in this file parsed empty -- its format has changed under the parser"

# 3. DECLARED -- what the runner says it checks, asked rather than inferred.
#    Field 2 is a repo-relative path or a dash; only the ones under this
#    directory are ours to have an opinion about.
GUARDS="$("$DISPATCH" precommit --list-guards 2>/dev/null)" || die "the runner refused --list-guards; its roster cannot be read"
[ -n "$GUARDS" ] || die "the runner's --list-guards printed nothing -- it declares guards, so an empty answer is a broken reader and not a gate with no guards"
DECLARED="$(printf '%s\n' "$GUARDS" | awk -F'\t' '$2 != "" && $2 != "-" { n = split($2, p, "/"); print p[n] }' | grep '_check\.sh$' | sort)"

# 4. INVOKED -- what the runner's body actually calls.
#
#    The needle is `$TOOLS/<name>`, and it is anchored on a variable this file
#    does not own, so it is CALIBRATED rather than trusted. My own watch-out,
#    fired three times already: a needle written from the author's head
#    enumerates the spellings the author remembers. If the runner stops keeping
#    its tools directory in a TOOLS variable, every answer below silently
#    becomes "invoked: none" -- which would read as nine simultaneous
#    regressions rather than as a broken instrument. So the shape is asserted
#    first and its absence REFUSES.
grep -q '^TOOLS=' "$RUNNER" || die "the runner no longer defines TOOLS= -- the invocation needle here is anchored on \"\$TOOLS/<tool>\" and must be re-derived before any answer it gives means anything"
INVOKED="$(grep -v '^[[:space:]]*#' "$RUNNER" | grep -o '\$TOOLS/[A-Za-z0-9_]*_check\.sh' | sed 's|^\$TOOLS/||' | sort -u)"
[ -n "$INVOKED" ] || die "the runner invokes no *_check.sh at all through \$TOOLS -- three are known to be wired, so this is the needle failing and not the gate emptying"

findings=""
add() { findings="${findings}  $1
"; }

has() { printf '%s\n' "$2" | grep -qx -- "$1"; }

# ---------------------------------------------------------------------------
# A. Every tool on disk is rostered, and every rostered tool is on disk.
# ---------------------------------------------------------------------------
while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$ROSTERED" || add "$t exists in the tools directory and has NO roster row -- declare it gated or manual, with a reason"
done <<EOF
$PRESENT
EOF

while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$PRESENT" || add "$t has a roster row and NO file -- the roster has outlived the instrument"
done <<EOF
$ROSTERED
EOF

# ---------------------------------------------------------------------------
# B. Each disposition matches what the runner actually does.
#
# `gated` is TWO claims, checked separately, because the guard-0 rot recorded in
# the runner's header was precisely their disagreement: named in the roster,
# implemented inline, and `int hooks` reported a three-guard gate as two.
# ---------------------------------------------------------------------------
gated_n=0
manual_n=0
while read -r name disp reason; do
  [ -n "$name" ] || continue
  case "$name" in \#*) continue ;; esac

  # Only judge rows whose file exists; a ghost row was already reported above
  # and would otherwise be counted twice under two different descriptions.
  has "$name" "$PRESENT" || continue

  case "$disp" in
    gated)
      gated_n=$((gated_n + 1))
      has "$name" "$DECLARED" || add "$name is rostered GATED but the runner does not name it in --list-guards -- a guard the gate runs and does not declare is invisible to \`int hooks\`"
      has "$name" "$INVOKED" || add "$name is rostered GATED but nothing in the runner's body invokes it -- this is the 0059 defect itself, in the roster meant to prevent it"
      ;;
    manual)
      manual_n=$((manual_n + 1))
      [ -n "$reason" ] || add "$name is rostered MANUAL with no reason -- an unlabelled instrument wearing a label"
      has "$name" "$INVOKED" && add "$name is rostered MANUAL and the runner invokes it -- the roster is wrong in the direction that reads as safe"
      has "$name" "$DECLARED" && add "$name is rostered MANUAL and the runner declares it in --list-guards -- \`int hooks\` is reporting a guard that does not run"
      ;;
    *)
      add "$name has disposition '$disp', which is neither gated nor manual"
      ;;
  esac
done <<EOF
$ROSTER
EOF

# ---------------------------------------------------------------------------
# C. The runner does not run anything the roster has never heard of.
# ---------------------------------------------------------------------------
while IFS= read -r t; do
  [ -n "$t" ] || continue
  has "$t" "$ROSTERED" || add "the runner invokes $t and the roster has no row for it"
done <<EOF
$INVOKED
EOF

total="$(printf '%s\n' "$PRESENT" | grep -c .)"

if [ -n "$findings" ]; then
  printf 'roster: %s instrument(s) in this commit; %s gated, %s manual; the roster and the runner DISAGREE\n' \
    "$total" "$gated_n" "$manual_n"
  printf '%s' "$findings"
  printf '  the roster is in %s -- fix the row or fix the runner, whichever is lying.\n' \
    "intent/st/ST0056/parity/tools/runner_roster_check.sh"
  exit 1
fi

printf 'roster: %s instrument(s) in this commit, all rostered; %s gated, %s manual; every disposition matches the runner\n' \
  "$total" "$gated_n" "$manual_n"
exit 0
