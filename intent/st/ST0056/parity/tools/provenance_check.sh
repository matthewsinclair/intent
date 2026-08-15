#!/bin/bash
# provenance_check.sh -- artefacts from ONE measurement must name ONE revision.
#
# WHY THIS IS A REFUSAL AND NOT A NOTE. On 2026-08-15 the burn sweep was re-run
# specifically to prove the measurement reproduced at `c60cdbd` -- it did,
# byte-for-byte. An hour later the register was regenerated against the main
# working tree while still being fed that baseline, and published "Measured at
# 892b88a". The DATA was byte-identical, so every row looked right, every class
# matched, the formatter was happy and the corpus check passed. Only the stamp
# was wrong, and the stamp is the one thing telling a reader which revision the
# artefact describes. `pertest.md` said `c60cdbd`; the two artefacts silently
# disagreed about their own provenance -- the exact split the re-sweep had just
# been run to disprove, reintroduced by the person who had just disproved it.
#
# It was caught by reading two stamps side by side. Nothing checked it.
#
# vc's framing, from a night in which three separate rules were broken by the
# people enforcing them: **a rule that depends on its author remembering it at
# the moment of use is not a control, it is a hope with good phrasing.** The
# things that actually worked all refused rather than reminded -- the clock
# guard, and lib_corpus.sh. This is that shape for provenance.
#
# GROUPS, NOT A GLOBAL EQUALITY, and the distinction is the whole design. Three
# independent measurements live in this directory and they are SUPPOSED to carry
# different stamps: the burn sweep, the command-surface inventory, and the
# dispatch table's observed data. A check demanding one revision across all of
# them would fail on its first run against a perfectly healthy tree, and the
# first thing anyone does with a check that cries wolf is switch it off -- which
# is the failure this whole toolchain keeps refusing.
#
# So the invariant is: artefacts produced by THE SAME RUN name the same
# revision. Adding an artefact means putting it in a group or leaving it out
# deliberately; an unlisted stamped artefact is REPORTED, never ignored,
# because silence about a file nobody assigned is how a group quietly stops
# covering what it claims to.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
P="$ROOT/intent/st/ST0056/parity"

rc=0

stamp_of() { grep -m1 -oE 'Measured at `[a-f0-9]{7,}`' "$1" 2>/dev/null | grep -oE '[a-f0-9]{7,}'; }

# check_group <name> <why they must agree> <file>...
check_group() {
  local name="$1" why="$2"; shift 2
  local f s first="" firstf="" n=0 bad=0
  for f in "$@"; do
    [ -f "$f" ] || continue
    s="$(stamp_of "$f")"
    if [ -z "$s" ]; then
      echo "provenance: ${f#$ROOT/} carries no revision stamp -- an unstamped artefact cannot be checked and cannot be trusted" >&2
      rc=1; continue
    fi
    n=$((n + 1))
    if [ -z "$first" ]; then first="$s"; firstf="$f"; continue; fi
    if [ "$s" != "$first" ]; then
      echo "provenance: $name disagrees about its own revision" >&2
      printf '  %s -> %s\n  %s -> %s\n' "${firstf#$ROOT/}" "$first" "${f#$ROOT/}" "$s" >&2
      echo "  $why" >&2
      bad=1; rc=1
    fi
  done
  [ "$n" -gt 0 ] || return 0
  [ "$bad" -eq 0 ] && printf 'ok: %-22s %s file(s) @ %s\n' "$name" "$n" "$first"
}

check_group "burn artefacts" \
  "Both are rendered from ONE sweep. A disagreement means one was regenerated against a different worktree than the baseline it was fed -- the data can be byte-identical and the provenance still wrong." \
  "$P/register.md" "$P/pertest.md"

check_group "command inventory" \
  "All cmd-*.md come from a single gen_inventory.sh run. A disagreement means the set was rebuilt piecemeal, so it describes no single state of the CLI." \
  "$P"/cmd-*.md

# Deliberately its OWN group of one: `surface/dispatch-table.md` stamps when the
# OBSERVED v2 data was measured, not when the view was last rendered. It moves
# on a re-probe and not on a re-render, so it is unrelated to the two above and
# pinning it to them would be wrong rather than strict.
check_group "dispatch-table view" "single artefact" "$ROOT/surface/dispatch-table.md"

# ANY STAMPED ARTEFACT NOT IN A GROUP IS REPORTED. A new generator that starts
# emitting stamps is exactly when this check needs to grow, and the only moment
# anyone will notice is now.
for f in "$P"/*.md "$ROOT/surface"/*.md; do
  [ -f "$f" ] || continue
  case "$f" in
    "$P/register.md"|"$P/pertest.md"|"$P"/cmd-*.md|"$ROOT/surface/dispatch-table.md") continue ;;
  esac
  [ -n "$(stamp_of "$f")" ] || continue
  echo "provenance: ${f#$ROOT/} carries a stamp but belongs to no group -- assign it or state why it stands alone" >&2
  rc=1
done

[ "$rc" -eq 0 ] && echo "provenance: every measurement group names one revision."
exit $rc
