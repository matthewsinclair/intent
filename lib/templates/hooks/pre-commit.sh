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
# Both cases now reach this block's OWN fail-open branch, which is loud and
# already existed. No new machinery -- the ordering was the whole defect.
# ---- Repo guards (delegated: this file names NO guard and holds NO roster) ----
#
# THE ROSTER IS NOT HERE, AND THAT IS THE POINT. This file is COPIED into
# `.git/hooks/pre-commit.intent`; the roster is READ LIVE out of `INTENT_HOME`,
# in `pre-commit-guards.sh`. Anything a consumer holds a frozen copy of cannot
# be updated by shipping canon, so the roster must not be something they hold.
#
# It used to be, and the comment here claimed otherwise in those words --
# "this is also what makes a new guard propagate without touching a consumer's
# .git/hooks/". The guard BODIES propagated; the array naming them did not.
# Measured on this repository 2026-08-20: canon rostered four guards, the
# installed hook ran one, and two had never run here at all. The full account,
# and the roster, are in `pre-commit-guards.sh`.
#
# NO APPLICABILITY PRE-FILTER HERE, DELIBERATELY, AND IT COSTS ONE `intent info`
# (measured: 46ms) ON EVERY COMMIT IN EVERY PROJECT. The previous version tested
# the roster's `applies-when` paths first so a project owing nothing paid
# nothing -- which requires knowing the roster, which is the thing that must not
# live in a copied file. A cheap stand-in (`[ -e intent ]`) would be a second,
# weaker copy of the roster's own applicability rule, and the next guard whose
# subject sits outside `intent/` would be silently excluded by a file that never
# mentions it. That is the defect being fixed, one line lower. The runner
# answers applicability, per guard, and exits 0 in silence when nothing applies.
#
# Resolution is a RUNTIME question, answered the way issue 0016 answered it for
# the Claude Code hooks: ask the CLI where it lives rather than substituting an
# absolute path at install time. `sed` rather than `awk $2` so a home directory
# containing spaces still resolves.
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

# THREE ABSENCES, KEPT APART. Issue 0042 was two of them collapsed into one
# `else`: when the RESOLVER fails every guard is missing at once, so the loop
# printed one benign-looking "not found" per guard and enforced nothing -- two
# mild warnings read as two small holes when the truth was that the gate was not
# running. Delegating the roster adds a third, and the same discipline applies.
#
#   1. resolver did not answer   ALL guards missing   the tool that finds them is broken
#   2. runner not in the install ALL guards missing   the install predates this mechanism
#   3. one guard file missing    ONE hole             reported by the runner itself
#
# 1 and 2 are both total, and they are still named separately because the
# remedies share nothing: 1 is `intent info` misbehaving (a shadowed binary, a
# binary outside its own tree), 2 is an install that needs updating.
#
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
# `<not set>` WITHOUT naming it, and it is exactly the property the dispatch
# below needs to hold. Matching the literal token would rebuild the identical
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
# under-enforces everywhere. So: if the runner can be located, it RUNS, and a
# failing resolver is said out loud instead of being acted on.
# A SELF-HOSTED INTENT CHECKOUT RESOLVES ITS GUARDS FROM ITSELF. THIS IS THE
# ONE EXCEPTION TO THE LIVE-ROSTER RULE ABOVE, AND THE RULE ITSELF IS UNCHANGED.
#
# Reading the roster live out of `INTENT_HOME` is deliberate and stays: it is
# how a guard added to canon reaches every consumer with no reinstall. It is
# correct for every project that USES Intent. It is wrong for exactly one --
# the project that IS Intent -- because there `INTENT_HOME` names a DIFFERENT
# checkout, so this repo's commits are guarded by another tree's copy of this
# repo's own files.
#
# MEASURED 2026-08-21 at c8555d4e. With the v2 CLI split out to
# `~/Devel/prj/Intentv2`, `intent info` in the Intent source tree resolves to
# that frozen checkout. All seven guard files were byte-identical that day, so
# nothing was broken -- and that is exactly the shape of the frozen-roster
# failure already on this estate's record, where an installed hook ran one
# guard of four and nothing said so. The drift arms on the first change to
# `lib/templates/hooks/`; `pre-commit-guards.sh` IS the roster file, so a
# roster admission is the likeliest trigger.
#
# TWO CHEAPER ANSWERS WERE DECLINED BY NAME (hv, 2026-08-21). A `.envrc`,
# because git hooks do not reliably inherit direnv -- green where you look and
# absent where it matters. And refreshing the frozen copy by hand, because an
# advisory that requires remembering is not a control.
#
# THE MARKER IS THE RUNNER ITSELF PLUS `VERSION`. A repository carrying
# `lib/templates/hooks/pre-commit-guards.sh` in Intent's own source layout IS
# an Intent source tree, and `VERSION` beside it makes an accidental collision
# negligible. Deliberately NOT `bin/intent`: the v2 shell is slated for pruning
# here, and a marker that a planned change deletes is one that fails silently
# later, which is the class this whole block exists to remove.
_repo_root="$(git rev-parse --show-toplevel 2>/dev/null || true)"
GUARD_HOME="$INTENT_HOME_RESOLVED"
GUARD_HOME_IS_SELF=""
if [ -n "$_repo_root" ] \
  && [ -f "$_repo_root/lib/templates/hooks/pre-commit-guards.sh" ] \
  && [ -f "$_repo_root/VERSION" ]; then
  GUARD_HOME="$_repo_root"
  GUARD_HOME_IS_SELF="yes"
