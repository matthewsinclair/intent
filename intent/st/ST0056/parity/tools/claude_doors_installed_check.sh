#!/usr/bin/env bash
# AC-07.7 -- every door the `claude` subsystem ships is reachable in an
# INSTALLED build, and a missing one reds a row rather than shipping silently.
#
# WHY THIS EXISTS AS A TOOL RATHER THAN A DRIVE IN A TRANSCRIPT
#
# The row is tag-gated: it can only be SATISFIED against an artefact that does
# not exist yet. Everything about it that is knowable today -- which doors,
# what a reachable one looks like, what the published build actually does --
# is knowable now, and re-deriving it after the tag is how a measurement gets
# taken twice and disagrees with itself. So the drive is written down and the
# post-tag check is a re-run of this file against the new keg.
#
# THE PREDICATE IS THE UNWIRED MARKER, NEVER THE EXIT CODE
#
# `claude ws list` in a project with no whiteboard exits 1 -- and that is the
# door WORKING, refusing for a real reason. An rc-based check would call it a
# failure and would have reported this estate broken in the one direction that
# looks like diligence. What distinguishes unreachable from refusing is the
# spine's own marker: rc=2 AND "is a known command that is not implemented
# yet". Anything else means an arm ran.
#
# `claude cwi` IS NOT A DOOR AND IS NOT CHECKED. `intent_claude_cwi` is the
# SCRIPT implementing `claude start` and `claude ws`; there is no `cwi` verb on
# any build, and calling one absent was a shorthand that read as a finding.
#
# SAFETY, AND IT IS NOT OPTIONAL. `claude start` invoked bare LAUNCHES A REAL
# CLAUDE CODE SESSION -- it hung a two-minute drive before this file existed.
# `claude start` is a declared member of `populations.not_probed` for that
# reason. CWI_DRY_RUN=1 is the script's own documented seam: it prints the
# argv it WOULD exec and exits. CWI_WB redirects the whiteboard root so the
# writing verbs never touch a real board. An arm that forgets either is not a
# slow test, it is a live session.
#
# Exit 0 = every door reachable in the build under test.
# Exit 1 = a door is unreachable, or the instrument could not be trusted.
set -uo pipefail
cd "$(git rev-parse --show-toplevel)" || exit 2

KEG="${KEG:-/opt/homebrew/Cellar/intent/3.0.0_1/bin/intent}"
TREE="${TREE:-$PWD/native/rust/target/release/intent}"
UNWIRED='is a known command that is not implemented yet'

# The doors, from AC-07.7's own text. The count is asserted before any verdict
# is trusted: a population that silently empties reports a clean estate.
# ORDER IS LOAD-BEARING: `new probe` runs before `start probe`, so `start`
# meets a node that EXISTS and reaches the dry-run exec. Driven against an
# absent node it refuses at rc=1, which this file's predicate scores
# "reachable" -- true, and the weakest true thing available. A refusal for
# want of its subject is close in shape to AC-07.7's own falsifier
# ("refusing for want of its own dependency"), so the arm is arranged to
# rule that reading out rather than to argue it away.
#
# A DOOR IS AN ARRAY, NOT A STRING. `ws new probe` must reach argv as TWO
# tokens, and the way to say that is `read -r -a` plus a quoted expansion --
# not an unquoted $verb leaning on word-splitting. The sibling instrument
# docs_defect_disposition_check.sh already does it this way for the same
# reason, and the shell critic is right to refuse the other spelling: under
# zsh the unquoted form does not split at all, so the tool would have driven
# a single verb named "new probe" and reported it unreachable on EVERY build.
DOORS_PRE=('list' 'new probe' 'hygiene')
DOORS_POST=('archive probe')
EXPECTED_DOORS=5

note() { printf '%s\n' "$*"; }

