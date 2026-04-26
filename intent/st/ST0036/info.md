---
verblock: "26 Apr 2026:v0.4: matts - Phase 0 elaborated; awaiting review"
intent_version: 2.10.0
status: WIP
slug: directory-relocation-intent-to-intent-config
created: 20260424
completed:
---

# ST0036: Directory relocation: `.intent/` → `intent/.config/`

## Status

**Phase 0 elaborated (2026-04-26); awaiting user review.** High-level scope + design + WP backlog live here, in `design.md`, and in `tasks.md`. All nine `WP/NN/info.md` files populated with forensic detail (objectives, context, deliverables, approach, acceptance criteria, dependencies, implementation notes, risks, verification steps, exit checklists). Ships alongside ST0035 in a single **v2.10.0** release. WP01 begins after Phase 0 review passes.

## Objective

Relocate Intent's per-project metadata directory from top-level `.intent/` to nested `intent/.config/`, eliminating the long-standing "two top-level directories" smell where both `intent/` (project artefacts) and `.intent/` (configuration) coexist. After this ST lands, every Intent project has a single `intent/` top-level directory, with `.config/`, `st/`, `docs/`, `llm/`, `plugins/`, `usr/` etc. all nested beneath it.

## Context

Since v2.0.0, Intent projects have had two sibling top-level directories:

- `intent/` — all project artefacts (steel threads, docs, work tracking, the rule library, plugin canon).
- `.intent/` — the project's own configuration metadata (`config.json`, `backup/`, migration state).

The split originated from thinking of `.intent/` as "dotfile-style config" like `.git/`. In practice it is a persistent source of friction:

- Visual clutter at the project root (two almost-identically-named directories).
- Documentation ambiguity — users forget whether `MODULES.md` lives under `intent/llm/` or `.intent/llm/`.
- Fleet-rollout friction — every downstream project has both directories; every migration touches both; every doc reference must specify which one.
- `.gitignore` gymnastics — `.intent/cache/` and `.intent/backup/` are always-ignored, but `.intent/config.json` is always-committed. A single nested tree simplifies ignore patterns.

Moving `.intent/` → `intent/.config/` collapses the two directories into one coherent namespace without losing the "hidden config" semantic (the leading dot on `.config/` within `intent/` still signals "configuration, not content").

This is a **breaking change** per semver. Anything scripting against `.intent/config.json` (CI pipelines, editor plugins, ad-hoc `jq` invocations, third-party tooling) stops working until updated. Intent is fail-forward: no symlink fallback, no backwards-compat shim. The migration prunes the old location.

## Why ship with ST0035 in v2.10.0

Early in ST0035, the version bump was scoped as **v2.9.1** — a refinement release. Mid-ST, the user decided to bundle this directory relocation into the same release. Rationale:

- One upgrade pass for fleet projects (no double-roll).
- One coherent "v2.9.0 → v2.10.0" upgrade narrative.
- Version bump to v2.10.0 (breaking directory move forces the minor bump within 2.x).
- Shared rollout machinery: ST0035's canary (Conflab/Lamplight/Laksa) and fleet (12 Intent + Pplr) rollout WPs carry both the LLM canon changes _and_ the directory move in a single `intent upgrade`.

The two STs remain distinct concerns — ST0035 is "LLM config canon", ST0036 is "directory layout" — but they coordinate via shared release timing.

## Success Criteria

1. Every Intent CLI command that reads or writes `.intent/*` reads or writes `intent/.config/*` instead.
2. `intent upgrade` (via `migrate_v2_9_0_to_v2_10_0`) relocates `.intent/` to `intent/.config/` atomically on first invocation, pruning the old location.
3. All templates, documentation, BATS fixtures, `.gitignore` / `.treeindexignore` patterns, downstream project AGENTS.md / CLAUDE.md references point at the new location.
4. Intent self-applies cleanly.
5. Canary rollout (Conflab / Lamplight / Laksa, shared with ST0035) passes `intent doctor` + `intent claude upgrade --check` clean.
6. Fleet rollout (12 Intent-managed projects + Pplr, shared with ST0035) completes.
7. CHANGELOG v2.10.0 entry explicitly calls out the breaking directory move.
8. Migration guide `intent/docs/migration-v2.10.0.md` (new) gives users a short "what to update in your scripts" cheat sheet.

## Non-Goals

- **Not** an opportunity to bundle unrelated v2.10.0 breaking changes. Tight scope: directory move + the doc/template/test updates that follow mechanically.
- **Not** a deprecation cycle. Intent's fail-forward posture: the migration moves and prunes; no "support both paths for one version" period.
- **Not** a rename of `intent/` itself or any other top-level rearrangement.

## Related Steel Threads

- **ST0034** (Done, v2.9.0) — prior release; established the rule library + critic family + extension system.
- **ST0035** (active, v2.10.0) — Canonical LLM Config + Fleet Rollout. Shares the v2.10.0 release window and the fleet rollout WPs. Critical coordination.
- **ST0001** (Done, historical) — original STP → Intent migration where `.intent/` was introduced.

## Risks

High-level; full register in `design.md` once Phase 0 elaborates.

- Third-party tooling hard-codes `.intent/config.json`; upgrade breaks it silently.
- User CI pipelines / editor plugins / aliases reference the old path.
- Migration atomicity — failure mid-move leaves a project in an inconsistent state.
- Some files under `.intent/` are user-curated (custom config); the migration must preserve them (whole-directory move, no content understanding).
- Coordination with ST0035's fleet rollout — if ST0036 slips, ST0035's rollout either waits or rolls first (requires a second pass later).

## Size and Estimate

**Size: M (possibly L).** Shape likely 8–10 WPs. Phase 0 forensic elaboration will produce one `WP/NN/info.md` per work package once the ST is actively picked up; high-level WP list in `tasks.md`.

## Phase 0 gate

Phase 0 elaboration complete (2026-04-26):

1. ✓ All nine `WP/NN/info.md` files populated with forensic detail.
2. ✓ `design.md` carries D1-D5 canon decisions + risk register.
3. ✓ `tasks.md` reflects finalised T-shirt sizing.
4. **Awaiting review** -- user reads each WP info.md, validates scope/deps/risks, approves before WP01 starts.

Given the bundling with ST0035, this review gate is the same conversation as ST0035's next checkpoint.

## Context for LLM

This ST relocates `.intent/` to `intent/.config/` as part of v2.10.0 (bundled with ST0035's LLM canon work). The high-level shape and motivation live here + `design.md` + `tasks.md`. All nine WP `info.md` files are now populated (Phase 0 complete 2026-04-26).

When picking implementation up post-review:

1. Confirm ST0035 progress -- ST0036 WPs interleave **before** ST0035/WP14.
2. Re-read this file + `design.md` + `tasks.md` + each `WP/NN/info.md` for the WP about to start.
3. Re-read `intent/docs/working-with-llms.md` for current-as-of canon (written during ST0035/WP03).
4. Run `intent wp start ST0036/01`; commit per WP exit checklist.
5. Proceed in dependency order: WP01 -> WP02 -> WP03 -> {WP04, WP05, WP06} -> WP07 (parallel with WP01-WP02) -> WP08 -> WP09 + ST0035/WP14.
