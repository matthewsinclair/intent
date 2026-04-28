---
verblock: "28 Apr 2026:v0.67: matts - ST0037 implementation complete; v2.11.0 ready to ship"
intent_version: 2.11.0
---

# Work In Progress

## Current State

**ST0037 implementation complete; v2.11.0 ready to ship.** Tests 863/863 local. `intent doctor` clean. `intent upgrade` self-applied: this project's `intent/.config/config.json` is now stamped at v2.11.0 with `languages: ["shell"]` (back-filled from pre-commit hook presence).

## ST0037 (shipping in v2.11.0)

Languages-in-use is now an explicit per-project configuration field, replacing four sites of filesystem-marker probing. The probe-based detection was a regression against design intent.

### What landed

- **Schema**: `languages: []` field in `intent/.config/config.json`. Array of canonical language names. Order is the explicit declaration; first entry is the primary where a primary is needed.
- **Migration** `migrate_v2_10_x_to_v2_11_0` in `bin/intent_helpers`. Idempotent. Back-fills from `intent/llm/RULES-<lang>.md` presence (alphabetical), falls back to `["shell"]` if a pre-commit hook is installed and back-fill is otherwise empty.
- **Helpers**: `get_project_languages`, `add_project_language`, `remove_project_language` in `bin/intent_helpers`. Atomic config writes via tempfile + `mv`.
- **CLI**: `intent lang init <lang>` now writes the field. New `intent lang remove <lang>` reverses init: deletes RULES-<lang>.md + ARCHITECTURE-<lang>.md, removes the marker-block entry, removes the language from config.
- **Probes replaced**: `in-session/SKILL.md`, `in-review/SKILL.md`, `in-tca-audit/SKILL.md`, `lib/templates/hooks/pre-commit.sh`. All four now read `(.languages // []) | .[]` from config.
- **Phantom skill refs stripped**: `/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials` (promised in WP06/WP12, never authored). Replaced with prose pointing at the rule library + critic-`<lang>` subagent path that already covers those four languages.
- **Latent bug fixed**: `create_v2_directory_structure()` was creating an empty `.intent/` unconditionally on every `intent upgrade`, which then conflicted with `intent_relocate_dotintent`'s safety check on already-relocated projects. Skips the creation when `intent/.config/` is in place.
- **Tests**: new `tests/unit/migrate_v2_10_x_to_v2_11_0.bats` (10 scenarios). Reworked `in_session_skill.bats` (probe assertions out, config-read + phantom-ref guards in), `critic_dispatch.bats` (full rewrite: config-driven sandboxes), `pre_commit_hook.bats` (config-driven setup + new empty-langs and lang-mismatch tests), `intent_lang.bats` (new `init` field-write + `remove` coverage), `init_commands.bats` (assert `languages: []` in fresh config).
- **Docs**: `intent/docs/working-with-llms.md` rewritten for the config-driven flow. `MODULES.md` updated for the new helpers and `intent lang remove` verb. Blog draft `####-claude-context-with-intent.md` paragraph rewritten.
- **VERSION** bumped to 2.11.0. **CHANGELOG** entry written.

## Next Up

Ship v2.11.0. The `scripts/release` orchestrator (added in v2.10.1) handles tag + multi-remote push + GitHub release. Workflow:

1. Stage and commit (single coherent commit; pre-commit gate will fire on shell + skill changes).
2. `scripts/release --dry-run --minor` to verify the bump path.
3. `scripts/release --minor` to cut.

Other v2.11.x backlog (deferred, not blocking):

- Audit `intent claude upgrade` Phase-2 CLAUDE.md substitution: a regex in there mangles "v2.0.0" in the migration-history paragraph because it's substituting greedily on any `v2.X.Y` not just the version banner. Hand-corrected this session; needs a proper fix before next release.
- Homebrew tap.
- `scripts/release` v2 polish (`--rollback`, log-to-file mirror, prettier output).

## Recent

- **2026-04-28 (post-compact)**: ST0037 implemented end-to-end. Schema migration, helpers, CLI verbs, probe replacements, BATS rework, docs and blog updates, CHANGELOG entry, VERSION bump. Pre-existing `.intent/` mkdir bug found and fixed during dogfood. 863/863 tests green; doctor clean; project self-upgraded successfully.
- **2026-04-28 evening**: language-detection regression diagnosed; ST0037 plan agreed (Option B, explicit config field).
- **2026-04-28 afternoon**: two blog drafts in `docs/blog/_drafts/`. Detrope-clean except for the language-detection paragraph that ST0037 now unblocks.
- **2026-04-28 mid-session**: skill-renderer trap regression guard test (`tests/unit/skill_renderer_trap.bats`); idempotence-test fix for AGENTS.md version probe.
- **2026-04-28 morning**: v2.10.1 shipped.

## Parked

_(None.)_
