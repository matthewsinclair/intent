# Design - ST0034: Agentic Software Engineering Suite

## Approach

ST0034 has three entangled initiatives that ship as a single Intent v2.9.0 release:

1. **Rationalise existing skills/subagents** around a new first-class concept: Rules.
2. **Introduce Critic subagents** covering Elixir, Rust, Swift, Lua — each with code/test modes.
3. **Build a user extension system** at `~/.intent/ext/<name>/` so users add skills, subagents, and rule packs without forking Intent. Worker-bee becomes the reference example.

The core design shift: **rules become the single source of truth**. Skills reference rule IDs; Critic subagents orchestrate rules against code; extensions contribute rules (and skills and subagents) through the same schema and discovery. This applies Intent's Highlander Rule to Intent itself.

Design is explicitly fail-forward: no deprecation stubs, no preservation of installed copies for deleted components. Migration actively prunes `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md`. Intent's user base is effectively the author and the 16-project fleet, which can absorb the breakage.

## Vision

```
                        +--------------------------+
                        |     Rules (atomic)       |
                        |                          |
                        |  rules/agnostic/*        | <- Highlander, PFIC,
                        |                          |    Thin Coordinator,
                        |  rules/<lang>/<cat>/*    |    No Silent Errors
                        |                          |
                        +------------+-------------+
                                     |
                +--------------------+--------------------+
                |                    |                    |
                v                    v                    v
        +---------------+   +----------------+   +----------------+
        | Skills        |   | Subagents      |   | User Extensions|
        | (in-*)        |   | (critic-<lang>)|   | (~/.intent/ext)|
        |               |   |                |   |                |
        | Reference     |   | Enforce rules  |   | Drop-in rules, |
        | rules by ID.  |   | against code.  |   | skills, and    |
        | Never inline  |   | Thin           |   | subagents.     |
        | rule content. |   | orchestrators. |   | Shadow canon   |
        +---------------+   +----------------+   | with warning.  |
                                                 +----------------+
```

## Design Decisions

### D1. Rules as first-class citizens

Rules live at `intent/plugins/claude/rules/` in the Intent repo. Each rule is its own directory with a `RULE.md` (YAML frontmatter + Markdown sections) and optional runnable `good.<ext>` / `bad.<ext>` files.

Language-agnostic rules at `rules/agnostic/` omit runnable examples and list `concretised_by:` language-specific rule IDs. Every agnostic rule must concretise in at least two language packs. This invariant stops `agnostic/` becoming a dumping ground for vague wisdom.

Rule IDs are stable for the life of a rule: slug renames OK (via `aliases:` frontmatter); numeric-suffix renumbering prohibited.

**Why first-class:** today the same rule ("pattern match over conditionals") lives in the `elixir` subagent, in `in-elixir-essentials` skill, and implicitly in `in-standards`. Three locations, no synchronisation contract. Making rules atomic collapses the duplication; skills become curated views over the library.

### D2. Rule ID scheme

Format: `IN-<LANG>-<CATEGORY>-<NNN>`

- `IN-AG-*` agnostic principles
- `IN-EX-CODE-*`, `IN-EX-TEST-*`, `IN-EX-ASH-*`, `IN-EX-PHX-*`, `IN-EX-LV-*` Elixir
- `IN-RS-*` Rust (plus sub-categories as the pack grows)
- `IN-SW-*` Swift
- `IN-LU-*` Lua

Rejected alternatives: dotted namespaces (`intent.elixir.test.no-shape`) are too long for inline reports and interact badly with Markdown link syntax; slugs alone provide no stable citation target when rules are renamed.

### D3. Rule schema (compatible with elixir-test-critic)

```yaml
---
id: IN-EX-TEST-001
upstream_id: test-critic-strong-assertions   # optional pointer to elixir-test-critic slug
slug: no-shape-tests
title: Strong assertions against concrete values
language: elixir
category: test
severity: critical                            # critical | warning | recommendation | style
applies_to: ["test/**/*_test.exs"]
references:
  - IN-AG-HIGHLANDER-001
aliases: []                                   # previous slugs, for stability
version: 1
---

## Problem
Shape assertions pass for any value of the right type; they do not prove the
function did what the test claims.

## Detection
Grep: `assert is_struct|assert is_map|assert is_list|refute is_nil` in `test/**/*_test.exs`.

## Bad example
See `bad.exs` (runnable: `mix test bad.exs`).

## Good example
See `good.exs`.

## When it applies
Any ExUnit test asserting on a return value of a fallible function.

## When it does not apply
Property-based tests that assert invariants; tests where the shape itself is the contract
(e.g. asserting a GenServer callback returns `{:noreply, socket}`).

## Further reading
- in-elixir-testing (skill)
- elixir-test-critic: test-critic-strong-assertions
```

