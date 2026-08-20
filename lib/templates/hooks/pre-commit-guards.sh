#!/usr/bin/env bash
#
# pre-commit-guards.sh -- THE GUARD ROSTER, and the loop that dispatches it.
#
# READ LIVE OUT OF `INTENT_HOME`. NEVER COPIED INTO A PROJECT. That sentence is
# the entire reason this file exists as a file, so it goes first.
#
# WHAT WENT WRONG, MEASURED ON THIS REPOSITORY 2026-08-20 AT `5dbac6fb`.
# `pre-commit.sh` is copied verbatim into `.git/hooks/pre-commit.intent` by
# `intent claude upgrade --apply`, and it used to carry the roster INSIDE it.
# The guard bodies were read live from `INTENT_HOME`, and its own comment said
# in these words that this "makes a new guard propagate without touching a
# consumer's .git/hooks/". **The bodies propagated. The roster did not.** So
# adding a guard to canon reached nobody until they reinstalled the hook:
#
#     shipped roster (canon)                     4 guards
#     this repo's installed pre-commit.intent    1 guard, hardcoded, dated Aug 14
#     bin/int precommit compensated for          1 of the missing 3
#     guards running NOWHERE in this repository  canon-ignore-guard.sh
#                                                append-only-guard.sh
#
# `append-only-guard.sh` was written because 492 lines of `.history/` were
# destroyed on 2026-08-17 and 19 events on 2026-08-19. It protected nothing here
# from the day it was written. **A guard nothing dispatches is indistinguishable
# from a guard that passes** -- this file's own subject matter, happening to the
# mechanism, for six days, in silence.
#
# So the rule is now structural rather than remembered: the copied file names no
# guard and holds no roster, and there is exactly one place a guard is declared.
# Adding one is a line in the array below and it reaches every consumer with no
# reinstall, no version bump and nothing to remember.
#
# THREE ABSENCES, NOT TWO, AND THEY MUST STAY APART (issue 0042, one level up).
# The hook already distinguished "the resolver did not answer" (ALL guards
# missing) from "one guard file is missing" (one hole), because collapsing them
# printed one benign "not found" per guard while the gate was in fact not
# running. Delegating the roster adds a third: **the runner itself missing**,
# which is again all-guards-missing but has a different remedy -- nothing is
# wrong with the guards and nothing is wrong with the resolver; the install is
# older than this mechanism. The hook owns absences 1 and 2 because they are
# about locating THIS file. This file owns absence 3.

set -u

# THE ROSTER. One guard per concern, declared once, here.
#
# Format is `applies-when|basename|what goes unchecked if it is missing`.
#
# ONE GUARD PER CONCERN, NOT ONE GUARD THAT GREW. The clock guard checks
# TIMESTAMPS (three checks, all about clocks); the header guard checks the
# HEADER BLOCK's format contract. They were kept apart by ruling (vc,
# 2026-08-16): folding a second concern into a file named for the first makes
# its name lie to the next reader, and it couples two controls that should be
# independently canaried and independently disabled.
#
# ALL OF THEM RUN, THEN THE GATE DECIDES. Stopping at the first refusal costs a
# node one commit attempt per defect, and a board with a bad stamp AND an
# escaped value is one editing session, not two. Each guard prints its own
# report; this loop only aggregates the verdict.
#
# THE APPLICABILITY TEST IS PER-GUARD AND NOT PER-ROSTER (dc 2026-08-19, hv
# approved). The roster was whiteboard-only and so was its gate -- one
# `[ -d intent/whiteboard ]` around the lot -- which meant a guard about
# anything else had nowhere to be declared. `applies-when` is a PATH whose
# presence makes the guard relevant, so a project without a board runs no
# whiteboard guard and a project without canon runs no canon guard.
GUARDS=(
  'intent/whiteboard|whiteboard-clock-guard.sh|timestamps are UNCHECKED'
  'intent/whiteboard|whiteboard-header-guard.sh|header values are UNCHECKED'
  'intent/.canon|canon-ignore-guard.sh|an ignore rule reaching canon is UNCHECKED'
  # `intent` RATHER THAN EITHER SUBJECT, AND THE WIDTH IS DELIBERATE (cc's
  # proposal, taken). This guard has TWO subjects -- `intent/whiteboard/*/.history/**`
  # and `intent/events.jsonl` -- so neither path alone is right, and two entries
  # would dispatch one guard twice. `intent` is the smallest path containing both.
  #
  # IT DOES WEAKEN THE PROPERTY ARGUED FOR ABOVE, and saying so is cheaper than
  # discovering it: this guard opts in by BEING AN INTENT PROJECT rather than by
  # its subjects existing, so an Intent project with neither an events log nor a
  # board still pays for it. The cost is one `git diff --cached --numstat` that
  # returns empty and exits 0, and **the guard re-tests both subjects itself
  # regardless of why it was dispatched** -- so `applies-when` is a cheap
  # pre-filter here and not the real gate.
  #
  # D53 retires ONE of the two subjects: the event log's home is the store and
  # `intent/events.jsonl` becomes an `intent export` product. `.history/**` is
  # untouched, five nodes fold into it daily, and it is where the 492-line loss
  # actually happened. The guard keeps both entries until the file form is gone.
  'intent|append-only-guard.sh|a write where an append was meant is UNCHECKED'
)

