# Claude Code Session Restart

## First actions after `/compact`

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, Elixir skills (Intent authors Elixir rules even though it is itself a bash project), and the Persistent reminders block (Highlander / Thin Coordinator / PFIC diligence + NEVER MANUALLY WRAP .MD FILES).
2. **Verify tree is clean.** Expected top commits (newest first): `2e99857` · `1ae5f61` · `f4c68b9` · `b760b39` · `09cad07` · `d3c147d` · `61fad69` · `546dc3d` · `199c605` · `aa9b7ca`. If `git status` shows uncommitted work beyond `.claude/settings.local.json` (user-local, ignore), investigate.
3. **Read `intent/restart.md` + `intent/wip.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/11/info.md`** — the next active WP. Continue from its Deliverables list.
5. If time permits before the active WP, **read `intent/st/ST0035/info.md` + `design.md`** for canon decisions refresher, and **`intent/st/NOT-STARTED/ST0036/info.md` + `design.md` + `tasks.md`** for the ST0036 context that bundles into v2.10.0.

## State (2026-04-24, end of session — 11 of 18 Done + ST0036 opened)

**Intent v2.10.0 in progress. ST0035 active; ST0036 sibling Phase 0 stub opened (ships bundled).**

- 11 of 18 WPs Done: **WP01–WP10 + WP12**. No WIP — clean handoff.
- Retargeted v2.9.1 → v2.10.0 to bundle ST0036 (directory relocation `.intent/` → `intent/.config/`).
- Decisions 1–5 resolved; decision 1 retargeted to 2.10.0.
- `.intent/config.json`: `intent_version: 2.10.0`.
- `VERSION`: `2.10.0`.
- Full test suite: 762/762 green.
- `intent doctor`: clean.

## WP-11 resume target

Full spec: `intent/st/ST0035/WP/11/info.md`. Summary of work:

Extend `intent claude upgrade --apply` to ship the full canon in one shot. Today it handles AGENTS.md + intent/llm/RULES.md + ARCHITECTURE.md (after WP10's flip). WP11 adds idempotent installation of:

1. `.claude/settings.json` from `lib/templates/.claude/settings.json` + three helper scripts (`session-context.sh`, `require-in-session.sh`, `post-tool-advisory.sh`) → `.claude/scripts/`.
2. `.git/hooks/pre-commit` from `lib/templates/hooks/pre-commit.sh` (chmod +x).
3. `.intent_critic.yml` from `lib/templates/_intent_critic.yml` (only if absent — respect user customisation).
4. Root `CLAUDE.md` from `lib/templates/llm/_CLAUDE.md` (only if absent or if Intent-generated marker present and user hasn't edited outside `<!-- user:start --> / <!-- user:end -->`).

Idempotency: running `--apply` twice produces byte-identical output. MODULES.md update. BATS scenarios for each install target + the absence/presence/user-edit matrix.

Downstream: WP14 (Intent self-dogfood) is the first consumer. WP15 (canary) + WP16 (fleet) use this as the rollout machinery.

## Next up after WP-11

1. **WP13** (S) — Update Intent's own `CLAUDE.md` to reference the canon. Needs WP09 ✓.
2. **ST0036 Phase 0 elaboration** — populate 9 `WP/NN/info.md` files. Gate before ST0036/WP01 starts.
3. **ST0036/WP01–WP08** — migration function, path probes, literal sweep, templates, BATS, gitignore, migration guide, Intent self-apply. Land BEFORE ST0035/WP14.
4. **WP14** (S) — Self-apply canon to Intent (dogfood). Post-ST0036, this carries BOTH canon + directory relocation in one pass.
5. **WP15/WP16/WP17** — Canary + fleet rollout + verification sweep. Fleet rollout also carries both concerns.
6. **WP18** (M) — `intent/usr/*.md` audit (can run in parallel with WP15/16; must land before WP17).

See `intent/st/ST0035/tasks.md` + `intent/st/NOT-STARTED/ST0036/tasks.md` for dependency graphs.

## Session conventions

- T-shirt sizing only (XS/S/M/L/XL/XXL) — no clock-time estimates.
- Compact/refresh at ~200–250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations (octal gotcha: use `ST0035` or `35`, not `0035`).
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, or session docs.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0.

## Lessons worth keeping from this session

- **Mid-ST version retargets are cheap before release tag.** v2.9.1 → v2.10.0 was ~5 files of string replacement when no tag existed. Check the "is it tagged?" question before committing to a bundling strategy.
- **Deprecation sweeps leave ghost readers.** Deleting `intent/llm/AGENTS.md` required updating 5 other code paths that still wrote to it (`intent_init`, `_generate_basic_agents_md`, `intent_doctor`, `intent_claude_upgrade`, a BATS test). Always grep for the deleted path and scope the WP accordingly.
- **Test suite hides stale-file false positives.** `docs_completeness.bats::agents_sync_idempotent` was passing on stale `intent/llm/AGENTS.md` content post-WP08 — both runs copied the same file that `sync` wasn't even writing anymore. Periodic audit: does each test actually exercise the code path it claims to?
- **`_replace_symlink_if_present` is the migration primitive.** Any fleet project arriving with the old layout (root AGENTS.md → symlink to intent/llm/AGENTS.md) gets transparent, idempotent migration during `intent agents sync`.

## Open follow-ups (outside ST0035 + ST0036)

- `docs/blog/_drafts/####-shell-critic-inception.md` — blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 / ST0036 scope.
