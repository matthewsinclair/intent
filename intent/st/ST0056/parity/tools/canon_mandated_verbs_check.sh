#!/usr/bin/env bash
#
# AC-00.3's falsifier, derived rather than remembered.
#
# **THE ROW'S DEFECT WAS THE ENUMERATION, NOT ANY DRIVE.** Three honest
# re-drives re-ran the same five verbs fixed at the first drive and never
# re-derived the population from canon, so none of them could have found a
# sixth. The list had become a proxy for *what canon names* and had stopped
# tracking it. This tool EXTRACTS the verbs from the shipped canon files and
# runs what it extracts, so the population is a measurement and not a memory.
#
# **IT EXISTED AS PROSE BEFORE IT EXISTED AS A FILE.** The row records that
# "the instrument now extracts the verbs from the shipped canon" -- and no
# instrument was committed, so the next reader inherited the sentence and not
# the check. A doc that names its own instrument does not make anyone run it.
#
# **THE PREDICATE IS rc==2 AND THE PHRASE, AND ONE CLAUSE CURRENTLY REDS
# NOTHING -- SAID HERE RATHER THAN LEFT TO LOOK LOAD-BEARING.** Measured
# 2026-09-04: all 5 rc=2 verbs in this population also carry the phrase, so the
# phrase clause excludes ZERO of them and no mutation of it can fail. It is
# kept because the OTHER direction is real and recorded -- `intent llm` prints
# the marker inside the agent guide at rc=0 -- but that verb is not named by
# any shipped canon file today, so it never enters this population. If a canon
# file ever names `llm`, this clause starts biting; until then it is insurance
# and not a check, and a reader should not count it as one.
#
# **A RED HERE IS NOT AUTOMATICALLY A DEFECT, AND hv HAS ALREADY RULED ONE CASE
# THE OTHER WAY.** On 2026-08-31 hv ruled that `config`, `ext` and `learn` SHIP
# DECLARED-AND-UNBUILT in 3.0.1. This tool has no way to know that: it reds any
# canon-named verb answering rc=2 with the marker, so if a canon file ever names
# one of those three it will report a state hv has explicitly blessed. None of
# them appears in the six files below today, which is why the current red set is
# `rules validate` ALONE, since A1 wired `claude subagents` on 2026-09-04 -- and
# `rules validate` is itself a RULING rather than a defect: vc ruled the same day
# that a verb canon NAMES stays declared and refuses at rc=2, and
# `usage-rules.md:140` names it. **So this tool's steady state is RED-AT-ONE, and
# a reader who "fixes" that red is undoing a ruling.** **A reader meeting a red
# should ask whether it is a defect or a ruling before treating it as the
# former** -- and the reason this warning is here rather than in a note
# elsewhere is that hv's ruling lived in a MODULES.md cell no node reads at
# pickup, and a fold buried it for four days.
#
# **WRITE VERBS ARE DECLARED SKIPPED RATHER THAN SILENTLY DROPPED.** A verb this
# tool does not run is not evidence of anything, and an unstated exclusion is
# how a census reports a clean number over a population it never looked at.

set -uo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../../.." && pwd)"
cd "$root" || exit 2

INTENT="${INTENT_BIN:-intent}"
PHRASE="is a known command that is not implemented yet"

# The shipped canon an agent actually loads, plus the templates a fresh project
# is seeded from. Both, because the falsifier's subject is a SWITCHED project
# and the template is not what such a project is reading.
CANON=(
  "AGENTS.md"
  "CLAUDE.md"
  "usage-rules.md"
  "lib/templates/llm/_AGENTS.md"
  "lib/templates/llm/_CLAUDE.md"
  "lib/templates/llm/_usage-rules.md"
)

# Verbs that WRITE. Declared here so the exclusion is visible in the output
# rather than inferred from a shorter list than expected.
SKIP_RE='^(claude (subagents|skills) (install|sync|uninstall)|claude upgrade|agents sync|agents init|upgrade)'

extract() {
  local f
  for f in "${CANON[@]}"; do
    [ -f "$f" ] || continue
    # `intent <verb...>` at the start of a line inside a fenced block or inline.
    # Trailing comments and arguments are cut: `<name>`, `--all`, `# note`.
    sed -n 's/^[[:space:]]*intent \([a-z][a-z0-9 -]*\).*$/\1/p' "$f"
  done | sed 's/[[:space:]]*$//' | sort -u
}

