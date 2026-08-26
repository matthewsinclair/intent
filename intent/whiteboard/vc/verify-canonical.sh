#!/usr/bin/env bash
# verify-canonical.sh -- does a project match the v3 canonical target state?
#
# vc's instrument for the fleet cutover. Reports per-check PASS/FAIL and exits
# non-zero if any check fails. READ-ONLY: it never writes to the project.
#
# WHY THIS EXISTS RATHER THAN A READ-BY-EYE: twenty-one projects checked against
# a prose spec is the weakest instrument this estate owns, and it is the one
# that has been wrong repeatedly today. This can be re-run, and it can be shown
# to FAIL -- see --self-test, which is the only reason to believe a PASS.
#
# THE CANON IT COMPARES AGAINST IS PRINTED ON EVERY RUN. After the brew install
# the binary doing the writing carries its own lib/templates in the Cellar; if
# that ever disagrees with the checkout, the disagreement must name its two
# subjects rather than read as a project defect (cc). Override with INTENT_CANON.
set -uo pipefail

CANON_SETTINGS="${INTENT_CANON:-${INTENT_SRC:-/Users/matts/Devel/prj/Intent}/lib/templates/.claude/settings.json}"
fails=0
proj="${1:-}"
printf 'canon: %s\n' "$CANON_SETTINGS"

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
    # cc, 2026-08-26: the three arms above are PROXIES that name HOW a file
    # diverged. They passed 11/11 on a file with canon's events and doors and
    # the wrong matcher and timeouts -- a project in exactly the state this
    # cutover exists to eliminate. Only a byte comparison sees VALUES.
    if cmp -s "$s" "$CANON_SETTINGS"; then
      ok "settings.json: byte-for-byte identical to canon"
    else
      bad "settings.json: DIFFERS from canon"
    fi
  fi

  # 2. config.json: stamped v3, required keys present.
  # intent_dir is NOT required: v3 defaults it (project.rs default_intent_dir)
  # and stamp_version does not write it, so requiring it demanded a key the
  # tool cannot produce. An absent key reading as `intent` IS the canonical
  # configuration.
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
    for k in project_name author languages; do
      jq -e --arg k "$k" 'has($k)' "$c" >/dev/null 2>&1 \
        && ok "config: $k present" || bad "config: $k MISSING"
    done
  fi

  # 3. root canon
  local f
  for f in AGENTS.md CLAUDE.md; do
    [ -f "$p/$f" ] && ok "root canon: $f" || bad "root canon: $f MISSING"
  done

  # 4. pre-commit chained EXACTLY ONCE. cc, 2026-08-26: `claude upgrade` looked
  # for the angle-form marker, every v2-written hook carries the colon form, so
  # it wrote a SECOND block and the idempotence control certified the doubled
  # state. `grep -q` passed it because it matches either form and both copies.
  # Anchored to the marker LINE shape, not a substring: Laksa's hook has a prose
  # line mentioning intent-chain-block that a substring match reads as a block.
  # devbin-cc, 2026-08-26: `core.hooksPath` makes `.git/hooks/` INERT. Laksa
  # sets `bin/hooks`; Intent itself sets `.githooks`. A tool that writes
  # `.git/hooks/pre-commit` there reports success into a file git never runs,
  # and this verifier was reading the same inert file. Resolve the dir the way
  # git does. The path git prints is relative to the project unless absolute.
  local hooksdir h
  hooksdir=$(git -C "$p" rev-parse --git-path hooks 2>/dev/null || echo ".git/hooks")
  case "$hooksdir" in /*) ;; *) hooksdir="$p/$hooksdir" ;; esac
  h="$hooksdir/pre-commit"
  printf '  %-7s %s\n' 'info' "hooks dir git runs: $hooksdir"
  if [ -f "$h" ]; then
    local starts ends
    starts=$(grep -cE '^# (intent-chain-block:start|>>> intent-chain-block >>>)' "$h")
    ends=$(grep -cE '^# (intent-chain-block:end|<<< intent-chain-block <<<)' "$h")
    if [ "$starts" -eq 1 ] && [ "$ends" -eq 1 ]; then
      ok "pre-commit: chain block present exactly once"
    else
      bad "pre-commit: $starts start / $ends end marker(s) (want 1/1)"
    fi
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
  # exactly that. Every arm now has something the fixture actually refuses,
  # including a DOUBLED hook, which is the fleet's real failure mode.
  cat > "$t/.claude/settings.json" <<'FIX'
{"hooks":{"SessionStart":[{"hooks":[{"command":"/Users/x/Devel/prj/Intent/lib/templates/.claude/scripts/session-context.sh"}]}],
"UserPromptSubmit":[{"hooks":[{"command":"/Users/x/Devel/prj/Intent/lib/templates/.claude/scripts/require-in-session.sh"}]}],
"Stop":[{"hooks":[{"command":"${CLAUDE_PROJECT_DIR:-.}/.claude/scripts/x.sh"}]}],
"PreToolUse":[{"hooks":[{"command":"${CLAUDE_PROJECT_DIR:-.}/.claude/scripts/y.sh"}]}]}}
FIX
  printf '{"intent_version":"2.19.0"}\n' > "$t/intent/.config/config.json"
  cat > "$t/.git/hooks/pre-commit" <<'FIX'
#!/bin/sh
# intent-chain-block:start (generated by intent claude upgrade)
"$_intent_chain" "$@" || exit $?
# intent-chain-block:end
# >>> intent-chain-block >>>
"$_intent_chain" "$@" || exit $?
# <<< intent-chain-block <<<
FIX
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
