#!/bin/bash
# of_n_labels_its_derivation.sh -- AT-00.12, covering AC-00.11.
#
# MODE 2 OF THE CRITERION, AND A DIFFERENT POPULATION FROM MODE 1 (vc's ruling,
# 2026-08-18). AT-00.11 drives instruments across a nested and a flat tree and asks
# whether M follows the EXAMINED population. That reaches only instruments whose M can
# MOVE under the relocation -- the ten dc nominated by a path-shape proxy. This row's
# population is every instrument that emits an `N of M` AT ALL, path shape or not, and
# it asks a different question: IS EACH OPERAND DERIVED, AND IF NOT, IS THAT VISIBLE?
#
# WHY THE TWO CANNOT BE ONE ROW: bundled, a green over ten would stand for a
# denominator of eleven-plus -- AC-00.11's own defect committed by the row enforcing
# AC-00.11. Separated, each green means something exact.
#
# THE HAZARD THIS INSTRUMENT IS BUILT AGAINST IS ITS OWN NATURAL FAILURE MODE: THE
# CAREFUL READ. A check that greps the output for a label passes on a label attached to
# nothing. So this never asks "does it say RECORDED" -- vocabulary is the wrong tool and
# I proved that on myself today: a keyword census called `self_provenance_check.sh`
# comment-only when it states its limit at :256 in words my vocabulary did not contain.
# THE SET OF WAYS TO STATE A LIMIT IS OPEN. So this asks a CLOSED question instead:
#
#   for each operand of an emitted ratio, does its value come from a POPULATION or from
#   a LITERAL, and if from a literal, can a reader of the OUTPUT tell?
#
# THREE CLASSES, AND ONLY ONE IS A FINDING:
#
#   DERIVED      the operand resolves to a command substitution or arithmetic -- it
#                counts something, so it cannot drift from what it counts.
#   RECORDED     the operand is a numeric LITERAL. AC-00.11's third arm allows this and
#                imposes two conditions: the number is LABELLED AT THE NUMBER, and what
#                would DERIVE it is NAMED. Both are checked, and NEITHER by vocabulary --
#                a DECLARED FORM makes them closed questions, the same remedy that made
#                `interrupt_rig.sh`'s non-die refusals countable:
#
#                    ... 24 [RECORDED: a hand count; DERIVED-BY: a marker per refusal site]
#
#                AT THE NUMBER is positional and therefore mechanical: the declaration
#                must open within DECL_WINDOW chars of the digits. DERIVED-BY must carry
#                non-empty text. A missing, distant, or empty-derivation declaration is a
#                finding; the declaration's PROSE is never graded.
#   ZEROLIT      A BARE `0`. REPORTED, NEVER FAILED, AND THE REASON IS A LIMIT OF THIS
#                TOOL RATHER THAN A DISPENSATION. `canon_commit_check.sh:386` emits
#                `ADDS 0 of $total` from a branch reached ONLY when the count is zero --
#                the literal STATES a measured result, it does not record a figure.
#                Distinguishing that from a genuinely recorded zero needs the guard
#                condition, which this tool does not read. IT WAS A FALSE POSITIVE HERE
#                BEFORE THIS CLASS EXISTED, and routing it to another node as a defect
#                would have been a finding manufactured by my own reach.
#
#   LAUNDERED    THE FINDING. The operand is a variable -- so it READS as derived to
#                anyone looking at the emission -- and every assignment to it in the
#                file is a literal. A recorded number wearing a derivation's clothes.
#                This is `a label attached to nothing` in numeric form, and unlike the
#                other two it is invisible both in the output AND at the emission site.
#
# `18 of 24` WOULD NOT HAVE BEEN CAUGHT HERE, AND SAYING SO IS THE POINT. Its 24 was a
# bare literal -- RECORDED, reported, not failed -- and it WAS labelled, accurately, as
# a hand count. It was wrong for a reason no declarative check can see: the numerator
# counted a different population. THAT is mode 1's question and this row does not claim
# it. A green here is not a claim that a ratio is correct; it is a claim that no operand
# is pretending to be derived.
set -u

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGETS=""
VERBOSE=0

die() { echo "of-n-labels: $*" >&2; exit 2; }

while [ $# -gt 0 ]; do
  case "$1" in
    --verbose) VERBOSE=1; shift ;;
    -h|--help) sed -n '2,45p' "${BASH_SOURCE[0]}"; exit 0 ;;
    -*) die "unknown option: $1" ;;
    *) TARGETS="$TARGETS $1"; shift ;;
  esac
done

[ -n "$TARGETS" ] || TARGETS="$(ls "$HERE"/*.sh 2>/dev/null)"
[ -n "$TARGETS" ] || die "no instruments to examine -- \$HERE=$HERE holds no *.sh. A zero here means the reach failed, not that the estate is clean."

