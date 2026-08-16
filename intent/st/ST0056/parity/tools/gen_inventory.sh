#!/bin/bash
#
# INPUTS ARE DECLARED, NOT INFERRED, and generator_inputs_check.sh reads these
# two lines. Repo-relative paths, each of which must be tracked by git: a
# committed artefact whose only input lives in a scratchpad is re-derivable
# today and not tomorrow, and nothing else records which of those it is.
# NOTE: the paths below are the COMMITTED inputs. Where this generator still
# reads a `$SP` scratch copy of one, that is a gap between what it needs and
# what it reaches for -- not a gap in the declaration.
# inputs: intent/st/ST0056/parity/probes/toplevel.tsv intent/st/ST0056/parity/tools/extract_verbs.sh intent/st/ST0056/parity/tools/extract_flags.sh intent/st/ST0056/parity/tools/lib_mdfmt.sh
# inputs-exempt: WT -- a detached git worktree at a committed revision. Re-derivable by `git worktree add <dir> <sha>` and not a file in this repository, so it can never be tracked. The rule is about inputs that are FILES.
# gen_inventory.sh -- emit the per-command parity inventory from measured data.
#
# The inventory is GENERATED, not written. It gets re-measured after the consumer
# sweeps and again during WP-06, and a hand-typed list cannot be diffed: the
# "314 AT rows" figure that was wrong by 5x sat in three documents precisely
# because nobody could re-run it. Every file below names the revision it was
# taken at.
#
# Sources, in order of authority:
#   1. runtime probes   -- what the tool actually did (probes/*.tsv)
#   2. dispatch arms    -- what the source intends (extract_verbs.sh)
#   3. arg-parsing arms -- the flag grammar (extract_flags.sh)
#   4. lib/help/*.help.md -- documentation, cross-checked for drift, never trusted
#
# Help is source 4 and last for a reason: it covers 11 of 27 commands, its
# @usage/@options/@arguments grammar is used by exactly one file, and it still
# describes `upgrade` as "Upgrade from STP to Intent v2.1.0" at v2.19.0.

set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# `SP` is OPTIONAL -- an override for a genuine re-probe. The committed input
# lives at `parity/probes/toplevel.tsv` (see the long note below, and the
# correction to what that note used to claim), so the default path needs no
# scratch directory at all.
SP="${SP:-}"
# `WT` is REQUIRED, and it used to default to `$SP/wt` -- a convenience that
# pointed at a scratch directory for a variable this script reads SOURCE from
# (`script_for`, the help files) and stamps its revision from. It is the same
# genuinely-exempt input `gen_register.sh` and `gen_pertest.sh` name: a detached
# worktree at the measured revision, re-derivable by `git worktree add` and not
# a file that could live here. Defaulting it to the current checkout would map
# today's source onto a measurement taken at `69d42a7`.
WT="${WT:?set WT -- a detached worktree at the revision the inventory names, not just any checkout}"
OUTDIR="${OUTDIR:-$HERE/..}"
# `git rev-parse --short` picks its own length as the repo grows: the 2026-08-14
# run produced `69d42a7`, the 2026-08-15 run `69d42a7f` for the SAME commit. The
# provenance guard groups these files by their stamp and correctly refused a set
# carrying both, reporting that it "describes no single state of the CLI" -- a
# true statement about a set that was actually consistent. Pinned to 7 so the
# stamp is a property of the commit and not of when it was rendered.
REV="$(cd "$WT" && git rev-parse --short=7 HEAD)"
# NO CLOCK READ, AND THE REASON IS IDEMPOTENCE -- NOT D42. This header used to
# carry `date -u`. A date in the output makes these views NON-IDEMPOTENT ACROSS
# DAYS: re-run tomorrow against the same committed TSV at the same revision and
# all 27 files change, one line each, for no reason -- which destroys the
# byte-identity that is the only content check these artefacts have. Measured in
# the small on 2026-08-15: rendering `cmd-version.md` from probe data captured on
# 08-14 made it assert a measurement that never happened on the day it named. The
# date is also redundant twice over -- git records when the generated file
# landed, and `REV` already identifies the state that was measured.
#
# The removal is right; the reason first written here was not. It cited D42 as
# "you never need the time, so never write one down", and hv has since narrowed
# that twice: RETURNING a time is fine, and reading a clock to make a decision or
# to stamp WHEN A COMMAND RAN into a GENERATED artefact is fine too -- what D42
# forbids is confecting a time into a source document. Stamping a generator's own
# run time into its own output is the permitted case, so D42 never prohibited
# this line and citing it made a good deletion look like compliance. THE ARGUMENT
# THAT ACTUALLY HOLDS IS THE ONE ABOVE, and it is a stronger one: idempotence is a
# property this file can check, whereas a rule cited from memory drifts under it.
TSV="${TSV:-${SP:+$SP/probes/toplevel.tsv}}"
TSV="${TSV:-$HERE/../probes/toplevel.tsv}"

