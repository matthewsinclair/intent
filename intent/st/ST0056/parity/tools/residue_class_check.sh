#!/usr/bin/env bash
#
# residue_class_check.sh -- does migration.md's residue table match what Phase A can emit?
#
# THE check for drift between the migration contract's residue-class table and
# the `FindingClass` variants the legacy scanner actually constructs. Set
# equality, both directions:
#
#   declared-but-not-emitted -- the contract promises a class the migrator
#     cannot produce, so an operator prepares for something that never comes.
#   emitted-but-not-declared -- the migrator reports a class the contract does
#     not describe, so an operator meets a word the spec cannot explain.
#
# The second direction is why this exists. On 2026-08-16 the table declared six
# classes and `legacy.rs` emitted eight, and the two undeclared ones --
# `field-not-recorded` and `unknown-scope` -- are the ONLY two Intent's own
# canary tree produces. Every instrument reported agreement, because nothing
# compared the two sides.
#
# It reads the IMPLEMENTATION, never a transcription of it. A guard built on a
# second copy of the vocabulary would drift exactly the way the table it guards
# drifted -- which is the whole finding.
#
# Reports; never gates. Exit 0 when the sets agree, 1 when they differ, and 2
# when either side parses EMPTY -- an empty declared set and an empty emitted
# set compare equal to each other, so a grammar change on either side would
# otherwise print the all-clear.

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
# Whether each path was GIVEN to us, recorded here because two lines below
# both are set and the distinction is gone. An explicit override is a
# deliberate act and keeps reading the path it was handed; a default is read
# from the index (see `stage_copy`).
CONTRACT_GIVEN="${CONTRACT:+yes}"
SCANNER_GIVEN="${SCANNER:+yes}"
CONTRACT="${CONTRACT:-$ROOT/intent/st/ST0056/migration.md}"
SCANNER="${SCANNER:-$ROOT/native/rust/crates/intentsvcs/src/legacy.rs}"

for f in "$CONTRACT" "$SCANNER"; do
  if [[ ! -f "$f" ]]; then
    echo "error: cannot read $f" >&2
    exit 2
  fi
done

# **BOTH SIDES ARE READ FROM THE INDEX, NEVER FROM THE WORKING TREE.**
#
# This gates commits, and it read two paths off disk until 2026-08-17. Four
# sessions work one checkout, so a peer's HALF-WRITTEN `legacy.rs` -- a
# constructor typed but its class not yet declared -- froze every node's
# commits on work they had never touched. Measured: cc mid-edit emitting
# `malformed-json` blocked ic's whiteboard commit, and the diagnosis cost
# longer than the fix.
#
# **This is the SECOND instrument in this directory with the defect and the
# first one was fixed two hours earlier** (`runner_roster_check.sh`, found by
# dc). The rule was in hand and was applied to the tool in front of me rather
# than to the class -- which is the same failure dc recorded against their own
# patch and vc against their prose arms, three times in one afternoon.
#
# The purpose is unchanged and the timing is unchanged: a class constructed AND
# STAGED is in the commit's index, so it is still caught on the day it arrives,
# which is the only day anybody can say whether it blocks or carries. What
# stops being caught is a keystroke in someone else's editor.
#
# `git show :<path>` honours `GIT_INDEX_FILE`, and git hands a hook a temporary
# index during a partial commit, so under `--only` this reads HEAD plus the
# committer's own named paths.
#
# An explicit `CONTRACT=` / `SCANNER=` override is a deliberate act by someone
# running this by hand, so it keeps reading the path it was given.
stage_copy() {
  local given="$1" path="$2" rel tmp
  [[ -n "$given" ]] && return 0   # explicit override: read what was asked for
  rel="${path#"$ROOT"/}"
  tmp="$(mktemp)"
  if ! git -C "$ROOT" show ":$rel" >"$tmp" 2>/dev/null; then
    rm -f "$tmp"
    echo "error: $rel is not in the index -- this check judges the commit, so a file the commit does not carry cannot be read" >&2
    exit 2
  fi
  printf '%s' "$tmp"
}

