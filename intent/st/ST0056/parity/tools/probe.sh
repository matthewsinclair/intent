#!/bin/bash
# probe.sh -- runtime surface capture for the ST0056 parity inventory.
#
# Captures, per invocation: exit code, stdout bytes, stderr bytes, and the first
# line of each stream, kept separate. Streams are redirected to files and counted
# rather than merged: this shell is zsh-adjacent and `cmd 2>&1 >/dev/null` tees
# stdout to the terminal under MULTIOS, so a merged capture measures the wrong
# thing (vc's watch-out).
#
# Isolation: INTENT_HOME is passed explicitly. bin/intent only self-resolves it
# when unset (bin/intent:12), so an inherited INTENT_HOME silently redirects
# every probe at the developer's live tree instead of the worktree under test.
#
# HOME is isolated too, and it was NOT isolated here until 2026-08-15. The
# 2026-08-14 matrix got it right by accident of the caller -- the ad-hoc driver
# did `export HOME="$SP/fakehome"` before sourcing this file, and that driver was
# never committed, so the isolation left with it. A re-probe written against this
# file alone therefore reads the DEVELOPER's machine: measured, `intent ext`
# answered `Extensions in /Users/matts/.intent/ext:` instead of the sandbox's
# `ok: no extensions installed`, a different code path and a different first
# line, not merely a different byte count.
#
# This is the parity.md rule about mechanisms turned on this file: the isolation
# that mattered lived in a sentence in someone's shell history, and the one that
# was written down (INTENT_HOME) is the one that survived. Both belong in the
# probe itself, where a caller cannot forget them.
FAKEHOME="${FAKEHOME:-$SP/fakehome}"
mkdir -p "$FAKEHOME"

set -u

# SP must be passed in or defaulted absolutely. Deriving it from BASH_SOURCE
# breaks the moment this file is sourced from zsh, where BASH_SOURCE is unset:
# SP then resolves somewhere plausible, every probe fails identically in `cd`,
# and the run reports a uniform rc=1 surface that looks like real data.
SP="${SP:-/private/tmp/claude-501/-Users-matts-Devel-prj-Intent/0482e68a-709f-45b1-ab98-44bc9c962bd1/scratchpad}"
WT="${WT:-$SP/wt}"
SBX="${SBX:-$SP/sandbox}"
OUT="${OUT:-$SP/probes}"
CAP="$OUT/raw"

mkdir -p "$CAP"

# Refuse to run against a missing worktree or sandbox. Without this the probes
# still "work": every one fails in `cd` or exec, and the run yields a uniform
# non-zero surface indistinguishable from a real finding.
[ -x "$WT/bin/intent" ] || { echo "probe: no intent binary at $WT/bin/intent" >&2; exit 2; }
[ -d "$SBX/intent" ]    || { echo "probe: $SBX is not an Intent project" >&2; exit 2; }

# probe <label> <cwd> <args...>
probe() {
  local label="$1"; shift
  local cwd="$1"; shift
  local o="$CAP/$label.out" e="$CAP/$label.err" rc
  ( cd "$cwd" && env INTENT_HOME="$WT" HOME="$FAKEHOME" "$WT/bin/intent" "$@" ) >"$o" 2>"$e"
  rc=$?
  local ob eb of ef
  ob=$(wc -c <"$o" | tr -d ' ')
  eb=$(wc -c <"$e" | tr -d ' ')
  of=$(head -1 "$o" 2>/dev/null | tr '\t' ' ')
  ef=$(head -1 "$e" 2>/dev/null | tr '\t' ' ')
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$label" "$rc" "$ob" "$eb" "$of" "$ef" "$*"
}

# probe_matrix <command> [subcommand]
# Four probes per unit: bare, --help, an unknown flag, and outside a project.
probe_matrix() {
  local cmd="$1" sub="${2:-}" tag
  tag="$cmd${sub:+-$sub}"
  if [ -n "$sub" ]; then
    probe "$tag.bare"    "$SBX" "$cmd" "$sub"
    probe "$tag.help"    "$SBX" "$cmd" "$sub" --help
    probe "$tag.badflag" "$SBX" "$cmd" "$sub" --zzz-not-a-flag
    probe "$tag.noproj"  "/"    "$cmd" "$sub"
  else
    probe "$tag.bare"    "$SBX" "$cmd"
    probe "$tag.help"    "$SBX" "$cmd" --help
    probe "$tag.badflag" "$SBX" "$cmd" --zzz-not-a-flag
    probe "$tag.noproj"  "/"    "$cmd"
  fi
}
