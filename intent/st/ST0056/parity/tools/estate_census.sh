#!/bin/bash
# estate_census.sh -- what does a v2 estate CONTAIN? The conservation denominator.
#
# AC-10.5 asks for artefact conservation, semantic completeness and prose
# conservation. All three are one equality with two sides: what the estate held,
# and what the migrator produced. This is the first side, and it is computed
# without reading the second.
#
# WHY THIS IS NOT RUST AND NOT `legacy.rs`. The equality is only worth computing
# if the two enumerations can disagree about something neither author
# anticipated. Sharing a parser makes it vacuous in exactly the direction nobody
# would notice: `legacy.rs` never reads `intent/issues/` at all, so the issue
# estate would migrate to nothing while every count reconciled perfectly against
# zero (ic, 2026-08-17). A census built on the same parser would have reported
# zero against zero and passed. Different language, different reading of the v2
# format, different author -- so the independence is structural rather than a
# discipline someone has to keep.
#
# WHAT IT ENUMERATES, AND WHY MORE THAN THE MIGRATOR READS. `legacy.rs` builds a
# `Thread` from `## Objective` and `## Context` and hardcodes `related:
# Vec::new()`. The estate carries far more than that -- `## Related Steel
# Threads` in 55 of 56 threads, `## Context for LLM` in 41, plus `## Scope`,
# `## Deliverables`, `## Success Criteria` -- and every one of those is authored
# prose with no destination in the v3 model. A census that enumerated only the
# sections the migrator reads would inherit the migrator's blind spot and
# certify its own denominator. So it enumerates every `## ` section it finds,
# and the conservation check asks the migrator to account for each: converted,
# named out-of-model, or reported as residue. There is no fourth disposition,
# and "not enumerated" is not one of the three.
#
# THE FILENAME TRAP THIS ALMOST SHIPPED WITH. `intent/st/ST0056/parity/cmd-info.md`
# is a generated surface artefact whose name ENDS in `info.md`. A predicate of
# `grep 'info.md$'` counts it as a steel thread and reports 57 where the estate
# holds 56 -- a true-looking count that has stopped discriminating between a
# thread and a file that merely ends the same way. The predicate here is an
# exact basename plus a parent directory matching `ST[0-9]{4}`, and the WP form
# is separated by its own `WP/<NN>/` parent rather than by exclusion.
#
# OUTPUT is TSV, one record per line, three record types, so a single pass feeds
# all three checks:
#
#   FILE   <path> <owner-kind> <owner-id> <v2-bucket>   every file, and whose it is
#   ENTITY <kind> <id> <path>                           identity, not just count
#   PROSE  <kind> <id> <section> <bytes> <raw-sha> <trimmed-sha>
#   COUNT  <kind> <n>                                   the summary, last
#
# WHY THE FILE RECORD CARRIES AN OWNER AND A BUCKET, WHICH IS THE WHOLE
# DIFFERENCE BETWEEN THIS CHECK AND A VACUOUS ONE (ic, 2026-08-17, measured).
# v2 RELOCATES a thread's directory on a status transition, so 55 of 56 threads
# live at `intent/st/<BUCKET>/<ID>/` while v3's canonical path is `st/<ID>/`.
# A migration writes fresh canon at the flat path and leaves every bucketed file
# exactly where it was -- nothing is deleted, nothing is corrupted, and a
# conservation check that asks "is every file the estate contained still
# present" answers 100%. All of them are there, byte-identical.
#
# The loss is not of bytes, it is of REACHABILITY: the model points at
# `st/ST0001/` and the authored prose is at `st/COMPLETED/ST0001/`. Two
# consequences a bare path listing cannot see. The regenerated files DOUBLE --
# two `info.md` per thread, one generated from the model and one v2 artefact
# that nothing regenerates and everything still reads. And the authored prose
# the model does not hold (`design.md`, `impl.md`, `tasks.md`, and the one-offs)
# is markdown sitting in the repository at a path the model does not know about.
#
# So the FILE record binds every file to its owning entity and records the
# bucket it was found in. That turns prose conservation into an equality between
# the prose the estate CONTAINED and the prose REACHABLE FROM THE MIGRATED
# MODEL, which is the claim AC-10.5 is actually making, rather than an equality
# between two file listings, which is the claim that passes on this.
#
# ENTITY carries an id because a count is the weaker claim: two threads that
# swap ids conserve the count perfectly. PROSE carries a sha rather than a
# length because a reflow can preserve a length, and "the migrator does not
# improve prose" is a byte claim.

