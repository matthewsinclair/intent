#!/usr/bin/env bats
# ST0043: intent upgrade as a convergent orchestrator (Architecture B).
#
# Red-first ATs cited by intent/st/ST0043/acceptance.md (AT-00.1, AT-01.1..01.7).
# These are RED against the current version-case-ladder bin/intent_upgrade and go
# GREEN once it becomes the orchestrator (+ bin/intent_migrations exists, the
# canon engine drops VERSION_BUMP, and migration code leaves intent_helpers).
#
# AT-00.1 and AT-01.5 are PRESERVE-guards: the happy path and the verified-backup
# abort already work today; the rewrite must not regress them (green throughout).

load "../lib/test_helper.bash"

UPGRADE="${INTENT_PROJECT_ROOT}/bin/intent_upgrade"
MIGRATIONS="${INTENT_PROJECT_ROOT}/bin/intent_migrations"
HELPERS="${INTENT_PROJECT_ROOT}/bin/intent_helpers"
CANON="${INTENT_PROJECT_ROOT}/intent/plugins/claude/bin/intent_claude_upgrade"

# Scaffold a project: $1 = config path, $2 = stamp, $3 = extra json (eg ',"languages":[]').
_scaffold() {
  local cfg="$1" stamp="$2" extra="$3"
  mkdir -p "$(dirname "$cfg")" intent/llm intent/st
  cat > "$cfg" <<EOF
{"intent_version":"$stamp","project_name":"OrchTest","author":"t","created":"2026-05-01","st_prefix":"ST"$extra}
EOF
  echo "# wip" > intent/wip.md
  printf '# Rules\n\n## Project-Specific Rules\n\nstuff\n' > intent/llm/RULES.md
  git init -q .; git config user.email t@t.com; git config user.name Tester; git add -A; git commit -qm init
}

