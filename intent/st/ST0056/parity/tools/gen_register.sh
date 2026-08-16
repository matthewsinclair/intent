#!/bin/bash
#
# INPUTS ARE DECLARED, NOT INFERRED, and generator_inputs_check.sh reads these
# two lines. Repo-relative paths, each of which must be tracked by git: a
# committed artefact whose only input lives in a scratchpad is re-derivable
# today and not tomorrow, and nothing else records which of those it is.
# NOTE: the paths below are the COMMITTED inputs. Where this generator still
# reads a `$SP` scratch copy of one, that is a gap between what it needs and
# what it reaches for -- not a gap in the declaration.
# inputs: intent/st/ST0056/parity/tools/burn-baseline.tsv intent/st/ST0056/parity/tools/fixture_probe.sh intent/st/ST0056/parity/tools/lib_classify.sh intent/st/ST0056/parity/tools/lib_corpus.sh intent/st/ST0056/parity/tools/lib_mdfmt.sh
# inputs-exempt: WT -- a detached git worktree at a committed revision. Re-derivable by `git worktree add <dir> <sha>` and not a file in this repository, so it can never be tracked. The rule is about inputs that are FILES.
# gen_register.sh -- first-pass keep/retire/deviate register from burn data.
#
# Primary axis is the BURN RATIO from burn.sh: how many of a file's tests change
# result when INTENT_BIN is redirected to /usr/bin/false. That is empirical where
# assertion-parsing is inferential -- treeindex_commands.bats looks like 53 CLI
# tests and is in fact 53 tests that exec bin/intent_treeindex directly, bypassing
# the dispatcher; no amount of reading its assertions reveals that, and one
# redirected binary does.
#
# Files that never reach the CLI are then sub-classified mechanically by HOW they
# miss it. Nothing is guessed: a file matching no rule is emitted as UNCLASSIFIED
# for vc to adjudicate, per the work order.

set -uo pipefail

# WT MUST BE THE TREE THE BURN WAS MEASURED IN, not merely a checkout that has
# the same files. The stamp is read from `git -C "$WT" rev-parse HEAD`, so
# passing the main working tree while feeding it a baseline measured elsewhere
# produces a register whose data is from one revision and whose stamp names
# another -- and the stamp is the ONLY thing telling a reader which.
#
# Done on 2026-08-15: regenerated against the main tree with a baseline measured
# at c60cdbd, and the register published "Measured at 892b88a". Data was
# byte-identical, so nothing looked wrong; `pertest.md` (correctly generated in a
# c60cdbd worktree) said c60cdbd, and the two artefacts silently disagreed about
# their own provenance -- the exact split the re-sweep had just been run to
# avoid. Caught by reading the two stamps side by side, not by any check.
#
# There is no mechanism here to enforce it, and that is stated rather than
# hidden: `burn.tsv` carries no revision, so this script cannot know where its
# input came from. Until it does, the discipline is the caller's:
#   git worktree add --detach <dir> <the measured rev>   then   WT=<dir>
# `SP` is now OPTIONAL -- an override for a genuine re-measure, not a
# precondition. `WT` stays required and that is not an oversight: this script
# reads SOURCE at the measured revision (fixture_probe.sh under `ROOT="$WT"`)
# and stamps `REV` from it, so there is no committed file that could stand in
# for it. Defaulting WT to the current checkout would stamp today's revision
# onto a register derived from an older burn, which is precisely the rumour with
# a decimal point the refusal below exists to prevent.
SP="${SP:-}"; WT="${WT:?set WT -- the worktree the BURN was measured in, not just any checkout}"
# THE INPUT IS COMMITTED, AT `tools/burn-baseline.tsv`, AND IS BYTE-IDENTICAL TO
# THE `burn.tsv` THAT PRODUCED THE COMMITTED REGISTER. Verified 2026-08-15 by
# diffing them and by regenerating: every data row came back identical.
#
# Stated here because the opposite was believed and written down. This generator
# was recorded as un-re-runnable on the grounds that "the burn inputs are gone",
# and that belief survived long enough to become the stated reason for not
# fixing two defects in it. The input was in the repository the whole time; only
# the FILENAME differed from the one this line expects. "It does not exist" is a
# claim about the filesystem, and it was never checked.
#
# To re-run:
#   git worktree add --detach <dir> <the measured rev>   # register.md names it
#   WT=<dir> OUT=<sp>/register.md bash tools/gen_register.sh
#
# THE `cp` IS GONE FROM THAT RECIPE, AND ITS ABSENCE IS THE POINT (dc, 2026-08-15).
# This file spent eighteen lines above stating that its input is committed at
# `tools/burn-baseline.tsv` and byte-identical to the `burn.tsv` that produced
# the committed register -- and then read `$SP/burn.tsv` anyway, with the step
# that connects the two living in a comment as a manual `cp`. **A generator that
# DOCUMENTS where its input lives and does not READ it there is re-derivable
# only by whoever reads the comment.** dc's guard checks that git holds the
# declared input; it cannot see that the code reaches a scratch copy instead.
# So the default is now the committed twin and `$SP` is an override for the
# genuine re-measure case, rather than the only path.
# Resolve the script's own directory to an ABSOLUTE path BEFORE the cd below --
# and now before BURN too, since BURN defaults relative to it.
# `dirname "${BASH_SOURCE[0]}"` is relative to the invocation, so reading it
# after `cd "$WT"` resolves against the worktree and the source fails.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BURN="${BURN:-${SP:+$SP/burn.tsv}}"
BURN="${BURN:-$HERE/burn-baseline.tsv}"

