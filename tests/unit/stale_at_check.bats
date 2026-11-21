#!/usr/bin/env bats
# intent/st/ST0056/parity/tools/stale_at_check.sh -- a to-write AT row whose
# cited file exists is reported, and the rows are read from the thread canon,
# never from a rendered acceptance.md (ST0096 AC-03.1). A thread's acceptance
# view is an aggregate view that leaves git, so the check must read the same
# rows whether the views are tracked, untracked or absent.
#
# And the one reading of the canon stale_at_check.sh and absent_at_check.sh
# share, lib_at_canon.sh (issue 0733): every unreadable thing refuses with exit
# 2 in BOTH tools, and each refusal is planted here.
#
# Every case runs over a throwaway project holding the tools, their libraries
# and the model's schema at their own paths, and a thread canon the case
# writes, so the verdict is about what each case plants. Every legal value a
# fixture plants is read from that schema through lib_at_canon.sh's own
# vocabulary read, and every illegal one is derived from it; every expected
# count is read from the planted canon with jq, never written down.

load "../lib/test_helper.bash"

TOOLS_REL="intent/st/ST0056/parity/tools"
SCHEMA_REL="schema/thread.schema.json"
TOOLS=(stale_at_check absent_at_check)

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/stale-at-XXXXXX")" && pwd)"
  PROJ="$TEST_TEMP_DIR/proj"
  mkdir -p "$PROJ/$TOOLS_REL" "$PROJ/intent/.config" "$PROJ/intent/.canon/st" "$PROJ/tests" "$PROJ/schema"
  printf '{}\n' >"$PROJ/intent/.config/config.json"
  local f
  for f in stale_at_check.sh absent_at_check.sh lib_at_canon.sh lib_citation.sh; do
    cp "${INTENT_PROJECT_ROOT}/$TOOLS_REL/$f" "$PROJ/$TOOLS_REL/"
  done
  cp "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" "$PROJ/$SCHEMA_REL"
  printf 'present\n' >"$PROJ/tests/present.bats"

  # shellcheck source=/dev/null # the copy setup made of lib_at_canon.sh
  VOCAB="$(cd "$PROJ" && . "$PROJ/$TOOLS_REL/lib_at_canon.sh" && at_canon_vocab && printf '%s' "$AT_CANON_VOCAB")"
  WIP="$(legal ThreadStatus wip)"
  NOT_STARTED="$(legal ThreadStatus not-started)"
  CANCELLED="$(legal ThreadStatus cancelled)"
  TO_WRITE="$(legal AtStatus to-write)"
  GREEN="$(legal AtStatus green)"
  RED="$(legal AtStatus red)"
  TEST_KIND="$(legal AtKind test)"
}

