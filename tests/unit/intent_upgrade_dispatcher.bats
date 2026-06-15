#!/usr/bin/env bats
# Behavioural guard for the convergent intent_upgrade orchestrator (ST0043). The
# version-case ladder is gone; these assert the orchestrator lands a project at
# the live target stamp, installs/syncs canon, and aborts safely on a failed
# backup. The orchestrator's full contract lives in
# tests/unit/intent_upgrade_orchestrator.bats.

load "../lib/test_helper.bash"

UPGRADE="${INTENT_PROJECT_ROOT}/bin/intent_upgrade"

@test "v2.10.x project lands at current target stamp, not hard-coded 2.11.0" {
  # Regression guard for v2.11.5: migrate_v2_10_x_to_v2_11_0 used to hard-code
  # the stamp to "2.11.0", silently downgrading the recorded intent_version on
  # any project walked up from v2.10.x via the migration path. Fix stamps
  # `get_intent_version` (the live target). This test asserts the project's
  # config carries the live target after upgrade.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-210-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  # Sandbox HOME: the upgrade tail-call syncs skills/agents into ~/.claude.
  setup_fake_home

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

  teardown_fake_home
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "v2.11.x project upgrades to current target without 'Unknown version'" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-211-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  # Sandbox HOME: the upgrade tail-call syncs skills/agents into ~/.claude.
  setup_fake_home

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

  teardown_fake_home
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "v2.11.7+ upgrade installs in-whiteboard skill" {
  # Regression guard for v2.11.7: intent_upgrade auto-installs the
  # in-whiteboard skill after the case dispatcher completes, so multi-session
  # coordination is available without a manual `intent claude skills install`
  # step. The project is stamped at 2.10.0 with no languages field so the
  # migration chain actually runs (rather than short-circuiting at the
  # "already at target" check, which would skip the auto-install entirely).
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-2117-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  # Fake HOME so the install writes into a sandbox, never the real ~/.claude.
  setup_fake_home

  mkdir -p intent/.config intent/llm intent/st
  # No `languages` field -- forces needs_v2_11_0_upgrade to return 0 so the
  # migration fires (rather than the dispatcher short-circuiting).
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.10.0","project_name":"Upgrade2117Test","author":"t","created":"2026-04-27","st_prefix":"ST"}
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
  assert_output_contains "Ensuring in-whiteboard skill is installed"

  [ -f "$HOME/.claude/skills/in-whiteboard/SKILL.md" ] \
    || fail "expected $HOME/.claude/skills/in-whiteboard/SKILL.md after upgrade"

  teardown_fake_home
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "v2.11.11+ upgrade wires canon subagent sync" {
  # Regression guard for v2.11.11: the critic-<lang> rule-resolution fix is
  # only effective if the corrected agent.md files re-sync to already-installed
  # ~/.claude/agents/ mirrors. The upgrade path must call `claude subagents
  # sync` (failure-tolerant, no --force) alongside the skills sync.
  run grep -F 'intent" claude subagents sync' "$UPGRADE"
  assert_success
}

@test "v2.11.11+ upgrade narrates subagent sync at runtime" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-21111-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1

  setup_fake_home

  mkdir -p intent/.config intent/llm intent/st
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.10.0","project_name":"Upgrade21111Test","author":"t","created":"2026-04-27","st_prefix":"ST"}
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
  assert_output_contains "Syncing canon subagent updates"

  teardown_fake_home
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}

@test "upgrade aborts before migration when backup cannot be created" {
  # ST0042 T3 regression guard: backup used `cp -r ... 2>/dev/null || true`
  # then printed "Backup created successfully" unconditionally, immediately
  # before destructive migration. A failed backup must abort with nothing
  # migrated.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-test-upgrade-bk-XXXXXX)"
  cd "${TEST_TEMP_DIR}" || exit 1
  setup_fake_home

  mkdir -p intent/.config intent/llm intent/st
  cat > intent/.config/config.json <<'EOF'
{"intent_version":"2.10.0","project_name":"BackupFailTest","author":"t","created":"2026-04-27","st_prefix":"ST"}
EOF
  echo "# wip" > intent/wip.md

  git init -q .
  git config user.email t@t.com
  git config user.name Tester
  git add -A
  git commit -qm "init"

  # Make .backup unusable as a directory so the backup copy fails.
  touch .backup

  run "${INTENT_BIN_DIR}/intent" upgrade
  assert_failure
  refute_output_contains "Backup created successfully"

  # Nothing migrated: stamp untouched.
  local stamped
  stamped=$(jq -r '.intent_version' intent/.config/config.json)
  [ "$stamped" = "2.10.0" ] || fail "expected untouched stamp 2.10.0, got '$stamped'"

  teardown_fake_home
  cd "${INTENT_PROJECT_ROOT}" || exit 1
  rm -rf "${TEST_TEMP_DIR}"
}
