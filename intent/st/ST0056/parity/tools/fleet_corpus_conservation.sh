#!/usr/bin/env bash
#
# fleet_corpus_conservation.sh -- AT-10.5, covering AC-10.5. Shared with AT-00.2.
#
# **AC-10.5: the fleet corpus -- Intent canary, then Lamplight/Utilz/Baize at
# named post-sweep revisions -- satisfies artefact conservation, semantic
# completeness and prose conservation, OR NAMES ITS RESIDUE.** Four members. The
# disjunction is the whole criterion: a member that loses nothing passes, and so
# does a member that loses something AND SAYS WHAT.
#
# ==========================================================================
# WHY THIS FILE EXISTS AT ALL, AND IT IS A REPAIR OF MY OWN ERROR
# ==========================================================================
#
# The row cited `native/rust/crates/intentsvcs/tests/fleet_corpus_ingest.rs`,
# **a file that has never existed**, and sat at `to-write` where an absent
# citation is lint-exempt. On 2026-08-31 vc moved it to `red` to correct a note
# whose load-bearing sentence had become false -- and `red` REQUIRES the cited
# artefact to exist, so the move turned a lint-exempt row into a live
# `absent_at_check` finding that refuses every node's commit.
#
# **THERE IS NO VERB THAT SETS A ROW BACK TO `to-write`.** `at` ships
# green/red/na; Machine 5's table declares `(any) -> to-write` via `at.set` and
# the CLI exposes no spelling for it, and `at new --status to-write` refuses a
# taken id by design. So the move was IRREVERSIBLE THROUGH THE TOOL -- issue
# 0033's class (a field the typed API cannot reach) arriving operationally
# rather than documentarily.
#
# **THE REPAIR IS THE ARTEFACT THE ROW WAS ALWAYS OWED, NOT A LABEL CHANGE.**
# Re-citing to an existing instrument was refused correctly: the contract
# requires the cited file to carry the literal row id, and `conservation_check.sh`
# is ic's and carries theirs. So this file is written, carries `AT-10.5`, and is
# a SHELL instrument rather than a Rust test on purpose -- **every red row in
# this estate cites a shell tool, because a red Rust test breaks `cargo test`
# for everyone and a red shell tool does not.**
#
# ==========================================================================
# WHAT IT DOES, AND WHY IT IS RED
# ==========================================================================
#
# It does NOT re-measure conservation -- `conservation_check.sh` does that and
# there is one of it. This asks the question that tool cannot: **HAS EVERY
# NAMED CORPUS MEMBER BEEN RUN, AND DID EACH ONE CONSERVE OR NAME?** A per-member
# verdict is not a fleet verdict, and nothing else in the estate joins them.
#
# **A MEMBER WITH NO RECORDED RUN IS UNRUN, NEVER PASSING.** That is the entire
# discipline here: absence of a finding is not a finding of absence, and a fleet
# criterion scored over the one member somebody happened to run is `AC-00.11`'s
# wrong-M with estates as the population.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

# THE MEMBERS ARE DECLARED BY THE CRITERION, NOT BY WHAT IS ON THIS DISK.
# Deriving them from present directories would make the denominator a function
# of the operator's machine -- the failure this file exists to refuse.
MEMBERS="canary lamplight utilz baize"

# Recorded runs live beside the tool, one file per member, written by whoever
# drives that member. Absent = unrun.
RUNS="${RUNS:-$HERE/fleet-runs}"

echo "AT-10.5 -- fleet corpus conservation, over the members AC-10.5 NAMES"
echo ""

run=0; unrun=0; conserved=0; named=0; lost=0; refused=0
unrun_list=(); refused_list=(); bare_refusal=()

for m in $MEMBERS; do
  f="$RUNS/$m.verdict"
  if [ ! -f "$f" ]; then
    unrun=$((unrun + 1)); unrun_list+=("$m")
    printf '    %-12s UNRUN      no recorded verdict at %s\n' "$m" "${f#"$ROOT"/}"
    continue
  fi
  run=$((run + 1))
  v="$(head -1 "$f" | tr -d '\r')"
  case "$v" in
    conserved*) conserved=$((conserved + 1)); printf '    %-12s CONSERVED  %s\n' "$m" "$v" ;;
    named*)     named=$((named + 1));         printf '    %-12s NAMED      %s\n' "$m" "$v" ;;
    # **THE FOURTH VERDICT** (vc, 2026-08-31, on cc's measurement). The migrator
    # REFUSED this member, so there is no migration -- and conservation is a
    # property OF a migration. With none it is not false, it is UNDEFINED.
    #
    # Both spellings this replaces are lies in opposite directions. `conserved`
    # is a counter that cannot fire reading as a counter that found nothing --
    # AC-00.11's wrong-M, and the exact shape of ic's DOUBLED zero. `lost`
    # asserts the migrator destroyed data when it correctly declined, inverting
    # a working refusal into a failure.
    #
    # **A REFUSAL MUST CARRY ITS REASON OR IT IS A SILENT EXCLUSION**, which is
    # the denominator attack wearing this verdict's clothes. The reason also
    # carries whether the exclusion is permanent (utilz: below the migration
    # floor, unfixable at its pin) or REMEDIABLE (lamplight: 6 repairable rows),
    # and collapsing those two would turn a temporary block into a standing one.
    refused*)
      refused=$((refused + 1)); refused_list+=("$m")
      [ "${v#refused}" = "" ] && bare_refusal+=("$m")
      printf '    %-12s REFUSED    %s\n' "$m" "$v" ;;
    *)          lost=$((lost + 1));           printf '    %-12s LOST       %s\n' "$m" "$v" ;;
  esac
