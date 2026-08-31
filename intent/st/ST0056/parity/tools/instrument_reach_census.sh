#!/usr/bin/env bash
#
# instrument_reach_census.sh -- AT-00.20, covering AC-00.16.
#
# **EVERY INSTRUMENT HAS A DECLARED REACH, AND NOTHING IN THE ESTATE OWNS THE
# GAPS BETWEEN THEM.** Minted from three instances by three nodes on
# 2026-08-30, each an instrument SOUND within its own reach with a shipped
# surface sitting in the gap.
#
# ==========================================================================
# WHY THE FIRST CUT COULD NOT HAVE CLOSED THE ROW
# ==========================================================================
#
# It measured ONE necessary condition -- that an instrument declares what it
# does not cover -- over ONE population, `crates/*/tests/*.rs`. It said so and
# was right to be red. **But its reach WAS the class it measures**: the vc
# instance that minted this criterion is a shipped SHELL surface, and a Rust
# scanner cannot see one. Side A below is that measurement over all four
# instrument families instead of one.
#
# **AC-00.16 HAS TWO CONJUNCTS AND ONLY THE FIRST WAS MEASURED**: instruments
# declare their reach, AND something names what the union does NOT cover.
# Side B is the second.
#
# ==========================================================================
# TWO SPELLINGS OF "COVERED" WERE TRIED AND BOTH ARE UNSOUND. MEASURED, NOT
# REASONED -- AND THE SECOND ONE PRODUCED FOUR CONFIDENT FALSE POSITIVES.
# ==========================================================================
#
# **`named` IS NOT COVERAGE.** `no_pm_state_in_output.rs:651` names
# `intent/plugins/claude/skills/in-tca-synthesize/SKILL.md` inside
# `FORMAT_TEMPLATES`, an EXEMPTION list. The file is named BECAUSE it is
# excluded, so mention-is-coverage scores the strongest available statement of
# NON-coverage as coverage.
#
# **AND `not named` IS NOT NON-COVERAGE, WHICH IS THE ERROR THAT NEARLY
# SHIPPED.** A basename census reported four shipped skill scripts as reached
# by nothing -- `cost-metrics.sh` and the three `tca-*.sh`. All four are false.
# `installed_payload()` in that same file WALKS `intent/plugins/claude`
# recursively, and `no_installed_payload_file_cites_intents_own_tracker`
# asserts over every file it returns. **An instrument that reads a tree names
# no file in it.** The four were caught only by driving the remedy -- opening
# the instrument the finding accused -- rather than by trusting the finding.
#
# **SO REACH IS A PREFIX QUESTION, AND THE ERROR IS DIRECTED ON PURPOSE.** A
# surface is covered when some instrument names a path that is it or contains
# it. That over-approximates: a literal may be an exemption or prose. **The
# over-approximation puts every error into false NEGATIVES, so the complement
# it prints is a SUBSET of the true complement -- everything reported is real,
# and some real members may be missing.** For an instrument whose output is a
# defect list that is the only acceptable direction; the opposite spelling
# spent this session manufacturing four.
#
# ==========================================================================
# WHY THE ROW IS STILL RED WHILE SIDE B PRINTS NOTHING
# ==========================================================================
#
# **The complement is empty and that is not the criterion.** AC-00.16 is
# scoped `for any PROPERTY the estate claims to hold`, and this computes a
# FILE-level union: is this file inside anything's reach, for any purpose at
# all. One coarse walk marks a whole tree covered -- `intent/plugins/claude`
# covers all 22 surfaces here, truthfully, for ONE property (tracker
# citations) and says nothing about any other.
#
# **A FILE-LEVEL UNION CANNOT SEE THE CLASS.** All three founding instances are
# a property asserted over a population narrower than the property's subject,
# in files that were being read the whole time. What closes this row is a
# union computed PER PROPERTY -- for each claim, the union of its instruments'
# reach against the population the claim covers -- and the register already
# holds the AC/AT pairing that would key it.
#
# **AND THE EXEMPTION MARKER THIS FILE CARRIED FOR ONE DRAFT IS GONE.** It had
# no members once the spelling was fixed, and a mechanism with no consumer
# distinguishes nothing (AT-11.5's finding). It goes back in when something
# needs it, not before.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="${ROOT:-$(cd "$HERE/../../../../.." && pwd)}"
cd "$ROOT" || { echo "error: cannot enter ${ROOT}" >&2; exit 2; }

# ==========================================================================
# POPULATIONS, BY MEMBERSHIP RULE RATHER THAN BY LIST
# ==========================================================================
# Globs, not lists. A hand list is AC-00.11's wrong-M with instruments as the
# population -- M derived from what was easy to enumerate.

