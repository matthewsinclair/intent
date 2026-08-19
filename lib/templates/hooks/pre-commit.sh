#!/usr/bin/env bash
#
# pre-commit.sh -- Intent critic gate (ST0035/WP-06)
#
# Purpose:
#   Run `intent critic <lang> --staged --severity-min <sev>` for each
#   language detected in the project, block the commit on findings at or
#   above the configured severity threshold, and fail-open when the
#   critic tooling itself is unavailable.
#
# Install:
#   Copied to `.git/hooks/pre-commit` (chmod +x) by
#   `intent claude upgrade --apply`. If a pre-existing hook is present,
#   the installer writes to `.git/hooks/pre-commit.intent` and prints
#   instructions for chaining instead of overwriting.
#
# Configuration:
#   Reads severity threshold from `.intent_critic.yml` at the project
#   root. Default: warning (block on CRITICAL + WARNING).
#
# Opt-out:
#   `git commit --no-verify` bypasses the hook. Use sparingly.
#
# Exit codes:
#   0  no findings at or above threshold (commit proceeds)
#   1  findings at or above threshold (commit blocked)
#   2+ reserved; hook itself always exits 0 or 1 after aggregating

# Don't set -e: we need exit codes to propagate through variables.
set -u

# ---- Discover project root ----

if ! command -v git >/dev/null 2>&1; then
  echo "intent critic gate: git not on PATH; skipping." >&2
  exit 0
fi

PROJECT_ROOT="$(git rev-parse --show-toplevel 2>/dev/null)"
if [ -z "$PROJECT_ROOT" ]; then
  echo "intent critic gate: not inside a git worktree; skipping." >&2
  exit 0
fi
cd "$PROJECT_ROOT" || exit 0