done

total=$((run + unrun))
scored=$((total - refused))
echo ""
echo "    partition: ${run} run + ${unrun} unrun = ${total} of 4 declared member(s)"
[ "$total" -eq 4 ] || { echo "error: partition does not close over the declared members" >&2; exit 2; }
echo "    of the run: ${conserved} conserved + ${named} named-its-residue + ${lost} lost-and-unnamed + ${refused} refused = ${run}"
[ $((conserved + named + lost + refused)) -eq "$run" ] || { echo "error: verdict partition does not close" >&2; exit 2; }

# **THE DENOMINATOR MOVES IN THE OUTPUT, AND THAT IS THE WHOLE DIFFERENCE
# BETWEEN A DECLARED EXCLUSION AND THE DENOMINATOR ATTACK** (vc, 2026-08-31).
# A silent drop from four members to two IS the attack. A drop the reader is
# told about, with each excluded member naming why, is the cure -- and the
# difference is entirely whether the reader is told.
echo "    conservation is claimed over ${scored} of 4 -- the other ${refused} are REFUSED by the migrator and named above"
if [ "${#bare_refusal[@]}" -gt 0 ]; then
  echo "error: refused without a reason (${bare_refusal[*]}) -- an exclusion that does not say why is a silent narrowing of the corpus" >&2
  exit 2
fi

cat <<'REACH'

REACH, in the output because a limit not in the output is not a limit the
reader has:
  COVERS      whether every member AC-10.5 names has a recorded verdict, and
              what that verdict was.
  DOES NOT    measure conservation. `conservation_check.sh` is the one home for
              that and this joins its per-member results; it does not repeat
              them, and a bug there is invisible here.
  DOES NOT    verify that a recorded verdict was honestly produced. It reads a
              file a human or a driver wrote. That is a real hole and it is
              smaller than the one it closes -- today the fleet figure is
              produced by nobody at all.
  DOES NOT    know whether a `named` residue was named BY THE MIGRATION or only
              by the check. It reads the verdict's own word. Both recorded
              `named` members say in their own text that the migration did NOT
              name it, which is why `named` does not pass below.
  UNOWNED     an UNRUN member is unrun and nothing here estimates it: a fleet
              criterion scored over the members somebody happened to run is
              AC-00.11's wrong-M with estates as the population.
REACH
echo ""

# **EACH RED CONDITION NAMES ITSELF, BECAUSE THE OLD MESSAGE EXPLAINED ONLY ONE
# OF THEM.** With every member recorded it printed "0 of 4 members have never
# been run ()" -- an empty list and a count of zero, while the actual cause was
# elsewhere. A verdict that misreports its own reason is worse than a bare exit
# code, because it sends the reader at the wrong thing.
#
# **AND `named` DOES NOT PASS.** The criterion is a disjunction -- conserve, OR
# name the residue -- and the namer it means is the MIGRATION. Both recorded
# `named` verdicts say in their own text that the migration did not name it
# (canary: "unnamed by the migration"; baize: "the migration does not name it"),
# so the second arm fails on both. If a member ever arrives whose MIGRATION
# named its residue, that is a genuinely different verdict and needs its own
# token; it would be indistinguishable from these two today, and passing it on
# the strength of a shared word is how a criterion gets satisfied by vocabulary.
red=()
[ "$unrun" -gt 0 ] && red+=("${unrun} member(s) never run (${unrun_list[*]-}) -- unrun is never passing")
[ "$lost" -gt 0 ] && red+=("${lost} member(s) lost artefacts without naming them")
[ "$named" -gt 0 ] && red+=("${named} member(s) carry residue the MIGRATION does not name -- issue 0183, and it blocks every member rather than any one of them")
[ "$conserved" -ne "$scored" ] && red+=("only ${conserved} of the ${scored} scored member(s) conserved outright")

if [ "${#red[@]}" -gt 0 ]; then
  echo "RED, on ${#red[@]} count(s):"
  for r in "${red[@]}"; do echo "  - $r"; done
  echo ""
  echo "THE CANARY'S STATE IS RECORDED AND IS NOT THE CRITERION: artefact conservation"
  echo "clean (STRANDED 0, ALTERED-ATTACHMENT 0, both shown able to fire by planted"
  echo "mutations), and prose residue TWO -- one defect, two census rows, issue"
  echo "CLOSED/0059 carrying two \`## Related\` sections with different authored bodies"
  echo "where canon holds one. The migration names 168 dispositions and does NOT name"
  echo "those two, so on this member NEITHER disjunct of AC-10.5 holds."
  exit 1
fi
echo "PASS -- every declared member ran; the ${scored} scored member(s) all conserved, and the ${refused} refused one(s) are excluded with their reasons named."
exit 0
