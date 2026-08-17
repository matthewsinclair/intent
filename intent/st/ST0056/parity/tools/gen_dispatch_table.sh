#!/bin/bash
#
# INPUTS ARE DECLARED, NOT INFERRED, and generator_inputs_check.sh reads these
# two lines. Repo-relative paths, each of which must be tracked by git: a
# committed artefact whose only input lives in a scratchpad is re-derivable
# today and not tomorrow, and nothing else records which of those it is.
# NOTE: the paths below are the COMMITTED inputs. Where this generator still
# reads a `$SP` scratch copy of one, that is a gap between what it needs and
# what it reaches for -- not a gap in the declaration.
# inputs: surface/dispatch-table.json intent/st/ST0056/parity/tools/lib_mdfmt.sh
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

  # EVERY target field reaches the view, not an enumerated five.
  #
  # MEASURED, not suspected: the canon carried 20 distinct `target` sub-fields
  # and the renderer named 5, so 15 were dropped SILENTLY -- among them the
  # backup config keys, `doctor`s two new obligations, and the whole TBC ruling.
  # The view is what a human reads; content only in the JSON is content only a
  # reader who opens the JSON will find.
  #
  # THE SKEW CHECK CANNOT SEE THIS, and that is the part worth keeping. It asks
  # whether the committed view matches what the generator PRODUCES -- so a lossy
  # generator is a perfect fixed point with itself and reports ok forever. Skew
  # tests re-derivability; nothing was testing COMPLETENESS. Same shape as every
  # other hole this session: the check answers its own question faithfully and is
  # silent about the one beside it.
  #
  # Rendering the REMAINDER generically beats extending the enumeration, because
  # an enumeration has to be updated by the same act that invalidates it -- and
  # that act is "ic adds a field", which is exactly when nobody is thinking about
  # the renderer. This way a new field appears in the view the day it is written,
  # and the class is gone rather than the instance fixed.
  def extras($skip):
    (. // {}) | to_entries
    | map(select(.key as $k | $skip | index($k) == null))
    | if length == 0 then empty
      else map(
        (.key | gsub("_"; " ")) as $label |
        # NO FENCED JSON. The first version emitted ```json + tojson, and the
        # repo formatter PRETTY-PRINTS fenced json -- so the committed view was
        # reformatted the instant it landed and could never again match what this
        # generator produces. The skew check caught it at the commit gate, which
        # is the guard working: an emitter that cannot survive the formatter is
        # exactly what lib_mdfmt.sh exists to prevent, and I reintroduced it one
        # layer up. Leaf-path bullets are lossless and formatter-stable.
        # SCALARS INLINE, and booleans/numbers are scalars too. The first version
        # tested only `string`, so `never_built: false` fell through to the
        # paths(scalars) branch -- and `paths` on a scalar yields NOTHING, so the
        # field rendered as a bare label with no value AND left a stray blank
        # line that broke the formatter fixed point. A renderer that drops a
        # value while keeping its label is worse than one that drops both: it
        # looks like the canon holds an empty field.
        if (.value | type) | IN("string","boolean","number")
          then "- **" + $label + ":** " + (.value | tostring)
        elif (.value | type) == "array" and ((.value | map(type) | unique) == ["string"])
          then "- **" + $label + ":**\n" + (.value | map("  - " + .) | join("\n"))
        else "- **" + $label + ":**\n"
          + ([ .value | paths(scalars) as $p |
               "  - `" + ($p | map(tostring) | join(".")) + "`: " + (getpath($p) | tostring)
             ] | join("\n")) end
      ) | join("\n") end;

  def targetextra: extras(["state","ratification","behaviour","question","note"]);

  # THE THIRD LEVEL, and it was the one still open. `extras` closed the entry and
  # `target` levels; FLAGS kept an enumerated renderer -- spellings, value, type,
  # help, accepts, note -- and no remainder, so a flag key outside those six was
  # authored into a view that could not show it.
  #
  # Measured before writing this, not suspected: `default` (6 instances) and
  # `required` (3) were ALREADY in the canon and had never once appeared in the
  # view. `ac withdraw --reason` carries `required: true` and says REQUIRED in
  # its own help text, and the machine-readable half of that promise was
  # invisible to every reader of the view. The disposition authored under EXP-05
  # would have been the third such key on the day it landed.
  #
  # Indented two levels because a flag is already a sub-bullet; scalars only,
  # because every flag key measured is a scalar and a nested one here would be a
  # schema change worth noticing rather than rendering silently.
  def flagextra:
    (. // {}) | to_entries
    | map(select(.key as $k | ["spellings","type","help","value","accepts","note"] | index($k) == null))
    | if length == 0 then ""
      else map(
        "\n    - **" + (.key | gsub("_"; " ")) + ":** " +
        (if (.value | type) | IN("string","boolean","number")
           then (.value | tostring)
           else (.value | tojson) end)
      ) | join("") end;

  # ONE flag renderer, used by BOTH the family and new_surface paths.
  #
  # It was inline in the family path only, and new_surface flags rendered
  # nowhere but as a `flagsig` summary in the table -- so five flags carried a
  # disposition, a help string and a basis that no reader of the view could see.
  # Found by counting: 88 rendered for 93 declared. NOT found by the completeness
  # loop, which greps for the LABEL and was satisfied by the 88 -- the exact
  # shape of check-that-cannot-fail this file keeps re-learning, and it went
  # green on the first run of the very loop written to close this class.
  def flagblock:
    if ((.flags // []) | length) == 0 then null
    else "- **Flags:**\n" + (.flags | map(
       "  - `" + (.spellings | join("`, `")) + "`" +
       (if .value then " `\(.value)`" else "" end) +
       " (\(.type))" +
       (if .help then " -- \(.help)" else "" end) +
       (if .accepts then "\n    - Accepts: \(.accepts)" else "" end) +
       (if .note then "\n    - " + .note else "" end) +
       flagextra
     ) | join("\n")) end;

  # The entry level had NO generic remainder and no completeness check -- only
  # `target` did. Closing the class one level down and leaving it open one level
  # up is the shape of a fix that reads as done: the three MCP fields authored
  # today are entry-level, and every one of them would have rendered nowhere
  # while the target-level check stayed green.
  # A SKIP LIST IS A PROMISE THAT SOMETHING ELSE RENDERS THE KEY, and it is only
  # as good as that promise. The first version of this list was copied from the
  # new_surface one and skipped `kind`, `basis`, `owner_wp` and `acceptance` --
  # none of which any family-entry renderer touches. `kind` was not hypothetical:
  # the `st` entry carries `kind: "family"` and the view rendered it nowhere,
  # while both the generic path and the completeness check agreed to ignore it.
  # So this list now holds ONLY keys rendered bespoke above, and everything else
  # falls through to `extras` and is checked. Found by mutation-testing the
  # check, not by reading it -- reading it is what produced the bad list.
  def entryextra: extras([
    "path","help","args","flags","v2","aliases","observed","target",
    "disposition","cross_ref",
    "exposed_on_mcp","read_or_mutate","mcp_review"
  ]);

  # The MCP fields render BESPOKE rather than through `extras`, because this is
  # the line a reviewer is actually here to read and `- **exposed on mcp:** true`
  # makes them do the join in their head. One sentence, both facts, and the
  # mutating case says so in bold -- the asymmetry is deliberate: an over-loud
  # `mutates` costs a second of attention, a quiet one costs a steel thread.
  def mcpline:
    "- **MCP:** "
    + (if .exposed_on_mcp then "exposed as an agent tool" else "not exposed" end)
    + " -- "
    + (if .read_or_mutate == "mutate" then "**mutates**" else "read-only" end);

  # "Wants review" must mean WANTS REVIEW, so only the two marks that ask for a
  # second opinion live under it. The first version folded `grounded_in` and the
  # plain notes in there too, and the result was ~40 rows flagged for review of
  # which most were simply CITING THEIR SOURCE -- which is the opposite of
  # wanting one. A marker that fires on the confident rows is noise, and noise on
  # a review list is spent exactly where the reviewer attention was meant to go.
  # Evidence and rationale get their own bullets; the flag stays scarce.
  #
  # NO APOSTROPHE ANYWHERE IN THIS BLOCK. JQ_LIB is a SINGLE-QUOTED shell
  # string, so one apostrophe in a COMMENT closes it and the rest of the line
  # becomes shell -- this paragraph originally read "where vc-apostrophe-s
  # attention", and bash reported `attention: command not found` from inside
  # what looks like a jq function library. Same family as the backtick trap
  # already recorded twice: a quote character inside a quoting context, in prose
  # nobody proof-reads for syntax. It failed loudly, but at the wrong layer --
  # the error names a shell command, not the string that swallowed it.
  def mcpreview:
    (.mcp_review // {}) as $r |
    ( (if ($r.uncertain // []) | length > 0 then
         ["- **Wants review:**"
          + "\n  - uncertain on " + ($r.uncertain | map("`" + . + "`") | join(", "))
          + (if $r.why_uncertain then "\n  - " + $r.why_uncertain else "" end)]
       else [] end)
    + (if $r.counterintuitive then
         ["- **Wants review -- the classification disagrees with the verb name:** " + $r.counterintuitive]
       else [] end)
    + (if $r.note then ["- **MCP note:** " + $r.note] else [] end)
    + (if $r.grounded_in then ["- **MCP classification grounded in:** " + $r.grounded_in] else [] end)
    ) as $bullets
    | if ($bullets | length) == 0 then empty else ($bullets | join("\n")) end;
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

# THE COUNTS IN `status` ARE DERIVED, NOT TRANSCRIBED, and an authored count
# that disagrees with the file it describes is a REFUSAL rather than a warning.
#
# Found 2026-08-15: the canon carried "All 27 v2 families authored + 6
# new-surface entries" while holding SEVEN. A hand-typed number sitting inside
# the artefact it counts is a second copy of data the file already has, and it
# went stale the moment cc authored an entry -- correctly, telling me at the
# time. Nothing could have caught it, because nothing was comparing the sentence
# to the rows.
#
# This is the same defect this whole directory exists to prevent, in the file
# that is supposed to be THE source of truth for the command surface. A view
# whose own header miscounts its rows undermines every number below it.
N_FAM="$(jq -r '.families | length' "$IN")"
N_NEW="$(jq -r '.new_surface | length' "$IN")"
if [ -n "$STATUS" ]; then
  claimed_fam="$(printf '%s' "$STATUS" | sed -nE 's/.*All ([0-9]+) v2 families.*/\1/p')"
  claimed_new="$(printf '%s' "$STATUS" | sed -nE 's/.*\+ ([0-9]+) new-surface.*/\1/p')"
  [ -z "$claimed_fam" ] || [ "$claimed_fam" = "$N_FAM" ] || \
    die "canon status claims $claimed_fam v2 families; the file holds $N_FAM. Fix the sentence or the rows -- a view that miscounts its own contents discredits every figure in it."
  [ -z "$claimed_new" ] || [ "$claimed_new" = "$N_NEW" ] || \
    die "canon status claims $claimed_new new-surface entries; the file holds $N_NEW. Fix the sentence or the rows -- a view that miscounts its own contents discredits every figure in it."
fi

# The stamp is not decoration. A record that does not name the commit it covers
# cannot be spotted as stale, and "full suite GREEN at HEAD" was false by three
# commits across four documents for exactly this reason. Refuse rather than
# emit an unstamped artefact.
[ -n "$MEASURED_AT" ] || die "canon has no measured_at -- refusing to emit an unstamped view"
[ -n "$MEASURED_ON" ] || die "canon has no measured_on -- refusing to emit an unstamped view"

# CANON MUST NOT NAME A RUST PATH THAT DOES NOT EXIST.
#
# 2026-08-15: the Rust tree moved twice in one morning -- `crates/` ->
# `native/crates/` -> `native/rust/crates/` -- and the table was rewritten and
# COMMITTED against the intermediate location. The path was verified present on
# disk before the edit and the verification was still worthless, because the
# tree was live under another node's hands. A point-in-time read of a moving
# target is not a control; this is.
#
# THE NEEDLE IS `crates/`, DELIBERATELY, NOT `native/rust/`. A prefix needle
# stops matching the moment the prefix changes and then passes in silence --
# which is the exact class this check exists to catch, and it would have been
# the third instance of it in this toolchain. Every relocation so far kept
# `crates/` in the path, so the needle survives the move that breaks a prefix.
#
# WHY THIS DOES NOT CRY WOLF, measured rather than assumed: 55 distinct
# path-shaped tokens live in canon and 8 do not resolve -- but every one of
# those 8 is either a prose placeholder (`bin/intent_`, `intent/llm/RULES-`)
# or a path named precisely BECAUSE it is absent (`lib/help/st.help.md`, one
# of the 17 commands with no help file, which is the finding). None contains
# `crates/`. A general path-existence check would fire on all 8 on its first
# run against a healthy tree, and the first thing anyone does with a check that
# cries wolf is switch it off.
#
# Zero matches is REPORTED, not silently passed: if canon ever stops naming the
# Rust tree, that is either fine or the needle has died, and the difference has
# to be visible to be decidable.
# `|| true` IS LOAD-BEARING, not defensive noise. This script runs under
# `set -euo pipefail`, and grep exits 1 on no-match, so without it a canon
# holding zero crates/ paths ABORTS THE WHOLE GENERATOR -- exit 1, empty
# stderr, no view, no explanation. Caught by the zero-match mutation below,
# never by reading. Second occurrence of this exact class in this toolchain:
# `corpus_require` was green under `set -uo pipefail` and dead under
# `set -euo pipefail` the same way, which is why "a guard verified in one
# harness is verified in THAT harness" is a standing watch-out.
RUST_REFS="$(grep -oE '[A-Za-z0-9_./-]*crates/[A-Za-z0-9_./-]+' "$IN" | sed 's/\.*$//' | sort -u || true)"
if [ -z "$RUST_REFS" ]; then
  echo "note: canon names no crates/ paths -- either correct, or this needle has stopped matching" >&2
else
  RUST_MISSING=""
  while IFS= read -r ref; do
    [ -n "$ref" ] || continue
    [ -e "$REPO_ROOT/$ref" ] || RUST_MISSING="$RUST_MISSING  $ref"$'\n'
  done <<< "$RUST_REFS"
  [ -z "$RUST_MISSING" ] || die "canon names Rust path(s) that do not exist on disk -- the tree moved and the table did not follow:
$RUST_MISSING  Fix the JSON canon and re-run. Never hand-edit the rendered view to match."
fi

# AN ENTRY WITH A v2 ANTECEDENT MUST CARRY ITS MEASUREMENT.
#
# The renderer above tolerates a missing `observed` block, because an addition
# has no v2 command and there was never anything to run -- fabricating an
# `observed` block for a command that does not exist would be inventing
# measurement, which is the one thing this toolchain refuses everywhere else.
#
# That tolerance is a HOLE unless this refusal exists beside it. Without it, a
# real measured command that lost its `observed` block renders as "nothing to
# observe -- no v2 antecedent", which is not a gap in the record, it is a FALSE
# STATEMENT about v2 generated automatically and indistinguishable from the
# truthful case. The tolerance and the refusal are one change; neither is safe
# alone.
MISSING_OBS="$(jq -r '
  .families[].entries[]
  | select((.v2 // "new-surface") != "new-surface")
  | select(has("observed") | not)
  | .path' "$IN")"
[ -z "$MISSING_OBS" ] || die "entries name a v2 antecedent but carry no observed block -- a measured command must ship its measurement, or the view will state there was nothing to measure:
$(printf '%s' "$MISSING_OBS" | sed 's/^/  /')"

# --- MCP DECLARATION: both fields, on every row, no defaults ----------------
# AC-09.1. The MCP tool list renders from this file, so a row that declares
# nothing has to be resolved SOMEHOW at render time, and every available
# resolution is wrong:
#
#   default to exposed    -- a new command joins the agent surface because
#                            somebody added a row, which is the opposite of a
#                            decision.
#   default to hidden     -- the surface silently loses commands and the only
#                            symptom is an agent that cannot do its job.
#   derive from the verb  -- dies on one pair alone: `ac gate` READS while
#                            `wp done` consults the same gate and WRITES, and
#                            the two do not share a spelling. Four more rows
#                            (`at lint`, `doctor`, `llm usage_rules`,
#                            `todo list`) mutate only under a flag or on first
#                            run, and `st edit` -- the most obviously-mutating
#                            verb name here -- writes nothing at all.
#
# So absence REFUSES. This is the same shape as `disposition: pending`: the
# uncertain state is written down and greppable, never expressed by leaving a
# field out, because absence-as-meaning reads as an oversight and cannot be
# counted. Uncertainty has its own home in `mcp_review.uncertain`, which names
# the soft FIELD -- the two lean opposite ways when unsure (exposed leans
# false, read_or_mutate leans mutate), so an unqualified doubt is unactionable.
#
# Checked over families AND new_surface together: the new-surface rows are
# where the exposure question is sharpest (`daemon`, `mcp`, `ingest`), and a
# check that walked only `.families` would pass while the riskiest rows in the
# file went undeclared. That exact miss -- a structured query that read one
# array and silently skipped a top-level sibling -- has already happened once
# in this table.
MCP_UNDECLARED="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select((.exposed_on_mcp | type) != "boolean"
             or ((.read_or_mutate // "") | IN("read","mutate") | not))
        | .path)
  | join("\n")' "$IN")"
[ -z "$MCP_UNDECLARED" ] || die "rows do not declare the MCP surface -- every entry needs \`exposed_on_mcp\` (boolean) and \`read_or_mutate\` (\"read\" or \"mutate\"). Refusing rather than defaulting: there is no safe default, and deriving from the verb is what this field exists to replace. Offending paths:
$(printf '%s' "$MCP_UNDECLARED" | sed 's/^/  /')"

# A row that does NOT ship must not be exposed on MCP. The MCP tool list renders
# from this file, so `exposed_on_mcp: true` on a retired row publishes a tool for
# a command that will not answer -- and an agent trusts a tool list far more than
# it trusts prose, because a listed tool reads as a capability the host verified.
#
# This held on all 112 rows the day it was written (81 exposed, 0 of them retired),
# and that is exactly why it is worth asserting rather than leaving alone: **a file
# clean by luck and a file clean by construction are identical in a diff**, which
# is this table's own stated reason for keeping `known_exposures`. The two fields
# are independently edited by different sessions -- retirement is a disposition
# call, exposure is an agent-safety call -- so nothing but this line keeps them
# from disagreeing, and the disagreement would be invisible in the green.
MCP_ON_DEAD="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select(.exposed_on_mcp == true
             and ((.disposition // "") == "retire" or (.target.state // "") == "retire"))
        | .path + " (disposition=" + (.disposition // "(absent)") + ", target.state=" + (.target.state // "(absent)") + ")")
  | join("\n")' "$IN")"
[ -z "$MCP_ON_DEAD" ] || die "rows are exposed on MCP but do not ship -- an agent would be offered a tool for a command that does not exist. Retire the exposure with the command, or the row is not really retired. Offending paths:
$(printf '%s' "$MCP_ON_DEAD" | sed 's/^/  /')"

# A DECLARED VALUE RESTING SOLELY ON A RETIRED DISPOSITION (vc's generalisation,
# 2026-08-16, of a finding on `doctor`).
#
# A row declared `mutate` that takes NO arguments and ships NO flags has nothing
# the operator can supply that could cause a write, so its classification rests
# entirely on its own action -- and if the only mutating thing it ever had is a
# flag now dispositioned `retire`, the classification is grounded in something
# withdrawn. `doctor` sat exactly there: `--fix` moved two config files aside in
# v2, v3 does not carry it, and `read_or_mutate` still said `mutate`.
#
# **The class is what earns a refusal, not the row.** vc's framing is the one to
# keep: a judgement is not overturned when the thing it judged stops existing,
# it is SUPERSEDED -- and nobody propagates a supersession, because the reasoning
# is still sound about a subject that is gone. That is `st_prefix`, the residue
# table declaring six against eight, and Machine 3 listing four AC states after
# the same file ratified the fifth. **A defended row is harder to correct than an
# undefended one, and that is a property of the defence.**
#
# NARROW BY CONSTRUCTION, which is what makes it a refusal rather than a report.
# The four conditions together matched exactly ONE row of 107 when it was
# written, and the six other argument-less flag-less mutations (`todo update`,
# `upgrade`, `agents generate`, `agents sync`, `claude prime`, `mcp`) all mutate
# through their own action and carry no retired flag, so none of them trip it.
#
# MUTATION-PROVEN, and it must be, because its correct steady state is SILENCE:
# with `doctor` fixed this arm reports nothing forever, which is the shape that
# rots unnoticed. Reproduce with
#   jq '.families |= map(.entries |= map(if .path == "doctor"
#       then .read_or_mutate = "mutate" else . end))' surface/dispatch-table.json > /tmp/t.json
#   IN=/tmp/t.json OUT=/tmp/t.md bash .../gen_dispatch_table.sh
# -> REFUSES, naming `doctor (--fix)`. Restore the file and it passes.
RETIRED_GROUNDING="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select((.disposition // "") != "retire" and (.target.state // "") != "retire"))
  | map(select(.read_or_mutate == "mutate"))
  | map(select(((.args // []) | length) == 0))
  | map(select(([.flags[]? | select(.disposition == "keep")] | length) == 0))
  | map(select(([.flags[]? | select(.disposition == "retire")] | length) > 0)
        | .path + " (" + ([.flags[] | select(.disposition == "retire") | .spellings[0]] | join(", ")) + ")")
  | join("\n")' "$IN")"
[ -z "$RETIRED_GROUNDING" ] || die "rows are declared \`mutate\` while taking no arguments, shipping no flags, and carrying a RETIRED flag as the only thing that ever wrote. The classification is grounded in something withdrawn -- reclassify the row, or say in its \`mcp_review\` what still mutates. Offending paths:
$(printf '%s' "$RETIRED_GROUNDING" | sed 's/^/  /')"

# A FAMILY ROOT THAT DECLARES NO VERB SLOT (vc's predicate, 2026-08-16).
#
# `spine.rs:build()` reads `subcommand_required` off the root's `subcommand` arg
# and defaults an ABSENT one to REQUIRED -- `.is_none_or(|slot| slot.arity ==
# "1")`. So a family that simply forgets to declare its slot is answered by a
# default nobody chose for it, in the restrictive direction, silently. `config`
# was that row: v3 refused `intent config` at exit 1 where v2 exits 0.
#
# **THE POPULATION IS NARROWER THAN THE OBVIOUS ONE, AND THE DIFFERENCE IS THE
# WHOLE REASON THIS ARM IS KEEPABLE.** Twelve single-entry rows also declare no
# slot -- `info`, `init`, `bootstrap`, `doctor`, `upgrade`, `organize`, `critic`,
# `learn`, `treeindex`, `fileindex`, `help`, `version` -- and NONE of them reach
# that branch: `is_none_or` sits inside the arm for a family whose verbs are
# sibling ENTRIES, and a lone leaf takes the `else` into `with_args`. vc measured
# 12 first, nearly filed it as a counter-example, and found the correct count is
# ZERO. **A guard written against the obvious count would be permanently noisy
# and switched off within a week.**
#
# So the predicate is vc's: a family with MORE THAN ONE shipped entry whose root
# declares no `type: "subcommand"` arg. That is exactly what reaches the default
# and no wider.
#
# GREEN BY CONSTRUCTION TODAY, WHICH IS WHY IT IS MUTATION-PROVEN. Every one of
# `st wp ac at issues todo config agents claude lang llm modules plugin ext`
# declares its slot as of `8b2d3d47`, so this arm is silent -- the shape that
# rots unnoticed. Reproduce the refusal with
#   jq 'del(.families[] | select(.name == "config") | .entries[]
#        | select(.path == "config") | .args)' surface/dispatch-table.json > /tmp/t.json
#   IN=/tmp/t.json OUT=/tmp/t.md bash .../gen_dispatch_table.sh
# -> REFUSES, naming `config`. Restore and it passes.
#
# AND THE NEGATIVE CONTROL, which is the arm vc's 12-vs-0 near-miss made
# necessary -- a refusal that fires on the wrong population is worse than none,
# because it teaches its readers to skim:
#   jq '.families |= map(if .name == "doctor" then .entries |= map(del(.args)) else . end)' ...
# -> SILENT. `doctor` is a single-entry leaf, so stripping its args cannot reach
# the branch this arm guards, and the arm correctly says nothing. Without this
# control, "it refused when I broke config" is equally consistent with an arm
# that would also refuse on all twelve leaves.
#
# **The other end is cc's and vc has routed it there: once the TABLE refuses an
# absent slot, `spine.rs` should stop tolerating one** -- `is_none_or` becoming
# an explicit refusal that names the invariant. The same fact asserted at both
# ends is what survives someone weakening either: as it stands, delete this arm
# and the code silently defaults again with nothing to say so.
NO_VERB_SLOT="$(jq -r '
  .families
  | map(select(([.entries[] | select((.disposition // "") != "retire"
                                  and (.target.state // "") != "retire")] | length) > 1))
  | map(. as $f
        | ($f.entries | map(select((.path | test(" ")) | not)) | .[0]) as $root
        | select($root != null)
        | select((($root.disposition // "") != "retire") and (($root.target.state // "") != "retire"))
        | select((([$root.args[]? | select(.type == "subcommand")]) | length) == 0)
        | $root.path)
  | join("\n")' "$IN")"
# RECOVERABILITY VERSUS `exposed_on_mcp` -- TWO-SIDED, and the second side is
# the one that rots.
#
# `recoverability` exists because vc ruled the withheld-13 policy earns a
# declared field, and the canary killed their first shape: `acts_upon` could not
# derive the partition, because `lang init` and `lang remove` act upon the
# identical thing and sit on opposite sides. Recoverability derives it -- MCP
# withholds a mutation the surface cannot undo -- **with four measured
# disagreements that are REPORTED rather than fitted to.**
#
# vc's ruling on those four is the reason this arm exists at all: **a derived
# field that merely reproduces the partition it was fitted to tells you nothing.
# One that reproduces it AND surfaces real gaps is earning its place.** `ext new`
# is one-way and exposed because the `ext` family ships no `remove` -- an
# INCOMPLETE FAMILY, not a mislabelled row, and its resolution is a scope call
# for hv. `at green` / `at red` / `at na` are one-way because issue 0033 destroys
# the row's authored note, so a documented round trip is lossy.
#
# ARM 1 refuses an UNDECLARED disagreement: a new one must be explained, not
# absorbed by whoever notices the number moved.
#
# ARM 2 refuses a STALE anomaly -- a row carrying `recoverability_anomaly` that
# no longer disagrees. **That is the half a known-exceptions list never has, and
# the reason it decays into a list of things that used to be true.** When 0033 is
# fixed and `at green` becomes `reversible`, this arm makes removing its note
# mandatory rather than optional; when `ext remove` ships, likewise.
#
# MUTATION-PROVEN, both arms:
#   jq '.families |= map(.entries |= map(if .path == "at green"
#       then del(.recoverability_anomaly) else . end))' ... -> REFUSES (arm 1)
#   jq '.families |= map(.entries |= map(if .path == "st done"
#       then .recoverability_anomaly = "x" else . end))' ... -> REFUSES (arm 2)
#
# The vocabulary and totality halves are deliberately NOT here. `check_vocabularies`
# in `dispatch.rs` already refuses a mutation with no recoverability, a read that
# carries one, and any value outside `recoverability_values` -- at binary load,
# so every test in the workspace fails loudly. A shell copy would be a second
# implementation of one rule, which is the Highlander failure this file's own
# `NO_VERB_SLOT` note argues against.
UNDECLARED_ANOMALY="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select((.disposition // "") != "retire" and (.target.state // "") != "retire"))
  | map(select(.read_or_mutate == "mutate"))
  | map(select(((.recoverability == "one-way") != (.exposed_on_mcp == false))
               and (.recoverability_anomaly | not))
        | .path + " (recoverability=" + (.recoverability // "(absent)")
                + ", exposed_on_mcp=" + (.exposed_on_mcp | tostring) + ")")
  | join("\n")' "$IN")"
[ -z "$UNDECLARED_ANOMALY" ] || die "rows disagree with the MCP withhold list derived from \`recoverability\`, and do not say why. MCP withholds a mutation the surface cannot undo; a row that breaks that either found a real gap or is misclassified. Record it in \`recoverability_anomaly\` -- do NOT adjust the label to make the number come out. Offending paths:
$(printf '%s' "$UNDECLARED_ANOMALY" | sed 's/^/  /')"

STALE_ANOMALY="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select((.disposition // "") != "retire" and (.target.state // "") != "retire"))
  | map(select(.recoverability_anomaly
               and ((.recoverability == "one-way") == (.exposed_on_mcp == false)))
        | .path)
  | join("\n")' "$IN")"
[ -z "$STALE_ANOMALY" ] || die "rows carry a \`recoverability_anomaly\` while AGREEING with the derived withhold list -- the disagreement it documents is gone, so the note now describes a state that no longer exists. Remove it. Offending paths:
$(printf '%s' "$STALE_ANOMALY" | sed 's/^/  /')"

# A RETIRED FLAG WHOSE CAPABILITY THE ROW STILL PROMISES (the retirement's blast
# radius). When a flag is dispositioned `retire`, the decision is recorded on the
# flag -- and every OTHER field on the row that described that capability keeps
# describing it, because nothing propagates a withdrawal outward.
#
# `doctor` is the measured instance and it carried THREE artefacts from ONE
# withdrawn subject: `read_or_mutate: mutate` (grounded in `--fix` and nothing
# else), an `mcp_review` defending that grounding, and a `help` string promising
# `Diagnose and fix ...`. All three were found by three separate accidents on
# three separate routes, none of them looking for this. That is the shape of a
# class rather than three bugs: **a withdrawal is a decision with one home and
# many readers, and the readers are not notified.**
#
# THE POPULATION IS SHIPPING ROWS ONLY, and the exclusion is not a convenience.
# 14 flags are `retire` across 8 rows, but `organize`, `treeindex` and `st_zero`
# are whole-command retirements -- their residue leaves with the row, so there is
# no surviving reader to mislead. Exactly 3 retired flags sit on a row that still
# ships (`todo done --flush`, `todo done --prune`, `doctor --fix`), and those 3
# are the entire risk surface.
#
# THE DETECTOR IS THE FLAG'S OWN SPELLING, AS A WORD, AND THE OBVIOUS ALTERNATIVE
# WAS BUILT FIRST AND REJECTED ON MEASUREMENT. Word-overlap between the flag help
# and the entry help looks strictly better -- it catches a paraphrase -- and on
# this corpus it reports `doctor (--fix -- still promised by: issues)` against the
# CORRECTED help string, because `issues` is the SUBJECT the command and the flag
# share by construction, not the capability. It also fired twice on `todo done`
# for the word `done`, which is the command's own name. Both needed suppressing,
# one by a path-word exclusion and one by a hand-list, and a refusal that needs a
# growing exceptions list to stay quiet is the cry-wolf shape this toolchain keeps
# refusing. **The spelling rule needs neither: 0 false positives on 107 shipping
# rows, because a flag's spelling names its action and its help names its subject.**
#
# WHAT IT CANNOT SEE, stated here because a check's error message is where its
# reader learns its scope and nothing verifies that message against its behaviour:
# **a promise that does not use the flag's own word.** A `--force` retired from a
# row whose help says `Overwrite existing files` passes this arm in silence. That
# is a known hole and not a claim of coverage -- narrowing the detector until it
# never lies was the trade, and the wider detector was measurably worse.
#
# MUTATION-PROVEN THREE WAYS, because its correct steady state is SILENCE:
#   A (steady) live table -> silent.
#   B (real)    jq '.families |= map(if .name=="doctor" then .entries |= map(
#                 if .path=="doctor" then .help="Diagnose and fix common Intent
#                 configuration issues" else . end) else . end)'
#               -> REFUSES `doctor (--fix -- still promised by the word: fix)`.
#               This is the historical defect, restored: the arm fires on the row
#               it was built for, using that row's real prior value.
#   C (scope)   same edit against `todo done`.help adding the word `flush`
#               -> REFUSES `todo done (--flush ...)`. Without C, `it fired when I
#               broke doctor` is equally consistent with an arm scoped to doctor.
#   D (family)  jq '.families |= map(if .name=="doctor" then .help="Diagnose and
#                 fix common Intent configuration issues" else . end)'
#               -> REFUSES. This is the arm's own blind spot, restored: B and C
#               both pass against the NARROW arm that reads entries only, so
#               without D nothing distinguishes the two versions of this check.
# THE ARM'S OWN BLAST RADIUS, FOUND ON ITS FIRST REAL RUN AND FIXED HERE RATHER
# THAN NOTED. Built against `.families[].entries[]` + `.new_surface[]` -- the
# population rule this file states everywhere -- it went silent on `doctor` the
# moment the ENTRY help was reworded, and `families[].help` still said `Diagnose
# and fix`. The string lives in TWO places and the arm was reading one.
#
# So the arm was itself an un-notified reader of the retirement it was built to
# chase, which is the class stating itself one level up. **The population rule was
# not wrong; it was answering `which rows are commands`, and this asks `which text
# faces a user`, and those are different questions over the same file.**
#
# `families[].help` is NOT a duplicate and must not be collapsed: 11 of 27
# families carry a help that differs from their root entry's, and in 10 of the 11
# it is a deliberately richer sentence (`ac`, `at`, `todo`, `agents`, `claude`,
# `critic`, `lang`, `llm`, `modules`, `ext`). `doctor` was the eleventh and the
# only one where the difference was staleness. It is also NOT read by the binary
# -- `pub struct Family` deserializes `name` and `entries` only -- so it is a view
# field this generator consumes, and a stale promise there reaches the committed
# `dispatch-table.md` rather than clap. Lower severity, same defect, same arm.
#
# The family help is attributed to the family ROOT entry (`path == family name`)
# and to no other, because that is the row it describes. A retired flag on a leaf
# verb does not oblige the family sentence.
RETIRED_PROMISE="$(jq -r '
  def words: ascii_downcase | [scan("[a-z]{3,}")];
  def scan_row($e; $extra):
    ([$e.help // ""] + [($e.args // [])[] | .help // ""] + [$extra]
     | join(" ") | words) as $facing
    | [ ($e.flags // [])[]
        | select(.disposition == "retire")
        | . as $f
        | ([$f.spellings[] | words] | flatten | unique) as $named
        | ($named | map(select(. as $w | $facing | any(. == $w)))) as $shared
        | select(($shared | length) > 0)
        | $e.path + " (" + $f.spellings[0] + " -- still promised by the word: "
          + ($shared | join(", ")) + ")" ];
  [ ( .families[] | . as $fam | $fam.entries[]
      | select((.disposition // "") != "retire" and (.target.state // "") != "retire")
      | scan_row(.; (if .path == $fam.name then ($fam.help // "") else "" end)) )
  , ( .new_surface[]
      | select((.disposition // "") != "retire" and (.target.state // "") != "retire")
      | scan_row(.; "") )
  ]
  | flatten
  | join("\n")' "$IN")"
[ -z "$RETIRED_PROMISE" ] || die "rows still promise a capability whose flag is dispositioned \`retire\`. The user-facing text names the withdrawn flag's own word, so the row advertises something v3 does not ship -- reword the help, or reopen the flag's disposition. A retirement is one decision with many readers and nothing notifies them. Offending paths:
$(printf '%s' "$RETIRED_PROMISE" | sed 's/^/  /')"

[ -z "$NO_VERB_SLOT" ] || die "family roots declare no \`type: subcommand\` arg, so \`spine.rs\` defaults their verb slot to REQUIRED -- a default nobody chose, in the restrictive direction. Declare the slot with the arity the family actually wants (\`1\` if the bare command is illegal, \`0..1\` if it does something of its own). Offending paths:
$(printf '%s' "$NO_VERB_SLOT" | sed 's/^/  /')"

# Every key on `Entry`, `Flag` and `Arg` is classified `declaration` or `note`,
# in `key_classes`. The list is AUTHORED because dc measured that no mechanical
# discriminator exists: not count (`read_or_mutate` is 112 rows and decides agent
# safety, `observed` is 93 and is a measurement), not type (`read_or_mutate` and
# `disposition_basis` are both strings). Semantics have to be declared by whoever
# wrote the register.
#
# An authored list rots in two directions and both arms are here. A NEW key that
# nobody classifies is how the declared-but-not-deserialized class got to five
# instances -- each one entered as prose, or as a declaration everyone assumed was
# wired, and nothing asked which. A key that OUTLIVES its rows is the quieter
# failure: the list still reads as a description of this file while describing a
# file that no longer exists, and it is the arm a list-maintainer never adds,
# because deleting rows feels like it cannot break a list of names.
#
# `Table` and `Target` are deliberately NOT checked. See `key_classes.scope`.
[ "$(jq -r 'has("key_classes")' "$IN")" = "true" ] || die "the table declares no \`key_classes\` -- the declaration/note split on \`Entry\`, \`Flag\` and \`Arg\` is authored, and without it a new key joins the canon with nobody deciding whether code must read it. That is exactly how the declared-but-not-deserialized class reached five instances."
KEY_UNCLASSED="$(jq -r '
  def keyset($items): $items | map(keys) | flatten | unique;
  [.families[].entries[], .new_surface[]] as $entries
  | ($entries | map(.flags // []) | flatten) as $flags
  | ($entries | map(.args  // []) | flatten) as $args
  | .key_classes as $kc
  | [ {t: "entry", present: keyset($entries), decl: $kc.entry.declaration, note: $kc.entry.note},
      {t: "flag",  present: keyset($flags),   decl: $kc.flag.declaration,  note: $kc.flag.note},
      {t: "arg",   present: keyset($args),    decl: $kc.arg.declaration,   note: $kc.arg.note} ]
  | map(. as $x
        | (($x.decl // []) + ($x.note // [])) as $all
        | (($x.present - $all)                        | map("unclassified -- decide whether code must read it: " + $x.t + "." + .))
        + ((($x.decl // []) - (($x.decl // []) - ($x.note // []))) | map("classified BOTH declaration and note: " + $x.t + "." + .))
        + (($all - $x.present)                        | map("classified but no row carries it: " + $x.t + "." + .)))
  | flatten | join("\n")' "$IN")"
[ -z "$KEY_UNCLASSED" ] || die "\`key_classes\` and the rows disagree:
$(printf '%s' "$KEY_UNCLASSED" | sed 's/^/  /')"

# `target.state` is a CLOSED vocabulary, and until now nothing closed it.
# `Target.state` is a bare `String` with `#[serde(default)]`, the values were
# listed only in a doc comment at dispatch.rs:172 and in this file's prose, and
# vc demonstrated the hole rather than argued it (2026-08-15): `banana` on
# `st start`, with a ratification naming parity.md, passed EVERY check in the
# repo including corrected_check.sh. Three homes for one vocabulary and no
# comparison between any two of them -- which is how the doc comment came to
# list five values while the table uses six, missing `new-surface` at 18 rows.
#
# `target_states` in the canon is now the one home, and the check runs BOTH
# ways. An undeclared state is the typo. A declared-but-unused state is the
# other half: a vocabulary that outlives its members reads as coverage, and the
# skip-list failure in surface_check.sh was exactly that shape.
#
# **THE SECOND DIRECTION IS VALID HERE ONLY BECAUSE THIS VOCABULARY HAS ONE
# HOME. Do not lift it to a vocabulary that does not.** `entry_dispositions` is
# shared with the keep/retire/deviate register, where `deviate` has 47 rows in
# `pertest.md` and 3 in `register.md` while having none here -- so the same
# refusal applied there would fire on a correct value, correctly declared, with
# 47 members one artefact away. **The premise is single-homedness, not
# declaration.** Caught by vc 2026-08-15 on ic proposing to drop `deviate` after
# reading zero rows: same rule, same author, same evening, and the second
# application would have been wrong.
# EVERY VOCABULARY CHECK BELOW REFUSES ON AN ABSENT VOCABULARY BEFORE IT
# REFUSES ON THE DATA, and all three needed it: measured 2026-08-15, deleting
# `flag_dispositions` reported 95 flags as undeclared, deleting `target_states`
# reported every invariant and entry, and deleting `entry_dispositions` reported
# every row. **A wall of offending rows reads as catastrophic data corruption
# and sends its reader to fix the data**, when the actual fault is one missing
# key. A missing measurement must present as a refusal to measure, never as a
# measurement of nothing -- the rule `class_vocab_check.sh` was built with and
# these three were not.
vocab_or_die() {
  local key="$1" n
  n="$(jq -r --arg k "$key" '(.[$k] // []) | length' "$IN")"
  [ "$n" -gt 0 ] || die "\`$key\` is absent or empty, so nothing can be checked against it. Refusing rather than reporting every row as undeclared, which is a true statement about nothing and points its reader at the data instead of at the missing key."
}

vocab_or_die "target_states"
STATE_UNDECLARED="$(jq -r '
  (.target_states // [] | map(.state)) as $vocab
  | [.invariants[], .families[].entries[], .new_surface[]]
  | map(select(((.target.state // "") | IN($vocab[]) | not))
        | ((.path // .id) + " = " + (.target.state // "(absent)")))
  | join("\n")' "$IN")"
[ -z "$STATE_UNDECLARED" ] || die "rows carry a \`target.state\` that \`target_states\` does not declare. The vocabulary is closed on purpose: an unrecognised state is a typo that every downstream check reads as a state it simply has no rule for, so it passes everything in silence. Offending rows:
$(printf '%s' "$STATE_UNDECLARED" | sed 's/^/  /')"

STATE_UNUSED="$(jq -r '
  ([.invariants[], .families[].entries[], .new_surface[]] | map(.target.state // "") | unique) as $used
  | (.target_states // []) | map(select((.state | IN($used[])) | not) | .state) | join("\n")' "$IN")"
[ -z "$STATE_UNUSED" ] || die "\`target_states\` declares states no row uses -- a vocabulary that outlives its members reads as coverage of something nobody classified. Remove them, or record why they are held open:
$(printf '%s' "$STATE_UNUSED" | sed 's/^/  /')"

# Entry `disposition`, same treatment as `target.state` and DELIBERATELY ONE
# DIRECTION ONLY. An undeclared or absent value is refused; a declared value no
# row uses is NOT checked, because this vocabulary is shared with the register
# (see `shared_vocabulary` in the canon, and the note above). `pending` is
# written explicitly and never expressed by omitting the field -- absence here
# was a real breach, on the 8 `new_surface[]` rows, of the rationale the canon
# gives for that very rule.
vocab_or_die "entry_dispositions"
DISPOSITION_UNDECLARED="$(jq -r '
  (.entry_dispositions // [] | map(.value)) as $vocab
  | [.families[].entries[], .new_surface[]]
  | map(select(((.disposition // "") | IN($vocab[]) | not))
        | (.path + " = " + (.disposition // "(absent)")))
  | join("\n")' "$IN")"
[ -z "$DISPOSITION_UNDECLARED" ] || die "rows carry a \`disposition\` that \`entry_dispositions\` does not declare. Absence is a value nobody wrote and reads as an oversight, so it is refused alongside a typo:
$(printf '%s' "$DISPOSITION_UNDECLARED" | sed 's/^/  /')"

# THE (`disposition`, `target.state`) PAIR IS THE DECLARED UNIT. The two fields
# answer different questions -- what becomes of the v2 command, versus what v3
# does -- so most combinations are meaningful, a few are self-contradictory, and
# which is which is a judgement that has to be RECORDED rather than inferred
# from the corpus. `legal_pairs` in the canon records it; 7 pairs across 112
# rows.
#
# It began as a narrower check: the two fields share exactly `retire` and
# `new-surface`, and they moved in lockstep on those with nothing enforcing it.
# **Lockstep with nothing enforcing it is a coincidence that reads as a rule.**
#
# It matters because `Entry::is_shipped()` reads BOTH and fails OPEN:
# `disposition != "retire" && target.state != "retire"`. So the redundancy is
# currently the only thing stopping a single hand-edit from shipping a retired
# command -- and its sibling `Flag::ships()` thirty-five lines away is a
# positive match (`== "keep"`) whose own comment says it deliberately does not
# default-allow, so a typo drops a flag rather than shipping one. Two guards,
# opposite risk directions, one written with the typo class in mind.
#
# WIDENED 2026-08-16 FROM THE TWO SHARED VALUES TO THE WHOLE PAIR, on vc's
# ruling answering the derivation question NO. The narrow version constrained
# only `retire` and `new-surface`, which **admitted 17 of the 35 possible pairs
# while the corpus used 7** -- ten permitted combinations nobody had decided
# were legal. The hole vc named is the one worth remembering, because both
# halves read as careful: `disposition: pending` with `target.state:
# as-observed` passed every check that existed. An honest blank about whether
# the command survives, against a confident claim that v3 reproduces what v2
# did. **Those cannot both be true of one row, and nothing said so.**
#
# The pair is now the declared unit (`legal_pairs` in the canon), so a new
# combination is an explicit decision rather than a silent widening. Both fields
# stay hand-authored: deriving one would make this very refusal VACUOUS while
# leaving it in the file looking exactly as load-bearing as it does now.
vocab_or_die legal_pairs

PAIR_ILLEGAL="$(jq -r '
  (.legal_pairs | map(.disposition + " / " + .target_state)) as $legal
  | [.families[].entries[], .new_surface[]]
  | map(((.disposition // "(absent)") + " / " + (.target.state // "(absent)")) as $p
        | select(($legal | index($p)) == null)
        | (.path + ": " + $p))
  | join("\n")' "$IN")"
[ -z "$PAIR_ILLEGAL" ] || die "row(s) declare a (\`disposition\`, \`target.state\`) pair the canon does not list in \`legal_pairs\`. The two fields answer DIFFERENT questions -- whether the v2 command survives, and what v3 does -- so most combinations are meaningful and a few are self-contradictory; the matrix is where that judgement is recorded rather than inferred. Two of the pairs are agreement constraints with teeth: \`retire\` and \`new-surface\` are one fact seen from two sides, and \`Entry::is_shipped()\` reads both fields and fails OPEN, so a single hand-edit out of agreement ships a retired command. Add the pair to \`legal_pairs\` with a gloss if it is legal, or fix the row:
$(printf '%s' "$PAIR_ILLEGAL" | sed 's/^/  /')"

# The review markers are only worth anything if they are legible to the
# reviewer, so a marker that names nothing is itself a defect: `uncertain: []`
# reads as "reviewed and confident" in a diff and as "somebody meant to fill
# this in" to the author, and no reader can tell which. Same for an
# `mcp_review` block with no content.
MCP_EMPTY_REVIEW="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select(has("mcp_review") and (
        ((.mcp_review | length) == 0)
        or ((.mcp_review.uncertain // null) != null and (.mcp_review.uncertain | length) == 0)))
        | .path)
  | join("\n")' "$IN")"
[ -z "$MCP_EMPTY_REVIEW" ] || die "rows carry an empty \`mcp_review\` -- a review marker that names no field is indistinguishable from a confident row in a diff. Either name the soft field or drop the block:
$(printf '%s' "$MCP_EMPTY_REVIEW" | sed 's/^/  /')"

# --- EXP-05: a flag cannot join the surface by being typed ------------------
# AC-06.8. `is_shipped()` reads an entry disposition, so a retired COMMAND never
# reaches clap; there was no equivalent one level down, and `spine.rs` built
# every declared flag on every shipped entry unconditionally. A table that can
# withdraw a command and cannot withdraw a flag is the gap, and the fix is a
# declaration rather than a heuristic: whether a flag is READ is a property of
# the renderer and never of its spelling.
#
# FIRST: `flags` must be PRESENT, not merely non-null. `flagsig` renders a
# missing key and an empty array identically as `--`, so four new_surface rows
# that had never authored their flags were indistinguishable in the view from
# rows that genuinely have none -- and a per-flag refusal cannot see a row with
# no flags array to walk. An absent key is an unanswered question wearing the
# costume of a settled one.
FLAGS_ABSENT="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select((.flags | type) != "array") | .path) | join("\n")' "$IN")"
[ -z "$FLAGS_ABSENT" ] || die "rows do not declare \`flags\` as an array -- an absent key and \`[]\` render identically as \`--\`, so 'this command has no flags' and 'nobody has said' are the same glyph. Write \`[]\` to mean none:
$(printf '%s' "$FLAGS_ABSENT" | sed 's/^/  /')"

# SECOND: every declared flag declares a disposition, in vocabulary.
# `keep` ships and must be read - `retire` is recorded from v2 and never reaches
# clap - `pending` does not ship - `intrinsic` ships and clap supplies it, so the
# renderer is not expected to read it.
#
# `pending` deliberately does NOT refuse. Ruled against the stricter option for
# the reason that cost four nodes commits in one afternoon: a guard that must be
# bypassed is a guard nobody keeps. The quiet-absence risk it leaves behind is
# answered elsewhere and on purpose -- `doctor` reports the pending count, so an
# undecided flag is visible without being a roadblock.
# The vocabulary is read from `flag_dispositions` in the canon rather than
# hardcoded here. It used to be the literal list `IN("keep","retire","pending",
# "intrinsic")` in this line -- a vocabulary living in a script that checks a
# file, which is the same shape as `target.state`'s having lived in a doc
# comment. One home, and the check reads it.
vocab_or_die "flag_dispositions"
FLAG_UNDECLARED="$(jq -r '
  (.flag_dispositions // [] | map(.value)) as $vocab
  | [.families[].entries[], .new_surface[]]
  | map(. as $e | (.flags // []) | map(select((.disposition // "") | IN($vocab[]) | not)
      | $e.path + ":" + (.spellings[0] // "?"))) | add // [] | join("\n")' "$IN")"
[ -z "$FLAG_UNDECLARED" ] || die "flags do not declare a \`disposition\` in vocabulary (keep, retire, pending, intrinsic) -- without one the spine builds every declared flag unconditionally, so a flag joins the v3 surface by being typed into this file. Offending flags:
$(printf '%s' "$FLAG_UNDECLARED" | sed 's/^/  /')"

# THIRD: a retired command cannot ship a flag. This is the one rule here that is
# derivable rather than authored, so it is checked rather than trusted -- an
# inherited value is exactly the kind that gets hand-edited out of agreement
# with the thing it was inherited from, and nothing else would notice.
FLAG_ORPHAN="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select(.disposition == "retire") | . as $e | (.flags // [])
      | map(select(.disposition | IN("keep","pending")) | $e.path + ":" + (.spellings[0] // "?")))
  | add // [] | join("\n")' "$IN")"
[ -z "$FLAG_ORPHAN" ] || die "flags declare they ship on a command that does not -- a retired command never reaches clap, so neither can its flags. Reconcile the flag with its entry:
$(printf '%s' "$FLAG_ORPHAN" | sed 's/^/  /')"

# FOURTH: a retired command cannot be an agent tool, which is the SAME derivable
# rule as FLAG_ORPHAN one level up. `exposed_on_mcp` is already refused when
# ABSENT, so every row states it -- but nothing checked it against whether the
# command ships, and a tool list is exactly where that goes unnoticed: the MCP
# surface has no `--help` a human reads by accident, so a retired command sitting
# in it is invisible until an agent calls it and gets nothing.
#
# GREEN WHEN ADDED, and deliberately added anyway, on FLAG_ORPHAN's own
# argument: an inherited value is the kind that gets hand-edited out of agreement
# with its source. `st_zero` went `retire` today and its three flags had to be
# reconciled by hand -- had it been `exposed_on_mcp: true`, nothing here would
# have said so. Mutation-tested rather than assumed, because a refusal that has
# never fired is indistinguishable from one that cannot.
MCP_ORPHAN="$(jq -r '
  [.families[].entries[], .new_surface[]]
  | map(select(.exposed_on_mcp == true)
      | select((.disposition == "retire") or (.target.state == "retire"))
      | .path)
  | join("\n")' "$IN")"
[ -z "$MCP_ORPHAN" ] || die "commands are exposed as agent tools but do not ship -- a retired command never reaches clap, so it cannot be an MCP tool either. Reconcile \`exposed_on_mcp\` with the retirement:
$(printf '%s' "$MCP_ORPHAN" | sed 's/^/  /')"

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
  (.target | targetextra),
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
    (flagblock // empty),
    (if .observed then
      "- **Exit codes:**\n" + (.observed.exit | map("  - `\(.code)` -- \(.when)") | join("\n"))
     else "- **Observed:** nothing to observe -- no v2 antecedent, so there was never anything to run" end),
    (if .observed then "- **stdout:** \(.observed.stdout | cell)" else empty end),
    (if .observed then "- **stderr:** \(.observed.stderr | cell)" else empty end),
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
    (.target | targetextra),
    mcpline,
    mcpreview,
    (if .cross_ref then "- **Cross-reference:** \(.cross_ref)" else empty end),
    (entryextra),
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
# NO `emit ""` here: each detail block below LEADS with its own blank line, so
# emitting one as well produced two in a row -- which the formatter collapses,
# breaking the fixed point. One owner per separator.

# EVERY authored field is rendered, because an authored field the view omits is
# unreviewable. `sync` carried a note whose second clause was wrong for a day and
# the human-reviewable face of this canon did not show it -- so the only thing
# that could ever have caught it was the author re-reading their own code, which
# is what happened. A view that silently drops a field is not a view of the file.
#
# THAT PARAGRAPH WAS ALREADY HERE AND THE CODE UNDER IT DROPPED FIELDS ANYWAY.
# It enumerated `acceptance` and `note`, so every later field -- the truth-model
# corrections, the export/backup distinction, the D34 wording, the whole `target`
# block including `backup`s `VACUUM INTO` requirement -- was written, committed,
# and invisible in the view. The rule was stated correctly and enforced by an
# enumeration that could not keep up with it: documentation reminds, only a
# control is load-bearing, demonstrated by the comment that says so.
jq -r "$JQ_LIB"'
  .new_surface[]? |
  # `// null`, NOT `// empty`, and the difference silently ate three entries.
  # `empty as $x | ...` yields NOTHING AT ALL -- the binding has no value to make,
  # so the whole expression vanishes rather than binding an absent one. So an
  # entry whose target held only `state` (sync, export, ingest) produced no output
  # even though its TOP-LEVEL extras were non-empty. Caught by re-checking the
  # field list, not by reading the output: the section still looked populated,
  # because `backup` -- the entry with both halves non-empty -- rendered fine.
  (extras(["path","args","flags","help","owner_wp","basis","target",
           "exposed_on_mcp","read_or_mutate","mcp_review"]) // null) as $top |
  ((.target | targetextra) // null) as $tgt |
  # Bespoke like the family entries, and the same three names are skipped above
  # so they are not rendered twice. Unlike $top/$tgt this is never null: every
  # row declares both fields or the run has already refused, so the block below
  # can no longer be empty -- which is why `daemon` and `mcp`, whose only other
  # authored key is `target.state`, now appear here at all. They were the two
  # rows this section had least to say about and the two the exposure question
  # is sharpest for.
  (mcpline + (mcpreview // "" | if . == "" then "" else "\n" + . end)) as $mcp |
  # `flags` is skipped in $top above on the promise that something renders it,
  # and for five flags nothing did. Same renderer as the family path.
  flagblock as $flg |
  # The separator blank line LEADS each block rather than trailing it. jq -r
  # already appends a newline per output, so a trailing "\n" produced a blank
  # after EVERY entry including the last -- and a trailing blank line at EOF is
  # not a fixed point of the formatter, which strips it. Leading gives exactly
  # one blank between entries and none at the end.
  "\n### `" + .path + "`\n\n"
    + ([$top, $flg, $tgt, $mcp] | map(select(. != null)) | join("\n"))
' "$IN" >> "$OUT_TMP"

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
# The aligner moved to lib_mdfmt.sh when gen_register.sh was found to have
# exactly the same formatter skew -- 232 differing lines between its committed
# view and a fresh regeneration of identical data. Two generators, one concern,
# so the awk lives once and both source it.
. "$HERE/lib_mdfmt.sh" || die "cannot source $HERE/lib_mdfmt.sh -- refusing to emit a view that will not survive the formatter"
md_align "$OUT_TMP" "$OUT_TMP.aligned" || die "table alignment failed"
mv "$OUT_TMP.aligned" "$OUT_TMP" || die "table alignment failed to land"

# --- INVARIANT INTEGRITY: citations resolve, and nothing is orphaned --------
#
# The canon declares 8 invariants and cites them by ID across 103 rows. Nothing
# checked that a citation RESOLVES. `INV-09` would have read exactly like a real
# reference -- and the whole point of the entry-level `defects` design is that
# "the rule text lives in exactly one place, the invariant", so a citation that
# points nowhere silently reintroduces the divergent copy it was built to avoid,
# in the direction nobody looks (a reference to nothing, rather than a second
# copy of something).
#
# THE POPULATION IS EVERY `INV-NN` TOKEN IN THE FILE, not the `defects[].id`
# array. Measured before writing this: citations live in BOTH -- the structured
# array AND free text like "outside a project ... (INV-03)". A check scoped to
# the structured half would have covered part of the population and reported
# confidently on all of it, which is this toolchain's most-repeated failure.
# Scanning every string is the enumerate-the-population form.
#
# Both directions, because they are different defects. A dangling citation is a
# typo. An UNCITED invariant is an orphan -- a rule that survived the removal of
# every row that referenced it, still declared, still read as governing, and
# governing nothing. Neither shows up in a diff.
INV_DECLARED="$(jq -r '[.invariants[].id] | sort | join(" ")' "$IN")"
INV_DANGLING="$(jq -r '
  ([.invariants[].id]) as $ok
  | [.. | strings | scan("INV-[0-9]+")] | unique
  | map(select(. as $i | $ok | index($i) | not)) | join(" ")' "$IN")"
# `del(.id)` on the invariants before scanning, and it is the whole check.
# WITHOUT IT THIS IS VACUOUS: the scan reads every string in the file, so an
# invariant's own `id` field counts as a citation of itself and NOTHING can ever
# be uncited. The first version had that bug, the mutation test caught it -- a
# declared-and-never-cited INV-99 sailed through -- and the measurement I had
# run by hand minutes earlier ("every invariant is cited somewhere") could not
# have returned any other answer. A check that cannot fail is not a weak check,
# it is a decoration, and this one had already produced a reassuring result.
#
# Only the `id` is removed, not the whole block: an invariant's rule text may
# legitimately cite ANOTHER invariant, and dropping the block would turn those
# into false orphans.
INV_UNCITED="$(jq -r '
  ((.invariants |= map(del(.id))) | [.. | strings | scan("INV-[0-9]+")] | unique) as $cited
  | [.invariants[].id] | map(select(. as $i | $cited | index($i) | not)) | join(" ")' "$IN")"
[ -n "$INV_DECLARED" ] || die "the canon declares no invariants -- either the block was removed or this needle has stopped matching"
[ -z "$INV_DANGLING" ] || die "citation(s) to undeclared invariant(s):$INV_DANGLING -- the rule text lives in exactly one place, so a citation pointing nowhere is a divergent copy in the direction nobody checks"
[ -z "$INV_UNCITED" ] || die "declared but never cited:$INV_UNCITED -- an invariant no row references is an orphan that still reads as governing. Cite it or retire it; leaving it is the stale-canon shape."

# INV-04 asserts the shipped surface exits 0, 1 or 2 ONLY. That is a claim about
# MEASURED data sitting in the same file as the measurements, so it can be
# checked against them rather than trusted. REPORTS THE DISAGREEMENT AND REFUSES
# rather than picking a side: a code outside the set means either a genuinely new
# exit path was measured (INV-04 needs updating) or a row is wrong, and those
# have opposite remedies. Same posture as drift_check.sh, for the same reason.
EXIT_ODD="$(jq -r '[.families[].entries[] | (.observed.exit // [])[] | .code]
  | unique | map(select(. != 0 and . != 1 and . != 2)) | join(" ")' "$IN")"
[ -z "$EXIT_ODD" ] || die "observed exit code(s) outside INV-04's set of 0/1/2:$EXIT_ODD -- the canon contradicts its own invariant. Either a new exit path was measured and INV-04 must be updated, or a row is wrong. Decide; this refuses rather than choosing."

# --- FORMATTER FIXED POINT: the view must survive the repo formatter --------
#
# lib_mdfmt.sh names two causes of generator/formatter skew and fixes only the
# first: it aligns tables (layout the renderer controls) and deliberately does
# NOT rewrite cell content, because rewriting a value would corrupt data. Its
# ruling on cause 2 -- MARKUP THE DATA CARRIES -- is "author the canon in the
# form the formatter already agrees with".
#
# THAT RULING HAD NO CONTROL, AND I BROKE IT THE DAY I RELIED ON IT. Four canon
# strings written today carried `*emphasis*`; the formatter rewrites it to
# `_emphasis_`; so the committed view could never again match its generator. It
# cost a blocked commit and a long diagnosis, and the message it blocked on --
# "the committed view is stale, or a row was hand-edited" -- named neither cause.
#
# REFUSING IS NOT REWRITING. The library ruled out silently normalising content,
# and it was right; this refuses and names the offending lines instead, so the
# rule stays "author it correctly" and becomes enforceable rather than merely
# stated. Checking the fixed point EXACTLY also beats matching emphasis with a
# regex: a pattern approximating CommonMark would false-positive on the globs
# this canon is full of (`bin/intent_*`, `doc_sections_*`), and it would only
# ever catch the one markup class I already know about.
if command -v npx >/dev/null 2>&1; then
  cp "$OUT_TMP" "$OUT_TMP.check.md" || die "cannot copy the render for the formatter check"
  if npx --no-install prettier --write "$OUT_TMP.check.md" >/dev/null 2>&1; then
    if ! diff -q "$OUT_TMP" "$OUT_TMP.check.md" >/dev/null 2>&1; then
      echo "error: the rendered view is NOT a fixed point of the repo formatter -- it would be rewritten the instant it landed, and every later regeneration would diff against the committed file for ever." >&2
      diff "$OUT_TMP" "$OUT_TMP.check.md" 2>&1 | sed -n '1,10p' | sed 's/^/  /' >&2
      rm -f "$OUT_TMP.check.md"
      die "author the canon in the form the formatter already agrees with (lib_mdfmt.sh, cause 2) -- most often *emphasis* which must be written _emphasis_"
    fi
  else
    # NOT a silent skip. A check that cannot run must say so, or its silence
    # reads as a pass -- the whole nothing-is-wrong / nothing-ran class.
    echo "note: prettier did not run, so the formatter fixed-point check was SKIPPED -- the pre-commit gate is the backstop" >&2
  fi
  rm -f "$OUT_TMP.check.md"
else
  echo "note: npx not found, so the formatter fixed-point check was SKIPPED -- the pre-commit gate is the backstop" >&2
fi

# --- COMPLETENESS: no authored field may be missing from the view -----------
#
# THE SKEW CHECK CANNOT CATCH THIS AND NEVER COULD. It asks whether the committed
# view matches what this generator PRODUCES, so a generator that drops a field is
# a perfect fixed point with itself and reports ok forever. Skew tests
# RE-DERIVABILITY; this tests COMPLETENESS, and they are different properties.
# Measured when the gap was found: 20 distinct `target` sub-fields in the canon,
# 5 named by the renderer, 15 dropped in silence -- including the backup config
# keys another node was blocked on and `doctor`s two new obligations.
#
# Every key NOT rendered bespoke must appear via the generic `extras` path. The
# skip lists below are the SAME sets passed to `extras`, which is what makes this
# non-circular: it compares the canon's keys against the rendered TEXT, so if the
# generic path stops firing the check goes red rather than quiet.
#
# This refuses rather than warns. The class has now bitten twice in one sitting
# -- once as an enumeration that could not keep up with the fields, once as
# `empty as $x` silently voiding three whole entries -- and both times the view
# still LOOKED populated. A one-off sweep would not have caught the second,
# because nothing downstream ever contradicts a sweep.
MISSING_FIELDS=""
for key in $(jq -r '
    [ (.families[].entries[] | (.target // {}) | keys[]),
      (.new_surface[] | (.target // {}) | keys[]),
      # INVARIANTS carry a `target` too and were not in this list. They render
      # through the same `targetextra`, so the generic path covered them by luck
      # while the CHECK did not -- and a rendered-but-unguarded field is one
      # renderer refactor away from being an unrendered one nobody notices.
      (.invariants[] | (.target // {}) | keys[]) ]
    - ["state","ratification","behaviour","question","note"]
    | unique | .[]' "$IN"); do
  label="$(printf '%s' "$key" | tr '_' ' ')"
  grep -qF -- "**$label:**" "$OUT_TMP" || MISSING_FIELDS="$MISSING_FIELDS $key"
done
for key in $(jq -r '
    [ .new_surface[] | keys[] ]
    - ["path","args","flags","help","owner_wp","basis","target",
       "exposed_on_mcp","read_or_mutate","mcp_review"]
    | unique | .[]' "$IN"); do
  label="$(printf '%s' "$key" | tr '_' ' ')"
  grep -qF -- "**$label:**" "$OUT_TMP" || MISSING_FIELDS="$MISSING_FIELDS $key"
done
# FAMILY ENTRY-LEVEL KEYS HAD NO CHECK AT ALL. The two loops above cover
# `target` sub-keys and `new_surface` top-level keys, which is what the gap
# looked like the day it was found -- and it left the biggest population in the
# file, the 100-odd family entries, entirely unguarded. The three MCP fields
# authored under AC-09.1 are entry-level, so they would have been written,
# committed, and invisible in the view while both existing loops stayed green.
# A completeness check with a hole in it is the same defect it was built to
# catch, one level up.
# THE SAME LIST as `entryextra` above, and it has to stay the same list: this
# loop compares the canon keys against the rendered TEXT, so if the generic path
# stops firing the check goes red rather than quiet. Skipping a key HERE that
# `extras` renders is harmless; skipping one that nothing renders is the hole.
for key in $(jq -r '
    [ .families[].entries[] | keys[] ]
    - ["path","help","args","flags","v2","aliases","observed","target",
       "disposition","cross_ref",
       "exposed_on_mcp","read_or_mutate","mcp_review"]
    | unique | .[]' "$IN"); do
  label="$(printf '%s' "$key" | tr '_' ' ')"
  grep -qF -- "**$label:**" "$OUT_TMP" || MISSING_FIELDS="$MISSING_FIELDS $key"
done
# FLAG-LEVEL KEYS, the third and last population. The two loops above walk
# `target` sub-keys and top-level keys; neither descends into `flags`, so the six
# bespoke flag keys were the whole of what any check knew about. `default` and
# `required` had been in the canon and out of the view for as long as they had
# existed, and no check anywhere was capable of saying so.
# THE SAME LIST as `flagextra`, for the same reason the entry list must match
# `entryextra`: this compares canon keys against rendered TEXT, so a key skipped
# here that nothing renders is the hole the loop exists to find.
for key in $(jq -r '
    [ (.families[].entries[] | (.flags // [])[] | keys[]),
      (.new_surface[] | (.flags // [])[] | keys[]) ]
    - ["spellings","type","help","value","accepts","note"]
    | unique | .[]' "$IN"); do
  label="$(printf '%s' "$key" | tr '_' ' ')"
  grep -qF -- "**$label:**" "$OUT_TMP" || MISSING_FIELDS="$MISSING_FIELDS flags.$key"
done

# The three skipped above are rendered BESPOKE, so their skip is a promise that
# something else renders them. An unverified promise in a completeness check is
# how the check becomes decoration, so it is verified here against the rendered
# text: every row emits one `- **MCP:**` line, and the count must match.
# AND THE FLAG LOOP ABOVE NEEDS THE SAME TREATMENT, for a reason it demonstrated
# on its own first run: it greps for the LABEL, so one rendering path satisfies
# it for every path. 88 of 93 flags rendered a `disposition` and the loop went
# green, because presence-of-label and completeness-of-population are different
# questions and only the second one matters. Counted, not sniffed.
FLAG_COUNT="$(jq -r '[.families[].entries[], .new_surface[]] | map(.flags // []) | add | length' "$IN")"
FLAG_RENDERED="$(grep -cE '^    - \*\*disposition:\*\* ' "$OUT_TMP" || true)"
[ "$FLAG_RENDERED" = "$FLAG_COUNT" ] || die "the view renders $FLAG_RENDERED flag dispositions for $FLAG_COUNT declared flags -- every flag must reach the view through the shared renderer, and a label-presence check cannot see a whole path that renders none"

MCP_ROWS="$(jq -r '[.families[].entries[], .new_surface[]] | length' "$IN")"
MCP_RENDERED="$(grep -cF -- '- **MCP:**' "$OUT_TMP" || true)"
[ "$MCP_RENDERED" = "$MCP_ROWS" ] || die "the view renders $MCP_RENDERED MCP lines for $MCP_ROWS declared rows -- the bespoke renderer is skipped by the completeness loops on the promise that it fires for every row, and it did not"
[ -z "$MISSING_FIELDS" ] || die "the view drops authored field(s) the canon carries:$MISSING_FIELDS -- a view that silently omits a field is not a view of the file, and the omitted field is unreviewable by anyone who reads the view instead of the JSON"

# Only now, with the whole view rendered, aligned and proved complete, does the
# committed file change.
mv "$OUT_TMP" "$OUT" || die "cannot write: $OUT"
trap - EXIT

echo "ok: rendered $(jq -r '[.families[].entries | length] | add' "$IN") entries across $FAMILY_COUNT family(s) -> $OUT" >&2
