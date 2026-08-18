#!/bin/bash
# conservation_check.sh -- did the migration lose anything, and can it say so?
#
# AC-10.5's three checks, in one pass over two enumerations that were built
# without reading each other: `estate_census.sh` says what the v2 estate
# CONTAINED, the migrated tree says what the migrator PRODUCED.
#
#   artefact conservation  every file the estate held is CONVERTED, RELOCATED,
#                          named OUT-OF-MODEL, or reported as residue. There is
#                          no fifth disposition, and "still on disk" is not one
#                          of the four.
#   semantic completeness  every countable entity survives, by id and not only
#                          by count -- two threads that swap ids conserve the
#                          count perfectly.
#   prose conservation     every authored section has a destination, and the
#                          bytes that arrive are the bytes that left.
#
# WHY "STILL ON DISK" IS NOT A DISPOSITION, WHICH IS THE ONE THING THIS TOOL
# EXISTS TO SAY (ic, 2026-08-17, measured before the check was built). v2
# expresses a thread's status as a DIRECTORY, so 55 of 56 threads live at
# `intent/st/<BUCKET>/<ID>/` while v3's canonical path is `st/<ID>/`. A
# migration writes fresh canon at the flat path and leaves every bucketed file
# where it was. Nothing is deleted. Nothing is corrupted. **A check comparing
# two file listings returns 100% while the model points at one path and half the
# estate's authored prose sits at another.** 387 files at the pinned corpus
# revision: 194 regenerated at the flat path -- which DOUBLES them, two
# `info.md` per thread, one generated and one v2 artefact nothing regenerates
# and everything still reads -- and 193 authored files the model does not hold.
#
# So reachability is the predicate, not presence. A file is conserved when the
# migrated model can find it, and `intent/st/COMPLETED/ST0001/design.md` is not
# findable from a model that points at `st/ST0001/`.
#
# WHAT A GREEN FROM THIS TOOL DOES NOT COVER, STATED SO NOBODY HAS TO INFER IT
# (dc, 2026-08-17). There are exactly two ways a migration loses content while
# every count reconciles, and they fail in OPPOSITE directions. **The record
# survives and its INTERIOR is dropped** -- a count of threads cannot see a
# missing section, so the check must look inside the record. That axis is
# covered here: every `## ` section is hashed and compared. **The record
# survives BYTE-FOR-BYTE and its REFERENTS stop resolving** -- `.claude/
# settings.json` is nothing but three `intent claude hook ...` invocations, so
# byte-identity is a claim about the file and says nothing about whether the
# commands it names still answer. **That axis is NOT covered here**, and a check
# built for either one is structurally silent about the other. AC-10.4 owns the
# `.claude/**` half. Recorded rather than built, deliberately: an unstated limit
# is how a green starts meaning something its author never claimed.
#
# A THIRD AXIS, AND THIS TOOL IS BLIND TO IT TOO (cc, 2026-08-18, found by a
# different method entirely). **The record survives, its prose survives, and a
# MODELLED SCALAR is corrupted.** Three issue titles in live canon -- 0011,
# 0014, 0035 -- carry literal double quotes, because `legacy.rs` strips v2's
# YAML quoting from `id` and from `wp.title` and not from `issue.title`.
#
# Every arm here returns clean on that. The census emits CORPUS, COUNT, ENTITY,
# FILE and PROSE rows; ENTITY checks identity by id so the entity IS present,
# COUNT reconciles, and PROSE hashes `## ` SECTION bodies so nothing compares a
# field. **A title can gain two bytes in canon and this tool reports 0
# findings** -- the same shape as the two axes above: a check built for section
# interiors is structurally silent about field values. Recorded, not built. cc
# found it by reading the parser, and a conservation check that silently did
# not cover it would have been the more expensive discovery.
#
# IT REFUSES RATHER THAN PASSES WHEN IT CANNOT SEE ITS SUBJECT. A migrated root
# with no `st/` canon exits 2, and so does an empty census. Both would otherwise
# produce a green over an empty comparison, which is the wrong-zero this whole
# programme keeps paying for: the same shape that let `legacy.rs` never read the
# issue estate while every count reconciled against zero.
#
# THE OUT-OF-MODEL SET IS THE MIGRATOR'S TO NAME, NOT THIS TOOL'S TO KNOW.
# `--out-of-model <file>` takes the migrator's own enumeration, one path per
# line. Hardcoding the set here would let the check certify its own denominator,
# and AC-10.8 already says the out-of-model set must be NAMED in the output
# rather than silently absent. A migrator that names nothing gets every
# unowned file reported UNACCOUNTED, which is the correct and loud answer.
#
# HOW TO GATE ON THIS TOOL, AND IT IS NOT ON THE COUNTERS (ic, 2026-08-18,
# DRIVEN on baize at `a519398d` -- real capture, real `intent upgrade`, this
# script at HEAD; I verified the three runs off their trees rather than took
# the numbers). I wrote the deletion precondition as "LOST-PROSE and
# UNACCOUNTED at ZERO" and THAT PHRASING IS WRONG. Both counters are
# independently zeroable while the estate is not conserved, and neither attack
# needs a bug in this file:
#
#   strip every PROSE row from the census   ->  LOST-PROSE 33 -> 0, ALTERED 103 -> 0
#   pass the 135 unowned paths --out-of-model ->  UNACCOUNTED 135 -> 0
#
# The second is this tool's DESIGNED behaviour, stated in the paragraph above.
# A gate phrased on that counter re-admits exactly the denominator-certification
# the flag exists to refuse -- through the gate's wording instead of the code.
#
# NEITHER ATTACK PRODUCED A GREEN, and that is the half worth keeping. Emptying
# the prose population made this tool LOUDER, not quieter: 522 findings -> 789,
# because the reverse arm rose ADDED 0 -> 403 and caught in C2 precisely what
# C1 had lost the ability to see. The out-of-model run held at exit 1 / 387.
# The two-directional design is what defeats the attack; a forward-only check
# would have returned a clean LOST-PROSE 0 with nothing else moving.
#
# SO GATE ON THE VERDICT AND ITS SCOPE, NEVER ON A COUNTER:
#
#   1. exit 0 AND a printed `conservation: 0 finding(s)` line. The ABSENCE of
#      that line is a REFUSAL, not a zero. Read the line; never parse a count
#      out of output that was never produced.
#   2. the printed denominator. `compared 700 of 1211` against `compared 0 of 0`
#      is the entire difference between the control and the emptied population,
#      and a gate reading two counters cannot see it. Require compared plus NOT
#      compared to equal the census total, and require every NOT-compared kind
#      to name the arm that covers it.
#   3. the SUBJECT, checked at the MOMENT OF THE ACT rather than over a window
#      (ic sharpened this, 2026-08-18, and the sharpening is what makes it
#      satisfiable). A verdict describes whatever `conservation: SUBJECT`
#      names, and an unpinned run describes a DIRECTORY that peers are still
#      writing to -- so refuse an unpinned subject flat; the tool already says
#      `unpinned` in its own voice. My first phrasing asked for a tree
#      "demonstrated identical" to the measured one, which reads as a property
#      of a time WINDOW, and in a repo with four live writers NO window is ever
#      demonstrably quiet -- unsatisfiable in precisely the situation it exists
#      for. Instead: digest the measured tree, RE-COMPUTE that digest
#      immediately before the irreversible step, and refuse on any difference.
#      Same guarantee, and it asks the deleter to prove the tree it is deleting
#      from is the tree that was measured, rather than asking anyone to prove a
#      quiet interval.
#   4. a DEMONSTRATED RED on a subject of the same shape. ic's control above is
#      that demonstration for this tool; cite it rather than re-argue it.
#
# WHICH DELETION THIS GATE LICENSES, BECAUSE THE PRECONDITION HAS TWO CONSUMERS
# AND WAS WRITTEN FOR ONE (ic asked, 2026-08-18, before running rather than
# inside a verdict). Two different populations are up for deletion and they need
# different instruments:
#
#   1. THE v2 RESIDUE -- `intent/issues/<BUCKET>/NNNN/*.md` and the bucketed
#      thread files, 443,643 bytes of issue prose alone. The claim that has to
#      hold is "the migration carried them", so the subject is a PINNED v2
#      census against a FRESH migration of that same estate. **This gate, and
#      this gate is the whole answer for this population.**
#
#   2. RENDERED VIEWS UNDER DISK-OPTIONAL -- `organize` dehydrating a `.md`
#      whose artefact is not realised. **NOT licensed by any conservation run,
#      and a conservation run is the WRONG instrument for it.** These are
#      licensed per-file at the moment of the act by the gate in
#      `realisation.md` 5.1: re-render from the store, compare bytes, refuse on
#      any difference. That is STRICTLY STRONGER here -- per file rather than
#      per estate, current rather than inherited, and checked at the instant of
#      deletion rather than carried over from a measurement of another tree.
#
# So a census of the pinned v2 estate against TODAY'S LIVE REPO answers no
# question anyone has. It would fold the migration's conservation together with
# every commit, the flatten at `1af21f4e`, and four nodes' authoring since --
# and could not separate them. **A verdict whose subject is "the migration plus
# everything that happened afterwards" is the instrument-independent-of-its-
# subject failure with a longer lever.**
#
# AND `LOST-PROSE 0` CARRIES NO DENOMINATOR EVEN WHEN IT IS HONEST. On a
# perfect run this tool is silent about 511 of 1211 sections -- the criterion
# and test kinds it declares uncompared, identity covered by LOST-ac/LOST-at.
# It discloses that inline, correctly. A gate phrase reading "LOST-PROSE 0"
# reads as 1211 when it means 700. Scope in a denominator, not an adjective.