# ---------------------------------------------------------------------------------
# CONTRACT AND REACH, IN THE OUTPUT AND NOT IN A COMMENT, AND FIRST.
# ---------------------------------------------------------------------------------
CANDIDATE_FILES="$(printf '%s\n' $TARGETS | grep -c . || true)"
echo "of-n-labels: CONTRACT -- for every emitted \`N of M\`, is each operand DERIVED from a"
echo "  population, or a LITERAL? A literal is reported and never failed. THE FINDING IS A"
echo "  LITERAL LAUNDERED THROUGH A VARIABLE: it reads as derived at the emission site and"
echo "  in the output, and it is neither."
echo "of-n-labels: SUBJECTS -- $CANDIDATE_FILES file(s), named below before any verdict."
echo "of-n-labels: REACH -- shell only, and only a ratio EMITTED ON ONE LINE by echo/printf/say"
echo "  with numeric-ish tokens both sides of \` of \`. NOT SEEN, stated because a pass must not"
echo "  read as coverage: Rust instruments; the extensionless executables under \`bin/\`; a ratio"
echo "  assembled across two emissions or inside a helper; a printf whose format string is built"
echo "  at runtime; and any operand whose variable is assigned outside this file."
echo

# ---------------------------------------------------------------------------------
# CLASSIFY ONE OPERAND. Echoes: DERIVED | RECORDED | LAUNDERED | UNCLASSIFIABLE
# ---------------------------------------------------------------------------------
classify_operand() {
  local op="$1" file="$2" name assigns lit_only
  # strip display punctuation that is not part of the operand
  op="$(printf '%s' "$op" | sed -e 's/^[`"(\[]*//' -e 's/[`"),.:;]*$//')"
  case "$op" in
    '')            echo UNCLASSIFIABLE; return ;;
    '$(('*)        echo DERIVED; return ;;      # arithmetic over something
    '$('*)         echo DERIVED; return ;;      # command substitution
    *[!0-9]*)      : ;;                          # not a bare number -- fall through
    0)             echo ZEROLIT; return ;;       # see ZEROLIT below
    *)             echo RECORDED; return ;;      # a bare numeric literal
  esac
  case "$op" in
    '${'*|'$'*)    name="$(printf '%s' "$op" | sed -e 's/^\${*//' -e 's/[:%#-].*$//' -e 's/}.*$//')" ;;
    *)             echo UNCLASSIFIABLE; return ;;
  esac
  [ -n "$name" ] || { echo UNCLASSIFIABLE; return; }
  # every assignment to this name IN THIS FILE
  assigns="$(grep -oE "(^|[^A-Za-z_])${name}=[^ ;]*" "$file" 2>/dev/null | sed "s/^.*${name}=//")"
  [ -n "$assigns" ] || { echo UNCLASSIFIABLE; return; }
  # DERIVED if ANY assignment counts something; LAUNDERED only if EVERY one is a literal
  if printf '%s\n' "$assigns" | grep -qE '\$\(|\$\(\(|`'; then echo DERIVED; return; fi
  lit_only="$(printf '%s\n' "$assigns" | grep -vE '^"?[0-9]+"?$' | grep -c . || true)"
  if [ "${lit_only:-1}" -eq 0 ]; then echo LAUNDERED; else echo UNCLASSIFIABLE; fi
}

# ---------------------------------------------------------------------------------
# DOES A RECORDED NUMBER CARRY ITS DECLARATION, AT THE NUMBER? Echoes: OK | NODECL |
# NODERIV | DISTANT. THIS IS THE ARM vc's TWO MUTANTS ARE AIMED AT, and it exists
# because the LAUNDERED check alone would pass BOTH of them -- it never asks about the
# label at all. An instrument that only recognises the exemplar's shape is an
# exemplar-matcher wearing a criterion's name.
DECL_WINDOW=48
check_declaration() {
  local text="$1" num="$2" after pos
  case "$text" in *"RECORDED:"*) : ;; *) echo NODECL; return ;; esac
  case "$text" in *"DERIVED-BY:"*) : ;; *) echo NODERIV; return ;; esac
  after="$(printf '%s' "$text" | sed -n 's/.*DERIVED-BY:[[:space:]]*//p' | sed 's/[]"].*$//')"
  [ -n "$after" ] || { echo NODERIV; return; }
  pos="$(printf '%s' "$text" | awk -v n="$num" '{ i = index($0, n); j = index($0, "RECORDED:"); if (i == 0 || j == 0) print -1; else print (j - i) }')"
  [ "$pos" -ge 0 ] 2>/dev/null || { echo DISTANT; return; }
  [ "$pos" -le "$DECL_WINDOW" ] && echo OK || echo DISTANT
}

EXAMINED=0; RATIOS=0; FINDINGS=0; UNCLASS=0
FINDING_LINES=""; RECORDED_LINES=""; UNCLASS_LINES=""; ZEROLIT_LINES=""

