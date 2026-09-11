#!/usr/bin/env bash
# commit-msg.sh -- Intent attribution guard.
#
# Reached only through this repository's tracked `.githooks/commit-msg`; no
# Intent verb installs it into another project.
#
# REFUSES any commit message carrying AI/Claude attribution. `CLAUDE.md` states
# the rule in capitals and without exception -- "DO NOT ADD CLAUDE TO GIT
# COMMITS. EVER. No Co-Authored-By lines, no Claude signatures, no AI
# attribution in commit messages" -- and until this hook existed the rule was
# enforced by nothing at all.
#
# MEASURED OVER THE WHOLE HISTORY, ANCHORED, BY SEPARATE NODES INDEPENDENTLY
# (Intent, 2026-09-09): commits carrying a `Claude-Session:` trailer exist, many
# of them already published --
# `git log --grep='^[[:space:]]*Claude-Session:' --oneline | wc -l` reports how
# many. None carries an anchored `Co-Authored-By` naming Claude, a
# generated-with line or the robot emoji.
# The practice stopped on 2026-09-07 for reasons outside this tree -- which is
# precisely the case a guard exists for: nothing would stop it resuming, and
# nothing would report it if it did.
#
# THE FIRST SHIPPED VERSION OF THIS HEADER CARRIED UNANCHORED COUNTS. Its
# trailer figure counted commits containing the SUBSTRING `Claude-Session`
# anywhere, which includes prose about the trailer; its Co-Authored-By hit was
# `6816e1e94`, a commit QUOTING the prohibition. An instrument that counts
# mentions as violations grows its own population every time somebody documents
# the rule -- and it did: the commit that added this guard became the second
# such "hit" because its message quotes CLAUDE.md.
#
# SO THE GUARD MATCHES TRAILERS AND NEVER PROSE, AND THAT IS THE WHOLE CONTRACT.
# The first version also carried `Generated with.*Claude` and a robot-emoji
# pattern, both unanchored. They refused any message DESCRIBING the rule --
# including the commit that documents this guard and the commit that fixes it,
# whose remedy line then told the author to delete their own sentence. dc drove
# it and reported it. The in-house precedent had already ruled the same way:
# `whiteboard-header-guard.sh` reads header blocks and never prose, because
# "nodes report this class to each other by quoting it, and scanning prose would
# make reporting the defect an offence". A commit message is prose by nature;
# its trailer block is the structured part, so the trailer block is the subject.
#
# DECLARED LIMIT, NOT AN OVERSIGHT: a footer carrying ONLY a robot-emoji or
# generated-with line and NO trailer would pass. Measured at zero occurrences
# across the history, and the standard tool footer emits the `Co-Authored-By:`
# trailer alongside it, which this guard catches. If a trailerless footer is
# ever observed, the fix is a LINE-ANCHORED pattern (`^[[:space:]]*` + the
# form), not a return to substring matching.
#
# HISTORY IS NOT REWRITTEN. This hook prevents the next one; it does not repair
# the existing ones, and rewriting them would invalidate every commit citation in
# canon, on every board and in every issue body.
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

# BOTH patterns are anchored at line start and BOTH require the trailer colon.
# That colon is what separates `Co-Authored-By: Claude <...>` (a trailer) from
# "Co-Authored-By trailers naming Claude" (a sentence about one).
patterns='^[[:space:]]*Claude-Session:|^[[:space:]]*Co-[Aa]uthored-[Bb]y:.*([Cc]laude|[Aa]nthropic)'

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