die() { echo "error: $1" >&2; exit 2; }

# THE EXTRACTORS ARE SIBLINGS OF THIS SCRIPT, NOT INHABITANTS OF THE SCRATCH DIR.
# They were resolved as `$SP/extract_verbs.sh` -- `$SP` being a caller-supplied
# THROWAWAY directory that holds the probe corpus -- so running this generator
# required hand-copying two tools out of `tools/` into the scratch first. A large
# part of why nobody re-ran it for a day: the documented invocation did not work
# and the working one was not documented.
#
# `HERE` was already computed at the top of this file and unused for this. The
# correct value was three lines away the whole time.
#
# AND BOTH CALLS CARRIED `2>/dev/null`, so a missing extractor produced an EMPTY
# verb or flag list rather than an error -- a missing measurement presenting as a
# measurement of nothing, on the exact axis this file exists to measure. It never
# fired, because the workaround was always applied; the committed 27 all carry
# content. Latent, not live, and latent only because the wrong thing was tedious
# rather than silent. Now it refuses.
extract() {
  local what="$1" target="$2" tool="$HERE/extract_$1.sh" out
  [ -f "$tool" ] || die "missing extractor: $tool -- refusing rather than emitting an empty $what list, which is indistinguishable from a command that genuinely has none"
  out="$(bash "$tool" "$target")" || die "extract_$what.sh failed on $target -- an extractor that errors must not be read as 'no $what'"
  printf '%s\n' "$out"
}

# Tables are aligned before they land, for the reason gen_dispatch_table.sh
# already gives: the repo formatter aligns markdown tables on save, so a
# generator emitting unaligned ones can never produce the file that is committed.
# Measured 2026-08-15: all 26 inventories differed from a fresh render on every
# table row while their CONTENT was identical, which is a re-derivability defect
# that would defeat any skew check pointed at them, and the reason the skew guard
# lists them as un-re-derivable rather than checking them.
# shellcheck source=/dev/null
. "$HERE/lib_mdfmt.sh" || die "cannot source $HERE/lib_mdfmt.sh -- refusing to emit views that will not survive the formatter"