set -uo pipefail

die() {
  echo "conservation: $*" >&2
  exit 2
}

CENSUS="" MIGRATED="" OOM="" BINARY="" DISPO=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out-of-model) OOM="${2:-}"; shift 2 || die "--out-of-model needs a file" ;;
    --binary) BINARY="${2:-}"; shift 2 || die "--binary needs a path" ;;
    --dispositions) DISPO="${2:-}"; shift 2 || die "--dispositions needs a file" ;;
    -*) die "unknown flag: $1" ;;
    *)
      if [ -z "$CENSUS" ]; then CENSUS="$1"
      elif [ -z "$MIGRATED" ]; then MIGRATED="$1"
      else die "unexpected argument: $1"
      fi
      shift
      ;;
  esac
done

[ -n "$CENSUS" ] && [ -n "$MIGRATED" ] ||
  die "usage: conservation_check.sh <census.tsv> <migrated-project-root> [--out-of-model <file>] [--binary <v3-intent>]"
[ -z "$BINARY" ] || [ -x "$BINARY" ] || die "no such executable: $BINARY"
[ -f "$CENSUS" ] || die "no such census: $CENSUS"
[ -d "$MIGRATED" ] || die "no such migrated root: $MIGRATED"
command -v jq >/dev/null 2>&1 || die "jq is required to read the canon"

n_census="$(awk -F'\t' '$1 == "FILE"' "$CENSUS" | wc -l | tr -d ' ')"
[ "$n_census" -gt 0 ] ||
  die "census names no files -- an empty denominator passes every migration ever written"

# WHICH ESTATE IS THIS A VERDICT ABOUT? Asked of the census rather than assumed
# from the filename, and printed on every run. Until 2026-08-17 the census TSV
# named nothing about itself, so four fleet members produced four files this tool
# could not tell apart -- and a verdict computed from the canary's census against
# a migrated Lamplight tree would have compared two unrelated estates and
# reported a number rather than a refusal.
awk -F'\t' '$1 == "PROSE" && $7 !~ /^[0-9a-f]{64}$/ { exit 1 }' "$CENSUS" ||
  die "census PROSE rows carry no trimmed sha -- produced before the census published both hashes, so this check cannot tell a content change from a whitespace normalisation; re-run estate_census.sh"

# `$3` IS POLYMORPHIC AND THE TRUNCATION WAS ONLY CORRECT FOR ONE OF ITS TWO
# SHAPES. estate_census.sh writes a git REVISION there when the estate carries a
# `.CAPTURE` marker and the estate's PATH when it does not, so `substr($3, 1, 12)`
# is a short sha on the first and `/Users/matts` on the second -- a real directory
# that is not the subject, printed beside the word "unpinned", which is itself
# correct. The reader is told the estate is unpinned and then told the wrong place.
#
# Found on the FIRST run against an unpinned estate, which was Intent's own -- the
# hoist subject. Every fleet member carries a marker, so this arm had never once
# executed in the life of the tool, and its greens were all from the pinned path.
SUBJECT="$(awk -F'\t' '$1 == "CORPUS" { print ($2 == "unpinned" ? $2 " @ " $3 : $2 " @ " substr($3, 1, 12)); exit }' "$CENSUS")"
[ -n "$SUBJECT" ] ||
  die "census carries no CORPUS record -- it was produced before the census named its own subject, and a verdict that cannot say which estate it describes is not a verdict; re-run estate_census.sh"
case "$SUBJECT" in
  unpinned*) echo "conservation: SUBJECT $SUBJECT -- an UNPINNED estate; this verdict describes a directory, not a revision" ;;
  *) echo "conservation: subject $SUBJECT" ;;
esac

# The canon root. `st/<ID>/thread.json` and `issues/<n>.json` are data-model.md's
# canonical paths; a tree with neither has not been migrated, and saying so is
# the whole difference between a refusal and a green.
# THE ARGUMENT IS A PROJECT ROOT -- the same kind of directory `estate_corpus.sh`
# captures and `estate_census.sh` reads. It did not used to be. `CANON` was the
# argument verbatim, so this tool alone among the three wanted the INTENT DIR,
# and on the first run against a real migrated tree its author passed the project
# root and got `holds no st/ canon` -- a REFUSAL, correctly worded, about a tree
# that had just been migrated successfully. One identifier, two subjects, in the
# session spent naming exactly that.
#
# Both spellings are accepted and **the resolution is PRINTED on every run**,
# because the two are distinguishable by structure and a silent choice between
# them is how the wrong tree gets measured without anyone finding out. Neither
# present is still a refusal: that arm is what stops an unmigrated tree passing.
if [ -d "$MIGRATED/intent/st" ]; then
  CANON="$MIGRATED/intent"
  echo "conservation: canon at $CANON (argument read as a project root)"
elif [ -d "$MIGRATED/st" ]; then
  CANON="$MIGRATED"
  echo "conservation: canon at $CANON (argument read as an intent dir)"
else
  die "$MIGRATED holds no st/ canon at either $MIGRATED/intent/st or $MIGRATED/st -- this is an UNMIGRATED tree, and a check that cannot see its subject does not pass it"
fi

WORK="$(mktemp -d)" || die "cannot create a scratch directory"
trap 'rm -rf "$WORK"' EXIT

: >"$WORK/oom"
[ -n "$OOM" ] && { [ -f "$OOM" ] || die "no such out-of-model file: $OOM"; sort -u "$OOM" >"$WORK/oom"; }

