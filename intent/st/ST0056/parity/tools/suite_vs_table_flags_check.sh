#!/bin/bash
# suite_vs_table_flags_check.sh -- does the BATS estate exercise a flag the
# dispatch table has never heard of, and does the table declare flags nothing
# green exercises?
#
# ==========================================================================
# WHY THIS EXISTS: THE SEAM BETWEEN TWO SOUND INSTRUMENTS
# ==========================================================================
#
# Both sides of this question are already measured, correctly, by tools that
# cannot see each other:
#
#   - `extract_flags.sh` parses the flag grammar out of a v2 script. Its only
#     consumer is `gen_inventory.sh`, a GENERATOR -- it renders a view.
#   - `lib_surface.sh` / the `flags[].spellings[]` jq path reads the table's
#     declared grammar. Its consumers are `gen_dispatch_table.sh` and
#     `gen_cut_surface.sh` -- both GENERATORS.
#   - `coverage_map.sh` joins the table to the estate at FAMILY granularity and
#     answers "is a command family exercised by nothing?".
#   - `surface_check.sh` compares the BINARY to the table.
#
# So the estate's flags are extracted, the table's flags are extracted, both are
# rendered for a reader, and NOTHING FORMS A VERDICT ABOUT THE PAIR. A flag the
# suite proves green and the table has never heard of is invisible to every one
# of the instruments above: coverage_map sees the family covered, surface_check
# sees the binary and the table agree about what the table contains, and the two
# generators render exactly what they were handed. **A seam between two sound
# instruments is invisible to both**, which is the whole reason this file exists
# rather than a wider glob on one of them.
#
# ==========================================================================
# THE MEMBERSHIP RULE IS vc's, AND ITS THIRD CLAUSE IS THE ONE THAT COSTS
# ==========================================================================
#
# A flag is NAMED BY THE SUITE when it is passed to the intent binary in an
# invocation the test ASSERTS SUCCEEDS. Three exclusions follow, and only the
# first two fall out of command attribution:
#
#   1. NOT THE INTENT BINARY. `--arg` is jq, `--oneline` / `--patch` /
#      `--porcelain` / `--is-ancestor` / `--exec` / `--no-verify` are git,
#      `--include` is grep, `--separate-stderr` is bats. This is handled
#      STRUCTURALLY rather than by a blocklist: only the segment of the line
#      AFTER the invocation token and BEFORE the next shell operator is read,
#      so `run --separate-stderr run_intent ...` excludes the bats flag by
#      position, and a pipe into grep excludes the grep flag by the same rule.
#      A blocklist would need a new entry every time the estate learns a tool.
#   2. NOT AN INVOCATION. A flag inside a comment is prose.
#   3. NOT A SUCCESS ASSERTION. This is the clause command attribution does not
#      give you. `--bogus` and `--some-arg` ARE passed to the intent binary, in
#      real invocations, and they are excluded by what the test CLAIMS about the
#      result. Without this clause the instrument reports every flag the estate
#      knows to be INVALID as part of the surface.
#
# ==========================================================================
# TWO CLASSES ARE REPORTED AND NEVER RESOLVED (vc)
# ==========================================================================
#
# MIXED -- a block that asserts BOTH success and failure. NOSTATUS -- a block
# that invokes the binary and asserts no status at all. Neither is resolvable
# from the text: which invocation the success attaches to is a judgement about
# intent, and a heuristic resolution is a place to be wrong with no marker on
# it. Both classes are COUNTED and their flags are held out of the verdict, so
# the population the verdict closes over is stated rather than assumed. The
# NOSTATUS class is a suite finding in its own right and is emitted as one.
#
# A `case "$status" in` block is deliberately routed to MIXED: it is a polarity
# decision this instrument cannot read, and calling it SUCCESS would be the
# heuristic resolution the ruling forbids.
#
# ==========================================================================
# WHY THE TABLE SIDE HAS A CONTROL AT ALL
# ==========================================================================
#
# W74a: a naive `grep -oE '"--[a-z-]+"'` over `dispatch-table.json` returns the
# SAME 74 long flags as the correct jq over `flags[].spellings[]` -- and as of
# the day this was written, the same SET, not merely the same count. The table's
# prose fields quote flags constantly, so the grep reads prose; it agrees with
# the correct extraction by coincidence, and that coincidence is not stable.
# **Two extractions agreeing means you have tested neither.** So the table
# extractor is driven against a planted fixture carrying a flag that exists ONLY
# in a prose field: the jq must not see it and the grep must, and the run
# REFUSES if the two agree, because agreement means the control has stopped
# discriminating.
#
# ==========================================================================
# REACH -- WHAT THIS DOES NOT SEE
# ==========================================================================
#
# Declared population: `tests/**/*.bats`. The complement is emitted every run.
# Invocations in setup helpers OUTSIDE any `@test` block are attributed to no
# block and are COUNTED AND NAMED rather than dropped -- a flag reaching the
# binary only from a helper is invisible to the polarity rule, so it is
# reported as a reach limit and not as a clean zero.
#
# Compares DASHED spellings, long and short. The table's one bare-word spelling
# (`help`, a bare arm sharing one clap row with `--help`/`-h`) is excluded BY
# NAME and printed, because a bare word is not a flag and matching it against
# the estate would compare a verb to a grammar.
#
# THE ADJACENT AXIS IS NOT BUILT HERE AND IS NAMED SO NOBODY THINKS IT WAS: a
# table row carries a `v2` field naming its v2 twin, and `extract_flags.sh` can
# read that twin. Joining a row's declared flags to its OWN declared twin is a
# second, stronger witness for the same seam over a different population (v2
# scripts, not tests). It is a separate instrument and it does not exist.
#
# Exit codes: 0 clean, 1 a finding, 2 cannot measure. REPORTS, never gates --
# most of direction B is a legitimate mid-ladder state (v3 flags the v2 estate
# never had), so a gate here would refuse every node commit for the shape of
# the ladder rather than for a defect.