drive_build() {
  local label="$1" bin="$2"
  local root out rc reach=0 unreach=0 seen=0

  if [ ! -x "$bin" ]; then
    note "  $label: NO BINARY at $bin -- not a clean result, an absent instrument"
    return 2
  fi
  root=$(mktemp -d); mkdir -p "$root/home" "$root/wb"
  ( cd "$root" && git init -q . && HOME="$root/home" "$bin" init >/dev/null 2>&1 )

  note "  $label -- $("$bin" --version 2>&1)"
  local verb; local -a parts
  for verb in "${DOORS_PRE[@]}"; do
    read -r -a parts <<< "$verb"
    out=$( cd "$root" && HOME="$root/home" CWI_WB="$root/wb" \
           "$bin" claude ws "${parts[@]}" </dev/null 2>&1 )
    rc=$?
    seen=$((seen+1))
    if [ "$rc" -eq 2 ] && grep -qF -- "$UNWIRED" <<<"$out"; then
      unreach=$((unreach+1)); note "    UNREACHABLE  claude ws $verb"
    else
      reach=$((reach+1));     note "    reachable    claude ws $verb   (rc=$rc)"
    fi
  done

  # `start` through the documented dry-run seam -- never bare -- and against
  # the node `ws new` just provisioned, so the arm reaches the exec rather
  # than a not-found refusal.
  out=$( cd "$root" && HOME="$root/home" CWI_WB="$root/wb" CWI_DRY_RUN=1 \
         "$bin" claude start probe </dev/null 2>&1 )
  rc=$?
  seen=$((seen+1))
  if [ "$rc" -eq 2 ] && grep -qF -- "$UNWIRED" <<<"$out"; then
    unreach=$((unreach+1)); note "    UNREACHABLE  claude start"
  else
    reach=$((reach+1))
    note "    reachable    claude start        (rc=$rc, CWI_DRY_RUN=1)"
    note "                 would exec: $(printf '%s' "$out" | grep -i 'claude' | head -1 | cut -c1-72)"
  fi

  for verb in "${DOORS_POST[@]}"; do
    read -r -a parts <<< "$verb"
    out=$( cd "$root" && HOME="$root/home" CWI_WB="$root/wb" \
           "$bin" claude ws "${parts[@]}" </dev/null 2>&1 )
    rc=$?
    seen=$((seen+1))
    if [ "$rc" -eq 2 ] && grep -qF -- "$UNWIRED" <<<"$out"; then
      unreach=$((unreach+1)); note "    UNREACHABLE  claude ws $verb"
    else
      reach=$((reach+1));     note "    reachable    claude ws $verb   (rc=$rc)"
    fi
  done
  rm -rf "$root"

  if [ "$seen" -ne "$EXPECTED_DOORS" ]; then
    note "  $label: DENOMINATOR BROKEN -- drove $seen doors, expected $EXPECTED_DOORS"
    return 2
  fi
  note "  $label: $reach reachable / $unreach unreachable of $seen"
  [ "$unreach" -eq 0 ]
}

note "AC-07.7: the claude subsystem's doors, in an INSTALLED build"
note ""
note "SUBJECT -- the published keg, which is what a consumer installs today:"
drive_build keg "$KEG"; keg_rc=$?
note ""
note "CONTROL -- the dev tree. This arm exists so a zero above is a FINDING"
note "rather than a broken harness: an instrument that cannot report 'reachable'"
note "has not measured anything by returning 'unreachable' five times."
drive_build tree "$TREE"; tree_rc=$?
note ""

if [ "$tree_rc" -ne 0 ]; then
  note "claude-doors: FAIL -- the control build does not show the doors reachable."
  note "  Either the tree regressed or this instrument is blind. Refusing to report"
  note "  a verdict on the keg from a harness that cannot produce the other answer."
  exit 1
fi
if [ "$keg_rc" -ne 0 ]; then
  note "claude-doors: AC-07.7 IS RED, AND MEASURED RATHER THAN ASSUMED."
  note "  Every door is reachable in the tree and unreachable in the published build,"
  note "  which is AC-07.7's own falsifier met exactly: the doors ship declared and"
  note "  refusing, and no criterion in WP-07 went red when 3.0.0 was cut."
  note "  This row is satisfiable only against the NEXT published artefact. Re-run"
  note "  this file with KEG= pointing at it; do not re-derive the question."
  exit 1
fi
note "claude-doors: ok -- every door reachable in the installed build under test"
exit 0
