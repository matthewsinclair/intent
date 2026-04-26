# Claude Code Session Restart -- narrative state

## Current state (2026-04-26, end of session -- ST0036 7 of 9 Done; WP08 in working tree; WP09 pending)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) 13 of 18 Done; WP14-WP18 pending. ST0036 (Directory relocation) 7 of 9 Done; WP08 half-done in the working tree, WP09 pending.**

### ST0035 shape

- **Done (13)**: WP01-WP13.
- **Not Started (5)**: WP14, WP15, WP16, WP17, WP18.

Critical path remaining: `ST0036/WP08 -> ST0036/WP09 -> ST0035/WP14 -> WP15 -> WP16 -> WP17`. WP18 (`intent/usr/*.md` audit) runs in parallel with WP15/WP16; must land before WP17.

### ST0036 shape

- **Done (7)**: WP01 (`4dcccce`), WP02 (`5369afd` + fix `33a99d0`), WP03 (`777c5b0`), WP04 (`5f8b61e` + earlier `f04db11`), WP05 (`b62ea58`), WP06 (`32df058`), WP07 (`1debc03`).
- **Not Started (2)**: WP08 (Intent self-apply -- half-done in the working tree), WP09 (coordination notes; pure docs).

### Progress this session (ST0036/WP04 + WP05 + WP06 + half-done WP08)

Five session commits, in order:

1. **`32df058` -- WP-06 ignore patterns**. New `lib/templates/_treeindexignore` template is the single source (Highlander cleanup of the inline heredoc in `bin/intent_treeindex::ensure_treeindexignore`). Canon installer ships it via new `INSTALL_TREEINDEXIGNORE` action. Granularity flipped from blanket `.intent/` to `intent/.config/cache/` + `intent/.config/backup/` so `config.json` stays indexed. End-to-end verified on synthetic v2.10.0 project: dry-run shows MISSING -> action queued; `--apply` installs byte-identical to template; second dry-run shows PRESENT and zero actions.
2. **`5f8b61e` -- WP-04 templates + generators**. Audit of `lib/templates/` + plugin generators for residual `.intent/` literals. Single material flip: `lib/templates/hooks/pre-commit.sh` (4 hits where the hook probed `.intent/config.json` to decide whether to skip the critic gate). Everything else either was already flipped in WP03 / WP06 / earlier fixes, or is `~/.intent/ext/` (user-level, KEEP). The `_usage-rules.md` Project Structure flip from the WP04 spec turned out to be N/A (no such section in the template; spec was speculative).
3. **`b62ea58` -- WP-05 BATS work**. `tests/lib/test_helper.bash::create_test_project` now mkdirs `intent/.config/` and stamps `intent_version: "2.10.0"`. 11 BATS files flipped (assertions + fixtures). New `tests/unit/migrate_v2_9_0_to_v2_10_0.bats` (6 scenarios: clean migration, idempotence, sentinel/symlink/conflict refusals, cross-FS placeholder skipped on macOS). Doctor sentinel scenario added to `global_commands.bats`. **Real bug fixed**: macOS BSD `mktemp /tmp/foo-XXXXXX.md` does not substitute X's when followed by a suffix -- creates the LITERAL file `foo-XXXXXX.md`. `agents_sync_idempotent` was silently broken on macOS by this; second run collided with "File exists". Fixed by dropping the `.md` suffix.
4. **WP-08 half-done in working tree (uncommitted)**. The user proposed and approved a diagnostic `mv .intent intent/.config` on Intent's own repo _before_ doing the per-test BATS flips. The result was a sharp signal: **+1 passing, 0 newly broken**. The +1 was test 374 (`intent critic dispatches to bin/intent_critic`) which had been failing because Intent's CLI rejected Intent's own repo as not-a-project (post-WP02 path probes were already correct -- they just needed the new layout to bind to). The 26 remaining failures were all per-test BATS fixture/assertion flips, none of which depended on Intent's own state. WP02 path probes were proven correct end-to-end. The rename remains uncommitted in the working tree as a half-done WP08; tomorrow's first task is to revert it and re-do via `intent upgrade` so the canon-apply Phase 3 also lands.

Verification: full BATS suite **774/774 green** post-WP05. `intent doctor` clean.

### Lessons worth keeping (this session)

- **A diagnostic mv catches what audits miss.** Renaming Intent's own `.intent/` -> `intent/.config/` before doing the per-test BATS flips was the user's proposal. Result: +1 passing, 0 newly broken. That single experiment proved WP02's path probes were complete and that the only remaining work was BATS fixture/assertion flips. Cheap diagnostic, sharp signal -- worth budgeting time for similar "what would happen if I just did the thing" probes early in any large mechanical sweep.
- **macOS BSD mktemp footgun**: `mktemp /tmp/foo-XXXXXX.md` creates the LITERAL file `foo-XXXXXX.md` -- the X's are not substituted when followed by a suffix. Two safe forms: `mktemp /tmp/foo-XXXXXX` (no suffix) or `mktemp -t foo-XXXXXX` (which on macOS lands in `$TMPDIR`, not `/tmp`). The bug is silent first time (literal file is created and "works") and fails on second run with "File exists". Easy to miss without idempotence-aware tests.
- **Highlander cleanup compounds**: WP06's extraction of the inline heredoc in `ensure_treeindexignore` to `lib/templates/_treeindexignore` made the canon installer's INSTALL_TREEINDEXIGNORE action a one-line `canon_install_file "$CANON_TREEINDEXIGNORE_SRC" ...` call. One template, two consumers, byte-identical install path. Same pattern as `_intent_critic.yml`, `_CLAUDE.md`, etc.
- **Commit-time interlock between WP05 and WP08**: WP05's "BATS green" requirement effectively depended on Intent being in v2.10.0 layout (because test 374 cross-checks the CLI from Intent's own repo). Resolved by keeping the manual mv uncommitted in the working tree -- WP05's commit shows BATS-only changes (suite is green at commit time because rename is on disk), WP08 will commit the rename + canon-apply diff together.
- **Path probes were proven by absence of failure.** WP02 flipped a lot of paths; the only way to be sure no literal slipped through was to physically change Intent's layout and watch what broke. Nothing did. That's stronger evidence than reading every grep result.

### Open follow-ups (outside ST0035 + ST0036)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 / ST0036 scope.

### Resume target -- ST0036/WP08 + WP09

See `.claude/restart.md` for the step-by-step resume sequence (revert mv -> `intent upgrade` -> verify -> commit -> WP09 doc updates).

### Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL) -- never clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations -- but until WP08 lands, manual Edit on WP info.md is the documented workaround for `intent wp done` rejecting Intent's own repo.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, restart.md.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Document first, code next, with a hard review gate after Phase 0.