cd "$WT" || { echo "gen_register: WT is not a directory: $WT" >&2; exit 2; }

# The no-burn rules moved to lib_classify.sh when gen_pertest.sh needed the
# SAME judgement at test granularity. Two copies of a rule set that must agree
# is exactly the drift lib_corpus.sh was written to catch, and it was found in
# this directory once already today.
. "$HERE/lib_classify.sh" || { echo "gen_register: cannot source $HERE/lib_classify.sh -- refusing to classify without the shared rules" >&2; exit 2; }
# Prove the needles still recognise the spellings they claim to cover BEFORE
# classifying 98 files with them. A rule that has quietly stopped matching a
# form produces a register that is plausible, stable and wrong -- which is
# exactly what happened to the `retire` needle, undetected until an unrelated
# question sent someone to read the call sites.
classify_calibrate || { echo "gen_register: classification rules failed calibration -- refusing to classify the estate with a needle that has stopped matching a form it covers" >&2; exit 2; }

# THE SECOND PREDICATE. Burn says whether a file reaches the v2 CLI; it is a
# v2-side measurement and structurally cannot say whether the file's own SETUP
# survives v3's file layout. Both runs are v2. So a file can burn 12/12, earn
# `keep`, and still fail every test under v3 before an assertion executes.
#
# cc measured that gap from the v3 side (2026-08-14 23:47Z): 8 of the 31 `keep`
# files cannot construct their fixtures at all. This column is the v2-side half
# of it -- computed statically here so the register carries both predicates on
# one row, rather than a consumer having to join two artefacts and get the join
# right. It runs no tests and adds no sweep cost.
#
# It REFUSES rather than degrading: an uncalibrated needle reporting `none` for
# every file is indistinguishable from a clean estate, and this register would
# then publish "no v3 exposure" as a finding. Same discipline as the corpus
# check -- absence is only evidence when the instrument is known to be alive.
EXPOSURE_TSV="$(mktemp "${TMPDIR:-/tmp}/gen_register_exposure.XXXXXX")"
ROOT="$WT" bash "$HERE/fixture_probe.sh" > "$EXPOSURE_TSV" || {
  echo "gen_register: fixture_probe.sh refused -- refusing to emit a register whose v3-exposure column would be silently empty" >&2
  rm -f "$EXPOSURE_TSV"; exit 2
}

