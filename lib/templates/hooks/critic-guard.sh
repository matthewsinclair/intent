#!/usr/bin/env bash
#
# critic-guard.sh -- the Intent critic gate, as a GUARD BODY rather than as part
# of the carrier. (A2), ruled by hv 2026-08-27 in vc's session.
#
# THIS FILE IS NOT ROSTERED YET AND THAT IS DELIBERATE, NOT AN OVERSIGHT.
# `pre-commit-guards.sh` does not name it, so nothing runs it and landing it
# changes no behaviour in any estate. The reason is measured: every installed
# `pre-commit.intent` STILL CONTAINS THE WHOLE GATE -- Lamplight's stale carrier
# greps 13 `intent critic` invocations and Intent's own greps 13 -- and no verb
# writes the carrier. So rostering this today would run the critic TWICE in all
# fifteen guarded estates until (B) ships and every carrier is refreshed.
#
# Correctness would be unchanged (both copies block on the same rc) and latency
# is negligible (a five-language pass with nothing staged measures 0s), but
# EVERY FINDING WOULD PRINT TWICE -- which is the permanently-noisy-aggregate
# failure `bin/.devbin/config.yaml` already warns about, where the real finding
# hides inside its own duplicate. The roster line is one line and is hv's to
# time; the body is needed under every ordering, so it lands now and lands inert.
#
# WHY THIS MOVE IS THE INVERSE OF THE INCIDENT IT RESEMBLES, and the distinction
# belongs in the code rather than only in a message. The roster was moved OUT of
# the carrier at `042985c8` because guard BODIES propagated and the array naming
# them did not -- "the roster travelled in the copied file, so it never
# travelled". Putting a refusing arm back INTO the carrier would reproduce that
# incident's precondition. This moves the same direction the roster went: out of
# the frozen layer, into the live one. cc raised the caution and it is the right
# caution; the answer is that the arrow points the other way.
#
# THE rc CONTRACT, WHICH IS THE WHOLE SAFETY OF THE MOVE. The runner invokes a
# guard as `bash "$g_path" || BLOCKED=1`, so ANY non-zero blocks -- while the
# carrier discriminates in a `case "$rc"` whose fail-open is a ruling rather
# than an oversight. Moved carelessly, that converts a ruled fail-open into a
# fail-closed silently, which is answering an open question in code where nobody
# would read it as an answer.
#
# IT DOES NOT NEED A TRANSLATION LAYER, AND THAT IS A MEASUREMENT RATHER THAN A
# HOPE: the gate already aggregates rather than passing `intent critic`'s code
# through. `1` and `3` set AGGREGATE and reach `exit 1`; `0` and every
# unrecognised code leave AGGREGATE alone and reach `exit 0`. So its exit status
# is ALREADY a verdict in the runner's own vocabulary -- block or do not -- and
# the ruled fail-open survives the move untouched. Driven, not read.
#
# ORIGIN: extracted verbatim from `pre-commit.sh:280-607`, the section from the
# not-an-Intent-project test through the UNENFORCED digest. Nothing in it
# referenced carrier scope, so this is a move rather than a rewrite. It relies
# on cwd being the project root, which the runner provides: the carrier does
# `cd "$PROJECT_ROOT"` and the runner adds no `cd` of its own, which is how all
# four existing guards already read project-relative paths.

set -u

# ---- Not an Intent project: skip. THIS TEST COMES FIRST, AND THE ORDER IS
# ---- THE WHOLE FIX ----
#
# The hook may have been copied by hand into a non-Intent repo, where
# `intent critic` would exit non-zero with "not in an Intent project" and block
# the commit for the wrong reason. We already cd'd to the git toplevel, and
# every later read (languages, .intent_critic.yml) is relative to it, so the
# gate's definition of "Intent project" is config.json at the git toplevel.
#
# **THIS IS THE PRECISE TEST, WHICH IS WHY THE CLI CHECK BELOW NO LONGER NEEDS
# TO BE FAIL-OPEN.** The CLI check used to sit here and skip on a missing
# `intent`, justified as "do not block work in a non-Intent repo" -- a
# population this test already covers exactly. One test was standing in for the
# other, and the substitute was the one that could not tell the two apart.
if [ ! -f "intent/.config/config.json" ]; then
  echo "intent critic gate: not inside an Intent project (intent/.config/config.json absent); skipping." >&2
  exit 0
fi

