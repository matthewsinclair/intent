#!/usr/bin/env bash
# vocab_adequacy_check.sh -- AT-00.14, covering AC-00.13.
#
# THE CRITERION SAYS THIS IS NOT DECIDABLE AND IT IS RIGHT, SO READ THE REACH
# BLOCK BEFORE THE VERDICT. AC-00.13: "Membership is checkable and adequacy is
# not, so the check that exists is the one that cannot see the defect." An
# instrument claiming to measure whether a vocabulary CAN EXPRESS the states in
# use would be claiming the thing the criterion says is unavailable.
#
# WHAT IS DECIDABLE IS ONE SIGNATURE OF THE DEFECT, AND IT IS THE SIGNATURE ALL
# THREE OF THE CRITERION'S INSTANCES LEFT BEHIND: when a vocabulary cannot hold
# a unit's true state, the author writes the truth in PROSE beside the enum and
# puts the nearest available value in the enum. The prose is the estate's own
# admission. So this checks for a row whose DECLARED state is contradicted by
# its OWN adjacent prose -- never for adequacy in general.
#
# AND THE DIRECTION IS PART OF THE SIGNATURE. AC-00.13's corollary: "THE WRONG
# VALUE IS ALWAYS THE ONE THAT READS AS MORE FINISHED, NEVER LESS." That is why
# nothing catches these by review -- the row looks done -- and why the membership
# checker returns green: membership holds. It is not a false claim, it is a true
# claim about the wrong question.
#
# THE REMEDY FOR A DANGLING VALUE IS THE WRONG REMEDY FOR AN INEXPRESSIBLE ONE,
# which the criterion states and this estate has already paid for once. Do not
# "fix" a finding here by re-anchoring the enum to match the prose: that
# certifies the unfinished state as finished, which is the defect, committed by
# its own remedy. The fix is a vocabulary that can hold the state.
#
# REACH, IN THE OUTPUT AND NOT ONLY HERE (AC-00.10, and `rig_selftest.sh:60`'s
# rule that a limit not in the output is not a limit the reader has):
#   * IN REACH -- a declared machine-read state sitting beside authored prose in
#     one object, where the prose says that state is not reached.
#   * OUT OF REACH -- a vocabulary inadequate with NO prose beside it. That is
#     the commoner and worse case and this instrument is blind to it by
#     construction: there is nothing to compare. It is not a residual to be
#     closed later by widening a regex.
#   * OUT OF REACH -- inadequacy the author never noticed. The signature IS the
#     author noticing and writing it down.
#   * The hedge vocabulary is OPEN and undercounts. A row saying its state is
#     unreached in words this does not carry reads as clean, and the false
#     negatives are the rows whose authors phrased it in their own words --
#     `self_provenance_check.sh`'s finding, which this instrument inherits
#     rather than escapes.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

TABLE="${TABLE:-$ROOT/surface/dispatch-table.json}"
MODE=live
while [ $# -gt 0 ]; do
  case "$1" in
    --table) TABLE="$2"; shift 2 ;;
    --fixtures) MODE=fixtures; shift ;;
    -h|--help) sed -n '2,40p' "${BASH_SOURCE[0]}"; exit 0 ;;
    *) echo "vocab-adequacy: unknown argument: $1" >&2; exit 2 ;;
  esac
done

die() { echo "vocab-adequacy: $*" >&2; exit 2; }
command -v jq >/dev/null 2>&1 || die "jq is required and is not on PATH"

# THE PAIR IS (declared state, authored prose) IN ONE OBJECT, and both halves
# are named rather than guessed: a pair assembled across objects would compare a
# state to prose about something else, which is the subject-mismatch defect
# AC-00.14 names in its fourth instance.
#
# `.rulings[].state` is the machine-read half (ic, 2026-08-22, `1e0a4722`);
# `.ratification` is the authored half the same migration created, whose commit
# message says of it: NOTHING PARSES IT. This instrument is the first reader.
pairs() {
  jq -r '
    [paths(type == "object") as $p | getpath($p) as $o
     | select(($o | type) == "object" and ($o | has("rulings")) and ($o | has("ratification")))
     | { at: ($p | map(tostring) | join(".")),
         declared: [$o.rulings[]?.state] | unique | join("+"),
         prose: ($o.ratification | tostring) }]
    | .[] | [.at, .declared, .prose] | @tsv
  ' "$1"
}