instruments=()
while IFS= read -r f; do instruments+=("$f"); done < <(
  {
    find native/rust/crates -path '*/tests/*.rs' -not -path '*/target/*'
    find tests -name '*.bats'
    find intent/st/ST0056/parity/tools -maxdepth 1 -name '*.sh'
    find lib/templates/hooks -name '*.sh'
  } 2>/dev/null | sort -u
)

# A SHIPPED SURFACE is shell the release puts in front of a user or an agent:
# skill and plugin executables, the shipped git-hook bodies, and the shipped
# Claude Code session-hook bodies. All four install onto a user's machine.
surfaces=()
while IFS= read -r f; do surfaces+=("$f"); done < <(
  {
    find intent/plugins -name '*.sh' -type f
    find intent/plugins -path '*/bin/*' -type f
    find lib/templates/hooks -name '*.sh' -type f
    find lib/templates/.claude/scripts -name '*.sh' -type f
  } 2>/dev/null | sort -u
)

[ "${#instruments[@]}" -gt 0 ] || { echo "error: instrument population is empty" >&2; exit 2; }
[ "${#surfaces[@]}" -gt 0 ] || { echo "error: surface population is empty" >&2; exit 2; }

echo "AT-00.20 -- instrument reach census, over ${#instruments[@]} instrument(s) and ${#surfaces[@]} shipped surface(s)"
echo ""

# ==========================================================================
# SIDE A -- DOES AN INSTRUMENT DECLARE WHAT IT DOES NOT COVER?
# ==========================================================================
# A MEASUREMENT, NOT A BAR. AC-00.16 puts the obligation on a PROPERTY the
# estate claims, not on every file containing an assertion, so `all 433 must
# declare a reach` is a bar the criterion does not set and this does not
# invent one.

declaring=0; silent=0
for f in "${instruments[@]}"; do
  if grep -qiE 'does not (read|cover|scan|reach|see)|out of scope|NOT in scope|says nothing about|DOES NOT ' "$f" 2>/dev/null; then
    declaring=$((declaring + 1))
  else
    silent=$((silent + 1))
  fi
done

echo "SIDE A -- declared reach, across every instrument family"
printf '    %s of %s instrument(s) state something they do NOT cover\n' "$declaring" "${#instruments[@]}"
printf '    partition: %s declaring + %s silent = %s\n' "$declaring" "$silent" "$((declaring + silent))"
[ $((declaring + silent)) -eq "${#instruments[@]}" ] || { echo "error: side A partition does not close" >&2; exit 2; }
echo ""

# ==========================================================================
# SIDE B -- THE UNION'S COMPLEMENT, BY PATH PREFIX
# ==========================================================================

prefixes="$(grep -rhoE '(native|tests|intent|lib|bin|scripts|schema|docs)/[A-Za-z0-9_./-]+' "${instruments[@]}" 2>/dev/null \
  | sed 's/[.,)"'"'"'`]*$//' | sort -u)"
nprefix="$(printf '%s\n' "$prefixes" | grep -c . )"

# THE DERIVED SET GETS THE SAME GUARD AS THE POPULATIONS, AND IT IS HERE
# BECAUSE IT FIRED. A draft lost this set to a shell error and reported 22 of
# 22 surfaces uncovered -- a closing partition, a confident list, and every
# entry wrong. An empty derivation must refuse, never answer.
[ "$nprefix" -gt 0 ] || { echo "error: derived prefix set is empty; a coverage answer from it would be vacuous" >&2; exit 2; }

