#!/usr/bin/env bash
# verify-canonical.sh -- does a project match the v3 canonical target state?
#
# vc's instrument for the fleet cutover. Reports per-check PASS/FAIL and exits
# non-zero if any check fails. READ-ONLY: it never writes to the project.
#
# WHY THIS EXISTS RATHER THAN A READ-BY-EYE: fifteen projects checked against a
# prose spec is the weakest instrument this estate owns, and it is the one that
# has been wrong repeatedly today. This can be re-run, and it can be shown to
# FAIL -- see --self-test, which is the only reason to believe a PASS.
set -uo pipefail

CANON_SETTINGS="${INTENT_SRC:-/Users/matts/Devel/prj/Intent}/lib/templates/.claude/settings.json"
fails=0
proj="${1:-}"

say() { printf '  %-7s %s\n' "$1" "$2"; }
ok()  { say PASS "$1"; }
bad() { say FAIL "$1"; fails=$((fails+1)); }

check_project() {
  local p="$1"
  printf '\n=== %s ===\n' "$p"

  [ -d "$p" ] || { bad "project directory does not exist"; return; }

  # 1. settings.json: three hooks, all dispatching the CLI door
  local s="$p/.claude/settings.json"
  if [ ! -f "$s" ]; then
    bad ".claude/settings.json missing"
  else
    local door abs rel
    door=$(grep -c 'intent claude hook' "$s")
    abs=$(grep -c 'lib/templates/.claude/scripts' "$s")
    rel=$(grep -c 'CLAUDE_PROJECT_DIR' "$s")
    [ "$door" -eq 3 ] && ok "settings.json: 3 hooks via the CLI door" \
                      || bad "settings.json: $door hooks via the door (want 3)"
    [ "$abs" -eq 0 ]  && ok "settings.json: no absolute checkout paths" \
                      || bad "settings.json: $abs absolute checkout path(s)"
    [ "$rel" -eq 0 ]  && ok "settings.json: no \$CLAUDE_PROJECT_DIR indirection" \
                      || bad "settings.json: $rel \$CLAUDE_PROJECT_DIR indirection(s)"
  fi

  # 2. config.json: stamped v3, required keys present
  local c="$p/intent/.config/config.json"
  if [ ! -f "$c" ]; then
    bad "intent/.config/config.json missing"
  else
    local v; v=$(jq -r '.intent_version // "absent"' "$c" 2>/dev/null)
    case "$v" in
      3.*) ok "config: intent_version $v" ;;
      *)   bad "config: intent_version $v (want 3.x)" ;;
    esac
    local k
    for k in project_name author intent_dir languages; do
      jq -e --arg k "$k" 'has($k)' "$c" >/dev/null 2>&1 \
        && ok "config: $k present" || bad "config: $k MISSING"
    done
  fi

  # 3. root canon
  local f
  for f in AGENTS.md CLAUDE.md; do
    [ -f "$p/$f" ] && ok "root canon: $f" || bad "root canon: $f MISSING"
  done

  # 4. pre-commit chained
  if [ -f "$p/.git/hooks/pre-commit" ]; then
    grep -q 'intent-chain-block' "$p/.git/hooks/pre-commit" \
      && ok "pre-commit: chain block present" \
      || bad "pre-commit: chain block ABSENT"
  else
    bad "pre-commit: hook not installed"
  fi
}

# --self-test: prove the instrument can FAIL before believing any PASS.
if [ "$proj" = "--self-test" ]; then
  t=$(mktemp -d)
  mkdir -p "$t/.claude" "$t/intent/.config" "$t/.git/hooks"
  # FIXTURE MUST TRIP EVERY ARM. dc, 2026-08-26: the first fixture had ZERO
  # hooks, so "no absolute paths" and "no $CLAUDE_PROJECT_DIR" passed 0-of-0 --
  # a green wearing the shape of coverage, INSIDE the control built to prevent
  # exactly that. Both arms now have something the fixture actually refuses.
  cat > "$t/.claude/settings.json" <<'FIX'
{"hooks":{"SessionStart":[{"hooks":[{"command":"/Users/x/Devel/prj/Intent/lib/templates/.claude/scripts/session-context.sh"}]}],
"UserPromptSubmit":[{"hooks":[{"command":"/Users/x/Devel/prj/Intent/lib/templates/.claude/scripts/require-in-session.sh"}]}],
"Stop":[{"hooks":[{"command":"${CLAUDE_PROJECT_DIR:-.}/.claude/scripts/x.sh"}]}],
"PreToolUse":[{"hooks":[{"command":"${CLAUDE_PROJECT_DIR:-.}/.claude/scripts/y.sh"}]}]}}
FIX
  printf '{"intent_version":"2.19.0"}\n' > "$t/intent/.config/config.json"
  printf '#!/bin/sh\n' > "$t/.git/hooks/pre-commit"
  check_project "$t"
  rm -rf "$t"
  printf '\nself-test: %s failure(s) detected.\n' "$fails"
  [ "$fails" -ge 11 ] && { echo "SELF-TEST PASS -- the instrument reports failures."; exit 0; }
  echo "SELF-TEST FAIL -- instrument did not detect a known-bad project. DO NOT TRUST ITS PASSES."; exit 1
fi

[ -n "$proj" ] || { echo "usage: verify-canonical.sh <project-dir> | --self-test" >&2; exit 2; }
check_project "$proj"
printf '\n%s: %s check(s) failed.\n' "$(basename "$proj")" "$fails"
[ "$fails" -eq 0 ]