set -u

ME="$(basename "$0")"
die() { printf '%s: CANNOT MEASURE -- %s\n' "$ME" "$*" >&2; exit 2; }

ROOT="$(cd "$(dirname "$0")/../../../../.." 2>/dev/null && pwd)" || die "cannot resolve the repo root from $0"
[ -d "$ROOT/.git" ] || die "resolved root $ROOT is not a git worktree"
cd "$ROOT" || die "cannot enter $ROOT"

TABLE="surface/dispatch-table.json"
SUITE_DIR="tests"
BARE_WORD_EXCLUSIONS="help"

command -v jq >/dev/null 2>&1 || die "jq is not on PATH; the table side cannot be read"
[ -f "$TABLE" ] || die "$TABLE is absent"
[ -d "$SUITE_DIR" ] || die "$SUITE_DIR/ is absent"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/suitetable.XXXXXX")" || die "cannot make a work directory"
trap 'rm -rf "$WORK"' EXIT

# --------------------------------------------------------------------------
# THE TWO EXTRACTORS. Each is a function so the controls drive the SAME code
# the verdict does -- a control that drives a copy proves nothing about the
# instrument, which is the whole lesson of the baseline that read `0 across 0`.
# --------------------------------------------------------------------------

table_flags() {                           # $1 a dispatch-table.json; dashed spellings, sorted
  jq -r '[.. | objects | select(has("flags")) | .flags[]?.spellings[]?]
         | unique | .[] | select(startswith("-"))' "$1" 2>/dev/null | sort -u
}

table_flags_naive() {                     # $1 same file; the W74a WRONG extraction, kept to discriminate
  grep -oE '"--[a-zA-Z0-9][a-zA-Z0-9-]*"' "$1" | tr -d '"' | sort -u
}

