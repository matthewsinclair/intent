---
verblock: "28 Apr 2026:v0.2: matts - Scope and design pointer"
intent_version: 2.10.1
status: WIP
slug: language-config-replace-filesystem-probes-with
created: 20260428
completed:
---

# ST0037: Language config: replace filesystem probes with explicit config

## Objective

Replace filesystem-probe-based language detection with an explicit per-project configuration field (`languages: []` in `intent/.config/config.json`). Make `intent lang init <lang>` and a new `intent lang remove <lang>` the only way languages enter or leave a project's "languages-in-use" set. Remove probes from all four canon sites that currently infer languages from file presence.

## Context

The design intent for Intent has always been: **languages-in-use is a per-project configuration decision, not a filesystem detection problem.** Filesystem presence (`mix.exs`, `Cargo.toml`, `Package.swift`, `.luarc.json`, shell shebangs) is unreliable evidence of intent — those files can show up for non-obvious reasons (a vendored example, a test fixture, a one-off script in `scripts/`).

In v2.10.x, four canon sites still probe the filesystem to decide which language-specific essentials to load:

- `intent/plugins/claude/skills/in-session/SKILL.md` (lines 39-47) — bootstrap detection table.
- `intent/plugins/claude/skills/in-review/SKILL.md` (lines 40-43, 77) — review skill, language-to-critic dispatch.
- `intent/plugins/claude/skills/in-tca-audit/SKILL.md` (lines 63-66) — TCA audit per-component critic dispatch.
- `lib/templates/hooks/pre-commit.sh` (lines 71-81) — pre-commit critic gate.

The imperative-config mechanism does exist: `intent lang init <lang>` installs `intent/llm/RULES-<lang>.md` and `intent/llm/ARCHITECTURE-<lang>.md` and adds an entry to the agnostic `intent/llm/RULES.md` Language Packs marker block. But **no consumer reads its output.** The four probe sites above all go around it.

This is a regression against design intent. Surfaced 2026-04-28 evening when a blog draft (`docs/blog/_drafts/####-claude-context-with-intent.md`) described the probe behaviour, the user flagged it as the opposite of what was supposed to be in place, and an audit confirmed all four sites.

User direction: "fix this PROPERLY, no half-measures" — Option B from the diagnostic discussion: explicit `languages: []` field in `intent/.config/config.json`, with a back-fill migration so existing fleet projects don't need user action.

This ST is a v2.11.0 ship line — the schema migration is a breaking change in the canon sense (even though the migration is automatic).

## Scope

14 items, full breakdown in `design.md`:

1. Schema: add `languages: []` to `intent/.config/config.json`.
2. Migration `migrate_v2_10_x_to_v2_11_0` in `bin/intent_helpers` — adds field, back-fills from `intent/llm/RULES-<lang>.md` presence.
3. `bin/intent_init` writes `languages: []` for new projects.
4. `bin/intent_lang init <lang>` writes the field (idempotent add).
5. New `bin/intent_lang remove <lang>` — writes the field (idempotent remove), deletes per-language `RULES-<lang>.md` and `ARCHITECTURE-<lang>.md`, removes the marker-block entry from agnostic `RULES.md`.
6. New helper `get_project_languages()` in `bin/intent_helpers`.
7. `intent/plugins/claude/skills/in-session/SKILL.md` — probe table replaced by config-driven prose.
8. `intent/plugins/claude/skills/in-review/SKILL.md` — probe block replaced by config read for critic dispatch.
9. `intent/plugins/claude/skills/in-tca-audit/SKILL.md` — same.
10. `lib/templates/hooks/pre-commit.sh` — probe block replaced by `get_project_languages` read.
11. BATS test rework: `tests/unit/in_session_skill.bats`, `tests/unit/pre_commit_hook.bats`, `tests/unit/intent_lang.bats`. Probe assertions out, config-read assertions in.
12. Docs: `intent/docs/working-with-llms.md` updates if it mentions language detection.
13. Blog draft `####-claude-context-with-intent.md` — rewrite "Language-specific essentials" paragraph.
14. Strip phantom skill refs to `/in-rust-essentials`, `/in-swift-essentials`, `/in-lua-essentials`, `/in-shell-essentials` from `in-session/SKILL.md` (lines 40-43) and `intent/docs/working-with-llms.md` (lines 300-303). Those skills were promised in WPNN ("ships in WP06" / "ships in WP12") but never landed; the rule-pack + critic-subagent path is the working mechanism for those four languages.

T-shirt: **M**.

## Related Steel Threads

- ST0035 — Canonical LLM Config + Fleet Rollout (v2.10.0). Established the three-file canon and pre-commit gate that ST0037 corrects in this dimension.
- ST0036 — Directory Relocation `.intent/` -> `intent/.config/` (v2.10.0). Established the config location ST0037 extends.

## Context for LLM

This document represents a single steel thread — a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document and the companion `design.md` to provide context about what needs to be done.

The full design — including polyglot-ordering convention, migration semantics, probe-site rewrite specs, and risk register — lives in `design.md`. Read that before writing any code.

### How to update this document

1. Update the status as work progresses (use `intent st` commands).
2. Update related documents (`design.md`, `impl.md`, `tasks.md`) as needed.
3. Mark the completion date when finished.
