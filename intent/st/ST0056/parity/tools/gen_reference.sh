#!/usr/bin/env bash
# gen_reference.sh -- the per-family command reference, emitted from the register.
#
# **A HAND-TYPED REFERENCE BESIDE `surface/dispatch-table.json` IS A TRANSCRIBED
# COPY OF A MEASURED MAPPING**, and `parity.md` already struck a column for
# being exactly that. The register declares every verb's path, args, flags,
# aliases, recoverability, MCP exposure and read-or-mutate; the clap surface is
# BUILT from it (`dispatch.rs:45`, `include_str!`), so the register at a
# revision is a sound statement of what that revision's binary exposes. Typing
# that out by hand produces a second home that drifts from the first, silently,
# and the docs' own index says so in its second paragraph.
#
# **EVERY PAGE IS KEYED TO A REVISION AND SAYS WHICH.** Same reason as
# `gen_cut_surface.sh`, which is its sibling: the register describes `main`, a
# release is a point on `main`, and `ac edit` / `at edit` are declared shipped at
# HEAD and exist in no published tag. A page that does not name its revision is
# true when written and silently false at the next merge.
#
# ==========================================================================
# THE PART TO READ BEFORE CHANGING WHAT THIS EMITS
# ==========================================================================
#
# **`observed` IS v2's BEHAVIOUR, NOT v3's, AND ON 28 ROWS IT IS v2's DEFECT.**
# This is the trap the whole generator is shaped around. A row carries
# `observed.exit` -- a measured list of v2's exit codes -- and separately a
# `target.state` saying what v3 does about it. Publishing `observed.exit` as
# "the exit codes" would document a defect the rewrite exists to fix, on 28
# rows, in the voice of a reference manual.
#
# So exit codes are emitted ONLY where the register supports the claim:
#
#   as-observed  51 rows, all with `observed.exit`. The state means "v3
#                reproduces what v2 was measured doing", so the measured list IS
#                v3's contract. Emitted.
#   corrected    28 rows. All 28 carry v2's `observed.exit`; only 10 carry a
#                `target.behaviour` saying what v3 does instead. The 10 emit the
#                prose; **the other 18 emit an honest gap and no exit codes.**
#   new-surface  32 rows, 31 with no exit data at all. Nothing to emit.
#   pending-hv   5 rows. The scope call is open; a guess here would read as a
#                decision, which is the honest-blank convention this table
#                already declares in its own about block.
#
# **THE HEADLINE MEASUREMENT, AND IT IS OWED UPWARDS RATHER THAN HIDDEN HERE:
# of 118 shipped rows the register can support an exit contract for 61.** That
# is not a flaw in this generator; it is the register's coverage, made visible
# by asking it a question a reader would ask. Each page carries its own tally.
#
# **REFUSALS HAVE NO DECLARED HOME IN THE REGISTER (issue 0142's structural
# half).** A row declares path, args, flags, target, disposition,
# recoverability, MCP exposure and read-or-mutate, and nothing about what it
# REFUSES -- so the only place a refusal can be stated is prose in a `help`
# string, which nothing checks and nothing updates when the behaviour moves.
# Four of 118 shipped rows do exactly that and **one of them is measured false**
# (`at green`, "reachable only from red", which v3 does not enforce). The pages
# therefore quote `help` as the tool's own words and flag every row whose help
# asserts a precondition, rather than presenting any of it as a contract.
#
# **THAT DETECTOR IS A FLOOR AND THE PAGES SAY SO.** It is a regex over English
# -- `only from`, `refuses`, `requires`, `must`, `cannot`, `unless` -- so a
# precondition phrased without one of those words is invisible to it. It is
# reproduced here rather than imported because 0142 records the same regex and
# the same population of four; a page emitting a different number from the issue
# would be two measurements of one thing.
#
# ==========================================================================
# THE SPLIT INTO PAGES, WHICH IS MECHANICAL AND NOT A JUDGEMENT
# ==========================================================================
#
# **PAGE FILENAMES CARRY AN `intent-` PREFIX, AND THAT IS NOT DECORATION.** The
# obvious naming -- one page per family, named for the family -- puts
# `claude.md` in the doc tree, and on a case-insensitive filesystem that IS
# `CLAUDE.md`: Claude Code discovers agent instructions by basename, so a
# generated reference page silently becomes project instructions loaded into
# every session in the repository. **Measured 2026-08-29 by it happening** --
# the `intent claude` page was written and came straight back as an
# instruction file. It is a class and it was already at three: `agents.md`
# against `AGENTS.md` and `modules.md` against `MODULES.md` are the same
# collision, differing only in being harmless today. The prefix removes the
# class rather than special-casing the one member that bites, which is what a
# reserved-name exception list would have done -- and that list would need
# maintaining for every convention any future consumer invents.
#
# One page per family that declares MORE THAN ONE entry. Every other shipping
# row -- the single-verb families (`doctor`, `init`, `version` ...) and the 11
# top-level `new_surface[]` rows (`search`, `sync`, `export` ...) -- goes on one
# `commands.md`, because from the reader's side they are the same object: a
# top-level command with no subcommands. **No hand-maintained list of families
# appears anywhere in this file**, which is the point: a family added to the
# register grows a page without anyone remembering to add one.

