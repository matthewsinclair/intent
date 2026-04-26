# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, plus per-language skills. Releases the `UserPromptSubmit` strict gate via the per-project sentinel.
2. **Verify the working tree.** `git status` should show two interesting items:
   - Deleted: `.intent/config.json` (the original location)
   - Untracked: `intent/.config/` (the manual mv from the WP05 diagnostic)
   - Plus user-local `.claude/settings.local.json` (ignore as always).
     If those two items are gone, somebody already committed the rename -- check `git log --oneline -5` for a `chore: ST0036/WP-08` commit.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0036/impl.md`** -- it carries the WP-by-WP commit table and the open items for tomorrow.

## State (2026-04-26, end of session)

**Intent v2.10.0 in progress. ST0035 13 of 18 Done. ST0036 7 of 9 Done; WP08 half-done in the working tree, WP09 pending.**

- **VERSION**: `2.10.0` (already stamped).
- **Layout**: working tree currently has Intent's metadata at `intent/.config/` (post-mv); HEAD still has it at `.intent/`. Reconcile by running WP08 properly first thing tomorrow (see below).
- **Tests**: 774/774 green (verified mid-WP05).
- **Doctor**: clean.
- **Recent commits (this session, newest first)**: `b62ea58` WP-05 BATS · `5f8b61e` WP-04 templates · `32df058` WP-06 ignore patterns. Five fixes from previous session also in HEAD: `255436c` `ef2dd0e` `f04db11` `81c2b30` `33a99d0`.

## Resume target -- ST0036/WP08 finalisation

Run order tomorrow:

1. `cd /Users/matts/Devel/prj/Intent && mv intent/.config .intent` -- revert the working-tree rename so Intent is back to v2.9.x layout.
2. `intent upgrade` -- invokes `migrate_v2_9_0_to_v2_10_0`:
   - Phase 1 (relocate): `intent_relocate_dotintent` does the atomic mv with sentinel + recovery handling.
   - Phase 2 (stamp): no-op (already 2.10.0).
   - Phase 3 (canon-apply): `intent claude upgrade --apply` runs; expect a few REFRESH actions for cosmetic drift (date stamps, etc.).
3. `tests/run_tests.sh` -- expect 774 green.
4. `intent doctor` -- expect clean.
5. `intent treeindex intent --prune` -- regenerate Intent's own treeindex against the new layout (was deferred from WP06 because Intent's CLI couldn't run on Intent until the relocation).
6. Commit: `chore: ST0036/WP-08 Intent self-apply v2.10.0 directory move`.

Then **WP09** (XS) -- pure docs:

- Add directory-state verification step to `intent/st/ST0035/WP/15/info.md`, `WP/16/info.md`, `WP/17/info.md`.
- Finalise `intent/st/ST0036/impl.md` (close the "open items" section).
- Verify CHANGELOG v2.10.0 entry covers both ST0035 + ST0036.
- Commit: `docs: ST0036/WP-09 coordinate directory move into ST0035 fleet rollout`.

Then ST0035 resumes at WP14 (Intent self-dogfood for the canon LLM config).

## Risks for tomorrow

- **Treeindex stale entries**: `intent treeindex intent --prune` may surface orphans from the pre-relocation layout. Expected; just prune.
- **AGENTS.md regen drift**: `intent claude upgrade --apply` calls `intent agents sync`. The regen produces a date-stamp diff. Per session-3 conventions, include the regen in the WP08 commit (it's the right state post-relocation).
- **Canon-apply `INSTALL_TREEINDEXIGNORE` will not fire** because Intent already has `intent/.treeindex/.treeindexignore` (we updated it in WP06 with the granular pattern). Expected `PRESENT (project-owned)`. Good.

## Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations -- but until WP08 lands, manual Edit on WP info.md is the documented workaround for `intent wp done` rejecting Intent's own repo.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims.
- Document first, code next.

## Lessons from this session

- **A diagnostic mv catches what audits miss.** The user proposed renaming Intent's own `.intent` -> `intent/.config` _before_ doing per-test BATS flips. Result: +1 passing, 0 newly broken. That single experiment proved WP02's path probes were complete and confirmed that the only remaining work was BATS fixture/assertion flips. Cheap diagnostic, sharp signal.
- **macOS BSD mktemp footgun**: `mktemp /tmp/foo-XXXXXX.md` creates the LITERAL file `foo-XXXXXX.md` -- the X's are not substituted when followed by a suffix. Drop the suffix or use `mktemp -t prefix` (which on macOS lands in `$TMPDIR`, not `/tmp`).
- **Highlander cleanup pays for itself**: `bin/intent_treeindex::ensure_treeindexignore` was an inline heredoc duplicating template content. Extracting to `lib/templates/_treeindexignore` is one source for both the auto-init path and the canon installer's INSTALL_TREEINDEXIGNORE action.
- **WP05 took longer than its M sizing**: the per-test flips were mechanical but the suite needed to be green at commit time, which created an interlock with WP08 (test 374 needed Intent in v2.10.0 layout). Keeping the manual mv uncommitted in working tree resolved the interlock without conflating WP05 + WP08 commits.

## Open follow-ups (outside ST0035 + ST0036)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 / ST0036 scope.
