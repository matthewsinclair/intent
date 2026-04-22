---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-09
title: "Migration and upgrade chain"
scope: Medium
status: Not Started
---

# WP-09: Migration and upgrade chain

## Objective

Implement `migrate_v2_8_2_to_v2_9_0` in `bin/intent_helpers` and wire it into the upgrade chain so every prior version case in `bin/intent_upgrade` chains through the new migration. Handle version stamp, `~/.intent/ext/` bootstrap, worker-bee seed installation, and aggressive prune of installed copies of deleted subagents (elixir, worker-bee). BATS-cover every starting-version case.

## Context

Intent's upgrade chain is load-bearing: every `migrate_v2_X_Y_to_v2_A_B` function must chain to the tip, and v2.8.2 required a slipstream fix because the 2.6.0/2.7.0 cases had gaps. This is the known-fragile piece, so WP09 builds aggressive test coverage and a canary-project dry-run before release.

Fail-forward migration: we actively delete `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md` if present, and remove their entries from `~/.intent/agents/installed-agents.json`. No preservation. Users who want worker-bee post-upgrade reinstall from the freshly seeded `~/.intent/ext/worker-bee/`.

## Deliverables

### New migration function

- `bin/intent_helpers` — new function `migrate_v2_8_2_to_v2_9_0` per spec below
- `bin/intent_helpers` — new predicate `needs_v2_9_0_upgrade`
- `bin/intent_helpers` — new helper `generate_ext_readme` that produces `~/.intent/ext/README.md` on bootstrap

### Upgrade chain wiring

- `bin/intent_upgrade` — new case `"2.8.2"` that calls only `migrate_v2_8_2_to_v2_9_0`
- `bin/intent_upgrade` — every prior case (0.0.0 through 2.8.1) extended to chain through `migrate_v2_8_2_to_v2_9_0` at the end
- `bin/intent_upgrade:93` gate — extended with `! needs_v2_9_0_upgrade "$VERSION"`
- `VERSION` file staged for update to `2.9.0` (final commit in WP11)

### BATS tests

