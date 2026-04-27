# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Now folded with orientation: reads restart files + project rules + `intent st list` first, then loads `/in-essentials`, `/in-standards`, plus per-language skills, then releases the `UserPromptSubmit` strict gate via the per-project sentinel. (Standalone `/in-start` still exists for orientation-only.)
2. **Verify the working tree.** `git status` should be clean. ST0035 15 of 19 Done; WP-15 closed (11 of 11 in-scope canaries pass).
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read `intent/st/ST0035/WP/15/canary-summary.md`** for the WP-15 outcome (decision: proceed to WP-16).
5. **Read `intent/st/ST0035/WP/16/info.md`** -- next WP. Note significant as-built drift: 8 of WP-16's original 13 projects were absorbed into WP-15 canary; 4-5 (Multiplyer, MeetZaya, Molt-matts, Courses/Agentic Coding, A3/a3-content) were user-manually upgraded between sessions; Pplr now out of scope. WP-16 needs the same as-built tidy + fleet summary that WP-15 just got.

## State (2026-04-27, end of session -- WP-15 closed)

**Intent v2.10.0. ST0035 15 of 19 Done. WP-15 closed; 11 of 11 in-scope canaries pass; aggregate summary at `intent/st/ST0035/WP/15/canary-summary.md`. Tests 791/791; doctor clean.**

This session post-compact: Conflab + Lamplight canary reports committed; WP-15 spec tidied + aggregate summary written + `intent wp done ST0035/15` clean. Plus `~/.claude` global repo cleanup (folded `/in-start` into `/in-session`; expanded `.gitignore`; checked in months of curated agents + skills + config; pushed to `matthewsinclair/cfg-claude` on GitHub).

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 791/791 green.
- **Doctor**: clean.
- **Pre-commit canonical layout**: canon body at `pre-commit.intent`; chain stub at `pre-commit`. Fresh installs, legacy single-file projects, and projects with foreign pre-commit hooks all converge on the chained architecture.

## What landed this session (newest first)

- `300334d` -- WP-15 spec tidy + canary aggregate summary + status flip to Done.
- `e5134ee` -- Conflab + Lamplight canary reports committed.
- 5 commits in `~/.claude` (private repo `matthewsinclair/cfg-claude` on GitHub; pushed):
  - `d0b9129` -- config: refresh CLAUDE.md, settings.json hooks; add restart.md + plugins/blocklist.json.
  - `89a2390` -- skills: add the in-\* skill suite (22 skills, 31 files, 7263 insertions).
  - `76ee882` -- agents: introduce per-language critics (5) + diogenes; retire monolithic elixir agent.
  - `dfb1d8a` -- chore: expand .gitignore (projects/, runtime caches, ide/, backups/).
  - `1b70f3e` -- in-session: fold orientation step in (was /in-start standalone).

## Resume target -- WP-16 reconciliation (S/M)

WP-16 ("Fleet rollout to remaining 13 projects") needs the same as-built tidy WP-15 just got. 8 of its 13 projects were absorbed into WP-15 canary; 4-5 were user-manually upgraded between sessions; Pplr is now out of scope per user.

Task:

1. Tidy `intent/st/ST0035/WP/16/info.md` to reflect as-built (drop the 8 absorbed projects; document the 5 user-manual ones; drop Pplr).
2. Write `intent/st/ST0035/WP/16/fleet-summary.md` with CLI verification of the 5 user-manual projects (`intent_version` + `intent/.config/` + chain block markers + .gitignore canonical).
3. `intent wp done ST0035/16`.
4. Caveats to flag in summary: Multiplyer has 1 known test failure (File.ls!/1 in catalog/sources/filesystem.ex:111 against stale fixture path); MeetZaya does not compile. Both user-out-of-scope.

After WP-16 closes, proceed to WP-17 (verification + dogfood journal) and WP-18 (`intent/usr/*.md` audit) in parallel.

## Risks for next session

- **WP-16 spec drift mirrors WP-15**: info.md still references `intent upgrade --dry-run` and `intent upgrade --apply` (neither exists; only `intent upgrade`). Same tidy approach as WP-15.
- **5 user-manual projects need verification**: Multiplyer / MeetZaya / Molt-matts / Courses/Agentic Coding / A3/a3-content all on v2.10.0 per user but no formal canary report exists. Quick CLI verification (single bash command per project) is enough.
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

- **Fold commands when they almost always go together.** `/in-start` and `/in-session` were separate skills (orientation vs. code-readiness). SessionStart hook only nudged `/in-session`, so `/in-start` got forgotten. Folded the orientation step into `/in-session`; standalone `/in-start` retained for the rare orientation-only case. Pattern applies broadly: if two skills are usually called together, fold the common case and keep the rare-case one separate.
- **WP closure pattern: tidy spec to as-built + write aggregate summary + `wp done`.** When a WP's original plan diverges from execution (scope expansion, stale references, decisions overtaken), the cleanest closure preserves the original-vs-built delta as a "scope as built" note and ships a summary. Don't re-litigate the plan; document what actually happened.
- **Personal repos need .gitignore discipline early.** `~/.claude` had 1.6GB of session transcripts as untracked because no `.gitignore` covered `projects/`. Runtime dirs accumulate fast. Whenever curating a config repo, gitignore runtime droppings before they swamp the working tree.

## Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
