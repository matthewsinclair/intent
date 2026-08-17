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

set -uo pipefail

die() {
  echo "conservation: $*" >&2
  exit 2
}

CENSUS="" MIGRATED="" OOM=""
while [ $# -gt 0 ]; do
  case "$1" in
    --out-of-model) OOM="${2:-}"; shift 2 || die "--out-of-model needs a file" ;;
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
  die "usage: conservation_check.sh <census.tsv> <migrated-root> [--out-of-model <file>]"
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

SUBJECT="$(awk -F'\t' '$1 == "CORPUS" { print $2 " @ " substr($3, 1, 12); exit }' "$CENSUS")"
[ -n "$SUBJECT" ] ||
  die "census carries no CORPUS record -- it was produced before the census named its own subject, and a verdict that cannot say which estate it describes is not a verdict; re-run estate_census.sh"
case "$SUBJECT" in
  unpinned*) echo "conservation: SUBJECT $SUBJECT -- an UNPINNED estate; this verdict describes a directory, not a revision" ;;
  *) echo "conservation: subject $SUBJECT" ;;
esac

# The canon root. `st/<ID>/thread.json` and `issues/<n>.json` are data-model.md's
# canonical paths; a tree with neither has not been migrated, and saying so is
# the whole difference between a refusal and a green.
CANON="$MIGRATED"
[ -d "$CANON/st" ] ||
  die "$MIGRATED holds no st/ canon -- this is an UNMIGRATED tree, and a check that cannot see its subject does not pass it"

WORK="$(mktemp -d)" || die "cannot create a scratch directory"
trap 'rm -rf "$WORK"' EXIT

: >"$WORK/oom"
[ -n "$OOM" ] && { [ -f "$OOM" ] || die "no such out-of-model file: $OOM"; sort -u "$OOM" >"$WORK/oom"; }

findings=0
: >"$WORK/log"
report() {
  echo "$1 $2" | tee -a "$WORK/log"
  findings=$((findings + 1))
}

# ---------------------------------------------------------------------------
# A. Artefact conservation -- reachability, not presence.
# ---------------------------------------------------------------------------
a_conv=0 a_reloc=0 a_oom=0
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
    # A v2 artefact still sitting under its status bucket. Two shapes, and they
    # are reported separately because their remedies differ: a file the model
    # regenerates is DOUBLED (two artefacts claiming one role -- the 0011 class,
    # manufactured by the migration rather than found in the estate), while a
    # file nothing regenerates is simply UNREACHABLE.
    case "$base" in
      info.md|acceptance.md)
        if [ -f "$CANON/st/$st/$base" ]; then
          report DOUBLED "$path (also generated at st/$st/$base -- two artefacts, one role)"
        else
          report UNREACHABLE "$path (owner $id has canon at st/$st/, this is not under it)"
        fi
        ;;
      *)
        if [ -f "$CANON/st/$st/$base" ]; then
          a_reloc=$((a_reloc + 1))
        else
          report UNREACHABLE "$path (owner $id has canon at st/$st/, this is not under it)"
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
compare_prose() {
  local label="$1" raw="$2" trim="$3" file="$4" got
  got="$(shasum -a 256 <"$file" | cut -d' ' -f1)"
  if [ "$got" = "$raw" ]; then
    c_ok=$((c_ok + 1))
    return
  fi
  trim_file "$file" "$WORK/trim"
  got="$(shasum -a 256 <"$WORK/trim" | cut -d' ' -f1)"
  if [ "$got" = "$trim" ]; then
    echo "NORMALISED-PROSE $label (content identical; leading/trailing whitespace differs)"
    c_norm=$((c_norm + 1))
  else
    report ALTERED-PROSE "$label (estate $raw, canon $got)"
  fi
}

c_ok=0 c_lost=0 c_norm=0
while IFS=$'\t' read -r _ kind id section bytes sha trim; do
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
    else
      jq -j ".wps[] | select(.seq == $seq) | .body // \"\"" "$j" >"$WORK/body" 2>/dev/null || : >"$WORK/body"
      section_text "$WORK/body" "$section" >"$WORK/f"
    fi
    compare_prose "wp $id '$section'" "$sha" "$trim" "$WORK/f"
    continue
  fi

  # ISSUES. data-model.md gives them `issues/<n>.json` plus an authored body at
  # `issues/<n>.md`, so the destination is declared and the only question is
  # whether anything arrived. A missing body file is a LOSS and not an absence:
  # the estate held the prose and the canon declares a home for it.
  if [ "$kind" = issue ]; then
    num="${id##*/}"; num="$(printf '%s' "$num" | sed 's|^0*||')"
    body="$CANON/issues/$num.md"
    if [ ! -f "$body" ]; then
      report LOST-PROSE "issue $id '$section' ($bytes bytes -- no issues/$num.md)"
      c_lost=$((c_lost + 1))
      continue
    fi
    section_text "$body" "$section" >"$WORK/f"
    compare_prose "issue $id '$section'" "$sha" "$trim" "$WORK/f"
    continue
  fi

  [ "$kind" = thread ] || continue
  st="$id"
  j="$CANON/st/$st/thread.json"
  [ -f "$j" ] || continue
  field=
  case "$section" in
    Objective) field=objective ;;
    Context) field=context ;;
    "Related Steel Threads") field=related ;;
    *)
      report LOST-PROSE "$st '$section' ($bytes bytes -- no modelled field, and the migrator did not name it)"
      c_lost=$((c_lost + 1))
      continue
      ;;
  esac
  if [ "$field" = related ]; then
    # `related` is a structured array rather than prose, so byte equality is the
    # wrong test: an EMPTY array against a populated section is the failure.
    if [ "$(jq -r '.related | length' "$j")" -eq 0 ]; then
      report LOST-PROSE "$st 'Related Steel Threads' ($bytes bytes -- canon carries an empty related[])"
      c_lost=$((c_lost + 1))
    else
      c_ok=$((c_ok + 1))
    fi
    continue
  fi
  jq -j ".\"$field\" // \"\"" "$j" >"$WORK/f" 2>/dev/null || : >"$WORK/f"
  compare_prose "$st '$section'" "$sha" "$trim" "$WORK/f"
done < <(awk -F'\t' '$1 == "PROSE"' "$CENSUS")

# ---------------------------------------------------------------------------
# The totals are printed on every run, pass or fail. A check that only speaks
# when it fails cannot be told from a check that never ran.
# ---------------------------------------------------------------------------
echo "conservation: $n_census estate file(s) -- converted $a_conv, relocated $a_reloc, out-of-model $a_oom"
# ALTERED is printed EXPLICITLY, including when it is zero, because against the
# real migrator the healthy reading is `conserved 0`. `sections()` trims every
# body, so nothing survives byte-identical and everything content-preserving lands
# in NORMALISED -- a summary that only published `conserved` would show a zero on
# a clean migration and read as total loss. The number that means loss is ALTERED.
# `grep -c` prints 0 AND exits 1 when nothing matches, so `|| echo 0` emits a
# SECOND zero and the arithmetic dies on "0\n0". The log is created up front, so
# swallowing the exit is all that is needed.
c_alt="$(grep -c '^ALTERED-PROSE ' "$WORK/log" || true)"
echo "conservation: prose -- ALTERED $c_alt (the number that means loss), conserved byte-identical $c_ok, whitespace-normalised $c_norm, without a destination $c_lost"
echo "conservation: $findings finding(s)"
[ "$findings" -eq 0 ]