for f in $TARGETS; do
  [ -r "$f" ] || { echo "of-n-labels: CANNOT READ $f -- not examined"; continue; }
  EXAMINED=$((EXAMINED + 1))
  base="$(basename "$f")"
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    ln="${hit%%:*}"; text="${hit#*:}"
    # both tokens around " of " must be numeric-ish, or it is English, not a ratio
    # SPLIT ON THE FIRST \` of \` AND TAKE THE ADJACENT TOKENS. The earlier sed did this
    # with a greedy \`.*\`, which drove the capture to EMPTY and made every ratio invisible.
    # The tool REFUSED rather than reporting a clean estate, which is the arm working.
    left="$(printf '%s' "$text"  | awk -F' of ' '{n=split($1,a," "); print a[n]}')"
    right="$(printf '%s' "$text" | awk -F' of ' '{split($2,b," "); print b[1]}')"
    case "$left"  in *[0-9]*|*'$'*) : ;; *) continue ;; esac
    case "$right" in *[0-9]*|*'$'*) : ;; *) continue ;; esac
    RATIOS=$((RATIOS + 1))
    cl="$(classify_operand "$left" "$f")"
    cr="$(classify_operand "$right" "$f")"
    row="$(printf '%s:%s  N=%s [%s]  M=%s [%s]' "$base" "$ln" "$left" "$cl" "$right" "$cr")"
    if [ "$cl" = LAUNDERED ] || [ "$cr" = LAUNDERED ]; then
      FINDINGS=$((FINDINGS + 1)); FINDING_LINES="$FINDING_LINES
    $row"
    elif [ "$cl" = UNCLASSIFIABLE ] || [ "$cr" = UNCLASSIFIABLE ]; then
      UNCLASS=$((UNCLASS + 1)); UNCLASS_LINES="$UNCLASS_LINES
    $row"
    elif [ "$cl" = ZEROLIT ] || [ "$cr" = ZEROLIT ]; then
      ZEROLIT_LINES="$ZEROLIT_LINES
    $row"
    elif [ "$cl" = RECORDED ] || [ "$cr" = RECORDED ]; then
      [ "$cl" = RECORDED ] && num="$left" || num="$right"
      d="$(check_declaration "$text" "$num")"
      case "$d" in
        OK)      RECORDED_LINES="$RECORDED_LINES
    $row  declaration OK" ;;
        NODECL)  FINDINGS=$((FINDINGS + 1)); FINDING_LINES="$FINDING_LINES
    $row  -- RECORDED with NO declaration: a literal reads as a measurement" ;;
        NODERIV) FINDINGS=$((FINDINGS + 1)); FINDING_LINES="$FINDING_LINES
    $row  -- declared RECORDED but DERIVED-BY names nothing: an exemption with no work-list" ;;
        DISTANT) FINDINGS=$((FINDINGS + 1)); FINDING_LINES="$FINDING_LINES
    $row  -- declaration is NOT AT the number: the ratio can be read without its status" ;;
      esac
    fi
  done <<INNER
$(grep -nE '(echo|printf|say)[^#]* of ' "$f" 2>/dev/null)
INNER
done

# ---------------------------------------------------------------------------------
# THE CLOSING COUNT CLOSES OVER WHAT WAS EXAMINED, NEVER OVER WHAT EXISTS.
# ---------------------------------------------------------------------------------
echo "of-n-labels: EXAMINED $EXAMINED of $CANDIDATE_FILES file(s); $RATIOS ratio(s) matched the reach above."
if [ "$EXAMINED" -eq 0 ]; then
  echo "of-n-labels: CANNOT MEASURE -- nothing was examined. A zero here is a reach failure and never a clean estate."
  exit 2
fi
if [ "$RATIOS" -eq 0 ]; then
  # THE VACUOUS-PASS ARM, EXPLICIT: a population of zero must never report all-clear.
  echo "of-n-labels: CANNOT MEASURE -- $EXAMINED file(s) examined and NOT ONE emitted a ratio this"
  echo "  reach can see. That is a statement about the matcher, not about the estate."
  exit 2
fi

# EXCEPTIONS ARE ENUMERATED, NOT SUCCESSES -- the same list as the reach statement.
[ -n "$RECORDED_LINES" ] && { echo; echo "of-n-labels: RECORDED operand(s) -- literal in the source, visible to any reader, NOT a finding:$RECORDED_LINES"; }
[ -n "$ZEROLIT_LINES" ] && { echo; echo "of-n-labels: BARE ZERO -- states a measured result rather than recording a figure; this tool cannot read the guard, so it claims nothing:$ZEROLIT_LINES"; }
[ -n "$UNCLASS_LINES" ] && { echo; echo "of-n-labels: UNCLASSIFIABLE -- this tool could not decide, so it claims nothing about them:$UNCLASS_LINES"; }

echo
if [ "$FINDINGS" -eq 0 ]; then
  echo "of-n-labels: across $RATIOS ratio(s) in $EXAMINED file(s), no operand is pretending to be"
  echo "  derived, and every recorded one is declared AT its number with a derivation named."
  echo "  THIS IS NOT A CLAIM THAT ANY RATIO IS CORRECT -- whether M closes over the examined"
  echo "  population is AT-00.11's question and is not asked here."
  exit 0
fi
echo "of-n-labels: FINDING -- $FINDINGS ratio(s):$FINDING_LINES"
echo
echo "  A LAUNDERED operand reads as derived and is not -- compute it, or emit the literal so"
echo "  a reader can see it is one. A RECORDED operand needs its declaration AT the number,"
echo "  naming what would derive it: a recorded M with no route to measured is a permanent"
echo "  exemption with no work-list."
exit 1