# MOVED ABOVE BOTH FAIL-OPEN EXITS, 2026-08-17, because it was BELOW them and
# the comment below has always said otherwise. Two exits sat between: no
# `intent` on PATH, and no `intent/.config/config.json`. Either one returned 0
# and NO whiteboard guard ran, with nothing printed to say so -- an exit written
# when there was one arm is a claim that the run is over.
#
# The population is not exotic: the whiteboard is opt-in by DIRECTORY PRESENCE,
# so a board in a repo that has not been `intent init`-ed is a state the design
# permits, and every one of them was silently unguarded (cc's framing).
#
# Both cases now reach the block's OWN fail-open branch, which is loud and
# already existed: it says the guards could not be located and names them. No
# new machinery -- the ordering was the whole defect.
# ---- Whiteboard guards (opt-in by directory presence) ----
#
# Run BEFORE the critic: they are exact, cheap, need no language, and a bad
# board is cheaper to fix before the slower checks have run.
#
# Opt-in by presence, exactly like the whiteboard itself -- a project without a
# board is not one these guards have an opinion about, and nothing changes for
# it.
#
# ONE GUARD PER CONCERN, DECLARED HERE, NOT ONE GUARD THAT GREW. The clock guard
# checks TIMESTAMPS (three checks, all about clocks); the header guard checks
# the HEADER BLOCK's format contract. They were kept apart by ruling (vc,
# 2026-08-16): folding a second concern into a file named for the first makes
# its name lie to the next reader, and it couples two controls that should be
# independently canaried and independently disabled. Adding a third is a line in
# this array.
#
# ALL OF THEM RUN, THEN THE GATE DECIDES. Stopping at the first refusal costs a
# node one commit attempt per defect, and a board with a bad stamp AND an
# escaped value is one editing session, not two. Each guard prints its own
# report; this loop only aggregates the verdict.
#
# Resolution is a RUNTIME question, answered the way issue 0016 answered it for
# the Claude Code hooks: ask the CLI where it lives rather than substituting an
# absolute path at install time. `intent` is already required on PATH above, and
# it knows its own home; `sed` rather than `awk $2` so a home directory
# containing spaces still resolves. **This is also what makes a new guard
# propagate without touching a consumer's .git/hooks/**: only this file is
# copied into a project, and it reads the guard bodies live out of INTENT_HOME.
#
# Format is `basename|what goes unchecked if it is missing`, so the fail-open
# message can name the specific hole rather than saying "a guard".
#
# THE APPLICABILITY TEST IS PER-GUARD AND NOT PER-ROSTER, AND THAT IS THE WHOLE
# GENERALISATION (dc 2026-08-19, hv approved). The roster was whiteboard-only and
# so was its gate -- one `[ -d intent/whiteboard ]` around the lot -- which meant
# a guard about anything else had nowhere to be declared. `canon-ignore-guard.sh`
# was written, shipped, and named in NO roster: it existed and never ran, which is
# precisely the invisible non-enforcement this mechanism exists to end, occurring
# inside the mechanism. **A guard nothing dispatches is indistinguishable from a
# guard that passes**, and no output anywhere said otherwise.
#
# Format is `applies-when|basename|what goes unchecked if it is missing`.
# `applies-when` is a PATH whose presence makes the guard relevant, so a project
# without a board runs no whiteboard guard and a project without canon runs no
# canon guard -- each opts in by the thing it protects existing, rather than by a
# roster-wide condition that has to be true for all of them at once.
GUARDS=(
  'intent/whiteboard|whiteboard-clock-guard.sh|timestamps are UNCHECKED'
  'intent/whiteboard|whiteboard-header-guard.sh|header values are UNCHECKED'
  'intent/.canon|canon-ignore-guard.sh|an ignore rule reaching canon is UNCHECKED'
  # `intent` RATHER THAN EITHER SUBJECT, AND THE WIDTH IS DELIBERATE (cc's
  # proposal, taken). This guard has TWO subjects -- `intent/whiteboard/*/.history/**`
  # and `intent/events.jsonl` -- so neither path alone is right, and two entries
  # would dispatch one guard twice. `intent` is the smallest path containing both.
  #
  # IT DOES WEAKEN THE PROPERTY THE COMMENT ABOVE ARGUES FOR, and saying so is
  # cheaper than discovering it: this guard opts in by BEING AN INTENT PROJECT
  # rather than by its subjects existing, so an Intent project with neither an
  # events log nor a board still pays for it. The cost is one
  # `git diff --cached --numstat` that returns empty and exits 0, and **the guard
  # re-tests both subjects itself regardless of why it was dispatched** -- so
  # `applies-when` is a cheap pre-filter here and not the real gate.
  'intent|append-only-guard.sh|a write where an append was meant is UNCHECKED'
)

# Applicable = at least one rostered guard's subject is present. Computed before
# the INTENT_HOME resolution below because resolving costs an `intent info` call,
# and a project with neither a board nor canon owes nothing and should pay nothing.
GUARDS_APPLY=0
for g_entry in "${GUARDS[@]}"; do
  [ -e "${g_entry%%|*}" ] && GUARDS_APPLY=1
done