set -uo pipefail

die() {
  echo "estate-census: $*" >&2
  exit 2
}

ROOT="${1:-}"
[ -n "$ROOT" ] || die "usage: estate_census.sh <estate-dir>   (the directory HOLDING intent/, eg a capture from estate_corpus.sh)"
[ -d "$ROOT/intent" ] || die "$ROOT holds no intent/ -- this takes the directory that CONTAINS the estate, not the estate itself"

cd "$ROOT" || die "cannot enter $ROOT"

SEC="$(mktemp -d)" || die "cannot create a scratch directory"
trap 'rm -rf "$SEC"' EXIT
RECORDS="$SEC/records.tsv"
: >"$RECORDS"

# CORPUS, FIRST, because a census that cannot name its own subject is an
# anonymous file of numbers (dc's axis, 2026-08-17, arriving in my own output
# while I was checking whether their `INTENT_VERSION` finding reached me).
#
# Four census files for four fleet members were indistinguishable by inspection,
# and `conservation_check.sh` consumed one blind -- hand it the canary's census
# and a migrated Lamplight tree and it compares two unrelated estates and reports
# a number. The identity is READ FROM THE CORPUS's own `CAPTURE`, which
# `estate_corpus.sh verify` checks against git's record of the tree, so it is not
# a claim this tool invents.
#
# **FIRST rather than last, and COUNT stays last, and the asymmetry is the
# point**: identity has to survive a truncated file, and completeness has to be
# absent from one. A census cut off halfway still says which estate it describes
# and visibly lacks its totals.
#
# An estate with no `CAPTURE` is reported `unpinned` with its path rather than
# left blank. A census of a live worktree is legitimate and useful; a census of a
# live worktree that reads like a census of a pinned corpus is not.
#
# Read as `./CAPTURE` and reported as `$PWD`: the `cd "$ROOT"` above already
# happened, so a RELATIVE `$ROOT` no longer resolves from here. That is the same
# defect class in one line -- a path whose referent moved while the identifier
# stayed the same.
if [ -f CAPTURE ]; then
  printf 'CORPUS\t%s\t%s\n' \
    "$(awk '$1 == "member:" { print $2 }' CAPTURE)" \
    "$(awk '$1 == "revision:" { print $2 }' CAPTURE)"
else
  printf 'CORPUS\tunpinned\t%s\n' "$PWD"
fi

# slug <text> -- a scratch-filename-safe form of an entity id. Only ever used to
# name a temporary file; nothing downstream reads it back.
slug() {
  printf '%s' "$1" | tr '/ ' '__'
}

# dump_sections <file> <kind> <id>
#
# One scratch file per `## ` section, holding the body VERBATIM, and one PROSE
# record naming it. The scratch name is derived from (kind, id, index), which is
# unique by construction -- no shared counter, so no ordering assumption between
# the writer and the namer.
dump_sections() {
  local file="$1" kind="$2" id="$3"
  awk -v pfx="$SEC/${2}__$(slug "$3")" '
    function flush_section() {
      if (name == "") return
      path = pfx "__" n ".sec"
      printf "%s", body > path
      close(path)
      printf "PROSE\t%s\t%s\t%s\t%d\t%s\n", kind, id, name, length(body), path
      body = ""
    }
    /^## / { flush_section(); n++; name = substr($0, 4); next }
    { if (name != "") body = body $0 "\n" }
    END { flush_section() }
  ' kind="$kind" id="$id" "$file" >>"$RECORDS"
}

