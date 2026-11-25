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
# **READS THE CANON EXTRACT RATHER THAN THE STORE, AND THE REASON IS NOT
# CONVENIENCE.** The store is SSOT and `canon_commit_check.sh` already gates
# the two agreeing, so either is sound. The extract is the one a FRESH CLONE
# has: a tool that could only run where a machine-local `.cache/intent.db`
# exists would refuse in CI and in every clone, which is the population most
# likely to carry a divergence nobody has looked at.
#
# **AND IT READS THAT EXTRACT FROM THE WORKING TREE.** It reads the thread
# canon under `$ROOT` through lib_at_canon.sh (`intent/.canon/st/*.json` and
# the rows under `intent/.canon/records/*/at/`, in the vocabulary of
# `$ROOT/schema/thread.schema.json`, without which it refuses), and tests each
# artefact with `[ -e "$ROOT/$file" ]`. git enters the verdict only through
# lib_read_only.sh's id door, in two places: the door answers from a copy of
# the working tree's `intent/` that `git ls-files` lists (tracked files, and
# untracked ones git does not ignore), and when the pair cannot answer, the
# paths the commit stages (`git diff --cached`) decide whether the run passes
# unchecked or refuses. The other `git` calls in this file are in the
# attribution block at the bottom, which cannot change a count or an exit code.
#
# **THIS SENTENCE USED TO OPEN `READS THE COMMITTED CANON EXTRACT`, AND THE WORD
# `COMMITTED` WAS NAMING THE ARTEFACT CLASS -- extract rather than store -- WHILE
# EVERY LATER READER TOOK IT AS A CLAIM ABOUT A GIT READ.** Three nodes did:
# vc's board diagnosed the blast-radius defect as *this guard reads COMMITTED
# canon*, cc's board carried the same reading, and dc wrote it into the
# attribution block below on 2026-09-02. **A whole remedy option was scoped
# against it** -- vc's option (D), *let the guard see the working tree*, which
# the measurement below says is already the status quo.
#
# **MEASURED 2026-09-02 (dc), TWO-SIDED, ON A SCRATCH REPO WHERE THE COMMITTED
# AND WORKING-TREE EXTRACTS DISAGREE.** Committed citation agreeing and worktree
# citation disagreeing -> rc=1, one disagreeing. The inverse, committed
# disagreeing and worktree agreeing -> rc=0, one agreeing. **The verdict tracks
# the working tree in both directions and the committed content moves nothing.**
# The inverse arm is the one that earns it: the first arm alone passes for a
# tool that reads BOTH.
#
# **SO THE BLAST RADIUS IS THE OPPOSITE MECHANISM TO THE ONE THREE BOARDS
# RECORD.** A guard whose population is the working tree is armed by a CREATE
# that is neither staged nor committed -- vc measured exactly that on
# 2026-08-31 (*a create only has to touch disk*) before the committed reading
# took hold two days later. Narrowing this tool to committed artefacts is a real
# option and it is the OPPOSITE of what option (D) is named; it is unowned and
# not taken here.
#
# Exit 0 clean, 1 findings, 2 refusal.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# The one rule for where the canon keeps a thread and its AT rows, read by the
# canon read below and by the attribution at the end, in either mode.
# shellcheck source=lib_at_canon.sh
. "$HERE/lib_at_canon.sh" || { echo "declared-kind: cannot read lib_at_canon.sh beside this tool" >&2; exit 2; }
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"

CITES=""
while [ $# -gt 0 ]; do
  case "$1" in
    --cites) CITES="${2:-}"; shift 2 ;;
    -h|--help) awk '/^#!/ { next } /^#/ { print; next } { exit }' "${BASH_SOURCE[0]}"; exit 0 ;;
    *) echo "declared-kind: unknown argument: $1" >&2; exit 2 ;;
  esac