mapfile_replacement=()
while IFS= read -r line; do
  [ -n "$line" ] && mapfile_replacement+=("$line")
done < <(extract)

total=0
unwired=0
skipped=0
declare -a UNWIRED_LIST=()

for verb in "${mapfile_replacement[@]}"; do
  if [[ "$verb" =~ $SKIP_RE ]]; then
    skipped=$((skipped + 1))
    echo "canon-verbs: SKIP (writes)  intent $verb"
    continue
  fi
  total=$((total + 1))
  # The verb is split deliberately and explicitly. An unquoted expansion here
  # is a no-op under zsh and this tool has been bitten by both directions of
  # that: `$=v` inside a bash script split nothing and reported unwired=0.
  read -r -a argv <<<"$verb"
  out="$("$INTENT" "${argv[@]}" 2>&1)"
  rc=$?
  # A herestring, not a pipeline. `grep -q` exits on first match and SIGPIPEs
  # the writer; under `pipefail` that 141 becomes the pipeline status and the
  # test reads FALSE on an input that matched. This tool would have classified
  # a real refusal as wired, silently, on a lost race.
  if [ "$rc" -eq 2 ] && grep -qF "$PHRASE" <<<"$out"; then
    unwired=$((unwired + 1))
    UNWIRED_LIST+=("$verb")
    echo "canon-verbs: UNWIRED rc=2   intent $verb"
  fi
done

echo "canon-verbs: examined $total canon-named verb(s), $skipped declared skipped (they write)"

# **THE REACH NOTE PRINTS ON EVERY RUN, GREEN OR RED, AND IT IS NOT DECORATION**
# (vc, 2026-09-04: *write that into the instrument own output, not just your
# board -- a caveat that lives beside the finding travels, and one that lives on
# a board does not*). This tool went from 3 red to 1 the day `claude subagents`
# was wired, and the tempting reading of that green -- the subagents family now
# matches canon -- is FALSE. Stated here so the next reader meets it beside the
# number rather than in a whiteboard archive.
cat <<'REACH'
canon-verbs: REACH -- what a green here does NOT say. Two independent blind spots, so closing either one alone changes nothing:
canon-verbs:   (1) THE PREDICATE IS rc==2 AND THE MARKER. That conjunct is deliberate -- `intent llm` prints the marker at rc=0 while working, so the marker alone would false-positive. The cost is that NO OTHER REFUSAL CAN BE CLASSIFIED. A clap parse error is rc=1 with no marker at all, so a flag that canon documents and the dispatch table does not declare reads as perfectly fine here. Measured 2026-09-04: `install --all` and `uninstall --all` are refused at rc=1 and this tool called that run green (issue 0236).
canon-verbs:   (2) THE WRITE VERBS ARE SKIPPED, listed above by name. A green says nothing whatever about `install`, `sync` or `uninstall` for ANY family -- only about the verbs this tool actually ran.
canon-verbs:   SO A GREEN MEANS: the canon-named verbs this tool RAN answer. It is not a statement about a family, and it is not a statement about flags. The defect class it cannot hold lives in the DISAGREEMENT between canon and the dispatch table, which is a comparison no instrument here makes.
REACH

# **THE POSITIVE CONTROL, AND unwired=0 IS EXACTLY WHAT A BROKEN PROBE
# RETURNS.** An absence-shaped defect and a clean estate produce the same
# number, so the instrument asserts it can still SEE a refusal before its zero
# is allowed to mean anything.
probe="$("$INTENT" claude nosuchverb 2>&1)"
prc=$?
if [ "$prc" -ne 1 ]; then
  echo "canon-verbs: REFUSING -- the negative control did not answer rc=1 (got $prc); this run's zero would be worthless" >&2
  exit 2
fi
if [ "$total" -eq 0 ]; then
  echo "canon-verbs: REFUSING -- extracted no verbs from canon at all; the extractor, not the estate, is what this measured" >&2
  exit 2
fi

if [ "$unwired" -gt 0 ]; then
  echo "canon-verbs: RED -- $unwired canon-named verb(s) refuse in a switched project:" >&2
  for v in "${UNWIRED_LIST[@]}"; do echo "  intent $v" >&2; done
  exit 1
fi

echo "canon-verbs: ok -- every canon-named verb this tool ran answers"
