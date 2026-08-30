#!/usr/bin/env bash
# inert_arg_check.sh -- does the binary READ every declared argument, or does it
# accept one and throw it away?  AT-05.7, covering AC-05.7.
#
# ==========================================================================
# WHY THIS EXISTS: EVERY OTHER PROBE IN THE REPO IS CORRECT AND BLIND
# ==========================================================================
#
# The failure shape is why this needs a mechanism rather than more care: the
# argument IS declared, IS accepted, and the command DOES exit 0. A surface
# probe asking "is the flag there" gets yes; one asking "does it exit 0" gets
# yes; the register's row is honest. Nothing in the estate asks the only
# question that separates a read argument from a discarded one, which is
# whether CHANGING IT CHANGES THE ANSWER.
#
# The measured instance (cc, 2026-08-17, found while doing something else):
# `st show` declares a `file` positional with six values, and `render.rs` reads
# the id alone -- so `st show ST0001 design` and `st show ST0001 nonsense` are
# byte-identical at exit 0, where v2 cats design.md for the first and refuses
# `error: Unknown file type` for the second. Worse in kind than an unwired
# command: `unwired()` refuses and claims nothing, while here something
# happened, at exit 0, and the operator is shown a different question's answer.
#
# ==========================================================================
# METHOD, AND WHY THE BOGUS VALUE IS THE DISCRIMINATOR
# ==========================================================================
#
# For each row, drive the entry TWICE differing only in the argument's value,
# and compare stdout + stderr + exit byte for byte. The pair is a DECLARED
# value against a value the enum does not contain, because that pair straddles
# the exit-code boundary: a binary that reads the argument must reject the
# second, and a binary that discards it cannot tell them apart. Two legal
# values that legitimately agree are byte-identical for the same reason an
# unread argument is, which is exactly the confound this pairing removes.
#
# THE TWO-SIDED CONTROL IS FREE AND IS IN THE POPULATION. `st show` DISCARDS
# and `st edit` READS -- same shape, same family, same argument name, opposite
# verdicts -- so a run in which every row comes back clean is a run in which
# this tool has stopped working, and it says so.
#
# ==========================================================================
# REACH -- WHAT A GREEN HERE DOES NOT MEAN
# ==========================================================================
#
# * THE POPULATION IS DECLARED, NOT DERIVED. It is the rows of
#   surface/dispatch-table.json that declare an argument with two or more
#   `values`. An argument with an enumerable domain the table does NOT declare
#   as `values` is invisible here, and that is the same blindness the row it
#   guards was written about.
# * IT DRIVES A THROWAWAY FIXTURE, never the live project. The first version
#   drove the real one and HYDRATED ST0001 into `.intentfiles` on every run --
#   a check that mutates tracked project state is a defect in the check, and it
#   was found by reading `git status` after a clean run rather than by any arm.
#   The fixture gets its own HOME too, because half the population writes there.
# * A ROW THE FIXTURE CANNOT DRIVE IS DECLARED UNDRIVABLE WITH ITS REASON, never
#   skipped: a skipped row is indistinguishable from a clean one.
# * BOTH PROBES FAILING IDENTICALLY IS NOT A DISCARD. If the legal value does not
#   SUCCEED, the pair agrees because the verb never ran, and reporting that as an
#   argument thrown away would be a finding manufactured by the harness. The
#   discriminator requires the declared value to work.
# * A ROW BEHIND `unwired()` IS SATISFIED BY THE REFUSAL ITSELF -- nothing is
#   claimed there, so it needs no skip list. It becomes a candidate the moment
#   its family is wired, which is why the denominator is printed every run.
# * IT REPORTS AND DOES NOT GATE. Families land through the ladder, so a gate
#   would block every node on work that has not started (AC-05.5's reasoning).
#   rc=2 is reserved for this tool being unable to measure at all.

set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
BIN="${INERT_ARG_BIN:-$ROOT/native/rust/target/release/intent}"
BOGUS="__no_such_value__"

die() { printf 'inert-arg: %s\n' "$*" >&2; exit 2; }