done

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
  # **NOT `committed`** -- this is the on-disk extract, verdict-bearing as it
  # stands in the working tree. See the header: the old label said `committed`
  # and three nodes read it as a git read.
  SOURCE_LABEL="canon extract as it stands in the WORKING TREE: intent/.canon/st/*.json and intent/.canon/records/*/at/*.json"
  # BOTH CANON FORMATS, by lib_at_canon.sh's one rule for where a thread's rows
  # are: `.tests[]` in a flat thread file, or one file per row under
  # records/<ID>/at/ in the release format (thread@4.0, ST0096 WP-05).
  # The rows are lib_at_canon.sh's one reading of the canon, the one
  # stale_at_check.sh and absent_at_check.sh use. at_canon_read reads every
  # file in one jq pass and gives each row with its thread, so this tool keeps
  # no rule of its own for which thread a row belongs to. The canon is read
  # under ROOT, the tree whose artefacts this tool judges, and a refusal says
  # so, because the lib's own words name paths under the working directory.
  # Issue Ia6e3769: a row jq could not read was no row, so the check judged
  # fewer rows than the canon holds and could pass over the one it skipped.
  # at_canon_read refuses with exit 2 on anything it cannot read, before it
  # gives any row.
  # **COST, MEASURED 2026-11-25 (dc)** with /usr/bin/time -p, 5 runs each,
  # darwin/arm64, load about 4 over 16 cores, over 657 cited rows: 2.56-2.94s
  # through the lib, against 2.71-2.88s for this tool's own single jq read
  # before it moved onto the lib (at 7bbbe4135). Both print the same partition.
  # The lib's read first made this tool 7.75-8.64s. It ran one grep for each
  # file to count the file's JSON values, and forked a subshell for every line
  # it split. It now counts in one pass and makes its tab once, which also took
  # absent_at_check.sh and stale_at_check.sh from seconds to 0.40-0.43s.
  CITE_TSV="$(
    cd "$ROOT" || { echo "declared-kind: cannot enter $ROOT to read its canon" >&2; exit 2; }
    at_thread() { :; }
    at_row() { [ -z "$5" ] || printf '%s\t%s\t%s\t%s\n' "$1" "$2" "$4" "$5"; }
    { at_canon_ready && at_canon_read "${AT_CANON_FILES[@]}"; } || {
      echo "declared-kind: the canon above is read under ROOT, $ROOT" >&2
      exit 2
    }
  )" || exit 2
fi

[ -n "$CITE_TSV" ] || { echo "declared-kind: no citations found -- refusing rather than reporting a clean estate over an empty set" >&2; exit 2; }

# ---------------------------------------------------------------------------
# WHICH CITED DIRECTORIES ARE THREADS, ASKED OF THE ID DOOR (ST0095 AC-12.1).
# A parity instrument lives at `intent/st/<thread>/parity/tools/`, and whether
# `<thread>` is a thread is the door's answer, not a pattern here, so an
# instrument under a hashed thread is judged as one under an ordinal is. Every
# candidate directory goes to the door in one call, read-only through
# lib_read_only.sh, which refuses rather than migrates this checkout's store. A
# directory the door does not answer as a thread is not an instrument's home,
# and its row is left unjudged and listed as such.
# ---------------------------------------------------------------------------
# shellcheck source=lib_read_only.sh
. "$HERE/lib_read_only.sh"
RO_ROOT="$ROOT"
RO_TAG="declared-kind"
RO_WORK="$(mktemp -d "${TMPDIR:-/tmp}/declared-kind.XXXXXX")" ||
  ro_die "no temporary directory for the id door's run, so this commit is unchecked rather than clean"
trap 'rm -rf "$RO_WORK"' EXIT
# tool_shape <path> -- the layout of a parity instrument, `intent/st/<dir>/
# parity/tools/<script>`; whether `<dir>` is a thread is the door's to say.
# Prints `<dir>` for such a path, through lib_read_only.sh's one reading of it.
tool_shape() {
  local asks dir
  case "$1" in *.sh | *.bash) ;; *) return 1 ;; esac
  asks="$(ro_st_tokens "$1")"
  [ -n "$asks" ] || return 1
  dir="${asks%%$'\t'*}"
  [ "$1" = "intent/st/$dir/parity/tools/${1##*/}" ] || return 1
  printf '%s' "$dir"
}

# The paths read exactly as the main loop below reads them, so the two cannot
# disagree on a row.
tool_paths=()
while IFS=$'\t' read -r _ _ _ file; do
  tool_shape "${file:-}" >/dev/null && tool_paths+=("$file")
done <<<"$CITE_TSV"
# What this verdict reads, for a pair that cannot answer (todo 88): the canon
# its rows come from, every file a row cites, and these tools. A commit that
# changes none of them passes unchecked, said as such; any other refuses.
RO_GOVERNS="intent/.canon/st
intent/.canon/records
intent/st/ST0056/parity/tools
$(printf '%s\n' "$CITE_TSV" | cut -f4 | awk 'NF && !seen[$0]++')"
[ "${#tool_paths[@]}" -eq 0 ] || ro_ids_for "${tool_paths[@]}"

# parity_tool <path> -- true when <path> is a script in a thread's parity tools.
parity_tool() {
  local dir
  dir="$(tool_shape "$1")" || return 1
  ro_id_is "$dir" thread
}

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
  # HERESTRING, NOT A PIPE. The awk exits on first match, so a pipe writer is
  # SIGPIPEd and bash prints `printf: Broken pipe` to stderr -- intermittently,
  # because it only fires when the roster outruns the pipe buffer. It surfaced
  # inside the commit gate on 2026-09-09 and not in a standalone run.
  # pipefail_sigpipe_check.sh is CORRECT to ignore this shape: its REACH note
  # says an exiting awk is not its subject because such exits are usually
  # deliberate. This one IS deliberate; what was not deliberate was the noise it
  # puts in every node's gate output. ROSTER is asserted non-empty above, so the
  # herestring's empty-capture-becomes-one-blank-line hazard cannot fire here.
  awk -v want="$1" '$1 == want { print $2; found=1; exit } END { if (!found) print "UNROSTERED" }' <<<"$ROSTER"
}

