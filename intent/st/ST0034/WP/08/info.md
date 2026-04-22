---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-08
title: "Worker-bee extraction"
scope: Small
status: Not Started
---

# WP-08: Worker-bee extraction

## Objective

Remove `worker-bee` from Intent canon entirely. Relocate its content to `lib/templates/ext-seeds/worker-bee/` as the seed for the reference extension. Author the `extension.json` manifest for the seed. Position worker-bee as the worked-example reference for third-party extension authors.

## Context

`worker-bee` (Worker-Bee Driven Design specialist) is domain-specific scaffolding, not core Intent machinery. It should not have been in the main repo's canon subagents. This WP extracts it, and the migration in WP09 seeds it into `~/.intent/ext/worker-bee/` on upgrade.

This is the proof that the extension system works end-to-end: take a real, non-trivial subagent (agent.md + metadata.json + 14 resource files) and move it out of canon without breaking its function. If worker-bee can be moved, any user-authored extension can.

Fail-forward design: migration aggressively prunes installed copies. Users who want worker-bee after the upgrade reinstall from the seeded ext.

## Deliverables

### Deletion from canon

- `intent/plugins/claude/subagents/worker-bee/` — entire directory deleted
- `intent/plugins/claude/subagents/.manifest/global-agents.json` — worker-bee entry removed

### Ext-seed at new location

```
lib/templates/ext-seeds/worker-bee/
  extension.json                                # NEW — manifest for ext format
  README.md                                     # NEW — explains the seed
  LICENSE                                       # optional
  subagents/
    worker-bee/
      agent.md                                  # moved from canon
      metadata.json                             # moved from canon
      resources/                                # moved from canon (14 files)
        config/
          wdd_patterns.yaml
        lib/
          ...
        templates/
          ...
        validation/
          ...
        README.md
        USER_GUIDE.md
```

### Manifest (`lib/templates/ext-seeds/worker-bee/extension.json`)

```json
{
  "schema": "intent-extension/v1",
  "name": "worker-bee",
  "version": "1.0.0",
  "description": "Worker-Bee Driven Design specialist for Elixir applications",
  "author": "matts",
  "license": "MIT",
  "homepage": "https://github.com/thebreakincoder/worker-bee",
  "intent_compat": { "min": "2.9.0", "max": "3.x" },
  "contributes": {
    "subagents": [{ "name": "worker-bee", "path": "subagents/worker-bee" }],
    "skills": [],
    "rules": []
  },
  "checksums": {}
}
```

`checksums` populated after WP02's `intent ext validate` can generate them.

### Seed README

- `lib/templates/ext-seeds/worker-bee/README.md`:
  - Explains that this is a seed (not a runtime extension).
  - Documents how migration (WP09) copies it to `~/.intent/ext/worker-bee/`.
  - Points at worker-bee's original documentation inside `subagents/worker-bee/resources/`.
  - Notes that further worker-bee development happens in `~/.intent/ext/worker-bee/` (user-local), not in Intent canon.

### Docs coordination

- `intent/docs/writing-extensions.md` (from WP02 skeleton) updated with worker-bee as the worked example (expanded in WP10)

### MODULES.md

- Remove worker-bee subagent row from MODULES.md.
- Add `lib/templates/ext-seeds/` row.
- Link to `intent/docs/writing-extensions.md`.

## Approach

1. **Audit existing worker-bee dependencies.** Grep for `worker-bee` across the repo; confirm which references are:
   - Canon path (will become stale) — update or remove
   - Docs/commentary (will need updating in WP10)
   - External (tests, configs) — update

2. **Relocate via `git mv`** to preserve history:

   ```bash
   mkdir -p lib/templates/ext-seeds/worker-bee/subagents
   git mv intent/plugins/claude/subagents/worker-bee lib/templates/ext-seeds/worker-bee/subagents/worker-bee
   ```

3. **Author `extension.json`.** Follow the schema from WP02. Use worker-bee's existing `metadata.json` to populate description, license hints.

4. **Author seed README.** Short — 30-50 lines. Explains the seed concept.

5. **Update canon `global-agents.json`.** Remove worker-bee entry (preserve sort order).

