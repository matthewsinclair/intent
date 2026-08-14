#!/bin/bash
# gen_dispatch_table.sh -- render `dispatch-table.md` (the view) from
# `dispatch-table.json` (the canon).
#
# WHY THERE IS A GENERATOR AT ALL, rather than a hand-written markdown table.
#
# The dispatch table is the command-surface SSOT (AC-05.1): the clap surface,
# the help text, the MCP tool list and the `intent llm` guide all render from
# it. That makes its FORMAT an architectural choice, not a preference, and
# design.md already made it. md-as-truth was proposed and REJECTED by hv
# (design.md:158) on the grounds that "the bespoke row-grammar tax recurs
# forever -- 0012/0017/close-gate were three instances". Authoring this table
# as a markdown table would be the fourth instance, in the artefact whose whole
# job is to stop v3 re-deriving its own surface description.
#
# So: JSON is the authored canon, markdown is a generated view, and the view
# carries a banner saying so. That is D02 (authored-once), D03 (JSON canon) and
# D04 (generated views committed) applied to the first v3 artefact that had a
# choice -- dogfooding the truth model rather than describing it.
#
# It is also the same shape as this directory's other generator: register.md is
# rendered from burn-baseline.tsv by gen_register.sh and says "do not hand-edit
# rows". One pattern, not two.
#
# NO SILENT EMPTY SURFACE. Every failure path here refuses loudly. A generator
# that emits a well-formed empty document on missing input produces something
# that reads exactly like a measurement of nothing, and the whole parity
# toolchain exists because that class of lie is expensive: gen_register.sh
# once wrote an unstamped register from a mistyped variable and did not
# complain, and the register was believed for the rest of the afternoon.
#
# Usage:
#   bash intent/st/ST0056/parity/tools/gen_dispatch_table.sh
#   IN=<canon.json> OUT=<view.md> bash .../gen_dispatch_table.sh

# IN-SH-CODE-003. `-e` is not redundant beside the explicit `die` guards below:
# an abort partway through leaves the render in $OUT_TMP, so the `mv` at the end
# never runs and the committed view survives untouched. Failing hard mid-render
# is therefore SAFER here than continuing, which is the opposite of the usual
# trade and the reason to take it rather than document an exception.
set -euo pipefail

die() {
  echo "error: $1" >&2
  exit 1
}

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ST_DIR="$(cd "$HERE/../.." && pwd)"
# The table lives at the WORKSPACE ROOT, not in the ST tree (vc ruling,
# 2026-08-14). `intent st done` does `mv "$CURRENT_DIR" "$NEW_DIR"`
# (bin/intent_st:392) into `intent/st/COMPLETED/`, so anything that compiles
# the table in -- the CLI's include_str! -- would stop resolving the moment
# ST0056 is marked Completed. That happens in WP-12, which IS the release.
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"

IN="${IN:-$REPO_ROOT/surface/dispatch-table.json}"
OUT="${OUT:-$REPO_ROOT/surface/dispatch-table.md}"

[ -f "$IN" ] || die "canon not found: $IN"
jq empty "$IN" 2>/dev/null || die "canon is not valid JSON: $IN"

# A cell that contains a pipe breaks the table it is rendered into. v2 hit this
# in `render_table` (IFS='|') and answered it by substituting `|` -> `/`, pinned
# by tests/unit/title_pipe_sanitize_guard.bats. Same answer here, for the same
# reason, so the view cannot be corrupted by its own content.
JQ_LIB='
  def cell: if . == null then "--" else (tostring | gsub("\\|"; "/") | gsub("\n"; " ")) end;
  def joincell(sep): if . == null or length == 0 then "--" else (map(tostring) | join(sep) | gsub("\\|"; "/")) end;
  def argsig:
    if . == null or length == 0 then "--"
    else (map(
      .name as $n | .arity as $a |
      if $a == "1" then "<\($n)>"
      elif $a == "0..1" then "[\($n)]"
      elif $a == "1..n" then "<\($n)>..."
      else "[\($n)]..." end
    ) | join(" ")) end;
  def flagsig:
    if . == null or length == 0 then "--"
    else (map(
      (.spellings | join("/")) as $s |
      if .value then "\($s) \(.value)" else $s end
    ) | join(", ") | gsub("\\|"; "/")) end;
'

# Render to a temp file and move into place only on success. Writing straight
# to $OUT would mean a REFUSED run leaves an empty committed view behind --
# which is the silent-empty-surface failure this script's header says it
# refuses, reproduced by the script itself. Found by mutation-testing the
# refusal paths, not by reading: the stamp guards below fire AFTER the point
# the output file had already been truncated.
OUT_TMP="$(mktemp)" || die "cannot create a temp file"
cleanup() { rm -f "$OUT_TMP"; }
trap cleanup EXIT