# ---------------------------------------------------------------------------
# FILE: every file in the estate, from the filesystem, with no filter at all.
#
# Unfiltered on purpose. `intent/whiteboard/` and ST0056's own generated parity
# artefacts (.tap, .tsv, .sh) are not v2 canon and convert to nothing -- which
# is exactly why they belong in the denominator. A migrator that leaves them
# unmentioned has silently dropped them; a migrator that names them out-of-model
# has done the right thing. Neither is checkable if the census quietly agrees
# they do not count.
# ---------------------------------------------------------------------------
#
# The owner classifier is path-shaped because the v2 layout is: a thread's
# status is expressed as a DIRECTORY, which is exactly the fact that makes
# relocation necessary and makes it invisible to a listing.
n_file=0 n_bucketed=0
while IFS= read -r f; do
  printf 'FILE\t%s\n' "${f#./}" >>"$RECORDS"
  n_file=$((n_file + 1))
done < <(find intent -type f | sort)

awk -F'\t' -v OFS='\t' '
  $1 != "FILE" { print; next }
  {
    n = split($2, p, "/")
    kind = "-"; id = "-"; bucket = "-"
    if (p[1] == "intent" && p[2] == "st") {
      # `intent/st/<BUCKET>/<STID>/...` or the flat `intent/st/<STID>/...`
      if (p[3] ~ /^ST[0-9][0-9][0-9][0-9]$/) { st = p[3]; i = 4 }
      else if (p[4] ~ /^ST[0-9][0-9][0-9][0-9]$/) { st = p[4]; bucket = p[3]; i = 5 }
      else { print; next }
      if (p[i] == "WP" && p[i+1] ~ /^[0-9]+$/) { kind = "wp"; id = st "/" p[i+1] }
      else { kind = "thread"; id = st }
    } else if (p[1] == "intent" && p[2] == "issues" && p[4] ~ /^[0-9]+$/) {
      # The id component must be NUMERIC. `intent/issues/CLOSED/.gitkeep` sits
      # at the arm level and has no issue at all; a length test admits it and
      # invents `CLOSED/.gitkeep` as an issue id -- an owner record naming a
      # thing that does not exist, which is worse than no owner record.
      kind = "issue"; id = p[3] "/" p[4]; bucket = p[3]
    }
    print $1, $2, kind, id, bucket
  }
' "$RECORDS" >"$RECORDS.owned" && mv "$RECORDS.owned" "$RECORDS" ||
  die "classifying file ownership failed"

# The number that makes ic's finding checkable rather than relayed: files that
# belong to an entity AND sit under a v2 status bucket. Every one of them is at
# a path the migrated model does not point at, and every one of them survives a
# file-listing conservation check untouched.
n_bucketed="$(awk -F'\t' '$1 == "FILE" && $3 != "-" && $5 != "-" && $5 != "OPEN" && $5 != "CLOSED"' "$RECORDS" | wc -l | tr -d ' ')"

n_thread=0 n_wp=0 n_ac=0 n_at=0 n_issue=0 n_open=0 n_closed=0

# Threads: exact basename `info.md`, parent directory an ST id. The v2 layout
# nests closed threads under a status bucket, so depth is not a predicate.
#
# LIVE and CLOSED are counted separately for the same reason OPEN and CLOSED
# issues are, and it is the sharpest thing this census says about its own
# corpus. The migrator routes the SAME unreadable row to BLOCK in a live thread
# and to CARRY in a closed one, so the BLOCK arm's only possible input is live
# threads. **The canary holds 52 completed + 2 cancelled against 1 wip + 1 not
# started.** Whatever a run of the migrator reports about residue, it was asked
# about two threads out of fifty-six -- and a corpus that cannot reach an arm
# and a corpus that reached it and found nothing produce the identical zero.
# Publishing the split is the difference between those two readings.
n_live=0 n_closed_thread=0
while IFS= read -r f; do
  id="$(basename "$(dirname "$f")")"
  printf 'ENTITY\tthread\t%s\t%s\n' "$id" "$f" >>"$RECORDS"
  n_thread=$((n_thread + 1))
  case "$(awk -F': *' '/^status:/ { print tolower($2); exit }' "$f")" in
    completed|complete|done|cancelled|canceled) n_closed_thread=$((n_closed_thread + 1)) ;;
    *) n_live=$((n_live + 1)) ;;
  esac
  dump_sections "$f" thread "$id"
