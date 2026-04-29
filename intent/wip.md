---
verblock: "29 Apr 2026:v0.68: matts - ST0039 implementation complete; v2.11.3 ready to ship"
intent_version: 2.11.3
---

# Work In Progress

## Current State

**ST0039 implementation complete; v2.11.3 ready to ship.** Defect fix for the Conflab field report (pre-commit critic gate emitting false positives derived from Greppable proxies the runner could not honour). `intent doctor` clean. BATS suite green. `scripts/release --dry-run --patch` previews 2.11.2 → 2.11.3 cleanly.

## ST0039 (shipping in v2.11.3)

The pre-commit critic runner now refuses Greppable proxies it cannot execute faithfully (pipes, `xargs`, `grep -L`, filter pipes, awk, multi-line composites) with a once-per-rule stderr diagnostic instead of silently degrading. 8 Elixir rules whose detection cannot be expressed as a single-file regex have had their proxy blocks stripped — they apply via `/in-review` only. `IN-EX-CODE-004` keeps its `error -> error` forwarder detector; the `case.*do$ | wc -l` counter line is gone.

### What landed

- **Strict-proxy contract** in `intent/plugins/claude/lib/critic_runner.sh`. New `critic_proxy_is_simple` predicate and `critic_patterns_from_grep_block` walker. Multi-pattern union with `(line, content)` dedupe. `critic_pattern_from_grep_command` deleted (no back-compat shim).
- **Stderr diagnostic** `note: skipping <rule_id> (proxy not headless-runnable)` once per rule. JSON output stays valid (stderr is separate by design).
- **Greppable proxy stripped** from 8 RULE.md files: IN-EX-TEST-003, IN-EX-CODE-003, IN-EX-LV-001, IN-EX-LV-003, IN-EX-PHX-001, IN-EX-ASH-001, IN-EX-ASH-002, IN-EX-TEST-004. Detection prose retained; one-line note pointing at the LLM `critic-elixir` subagent path.
- **Surgical edit on IN-EX-CODE-004** (with-for-railway): counter line removed, `error -> error` forwarder detector kept.
- **Rule simplifications**: IN-EX-TEST-005, IN-EX-TEST-006, IN-EX-TEST-007 had their `--include='*_test.exs'` flag dropped from the proxy (the `applies_to` field already scopes them).
- **Tests**: new `tests/unit/critic_runner_proxies.bats` covering predicate cases, multi-pattern union, stripped-proxy regressions, false-positive reproductions of the Conflab cases, and positive controls. Extensions to `tests/unit/intent_critic.bats` (stderr/JSON separation via `run --separate-stderr`) and `tests/unit/pre_commit_hook.bats` (single-step `case` and compliant async test no longer block).
- **Docs**: `intent/docs/critics.md` "Mechanical subset only" paragraph rewritten to document the strict-proxy contract.
- **VERSION** stays at 2.11.2 in the working tree; `scripts/release --patch` will bump to 2.11.3.
- **CHANGELOG** [2.11.3] section written.

## Next Up

Ship v2.11.3. Workflow:

1. Stage and commit (single coherent commit; pre-commit gate fires on the runner change since this project is `languages: ["shell"]`).
2. `scripts/release --dry-run --patch` to verify the bump path.
3. `scripts/release --patch` to cut.

Other v2.11.x backlog (deferred, not blocking):

- Audit `intent claude upgrade` Phase-2 CLAUDE.md substitution: a regex in there mangles "v2.0.0" in the migration-history paragraph because it's substituting greedily on any `v2.X.Y` not just the version banner. Hand-corrected this session; needs a proper fix before next release.
- Homebrew tap.
- `scripts/release` v2 polish (`--rollback`, log-to-file mirror, prettier output).

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

- **2026-04-29**: ST0039 implemented end-to-end. Strict-proxy contract in critic runner; 8 Elixir rules stripped + 1 surgical edit on IN-EX-CODE-004; multi-pattern union + once-per-rule stderr diagnostic; new BATS file plus extensions to existing critic + pre-commit suites. BATS green. v2.11.3 dry-run clean.
- **2026-04-29**: Field bug report received: pre-commit critic gate false positives on canonical Elixir idioms (single-step `case`, compliant `use ExUnit.Case, async: true`). Diagnosis traced two distinct runner bugs (`grep -L` inversion, multi-line proxy first-line-only) plus 8 over-broad rule proxies.
- **2026-04-28 (post-compact)**: ST0037 implemented end-to-end. v2.11.0 shipped. Schema migration, helpers, CLI verbs, probe replacements, BATS rework, docs and blog updates, CHANGELOG entry, VERSION bump.
- **2026-04-28 evening**: language-detection regression diagnosed; ST0037 plan agreed (Option B, explicit config field).
- **2026-04-28 afternoon**: two blog drafts in `docs/blog/_drafts/`. Detrope-clean except for the language-detection paragraph that ST0037 unblocked.
- **2026-04-28 mid-session**: skill-renderer trap regression guard test (`tests/unit/skill_renderer_trap.bats`); idempotence-test fix for AGENTS.md version probe.
- **2026-04-28 morning**: v2.10.1 shipped.

## Parked

_(None.)_
