#!/usr/bin/env bash
# implemented_check.sh -- does a row the register says SHIPS actually do
# anything, or does it answer "not implemented yet"?
#
# WHY THIS EXISTS, AND WHY NO SIBLING COVERS IT.
# Every other surface instrument compares the table to the binary's SHAPE.
# `surface_check.sh` probes `--help` and compares flags; `dispatch_ssot.rs`
# compares the command SET in both directions; `read_claim_probe.sh` witnesses
# `read_or_mutate`. All three report AGREEMENT on `intent claude hook` -- a row
# marked `keep` / `as-observed`, so the canon asserts it survives into v3 -- and
# the command is a session lockout. It is present, it is correctly shaped, it
# takes its `<NAME>`, it parses, it answers. It answers exit 2, which Claude
# Code's `UserPromptSubmit` contract reads as BLOCK (issue 0043, confirmed live
# across five arms against Claude Code 2.1.233).
#
# So the register can distinguish DECLARED from RETIRED and cannot distinguish
# WIRED from WIRED-AND-IMPLEMENTED. That gap is what this closes, on vc's
# discriminator (2026-08-16): **a row claiming to ship whose command answers
# EXIT_UNAVAILABLE is not shipping, whatever its shape says.**
#
# IT REPORTS AND NEVER GATES, and that is not timidity. Most of the surface is
# legitimately unwired mid-ladder -- that is what a work-package ladder IS -- so
# a gate here refuses every node's commit for a state everyone already knows.
# The number it prints is the useful thing: how much of what the register
# promises the binary currently delivers.
#
# THE NEEDLE IS DERIVED, NOT WRITTEN DOWN, and its absence REFUSES.
# A hardcoded marker that stops matching turns this whole script green and
# silent -- it would report every row implemented on the day the message was
# reworded, which is the precise failure this ST has now been bitten by seven
# times. So the marker is extracted from the `render.rs` format string that
# PRODUCES it, and a `render.rs` that no longer contains one is an inability to
# measure rather than a clean run.
#
# SAFETY, which is the reason a bare-invocation sweep is defensible at all.
# `unwired()` fires at the dispatch match, BEFORE the project is opened, so an
# unimplemented row cannot reach any code that touches a filesystem. A row that
# IS implemented runs -- so every probe gets its OWN fresh scratch directory and
# a sandboxed `HOME`, the scratch root is refused if it resolves inside an
# Intent project, and a timeout bounds anything that hangs. Four rows are
# excluded BY NAME and printed on every run: a silent cap reads as coverage.
#
# MUTATION-TESTED, THREE ARMS, RUN RATHER THAN INTENDED -- reproduce with the
# commands below, which are the ones that were actually driven:
#
#   A -- `RENDER_RS=<a file with no marker> ...`
#        -> REFUSES, exit 2, "could not derive the unimplemented marker".
#        **This is the arm that matters, because the failure it guards is a
#        GREEN**: a reworded message would otherwise report every row
#        implemented, in the reassuring direction, with no sign anything broke.
#
#   B1 -- `BIN=<a stub that exits 2 with NO marker> ...`
#        -> zero rows matched, so the zero-match refusal fires. Had the
#        classifier keyed on the EXIT CODE it would have reported 98 rows
#        unimplemented instead.
#
#   B2 -- `BIN=<a stub that exits 0 WITH the marker> ...`
#        -> 98 of 98 classified unimplemented.
#
#   B1 and B2 are one proof from two sides: exit 2 alone does not classify, and
#   the marker alone does. That is required rather than stylistic -- **exit 2 is
#   ambiguous by design**, since `require-in-session.sh` returns it when the
#   gate deliberately blocks a prompt, and that is the gate WORKING.
#
# The proofs live here rather than in a commit message on purpose (vc's standing
# correction): a commit message is durable and NOT co-located, and `git log
# --follow` is a step nobody takes before trusting a green.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
ST_DIR="$(cd "$PARITY_DIR/.." && pwd)"
REPO_ROOT="$(cd "$ST_DIR/../../.." && pwd)"

# v3 by default, unlike `read_claim_probe.sh`, and deliberately: that probe asks
# a PARITY question whose basis is v2's behaviour, this one asks how much of v3
# exists. Overridable so the mutation arms can drive it.
BIN="${BIN:-$REPO_ROOT/native/rust/target/release/intent}"
TABLE="${TABLE:-$REPO_ROOT/surface/dispatch-table.json}"
RENDER_RS="${RENDER_RS:-$REPO_ROOT/native/rust/crates/intent-cli/src/render.rs}"
PROBE_TIMEOUT="${PROBE_TIMEOUT:-10}"

