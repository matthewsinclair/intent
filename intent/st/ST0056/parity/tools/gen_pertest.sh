#!/bin/bash
# gen_pertest.sh -- per-test rows for the `pending` files.
#
# A `pending` file is one the burn ratio found MIXED: some of its tests reach
# the CLI and some do not, in one file. That is an honest first-pass verdict and
# a useless one to plan against -- WP-05 cannot point a mixed file at the v3
# binary, because half of it would go red for reasons that have nothing to do
# with conformance. This tool resolves the file into its two halves by name.
#
# IT RUNS NO TESTS. It reads the TAP that burn.sh captured under BURN_TAP_DIR.
# The alternative was 487 filtered `bats -f` invocations, one per test; the
# cheap method was already in hand and is exact:
#
#   the default binding is green (no `not ok`)
#   the mutant binding fails exactly the tests that reach the CLI
#   therefore the mutant run's `not ok` set IS the burning set, BY NAME
#
# Two runs per file, already paid for by the sweep. And because the run logic
# stays in burn.sh, there is one instrument and one measurement behind both the
# file-level register and these rows -- they cannot disagree about what was
# measured, only about how finely it was reported.
#
# THE GREEN BASELINE IS CHECKED PER FILE, NOT ASSUMED. If the default TAP has
# any failure, the mutant `not ok` set is a mixture of "reaches the CLI" and
# "was already broken", and splitting on it would silently mark a broken test
# as conformance coverage. Such a file is reported and NOT split.

set -uo pipefail

