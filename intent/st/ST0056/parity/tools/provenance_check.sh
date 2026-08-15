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

# SCOPE -- what this run is allowed to look at.
#
# `staged`  read every group member out of the INDEX, and check only the groups
#           this commit actually touches.
# `tree`    read the working tree, check everything (a manual run).
# `auto`    staged when anything is staged, tree otherwise.
#
# BOTH HALVES OF `staged` FIX A REAL BLOCK, and dc found them by being blocked
# (2026-08-15). This guard globbed the FILESYSTEM, so it assessed a `cmd-*.md`
# that was untracked and mid-generation in ic's tree -- and refused dc's commit,
# which touched `bin/.devbin/`, MODULES.md and dc's own board, and nothing in
# `parity/` at all. **One node's in-flight work became a commit freeze for every
# node, naming files they had never touched**, clearable only by editing someone
# else's work or by `--no-verify`. dc did neither and diagnosed instead, which is
# the only reason this is written down rather than bypassed.
#
# Reading the index fixes that: an untracked file is not in it, and neither is an
# unstaged edit. Scoping to touched groups fixes the other half -- a split that
# already exists in HEAD would otherwise block every commit by everyone until
# somebody repaired it, and **a guard that must be bypassed to work is a guard
# nobody keeps.** That rule is not new here; it is the clock guard's check C,
# which deliberately refuses only on stamps the current commit ADDS. This file
# already cited the clock guard as its model and had inherited the refusal
# without the scoping.
SCOPE="${PROV_SCOPE:-auto}"
STAGED="$(git -C "$ROOT" diff --cached --name-only 2>/dev/null)"
if [ "$SCOPE" = auto ]; then
  if [ -n "$STAGED" ]; then SCOPE=staged; else SCOPE=tree; fi
fi

# Read a group member. In `staged` mode the INDEX is the source, so a file that
# is untracked or only edited in the worktree does not participate at all.
content_of() {
  if [ "$SCOPE" = staged ]; then
    git -C "$ROOT" show ":${1#$ROOT/}" 2>/dev/null
  else
    cat "$1" 2>/dev/null
  fi
}

stamp_of() { content_of "$1" | grep -m1 -oE 'Measured at `[a-f0-9]{7,}`' | grep -oE '[a-f0-9]{7,}'; }

# ABBREVIATED SHAs ARE NOT COMPARABLE AS TEXT, and this compared them as text.
# `git rev-parse --short` chooses its length from the repo's object count, so it
# GROWS: the same commit rendered `69d42a7` on 2026-08-14 and `69d42a7f` a day
# later, and this guard reported that two artefacts "disagree about their own
# revision" while both named the same one. It misread the stamp and accused the
# artefact -- inverting the thing its own preamble says it exists to protect.
#
# dc found it and noted the part that makes it worse than a one-off: the
# crossover moves ahead of us as the repo grows, so it will land on whoever is
# committing at the time rather than on whoever caused it.
#
# Resolving through git compares commits instead of strings. An unresolvable
# stamp (a shallow clone, a commit not present) falls back to the literal text,
# so this degrades to the old behaviour rather than erroring on a tree it cannot
# fully see.
resolve() {
  local full
  full="$(git -C "$ROOT" rev-parse --verify --quiet "$1^{commit}" 2>/dev/null)"
  if [ -n "$full" ]; then printf '%s' "$full"; else printf '%s' "$1"; fi
}

# check_group <name> <why they must agree> <file>...
check_group() {
  local name="$1" why="$2"; shift 2
  local f s r first="" firstr="" firstf="" n=0 bad=0 touched=0 offenders=""

  # In `staged` mode, a group nobody touched is not this commit's business.
  if [ "$SCOPE" = staged ]; then
    for f in "$@"; do
      case "$STAGED" in *"${f#$ROOT/}"*) touched=1; break ;; esac
    done
    [ "$touched" -eq 1 ] || return 0
  fi

  for f in "$@"; do
    s="$(stamp_of "$f")"
    # No stamp AND no content means the file is simply not in scope -- untracked
    # in `staged` mode, or absent. Only a file we can READ and that carries no
    # stamp is a finding; the distinction is what stops an untracked artefact
    # being reported as unstamped.
    if [ -z "$s" ]; then
      [ -n "$(content_of "$f")" ] || continue
      echo "provenance: ${f#$ROOT/} carries no revision stamp -- an unstamped artefact cannot be checked and cannot be trusted" >&2
      rc=1; continue
    fi
    r="$(resolve "$s")"
    n=$((n + 1))
    if [ -z "$first" ]; then first="$s"; firstr="$r"; firstf="$f"; continue; fi
    if [ "$r" != "$firstr" ]; then
      # COLLECT, REPORT ONCE. Reporting inside the loop printed the same
      # sentence once per disagreeing file -- 26 identical lines for a
      # single-file mistake, with the one useful line (which file, which stamp)
      # buried among them. A guard that prints 26 lines to say one thing teaches
      # its readers to skim, which costs it the next real finding.
      offenders="$offenders
  ${f#$ROOT/} -> $s"
      bad=1; rc=1
    fi
  done
  if [ "$bad" -eq 1 ]; then
    echo "provenance: $name disagrees about its own revision" >&2
    printf '  %s -> %s (first)%s\n' "${firstf#$ROOT/}" "$first" "$offenders" >&2
    echo "  $why" >&2
  fi
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
# Scoped the same way, and for the same reason: an unassigned artefact is worth
# reporting to the node that ADDS it, and worth reporting to nobody else. The
# unscoped version was the second half of dc's block -- it globbed the worktree
# too, so an in-flight file appeared here as well.
for f in "$P"/*.md "$ROOT/surface"/*.md; do
  [ -f "$f" ] || continue
  case "$f" in
    "$P/register.md"|"$P/pertest.md"|"$P"/cmd-*.md|"$ROOT/surface/dispatch-table.md") continue ;;
  esac
  if [ "$SCOPE" = staged ]; then
    case "$STAGED" in *"${f#$ROOT/}"*) ;; *) continue ;; esac
  fi
  [ -n "$(stamp_of "$f")" ] || continue
  echo "provenance: ${f#$ROOT/} carries a stamp but belongs to no group -- assign it or state why it stands alone" >&2
  rc=1
done

[ "$rc" -eq 0 ] && echo "provenance: every measurement group names one revision."
exit $rc
