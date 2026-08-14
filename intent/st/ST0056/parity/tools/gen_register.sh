#!/bin/bash
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
SP="${SP:?}"; WT="${WT:?}"
BURN="$SP/burn.tsv"

# Resolve the script's own directory to an ABSOLUTE path BEFORE the cd below.
# `dirname "${BASH_SOURCE[0]}"` is relative to the invocation, so reading it
# after `cd "$WT"` resolves against the worktree and the source fails.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$WT" || { echo "gen_register: WT is not a directory: $WT" >&2; exit 2; }

# DECIDED, not inferred. Every rule below is a grep, and a grep cannot tell code
# from data: a file that carries a call-site pattern as a test FIXTURE matches
# the rule for making that call. `intent_bin_retarget_guard.bats` is the live
# case -- it greps the estate for `bin/intent_<sub>` spellings and holds them as
# literal strings, so the sub-script rule fires on a file that invokes nothing.
#
# Rather than loosen the rule until it stops noticing (which would blind it to
# the real sites), such files are named here with the reason. Same discipline as
# the guard's own allowlist: a classification the machine cannot make is stated
# by a human, never guessed, because a wrong `retire` is coverage that vanishes
# at the cut with nobody watching.
#
# Format: <basename>|<class>|<basis>|<note>
OVERRIDES="
intent_bin_retarget_guard.bats|out-of-scope|harness invariant, decided|Guards the \$INTENT_BIN invariant across the estate by reading test SOURCE; it invokes no CLI at all. It holds \`bin/intent_<sub>\` spellings as literal needles, which the sub-script rule cannot tell from a call site. Carries into v3 unchanged in purpose -- whatever the binary is, the estate must reach it through one name.
whiteboard_clock_guard.bats|out-of-scope|hook behaviour, decided|Exercises a pre-commit hook in a throwaway git repo, not the Intent CLI. Unaffected by the binary swap.
organize_commands.bats|retire|hv ruling 2026-08-14|Retires with the command. hv ruled \`organize\` vestigial by construction -- a strictly structured model cannot hold data in the wrong spot or format -- so both implementations are planned retires (parity.md, 2026-08-14; via vc). Classified by ruling, not by burn.
"

lookup_override() {
  printf '%s\n' "$OVERRIDES" | grep -F "$(basename "$1")|" | head -1 | cut -d'|' -f2-
}

classify_none() {
  local f="$1"
  if grep -qE '\$\{INTENT_BIN_DIR\}/intent_[a-z_]+' "$f"; then
    echo "deviate|sub-script entry point|Invokes bin/intent_<sub> directly, bypassing the dispatcher (PROJECT_ROOT resolution, INTENT_ORIG_CWD, cd to root -- bin/intent:198-218). No equivalent under one binary; needs a semantic rewrite, not a path swap."
  elif grep -qE 'source "\$\{?(INTENT_PROJECT_ROOT|INTENT_HOME)\}?/bin/intent|source .*(rules_lib|critic_runner)\.sh' "$f"; then
    echo "retire|shell-function unit test|Sources a shell file and calls its functions directly. Dies with bash; there is no binary to retarget."
  elif ! grep -qE 'run_intent|\$INTENT_BIN\b' "$f"; then
    # Never invokes the CLI in any form. Not a conformance test at all: it pins
    # this repository's own content (skills, rules, docs, attribution) and
    # survives a binary swap untouched. This is the widened rule -- the first
    # version keyed on `git ls-files`/`grep -r` and left seven such files
    # UNCLASSIFIED, certifying the shapes it already knew about.
    echo "out-of-scope|no CLI invocation|Never invokes the CLI. Asserts this repository's own content, not the command surface; survives a binary swap untouched and is not a conformance test."
  else
    # Invokes the CLI and yet nothing changes when the binary is redirected.
    # That combination is genuinely odd and is flagged rather than guessed.
    echo "UNCLASSIFIED|invokes CLI, zero burn|Calls the CLI but no test changes result when the binary is redirected. Either the invocation is inert or the assertions do not depend on it. Needs adjudication."
  fi
}

REV="$(cd "$WT" && git rev-parse --short HEAD 2>/dev/null)"
# A register that cannot name its revision is a rumour with a decimal point --
# the exact defect this artefact was built to avoid. It emitted `Measured at ``
# once, silently, from a mistyped WT. Refuse rather than publish that.
[ -n "$REV" ] || { echo "gen_register: cannot resolve a revision from WT=$WT; refusing to write an unstamped register" >&2; exit 2; }
DATE="$(date -u +%Y-%m-%d)"
OUT="${OUT:-$SP/register.md}"

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