done < <(find intent/st -type f -name info.md | awk -F/ '$(NF-1) ~ /^ST[0-9][0-9][0-9][0-9]$/' | sort)

# Work packages: `.../WP/<NN>/info.md`. The id is scoped by its thread, because
# `01` alone names 56 different things.
while IFS= read -r f; do
  nn="$(basename "$(dirname "$f")")"
  st="$(basename "$(dirname "$(dirname "$(dirname "$f")")")")"
  printf 'ENTITY\twp\t%s/%s\t%s\n' "$st" "$nn" "$f" >>"$RECORDS"
  n_wp=$((n_wp + 1))
  dump_sections "$f" wp "$st/$nn"
done < <(find intent/st -type f -name info.md | awk -F/ '$(NF-2) == "WP"' | sort)

# Criteria and tests: the v2 AC/AT row grammar, one row per line in an
# `acceptance.md`. The whole row is hashed as a PROSE record, not just the id:
# an AC whose id survives and whose criterion was rewritten has not been
# conserved, and a check on ids alone cannot see that.
while IFS= read -r f; do
  st="$(basename "$(dirname "$f")")"
  while IFS= read -r row; do
    kind=
    case "$row" in
      "- AC-"*) kind=criterion ;;
      "- AT-"*) kind=test ;;
      *) continue ;;
    esac
    rid="${row#- }"
    rid="${rid%% *}"
    printf 'ENTITY\t%s\t%s/%s\t%s\n' "$kind" "$st" "$rid" "$f" >>"$RECORDS"
    printf '%s' "$row" >"$SEC/${kind}__$(slug "$st/$rid").sec"
    printf 'PROSE\t%s\t%s/%s\t%s\t%d\t%s\n' \
      "$kind" "$st" "$rid" row "${#row}" "$SEC/${kind}__$(slug "$st/$rid").sec" >>"$RECORDS"
    if [ "$kind" = criterion ]; then n_ac=$((n_ac + 1)); else n_at=$((n_at + 1)); fi
  done <"$f"
done < <(find intent/st -type f -name acceptance.md | sort)

# Issues. THE class `legacy.rs` does not read at all, which is why it is
# enumerated in its own right rather than folded into a file count: a migrator
# that converts none of them still conserves every FILE if the files are merely
# listed. The ARM matters as much as the total -- the migrator routes issue
# findings to BLOCK in live work and CARRY in closed -- so OPEN and CLOSED are
# counted separately and a corpus with an empty arm is visible as such.
while IFS= read -r f; do
  id="$(basename "$(dirname "$f")")"
  arm="$(basename "$(dirname "$(dirname "$f")")")"
  printf 'ENTITY\tissue\t%s/%s\t%s\n' "$arm" "$id" "$f" >>"$RECORDS"
  n_issue=$((n_issue + 1))
  case "$arm" in
    OPEN) n_open=$((n_open + 1)) ;;
    CLOSED) n_closed=$((n_closed + 1)) ;;
    *) die "issue $f sits under $arm, which is neither OPEN nor CLOSED -- an unclassed arm is not silently counted" ;;
  esac
  dump_sections "$f" issue "$arm/$id"
done < <(find intent/issues -type f -name '*.md' | sort)

