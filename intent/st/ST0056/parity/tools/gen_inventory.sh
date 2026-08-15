#!/bin/bash
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
SP="${SP:?set SP}"
WT="${WT:-$SP/wt}"
OUTDIR="${OUTDIR:?set OUTDIR}"
# `git rev-parse --short` picks its own length as the repo grows: the 2026-08-14
# run produced `69d42a7`, the 2026-08-15 run `69d42a7f` for the SAME commit. The
# provenance guard groups these files by their stamp and correctly refused a set
# carrying both, reporting that it "describes no single state of the CLI" -- a
# true statement about a set that was actually consistent. Pinned to 7 so the
# stamp is a property of the commit and not of when it was rendered.
REV="$(cd "$WT" && git rev-parse --short=7 HEAD)"
RENDERED="$(date -u +%Y-%m-%d)"
TSV="$SP/probes/toplevel.tsv"

die() { echo "error: $1" >&2; exit 2; }

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
# The probe scratch (`$SP`) is a throwaway directory and `probes/toplevel.tsv`
# has NEVER been tracked (`git log --all -- '*toplevel.tsv'` is empty), so the
# input for the committed 2026-08-14 measurement at `69d42a7` no longer exists
# anywhere on disk.
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
    *)      verbs="$(bash "$SP/extract_verbs.sh" "$script" 2>/dev/null | grep -vE '^(help|--help|-h)$' | tr '\n' ' ')" ;;
  esac
  [ "$cmd" = version ] || flags="$(bash "$SP/extract_flags.sh" "$script" 2>/dev/null | tr '\n' ' ')"

  {
    printf '# Parity inventory: `intent %s`\n\n' "$cmd"
    # "Measured at <rev>, <date>" read as one claim and the date was the RENDER
    # date, not the measurement date. Harmless while they coincided; the moment
    # `cmd-version.md` was rendered on 08-15 from probe data captured on 08-14
    # it asserted a measurement that never happened on the day it names. The two
    # facts are now separated and the probe input is named, which is the thing a
    # reader actually needs to re-derive this.
    printf '> Measured at `%s` from `parity/probes/toplevel.tsv`. Rendered %s by `parity/tools/gen_inventory.sh` -- re-run it rather than editing this file.\n\n' "$REV" "$RENDERED"

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
  "$(printf '%s\n' "$PROBED" | wc -l | tr -d ' ')" "${TSV#$SP/}" >&2
