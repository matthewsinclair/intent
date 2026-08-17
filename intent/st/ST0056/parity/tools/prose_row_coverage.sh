#!/usr/bin/env bash
# **WHICH `as-observed` ROWS DELEGATE TO A v2 TEST THAT DOES NOT LOOK?**
#
# The contract's headline is "v3 is green when v2's own suite cannot tell the
# difference". vc's qualification (canon at `f1374675`): **cannot tell the
# difference IN THE WAYS IT LOOKS.** A byte no v2 test asserts is a byte where
# the two tools diverge freely while the harness reports parity.
#
# An `as-observed` row is a claim that v3's output equals v2's. Where the row's
# `observed.stdout` is PROSE ("the info.md contents"), the row is not a failed
# assertion -- it is an assertion DELEGATED to the v2 suite, which is what the
# contract intends. **So the question is whether the delegate actually checks.**
#
# `wp show` is the measured case that started this: the row said the output was
# unchanged in kind, v3 printed `status: wip` where v2 printed `status: WIP`,
# and `tests/unit/wp_commands.bats:565` -- "wp show displays WP info.md content"
# -- asserts the title and the objective and never the status line. Four
# mechanisms silent: the row's prose, the test pinning the wrong spelling, the
# read-back rule (not yet adopted), and the v2 suite nominated as the certifier.
#
# THIS TOOL DOES NOT DECIDE WHETHER COVERAGE IS ADEQUATE. It reports three
# states per row and refuses to collapse them, because "no test at all" and "a
# test that only checks exit status" fail for different reasons and want
# different fixes.
#
#   uncovered  -- no v2 test invokes the command at all
#   status     -- tests invoke it and assert only success/failure, never output
#   output     -- at least one test makes an assertion about the output bytes
#
# **`output` is NOT a pass.** It says a test looks at some bytes, not that it
# looks at the bytes the row describes -- `wp commands` lands in `output` on the
# strength of asserting a title. Narrowing that is a read, not a grep, and this
# tool refuses to pretend otherwise rather than reporting a number that would be
# believed. The three counts bound the problem; they do not solve it.
set -uo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
TABLE="$ROOT/surface/dispatch-table.json"
TESTS="$ROOT/tests"

[ -f "$TABLE" ] || { echo "error: no dispatch table at $TABLE" >&2; exit 2; }
[ -d "$TESTS" ] || { echo "error: no test estate at $TESTS" >&2; exit 2; }

# Rows claiming output parity with v2, split by whether the row states a literal.
# The split matters: a literal row can be asserted directly from the table, a
# prose row can only delegate.
rows_json() {
  jq -r '
    [ .families[].entries[]
      | select(.target.state == "as-observed")
      | { path, stdout: (.observed.stdout // "") } ]
    | .[]
    | [ .path,
        (if (.stdout | test("[:`]|->")) then "literal" else "prose" end),
        .stdout ]
    | @tsv
  ' "$TABLE"
}

# Every bats block that invokes `intent <path...>`, printed as file:line.
# Matches the command with a word boundary after it so `st show` does not also
# claim `st show_zero`.
invocations() {
  local path="$1" pat
  pat="intent[[:space:]]+${path}([[:space:]]|\$|\")"
  grep -rEn "$pat" "$TESTS" --include='*.bats' 2>/dev/null || true
}

# Does the @test BLOCK containing an invocation assert on output?
#
# **Block granularity, because file granularity could not discriminate.** The
# first cut of this asked whether the FILE asserted output anywhere and put 58
# of 62 rows in one bucket -- a number that answers "does this file contain any
# output assertion", which is not the question and is nearly always yes. A bats
# file is a flat list of `@test "..." {` blocks closed by `}` at column 0, so
# the enclosing block is findable without parsing the language.
#
# It is still LENIENT by construction and the counts are still a FLOOR: a block
# that asserts SOME output counts, even when the bytes it asserts are not the
# row's. `wp show` is the measured proof -- its block asserts the title and the
# objective, never the status line, and it lands in `output` here.
asserts_output() {
  local hits="$1" f line
  while IFS= read -r hit; do
    [ -n "$hit" ] || continue
    f="${hit%%:*}"; line="${hit#*:}"; line="${line%%:*}"
    awk -v n="$line" '
      /^@test /       { start = NR; body = "" }
      NR >= start     { body = body $0 "\n" }
      /^}/ && NR >= n && start && start <= n {
        if (body ~ /assert_output|assert_line|refute_output|\[\[ "\$output"/) exit 0
        exit 1
      }
    ' "$f" && return 0
  done <<< "$hits"
  return 1
}

declare -a UNCOVERED=() STATUS_ONLY=() OUTPUT=()
lit=0; prose=0

while IFS=$'\t' read -r path kind stdout; do
  [ -n "$path" ] || continue
  [ "$kind" = "literal" ] && lit=$((lit + 1)) || prose=$((prose + 1))

  hits="$(invocations "$path")"
  if [ -z "$hits" ]; then
    UNCOVERED+=("$path	$kind	$stdout")
    continue
  fi
  n="$(printf '%s\n' "$hits" | wc -l | tr -d ' ')"
  if asserts_output "$hits"; then
    OUTPUT+=("$path	$kind	$n")
  else
    STATUS_ONLY+=("$path	$kind	$n")
  fi
done < <(rows_json)

total=$((lit + prose))
echo "as-observed rows: $total  ($lit literal, $prose prose)"
echo

# Takes the bucket NAME, not its expansion: `"${arr[@]:-}"` yields one empty
# string for an empty array, so an empty bucket printed as size 1 while its
# listing showed nothing. The count and the listing disagreed and only the
# listing was honest.
emit() {
  local label="$1" name="$2"
  local -a arr=()
  eval "arr=(\"\${${name}[@]}\")"
  echo "== $label: ${#arr[@]} =="
  if [ "${#arr[@]}" -eq 0 ]; then echo "  (none)"; echo; return; fi
  printf '  %s\n' "${arr[@]}" | sed 's/\t/  |  /g'
  echo
}

emit "UNCOVERED -- no v2 test invokes the command" UNCOVERED
emit "STATUS-ONLY -- invoked, but no asserting block" STATUS_ONLY
echo "== OUTPUT -- some invoking block asserts some output: ${#OUTPUT[@]} =="
echo "  (NOT a coverage result -- see below)"
echo

gap=$(( ${#UNCOVERED[@]} + ${#STATUS_ONLY[@]} ))
echo "MECHANICALLY CERTAIN: $gap of $total as-observed rows have no v2 test that invokes them at all."
echo
echo "AND THAT IS THE ONLY NUMBER HERE WORTH BELIEVING. The other ${#OUTPUT[@]} are UNANSWERED,"
echo "not covered. Across the estate 702 @test blocks invoke intent and 160 of those"
echo "assert no output at all, so \"some invoking block asserted something\" is satisfied by"
echo "almost every command and discriminates almost nothing. \`wp show\` sits in OUTPUT on"
echo "the strength of asserting a title and an objective, and it is the row whose status"
echo "line diverged unnoticed."
echo
echo "THE QUESTION THIS TOOL CANNOT ANSWER -- for each row, does a v2 test assert the"
echo "bytes the ROW describes -- is not a grep. It is the differential: run the v2 suite"
echo "against v3 through INTENT_BIN and find the tests that pass against BOTH binaries"
echo "while the binaries differ. Those are exactly the bytes where v2 and v3 diverge"
echo "freely while the harness reports parity. That harness is AT-00.1"
echo "(tests/conformance/run_v2_suite.bash), it exists, and it is red -- so the number"
echo "vc asked for is already specified work rather than a new instrument."
