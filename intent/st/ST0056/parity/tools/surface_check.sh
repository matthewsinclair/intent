#!/usr/bin/env bash
# surface_check.sh -- does the BINARY agree with the TABLE about the surface?
#
# The dispatch table is the SSOT for the v3 command surface (AC-05.1). Every
# other check in this directory asks whether the table is internally consistent,
# or whether a generated VIEW of it is current. Nothing asked the only question
# that makes it a source of truth rather than a description: does the thing it
# claims to generate actually match it.
#
# THIS GAP HAS ALREADY COST SOMETHING, which is why it is being closed rather
# than noted. INV-07 and six command rows sat at `pending-hv` asking whether v3
# should reproduce v2's non-zero `--help` exit, while the binary had answered
# `corrected` hours earlier -- deliberately, with a source comment saying so.
# The contract said open, the binary said closed, and no check anywhere could
# see the disagreement. It was found by hand, by someone counting the hv queue.
#
# It REPORTS, and deliberately does not refuse. The v3 build is mid-ladder: most
# commands are not wired yet, so a gate here would block every node on work that
# has not started. A guard that must be bypassed is a guard nobody keeps. What it
# refuses on is its own inability to measure -- no binary, no table, nothing
# probed -- because a check that cannot run must say so rather than pass.
set -uo pipefail

SP="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(cd "$SP/../../../.." && pwd)"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
BIN="${BIN:-$REPO_ROOT/native/rust/target/release/intent}"

die() { echo "error: $1" >&2; exit 2; }

[ -f "$TABLE" ] || die "no dispatch table at $TABLE"
[ -x "$BIN" ] || die "no v3 binary at $BIN -- build it first (\`int build cli\`). Refusing rather than reporting a clean surface: an absent binary and a correct one are not the same result, and only one of them is worth printing."

ROWS="$(jq -r '[.families[].entries[], .new_surface[]] | length' "$TABLE")"
[ "$ROWS" -gt 0 ] || die "the table declares no rows -- an empty table makes every check below vacuously green"

# --- probe -----------------------------------------------------------------
# `--help` is the only probe safe to run across the whole surface without a
# sandbox: it is side-effect free by construction, which is what makes a sweep
# over 100+ declared paths a measurement rather than a hazard.
PROBED=0; WIRED=0; VIOL=""; NOTE=""