covered=0; uncovered=()
for s in "${surfaces[@]}"; do
  hit=""
  while IFS= read -r p; do
    [ -n "$p" ] || continue
    case "$s" in "$p"|"$p"/*) hit="$p"; break ;; esac
  done <<< "$prefixes"
  if [ -n "$hit" ]; then covered=$((covered + 1)); else uncovered+=("$s"); fi
done

echo "SIDE B -- the union's complement: shipped surfaces under NO instrument's reach"
printf '    %s distinct repo-relative path literal(s) across the instrument corpus\n' "$nprefix"
printf '    partition: %s covered + %s UNCOVERED = %s\n' "$covered" "${#uncovered[@]}" "$((covered + ${#uncovered[@]}))"
[ $((covered + ${#uncovered[@]})) -eq "${#surfaces[@]}" ] || { echo "error: side B partition does not close" >&2; exit 2; }
echo ""
if [ "${#uncovered[@]}" -gt 0 ]; then
  echo "    UNCOVERED -- shipped, and under no path any instrument names:"
  for u in "${uncovered[@]}"; do printf '      %s\n' "$u"; done
  echo ""
fi


# ==========================================================================
# SIDE C -- THE UNION PER PROPERTY, WHICH IS WHAT THE CRITERION ASKS FOR
# ==========================================================================
# Sides A and B are per-FILE. AC-00.16 is scoped `for any property the estate
# claims to hold`, so the unit is a CLAIM and its certificates. The register
# holds that pairing: an AT row carries a `file` and a `covers` list, so
# claim -> certificates is derived, never listed.
#
# **AND THE PREDICATE BELOW IS UNSOUND IN BOTH DIRECTIONS. MEASURED, THREE
# SPELLINGS, NOT REASONED -- THE THIRD TIME THIS FILE HAS PAID FOR ASSUMING A
# SPELLING OF "COVERED".** Declaring a reach is a SEMANTIC act performed with
# ordinary words, so no grep decides it:
#
#   narrow      142 declaring / 36 silent. 16 of those 36 declare a reach in
#               words it does not match -- `NOT covered`, `Nothing here
#               captures`, `deliberately NOT`. It INVENTS defects, which is
#               the forbidden direction for a defect list.
#   wide        158 / 20, by admitting the bare token `reach`. Every rescue
#               sampled was a MENTION: "reach for `sync`", "did not reach the
#               operator", "an ignore rule reaching canon". None declares
#               anything. That is 1d's mention-is-not-an-instance, committed
#               inside the instrument built to catch it.
#   calibrated  150 / 28, and still wrong both ways: of four sampled rescues
#               two are genuine (`AC-14.7 and AC-14.8 are NOT covered`) and
#               two are about the SUBJECT's behaviour, not the instrument's
#               reach (`deliberately NOT rewritten -- authority follows
#               authorship`).
#
# **SO SIDE C REPORTS A PARTITION AND REFUSES TO CALL IT A DEFECT LIST.** The
# count is real; the membership is not decidable by this instrument. What
# closes AC-00.16 is a STRUCTURAL declaration -- a REACH block in a fixed
# form, which this file emits and which is exactly the emit partner 1g says a
# consumed-but-never-emitted form needs. That is a criterion amendment and it
# is hv's, not the author's: the author of a check is not a safe source for
# that check's own denominator, and this author's amendment would make this
# author's red row greenable.

CLAIMS_TSV="$(jq -rs '
  [ .[] | .id as $st | (.tests // [])[]
    | select((.file // "") | length > 0)
    | .file as $f | (.covers // [])[] | "\($st)/\(.)\t\($f)" ]
  | unique | .[]' intent/.canon/st/*.json 2>/dev/null)"

nclaims="$(printf '%s\n' "$CLAIMS_TSV" | awk -F'\t' 'NF==2 {print $1}' | sort -u | grep -c . )"

# THE DERIVED SET REFUSES WHEN EMPTY, EXACTLY AS THE POPULATIONS AND SIDE B'S
# PREFIX SET DO. Same reason: a draft that loses this to a shell error would
# print a closing partition over nothing and read as a clean pass.
[ "$nclaims" -gt 0 ] || { echo "error: derived claim set is empty; a per-property answer from it would be vacuous" >&2; exit 2; }

DECLARES='does not (read|cover|scan|reach|see|assert|check|test|touch|know)|out of scope|NOT in scope|says nothing|DOES NOT |only (covers|checks|reads|asserts|tests|exercises|knows)|limited to|deliberately (out|excluded|not)|no attempt|not covered|nothing (here|in this file)'

c_all=0; c_some=0; c_none=0; c_absent=0; multi=0
some_list=()
while IFS= read -r ac; do
  [ -n "$ac" ] || continue
  d=0; s=0; n=0
  while IFS= read -r f; do
    [ -n "$f" ] || continue
    n=$((n + 1))
    [ -f "$f" ] || continue
    if grep -qiE "$DECLARES" "$f" 2>/dev/null; then d=$((d + 1)); else s=$((s + 1)); fi
  done < <(printf '%s\n' "$CLAIMS_TSV" | awk -F'\t' -v a="$ac" '$1 == a {print $2}')
  [ "$n" -gt 1 ] && multi=$((multi + 1))
  if   [ $((d + s)) -eq 0 ]; then c_absent=$((c_absent + 1))
  elif [ "$s" -eq 0 ];       then c_all=$((c_all + 1))
  elif [ "$d" -eq 0 ];       then c_none=$((c_none + 1))
  else c_some=$((c_some + 1)); some_list+=("$ac ($d declaring, $s silent)")
  fi
done < <(printf '%s\n' "$CLAIMS_TSV" | awk -F'\t' 'NF==2 {print $1}' | sort -u)

echo "SIDE C -- the union PER PROPERTY: each claim, against its own certificates"
printf '    %s claim(s) carry at least one certificate; %s carry more than one\n' "$nclaims" "$multi"
printf '    partition: %s all-declare + %s SOME + %s none + %s all-absent = %s\n' \
  "$c_all" "$c_some" "$c_none" "$c_absent" "$((c_all + c_some + c_none + c_absent))"
[ $((c_all + c_some + c_none + c_absent)) -eq "$nclaims" ] || { echo "error: side C partition does not close" >&2; exit 2; }
echo ""
echo "    ALL-ABSENT is an EXCLUSION WITH ITS REASON NAMED, not a silent drop: every"
echo "    certificate is to-write, so the property has no instrument to interrogate yet."
echo "    The denominator moves visibly rather than the corpus shrinking quietly."
echo ""
if [ "${#some_list[@]}" -gt 0 ]; then
  echo "    SOME -- the sharp form. A claim whose certificates DISAGREE about declaring"
  echo "    reach has a union nobody owns, and each file still passes its own inspection:"
  for m in "${some_list[@]}"; do printf '      %s\n' "$m"; done
  echo ""
fi
printf '    AND THE SHARP FORM IS NEARLY UNPOPULATED HERE: %s of %s claims have exactly ONE\n' "$((nclaims - multi))" "$nclaims"
echo "    certificate, so for those the union IS the instrument and Side C asks the same"
echo "    question as Side A. Per-property is the right unit and this estate's shape is"
echo "    what limits what it can see -- a fact the reader needs and the count does not say."
echo ""
cat <<'REACH'
REACH, in the output because a limit not in the output is not a limit the
reader has:
  COVERS      Side A: Rust integration tests, the BATS suite, the parity shell
              instruments, the shipped git hooks. Side B: shipped shell
              surfaces -- plugin and skill executables, shipped git-hook and
              session-hook bodies.
  COVERS      Side C: every claim in the register carrying a certificate,
              keyed by the AT `covers` list. This is the per-property union
              the row was owed, derived from the register rather than listed.
  DOES NOT    DECIDE whether a certificate declares its reach. Side C's
              predicate is LEXICAL and unsound in BOTH directions -- measured,
              three spellings, not reasoned. Its partition is a real count
              over a real population; its MEMBERSHIP is not a defect list and
              must not be read as one.
  DOES NOT    cover colocated `#[cfg(test)]` modules in src/, the critic rule
              library, or any NON-SHELL shipped surface. The Rust binaries are
              shipped surfaces and are deliberately out: `intent` is the
              subject of most of the estate, so a coverage test over it would
              report a truth that means nothing.
  DIRECTED    the prefix rule OVER-approximates coverage, so every error lands
              in false NEGATIVES. What it prints is real; what it prints is
              not everything. Both naive spellings were measured wrong first --
              `named` (exemption lists) and `not named` (directory walks, four
              false positives in one draft).
  OVERLAPS    `lib/templates/hooks/*.sh` are in BOTH populations. A shipped
              guard is a surface a user runs and an instrument the estate
              trusts; nothing here lets one cover itself by naming its own
              path, because the literal it would match on is its own filename
              in some OTHER instrument's corpus, not in its own.
REACH
echo ""

if [ "${#uncovered[@]}" -gt 0 ]; then
  echo "RED. ${#uncovered[@]} shipped surface(s) sit under no instrument's reach at all."
  exit 1
fi
cat <<'VERDICT'
RED, AND THE REASON HAS CHANGED -- WHICH IS THE PROGRESS.
Side B's complement is EMPTY and Side C now computes the per-property union the
row was owed, so neither of the two things that previously kept this row red is
what keeps it red now.

WHAT KEEPS IT RED IS THAT AC-00.16's FIRST CONJUNCT IS NOT MACHINE-DECIDABLE AS
WRITTEN. `the instruments asserting it declare their reach` asks whether prose
performs a semantic act, and three calibrated spellings each failed in a
DIFFERENT direction over the same 178 files: narrow INVENTS defects (16 of 36
were declaring in words it did not match), wide rescues on MENTIONS of the token
`reach`, calibrated does both at a smaller rate. A criterion decided by
vocabulary is not decided, and this instrument will not launder a judgement it
cannot make into a verdict it prints.

TWO THINGS THE PARTITION DOES SAY, AND BOTH ARE REAL. One claim -- ST0056/AC-00.7
-- has certificates that DISAGREE about declaring reach, which is the sharp form:
a union nobody owns while each file passes its own inspection. And 322 of 328
claims carry exactly ONE certificate, so per-property collapses to per-instrument
for 98% of the estate. The unit is right; the estate's shape bounds what it sees.

MOVES WHEN the declaration becomes STRUCTURAL rather than lexical -- a REACH
block in a fixed form, of which this file is the only instance and therefore the
emit partner a never-emitted form needs. THAT IS A CRITERION AMENDMENT AND IT IS
hv's, NOT THIS AUTHOR'S: the author of a check is not a safe source for that
check's own denominator, and this author's amendment would make this author's own
red row greenable.
VERDICT
exit 1
