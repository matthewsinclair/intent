# Claude Code Session Restart

## WIP

**ST0034 active, 8/12 WPs done.** Intent v2.8.2 (release bump is WP11). 48 rules validator-clean. 633/633 BATS green. Tree clean.

**Next up: WP07 (Critic subagent family)** — author `critic-elixir` first from the `critic-shell` template at `intent/plugins/claude/subagents/critic-shell/agent.md`, then clone for Rust/Swift/Lua. Size: Large (3-5 focused hours). Acceptance criteria in `intent/st/ST0034/WP/07/info.md`.

Remaining after WP07: WP09 (migration chain) · WP10 (docs) · WP11 (release + fleet). Critical path is **WP07 → WP09 → WP10 → WP11**.

## Session bootstrap

After `/compact` or context reset, run **`/in-session`** — loads `/in-essentials`, `/in-standards`, and language-specific skills (Elixir/Ash/LiveView when `mix.exs` matches). Intent itself is a bash project but the Elixir skills are the right set because Intent authors rules _for_ Elixir projects.

## Recent

- **2026-04-22**: WP06 + WP08 + WP12 shipped in one pre-compact session. Commits `44e05d1`, `c17d03b`, `65f3cea`, `d2edb59`. `/in-session` skill authored in the repo and installed globally the following session.
- **2026-04-15**: v2.8.2 released. ST0032 + ST0033 + upgrade-chain gap fix. Fleet of 16 projects at 2.8.2.

## Deferred

- **TCA suite retrospection**: `in-tca-*` likely subsumed by `critic-<lang>` family — rewrite or retire post-WP07. Tomorrow's problem.
- **WP12 dogfood journal Entries 1-3**: post-release. Requires live `Task(subagent_type="critic-shell")` invocations.
- **Blog draft** `docs/blog-drafts/shell-critic-inception.md`: publication gated on real dogfood runs.
- **Worker-bee seed `intent_compat.min` bump**: WP11 must bump 2.8.2 → 2.9.0 in lockstep with VERSION.

## Parked

- ST0010, ST0015: in `intent/st/NOT-STARTED/`

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations
- NEVER manually wrap lines in markdown
- NO Claude attribution in commits
- NEVER report test / skill / subagent counts in release notes, CHANGELOG, wip.md, or session docs (vanity metrics)