# THE PROSE MUST SAY THIS ROW'S OWN RATIFICATION IS UNREACHED, AND PROXIMITY IS
# WHAT MAKES THAT A DIFFERENT QUESTION FROM "THE WORD APPEARS SOMEWHERE".
#
# MEASURED WHILE BUILDING THIS, AND BOTH DIRECTIONS COST: a bare
# `outstanding|pending|awaiting` over the whole field returned 10 hits of which
# ONE was real -- seven were one boilerplate string repeated across rows and two
# were HISTORY (`it was pending-hv, which was honest and is now answered`,
# describing a state the row has LEFT). Tightening it with a fixed 70-character
# leading context then DROPPED the one true positive, whose hedge sits 50
# characters into the field. A loose pattern banked nine falsehoods and the
# tight one banked a clean sweep; neither would have been noticed without a
# prediction to score against.
#
# So the subject is bound to the hedge rather than to the field: the words
# `ratification` or `ruling` must be what is outstanding.
#
# AND A NEGATION IS NOT A HEDGE, WHICH COST A THIRD REWRITE. The subject-bound
# pattern above predicted 1 and returned 8: seven rows carry `hv ratification is
# therefore NOT outstanding -- it predates the question`, and a window wide
# enough to bind the subject to the hedge is wide enough to span the `NOT` that
# reverses it. **The instrument read a row SAYING IT IS FINE as a row reporting
# a problem** -- and it is one boilerplate string on seven rows, so a single
# unnoticed sentence becomes seven findings. The prediction is the only thing
# that caught it; the run is well-formed and internally consistent at 8.
hedged() {
  printf '%s' "$1"     | grep -oiE '(ratification|ruling)[^.]{0,40}(is )?(still )?(outstanding|pending|awaiting|not yet)|(outstanding|pending|awaiting)[^.]{0,20}(ratification|ruling)'     | grep -qivE '\b(not|no longer|never|isn.t|nothing)\b'
}

# AND HISTORY IS NOT A HEDGE. A row narrating a state it has left is the
# opposite of this defect -- it is the record working.
is_history() {
  printf '%s' "$1" | grep -qiE '(was|were) `?(pending|provisional|outstanding)|is now answered|-> `?corrected'
}

HITS=0
scan() {
  local table="$1" label="$2" hits=0 rows=0 skipped=0
  while IFS=$'\t' read -r at declared prose; do
    [ -n "$at" ] || continue
    rows=$((rows + 1))
    case "$declared" in
      *ratified*) ;;
      *) skipped=$((skipped + 1)); continue ;;
    esac
    # ARM TWO -- A STATE MAPPING TO TWO VALUES, which AT-00.14 names as the
    # second mutant and which the model can express: `rulings` is an ARRAY (it
    # had to be -- one row carries two rulings) so a row can declare `ratified`
    # AND `provisional` about itself and both sit in the machine-read half.
    # MEASURED: five rows carry two rulings today and all five AGREE, so this
    # arm is driven on a fixture rather than on the estate, and its live zero is
    # a real zero rather than an arm that has never fired.
    if [ "${declared#*+}" != "$declared" ]; then
      hits=$((hits + 1))
      printf 'FINDING\t%s\tdeclared=%s\t%s\n' "$at" "$declared" \
        "two distinct declared states on one row -- the row says two things about itself and the vocabulary cannot say which"
      continue
    fi
    # ARM ONE -- A STATE MAPPING TO NO VALUE: the truth is in the prose because
    # the enum could not hold it.
    if hedged "$prose" && ! is_history "$prose"; then
      hits=$((hits + 1))
      printf 'FINDING\t%s\tdeclared=%s\t%s\n' "$at" "$declared" \
        "$(printf '%s' "$prose" | grep -oiE '.{0,50}(ratification|ruling)[^.]{0,40}(outstanding|pending|awaiting|not yet)[^.]{0,30}' | head -1)"
    fi
  done < <(pairs "$table")
  printf '%s\t%s\t%s\t%s\n' "$label" "$rows" "$skipped" "$hits" >&2
  # THE COUNT LEAVES BY A GLOBAL AND THE FINDINGS LEAVE BY STDOUT, because they
  # were both on stdout in the first version and `$(scan ...)` captured the
  # findings INTO the count -- so the contradicting fixture scored "FINDING...1"
  # against a wanted "1" and the arm reported a miss it had not made. Caught by
  # the fixtures, which is what they are for.
  HITS="$hits"
}

