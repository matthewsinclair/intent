#!/usr/bin/env bash
# declared_kind_check.sh -- does a row's cited artefact match the KIND the row declared?
#
# Witness for AT-00.16, covering AC-00.15. The register requires a cited file to
# name the row that cites it, so this id is structural rather than a pointer
# into a tracker.
#
# **THE DEFECT IS NOT THAT A CITATION FAILS TO RESOLVE. IT IS THAT IT RESOLVES
# TO THE WRONG KIND OF THING.** AC-01.7 is *parses but does not resolve*; this
# is *resolves, and the artefact is not what the row said it was*. Passing the
# existence check is exactly what makes a reader stop asking what it resolved
# to -- existence is to type as correctness is to validity.
#
# **THE STALE-AT GUARD IS STRUCTURALLY BLIND TO THIS AND SO IS EVERY OTHER
# INSTRUMENT IN THE ESTATE.** `stale_at_check.sh` catches a `to-write` row
# citing a file that EXISTS. Nothing catches a row whose file exists and is a
# library, a data producer, or a Rust file with no test in it.
#
# **WHAT DECLARES THE KIND: THE CITATION PATH ITSELF.** A row does not carry a
# `kind` field, so the declaration is the shape of the path it cites --
# `parity/tools/*.sh` declares a shell instrument, `crates/*/tests/*.rs`
# declares a Rust integration test, `*.bats` declares a BATS test. That is a
# real declaration and it is checkable against the bytes at the other end.
#
# ==========================================================================
# WHAT THIS TOOL CANNOT SEE, STATED HERE BECAUSE IT IS SUBJECT TO AC-00.16
# ==========================================================================
#
# **IT CHECKS AGREEMENT AT THE DECLARED PATH AND NOWHERE ELSE.** The case that
# MOTIVATED AC-00.15 is not the case this catches, and the criterion says so in
# its own text: cc drafted `clone_carries_canon.rs` against rows declaring
# `canon_clone_completeness.sh`. Different name, different extension, different
# directory -- so the declared paths would still have been EMPTY, `to-write`
# would have stayed literally accurate, and the wrong-kind artefact would have
# sat at a path no row mentions. **That is an ORPHANED ARTEFACT, the opposite
# end of the same gate-visibility defect, and it is not this tool's subject.**
# A reader who takes a green here as *no kind defects in the estate* has read
# more than this says.
#
# It also does not judge:
#
#   - a citation whose file is ABSENT. There is no artefact to disagree with,
#     and an unwritten row is legitimate. That is AC-01.7's population.
#   - a citation OUTSIDE the parity roster's reach. `tests/conformance/*.bash`
#     and `lib/templates/hooks/*.sh` are real witnesses under no roster, so
#     this tool reports them UNJUDGED and never defaults them clean.
#   - CORRECTNESS. A `.rs` carrying `#[test]` that tests the wrong thing is the
#     right KIND and passes here. Kind is the whole claim.
#
# **READS THE COMMITTED CANON EXTRACT, NOT THE STORE, AND THE REASON IS NOT
# CONVENIENCE.** The store is SSOT and `canon_commit_check.sh` already gates
# the two agreeing, so either is sound. The extract is the one a FRESH CLONE
# has: a tool that could only run where a machine-local `.cache/intent.db`
# exists would refuse in CI and in every clone, which is the population most
# likely to carry a divergence nobody has looked at.
#
# Exit 0 clean, 1 findings, 2 refusal.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

CITES=""
while [ $# -gt 0 ]; do
  case "$1" in
    --cites) CITES="${2:-}"; shift 2 ;;
    -h|--help) sed -n '2,60p' "${BASH_SOURCE[0]}"; exit 0 ;;
    *) echo "declared-kind: unknown argument: $1" >&2; exit 2 ;;
  esac
done

command -v jq >/dev/null 2>&1 || { echo "declared-kind: jq is required" >&2; exit 2; }