while IFS=$'\t' read -r path disp arity flagjson; do
  [ -n "$path" ] || continue
  PROBED=$((PROBED + 1))

  # A MALFORMED ROW MUST PRESENT AS A REFUSAL, NEVER AS ZERO FINDINGS. The field
  # shift above was invisible precisely because a row with no flag JSON checks
  # no flags and reports nothing, which is indistinguishable from a clean row.
  case "$flagjson" in
    '['*) ;;
    *) die "row \`$path\` did not yield a flag array (got \"${flagjson:0:40}\") -- the TSV columns have shifted, and a row whose flags cannot be read reports clean rather than broken" ;;
  esac

  out="$($BIN $path --help 2>&1)"; rc=$?

  # Unreachable. For a retired command that is the CORRECT outcome and the only
  # place this script confirms a negative -- `treeindex` being absent is the
  # entry-level disposition working, and worth counting as a pass rather than
  # skipping in silence.
  if [ $rc -ne 0 ] || printf '%s' "$out" | grep -q 'unrecognized subcommand'; then
    if [ "$disp" = "retire" ]; then
      NOTE="$NOTE
  ok(retire): \`$path\` is absent from the surface, as declared"
    fi
    continue
  fi
  WIRED=$((WIRED + 1))

  # --- declared arity vs what clap actually requires -----------------------
  # Read from the USAGE LINE rather than by invoking the command bare, and the
  # difference matters: `--help` is side-effect free and a bare invocation is
  # not. `intent todo` generates `todo.md` when absent, so a sweep that probed
  # bare invocations to measure arity would be writing files to find out whether
  # it was allowed to. clap answers the same question in its own usage string --
  # `<COMMAND>` for a required slot, `[COMMAND]` for an optional one -- so the
  # measurement is free and the hazard never arises.
  if [ "$arity" != "-" ] && [ "$arity" != "null" ]; then
    usage="$(printf '%s' "$out" | grep -m1 '^Usage:')"
    case "$usage" in
      *'<COMMAND>'*) clap_requires=yes ;;
      *'[COMMAND]'*) clap_requires=no ;;
      *)             clap_requires=unknown ;;
    esac
    case "$arity:$clap_requires" in
      '0..1:yes')
        VIOL="$VIOL
  ARITY     \`$path\` -- declared \`arity: \"0..1\"\` (bare invocation legal) and clap REQUIRES a subcommand" ;;
      '1:no')
        VIOL="$VIOL
  ARITY     \`$path\` -- declared \`arity: \"1\"\` (slot must be filled) and clap accepts the bare command" ;;
    esac
  fi

  # A flag is "on the surface" if clap prints any of its spellings in the help
  # block. Matched on the spelling with a word boundary rather than substring:
  # `-v` is a substring of `--verbose`, and a substring test would report the
  # short form present whenever the long one was.
  while IFS=$'\t' read -r fdisp spellings; do
    [ -n "$fdisp" ] || continue
    present=no
    for s in $spellings; do
      if printf '%s' "$out" | grep -qE "(^|[[:space:],])$(printf '%s' "$s" | sed 's/[^a-zA-Z0-9-]/\\&/g')([[:space:],=]|$)"; then
        present=yes; break
      fi
    done
    case "$fdisp:$present" in
      keep:no|intrinsic:no)
        VIOL="$VIOL
  MISSING   \`$path\` $spellings -- declared \`$fdisp\` (ships) and the surface does not offer it" ;;
      retire:yes|pending:yes)
        VIOL="$VIOL
  PRESENT   \`$path\` $spellings -- declared \`$fdisp\` (does not ship) and the surface offers it" ;;
    esac
  done < <(printf '%s' "$flagjson" | jq -r '.[] | "\(.disposition)\t\(.spellings | join(" "))"')

done < <(jq -r '[.families[].entries[], .new_surface[]] | .[]
  # A row qualifies on EITHER having flags or declaring a subcommand slot. The
  # first version selected on flags alone, which silently excluded every
  # flagless family from the arity check -- `lang`, `llm`, `modules` and
  # `agents` all declare `0..1` and carry no flags at all, so the population the
  # check was built to measure was the population it could not see.
  | select(((.flags | length) > 0)
        or (((.args // []) | map(select(.type == "subcommand")) | length) > 0))
  # NO FIELD IS EVER EMPTY -- the `-` placeholders are load-bearing, not tidiness.
  # `read -r a b c d` with `IFS=$'\t'` COLLAPSES an empty field, in bash and zsh
  # alike (verified in both). An absent arity therefore shifted the flag JSON one
  # column left, `flagjson` came back empty, and the inner loop produced NOTHING
  # -- so every flag violation on every row without a subcommand slot vanished in
  # silence. It cost `doctor`, `bootstrap`, `sync`, `ingest` and `fileindex`.
  #
  # THE SHAPE OF THE FAILURE IS THE PART WORTH REMEMBERING. The run reported MORE
  # coverage and FEWER findings at the same time -- 59 probed against 46, 11
  # findings against 13 -- which reads as a better run. It was caught only
  # because the earlier output was still on screen to compare against.
  #
  # And the fix above was written with an apostrophe in this very comment, which
  # closed the single-quoted jq program and broke the script. Same class as the
  # bug being described, one layer up: prose nobody proof-reads for syntax,
  # sitting inside a quoting context.
  | [ .path,
      (.disposition // "-"),
      ((((.args // []) | map(select(.type == "subcommand")) | first) // {}) | .arity // "-"),
      (.flags // [] | tojson) ] | @tsv' "$TABLE")

[ "$PROBED" -gt 0 ] || die "probed nothing -- the table has $ROWS rows and the extractor matched none of them, which reports a clean surface by measuring an empty one"

# --- report ----------------------------------------------------------------
printf 'surface: probed %d declared commands, %d reachable in this build\n' "$PROBED" "$WIRED"
[ -n "$NOTE" ] && printf '%s\n' "$NOTE"

if [ -z "$VIOL" ]; then
  echo "surface: the binary and the table agree on every flag of every reachable command."
  exit 0
fi

printf '\nsurface: the binary and the table DISAGREE:%s\n' "$VIOL"
cat <<'EOF'

These are not defects in the table. Three classes, all in the spine:

PRESENT is the EXP-05 gap. `spine.rs` builds every declared flag on every
shipped entry, so a disposition is honoured at the command level (a retired
command is absent from the surface) and ignored one level down.

MISSING is most often a flag with no long spelling: `spine.rs:152-159` drops it
through a bare `continue` with no diagnostic. A family-level flag is the other
cause -- `build()` attaches a family's own flags only when it has NO verbs, so a
flag declared on a family that has verbs reaches every leaf and not the family.

ARITY is `build()` hardcoding `subcommand_required(true)` for any family with
verbs, against the slot's declared arity. `with_args` gets this right in the
same file, and the comment above it states the rule correctly: `1` means the
slot must be filled, `0..1` means the bare command is legal and does something
of its own. The rule is implemented once properly and once by hand.
EOF
exit 0