# THE PROBE TSV IS AN INPUT THIS SCRIPT ONLY EVER READS, AND ITS ABSENCE WAS
# SILENT. Measured 2026-08-15, not suspected:
#
#   awk against a missing file prints to stderr, exits 2, and produces NOTHING
#   -- the `-|-|-|-|-` fallback in probe_row never runs, because END does not
#   execute when the file cannot be opened.
#
# This script runs under `set -uo pipefail` with NO `-e`, so that failure did
# not stop it. It would have carried on and rewritten all 26 `cmd-*.md` files
# with empty probe fields -- overwriting the measurement with a plausible-looking
# husk, at the revision stamp of the good data.
#
# THE TRAP IS THAT EVERY GENERATED FILE INSTRUCTS THE READER TO DO EXACTLY THIS.
# Each `cmd-*.md` header reads "re-run it rather than editing this file", which
# is right advice that silently destroyed the file the day its input went away.
# THE SENTENCE THAT USED TO BE HERE WAS FALSE, AND IT WAS FALSE IN THE MOST
# expensive direction. It read: "`probes/toplevel.tsv` has NEVER been tracked
# (`git log --all -- '*toplevel.tsv'` is empty), so the input for the committed
# 2026-08-14 measurement at `69d42a7` no longer exists anywhere on disk."
#
# It is tracked, at `parity/probes/toplevel.tsv`, since `d9f76c5f` -- **my own
# commit recovering it**, made after discovering the file was on disk the whole
# time and merely untracked. The comment is the FOSSIL of the belief that commit
# refuted, left behind asserting it. Found by dc 2026-08-16, who read it as
# licence, which is exactly what a stale "it does not exist" becomes: the claim
# stopped being a gap and became the stated REASON not to fix things.
#
# git answers questions about HISTORY. "Does it exist" is a question about the
# FILESYSTEM, and an empty `git log --all` was never an answer to it. The
# refusal below still earns its place -- the awk-against-a-missing-file failure
# is real and silent -- but it now guards an input that HAS a committed home,
# and `TSV` defaults there rather than to a scratch directory.
#
# So this refuses instead. It cannot restore the input, and that is the point:
# a missing measurement must present as a REFUSAL to measure, never as a
# measurement of nothing. The reproducibility this file's own header argues for
# -- "a hand-typed list cannot be diffed ... nobody could re-run it" -- is the
# thing the untracked input quietly took away.
[ -f "$TSV" ] || die "probe data not found: $TSV -- this script READS the probe TSV and cannot produce it. Re-run the probe step against a worktree at the target revision first. REFUSING rather than emitting an inventory with empty probe fields, which would overwrite good measurement with a husk carrying a valid-looking revision stamp."
[ "$(awk 'END{print NR}' "$TSV" 2>/dev/null || echo 0)" -gt 1 ] || die "probe data has no rows: $TSV -- header-only or empty. Every command would take the dash fallback in probe_row and the inventory would render as a complete document describing nothing measured."
# NO BACKTICKS IN A DOUBLE-QUOTED DIE MESSAGE. The first version of the line
# above quoted the fallback as `-|-|-|-|-`, and bash ran it as a command
# substitution: five "command not found" lines above the real error, and the
# quoted text silently deleted from the message. An error message that mangles
# itself is worse than a terse one -- it is loudest exactly when someone is
# already debugging. Backticks inside SINGLE quotes are literal and fine, which
# is why the two hits in gen_pertest.sh and gen_register.sh are not this bug.

mkdir -p "$OUTDIR"

# probe_row <label> -> "rc|out|err|outfirst|errfirst"
probe_row() {
  awk -F'\t' -v L="$1" 'NR>1 && $1==L {printf "%s|%s|%s|%s|%s", $2,$3,$4,$5,$6; found=1} END{if(!found) printf "-|-|-|-|-"}' "$TSV"
}

emit_probe_table() {
  local cmd="$1" k r
  printf '| probe | exit | stdout | stderr | first line |\n'
  printf '| ----- | ---- | ------ | ------ | ---------- |\n'
  for k in bare help badflag noproj; do
    r="$(probe_row "$cmd.$k")"
    local rc ob eb of ef
    IFS='|' read -r rc ob eb of ef <<< "$r"
    [ "$rc" = "-" ] && continue
    local first="$of"; [ -z "$first" ] && first="$ef"
    [ -z "$first" ] && first="_(no output)_"
    local label="$k"
    case "$k" in
      bare)    label='`intent '"$cmd"'`' ;;
      help)    label='`--help`' ;;
      badflag) label='unknown flag' ;;
      noproj)  label='outside a project' ;;
    esac
    printf '| %s | %s | %sB | %sB | %s |\n' "$label" "$rc" "$ob" "$eb" "${first//|/\\|}"
  done
}