if [ "$GUARDS_APPLY" -eq 1 ]; then
  # ONE GUARD ABSENT AND THE RESOLVER ABSENT ARE DIFFERENT ABSENCES (issue 0042).
  # These were one `else` branch, and it could not tell them apart: when the
  # resolution fails EVERY guard is missing at once, so the loop printed one
  # benign-looking "not found" per guard and enforced nothing. Two mild warnings
  # read as two small holes; the truth was that the gate was not running -- and
  # it fails open, so the commit proceeds either way and nothing else ever
  # reports it.
  #
  # Captured WITHOUT a pipe before `$?` is read: `x="$(cmd | sed)"; rc=$?` gives
  # sed's status, and that mistake has cost this estate four wrong diagnoses.
  wb_info_out="$(intent info 2>&1)"; wb_info_rc=$?
  # Trailing whitespace is stripped as well as leading, and that is vc's measured
  # hardening rather than a defensive reflex: the line is COLUMN-PADDED ON THE
  # LEFT today (`  INTENT_HOME:     /path`) with no padding on the right, so a
  # renderer that ever pads the other side would hand `-d` a path with trailing
  # spaces and turn a working resolver into a loud false block. One token closes
  # it. Written as a single addressed block rather than two `p` expressions,
  # which would print the line twice, and without GNU's `T`, which BSD sed lacks.
  INTENT_HOME_RESOLVED="$(printf '%s\n' "$wb_info_out" | sed -n '/^ *INTENT_HOME:/ { s/^ *INTENT_HOME: *//; s/ *$//; p; }' | head -1)"
  WB_BLOCKED=0

  # THE SIGNAL WAS ALREADY IN HAND AND WENT UNUSED. This tested EMPTINESS alone,
  # which was the true signature of an unresolvable install on the day it was
  # written: `intent info` was unimplemented, printed no INTENT_HOME line at all,
  # and the `sed` above yielded nothing. It now prints `INTENT_HOME: <not set>`
  # -- v2 has always rendered that token (`bin/intent_info`) and v3 reproduces it
  # deliberately so this parse never comes back empty -- which is better for a
  # human and NON-EMPTY, so the branch below became unreachable in exactly the
  # condition it exists for. Measured on a brew-shaped install (a binary sitting
  # outside its own tree): exit 1, resolution `<not set>`, and the loop then hunted
  # for guards under `<not set>/lib/templates/hooks/` and reported two small holes.
  # Neither change was wrong and nothing connected them, because the coupling is a
  # `sed` over display text and is written down nowhere but here.
  #
  # So gate on `! -d`: it answered, and the answer is not a place. That subsumes
  # the old emptiness test (an empty string is not a directory), it catches
  # `<not set>` WITHOUT naming it, and it is exactly the property the loop below
  # needs to hold. Matching the literal token would rebuild the identical
  # coupling one token over, and the next rendering change would break it again
  # in the same silence.
  #
  # `wb_info_rc` REPORTS AND DOES NOT GATE, WHICH IS A DELIBERATE DEPARTURE from
  # the shape agreed with vc ("branch on rc as well as emptiness"), recorded here
  # rather than resolved quietly. Gating on rc makes the guards conditional on an
  # exit code whose meanings are still being settled -- vc's own 0045 measured
  # that `Facade::open` gates EVERY command and the migration refusal returns 1.
  # The day `info` inherits that, rc is non-zero in every unmigrated project (ie
  # every consumer, the moment before it upgrades) while INTENT_HOME resolves
  # perfectly, and gating would silently stop the guards estate-wide -- the exact
  # class this branch exists to prevent, delivered by the fix for it. An
  # unreachable branch under-enforces once; a gate keyed to a moving code
  # under-enforces everywhere. So: if the guards can be located, they RUN, and
  # a failing resolver is said out loud instead of being acted on.
  if [ ! -d "$INTENT_HOME_RESOLVED" ]; then
    # TOTAL non-enforcement, reported once and as itself. Named separately
    # because the remedy is different in kind: nothing is wrong with the guards
    # and there is nothing to install -- the tool that locates them did not
    # answer, so fixing any one guard would change nothing.
    echo "intent gate: NO guard ran for this commit -- not one is missing, ALL are." >&2
    # The resolved value is QUOTED BACK rather than described, because the two
    # ways this fails look nothing alike to an operator and only one of them is
    # obviously wrong: an empty resolution reads as "the tool said nothing",
    # while `<not set>` reads as a legitimate answer until you notice it is not
    # a path. Naming it is what makes the second case self-evident.
    echo "  no usable INTENT_HOME (\`intent info\` exit ${wb_info_rc}, resolved to '${INTENT_HOME_RESOLVED}'), so the guards could not be located." >&2
    # Only the APPLICABLE ones are named. Listing a canon guard as skipped in a
    # project with no canon would report a hole that does not exist, and a
    # message that overstates is one a reader learns to discount.
    g_skipped=""
    for g_entry in "${GUARDS[@]}"; do
      g_rest="${g_entry#*|}"
      [ -e "${g_entry%%|*}" ] && g_skipped="$g_skipped ${g_rest%%|*}"
    done
    echo "  skipped:${g_skipped}" >&2
    echo "  the guards are fine; the tool that finds them is what did not answer." >&2
    echo "  check \`intent info\` -- a binary running outside its own install tree, or a v3 binary shadowing a v2 install on PATH, are the known causes (issues 0036/0043)." >&2
    # Deliberately fail-open, and this is a considered call rather than an
    # oversight. A gate that blocks every commit the moment `intent` is shadowed
    # is issue 0043 rebuilt on the git side, and 0043 is a hard publication hold
    # precisely because a tool that refuses everything is worse than one that
    # says so. A guard that must be bypassed is a guard nobody keeps.
  else
    # rc REPORTS, IT DOES NOT GATE -- and it is reported HERE rather than above
    # the branch, for two reasons vc priced before I did. The total-failure block
    # already names the code in its own message, so saying it twice is how a
    # block starts getting skimmed. And this is the only place the code is
    # genuinely ANOMALOUS: the resolution worked, the guards are about to run,
    # and the tool still said it failed. That is the earliest signal this
    # coupling is breaking again, and it is the whole reason the code is read.
    #
    # Priced knowingly: the day `info` inherits the migration refusal's non-zero
    # code, this prints on every commit in every unmigrated project until it
    # upgrades. That is a line of noise where gating would have been silent
    # non-enforcement, which is the right direction -- but a line a reader cannot
    # act on is how a gate's output stops being read, so it says outright that
    # the guards ran and that nothing is owed.
    if [ "$wb_info_rc" -ne 0 ]; then
      echo "intent gate: \`intent info\` exited ${wb_info_rc}, but the guards WERE located and are running -- nothing to do here." >&2
    fi
    for wb_entry in "${GUARDS[@]}"; do
      wb_when="${wb_entry%%|*}"
      wb_rest="${wb_entry#*|}"
      wb_name="${wb_rest%%|*}"
      wb_unchecked="${wb_rest#*|}"
      # NOT APPLICABLE IS SILENT, AND ABSENT IS LOUD. A project with a board and
      # no canon must not be told a canon guard did not run -- it has nothing to
      # guard, so there is no hole. The two are different facts and only one of
      # them is a finding.
      [ -e "$wb_when" ] || continue
      wb_guard="${INTENT_HOME_RESOLVED}/lib/templates/hooks/${wb_name}"
      if [ -f "$wb_guard" ]; then
        bash "$wb_guard" || WB_BLOCKED=1
      else
        # Reached only with INTENT_HOME resolved, so this really is one hole and
        # the other guards really did run. Named, not silent: a board present
        # with no guard behind it is exactly the invisible non-enforcement this
        # whole mechanism exists to end.
        echo "intent gate: ${wb_when} present but ${wb_name} was not found;" >&2
        echo "  ${wb_unchecked} this commit. (looked in: ${wb_guard})" >&2
      fi
    done
  fi
  [ "$WB_BLOCKED" -eq 0 ] || exit 1
