#!/usr/bin/env bats
# Issue 0677: every trope the catalogue marks `detection: automated` is run
# under the one program IN-PR-STYLE-004 names, on a line its pattern must
# match and a line it must not. The patterns are written in perl's dialect
# ((?i), \b, \s inside a bracket), and grep engines read them three ways, so a
# verdict is only a property of the rule and the file under one engine.

load "../lib/test_helper.bash"

CATALOGUE="${INTENT_PROJECT_ROOT}/intent/plugins/claude/skills/in-detrope/data/trope-catalog.md"
PROBES="${INTENT_PROJECT_ROOT}/tests/fixtures/tropes/probes.tsv"

# "ID<TAB>pattern" for every automated trope.
automated_patterns() {
  /usr/bin/perl -ne '
    $id = $1 if /^id:\s*(\S+)/;
    $det = $1 if /^detection:\s*(\S+)/;
    print "$id\t$1\n" if /^- \*\*Regex\*\*: `(.*)`\s*$/ && $det eq "automated";
  ' "$CATALOGUE"
}

matches() {
  local pattern=$1 line=$2 file="$BATS_TEST_TMPDIR/probe.txt"
  printf '%s\n' "$line" > "$file"
  LC_ALL=C.UTF-8 /usr/bin/perl -CSDA -ne "$(trope_recipe_program)" "$pattern" "$file" | grep -c . || true
}

@test "the rule names one program, and this test reads it from the rule" {
  program="$(trope_recipe_program)"
  [ -n "$program" ] || fail "IN-PR-STYLE-004 names no perl program"
  # shellcheck disable=SC2016 # the literal perl text, not an expansion
  [[ "$program" == *'qr/$re/i'* ]] || fail "the program read from the rule is not the matcher: $program"
}

@test "the program sees FMT-02's space form that a POSIX bracket misses" {
  pattern="$(automated_patterns | awk -F '\t' '$1 == "FMT-02" { print $2 }')"
  [ -n "$pattern" ] || fail "the catalogue has no automated FMT-02"
  [ "$(matches "$pattern" '- **Bold lead** followed by a space')" = "1" ] || fail "FMT-02 missed the space form"
}

@test "every automated trope has a probe row, and every row names one" {
  missing=""
  while IFS=$'\t' read -r id _; do
    grep -q "^${id}"$'\t' "$PROBES" || missing="$missing $id"
  done < <(automated_patterns)
  [ -z "$missing" ] || fail "automated tropes with no probe row:$missing"
  stale=""
  while IFS=$'\t' read -r id _; do
    case "$id" in "#"*|"") continue ;; esac
    automated_patterns | cut -f1 | grep -qx "$id" || stale="$stale $id"
  done < "$PROBES"
  [ -z "$stale" ] || fail "probe rows naming no automated trope:$stale"
}

@test "every automated pattern matches its line and passes its control" {
  ran=0
  wrong=""
  while IFS=$'\t' read -r id pattern; do
    row="$(grep "^${id}"$'\t' "$PROBES")"
    yes="$(printf '%s' "$row" | cut -f2)"
    no="$(printf '%s' "$row" | cut -f3)"
    [ "$(matches "$pattern" "$yes")" = "1" ] || wrong="$wrong $id(missed)"
    [ "$(matches "$pattern" "$no")" = "0" ] || wrong="$wrong $id(false-hit)"
    ran=$((ran + 1))
  done < <(automated_patterns)
  [ "$ran" -eq "$(automated_patterns | grep -c .)" ] || fail "ran $ran of the catalogue's automated patterns"
  [ "$ran" -gt 0 ] || fail "the catalogue yielded no automated pattern, so nothing was tested"
  [ -z "$wrong" ] || fail "patterns that read wrongly under the named engine:$wrong"
}