# inputs: surface/dispatch-table.json intent/st/ST0056/parity/tools/lib_surface.sh intent/st/ST0056/parity/tools/lib_mdfmt.sh
# inputs-exempt: REV -- a git revision. The whole point of this generator is that it reads its inputs AT a named commit rather than from the working tree, so the revision is the one input that cannot be a tracked file: it is the coordinate the tracked files are read at. Re-derivable by `git rev-parse`.

set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(git -C "$HERE" rev-parse --show-toplevel)"
# shellcheck source=lib_surface.sh
. "$HERE/lib_surface.sh"
# shellcheck source=lib_mdfmt.sh
. "$HERE/lib_mdfmt.sh"

REV="HEAD"
BASELINE="v3.0.0"
OUTDIR="$ROOT/docs/reference"
GENERATOR="intent/st/ST0056/parity/tools/gen_reference.sh"

usage() {
  cat >&2 <<'USAGE'
usage: gen_reference.sh [--rev <rev>] [--baseline <rev>] [--out <dir>]

  --rev       the revision the pages are ABOUT (default: HEAD)
  --baseline  the released revision presence is reported against (default: v3.0.0)
  --out       write pages here (default: docs/reference)

Emits one page per multi-verb family plus `commands.md` for the single-verb and
top-level rows. Every page names the revision it was generated from.
USAGE
  exit 2
}

while [ $# -gt 0 ]; do
  case "$1" in
    --rev)      REV="${2:?--rev needs a revision}"; shift 2 ;;
    --baseline) BASELINE="${2:?--baseline needs a revision}"; shift 2 ;;
    --out)      OUTDIR="${2:?--out needs a directory}"; shift 2 ;;
    -h|--help)  usage ;;
    *)          echo "error: unknown argument \`$1\`" >&2; usage ;;
  esac
done

TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

REV_SHA="$(surface_resolve_rev "$REV")"
surface_table_at "$REV_SHA" "$TMP/rev.json"

# The baseline is what "is this verb in the release?" is asked against. It is
# allowed to be absent -- an estate with no published tag is a real state -- and
# the pages then say so rather than reporting every verb as missing, which is
# what an empty list would have made them do.
BASE_SHA=""
BASE_SHIPPED="[]"
if BASE_SHA="$(surface_resolve_rev "$BASELINE" 2>/dev/null)" \
   && surface_table_at "$BASE_SHA" "$TMP/base.json" 2>/dev/null; then
  BASE_SHIPPED="$(DISPATCH_TABLE="$TMP/base.json" surface_shipped | jq -R . | jq -sc .)"
else
  echo "note: baseline \`${BASELINE}\` has no register; pages will not report release presence" >&2
  BASE_SHA=""
fi

WHEN="$(date -u +'%Y-%m-%d %H:%MZ')"

