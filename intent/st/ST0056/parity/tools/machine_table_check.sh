#!/usr/bin/env bash
# machine_table_check.sh -- does `transitions.rs` implement the ratified state
# machines EXACTLY as `data-model.md` declares them?
#
# ==========================================================================
# WHY THIS EXISTS: THE PAIR WAS ONE-WAY
# ==========================================================================
#
# `data-model.md` ratifies four state machines as tables. `transitions.rs`
# carries them "transcribed" -- its own word, in four comments. A third copy in
# `mutation_completeness.rs` is asserted against the second, so hops 2->3 and
# 3->code are all machine-guarded. **Hop 1 -- the ratified table into the Rust
# -- was guarded by nothing, and every reference to the document in the entire
# source tree is a PROSE CITATION inside a comment.**
#
# So drift could only run one way: toward the RATIFIED DOCUMENT being the half
# that goes wrong while the code looks right, with no instrument able to see
# it. It ran. hv ruled the `Fiat` exits on 2026-08-29, the implementing commit
# (`b7a3e771`) put them in `transitions.rs`, the table sat unchanged, and dc
# found the gap BY EYE and flagged it in prose. Nothing else could have.
#
# **AND THE TWO HALVES DISAGREED ABOUT WHICH WAS THE AUTHORITY, which is why
# the drift had no owner.** The document's status line says the Rust type layer
# is the authored master; `transitions.rs` says the ratified machine is
# data-model.md. Both sides were reading their own file honestly. For the
# MACHINES the document is the authority, and that is now written on the page.
#
# hv ruled this instrument on 2026-08-29 13:16Z, over the alternative of
# generating the Rust from the table -- which lost on measured cost: all four
# existing generators here emit JSON or markdown and none emits Rust, the
# markdown formatter is a second writer on the input, and the page keeps its
# ratification prose either way so it never was one home.
#
# ==========================================================================
# THE COUNTING TRAP, WHICH IS THE WHOLE DIFFICULTY
# ==========================================================================
#
# **THE TWO NOTATIONS DO NOT COUNT THE SAME THINGS.** The document writes one
# row per `(from, to)` pair. `transitions.rs` writes one `Edge::` per
# verb-and-landing with a LIST of from-states, so `ac.descope` is THREE rows
# there and ONE edge here. The AC machine is 14 transition rows plus 2 entry
# rows in the document, and 10 `Edge::` declarations in the Rust.
#
# **Both counts are correct and they never match.** Any comparison that counts
# before it expands reports a permanent false divergence -- which is a check
# that reds forever, and an operator learns to ignore exactly that. EXPAND BOTH
# SIDES TO `(from, to, verb)` TUPLES FIRST, THEN COMPARE SETS.
#
# ==========================================================================
# THREE AXES, AND ONLY TWO OF THEM CAN GATE
# ==========================================================================
#
#   A  ENTRY   the document's `_(none)_ -> X` rows against `initial: &[...]`
#   B  EDGES   `(from, to, verb)` as sets, expanded on both sides
#   C  GUARDS  the document's Guard column against `&[Guard::...]`
#
# **A and B are mechanical: both sides are a controlled vocabulary and the
# normalisation is total.** They gate.
#
# **C IS NOT, AND SAYING SO IS THIS FILE'S SECOND FINDING.** The document's
# Guard column is FREE PROSE, and in Machine 3 it does not hold preconditions
# at all: `clears evidence (cc built this)` is an EFFECT, `**non-test** --
# lands on entry state` is a LANDING RULE naming which of two rows you are
# reading. Neither is a guard, and the code's actual guards for those rows are
# undeclared in the document. So axis C REPORTS and never gates.
#
# **AND AN UNMAPPABLE CELL IS COUNTED SEPARATELY FROM AN AGREEING ONE, WHICH IS
# THE POINT OF THE AXIS.** Defaulting unmappable prose to "no guard" would make
# a genuinely missing guard read as clean -- a true green from an instrument
# that could not have answered, which is this estate's dominant failure class.
# `UNMEASURED` is a third verdict here, printed as loudly as a finding.
#
# ==========================================================================
# THE POPULATION IS DECLARED, NEVER TRAVERSED
# ==========================================================================
#
# `MACHINE_MAP` names the `(machine number, entity, field)` joins, and
# `UNTABLED` names the `Disposition::State` fields the document deliberately
# does NOT table, each with the document's own reason. **Neither count is
# written in this prose**: the lists are the population, and a number beside
# them is a second home that drifts silently toward reading as complete. The
# ratified document carried exactly that and said FIVE state fields after
# there were six.
#
# **Between them they assert the population exactly.** A new `### Machine N`
# with no join is a refusal, not a skip; a new `Disposition::State` field in
# neither list is a FINDING, because the document implying a complete set when
# it was not is the defect that produced the "Two more state fields exist"
# section in the first place. An omission and a gap are indistinguishable to
# the next reader unless somebody makes the difference cost something.
#
# ==========================================================================
# EXIT CODES AND POSTURE
# ==========================================================================
#
#   0  clean, or axis-C findings only
#   1  an axis-A or axis-B divergence, or an undeclared machine or field
#   2  cannot measure
#
# **THIS ONE GATES.** The whole reason it exists is that the only thing
# standing between a ruled machine change and a silently stale ratified table
# was one peer noticing by eye. A report-only posture rebuilds that.
#
# **A BACKTICK STANDING IN FOR AN APOSTROPHE INSIDE A SINGLE-QUOTED `printf` IS
# LOAD-BEARING, NOT A LEAK.** ic found the general class in Rust doc comments on
# 2026-08-30 -- backticks doing an apostrophe's job, two of which pair and render
# everything between them as inline code -- and it is a real defect there. dc
# applied the same fix to the findings prose BELOW and broke this file: a shell
# single-quoted string cannot contain an apostrophe, so `the document's` ends the
# string and the next `(` is a syntax error 8 lines later, pointing at a line
# nobody touched. **The rule transfers to the artefact and not to the quoting
# context.** These strings are reworded to drop the possessive rather than
# escaped, so neither the wart nor the hazard is left for the next reader.
#
# `-e` is deliberately off: this is a findings script that must reach its
# verdict line, and a non-zero from a `grep -c` mid-count would otherwise end
# the run with no output at all -- a silent pass wearing a crash.
#
# **THE GATING INPUTS ARE READ FROM THE INDEX, NEVER THE WORKING TREE.** Four
# sessions work one checkout; reading either file off disk means refusing a
# commit over a peer's half-typed edit to a file the committer never touched.
# That has happened four times in this directory. `lib_staged.sh` is the one
# home for the mechanism.
# ==========================================================================
# MUTATION PROOFS, run 2026-08-29 at `021ba259` (vc)
# ==========================================================================
#
# Co-located because a check whose failure path has never fired is a claim, not
# an instrument -- and this one GATES, so it prints a verdict on every commit by
# four nodes. Re-run with the `DOC=` / `RS=` overrides this file already reads
# against copies; nothing else is needed.
#
#   control                                  -> 0, agree exactly
#   the two `Fiat` rows dropped from Machine 3
#                                            -> 1, CODE-ONLY fiat|unsatisfied|ac.reinstate
#                                                  CODE-ONLY fiat|computed|ac.reinstate
#   an `Edge::` declaration removed          -> 1, RATIFIED-ONLY hold|wip|st.resume
#   `initial:` moved and the table not       -> 1, ENTRY DIVERGENCE both directions
#   a new `Disposition::State` field         -> 1, UNDECLARED Widget.phase
#   a `### Machine N` with no join           -> 2 (never "clean")
#   Machine 5's heading renamed away         -> 2, "has NO `### Machine 5` section at all"
#   Machine 5's table header row broken      -> 2, "heading IS present ... MALFORMED, not missing"
#
# **THE LAST TWO ARE ONE PAIR AND ONLY THE PAIR PROVES ANYTHING** (added 2026-08-30
# with the split they test). Both are rc=2 and both were rc=2 before the split as
# well -- what changed is that they now print DIFFERENT sentences naming DIFFERENT
# fixes, and an arm that only checks the code cannot see that. vc read the old
# shared sentence in the field and reported a peer as mid-write on a section that
# had never been written.
#   ONE table header row renamed             -> 2 (never 4 confident findings)
#   EVERY table header row renamed           -> 2
#   every `Machine N` heading renamed        -> 2
#   `pub const FIELDS` renamed               -> 2
#   a mapped field renamed out of `FIELDS`   -> 2
#   an `UNTABLED` field mutated              -> 0  **the negative control**
#
# **ARM 1 IS THE ONE THAT MATTERS: it reproduces the exact drift this file was
# built for**, on the real document with the real ruling removed, and names both
# missing rows. Everything else is a guard on the guard.
#
# **THE ONE-TABLE-HEADER ARM FOUND A DEFECT IN THIS FILE AND IS WHY THE
# PER-MACHINE POPULATION GUARDS EXIST.** Before them, a whole-file emptiness
# check passed while ONE machine parsed nothing, and every edge of that machine
# reported CODE-ONLY: four confident findings at exit 1, produced by reading
# nothing, pointing at the transcription -- the one file that was not wrong.
#
# **AND ONE ARM PLANTED NOTHING AND REPORTED CLEAN.** A `sed` with the wrong
# indent matched no line, the instrument correctly passed an unmutated file, and
# the green was read as an arm result. Every arm above now proves its plant
# landed (`diff` is non-empty) BEFORE its verdict is allowed to mean anything --
# a mutation that did not mutate is the purest form of a true green from an
# instrument that could not have answered.
#
# **THE NEGATIVE CONTROL IS NOT DECORATION HERE.** Eleven arms that all fire
# prove this reacts; only the `UNTABLED` arm proves it DISCRIMINATES -- that
# `WorkPackage.scope` moving underneath it is silence rather than noise, which
# is the property that keeps the untabled fields untabled instead of becoming a
# permanent red the operator learns to skip.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"