# ASSERT THE SCHEMA RATHER THAN COUNTING COLUMNS AND HOPING. The lookup below
# reads a positional field, and a positional read is exactly what survives a
# schema change without complaining -- which it just did: adding fixture_probe's
# `region` column shifted `exposure` from field 4 to field 5, and the register
# quietly published the region COUNT as its exposure value. Every row still
# looked like a row. One line of header check turns that from a silent
# mis-report into a refusal.
EXPECTED_HDR=$'file\tstatus_dir\tgen_view\tregion\texposure'
ACTUAL_HDR="$(head -1 "$EXPOSURE_TSV")"
[ "$ACTUAL_HDR" = "$EXPECTED_HDR" ] || {
  echo "gen_register: fixture_probe.sh emitted an unexpected header -- refusing rather than reading fields by position against a schema that has moved" >&2
  printf '  expected: %s\n  actual:   %s\n' "$EXPECTED_HDR" "$ACTUAL_HDR" >&2
  exit 2
}

# bash 3.x on macOS has no associative arrays, so the lookup is a grep over the
# temp file. 98 rows x 98 lookups is nothing, and it keeps the map on disk where
# it can be inspected after a bad run.
exposure_for() {
  local hit
  # Field 5, and it is NOT a magic number: fixture_probe emits
  # file / status_dir / gen_view / region / exposure. Adding the region column
  # shifted `exposure` from 4 to 5 and this line kept reading 4, so the register
  # briefly carried the region COUNT (0 or 1) in the exposure column -- a
  # positional read silently surviving a schema change, which is the reason the
  # header is asserted below rather than trusted.
  hit="$(awk -F'\t' -v f="$1" '$1 == f {print $5; exit}' "$EXPOSURE_TSV")"
  # An empty lookup is a MISSING measurement, not a clean one. The probe walks
  # the same on-disk estate the burn TSV covers, so a miss means the two
  # disagree about what exists -- report it rather than printing `none`.
  [ -n "$hit" ] && printf '%s' "$hit" || printf 'UNPROBED'
}


# parity.md:32 -- "each [deviate row] carries a D-number ratified in design.md
# before the port lands". Meaningful only for `deviate`: a `keep` row changes
# nothing and has nothing to ratify, so printing a ref there would dilute the
# column into noise and make the one row that matters harder to find.
ratification_for() {
  case "$2" in
    deviate) lookup_ratification "$1" ;;
    *)       printf 'n/a' ;;
  esac
}

# `--short=7`, PINNED. git chooses abbreviation length from the repo's object
# count, so it GROWS: this file was stamped `c60cdbd` and a re-run today produces
# `c60cdbdf` for the same commit -- a one-character diff that makes the artefact
# non-reproducible and that the provenance guard reads as two artefacts naming
# different revisions. Same fix as gen_inventory.sh, which had it first; leaving
# the sibling unpinned meant the guard could still be tripped by a re-run.
REV="$(cd "$WT" && git rev-parse --short=7 HEAD 2>/dev/null)"
# A register that cannot name its revision is a rumour with a decimal point --
# the exact defect this artefact was built to avoid. It emitted `Measured at ``
# once, silently, from a mistyped WT. Refuse rather than publish that.
[ -n "$REV" ] || { echo "gen_register: cannot resolve a revision from WT=$WT; refusing to write an unstamped register" >&2; exit 2; }
# Defaults to the COMMITTED artefact, matching gen_dispatch_table.sh, so a
# re-run regenerates in place and `view_skew_check.sh` has something to compare.
# It cannot happen by accident: `WT` is still required and has no default, so
# nobody reaches this line without having named a worktree on purpose.
OUT="${OUT:-${SP:+$SP/register.md}}"
OUT="${OUT:-$HERE/../register.md}"
OUT_TMP="$(mktemp "${TMPDIR:-/tmp}/gen_register.XXXXXX")"
# One trap rather than a rm at each exit: this script has several refuse-and-
# exit paths and each new one is a chance to leak a temp file that nobody
# notices because leaking is silent.
trap 'rm -f "${OUT_TMP:-}" "${OUT_TMP:-}.aligned" "${EXPOSURE_TSV:-}"' EXIT

