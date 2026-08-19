#!/bin/bash
# of_n_population.sh -- ENUMERATE the population AT-00.11 and AT-00.12 drive.
#
# WHY THIS EXISTS, AND IT IS A DEFECT OF MINE RATHER THAN A FEATURE REQUEST.
# AC-00.11 records `dc nominated 10 of 41 parity tools by a two-grep proxy`.
# THAT PROBE WAS NEVER COMMITTED. There is no nominating tool in this directory
# and no board history carries its text, so the only surviving artefact is the
# figure `10 of 41` in a criterion's prose. A reconstruction of the two arms
# returns 14, not 10. **The record survived and its subject did not** -- the same
# shape as a marker naming a commit but identifying no artefact, and as a proxy
# that is not the parser. Here the record is a COUNT INSIDE A CRITERION, which
# makes it AC-00.11's own third arm pointed at AC-00.11: a recorded number
# wearing a derived number's clothes.
#
# So this file replaces a remembered number with an enumerated one. Its OUTPUT
# is the population. The criterion's prose is not, and neither is any message.
#
# THE TWO POPULATIONS ARE DIFFERENT AND THE ROWS SAY SO:
#   AT-00.11  instruments whose M CAN MOVE under the relocation -- those
#             carrying an `intent/st` path shape. A two-tree differential over
#             an instrument WITHOUT one is a vacuous pass: nested and flat emit
#             byte-identical output, and an observable that cannot move is not
#             a check.
#   AT-00.12  EVERY instrument that emits an `N of M`, path shape or not.
#
# AND THE TWO TENS THAT ARE NOT THE SAME TEN. `10 gated of 17 rostered` is a
# fact about the ROSTER in runner_roster_check.sh and is the ORDERING rule
# (gated first). `10 of 41` was the NOMINATION over this whole directory. They
# name different populations and coincide only in the numeral. Five of the ten
# gated tools carry no path shape at all, so half of a `drive the gated ten`
# plan would have returned clean and meant nothing.
#
# THIS TOOL NOMINATES; IT DOES NOT ADJUDICATE. Its second arm is a regex over
# source text, and a proxy is not the parser -- of_n_labels_its_derivation.sh
# is the parser for the declarative half, and AT-00.11's differential is the
# parser for the estate half. Every count below closes over what THIS RUN
# examined, and the reach section states what the arms cannot see.
#
# Reads source text only: no worktree, no binary, no clock, no store.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROSTER_SRC="$HERE/runner_roster_check.sh"

die() {
  printf 'of_n_population: %s\n' "$1" >&2
  exit 2
}

[ -d "$HERE" ] || die "cannot resolve my own directory"
[ -f "$ROSTER_SRC" ] || die "runner_roster_check.sh is absent -- gatedness is unreadable and a population without it cannot be ordered"

# ---------------------------------------------------------------------------
# GATEDNESS, read STRUCTURALLY out of the roster rather than by line number.
#
# A line number in a durable record expires, and this estate has already
# shipped four rotted ones into a criterion. So the block is found by its
# opening `ROSTER='` and read to the closing bare quote -- which survives the
# roster growing, shrinking, or moving in the file.
# ---------------------------------------------------------------------------
roster_rows() {
  awk "
    /^ROSTER='/ { inside = 1; next }
    inside && /^'/ { exit }
    inside && NF { print \$1, \$2 }
  " "$ROSTER_SRC"
}

ROWS="$(roster_rows)"
[ -n "$ROWS" ] || die "the roster parsed EMPTY -- its format has changed under this parser, and an empty roster would silently reclassify every tool as unrostered"

gatedness_of() {
  local name="$1" cls
  cls="$(printf '%s\n' "$ROWS" | awk -v n="$name" '$1 == n { print $2; exit }')"
  [ -n "$cls" ] || cls="unrostered"
  printf '%s' "$cls"
}

# ---------------------------------------------------------------------------
# THE TWO ARMS. Both are proxies over source text and both are stated as such.
# ---------------------------------------------------------------------------

# ARM 1 -- does the instrument name the canon path shape? This is the property
# that decides whether the relocation can move its M at all.
names_path_shape() {
  grep -q 'intent/st' "$1"
}

# ARM 2 -- does the instrument emit an `N of M` verdict? A NOMINATION only:
# it matches the printf-shaped forms this estate actually writes, and a verdict
# assembled from variables across two statements is invisible to it.
emits_of_n() {
  grep -qE '(%[sd][^"]*of |[0-9]+ of |of %[sd]|of \$)' "$1"
}

# ---------------------------------------------------------------------------
# THE SWEEP. Every count printed below is incremented inside this one loop, so
# each closes over the files THIS RUN opened -- never over a glob, a filter, or
# a figure carried in from anywhere else. That is AC-00.11 applied to the tool
# that serves AC-00.11.
# ---------------------------------------------------------------------------
examined=0
globbed=0
p11_gated=""; p11_manual=""; p11_unrostered=""
p12_gated=""; p12_manual=""; p12_unrostered=""
neither=0