Bash cannot parse YAML frontmatter directly. A generated `rules/index.json` (produced by `intent claude rules index`) is the bash-readable catalogue. Every field the CLI reads (id, severity, language, category) stays flat in the frontmatter so the JSON generator is a straightforward shell + jq pipeline. Nested fields are reserved for fields consumed only by Claude (which reads the Markdown directly).

### D4. elixir-test-critic integration: reference-and-recommend with schema adoption

Intent does not vendor all 81 upstream rules. Instead:

1. Intent adopts elixir-test-critic's frontmatter schema verbatim, adding three Intent-specific optional fields (`upstream_id`, `references`, `aliases`). Upstream tools ignore unknown fields.
2. Intent ships a curated subset of ~15-20 Elixir rules that cover what `in-elixir-*` skills already promise today, plus the agnostic pack. These rules are Intent's voice, attributed to elixir-test-critic where schema or principle is borrowed.
3. `in-elixir-testing` skill body ends with a pointer to the upstream plugin for the full 81 rules.
4. `critic-elixir` subagent detects whether the elixir-test-critic plugin is installed and, if so, loads its rules alongside Intent's, deduplicating by `upstream_id` where present.

**Attribution**: `intent/plugins/claude/rules/_attribution/elixir-test-critic.md` carries the full MIT notice (copyright 2026 Manuel Zubieta), source URL, and commit hash at time of port. Every rule that derives principle or detection heuristic from upstream carries `upstream_id:` in frontmatter. No rule prose is copied verbatim; Intent rewrites in its own voice.

**Why this over alternatives**:

- Fork-and-vendor 81 rules creates monthly merge debt and MIT per-file-notice obligations
- Reference-only (external plugin required) breaks Intent's existing `in-elixir-*` skills if users don't install the upstream
- Re-implement-from-scratch loses schema compatibility, making future upstream merges impossible

### D5. Critic subagent family (single per language, two modes)

Four new subagents:

- `critic-elixir` (modes: `code`, `test`)
- `critic-rust` (modes: `code`, `test`)
- `critic-swift` (modes: `code`, `test`)
- `critic-lua` (modes: `code`, `test`)

One subagent per language with modes beats two subagents per language because:

- Rule loading is shared across code and test modes (agnostic + language common); no duplicated mechanics.
- Invocation is cleaner: `Task(subagent_type="critic-elixir", prompt="review lib/x.ex")` vs. remembering two subagent names.
- Severity filtering, project config, and report format are uniform across modes.

**Contract** (uniform across all four critics):

- **Input**: invocation string specifying mode + target paths; optional `.intent_critic.yml` project config.
- **Process**: enumerate `rules/agnostic/*` + `rules/<lang>/<mode>/*` + `rules/<lang>/common/*`; read target files; apply each rule's Detection; collect violations.
- **Output**: stable, machine-parseable report grouped by severity, each finding citing rule ID, file:line, quoted snippet, suggested-fix summary.

`diogenes` remains as the Socratic test-spec generator. `critic-elixir test-check` can call `diogenes` for spec validation in test-specification workflows. They complement: `diogenes` produces specs; `critic-elixir` enforces rules.

### D6. elixir subagent removal (aggressive prune)

The `elixir` subagent is removed entirely in v2.9.0. Rule-like content migrates to `rules/elixir/**`; the orchestration role is subsumed by `critic-elixir`. No stub, no deprecation flag, no grace period.

**Migration actively prunes**: `migrate_v2_8_2_to_v2_9_0` deletes `~/.claude/agents/elixir.md` if present and removes its entry from `~/.intent/agents/installed-agents.json`. No orphans.

### D7. Extension system: `~/.intent/ext/<name>/`

Each extension is a self-contained directory with a declarative `extension.json` manifest and any combination of subagents, skills, and rule packs.

```
~/.intent/ext/
  <ext-name>/
    extension.json               # required manifest
    README.md                    # recommended
    LICENSE                      # optional but strongly recommended
    subagents/<name>/
      agent.md
      metadata.json
      resources/                 # optional, self-contained
    skills/<slug>/
      SKILL.md
      scripts/                   # optional
      data/                      # optional
    rules/<lang>/<slug>/
      RULE.md
      good.*
      bad.*
```

