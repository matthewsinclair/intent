#!/usr/bin/env bash
#
# index-freshness.bash -- can the index answer for these paths?
#
# ONE FUNCTION, AND NO HOOK OWNS A FRESHNESS RULE OF ITS OWN. The PostToolUse
# hook beside this file appends nothing when the index is not complete for the
# paths involved. A PreToolUse redirect of the same kind, if Intent ever ships
# one, would fire only when it IS complete for the paths a pattern would have
# reached. Same rule, and one implementation, so the two cannot drift.
#
# AND THE FIELD NAMED `complete` IS NOT THAT RULE, WHICH IS THE WHOLE REASON
# THIS FILE EXISTS. `index.complete` is `skipped` and `stale` both being empty --
# a claim about the WHOLE index -- so ONE unreadable file anywhere in the tree
# makes it false for a query about a path nowhere near it. Read as the rule, it
# silences a hook that had a perfectly good answer, and the failure is invisible:
# a hook that says nothing looks exactly like a hook with nothing to say.
#
# The envelope already carries what the criteria actually ask for. `skipped` and
# `stale` are lists of PATHS, so complete FOR THESE PATHS means no entry in
# either list beneath them. `complete` is kept as a fast path -- when it is true
# both lists are empty and no prefix can match -- and never as the rule.
#
# NOT A HOOK, AND THE EXTENSION IS WHY. Every `*.sh` in this directory is a hook
# `intent claude hook <name>` can run. That rule is right and this file is not a
# hook, so it carries a different extension rather than being carved out of a
# population by a predicate -- a filtered exemption and an accident look
# identical to the filter.
#
# Sourced, never executed:
#   . "${BASH_SOURCE[0]%/*}/index-freshness.bash"

# intent_index_answers_for <envelope-json> <path-prefix>
#
# 0 when the index is complete for everything beneath <path-prefix>, 1 otherwise.
# An empty or `.` prefix asks about the whole project.
#
# REFUSES RATHER THAN GUESSES. An envelope this cannot read -- no `index` block,
# jq absent, malformed JSON -- returns 1, because the caller's question is *may I
# trust this*, and an unreadable answer is not a yes.
intent_index_answers_for() {
  local envelope="$1" prefix="${2:-}"

  command -v jq >/dev/null 2>&1 || return 1
  [ -n "$envelope" ] || return 1

  printf '%s' "$envelope" | jq -e 'has("index") and (.index | has("skipped") and has("stale"))' >/dev/null 2>&1 || return 1

  # The fast path, and it is only ever a shortcut for the rule below: `complete`
  # is true exactly when both lists are empty, in which case no prefix matches.
  if printf '%s' "$envelope" | jq -e '.index.complete == true' >/dev/null 2>&1; then
    return 0
  fi

  case "$prefix" in
    ""|".") prefix="" ;;
    */) prefix="${prefix%/}" ;;
  esac

  # A path is BENEATH the prefix when it equals it or continues it after a `/`.
  # Compared as strings after both are normalised, so `src` does not match
  # `srcfoo` -- the trap a bare `startswith` walks into.
  printf '%s' "$envelope" | jq -e --arg p "$prefix" '
    def beneath($path): $p == "" or $path == $p or ($path | startswith($p + "/"));
    [ (.index.skipped // [] | .[] | .path), (.index.stale // [] | .[]) ]
    | map(select(beneath(.)))
    | length == 0
  ' >/dev/null 2>&1
}