# The census's section names, per entity, with WP sequence numbers UNPADDED so the
# two sides agree on identity: the census reads `01` from a directory name and the
# canon holds `1` in a field.
awk -F'\t' -v OFS='\t' '$1 == "PROSE" {
  id = $3
  if ($2 == "wp") { n = split(id, p, "/"); sub(/^0+/, "", p[2]); id = p[1] "/" p[2] }
  print $2, id, $4
}' "$CENSUS" | sort -u >"$WORK/census_sections"

findings=0
: >"$WORK/log"
report() {
  echo "$1 $2" | tee -a "$WORK/log"
  findings=$((findings + 1))
}

# ---------------------------------------------------------------------------
# A. Artefact conservation -- reachability, not presence.
# ---------------------------------------------------------------------------
a_conv=0 a_reloc=0 a_oom=0 c_doubled=0 c_stranded=0
while IFS=$'\t' read -r _ path kind id bucket; do
  case "$kind" in
    thread|wp) st="${id%%/*}" ;;
    issue) st="" ;;
    *) st="" ;;
  esac

  # Owned by an entity whose canon exists at the flat path.
  if [ -n "$st" ] && [ -f "$CANON/st/$st/thread.json" ]; then
    base="${path##*/}"
    if [ "$bucket" = "-" ]; then
      a_conv=$((a_conv + 1))
      continue
    fi
    # A v2 artefact still sitting under its status bucket. THE MIGRATION DOES NOT
    # EMPTY THE BUCKETS -- deliberately, so a re-run does not collide on ids --
    # so every one of these is expected to still be on disk. Expected is not a
    # disposition, and the population splits in two with OPPOSITE ones. The test
    # is the only one that matters and it is asked of the canon, not of the name:
    # **does a counterpart exist under `st/<ID>/`?**
    #
    #   yes -> DOUBLED. A redundant original. The authored content reached canon
    #          (ALTERED 0 / ADDED 0 over the compared sections is the evidence),
    #          so the bucket copy is superseded and safe to rule out-of-model.
    #   no  -> STRANDED. **THIS IS THE ONLY COPY.** Authored prose -- design.md,
    #          impl.md, tasks.md -- that the migration neither moved nor named,
    #          reachable from nothing in the model. Not redundant, not
    #          out-of-model, just left behind. Half of a two-ended migration.
    #
    # Named STRANDED rather than UNREACHABLE because the class name is what a
    # reader acts on, and "unreachable" describes the file's position while
    # "stranded" describes its being the last copy. cc measured 171 of 269 bucket
    # documents in this population on Intent's own estate at `d4648020` and it is
    # the number that has to reach zero; it was being read as noise while it sat
    # in one merged count with a population that is genuinely fine.
    case "$base" in
      info.md|acceptance.md)
        if [ -f "$CANON/st/$st/$base" ]; then
          report DOUBLED "$path (also generated at st/$st/$base -- two artefacts, one role)"
          c_doubled=$((c_doubled + 1))
        else
          report STRANDED "$path (the only copy -- owner $id has canon at st/$st/ and the model regenerates this name, but nothing did)"
          c_stranded=$((c_stranded + 1))
        fi
        ;;
      *)
        if [ -f "$CANON/st/$st/$base" ]; then
          a_reloc=$((a_reloc + 1))
        else
          report STRANDED "$path (the only copy -- authored prose under $id's v2 bucket, neither moved nor named out-of-model)"
          c_stranded=$((c_stranded + 1))
        fi
        ;;
    esac
    continue
  fi

  if [ "$kind" = issue ]; then
    num="${id##*/}"
    if [ -f "$CANON/issues/$num.json" ]; then
      a_conv=$((a_conv + 1))
    else
      report UNCONVERTED "$path (issue $id has no issues/$num.json)"
    fi
    continue
  fi

  # Unowned, or owned by an entity with no canon at all.
  if grep -qxF "$path" "$WORK/oom"; then
    a_oom=$((a_oom + 1))
  elif [ -n "$st" ]; then
    report UNCONVERTED "$path (owner $id has no st/$st/thread.json)"
  else
    report UNACCOUNTED "$path (no owner, and the migrator did not name it out-of-model)"
  fi
done < <(awk -F'\t' '$1 == "FILE"' "$CENSUS")

# ---------------------------------------------------------------------------
# B. Semantic completeness -- by id, then by count.
# ---------------------------------------------------------------------------
census_ids() { awk -F'\t' -v k="$1" '$1 == "ENTITY" && $2 == k { print $3 }' "$CENSUS" | sort -u; }

canon_thread_ids() { find "$CANON/st" -name thread.json -maxdepth 2 2>/dev/null | while IFS= read -r j; do jq -r '.id' "$j"; done | sort -u; }
canon_wp_ids() { find "$CANON/st" -name thread.json -maxdepth 2 2>/dev/null | while IFS= read -r j; do jq -r '.id as $t | .wps[]? | "\($t)/\(.seq)"' "$j"; done | sort -u; }
canon_ac_ids() { find "$CANON/st" -name thread.json -maxdepth 2 2>/dev/null | while IFS= read -r j; do jq -r '.id as $t | .criteria[]? | "\($t)/\(.id)"' "$j"; done | sort -u; }
canon_at_ids() { find "$CANON/st" -name thread.json -maxdepth 2 2>/dev/null | while IFS= read -r j; do jq -r '.id as $t | .tests[]? | "\($t)/\(.id)"' "$j"; done | sort -u; }
canon_issue_ids() { find "$CANON/issues" -name '*.json' 2>/dev/null | sed 's|.*/||; s|\.json$||' | sort -u; }

compare_ids() {
  local kind="$1" left="$2" right="$3"
  local lost extra
  lost="$(comm -23 "$left" "$right")"
  extra="$(comm -13 "$left" "$right")"
  while IFS= read -r x; do [ -n "$x" ] && report "LOST-$kind" "$x (in the estate, not in the canon)"; done <<EOF
$lost
EOF
  while IFS= read -r x; do [ -n "$x" ] && report "INVENTED-$kind" "$x (in the canon, not in the estate)"; done <<EOF
$extra
EOF
}

# Issue ids are `<ARM>/<n>` in the census and bare `<n>` in the canon, because
# the arm is a v2 directory and v3 holds status in the record. Compared on the
# number, which is the identity both sides agree about.
census_ids thread >"$WORK/c.thread"; canon_thread_ids >"$WORK/m.thread"
census_ids wp | sed 's|/0*\([0-9]\)|/\1|' >"$WORK/c.wp"; canon_wp_ids >"$WORK/m.wp"
census_ids criterion >"$WORK/c.ac"; canon_ac_ids >"$WORK/m.ac"
census_ids test >"$WORK/c.at"; canon_at_ids >"$WORK/m.at"
census_ids issue | sed 's|.*/||' | sed 's|^0*||' >"$WORK/c.issue"; canon_issue_ids | sed 's|^0*||' >"$WORK/m.issue"

for k in thread wp ac at issue; do
  sort -u -o "$WORK/c.$k" "$WORK/c.$k"
  sort -u -o "$WORK/m.$k" "$WORK/m.$k"
  compare_ids "$k" "$WORK/c.$k" "$WORK/m.$k"
done

# ---------------------------------------------------------------------------
# C. Prose conservation -- a destination, and the same bytes on arrival.
#
# Only three thread sections are modelled: `Objective`, `Context` and `Related
# Steel Threads` (data-model.md, the info.md mixed-file resolution). Every other
# `## ` section in a thread has NO field to land in, and this is where that
# shows up as a count rather than as an opinion.
# ---------------------------------------------------------------------------

