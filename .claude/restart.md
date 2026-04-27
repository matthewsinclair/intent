# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, plus per-language skills. Releases the `UserPromptSubmit` strict gate via the per-project sentinel.
2. **Verify the working tree.** `git status` should be clean. ST0035 14 of 19 Done; WP-15 (canary rollout) WIP at 2 of 16.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/15/info.md`** -- WP-15 spec (note: `intent upgrade --dry-run` doesn't exist; Sites subdir doesn't exist; Pplr out of scope -- spec tidy-up is a sub-task before more canaries).
5. **Read `intent/st/ST0035/WP/15/canary-reports/laksa.md` and `anvil.md`** -- both reports double as templates for the next canary (12-point verification commands; legacy migration note for older projects).

## State (2026-04-27, end of session -- WP-15 canary 9 of 11 in-scope)

**Intent v2.10.0. ST0035 14 of 19 Done. WP-15 canary at 9 of 11 in-scope. Conflab + Lamplight deferred (busy); Pplr out of scope. Tests 788/788; doctor clean.**

This session: batch-applied canon to Molt, Utilz, arca_cli, arca_config, arca_notionex, Prolix, MicroGPTEx. All pushed to `local`. Reports under `intent/st/ST0035/WP/15/canary-reports/`.

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 788/788 green.
- **Doctor**: clean.
- **Pre-commit canonical layout**: canon body at `pre-commit.intent`; chain stub at `pre-commit`. Fresh installs, legacy single-file projects, and projects with foreign pre-commit hooks all converge on the chained architecture.

## What landed this session (newest first)

- WP-15 batch: `docs:` commit covering 5 canary reports (arca_cli, arca_config, arca_notionex, Prolix, MicroGPTEx).
- 7 project-side commits: Molt (`7abd972`), Utilz (`ed31017`), arca_cli (`2e7c14f`), arca_config (`ca85f26`), arca_notionex (`9de67e9`), Prolix (`4508e94`), MicroGPTEx (`b375d1f`). All pushed to `local`.
- `3e113ff` -- WP-15 Utilz canary report.
- `0a1d21e` -- WP-15 Molt canary report.
- `17fc5ca` -- session wrap (post-compact, before this batch).
- `0724f88` -- WP-15 Anvil canary report.
- `d5b9203` -- canon-installer: LEGACY single-file pre-commit migration + INSTALL_PRE_COMMIT now installs chained architecture from the start. +3 BATS scenarios.
- `39c63bd` (in **Anvil**) -- `Intent upgrade to 2.10.0` (user-authored; canon + flybys).

## Resume target -- Conflab + Lamplight (last two canaries)

9 of 11 in-scope done. Conflab + Lamplight deferred (busy). Recipe is mature -- just apply the same flow:

1. Clean tree (reset stale `.intent/config.json` bump if present).
2. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent upgrade )` -- one-pass chain + canon-apply.
3. 12-point verification.
4. Add `/AGENTS.md.bak` to project `.gitignore` if missing.
5. Commit + push to `local` (NOT upstream).
6. Write `intent/st/ST0035/WP/15/canary-reports/<project>.md` and commit in Intent.

After both land, run `intent wp done ST0035/15` and proceed to WP-16/17/18.

## Risks for next session

- **WP-15 spec drift**: info.md mentions `intent upgrade --dry-run` (doesn't exist), Sites subdir (Laksa doesn't have one), and "Conflab + Lamplight + Laksa" as the canary set (now 16 projects minus Pplr). Worth a 5-minute tidy-up before the next canary.
- **Per-project pre-flight**: any project with stale `.intent/config.json` bumps or other pending edits should be reset / committed before canon-apply. Laksa had a stale 2.8.2 -> 2.9.0 bump that would have collided with the chain migration.
- **CLAUDE.md drift in older projects**: pre-existing user CLAUDE.md (STP-era text) is preserved by the canon. Refresh is a separate decision; track per-project for the WP-17 dogfood journal.

## Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims; auto-detection rejected.
- Document first, code next.
- Pre-flight every canary: clean tree before applying.

## Lessons from this session

- **Each canary surfaces fleet-wide canon-installer bugs.** Anvil exposed the legacy single-file pre-commit (canon body at `pre-commit`, no `pre-commit.intent`); the installer reported `UP TO DATE` and took no action -- silent skip on every project that ran `intent claude upgrade` before chaining landed. Fix: detection branch + `MIGRATE_LEGACY_PRE_COMMIT` action + `INSTALL_PRE_COMMIT` updated to install chained architecture from the start (so fresh installs and migrations converge on the same end state). Each canary is also a proof-of-correctness for the installer itself.
- **Fresh installs and re-applies must converge on the same architecture.** If fresh installs produce state X and re-applies migrate to state Y, you have a non-idempotent installer that lies. Always check: does running the installer on a fresh project produce the same output as running it twice on a migrated one? If not, fix the installer, not the migration.
- **Old projects are dependency-bombs.** Anvil hadn't been compiled in months; `mix deps.get` hit two unrelated breakages (`lazy_html :only` conflict; `Anvil.Projects.create -> create_project` Ash code-interface drift). Plan canary sessions with the assumption that old projects will need flyby fixes -- budget time accordingly.

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