if [ "$MODE" = fixtures ]; then
  # THE FIXTURES DRIVE BOTH VERDICTS, because an arm that has only ever
  # returned its expected answer on the live estate has not been driven --
  # AC-00.10, and the two defects above are what it looks like when it has not.
  tmp="$(mktemp -d)"
  trap 'rm -rf "$tmp"' EXIT

  cat > "$tmp/agreeing.json" <<'JSON'
{ "invariants": [ { "id": "FIX-GREEN", "target": {
  "state": "corrected",
  "rulings": [ { "state": "ratified", "authority": "hv", "date": "2026-01-01", "record": "deadbeef" } ],
  "ratification": "hv ratified this outright at the bounce. Nothing is outstanding." } } ] }
JSON

  cat > "$tmp/contradicting.json" <<'JSON'
{ "invariants": [ { "id": "FIX-RED", "target": {
  "state": "corrected",
  "rulings": [ { "state": "ratified", "authority": "vc", "date": "2026-01-01", "record": "deadbeef" } ],
  "ratification": "vc-ruled under the standing grant. hv ratification outstanding, NOT blocking." } } ] }
JSON

  cat > "$tmp/ambiguous.json" <<'JSON'
{ "invariants": [ { "id": "FIX-TWO", "target": {
  "state": "corrected",
  "rulings": [ { "state": "ratified", "authority": "hv", "date": "2026-01-01", "record": "deadbeef" },
               { "state": "provisional", "authority": "vc", "date": "2026-01-02", "record": "cafebabe" } ],
  "ratification": "Two rulings, disagreeing about how settled this is." } } ] }
JSON

  cat > "$tmp/history.json" <<'JSON'
{ "invariants": [ { "id": "FIX-HISTORY", "target": {
  "state": "corrected",
  "rulings": [ { "state": "ratified", "authority": "vc", "date": "2026-01-01", "record": "deadbeef" } ],
  "ratification": "It was `pending-hv`, which was honest and is now answered." } } ] }
JSON

  fail=0
  scan "$tmp/agreeing.json" fixture-agreeing 2>/dev/null >/dev/null; g="$HITS"
  scan "$tmp/contradicting.json" fixture-contradicting 2>/dev/null >/dev/null; r="$HITS"
  scan "$tmp/ambiguous.json" fixture-ambiguous 2>/dev/null >/dev/null; a="$HITS"
  scan "$tmp/history.json" fixture-history 2>/dev/null >/dev/null; h="$HITS"
  echo "fixtures: agreeing=$g (want 0), maps-to-none=$r (want 1), maps-to-two=$a (want 1), history=$h (want 0)"
  [ "$g" = 0 ] || { echo "vocab-adequacy: a row whose prose AGREES was reported"; fail=1; }
  [ "$r" = 1 ] || { echo "vocab-adequacy: a row whose prose CONTRADICTS was not reported"; fail=1; }
  [ "$a" = 1 ] || { echo "vocab-adequacy: a row declaring TWO states was not reported (AT-00.14 mutant ii)"; fail=1; }
  [ "$h" = 0 ] || { echo "vocab-adequacy: a row NARRATING a state it has left was reported"; fail=1; }
  if [ "$fail" = 0 ]; then
    echo "vocab-adequacy: 4 fixture(s), both AT-00.14 mutants driven (maps-to-none, maps-to-two) plus a negative control and a history control"
    echo "vocab-adequacy: the history control is the one that would otherwise bank two falsehoods, and the negative control is what stops four passes being one method used four times"
    exit 0
  fi
  exit 2
fi

[ -f "$TABLE" ] || die "no dispatch table at $TABLE"
scan "$TABLE" live
hits="$HITS"

cat <<'REACH'
vocab-adequacy: REACH -- this finds ONE SIGNATURE of an inadequate vocabulary:
vocab-adequacy:   a declared state contradicted by the authored prose beside it.
vocab-adequacy:   It does NOT measure adequacy, which AC-00.13 states is not
vocab-adequacy:   checkable. A vocabulary inadequate with no prose beside it, or
vocab-adequacy:   one whose author never noticed, is invisible here BY
vocab-adequacy:   CONSTRUCTION -- not a residual to close by widening a regex.
REACH

if [ "$hits" -eq 0 ]; then
  echo "vocab-adequacy: no declared state is contradicted by its own prose"
  echo "vocab-adequacy: and a clean run is worth what the reach block says it is worth, which is less than it looks"
  exit 0
fi

echo "vocab-adequacy: $hits row(s) declare a state their own prose says is not reached"
echo "vocab-adequacy: DO NOT re-anchor the enum to match the prose -- that certifies the unfinished"
echo "vocab-adequacy:   state as finished, which is AC-00.13's stated corollary: the remedy for a"
echo "vocab-adequacy:   DANGLING value is the wrong remedy for an INEXPRESSIBLE one."
exit 1