die() { echo "error: $1" >&2; exit 2; }

command -v jq >/dev/null 2>&1 || die "jq is required and was not found on PATH"
[ -x "$BIN" ]   || die "no runnable binary at $BIN -- build it with \`bin/int build cli\`; a sweep with nothing to run is an inability to measure, not a pass"
[ -f "$TABLE" ] || die "no dispatch table at $TABLE"

# --- the marker, derived from the code that emits it --------------------------
# `render.rs` builds it as a format string; the literal tail is the stable part
# (the head interpolates the path). Extract that tail rather than restate it.
MARKER="$(sed -n 's/.*\(is a known command that is not implemented yet\).*/\1/p' "$RENDER_RS" | head -n 1)"
[ -n "$MARKER" ] || die "could not derive the unimplemented marker from $RENDER_RS -- the message was reworded and this script would otherwise report every row implemented"

# --- the timeout, resolved the way burn.sh resolves it ------------------------
# GNU `timeout` is not on macOS by default. Run WITHOUT one rather than skip the
# sweep, and say which happened: a sweep that silently drops every row is worse
# than one that might hang on a row nobody expected to.
if command -v timeout >/dev/null 2>&1; then
  TO=(timeout "$PROBE_TIMEOUT")
elif command -v gtimeout >/dev/null 2>&1; then
  TO=(gtimeout "$PROBE_TIMEOUT")
else
  TO=()
fi

# --- rows excluded BY NAME, printed every run ---------------------------------
# Two would never return (`daemon` and `mcp` serve until killed, and the timeout
# would classify a working server as a hang), and two write outside the sandbox
# by design (`claude upgrade` installs into the user's `~/.claude`, `claude
# start` launches a session). Naming them here is the whole point: a cap that is
# not printed reads as coverage.
#
# NEWLINE-separated, not space-separated, because two of the four are TWO-WORD
# PATHS -- a space-delimited list read word-wise would exclude a `claude` family
# and an `upgrade` row that were never named, which is the wrong-separator
# defect in its exact classic form. One home for the list; the predicate reads
# it rather than restating it.
EXCLUDED='daemon
mcp
claude upgrade
claude start'
is_excluded() {
  local p="$1" e
  while IFS= read -r e; do
    [ "$e" = "$p" ] && return 0
  done <<EOF
$EXCLUDED
EOF
  return 1
}

# --- a scratch root, refused if it is inside a live tree ----------------------
SCRATCH="$(mktemp -d)"
[ -n "$SCRATCH" ] && [ -d "$SCRATCH" ] || die "could not create a scratch directory"
cleanup() { [ -n "${SCRATCH:-}" ] && rm -rf "$SCRATCH"; }
trap cleanup EXIT

dir_is_inside_a_project() {
  local d="$1"
  while [ "$d" != "/" ] && [ -n "$d" ]; do
    [ -f "$d/intent/.config/config.json" ] && return 0
    d="$(dirname "$d")"
  done
  return 1
}
dir_is_inside_a_project "$SCRATCH" \
  && die "the scratch directory $SCRATCH resolves inside an Intent project -- refusing to invoke commands in a live tree"

