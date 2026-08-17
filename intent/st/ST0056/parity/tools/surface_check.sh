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

# `PARITY_DIR`, not `SP` -- the house name, taken from drift_check.sh which had
# it right. This file said `SP`, and `SP` means "the scratch directory this run
# owns" in six sibling tools that share this directory and get read together.
# Same name, two meanings, one directory: harmless today because nothing here
# reads it as a scratchpad, and precisely the setup where the next person to add
# a line does.
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PARITY_DIR="$(cd "$HERE/.." && pwd)"
REPO_ROOT="$(cd "$PARITY_DIR/../../../.." && pwd)"
DEFAULT_TABLE="$REPO_ROOT/surface/dispatch-table.json"
TABLE="${TABLE:-$DEFAULT_TABLE}"
BIN="${BIN:-$REPO_ROOT/native/rust/target/release/intent}"

die() { echo "error: $1" >&2; exit 2; }

[ -f "$TABLE" ] || die "no dispatch table at $TABLE"

# The four populations have ONE home -- `.populations` in the table, read by
# `lib_surface.sh` (issue 0037). This script previously spelled the walk inline
# FOUR times, twice with the both-predicates retire filter written out by hand,
# agreeing with the library only by coincidence.
#
# BOUND TO **THIS** SCRIPT'S TABLE, and that is not decoration. `TABLE` is
# overridable by the caller and the library resolves its own default; without
# this line a `TABLE=/some/other.json` run would CHECK one file and ENUMERATE
# another, silently, and report the result as if both were the same file. That is
# the same shape as the defect this consolidation exists to end.
DISPATCH_TABLE="$TABLE"
. "$HERE/lib_surface.sh"
[ -x "$BIN" ] || die "no v3 binary at $BIN -- build it first (\`int build cli\`). Refusing rather than reporting a clean surface: an absent binary and a correct one are not the same result, and only one of them is worth printing."

# --- staleness -------------------------------------------------------------
# A STALE BINARY IS AN INABILITY TO MEASURE, NOT A MEASUREMENT, so it refuses
# here alongside the absent one rather than reporting.
#
# THIS COST A REAL FALSE REPORT AND THAT IS WHY IT EXISTS. On 2026-08-15 this
# script reported ARITY and MISSING findings that cc had fixed and pushed 14
# minutes before the binary on disk was built. The output was indistinguishable
# from a genuine regression, and the only thing that stopped it reaching cc as
# one was noticing the mtime by hand. **A stale binary does not fail loudly --
# it produces a plausible, well-formatted, entirely wrong report**, and the
# findings it invents are precisely the ones somebody just fixed, so it argues
# hardest exactly when it is most wrong.
#
# `find -newer` rather than `stat`: BSD and GNU `stat` take different format
# flags, and the one thing this check must not do is fail differently on the
# platform it is not being run on.
# THE OVERRIDDEN TABLE IS DELIBERATELY EXCLUDED, and that is a correction to
# this check rather than an exemption from it. `TABLE` is env-overridable for
# exactly one purpose -- probing the binary against a VARIANT table, which is how
# the flag half gets mutation-tested -- so a variant is a table the binary was
# never built from BY CONSTRUCTION, and asking whether the binary postdates it is
# the wrong question. Measured the moment it mattered: the first mutation run
# after this refusal landed was refused by it, on a synthetic table written
# seconds earlier. A check that blocks its own mutation test makes "green"
# unfalsifiable, which is the class this whole file exists to fight.
#
# The SOURCE half is checked unconditionally, because a binary older than
# `spine.rs` is stale no matter which table it is being asked about.
STALE_INPUTS="$REPO_ROOT/native/rust/crates/intent-cli/src"
[ "$TABLE" = "$DEFAULT_TABLE" ] && STALE_INPUTS="$TABLE $STALE_INPUTS"
# shellcheck disable=SC2086 -- STALE_INPUTS is a deliberate path list
STALE="$(find $STALE_INPUTS -newer "$BIN" -print 2>/dev/null)"
if [ -n "$STALE" ]; then
  die "the binary at $BIN is OLDER than $(printf '%s\n' "$STALE" | wc -l | tr -d ' ') of its own inputs -- rebuild it first (\`int build cli\`, ~30s).
  newest offenders: $(printf '%s\n' "$STALE" | sed "s|$REPO_ROOT/||" | head -3 | tr '\n' ' ')
  Refusing rather than reporting: a stale binary yields a plausible report of findings that are already fixed, which is worse than no report because it reads like a regression."
fi

ROWS="$(surface_declared | grep -c .)"
[ "$ROWS" -gt 0 ] || die "the table declares no rows -- an empty table makes every check below vacuously green"