AWK_SUITE='
function flush_block(   i, pol) {
  if (!inblock) return
  pol = 0
  if (b_succ) pol += 1
  if (b_fail) pol += 2
  if (b_ninv == 0) { blocks_no_inv++; return }
  blocks_with_inv++
  if (pol == 1)      { cls = "SUCCESS";  blocks_succ++ }
  else if (pol == 2) { cls = "FAILURE";  blocks_fail++ }
  else if (pol == 3) { cls = "MIXED";    blocks_mixed++ }
  else               { cls = "NOSTATUS"; blocks_nostat++ }
  for (i = 1; i <= b_nf; i++) print b_flag[i] "\t" cls "\t" FILENAME ":" b_line
}
function seg_after(line, tok,   p, s) {
  p = index(line, tok)
  if (p == 0) return ""
  s = substr(line, p + length(tok))
  sub(/[ \t]\|.*$/,   "", s)
  sub(/[ \t]&&.*$/,   "", s)
  sub(/[ \t]\|\|.*$/, "", s)
  sub(/;.*$/,         "", s)
  sub(/[ \t]2?>.*$/,  "", s)
  return s
}
FNR == 1 {
  flush_block(); inblock = 0
  ntok = 3; tok[1] = "run_intent"; tok[2] = "$INTENT_BIN"; tok[3] = "${INTENT_BIN}"
  files++
}
/^[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=["'"'"']?\$\{?INTENT_BIN\}?/ {
  a = $0; sub(/^[[:space:]]*/, "", a); sub(/=.*$/, "", a)
  ntok++; tok[ntok] = "$" a
  ntok++; tok[ntok] = "${" a "}"
  aliases++
}
/^@test/ { flush_block(); inblock = 1; b_ninv = 0; b_nf = 0; b_succ = 0; b_fail = 0; b_line = FNR; blocks++; next }
inblock && /^}[[:space:]]*$/ { flush_block(); inblock = 0; next }
{
  line = $0
  if (line ~ /^[[:space:]]*#/) next
  if (!inblock) {
    for (i = 1; i <= ntok; i++)
      if (index(line, tok[i]) > 0 && seg_after(line, tok[i]) != "") { helper_inv++; helper_file[FILENAME] = 1; break }
    next
  }
  if (line ~ /assert_success/)             b_succ = 1
  if (line ~ /\$\{?status\}?" -eq 0 \]/)   b_succ = 1
  if (line ~ /assert_failure/)             b_fail = 1
  if (line ~ /\$\{?status\}?" -ne 0 \]/)   b_fail = 1
  if (line ~ /\$\{?status\}?" -eq [1-9]/)  b_fail = 1
  if (line ~ /case[[:space:]]+"\$status"/) { b_succ = 1; b_fail = 1 }
  for (i = 1; i <= ntok; i++) {
    if (index(line, tok[i]) > 0) {
      s = seg_after(line, tok[i])
      if (s == "") continue
      b_ninv++
      n = split(s, w, /[ \t=]+/)
      for (j = 1; j <= n; j++)
        if (w[j] ~ /^--[a-zA-Z0-9][a-zA-Z0-9-]*$/ || w[j] ~ /^-[a-zA-Z0-9]$/) { b_nf++; b_flag[b_nf] = w[j] }
      break
    }
  }
}
END {
  flush_block()
  hf = 0; for (k in helper_file) hf++
  printf "files\t%d\nblocks\t%d\nwith_inv\t%d\nno_inv\t%d\nsucc\t%d\nfail\t%d\nmixed\t%d\nnostatus\t%d\naliases\t%d\nhelper_inv\t%d\nhelper_files\t%d\n",
         files, blocks, blocks_with_inv, blocks_no_inv, blocks_succ, blocks_fail, blocks_mixed, blocks_nostat, aliases, helper_inv, hf > STATS
}
'

# ONE awk PROCESS OVER THE WHOLE POPULATION, never xargs. The block counters and
# the alias table are per-process state, so a split into two invocations would
# reset them mid-population and the second STATS write would truncate the first
# -- a wrong count that reads exactly like a right one.
suite_flags() {                           # $1 a directory of .bats; writes stats to $2, prints flag<TAB>class<TAB>site
  local d="$1" stats="$2" f
  local files=()
  while IFS= read -r f; do files+=("$f"); done < <(find -L "$d" -name '*.bats' -type f | sort)
  [ "${#files[@]}" -gt 0 ] || return 1
  awk -v STATS="$stats" "$AWK_SUITE" "${files[@]}"
}

# --------------------------------------------------------------------------
# CONTROLS. Every one drives the extractor functions above -- the same code the
# verdict uses -- against planted fixtures, and the run REFUSES if any control
# fails to fire. A control that cannot fail is decoration.
# --------------------------------------------------------------------------

CTL_FAIL=0
CTL_RUN=0
ctl() {                                   # $1 label, $2 expectation met (0/1)
  CTL_RUN=$((CTL_RUN + 1))
  if [ "$2" -eq 0 ]; then
    printf '%s: ok -- control -- %s\n' "$ME" "$1"
  else
    printf '%s: CONTROL FAILED -- %s\n' "$ME" "$1" >&2
    CTL_FAIL=$((CTL_FAIL + 1))
  fi
}
has() { grep -qxF -- "$2" "$1"; }              # $1 file, $2 exact line; `--` or a flag-shaped pattern is read as an option

FIX="$WORK/fixtures"
mkdir -p "$FIX/table" "$FIX/suite" "$FIX/empty"

cat > "$FIX/table/dispatch-table.json" <<'JSON'
{
  "populations": {
    "shipped": [
      {
        "path": "demo",
        "flags": [
          { "spellings": ["--declared", "-d"], "type": "bool" }
        ],
        "help": "--prose-only",
        "note": "a STRING VALUE that is itself a flag, outside spellings[] -- the naive grep must match it and jq must not"
      }
    ]
  },
  "decoy": [ { "spellings": ["--not-under-flags"] } ]
}
JSON

table_flags     "$FIX/table/dispatch-table.json" > "$FIX/t_jq.txt"
table_flags_naive "$FIX/table/dispatch-table.json" > "$FIX/t_grep.txt"

# C1 positive: the declared spelling is read.
ctl "table extractor reads a declared spelling (--declared)" \
    "$(has "$FIX/t_jq.txt" '--declared' && echo 0 || echo 1)"
# C2 W74a: a flag living only in PROSE must be invisible to jq and visible to the grep.
c2=1
if ! has "$FIX/t_jq.txt" '--prose-only' && has "$FIX/t_grep.txt" '--prose-only'; then c2=0; fi
ctl "table extractor ignores a flag that exists only in prose, and the naive grep does NOT (the two disagree, so the control still discriminates)" "$c2"
# C3 the jq path is anchored on `flags`, not on any array named spellings.
ctl "table extractor ignores a spellings[] that is not under flags[]" \
    "$(has "$FIX/t_jq.txt" '--not-under-flags' && echo 1 || echo 0)"

cat > "$FIX/suite/ctl.bats" <<'BATS'
#!/usr/bin/env bats
MYBIN="$INTENT_BIN"

setup_helper() {
  run run_intent st new "from a helper"   # attributed to no block, must be COUNTED
  assert_success
}

# run run_intent nope --commentflag       <- prose, never an invocation

@test "success block" {
  run run_intent st list --succonly | grep --include=nope x
  assert_success
}

@test "failure block" {
  run run_intent st list --failonly
  assert_failure
}

@test "mixed block" {
  run run_intent st list --mixedonly
  assert_success
  run run_intent st list --mixedtwo
  assert_failure
}

@test "no status assertion" {
  run run_intent st list --nostatusonly
  assert_output_contains "x"
}

@test "alias block" {
  run "$MYBIN" st list --aliasonly
  assert_success
}
BATS

suite_flags "$FIX/suite" "$FIX/s_stats.tsv" > "$FIX/s_raw.tsv" || die "the control fixture produced no suite read"
awk -F'\t' '$2=="SUCCESS"  {print $1}' "$FIX/s_raw.tsv" | sort -u > "$FIX/s_succ.txt"
awk -F'\t' '$2=="FAILURE"  {print $1}' "$FIX/s_raw.tsv" | sort -u > "$FIX/s_fail.txt"
awk -F'\t' '$2=="MIXED"    {print $1}' "$FIX/s_raw.tsv" | sort -u > "$FIX/s_mixed.txt"
awk -F'\t' '$2=="NOSTATUS" {print $1}' "$FIX/s_raw.tsv" | sort -u > "$FIX/s_nostat.txt"
fx() { awk -F'\t' -v k="$1" '$1==k {print $2}' "$FIX/s_stats.tsv"; }

ctl "suite extractor takes a flag from a success-asserting invocation (--succonly)" \
    "$(has "$FIX/s_succ.txt" '--succonly' && echo 0 || echo 1)"
ctl "suite extractor reads ONLY the intent segment: the grep flag past the pipe is not captured (--include)" \
    "$(grep -qxF -- '--include' "$FIX/s_raw.tsv" && echo 1 || echo 0)"
ctl "a flag in a comment is prose, never an invocation (--commentflag)" \
    "$(grep -qF -- '--commentflag' "$FIX/s_raw.tsv" && echo 1 || echo 0)"
c_pol=1
if ! has "$FIX/s_succ.txt" '--failonly' && has "$FIX/s_fail.txt" '--failonly'; then c_pol=0; fi
ctl "polarity is two-sided: a failure-asserted flag is EXCLUDED from success and PRESENT in failure (--failonly)" "$c_pol"
c_mix=1
if ! has "$FIX/s_succ.txt" '--mixedonly' && has "$FIX/s_mixed.txt" '--mixedonly'; then c_mix=0; fi
ctl "a block asserting both polarities is reported as MIXED and resolved into neither (--mixedonly)" "$c_mix"
c_ns=1
if ! has "$FIX/s_succ.txt" '--nostatusonly' && has "$FIX/s_nostat.txt" '--nostatusonly'; then c_ns=0; fi
ctl "a block asserting no status at all is reported as NOSTATUS and resolved into neither (--nostatusonly)" "$c_ns"
ctl "the alias arm fires: a flag reaching the binary through a variable assigned from \$INTENT_BIN is captured (--aliasonly)" \
    "$(has "$FIX/s_succ.txt" '--aliasonly' && echo 0 || echo 1)"
ctl "an invocation outside any @test block is COUNTED as a reach limit rather than dropped" \
    "$([ "$(fx helper_inv)" -ge 1 ] && echo 0 || echo 1)"
ctl "an empty population REFUSES rather than reporting a clean zero" \
    "$(suite_flags "$FIX/empty" "$FIX/e_stats.tsv" >/dev/null 2>&1 && echo 1 || echo 0)"

[ "$CTL_FAIL" -eq 0 ] || die "$CTL_FAIL control(s) failed; no verdict is offered on the real population"

# --------------------------------------------------------------------------
# THE REAL POPULATION.
# --------------------------------------------------------------------------

table_flags "$TABLE" > "$WORK/table.txt"
TABLE_N=$(wc -l < "$WORK/table.txt" | tr -d ' ')
[ "$TABLE_N" -gt 0 ] || die "$TABLE yielded no dashed spellings at flags[].spellings[]; the table side is unreadable, not empty"

suite_flags "$SUITE_DIR" "$WORK/stats.tsv" > "$WORK/raw.tsv" || die "no .bats file found under $SUITE_DIR/; the suite side is unreadable, not empty"
st() { awk -F'\t' -v k="$1" '$1==k {print $2+0}' "$WORK/stats.tsv"; }

awk -F'\t' '$2=="SUCCESS"  {print $1}' "$WORK/raw.tsv" | sort -u > "$WORK/succ.txt"
awk -F'\t' '$2=="MIXED"    {print $1}' "$WORK/raw.tsv" | sort -u > "$WORK/mixed.txt"
awk -F'\t' '$2=="NOSTATUS" {print $1}' "$WORK/raw.tsv" | sort -u > "$WORK/nostat.txt"
SUCC_N=$(wc -l < "$WORK/succ.txt" | tr -d ' ')
[ "$SUCC_N" -gt 0 ] || die "no flag reached the binary in a success-asserting block; the extractor is not doing what this tool reports it as doing"

comm -23 "$WORK/succ.txt"  "$WORK/table.txt" > "$WORK/gap_suite.txt"
comm -13 "$WORK/succ.txt"  "$WORK/table.txt" > "$WORK/gap_table.txt"
comm -12 "$WORK/succ.txt"  "$WORK/table.txt" > "$WORK/both.txt"
GAP_SUITE=$(wc -l < "$WORK/gap_suite.txt" | tr -d ' ')
GAP_TABLE=$(wc -l < "$WORK/gap_table.txt" | tr -d ' ')
BOTH_N=$(wc -l < "$WORK/both.txt" | tr -d ' ')

# TWO-SIDED MEMBERSHIP CONTROL ON THE REAL DATA. A join whose intersection is
# empty is not a finding of total disagreement, it is an extractor mismatch --
# the two sides spelling the same flag differently. Refuse rather than report.
[ "$BOTH_N" -gt 0 ] || die "the two sides share NO flag at all ($SUCC_N suite, $TABLE_N table); that is an extractor mismatch, not a finding"

# --------------------------------------------------------------------------
# REACH -- printed before the verdict, every run.
# --------------------------------------------------------------------------

BATS_OUTSIDE=$(find . -name '*.bats' -type f -not -path "./$SUITE_DIR/*" -not -path '*/target/*' -not -path './.git/*' | wc -l | tr -d ' ')
printf '%s: DECLARED POPULATION -- %s/**/*.bats (%d file(s), %d @test block(s)); %s at flags[].spellings[], dashed only\n' \
  "$ME" "$SUITE_DIR" "$(st files)" "$(st blocks)" "$TABLE"
printf '%s: COMPLEMENT -- %d .bats file(s) elsewhere in the repo, NOT EXAMINED\n' "$ME" "$BATS_OUTSIDE"
printf '%s: EXCLUDED BY NAME -- table bare-word spelling(s): %s (a bare arm is not a flag)\n' "$ME" "$BARE_WORD_EXCLUSIONS"
printf '%s: REACH LIMIT -- %d intent invocation(s) across %d file(s) sit OUTSIDE any @test block (setup helpers); they carry no polarity and are in no class\n' \
  "$ME" "$(st helper_inv)" "$(st helper_files)"
printf '%s: BLOCKS -- %d invoke the binary, %d do not; of those that do: %d success-uniform, %d failure-uniform, %d MIXED, %d NOSTATUS (%d alias(es) discovered)\n' \
  "$ME" "$(st with_inv)" "$(st no_inv)" "$(st succ)" "$(st fail)" "$(st mixed)" "$(st nostatus)" "$(st aliases)"

if [ -s "$WORK/mixed.txt" ] || [ -s "$WORK/nostat.txt" ]; then
  printf '%s: HELD OUT, REPORTED AND NEVER RESOLVED -- %d flag(s) reach the binary only from MIXED blocks and %d only from NOSTATUS blocks\n' \
    "$ME" "$(comm -23 "$WORK/mixed.txt" "$WORK/succ.txt" | wc -l | tr -d ' ')" \
          "$(comm -23 "$WORK/nostat.txt" "$WORK/succ.txt" | wc -l | tr -d ' ')"
fi

# --------------------------------------------------------------------------
# VERDICT.
# --------------------------------------------------------------------------

RC=0
if [ "$GAP_SUITE" -gt 0 ]; then
  RC=1
  printf '%s: FINDING -- %d flag(s) the estate asserts GREEN against the binary that %s does not declare:\n' "$ME" "$GAP_SUITE" "$TABLE"
  while IFS= read -r f; do
    printf '    %-16s first asserted at %s\n' "$f" "$(awk -F'\t' -v k="$f" '$1==k && $2=="SUCCESS" {print $3; exit}' "$WORK/raw.tsv")"
  done < "$WORK/gap_suite.txt"
fi

if [ "$GAP_TABLE" -gt 0 ]; then
  RC=1
  printf '%s: FINDING -- %d declared flag(s) that NO success-asserting block exercises:\n' "$ME" "$GAP_TABLE"
  printf '    %s\n' "$(paste -sd' ' - < "$WORK/gap_table.txt")"
fi

printf '%s: %d suite flag(s) vs %d declared, %d in both; %d suite-only, %d table-only (%d control(s) fired)\n' \
  "$ME" "$SUCC_N" "$TABLE_N" "$BOTH_N" "$GAP_SUITE" "$GAP_TABLE" "$CTL_RUN"
exit "$RC"
