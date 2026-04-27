# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, plus per-language skills. Releases the `UserPromptSubmit` strict gate via the per-project sentinel.
2. **Verify the working tree.** `git status` should be clean. ST0035 14 of 19 Done; WP-15 (canary rollout) WIP at 2 of 16.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/15/info.md`** -- WP-15 spec (note: `intent upgrade --dry-run` doesn't exist; Sites subdir doesn't exist; Pplr out of scope -- spec tidy-up is a sub-task before more canaries).
5. **Read `intent/st/ST0035/WP/15/canary-reports/laksa.md` and `anvil.md`** -- both reports double as templates for the next canary (12-point verification commands; legacy migration note for older projects).

## State (2026-04-27, end of session post-compact)

**Intent v2.10.0. ST0035 14 of 19 Done. Anvil canary done (2 of 16 in-scope projects). Surfaced + fixed a fleet-wide canon-installer gap (LEGACY single-file pre-commit migration). Conflab + Lamplight deferred (busy); Pplr out of scope. Tests 788/788; doctor clean.**

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 788/788 green (was 785; +3 new MIGRATE_LEGACY_PRE_COMMIT scenarios).
- **Doctor**: clean.
- **Pre-commit canonical layout**: canon body at `pre-commit.intent`; chain stub at `pre-commit`. Fresh installs and legacy projects both produce the chained architecture.

## What landed this session (newest first)

- `0724f88` -- WP-15 Anvil canary report.
- `d5b9203` -- canon-installer: LEGACY single-file pre-commit migration + INSTALL_PRE_COMMIT now installs chained architecture from the start. +3 BATS scenarios; fresh-install test asserts chained layout.
- `39c63bd` (in **Anvil**) -- `Intent upgrade to 2.10.0` (user-authored single commit; canon application + flybys: lazy_html `:only` removal; Anvil.Projects.create -> create_project for Ash 3.24 compat).

## Resume target -- next canary (ST0035/WP-15 continued)

User direction: do other fleet projects one at a time before Conflab/Lamplight (busy). Pplr out of scope.

Candidates: **Molt**, **Utilz**, **Arca**, **Prolix**, **MicroGPTEx**, **Sites**. Recipe (Laksa-tested):

1. Clean tree on the canary project (reset any stale `.intent/config.json` bumps).
2. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent claude upgrade )` -- canon dry-run.
3. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent upgrade )` -- chain migration + canon-apply.
4. 12-point verification (commands in Laksa's canary report).
5. Add `/AGENTS.md.bak` to project `.gitignore` if missing.
6. Commit + push to `local` (NOT upstream).
7. Write `intent/st/ST0035/WP/15/canary-reports/<project>.md`.
8. Commit the report in Intent.

After 2-3 more canaries, consider switching to batch mode for the rest of the ecosystem.

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