. "$HERE/lib_mdfmt.sh" || { echo "gen_register: cannot source $HERE/lib_mdfmt.sh -- refusing to emit a view that will not survive the formatter" >&2; exit 2; }

# CORPUS COVERAGE -- refuse a TSV that does not cover the on-disk estate.
#
# Measured 2026-08-14, and it had already happened: the committed
# `burn-baseline.tsv` carried 94 data rows against a 97-row register. Three
# files landed AFTER the baseline was taken, so the artefact this register
# names as its provenance could no longer reproduce it. Nothing noticed,
# because a register built from a short TSV is not malformed -- it is simply,
# silently, three files smaller than the estate it claims.
#
# That is fatal to AC-05.3 in particular. The AC says every file in the estate
# is classified; a register built from a partial TSV answers that question
# affirmatively about a corpus it chose for itself, which is the vacuous-green
# shape. The check lives in lib_corpus.sh because coverage_map.sh had the same
# bug independently -- see that file's header.
#
# The source is fatal in its own right. Without the `||`, a missing library
# under `set -uo pipefail` (no `-e`) merely prints and carries on to the next
# line -- so the guard silently would not run, which is the precise failure
# class this whole file exists to refuse.
. "$HERE/lib_corpus.sh" || { echo "gen_register: cannot source $HERE/lib_corpus.sh -- refusing to generate without the corpus-coverage guard" >&2; exit 2; }
corpus_require "$BURN" "gen_register" "$WT" || exit 2

