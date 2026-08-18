#!/usr/bin/env bash
#
# critic_global_rig.sh -- Half A of the pre-commit critic gate (ST0056).
#
# THE FIX UNDER TEST: add `critic` to GLOBAL_COMMANDS at bin/intent:55 so it
# dispatches BEFORE the version guard at bin/intent:277. Without it, `intent
# critic <lang>` on a v3-declared project hits the guard, exits 2, and the
# pre-commit hook's `*)` branch fails open -- a gate that enforces nothing
# while reporting success.
#
# WHY THIS RIG DOES NOT CLONE THE PROJECT. Control and subject are two copies
# of `bin/intent` in a scratch dir, both pointed at the REAL tree via
# INTENT_HOME, so the only difference between them is the line under test.
# The PROJECT under test is a purpose-built fixture, NOT this repo, because
# test 2 must stage a deliberate violation and this repo's git index is shared
# with concurrent peer sessions.
#
# WHAT THE PREVIOUS RIG GOT WRONG, kept here because the correction is the
# point. Its test 2 asserted `rc=0 or 1` on `critic shell`, labelled "the
# critic actually ran". rc=0 is exactly what the DARK gate returns, and shell
# has 0 of 6 rules carrying a proxy -- so the assertion could not fail before
# the fix, after it, or under any fix. Test 2 below runs `elixir` (the only
# armed pack) against a STAGED violation and asserts rc=1 with a named rule.
# Test 3 is its mandatory partner: the SAME command over a CLEAN file must
# return rc=0, or rc=1 proves nothing about the violation.
#
# Usage:  bash intent/st/ST0056/parity/tools/critic_global_rig.sh
# Exit:   0 all cases pass, 1 any case fails, 2 the rig could not run.

set -euo pipefail

TOOLS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REAL_TREE="$(cd "$TOOLS_DIR/../../../../.." && pwd)"

[ -f "$REAL_TREE/bin/intent" ] || { echo "rig: cannot find bin/intent under $REAL_TREE" >&2; exit 2; }
[ -f "$REAL_TREE/VERSION" ]    || { echo "rig: cannot find VERSION under $REAL_TREE" >&2; exit 2; }

SCRATCH="$(mktemp -d "${TMPDIR:-/tmp}/critic_global_rig.XXXXXX")"
trap 'rm -rf "$SCRATCH"' EXIT

PASS=0
FAIL=0

report() {
  local verdict="$1" name="$2" detail="$3"
  if [ "$verdict" = "PASS" ]; then
    PASS=$((PASS + 1))
    printf '  PASS  %-52s %s\n' "$name" "$detail"
  else
    FAIL=$((FAIL + 1))
    printf '  FAIL  %-52s %s\n' "$name" "$detail"
  fi
}

# ---- Control and subject -------------------------------------------------

cp "$REAL_TREE/bin/intent" "$SCRATCH/intent.orig"
cp "$REAL_TREE/bin/intent" "$SCRATCH/intent.mut"
sed -i '' 's/^GLOBAL_COMMANDS="help doctor/GLOBAL_COMMANDS="critic help doctor/' "$SCRATCH/intent.mut" 2>/dev/null \
  || sed -i 's/^GLOBAL_COMMANDS="help doctor/GLOBAL_COMMANDS="critic help doctor/' "$SCRATCH/intent.mut"
chmod +x "$SCRATCH/intent.orig" "$SCRATCH/intent.mut"

# A change proved against a copy of itself is proved against nothing.
if cmp -s "$SCRATCH/intent.orig" "$SCRATCH/intent.mut"; then
  echo "rig: ABORT -- the mutation did not apply; subject is byte-identical to control." >&2
  echo "rig: bin/intent:55 no longer matches the expected GLOBAL_COMMANDS prefix." >&2
  exit 2
fi

# CANARY MODE. RIG_CANARY=1 drives the CONTROL through every subject case, so
# the rig is asked to produce the failures it claims to detect. A rig that
# passes in both modes is measuring something other than the fix. Expected
# under canary: 1, 4 and 5 PASS (they do not depend on the fix); 2 and 3 FAIL
# (the critic is refused at rc=2 instead of reporting); and 6 FAILS BY THE
# COMMIT SUCCEEDING -- which is the dark gate demonstrated live rather than
# argued.
SUBJECT="$SCRATCH/intent.mut"
SUBJECT_LABEL="subject (fix applied)"
if [ "${RIG_CANARY:-0}" = "1" ]; then
  SUBJECT="$SCRATCH/intent.orig"
  SUBJECT_LABEL="CANARY -- control driven as subject; 2, 3 and 6 MUST fail"
fi

# ---- The fixture project -------------------------------------------------

FIX="$SCRATCH/fixture"
mkdir -p "$FIX/intent/.config" "$FIX/test"
cat > "$FIX/intent/.config/config.json" <<'JSON'
{
  "author": "rig",
  "created": "2026-08-18",
  "intent_dir": "intent",
  "intent_version": "3.0.0-dev",
  "languages": ["elixir"],
  "project_name": "critic_global_rig_fixture",
  "st_prefix": "ST"
}
JSON

