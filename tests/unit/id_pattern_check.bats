#!/usr/bin/env bats
# intent/st/ST0056/parity/tools/id_pattern_check.sh -- the commit gate refuses a
# line that spells the shape of an id unless the census names its file and a
# needle on it, and the census only shrinks (ST0095 AC-12.1). It judges the
# whole index against HEAD, so a row dropped, a site copied or a row moved to
# stand for new sites is refused as well as a new line.
#
# Every case runs over a throwaway repository holding the tool at its own path,
# one census site in bin/site.sh and its row, so the verdict is about what each
# case stages and nothing this repository carries. A line's expected number is
# read from the file the case writes, never written down.

load "../lib/test_helper.bash"

TOOL_REL="intent/st/ST0056/parity/tools/id_pattern_check.sh"
CENSUS_REL="intent/st/ST0056/parity/data/id_patterns.tsv"
NEEDLE='^ST[0-9]+'
SITE="grep -c '^ST[0-9]+' list"

# g -- git in the fixture repository, with the user's signing and hooks off.
g() {
  git -C "$REPO" -c commit.gpgsign=false -c core.hooksPath=/dev/null "$@"
}

setup() {
  TEST_TEMP_DIR="$(cd "$(mktemp -d "${TMPDIR:-/tmp}/id-pattern-XXXXXX")" && pwd)"
  REPO="$TEST_TEMP_DIR/repo"
  git -c init.defaultBranch=main init -q "$REPO"
  g config user.name t
  g config user.email t@t
  g config maintenance.auto false
  mkdir -p "$REPO/$(dirname "$TOOL_REL")" "$REPO/$(dirname "$CENSUS_REL")" "$REPO/bin"
  cp "${INTENT_PROJECT_ROOT}/$TOOL_REL" "$REPO/$TOOL_REL"
  printf '#!/usr/bin/env bash\n%s\n' "$SITE" >"$REPO/bin/site.sh"
  row bin/site.sh "$NEEDLE" >"$REPO/$CENSUS_REL"
  g add -A
  g commit -q -m base
}

teardown() {
  if [ -n "${TEST_TEMP_DIR:-}" ]; then
    rm -rf "$TEST_TEMP_DIR"
  fi
}

# row <file> <needle> -- one census row.
row() {
  printf '%s\t%s\twhat it reads\twhat replaces it\n' "$1" "$2"
}

# append <file> <line> -- adds <line> to <file> in the repository and stages it.
append() {
  mkdir -p "$REPO/$(dirname "$1")"
  printf '%s\n' "$2" >>"$REPO/$1"
  g add -- "$1"
}

# line_of <file> <text> -- the number of the line of <file> that is <text>.
line_of() {
  awk -v t="$2" '$0 == t { print NR; exit }' "$REPO/$1"
}

guard() {
  run bash "$REPO/$TOOL_REL"
}

refuses() {
  [ "$status" -eq 1 ] || fail "the guard exited $status, where a finding is 1: $output"
}

@test "a line that is no pattern for an id passes, and every control arm fires" {
  append bin/site.sh "echo 'see ST0056 and issue 0731'"
  guard
  assert_success
  assert_output_contains "Every arm fired"
}

@test "a new pattern for an id refuses, naming its file and line" {
  local want
  append bin/site.sh "awk '/^AT-[0-9]/' rows"
  want="$(wc -l <"$REPO/bin/site.sh" | tr -d ' ')"
  guard
  refuses
  assert_output_contains "bin/site.sh:${want}:"
  assert_output_contains "the census names no needle on it"
}

@test "a copy of a listed site refuses as growth" {
  append bin/site.sh "$SITE"
  guard
  refuses
  assert_output_contains "needle ${NEEDLE} is on"
}

@test "a listed site moved within its file passes" {
  printf '#!/usr/bin/env bash\necho first\n%s\n' "$SITE" >"$REPO/bin/site.sh"
  g add -- bin/site.sh
  guard
  assert_success
}

@test "a census row whose site has gone refuses, naming the row" {
  printf '#!/usr/bin/env bash\necho routed\n' >"$REPO/bin/site.sh"
  g add -- bin/site.sh
  guard
  refuses
  assert_output_contains "census row bin/site.sh :: ${NEEDLE}"
}

