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
set -u

# SP MUST BE PASSED IN. It is not defaulted, and the previous default is the
# reason this ordering was rewritten (ic, 2026-08-15, from vc's sweep suggestion
# after three defects in one day shared this cause).
#
# It used to read `SP="${SP:-/private/tmp/claude-501/.../<uuid>/scratchpad}"` --
# ONE HISTORICAL SESSION'S SCRATCH DIRECTORY, named by UUID, hardcoded as the
# fallback for every future run. It still resolves today only because that
# directory happens not to have been reaped yet, which is luck rather than a
# property. Deriving from BASH_SOURCE is genuinely wrong here for the reason the
# old comment gave -- this file is sourced, and BASH_SOURCE is unset under zsh,
# so SP would resolve somewhere plausible and every probe would fail identically
# in `cd`, yielding a uniform rc=1 surface that reads as real data. **But the
# answer to "cannot be derived" is REFUSE, not "default to wherever it worked
# once."** A default that points at another session's temp directory is the
# absent-input class that has now cost this estate three separate defects.
SP="${SP:?set SP -- the scratch directory this run owns. NOT defaulted: the only honest fallback is a refusal, since a probe run against the wrong root produces a complete, uniform, entirely fictional surface}"
WT="${WT:-$SP/wt}"
SBX="${SBX:-$SP/sandbox}"
OUT="${OUT:-$SP/probes}"
CAP="$OUT/raw"

# FAKEHOME IS COMPUTED HERE, AFTER SP EXISTS, AND THAT IS THE WHOLE FIX.
# It used to sit twenty-six lines above, BEFORE the SP default and three lines
# before `set -u`. With SP unset it expanded to the literal `/fakehome` and the
# run then did `mkdir -p /fakehome` -- so **the HOME isolation this file's own
# header calls its hard-won lesson was silently defeated in exactly the case the
# default existed to cover.** `set -u` would have caught it; `set -u` was on the
# next-but-two line. A guard that arrives after the statement it protects is not
# a guard. Latent rather than live, because the sole caller passes SP.
FAKEHOME="${FAKEHOME:-$SP/fakehome}"

mkdir -p "$FAKEHOME" "$CAP"

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