for path in "$HERE"/*.sh; do
  globbed=$((globbed + 1))
  [ -f "$path" ] || continue
  f="${path##*/}"
  examined=$((examined + 1))
  cls="$(gatedness_of "$f")"
  in11=0; in12=0
  names_path_shape "$path" && in11=1
  emits_of_n "$path" && in12=1
  if [ "$in11" -eq 1 ]; then
    case "$cls" in
      gated)  p11_gated="$p11_gated $f" ;;
      manual) p11_manual="$p11_manual $f" ;;
      *)      p11_unrostered="$p11_unrostered $f" ;;
    esac
  fi
  if [ "$in12" -eq 1 ]; then
    case "$cls" in
      gated)  p12_gated="$p12_gated $f" ;;
      manual) p12_manual="$p12_manual $f" ;;
      *)      p12_unrostered="$p12_unrostered $f" ;;
    esac
  fi
  [ "$in11" -eq 0 ] && [ "$in12" -eq 0 ] && neither=$((neither + 1))
done

[ "$examined" -gt 0 ] || die "no instruments examined -- an empty population and a clean sweep compare equal, so this is a refusal and not a pass"

count_of() {
  local n=0 x
  for x in $1; do n=$((n + 1)); done
  printf '%d' "$n"
}

emit_group() {
  local label="$1" items="$2" n
  n="$(count_of "$items")"
  if [ "$n" -eq 0 ]; then
    printf '  %-12s %2d  (none)\n' "$label" "$n"
  else
    printf '  %-12s %2d\n' "$label" "$n"
    printf '%s\n' "$items" | tr ' ' '\n' | grep . | sed 's/^/                  /'
  fi
}

n11=$(( $(count_of "$p11_gated") + $(count_of "$p11_manual") + $(count_of "$p11_unrostered") ))
n12=$(( $(count_of "$p12_gated") + $(count_of "$p12_manual") + $(count_of "$p12_unrostered") ))

printf 'of_n_population -- the population AT-00.11 and AT-00.12 drive, ENUMERATED\n'
printf '\n'
printf 'EXAMINED %d instrument(s) of %d matched by the glob.\n' "$examined" "$globbed"
printf '  Both operands are counted in the sweep loop below, and they are DIFFERENT\n'
printf '  counters: the glob population and the files actually opened. An N of N here\n'
printf '  means nothing was skipped, not that the ratio is decorative.\n'
printf '  THIS FILE IS IN ITS OWN POPULATION. Excluding the tool that grades the\n'
printf '  estate treats a one-file problem that this is not (ic, on the AT-00.12\n'
printf '  self-grading hazard), and it emits an N of M like everything else here.\n'
printf '\n'

printf 'AT-00.11 POPULATION -- names the `intent/st` path shape, so its M CAN move.\n'
printf '  %d of %d examined. ORDERED BY GATEDNESS, which is the adjudication order.\n' "$n11" "$examined"
emit_group "gated"      "$p11_gated"
emit_group "manual"     "$p11_manual"
emit_group "unrostered" "$p11_unrostered"
printf '\n'

printf 'AT-00.12 POPULATION -- emits an `N of M`, path shape or not.\n'
printf '  %d of %d examined. A vacuous pass under a two-tree differential when it\n' "$n12" "$examined"
printf '  carries no path shape, which is why this row is not AT-00.11 with a wider net.\n'
emit_group "gated"      "$p12_gated"
emit_group "manual"     "$p12_manual"
emit_group "unrostered" "$p12_unrostered"
printf '\n'

printf 'NEITHER ARM: %d of %d examined.\n' "$neither" "$examined"
printf '\n'

# ---------------------------------------------------------------------------
# REACH. Stated in the OUTPUT rather than left for a reader to infer from a
# pass, because a clean run over a nominated set is not a clean run over the
# estate -- and the last time that went unsaid, the horizon of the nominating
# probe travelled into the drive method and made it blind to the same eleventh.
# ---------------------------------------------------------------------------
printf 'REACH -- what these arms CANNOT see, so that no pass is read as coverage:\n'
printf '  1. ARM 2 IS A PROXY AND A PROXY IS NOT THE PARSER. It matches printf-shaped\n'
printf '     `N of M` forms. A verdict assembled across two statements, or built from\n'
printf '     variables named nothing like `of`, is invisible to it. Adjudicate with\n'
printf '     of_n_labels_its_derivation.sh, never with this count.\n'
printf '  2. ARM 1 IS A LITERAL. An instrument deriving the canon path from a variable,\n'
printf '     a config read, or a caller argument carries no `intent/st` literal and is\n'
printf '     structurally absent from the AT-00.11 column. `10 IS A FLOOR` was said of\n'
printf '     the original nomination and it is said again here, of this one.\n'
printf '  3. THE ROSTER GLOBS `*_check.sh` ONLY, so a parity tool named any other way\n'
printf '     can never hold a roster row and lands in `unrostered` by its FILENAME\n'
printf '     rather than by any judgement. interrupt_rig.sh is one, and AC-00.10 is\n'
printf '     entirely about it. `unrostered` here means UNCLASSIFIED, not `not gated`.\n'
printf '  4. THIS TOOL NOMINATES. It has driven nothing and opened no tree.\n'
printf '\n'
printf 'ok: population enumerated; take this output as the population, not a remembered count\n'