- `tests/unit/ext_migration.bats` — covers:
  - Fresh v2.8.2 → v2.9.0 migration (full path)
  - Already-migrated no-op (idempotent second run)
  - Installed elixir prune (pre-state file present → post-state file absent)
  - Installed worker-bee prune (same)
  - Ext dir bootstrap (creates `~/.intent/ext/` + README if absent)
  - Worker-bee seed (copies from `lib/templates/ext-seeds/worker-bee/` if target absent)
  - Seed skip when target already exists (user has customised; don't overwrite)
  - Old-version chain case: v2.0.0 → v2.9.0 chains through all migrations

### Dry-run verification

- Three canary fleet projects (e.g. Anvil, Molt, Utilz) — dry-run the migration against each to confirm clean upgrade before WP11 release rollout

## Approach

1. **Implement `migrate_v2_8_2_to_v2_9_0` function** per spec.

2. **Implement `needs_v2_9_0_upgrade` predicate.**

3. **Implement `generate_ext_readme` helper** producing a README that explains `~/.intent/ext/`, how to list extensions, how to create one, and that worker-bee is the worked example.

4. **Extend existing migration chain.** Every case in `bin/intent_upgrade:136-295` must chain to `migrate_v2_8_2_to_v2_9_0`. Audit each case and append the new call.

5. **Extend gate check** at `bin/intent_upgrade:93` with `! needs_v2_9_0_upgrade "$VERSION"`.

6. **Write BATS tests.** Use `HOME=$BATS_TMPDIR` (or similar) isolation. Set up various starting states. Run migration. Assert post-state.

7. **Dry-run canary projects.** In a sandbox (e.g. copy to `/tmp/anvil-dry` before running upgrade), simulate `intent upgrade --apply`. Verify v2.9.0 landing, pruned state, ext dir created.

8. **MODULES.md updates** — register `migrate_v2_8_2_to_v2_9_0`, `needs_v2_9_0_upgrade`, `generate_ext_readme`.

## Migration Function Specification

```bash
# migrate_v2_8_2_to_v2_9_0
#
# Migrates an Intent project from v2.8.2 to v2.9.0.
# Fail-forward: prunes installed copies of removed subagents (elixir, worker-bee).
# Seeds ~/.intent/ext/worker-bee/ from canon ext-seeds if absent.
#
# Idempotent via directory/file existence checks.
#
# Arguments: none (operates on CWD's .intent/config.json and user-global ~/.intent/, ~/.claude/)
migrate_v2_8_2_to_v2_9_0() {
  # 1. Version stamp
  local config_file=".intent/config.json"
  if [ -f "$config_file" ]; then
    local tmp=$(mktemp)
    jq '.intent_version = "2.9.0"' "$config_file" > "$tmp" && mv "$tmp" "$config_file"
    echo "  stamped: intent_version = 2.9.0"
  fi

  # 2. Bootstrap ~/.intent/ext/ (user-global; one-time effect)
  local ext_dir="$HOME/.intent/ext"
  if [ ! -d "$ext_dir" ]; then
    mkdir -p "$ext_dir"
    generate_ext_readme "$ext_dir/README.md"
    echo "  bootstrapped: $ext_dir"
  fi

  # 3. Seed worker-bee ext (if not already present)
  local wb_ext="$ext_dir/worker-bee"
  local wb_seed="$INTENT_HOME/lib/templates/ext-seeds/worker-bee"
  if [ ! -d "$wb_ext" ] && [ -d "$wb_seed" ]; then
    cp -r "$wb_seed" "$wb_ext"
    echo "  seeded: ~/.intent/ext/worker-bee (from canon ext-seeds)"
  elif [ -d "$wb_ext" ]; then
    echo "  skipped: ~/.intent/ext/worker-bee (already present, not overwriting)"
  fi

  # 4. Prune installed copies of deleted subagents
  for agent in elixir worker-bee; do
    local installed="$HOME/.claude/agents/${agent}.md"
    if [ -f "$installed" ]; then
      rm "$installed"
      echo "  pruned: ~/.claude/agents/${agent}.md"
    fi
    local manifest="$HOME/.intent/agents/installed-agents.json"
    if [ -f "$manifest" ]; then
      local tmp=$(mktemp)
      jq "(.installed) |= map(select(.name != \"${agent}\"))" "$manifest" > "$tmp" \
        && mv "$tmp" "$manifest"
    fi
  done

  echo ""
  echo "  v2.9.0 changes:"
  echo "    - NEW: ~/.intent/ext/ extension system for user skills/subagents/rules"
  echo "    - NEW: critic-elixir, critic-rust, critic-swift, critic-lua subagents"
  echo "    - REMOVED: elixir subagent (content split into rule library + critic-elixir)"
  echo "    - RELOCATED: worker-bee now at ~/.intent/ext/worker-bee/"
  echo "    - Installed copies of elixir and worker-bee have been pruned."
  echo "    - To reinstall worker-bee: intent claude subagents install worker-bee"
}

# needs_v2_9_0_upgrade VERSION_STRING
# Returns 0 if upgrade is needed (version < 2.9.0), 1 otherwise.
needs_v2_9_0_upgrade() {
  local v="$1"
  case "$v" in
    "2.9.0"|"2.9."*|"2.10."*|"3."*) return 1 ;;
    *) return 0 ;;
  esac
}

# generate_ext_readme TARGET_PATH
# Writes a README for ~/.intent/ext/ explaining the extension system.
generate_ext_readme() {
  local target="$1"
  cat > "$target" <<'EOF'
# Intent User Extensions

This directory holds user-local Intent extensions: skills, subagents, and rule
packs that you install without forking the Intent repo.

## Listing extensions

    intent ext list

## Creating an extension

    intent ext new my-ext --subagent    # scaffold a subagent
    intent ext new my-ext --skill       # scaffold a skill
    intent ext new my-ext --rule-pack   # scaffold a rule pack

## Reference example

`worker-bee` (seeded here on Intent v2.9.0 upgrade) is the reference extension.
See intent/docs/writing-extensions.md for the walkthrough.
EOF
}
```

## Acceptance Criteria

### Functions

- [ ] `migrate_v2_8_2_to_v2_9_0` exists in `bin/intent_helpers`
- [ ] `needs_v2_9_0_upgrade` predicate exists and returns correctly for versions < 2.9.0
- [ ] `generate_ext_readme` produces the expected README content

### Migration behaviour

- [ ] Running migrate on a clean v2.8.2 project updates `.intent/config.json:intent_version` to "2.9.0"
- [ ] `~/.intent/ext/` is created with README if absent
- [ ] `~/.intent/ext/` left alone if already present
- [ ] `~/.intent/ext/worker-bee/` seeded from `lib/templates/ext-seeds/worker-bee/` if absent
- [ ] `~/.intent/ext/worker-bee/` not overwritten if present (respects user customisation)
- [ ] `~/.claude/agents/elixir.md` deleted if present
- [ ] `~/.claude/agents/worker-bee.md` deleted if present
- [ ] `~/.intent/agents/installed-agents.json` has elixir and worker-bee entries removed
- [ ] Running migrate twice is a no-op (idempotent)

### Chain wiring

- [ ] Every case in `bin/intent_upgrade:136-295` chains through `migrate_v2_8_2_to_v2_9_0`
- [ ] New case `"2.8.2"` calls only `migrate_v2_8_2_to_v2_9_0`
- [ ] Gate check at `bin/intent_upgrade:93` includes `! needs_v2_9_0_upgrade`
- [ ] `intent upgrade` from v2.0.0 through v2.8.1 all land at v2.9.0 after one run

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP09.

- [ ] `tests/unit/ext_migration.bats` extended to cover:
  - [ ] Fresh-install case (v2.8.2 → v2.9.0 clean)
  - [ ] Already-migrated no-op (second run is idempotent)
  - [ ] Prune of installed elixir (`~/.claude/agents/elixir.md` removed if present; no error if absent)
  - [ ] Prune of installed worker-bee (`~/.claude/agents/worker-bee.md` removed if present; no error if absent)
  - [ ] Seed of worker-bee when `~/.intent/ext/worker-bee/` absent (copy from `lib/templates/ext-seeds/`)
  - [ ] Skip of seed when `~/.intent/ext/worker-bee/` already present (respects user customisation)
  - [ ] Registry cleanup: `~/.intent/agents/installed-agents.json` has no rows for `elixir` or `worker-bee` post-migration
  - [ ] Chain coverage: one case per prior version (2.0.0 through 2.8.1) exercising `migrate_v2_8_2_to_v2_9_0`
- [ ] Fixtures under `tests/fixtures/upgrade/{v2.0.0-project,v2.7.0-project,v2.8.2-project}/` simulating stale project states
- [ ] All BATS tests pass on macOS bash 3.x (no `declare -A`, no `readarray`, no `${VAR^}`)

### Tests to update

- [ ] Any existing upgrade-chain test (if present) gets the new v2.9.0 case appended without disrupting prior cases
- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

### Canary verification

- [ ] Dry-run in three canary projects (Anvil, Molt, Utilz, or similar) succeeds without error
- [ ] Post-dry-run: each canary has `intent_version: 2.9.0`, `~/.intent/ext/` populated, elixir/worker-bee pruned from `~/.claude/agents/`
- [ ] Canary verification passes before WP11 release is tagged

### No collateral damage

- [ ] Other installed subagents unchanged (diogenes, socrates, intent subagent still installed if they were before)
- [ ] Other skills unchanged
- [ ] `~/.intent/agents/installed-agents.json` manifest is valid JSON after migration

## Dependencies

- **WP02** (extension system): `~/.intent/ext/` layout and discovery required.
- **WP08** (worker-bee extraction): `lib/templates/ext-seeds/worker-bee/` must exist before migration can seed it.

## Implementation Notes

### Exact files to modify

- `/Users/matts/Devel/prj/Intent/bin/intent_helpers` — add three new functions (`migrate_v2_8_2_to_v2_9_0`, `needs_v2_9_0_upgrade`, `generate_ext_readme`)
- `/Users/matts/Devel/prj/Intent/bin/intent_upgrade` — extend gate check at line 93; add new case for "2.8.2"; chain every prior case through `migrate_v2_8_2_to_v2_9_0`
- `/Users/matts/Devel/prj/Intent/VERSION` — set to `2.9.0` (final commit in WP11)

### Chain extension pattern

Before (v2.8.2 precedent):

```bash
case "$VERSION" in
  "2.0.0")
    migrate_v2_0_0_to_v2_1_0 .
    migrate_v2_1_0_to_v2_2_0 .
    # ...
    migrate_v2_8_1_to_v2_8_2 .
    ;;
```

After:

```bash
case "$VERSION" in
  "2.0.0")
    migrate_v2_0_0_to_v2_1_0 .
    migrate_v2_1_0_to_v2_2_0 .
    # ...
    migrate_v2_8_1_to_v2_8_2 .
    migrate_v2_8_2_to_v2_9_0 .
    ;;
```

Every existing case gets one new line.

### Gate check

`bin/intent_upgrade:93` currently chains `needs_v2_X_Y_upgrade` calls. Add:

```bash
if ! needs_v2_migration "$VERSION" && ... && ! needs_v2_8_2_upgrade "$VERSION" && ! needs_v2_9_0_upgrade "$VERSION"; then
  echo "ok: up to date at v$VERSION"
  ...
```

### Canary projects

Per MEMORY.md, the fleet is:

Anvil, Conflab, Intent, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz, Courses/Agentic Coding, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex (16 total; A3/\* skipped).

Canary batch (5 projects):

- Anvil (established, full feature use)
- Arca/arca_cli (smaller, different archetype)
- Arca/arca_config (small, config-focused)
- Arca/arca_notionex (integration-heavy)
- Conflab (real working project)

Dry-run procedure:

```bash
# Copy project to sandbox
cp -r ~/Devel/prj/Anvil /tmp/dry-anvil
cd /tmp/dry-anvil
HOME=/tmp/dry-home intent upgrade --apply
# Verify
cat .intent/config.json | jq .intent_version  # expect "2.9.0"
ls /tmp/dry-home/.intent/ext/                 # expect worker-bee/
ls /tmp/dry-home/.claude/agents/              # expect no elixir.md, no worker-bee.md
```

### Order of operations in migration

Deliberately: version stamp first, so that if the migration fails partway, subsequent runs see a post-2.8.2 version and don't re-run prior migrations. Then ext bootstrap, then seed, then prune. Prune is last because it's the most destructive.

### jq manifest surgery safety

`jq '(.installed) |= map(select(.name != "elixir"))'` correctly handles:

- Manifest with elixir entry: removes it.
- Manifest without elixir entry: returns manifest unchanged.
- Empty `installed` array: returns unchanged.
- Missing `installed` key: jq errors — guard with `[ -f "$manifest" ]` check.

## Risks and Edge Cases

### R8: Upgrade chain fragility

v2.8.2 had a slipstream fix for missed chain cases. Mitigation: audit every case in `bin/intent_upgrade`; BATS test covers all starting versions.

### Migration runs during Claude session

User might run `intent upgrade --apply` with Claude attached. Migration deletes `~/.claude/agents/elixir.md` mid-session. Claude continues to work (agent.md is loaded at session start, not per-invocation). New sessions will see the new state. Document behaviour.

### Partial state: ext dir present but no README

If a user manually created `~/.intent/ext/` before upgrade, our bootstrap skip preserves their directory — but no README. Acceptable; first `intent ext list` will work anyway.

### Partial state: worker-bee ext dir present but missing files

User manually created `~/.intent/ext/worker-bee/` but without the seed content. Our guard skips seeding. User gets a broken ext. Mitigation: `intent ext validate worker-bee` (WP02) will flag it; user can delete and re-run migration (idempotent).

### Concurrent invocations

Two terminals running `intent upgrade --apply` simultaneously. Race condition on `~/.intent/`. Accept as out-of-scope; document in release notes.

### jq not installed

Intent already requires jq (see `bin/intent_doctor`). No new dependency.

### Test environment isolation

BATS tests must not touch the real `$HOME`. Use `HOME=$BATS_TEST_TMPDIR` or similar. Existing test helpers at `tests/lib/test_helper.bash` likely have patterns for this.

## Testing Approach

### Full BATS

- `tests/run_tests.sh` must be green before WP09 closes.
- New file `tests/unit/ext_migration.bats` covers every case in the Acceptance Criteria.

### Example BATS test

```bash
@test "migrate_v2_8_2_to_v2_9_0 seeds worker-bee ext when absent" {
  setup_fake_home
  stamp_intent_version "2.8.2"
  run migrate_v2_8_2_to_v2_9_0 .
  assert_success
  assert [ -d "$HOME/.intent/ext/worker-bee" ]
  assert [ -f "$HOME/.intent/ext/worker-bee/extension.json" ]
}

@test "migrate_v2_8_2_to_v2_9_0 does not overwrite existing worker-bee ext" {
  setup_fake_home
  mkdir -p "$HOME/.intent/ext/worker-bee"
  echo "custom" > "$HOME/.intent/ext/worker-bee/marker.txt"
  stamp_intent_version "2.8.2"
  run migrate_v2_8_2_to_v2_9_0 .
  assert_success
  assert [ "$(cat $HOME/.intent/ext/worker-bee/marker.txt)" = "custom" ]
}

@test "migrate_v2_8_2_to_v2_9_0 prunes installed elixir" {
  setup_fake_home
  mkdir -p "$HOME/.claude/agents"
  echo "stub" > "$HOME/.claude/agents/elixir.md"
  run migrate_v2_8_2_to_v2_9_0 .
  assert_success
  assert [ ! -f "$HOME/.claude/agents/elixir.md" ]
}
```

### Canary dry-run

Automate the canary dry-run in a script `scripts/ci/canary-dry-run.sh`:

```bash
for proj in Anvil Arca/arca_cli Arca/arca_config Arca/arca_notionex Conflab; do
  cp -r ~/Devel/prj/$proj /tmp/canary-$(basename $proj)
  cd /tmp/canary-$(basename $proj)
  HOME=/tmp/canary-$(basename $proj)-home intent upgrade --apply || exit 1
  cat .intent/config.json | jq -r .intent_version | grep -q "^2.9.0$" || exit 1
  cd -
done
```

## Size and Estimate

- **Size**: M (Medium, 2-3 sessions).
- **Session 1**: Migration function + predicate + README generator + unit BATS.
- **Session 2**: Chain wiring across all prior cases; gate check extension; chain-case BATS.
- **Session 3**: Canary dry-run script, execution against 3-5 projects, triage any issues.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] BATS suite green
- [ ] Canary dry-run clean on at least 3 projects
- [ ] Migration output is chatty and informative (user sees what was pruned)
- [ ] MODULES.md updated
- [ ] Ready for WP11 release (don't bump VERSION yet; WP11 does the final stamp)
