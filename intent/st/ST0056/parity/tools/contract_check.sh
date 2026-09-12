#!/bin/bash
# contract_check.sh -- does every shipped field have a contract row, and every
#                      contract row a shipped field?
#
# THE PROBLEM (ST0069 WP-16). Two artefacts describe the same entities:
#
#   schema/*.schema.json     GENERATED from the Rust types -- what actually ships
#   intent/st/ST0056/data-model.md   AUTHORED prose -- what we said would ship
#
# Nothing has ever compared them. A property can be added to a type, published
# into the schema face, and reach API consumers with NO contract row anywhere --
# that is `issue.body`, which looked specified while it was homeless. And a row
# can describe a field nobody implemented, which is the more dangerous half: a
# reader trusts the document, and the document is the only thing that was ever
# wrong.
#
# BOTH DIRECTIONS, PER ENTITY, because they fail differently and a one-sided
# check leaves the untested direction free to be a constant pass.
#
# IT IS A SIBLING OF `drift_check.sh` AND NEVER AN ARM OF IT. That check
# compares generated faces to the Rust types. This one compares a PROSE
# DOCUMENT to a SCHEMA. Folding them puts two questions behind one exit code,
# and the remedies differ: drift wants a regeneration, this wants an author.
#
# ==========================================================================
# THE POPULATION IS DERIVED AT RUN TIME. IT IS NEVER ENUMERATED HERE.
# ==========================================================================
#
# AC-16.2 is explicit about this and it is explicit because the criterion
# itself got it wrong: the row carried a nine-name list until 2026-08-29, and
# derivation yielded eleven -- the two it omitted being `Invoker` and `Subject`.
# So the criterion warning against "a list maintained beside it" WAS that list,
# in the same sentence. Measured again on 2026-09-12 it yields twelve. The
# schema grew and no list would have.
#
#   SCHEMA SIDE -- every object-typed definition with properties, plus each
#   face's root when the root itself has properties, read from the three
#   published faces at run time.
#
#   DOCUMENT SIDE -- every heading in `data-model.md` whose first following
#   table is a field table (first column header `Field` or `Entity`). Also
#   derived: `file_index` and `doc_section` describe themselves in inline prose
#   rather than a table, so they are not entity tables and this check does not
#   claim them. That is a property of the document, measured, not a decision
#   taken here.
#
# ==========================================================================
# THE JOIN IS A DECLARED MAP, AND IT IS THE ONE THING HERE THAT IS A LIST
# ==========================================================================
#
# **THE TWO SIDES DO NOT SHARE A NAMING CONVENTION AND CANNOT BE JOINED BY
# NAME.** The schema is PascalCase per JSON entity (`Thread`, `AcceptanceTest`,
# `Envelope`); the document is snake_case per DB entity (`steel_thread`,
# `acceptance_test`, `event_log`), and some entities sit at `#####` depth under
# a parent rather than at `###`. A name-match would report `Envelope` as
# homeless while its table sat at `event_log` with all seven rows present --
# which is exactly the false finding this map exists to stop, and exactly the
# false finding the author of this script filed before writing it.
#
# So the join is DECLARED, once, here. One map, one home.
#
#   - A schema entity with NO map entry REFUSES. That is the entity-level form
#     of a property with no row, and it is why the map cannot rot quietly: a
#     new published entity fails this check on the day it ships.
#   - A map entry whose table cannot be FOUND or PARSED refuses loudly, naming
#     the entity and the table (AC-16.3). `data-model.md` is hand-authored
#     markdown with prose between its tables, so an unparseable table is the
#     likely case rather than the exotic one, and a skipped entity reported as
#     checked is the defect this check exists to remove, one level up.
#   - A document entity table that NO map entry claims REFUSES. Same rule from
#     the other side.
#
# A `-` in the schema column declares a document table with no published face.
# **IT IS NOT AN EXCUSE LIST AND IT IS NOT SCOPED TO WHAT IS ALREADY CLEAN.**
# Each entry carries a class and a reason. `pending:` is RED -- the contract is
# owed and the check says so every run until it lands. `never:` is reported in
# its own bucket and does not gate. The ratio to watch is drift_check.sh's:
# if a later run produces only `never:`, this check has stopped working.
#
# ==========================================================================
# WHAT THIS FILE IS THE WITNESS FOR (ST0069 WP-16)
# ==========================================================================
#
#   AT-16.1  covers AC-16.1 -- the join, refusing in BOTH directions: a
#            property with no row, and a row with no property.
#   AT-16.2  covers AC-16.2 -- every modelled entity checked, denominator
#            printed, population DERIVED at run time and enumerated nowhere.
#   AT-16.3  covers AC-16.3 -- a table the parser cannot read fails loudly,
#            naming the entity and the table, and is never skipped.
#   AT-16.4  covers AC-16.4 -- driven RED in both directions on planted
#            defects before any green was trusted; recipe below.
#
# ==========================================================================
# HOW TO DRIVE IT RED, WHICH IS THE ONLY REASON TO TRUST IT GREEN (AC-16.4)
# ==========================================================================
#
# `MODEL` points the check at a copy, so the demonstrations plant defects in a
# scratch file and never touch the shared document. Each arm names the finding
# it must produce; an arm that produces a DIFFERENT finding has not fired.
#
#   cp intent/st/ST0056/parity/tools/../../data-model.md /tmp/base.md
#
#   arm 1, a shipped property loses its row:
#     grep -v '^| slug ' /tmp/base.md > /tmp/arm1.md
#     MODEL=/tmp/arm1.md contract_check.sh
#     -> Thread.slug -- SHIPPED WITH NO CONTRACT ROW
#
#   arm 2, a row describes a property nothing ships:
#     awk '/^\| slug /{print "| phantom_field | string | planted |"} {print}' \
#       /tmp/base.md > /tmp/arm2.md
#     MODEL=/tmp/arm2.md contract_check.sh
#     -> Thread.phantom_field -- CONTRACT ROW WITH NO SHIPPED PROPERTY
#
#   arm 3 (AC-16.3), a mapped table the parser cannot read: delete the field
#     rows from `related`'s table and leave the heading.
#     -> REFUSED (AC-16.3): entity `Related` maps to table `related` ...
#
#   The row-shaped reader is driven the same way, because a code path that has
#   never been driven is exactly what this criterion is about:
#     drop `fyi` from the wb_message cell   -> WbMessage.fyi SHIPPED WITH NO CONTRACT ROW
#     add a name to the wb_item cell        -> WbItem.<name> CONTRACT ROW WITH NO SHIPPED PROPERTY
#     rename the `wb_node` row label        -> REFUSED (AC-16.3), the mapped row is not found
#
# **BOTH ARMS ARE REQUIRED AND THE REASON IS NOT SYMMETRY FOR ITS OWN SAKE.** A
# one-sided demonstration leaves the untested direction free to be a constant
# pass, and a constant pass is indistinguishable from agreement in every run
# that follows.
#
# **AND THE EXIT CODE IS NOT THE DISCRIMINATOR WHILE ANY `pending:` ROW
# STANDS.** The unplanted document already exits 1 on the whiteboard entities,
# so an arm is judged by the finding LINE it adds, never by the code. The
# control that makes the arms mean something is that the planted line is ABSENT
# from the unplanted run -- driven, not assumed.
#
# Exit 0 when the two descriptions agree, 1 when they do not, 2 on an
# environment or usage error. Non-zero at 1 is NOT a failure of this script; it
# is the finding, and it wants an author.

