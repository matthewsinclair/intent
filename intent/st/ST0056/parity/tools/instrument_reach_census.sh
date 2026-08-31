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

cat <<'REACH'
REACH, in the output because a limit not in the output is not a limit the
reader has:
  COVERS      Side A: Rust integration tests, the BATS suite, the parity shell
              instruments, the shipped git hooks. Side B: shipped shell
              surfaces -- plugin and skill executables, shipped git-hook and
              session-hook bodies.
  DOES NOT    ask WHICH property a surface is covered FOR. This is the gap
              that keeps the row red, and it is the class itself: all three
              founding instances were files being read the whole time, for a
              property narrower than the one being claimed.
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
echo "RED, AND NOT BECAUSE THE COMPLEMENT IS NON-EMPTY -- IT IS EMPTY."
echo "Every shipped shell surface is inside some instrument's reach. The row stays"
echo "red because a FILE-level union is not the criterion: AC-00.16 is scoped *for"
echo "any property the estate claims*, and this cannot say WHICH property a file is"
echo "covered for. One walk of intent/plugins/claude marks all 22 covered, truthfully,"
echo "for tracker citations alone."
echo ""
echo "MOVES WHEN the union is computed PER PROPERTY -- for each claim, its"
echo "instruments' reach against the population the claim covers. The register"
echo "already holds the AC/AT pairing that would key it."
exit 1