# --- filling a row's declared argument slots ----------------------------------
# A slot carrying `values` is filled from the table. Where those values are
# verbs, a READ-ish one is preferred when the row offers one -- the property
# being measured is a fact about the FAMILY (did dispatch answer at all), which
# is the same whichever verb fills the slot, so there is no reason to drive a
# mutating verb when a listing one is declared beside it.
#
# **A `subcommand` SLOT IS FILLED ONLY ON A LEAF ROW, and this took three goes
# to get right -- each wrong version manufactured findings that looked real.**
# The discriminator is `spine.rs`'s own and nothing else will do: `build()`
# hangs a FAMILY's verbs off it as sibling ENTRIES and reads only the ARITY from
# the slot, while `with_args` expands a LEAF's slot `values` into real
# subcommands. So the slot on `claude` is descriptive and the slot on `claude
# skills` is load-bearing, and they must be filled differently.
#
# Filling every slot from its values produced `intent claude subagents`, which
# then wants a third level. Filling none of them left `claude skills` dying at
# clap. Filling with a dummy produced `unrecognized subcommand IN-PROBE`. **All
# three printed a list of rows that read exactly like defects in the binary.**
#
# Required FLAGS are supplied too, with the row's own declared placeholder token
# stripped of its delimiters. `ac satisfy --evidence` is `required: true`, so a
# positional-only filler left three rows dying at clap and reported as though
# something about them had been measured.
placeholders() {
  jq -r --arg p "$1" '
    ([.families[].entries[], .new_surface[]] | map(select(.path == $p)) | .[0]) as $e
    | (($p | test(" ")) as $is_leaf
       | ($e.args // [])
        | map(
            if .type == "subcommand" then
              (if $is_leaf and ((.values // []) | length > 0)
               then ((.values | map(select(. == "list" or . == "show" or . == "status")) | first)
                      // (.values | first))
               else empty end)
            elif (.values // []) | length > 0 then (.values | first)
            elif (.arity == "1" or .arity == "1..n") then "IN-PROBE"
            else empty
            end
          ))
      + (($e.flags // [])
        | map(select(.disposition == "keep" and .required == true))
        | map(.spellings[0] + (if .value then " IN-PROBE" else "" end)))
    | join(" ")' "$TABLE"
}

# Whether the row is a FAMILY whose verb slot must be filled by a sibling row.
# Such a row cannot be invoked on its own terms at all, so the sweep reports it
# as answered-by-its-verbs rather than pretending to have measured it.
needs_a_sibling_verb() {
  jq -e -r --arg p "$1" '
    ([.families[].entries[], .new_surface[]] | map(select(.path == $p)) | .[0]) as $e
    | (($e.args // []) | map(select(.type == "subcommand" and .arity == "1")) | length > 0)
      and (($p | test(" ")) | not)' "$TABLE" >/dev/null 2>&1
}

# --- clap's own refusals, which mean the probe never reached dispatch ---------
# Reported separately and NEVER folded into the answered count. A row clap turns
# away says nothing about whether it is implemented, and counting it as answered
# is precisely the over-count that hid `claude hook`.
UNREACHED_NEEDLES='were not provided
requires a subcommand
unrecognized subcommand
invalid value
unexpected argument'

# --- the sweep ----------------------------------------------------------------
SHIPPED=0
PROBED=0
UNIMPL=0
SKIPPED=0
UNREACHED=0
BYVERB=0
UNIMPL_LIST=""
UNREACHED_LIST=""
BYVERB_LIST=""

# `.families[].entries[]` is NEVER the population -- it is 104 of 112 rows, and
# the other 8 are the top-level `new_surface` array. Enumerate both.
while IFS= read -r path; do
  [ -n "$path" ] || continue
  SHIPPED=$((SHIPPED + 1))

  if is_excluded "$path"; then
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  if needs_a_sibling_verb "$path"; then
    BYVERB=$((BYVERB + 1))
    BYVERB_LIST="$BYVERB_LIST
  $path"
    continue
  fi

  # A FRESH directory per row, so a row that really does create a project
  # (`init`, `bootstrap`) cannot change what the next row meets. HOME is
  # sandboxed alongside it: v3 resolves its install from `current_exe()`, so
  # nothing legitimate needs the real one, and anything that reaches for it is
  # contained rather than trusted.
  ONE="$SCRATCH/$(printf '%s' "$path" | tr ' /' '__')"
  mkdir -p "$ONE" || die "could not create a per-row scratch directory at $ONE"

  # **THE DECLARED ARGUMENTS ARE SUPPLIED, AND THE FIRST VERSION OF THIS SCRIPT
  # DID NOT SUPPLY THEM -- WHICH MADE IT BLIND TO THE ROW IT WAS BUILT FOR.**
  # `claude hook` takes a required `<NAME>`, so a bare invocation dies in clap
  # at exit 1 and never reaches the dispatch match where `unwired()` lives. The
  # sweep reported 33 unimplemented rows and `claude hook` -- issue 0043's
  # lockout, the entire reason this file exists -- was not among them. A check
  # that cannot see its own motivating case is a decoration, and it read as a
  # clean measurement of 103 rows.
  #
  # Placeholders come from the table's own declarations, so nothing is guessed:
  # a slot with `values` is filled from them, anything else required gets an
  # obviously-fake token. A fake `st-id` fails validation INSIDE the command,
  # which is a REACH and exactly what is being measured -- the question is
  # whether dispatch was entered, never whether the command approved of its
  # arguments.
  ARGV="$(placeholders "$path")"
  # shellcheck disable=SC2086
  OUT="$(cd "$ONE" && HOME="$ONE" "${TO[@]}" "$BIN" $path $ARGV 2>&1)"
  PROBED=$((PROBED + 1))

  # **CLASSIFIED ON THE OUTPUT, NEVER ON THE EXIT CODE.** Exit 2 is ambiguous by
  # design: `require-in-session.sh` returns it when the gate deliberately
  # blocks, which is the gate working. The marker separates "Intent refused on
  # your behalf" from "Intent cannot answer at all"; the number cannot.
  case "$OUT" in
    *"$MARKER"*)
      UNIMPL=$((UNIMPL + 1))
      UNIMPL_LIST="$UNIMPL_LIST
  $path"
      ;;
    *)
      while IFS= read -r needle; do
        [ -n "$needle" ] || continue
        case "$OUT" in
          *"$needle"*)
            UNREACHED=$((UNREACHED + 1))
            UNREACHED_LIST="$UNREACHED_LIST
  $path -- $needle"
            break
            ;;
        esac
      done <<NEEDLES
$UNREACHED_NEEDLES
NEEDLES
      ;;
  esac
done <<EOF
$(jq -r '[.families[].entries[], .new_surface[]]
         | map(select((.disposition != "retire") and (.target.state != "retire")))
         | .[].path' "$TABLE")
EOF

# --- refusals on an inability to measure --------------------------------------
# A missing measurement must present as a REFUSAL TO MEASURE, never as a
# measurement of nothing: "no row was unimplemented" and "no row was examined"
# are the same output, and only a count tells them apart.
[ "$SHIPPED" -gt 0 ] || die "the table yielded no shipped rows -- an empty population compares equal to a clean one"
[ "$PROBED" -gt 0 ]  || die "every shipped row was excluded -- nothing was probed"

# --- the report ---------------------------------------------------------------
# **`PROBED`, NOT `SHIPPED`, AND THE WORD IS THE FIX.** This line read `%d of %d
# shipped rows` while the very next line reported `107 shipped of the table` -- one
# word naming two numbers two lines apart, in a coverage instrument. The probed
# population is shipped MINUS the 4 excluded by name MINUS the 5 family rows their
# verbs answer, and a reader who carries 98 away as the surface size is carrying a
# figure this tool never measured. Same class as the enumerator in issue 0037: a
# label wider than the set under it.
# **ZERO MATCHES IS A REFUSAL, NOT A CLEAN RUN.** Either the marker was reworded
# and every row is now silently counted implemented, or the ladder is finished.
# Both deserve a human; neither is something this script may decide.
[ "$UNIMPL" -gt 0 ] || die "no row matched \`$MARKER\` -- either the message was reworded (and this sweep would report every row implemented) or the surface is now complete. Check which before trusting a green"

printf 'implemented: %d of %d probed rows answered; %d answered `%s`; %d never reached dispatch\n' \
  "$((PROBED - UNIMPL - UNREACHED))" "$PROBED" "$UNIMPL" "$MARKER" "$UNREACHED"
printf '  population: %d shipped of the table, %d excluded by name (%s), %d family rows answered by their verbs\n' \
  "$SHIPPED" "$SKIPPED" "$(printf '%s' "$EXCLUDED" | tr '\n' ',' | sed 's/,/, /g; s/, $//')" "$BYVERB"
[ "${#TO[@]}" -gt 0 ] || printf '  note: no `timeout` binary on PATH -- the sweep ran unbounded\n'

if [ "$UNIMPL" -gt 0 ]; then
  printf '\nDECLARED BUT NOT IMPLEMENTED -- the register says these ship:%s\n' "$UNIMPL_LIST"
  printf '\n  Each answers exit 2, which the pre-commit gate reads as fail-open and\n'
  printf '  Claude Code UserPromptSubmit reads as BLOCK. That is issue 0043, and it is\n'
  printf '  why this list is not merely a progress bar.\n'
fi

if [ "$BYVERB" -gt 0 ]; then
  printf '\nANSWERED BY THEIR VERBS -- family rows whose slot must be filled by a sibling:%s\n' "$BYVERB_LIST"
  printf '\n  clap requires a subcommand on these, so there is no invocation of the family\n'
  printf '  alone to classify. Their implementation state is the state of the verb rows\n'
  printf '  above, which ARE swept -- named here so the arithmetic closes.\n'
fi

if [ "$UNREACHED" -gt 0 ]; then
  printf '\nNEVER REACHED DISPATCH -- this sweep says NOTHING about these rows:%s\n' "$UNREACHED_LIST"
  printf '\n  clap turned the invocation away before the dispatch match, so neither\n'
  printf '  `implemented` nor `unimplemented` was measured. Fix the placeholder the\n'
  printf '  row needs; do not read the absence of a finding here as a pass.\n'
fi

# Reports. Never gates -- see the header.
exit 0