# section_text <file> <section-name> -- the same extraction the census performs,
# applied to the canon side. Written once and used by both the WP and issue arms
# so the two sides cannot drift into different ideas of where a section ends.
#
# IT TAKES A FILE AND NOT A STRING, AND THAT IS THE FIX FOR A DEFECT THAT MADE
# THIS WHOLE ARM INCAPABLE OF PASSING. Command substitution strips every trailing
# newline, and `jq -r` appends one, so a value that made a round trip through
# either arrived a byte different from the value that left. Both are silent, both
# are at the boundary, and prose conservation is a BYTE claim -- so every
# comparison reported ALTERED and no run could ever report CONSERVED.
#
# **THE POSITIVE CONTROL DID NOT CATCH IT BECAUSE I BUILT THE CONTROL AND THE
# CHECK TOGETHER.** The first demonstration constructed its fixture with
# `--arg o "$OBJ<newline>"`, compensating for jq's appended newline by hand, and
# reported "12 of 12 conserved". It agreed for a reason that had nothing to do
# with being right -- the same shape as a manifest regenerated by the act it is
# meant to verify, arriving in a test fixture. What found it was a fixture built
# by a different method, in a different language, at fleet scale: 564 of 564
# prose comparisons ALTERED, on a tree constructed to match.
section_text() {
  awk -v want="$2" '
    /^## / { name = substr($0, 4); next }
    { if (name == want) body = body $0 "\n" }
    END { printf "%s", body }
  ' "$1"
}

# trim_file <src> <dst> -- the same normalisation `legacy.rs`'s `sections()`
# applies (`buffer.trim()`), reproduced here ONLY so the two kinds of difference
# can be told apart. It is not adopted as truth: see `compare_prose`.
trim_file() {
  awk 'BEGIN { RS = "\0" } { gsub(/^[ \t\r\n]+|[ \t\r\n]+$/, ""); printf "%s", $0 }' "$1" >"$2"
}

# compare_prose <label> <estate-raw-sha> <estate-trim-sha> <canon-file>
#
# THREE OUTCOMES, NOT TWO, AND THE MIDDLE ONE IS THE POINT. `legacy.rs`'s
# `sections()` trims every section body, and the estate's raw section bytes
# include the blank line after the heading and the one before the next -- markdown
# LAYOUT rather than prose. A raw-byte equality therefore reports ALTERED for
# every section in every estate: 403 of 403 on Baize before this existed, which is
# a check that cannot pass and so cannot discriminate.
#
# **This tool does not rule that trimming is acceptable, and it must not.** That
# is the contract's call, and a check that silently adopted the migrator's own
# normalisation would be certifying it. What it does is REPORT WHICH KIND of
# difference occurred, and count them separately, because "the content changed"
# and "the whitespace was normalised" are two questions and answering both with
# one number is the shape this programme keeps paying for.
#
#   CONSERVED         bytes identical
#   NORMALISED-PROSE  identical after trim -- reported, counted, NOT a finding
#   ALTERED-PROSE     content differs -- a finding
#
# THE FIFTH ARGUMENT SPLITS *CONSERVED* INTO TWO POPULATIONS WITH THE SAME BYTES
# AND DIFFERENT MEANINGS (cc, 2026-08-17). A section that survives into a
# DECLARED field -- `Thread.objective`, `Thread.context` -- is modelled: the
# system knows what it is. A section that survives into a CATCH-ALL --
# `WorkPackage.body` (D28), and `Thread.body` when it lands -- is carried: the
# bytes are safe and the system knows nothing about them beyond their heading.
#
# **Conservation cannot tell them apart and should not try; the summary must.**
# Both are byte-equal and both are correct outcomes, so no finding is warranted
# either way. But the day someone asks whether Intent knows what threads relate
# to what, `## Related Steel Threads` will be sitting in a catch-all reading as
# CONSERVED, and a single number would answer yes when the answer is no. cc is
# about to move 55 threads from LOST to conserved by carrying the section
# verbatim -- the right fix, because the alternative parses 126 authored rows of
# which only 58% fit the declared `{id}: {note}` shape, and rendering them back
# would rewrite 53 rows nobody wrote. **A count that hid that would make the
# estate look more modelled than it is on the strength of a conservation win.**
#
# Same shape as the DOUBLED/STRANDED split above: one count, two populations,
# opposite answers to a question the count was not being asked.
# The migrator's own per-section drop records, keyed the way this tool labels
# sections. `--dispositions` takes the `upgrade` output (or any file holding its
# `byte-identical to` lines); absent it, nothing is declared and every removal
# stays a finding, which is the correct default for a check whose job is loss.
EMPTY_SHA="$(printf '' | shasum -a 256 | cut -d' ' -f1)"
c_drop=0
: >"$WORK/declared"
if [ -n "$DISPO" ]; then
  [ -f "$DISPO" ] || die "no such dispositions file: $DISPO"
  # `intent/st/<bucket>/ST0024/WP/06/info.md -- ## Acceptance -- byte-identical ...`
  # becomes `wp ST0024/06 'Acceptance'`, the label compare_prose is called with.
  # THE RECORD CARRIES A VERDICT AND THE TWO VERDICTS MEAN OPPOSITE THINGS HERE.
  # `dropped` takes the section OUT of canon, so canon must be empty. `deferred`
  # leaves canon untouched and changes only the view, so the section must still
  # compare byte-identical and is NOT a drop. Reading both as drops would expect
  # emptiness where content correctly remains.
  #
  # The verdict was inserted BETWEEN the heading and the evidence, and the first
  # version of this parser required those adjacent. It read 0 of 39 within hours
  # of being written -- and the guard below is the only reason that surfaced as
  # a refusal rather than as "ALTERED 39, loss detected" against a migration
  # that lost nothing.
  #
  # The sequence keeps the directory's OWN digits -- `WP/06` is labelled
  # `ST0024/06`, not `ST0024/6`. A `0*` here stripped the zero and matched only
  # the seq-10-and-above rows: 2 of 39. It still PRODUCED the class, so
  # "DECLARED-DROP appears in the output" would have read as working. Only the
  # count caught it, which is why the count is printed below.
  # TWO SHAPES, BECAUSE THERE ARE TWO POPULATIONS. A work-package drop is
  # `.../ST0024/WP/06/info.md` and a THREAD drop is `.../ST0024/info.md` -- no
  # `WP/NN` segment at all -- and `compare_prose` labels them differently too
  # (`wp ST0024/06 '<s>'` against `ST0024 '<s>'`, the call at the thread site).
  # The second sed cannot match a work-package line: it requires `ST[0-9]+`
  # IMMEDIATELY before `/info.md`, and what sits there in a WP path is the
  # sequence.
  {
    sed -nE "s|^[[:space:]]*.*/(ST[0-9]+)/WP/([0-9]+)/info\.md -- ## ([A-Za-z ]+) -- dropped --.*|wp \1/\2 '\3'|p" "$DISPO"
    sed -nE "s|^[[:space:]]*.*/(ST[0-9]+)/info\.md -- ## ([A-Za-z ]+) -- dropped --.*|\1 '\2'|p" "$DISPO"
  } | sort -u >"$WORK/declared"
  n_dec="$(wc -l <"$WORK/declared" | tr -d ' ')"
  n_raw="$(grep -cE ' -- dropped -- ' "$DISPO" || true)"
  n_defer="$(grep -cE ' -- deferred -- ' "$DISPO" || true)"
  {
    sed -nE "s|^[[:space:]]*.*/(ST[0-9]+)/WP/([0-9]+)/info\.md -- ## ([A-Za-z ]+) -- deferred --.*|wp \1/\2 '\3'|p" "$DISPO"
    sed -nE "s|^[[:space:]]*.*/(ST[0-9]+)/info\.md -- ## ([A-Za-z ]+) -- deferred --.*|\1 '\2'|p" "$DISPO"
  } | sort -u >"$WORK/deferred"
  # THE DENOMINATOR COMES FROM THE FILE, NEVER FROM THE PARSE, AND THAT IS THE
  # WHOLE POINT OF THIS REFUSAL. The earlier form asked only whether the parse
  # was ZERO, and printed `n_dec declared drop(s)` -- so when `Thread.body`
  # landed 35 thread-level drops this parser could not match, it read 80 of 115
  # and reported "80 matched of 80 declared". Complete, against its own blind
  # spot. All 35 were then reported as LOST-PROSE, and 32 of them carried the
  # sentence "the migrator did not name it" about sections the migrator had
  # named in the very file being parsed -- 13,698 bytes of loss that had been
  # explained. A partial parse is not a smaller version of a total parse
  # failure; it is the harder one, because it produces the class and looks
  # like it works. Equality, not "greater than zero".
  [ "$n_dec" -eq "$n_raw" ] ||
    die "$DISPO holds $n_raw drop record(s) and this tool parsed $n_dec -- a shape has moved or a new one has appeared, and the $((n_raw - n_dec)) unparsed record(s) would every one be reported as loss the migrator never explained"
  echo "conservation: dispositions -- $n_dec of $n_raw declared drop(s) parsed and $n_defer declared deferral(s) read from $DISPO"