{
  cat <<PREAMBLE
# Keep / retire / deviate register -- first pass (ST0056 / WP-01, AC-01.3)

> Measured at \`$REV\` by ic. Regenerated by \`tools/gen_register.sh\` from \`tools/burn.sh\` output; do not hand-edit rows.

## How each row was decided

The classification axis is the **burn ratio**: each file is run twice, once with the default \`INTENT_BIN\` and once with \`INTENT_BIN=/usr/bin/false\`, and the delta is the number of tests that actually reach the top-level CLI.

This is empirical where reading assertions is inferential, and the difference is not academic. \`treeindex_commands.bats\` reads as 53 CLI tests and is in fact 53 tests that exec \`bin/intent_treeindex\` directly, bypassing the dispatcher entirely; its burn is zero. \`claude_with_intent.bats\` read as a CLI test, reported zero burn, and turned out to alias the binary through an unbraced \`\$INTENT_BIN_DIR/intent\` that the retarget sweep had missed -- **the burn measurement found a hole in the retarget that four separate grep passes did not.**

| burn | meaning | class |
| ---- | ------- | ----- |
| all tests | the file exercises the CLI and nothing else | **keep** -- runs unmodified against \`INTENT_BIN\` |
| zero, and it calls \`bin/intent_<sub>\` | tests an entry point v3 will not have | **deviate** -- semantic rewrite, not a path swap |
| zero, and it sources a shell file | unit-tests a bash function | **retire** -- dies with the shell |
| zero, and it never invokes the CLI | pins this repo's own content | **out-of-scope** -- not a conformance test |
| some | mixed concerns in one file | **pending** -- needs per-test rows before WP-05 leans on it |
| any, but baseline not green | the delta carries no information | **UNCLASSIFIED** |

A file matching no rule is emitted UNCLASSIFIED rather than assigned a best guess. A wrong \`retire\` is coverage that disappears at the cut with nobody watching, which is the defect the AT grammar existed to kill; the refuse-lossy discipline applies to classification exactly as it applies to migration.

**Vocabulary (vc ruling, 2026-08-14): \`keep · retire · deviate · pending\`, shared verbatim with the dispatch table at \`surface/dispatch-table.json\`, with \`pending\` written explicitly and never implied by omitting a field.** Absence-as-meaning is un-greppable and reads as an oversight. The payoff is that **AC-05.3 becomes mechanical rather than eyeballed**: no row carries \`pending\` at close.

This pass renames \`split\` to \`pending\`. Nothing is lost -- the reason a row is pending was always carried by the \`basis\` column (\`partial burn\`), not by the class name, so the name was free to become the one the other artefact uses.

**Two values sit outside that four, deliberately, and ic flagged the divergence rather than collapsing it.** \`out-of-scope\` and \`UNCLASSIFIED\` are not dispositions on the same axis as the other four:

- \`out-of-scope\` answers _is this in the parity contract at all_ -- a decided answer, not a deferred one. Folding it into \`keep\` would claim a repo-content test is part of the conformance suite; folding it into \`retire\` would schedule a perfectly good test for deletion. Neither is true, and the orthogonal axis is real.
- \`UNCLASSIFIED\` is a MEASUREMENT FAILURE, not a deferred decision: the baseline was not green, so the burn delta means nothing. It must be zero at close for the same reason \`pending\` must, but the remedy is different -- \`pending\` needs a judgement, \`UNCLASSIFIED\` needs a working measurement.

**Scope note:** \`pending\` is a first-pass verdict, not a final one. Those files carry both portable and non-portable tests and need per-test rows; this pass deliberately stops at the file level rather than guessing which half is which.

## The \`v3 exposure\` column -- a SECOND predicate, orthogonal to burn

Burn asks whether a file reaches the **v2** CLI. Both of its runs are v2, one with the binary redirected, so it structurally cannot ask whether the file's own SETUP survives v3's file layout. Those are different questions and only the first was being measured -- which means \`keep\` was quietly promising something its evidence never established. A file can burn 12/12, earn \`keep\`, and fail every one of those tests under v3 **before a single assertion runs**, because its fixture wrote to a directory v3 does not have.

Found by cc from the v3 side (2026-08-14): they ran the \`keep\` set against the real v3 binary and got files where 17 of 17 reds trace to one cause in \`setup\`. This column is the v2-side half, computed statically by \`tools/fixture_probe.sh\` so both predicates sit on one row.

| value | what it means | remedy |
| ----- | ------------- | ------ |
| \`none\` | no literal v2 estate path | nothing -- burn's verdict stands |
| \`status-dir\` | writes \`intent/st/{COMPLETED,NOT-STARTED,CANCELLED}/\` | v3 holds status as a FIELD in \`st/<ID>/thread.json\`; there is no such directory, so the write fails outright |
| \`gen-view\` | hand-writes an \`info.md\` / \`acceptance.md\` under an st path | v3 GENERATES both. Worse than a failed write: it succeeds and is then outvoted by regeneration, or refused by the skew check |

**The two are not the same repair, and merging them overstates the cost.** A file that hand-builds the estate with \`mkdir\` converts to CLI-built fixtures, which is real work. A file that builds through the CLI and then reaches in at a literal path needs the path resolved, which is not. Several of the exposed files are the second kind and read as the first.

**This column reports EXPOSURE, not breakage.** Whether a given file actually goes red under v3 is a v3-side question owned by whoever runs it there; what is measurable from here is whether the file hardcodes a layout assumption at all -- the necessary condition, not the sufficient one. Reading it as the latter would repeat, in a new column, exactly the error it exists to correct.

Authored prose (\`design.md\`, \`impl.md\`, \`tasks.md\`) stays authored in v3, so fixtures touching those are deliberately not flagged.

## Rows

PREAMBLE
  printf '| test file | tests | burn | class | v3 exposure | ratification | basis | notes |\n'
  printf '| --------- | ----- | ---- | ----- | ----------- | ------------ | ----- | ----- |\n'
  tail -n +2 "$BURN" | while IFS=$'\t' read -r f total dfail burn status; do
    # A decided classification wins over any inferred one, whatever the burn
    # says. These are the files a grep cannot judge -- see OVERRIDES.
    ov="$(lookup_override "$f")"
    if [ -n "$ov" ]; then
      IFS='|' read -r cls basis note <<< "$ov"
      printf '| `%s` | %s | %s/%s | %s | %s | %s | %s | %s |\n' "$f" "$total" "$burn" "$total" "$cls" "$(exposure_for "$f")" "$(ratification_for "$f" "$cls")" "$basis" "$note"
      continue
    fi
    case "$status" in
      FULL)
        printf '| `%s` | %s | %s/%s | keep | %s | n/a | full burn | Every test changes result when the binary is redirected: the file exercises the CLI and nothing else. |\n' "$f" "$total" "$burn" "$total" "$(exposure_for "$f")"
        ;;
      NONE)
        IFS='|' read -r cls basis note <<< "$(classify_no_burn "$f")"
        printf '| `%s` | %s | 0/%s | %s | %s | %s | %s | %s |\n' "$f" "$total" "$total" "$cls" "$(exposure_for "$f")" "$(ratification_for "$f" "$cls")" "$basis" "$note"
        ;;
      MIXED)
        printf '| `%s` | %s | %s/%s | pending | %s | n/a | partial burn | %s of %s tests reach the CLI; the remainder do not. Needs per-test rows before WP-05 relies on it. |\n' "$f" "$total" "$burn" "$total" "$(exposure_for "$f")" "$burn" "$total"
        ;;
      UNSTABLE)
        printf '| `%s` | %s | -- | UNCLASSIFIED | %s | n/a | unstable baseline | %s test(s) already fail with the default binding, so the burn delta carries no information. Fix or explain before classifying. |\n' "$f" "$total" "$(exposure_for "$f")" "$dfail"
        ;;
      TIMEOUT)
        printf '| `%s` | %s | -- | UNCLASSIFIED | %s | n/a | measurement timed out | The run exceeded BURN_TIMEOUT and was killed, so neither binding produced a usable failure count. This is not a slow test and not a passing one: no measurement exists. Re-run this file alone before classifying. |\n' "$f" "$total" "$(exposure_for "$f")"
        ;;
      *)
        # NO SILENT ERRORS. burn.sh grew a TIMEOUT status on 2026-08-14 and this
        # case did not grow the matching arm, so for a few hours a timed-out
        # file would have fallen straight through and been emitted NOWHERE --
        # and a row missing from the register is indistinguishable from a file
        # that does not exist. The corpus-coverage check above would not have
        # caught it either: coverage compares the TSV to disk, and a dropped row
        # is lost AFTER that comparison passes.
        #
        # So this arm is the general fix rather than a second special case. Any
        # status this generator does not recognise becomes a loud UNCLASSIFIED
        # row naming the unknown value, because the failure mode to design
        # against is the register that quietly got smaller.
        printf '| `%s` | %s | -- | UNCLASSIFIED | %s | n/a | unrecognised burn status `%s` | burn.sh emitted a status this generator has no arm for. Emitted rather than dropped: a row silently absent from the register reads as a file that does not exist. Teach the generator this status, or fix the sweep that produced it. |\n' "$f" "$total" "$(exposure_for "$f")" "$status"
        ;;
    esac
  done

  # Summary is computed from the rows just emitted, not tallied by hand.
  #
  # UNMEASURED FILES ARE EXCLUDED FROM THE DENOMINATOR, not silently folded in.
  # An UNSTABLE or TIMEOUT row carries `--` in the burn column, which awk reads
  # as 0 in numeric context -- so summing blind would count every unmeasured
  # test as "does not reach the CLI" and quietly depress the percentage with
  # data that does not exist. The measured corpus and the whole corpus are
  # different numbers and the summary now says which is which.
  TOT=$(awk -F'\t' 'NR>1{T+=$2} END{print T+0}' "$BURN")
  TOT_M=$(awk -F'\t' 'NR>1 && $5!="UNSTABLE" && $5!="TIMEOUT"{T+=$2} END{print T+0}' "$BURN")
  CLI=$(awk -F'\t' 'NR>1 && $5!="UNSTABLE" && $5!="TIMEOUT"{B+=$4} END{print B+0}' "$BURN")
  DFAIL=$(awk -F'\t' 'NR>1 && $3!="--"{D+=$3} END{print D+0}' "$BURN")
  NTIMEOUT=$(awk -F'\t' 'NR>1 && $5=="TIMEOUT"{c++} END{print c+0}' "$BURN")
  NUNSTABLE=$(awk -F'\t' 'NR>1 && $5=="UNSTABLE"{c++} END{print c+0}' "$BURN")
  printf '\n## Summary\n\n'
  printf '| class | files | what WP-05 does with them |\n'
  printf '| ----- | ----- | ------------------------- |\n'
  for k in keep pending deviate retire out-of-scope UNCLASSIFIED; do
    # Counts come from the rows THIS RUN has just emitted, so it reads
    # $OUT_TMP -- the file currently being written -- not $OUT, which still
    # holds the PREVIOUS register until the mv at the end. Reading $OUT here
    # tallied the last run's classes into this run's summary, and the error was
    # invisible whenever the classes happened not to change between runs, which
    # is most of the time. The shell writes each printf straight through with no
    # userspace buffering, so the rows above are on disk by the time awk runs.
    n=$(awk -F'|' -v K="$k" '/^\| `tests\// {gsub(/^ +| +$/,"",$5); if ($5==K) c++} END{print c+0}' "$OUT_TMP" 2>/dev/null)
    [ "${n:-0}" = "0" ] && continue
    # The `keep` line is EXPOSURE-AWARE, because the unqualified version was the
    # over-promise this column was added to correct: `keep` is assigned on burn,
    # burn is a v2-side measurement, and a file can burn fully and still fail
    # every test under v3 before an assertion runs. Saying "run unmodified"
    # about all of them is the claim cc falsified by running them.
    kexp=$(awk -F'|' '/^\| `tests\// {gsub(/^ +| +$/,"",$5); gsub(/^ +| +$/,"",$6); if ($5=="keep" && $6!="none" && $6!="") c++} END{print c+0}' "$OUT_TMP" 2>/dev/null)
    case "$k" in
      keep)
        if [ "${kexp:-0}" -gt 0 ]; then
          w="Run unmodified against the v3 binary -- EXCEPT the $kexp carrying v3-layout exposure (see that column), whose fixtures hardcode v2 estate paths and fail in setup before any assertion runs. The rest are the conformance suite."
        else
          w='Run unmodified against the v3 binary. These are the conformance suite.'
        fi ;;
      pending)      w='Need per-test rows first: each mixes tests that reach the CLI with tests that do not. At close, no `pending` row may remain for a file touching a CORE family; non-core `pending` rows are deferred to AC-00.1 by name (vc ruling, 2026-08-14, superseding the earlier "bucket EMPTY at close").' ;;
      deviate)      w='Rewrite against the single-binary entry point, or retire with the sub-script they exercise.' ;;
      retire)       w='Retire with the shell. No binary to point them at.' ;;
      out-of-scope) w='Leave alone. They pin this repo content and are unaffected by the binary swap.' ;;
      *)            w='Adjudicate before WP-05 relies on them.' ;;
    esac
    printf '| **%s** | %s | %s |\n' "$k" "$n" "$w"
  done
  printf '\n**%s of %s MEASURED tests (%s) actually reach the CLI.** The remaining %s do not, and cannot serve as v3 conformance evidence whatever their assertions say. That number is the honest size of the conformance estate, and it is the figure WP-05 should plan against rather than the 1235 headline.\n' \
    "$CLI" "$TOT_M" "$(echo "$CLI $TOT_M" | awk '{if ($2>0) printf "%.0f%%", 100*$1/$2; else print "n/a -- nothing measured"}')" "$((TOT_M - CLI))"

  # The denominator is stated only when it is not the whole estate. Printing
  # "0 tests unmeasured" on every clean run trains the reader to skip the line,
  # which is how the one run that matters gets skipped too.
  if [ "$TOT" -ne "$TOT_M" ]; then
    printf '\n**%s of the %s tests in the estate were NOT MEASURED and are excluded from that ratio** -- %s file(s) timed out, %s had a non-green baseline. They are UNCLASSIFIED rows, not zero-burn rows: no measurement exists for them, which is a different claim from "does not reach the CLI" and must not be averaged in as if it were.\n' \
      "$((TOT - TOT_M))" "$TOT" "$NTIMEOUT" "$NUNSTABLE"
  fi

  # The old wording here asserted "all N tests pass with the default INTENT_BIN"
  # unconditionally, from a template rather than from the data -- so a run with
  # a red baseline or a timed-out file would have published a clean bill of
  # health it had just measured to be false. The claim is now conditional on
  # the numbers that back it.
  if [ "$DFAIL" -eq 0 ] && [ "$NTIMEOUT" -eq 0 ]; then
    printf '\nBaseline: all %s tests pass with the default `INTENT_BIN`, so the retarget is behaviour-neutral. That run was taken in a sacrificial worktree and is evidence, not certification -- the authoritative full-suite run is matts.\n' "$TOT"
  else
    printf '\n**Baseline NOT clean, and the retarget cannot be called behaviour-neutral on this run.** %s test(s) fail under the default `INTENT_BIN` and %s file(s) timed out. Every burn delta from an affected file is uninformative and its row is UNCLASSIFIED. Repair the measurement and re-run before reading anything else in this table as evidence.\n' "$DFAIL" "$NTIMEOUT"
  fi
} > "$OUT_TMP"

