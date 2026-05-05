#!/usr/bin/env bats
# Regression guard: bin/intent_upgrade's version-case dispatcher must accept
# v2.11.x source versions. v2.11.0 -> v2.11.1 broke because the dispatcher
# had no case for "2.11.0" and fell into `*) error "Unknown version"`. The
# fix uses a "2.11.*" glob; this test pins that contract.

load "../lib/test_helper.bash"

UPGRADE="${INTENT_PROJECT_ROOT}/bin/intent_upgrade"

@test "dispatcher case statement covers v2.11.x via glob" {
  # The pattern must be the glob form `"2.11."*)` so future patches don't
  # require a fresh case each release.
  run grep -E '^[[:space:]]+"2\.11\."\*\)' "$UPGRADE"
  assert_success
}

@test "v2.10.x project lands at current target stamp, not hard-coded 2.11.0" {
  # Regression guard for v2.11.5: migrate_v2_10_x_to_v2_11_0 used to hard-code
  # the stamp to "2.11.0", silently downgrading the recorded intent_version on
  # any project walked up from v2.10.x via the migration path. Fix stamps
  # `get_intent_version` (the live target). This test asserts the project's
  # config carries the live target after upgrade.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-210-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  mkdir -p intent/.config intent/llm intent/st
  # No `languages` field -- forces needs_v2_11_0_upgrade to return 0 so the
  # migration fires (rather than the dispatcher short-circuiting).
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.10.0","project_name":"Upgrade210Test","author":"t","created":"2026-04-27","st_prefix":"ST"}
EOF
  echo "# wip" > intent/wip.md
  cat > intent/llm/RULES.md <<'EOF'
# Rules

## Project-Specific Rules

stuff
EOF

  git init -q .
  git config user.email t@t.com
  git config user.name Tester
  git add -A
  git commit -qm "init"

  run "${INTENT_BIN_DIR}/intent" upgrade --no-backup
  assert_success

  local target
  target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  local stamped
  stamped=$(jq -r '.intent_version' intent/.config/config.json)
  [ "$stamped" = "$target" ] || fail "expected stamp '$target', got '$stamped'"

  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "v2.11.x project upgrades to current target without 'Unknown version'" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-211-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  mkdir -p intent/.config intent/llm intent/st
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.11.0","project_name":"Upgrade211Test","author":"t","created":"2026-04-28","st_prefix":"ST","languages":[]}
EOF
  # Minimal scaffolding required by the upgrade flow.
  echo "# wip" > intent/wip.md
  cat > intent/llm/RULES.md <<'EOF'
# Rules

## Project-Specific Rules

stuff
EOF

  git init -q .
  git config user.email t@t.com
  git config user.name Tester
  git add -A
  git commit -qm "init"

  # Run the upgrade. Expectation: NOT "Unknown version: 2.11.0".
  run "${INTENT_BIN_DIR}/intent" upgrade --no-backup
  refute_output_contains "Unknown version: 2.11.0"

  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}