emit() { printf '%s\n' "$1" >> "$OUT_TMP"; }

MEASURED_AT="$(jq -r '.measured_at // empty' "$IN")"
MEASURED_ON="$(jq -r '.measured_on // empty' "$IN")"
MEASURED_BY="$(jq -r '.measured_by // empty' "$IN")"
STATUS="$(jq -r '.status // empty' "$IN")"

# The stamp is not decoration. A record that does not name the commit it covers
# cannot be spotted as stale, and "full suite GREEN at HEAD" was false by three
# commits across four documents for exactly this reason. Refuse rather than
# emit an unstamped artefact.
[ -n "$MEASURED_AT" ] || die "canon has no measured_at -- refusing to emit an unstamped view"
[ -n "$MEASURED_ON" ] || die "canon has no measured_on -- refusing to emit an unstamped view"

emit "# Command dispatch table -- Intent v3 (ST0056, AC-05.1)"
emit ""
emit "> GENERATED VIEW -- the canon is \`dispatch-table.json\` beside this file. Regenerate with \`parity/tools/gen_dispatch_table.sh\`; do not hand-edit rows. Measured at \`$MEASURED_AT\` on $MEASURED_ON by $MEASURED_BY."
emit ""
if [ -n "$STATUS" ]; then emit "**Status:** $STATUS"; emit ""; fi

jq -r '.about[]? | "- " + .' "$IN" >> "$OUT_TMP"
emit ""

# Provenance is rendered, never summarised into the one-line stamp above. A
# view that showed a single `measured_at` while the canon recorded two would
# hide precisely what the block exists to disclose.
if jq -e '.provenance' "$IN" >/dev/null 2>&1; then
  emit "## Provenance"
  emit ""
  jq -r '.provenance |
    "- **Source reads and live probes at:** `\(.source_reads_and_live_probes_at)`",
    "- **Runtime probe matrix at:** `\(.probe_matrix_at)`",
    "- **Why two revisions:** \(.why_two_revisions)",
    "- **Re-validated after those bin/ changes:**",
    (.revalidated_after_the_bin_changes[] | "  - " + .),
    "- **Known limit:** \(.known_limit)"' "$IN" >> "$OUT_TMP"
  emit ""
fi

# --- Surface-wide invariants ------------------------------------------------
emit "## Surface-wide invariants"
emit ""
emit "Rules that hold across the whole command surface. They are stated once here rather than repeated on every entry, and WP-05 must honour them at the framework layer -- several are things clap does differently by default, so inheriting the default silently breaks parity."
emit ""
emit "| id | invariant | v3 target |"
emit "| -- | --------- | --------- |"
jq -r "$JQ_LIB"'
  .invariants[]? |
  "| \(.id) | \(.title | cell) | \(.target.state | cell) |"
' "$IN" >> "$OUT_TMP"
emit ""

jq -r "$JQ_LIB"'
  .invariants[]? |
  "### \(.id) -- \(.title)\n",
  "\(.rule)\n",
  "- **v2:** \(.v2 | cell)",
  (if .evidence then "- **Evidence:** \(.evidence | cell)" else empty end),
  (if .evidence_class then ("- **Evidence class:** `\(.evidence_class.class)` -- \(.evidence_class.why)" +
     (if .evidence_class.pinned_by then "\n  - Pinned by: \(.evidence_class.pinned_by)" else "" end)) else empty end),
  "- **Target:** `\(.target.state)`" +
    (if .target.ratification then " -- ratified: \(.target.ratification)" else "" end) +
    (if .target.behaviour then " -- behaviour: \(.target.behaviour)" else "" end) +
    (if .target.question then "\n- **Open question for hv:** \(.target.question)" else "" end),
  (if .target.note then "- **Note:** \(.target.note)" else empty end),
  (if .implementation_note then "- **Implementation constraint:** \(.implementation_note)" else empty end),
  (if .exceptions then ("- **Exceptions:**\n" + (.exceptions | map("  - " + .) | join("\n"))) else empty end),
  ""
' "$IN" >> "$OUT_TMP"

# --- Families ---------------------------------------------------------------
FAMILY_COUNT="$(jq -r '.families | length' "$IN")"
[ "$FAMILY_COUNT" -gt 0 ] || die "canon has no families -- refusing to emit an empty surface"