fi
GUARD_RUNNER="${GUARD_HOME}/lib/templates/hooks/pre-commit-guards.sh"

# THE THREE ABSENCE BRANCHES BELOW NOW TEST `GUARD_HOME`, NOT
# `INTENT_HOME_RESOLVED`, AND THAT IS LOAD-BEARING RATHER THAN TIDY. Absence 1
# blames the resolver by name. If the self-hosted branch won, the resolver is
# irrelevant to this commit, so keying the test to `INTENT_HOME_RESOLVED` would
# send an operator at `intent info` over a gate that never consulted it -- a
# true statement about the wrong component, which is the failure this file
# spends absences 1 and 2 keeping apart.
if [ ! -d "$GUARD_HOME" ]; then
  # ABSENCE 1. TOTAL non-enforcement, reported once and as itself. Nothing is
  # wrong with the guards and there is nothing to install -- the tool that
  # locates them did not answer, so fixing any one guard would change nothing.
  echo "intent gate: NO guard ran for this commit -- not one is missing, ALL are." >&2
  # The resolved value is QUOTED BACK rather than described, because the two
  # ways this fails look nothing alike to an operator and only one of them is
  # obviously wrong: an empty resolution reads as "the tool said nothing",
  # while `<not set>` reads as a legitimate answer until you notice it is not
  # a path. Naming it is what makes the second case self-evident.
  echo "  no usable INTENT_HOME (\`intent info\` exit ${wb_info_rc}, resolved to '${INTENT_HOME_RESOLVED}'), so the guard runner could not be located." >&2
  # NO LIST OF SKIPPED GUARDS, AND THE ABSENCE IS THE HONEST ANSWER. This used
  # to name the applicable ones, from the copied roster -- so a consumer whose
  # hook was stale was handed a STALE list of what had been skipped, omitting
  # precisely the guards added since they last installed. We cannot reach the
  # install, so we do not know its roster, and saying so beats reciting an old
  # one with no marking that it is old.
  echo "  which guards were owed is unknown -- the roster lives in the install this could not find." >&2
  echo "  the guards are fine; the tool that finds them is what did not answer." >&2
  echo "  check \`intent info\` -- a binary running outside its own install tree, or a v3 binary shadowing a v2 install on PATH, are the known causes (issues 0036/0043)." >&2
  # Deliberately fail-open, and this is a considered call rather than an
  # oversight. A gate that blocks every commit the moment `intent` is shadowed
  # is issue 0043 rebuilt on the git side, and 0043 is a hard publication hold
  # precisely because a tool that refuses everything is worse than one that
  # says so. A guard that must be bypassed is a guard nobody keeps.
elif [ ! -f "$GUARD_RUNNER" ]; then
  # ABSENCE 2. Also total, and deliberately NOT worded like absence 1: the
  # resolver worked and named a real directory, so telling the operator to go
  # and check `intent info` would send them at the one component that is fine.
  echo "intent gate: NO guard ran for this commit -- this install has no guard runner." >&2
  echo "  looked in: ${GUARD_RUNNER}" >&2
  if [ -n "$GUARD_HOME_IS_SELF" ]; then
    # The self-hosted branch cannot reach here by the marker test -- it REQUIRES
    # the runner to exist. So this is a race (the file removed between the test
    # and now) and must not be worded as a stale install, which is the one
    # remedy that would be wrong.
    echo "  this repository IS an Intent source tree, so the guards were taken from it and not from INTENT_HOME." >&2
    echo "  the runner was present when resolved and is gone now -- check for a concurrent write, not a stale install." >&2
  else
    echo "  INTENT_HOME resolved cleanly to '${INTENT_HOME_RESOLVED}', so the resolver is not the problem;" >&2
    echo "  that install predates the delegated roster. Update it, then re-run \`intent claude upgrade --apply\`." >&2
  fi
  # Fail-open, for absence 1's reason. An operator mid-upgrade must not be
  # unable to commit the upgrade.