CONTRACT_SRC="$(stage_copy "$CONTRACT_GIVEN" "$CONTRACT")" || exit 2
SCANNER_SRC="$(stage_copy "$SCANNER_GIVEN" "$SCANNER")" || exit 2
[[ -n "$CONTRACT_SRC" ]] && CONTRACT="$CONTRACT_SRC"
[[ -n "$SCANNER_SRC" ]] && SCANNER="$SCANNER_SRC"
trap 'rm -f "$CONTRACT_SRC" "$SCANNER_SRC"' EXIT

# Declared: the kebab-case class names in the residue table's first column.
# The table is the rows between the header and the blank line that ends it.
declared="$(
  awk '
    /^\| Class  *\| Meaning/ { intable = 1; next }
    intable && /^\| *-+ *\|/ { next }
    intable && !/^\|/        { exit }
    intable {
      gsub(/^\| */, ""); sub(/ *\|.*/, "")
      if ($0 ~ /^[a-z][a-z-]*$/) print
    }
  ' "$CONTRACT" | sort -u
)"

# Emitted: FindingClass variants the scanner constructs, in their wire spelling.
# `#[serde(rename_all = "kebab-case")]` on the enum, so CamelCase -> kebab-case.
# `|| true` is load-bearing, not defensive noise. Under `set -euo pipefail` a
# grep that matches NOTHING exits 1, the pipeline inherits it, and the script
# dies HERE -- before the empty-population refusal below can say why. Canaried:
# renaming the constructor exited 1 with no output at all, which reads as "the
# sets differ" and is the silent failure this check exists to catch, in the
# check itself.
emitted="$(
  { grep -ohE 'FindingClass::[A-Za-z]+' "$SCANNER" || true; } \
    | sed 's/FindingClass:://' \
    | sed -E 's/([a-z0-9])([A-Z])/\1-\2/g' \
    | tr '[:upper:]' '[:lower:]' \
    | sort -u
)"

n_declared="$(printf '%s\n' "$declared" | grep -c . || true)"
n_emitted="$(printf '%s\n' "$emitted" | grep -c . || true)"

# The empty-population refusal. Both sides are extracted by pattern, and a
# pattern that stops matching yields an empty set rather than an error -- at
# which point the comparison below succeeds and says so.
if [[ "$n_declared" -eq 0 ]]; then
  echo "error: parsed 0 classes from the residue table in $CONTRACT -- the table's shape moved" >&2
  exit 2
fi
if [[ "$n_emitted" -eq 0 ]]; then
  echo "error: parsed 0 FindingClass variants from $SCANNER -- the constructor spelling moved" >&2
  exit 2
fi

missing="$(comm -13 <(printf '%s\n' "$declared") <(printf '%s\n' "$emitted"))"
extra="$(comm -23 <(printf '%s\n' "$declared") <(printf '%s\n' "$emitted"))"

status=0

if [[ -n "$missing" ]]; then
  status=1
  while read -r c; do
    [[ -n "$c" ]] || continue
    echo "residue-class: $c -- EMITTED by the scanner, NOT DECLARED in migration.md"
    echo "  remedy: add a row to the residue table, or stop emitting the class"
  done <<< "$missing"
fi

if [[ -n "$extra" ]]; then
  status=1
  while read -r c; do
    [[ -n "$c" ]] || continue
    echo "residue-class: $c -- DECLARED in migration.md, NOT EMITTED by the scanner"
    echo "  remedy: implement the class, or retire the row and say why"
  done <<< "$extra"
fi

if [[ "$status" -eq 0 ]]; then
  echo "ok: $n_declared declared class(es) and $n_emitted emitted class(es) agree exactly"
else
  echo "residue-class: $n_declared declared, $n_emitted emitted -- the contract and the migrator disagree"
fi

exit "$status"