for i in $(seq 0 $((FAMILY_COUNT - 1))); do
  F="$(jq -c ".families[$i]" "$IN")"
  NAME="$(printf '%s' "$F" | jq -r '.name')"
  V2SRC="$(printf '%s' "$F" | jq -r '.v2_source // "new-surface"')"
  WP="$(printf '%s' "$F" | jq -r '.owner_wp // "--"')"
  FHELP="$(printf '%s' "$F" | jq -r '.help // ""')"

  emit "## Family: \`$NAME\`"
  emit ""
  emit "$FHELP"
  emit ""
  emit "- **v2 source:** \`$V2SRC\`"
  # Wrap in backticks ONLY when the value does not already contain one.
  # Wrapping a value that carries its own backticks produces a nested span the
  # markdown formatter then "normalises" by collapsing the spaces inside it --
  # turning `a` against `b` into `a`against`b`, which inverts nothing here but
  # silently could. Caught by the skew check on this very file, which is the
  # first time that check has paid for itself.
  emit "- **v2 help file:** $(printf '%s' "$F" | jq -r '
    if .v2_help_file == null then "none"
    elif (.v2_help_file | test("`")) then .v2_help_file
    else "`" + .v2_help_file + "`" end')"
  emit "- **Owning work package:** $WP"
  if printf '%s' "$F" | jq -e '.bats_coverage' >/dev/null 2>&1; then
    emit "- **BATS coverage:** $(printf '%s' "$F" | jq -r '.bats_coverage | "\(.burning_tests) burning test(s) across \(.files_real) file(s)" + (if .files_vacuous > 0 then ", plus \(.files_vacuous) file(s) that name it but never reach the CLI" else "" end) + " -- **\(.verdict)**"')"
  fi
  emit ""
  printf '%s' "$F" | jq -r '.family_notes[]? | "- " + .' >> "$OUT_TMP"
  # A false predicate is not an error. Under `set -e` a bare `cond && action`
  # tail would abort the whole render whenever a family simply has no notes.
  if printf '%s' "$F" | jq -e '(.family_notes // []) | length > 0' >/dev/null 2>&1; then emit ""; fi

  emit "| command | args | flags | help | disposition |"
  emit "| ------- | ---- | ----- | ---- | ----------- |"
  printf '%s' "$F" | jq -r "$JQ_LIB"'
    .entries[]? |
    "| `\(.path)`\((.aliases // []) | if length > 0 then " (alias " + (map("`" + . + "`") | join(", ")) + ")" else "" end) | \(.args | argsig | gsub("\\|"; "/")) | \(.flags | flagsig) | \(.help | cell) | \(.disposition | cell) |"
  ' >> "$OUT_TMP"
  emit ""

  printf '%s' "$F" | jq -r "$JQ_LIB"'
    .entries[]? |
    "### `\(.path)`\n",
    "\(.help)\n",
    "- **v2:** \(.v2 | cell)",
    (if (.args | length) > 0 then ("- **Arguments:**\n" + (.args | map(
       "  - `\(.name)` (\(.type), arity `\(.arity)`)" +
       (if .default then ", default `\(.default)`" else "" end) +
       (if .values then " -- one of: " + (.values | map("`" + . + "`") | join(", ")) else "" end) +
       (if .note then "\n    - " + .note else "" end)
     ) | join("\n"))) else empty end),
    (if (.flags | length) > 0 then ("- **Flags:**\n" + (.flags | map(
       "  - `" + (.spellings | join("`, `")) + "`" +
       (if .value then " `\(.value)`" else "" end) +
       " (\(.type))" +
       (if .help then " -- \(.help)" else "" end) +
       (if .accepts then "\n    - Accepts: \(.accepts)" else "" end) +
       (if .note then "\n    - " + .note else "" end)
     ) | join("\n"))) else empty end),
    "- **Exit codes:**\n" + (.observed.exit | map("  - `\(.code)` -- \(.when)") | join("\n")),
    "- **stdout:** \(.observed.stdout | cell)",
    "- **stderr:** \(.observed.stderr | cell)",
    (if .observed.side_effects then ("- **Side effects:**\n" + (.observed.side_effects | map("  - " + .) | join("\n"))) else empty end),
    (if .observed.notes then "- **Observed notes:** \(.observed.notes)" else empty end),
    (if .observed.defects then ("- **Defects observed in v2:**\n" + (.observed.defects | map(
       # `id: local` means an entry-specific defect that no invariant covers,
       # so there is no ID to point at and the detail stands alone. Any other
       # id names an invariant: print the ID plus this entry`s locus and STOP.
       # Restating the rule here is the divergent copy ruling 3 exists to stop.
       if type == "object" then
         (if .id == "local" then "  - " + .detail
          else "  - " + .id + (if .where then " at " + .where else "" end) end)
       else "  - " + . end
     ) | join("\n"))) else empty end),
    "- **Target:** `\(.target.state)`" +
      (if .target.ratification then " -- ratified: \(.target.ratification)" else "" end) +
      (if .target.behaviour then " -- behaviour: \(.target.behaviour)" else "" end),
    (if .target.question then "- **Open question for hv:** \(.target.question)" else empty end),
    (if .target.note then "- **Note:** \(.target.note)" else empty end),
    (if .cross_ref then "- **Cross-reference:** \(.cross_ref)" else empty end),
    ""
  ' >> "$OUT_TMP"