teardown() {
  if [ -n "${TEST_TEMP_DIR:-}" ]; then
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# legal <def> <value> -- <value>, once the schema read holds it under <def>; a
# fixture value the model no longer has fails setup rather than planting it.
legal() {
  jq -e --arg d "$1" --arg x "$2" '.[$d] | index([$x]) | values' <<<"$VOCAB" >/dev/null ||
    { echo "setup: $2 is not a $1 in $SCHEMA_REL" >&2; return 1; }
  printf '%s' "$2"
}

# illegal <def> -- a word the schema read does not hold under <def>, derived
# from one it does.
illegal() {
  jq -r --arg d "$1" '.[$d] as $s | ($s[0] + "x") | select(. as $w | $s | index([$w]) | not)' <<<"$VOCAB"
}

# thread <id> <status> <tests-json> -- writes a thread's canon.
thread() {
  printf '{"status":"%s","tests":%s}\n' "$2" "$3" >"$PROJ/intent/.canon/st/$1.json"
}

# at <id> <status> <file> -- one test AT row as the canon carries it.
at() {
  printf '{"id":"%s","kind":"%s","status":"%s","file":"%s"}' "$1" "$TEST_KIND" "$2" "$3"
}

# live_rows / live_cited / skipped -- the population the planted canon holds.
live_rows() {
  jq -s --arg n "$NOT_STARTED" --arg c "$CANCELLED" '[.[] | select(.status != $n and .status != $c) | (.tests // [])[]] | length' "$PROJ"/intent/.canon/st/*.json
}
live_cited() {
  jq -s --arg n "$NOT_STARTED" --arg c "$CANCELLED" --arg w "$TO_WRITE" '[.[] | select(.status != $n and .status != $c) | (.tests // [])[] | select(.status == $w and .file != "")] | length' "$PROJ"/intent/.canon/st/*.json
}
skipped() {
  jq -s --arg n "$NOT_STARTED" --arg c "$CANCELLED" '[.[] | select(.status == $n or .status == $c)] | length' "$PROJ"/intent/.canon/st/*.json
}

check() {
  run bash "$PROJ/$TOOLS_REL/stale_at_check.sh" "$@"
}

# both <text> -- each tool refuses with exit 2, says <text>, and prints no
# finding: the reader checks everything before either tool sees a row.
both() {
  local tool
  for tool in "${TOOLS[@]}"; do
    run bash "$PROJ/$TOOLS_REL/$tool.sh"
    [ "$status" -eq 2 ] || fail "$tool exited $status, not 2: $output"
    assert_output_contains "$1"
    refute_output_contains "stale: "
    refute_output_contains "absent: "
  done
}

# bait -- a row each tool would report: a to-write row whose file exists, and a
# green row whose file does not.
bait() {
  printf '%s,%s' "$(at AT-99.1 "$TO_WRITE" tests/present.bats)" "$(at AT-99.2 "$GREEN" tests/absent.bats)"
}

@test "with no acceptance.md on disk the rows are read from canon, and a thread with no tests key holds none" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/absent.bats),$(at AT-01.2 "$GREEN" tests/present.bats),$(at AT-01.3 "$TO_WRITE" '')]"
  printf '{"status":"%s"}\n' "$WIP" >"$PROJ/intent/.canon/st/ST0002.json"
  [ -z "$(find "$PROJ/intent" -name acceptance.md)" ]
  check
  assert_success
  assert_output_contains "ok: examined $(live_cited) to-write row(s) with a citation, of $(live_rows) AT row(s)"
}

@test "a to-write row whose cited file exists is reported with its thread" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/present.bats),$(at AT-01.2 "$TO_WRITE" tests/absent.bats)]"
  check
  assert_success
  assert_output_contains "stale: ST0001/AT-01.1"
  assert_output_contains "cites tests/present.bats -- the file EXISTS"
  refute_output_contains "AT-01.2"
}

@test "a citation carrying a line is checked for its file" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/present.bats:12)]"
  check
  assert_success
  assert_output_contains "stale: ST0001/AT-01.1"
}

@test "a rendered view that disagrees with canon is not read" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" tests/present.bats)]"
  mkdir -p "$PROJ/intent/st/ST0001"
  printf -- '- AT-01.1 `tests/present.bats` -- covers -- status: to-write\n' >"$PROJ/intent/st/ST0001/acceptance.md"
  check
  assert_success
  assert_output_contains "examined $(live_cited) to-write row(s)"
  refute_output_contains "stale:"
}

@test "a not-started or cancelled thread is skipped and named on the count line" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/absent.bats)]"
  thread ST0002 "$NOT_STARTED" "[$(at AT-01.1 "$TO_WRITE" tests/present.bats)]"
  thread ST0003 "$CANCELLED" "[$(at AT-01.1 "$TO_WRITE" tests/present.bats)]"
  check
  assert_success
  assert_output_contains "of $(live_rows) AT row(s) (skipped $(skipped) not-started/cancelled thread(s): ST0002($NOT_STARTED) ST0003($CANCELLED))"
  refute_output_contains "stale:"
}

@test "a named thread is read alone, and an unknown one refuses with exit 2" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/present.bats)]"
  thread ST0002 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/absent.bats)]"
  check ST0002
  assert_success
  refute_output_contains "stale:"
  check ST0009
  [ "$status" -eq 2 ]
  assert_output_contains "no thread canon for ST0009"
}

@test "no live thread refuses with exit 2" {
  thread ST0001 "$CANCELLED" "[$(at AT-01.1 "$TO_WRITE" tests/absent.bats)]"
  check
  [ "$status" -eq 2 ]
  assert_output_contains "no live thread canon"
}

@test "absent_at reports a green row whose cited file is not there, and passes one that is" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" tests/present.bats),$(at AT-01.2 "$GREEN" tests/absent.bats)]"
  run bash "$PROJ/$TOOLS_REL/absent_at_check.sh"
  [ "$status" -eq 1 ]
  assert_output_contains "absent: ST0001/AT-01.2 is $GREEN and cites tests/absent.bats"
  refute_output_contains "AT-01.1"
}

# THE REFUSALS, one planted fixture each, driven in both tools (issue 0733).
# Each case is numbered by the item of 0733's measure it plants; item 3's exit
# codes are the two exit-2 cases above, and item 4 (the top level only) is not
# a refusal. Wherever rows are read, a thread of bait sits beside the planted
# defect, and `both` holds that neither tool reports it.

@test "1: no canon directory refuses in both" {
  rmdir "$PROJ/intent/.canon/st"
  both "no thread canon at intent/.canon/st"
}

@test "2: a canon directory with no thread file refuses in both" {
  both "no thread canon file under intent/.canon/st"
}

@test "no jq on PATH refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  mkdir -p "$TEST_TEMP_DIR/nojq"
  ln -s "$(command -v dirname)" "$TEST_TEMP_DIR/nojq/dirname"
  local tool
  for tool in "${TOOLS[@]}"; do
    run env PATH="$TEST_TEMP_DIR/nojq" "$BASH" "$PROJ/$TOOLS_REL/$tool.sh"
    [ "$status" -eq 2 ] || fail "$tool exited $status, not 2: $output"
    assert_output_contains "jq is required to read the thread canon"
  done
}

@test "5: a thread file jq cannot parse refuses in both, and jq's message names it" {
  thread ST0099 "$WIP" "[$(bait)]"
  printf '{"status":"%s","tests":[' "$WIP" >"$PROJ/intent/.canon/st/ST0002.json"
  both "intent/.canon/st/ST0002.json: jq cannot parse it"
  refute_output_contains "ST0099.json: jq cannot parse it"
}

@test "5: a thread file holding no JSON value, or two, refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  : >"$PROJ/intent/.canon/st/ST0002.json"
  both "\"intent/.canon/st/ST0002.json\": holds 0 JSON value(s), not one"
  thread ST0002 "$WIP" "[$(at AT-02.1 "$GREEN" tests/present.bats)]"
  thread ST0002 "$WIP" "[$(at AT-02.1 "$GREEN" tests/present.bats)]"
  cat "$PROJ/intent/.canon/st/ST0002.json" "$PROJ/intent/.canon/st/ST0002.json" >"$TEST_TEMP_DIR/two.json"
  mv "$TEST_TEMP_DIR/two.json" "$PROJ/intent/.canon/st/ST0002.json"
  both "\"intent/.canon/st/ST0002.json\": holds 2 JSON value(s), not one"
  rm "${PROJ:?}/intent/.canon/st/ST0099.json"
  : >"$PROJ/intent/.canon/st/ST0002.json"
  both "the canon pass printed nothing"
}

@test "a thread file whose name holds a tab refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread "$(printf 'ST0002\tx')" "$WIP" "[$(at AT-02.1 "$GREEN" tests/present.bats)]"
  both 'the thread file name holds a control character'
}

@test "6: a thread status that is missing or not a ThreadStatus refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  printf '{"tests":[%s]}\n' "$(at AT-01.1 "$GREEN" tests/present.bats)" >"$PROJ/intent/.canon/st/ST0001.json"
  both "ST0001: thread status null is not a ThreadStatus"
  thread ST0001 "$(illegal ThreadStatus)" "[$(at AT-01.1 "$GREEN" tests/present.bats)]"
  both "ST0001: thread status \"$(illegal ThreadStatus)\" is not a ThreadStatus"
}

@test "7: a tests field that is present and not an array refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" "{\"a\":$(at AT-01.1 "$GREEN" tests/present.bats)}"
  both "ST0001: tests is object, not an array"
  thread ST0001 "$WIP" null
  both "ST0001: tests is null, not an array"
}

@test "a row that is not an object refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" '["AT-01.1"]'
  both "ST0001/?: an AT row is string, not an object"
}

@test "8: a row status that is missing or not an AtStatus refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" "[{\"id\":\"AT-01.1\",\"kind\":\"$TEST_KIND\",\"state\":\"$GREEN\",\"file\":\"tests/present.bats\"}]"
  both "ST0001/AT-01.1: status null is not an AtStatus"
  thread ST0001 "$WIP" "[$(at AT-01.1 "$(illegal AtStatus)" tests/present.bats)]"
  both "ST0001/AT-01.1: status \"$(illegal AtStatus)\" is not an AtStatus"
}

@test "9: a row kind that is missing or not an AtKind refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" "[{\"id\":\"AT-01.1\",\"status\":\"$GREEN\",\"file\":\"tests/present.bats\"}]"
  both "ST0001/AT-01.1: kind null is not an AtKind"
  thread ST0001 "$WIP" "[{\"id\":\"AT-01.1\",\"kind\":\"$(illegal AtKind)\",\"status\":\"$GREEN\",\"file\":\"tests/present.bats\"}]"
  both "ST0001/AT-01.1: kind \"$(illegal AtKind)\" is not an AtKind"
}

@test "10: a row id that is missing, empty, not a string or holds a tab refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" "[{\"id\":7,\"kind\":\"$TEST_KIND\",\"status\":\"$TO_WRITE\",\"file\":\"tests/absent.bats\"}]"
  both "ST0001/7: id 7 is missing, empty, not a string or holds a control character"
  thread ST0001 "$WIP" "[{\"kind\":\"$TEST_KIND\",\"status\":\"$TO_WRITE\",\"file\":\"tests/absent.bats\"}]"
  both "ST0001/null: id null is missing"
  thread ST0001 "$WIP" "[$(at '' "$TO_WRITE" tests/absent.bats)]"
  both "ST0001/\"\": id \"\" is missing"
  thread ST0001 "$WIP" "[$(at 'AT-01\t1' "$TO_WRITE" tests/absent.bats)]"
  both "ST0001/\"AT-01\\t1\": id \"AT-01\\t1\" is missing"
}

@test "11: a row file that is not a string, holds a control character or names only a line refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  thread ST0001 "$WIP" "[{\"id\":\"AT-01.1\",\"kind\":\"$TEST_KIND\",\"status\":\"$GREEN\",\"file\":7}]"
  both "ST0001/AT-01.1: file 7 is neither null nor a string"
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" 'tests/pre\tsent.bats')]"
  both "ST0001/AT-01.1: file \"tests/pre\\tsent.bats\" is neither null nor a string"
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" 'tests/present.bats\u0000')]"
  both "ST0001/AT-01.1: file \"tests/present.bats\\u0000\" is neither null nor a string"
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" ':12')]"
  both "ST0001/AT-01.1: file \":12\" names a line and no file"
}

@test "12: a line of the canon pass the reader cannot read refuses in both" {
  thread ST0099 "$WIP" "[$(bait)]"
  # A jq that prints one stray line after the call that passes --argjson, the
  # canon pass, so the vocabulary read is left as it is.
  mkdir -p "$TEST_TEMP_DIR/shim"
  printf '%s\n' '#!/usr/bin/env bash' "\"$(command -v jq)\" \"\$@\"" 'rc=$?' \
    'case " $* " in *" --argjson "*) printf "STRAY\tline\n" ;; esac' 'exit "$rc"' >"$TEST_TEMP_DIR/shim/jq"
  chmod +x "$TEST_TEMP_DIR/shim/jq"
  local tool
  for tool in "${TOOLS[@]}"; do
    run env PATH="$TEST_TEMP_DIR/shim:$PATH" bash "$PROJ/$TOOLS_REL/$tool.sh"
    [ "$status" -eq 2 ] || fail "$tool exited $status, not 2: $output"
    assert_output_contains "the canon pass printed a line this reader cannot read: STRAY"
    refute_output_contains "stale: "
    refute_output_contains "absent: "
  done
}

@test "thread files holding no AT row refuse in both" {
  thread ST0001 "$WIP" '[]'
  both "no AT rows in"
}

@test "a tool with no project root above it refuses in both" {
  local loose="$TEST_TEMP_DIR/loose/tools" f tool d
  mkdir -p "$loose"
  # The fixture, not the tool, is at fault if a directory above it is a project.
  d="$loose"
  while [ "$d" != / ]; do
    d="$(dirname "$d")"
    [ ! -f "$d/intent/.config/config.json" ] || fail "fixture: $d is an Intent project, so no case here can lack a root"
  done
  for f in stale_at_check.sh absent_at_check.sh lib_at_canon.sh lib_citation.sh; do
    cp "$PROJ/$TOOLS_REL/$f" "$loose/"
  done
  for tool in "${TOOLS[@]}"; do
    run bash "$loose/$tool.sh"
    [ "$status" -eq 2 ] || fail "$tool exited $status, not 2: $output"
    assert_output_contains "no Intent project root above $loose"
  done
}

@test "absent_at refuses when no row is a green or red test row with a citation" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$TO_WRITE" tests/absent.bats)]"
  run bash "$PROJ/$TOOLS_REL/absent_at_check.sh"
  [ "$status" -eq 2 ]
  assert_output_contains "NONE is a green or red test row with a citation"
}

# THE GATE MAY RUN THESE UNDER macOS's /bin/bash 3.2, which has no associative
# array: v1 used one and refused the real canon there. Every other case runs
# the bash on PATH, so this one runs /bin/bash itself, and skips where that is
# bash 4 or later (CI's Linux).
@test "both tools read, skip, report and refuse the same under /bin/bash 3" {
  [ -x /bin/bash ] || skip_other_system "no /bin/bash here"
  local major
  major="$(/bin/bash -c 'echo "${BASH_VERSINFO[0]}"')"
  [ "$major" -lt 4 ] || skip_other_system "/bin/bash here is bash $major, not 3"
  thread ST0001 "$WIP" "[$(bait)]"
  thread ST0002 "$CANCELLED" "[$(at AT-02.1 "$TO_WRITE" tests/present.bats)]"
  run /bin/bash "$PROJ/$TOOLS_REL/stale_at_check.sh"
  assert_success
  assert_output_contains "stale: ST0001/AT-99.1"
  refute_output_contains "ST0002/AT-02.1"
  assert_output_contains "of $(live_rows) AT row(s) (skipped $(skipped) not-started/cancelled thread(s): ST0002($CANCELLED))"
  run /bin/bash "$PROJ/$TOOLS_REL/absent_at_check.sh"
  [ "$status" -eq 1 ]
  assert_output_contains "absent: ST0001/AT-99.2"
  : >"$PROJ/intent/.canon/st/ST0003.json"
  local tool
  for tool in "${TOOLS[@]}"; do
    run /bin/bash "$PROJ/$TOOLS_REL/$tool.sh"
    [ "$status" -eq 2 ] || fail "$tool exited $status under /bin/bash, not 2: $output"
    assert_output_contains "\"intent/.canon/st/ST0003.json\": holds 0 JSON value(s), not one"
  done
}

@test "the vocabulary read refuses in both on a schema with an enum gone or empty, an empty schema or none" {
  thread ST0099 "$WIP" "[$(bait)]"
  jq 'del(.["$defs"].AtKind)' "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" >"$PROJ/$SCHEMA_REL"
  both "no values at .[\"\$defs\"].AtKind (or .definitions.AtKind) in $SCHEMA_REL"
  jq '.["$defs"].AtKind.enum = []' "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" >"$PROJ/$SCHEMA_REL"
  both "no values at .[\"\$defs\"].AtKind"
  : >"$PROJ/$SCHEMA_REL"
  both "$SCHEMA_REL gave no vocabulary for ThreadStatus, AtStatus, AtKind"
  rm "${PROJ:?}/$SCHEMA_REL"
  both "no $SCHEMA_REL to read the canon's vocabulary from"
}

@test "a word a tool judges by that the model no longer has refuses in that tool" {
  thread ST0001 "$WIP" "[$(at AT-01.1 "$GREEN" tests/present.bats)]"
  jq --arg w "$RED" '(.["$defs"].AtStatus.oneOf[] | select(has("enum")) | .enum) -= [$w]' "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" >"$PROJ/$SCHEMA_REL"
  run bash "$PROJ/$TOOLS_REL/absent_at_check.sh"
  [ "$status" -eq 2 ]
  assert_output_contains "this tool judges by the word \"$RED\", which is not a value of AtStatus"
  jq --arg w "$TO_WRITE" '(.["$defs"].AtStatus.oneOf[] | select(has("enum")) | .enum) -= [$w]' "${INTENT_PROJECT_ROOT}/$SCHEMA_REL" >"$PROJ/$SCHEMA_REL"
  check
  [ "$status" -eq 2 ]
  assert_output_contains "this tool judges by the word \"$TO_WRITE\", which is not a value of AtStatus"
}