# ---------------------------------------------------------------------------
# The citation list. `--cites` takes a TSV of `thread<TAB>at<TAB>status<TAB>file`
# so the arms can be driven against fixtures without mutating the estate.
#
# **THE SOURCE IS PRINTED IN THE OUTPUT.** A drive that silently ran against
# the real tree would report a success that measured nothing -- the failure mode
# a non-overridable ROOT produces elsewhere in this directory.
# ---------------------------------------------------------------------------
SOURCE_LABEL=""
if [ -n "$CITES" ]; then
  [ -f "$CITES" ] || { echo "declared-kind: no such citation file: $CITES" >&2; exit 2; }
  SOURCE_LABEL="fixture: $CITES"
  CITE_TSV="$(cat "$CITES")"
else
  CANON_DIR="$ROOT/intent/.canon/st"
  [ -d "$CANON_DIR" ] || { echo "declared-kind: no canon extract at $CANON_DIR" >&2; exit 2; }
  SOURCE_LABEL="committed canon extract: intent/.canon/st/*.json"
  # **ONE jq OVER EVERY THREAD, NOT ONE PER THREAD.** The per-file loop spawned
  # 69 processes and was most of a second; `input_filename` carries the thread id
  # that `basename` was being spawned to supply. Cost is a disposition argument,
  # so a tool asking to be gated does not get to be gratuitously slow first.
  CITE_TSV="$(
    jq -r '
      (input_filename | sub(".*/"; "") | sub("\\.json$"; "")) as $t
      | (.tests // [])[]
      | select((.file // "") != "")
      | [$t, .id, .status, .file] | @tsv
    ' "$CANON_DIR"/*.json
  )"
fi

[ -n "$CITE_TSV" ] || { echo "declared-kind: no citations found -- refusing rather than reporting a clean estate over an empty set" >&2; exit 2; }

# ---------------------------------------------------------------------------
# The roster, read from the tool that owns it. NOT a second copy: this asks
# `runner_roster_check.sh` what disposition it holds for a file, rather than
# minting a parallel list that would drift from it.
# ---------------------------------------------------------------------------
ROSTER_SRC="$ROOT/intent/st/ST0056/parity/tools/runner_roster_check.sh"
[ -f "$ROSTER_SRC" ] || { echo "declared-kind: cannot read the roster at $ROSTER_SRC" >&2; exit 2; }
ROSTER="$(sed -n "/^ROSTER='/,/^'/p" "$ROSTER_SRC" | sed '1d;$d')"
ROSTER_N="$(printf '%s\n' "$ROSTER" | awk 'NF && $1 ~ /\.(sh|bash)$/ {n++} END {print n+0}')"
[ "$ROSTER_N" -gt 0 ] || { echo "declared-kind: parsed 0 roster rows -- the parse is broken, not the estate" >&2; exit 2; }

roster_disposition() {
  printf '%s\n' "$ROSTER" | awk -v want="$1" '$1 == want { print $2; found=1; exit } END { if (!found) print "UNROSTERED" }'
}

# ---------------------------------------------------------------------------
# Arms. Each returns the DISAGREEMENT reason, or nothing when the kinds agree.
# ---------------------------------------------------------------------------
kind_disagreement() {
  local path="$1" abs="$ROOT/$1"
  case "$path" in
    intent/st/ST[0-9][0-9][0-9][0-9]/parity/tools/*.sh|intent/st/ST[0-9][0-9][0-9][0-9]/parity/tools/*.bash)
      local d; d="$(roster_disposition "$(basename "$path")")"
      case "$d" in
        UNROSTERED)        echo "declares a parity instrument; the roster does not carry it at all" ;;
        not-an-instrument) echo "declares a witness; the roster records it as not-an-instrument, which forms no verdict" ;;
      esac
      ;;
    native/rust/crates/*/tests/*.rs)
      grep -qE '#\[(tokio::)?test\]' "$abs" || echo "declares a Rust integration test; the file carries no #[test]"
      ;;
    native/rust/crates/*/src/*.rs|native/rust/crates/*/src/*/*.rs|native/rust/crates/*/src/*/*/*.rs)
      # **A ROW CITING A SOURCE FILE DECLARES A COLOCATED TEST MODULE**, which is
      # as checkable as an integration test and was UNJUDGED here until measured.
      # Leaving it unjudged would have under-reached by 6 of 11 rows while the
      # output said, truthfully, that it claimed nothing about them.
      grep -qE '#\[(tokio::)?test\]' "$abs" || echo "declares a colocated test module; the source file carries no #[test]"
      ;;
    *.bats)
      grep -qE '^@test' "$abs" || echo "declares a BATS test; the file carries no @test"
      ;;
  esac
}