# The shipped population, read from `.populations` through the library rather
# than walked here. `.families[].entries[]` is too narrow and too wide at once
# and the same hand-written jq produced the same wrong population five times in
# one week; a sixth walk in this file would be the sixth instance.
DISPATCH_TABLE="$TMP/rev.json" surface_shipped | LC_ALL=C sort > "$TMP/shipped"

# ==========================================================================
# THE RENDERER
# ==========================================================================
# jq rather than shell: the input is structured and the output is text, which is
# the one shape jq is actually for. A shell loop over 129 rows re-parsing the
# table per row is how the earlier inline walks got their populations wrong.
read -r -d '' JQ_LIB <<'JQEOF' || true
def bt: "`" + . + "`";
def nz: if . == null or . == "" then null else . end;

# A cell that must survive the markdown table. A pipe inside a value ends the
# cell and shifts every column after it, silently -- the row still renders.
#
# **AND A RAW `<name>` IS PARSED AS AN HTML TAG AND RENDERS AS NOTHING**, which
# drops text off the page with no error anywhere. The register's text fields are
# markdown -- 132 of the exit-code notes carry backticks -- so a blanket escape
# would corrupt every code span that legitimately contains `<ID>`. The escape is
# applied ONLY outside code spans: split on the backtick, and the even-indexed
# segments are what the renderer treats as prose. An odd number of backticks in
# one value would misalign that parity; no such value exists in the register
# today, and this comment is what the next one meets.
def md_prose:
  (. // "") | tostring | split("`") | to_entries
  | map(if (.key % 2) == 0 then (.value | gsub("<"; "&lt;")) else .value end)
  | join("`");

def cell: md_prose | gsub("\\|"; "\\|") | gsub("\n"; " ");

def usage_arg:
  .name as $n
  | (if .arity == "0..1" then "[<" + $n + ">]"
     elif .arity == "0..n" then "[<" + $n + ">...]"
     elif .arity == "1..n" then "<" + $n + ">..."
     else "<" + $n + ">" end);

# Flags dispositioned `retire` were recorded from v2 and never reach clap, so
# documenting them would invent surface. `intrinsic` is clap's own `--help`,
# stated once per page instead of on every row.
def live_flags: [ (.flags // [])[] | select((.disposition // "keep") == "keep") ];

def usage_line:
  "intent " + .path
  + ([ (.args // [])[] | " " + usage_arg ] | join(""))
  + (if (live_flags | length) > 0 then " [flags]" else "" end);

# Issue 0142's detector, reproduced rather than imported: the issue records this
# regex and the population of four it finds, and a page reporting a different
# number would be a second measurement of one thing.
def asserts_precondition:
  ((.help // "") | test("only from|refuses|requires|must |cannot|unless"; "i"));

def presence($base; $has_base):
  if ($has_base | not) then "--"
  elif (.path as $p | $base | index($p)) then "yes"
  else "**no -- newer than the release**" end;

def arg_rows:
  [ (.args // [])[]
    | "| " + (.name | bt | cell)
      + " | " + (.type | bt | cell)
      + " | " + (if (.arity | startswith("0")) then "no" else "yes" end)
      + " | " + ([ (.values // [])[] | bt ] | join(", ") | cell)
      + " | " + ((.note // .default // "") | cell)
      + " |" ];

def takes:
  if .type == "bool" then "no"
  else ((.value // .accepts // .type) | tostring) as $v
    | if ($v | test("\\|")) then ([ $v | split("|")[] | bt ] | join(" or "))
      else ($v | bt) end
  end;

def flag_rows:
  [ live_flags[]
    | "| " + ([ .spellings[] | bt ] | join(", ") | cell)
      + " | " + (takes | cell)
      + " | " + (if (.default // "") == "" then "" else ((.default | tostring) | bt | cell) end)
      + " | " + ((.help // "") | cell)
      + " |" ];

# **THE HONEST-EXIT RULE.** See the header. `observed` is v2 and on 28 rows it is
# v2's defect, so it is published only where `target.state` warrants it.
def exit_block:
  (.target.state // "none") as $s
  | if $s == "as-observed" and ((.observed.exit // []) | length) > 0 then
      ["**Exit codes.** v3 reproduces the behaviour measured on v2 for this verb.", "",
       "| Code | When |", "| --- | --- |"]
      + [ (.observed.exit)[] | "| " + (.code | tostring) + " | " + (.when | cell) + " |" ]
    elif $s == "corrected" and ((.target.behaviour // "") != "") then
      ["**v3 corrects v2 here.** " + (.target.behaviour)]
    elif $s == "corrected" then
      ["**The register records that v3 corrects v2 on this verb and does not record what it corrects it TO.** The measured exit codes it carries are v2's, and v2's are what the correction exists to change, so they are not reproduced here. `intent " + .path + " --help`, and your own binary, are the authority."]
    elif $s == "new-surface" then
      ["**New in v3**, so there is no measured antecedent and the register carries no exit contract for it."]
    elif $s == "pending-hv" then
      ["**The scope call on this verb is open.** The register records an honest blank rather than a guess, and so does this page."]
    elif $s == "undefined" then
      ["**v2 exhibited no defined behaviour here**, so there is nothing for v3 to be faithful to and no measured contract to quote."]
    elif $s == "deviate" and ((.target.behaviour // "") != "") then
      ["**v3 deliberately diverges from v2 here.** " + (.target.behaviour)]
    else
      ["The register carries no v3 exit contract for this verb."]
    end;

def has_exit_contract:
  (.target.state // "none") as $s
  | ($s == "as-observed" and ((.observed.exit // []) | length) > 0)
    or ($s == "corrected" and ((.target.behaviour // "") != ""))
    or ($s == "deviate" and ((.target.behaviour // "") != ""));

def verb_section($base; $has_base):
  ["## " + ("intent " + .path | bt), ""]
  + [ (.help // "_The register records no description for this verb._") ]
  + (if asserts_precondition then
       ["",
        "**That description asserts a precondition, and nothing checks it.** The register has no structural home for refusals, so a refusal can only be stated as prose in a help string -- which is a contract nothing verifies and nothing updates when the behaviour moves. One of the four such strings in the surface is measured false. See issue `0142`; treat this line as a description, not a guarantee."]
     else [] end)
  + ["", "```", (usage_line), "```", ""]
  + (if (.aliases // []) | length > 0 then
       ["Also spelled " + ([ (.aliases)[] | bt ] | join(", ")) + ".", ""] else [] end)
  + ["| | |", "| --- | --- |",
     "| In " + (if $has_base then ($baseline | bt) else "the release" end) + " | " + (presence($base; $has_base)) + " |",
     "| Reads or writes | " + ((.read_or_mutate // "not recorded") | cell) + " |",
     "| Undo | " + (if .recoverability then (.recoverability | cell)
                        elif (.read_or_mutate == "read") then "not applicable -- the register records this verb as a read"
                        else "not recorded" end) + " |",
     "| Over MCP | " + (if .exposed_on_mcp then "yes" else "no" end) + " |", ""]
  + (if ((.args // []) | length) > 0 then
       ["**Arguments**", "",
        "| Argument | Type | Required | Values | Notes |",
        "| --- | --- | --- | --- | --- |"] + arg_rows + [""]
     else [] end)
  + (if (live_flags | length) > 0 then
       ["**Flags**", "",
        "| Flag | Takes | Default | What it does |",
        "| --- | --- | --- | --- |"] + flag_rows + [""]
     else [] end)
  + exit_block
  + [""];
JQEOF

# The page renderer. `$rows` is the set of entries this page is about; every
# figure on the page is computed from that set rather than from the whole table,
# so a per-page tally can never quietly become an estate-wide one.
read -r -d '' JQ_PAGE <<'JQEOF' || true
  ($rows | map(select(.disposition != "retire"))) as $live
  | ($rows | map(select(.disposition == "retire"))) as $dead
  | ($live | map(select(has_exit_contract)) | length) as $with_exit
  | ($live | map(select(asserts_precondition)) | length) as $preconditions
  | ([ "# " + $title, "",
       "**Generated by " + ($gen | bt) + " at " + $when + ". Do not edit -- re-run it.**", "",
       (if $blurb == "" then empty else $blurb end),
       (if $blurb == "" then empty else "" end),
       "| | |", "| --- | --- |",
       "| Revision this describes | " + ($revsha | bt) + " (" + ($rev | bt) + ") |",
       "| Release presence is reported against | " + (if $has_base then ($baseline | bt) else "none -- no register at the baseline" end) + " |",
       "| Commands on this page | " + ($live | length | tostring) + " |",
       "| ...with a stated exit contract | " + ($with_exit | tostring) + " |",
       "",
       "**The register is a declaration, not a behaviour claim.** The command surface is built from `surface/dispatch-table.json`, so the register at a revision states what that revision exposes. It does not state that any of it works. Where this page and your binary disagree, **your binary is right**.",
       "" ]
     + (if $with_exit < ($live | length) then
          [ "**Exit codes are stated for " + ($with_exit | tostring) + " of these " + ($live | length | tostring) + " commands, and that is the register's coverage rather than an omission here.** A row carries the exit codes measured on _v2_; whether v3 reproduces them is a separate field. Where the register says v3 corrects v2 without recording what it corrects it to, this page says so instead of reprinting the behaviour the rewrite exists to remove.", "" ]
        else [] end)
     + (if $preconditions > 0 then
          [ "**" + ($preconditions | tostring) + " description" + (if $preconditions == 1 then " on this page asserts" else "s on this page assert" end) + " a precondition, flagged inline below.** Refusals have no declared home in the register, so they can only be stated as prose in a help string. The detector that finds them is a regex over English and is therefore a floor, not a ceiling: a precondition phrased without one of its words is invisible to it. Issue `0142`.", "" ]
        else [] end)
     + [ "## The commands", "",
         "| Command | What it does | In " + (if $has_base then $baseline else "release" end) + " | Reads or writes | Undo |",
         "| --- | --- | --- | --- | --- |" ]
     + [ $live[] | "| " + (("intent " + .path) | bt | cell)
                   + " | " + ((.help // "") | cell)
                   + " | " + (presence($base; $has_base))
                   + " | " + ((.read_or_mutate // "--") | cell)
                   + " | " + ((.recoverability // "--") | cell) + " |" ]
     + [ "" ]
     + (if ($dead | length) > 0 then
          [ "## Retired", "",
            "These do not exist in v3. They refuse with an exit code that distinguishes _removed_ from _never built_, so a v2-era script gets an answer rather than an unknown-command error.", "",
            "| Command | What it did in v2 |", "| --- | --- |" ]
          + [ $dead[] | "| " + (("intent " + .path) | bt | cell) + " | " + ((.help // "") | cell) + " |" ]
          + [ "" ]
        else [] end)
     + ($live | map(verb_section($base; $has_base)) | add)
     + [ "## Refusals", "",
         "**There is no per-command refusal list, because the register does not carry one.** A row declares its path, arguments, flags, target, disposition, recoverability, MCP exposure and read-or-mutate, and nothing structural about what it refuses. Where a command's exit codes appear above, those _are_ its refusal contract; where they do not, the register has nothing to publish.",
         "",
         "The estate-wide roster of refusal variants is in [what Intent ships at this revision](cut-surface.md), taken from a roster that declares itself exhaustive and is checked as one. **It is not attributed to commands anywhere**, and attributing it by reading the variant names would be a guess dressed as a reference." ])
  | .[]
JQEOF

render_page() {
  local rows_file="$1" title="$2" blurb="$3" out="$4"
  jq -r \
    --slurpfile rows "$rows_file" \
    --argjson base "$BASE_SHIPPED" \
    --argjson has_base "$( [ -n "$BASE_SHA" ] && echo true || echo false )" \
    --arg title "$title" \
    --arg blurb "$blurb" \
    --arg gen "$GENERATOR" \
    --arg when "$WHEN" \
    --arg rev "$REV" \
    --arg revsha "$REV_SHA" \
    --arg baseline "$BASELINE" \
    "$JQ_LIB"'
     ($rows[0]) as $rows | '"$JQ_PAGE" \
    --null-input > "$TMP/raw.md"
  md_align "$TMP/raw.md" "$out"
}

mkdir -p "$OUTDIR"
PAGES=0
VERBS=0
: > "$TMP/manifest.tsv"

# ONE PAGE PER MULTI-VERB FAMILY. The list is read off the register, never held
# here: a family added to the table grows a page without anyone remembering to.
while IFS= read -r fam; do
  jq -c --arg f "$fam" '[.families[] | select(.name == $f) | .entries[]]' "$TMP/rev.json" > "$TMP/rows.json"
  # Through `md_prose`, not raw: the index and the page header both render this
  # outside jq's `cell`, and `ext`'s own help carries `~/.intent/ext/<name>/`,
  # which a markdown renderer reads as an HTML tag and drops off the page.
  blurb="$(jq -r --arg f "$fam" "$JQ_LIB"'[.families[] | select(.name == $f) | .help] | (.[0] // "") | md_prose' "$TMP/rev.json")"
  render_page "$TMP/rows.json" "\`intent $fam\`" "$blurb" "$OUTDIR/intent-$fam.md"
  n="$(jq 'map(select(.disposition != "retire")) | length' "$TMP/rows.json")"
  # THE BACKTICKS HERE ARE DATA, NOT SYNTAX -- this is the page title, rendered
  # as code. Unescaped inside double quotes they are command substitution, and
  # a blanket edit over this file's printf lines made them exactly that: the
  # manifest filled with the output of `intent ext`, `intent issues` and the
  # rest, and the index came out 1361 lines long.
  printf '%s\t%s\t%s\t%s\n' "\`intent $fam\`" "intent-$fam.md" "$blurb" "$n" >> "$TMP/manifest.tsv"
  PAGES=$((PAGES + 1))
  VERBS=$((VERBS + n))
done < <(jq -r '.families[] | select((.entries | length) > 1) | .name' "$TMP/rev.json")

# EVERYTHING ELSE ON ONE PAGE. Single-verb families and the top-level
# `new_surface[]` rows are the same object from the reader's side, and 20
# one-verb pages is a table of contents pretending to be a reference.
jq -c '[ (.families[] | select((.entries | length) == 1) | .entries[]), .new_surface[] ]
       | sort_by(.path)' "$TMP/rev.json" > "$TMP/rows.json"
TOP_BLURB="The commands that take no subcommand: project lifecycle, the store, and the tooling verbs."
render_page "$TMP/rows.json" "Top-level commands" "$TOP_BLURB" "$OUTDIR/commands.md"
TOP_N="$(jq 'map(select(.disposition != "retire")) | length' "$TMP/rows.json")"
printf '%s\t%s\t%s\t%s\n' "Top-level commands" "commands.md" "$TOP_BLURB" "$TOP_N" >> "$TMP/manifest.tsv"
PAGES=$((PAGES + 1))
VERBS=$((VERBS + TOP_N))

SHIPPED_N="$(wc -l < "$TMP/shipped" | tr -d ' ')"

# ==========================================================================
# THE INDEX
# ==========================================================================
#
# **ITS FAMILY TABLE WAS HAND-TYPED AND WENT SHORT BY TWELVE**, which is the
# drift the index's own second paragraph warns about, in the index. Measured
# 2026-08-29 against the register: `info`, `bootstrap`, `organize`, `llm`,
# `learn`, `fileindex`, `version`, `schema`, `daemon`, `mcp`, `edit` and
# `events` all ship and appeared in no row, so a reader looking for `intent
# events` found the reference silent and would reasonably conclude it does not
# exist. **The prose is carried forward and the MEASUREMENTS are generated** --
# the counts, the table, the retired list, and the release comparison -- so the
# only way the table can go short now is for the register to.
#
# The prose is vc's, from `cf7cd4c8` and `6afd9d89`, moved here rather than
# rewritten. What changed is the parts that are claims about the surface.
render_index() {
  local out="$1" retired_n added removed
  retired_n="$(DISPATCH_TABLE="$TMP/rev.json" surface_retired | wc -l | tr -d ' ')"
  {
    cat <<HDR
# Command reference

**Generated by \`${GENERATOR}\` at ${WHEN}. Do not edit -- re-run it.**

**This section is generated, and it is generated against a specific revision rather than against \`main\`.** That distinction is the reason it exists as its own build step instead of being written by hand.

## Why it is not written by hand

Intent's command surface is declared in a register -- \`surface/dispatch-table.json\` -- which is the one home for what verbs exist, what arguments and flags they take, and what they do. **A hand-typed reference beside that register would be a transcribed copy of a measured mapping, and it would drift from the thing it copies.** So the reference is emitted from the register by a generator, and the emitted output names the revision it was made from.

**That is not a hypothetical.** The family table below was hand-typed once and was short by twelve shipping commands, in the document whose own second paragraph says why that happens.

## Why the revision matters more than you would expect

**The register describes \`main\`. A release is a point on \`main\`, and the two are not the same claim.** A reference written from the register without pinning a revision documents verbs the installed tool does not have, and a reader following it gets \`error:\` from a command the documentation told them to run.

So every page states, per verb, whether it is present in the release this documentation describes. **A verb in the register is not a verb in your binary.**

## Reading it against your own install

The register is a claim about a source tree. Your binary is the authority on itself:

\`\`\`
  \$ intent --version
  \$ intent <family> --help
\`\`\`

Where this reference and your binary disagree, **your binary is right** and the disagreement is worth reporting.

## The surface, by family

Intent ships **${SHIPPED_N}** commands at \`${REV}\`, across these pages.

| Page | What it covers | Commands |
| --- | --- | --- |
HDR
    while IFS=$'\t' read -r title file blurb n; do
      printf '| [%s](%s) | %s | %s |\n' "$title" "$file" "$blurb" "$n"
    done < "$TMP/manifest.tsv"
    printf '\n'

    printf '**%s commands are retired** and refuse with an exit code that distinguishes "this was removed" from "this was never built":\n\n' "$retired_n"
    DISPATCH_TABLE="$TMP/rev.json" surface_retired | sed 's/^/- `/; s/$/`/'
    printf '\n'

    if [ -n "$BASE_SHA" ]; then
      DISPATCH_TABLE="$TMP/base.json" surface_shipped | LC_ALL=C sort > "$TMP/base.shipped"
      added="$(LC_ALL=C comm -23 "$TMP/shipped" "$TMP/base.shipped" | sed 's/^/`/; s/$/`/' | paste -sd, - | sed 's/,/, /g' || true)"
      removed="$(LC_ALL=C comm -13 "$TMP/shipped" "$TMP/base.shipped" | sed 's/^/`/; s/$/`/' | paste -sd, - | sed 's/,/, /g' || true)"
      printf '## Against the published release\n\n'
      printf 'Measured by comparing the register at `%s` with the register at `%s`.\n\n' "$REV" "$BASELINE"
      if [ -n "$added" ]; then
        printf -- '- **Newer than `%s`, so not in an installed copy of it:** %s\n' "$BASELINE" "$added"
      fi
      if [ -n "$removed" ]; then
        printf -- '- **In `%s` and removed since, so a script that calls it will stop working:** %s\n' "$BASELINE" "$removed"
      fi
      if [ -z "$added" ] && [ -z "$removed" ]; then
        printf 'The surface at `%s` is identical to `%s`.\n' "$REV" "$BASELINE"
      fi
      printf '\n'
    fi

    printf -- '---\n\n'
    printf 'Also here: [what Intent ships at this revision](cut-surface.md) -- the full shipped surface and the estate-wide roster of refusal variants.\n'
  } > "$TMP/index_raw.md"
  md_align "$TMP/index_raw.md" "$out"
}

render_index "$OUTDIR/index.md"
PAGES=$((PAGES + 1))

echo "ok: wrote ${PAGES} pages to ${OUTDIR#$ROOT/} (rev ${REV_SHA:0:12}, ${VERBS} commands documented of ${SHIPPED_N} shipped)"
