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
WB_GUARDS=(
  'whiteboard-clock-guard.sh|timestamps are UNCHECKED'
  'whiteboard-header-guard.sh|header values are UNCHECKED'
)

if [ -d "intent/whiteboard" ]; then
  INTENT_HOME_RESOLVED="$(intent info 2>/dev/null | sed -n 's/^ *INTENT_HOME: *//p' | head -1)"
  WB_BLOCKED=0
  for wb_entry in "${WB_GUARDS[@]}"; do
    wb_name="${wb_entry%%|*}"
    wb_unchecked="${wb_entry#*|}"
    wb_guard="${INTENT_HOME_RESOLVED}/lib/templates/hooks/${wb_name}"
    if [ -n "$INTENT_HOME_RESOLVED" ] && [ -f "$wb_guard" ]; then
      bash "$wb_guard" || WB_BLOCKED=1
    else
      # Named, not silent: a board present with no guard behind it is exactly
      # the invisible non-enforcement this whole mechanism exists to end.
      echo "intent gate: intent/whiteboard/ present but ${wb_name} was not found;" >&2
      echo "  ${wb_unchecked} this commit. (looked in: ${wb_guard})" >&2
    fi
  done
  [ "$WB_BLOCKED" -eq 0 ] || exit 1
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
    case "$rc" in
      0) ;;
      1)
        printf '%s\n' "$out" >&2
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