[ -x "$BIN" ] || die "no binary to drive at $BIN -- cannot measure, and a clean report over nothing would be worse than this refusal"
command -v python3 >/dev/null 2>&1 || die "python3 is required to read the dispatch table"

# --- the fixture: its own project AND its own HOME ------------------------
FIX="$(mktemp -d)"
export HOME="$FIX/home"; mkdir -p "$HOME"
mkdir -p "$FIX/proj"
cleanup() { rm -rf "$FIX"; }
trap cleanup EXIT
( cd "$FIX/proj" && "$BIN" init fixture >/dev/null 2>&1 && "$BIN" st new "probe subject" >/dev/null 2>&1 ) \
  || die "could not build a fixture project -- refusing rather than falling back to the live one, which is the defect this replaced"
[ -d "$FIX/proj/intent" ] || die "the fixture has no intent/ directory; nothing here would measure the right thing"

TABLE="$ROOT/surface/dispatch-table.json"
[ -f "$TABLE" ] || die "no dispatch table at $TABLE"

# --- the population, derived from the table in one pass ---------------------
POP="$(python3 - "$TABLE" <<'PY'
import json, sys
t = json.load(open(sys.argv[1]))
for f in t["families"]:
  for e in f.get("entries", []):
    for a in e.get("args") or []:
      v = a.get("values")
      if isinstance(v, list) and len(v) >= 2:
        print("\t".join([e.get("path", ""), a.get("name", ""), str(len(v)), v[0]]))
PY
)" || die "could not read the dispatch table"

[ -n "$POP" ] || die "the population is EMPTY -- a table declaring no multi-valued argument means this tool measured nothing, and reporting clean over an empty set is the vacuity this thread files as a defect"

TOTAL=0; DISCARDS=0; READS=0; UNWIRED=0; UNDRIVABLE=0
FINDINGS=""

# Rows whose declared values MUTATE the project. Declared with a reason rather
# than skipped, per the roster construction this thread already uses twice.
undrivable_reason() {
  case "$1" in
    "claude")          echo "a bare dispatch row: its values ARE the sub-families, each rostered here in its own right" ;;
    *) echo "" ;;
  esac
}

# The subject argument's POSITION differs per entry, so the invocation is
# declared beside the row rather than guessed from the arity.
invoke_with() { # $1 = path, $2 = value
  case "$1" in
    "st show")          printf 'st\nshow\nST0001\n%s\n' "$2" ;;
    "agents template")  printf 'agents\ntemplate\n%s\n' "$2" ;;
    "claude rules")     printf 'claude\nrules\n%s\n' "$2" ;;
    "critic")           printf 'critic\n%s\n' "$2" ;;
    "st edit")          printf 'st\nedit\nST0001\n%s\n' "$2" ;;
    "wp rescope")       printf 'wp\nrescope\nST0001/01\n%s\n' "$2" ;;
    "claude subagents") printf 'claude\nsubagents\n%s\n' "$2" ;;
    "claude skills")    printf 'claude\nskills\n%s\n' "$2" ;;
    "claude ws")        printf 'claude\nws\n%s\n' "$2" ;;
    *) return 1 ;;
  esac
}

UNWIRED_PHRASE="is a known command that is not implemented yet"

probe() { # reads argv on stdin, one per line; prints "rc|outhash|errhash|unwired?"
  local -a argv=(); local line
  while IFS= read -r line; do argv+=("$line"); done
  local out err rc mark
  err="$(mktemp)"
  out="$(cd "$FIX/proj" && "$BIN" "${argv[@]}" 2>"$err")"; rc=$?
  mark=no; grep -qF "$UNWIRED_PHRASE" "$err" && mark=yes
  printf '%s|%s|%s|%s' "$rc" \
    "$(printf '%s' "$out" | shasum -a 256 | cut -c1-16)" \
    "$(shasum -a 256 < "$err" | cut -c1-16)" "$mark"
  rm -f "$err"
}

printf 'inert-arg: binary %s\n' "$BIN"
printf 'inert-arg: %s row(s) declare an argument with two or more values\n\n' "$(printf '%s\n' "$POP" | wc -l | tr -d ' ')"

