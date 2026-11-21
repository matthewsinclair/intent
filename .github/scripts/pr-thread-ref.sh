#!/usr/bin/env bash
#
# pr-thread-ref.sh -- the pull-request check's thread reference (ST0095 WP-12).
#
# Reads the pull request's body from PR_BODY and asks the binary's id door,
# `intent id` (ST0095 WP-10), which of its words names a thread, in place of a
# pattern for an id's shape, so a git-style id is found as an ordinal is. The
# first word the door answers as a thread, or as a work package of one,
# decides: the door's `id` verdict names the record's canon, so it is the
# existence check too.
#
# Exit 0: a thread is named and exists, or no word names one (a notice only).
# Exit 1: the first word that names a thread names one this project does not
#   hold, or names more than one.
# Exit 2: the door did not answer, eg a binary built before WP-10 has no
#   `intent id`, or it answered fewer words than it was asked. That fails the
#   job: this script never falls back to a pattern.
#
# Run from the project root, with INTENT_BIN naming the binary the job built
# (pr-checks.yml sets it). bash 3.2 compatible.

set -euo pipefail

BODY="${PR_BODY:-}"

echo "Checking if PR references a steel thread..."

no_reference() {
  echo "ℹ️  No steel thread reference found in PR description"
  echo "Consider referencing the related steel thread by its id, as Intent prints it, in your PR"
  exit 0
}

# unanswered <what> -- the door did not answer: fail the job, never pass it.
unanswered() {
  echo "❌ Error: ${INTENT_BIN:-the intent binary} did not answer \`intent id\`: $1, so the PR body's thread reference is unchecked, not absent" >&2
  echo "  remedy: build the binary this job builds, from this tree, which carries the id door (ST0095 WP-10), and run the check again" >&2
  exit 2
}

[ -n "${INTENT_BIN:-}" ] || unanswered "INTENT_BIN is not set, so no binary was named"
[ -x "$INTENT_BIN" ] || unanswered "there is no executable at $INTENT_BIN"

# The body's words, edge punctuation stripped (so no word reaches the door as a
# flag), each once, in body order.
WORDS="$(printf '%s\n' "$BODY" | tr -s '[:space:]' '\n' |
  sed -e 's/^[^[:alnum:]]*//' -e 's/[^[:alnum:]]*$//' | awk 'NF && !seen[$0]++')"
[ -n "$WORDS" ] || no_reference

# Every word goes to the door in one call. Its exit is 1 whenever any word is
# not an id, which is most of them, so the exit alone says nothing: the answer
# is judged by its lines.
OLDIFS="$IFS"
IFS='
'
set -f
# shellcheck disable=SC2086 # split on newlines only, with globbing off
set -- $WORDS
set +f
IFS="$OLDIFS"
RC=0
ANSWERS="$("$INTENT_BIN" id "$@" 2>&1)" || RC=$?

# THE DOOR ANSWERED ONLY IF IT GAVE ONE OF ITS FOUR VERDICTS FOR EVERY WORD.
# Anything else is a door that did not answer: clap's refusal of an unknown
# subcommand from a binary without `intent id`, which exits 1 like a word that
# is not an id, or a binary that printed nothing at all.
[ "$RC" -le 1 ] || unanswered "it exited $RC"
STRAY="$(printf '%s\n' "$ANSWERS" | awk 'NF && $1 != "id" && $1 != "unknown" && $1 != "ambiguous" && $1 != "not-an-id"' | head -3)"
[ -z "$STRAY" ] || unanswered "it said: $STRAY"
VERDICTS="$(printf '%s\n' "$ANSWERS" | awk 'NF' | wc -l | tr -d ' ')"
[ "$VERDICTS" -eq "$#" ] || unanswered "it gave $VERDICTS verdict(s) for the $# word(s) it was asked"

# The first word that names a thread decides: found, unknown or ambiguous. A
# bare number is asked as a thread or an issue (kind=thread,issue) and is not a
# thread reference on its own.
VERDICT="$(printf '%s\n' "$ANSWERS" | awk '
  {
    kind = ""; id = ""; token = ""
    for (i = 2; i <= NF; i++) {
      if ($i ~ /^kind=/) kind = substr($i, 6)
      else if ($i ~ /^id=/) id = substr($i, 4)
      else if ($i ~ /^token=/) token = substr($i, 7)
    }
  }
  kind != "thread" && kind != "wp" { next }
  $1 == "id" { sub("/.*", "", id); print "found " id; exit }
  $1 == "unknown" { print "unknown " token; exit }
  $1 == "ambiguous" { print "ambiguous " token; exit }
')"

case "$VERDICT" in
  "") no_reference ;;
  "unknown "*)
    echo "❌ Error: Steel thread ${VERDICT#unknown } not found"
    exit 1
    ;;
  "ambiguous "*)
    echo "❌ Error: ${VERDICT#ambiguous } names more than one thread; give more of its id"
    exit 1
    ;;
esac

echo "✅ Steel thread reference found"
echo "Referenced thread: ${VERDICT#found }"
echo "✅ Steel thread ${VERDICT#found } exists"