# --- probe -----------------------------------------------------------------
# `--help` is the only probe safe to run across the whole surface without a
# sandbox: it is side-effect free by construction, which is what makes a sweep
# over 100+ declared paths a measurement rather than a hazard.
PROBED=0; WIRED=0; VIOL=""; NOTE=""

while IFS=$'\t' read -r path disp arity nflags flagjson; do
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
  fseen=0
  while IFS=$'\t' read -r fdisp spellings; do
    [ -n "$fdisp" ] || continue
    fseen=$((fseen + 1))
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

  # COUNT BOTH SIDES. The projection above fixes the cause; this refuses the
  # CLASS, and the class is what actually cost us -- twice now in this one loop,
  # once from a collapsed empty TSV field and once from a broken JSON escape.
  # Both times the inner loop silently iterated zero times and the run reported
  # a clean surface, because **"no flag violated anything" and "no flag was
  # examined" produce identical output.** The row count cannot tell them apart;
  # only the flag count can. A future prose field, a future separator, a future
  # jq version -- none of them get to be silent here again.
  [ "$fseen" = "$nflags" ] || die "\`$path\` declares $nflags flag(s) and only $fseen survived the TSV round-trip, so the rest were never checked against the binary. Refusing rather than reporting: an unexamined flag and a clean flag are the same output, and this loop has now lost flags twice in two different ways."