set -uo pipefail

die() { echo "error: $1" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$HERE/../../../../.." && pwd)"

MODEL="${MODEL:-$REPO_ROOT/intent/st/ST0056/data-model.md}"
FACES="${FACES:-$REPO_ROOT/schema/thread.schema.json $REPO_ROOT/schema/issue.schema.json $REPO_ROOT/schema/event.schema.json $REPO_ROOT/schema/board.schema.json}"

[ -f "$MODEL" ] || die "data-model.md not found: $MODEL"
for f in $FACES; do [ -f "$f" ] || die "published face not found: $f"; done

# ==========================================================================
# THE DECLARED MAP -- <json entity>|<data-model heading>|<class>:<reason>
#
# The heading is the first token of the heading line, backticks stripped. The
# third field is present ONLY on `-` rows (a document table with no face).
# ==========================================================================
MAP="$(cat <<'EOF'
Thread|steel_thread
WorkPackage|work_package
Criterion|acceptance_criterion
AcceptanceTest|acceptance_test
Attachment|attachment
Related|related
Legacy|legacy
Issue|issue
Envelope|event_log
FiatRecord|fiat_record
Invoker|invoker
Subject|subject
WbNode|wb_node#wb_node
WbItem|wb_node#wb_item
WbMessage|wb_node#wb_message
Board|board
-|project|never:`intent/.config/config.json` is CONFIGURATION read by `Config`, not a canon entity with a published JSON face. Its contract is the file format itself, and its catch-all row (`Config::extra`, carried verbatim) means a property set could not be closed even in principle. RATIFIED by vc under the pen, 2026-09-12, on this line's own reasoning. Declared here rather than skipped silently: it is reported every run and it never gates.
EOF
)"