# Trips IN-EX-TEST-002 (critical, no-process-sleep), proxy `Process\.sleep\(`.
cat > "$FIX/test/violation_test.exs" <<'EX'
defmodule ViolationTest do
  use ExUnit.Case, async: true

  test "invariant: the rig can produce a non-zero" do
    start_the_worker()
    Process.sleep(100)
    assert worker_done?()
  end
end
EX

cat > "$FIX/test/clean_test.exs" <<'EX'
defmodule CleanTest do
  use ExUnit.Case, async: true

  test "invariant: the discriminator can also produce a zero" do
    assert_receive {:worker_done, :ok}
  end
end
EX

git -C "$FIX" init -q
git -C "$FIX" config user.email rig@example.invalid
git -C "$FIX" config user.name rig
git -C "$FIX" add intent test/clean_test.exs

echo "critic gate -- Half A rig"
echo "  real tree     $REAL_TREE"
echo "  tree commit   $(git -C "$REAL_TREE" rev-parse --short HEAD) ($(git -C "$REAL_TREE" status --porcelain | wc -l | tr -d ' ') files dirty)"
echo "  tool version  $(cat "$REAL_TREE/VERSION")"
echo "  fixture       declares 3.0.0-dev, languages [elixir]"
echo "  mode          $SUBJECT_LABEL"
echo

run() {
  local bin="$1"; shift
  local out rc
  set +e
  out="$(cd "$FIX" && INTENT_HOME="$REAL_TREE" "$bin" "$@" 2>&1)"
  rc=$?
  set -e
  RUN_OUT="$out"
  RUN_RC="$rc"
}

# ---- 1. The control reproduces the defect --------------------------------

run "$SCRATCH/intent.orig" critic elixir --staged
if [ "$RUN_RC" -eq 2 ] && printf '%s' "$RUN_OUT" | grep -q 'declares Intent v3.0.0-dev'; then
  report PASS "1 control: critic refused by version guard" "rc=2, guard message present"
else
  report FAIL "1 control: critic refused by version guard" "rc=$RUN_RC (expected 2 + guard message)"
fi

# ---- 2. THE RED. Subject finds a STAGED violation ------------------------

git -C "$FIX" add test/violation_test.exs
run "$SUBJECT" critic elixir --staged --format text
if [ "$RUN_RC" -eq 1 ] && printf '%s' "$RUN_OUT" | grep -q 'IN-EX-TEST-002'; then
  report PASS "2 subject: staged violation -> rc=1, named rule" "rc=1, IN-EX-TEST-002"
else
  report FAIL "2 subject: staged violation -> rc=1, named rule" "rc=$RUN_RC out=$(printf '%s' "$RUN_OUT" | head -1)"
fi

# ---- 3. The discriminator's other direction ------------------------------

git -C "$FIX" reset -q HEAD -- test/violation_test.exs 2>/dev/null || git -C "$FIX" rm -q --cached test/violation_test.exs
run "$SUBJECT" critic elixir --staged --format text
if [ "$RUN_RC" -eq 0 ]; then
  report PASS "3 subject: clean staged set -> rc=0" "rc=0 (so test 2's rc=1 came from the violation)"
else
  report FAIL "3 subject: clean staged set -> rc=0" "rc=$RUN_RC out=$(printf '%s' "$RUN_OUT" | head -1)"
fi

# ---- 4 & 5. The version guard is NOT weakened ----------------------------

for verb in st wp; do
  run "$SUBJECT" "$verb" list
  if [ "$RUN_RC" -eq 2 ] && printf '%s' "$RUN_OUT" | grep -q 'declares Intent v3.0.0-dev'; then
    report PASS "4/5 subject: '$verb list' STILL refused at rc=2" "guard intact"
  else
    report FAIL "4/5 subject: '$verb list' STILL refused at rc=2" "rc=$RUN_RC (expected 2)"
  fi
done

# ---- 6. END TO END. The hook must BLOCK a commit -------------------------
#
# The proof of Half A is a RED. A green commit after the fix is the same
# observation the dark gate has been emitting since the hoist.

mkdir -p "$SCRATCH/path"
cp "$SUBJECT" "$SCRATCH/path/intent"
chmod +x "$SCRATCH/path/intent"
cp "$REAL_TREE/lib/templates/hooks/pre-commit.sh" "$FIX/.git/hooks/pre-commit"
chmod +x "$FIX/.git/hooks/pre-commit"

git -C "$FIX" add test/violation_test.exs
set +e
COMMIT_OUT="$(cd "$FIX" && PATH="$SCRATCH/path:$PATH" INTENT_HOME="$REAL_TREE" git commit -m 'rig: this commit must be REFUSED' 2>&1)"
COMMIT_RC=$?
set -e

if [ "$COMMIT_RC" -ne 0 ] && printf '%s' "$COMMIT_OUT" | grep -q 'commit blocked by findings'; then
  report PASS "6 END-TO-END: hook BLOCKED the commit" "rc=$COMMIT_RC, gate message present"
else
  report FAIL "6 END-TO-END: hook BLOCKED the commit" "rc=$COMMIT_RC (expected non-zero + block message)"
  printf '%s\n' "$COMMIT_OUT" | sed 's/^/        /' | head -12
fi

echo
echo "  $PASS passed, $FAIL failed"
[ "$FAIL" -eq 0 ] || exit 1
exit 0