else
  # rc REPORTS, IT DOES NOT GATE -- and it is reported HERE rather than above
  # the branch, for two reasons vc priced before I did. The total-failure blocks
  # already name the code in their own messages, so saying it twice is how a
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
    echo "intent gate: \`intent info\` exited ${wb_info_rc}, but the guard runner WAS located and is running -- nothing to do here." >&2
  fi
  # WHICH TREE THE GUARDS CAME FROM, SAID OUT LOUD, AND ONLY WHEN IT IS NOT THE
  # DEFAULT. hv's constraint on this mechanism was that a wrong answer must be
  # LOUD, and the wrong answer here is a repo running another checkout's copy of
  # its own guards. One line names the tree, so the answer is checkable at the
  # point of use instead of reconstructible afterwards.
  #
  # SILENT IN THE ORDINARY CASE, DELIBERATELY. For every project that USES
  # Intent this branch never fires, and a line on every commit in every project
  # restating the expected outcome is how a gate's output stops being read --
  # measured on this estate the same day this was written, where 82 lines of
  # entirely correct `ok:` masked four guards that were not present at all.
  if [ -n "$GUARD_HOME_IS_SELF" ]; then
    echo "intent gate: guards read from THIS repository (${GUARD_HOME}/lib/templates/hooks), not from INTENT_HOME." >&2
  fi
  bash "$GUARD_RUNNER" || exit 1
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
#
# **THE CODES BELOW ARE `intent critic`'s, AND THIS LEGEND WAS WRONG IN BOTH
# DIRECTIONS UNTIL 2026-08-20** -- it omitted 3 entirely and called 2 an
# invocation error, which is a CAUSE rather than a code. Driven, not read:
#
#   0 = clean
#   1 = findings at or above threshold          (BLOCKS)
#   3 = REFUSED -- a rule this project armed could not be enforced here (BLOCKS)
#   * = anything else. The gate does NOT know what it means. Fails open, LOUDLY.
#
# **`2` IS DELIBERATELY NOT LISTED, AND THAT IS THE FIX RATHER THAN AN OMISSION.**
# v2 uses 2 for a usage error; v3 uses it for `known command, not implemented`.
# One code, two meanings, two binaries -- so a legend that named 2 would be
# false of whichever binary it was not describing. The gate treats every
# unrecognised code identically and says so, which is the only claim it is
# entitled to make.

AGGREGATE=0
# Languages whose critic did not run. Collected rather than counted so the
# digest below can NAME them -- see the summary block after the loop.
UNENFORCED=()
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
        # **THE GATE DOES NOT KNOW WHY, AND MUST NOT SAY THAT IT DOES.** This
        # arm read `invocation error (exit $rc); fail-open` -- a DIAGNOSIS the
        # gate never made. It knows the code was unrecognised and nothing else.
        # Under a v3 binary it printed `invocation error` over a checker that
        # ran perfectly and simply is not built yet, which is a confident claim
        # about a cause it did not measure -- the class that cost this estate
        # seven wrong readings on 2026-08-20, and the only one of them with a
        # live consumer in every project that installs this hook.
        #
        # **THE FAIL-OPEN IS UNCHANGED AND IS A RULING, NOT AN OVERSIGHT.** A
        # gate that blocks the moment `intent` is shadowed is issue 0043 rebuilt
        # on the git side. What changes is only what the gate CLAIMS.
        #
        # **AND IT STATES THE CONSEQUENCE FOR THE COMMIT, NOT THE FATE OF THE
        # COMMAND** (dc). `did not check (exit 2)` is honest and is still a fact
        # about the tool; the operator's question is what happened to their
        # commit, and the answer is that a language they declared went
        # unenforced. A gate that fails open must at minimum be clear that it did.
        #
        # Only the LANGUAGE is named, never the rules: the gate knows what it
        # dispatched and does not know which rules would have fired. Naming
        # those would be the same overreach one level down.
        UNENFORCED+=("$lang")
        echo "intent critic ($lang) did not check (exit $rc) -- $lang is UNENFORCED in this commit." >&2
        [ -n "$out" ] && printf '%s\n' "$out" >&2
        ;;
    esac
  done
fi

# **ONE DIGEST WITH A DENOMINATOR, NOT N IDENTICAL LINES** (dc). Five declared
# languages all answering an unrecognised code printed five near-identical
# lines, and **a report that never changes trains its reader to stop looking**.
#
# **THE DENOMINATOR IS THE LOAD-BEARING PART.** `1 of 5` is a bad day; `5 of 5`
# is a gate that is not running at all, and those must never look alike. It is
# the same `of N` discipline this estate applies everywhere else, and it is what
# makes the line impossible to skim past on the day it changes.
if [ "${#UNENFORCED[@]}" -gt 0 ]; then
  echo "" >&2
  echo "intent critic gate: ${#UNENFORCED[@]} of ${#LANGS[@]} declared language(s) went UNENFORCED (${UNENFORCED[*]})." >&2
  echo "  the commit is NOT blocked by this -- the gate fails open on its own breakage by design." >&2
  echo "  nothing else reports this, so if it persists the gate is not protecting what you think it is." >&2
fi

if [ "$AGGREGATE" -eq 1 ]; then
  echo "" >&2
  echo "intent critic gate: commit blocked by findings at severity >= $SEVERITY." >&2
  echo "  review the findings above, fix them, and re-commit." >&2
  echo "  to bypass (use sparingly): git commit --no-verify" >&2
  exit 1
fi

exit 0
