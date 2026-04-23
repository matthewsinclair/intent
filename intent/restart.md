# Claude Code Session Restart

## Current State

Intent v2.8.2 (VERSION unchanged; release bump is WP11). **ST0034 active — 10/12 WPs done.** WP09 (Migration and upgrade chain) closed in this session. Working tree has uncommitted WP09 work; commit pending review (see `.claude/restart.md`).

## ST0034 status (as of 2026-04-23)

| Status        | WPs                                                                                                                                                                                                           |
| ------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Done (10)     | WP01 schema · WP02 ext system · WP03 rationalisation · WP04 agnostic · WP05 Elixir · WP06 Rust/Swift/Lua · WP07 critic family · WP08 worker-bee extraction · WP09 migration chain · WP12 shell + critic-shell |
| Remaining (2) | WP10 docs · WP11 release + fleet upgrade                                                                                                                                                                      |

Critical path: **WP10 → WP11**.

## What WP09 shipped

- `bin/intent_helpers`: three new functions sitting beside the v2.8.x helpers.
  - `migrate_v2_8_2_to_v2_9_0` — version stamp → ext bootstrap → worker-bee seed copy → prune installed elixir + worker-bee from `~/.claude/agents/` and `~/.intent/agents/installed-agents.json`. Idempotent via presence checks at every step.
  - `needs_v2_9_0_upgrade` — predicate, returns 0 for any version < 2.9.0; recognises the 2.9.x / 2.10.x / 3.x families as already-migrated.
  - `generate_ext_readme` — emits the README that ships with a freshly bootstrapped `~/.intent/ext/`. Pure file-emission, safe to call repeatedly.
- `bin/intent_upgrade` chain wiring: gate check at line 93 extended; new case arm `"2.8.2"`; every chain-tail (16 of them — 15 in the main case block + 1 in the pre-v2 chain) gained a trailing `migrate_v2_8_2_to_v2_9_0 .` call.
- `tests/unit/ext_migration.bats` (28 tests): predicate behaviour, README emission, version stamp + legacy-key cleanup, ext bootstrap + skip-when-present, worker-bee seed copy + skip-when-present, agent-file prune, manifest cleanup, idempotency, v2.8.1 → v2.9.0 chain coverage, plus three static gates on `bin/intent_upgrade` wiring.
- `intent/llm/MODULES.md`: `generate_ext_readme` registered alongside the existing `migrate_v2_8_2_to_v2_9_0` and `needs_v2_9_0_upgrade` entries.

## WP09 follow-up (do not block WP10)

- **Canary dry-run** against 3-5 fleet projects (Anvil, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex, Conflab per the WP09 spec). Procedure documented in WP/09/info.md §Canary projects. Touches real fleet projects outside this repo, so deliberately gated on user. Run before WP11 tags v2.9.0.

## What WP07 shipped (prior session)

- Four critic subagents (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`), each a thin orchestrator emitting a stable severity-grouped report.
- `critic-shell/agent.md` retrofitted to the same report format (Highlander across the family).
- `in-review/SKILL.md` stage-2 dispatcher (mix.exs / Cargo.toml / Package.swift / .luarc.json / shebang probes; polyglot user prompt).
- `intent/docs/critics.md` contract reference; `.intent_critic.yml` schema + sample.
- 16 fixtures + 3 BATS suites; 16-row Phase 7 verification matrix green.

## Recent commits

- `398de76` — WP07: critic subagent family (elixir/rust/swift/lua)
- `44e05d1` — WP12: shell rule pack + critic-shell subagent
- `c17d03b` — WP06: Rust, Swift, Lua rule packs
- `65f3cea` — WP08: extract worker-bee from canon to ext-seed
- `d2edb59` — Add /in-session bootstrap skill

## Deferred / observations

- **TCA skills retrospection**: with `critic-<lang>` family live, the `in-tca-*` suite is largely subsumed (TCA becomes "run the critics and synthesize findings"). Rewrite against the critic contract or retire — pick one before WP10/WP11 ship.
- **WP12 dogfood journal Entries 1-3**: deferred post-release.
- **Blog draft `docs/blog-drafts/shell-critic-inception.md`**: publication gated on real dogfood findings.
- **Worker-bee seed manifest `intent_compat.min`**: currently 2.8.2. WP11 must bump to 2.9.0 in lockstep with VERSION.
- **WP07 follow-ups** (small): align Diogenes fixture-context handling across the four critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`
