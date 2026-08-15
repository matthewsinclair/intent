#!/bin/bash
# probe_toplevel.sh -- the driver that PRODUCES probes/toplevel.tsv.
#
# This script did not exist. The 2026-08-14 matrix at 69d42a7 was captured by an
# ad-hoc loop in a session that has since ended, writing into a throwaway scratch
# directory, and the TSV was never tracked. So the input behind 26 committed
# `cmd-*.md` inventories was unreproducible and gone (parity.md, measurement rule
# 13), and `gen_inventory.sh` -- which only ever READS that TSV -- would have
# rewritten all 26 as husks carrying the good revision's stamp. It now refuses
# instead; this is the other half of that fix. A refusal tells you the input is
# missing. It does not give you a way to make one.
#
# The rule the missing driver broke: if the input is not committed, the output is
# stamp-only. The TSV this writes is meant to be COMMITTED alongside the
# inventories it explains, which is what turns 26 stamp-only artefacts into
# content-checkable ones.
#
# Usage:
#   SP=<scratch> REV=<git-rev> bash probe_toplevel.sh
#
# It builds nothing and destroys nothing outside $SP.

set -uo pipefail

die() { echo "error: $1" >&2; exit 2; }

SP="${SP:?set SP -- the scratch directory this run owns}"
WT="${WT:-$SP/wt}"
SBX="${SBX:-$SP/sandbox}"
OUT="${OUT:-$SP/probes}"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY="$(cd "$HERE/.." && pwd)"

[ -x "$WT/bin/intent" ] || die "no intent binary at $WT/bin/intent -- create the worktree first"

# THE PROBE MATRIX MUTATES ITS OWN SANDBOX, so it is not idempotent and a second
# run against a used sandbox measures a different branch. Measured, not feared:
# `intent todo` bare generates `intent/todo.md` when it is absent, so run one
# captures `no intent/todo.md yet -- generating...` and run two captures the read
# path. Two runs of the same script at the same revision disagreed on a first
# line, and nothing in the output said which run it was.
#
# That is the same command whose dispatch-table row is classified `mutate` for
# exactly this reason, and it is the shape that makes the classification worth
# having: it reads on every run after the first, so the mutation is invisible
# unless something insists on a fresh start.
#
# So the sandbox is rebuilt here rather than reused, and rebuilt by THIS script
# rather than by the caller -- the last time sandbox setup lived in the caller,
# it was an uncommitted shell loop and the whole measurement became
# unreproducible when the session ended.
rm -rf "$SBX"
mkdir -p "$SBX"
( cd "$SBX" && env INTENT_HOME="$WT" HOME="${FAKEHOME:-$SP/fakehome}" "$WT/bin/intent" init "ProbeSandbox" ) >/dev/null 2>&1
[ -d "$SBX/intent" ] || die "could not create a fresh sandbox at $SBX -- \`intent init\` produced no intent/ directory. Refusing rather than probing a half-built project, which would answer every not-in-a-project probe correctly and every in-project probe wrongly."

# THE POPULATION IS DERIVED, NOT RETYPED. Enumerating it here by hand would put
# a second copy of gen_inventory.sh's command list in the tree, and the two would
# diverge exactly when someone adds a command -- the failure mode where a
# re-probe silently measures fewer units than the inventory renders, and reports
# a clean diff for the ones it happened to cover.
#
# The committed `cmd-*.md` files ARE the population: one per measured unit, by
# construction, because gen_inventory writes exactly one per `gen` call. Deriving
# from them means a re-probe covers precisely what is committed -- which is the
# question a re-probe exists to answer -- and a command added to the inventory
# without a re-probe shows up as a file with no rows rather than as silence.
CMDS="$(cd "$PARITY" && ls cmd-*.md 2>/dev/null | sed 's/^cmd-//; s/\.md$//')"
[ -n "$CMDS" ] || die "no cmd-*.md inventories found under $PARITY -- nothing to re-probe, and an empty population would produce an empty TSV that reads like a completed run"
N_CMDS="$(printf '%s\n' "$CMDS" | wc -l | tr -d ' ')"

# shellcheck source=/dev/null
SP="$SP" WT="$WT" SBX="$SBX" OUT="$OUT" . "$HERE/probe.sh" || die "cannot source probe.sh"

mkdir -p "$OUT"
TSV="$OUT/toplevel.tsv"

{
  # A header, because gen_inventory.sh reads `NR>1`. A headerless TSV would lose
  # its first command silently rather than failing.
  printf 'label\trc\tstdout_bytes\tstderr_bytes\tstdout_first\tstderr_first\targs\n'
  for c in $CMDS; do
    probe_matrix "$c"
  done
} > "$TSV"

ROWS="$(awk 'END{print NR-1}' "$TSV")"
WANT="$(( N_CMDS * 4 ))"

# ASSERT THE RUN REACHED THE THING IT WAS AIMED AT. probe_matrix emits four rows
# per command unconditionally, so a short count means probes did not run at all
# rather than that some command declined to answer -- and a TSV short by a few
# rows is indistinguishable from a complete one to every downstream reader.
[ "$ROWS" = "$WANT" ] || die "wrote $ROWS probe rows for $N_CMDS commands; expected $WANT (four per command). The matrix is incomplete and gen_inventory.sh would render the gaps as dashes, which reads as 'measured, no output'."

echo "ok: $ROWS probe rows across $N_CMDS command(s) -> $TSV" >&2
echo "     worktree at $(cd "$WT" && git rev-parse --short HEAD)" >&2