6. **Validate against ext schema.** Once WP02's `intent ext validate` exists, run:

   ```bash
   INTENT_EXT_DIR=lib/templates/ext-seeds intent ext validate worker-bee
   ```

   Expect: pass (no path-traversal, well-formed manifest, contributed files exist).

7. **Verify resources are self-contained.** Audit `resources/` for any `$INTENT_HOME` path references, absolute paths that break post-move, or cross-directory symlinks. worker-bee was flagged self-contained in Phase 1 research, but re-verify.

8. **Register in MODULES.md.** Add `lib/templates/ext-seeds/` entry. Remove canon worker-bee row.

9. **Hand off to WP09.** Migration function will reference `lib/templates/ext-seeds/worker-bee/` as the seed source.

## Acceptance Criteria

### Deletion

- [ ] `intent/plugins/claude/subagents/worker-bee/` directory does not exist
- [ ] `intent/plugins/claude/subagents/.manifest/global-agents.json` does not list worker-bee
- [ ] `grep -rn "subagents/worker-bee" intent/ lib/` returns only references in `lib/templates/ext-seeds/` and docs (no canon paths)

### Ext seed

- [ ] `lib/templates/ext-seeds/worker-bee/extension.json` exists and conforms to schema
- [ ] `lib/templates/ext-seeds/worker-bee/subagents/worker-bee/agent.md` exists
- [ ] `lib/templates/ext-seeds/worker-bee/subagents/worker-bee/metadata.json` exists
- [ ] All 14 resource files relocated intact (count matches original)
- [ ] `lib/templates/ext-seeds/worker-bee/README.md` exists and documents the seed concept
- [ ] No `$INTENT_HOME` path references in seed files (self-contained)
- [ ] `INTENT_EXT_DIR=lib/templates/ext-seeds intent ext validate worker-bee` passes (once WP02 validator exists)

### Integration

- [ ] `intent claude subagents list` does not show worker-bee (canon path gone, seed not on search path)
- [ ] After WP09 migration is simulated: `~/.intent/ext/worker-bee/` is populated from the seed
- [ ] After simulated migration: `intent claude subagents list` shows worker-bee with `[ext:worker-bee]` tag

### Docs

- [ ] `intent/docs/writing-extensions.md` references worker-bee (full walkthrough expanded in WP10)
- [ ] MODULES.md updated with new paths, old rows removed

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP08.

- [ ] `tests/unit/ext_seed_validity.bats` — seed directory passes `intent ext validate worker-bee` as-shipped
- [ ] `tests/unit/ext_migration.bats::seed_first_run` — first-run migration copies `lib/templates/ext-seeds/worker-bee/` into `~/.intent/ext/worker-bee/`
- [ ] `tests/unit/ext_migration.bats::seed_idempotent` — second run is a no-op; pre-existing `~/.intent/ext/worker-bee/` preserved (no overwrite)
- [ ] `tests/unit/ext_migration.bats::prune_installed_worker_bee` — if `~/.claude/agents/worker-bee.md` exists pre-migration, it is deleted; if not, no error
- [ ] `tests/unit/ext_migration.bats::prune_installed_registry` — `~/.intent/agents/installed-agents.json` has no row for `worker-bee` after migration

### Tests to update

- [ ] `tests/unit/agent_commands.bats` — any test that lists canon subagents must no longer expect `worker-bee` in output
- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP02** (extension system): the `extension.json` schema and `intent ext validate` must exist before WP08 can author and validate the manifest.

## Implementation Notes

### Exact paths

- Canon to delete: `/Users/matts/Devel/prj/Intent/intent/plugins/claude/subagents/worker-bee/`
- Seed destination: `/Users/matts/Devel/prj/Intent/lib/templates/ext-seeds/worker-bee/`
- Manifest target: `/Users/matts/Devel/prj/Intent/lib/templates/ext-seeds/worker-bee/extension.json`
- Canon manifest to update: `/Users/matts/Devel/prj/Intent/intent/plugins/claude/subagents/.manifest/global-agents.json`

### `git mv` procedure