judged_kind() {
  case "$1" in
    intent/st/ST[0-9][0-9][0-9][0-9]/parity/tools/*.sh|intent/st/ST[0-9][0-9][0-9][0-9]/parity/tools/*.bash) return 0 ;;
    native/rust/crates/*/tests/*.rs) return 0 ;;
    native/rust/crates/*/src/*.rs|native/rust/crates/*/src/*/*.rs|native/rust/crates/*/src/*/*/*.rs) return 0 ;;
    *.bats) return 0 ;;
    *) return 1 ;;
  esac
}

TOTAL=0; ABSENT=0; AGREE=0; DISAGREE=0; UNJUDGED=0
FINDINGS=""; UNJUDGED_ROWS=""

while IFS=$'\t' read -r thread at status file; do
  [ -n "${file:-}" ] || continue
  TOTAL=$((TOTAL + 1))
  if [ ! -e "$ROOT/$file" ]; then
    ABSENT=$((ABSENT + 1)); continue
  fi
  if ! judged_kind "$file"; then
    UNJUDGED=$((UNJUDGED + 1))
    UNJUDGED_ROWS="$UNJUDGED_ROWS
    $thread $at -- $file"
    continue
  fi
  reason="$(kind_disagreement "$file")"
  if [ -n "$reason" ]; then
    DISAGREE=$((DISAGREE + 1))
    FINDINGS="$FINDINGS
    $thread $at ($status) -- $file
      $reason"
  else
    AGREE=$((AGREE + 1))
  fi
done <<< "$CITE_TSV"

echo "declared-kind: source -- $SOURCE_LABEL"
echo "declared-kind: roster -- $ROSTER_N row(s) read from runner_roster_check.sh"
echo

if [ -n "$UNJUDGED_ROWS" ]; then
  echo "declared-kind: UNJUDGED -- a real witness under no roster this tool can read, so it claims nothing:$UNJUDGED_ROWS"
  echo
fi

# **THE PARTITION CLOSES OR THE TOOL REFUSES.** Per AC-00.12: an instrument
# that prints a partition states the sum and asserts it, rather than leaving a
# reader to add up. A remainder of +1 is as silent as a remainder of -1 is loud.
echo "declared-kind: PARTITION of the $TOTAL cited row(s) -- $ABSENT absent,"
echo "  $UNJUDGED unjudged, $AGREE agreeing, $DISAGREE disagreeing."
SUM=$((ABSENT + UNJUDGED + AGREE + DISAGREE))
if [ "$SUM" -ne "$TOTAL" ]; then
  echo "declared-kind: THE PARTITION DOES NOT CLOSE: $ABSENT + $UNJUDGED + $AGREE + $DISAGREE = $SUM against $TOTAL." >&2
  exit 2
fi
echo "  THE PARTITION CLOSES: $ABSENT + $UNJUDGED + $AGREE + $DISAGREE = $TOTAL."
echo

echo "declared-kind: REACH -- it checks agreement AT THE DECLARED PATH."
echo "  It cannot see an ORPHANED artefact at a path no row cites, which is the"
echo "  other end of the same defect and is NOT this tool's subject. It does not"
echo "  judge an absent citation (AC-01.7's population), a witness outside the"
echo "  parity roster, or whether a right-kind artefact tests the right thing."
echo

if [ "$DISAGREE" -gt 0 ]; then
  echo "declared-kind: FINDING -- $DISAGREE row(s) whose artefact is not the kind the row declared:$FINDINGS"
  echo
  echo "  A row declares its artefact's KIND by the path it cites. Repoint the row"
  echo "  at what actually satisfies it, or build the kind the row declared."
  exit 1
fi

echo "declared-kind: no cited artefact disagrees with the kind its row declared."
exit 0