fi

declared_drop() {
  [ -s "$WORK/declared" ] || return 1
  grep -qxF "$1" "$WORK/declared"
}

declared_defer() {
  [ -s "$WORK/deferred" ] || return 1
  grep -qxF "$1" "$WORK/deferred"
}

compare_prose() {
  local label="$1" raw="$2" trim="$3" file="$4" dest="${5:-modelled}" got
  got="$(shasum -a 256 <"$file" | cut -d' ' -f1)"
  # A DEFERRAL CLAIMS CANON IS UNCHANGED, SO EMPTY CANON REFUTES IT. Same
  # separation as the drop: the verdict is the migrator's CLAIM, the state of
  # canon is this tool's OBSERVATION. Checked before the conserved path so a
  # deferral that actually removed cannot pass as anything.
  if declared_defer "$label" && [ "$got" = "$EMPTY_SHA" ]; then
    report DEFERRAL-REFUTED "$label (declared deferred -- canon should be UNCHANGED -- and canon is empty, so it was removed)"
    return
  fi
  if [ "$got" = "$raw" ]; then
    c_ok=$((c_ok + 1))
    [ "$dest" = carried ] && c_carried=$((c_carried + 1)) || c_modelled=$((c_modelled + 1))
    return
  fi
  trim_file "$file" "$WORK/trim"
  got="$(shasum -a 256 <"$WORK/trim" | cut -d' ' -f1)"
  if [ "$got" = "$trim" ]; then
    echo "NORMALISED-PROSE $label (content identical; leading/trailing whitespace differs)"
    c_norm=$((c_norm + 1))
    [ "$dest" = carried ] && c_carried=$((c_carried + 1)) || c_modelled=$((c_modelled + 1))
  elif declared_drop "$label" && [ "$got" = "$EMPTY_SHA" ]; then
    # A DECLARED DROP IS NOT LOSS, AND VERIFYING IT IS NOT THE SAME AS TRUSTING
    # IT. On Baize, cc's template-scaffolding drop turned 39 conserved sections
    # into 39 ALTERED-PROSE findings under a line that calls itself "the number
    # that means loss" -- so an instrument built to catch loss was reporting a
    # deliberate, per-section-documented removal as exactly that.
    #
    # BOTH CONDITIONS ARE LOAD-BEARING. The disposition is a CLAIM by the
    # migrator; emptiness is the OBSERVATION. Honouring the claim alone would
    # let any future drop launder real alteration through a disposition line,
    # which is a check certifying its own subject. A section declared dropped
    # that still has content in canon stays ALTERED, because that is not what
    # was declared.
    echo "DECLARED-DROP $label (removed, and the migrator says why -- verified empty in canon)"
    c_drop=$((c_drop + 1))
  else
    report ALTERED-PROSE "$label (estate $raw, canon $got)"
  fi
}

# EVERY CENSUS PROSE KIND IS DECLARED HERE, COMPARED OR NOT, WITH A REASON.
#
# THIS EXISTS BECAUSE THE ARMS BELOW ENDED IN `[ "$kind" = thread ] || continue`,
# which is a SILENT DROP. On the first run against a real migrated tree it
# swallowed 509 of 2289 census rows -- every `criterion` and every `test` -- and
# the summary line published `conserved 820, normalised 259, without a
# destination 701` with no denominator, so nothing in the output was false and
# nothing in it said that 22% of the census had not been looked at. It took
# subtracting three printed numbers from a fourth printed elsewhere to see it.
#
# **This is the second time in one session, the same defect one layer over.** The
# first cut of this arm compared threads only while counting 974 WP and 503 issue
# sections it never opened; that was fixed by adding two arms. Adding arms does
# not close the class, because the class is that the loop can grow a kind it has
# no arm for and say nothing. A table that must name every kind does close it:
# an unknown kind now REFUSES rather than passing through, so the next kind
# anyone adds to the census cannot be silently uncompared.
#
# `criterion` and `test` are DECLARED-UNCOMPARED rather than quietly missing, and
# the reason is real. The census records the whole authored ROW (table pipes and
# all); canon holds the AC's `text` and the AT's `note` as fields, so there is no
# byte on either side that is the same byte on the other, and comparing them
# needs the census to hash the text CELL as well as the row. That is a change to
# two instruments and a re-census of four members, which is not tonight's work
# under the hold. **Their identity IS checked** -- LOST-ac and LOST-at ran clean,
# so all 281 criteria and 228 tests survive by id. What is unchecked is their
# INTERIOR, which is exactly the axis dc's synthesis says a check must not be
# silent about. So it says it, on every run, in the summary.
prose_kind_disposition() {
  case "$1" in
    thread | wp | issue) echo compared ;;
    criterion) echo "uncompared:census hashes the whole AC row, canon holds .criteria[].text -- no common bytes; identity covered by LOST-ac" ;;
    test) echo "uncompared:census hashes the whole AT row, canon holds .tests[].note -- no common bytes; identity covered by LOST-at" ;;
    *) echo unknown ;;
  esac
}