done

# --- Outstanding + new surface ----------------------------------------------
# --- Known exposures --------------------------------------------------------
# Rendered, not merely stored. A file that is clean by luck and a file that is
# clean by construction look identical in a diff, and only one stays clean.
if jq -e '.known_exposures' "$IN" >/dev/null 2>&1; then
  emit "## Known exposures -- defects this file does not have, and is not protected against"
  emit ""
  jq -r '.known_exposures[] |
    "### \(.id) -- \(.title)\n",
    "- **Detail:** \(.detail)",
    "- **Resolution:** \(.resolution)",
    (if .consequence_for_this_generator then "- **Consequence for the generator:** \(.consequence_for_this_generator)" else empty end),
    ""' "$IN" >> "$OUT_TMP"
fi

# --- Coverage findings ------------------------------------------------------
if jq -e '.coverage_findings' "$IN" >/dev/null 2>&1; then
  emit "## Parity holes -- what the BATS estate does NOT cover"
  emit ""
  emit "A command family with no burning coverage is a parity hole: v3 can change it freely and the conformance suite stays green. Produced by \`parity/tools/coverage_map.sh\`, which joins these families against \`burn-baseline.tsv\` -- the join matters, because a naive grep reports \`treeindex\` as well covered when all 53 of its tests exec \`bin/intent_treeindex\` directly and the dispatcher never sees them."
  emit ""
  emit "| family | files (real) | files (vacuous) | burning tests | verdict |"
  emit "| ------ | ------------ | --------------- | ------------- | ------- |"
  jq -r "$JQ_LIB"'
    .families[] | select(.bats_coverage) |
    "| `\(.name)` | \(.bats_coverage.files_real) | \(.bats_coverage.files_vacuous) | \(.bats_coverage.burning_tests) | \(.bats_coverage.verdict | cell) |"
  ' "$IN" >> "$OUT_TMP"
  emit ""
  jq -r '.coverage_findings[] |
    "### `\(.family)` -- \(.verdict)\n",
    "- **Finding:** \(.finding)",
    "- **Why it matters:** \(.why_it_matters)",
    (if .trap then "- **The trap:** \(.trap)" else empty end),
    (if .action then "- **Action:** \(.action)" else empty end),
    ""' "$IN" >> "$OUT_TMP"
fi

emit "## Families outstanding"
emit ""
# An empty list must SAY it is empty. Rendering the prose and then nothing
# leaves the reader unable to tell "none left" from "the loop broke" -- the same
# absence-as-meaning failure that made `pending` an explicit value rather than a
# missing field (vc ruling 2, 2026-08-14).
if [ "$(jq -r '(.families_outstanding // []) | length' "$IN")" -eq 0 ]; then
  emit "**None.** Every v2 command family is authored."
else
  emit "Not yet authored. Named individually rather than counted, so a family that quietly never gets written is visible as a gap rather than absent from a total."
  emit ""
  jq -r '.families_outstanding[]? | "- `" + . + "`"' "$IN" >> "$OUT_TMP"
fi
emit ""

emit "## New surface (no v2 antecedent, no parity obligation)"
emit ""
emit "| command | args | flags | help | owning WP | basis |"
emit "| ------- | ---- | ----- | ---- | --------- | ----- |"
jq -r "$JQ_LIB"'
  .new_surface[]? |
  "| `\(.path)` | \(.args | argsig | gsub("\\|"; "/")) | \(.flags | flagsig) | \(.help | cell) | \(.owner_wp | cell) | \(.basis | cell) |"
' "$IN" >> "$OUT_TMP"
emit ""
jq -r '.new_surface[]? | select(.acceptance) | "- `" + .path + "` -- acceptance: " + .acceptance' "$IN" >> "$OUT_TMP"