fi

# ---- Fail-open on missing intent CLI ----

if ! command -v intent >/dev/null 2>&1; then
  echo "intent critic gate: 'intent' CLI not on PATH; skipping." >&2
  echo "  install Intent or add its bin/ to PATH to enable the gate." >&2
  exit 0
fi

# Fail-open if this repo isn't an Intent project (the hook may have been
# copied manually into a non-Intent repo). Without this check,
# `intent critic` would exit non-zero with a "not in an Intent project"
# message and the commit would be blocked for the wrong reason.
# We already cd'd to the git toplevel above, and every later read
# (languages, .intent_critic.yml) is relative to it, so the gate's
# definition of "Intent project" is config.json at the git toplevel.
if [ ! -f "intent/.config/config.json" ]; then
  echo "intent critic gate: not inside an Intent project (intent/.config/config.json absent); skipping." >&2
  exit 0
fi

# ---- Read declared languages from project config ----
#
# v2.11.0+: languages-in-use is an explicit `languages` array in
# intent/.config/config.json (see ST0037). The hook reads the field and
# dispatches one critic per language. Empty array means no language critics
# run (only the agnostic checklist applies upstream of this hook).

LANGS=()
if command -v jq >/dev/null 2>&1 && [ -f "intent/.config/config.json" ]; then
  while IFS= read -r lang; do
    [ -n "$lang" ] && LANGS+=("$lang")
  done < <(jq -r '(.languages // []) | .[]' intent/.config/config.json 2>/dev/null)