**Manifest** (`extension.json`):

```json
{
  "schema": "intent-extension/v1",
  "name": "worker-bee",
  "version": "1.0.0",
  "description": "Worker-Bee Driven Design specialist",
  "author": "matts",
  "license": "MIT",
  "homepage": "https://github.com/...",
  "intent_compat": { "min": "2.9.0", "max": "3.x" },
  "contributes": {
    "subagents": [{ "name": "worker-bee", "path": "subagents/worker-bee" }],
    "skills": [],
    "rules": []
  },
  "checksums": { "subagents/worker-bee/agent.md": "sha256:..." }
}
```

Only paths declared in `contributes` are exposed to discovery. Prevents stray files from accidentally shadowing canon.

### D8. Discovery and precedence

Root search order (highest first):

1. `$INTENT_EXT_DIR` (env override for tests)
2. `$HOME/.intent/ext/*/` (user extensions)
3. `$INTENT_HOME/intent/plugins/claude/{subagents,skills}/` (canon)

When an extension shadows canon (same name), user-ext wins, but every `list`, `show`, and `install` emits a visible shadow warning. No silent shadowing.

**Callback refactor** (additive, backward compatible). Today, `intent/plugins/claude/lib/claude_plugin_helpers.sh:7-21` defines 4 config vars and 8 callbacks. The refactor adds ONE optional callback and ONE library-side helper:

- New callback (optional): `plugin_get_source_roots` — echoes newline-separated root directories in precedence order. Default implementation (used if the plugin doesn't define it) returns only the canon root, matching v2.8.x behaviour exactly.
- New helper: `plugin_resolve_source_file NAME` — walks roots, returns the first hit. Called internally by install/sync logic.

Existing 8 callbacks unchanged. Bash 3.x compatible (newline-separated lists, no associative arrays).

### D9. Worker-bee extraction (aggressive prune + ext seed)

`worker-bee` is removed from Intent canon entirely. Content relocates to `lib/templates/ext-seeds/worker-bee/` as the seed for the reference extension.

**On upgrade**, `migrate_v2_8_2_to_v2_9_0`:

1. Creates `~/.intent/ext/` if absent (with a README).
2. Seeds `~/.intent/ext/worker-bee/` from `$INTENT_HOME/lib/templates/ext-seeds/worker-bee/` if the target does not already exist.
3. Deletes `~/.claude/agents/worker-bee.md` if present (aggressive prune).
4. Removes worker-bee entry from `~/.intent/agents/installed-agents.json`.
5. Removes worker-bee from `intent/plugins/claude/subagents/.manifest/global-agents.json` in canon (repo-level change).

Users who still want worker-bee after the prune run `intent claude subagents install worker-bee`; discovery resolves to `~/.intent/ext/worker-bee/subagents/worker-bee/` via the new `plugin_get_source_roots` callback. Fresh install, fresh state.

### D10. Skill and subagent rationalisation

- **`elixir` subagent**: DELETED. Rule-like prose migrates into `rules/elixir/**`; orchestration role subsumed by `critic-elixir` (D6). No stub.
- **`elixir/antipatterns.md`** (1852 lines): atomised into rule files under `rules/elixir/antipatterns/`.
- **`elixir/testing.md`**, **`style.md`**, **`ash-ecto.md`**, **`liveview.md`**: migrated to `rules/elixir/{test,code,ash,phoenix}/`.
- **`in-elixir-essentials`**, **`in-elixir-testing`**, **`in-phoenix-liveview`**, **`in-ash-ecto-essentials`**: refactored from content-holding skills to rule-reference skills (SKILL.md lists `rules:` by ID; Claude reads the individual `RULE.md` files on demand).
- **`in-standards`** (hollow today): references `rules/agnostic/*` and becomes an actionable load-these-rules directive.
- **`in-review`** stage-2: language-parameterised; detects project ecosystem (mix.exs / Cargo.toml / Package.swift / `*.lua` entry) and delegates to the right `critic-<lang>`.
- **`diogenes`**, **`socrates`**, **`intent`**: unchanged in role.
- **`worker-bee`**: relocated to `~/.intent/ext/worker-bee/` (D9).

## Naming Conventions

### Rule IDs

`IN-<LANG>-<CATEGORY>-<NNN>`. Fixed segments. Numeric suffix never reused.

### Skills

Unchanged `in-*` prefix. All 22 existing skills keep their names. No renames in v2.9.0.

### Subagents

- `critic-<lang>` — new family for rule enforcement (elixir, rust, swift, lua).
- `elixir` — DELETED in v2.9.0.
- `worker-bee` — REMOVED from canon, relocated to `~/.intent/ext/worker-bee/`.
- `diogenes`, `socrates`, `intent` — unchanged.

### Directories

- `intent/plugins/claude/rules/` — canonical rule library.
- `intent/plugins/claude/rules/_attribution/` — MIT notices for ported content.
- `intent/plugins/claude/rules/_schema/` — schema docs and tooling.
- `lib/templates/ext-seeds/` — seeds for user extensions.
- `~/.intent/ext/` — user extension root (created by migration).

## Alternatives Considered

### Critic topology: one subagent per language vs. separate code/test critics

Rejected the separate-per-mode approach. Duplicates rule-loading machinery, doubles subagent count, invites confusion about which to invoke. Mode-as-argument is cleaner.

### elixir-test-critic integration: fork-and-vendor vs. reference-and-recommend

Rejected full fork-and-vendor. MIT license obligates per-file notices; upstream is actively maintained (last commit 2026-04-17) which guarantees merge debt; Intent doesn't need all 81 rules for v2.9.0 parity with its existing `in-elixir-*` skills. Schema adoption + curated subset + upstream recommendation gives us depth-on-demand without the maintenance tax.

### Rule IDs: dotted namespaces vs. hyphenated

Rejected dotted (`intent.elixir.test.no-shape`). Too long for inline reports; interacts poorly with Markdown link syntax; awkward in grep output. Short hyphenated IDs survive every usage context.

### Plugin manifest format: reuse plugin.json vs. new extension.json

Rejected reuse. Existing `plugin.json` describes CLI commands; extensions describe content. Conflating them would force misleading empty fields and hurt validation. New format, clear purpose.

### Extension system: flat drop-in vs. self-contained directory with manifest

Rejected flat drop-in. No place for version/author/compat metadata; can't bundle skill + subagent + rules atomically; no manifest to validate. Wins on simplicity, loses on provenance, which is the core value.

### Release numbering: v2.9.0 vs. v3.0.0

Picked v2.9.0. Breaking changes (elixir and worker-bee removal from canon) have a contained blast radius in the 16-project fleet. New capabilities are additive. v3.0.0 reserved for contract-level breaks (config schema redesign, callback contract change, substrate change).

### Worker-bee migration: preserve-installed-copies vs. aggressive-prune

Picked aggressive-prune per explicit user direction. Intent's user base is effectively the author; fail-forward beats preservation dances. Installed copies are noise; clean slate forces users to opt back in if they actually want the component.

### elixir subagent lifecycle: deprecate-then-remove vs. remove-in-one-release

Picked remove-in-one-release per fail-forward preference. No stubs, no two-release grace period. Users update invocations or accept the removal.

## Risk Register

Backwards-compatibility risks are explicitly not in scope (fail-forward design).

| #   | Risk                                                                      | Impact  | Likelihood     | Mitigation                                                                                                                                                             |
| --- | ------------------------------------------------------------------------- | ------- | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | Rule ID churn as library grows                                            | Med     | Med            | Numeric suffix permanent once shipped; slug renames via `aliases:` frontmatter only; documented in `rules/_schema/`                                                    |
| R2  | Rule-to-skill drift (skill cites rule X but rule edited to not enforce X) | Med     | High           | `intent claude rules index` validates every skill's `rules:` list resolves to existing rule files; CI gate before release                                              |
| R3  | elixir-test-critic schema changes upstream                                | Low     | Med            | Pin Intent to a specific upstream commit hash; document freeze in `_attribution/`; adopt upstream schema bumps only on Intent major versions                           |
| R4  | Bash cannot parse frontmatter                                             | Certain | Low            | `rules/index.json` (shell + jq generated) is the bash-readable catalogue; frontmatter is Markdown-only; flat schema keys for jq reliability                            |
| R5  | Agnostic rules become a dumping ground                                    | Med     | Med            | Invariant: every agnostic rule lists at least 2 `concretised_by:` language rules. WP04 acceptance enforces this.                                                       |
| R6  | Critic reports are noisy (20 violations per file)                         | Med     | High initially | Severity tiers; `.intent_critic.yml` project-level disable; default report shows critical + warning only                                                               |
| R7  | Bash 3.x incompat sneaks into callback refactor                           | Med     | Med            | Existing callbacks already avoid `declare -A`; same patterns; CI runs on macOS; code review checklist                                                                  |
| R8  | Upgrade chain breakage (v2.8.2 slipstream precedent)                      | High    | Med            | BATS test exercises every starting-version case; dry-run against three fleet projects before WP11 rollout; no release commit until all dry-runs clean                  |
| R9  | Highlander violations: rule text duplicated across language packs         | Med     | High           | Agnostic pack is authoritative for Highlander/PFIC/Thin-Coord/No-Silent-Errors; language rules `references:` them, never restate. WP07 runs `grep -F` duplication scan |
| R10 | No CI environment for Rust/Swift/Lua to validate runnable examples        | Med     | High           | Ship textual good/bad examples only for WP06 languages; record limitation in `rules/_schema/`; validation happens when `critic-<lang>` runs against real projects      |
| R11 | MIT attribution non-compliance                                            | High    | Low            | Attribution file gates WP05 start; full MIT text + 2026 Manuel Zubieta + source URL + commit hash required; in-review checklist item                                   |
| R12 | Scope creep during rule authoring                                         | High    | High           | WP06 caps rules per language at 5-15 for v2.9.0; additional rules via user extensions or future ST                                                                     |
| R13 | Markdown linter auto-format inflates diffs during rule authoring          | Low     | High           | Single markdownlint baseline pass at WP01 start; include linter-only changes in commits per MEMORY.md guidance                                                         |
| R14 | Shadow warnings too noisy                                                 | Med     | Med            | Warning only when shadow is detected on list/show/install, not on every invocation; document `INTENT_EXT_DISABLE=1` escape hatch                                       |
| R15 | Fail-forward breakage missed by pre-release checks                        | Med     | Med            | Full BATS suite green before release; `intent doctor` clean on Intent repo; canary batch (5 projects) runs full post-upgrade acceptance before batch 2                 |

## Critical Path and Parallelisation

```
WP01 (schema) ---> WP02 (ext system) ---> WP08 (worker-bee extract) --+
   |                                                                  |
   +--> WP03 (rationalisation) --+                                     |
   |                             |                                     |
   +--> WP04 (agnostic pack) ----+--> WP07 (critic family) -----------+
   |                             |                                     |
   +--> WP05 (elixir pack) ------+                                     |
   |                             |                                     |
   +--> WP06 (rust/swift/lua) ---+                                     |
                                                                       v
                                             WP09 (migration) ---> WP10 (docs)
                                                       |                |
                                                       +--> WP11 (release and fleet)
```

**Serial critical path**: WP01 → WP02 → WP08 → WP09 → WP11. Roughly 18-22 sessions.

**Parallel opportunities**:

- WP03, WP04, WP05, WP06 unblock after WP01 (different directories, minimal shared files).
- WP07 is the main join point.
- WP10 can begin draft in parallel with WP09 once WP07 lands.

**Recommended single-operator sequence**: WP01, WP02, then WP04, WP05, WP03, then WP06, WP08, WP07, then WP09 and WP10 in parallel, WP11.

## Release Strategy

### Version: v2.9.0

Breaking changes are explicit (`elixir` subagent deleted, `worker-bee` removed from canon) but blast radius is contained to the author's own fleet. Fail-forward by design. New capabilities are additive (rules, critics, extension system). Reserve v3.0.0 for a contract-level break.

### Release Checklist

Run from `/Users/matts/Devel/prj/Intent`:

1. `git status` and `git log --oneline -5` — confirm clean tree.
2. Edit `VERSION` to `2.9.0`.
3. Update `CHANGELOG.md` with `[2.9.0] - YYYY-MM-DD` section. No test/skill/subagent counts.
4. Write `docs/releases/2.9.0/RELEASE_NOTES.md`. Thank Manuel Zubieta / elixir-test-critic for schema inspiration.
5. Update `intent/wip.md` and `.claude/restart.md` with 2.9.0 summary.
6. Stage specific files and commit with message `Release v2.9.0 -- Agentic Software Engineering Suite (ST0034)`. No Claude attribution.
7. `git tag -f v2.9.0 HEAD`.
8. Push to both remotes: `git push local main`, `git push -f local v2.9.0`, `git push upstream main`, `git push -f upstream v2.9.0`.
9. `gh release create v2.9.0 --notes-file docs/releases/2.9.0/RELEASE_NOTES.md`.
10. Fleet upgrade in 3 batches of 5-6 (canary first), halt on any error.
11. Final `intent/wip.md` / `.claude/restart.md` update with release confirmation.

If a slipstream is needed: prefer v2.9.1 over re-tagging.

## Testing Strategy

Testing discipline is a first-class ST0034 concern, not an afterthought. The BATS suite at `tests/run_tests.sh` is the canonical gate: every WP lands with the suite green, and release v2.9.0 does not ship until it is pristine.

### The pristine invariant

**At the end of every WP, `./tests/run_tests.sh` must exit 0 with no skips attributable to ST0034 work.** Pre-existing environment-gated skips (e.g. `skip_if_no_elixir` when the Elixir runtime is missing) are acceptable; new skips introduced to paper over broken tests are not.

Concretely, a WP is not considered "done" for the purposes of this ST until:

1. `./tests/run_tests.sh` exits 0.
2. Total test count has not regressed.
3. Any tests disabled or refactored as part of the WP are documented in that WP's `impl.md`.

This invariant is enforced by the release checklist (see §Release gate below) — but operationally, the author runs the suite after every WP's commits, not only at release time.

### Baseline at WP-01 start

469 BATS tests across 21 unit + 2 integration files, all passing. This is the starting line; no regressions allowed.

### Per-WP test surfaces

#### WP01 — Architecture and rule schema (design-only)

No runtime changes; no test additions. The archetype's `good_test.exs` / `bad_test.exs` are validated ad-hoc by `elixir <path>` during WP01 authoring (both must exit 0) and become CI inputs once WP02 ships the rule validator.

- **Tests to add**: none.
- **Tests to update**: none.

#### WP02 — Extension system foundation

Core test territory. All new.

- **Tests to add**:
  - `tests/unit/ext_commands.bats` — `intent ext list|show|validate|new` command surface (argument parsing, error paths, output format).
  - `tests/unit/ext_discovery.bats` — multi-root search order (`$INTENT_EXT_DIR` → `~/.intent/ext/` → canon), shadow warning emission on collisions, `INTENT_EXT_DISABLE=1` escape hatch.
  - `tests/unit/rule_validator.bats` — `intent claude rules validate` against the archetype (must pass) and against fixtures with known frontmatter errors (must report specific violations).
  - `tests/unit/rule_index.bats` — `intent claude rules index` shell+jq pipeline round-trip (index.json is regenerable and deterministic).
- **Fixtures to add**:
  - `tests/fixtures/extensions/valid-ext/` — minimal manifest, one skill, one subagent.
  - `tests/fixtures/extensions/malformed-ext/` — invalid `extension.json` (missing `schema`, unknown keys, bad semver).
  - `tests/fixtures/extensions/shadow-ext/` — name collides with a canon skill (`in-standards`).
  - `tests/fixtures/extensions/traversal-ext/` — manifest declares paths with `../` (must be rejected).
  - `tests/fixtures/rules/{valid,missing-frontmatter,bad-id,duplicate-id,unresolved-reference,unknown-field}/` — validator error paths.
- **Tests to update**: none from the pristine suite; WP02 adds net-new code paths.

#### WP03 — Skill and subagent rationalisation

Deletes the `elixir` subagent from canon. Existing tests that use `elixir` as a fixture must swap.

- **Tests to update** (in `tests/unit/agent_commands.bats`):
  - ~15 cases at lines 90, 178, 181, 188, 195, 205, 389, 396, 457, 461, 464, 469, 474, 481, 589, 591, 608, 611, 624, 629, 631, 737, 745, 749 (baseline grep).
  - Swap strategy: use `intent` (existing, always shipping) as the primary canonical subagent fixture. For tests that need _two_ subagents to exercise batch paths, use `intent` + `diogenes`.
  - One negative case added explicitly: `intent claude subagents show elixir` must report "not found" (proves WP03 completed the deletion).
- **Tests to update** (in `tests/unit/skills_commands.bats`):
  - Cases that currently install `in-elixir-essentials`: skill still exists after WP03 (content shifts to rule references). Tests that assert only on install/uninstall/sync mechanics continue to pass unchanged. Any test that greps SKILL.md for specific content must be refreshed against the refactored SKILL.md.
- **Tests to add**:
  - `tests/unit/rule_reference_skills.bats` — asserts that `in-standards`, `in-elixir-essentials`, `in-elixir-testing`, `in-phoenix-liveview`, `in-ash-ecto-essentials` each have a `rules:` list in frontmatter, and every listed ID resolves against the rule library.
  - `tests/unit/highlander_audit.bats` — greps for duplicated rule prose across skills + rules (the Highlander enforcement for rule content). Builds on the existing Highlander rule in CLAUDE.md.

#### WP04 — Agnostic rule pack

- **Tests to add**:
  - `tests/unit/rule_pack_agnostic.bats` — Highlander, PFIC, Thin Coordinator, No Silent Errors each have a well-formed `RULE.md`, each cites at least 2 `concretised_by:` language rules (invariant gate).
- **Tests to update**: none.

#### WP05 — Elixir rule pack

- **Tests to add**:
  - `tests/unit/rule_pack_elixir.bats` — every rule under `rules/elixir/**/RULE.md` has valid frontmatter and passes the validator. Rules with `upstream_id:` have a matching row in `_attribution/elixir-test-critic.md`.
  - `tests/unit/rule_pack_elixir_runnable.bats` — for each Elixir rule with `good_test.exs` / `bad_test.exs`, both exit 0 under `elixir <path>`. Gated by `skip_if_no_elixir`.
  - `tests/unit/attribution_compliance.bats` — every `upstream_id:` value corresponds to a real slug at the pinned commit; attribution file's pinned commit SHA matches the verification snippet.
- **Tests to update**: none.

#### WP06 — Rust, Swift, Lua rule packs

- **Tests to add**:
  - `tests/unit/rule_pack_rust.bats` — frontmatter validity, required sections present, fenced `rust` blocks in `## Bad` / `## Good`.
  - `tests/unit/rule_pack_swift.bats` — same for Swift.
  - `tests/unit/rule_pack_lua.bats` — same for Lua.
  - All three verify textual-only convention per `CI-LIMITATIONS.md`: no `.rs` / `.swift` / `.lua` sibling files exist.
- **Tests to update**: none.

#### WP07 — Critic subagent family

- **Fixtures to add**: `tests/fixtures/critics/<lang>/` for each language, each containing:
  - `known-violating/` — source files that MUST be flagged by the Critic (positive detection).
  - `known-clean/` — source files that MUST NOT be flagged (negative detection).
  - A `manifest.txt` listing the rule IDs each file is expected to trigger (or not).
- **Tests to add**:
  - `tests/unit/critic_dispatch.bats` — `in-review` stage-2 language detection (filesystem probes for `mix.exs`, `Cargo.toml`, `Package.swift`, `.lua` files); verifies correct Critic is selected per ecosystem.
  - `tests/unit/critic_report_format.bats` — stable report shape across all four Critics: severity grouping, rule-ID citation format, suggested-fix line, summary line.
  - `tests/unit/critic_config.bats` — `.intent_critic.yml` per-project config (disabled rules, severity threshold) is honoured; invalid config is reported not silently ignored.
- **Tests to update**: none from pristine; new critic-\* subagents are net-new to `agent_commands.bats`-style fixtures (consider adding a `critic-elixir` install test case, optional).

#### WP08 — Worker-bee extraction

- **Tests to add**:
  - `tests/unit/ext_migration.bats` — first-run seed copies `lib/templates/ext-seeds/worker-bee/` into `~/.intent/ext/worker-bee/`; second run is a no-op; pre-existing `~/.intent/ext/worker-bee/` is preserved.
  - `tests/unit/ext_migration.bats::prune_installed_worker_bee` — if `~/.claude/agents/worker-bee.md` exists pre-migration, it is deleted; if not, no error.
  - `tests/unit/ext_seed_validity.bats` — the seed directory passes `intent ext validate worker-bee` as-shipped.
- **Tests to update**: `tests/unit/agent_commands.bats` — any test that lists canon subagents must no longer see `worker-bee`.

#### WP09 — Migration and upgrade chain

- **Tests to add** (extend `tests/unit/ext_migration.bats`):
  - One case per prior version (2.0.0 → 2.1.0 → 2.2.0 → 2.3.0 → 2.5.0 → 2.6.0 → 2.7.0 → 2.8.0 → 2.8.1 → 2.8.2 → 2.9.0) exercising the full chain through `migrate_v2_8_2_to_v2_9_0`.
  - Idempotency gate: run migration twice in a row, second run is a no-op, no errors.
  - Aggressive-prune assertions: after migration, `~/.claude/agents/elixir.md` and `~/.claude/agents/worker-bee.md` do not exist even if they did pre-migration.
  - `~/.intent/agents/installed-agents.json` has no rows for `elixir` or `worker-bee`.
- **Tests to update**: any existing upgrade-chain test (if present) gets the new case appended without disrupting prior cases.

#### WP10 — Documentation

- **Tests to add**: `tests/unit/docs_completeness.bats` — presence of `intent/docs/writing-extensions.md`, `intent/docs/rules.md`, `intent/docs/critics.md`, and cross-references from CLAUDE.md / MODULES.md resolve.
- **Tests to update**: `tests/unit/agent_commands.bats::AGENTS_sync` round-trip test stays green with regenerated AGENTS.md.

#### WP11 — Release and fleet upgrade

No test additions. Release gate is the full suite (see below).

### Fixture inventory (consolidated)

All WP02-WP09 fixtures live under `tests/fixtures/`. Organisation:

```
tests/fixtures/
├── extensions/
│   ├── valid-ext/               # WP02
│   ├── malformed-ext/           # WP02
│   ├── shadow-ext/              # WP02
│   └── traversal-ext/           # WP02
├── rules/
│   ├── valid/                   # WP02 — validator happy path
│   ├── missing-frontmatter/     # WP02 — validator error paths
│   ├── bad-id/                  # WP02
│   ├── duplicate-id/            # WP02
│   ├── unresolved-reference/    # WP02
│   └── unknown-field/           # WP02
├── critics/
│   ├── elixir/
│   │   ├── known-violating/     # WP07
│   │   ├── known-clean/         # WP07
│   │   └── manifest.txt         # WP07
│   ├── rust/…                   # WP07
│   ├── swift/…                  # WP07
│   └── lua/…                    # WP07
└── upgrade/                     # WP09
    ├── v2.0.0-project/          # simulated stale project
    ├── v2.7.0-project/
    └── v2.8.2-project/
```

Fixtures are checked into the repo. They are NOT generated by tests — they are inputs. Fixture rot is itself a test failure: if a fixture goes stale (e.g. a manifest.json format change), the fixture is updated, not the assertion relaxed.

### Release gate

Before tagging v2.9.0:

1. `./tests/run_tests.sh` — exits 0, total test count ≥ 469 (baseline) + WP additions.
2. `intent doctor` in the Intent repo — clean.
3. Rule validator full pass (WP02 delivers): `intent claude rules validate` exits 0.
4. Rule index regenerates cleanly: `intent claude rules index` produces a deterministic `rules/index.json` (byte-identical on re-run).
5. Archetype runnable: `elixir intent/plugins/claude/rules/_schema/archetype/strong-assertions/good_test.exs` and `elixir .../bad_test.exs` both exit 0.
6. Canary fleet batch (5 projects): each passes post-upgrade acceptance before batch 2 begins.

If any gate fails, do not tag. Fix root cause, re-run from gate 1.

### Integration scenarios

Covered in the BATS suite where possible, documented here when they exercise cross-component behaviour:

- **Fresh clone upgrade**: clone Intent at a 2.0.0-era checkpoint, `intent upgrade --apply`, verify landing at 2.9.0 clean. Exercised by `tests/unit/ext_migration.bats` via fixture projects.
- **User-ext shadow scenario**: install a test extension named `in-standards` (colliding with canon); verify warning on every list/show/install, verify ext version is served by discovery. Covered by `tests/unit/ext_discovery.bats`.
- **User-ext non-shadow**: install a novel extension with a unique skill name; verify it appears in `intent claude skills list`. Covered by `tests/unit/ext_commands.bats`.
- **Ext discovery post-upgrade**: after upgrade, `intent claude subagents list` shows no `elixir`, no `worker-bee` from canon; `worker-bee` sourced from `~/.intent/ext/` if seeded; new `critic-*` all present. Covered by fleet validation (manual) because it exercises `~/.intent/` state on the author's machine.

### Fleet validation (unchanged from prior section)

Canary batch (5 projects) runs full post-upgrade acceptance before batch 2 starts:

- `.intent/config.json` intent_version = 2.9.0.
- `intent doctor` reports clean.
- `intent claude subagents list` shows no `elixir`, no `worker-bee` from canon.
- `~/.intent/ext/worker-bee/` exists and validates.
- Critic subagents discoverable.
- Existing STs and WPs unchanged.

### What "pristine" excludes

The pristine invariant is about the BATS suite specifically. It does NOT require:

- All skill/subagent `.md` files to be unchanged (WP03/WP10 modify many).
- All upstream-sourced content to be stable (we pin, but upstream can force-push — tracked separately in `_attribution/`).
- Zero lint warnings across the repo (linter discipline is a separate concern; see MEMORY.md for `feedback_markdownlint_baseline.md` once it exists).

Scope: `./tests/run_tests.sh` exits 0, no new skips attributable to this ST, total count ≥ 469 + WP additions.