> Measured at \`$REV\` on $DATE by ic. Regenerated by \`tools/gen_register.sh\` from \`tools/burn.sh\` output; do not hand-edit rows.

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

- \`out-of-scope\` answers *is this in the parity contract at all* -- a decided answer, not a deferred one. Folding it into \`keep\` would claim a repo-content test is part of the conformance suite; folding it into \`retire\` would schedule a perfectly good test for deletion. Neither is true, and the orthogonal axis is real.
- \`UNCLASSIFIED\` is a MEASUREMENT FAILURE, not a deferred decision: the baseline was not green, so the burn delta means nothing. It must be zero at close for the same reason \`pending\` must, but the remedy is different -- \`pending\` needs a judgement, \`UNCLASSIFIED\` needs a working measurement.

**Scope note:** \`pending\` is a first-pass verdict, not a final one. Those files carry both portable and non-portable tests and need per-test rows; this pass deliberately stops at the file level rather than guessing which half is which.

## Rows

PREAMBLE
  printf '| test file | tests | burn | class | basis | notes |\n'
  printf '| --------- | ----- | ---- | ----- | ----- | ----- |\n'
  tail -n +2 "$BURN" | while IFS=$'\t' read -r f total dfail burn status; do
    # A decided classification wins over any inferred one, whatever the burn
    # says. These are the files a grep cannot judge -- see OVERRIDES.
    ov="$(lookup_override "$f")"
    if [ -n "$ov" ]; then
      IFS='|' read -r cls basis note <<< "$ov"
      printf '| `%s` | %s | %s/%s | %s | %s | %s |\n' "$f" "$total" "$burn" "$total" "$cls" "$basis" "$note"
      continue
    fi
    case "$status" in
      FULL)
        printf '| `%s` | %s | %s/%s | keep | full burn | Every test changes result when the binary is redirected: the file exercises the CLI and nothing else. |\n' "$f" "$total" "$burn" "$total"
        ;;
      NONE)
        IFS='|' read -r cls basis note <<< "$(classify_none "$f")"
        printf '| `%s` | %s | 0/%s | %s | %s | %s |\n' "$f" "$total" "$total" "$cls" "$basis" "$note"
        ;;
      MIXED)
        printf '| `%s` | %s | %s/%s | pending | partial burn | %s of %s tests reach the CLI; the remainder do not. Needs per-test rows before WP-05 relies on it. |\n' "$f" "$total" "$burn" "$total" "$burn" "$total"
        ;;
      UNSTABLE)
        printf '| `%s` | %s | -- | UNCLASSIFIED | unstable baseline | %s test(s) already fail with the default binding, so the burn delta carries no information. Fix or explain before classifying. |\n' "$f" "$total" "$dfail"
        ;;
      TIMEOUT)
        printf '| `%s` | %s | -- | UNCLASSIFIED | measurement timed out | The run exceeded BURN_TIMEOUT and was killed, so neither binding produced a usable failure count. This is not a slow test and not a passing one: no measurement exists. Re-run this file alone before classifying. |\n' "$f" "$total"
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
        printf '| `%s` | %s | -- | UNCLASSIFIED | unrecognised burn status `%s` | burn.sh emitted a status this generator has no arm for. Emitted rather than dropped: a row silently absent from the register reads as a file that does not exist. Teach the generator this status, or fix the sweep that produced it. |\n' "$f" "$total" "$status"
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
    n=$(awk -F'|' -v K="$k" '/^\| `tests\// {gsub(/^ +| +$/,"",$5); if ($5==K) c++} END{print c+0}' "$OUT" 2>/dev/null)
    [ "${n:-0}" = "0" ] && continue
    case "$k" in
      keep)         w='Run unmodified against the v3 binary. These are the conformance suite.' ;;
      pending)      w='Need per-test rows first: each mixes tests that reach the CLI with tests that do not. AC-05.3 requires this bucket EMPTY at close.' ;;
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
} > "$OUT"

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