# THIS ONE IS DELIBERATELY NOT `lib_surface.sh`, AND THE DISTINCTION MATTERS.
# The other three walks in this file were POPULATIONS -- a list of paths -- and
# are now `surface_declared` / `surface_shipped`. This is a ROW QUERY: it needs
# the whole object (`flags`, `args`, arity) and the library emits paths only.
# Rewriting it to enumerate paths and then look each row back up would be more
# code, more lookups, and no fewer homes.
#
# So `[.families[].entries[], .new_surface[]]` survives here as the ROW
# ENUMERATION idiom, which is not the same thing as a population even though it
# is spelled identically -- and that is exactly why it is called out. **The next
# person to need a list of paths will find this line and copy it**, which is how
# `.families[].entries[]` alone got copied five times (issue 0037). If you are
# copying this for a POPULATION, you want `lib_surface.sh` instead.
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
  #
  # PROJECTED TO THE TWO FIELDS THE INNER LOOP READS, AND THAT IS THE FIX FOR A
  # SECOND SILENT-EMPTY OF THE SAME FAMILY, FOUND 2026-08-16. This emitted the
  # WHOLE flag object through `tojson`, prose and all. **`@tsv` escapes
  # backslashes**, so a flag whose prose contains an embedded `\"` came back with
  # its backslashes doubled: jq then read `\\` as one escaped backslash and took
  # the NEXT `"` as the end of the string, and died with `Invalid numeric literal
  # at line 1, column 260`. The inner loop got nothing, so **every flag check on
  # that row vanished -- and the script still printed "the binary and the table
  # agree on every flag of every reachable command."**
  #
  # It was ONE row, `upgrade`, and the prose that broke it was mine, written the
  # day before while correcting the two flags on that row to `pending`. Quoting the
  # basis I was overturning put `\"` in the field. **The row I had just fixed is
  # the row that stopped being checked**, and nothing said so.
  #
  # `disposition` and `spellings` are all the loop consumes; the prose was
  # freight, and freight is where the quoting hazard lives. Carrying less is a
  # smaller fix than escaping better, and it cannot regress the same way.
  | [ .path,
      (.disposition // "-"),
      ((((.args // []) | map(select(.type == "subcommand")) | first) // {}) | .arity // "-"),
      ((.flags // []) | length),
      ((.flags // []) | map({disposition, spellings}) | tojson) ] | @tsv' "$TABLE")

[ "$PROBED" -gt 0 ] || die "probed nothing -- the table has $ROWS rows and the extractor matched none of them, which reports a clean surface by measuring an empty one"

# --- invariants ------------------------------------------------------------
# The section above asks whether the binary offers the FLAGS and ARITY the table
# declares. This one asks whether it OBEYS THE INVARIANTS the table declares,
# which is a different question about the same file and belongs in the same
# script rather than a sibling: one binary resolution, one staleness refusal,
# one report format. Two scripts would be the divergent-copy shape.
#
# WHY IT IS WORTH RUNNING WHEN IT PASSES. Two of these invariants are ratified
# `corrected` -- the table ASSERTS that v3 fixed a v2 defect -- and until now
# nothing anywhere tested that assertion. A `corrected` row is a claim about the
# binary with no test behind it, which is the register-vs-truth axis pointed at
# my own artefact.
#
# THE PROBE MAP IS HAND-WRITTEN AND THAT IS THE HAZARD, so it is guarded. You
# cannot derive "how do I check INV-07" from its prose, so each id is bound to a
# probe by hand -- and a hand-written map silently stops covering the table the
# day someone adds INV-09. So the ids are partitioned into probed and
# declared-unprobeable WITH A REASON, and an id in neither REFUSES. A skip list
# is a promise that something else covers the key, and it is only ever as good
# as that promise.
INV_PROBED="INV-01 INV-02 INV-03 INV-04 INV-06 INV-07 INV-08"
INV_SKIPPED="INV-05"
inv_skip_reason() {
  case "$1" in
    INV-05) echo "a property of v2 SOURCE (an unreachable second call after error), not of observable v3 behaviour -- there is nothing to probe, which is why it is not merely unimplemented" ;;
    *) echo "no reason recorded" ;;
  esac
}

INV_DECLARED="$(jq -r '.invariants[].id' "$TABLE" | sort)"
INV_KNOWN="$(printf '%s %s' "$INV_PROBED" "$INV_SKIPPED" | tr ' ' '\n' | grep -v '^$' | sort)"
INV_UNKNOWN="$(comm -23 <(printf '%s\n' "$INV_DECLARED") <(printf '%s\n' "$INV_KNOWN"))"
INV_PHANTOM="$(comm -13 <(printf '%s\n' "$INV_DECLARED") <(printf '%s\n' "$INV_KNOWN"))"
[ -z "$INV_UNKNOWN" ] || die "the table declares invariant(s) this check has no probe for and does not skip: $(printf '%s' "$INV_UNKNOWN" | tr '\n' ' ') -- add a probe or add it to INV_SKIPPED with a reason. Refusing rather than reporting on the subset it happens to know: a check that quietly stops covering the table is worse than one that stops running."
[ -z "$INV_PHANTOM" ] || die "this check names invariant(s) the table does not declare: $(printf '%s' "$INV_PHANTOM" | tr '\n' ' ') -- the id was renamed or removed and the probe now measures nothing while still reporting a pass."

# Every SHIPPED path, probed with one bad flag. `--help` is already probed
# above; this needs a FAILING invocation, because five of the six invariants are
# properties of the failure path and are unobservable on success.
#
# BOTH FIELDS, BECAUSE `is_shipped()` READS BOTH -- and this file read only
# `disposition` until 2026-08-16. It was not wrong: `gen_dispatch_table.sh`
# REFUSES a row where `disposition` and `target.state` disagree on `retire`
# (bidirectionally), so the two filters select the same 107 paths today and
# provably must. **The correctness was real and it lived in another file.**
# That is the shape vc named on the `upgrade` retirement -- a check that is
# right because of an invariant it does not state is a check that goes wrong
# silently when the invariant is relaxed, and nothing here would have said so.
# The Rust predicate is `disposition != "retire" && target.state != "retire"`;
# shell cannot call it, so where shell must restate a predicate it restates the
# WHOLE of it. Half a predicate is the half that drifts.
#
# `upgrade` is the standing proof the two fields CAN disagree in general: it is
# `keep` + `deviate`, because whether a v2 command survives and what v3 does
# with it are different questions. `retire` is the one value where they are the
# same fact from two sides, which is exactly why the refusal exists.
INV_N=0; INV_VIOL=""
while IFS= read -r p; do
  INV_N=$((INV_N + 1))
  # shellcheck disable=SC2086 -- $p is a multi-word command path and MUST split
  ihelp_rc=0; $BIN $p --help >/dev/null 2>&1 || ihelp_rc=$?
  iout="$($BIN $p --zzz-not-a-flag 2>/dev/null)"; irc=$?
  ierr="$($BIN $p --zzz-not-a-flag 2>&1 >/dev/null)"
  iline="$(printf '%s' "$ierr" | head -1)"

  [ "$ihelp_rc" = "0" ] || INV_VIOL="$INV_VIOL
  INV-07    \`$p\` -- \`--help\` exits $ihelp_rc; the row is ratified \`corrected\`, which asserts v3 exits 0"
  [ "$irc" = "0" ] && INV_VIOL="$INV_VIOL
  INV-08    \`$p\` -- accepts an unknown flag SILENTLY at exit 0"
  [ "$irc" = "2" ] && INV_VIOL="$INV_VIOL
  INV-02    \`$p\` -- usage error exits 2 (clap's default); the invariant is exit 1"
  case "$irc" in 0|1|2) ;; *) INV_VIOL="$INV_VIOL
  INV-04    \`$p\` -- exit $irc is outside the observed set {0, 1, 2}" ;; esac
  { [ "$irc" != "0" ] && [ -n "$iout" ]; } && INV_VIOL="$INV_VIOL
  INV-06    \`$p\` -- writes to STDOUT on a failing invocation"
  { [ -n "$iline" ] && ! printf '%s' "$iline" | grep -qE '^error: '; } && INV_VIOL="$INV_VIOL
  INV-01    \`$p\` -- first stderr line is not the lowercase \`error: \` voice: $(printf '%s' "$iline" | cut -c1-50)"
done < <(surface_shipped)

[ "$INV_N" -gt 0 ] || die "the invariant sweep probed no paths -- it reports clean invariants by measuring nothing"

# --- INV-03: the project-context gate --------------------------------------
# Runs from OUTSIDE a project, which is why it is a separate sweep: every other
# probe above is indifferent to where it stands, and this one is entirely about
# where it stands.
#
# WHY IT IS BUILT NOW, HAVING BEEN DECLINED AN HOUR AGO. It was skipped on a
# measurement -- only ~5 declared paths reach the gate on a bare invocation,
# because clap's missing-argument error fires first on everything that takes an
# argument, and a 5-member UNIFORMITY check that already passes is a decoration.
# That reasoning was right for the question being asked then. **vc's ruling
# changed the question.** INV-03 is now ratified `corrected`, so the table
# ASSERTS that v3 deliberately speaks a different message from v2 -- and that
# made it the ONLY `corrected` invariant with nothing witnessing its claim,
# which is the exact argument that justified probing INV-06/07/08. A claim about
# the binary with no test behind it is the register-vs-truth axis pointed at my
# own artefact.
#
# So the assertion is NOT "the gate is uniform" (weak, always true). It is
# **"no path emits the v2 form"** -- a direct witness to the correction, which
# fails the moment anyone reintroduces the old wording.
INV3_TMP="$(mktemp -d "${TMPDIR:-/tmp}/surfchk-gate-XXXXXX")" || die "cannot make a scratch directory to probe the project gate from"
trap 'rm -rf "$INV3_TMP"' EXIT
# The scratch dir must not sit inside a project, or every command finds one and
# the gate never fires -- a green measured on the wrong ground.
( cd "$INV3_TMP" && d="$PWD"; while [ "$d" != "/" ]; do [ -f "$d/intent/.config/config.json" ] && exit 1; d="$(dirname "$d")"; done; exit 0 ) \
  || die "the scratch directory $INV3_TMP is INSIDE an Intent project, so the gate would never fire and INV-03 would pass by standing in the wrong place"

INV3_GATED=0
while IFS= read -r p; do
  # shellcheck disable=SC2086 -- $p is a multi-word command path and MUST split
  gerr="$(cd "$INV3_TMP" && $BIN $p 2>&1 >/dev/null)"
  gline="$(printf '%s' "$gerr" | head -1)"
  case "$gline" in
    *"no Intent project found"*) INV3_GATED=$((INV3_GATED + 1)) ;;
    *"not in an Intent project directory"*)
      INV_VIOL="$INV_VIOL
  INV-03    \`$p\` -- emits v2's gate wording; the row is ratified \`corrected\`, which asserts v3 does not" ;;
  esac
done < <(surface_shipped)

# --- report ----------------------------------------------------------------
printf 'surface: probed %d declared commands, %d reachable in this build\n' "$PROBED" "$WIRED"
[ -n "$NOTE" ] && printf '%s\n' "$NOTE"

# The invariant line prints its POPULATION and its SKIPS every run, pass or
# fail. A bare "invariants: ok" is the shape that reassures without informing --
# it reads identically whether it swept 105 paths or none, and the two skips are
# the part a reader most needs to know is NOT covered.
printf 'invariants: %d path(s) probed against %s; NOT checked here: %s\n' \
  "$INV_N" "$(printf '%s' "$INV_PROBED" | tr ' ' ',')" "$(printf '%s' "$INV_SKIPPED" | tr ' ' ',')"
printf '  INV-03: %d of %d path(s) actually reach the project gate outside a project (clap arg errors fire first on the rest)\n' "$INV3_GATED" "$INV_N"
for s in $INV_SKIPPED; do printf '  skipped %s -- %s\n' "$s" "$(inv_skip_reason "$s")"; done
if [ -n "$INV_VIOL" ]; then
  printf '\ninvariants: the binary VIOLATES what the table declares:%s\n' "$INV_VIOL"
else
  printf '  all %s hold across every declared non-retire path.\n' "$(printf '%s' "$INV_PROBED" | wc -w | tr -d ' ')"
fi

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