# gen <command> <script-path> [help-basename]
gen() {
  local cmd="$1" script="$2" helpname="${3:-$1}"
  # `cmd-` prefix, not a bare `<command>.md`. Two of these command names collide
  # with filenames the toolchain treats as special, and macOS matches filenames
  # case-insensitively: a generated `claude.md` is picked up as this directory's
  # CLAUDE.md and injected into every future session as instructions, and
  # `agents.md` shadows the AGENTS.md convention the same way. The prefix makes
  # the collision unconstructible rather than special-casing the two known names.
  local f="$OUTDIR/cmd-$cmd.md"
  local helpfile="$WT/lib/help/$helpname.help.md"
  local verbs flags

  # Three commands cannot be read off a single script's dispatch:
  #   ac/at  -- intent_acceptance dispatches on the NOUN first (ac|at), so the
  #             extractor returns "ac at" rather than either verb set. The real
  #             verbs are the inner case at intent_acceptance:1330 and :1343.
  #   claude -- dispatched in bin/intent:79-118, not in any one plugin script.
  # Each is stated explicitly and verified by invocation like every other row.
  case "$cmd" in
    ac)     verbs="list status satisfy gate descope withdraw rescope reinstate" ;;
    at)     verbs="list lint red green na done notdone" ;;
    claude) verbs="subagents skills upgrade prime rules hook ws start" ;;
    # `version` is a GLOBAL_COMMANDS arm inside bin/intent, not a script of its
    # own, so running the extractors over bin/intent would return the whole
    # top-level dispatch as `version`s subcommands and every global flag as its
    # flags. Stated empty, and true: the dispatch table records no args and no
    # flags for it, and all four probes print the same 22-byte line at exit 0.
    version) verbs=""; flags="" ;;
    *)      verbs="$(extract verbs "$script" | grep -vE '^(help|--help|-h)$' | tr '\n' ' ')" ;;
  esac
  [ "$cmd" = version ] || flags="$(extract flags "$script" | tr '\n' ' ')"

  {
    printf '# Parity inventory: `intent %s`\n\n' "$cmd"
    # "Measured at <rev>, <date>" read as one claim and the date was the RENDER
    # date. Splitting the two was the first fix and keeping the date was the
    # wrong half of it -- see the D42 note above. What a reader needs to
    # re-derive this is the revision and the input, and both are named here.
    printf '> Measured at `%s` from `parity/probes/toplevel.tsv`. Generated by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.\n\n' "$REV"

    # printf '- ...' would be parsed as an option: a leading dash in the FORMAT
    # string makes bash's builtin reject the whole call. Route these through %s.
    printf '%s\n' "- **v2 source**: \`${script#$WT/}\`"
    if [ -f "$helpfile" ]; then
      printf '%s\n' "- **help file**: \`lib/help/$helpname.help.md\`"
    else
      printf '%s\n' "- **help file**: none -- \`intent help $cmd\` falls through to the \"no help available\" path (\`bin/intent_help:37\`)"
    fi

    if [ -n "${verbs// /}" ]; then
      printf '\n## Subcommands\n\n'
      printf 'Taken from the main dispatch and verified by invocation. `help`/`--help`/`-h` arms are omitted; they are covered in the probe table.\n\n'
      printf '| verb |\n| ---- |\n'
      for v in $verbs; do printf '| `%s` |\n' "$v"; done
    else
      printf '\n## Subcommands\n\nNone -- single-action command (no dispatch `case` on a command variable).\n'
    fi

    if [ -n "${flags// /}" ]; then
      printf '\n## Flags\n\n'
      printf 'Parsed as `case` arms anywhere in the script, including nested arg loops.\n\n'
      printf '| flag |\n| ---- |\n'
      for v in $flags; do printf '| `%s` |\n' "$v"; done
    else
      printf '\n## Flags\n\nNone parsed.\n'
    fi

    printf '\n## Observed behaviour\n\n'
    emit_probe_table "$cmd"
    # NO trailing blank line. `emit_probe_table` already ends its last row with a
    # newline, so this printf added a blank at EOF -- which the repo formatter
    # strips, so the committed file could never equal what this generator
    # produces. Same trap gen_dispatch_table.sh records for its own separators:
    # jq -r and printf both supply the newline, and adding one more is invisible
    # until a formatter disagrees with you about it.
  } > "$f"
  md_align "$f" "$f.aligned" || die "table alignment failed for $f"
  mv "$f.aligned" "$f" || die "aligned $f failed to land"
  printf '  wrote %s\n' "${f#$OUTDIR/}"
}