```bash
mkdir -p lib/templates/ext-seeds/worker-bee/subagents
git mv intent/plugins/claude/subagents/worker-bee \
       lib/templates/ext-seeds/worker-bee/subagents/worker-bee
# Now intent/plugins/claude/subagents/worker-bee/ is gone; new location has the dir.
# Then add extension.json and README.md at the top of the seed:
```

Then author `extension.json` and `README.md` at `lib/templates/ext-seeds/worker-bee/`.

### Resources audit

Before WP08 concludes, scan `resources/` for:

```bash
grep -rn "\$INTENT_HOME" lib/templates/ext-seeds/worker-bee/
grep -rn "/Users/" lib/templates/ext-seeds/worker-bee/       # machine-specific paths
grep -rn "intent/plugins" lib/templates/ext-seeds/worker-bee/ # canon references
```

Any hits must be fixed or explicitly documented as assumptions about ext context.

### Checksum population

`extension.json`'s `checksums` field is optional. Populate if WP02 validator can generate them:

```bash
intent ext validate worker-bee --update-checksums  # if this flag is implemented; otherwise leave empty
```

Empty checksums is valid for v2.9.0.

### MODULES.md change

Before WP08:

```
| Worker-Bee subagent | `intent/plugins/claude/subagents/worker-bee/` | WDD specialist |
```

After WP08:

```
| Ext seeds (source for migration)  | `lib/templates/ext-seeds/`                    | Reference extensions; worker-bee is the canonical example  |
```

Add a note in MODULES.md: "worker-bee moved from canon to ext seeds in v2.9.0. See `intent/docs/writing-extensions.md` for the extraction story."

### Migration coordination

WP09 references this seed path. Change only in coordination:

- Seed path: `lib/templates/ext-seeds/worker-bee/`
- Target path on upgrade: `~/.intent/ext/worker-bee/`

## Risks and Edge Cases

### Resources reference paths that break post-move

Audited but possible. Mitigation: full `grep` pass after relocation.

### Duplicate history across move

`git mv` preserves history. If someone does `git log lib/templates/ext-seeds/worker-bee/subagents/worker-bee/agent.md`, they see pre-move history. Good.

### Reviewers miss that seed is not runtime

Seed is under `lib/templates/`. Without the README, a future contributor might try to run worker-bee from the seed path directly. README makes the runtime location clear.

### Path-traversal check in validate

WP02's validator might flag relative paths in the seed as suspicious. The seed's paths are all relative to the seed root (`subagents/worker-bee`) which is correct. Ensure the validator handles seeds the same way it handles installed extensions.

### Aggressive prune coordination (WP09)

WP09's migration deletes `~/.claude/agents/worker-bee.md` on upgrade. That's WP09's concern; WP08 only provides the seed. Document the handoff.

### Historical value

worker-bee's agent.md and resources represent non-trivial authoring work. Ensure nothing is lost in the move; diff-check contents pre- and post-move.

## Testing Approach

### Filesystem diff

Pre-move list of files under `intent/plugins/claude/subagents/worker-bee/` matches post-move list under `lib/templates/ext-seeds/worker-bee/subagents/worker-bee/`. Same count, same content (md5 if paranoid).

### Schema validation

`intent ext validate <seed>` passes.

### Simulated migration

Run WP09's migration function against a throwaway home dir (`HOME=/tmp/fake-home intent upgrade --apply`) and verify `~/.intent/ext/worker-bee/` is populated identically to the seed.

### Discoverability

After simulated migration, `intent claude subagents list` (with `$HOME=/tmp/fake-home`) shows worker-bee with `[ext:worker-bee]` tag.

### Regression check

Other subagents in canon (`intent`, `diogenes`, `socrates`) are unaffected. `intent claude subagents list` shows them without changes.

## Size and Estimate

- **Size**: S (Small, 1 session).
- Mostly mechanical: git mv + author manifest + update canon manifest + audit resources.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] worker-bee absent from canon
- [ ] Seed populated and validated
- [ ] Resources confirmed self-contained
- [ ] MODULES.md updated
- [ ] WP09 migration function ready to reference the seed
- [ ] `git log` shows preserved history for moved files
- [ ] Release notes draft mentions "worker-bee relocated from canon to reference extension"