@test "convergent upgrade from a v2.9.x project lands at target (relocate then languages then canon then stamp-once)" {
  # PRESERVE-guard: the happy path the current ladder already produces.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-e2e-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold ".intent/config.json" "2.9.0" ""
  run "$INTENT_BIN" upgrade --no-backup
  assert_success
  [ -f intent/.config/config.json ] || fail "expected relocated intent/.config/config.json"
  [ ! -d .intent ] || fail "expected old .intent/ removed"
  local target; target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  [ "$(jq -r '.intent_version' intent/.config/config.json)" = "$target" ] || fail "stamp != target"
  jq -e 'has("languages")' intent/.config/config.json >/dev/null || fail "expected languages field"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "interrupted upgrade re-run completes only the remaining work via state probe" {
  # F-UPG-3: stamp was bumped to 2.11.0 but the .intent/ -> intent/.config/
  # relocation never happened. The current ladder dispatches on the stamp and
  # runs ONLY the languages migration -- never relocating -- a silent half-
  # migration. The orchestrator must probe on-disk state and relocate.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-interrupt-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold ".intent/config.json" "2.11.0" ""
  run "$INTENT_BIN" upgrade --no-backup
  assert_success
  [ -f intent/.config/config.json ] || fail "expected relocation to complete on re-run"
  [ ! -d .intent ] || fail "expected stale .intent/ removed on re-run"
  local target; target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  [ "$(jq -r '.intent_version' intent/.config/config.json)" = "$target" ] || fail "stamp != target"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "no ledger step writes the version; only the orchestrator stamps, last" {
  [ -f "$MIGRATIONS" ] || fail "expected bin/intent_migrations (upgrade-only ledger scope)"
  # The jq that writes the field now lives in intent_helpers::stamp_project_version,
  # shared with bin/release. The invariant is unchanged -- within the upgrade path
  # the orchestrator is the only thing that stamps -- so the guard follows the
  # stamp to its new home rather than looking for the raw field write here.
  grep -q 'stamp_project_version' "$UPGRADE" || fail "orchestrator must invoke the stamper"
  if grep -qE '\.intent_version[[:space:]]*=' "$MIGRATIONS"; then
    fail "a ledger step writes intent_version; only the orchestrator may stamp"
  fi
  # A ledger step could now stamp by CALLING the shared helper rather than by
  # hand-rolling the jq. Close that route too.
  if grep -q 'stamp_project_version' "$MIGRATIONS"; then
    fail "a ledger step invokes the stamper; only the orchestrator may stamp"
  fi
}

@test "future or unknown version does not hard-fail before mutation" {
  if grep -q 'Unknown version' "$UPGRADE"; then
    fail "orchestrator must not hard-fail on unknown versions"
  fi
  if grep -qE '"2\.[0-9]+\.[0-9x]+"\)' "$UPGRADE"; then
    fail "orchestrator must have no enumerated version-case arms"
  fi
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-future-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.10.5" ',"languages":[]'
  run "$INTENT_BIN" upgrade --no-backup
  assert_success
  refute_output_contains "Unknown version"
  local target; target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  [ "$(jq -r '.intent_version' intent/.config/config.json)" = "$target" ] || fail "stamp != target"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "semver sanity refuses downgrade and missing VERSION before any mutation" {
  # (a) downgrade: current > target must error before any backup/mutation.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-downgrade-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "9.9.9" ',"languages":[]'
  run "$INTENT_BIN" upgrade
  assert_failure
  [ ! -d .backup ] || fail "downgrade must be refused BEFORE any backup/mutation"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
  # (b) no detectable version: error before mutation.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-nover-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  mkdir -p intent/.config intent/llm intent/st
  echo '{"project_name":"NoVer","author":"t","st_prefix":"ST"}' > intent/.config/config.json
  git init -q .; git config user.email t@t.com; git config user.name Tester; git add -A; git commit -qm init
  run "$INTENT_BIN" upgrade
  assert_failure
  [ ! -d .backup ] || fail "missing version must be refused BEFORE any backup/mutation"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "backup failure surfaces via error() and aborts before mutation" {
  # PRESERVE-guard (ST0042 T3): a failed backup aborts, stamp untouched.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-backup-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.10.0" ""
  touch .backup   # .backup is now a file, so the backup dir copy fails
  run "$INTENT_BIN" upgrade
  assert_failure
  refute_output_contains "Backup created successfully"
  [ "$(jq -r '.intent_version' intent/.config/config.json)" = "2.10.0" ] || fail "stamp must be untouched after a failed backup"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "verified backup tolerates a broken symlink in the source (cp -R, not -r)" {
  # Regression: a stale/broken symlink in the backed-up tree (eg a pre-v2.10.0
  # cross-repo link to a since-moved file -- surfaced on MeetZaya) must not abort
  # the upgrade. BSD/macOS `cp -r` FOLLOWS the link and fails on the dead target;
  # `cp -R` preserves it. Red on macOS against `cp -r`; green with `cp -R`.
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-orch-symlink-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.12.0" ',"languages":["shell"]'
  ln -s /no/such/target/usage-rules.md intent/llm/broken.md   # dangling symlink
  run "$INTENT_BIN" upgrade
  assert_success
  [ -d .backup ] || fail "backup should have been created"
  find .backup -name broken.md -type l | grep -q . || fail "broken symlink must be preserved in the backup, not followed"
  local target; target=$(cat "${INTENT_PROJECT_ROOT}/VERSION")
  [ "$(jq -r '.intent_version' intent/.config/config.json)" = "$target" ] || fail "stamp != target"
  teardown_fake_home; cd "${INTENT_PROJECT_ROOT}" || exit 1; rm -rf "$TEST_TEMP_DIR"
}

@test "only intent_upgrade writes the stamp and the canon engine carries no version-bump" {
  if grep -q 'VERSION_BUMP' "$CANON"; then
    fail "canon engine must not carry VERSION_BUMP (orchestrator is sole stamper)"
  fi
  grep -q 'stamp_project_version' "$UPGRADE" || fail "orchestrator must invoke the stamper"
  # And it must route through the shared helper rather than re-growing its own
  # copy of the write -- the duplication that let bin/release ship without one.
  if grep -qE "jq .*\.intent_version[[:space:]]*=" "$UPGRADE"; then
    fail "orchestrator hand-rolls the stamp; it must route through stamp_project_version"
  fi
}

@test "migration code is not sourced into non-upgrade commands" {
  if grep -qE '^(migrate_v|needs_v|intent_relocate_dotintent)' "$HELPERS"; then
    fail "migration code must move out of intent_helpers into bin/intent_migrations"
  fi
  grep -qE '^detect_project_version' "$HELPERS" || fail "detect_project_version must stay shared in intent_helpers"
  [ -f "$MIGRATIONS" ] || fail "expected bin/intent_migrations"
  grep -qE 'step_[a-z_]+_run|intent_relocate_dotintent' "$MIGRATIONS" || fail "ledger/migration code must live in bin/intent_migrations"
}

@test "the ledger converges the Language Packs block via lang sync, never lang init" {
  # `intent upgrade` is the CONVERGENT orchestrator, so a stale tool-managed
  # block is exactly what it should drive to target. Before the lang_packs step
  # the issue-0005 correction only reached a project that happened to re-run
  # `intent lang init` by hand, leaving every consumer stale.
  grep -q 'lang_packs' "$UPGRADE" || fail "lang_packs is not in the LEDGER"
  grep -q '^step_lang_packs_needs' "$MIGRATIONS" || fail "step_lang_packs_needs missing"
  grep -q '^step_lang_packs_run' "$MIGRATIONS" || fail "step_lang_packs_run missing"
  grep -q '^step_lang_packs_verify' "$MIGRATIONS" || fail "step_lang_packs_verify missing"

  # It must delegate to `lang sync`. `lang init` copies RULES-<lang>.md over
  # whatever is on disk, and projects hand-edit those files -- running it
  # unattended during an upgrade would destroy that work.
  grep -q 'lang sync' "$MIGRATIONS" || fail "the step must delegate to 'intent lang sync'"
  if grep -qE '"\$INTENT_BIN/intent" lang init' "$MIGRATIONS"; then
    fail "a ledger step calls 'lang init', which overwrites hand-edited RULES-<lang>.md"
  fi
}

@test "the final stamp survives set -e when the config is already at target" {
  # The orchestrator runs under `set -e`, and the stamper returns 1 for
  # already-at-target. A bare call plus `case $?` would abort the run before it
  # reported anything: all the work done, then a silent non-zero exit.
  if grep -qE '^stamp_project_version "\$TARGET_VERSION"' "$UPGRADE"; then
    fail "stamper is called bare under set -e; a return of 1 aborts the orchestrator"
  fi
  grep -q 'if stamp_project_version' "$UPGRADE" || fail "the stamp must be called in a condition so set -e cannot abort it"
}

# --- FENCES (2026-08-24 config sweep) -------------------------------------
#
# These exist because the fix they guard was WRONG ON THE FIRST CUT and only a
# whole-tree fingerprint caught it. `--dry-run` gated the ledger, the canon call
# and the stamp but NOT the backup, so the "dry" run wrote .backup/ and took a
# clean fixture to 1 dirty path -- while printing "dry run: nothing was
# modified". Reading the output would have passed it. Only comparing the tree
# to itself failed it.

@test "dry-run mutates NOTHING: whole-tree fingerprint is identical before and after" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-dryrun-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.9.0" ',"languages":[]'

  local before after
  before="$(find . -path ./.git -prune -o -type f -print | LC_ALL=C sort | xargs shasum 2>/dev/null | shasum | cut -d' ' -f1)"
  run "$INTENT_BIN" upgrade --dry-run
  assert_success
  after="$(find . -path ./.git -prune -o -type f -print | LC_ALL=C sort | xargs shasum 2>/dev/null | shasum | cut -d' ' -f1)"

  [ "$before" = "$after" ] || fail "dry-run changed the tree: $before -> $after"
}

@test "dry-run takes NO backup: the backup is itself a mutation" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-dryrun-bk-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.9.0" ',"languages":[]'

  run "$INTENT_BIN" upgrade --dry-run
  assert_success
  [ ! -d .backup ] || fail "dry-run created .backup/ -- the exact defect this flag exists to prevent"
}

@test "dry-run leaves the version stamp untouched" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-dryrun-stamp-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.9.0" ',"languages":[]'

  run "$INTENT_BIN" upgrade --dry-run
  assert_success
  local stamp; stamp="$(jq -r '.intent_version' intent/.config/config.json)"
  [ "$stamp" = "2.9.0" ] || fail "dry-run stamped the project: expected 2.9.0, got $stamp"
}

# POSITIVE CONTROL. Without this the three tests above pass on a --dry-run that
# does nothing at all, including when the flag silently stops the upgrade from
# working. A fence that cannot fail is not a fence.
@test "positive control: the SAME fixture without --dry-run does mutate and stamp" {
  TEST_TEMP_DIR="$(mktemp -d /tmp/intent-dryrun-pc-XXXXXX)"; cd "$TEST_TEMP_DIR" || exit 1
  setup_fake_home
  _scaffold "intent/.config/config.json" "2.9.0" ',"languages":[]'

  run "$INTENT_BIN" upgrade --no-backup
  assert_success
  local target stamp
  target="$(cat "${INTENT_PROJECT_ROOT}/VERSION")"
  stamp="$(jq -r '.intent_version' intent/.config/config.json)"
  [ "$stamp" = "$target" ] || fail "apply path did not stamp: expected $target, got $stamp"
}