TMP="$(mktemp -d)" || die "could not create a temporary directory"
trap 'rm -rf "$TMP"' EXIT

# ---------------------------------------------------------------- schema side
# Every object-typed definition carrying properties, plus each face's root when
# the root has properties of its own. Emitted as <entity>\t<property>.
: > "$TMP/schema.tsv"
for f in $FACES; do
  jq -r '
    def objs:
      ( if (.properties // null) != null
        then [{ name: (.title // "ROOT"), props: (.properties | keys) }]
        else [] end )
      + [ ((.["$defs"] // .definitions // {}) | to_entries[])
          | select(.value.properties != null)
          | { name: .key, props: (.value.properties | keys) } ];
    objs[] | .name as $n | .props[] | "\($n)\t\(.)"
  ' "$f" >> "$TMP/schema.tsv" || die "could not read the published face: $f"
done
[ -s "$TMP/schema.tsv" ] || die "the published faces yielded no entities at all -- refusing rather than reporting a clean run"
cut -f1 "$TMP/schema.tsv" | LC_ALL=C sort -u > "$TMP/schema.entities"

# -------------------------------------------------------------- document side
# A heading whose FIRST following table is a field table is an entity table.
# Everything else in the document -- the state machines, the ratified transition
# tables, the prose -- is not, and is not reported as missing.
awk '
  /^#{3,5} / {
    line = $0
    sub(/^#+ +/, "", line)
    split(line, w, / /)
    name = w[1]
    gsub(/`/, "", name)
    gsub(/[^A-Za-z0-9_].*$/, "", name)
    heading = name; hline = NR; seen_table = 0
    next
  }
  /^\|/ {
    if (heading != "" && seen_table == 0) {
      seen_table = 1
      first = $0
      sub(/^\| */, "", first)
      sub(/ *\|.*$/, "", first)
      gsub(/^ +| +$/, "", first)
      if (first == "Field" || first == "Entity") {
        printf "%s\t%d\t%d\n", heading, hline, NR
      }
      heading = ""
    }
    next
  }
' "$MODEL" > "$TMP/doc.tables" || die "could not scan $MODEL for entity tables"
[ -s "$TMP/doc.tables" ] || die "no entity table was found in $MODEL at all -- refusing rather than reporting a clean run"
cut -f1 "$TMP/doc.tables" | LC_ALL=C sort -u > "$TMP/doc.entities"

# Read one entity table's field names off the document, given its header line.
# Refuses (empty output, non-zero) rather than guessing when the table is not
# the shape a field table is.
# TWO TABLE SHAPES, AND THE SECOND IS NOT A CONCESSION. A `Field`-headed table
# is one row per property. An `Entity`-headed table is one row per ENTITY with
# its properties in a cell -- which is how the coordination entities are
# written, three of them under one heading, because they are three shapes of one
# thing and splitting them would separate rows that are read together. A map
# entry addresses that shape as `<heading>#<row>`.
#
# **THE CELL IS PARSED BY ITS FIRST BACKTICKED TOKEN PER COMMA-SEPARATED
# ELEMENT**, so `status` (`active . paused`) yields `status` and not its value
# list, and the optional and array markers (`?`, `[]`) are stripped -- they are
# the document saying what the schema says with `type` and `required`, not part
# of a name.
read_row_fields() {
  awk -v start="$1" -v want="$2" '
    NR <= start { next }
    /^\|/ {
      line = $0
      sub(/^\| */, "", line)
      n = split(line, cell, / *\| */)
      label = cell[1]; gsub(/`/, "", label); gsub(/^ +| +$/, "", label)
      if (label ~ /^-+$/) { next }
      if (label != want) { next }
      body = cell[2]
      m = split(body, parts, /, /)
      for (i = 1; i <= m; i++) {
        if (match(parts[i], /`[^`]+`/)) {
          f = substr(parts[i], RSTART + 1, RLENGTH - 2)
          gsub(/\?$/, "", f); gsub(/\[\]$/, "", f)
          if (f != "") print f
        }
      }
      exit
    }
    /^[^|]/ { if (seen) exit; }
  ' "$MODEL"
}

read_table_fields() {
  awk -v start="$1" '
    NR <= start { next }
    /^\|/ {
      body = $0
      sub(/^\| */, "", body)
      sub(/ *\|.*$/, "", body)
      gsub(/^ +| +$/, "", body)
      if (body ~ /^-+$/) { next }
      gsub(/`/, "", body)
      gsub(/\\_/, "_", body)
      gsub(/~~/, "", body)
      if (body == "") { next }
      print body
      next
    }
    { if (started) exit; }
    /^[^|]/ { started = 1 }
  ' "$MODEL"
}

# ------------------------------------------------------------------ the check
FINDINGS=0
CHECKED=0
PROPS=0
DECLARED_NEVER=""
note() { printf '%s\n' "$1"; }
finding() { FINDINGS=$((FINDINGS + 1)); printf '%s\n' "$1"; }

faces_display=""
for f in $FACES; do faces_display="$faces_display ${f#$REPO_ROOT/}"; done
echo "contract_check: schema faces --$faces_display"
echo "contract_check: contract document -- ${MODEL#$REPO_ROOT/}"
echo

# 1. Every schema entity must be mapped.
while IFS= read -r ent; do
  row="$(printf '%s\n' "$MAP" | awk -F'|' -v e="$ent" '$1 == e {print; exit}')"
  if [ -z "$row" ]; then
    finding "REFUSED: schema entity \`$ent\` has no entry in the declared map -- a published entity with no contract row, at the entity level. Add it to the map and give it a table."
  fi
done < "$TMP/schema.entities"

# 2. Every document entity table must be claimed.
while IFS= read -r heading; do
  row="$(printf '%s\n' "$MAP" | awk -F'|' -v h="$heading" '{split($2, a, "#"); if (a[1] == h) {print; exit}}')"
  if [ -z "$row" ]; then
    finding "REFUSED: contract table \`$heading\` in ${MODEL##*/} is claimed by no schema entity and is not declared faceless -- a contract describing something nobody implemented."
  fi
done < "$TMP/doc.entities"

# 3. Declared-faceless rows: `pending:` is red, `never:` is reported.
while IFS='|' read -r ent heading rest; do
  [ "$ent" = "-" ] || continue
  class="${rest%%:*}"; reason="${rest#*:}"
  case "$class" in
    pending) finding "PENDING (red by construction): \`$heading\` -- $reason" ;;
    never)   DECLARED_NEVER="$DECLARED_NEVER
  declared faceless -- \`$heading\`: $reason" ;;
    *)       finding "REFUSED: map row for \`$heading\` declares no class; a \`-\` row must say \`pending:\` or \`never:\` and why." ;;
  esac
done <<EOF
$MAP
EOF

# 4. Per mapped entity, both directions.
while IFS='|' read -r ent heading rest; do
  [ "$ent" = "-" ] && continue
  [ -n "$ent" ] || continue

  if ! grep -q "^$ent$" "$TMP/schema.entities"; then
    finding "REFUSED: the map claims schema entity \`$ent\` and no published face defines it -- the map has rotted, or the entity was withdrawn without its row."
    continue
  fi

  row=""
  where="$heading"
  case "$heading" in
    *"#"*) row="${heading#*#}"; heading="${heading%%#*}" ;;
  esac

  tline="$(awk -F'\t' -v h="$heading" '$1 == h {print $3; exit}' "$TMP/doc.tables")"
  if [ -z "$tline" ]; then
    finding "REFUSED (AC-16.3): entity \`$ent\` maps to table \`$where\` in ${MODEL##*/}, and no field table was found under that heading. Not skipped, not assumed clean."
    continue
  fi

  if [ -n "$row" ]; then
    read_row_fields "$tline" "$row" | LC_ALL=C sort -u > "$TMP/doc.fields"
  else
    read_table_fields "$tline" | LC_ALL=C sort -u > "$TMP/doc.fields"
  fi
  if [ ! -s "$TMP/doc.fields" ]; then
    finding "REFUSED (AC-16.3): entity \`$ent\` maps to table \`$where\` at line $tline, and the table could not be parsed into field names. The refusal names the entity and the table rather than reporting it checked."
    continue
  fi

  awk -F'\t' -v e="$ent" '$1 == e {print $2}' "$TMP/schema.tsv" | LC_ALL=C sort -u > "$TMP/schema.fields"

  missing_row="$(LC_ALL=C comm -23 "$TMP/schema.fields" "$TMP/doc.fields")"
  missing_prop="$(LC_ALL=C comm -13 "$TMP/schema.fields" "$TMP/doc.fields")"

  n_props="$(wc -l < "$TMP/schema.fields" | tr -d ' ')"
  PROPS=$((PROPS + n_props))
  CHECKED=$((CHECKED + 1))

  if [ -n "$missing_row" ]; then
    while IFS= read -r p; do
      [ -n "$p" ] && finding "$ent.$p -- SHIPPED WITH NO CONTRACT ROW (in the schema, absent from \`$where\`)"
    done <<EOF
$missing_row
EOF
  fi
  if [ -n "$missing_prop" ]; then
    while IFS= read -r p; do
      [ -n "$p" ] && finding "$ent.$p -- CONTRACT ROW WITH NO SHIPPED PROPERTY (in \`$where\`, absent from the schema)"
    done <<EOF
$missing_prop
EOF
  fi
done <<EOF
$MAP
EOF

# ----------------------------------------------------------------- the verdict
# THE DENOMINATOR IS PRINTED WHETHER OR NOT ANYTHING WAS FOUND (AC-16.2),
# because a check that silently skipped an entity and a check that found
# nothing produce the same line otherwise.
MAPPED="$(printf '%s\n' "$MAP" | awk -F'|' '$1 != "-" && $1 != "" {n++} END {print n+0}')"
DECLARED="$(wc -l < "$TMP/schema.entities" | tr -d ' ')"

echo
if [ -n "$DECLARED_NEVER" ]; then
  echo "contract_check: declared faceless, reported and not gating --$DECLARED_NEVER"
  echo
fi
echo "contract_check: $CHECKED of $MAPPED mapped entities checked, $PROPS properties; $DECLARED entities derived from the published faces at run time."

if [ "$FINDINGS" -eq 0 ]; then
  echo "contract_check: every shipped property has a contract row and every contract row has a shipped property."
  exit 0
fi
echo "contract_check: $FINDINGS finding(s). This is the finding, not a failure of the check; it wants an author."
exit 1
