# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials`, `/in-standards`, plus per-language skills. Releases the `UserPromptSubmit` strict gate via the per-project sentinel.
2. **Verify the working tree.** `git status` should show only user-local `.claude/settings.local.json`. ST0036 is closed; Intent is at `intent/.config/` layout post-WP08.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/14/info.md`** -- next WP to execute (Intent self-dogfood for canon LLM config).

## State (2026-04-27, end of session)

**Intent v2.10.0 in progress. ST0035 13 of 19 Done (WP14-WP18 + new WP19 remaining). ST0036 9 of 9 Done (closed; moved to COMPLETED).**

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/` (post-WP08 relocation; rename history preserved via `git log --follow`).
- **Tests**: 781/781 green (774 baseline + 4 needs_v2_10_0_upgrade scenarios + 3 canon-installer scenarios).
- **Doctor**: clean.
- **Backup tag**: `wp08-pre-relocate` at `69069eca`. Discardable; delete with `git tag -d wp08-pre-relocate` after a stable session.

## What landed this session (newest first)

- `1497885` -- WP-09 cross-thread coordination (12-point checklist; v2.9.1 -> v2.10.0 flips; impl.md finalised).
- `5c782b3` -- WP-08 Intent self-apply v2.10.0 directory move.
- `a7c27c3` -- WP-19 Phase 0 spec (per-language canon command + `intent init --lang` flag).
- `ebd6620` -- WP-11 canon-installer fix (PROJECT_NAME from config.json + always-`_default` templates).
- `01159ff` -- WP-01 dispatcher fix (layout-aware early-exit + `needs_v2_10_0_upgrade` shortcut + `2.10.0` case).

## Resume target -- ST0035/WP14 (Intent self-dogfood)

WP08 already executed Phase 3 (canon-apply) on Intent during the relocation. WP14 becomes a verification sweep:

1. Re-run `intent claude upgrade` (no `--apply`); should report all canon artefacts UP TO DATE / PRESENT.
2. Run `intent doctor` -- expect clean.
3. Run `tests/run_tests.sh` -- expect 781/781 green.
4. Verify session hooks fire: SessionStart prints context; UserPromptSubmit gate fires (then released by `/in-session`); Stop reminds `/in-finish`.
5. Verify `.git/hooks/pre-commit.intent` is callable (chain not yet wired into `pre-commit` itself; per WP-11 chain block snippet).
6. Document outcomes in `intent/st/ST0035/WP/14/info.md`; mark Done.
7. Commit: `chore: ST0035/WP-14 Intent self-dogfood verification`.

After WP14: ST0035 resumes at WP15 (Conflab/Lamplight/Laksa canary; carries both ST0035 + ST0036 concerns per WP-09 coordination).

## Risks for next session

- **Backup tag cleanup**: `git tag -d wp08-pre-relocate` once next session confirms stability.
- **Pre-commit chain not active**: canon-installer left `.git/hooks/pre-commit.intent` but did NOT modify the existing `pre-commit` to call it. To activate: paste the `Intent critic gate` chain block from canon-installer output into `.git/hooks/pre-commit`.
- **WP19 implementation**: per-language canon command is now spec'd (Phase 0 elaborated) but NOT implemented. Sized M (2-3 sessions). Lands when ST0035 fleet rollout (WP15-17) is far enough along to inform the per-language template stubs.

## Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims; auto-detection rejected (use explicit user choice instead).
- Document first, code next.

## Lessons from this session

- **WP08's "moment of truth" worked.** The manual-mv WP05 diagnostic was a useful smoke-test, but it bypassed the dispatcher and the canon-installer entirely. WP08's proper run surfaced two real bugs (dispatcher half-fix, canon-installer wrong-shape) plus spawned a needed feature (WP19). The fix-forward rhythm (file the gap, fix in place, ship cleaner) was natural.
- **Auto-detection of project language is a dead end.** Real projects are polyglot (Elixir + Swift + Rust + Lua + Bash + HTML/CSS/JS). Picking a single "primary" misrepresents the project shape. Replaced with explicit user choice via `intent lang init <lang>` + `intent init --lang ...` (WP19 spec).
- **Layout-keyed idempotence beats stamp-keyed.** ST0035 retargeted v2.9.1 -> v2.10.0 mid-development, which left Intent stamped 2.10.0 but at the .intent/ layout -- a state that any stamp-only check would miss. The dispatcher fix (now layout-aware in three coordinated places) prevents the same trap for any project that gets stamped before being relocated.
- **PROJECT_DIR resolution matters.** `basename "."` returns `.`, which became `# .` as a regenerated CLAUDE.md title. Resolve to absolute path before basename, and prefer `project_name` from `intent/.config/config.json` (canonical, user-set).
- **Linter cooperates.** Markdown linter auto-aligns table columns on save; commits include both content edits and linter touch-ups. No friction.

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures. Not in ST0035 scope.
- `git tag -d wp08-pre-relocate` once stable.