# --- Column-align every table BEFORE the file lands -------------------------
#
# NOT cosmetic. Two independent reasons, and the second is the load-bearing one.
#
# 1. House rule: "All markdown tables must be column-aligned" (in-standards).
#
# 2. THE SKEW CHECK. A generated view that is committed (D04) is verified by
#    regenerating it and requiring an empty diff -- that is AC-03.4, and it is
#    how a hand-edited view gets caught instead of silently outvoted. This
#    repository's markdown formatter runs in the pre-commit gate and aligns
#    tables. So an unaligned generator produces: generator writes narrow ->
#    formatter widens on commit -> next regeneration narrows again -> the skew
#    check reports drift, forever, on a file nobody touched. The view renderer
#    must emit EXACTLY what the formatter would produce, or the two fight and
#    the check cries wolf until someone turns it off.
#
#    Found by committing this file and watching it happen, not by reasoning
#    about it. It generalises straight to WP-03: every view v3 generates
#    (info.md, acceptance.md, steel_threads.md, todo.md) lands in repositories
#    with formatters, and "deterministic and idempotent" (AC-03.2) has to mean
#    idempotent THROUGH the formatter, not just through the renderer.
#
# DO NOT DELETE THIS WHEN AC-07.6 LANDS. Two justifications, only one expires.
# AC-07.6 excludes generated views from the formatter repo-wide, which retires
# reason (2) above -- there is no longer a formatter to agree with. Reason (1)
# does not expire: `in-standards` requires every markdown table to be
# column-aligned, and that governs what this renderer EMITS regardless of who
# else writes the file. The formatter was correcting a real defect here, not
# imposing a preference. (vc ruling, 2026-08-14, recorded in acceptance.md
# under the AC-07.6 disposition.)
#
# AND THE HALF THAT IS EASIER TO MISS: once the formatter stops writing these
# files, "aligned" needs a definition THIS CODE OWNS. Today the word means, in
# practice, "whatever prettier does" -- an external authority we happen to
# agree with. That meaning evaporates with the exclusion, and if nobody
# notices, "aligned" becomes undefined and the next renderer picks its own
# reading. That is the divergent-copy problem arriving through a VACATED
# definition rather than a duplicated one, which is harder to see because
# nothing was copied.
#
# So from AC-07.6 onward the rule below IS the definition, not a check against
# one: one space of padding inside each cell, columns padded to the widest
# cell, separator rows filled with dashes to the same width. A future reader
# deleting this block is only half the hazard; the other half is one keeping it
# while quietly meaning something different by "aligned".
ALIGNER='
  function flush(  i, j, w, out, cell, n) {
    if (rows == 0) return
    for (i = 1; i <= rows; i++)
      for (j = 1; j <= cols[i]; j++)
        if (length(cellv[i, j]) > w_[j]) w_[j] = length(cellv[i, j])
    for (i = 1; i <= rows; i++) {
      out = "|"
      for (j = 1; j <= cols[i]; j++) {
        if (sep[i]) { cell = ""; while (length(cell) < w_[j]) cell = cell "-" }
        else { cell = cellv[i, j]; while (length(cell) < w_[j]) cell = cell " " }
        out = out " " cell " |"
      }
      print out
    }
    rows = 0; n = 0
    for (j in w_) delete w_[j]
  }
  /^[ \t]*\|.*\|[ \t]*$/ {
    line = $0
    sub(/^[ \t]+/, "", line); sub(/[ \t]+$/, "", line)
    sub(/^\|/, "", line); sub(/\|$/, "", line)
    rows++
    n = split(line, parts, "\\|")
    cols[rows] = n
    issep = 1
    for (j = 1; j <= n; j++) {
      c = parts[j]
      gsub(/^[ \t]+|[ \t]+$/, "", c)
      cellv[rows, j] = c
      if (c !~ /^:?-+:?$/) issep = 0
    }
    sep[rows] = issep
    next
  }
  { flush(); print }
  END { flush() }
'
awk "$ALIGNER" "$OUT_TMP" > "$OUT_TMP.aligned" || die "table alignment failed"
mv "$OUT_TMP.aligned" "$OUT_TMP" || die "table alignment failed to land"

# Only now, with the whole view rendered and aligned, does the committed file
# change.
mv "$OUT_TMP" "$OUT" || die "cannot write: $OUT"
trap - EXIT

echo "ok: rendered $(jq -r '[.families[].entries | length] | add' "$IN") entries across $FAMILY_COUNT family(s) -> $OUT" >&2
