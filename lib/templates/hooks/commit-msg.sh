#!/usr/bin/env bash
# commit-msg.sh -- Intent attribution guard.
#
# REFUSES any commit message carrying AI/Claude attribution. `CLAUDE.md` states
# the rule in capitals and without exception -- "DO NOT ADD CLAUDE TO GIT
# COMMITS. EVER. No Co-Authored-By lines, no Claude signatures, no AI
# attribution in commit messages" -- and until this hook existed the rule was
# enforced by nothing at all.
#
# MEASURED BEFORE IT WAS WRITTEN (Intent, 2026-09-09): 1,789 commits in this
# repository carry a `Claude-Session:` trailer and one carries a
# `Co-Authored-By` naming Claude. At least 392 are already published. The
# practice stopped on 2026-09-07 for reasons outside this tree -- which is
# precisely the case a guard exists for: nothing would stop it resuming, and
# nothing would report it if it did.
#
# HISTORY IS NOT REWRITTEN. This hook prevents the next one; it does not repair
# the 1,789, and rewriting them would invalidate every commit citation in canon,
# on five boards and in every issue body.
#
# It NEVER edits the message. A guard that silently strips the line hides the
# class from whoever needs to learn their tooling is adding it; this one prints
# the offending line so the fix is a deletion.

set -euo pipefail

msg_file="${1:-}"
if [ -z "$msg_file" ] || [ ! -f "$msg_file" ]; then
  echo "commit-msg: no message file handed to the hook; refusing rather than passing." >&2
  exit 1
fi

# Comment lines are git's own scissors/help text and are never committed, so
# they are stripped before matching -- otherwise a template mentioning the rule
# would refuse every commit.
body="$(grep -v '^#' "$msg_file" || true)"

# Each pattern is anchored at what it actually matches. `Claude-Session` and
# `Co-Authored-By` are trailers; the generated-with line is free prose.
patterns='^[[:space:]]*Claude-Session:|^[[:space:]]*Co-[Aa]uthored-[Bb]y:.*([Cc]laude|[Aa]nthropic)|[Gg]enerated with.*[Cc]laude|🤖.*[Cc]laude'

hits="$(printf '%s\n' "$body" | grep -nEi "$patterns" || true)"

if [ -n "$hits" ]; then
  echo "commit-msg: REFUSED -- this message carries AI attribution." >&2
  echo "" >&2
  printf '%s\n' "$hits" | sed 's/^/    /' >&2
  echo "" >&2
  echo "  CLAUDE.md: DO NOT ADD CLAUDE TO GIT COMMITS. EVER. No Co-Authored-By" >&2
  echo "  lines, no Claude signatures, no AI attribution in commit messages." >&2
  echo "" >&2
  echo "  remedy: delete the line(s) above from the commit message. If a tool is" >&2
  echo "  adding them, turn that off -- this hook only catches the ones it sees." >&2
  exit 1
fi

exit 0