@test "a census row dropped while its site stays refuses" {
  printf '# the census, emptied\n' >"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  guard
  refuses
  assert_output_contains "bin/site.sh:$(line_of bin/site.sh "$SITE"):"
}

@test "a row moved to a new site while the old one stays refuses" {
  append bin/other.sh "$SITE"
  row bin/other.sh "$NEEDLE" >"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  guard
  refuses
  assert_output_contains "bin/site.sh:$(line_of bin/site.sh "$SITE"):"
}

@test "one removed row standing for several new sites refuses as growth" {
  local f
  : >"$REPO/$CENSUS_REL"
  for f in a b c; do
    append "bin/$f.sh" "$SITE"
    row "bin/$f.sh" "$NEEDLE" >>"$REPO/$CENSUS_REL"
  done
  g rm -q -- bin/site.sh
  g add -- "$CENSUS_REL"
  guard
  refuses
  assert_output_contains "needle ${NEEDLE} is on"
}

@test "a site renamed with its row passes" {
  g mv bin/site.sh bin/moved.sh
  row bin/moved.sh "$NEEDLE" >"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  guard
  assert_success
}

@test "a comment line and a file outside the roots pass" {
  append bin/site.sh "# grep -c '^AT-[0-9]' rows, described"
  append tests/fixture.sh "awk '/^AT-[0-9]/' rows"
  guard
  assert_success
}

@test "a new pattern under the parity tools' glob root is found" {
  append intent/st/ST0001/parity/tools/x.sh "awk '/^AT-[0-9]/' rows"
  guard
  refuses
  assert_output_contains "intent/st/ST0001/parity/tools/x.sh:$(line_of intent/st/ST0001/parity/tools/x.sh "awk '/^AT-[0-9]/' rows"):"
}

@test "with no census in the index it gives no verdict" {
  g rm -q --cached -- "$CENSUS_REL"
  guard
  [ "$status" -eq 2 ] || fail "a missing census exited $status, where no verdict is 2"
  assert_output_contains "not in the index"
}

@test "a detector that finds nothing fails the control, and no verdict is offered" {
  sed -i.orig "s/^export ID_PATTERN_RE=.*/export ID_PATTERN_RE='matches-nothing-zz'/" "$REPO/$TOOL_REL"
  rm -f "$REPO/$TOOL_REL.orig"
  guard
  [ "$status" -eq 2 ] || fail "a blind detector exited $status, where no verdict is 2"
  assert_output_contains "CONTROL FAILED"
}

@test "a detector that finds everything fails the control, and no verdict is offered" {
  sed -i.orig "s/^export ID_PATTERN_RE=.*/export ID_PATTERN_RE='.'/" "$REPO/$TOOL_REL"
  rm -f "$REPO/$TOOL_REL.orig"
  guard
  [ "$status" -eq 2 ] || fail "an over-matching detector exited $status, where no verdict is 2"
  assert_output_contains "CONTROL FAILED"
}

# A needle that is part of another file's needle is counted only in the files
# whose rows list it, at HEAD as in the index, so the other file lends it no
# room to grow.
@test "a copy of a short needle's site refuses, though a longer needle elsewhere holds it" {
  append bin/short.sh "grep -oE 'ST[0-9]+' f"
  row bin/short.sh 'ST[0-9]+' >>"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  g commit -q -m "a second site"
  append bin/short.sh "grep -oE 'ST[0-9]+' g"
  guard
  refuses
  assert_output_contains "needle ST[0-9]+ is on"
}

@test "a row added for a site traded out of another file refuses, though no count grew" {
  append bin/site.sh "$SITE"
  g commit -q -m "the site twice"
  printf '#!/usr/bin/env bash\n%s\n' "$SITE" >"$REPO/bin/site.sh"
  g add -- bin/site.sh
  append bin/new.sh "$SITE"
  row bin/new.sh "$NEEDLE" >>"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  guard
  refuses
  assert_output_contains "census row added: bin/new.sh :: ${NEEDLE}"
}

@test "a census row without its four fields gives no verdict" {
  printf 'bin/site.sh\t%s\n' "$NEEDLE" >>"$REPO/$CENSUS_REL"
  g add -- "$CENSUS_REL"
  guard
  [ "$status" -eq 2 ] || fail "a malformed row exited $status, where no verdict is 2"
  assert_output_contains "needs four non-empty tab-separated fields"
}