# ---------------------------------------------------------------------------
# Arms. Each returns the DISAGREEMENT reason, or nothing when the kinds agree.
# ---------------------------------------------------------------------------
kind_disagreement() {
  local path="$1" abs="$ROOT/$1"
  if parity_tool "$path"; then
    local d; d="$(roster_disposition "$(basename "$path")")"
    case "$d" in
      UNROSTERED)        echo "declares a parity instrument; the roster does not carry it at all" ;;
      not-an-instrument) echo "declares a witness; the roster records it as not-an-instrument, which forms no verdict" ;;
    esac
    return 0
  fi
  case "$path" in
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
  parity_tool "$1" && return 0
  case "$1" in
    native/rust/crates/*/tests/*.rs) return 0 ;;
    native/rust/crates/*/src/*.rs|native/rust/crates/*/src/*/*.rs|native/rust/crates/*/src/*/*/*.rs) return 0 ;;
    *.bats) return 0 ;;
    *) return 1 ;;
  esac
}

TOTAL=0; ABSENT=0; AGREE=0; DISAGREE=0; UNJUDGED=0
FINDINGS=""; UNJUDGED_ROWS=""; FINDING_THREADS=""

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
    FINDING_THREADS="$FINDING_THREADS$thread
"
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
  echo

  # -------------------------------------------------------------------------
  # **WHOSE DEFECT IS THIS?** A half-landed citation red-gates the WHOLE TREE,
  # including for nodes with no part in it: the citation and the roster row are
  # read from the same working tree, so whichever half lands first is visible to
  # every node while the other is not.
  #
  # **THIS COMMENT READ `reads COMMITTED canon against a WORKING-TREE roster`
  # WHEN IT LANDED AT `eff993b2`, AND THAT WAS FALSE.** Both sides are the
  # working tree; it was taken from the header's own wording rather than driven,
  # and it is corrected here with the header. The measured incident below is
  # unaffected -- it is an observation, not a mechanism.
  #
  # Measured 2026-09-02: it caught dc at 08:26Z and vc at 08:28Z, four minutes
  # apart, neither having touched ST0056's parity tools. Both then went looking
  # for a defect in their own change. **That search is the cost this block
  # exists to remove**, and it is pure attribution -- the finding above stands
  # either way, and nothing here excuses it or changes the exit code.
  # -------------------------------------------------------------------------
  echo "declared-kind: WHOSE CHANGE PUT THESE ROWS HERE -- read this before diagnosing your own:"
  DEDUPED_THREADS="$(printf '%s\n' "$FINDING_THREADS" | awk 'NF && !seen[$0]++')"
  while IFS= read -r t; do
    [ -n "$t" ] || continue
    # A thread's AT rows are their own files under records/<ID>/at/ in the
    # release format, and its thread file in the flat format. at_canon_expand
    # names them, thread file first, and the log of the row files says whose
    # change put the rows here. Since ST0096 WP-05 the thread file holds no
    # rows, so its log names the last change to the thread's cover.
    at_canon_expand "$ROOT/$AT_CANON_DIR/$t.json"
    rows=("${AT_CANON_INPUTS[@]:1}")
    [ "${#rows[@]}" -gt 0 ] || rows=("${AT_CANON_INPUTS[0]}")
    last="$(git -C "$ROOT" log -1 --format='%h  %an, %ar -- %s' -- "${rows[@]}" 2>/dev/null)"
    [ -n "$last" ] || last="(no git history readable for $t's rows: ${rows[*]#"$ROOT"/})"
    echo "    $t -- citing canon last moved at $last"
  done <<< "$DEDUPED_THREADS"

  ROSTER_REL="intent/st/ST0056/parity/tools/runner_roster_check.sh"
  ROSTER_DIRT="$(git -C "$ROOT" status --porcelain -- "$ROSTER_REL" 2>/dev/null)"
  if [ -n "$ROSTER_DIRT" ]; then
    echo "    THE ROSTER IS UNCOMMITTED IN THE SHARED TREE right now:"
    echo "      $ROSTER_DIRT"
    echo "    -- a landing is IN FLIGHT through it, and this is very likely its other half."
  else
    echo "    the roster is clean in the shared tree, so no landing is in flight through it."
  fi

  echo
  echo "  0210: a new parity file needs its BYTES, its ROSTER ROW and its RUNNER"
  echo "  DISPATCH in ONE commit, or the tree red-gates for nodes who touched none"
  echo "  of it. **If the commit named above is not yours, this is that landing in"
  echo "  flight: tell its author, do not repair it** -- two nodes fixing one"
  echo "  half-landing from opposite ends is worse than the block."
  exit 1
fi

echo "declared-kind: no cited artefact disagrees with the kind its row declared."
exit 0
