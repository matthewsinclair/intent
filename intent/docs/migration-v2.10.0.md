# Intent v2.9.0 → v2.10.0 Migration Guide

This guide covers the upgrade from Intent v2.9.0 to v2.10.0 — what changes for projects on the ground, and what to do if anything goes wrong.

v2.10.0 bundles two steel threads in one release:

- **ST0035** — the canonical LLM config (root-level `AGENTS.md` + `CLAUDE.md` + `usage-rules.md`, session hooks, pre-commit critic gate). No path changes for projects already on v2.9.0; the canon-apply step ships new templates and may refresh hand-edited canon files.
- **ST0036** — the per-project metadata directory move from top-level `.intent/` to nested `intent/.config/`. This is the breaking change in the version. Migration is automatic on `intent upgrade` and atomic; no backwards-compat probe ships.

Most users will not notice the difference after `intent upgrade` runs. The exceptions — script authors, CI maintainers, anyone with hand-rolled tooling that probes `.intent/` — are addressed below.

## Summary

| Step                                                 | Action                                                                                                                                                                          |
| ---------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 1. Bring repo to a clean state                       | Commit or stash any in-flight edits.                                                                                                                                            |
| 2. `cd <project>`                                    | Run from the project root (or anywhere inside it; v2.8.2+ is cwd-resilient).                                                                                                    |
| 3. `intent upgrade`                                  | Runs `migrate_v2_9_0_to_v2_10_0`: relocates `.intent/` → `intent/.config/` atomically, stamps the new path to `2.10.0`, then invokes `intent claude upgrade --apply` for canon. |
| 4. `intent doctor`                                   | Confirms the new layout. Should report clean.                                                                                                                                   |
| 5. Update any external scripts / CI / editor plugins | See **What to update on your side** below.                                                                                                                                      |
| 6. Commit the migration                              | The relocation is a real diff in your working tree; capture it.                                                                                                                 |

## What moved

The per-project metadata directory and its entire contents move from top-level `.intent/` to nested `intent/.config/`:

| Before (v2.9.0)                  | After (v2.10.0)                         | Notes                                                                                                               |
| -------------------------------- | --------------------------------------- | ------------------------------------------------------------------------------------------------------------------- |
| `.intent/config.json`            | `intent/.config/config.json`            | Project metadata; version stamp.                                                                                    |
| `.intent/cache/`                 | `intent/.config/cache/`                 | Always-gitignored.                                                                                                  |
| `.intent/backup/`                | `intent/.config/backup/`                | Always-gitignored.                                                                                                  |
| `.intent/<custom>`               | `intent/.config/<custom>`               | Whole-tree move; nothing left behind.                                                                               |
| `.intent/.migration-in-progress` | `intent/.config/.migration-in-progress` | Sentinel file; only present mid-migration or after a failure. See [Recovery](#recovery-from-interrupted-migration). |

What did **not** move:

- `~/.intent/ext/` — the user-level extension root. Unchanged. Always at the user level; no per-project dependency.
- `.intent_critic.yml` — file at the project root, not under `.intent/`. Path unchanged.
- `intent/.treeindex/` — already lives under `intent/`. Path unchanged.
- All other `intent/` subdirectories (`intent/st/`, `intent/docs/`, `intent/llm/`, `intent/eng/`). Unchanged.

The new layout removes the "two top-level dirs" smell (`.intent/` + `intent/`) by nesting configuration under the existing `intent/` umbrella. There is now exactly one Intent-owned directory at the project root.

## How to upgrade

For a single project:

```bash
cd <project>
intent upgrade
intent doctor
git add intent/.config/
git status      # confirm .intent/ is gone, intent/.config/ is staged
git commit -m "chore: upgrade to Intent v2.10.0"
```

The `intent upgrade` step runs `migrate_v2_9_0_to_v2_10_0`, which:

1. Writes a sentinel at `.intent/.migration-in-progress` (announces intent and gates recovery).
2. Refuses to proceed if the destination `intent/.config/` already exists with conflicting content, or if either path is a symlink.
3. Atomically `mv .intent intent/.config` (with EXDEV cross-filesystem fallback: `cp -a` + checksum + `rm`).
4. Removes the sentinel from its new location post-move.
5. Stamps `intent/.config/config.json`'s `intent_version` to `2.10.0`.
6. Invokes `intent claude upgrade --apply` to plant any newly-shipped canon artefacts (refreshed `usage-rules.md`, hook stanzas in `.claude/settings.json`, pre-commit hook chain, etc.).

The migration is idempotent on layout state — running `intent upgrade` again on an already-relocated project is a no-op for ST0036 and only re-runs the canon apply step (also idempotent).

For fleet rollouts, the per-project recipe is the same. ST0035/WP15-WP17 cover the dogfood + canary + fleet sweep across the 17 active projects.

## What to update on your side

Anywhere your tooling references the per-project `.intent/` path needs flipping. The full surface:

- **Shell aliases / scripts** that read or write `.intent/...`:

  ```bash
  # before
  alias intent-config='cat .intent/config.json'
  jq .project_name .intent/config.json

  # after
  alias intent-config='cat intent/.config/config.json'
  jq .project_name intent/.config/config.json
  ```

- **CI pipelines** that probe `.intent/config.json` for the project version, name, or any metadata field:

  ```yaml
  # before
  - run: jq -r '.intent_version' .intent/config.json

  # after
  - run: jq -r '.intent_version' intent/.config/config.json
  ```

- **Editor plugins** (VS Code, Vim, etc.) with project detection rules looking for `.intent/`. Update to look for `intent/.config/` instead. Most editor plugins use `intent doctor` or `intent --version` rather than direct path probes; if yours uses `intent`, no changes needed.

- **`.gitignore`**: `intent init` and the migration both write the canonical entries. If you have hand-edited `.gitignore` rules referencing `.intent/cache/` or `.intent/backup/`, flip them to `intent/.config/cache/` and `intent/.config/backup/`.

- **Custom scripts under `bin/` or `scripts/`** in your own project (not Intent's). Same flip.

- **Documentation** in your project that describes where Intent metadata lives. Flip the path.

What does **not** need updating:

- `~/.intent/ext/` references (extension root, user-level).
- `.intent_critic.yml` references (file at project root, not under `.intent/`).
- Anything that goes through the `intent` CLI rather than reading paths directly.

If your tooling uses the `intent` CLI rather than path-probes, you are unaffected: `intent doctor`, `intent st list`, `intent wp list`, `intent agents sync`, `intent claude upgrade`, etc. all resolve project root through `find_project_root`, which now looks for `intent/.config/config.json`.

## Recovery from interrupted migration

If `intent upgrade` failed mid-relocation, you may see a sentinel file. The migration writes it before any irreversible work and removes it on success. Its presence indicates the migration was interrupted.

The sentinel may be at one of two paths depending on how far the migration progressed:

- `.intent/.migration-in-progress` — relocation aborted before `mv`. Source dir is intact.
- `intent/.config/.migration-in-progress` — `mv` succeeded but a later step (stamp or canon-apply) failed. Destination dir is intact; source is gone.

`intent doctor` checks for both and reports the location explicitly:

```
checking: interrupted migration error: sentinel detected at intent/.config/.migration-in-progress
  see intent/docs/migration-v2.10.0.md#recovery-from-interrupted-migration
```

### Diagnose

Run these in order to determine the exact state:

```bash
ls -la .intent intent/.config 2>/dev/null
test -f .intent/.migration-in-progress && echo "sentinel: pre-mv"
test -f intent/.config/.migration-in-progress && echo "sentinel: post-mv"
test -f intent/.config/config.json && jq -r '.intent_version' intent/.config/config.json
```

### Recover by case

**Case 1: Sentinel at `.intent/.migration-in-progress`, source dir intact, destination absent or empty.**

The migration aborted before `mv`. Safe to remove the sentinel and retry:

```bash
rm .intent/.migration-in-progress
intent upgrade
```

**Case 2: Sentinel at `intent/.config/.migration-in-progress`, destination dir present, source dir gone.**

The `mv` succeeded but the version stamp or canon-apply step failed. The data is in the right place; only the post-move steps need finishing. Inspect first:

```bash
jq -r '.intent_version' intent/.config/config.json
```

If the version stamp is correct (`2.10.0`), the failure happened during canon-apply. Remove the sentinel and re-run; canon-apply is idempotent:

```bash
rm intent/.config/.migration-in-progress
intent upgrade
```

If the version stamp is still `2.9.0` (or earlier), the stamp step failed. Update it manually, remove the sentinel, then re-run:

```bash
jq '.intent_version = "2.10.0"' intent/.config/config.json > /tmp/c.json && mv /tmp/c.json intent/.config/config.json
rm intent/.config/.migration-in-progress
intent upgrade
```

**Case 3: Both `.intent/` and `intent/.config/` present, both contain files.**

The migration aborted during `mv` mid-stream. This should not happen with the atomic `mv` path; it can only happen if EXDEV fallback was active and the `cp -a` succeeded but the post-`cp` `rm` failed (e.g., permission change mid-copy, or out-of-disk).

Both directories may have valid content. Compare them carefully:

```bash
diff -r .intent intent/.config
```

If identical, the source `.intent/` is the leftover from a failed `rm`. Remove it manually and remove the sentinel:

```bash
rm -rf .intent
rm intent/.config/.migration-in-progress
intent upgrade
```

If they differ (rare; only possible if you wrote to `.intent/` after the failed migration), pick the canonical version, copy the missing pieces across, then proceed as above. If unsure, restore from your last commit (the per-project `.intent/` was tracked in git in v2.9.0 and earlier) and re-run.

**Case 4: Destination `intent/.config/` is incomplete (missing files that should be there).**

This indicates a partial `cp -a` during EXDEV fallback. Restore from your last commit:

```bash
git checkout HEAD -- .intent
rm -rf intent/.config
intent upgrade
```

Or, if you have a manual backup elsewhere, restore from that.

### When in doubt

- The previous `.intent/` content is in your git history (Intent never untracked it). `git log --all --oneline -- .intent/` and `git show <hash>:.intent/config.json` recover any specific past state.
- `intent doctor` is the authoritative end-state check. If it reports clean post-recovery, the migration is done.
- If the recovery instructions above do not match what you see, open an issue at https://github.com/matthewsinclair/intent/issues with the output of the four diagnostic commands above.

## FAQ

**Why did Intent move `.intent/` to `intent/.config/`?**

Two top-level Intent-owned directories (`intent/` and `.intent/`) was a smell. The fix is structural: nest configuration under the existing `intent/` umbrella so the project root surfaces exactly one Intent directory. Cosmetic on the surface, but it removes a class of "where does this go?" question for both authors and downstream tooling.

**Does this affect `~/.intent/ext/`?**

No. The user-level extension root is unrelated to per-project metadata and is not touched by the migration. `intent ext list`, `intent ext show`, and any extension you have installed continue to work without changes.

**What if I had custom files under `.intent/`?**

They move with everything else. The migration relocates the entire directory tree, not a curated subset. Your custom files end up at `intent/.config/<your-file>` with permissions and content preserved (atomic `mv`; or `cp -a` for cross-filesystem).

**Will my `.git/hooks/pre-commit` keep working?**

Yes. The pre-commit hook (shipped via ST0035) calls `intent critic` and reads `.intent_critic.yml` at the project root. Neither path involves `.intent/`. The hook is unaffected by ST0036.

**Is rollback to v2.9.0 possible?**

Not via `intent` itself. Intent ships fail-forward — there is no `intent downgrade`. To roll back, restore the previous `.intent/` from git history and pin to the v2.9.0 release of Intent:

```bash
git checkout v2.9.0-tag-or-commit -- .intent
rm -rf intent/.config
git checkout v2.9.0 -- .  # or wherever your v2.9.0 Intent is
```

In practice, the migration is small and low-risk; rollback should rarely be needed. If it is, please open an issue describing the failure mode.

**What about the LLM canon files (AGENTS.md, CLAUDE.md, usage-rules.md)?**

ST0035 ships those independently of the directory move. The `intent claude upgrade --apply` step (called automatically by the v2.10.0 migration) installs or refreshes them per the templates at `lib/templates/`. Hand-edited content is preserved between `<!-- user:start --> / <!-- user:end -->` markers. See `intent/docs/working-with-llms.md` for the canon design.

## See also

- `CHANGELOG.md` — v2.10.0 entry, including the **Breaking** subsection.
- `intent/docs/working-with-llms.md` — canon LLM-config tech note (ST0035 deliverable).
- `intent/docs/critics.md`, `intent/docs/rules.md`, `intent/docs/writing-extensions.md` — other v2.9.0+ tech notes; unchanged by v2.10.0.

---

_Document stamp: Intent v2.10.0, ST0036/WP-07, 2026-04-26._