# ALIGN THROUGH THE FORMATTER, then move into place.
#
# Two properties, both learned by committing and watching rather than by
# reasoning. First, the view must come out at the repo formatter's own column
# widths, or every regeneration diffs against the committed file for ever and
# any regenerate-and-compare check cries wolf on its first run. Second, the
# render goes to a temp and only `mv`s on success, so an abort leaves the
# committed view untouched instead of truncated -- gen_dispatch_table.sh
# published an EMPTY view exactly once by writing straight to its output and
# then refusing partway through.
md_align "$OUT_TMP" "$OUT_TMP.aligned" || { echo "gen_register: table alignment failed -- committed register left untouched" >&2; rm -f "$OUT_TMP"; exit 2; }
mv "$OUT_TMP.aligned" "$OUT" || { echo "gen_register: cannot move the rendered register into $OUT" >&2; exit 2; }
rm -f "$OUT_TMP"

# PROVENANCE IS EMITTED, NEVER HAND-COPIED.
#
# The register's header names `tools/burn-baseline.tsv` as the measurement
# behind it. That file went stale precisely because it was a MANUAL copy of an
# ephemeral `$SP/burn.tsv`, taken at some moment nobody recorded: the register
# was regenerated three times, the baseline was copied once, and the two drifted
# apart in silence while the header went on claiming they matched.
#
# Emitting both from one run closes that at the source. There is exactly one
# writer for the baseline and it is the same code path that writes the register,
# so the pair cannot come from different sweeps -- the copy step that could get
# it wrong no longer exists.
BASELINE_OUT="${BASELINE_OUT:-$(dirname "$OUT")/burn-baseline.tsv}"
cp "$BURN" "$BASELINE_OUT" || {
  echo "gen_register: wrote the register but could NOT write its baseline to $BASELINE_OUT" >&2
  echo "  The register on disk is unusable as evidence until its provenance lands beside it -- that pairing is the whole point. Do not commit it alone." >&2
  exit 2
}

# Count only data rows: the preamble also contains tables, so anchor on the
# leading `tests/ path cell rather than on line position.
echo "rows: $(grep -c '^| `tests/' "$OUT")"
awk -F'|' '/^\| `tests\// {gsub(/^ +| +$/,"",$5); c[$5]++} END {for (k in c) printf "  %-14s %s\n", k, c[k]}' "$OUT"
# Field indices: a leading "|" makes $1 empty, so file=$2, tests=$3, burn=$4,
# class=$5. Reading burn from $3 silently reported "0 tests".
awk -F'|' '/^\| `tests\// {gsub(/[^0-9]/,"",$3); gsub(/[^0-9\/]/,"",$4); split($4,a,"/"); T+=$3; B+=a[1]} END {printf "  %s tests, %s reaching the CLI (%.0f%%)\n", T, B, 100*B/T}' "$OUT"