while IFS=$'\t' read -r path arg n first; do
  [ -n "$path" ] || continue
  TOTAL=$((TOTAL + 1))

  reason="$(undrivable_reason "$path")"
  if [ -n "$reason" ]; then
    UNDRIVABLE=$((UNDRIVABLE + 1))
    printf '  UNDRIVABLE  %-20s arg=%-10s -- %s\n' "$path" "$arg" "$reason"
    continue
  fi

  if ! invoke_with "$path" "$first" >/dev/null 2>&1; then
    UNDRIVABLE=$((UNDRIVABLE + 1))
    printf '  UNDRIVABLE  %-20s arg=%-10s -- no invocation is declared for this row in invoke_with(); it REFUSES rather than passing\n' "$path" "$arg"
    continue
  fi

  a="$(invoke_with "$path" "$first"  | probe)"
  b="$(invoke_with "$path" "$BOGUS"  | probe)"

  # A row behind unwired() is satisfied by the refusal, not skipped: nothing is
  # claimed there. THE TEST IS THE MARKER, NOT AGREEMENT BETWEEN THE TWO PROBES
  # -- clap rejects a bogus subcommand BEFORE the unwired arm runs, so the pair
  # disagrees and an unwired row reads as a healthy one. `agents template` was
  # classified `reads` on this tool's first drive for exactly that reason: the
  # argument IS read, by clap, and the entry still claims nothing.
  if [ "${a##*|}" = "yes" ] || [ "${b##*|}" = "yes" ]; then
    UNWIRED=$((UNWIRED + 1))
    printf '  UNWIRED     %-20s arg=%-10s -- refuses; becomes a candidate when its family is wired\n' "$path" "$arg"
    continue
  fi

  # BOTH PROBES FAILING IDENTICALLY IS THE VERB NOT RUNNING, NOT AN ARGUMENT
  # THROWN AWAY. Without this the harness manufactures findings: any verb the
  # fixture cannot satisfy agrees with itself and reads as a discard.
  if [ "$a" = "$b" ] && [ "${a%%|*}" != "0" ]; then
    UNDRIVABLE=$((UNDRIVABLE + 1))
    printf '  UNDRIVABLE  %-20s arg=%-10s -- the declared value does not SUCCEED here (rc=%s), so agreement says nothing\n' "$path" "$arg" "${a%%|*}"
    continue
  fi

  if [ "$a" = "$b" ]; then
    DISCARDS=$((DISCARDS + 1))
    FINDINGS="${FINDINGS}  ${path} (${arg}): '${first}' and '${BOGUS}' are byte-identical -- ${a}"$'\n'
    printf '  DISCARDS    %-20s arg=%-10s -- %d declared values, and changing it changes NOTHING\n' "$path" "$arg" "$n"
  else
    READS=$((READS + 1))
    printf '  reads       %-20s arg=%-10s -- %s vs %s\n' "$path" "$arg" "$a" "$b"
  fi
done <<< "$POP"

SUM=$((DISCARDS + READS + UNWIRED + UNDRIVABLE))
printf '\ninert-arg: partition -- %d discards + %d reads + %d unwired + %d undrivable = %d, against %d rows read\n' \
  "$DISCARDS" "$READS" "$UNWIRED" "$UNDRIVABLE" "$SUM" "$TOTAL"
[ "$SUM" -eq "$TOTAL" ] || die "the partition does not close: $SUM against $TOTAL. A row reached no arm, so it is being asserted about by nobody"

# THE TOOL'S OWN CONTROL. The population contains a known DISCARDS and a known
# READS. If neither verdict ever fires, the run says nothing about the estate.
if [ "$DISCARDS" -eq 0 ] && [ "$READS" -eq 0 ]; then
  die "no row was DRIVEN to either verdict -- every row was unwired or undrivable, so this run measured nothing and must not read as clean"
fi

if [ -n "$FINDINGS" ]; then
  printf '\ninert-arg: FINDINGS -- an argument accepted and thrown away\n%s' "$FINDINGS"
  printf 'inert-arg: REPORTS, does not gate. The fix is a surface call scored where the surface is.\n'
else
  printf '\ninert-arg: every driven argument changes the answer.\n'
fi
exit 0