die() { echo "error: $1" >&2; exit 2; }

# shellcheck source=lib_staged.sh
. "$HERE/lib_staged.sh"
trap staged_cleanup EXIT

# Captured BEFORE the defaults are applied: an explicit override is a
# deliberate act by someone driving this against a planted corpus, which is how
# the mutation arms below are run, so it keeps reading the path it was handed.
DOC_GIVEN="${DOC:+yes}"
RS_GIVEN="${RS:+yes}"
DOC="${DOC:-$ST_DIR/data-model.md}"
RS="${RS:-$REPO_ROOT/native/rust/crates/intentsvcs/src/transitions.rs}"

[ -f "$DOC" ] || die "no data model at $DOC"
[ -f "$RS" ]  || die "no transitions table at $RS"

# `|| exit 2` is load-bearing rather than defensive noise: `staged_copy` refuses
# by exiting, and it runs in a command substitution, so that exit ends the
# SUBSHELL only. Without this the refusal becomes a read of "" and a confident
# green over nothing.
DOC="$(staged_copy "$DOC_GIVEN" "$DOC")" || exit 2
RS="$(staged_copy "$RS_GIVEN" "$RS")"   || exit 2

# --- the declared population -------------------------------------------------
# <machine number> <entity> <field>
MACHINE_MAP='
1 Thread status
2 WorkPackage status
3 Criterion state
4 Issue status
5 AcceptanceTest status
'
# `Disposition::State` fields the document deliberately does not table, with
# its reason. NEVER a skip list: a row here is a claim the document makes, and
# the document is where it has to be defended.
UNTABLED='
WorkPackage scope
'