# --verify -- re-derive every non-burning row's class from the SOURCE and report
# disagreement with what the committed artefact records. Needs no TAP and runs
# no tests, so it works long after the sweep's temp directory has gone.
#
# It exists because the classification rules live in lib_classify.sh and can be
# CORRECTED, while regenerating this artefact needs a TAP capture that is
# ephemeral -- it lives in the sweep's temp directory and dies with it. That
# asymmetry is a drift generator: fix a rule, regenerate the file-level register
# in seconds, and the per-test register silently keeps the old answer with
# nothing to signal it.
#
# THE SWEEP IS CHEAP AND THIS COMMENT USED TO SAY OTHERWISE. It said "multi-hour",
# taking that from the one measured run -- which spent nearly all of its time
# HUNG on a single file rather than measuring. Timed properly on 2026-08-15:
# **7m52s for all 98 files, both bindings, 896K of TAP.** The hang did not
# reproduce, so the cost that justified calling this expensive was a defect, not
# a property of the sweep.
#
# That does NOT make this mode redundant, and the distinction is worth keeping
# straight: --verify answers "is the artefact stale?", which a re-sweep cannot
# answer without doing the work first. Detection is the cheap half and it stays
# cheap. What changes is the REMEDY -- eight minutes, not an expedition -- so a
# stale row should now be fixed rather than deferred with a note.
#
# Live case, and the reason this mode was written rather than a note left on a
# board: the `retire` needle required a literal double quote after `source`, so
# it missed every site sourcing from inside a `bash -c "..."` -- where the inner
# quote MUST be single. Fixing it moved one file in the register and two rows
# here, and the two rows were unreachable because the TAP was gone. A remembered
# defect is a forgotten defect; a reported one is not.
#
# The burn column is NOT re-derived. That would mean reading this artefact to
# regenerate itself, and a provenance chain that closes on its own output cannot
# detect the error it was built to catch.
if [ "${1:-}" = "--verify" ]; then
  HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  . "$HERE/lib_classify.sh" || { echo "gen_pertest --verify: cannot source lib_classify.sh" >&2; exit 2; }
  # Calibrate here too, and the reason is specific to this mode: --verify
  # compares the artefact against the CURRENT rules, so a broken needle does not
  # merely mis-verify -- it reports the correct committed rows as stale and
  # sends someone off to "fix" them. A wrong answer that generates work is worse
  # than a wrong answer that sits still.
  classify_calibrate || { echo "gen_pertest --verify: classification rules failed calibration -- refusing to call rows stale against a needle that is itself broken" >&2; exit 2; }
  ART="${ART:-$HERE/../pertest.md}"
  [ -f "$ART" ] || { echo "gen_pertest --verify: no artefact at $ART" >&2; exit 2; }
  WT="${WT:-$(cd "$HERE/../../../../.." && pwd)}"
  cd "$WT" || { echo "gen_pertest --verify: WT is not a directory: $WT" >&2; exit 2; }

  vbody="$(mktemp "${TMPDIR:-/tmp}/gen_pertest_verify.XXXXXX")"
  vrows="$(mktemp "${TMPDIR:-/tmp}/gen_pertest_rows.XXXXXX")"
  trap 'rm -f "$vbody" "$vrows"' EXIT

  # \001 stands in for an escaped pipe through the field split, exactly as the
  # tally below does -- a test name may legitimately contain one.
  awk -F'|' '{gsub(/\\\|/, "\001")} /^\| `tests\// {
    gsub(/^ +| +$/,"",$2); gsub(/`/,"",$2);
    gsub(/^ +| +$/,"",$3); gsub(/^ +| +$/,"",$4); gsub(/^ +| +$/,"",$5);
    if ($4=="no") print $2"\t"$3"\t"$5
  }' "$ART" > "$vrows"

  vchecked=0; vmoved=0; vmissing=0
  while IFS=$'\t' read -r vf vt vrec; do
    vt="$(printf '%s' "$vt" | tr '\001' '|')"
    if ! extract_test_body "$vf" "$vt" "$vbody"; then
      # NOT skipped silently. A body that cannot be found is an unverified row,
      # which is a different claim from a verified-clean one.
      printf 'UNVERIFIED  %s\n            %s\n            body not found -- suspect the extraction heuristic before the classifier\n' "$vf" "$vt"
      vmissing=$((vmissing + 1)); continue
    fi
    vchecked=$((vchecked + 1))
    vnew="$(classify_no_burn "$vbody" | cut -d'|' -f1)"
    if [ "$vnew" != "$vrec" ]; then
      printf 'STALE       %s\n            %s\n            recorded %s, rules now say %s\n' "$vf" "$vt" "$vrec" "$vnew"
      vmoved=$((vmoved + 1))
    fi
  done < "$vrows"

  echo "---"
  echo "non-burning rows verified: $vchecked   stale: $vmoved   unverifiable: $vmissing"
  if [ "$vmoved" -gt 0 ] || [ "$vmissing" -gt 0 ]; then
    echo "The artefact disagrees with the current rules. Regenerating it needs a TAP capture:"
    echo "  BURN_TAP_DIR=<dir> burn.sh ...   then   TAP_DIR=<dir> gen_pertest.sh"
    exit 1
  fi
  echo "pertest.md agrees with the current classification rules."
  exit 0
fi

SP="${SP:?set SP -- directory holding burn.tsv}"
WT="${WT:?set WT -- the worktree the sweep measured}"
BURN="$SP/burn.tsv"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# THE TAP INPUT IS COMMITTED, AT `tools/tap-baseline/`, and defaults to it --
# the same shape `gen_register.sh` uses for `tools/burn-baseline.tsv`, because
# a generator whose documented invocation does not work is a generator nobody
# can re-run. Ruled by vc 2026-08-15 after ic flagged that this artefact's only
# input lived under a `/tmp` scratchpad: **a committed generated artefact whose
# only input is in `/tmp` is re-derivable today and not tomorrow, and nothing
# anywhere records which of those two states it is in.**
#
# COMMITTED RAW RATHER THAN COMPRESSED, and that is a measurement rather than a
# preference. vc ruled "compressed if it helps" and it does not: measured into
# a scratch git repo, the 196 raw files pack to 220K and a `.tar.gz` of the same
# corpus packs to 200K. **10%, not the 11x the raw sizes suggest** -- git's own
# zlib does the work either way. 20K does not buy making an audit artefact
# opaque, undiffable, and un-deltable against every future capture.
#
# AND IT CANNOT BE RE-CAPTURED IDENTICALLY, which is the real argument for
# keeping it rather than a note about convenience. Two independent runs of the
# same corpus were compared: 193 of 196 files byte-identical, and the 3 that
# differ do so ONLY in TAP `#` diagnostic lines carrying `mktemp` directory
# names and the worktree path. The RESULTS are identical; the noise is not
# reproducible. So this corpus is one measurement, preserved, not a derivable
# artefact -- re-running the sweep produces an equally valid corpus that is not
# this one.
#
# Re-run (WT is re-derivable, unlike the TAP -- the revision is committed):
#   git worktree add <wt> c60cdbd
#   mkdir -p <sp> && cp tools/burn-baseline.tsv <sp>/burn.tsv
#   SP=<sp> WT=<wt> bash tools/gen_pertest.sh
# Verified 2026-08-15: reproduces the committed `pertest.md` BYTE-IDENTICALLY.
TAP_DIR="${TAP_DIR:-$HERE/tap-baseline}"
. "$HERE/lib_classify.sh" || { echo "gen_pertest: cannot source lib_classify.sh -- refusing to classify without the shared rules" >&2; exit 2; }
classify_calibrate || { echo "gen_pertest: classification rules failed calibration -- refusing to emit per-test rows from a needle that has stopped matching a form it covers" >&2; exit 2; }
. "$HERE/lib_mdfmt.sh"    || { echo "gen_pertest: cannot source lib_mdfmt.sh -- refusing to emit a view that will not survive the formatter" >&2; exit 2; }
. "$HERE/lib_corpus.sh"   || { echo "gen_pertest: cannot source lib_corpus.sh -- refusing to report without the corpus guard" >&2; exit 2; }

[ -d "$TAP_DIR" ] || { echo "gen_pertest: no TAP directory at $TAP_DIR -- re-run burn.sh with BURN_TAP_DIR set" >&2; exit 2; }
cd "$WT" || { echo "gen_pertest: WT is not a directory: $WT" >&2; exit 2; }

corpus_require "$BURN" "gen_pertest" "$WT" || exit 2

# `--short=7`, PINNED -- git grows the abbreviation with the object count, so the
# same commit renders `c60cdbd` today and `c60cdbdf` tomorrow. Read after the
# `cd "$WT"` above, so this is the MEASURED tree and not the caller's.
REV="$(git rev-parse --short=7 HEAD 2>/dev/null)"
[ -n "$REV" ] || { echo "gen_pertest: cannot resolve a revision from WT=$WT; refusing to write an unstamped artefact" >&2; exit 2; }
OUT="${OUT:-$SP/pertest.md}"
OUT_TMP="$(mktemp "${TMPDIR:-/tmp}/gen_pertest.XXXXXX")"
BODY_TMP="$(mktemp "${TMPDIR:-/tmp}/gen_pertest_body.XXXXXX")"
trap 'rm -f "$OUT_TMP" "$OUT_TMP.aligned" "$BODY_TMP"' EXIT

tap_slug() { printf '%s' "$1" | tr '/' '_'; }

# A test name is free text and lands in a markdown table cell, so a literal pipe
# would silently add a column. Escaped rather than stripped: the name must stay
# greppable back to the source.
md_cell() { printf '%s' "$1" | sed 's/|/\\|/g'; }

# TAP lines are `ok N name` / `not ok N name`. The captured stream is 2>&1, so
# it also carries diagnostics; anchor on the TAP grammar rather than taking
# every line. `# skip` / `# todo` directives are kept as part of the name and
# surface in the row rather than being silently dropped.
tap_passed()  { grep -E '^ok [0-9]+ '     "$1" 2>/dev/null | sed -E 's/^ok [0-9]+ //'; }
tap_failed()  { grep -E '^not ok [0-9]+ ' "$1" 2>/dev/null | sed -E 's/^not ok [0-9]+ //'; }

refusals=0
split_files=0

{
  cat <<PREAMBLE
# Per-test register for the pending files (ST0056 / WP-05, AC-05.3)

> Measured at \`$REV\` by ic. Generated by \`tools/gen_pertest.sh\` from the TAP \`tools/burn.sh\` captured under \`BURN_TAP_DIR\`; do not hand-edit rows.

## What a row means

A **pending** file mixes tests that reach the CLI with tests that do not. The file-level register says so honestly and cannot say more; this table names which is which, so WP-05 can point the reaching half at the v3 binary and treat the rest on its merits.

The split is **measured, not read**. Under the default binding every test passes; under \`INTENT_BIN=/usr/bin/false\` exactly the tests that reach the CLI fail. So the mutant run's \`not ok\` set is the burning set, by name, with no assertion-parsing involved.

| the mutant run says | meaning                                | class                                        |
| ------------------- | -------------------------------------- | -------------------------------------------- |
| \`not ok\`            | the test's result depends on the binary | **keep** -- real conformance coverage        |
| \`ok\`                | the binary is irrelevant to it          | classified by why, using the shared rules    |

**One class of row is NOT measured, and says so.** A test asserting a FAILURE passes under both bindings, because \`/usr/bin/false\` fails too -- so the burn ratio is structurally blind to it and a zero there means "the instrument cannot see this", not "this does not reach the CLI". Those rows carry **\`basis: read, not measured\`**, are **excluded from every burn arithmetic**, and are **counted separately** in the tally (vc ruling, 2026-08-15). The register's authority is that it never reads assertions; that holds for every measured row, and these sit visibly outside it rather than quietly inside. The blindness is one-directional -- burn under-counts CLI reach and never over-counts -- so every burn figure here is a FLOOR.

Files whose class was **decided by ruling** rather than measured are listed at the foot and deliberately not split: per-test rows would quietly contradict the ruling the file row carries.

## Rows

PREAMBLE

  printf '| test file | test | burns | class | basis |\n'
  printf '| --------- | ---- | ----- | ----- | ----- |\n'

  overridden=""
  while IFS=$'\t' read -r f total dfail burn status; do
    [ "$status" = "MIXED" ] || continue

    if [ -n "$(lookup_override "$f")" ]; then
      overridden="$overridden $f"
      continue
    fi

    dtap="$TAP_DIR/$(tap_slug "$f").default.tap"
    mtap="$TAP_DIR/$(tap_slug "$f").mutant.tap"
    if [ ! -f "$dtap" ] || [ ! -f "$mtap" ]; then
      printf '| `%s` | -- | -- | UNCLASSIFIED | TAP missing | No captured TAP for this file, so its tests cannot be named. A missing measurement is not an empty one -- re-run burn.sh with BURN_TAP_DIR set. |\n' "$f"
      refusals=$((refusals + 1))
      continue
    fi

    dfail_n="$(tap_failed "$dtap" | grep -c . || true)"
    if [ "$dfail_n" -ne 0 ]; then
      printf '| `%s` | -- | -- | UNCLASSIFIED | baseline not green | %s test(s) fail under the DEFAULT binding, so the mutant failures mix "reaches the CLI" with "was already broken". Splitting on that would mark a broken test as conformance coverage. |\n' "$f" "$dfail_n"
      refusals=$((refusals + 1))
      continue
    fi

    # CROSS-CHECK the TAP against the TSV. Two independent derivations of the
    # same number: the TSV's BURN came from counting `not ok` at sweep time, this
    # comes from naming them now. If they disagree, one of the two artefacts is
    # stale or the parse is wrong, and either way the rows would be fiction.
    burning="$(tap_failed "$mtap")"
    nburn="$(printf '%s' "$burning" | grep -c . || true)"
    if [ "$nburn" -ne "$burn" ]; then
      printf '| `%s` | -- | -- | UNCLASSIFIED | TAP and TSV disagree | The TSV records burn %s; the captured TAP names %s failing test(s). The artefacts describe different runs, so no split is trustworthy. Re-run the sweep and regenerate both together. |\n' "$f" "$burn" "$nburn"
      refusals=$((refusals + 1))
      continue
    fi

    split_files=$((split_files + 1))

    printf '%s\n' "$burning" | while IFS= read -r t; do
      [ -n "$t" ] || continue
      printf '| `%s` | %s | yes | keep | full burn on this test | Result changes when the binary is redirected: this test exercises the CLI and is real conformance coverage. |\n' "$f" "$(md_cell "$t")"
    done

    tap_passed "$mtap" | while IFS= read -r t; do
      [ -n "$t" ] || continue
      if extract_test_body "$f" "$t" "$BODY_TMP"; then
        IFS='|' read -r cls basis note <<< "$(classify_no_burn "$BODY_TMP")"
      else
        cls="UNCLASSIFIED"; basis="body not found"
        note="The TAP names this test but its @test block could not be extracted from the source, so no rule was applied to it. Suspect the block-extraction heuristic (see lib_classify.sh) before the classifier."
      fi
      printf '| `%s` | %s | no | %s | %s | %s |\n' "$f" "$(md_cell "$t")" "$cls" "$basis" "$note"
    done
  done < <(tail -n +2 "$BURN")

  printf '\n## Files not split, by ruling\n\n'
  if [ -n "$overridden" ]; then
    for f in $overridden; do
      printf -- '- `%s` -- %s\n' "$f" "$(lookup_override "$f" | cut -d'|' -f2)"
    done
  else
    printf -- '_(none)_\n'
  fi
} > "$OUT_TMP"

md_align "$OUT_TMP" "$OUT_TMP.aligned" || { echo "gen_pertest: table alignment failed -- committed view left untouched" >&2; exit 2; }
mv "$OUT_TMP.aligned" "$OUT" || { echo "gen_pertest: cannot move the rendered view into $OUT" >&2; exit 2; }

# Row counts are read back from the emitted FILE rather than tallied in flight,
# and the reason is worth stating because half of it is a real constraint and
# half is a choice.
#
# The constraint: the two inner row loops sit on the right of a pipe, so they
# run in subshells and any counter incremented inside them is lost at the pipe.
# (The outer file loop is fed by process substitution inside a brace group, so
# `split_files` and `refusals` DO survive -- those are tallied in flight.)
#
# The choice: even where a tally would survive, counting the artefact is the
# stronger check. It reports what was actually written, not what the code
# believed it wrote, and those diverge exactly when something has gone wrong.
echo "rows: $(grep -c '^| `tests/' "$OUT")"
# STRIP ESCAPED PIPES BEFORE SPLITTING ON PIPE. A test name may contain a
# literal `|` -- claude_with_intent.bats has "invocable as intent claude
# start|ws through the dispatch" -- which md_cell escapes to `\|` so the table
# renders correctly. awk -F'|' splits on it anyway, shifting every later column
# by one, and the tally then reported a class of `yes` (the BURNS column) for
# that row. The ROW was right and the COUNT was wrong, which is the more
# dangerous way round: the artefact looks fine and only the summary lies.
awk -F'|' '{gsub(/\\\|/, "!")} /^\| `tests\// {
  gsub(/^ +| +$/,"",$5); gsub(/^ +| +$/,"",$6);
  # A `read, not measured` row is counted SEPARATELY and never folded into its
  # class total. The ruling by vc makes the evidence class mechanical rather
  # than trusted: these rows rest on reading an assertion, every other row rests
  # on a measurement, and a tally that merges them launders the difference.
  # (No apostrophes in here -- this awk program is a single-quoted shell string,
  # so one would close it and hand the rest of the block to the shell.)
  if ($6 == "read, not measured") c[$5 " (read)"]++; else c[$5]++
} END {for (k in c) printf "  %-20s %s\n", k, c[k]}' "$OUT"
echo "files split: $split_files   refused: $refusals"