c_ok=0 c_lost=0 c_norm=0 c_seen=0 c_declared=0 c_modelled=0 c_carried=0
declared_kinds=""
while IFS=$'\t' read -r _ kind id section bytes sha trim; do
  c_seen=$((c_seen + 1))
  disp="$(prose_kind_disposition "$kind")"
  case "$disp" in
    unknown)
      die "census PROSE kind '$kind' is in no arm and in no declaration -- it would have been dropped silently, which is the one outcome this table exists to make impossible; add it to prose_kind_disposition()"
      ;;
    uncompared:*)
      c_declared=$((c_declared + 1))
      case "$declared_kinds" in
        *" $kind:"*) : ;;
        *) declared_kinds="$declared_kinds $kind:${disp#uncompared:}" ;;
      esac
      continue
      ;;
  esac
  # THE PREAMBLE HAS NO DESTINATION IN ANY OF THE THREE ARMS, so it is reported
  # once here rather than falling through each of them. In the WP and issue arms
  # it would otherwise be looked for as a `## (preamble)` heading inside a body,
  # found absent, and reported ALTERED -- a content-loss finding for a region that
  # was never a section. Wrong class, right alarm, and the wrong class is what a
  # reader acts on.
  # THE DESTINATION NOW EXISTS: `preamble` on both `steel_thread` and
  # `work_package` (specced at `83c2d48e`, built by cc at `283dd00a`). Until that
  # landed this arm reported LOST unconditionally, which was correct then and is
  # a WRONG ZERO now -- it would report a working field as total loss.
  #
  # AND I PRICED THE FIX BEFORE FIXING THIS, WHICH IS THE DEFECT WORTH RECORDING.
  # I published `LOST-PROSE 575 -> 555` to cc as the number their build would
  # produce. It could not have: no code path here consults `.preamble`, so a
  # correct build and a build that did nothing produce the identical output. cc
  # hit the spec exactly -- 15 thread-level and 5 work-package, the split I said
  # was the part they could most easily get wrong -- and my instrument reported
  # no movement at all. **A prediction that requires a change to the instrument
  # is not a prediction about the subject**, and had cc trusted my number they
  # would have gone hunting a defect in correct work.
  if [ "$section" = "(preamble)" ]; then
    case "$kind" in
      thread)
        j="$CANON/st/$id/thread.json"
        [ -f "$j" ] || { report LOST-PROSE "thread $id prose above the first heading ($bytes bytes -- no thread.json)"; c_lost=$((c_lost + 1)); continue; }
        jq -j '.preamble // ""' "$j" >"$WORK/f" 2>/dev/null || : >"$WORK/f"
        compare_prose "$id '(preamble)'" "$sha" "$trim" "$WORK/f" modelled
        continue
        ;;
      wp)
        st="${id%%/*}"; seq="${id##*/}"; seq="$(printf '%s' "$seq" | sed 's|^0*||')"
        j="$CANON/st/$st/thread.json"
        [ -f "$j" ] || { report LOST-PROSE "wp $id prose above the first heading ($bytes bytes -- no thread.json)"; c_lost=$((c_lost + 1)); continue; }
        jq -j ".wps[] | select(.seq == $seq) | .preamble // \"\"" "$j" >"$WORK/f" 2>/dev/null || : >"$WORK/f"
        compare_prose "wp $id '(preamble)'" "$sha" "$trim" "$WORK/f" modelled
        continue
        ;;
      *)
        # Issues have no preamble field and are not claimed to. Still a loss, and
        # still named as one rather than folded into the arms above.
        report LOST-PROSE "$kind $id prose above the first heading ($bytes bytes -- no preamble field on this entity)"
        c_lost=$((c_lost + 1))
        continue
        ;;
    esac
  fi
  # WORK PACKAGES. `WorkPackage` carries `objective` plus a `body` holding every
  # other section (D28), so a WP section always has a destination -- the question
  # is whether the bytes survived it. Checked rather than assumed, because "there
  # is a field for it" and "the field holds what left" are different claims.
  if [ "$kind" = wp ]; then
    st="${id%%/*}"; seq="${id##*/}"; seq="$(printf '%s' "$seq" | sed 's|^0*||')"
    j="$CANON/st/$st/thread.json"
    [ -f "$j" ] || continue
    if [ "$section" = Objective ]; then
      jq -j ".wps[] | select(.seq == $seq) | .objective // \"\"" "$j" >"$WORK/f" 2>/dev/null || : >"$WORK/f"
      dest=modelled
    else
      jq -j ".wps[] | select(.seq == $seq) | .body // \"\"" "$j" >"$WORK/body" 2>/dev/null || : >"$WORK/body"
      section_text "$WORK/body" "$section" >"$WORK/f"
      # D28's catch-all. The bytes are safe; nothing knows what they mean.
      dest=carried
    fi
    compare_prose "wp $id '$section'" "$sha" "$trim" "$WORK/f" "$dest"
    continue
  fi

  # ISSUES. THIS ARM ASKED A DISK QUESTION ABOUT A STORE-RESIDENT MODEL, AND IT
  # COULD NOT REACH ZERO NO MATTER WHAT ANYONE BUILT (ic, 2026-08-18, measured on
  # the first post-`attachments` baseline: 332 of 384 LOST-PROSE were this one
  # line). It looked for `issues/<n>.md` because data-model.md declared that home
  # when files were canon. **D01 REVERSED, and my instrument did not follow it.**
  # cc has since put the prose in `issues/<n>.json`'s `body`; disk-optional
  # deliberately does not write `issues/<n>.md`; and under `realisation.md` that
  # file would exist only for REALISED issues, which would make a conservation
  # number depend on the realisation set. **A check that reads disk is measuring
  # the projection, not the truth.**
  #
  # ic verified the bytes were conserved the whole time -- 40 of 40 bodies
  # non-empty, 434,437 bytes, issue 0001 byte-identical to its v2 source -- so
  # the arm was reporting 432 KB of loss over prose sitting safely in the store.
  #
  # An EMPTY destination is still LOSS and is still reported. What changed is
  # WHERE the destination is looked for, not whether one is required.
  if [ "$kind" = issue ]; then
    num="${id##*/}"; num="$(printf '%s' "$num" | sed 's|^0*||')"
    j="$CANON/issues/$num.json"
    if [ ! -f "$j" ]; then
      report LOST-PROSE "issue $id '$section' ($bytes bytes -- no issues/$num.json)"
      c_lost=$((c_lost + 1))
      continue
    fi
    jq -j '.body // ""' "$j" >"$WORK/ibody" 2>/dev/null || : >"$WORK/ibody"
    section_text "$WORK/ibody" "$section" >"$WORK/f"
    if [ ! -s "$WORK/f" ]; then
      report LOST-PROSE "issue $id '$section' ($bytes bytes -- issues/$num.json holds no such section in .body)"
      c_lost=$((c_lost + 1))
      continue
    fi
    # Bytes preserved, structure not modelled, so it is carried for the same
    # reason a WP's `body` is.
    compare_prose "issue $id '$section'" "$sha" "$trim" "$WORK/f" carried
    continue
  fi

  # `thread` is the only kind the table routes here. A kind reaching this line
  # that is not `thread` means the table and the arms have drifted apart, which
  # is the silent drop rebuilt, so it refuses rather than continuing.
  [ "$kind" = thread ] ||
    die "kind '$kind' is declared compared but reached no arm -- prose_kind_disposition() and the arms above have drifted apart"
  st="$id"
  j="$CANON/st/$st/thread.json"
  [ -f "$j" ] || continue
  field=
  case "$section" in
    Objective) field=objective ;;
    Context) field=context ;;
    "Related Steel Threads") field=related ;;
    # `Thread.body` (cc, `bcbd02cd`) is the thread-level twin of D28's WP
    # catch-all, so every remaining section now has a destination and this arm
    # asks the same question the WP arm asks rather than assuming the answer.
    #
    # WHAT WAS HERE BEFORE AND WHY IT WAS WORSE THAN STALE. This branch reported
    # `LOST-PROSE ... no modelled field, and the migrator did not name it` -- and
    # the second clause was a hardcoded SENTENCE, not a reading. It never
    # consulted the dispositions file it was asserting about. When `Thread.body`
    # landed 35 declared thread-level drops, all 35 came through here and 32 were
    # printed as unexplained loss with a claim about cc's output that this tool
    # had not read: 13,698 bytes. My own separation, broken in my own instrument
    # -- the disposition is the migrator's CLAIM and emptiness is my OBSERVATION,
    # and a check that states the claim without observing it certifies itself.
    *) field=body ;;
  esac
  if [ "$field" = related ]; then
    # `related` is a structured array rather than prose, so byte equality is the
    # wrong test: an EMPTY array against a populated section is the failure.
    if [ "$(jq -r '.related | length' "$j")" -eq 0 ]; then
      # THE SAME OMISSION ONE LEVEL DOWN, and it is why the run above still read
      # `112 matched of 115`. An empty `related[]` is the right OBSERVATION for
      # all 55 of this estate's sections, but 3 of them the migrator declared
      # dropped as template scaffolding no author wrote -- so emptiness is the
      # expected end state for those, and unexplained loss for the other 52. The
      # observation alone cannot tell those apart; the claim has to be read.
      if declared_drop "$st 'Related Steel Threads'"; then
        echo "DECLARED-DROP $st 'Related Steel Threads' (removed, and the migrator says why -- verified empty in canon)"
        c_drop=$((c_drop + 1))
      else
        # THE STRUCTURE BEING EMPTY IS NOT THE TEXT BEING GONE, AND THIS TOOL
        # ALREADY HAD A WORD FOR THE DIFFERENCE (ic, 2026-08-18). `related[]` is
        # empty for all 56 threads, and the section's bytes reached the D28
        # catch-all -- verified independently on ST0010, whose `.body` opens with
        # `## Related Steel Threads` followed by both rows, byte-for-byte the
        # rendered view. **What is missing is the MODELLING, not the prose**,
        # which is exactly `CARRIED` -- "bytes safe, meaning unmodelled" -- and
        # this arm was calling it LOST. Two verdicts of mine disagreeing about
        # one set of sections.
        #
        # So ask the catch-all before concluding. An absent section is still
        # LOSS, reported as such rather than handed to `compare_prose`, which
        # would call an empty destination ALTERED -- the number that means the
        # bytes arrived WRONG, which is a different and worse claim.
        jq -j '.body // ""' "$j" >"$WORK/rbody" 2>/dev/null || : >"$WORK/rbody"
        section_text "$WORK/rbody" 'Related Steel Threads' >"$WORK/f"
        if [ ! -s "$WORK/f" ]; then
          report LOST-PROSE "$st 'Related Steel Threads' ($bytes bytes -- empty related[] AND no such section in .body)"
          c_lost=$((c_lost + 1))
        else
          compare_prose "$st 'Related Steel Threads'" "$sha" "$trim" "$WORK/f" carried
        fi
      fi
    else
      c_ok=$((c_ok + 1))
    fi
    continue
  fi
  # The catch-all holds every unmodelled section concatenated, so the named one
  # is cut out of it first -- the same two steps the WP arm takes, through the
  # same `section_text`, because two ways of asking one question is how the two
  # answers start to differ.
  if [ "$field" = body ]; then
    jq -j ".body // \"\"" "$j" >"$WORK/body" 2>/dev/null || : >"$WORK/body"
    section_text "$WORK/body" "$section" >"$WORK/f"
    compare_prose "$st '$section'" "$sha" "$trim" "$WORK/f" carried
    continue
  fi
  jq -j ".\"$field\" // \"\"" "$j" >"$WORK/f" 2>/dev/null || : >"$WORK/f"
  compare_prose "$st '$section'" "$sha" "$trim" "$WORK/f" modelled