# THE GUARDS ARE THIS FILE'S SIBLINGS, BY CONSTRUCTION.
#
# Derived from our own path rather than taken as an argument or re-resolved from
# `intent info`. The caller already resolved an install in order to find us, and
# asking a second time invites the two answers to differ -- a half-resolved
# `INTENT_HOME` could then pair this roster with another install's guard bodies,
# or with none, and the failure would look like "guard missing" rather than like
# the resolution bug it is. Siblings cannot disagree about which install they
# are in.
GUARD_HOME="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

BLOCKED=0
RAN=0
SKIPPED=0
MISSING=0

for g_entry in "${GUARDS[@]}"; do
  g_when="${g_entry%%|*}"
  g_rest="${g_entry#*|}"
  g_name="${g_rest%%|*}"
  g_unchecked="${g_rest#*|}"

  # NOT APPLICABLE IS SILENT, AND ABSENT IS LOUD. A project with a board and no
  # canon must not be told a canon guard did not run -- it has nothing to guard,
  # so there is no hole. The two are different facts and only one is a finding.
  if [ ! -e "$g_when" ]; then
    SKIPPED=$((SKIPPED + 1))
    continue
  fi

  g_path="${GUARD_HOME}/${g_name}"
  if [ -f "$g_path" ]; then
    RAN=$((RAN + 1))
    bash "$g_path" || BLOCKED=1
  else
    MISSING=$((MISSING + 1))
    # Reached only with the runner located, so this really is one hole and the
    # other guards really did run. Named, not silent: a subject present with no
    # guard behind it is exactly the invisible non-enforcement this whole
    # mechanism exists to end.
    echo "intent gate: ${g_when} present but ${g_name} was not found;" >&2
    echo "  ${g_unchecked} this commit. (looked in: ${g_path})" >&2
  fi
done

# **ONE LINE ON STDOUT, AND IT EXISTS BECAUSE SILENCE ON SUCCESS IS
# INDISTINGUISHABLE FROM NOT RUNNING** (cc, 2026-08-20, measured on the first
# commit after the `core.hooksPath` redirect). Every other message in this file
# goes to stderr and only on a problem, so a passing run printed NOTHING -- and
# a runner that was never dispatched prints nothing too. That is the sentence
# this whole mechanism was built to delete, arriving on the success path.
#
# A COUNT, NOT A LIST, and the distinction is the one refused a hundred lines
# up. Listing which guards were skipped means reciting a roster the reader
# cannot check; saying HOW MANY ran answers the only question silence leaves
# open -- did anything happen. `skipped` is the not-applicable population and is
# a normal, healthy number: a project with no canon skips the canon guard and
# owes nothing.
#
# `missing` is reported separately and never folded into `skipped`, because
# they are opposite facts: skipped means there was nothing to guard, missing
# means there was and no guard was there. The per-guard report above already
# names each one; this is the total, so a reader who saw no detail lines can
# still tell the difference between three and zero.
printf 'guards: %d ran, %d skipped (not applicable)' "$RAN" "$SKIPPED"
[ "$MISSING" -gt 0 ] && printf ', %d MISSING' "$MISSING"
printf '\n'

exit "$BLOCKED"