# ---------------------------------------------------------------------------
# Hash every section in ONE batch and substitute the shas back in. Per-section
# hashing would be one subprocess per section; this is one for the estate.
# ---------------------------------------------------------------------------
n_sec="$(awk -F'\t' '$1 == "PROSE"' "$RECORDS" | wc -l | tr -d ' ')"
if [ "$n_sec" -gt 0 ]; then
  # A TRIMMED TWIN PER SECTION, AND THE REASON IS A MEASUREMENT RATHER THAN A
  # PREFERENCE. `legacy.rs`'s `sections()` calls `buffer.trim()` on every section
  # body, so the canon can NEVER hold the estate's raw section bytes: those
  # include the blank line after the heading and the blank line before the next
  # one, which belong to the markdown LAYOUT rather than to the prose. A raw-byte
  # equality would report ALTERED for every section in every estate and would be
  # useless -- 403 of 403 on Baize before this existed.
  #
  # **The census does NOT adopt the migrator's trim as truth**, which is the
  # whole point: it publishes BOTH hashes and lets the check say which kind of
  # difference it found. Content changed and whitespace normalised are two
  # questions, and answering them with one number is the shape this programme
  # keeps paying for.
  while IFS= read -r s; do
    awk 'BEGIN { RS = "\0" } { gsub(/^[ \t\r\n]+|[ \t\r\n]+$/, ""); printf "%s", $0 }' "$s" >"$s.trim"
  done < <(find "$SEC" -name '*.sec' | sort)
  awk -F'\t' '$1 == "PROSE" { print $6; print $6 ".trim" }' "$RECORDS" | sort -u >"$SEC/paths"
  # `shasum` prints `<sha>  <path>`, so the join key is the path and the batch
  # boundaries xargs chooses cannot reorder anything that matters.
  xargs shasum -a 256 <"$SEC/paths" >"$SEC/hashes" 2>/dev/null ||
    die "hashing $n_sec section bodies failed"
  hashed="$(wc -l <"$SEC/hashes" | tr -d ' ')"
  [ "$hashed" -eq "$(wc -l <"$SEC/paths" | tr -d ' ')" ] ||
    die "hashed $hashed of $(wc -l <"$SEC/paths" | tr -d ' ') section bodies -- a partial hash set would publish some sections unpinned"
fi

awk -F'\t' -v OFS='\t' '
  NR == FNR {
    # `shasum` separates with two spaces; the path is everything after them and
    # may itself contain spaces, so the split is on the FIRST occurrence only.
    i = index($0, "  ")
    sha[substr($0, i + 2)] = substr($0, 1, i - 1)
    next
  }
  # Field 6 becomes the RAW sha and a seventh is appended: the TRIMMED sha. Two
  # hashes because the migrator trims and the estate does not, so "the bytes
  # differ" and "the content differs" are different findings with different
  # remedies.
  $1 == "PROSE" { raw = $6; $6 = sha[raw]; $7 = sha[raw ".trim"]; print; next }
  { print }
' "$SEC/hashes" "$RECORDS" 2>/dev/null || die "substituting section hashes failed"

# COUNT last, so a truncated run cannot be mistaken for a complete one.
printf 'COUNT\tfile\t%d\n' "$n_file"
printf 'COUNT\tthread\t%d\n' "$n_thread"
printf 'COUNT\twp\t%d\n' "$n_wp"
printf 'COUNT\tcriterion\t%d\n' "$n_ac"
printf 'COUNT\ttest\t%d\n' "$n_at"
printf 'COUNT\tissue\t%d\n' "$n_issue"
printf 'COUNT\tissue_open\t%d\n' "$n_open"
printf 'COUNT\tissue_closed\t%d\n' "$n_closed"
printf 'COUNT\tsection\t%d\n' "$n_sec"
printf 'COUNT\tbucketed_file\t%d\n' "$n_bucketed"
printf 'COUNT\tthread_live\t%d\n' "$n_live"
printf 'COUNT\tthread_closed\t%d\n' "$n_closed_thread"
