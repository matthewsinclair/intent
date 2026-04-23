# Claude Code Session Restart

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 11/12 WPs done.** WP10 (Documentation) closed and committed (`6bb9d0d`). Tree is clean. Next: fix Task #26 (`intent agents sync` generator regressions surfaced during WP10) before WP11, so the fleet upgrade in WP11 regenerates clean AGENTS.md files.

## ST0034 status (as of 2026-04-23)

| Status        | WPs                                                                                                                                                                                                                                |
| ------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Done (11)     | WP01 schema · WP02 ext system · WP03 rationalisation · WP04 agnostic · WP05 Elixir · WP06 Rust/Swift/Lua · WP07 critic family · WP08 worker-bee extraction · WP09 migration chain · WP10 documentation · WP12 shell + critic-shell |
| Remaining (1) | WP11 release + fleet upgrade                                                                                                                                                                                                       |

Critical path: **WP11**.

## What WP10 shipped

### Three new docs

- `intent/docs/rules.md` — rule library guide (schema, IDs, authoring, validation, attribution, rule-reference skill pattern, critic consumption, ext-supplied rules).
- `intent/docs/writing-extensions.md` — expanded from the WP02 skeleton with the worker-bee worked example: anatomy on disk, manifest, lifecycle (scaffold/author/validate/install), debugging, deferred-publishing note.
- `intent/docs/critics.md` — added the registration-freeze operational note (subagent installs only visible after next session start).

### Updated docs and config

- CLAUDE.md (v2.9.0 architecture, drops elixir, adds critic-\* family, worker-bee relocation note, Migration Notes for v2.8.2 → v2.9.0).
- DECISION_TREE.md (three new branches: rule placement, skill placement, rule-vs-skill-vs-subagent).
- creating-custom-agents.md (canon vs ext distinction).
- Help files: `lib/help/{ext,rules,claude}.help.md` updated.
- AGENTS.md regenerated via `intent agents sync` (with a noted generator-deficiency follow-up).

### Mid-WP scope expansion (TCA suite + Elixir skill frontmatter)

Originally out of WP10 scope, folded in mid-WP after the TCA suite was found to still use pre-v2.9.0 R-numbering. Full Option A refactor:

- `in-tca-init` selects rule packs by ecosystem; drops invented R1-R15 numbering.
- `in-tca-audit` dispatches `critic-<lang>` per WP; drops the 100-line custom audit-prompt template.
- `in-tca-synthesize` consumes the stable critic report schema (CRITICAL/WARNING/RECOMMENDATION/STYLE + IN-\* IDs); maps cleanly to the existing P0/P1/P2a/P2b/P3 priority tiers.
- `in-tca-remediate` and `in-tca-finish` cite IN-\* IDs throughout; remediation FP guidance points at `.intent_critic.yml` for project-wide carve-outs.
- `intent/docs/total-codebase-audit.md` (1195 lines) updated: §0.1, §1.1, §1.2, §2.1, §2.2, Phase 0.5 grep section, Appendix B, Appendix C all reworked. Appendices D / E / F preserved with explicit pre-v2.9.0 framing notes.
- `in-elixir-essentials` and `in-elixir-testing` declare machine-readable `rules:` frontmatter listing the 13 IN-EX-CODE-_ and IN-EX-TEST-_ IDs they cite.

### CHANGELOG and release notes drafts (staged for WP11)

- `CHANGELOG.md` v2.9.0 entry drafted under `[Unreleased]` → `[2.9.0]`. No vanity metrics.
- `docs/releases/2.9.0/RELEASE_NOTES.md` drafted with full narrative (Rules library, Critic subagents, User extensions, Breaking changes, Upgrade, Acknowledgements, Migration notes for fleet projects).
- Both include the elixir-test-critic acknowledgement.

### Tests

- `tests/unit/docs_completeness.bats` (11 tests): 3 docs presence; cross-references from CLAUDE.md / MODULES.md / DECISION_TREE.md resolve; rule library + ext system mentioned in DECISION_TREE; cross-refs from new docs to siblings resolve; no_dead_refs to deleted elixir/canon worker-bee paths; agents_sync_idempotent.
- Full BATS suite: 707/707 ok (was 696 before WP10).
- `intent claude rules validate`: 48/48 ok.
- `intent doctor`: clean.

## WP10 follow-up (do not block WP11)

- **`intent agents sync` generator deficiencies**: drops `intent wp` commands (replaced with stale `intent bl`), can't detect Bats test framework, renders empty descriptions for some subagents. Not WP10 scope; needs dedicated ST or WP. Hand-edits to AGENTS.md get wiped by the next sync until fixed.

## What WP09 shipped (prior session)

- `bin/intent_helpers`: `migrate_v2_8_2_to_v2_9_0`, `needs_v2_9_0_upgrade`, `generate_ext_readme`. All idempotent.
- `bin/intent_upgrade` chain wiring: gate check + `"2.8.2"` case + 16 chain-tails.
- `tests/unit/ext_migration.bats`: 28 tests.

## Recent commits

- `6bb9d0d` — WP10: documentation pass + TCA suite refactor for rule library
- `b79e1a2` — WP09: v2.8.2 -> v2.9.0 migration step + chain wiring
- `398de76` — WP07: critic subagent family (elixir/rust/swift/lua)
- `44e05d1` — WP12: shell rule pack + critic-shell subagent
- `c17d03b` — WP06: Rust, Swift, Lua rule packs
- `65f3cea` — WP08: extract worker-bee from canon to ext-seed

## Deferred / observations

- **WP09 canary dry-run**: against fleet projects (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab). Procedure documented in WP/09/info.md §Canary projects. Touches real fleet projects outside this repo, so deliberately gated on user. Run before WP11 tags v2.9.0.
- **WP12 dogfood journal Entries 1-3**: deferred post-release.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood findings.
- **Worker-bee seed manifest `intent_compat.min`**: currently 2.8.2. WP11 must bump to 2.9.0 in lockstep with VERSION.
- **WP07 follow-ups** (small): align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