# THE POPULATION IS DERIVED FROM THE PROBE DATA, NOT LISTED HERE.
#
# It used to be a hand-written spec list of 22 plus four explicit calls. That
# list rendered 26 inventories while the probe matrix held 27 units, so `version`
# was measured on 2026-08-14 and has had no `cmd-version.md` since -- silently,
# because 26 is a number that looks like an answer. The generator's SCOPE was
# authored while its CONTENT was measured, and nothing compared the two (vc,
# 2026-08-15: "a silent 104-of-108 is worse than a loud failure").
#
# Same defect as the entry-level completeness gap in gen_dispatch_table.sh and
# as AC-02.6's hand-maintained table roster: an enumeration that has to be
# updated by the same act that invalidates it, and that act is never the act
# anyone is thinking about. Derive the population from something that must be
# complete -- here, the measurement itself -- and a unit cannot go missing
# without the run saying so.
PROBED="$(awk -F'\t' 'NR>1{split($1,a,"."); print a[1]}' "$TSV" | sort -u)"
[ -n "$PROBED" ] || die "no command labels in $TSV -- refusing rather than rendering an empty inventory"

# The command -> script mapping is still authored, because it CANNOT be derived:
# a probe label records what was invoked, never which file implements it. That is
# fine as long as it is a LOOKUP over a derived population rather than the
# population itself -- a command with no mapping now refuses, where before it
# simply never appeared.
script_for() {
  case "$1" in
    ac|at)     printf '%s' "$WT/bin/intent_acceptance" ;;
    agents)    printf '%s' "$WT/intent/plugins/agents/bin/intent_agents" ;;
    claude)    printf '%s' "$WT/intent/plugins/claude/bin/intent_claude_rules" ;;
    version)   printf '%s' "$WT/bin/intent" ;;
    st_zero)   printf '%s' "$WT/bin/intent_st_zero" ;;
    *)         [ -f "$WT/bin/intent_$1" ] && printf '%s' "$WT/bin/intent_$1" ;;
  esac
}

# TWO PASSES. Validating as it generates would leave a half-written OUTDIR on
# refusal -- 20 fresh inventories beside 6 stale ones, all carrying the same
# revision stamp, which is the husk failure this file already refuses at its
# input and would have reintroduced at its output.
UNMAPPED=""
for c in $PROBED; do
  [ -n "$(script_for "$c")" ] || UNMAPPED="$UNMAPPED $c"
done
[ -z "$UNMAPPED" ] || die "probe data covers command(s) with no source mapping:$UNMAPPED -- add them to script_for or explain why a measured command has no implementation. REFUSING rather than skipping, because skipping is how \`version\` stayed invisible through 26 renders."

for c in $PROBED; do
  gen "$c" "$(script_for "$c")"
done

# FORMATTER FIXED POINT. A generator whose output the repo formatter rewrites can
# never produce the committed file, so every skew check pointed at it reports a
# difference forever and the artefacts get declared un-re-derivable instead --
# which is how these 27 spent a day being guarded by their stamp alone. Caught
# here rather than trusted: prettier ran and stripped a trailing blank line, and
# nothing else in the toolchain would have said so.
if command -v npx >/dev/null 2>&1; then
  for f in "$OUTDIR"/cmd-*.md; do
    cp "$f" "$f.fpcheck" || die "cannot copy $f for the fixed-point check"
    npx --no-install prettier --write "$f.fpcheck" >/dev/null 2>&1 || { rm -f "$f.fpcheck"; continue; }
    if ! diff -q "$f" "$f.fpcheck" >/dev/null 2>&1; then
      diff "$f" "$f.fpcheck" | head -6 >&2
      rm -f "$f.fpcheck"
      die "the formatter rewrites $(basename "$f") -- this generator cannot produce the file that gets committed, so no skew check over these will ever pass"
    fi
    rm -f "$f.fpcheck"
  done
fi

printf 'ok: %s inventories from %s probed unit(s) in %s\n' \
  "$(printf '%s\n' "$PROBED" | wc -l | tr -d ' ')" \
  "$(printf '%s\n' "$PROBED" | wc -l | tr -d ' ')" "$(basename "$TSV")" >&2
# `${TSV#$SP/}` was here to shorten the path for display, and it broke the
# moment `SP` became optional: with SP empty the pattern is `/`, so it stripped
# the LEADING SLASH and printed an absolute path as a relative one, `/../` and
# all. A prefix-strip whose prefix can be empty strips whatever the delimiter
# alone matches -- silently, and only in the display.
