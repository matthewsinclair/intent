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
if [ ! -f ".intent/config.json" ]; then
  # Walk up just in case the hook is invoked from a subdirectory.
  _probe="$PWD"
  while [ "$_probe" != "/" ] && [ -n "$_probe" ]; do
    [ -f "$_probe/.intent/config.json" ] && break
    _probe="$(dirname "$_probe")"
  done
  if [ ! -f "$_probe/.intent/config.json" ]; then
    echo "intent critic gate: not inside an Intent project (.intent/config.json absent); skipping." >&2
    exit 0
  fi
fi

# ---- Detect languages to critique ----

LANGS=()
[ -f "mix.exs" ]                  && LANGS+=(elixir)
[ -f "Cargo.toml" ]               && LANGS+=(rust)
[ -f "Package.swift" ]            && LANGS+=(swift)
[ -f ".luarc.json" ]              && LANGS+=(lua)

# Always include shell so staged bash/zsh scripts are checked regardless
# of the project's primary language.
LANGS+=(shell)

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
for lang in "${LANGS[@]}"; do
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

if [ "$AGGREGATE" -eq 1 ]; then
  echo "" >&2
  echo "intent critic gate: commit blocked by findings at severity >= $SEVERITY." >&2
  echo "  review the findings above, fix them, and re-commit." >&2
  echo "  to bypass (use sparingly): git commit --no-verify" >&2
  exit 1
fi

exit 0
