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

set -uo pipefail

die() {
  echo "error: $1" >&2
  exit 1
}

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ST_DIR="$(cd "$HERE/../.." && pwd)"

IN="${IN:-$ST_DIR/dispatch-table.json}"
OUT="${OUT:-$ST_DIR/dispatch-table.md}"

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
[ -n "$STATUS" ] && { emit "**Status:** $STATUS"; emit ""; }

jq -r '.about[]? | "- " + .' "$IN" >> "$OUT_TMP"
emit ""

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
  emit "- **v2 help file:** $(printf '%s' "$F" | jq -r 'if .v2_help_file then "`" + .v2_help_file + "`" else "none" end')"
  emit "- **Owning work package:** $WP"
  emit ""
  printf '%s' "$F" | jq -r '.family_notes[]? | "- " + .' >> "$OUT_TMP"
  printf '%s' "$F" | jq -e '.family_notes | length > 0' >/dev/null 2>&1 && emit ""

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
    (if .observed.defects then ("- **Defects observed in v2:**\n" + (.observed.defects | map("  - " + .) | join("\n"))) else empty end),
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
emit "## Families outstanding"
emit ""
emit "Not yet authored. Named individually rather than counted, so a family that quietly never gets written is visible as a gap rather than absent from a total."
emit ""
jq -r '.families_outstanding[]? | "- `" + . + "`"' "$IN" >> "$OUT_TMP"
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

# Only now, with the whole view rendered, does the committed file change.
mv "$OUT_TMP" "$OUT" || die "cannot write: $OUT"
trap - EXIT

echo "ok: rendered $(jq -r '[.families[].entries | length] | add' "$IN") entries across $FAMILY_COUNT family(s) -> $OUT" >&2