# ---- An Intent project whose CLI is missing FAILS. It does not skip ----
#
# **A GATE THAT CANNOT RUN, IN A PROJECT THAT DECLARED IT, IS A FAILURE AND
# NEVER A SKIP.** Reaching this line means config.json is present, so this IS an
# Intent project and the gate was asked for; a missing CLI is then a broken
# installation, not a repo the gate does not apply to.
#
# **MEASURED BEFORE THE CHANGE (vc, 2026-08-27): all 17 estates carrying this
# hook ARE Intent projects, so the fail-open protected nobody** -- and it cost
# 12 ungated commits across 3 estates inside one 9-minute window, two of them
# the committing node's own. A skip is indistinguishable from a pass to
# everything downstream, which is why the cost went unnoticed while the log
# looked healthy.
#
# **AND THIS FILE ALREADY DIAGNOSED THE CLASS ONE BLOCK UP.** The 2026-08-17
# comment recording the whiteboard guard's move ABOVE both fail-open exits says
# it exactly: *an exit written when there was one arm is a claim that the run is
# over.* Both exits below it were written when the critic gate was the only arm
# after them. The diagnosis was right and stopped at the arm that prompted it.
#
# The asymmetry -- skip where the tool does not apply, fail where it applies and
# cannot run -- is devbin-vc's, already implemented in `check format`.
# **RESOLVED, THEN TESTED FOR EXECUTABILITY -- because `command -v` does not do
# the second thing.** This condition used to be `! command -v intent`, and that
# is exactly two of the five states short: a plain non-executable file (C) and a
# link to a non-executable target (E) are both FOUND by `command -v`, sailed
# past this arm, and died at the invocation site as exit 126 -- where the gate
# reported the language UNENFORCED and let the commit through.
#
# **hv RULING 4 (2026-08-27): A GATE THAT CANNOT LOCATE WHAT IT NEEDS REFUSES,
# IT DOES NOT SKIP.** This arm ALREADY refused for A, B and D; C and E fell
# through on an accident of what `command -v` tests, not on a policy anyone
# chose. Closing it makes the five states agree rather than adding a new
# severity: the gate is not becoming stricter, it is becoming consistent.
#
# **AND THIS IS NOT ISSUE 0043 REBUILT ON THE GIT SIDE.** That objection applies
# to blocking when the tool RAN and answered; it does not apply here, because
# the tool did not run at all and this arm has blocked for three of these five
# states since it was written. The fail-open below (exit 2, unimplemented) is
# untouched and remains a ruling.
_cv="$(command -v intent 2>/dev/null || true)"
if [ -z "$_cv" ] || [ ! -x "$_cv" ] || [ -d "$_cv" ]; then
  # ---- WHICH ABSENCE? `command -v` COLLAPSES THREE STATES INTO ONE EMPTY ANSWER ----
  #
  # **THE REMEDY WAS WRONG FOR MOST OPERATORS WHO WOULD EVER SEE IT** (ic,
  # measured live 2026-08-27): during a release build the CLI goes ABSENT rather
  # than merely changing, so every node in the fleet hit this arm at once -- and
  # the message told them to INSTALL INTENT, which races a build already in
  # flight. `~/.local/bin/intent` is a symlink into the release tree here, so the
  # window is real and estate-wide, not hypothetical.
  #
  # **THREE, AND THE NUMBER WAS FOUR UNTIL A TEST DROVE IT.** ic and I both
  # recorded that `command -v` answers empty for FOUR states, and I wrote four
  # branches on that basis. Measured under bash on all five planted states:
  #
  #   A  nothing on PATH by that name      rc=1 EMPTY   reaches here
  #   B  link, target does not resolve     rc=1 EMPTY   reaches here
  #   D  a DIRECTORY of that name          rc=1 EMPTY   reaches here
  #   C  plain file, not +x                rc=0 FOUND   NEVER reaches here
  #   E  link, target resolves, not +x     rc=0 FOUND   NEVER reaches here
  #
  # **`command -v` DOES NOT TEST EXECUTABILITY**, which is why the condition
  # above resolves the path and asks `-x` itself. C and E USED TO sail through
  # here and die at the invocation site as exit 126, reported as UNENFORCED and
  # not blocking. They now arrive, so all five states are handled in one place
  # and the `rc` column above describes `command -v` alone rather than this
  # arm's reach.
  #
  # **THE TWO BRANCHES I ORIGINALLY WROTE FOR C AND E WERE UNREACHABLE** -- dead
  # code reading as coverage, in the very file that exists to stop that. They
  # were removed when the table was driven, and what follows is a single branch
  # written against a state that now actually arrives, which is a different
  # thing from restoring them.
  #
  # The discriminators are `-L` (is the PATH name a link at all) and `-e` (does
  # what it names resolve); `-e` is FALSE for a dangling symlink, which is why
  # `-L` is asked first. `-d` is asked because a directory is SEARCHABLE, so it
  # passes an executable test and would otherwise fall into the residue branch
  # while the answer was one test away.
  #
  # **AND ONE STATE CANNOT REACH ANY OF THIS: a binary that is executable and
  # TRUNCATED.** It satisfies `command -v`, so the gate never arrives -- it fails
  # when something runs it. Named rather than checked: a check that appeared to
  # cover it would be a claim this gate cannot make from the filesystem alone.
  # **WHAT `command -v` RESOLVED BEATS A PATH WALK, WHEN THERE IS ONE.** For C
  # and E the shell has already told us the exact file it would have run; re-
  # deriving it by walking PATH could name a DIFFERENT entry and give the
  # operator the wrong file to fix.
  _cand="$_cv"
  _ifs_saved="$IFS"
  IFS=':'
  if [ -z "$_cand" ]; then
    for _dir in $PATH; do
      [ -n "$_dir" ] || _dir="."
      if [ -L "$_dir/intent" ] || [ -e "$_dir/intent" ]; then
        _cand="$_dir/intent"
        break
      fi
    done
  fi
  IFS="$_ifs_saved"

  echo "intent critic gate: 'intent' CLI is not runnable, and this IS an Intent project." >&2
  echo "  refusing rather than skipping: a declared gate that cannot run is a failure." >&2
  if [ -z "$_cand" ]; then
    echo "  state: no PATH entry named 'intent' exists at all." >&2
    echo "  remedy: install Intent, or add its bin/ to PATH, then re-commit." >&2
  elif [ -L "$_cand" ] && [ ! -e "$_cand" ]; then
    echo "  state: $_cand is a link whose target does not resolve." >&2
    echo "         it points at $(readlink "$_cand" 2>/dev/null || echo '<unreadable>')" >&2
    echo "         a build has removed the artefact. DO NOT reinstall -- that races it." >&2
    echo "  remedy: wait for the build to finish, then re-commit." >&2
  elif [ -d "$_cand" ]; then
    echo "  state: $_cand is a DIRECTORY, not a program." >&2
    echo "         a directory is searchable, so it passes an executable test and" >&2
    echo "         still cannot be run. Something on PATH shadows the real CLI." >&2
    echo "  remedy: rename it, or reorder PATH so Intent's bin/ comes first." >&2
  elif [ ! -x "$_cand" ]; then
    # **C AND E, THE TWO STATES THAT USED TO BE A FAIL-OPEN.** One branch, not
    # two, and that is deliberate: C (a plain file without +x) and E (a link to
    # a target without +x) differ in how the operator got here and not at all in
    # what they must do, and `chmod +x` is the remedy for both. The earlier
    # attempt at this gave them a branch each and neither could ever run.
    #
    # **THE HISTORY IS IN THIS COMMENT AND NOT ON THE OPERATOR'S TERMINAL.** A
    # draft of this branch printed "this used to reach the critic and fail as
    # exit 126 -- reported as UNENFORCED"; true, and none of the reader's
    # business. It is D37's rule -- our own backlog is not output -- and it also
    # defeated the test asserting the old wording was gone, because the message
    # contained the word the assertion was looking for.
    echo "  state: $_cand exists but is not executable." >&2
    if [ -L "$_cand" ]; then
      echo "         it is a link to $(readlink "$_cand" 2>/dev/null || echo '<unreadable>')," >&2
      echo "         and it is the TARGET that lacks the executable bit." >&2
    fi
    echo "  remedy: chmod +x the file above, or reorder PATH so Intent's bin/ comes first." >&2
  else
    # **RESIDUE, AND IT IS A NAMING RATHER THAN A GUARD.** A, B and D are each
    # driven by a test. Nothing reaches here that anyone has constructed: an
    # entry present, executable, not a directory, and still not found. It says
    # so plainly instead of guessing, because a confident wrong remedy costs
    # more than an admitted gap -- and a non-executable file is NOT this case,
    # it is C or E, which now have the branch directly above.
    echo "  state: $_cand looks executable, so the lookup failed for a reason this" >&2
    echo "         gate cannot name from the filesystem -- report it rather than" >&2
    echo "         working around it." >&2
    echo "  remedy: run 'command -v intent; type intent' by hand and read the answer." >&2
  fi
  echo "  to bypass this one commit (use sparingly): git commit --no-verify" >&2
  exit 1
fi

# ---- Read declared languages from project config ----
#
# v2.11.0+: languages-in-use is an explicit `languages` array in
# intent/.config/config.json. The hook reads the field and
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
    # exactly one such code existed. The rule is that a tool-armed rule REFUSES
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
