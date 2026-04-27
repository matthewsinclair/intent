# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Now folded with orientation: reads restart files + project rules + `intent st list` first, then loads `/in-essentials`, `/in-standards`, plus per-language skills, then releases the `UserPromptSubmit` strict gate via the per-project sentinel. (Standalone `/in-start` still exists for orientation-only.)
2. **Verify the working tree.** `git status` should be clean. ST0035 16 of 19 Done; WP-15 + WP-16 both closed.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/15/canary-summary.md` and `WP/16/fleet-summary.md`** for the rollout outcomes (decision: proceed to WP-17 + WP-18 in parallel).
5. **Read `intent/st/ST0035/WP/17/info.md` and `WP/18/info.md`** -- next WPs.

## State (2026-04-27, end of session -- WP-15 + WP-16 closed)

**Intent v2.10.0. ST0035 16 of 19 Done. WP-15 + WP-16 both closed; all in-scope fleet projects on canon (8 via canary + 5 user-manual + 1 OOS). Tests 791/791; doctor clean.**

This session post-compact: Conflab + Lamplight canary reports committed; WP-15 spec + aggregate summary + close; WP-16 spec + fleet summary + close; 3 leftover `.intent/` directories cleaned up in fleet (Multiplyer, MeetZaya, Courses/Agentic Coding); plus `~/.claude` global repo cleanup (folded `/in-start` into `/in-session`; expanded `.gitignore`; checked in months of curated agents + skills + config; pushed to `matthewsinclair/cfg-claude` on GitHub).

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 791/791 green.
- **Doctor**: clean.
- **Pre-commit canonical layout**: canon body at `pre-commit.intent`; chain stub at `pre-commit`. Fresh installs, legacy single-file projects, and projects with foreign pre-commit hooks all converge on the chained architecture.

## What landed this session (newest first)

- `216edc5` -- WP-16 spec tidy + fleet-summary + status flip to Done.
- `e73a84c6` (Multiplyer), `d2c8a2d7` (MeetZaya), `8c8431a` (Courses/Agentic Coding) -- `git rm -rf .intent/` cleanup of leftover legacy directories tracked at HEAD with stale config; pushed to `local`.
- `34bed8e` -- session wrap (between WP-15 and WP-16 closure).
- `300334d` -- WP-15 spec tidy + canary aggregate summary + status flip to Done.
- `e5134ee` -- Conflab + Lamplight canary reports committed.
- 5 commits in `~/.claude` (pushed to `matthewsinclair/cfg-claude` on GitHub):
  - `d0b9129` -- config: refresh CLAUDE.md, settings.json hooks; add restart.md + plugins/blocklist.json.
  - `89a2390` -- skills: add the in-\* skill suite.
  - `76ee882` -- agents: introduce per-language critics + diogenes; retire monolithic elixir agent.
  - `dfb1d8a` -- chore: expand .gitignore (projects/, runtime caches, ide/, backups/).
  - `1b70f3e` -- in-session: fold orientation step in (was /in-start standalone).

## Resume target -- WP-17 (verification + dogfood journal) || WP-18 (`intent/usr/*.md` audit)

Both can run in parallel (per WP-17 spec, WP-18 must close before WP-17 closes).

**WP-17 (S)**: per-project dogfood journal -- 13 in-scope projects \* 12-point matrix. Capture observations from canary + user-manual rollouts. Worth flagging in journal:

- `.claude/` overly-broad-gitignored in three projects pre-NORMALIZE_GITIGNORE (Utilz, arca_notionex, MicroGPTEx); now uniform fleet-wide.
- CLAUDE.md user sections preserved across all canaries (correct behaviour); per-project drift between user content and canon date stamps.
- **User-manual upgrade gotcha**: leftover `.intent/` not auto-cleaned. Worth deciding in WP-17 whether `intent upgrade` should warn / auto-stage the deletion.

**WP-18 (M)**: review/update or retire `intent/usr/*.md`. Independent of WP-17 mechanically.

**WP-19** (independent, M, ~2-3 sessions): per-language canon (`intent lang init` + `intent init --lang`). Phase 0 elaborated; closes ST0035.

## Risks for next session

- **WP-17 spec drift likely**: info.md may have stale references similar to WP-15/WP-16. Apply the same closure pattern if needed (tidy spec to as-built + write summary + `wp done`).
- **WP-18 scope ambiguity**: "review and update (or retire) `intent/usr/*.md`" needs a decision per file. Walk the directory first; expect some files to be retired wholesale.
- **CLAUDE.md drift in older projects**: pre-existing user CLAUDE.md (STP-era text) is preserved by the canon. Refresh decision per project belongs in WP-17 journal.
- **`intent upgrade` cleanup gap**: WP-17 should decide whether `intent upgrade` warns/auto-stages the legacy `.intent/` deletion. Currently it leaves cleanup to user discipline; this session caught 3 projects post-fact.

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

- **Fold commands when they almost always go together.** `/in-start` and `/in-session` were separate skills (orientation vs. code-readiness). SessionStart hook only nudged `/in-session`, so `/in-start` got forgotten. Folded the orientation step into `/in-session`; standalone `/in-start` retained for the rare orientation-only case. Pattern applies broadly: if two skills are usually called together, fold the common case and keep the rare-case one separate.
- **WP closure pattern: tidy spec to as-built + write aggregate summary + `wp done`.** When a WP's original plan diverges from execution (scope expansion, stale references, decisions overtaken), the cleanest closure preserves the original-vs-built delta as a "scope as built" note and ships a summary. Don't re-litigate the plan; document what actually happened.
- **Personal repos need .gitignore discipline early.** `~/.claude` had 1.6GB of session transcripts as untracked because no `.gitignore` covered `projects/`. Runtime dirs accumulate fast. Whenever curating a config repo, gitignore runtime droppings before they swamp the working tree.

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