fi

# ---- Load severity threshold from .intent_critic.yml ----

SEVERITY="warning"
if [ -f ".intent_critic.yml" ]; then
  config_sev="$(awk '
    /^severity_min:[[:space:]]*/ {
      v = $0
      sub("^severity_min:[[:space:]]*", "", v)
      gsub("[[:space:]\"'\'']", "", v)
      sub("#.*$", "", v)
      print v
      exit
    }
  ' .intent_critic.yml 2>/dev/null)"
  case "$config_sev" in
    critical|warning|recommendation|style) SEVERITY="$config_sev" ;;
  esac
fi

# ---- Run critic per language ----
# Exit codes per language:
#   0 = clean
#   1 = findings at or above threshold
#   2 = invocation error (fail-open for that language)

AGGREGATE=0
# Length-guard the loop. Under `set -u` (set above), expanding "${LANGS[@]}"
# on an empty array errors as "unbound variable" on some bash versions
# (notably the CI macOS runner). v2.11.0 introduced the empty-array path
# (config languages: [] = no critic runs); the explicit length check makes
# the iteration safe across bash versions.
if [ "${#LANGS[@]}" -gt 0 ]; then
  for lang in "${LANGS[@]}"; do
    # Every declared language is dispatched; `intent critic` owns the code-vs-
    # prose classification (its single registry). A prose / on-demand discipline
    # (author, content) returns a clean exit 0 no-op, so it neither blocks nor
    # prints a spurious "fail-open" line -- the gate needs no language knowledge
    # of its own, and cannot drift from the CLI (issue 0003).
    # Capture output so we can surface findings only when present.
    out="$(intent critic "$lang" --staged --severity-min "$SEVERITY" --format text 2>&1)"
    rc=$?
    # THREE OUTCOMES, NOT TWO-AND-A-BUCKET. `*)` used to swallow every rc that
    # was not 0 or 1 and fail open on all of them, which was fine only while
    # exactly one such code existed. AC-07.4 rules that a tool-armed rule REFUSES
    # when its tool is absent on a project that armed it, and that refusal needs
    # a code this gate can tell apart from "the critic could not start". Adding
    # the refusal without this branch would have put a THIRD condition in a
    # bucket that already conflated two -- deliberately this time, which is worse
    # than the accident. So both ends move together, in one commit.
    #
    # 3 BLOCKS AND 2 FAILS OPEN, AND THE DIFFERENCE IS WHETHER ANYONE CAN ACT.
    # An invocation error means the gate is broken; blocking every commit until
    # someone fixes the gate is issue 0043 rebuilt on the git side, and a guard
    # that must be bypassed is a guard nobody keeps. A refusal is the opposite:
    # the project ARMED a rule, the tool is not here, and there are two ordinary
    # remedies the developer owns -- install the tool, or disarm the rule. **A
    # gate should fail open on its own breakage and closed on yours.**
    case "$rc" in
      0) ;;
      1)
        printf '%s\n' "$out" >&2
        AGGREGATE=1
        ;;
      3)
        # The findings, if any, are still printed: a refusal does not make the
        # rest of the run uninteresting, and suppressing them would trade one
        # silent gap for another.
        [ -n "$out" ] && printf '%s\n' "$out" >&2
        echo "intent critic ($lang) REFUSED: a rule this project armed could not be enforced here." >&2
        echo "  this is not a gate failure -- the gate is telling you it cannot cover what you asked for." >&2
        echo "  remedy: install the missing tool, or disarm that rule for this project." >&2
        AGGREGATE=1
        ;;
      *)
        echo "intent critic ($lang) invocation error (exit $rc); fail-open." >&2
        [ -n "$out" ] && printf '%s\n' "$out" >&2
        ;;
    esac
  done
fi

if [ "$AGGREGATE" -eq 1 ]; then
  echo "" >&2
  echo "intent critic gate: commit blocked by findings at severity >= $SEVERITY." >&2
  echo "  review the findings above, fix them, and re-commit." >&2
  echo "  to bypass (use sparingly): git commit --no-verify" >&2
  exit 1
fi

exit 0