done < <(awk -F'\t' '$1 == "PROSE"' "$CENSUS")

# ---------------------------------------------------------------------------
# C2. THE OTHER DIRECTION: prose the CANON holds and the estate never did.
#
# ic, 2026-08-17, measured on real v2 bytes: the migration ACCRETES. A WP's
# `info.md` went 8562 -> 8840 -> 9190 -> 9540 bytes over three runs with no fixed
# point, because Phase A's D28 catch-all cannot tell a GENERATED section from an
# authored one, sweeps the rendered `## Acceptance` and the do-not-edit banner
# into `WorkPackage.body`, and the next render re-emits them plus fresh copies.
# The banner was in `thread.json` three times, as committed canon. Neither run
# blocked.
#
# **EVERY OTHER PREDICATE IN THIS FILE PASSES THAT.** Reachability holds -- every
# file is where the model points. Presence holds. DOUBLED sees one `info.md`, not
# two. And prose conservation passes because it iterates the ESTATE's sections and
# finds every one of them intact: the authored bytes are all still there, with
# generated bytes interleaved. **Prose GAINED is invisible to a check that only
# asks whether what left arrived.**
#
# So the equality has to be two-sided, which is the same discipline already
# applied to FILE (the EXTRA arm) and not, until now, to PROSE. A superset is not
# conservation.
c_added=0
while IFS= read -r j; do
  st="$(jq -r '.id' "$j" 2>/dev/null)"
  [ -n "$st" ] || continue
  while IFS=$'\t' read -r seq heading; do
    [ -n "$heading" ] || continue
    grep -qxF "wp	$st/$seq	$heading" "$WORK/census_sections" && continue
    report ADDED-PROSE "wp $st/$seq '$heading' (in the canon, in no estate section -- a superset is not conservation)"
    c_added=$((c_added + 1))
  done < <(jq -r '.wps[]? | .seq as $s | (.body // "") | split("\n")[] | select(startswith("## ")) | "\($s)\t\(.[3:])"' "$j" 2>/dev/null)
done < <(find "$CANON/st" -name thread.json -maxdepth 2 2>/dev/null | sort)

# THE ARM ABOVE READS `.wps[].body` -- THE MODEL -- AND cc FOUND ACCRETION THE
# MODEL CANNOT HOLD. 40 of 140 migrated work-package VIEWS ship with two
# `## Acceptance` sections: the authored one is carried in `body` (so it has a
# census counterpart and is correctly not ADDED), and the renderer then emits
# its own. The second copy exists only in the rendered file and appears in no
# `thread.json` at all.
#
# So "ADDED 0" was a claim about the MODEL being published as a claim about
# ACCRETION. The subject and the report were different and the line could not
# tell them apart -- the same defect this tool has now shipped three times, and
# the reason the summary below names its subject rather than its adjective.
# The cheap two-sided test the model cannot do is asked of the file: one
# rendered view must not carry the same heading twice.
c_dup=0
while IFS= read -r v; do
  dups="$(grep '^## ' "$v" 2>/dev/null | sort | uniq -d)"
  [ -n "$dups" ] || continue
  while IFS= read -r h; do
    [ -n "$h" ] || continue
    report DOUBLED-SECTION "${v#"$CANON"/} carries '${h#\#\# }' twice (one carried, one generated -- two artefacts, one role)"
    c_dup=$((c_dup + 1))
  done <<EOF
$dups
EOF
done < <(find "$CANON/st" -path '*/WP/*' -name info.md 2>/dev/null | sort)

# ---------------------------------------------------------------------------
# The totals are printed on every run, pass or fail. A check that only speaks
# when it fails cannot be told from a check that never ran.
# ---------------------------------------------------------------------------
echo "conservation: $n_census estate file(s) -- converted $a_conv, relocated $a_reloc, out-of-model $a_oom"
# THE TWO BUCKET POPULATIONS ARE PRINTED SEPARATELY BECAUSE THEIR DISPOSITIONS
# ARE OPPOSITE. Merged, they read as one large expected-noise number and the half
# that is real loss hides inside the half that is fine. STRANDED is the one that
# has to reach zero.
echo "conservation: v2 status buckets -- DOUBLED $c_doubled (superseded originals, content reached canon), STRANDED $c_stranded (THE ONLY COPY, reachable from nothing)"
# ALTERED is printed EXPLICITLY, including when it is zero, because against the
# real migrator the healthy reading is `conserved 0`. `sections()` trims every
# body, so nothing survives byte-identical and everything content-preserving lands
# in NORMALISED -- a summary that only published `conserved` would show a zero on
# a clean migration and read as total loss. The number that means loss is ALTERED.
# `grep -c` prints 0 AND exits 1 when nothing matches, so `|| echo 0` emits a
# SECOND zero and the arithmetic dies on "0\n0". The log is created up front, so
# swallowing the exit is all that is needed.
c_alt="$(grep -c '^ALTERED-PROSE ' "$WORK/log" || true)"
# THE DENOMINATOR IS PRINTED WITH THE NUMERATORS, AND THE TWO ARE RECONCILED.
#
# SCOPE GOES IN A DENOMINATOR, NEVER IN AN ADJECTIVE -- a rule this file's author
# has been applying to other people's work all week while this line published
# four counts against no total. Every one of them was true. Together they covered
# 1780 of 2289 rows and nothing said so.
#
# The equality is the point, not the print: `compared + declared-uncompared` must
# equal the rows READ, or some row took a path that counts it nowhere. An
# equality refuses the documentation fix as well as the code fix, and only one of
# those is ever what you wanted.
c_acct=$((c_ok + c_norm + c_lost + c_alt + c_declared + c_drop))
[ "$c_acct" -eq "$c_seen" ] ||
  die "prose accounting does not reconcile: $c_seen census row(s) read, $c_acct dispositioned (ok $c_ok, normalised $c_norm, lost $c_lost, altered $c_alt, declared-uncompared $c_declared, declared-drop $c_drop) -- the difference went somewhere this tool cannot name"