# COMPUTED, never transcribed: this line used to read "the four ratified
# machines" and a fifth was ratified underneath it. A verdict that states a
# number it did not count is the same defect the population guards above exist
# to catch, one layer out -- and this one prints on every commit by four nodes.
N_MACHINES="$(printf '%s\n' "$MACHINE_MAP" | awk 'NF' | wc -l | tr -d ' ')"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/machine_table_check.XXXXXX")" || die "cannot create a working directory"
cleanup_all() { rm -rf "$WORK"; staged_cleanup; }
trap cleanup_all EXIT

DOC_TSV="$WORK/doc.tsv"
RS_TSV="$WORK/rs.tsv"
RS_FIELDS="$WORK/rs_fields.txt"

# --- side 1: the ratified tables ---------------------------------------------
# Scoped to the FIRST `| From | To | Verb | Guard |` table under each `Machine
# N` heading. Scoped rather than whole-file because this document carries other
# four-column tables (the `field | disposition` ruling table sits between
# Machines 3 and 4), and deciding scope by where a shape happens to appear is a
# defect this directory has already shipped once.
awk '
  function trim(s)   { sub(/^[ \t]+/, "", s); sub(/[ \t]+$/, "", s); return s }
  function unmark(s) { gsub(/\*\*/, "", s); gsub(/`/, "", s); return trim(s) }
  function kebab(s,   out, i, c) {
    if (s == "(none)" || s == "") return s
    out = ""
    for (i = 1; i <= length(s); i++) {
      c = substr(s, i, 1)
      if (c ~ /[A-Z]/ && i > 1) out = out "-"
      out = out tolower(c)
    }
    return out
  }
  function verbdot(s,   n, a) {
    n = split(s, a, /[ \t]+/)
    if (n < 2) return a[1]
    return a[1] "." a[2]
  }
  /^#+[ ]Machine[ ][0-9]+/ {
    line = $0
    sub(/^#+[ ]Machine[ ]/, "", line)
    m = line + 0
    intable = 0
    print "MACHINE\t" m
    next
  }
  m > 0 && !done[m] && /^\|[ ]*From[ ]*\|[ ]*To[ ]*\|[ ]*Verb[ ]*\|[ ]*Guard[ ]*\|/ { intable = 1; next }
  m > 0 && intable && /^\|[ -]+\|[ -]+\|[ -]+\|[ -]+\|[ \t]*$/ { next }
  m > 0 && intable && /^\|/ {
    n = split($0, cell, "|")
    if (n < 5) next
    f = unmark(cell[2]); t = unmark(cell[3]); v = unmark(cell[4]); g = unmark(cell[5])
    sub(/^_\(none\)_$/, "(none)", f)
    print "ROW\t" m "\t" kebab(f) "\t" kebab(t) "\t" verbdot(v) "\t" g
    next
  }
  m > 0 && intable && !/^\|/ { intable = 0; done[m] = 1 }
' "$DOC" > "$DOC_TSV"

grep -q '^ROW' "$DOC_TSV" || die "no ratified machine rows parsed out of $DOC -- the \`### Machine N\` heading or the \`| From | To | Verb | Guard |\` header did not match, so the tables could not be read. Refusing rather than reporting every Rust edge as undeclared, which is what an empty table would produce."

# --- side 2: the transcription ------------------------------------------------
# Comment lines are dropped first: the doc comments in this file cite
# `Edge::exits` and `Guard::` in prose, and a parser that read them would
# manufacture edges nobody declared. Everything is then collapsed to one line
# per syntactic marker, so a multi-line `Edge::guarded(...)` is one record.
awk '
  /pub const FIELDS/ { started = 1 }
  started {
    t = $0; sub(/^[ \t]+/, "", t)
    if (t ~ /^\/\//) next
    buf = buf " " $0
  }
  END {
    gsub(/[ \t]+/, " ", buf)
    gsub(/Field \{/,           "\n@F ", buf)
    gsub(/entity:/,            "\n@E ", buf)
    gsub(/ field:/,            "\n@N ", buf)
    gsub(/Disposition::State/, "\n@S ", buf)
    gsub(/initial:/,           "\n@I ", buf)
    gsub(/Edge::/,             "\n@G ", buf)
    gsub(/orphans:/,           "\n@O ", buf)
    print buf
  }
' "$RS" | awk '
  function q(line, k,   n, a) { n = split(line, a, "\""); if (n < 2 * k) return ""; return a[2 * k] }
  /^@F/ { entity = ""; fname = ""; isstate = 0; next }
  /^@E/ { entity = q($0, 1); next }
  /^@N/ { fname  = q($0, 1); next }
  /^@S/ { isstate = 1; print "FIELD\t" entity "\t" fname; next }
  /^@I/ {
    if (!isstate) next
    line = $0
    p = index(line, "]")
    if (p > 0) line = substr(line, 1, p)
    n = split(line, a, "\"")
    for (i = 2; i <= n; i += 2) print "INITIAL\t" entity "\t" fname "\t" a[i]
    next
  }
  /^@G/ {
    if (!isstate) next
    n = split($0, a, "\"")
    if (n < 4) next
    verb = a[2]
    to = ""; toidx = 0; nf = 0
    for (i = 4; i <= n; i += 2) {
      if (index(a[i - 1], "]") > 0) { to = a[i]; toidx = i; break }
      nf++; froms[nf] = a[i]
    }
    if (toidx == 0) next
    tail = ""
    for (i = toidx + 1; i <= n; i++) tail = tail a[i]
    guards = ""
    while (match(tail, /Guard::[A-Za-z]+/)) {
      g = substr(tail, RSTART + 7, RLENGTH - 7)
      guards = guards (guards == "" ? "" : ",") g
      tail = substr(tail, RSTART + RLENGTH)
    }
    if (nf == 0) print "EDGE\t" entity "\t" fname "\t(any)\t" to "\t" verb "\t" guards
    for (i = 1; i <= nf; i++) print "EDGE\t" entity "\t" fname "\t" froms[i] "\t" to "\t" verb "\t" guards
    for (i = 1; i <= nf; i++) delete froms[i]
    next
  }
  /^@O/ {
    if (!isstate) next
    line = $0
    n = split(line, a, "\"")
    for (i = 2; i <= n; i += 2) if (a[i] ~ /^[a-z0-9-]+$/ && a[i - 1] ~ /\($/) print "ORPHAN\t" entity "\t" fname "\t" a[i]
    next
  }
' > "$RS_TSV"

grep -q '^EDGE' "$RS_TSV" || die "no edges parsed out of $RS -- \`pub const FIELDS\` or the \`Edge::\` grammar did not match, so the transcription could not be read. Refusing rather than reporting every ratified row as unimplemented."

awk -F'\t' '$1 == "FIELD" { print $2, $3 }' "$RS_TSV" | sort -u > "$RS_FIELDS"

# --- guard vocabulary ---------------------------------------------------------
# The document's Guard-column keyword -> the `Guard` variant it names. DECLARED
# and deliberately small: a mapping that grew to absorb every prose form would
# become a third home for the fact and would lose the ability to say
# UNMEASURED, which is the one verdict this axis exists to produce. It lives
# inside the axis-C awk below rather than in a shell function, and that is a
# COST decision with a measurement behind it: a function-per-row shell form
# spawned about ten processes per row and cost 620-720ms, which is more than
# any other guard in this gate spends, for an axis that never gates. One awk
# per machine is the same comparison at a fraction of it.
#
# BOTH SIDES GO THROUGH `canon`, which emits a fixed alphabetical order. The
# Rust side lists guards in source order (`NonTestOnly, EvidenceRecorded`), so
# without it a correct pair compares unequal on spelling alone.

# --- the comparison -----------------------------------------------------------
FINDINGS=0
GUARD_AGREE=0
GUARD_DISAGREE=0
GUARD_UNMEASURED=0
TOTAL_ROWS=0
GUARD_REPORT="$WORK/guards.txt"
: > "$GUARD_REPORT"

printf 'machine-table: comparing the ratified tables in %s against the transcription in %s\n' \
  "${DOC#"$REPO_ROOT"/}" "${RS#"$REPO_ROOT"/}"

# Every `### Machine N` heading must have a join. A heading with none is a
# machine nobody can check, and reporting it as clean is the failure this file
# is named for.
DOC_MACHINES="$(awk -F'\t' '$1 == "MACHINE" { print $2 }' "$DOC_TSV" | sort -un)"
for m in $DOC_MACHINES; do
  if ! printf '%s\n' "$MACHINE_MAP" | awk -v m="$m" 'NF && $1 == m { found = 1 } END { exit !found }'; then
    die "\`Machine $m\` is declared in $DOC and \`MACHINE_MAP\` has no join for it -- add the (machine, entity, field) row, because a machine with no counterpart cannot be checked and must not be reported clean"
  fi
done

# Every `Disposition::State` field must be tabled or declared untabled.
while read -r entity fname; do
  [ -z "$entity" ] && continue
  if printf '%s\n' "$MACHINE_MAP" | awk -v e="$entity" -v f="$fname" 'NF && $2 == e && $3 == f { found = 1 } END { exit !found }'; then
    continue
  fi
  if printf '%s\n' "$UNTABLED" | awk -v e="$entity" -v f="$fname" 'NF && $1 == e && $2 == f { found = 1 } END { exit !found }'; then
    continue
  fi
  printf '\nmachine-table: UNDECLARED  %s.%s is `Disposition::State` and is in neither `MACHINE_MAP` nor `UNTABLED`\n' "$entity" "$fname"
  printf '  -- ratify a machine for it in %s and add the join, or record in `UNTABLED` why the document does not table it. The document implying a complete set of machines when it was not is the defect that produced its own "Two more state fields exist" section.\n' "${DOC#"$REPO_ROOT"/}"
  FINDINGS=$((FINDINGS + 1))
done < "$RS_FIELDS"

while read -r m entity fname; do
  [ -z "$m" ] && continue

  DOC_ENTRY="$(awk -F'\t' -v m="$m" '$1 == "ROW" && $2 == m && $3 == "(none)" { print $4 }' "$DOC_TSV" | sort -u)"
  RS_INITIAL="$(awk -F'\t' -v e="$entity" -v f="$fname" '$1 == "INITIAL" && $2 == e && $3 == f { print $4 }' "$RS_TSV" | sort -u)"
  DOC_EDGES="$(awk -F'\t' -v m="$m" '$1 == "ROW" && $2 == m && $3 != "(none)" { print $3 "|" $4 "|" $5 }' "$DOC_TSV" | sort -u)"
  RS_EDGES="$(awk -F'\t' -v e="$entity" -v f="$fname" '$1 == "EDGE" && $2 == e && $3 == f { print $4 "|" $5 "|" $6 }' "$RS_TSV" | sort -u)"

  nde="$(printf '%s\n' "$DOC_EDGES" | grep -c .)"
  nre="$(printf '%s\n' "$RS_EDGES"  | grep -c .)"
  TOTAL_ROWS=$((TOTAL_ROWS + nde))

  # **PER-MACHINE POPULATION GUARDS, and arm 6 is why they are here rather than
  # only the whole-file ones below the parsers.** A single table going
  # unreadable -- one renamed header row, one changed column name -- empties
  # ONE side of ONE machine, and every edge of the other side then reports as
  # undeclared: a wall of confident findings produced by reading nothing, all
  # of them pointing at the file that is not wrong. Measured on this file
  # before the guard existed: renaming Machine 1's header row produced four
  # divergences at exit 1 against a transcription that was correct.
  # **AN ABSENT SECTION AND A MALFORMED ONE PRINTED THE SAME SENTENCE, and they
  # call for opposite fixes** (vc, 2026-08-30, having misread it in the field:
  # "its table did not match" reads as *the table is there and broken*, and vc
  # reported a node as mid-write on a section that had never existed). The
  # machine NUMBERS come from `MACHINE_MAP`, which is keyed to the code, so a
  # joined machine with no `### Machine N` heading in the document is the normal
  # shape of "the ratified table was never written" -- the one this gate exists
  # to catch. Separating them costs one grep of an already-parsed file.
  if awk -F'\t' -v m="$m" '$1 == "MACHINE" && $2 == m { found = 1 } END { exit !found }' "$DOC_TSV"; then
    [ "$nde" -gt 0 ] || die "Machine $m is joined to $entity.$fname and its \`### Machine $m\` heading IS present in ${DOC#"$REPO_ROOT"/}, but ZERO ratified transition rows parsed out from under it -- the \`| From | To | Verb | Guard |\` header did not match, so the ratified side could not be read. Refusing rather than reporting every implemented edge as undeclared, which is what an empty table would produce. THE TABLE IS MALFORMED, not missing."
  else
    [ "$nde" -gt 0 ] || die "Machine $m is joined to $entity.$fname and ${DOC#"$REPO_ROOT"/} has NO \`### Machine $m\` section at all -- the ratified table was never written, or was written under a different number. The code declares this machine and the document does not, which is the one-way drift this check exists to catch: add the section, or remove the \`MACHINE_MAP\` join and record the field in \`UNTABLED\` with the document's reason."
  fi
  [ "$nre" -gt 0 ] || die "Machine $m is joined to $entity.$fname and the transcription declares ZERO edges for it -- the field is missing from \`FIELDS\`, or its disposition is no longer \`Disposition::State\`. Refusing rather than reporting every ratified row as unimplemented."

  printf '  Machine %s -> %s.%s: %s ratified transition rows, %s expanded edges\n' "$m" "$entity" "$fname" "$nde" "$nre"

  # --- axis A: entry ----------------------------------------------------------
  a_only="$(comm -23 <(printf '%s\n' "$DOC_ENTRY" | grep -v '^$') <(printf '%s\n' "$RS_INITIAL" | grep -v '^$'))"
  b_only="$(comm -13 <(printf '%s\n' "$DOC_ENTRY" | grep -v '^$') <(printf '%s\n' "$RS_INITIAL" | grep -v '^$'))"
  if [ -n "$a_only" ] || [ -n "$b_only" ]; then
    printf '\nmachine-table: ENTRY DIVERGENCE on Machine %s (%s.%s)\n' "$m" "$entity" "$fname"
    [ -n "$a_only" ] && printf '%s\n' "$a_only" | sed 's/^/  RATIFIED-ONLY  entry state /'
    [ -n "$b_only" ] && printf '%s\n' "$b_only" | sed 's/^/  CODE-ONLY      initial value /'
    printf '  -- the `_(none)_ ->` rows in the document and `initial: &[...]` in the code are the same fact in two notations. One of them moved.\n'
    FINDINGS=$((FINDINGS + 1))
  fi

  # --- axis B: edges ----------------------------------------------------------
  d_only="$(comm -23 <(printf '%s\n' "$DOC_EDGES" | grep -v '^$') <(printf '%s\n' "$RS_EDGES" | grep -v '^$'))"
  r_only="$(comm -13 <(printf '%s\n' "$DOC_EDGES" | grep -v '^$') <(printf '%s\n' "$RS_EDGES" | grep -v '^$'))"
  if [ -n "$d_only" ] || [ -n "$r_only" ]; then
    printf '\nmachine-table: EDGE DIVERGENCE on Machine %s (%s.%s)\n' "$m" "$entity" "$fname"
    [ -n "$d_only" ] && printf '%s\n' "$d_only" | sed 's/^/  RATIFIED-ONLY  /'
    [ -n "$r_only" ] && printf '%s\n' "$r_only" | sed 's/^/  CODE-ONLY      /'
    printf '  -- format is `from|to|verb`, EXPANDED on both sides. A CODE-ONLY edge means a ruling reached the transcription and not the table it was ratified in, which is the direction this pair drifts by construction. A RATIFIED-ONLY edge means the table declares a transition no verb implements.\n'
    FINDINGS=$((FINDINGS + 1))
  fi

  # --- axis C: guards (reported, never gating) ---------------------------------
  awk -F'\t' -v m="$m" -v e="$entity" -v fl="$fname" -v out="$GUARD_REPORT" '
    function canon(s,   o) {
      o = ""
      if (index(s, "EvidenceRecorded")) o = o ",EvidenceRecorded"
      if (index(s, "GatePass"))         o = o ",GatePass"
      if (index(s, "NonTestOnly"))      o = o ",NonTestOnly"
      if (index(s, "ReasonRecorded"))   o = o ",ReasonRecorded"
      if (index(s, "TargetExists"))     o = o ",TargetExists"
      return (o == "") ? "NONE" : substr(o, 2)
    }
    function docguards(prose,   l, o) {
      l = tolower(prose)
      if (l == "" || l == "--") return "NONE"
      o = ""
      if (index(l, "reason recorded"))      o = o " ReasonRecorded"
      if (index(l, "ac gate pass"))         o = o " GatePass"
      if (index(l, "target thread exists")) o = o " TargetExists"
      if (index(l, "non-test"))             o = o " NonTestOnly"
      if (index(l, "evidence given"))       o = o " EvidenceRecorded"
      return (o == "") ? "UNMAPPABLE" : canon(o)
    }
    NR == FNR {
      if ($1 == "EDGE" && $2 == e && $3 == fl) rs[$4 "|" $5 "|" $6] = canon($7)
      next
    }
    $1 == "ROW" && $2 == m && $3 != "(none)" {
      k = $3 "|" $4 "|" $5
      want = docguards($6)
      have = (k in rs) ? rs[k] : "NONE"
      if (want == "UNMAPPABLE") {
        unmeasured++
        printf "UNMEASURED  M%s %s  column reads \"%s\"; code has %s\n", m, k, $6, have >> out
      } else if (want == have) {
        agree++
      } else {
        disagree++
        printf "DISAGREE    M%s %s  column reads \"%s\" (-> %s); code has %s\n", m, k, $6, want, have >> out
      }
    }
    END { printf "%d %d %d\n", agree + 0, disagree + 0, unmeasured + 0 }
  ' "$RS_TSV" "$DOC_TSV" > "$WORK/gc.$m"
  read -r ga gd gu < "$WORK/gc.$m"
  GUARD_AGREE=$((GUARD_AGREE + ga))
  GUARD_DISAGREE=$((GUARD_DISAGREE + gd))
  GUARD_UNMEASURED=$((GUARD_UNMEASURED + gu))
done <<< "$(printf '%s\n' "$MACHINE_MAP" | awk 'NF { print $1, $2, $3 }')"

# --- axis C report ------------------------------------------------------------
printf '\nmachine-table: GUARD AXIS (reported, never gating): %d agree, %d disagree, %d UNMEASURED of %d rows\n' \
  "$GUARD_AGREE" "$GUARD_DISAGREE" "$GUARD_UNMEASURED" "$TOTAL_ROWS"
if [ -s "$GUARD_REPORT" ]; then
  sed 's/^/  /' "$GUARD_REPORT"
  printf '  -- UNMEASURED is NOT clean: the Guard column in the document is free prose, and these cells name an EFFECT or a LANDING RULE rather than a precondition, so the axis has no verdict for them and the actual guard in the code is undeclared in the ratified document. The fix is a controlled vocabulary in the Guard column, which is a change to a ratified table and therefore hv to rule.\n'
fi

# --- verdict ------------------------------------------------------------------
if [ "$FINDINGS" -gt 0 ]; then
  printf '\nVERDICT: %d divergence(s) between the ratified machines and their transcription.\n' "$FINDINGS"
  exit 1
fi
printf '\nVERDICT: the %s ratified machines and their transcription agree exactly on entry states and on every expanded (from, to, verb) edge.\n' "$N_MACHINES"
exit 0