echo "conservation: prose -- ALTERED $c_alt (the number that means loss), ADDED $c_added (accretion IN THE MODEL -- \`.wps[].body\`, not in rendered views), conserved byte-identical $c_ok, whitespace-normalised $c_norm, without a destination $c_lost"
echo "conservation: views -- DOUBLED-SECTION $c_dup (accretion IN THE RENDERING: one file, one heading, two copies -- invisible to the line above)"
if [ -n "$DISPO" ]; then
  n_dec="$(wc -l <"$WORK/declared" | tr -d ' ')"
  echo "conservation: prose -- DECLARED-DROP $c_drop matched of $n_dec declared (removed on purpose, named per-section AND verified empty in canon -- not loss, not counted as ALTERED above)"
  echo "conservation: views -- DECLARED-DEFERRAL $n_defer declared (generated section stood down; canon unchanged, and any whose canon is EMPTY is reported DEFERRAL-REFUTED above)"
  [ "$c_drop" -eq "$n_dec" ] ||
    echo "conservation: prose -- WARNING: $((n_dec - c_drop)) declared drop(s) matched no census section, so they are reported as ALTERED or not at all -- a declared drop this tool cannot find is a claim it cannot check"
else
  echo "conservation: prose -- DECLARED-DROP not measured -- no --dispositions given, so every removal above is counted as loss whether or not the migrator named it"
fi
echo "conservation: prose -- compared $((c_seen - c_declared)) of $c_seen census section(s); NOT compared $c_declared, declared:${declared_kinds:- none}"
# The conserved population split by what the destination KNOWS. Not "did the
# bytes survive" -- that is the line above -- but "does the model understand
# them". A section in a declared field is modelled; a section in a catch-all is
# carried, safe and opaque. Both are correct outcomes and neither is a finding.
echo "conservation: prose -- of $((c_ok + c_norm)) conserved, MODELLED $c_modelled (reached a declared field), CARRIED $c_carried (reached a catch-all -- bytes safe, meaning unmodelled)"
# LIVENESS. Everything above asks whether the BYTES survived. Nothing above
# asks whether the tool can still OPEN the estate, and on 2026-08-17 that gap
# certified three fleet members as CONVERTED while every one of them refused
# every read verb afterwards. ic named the shape: a conservation green sitting
# on top of a liveness failure, where the two instruments cannot see each other.
#
# TWO THINGS THIS DELIBERATELY DOES NOT TRUST.
#
# Not the EXIT CODE -- a lockout is free to exit 0, and the failure it is
# guarding against is precisely a tool that reports success and does nothing
# (a second `intent upgrade` on a locked-out estate exits 0, claims 311 files
# written, and clears nothing).
#
# Not the SHAPE OF THE OUTPUT either, which is the subtler one and is ic's
# false green: an estate emptied of 55 of its 56 threads renders a table with
# a header and exits 0, so "it printed a table" is health's clothes rather than
# health. The assertion is therefore that the output NAMES threads the census
# knows this estate to have. That is derived from the subject rather than
# keyed to a string, so it cannot be satisfied by an estate the tool has
# stopped being able to see.
liveness_ids="$(awk -F'\t' '$1 == "FILE"' "$CENSUS" |
  grep -oE 'ST[0-9]{4}' | sort -u)"
n_ids="$(printf '%s\n' "$liveness_ids" | grep -c 'ST[0-9]' || true)"

if [ -z "$BINARY" ]; then
  # SCOPE GOES IN A DENOMINATOR, NEVER IN AN ADJECTIVE -- and an unmeasured
  # arm is scope. Silence here would let a reader take the conservation green
  # as a verdict on the estate's health, which is the exact misreading that
  # produced the finding this arm exists for.
  echo "conservation: LIVENESS NOT MEASURED -- no --binary given; every line above is about BYTES, not about whether the tool can open this estate"
elif [ "$n_ids" -eq 0 ]; then
  die "census names no ST ids, so liveness has nothing to assert against -- a check that cannot fail does not pass"
else
  # STDOUT ONLY, AND THE `2>&1` THAT WAS HERE FIRST IS THE REASON THIS COMMENT
  # IS. The refusal message itself NAMES thread ids -- "9 steel threads carry
  # v2 canon this binary cannot read" is followed by them -- so an id-in-output
  # test over merged streams matched the ERROR TEXT and pronounced a fully
  # locked-out estate alive. That is this programme's standing class one more
  # time: the thing measured (does the tool LIST threads) and the thing
  # reported (does this text contain an id) were different, and the output
  # could not tell them apart. Errors go to stderr; the answer is on stdout.
  # `--status all` IS LOAD-BEARING AND ITS ABSENCE COST A FALSE FAILURE ON EVERY
  # MEMBER. Bare `st list` renders "in progress only" by design, so a v3-native
  # estate whose one thread is Triage lists ZERO rows and looks locked out. The
  # census is the whole population; the default filter is a different one; and a
  # silent default denominator is this programme's standing class wearing its
  # least suspicious clothes. Asserted against the same population the census
  # counts, or the ratio below is comparing two different estates.
  live_out="$(cd "$MIGRATED" && "$BINARY" st list --status all 2>/dev/null)"
  live_rc=$?
  named=0
  for id in $liveness_ids; do
    case "$live_out" in *"$id"*) named=$((named + 1)) ;; esac
  done
  # Two tiers, because the two failures look nothing alike and only one of them
  # announces itself. NAMED 0 is the lockout. NAMED but short of the census is
  # the quiet one -- an estate that opens, exits 0 and renders a table listing
  # fewer threads than it holds, which is ic's false green wearing a green exit
  # code. Neither is caught by the exit code and neither by the table's shape.
  if [ "$named" -eq 0 ]; then
    report LIVENESS "st list named 0 of $n_ids census thread(s) (exit $live_rc) -- the bytes above survived and the tool cannot read them"
    echo "conservation: LIVENESS FAILED -- migrated, and then unreadable"
  elif [ "$named" -lt "$n_ids" ]; then
    report LIVENESS "st list named only $named of $n_ids census thread(s) (exit $live_rc) -- it opens and under-reports, which no exit code and no table shape will tell you"
    echo "conservation: LIVENESS PARTIAL -- readable, and short by $((n_ids - named)) thread(s)"
  else
    echo "conservation: LIVENESS ok -- st list named all $n_ids census thread(s) (exit $live_rc)"
  fi
fi

echo "conservation: $findings finding(s)"
[ "$findings" -eq 0 ]
