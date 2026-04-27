# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Now folded with orientation: reads restart files + project rules + `intent st list` first, then loads `/in-essentials`, `/in-standards`, plus per-language skills, then releases the `UserPromptSubmit` strict gate via the per-project sentinel. (Standalone `/in-start` still exists for orientation-only.)
2. **Verify the working tree.** `git status` should be clean if release engineering completed; otherwise check `intent/wip.md` for the in-flight commit.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read CHANGELOG.md v2.10.0 entry** for the shipped surface.

## State (2026-04-27, end of session -- v2.10.0 SHIPPED)

**Intent v2.10.0 shipped 2026-04-27.** Tag `v2.10.0` at `cf37292`; pushed to `local` (Dropbox) + `upstream` (GitHub); GitHub release live at https://github.com/matthewsinclair/intent/releases/tag/v2.10.0. ST0035 19 of 19 Done; ST0036 9 of 9 Done; both in `intent/st/COMPLETED/`. CHANGELOG v2.10.0 finalised. Tests 810/810 green. `intent doctor` clean. No active ST.

This session post-second-compact: WP-18 retire intent/usr/\*.md (3 files canon-stale, replaced README + blog + migration cross-refs). WP-17 spec tidy + 14-row verification matrix + dogfood journal + decision on user-manual upgrade gotcha. WP-19 per-language canon (`intent lang` + `intent init --lang` + per-language stub templates + intent_init lays down agnostic \_default RULES.md). ST0035 marked complete via `intent st done ST0035`. CHANGELOG v2.10.0 flipped from "in progress" to release date. Tag + push + GitHub release executed.

- **VERSION**: `2.10.0` shipped.
- **Layout**: `intent/.config/`.
- **Tests**: 810/810 green.
- **Doctor**: clean.
- **Tag**: `v2.10.0` at `cf37292`; pushed to both remotes.
- **GitHub release**: live at the URL above.

## What landed this session (newest first)

- (in flight) -- final session wrap commit: MEMORY.md + this restart + intent/wip.md + intent/restart.md flipped from "ready to publish" to "shipped".
- `cf37292` -- release-prep: ST0035 marked Done via `intent st done ST0035` (moved to intent/st/COMPLETED/); CHANGELOG v2.10.0 finalised. Tagged as `v2.10.0` and pushed to both remotes; GitHub release published.
- `6c1f41e` -- ST0035/WP-19 per-language canon + intent init --lang.
- `92e1ab7` -- ST0035/WP-17 spec tidy + verification matrix + dogfood journal.
- `329e9f3` -- ST0035/WP-18 retire intent/usr/\*.md.
- `54c6ea9` -- session wrap (between compacts).
- `216edc5` -- ST0035/WP-16 closure (fleet summary + 3 cleanup commits in fleet projects).
- `300334d` -- ST0035/WP-15 closure (canary aggregate summary).
- `e5134ee` -- ST0035/WP-15 Conflab + Lamplight canary reports.

Plus 5 commits in `~/.claude` (pushed to `matthewsinclair/cfg-claude` on GitHub):

- `d0b9129` -- config: refresh CLAUDE.md, settings.json hooks; add restart.md + plugins/blocklist.json.
- `89a2390` -- skills: add the in-\* skill suite.
- `76ee882` -- agents: introduce per-language critics + diogenes; retire monolithic elixir agent.
- `dfb1d8a` -- chore: expand .gitignore (projects/, runtime caches, ide/, backups/).
- `1b70f3e` -- in-session: fold orientation step in (was /in-start standalone).

## Resume target -- next ST or v2.10.x backlog

v2.10.0 shipped. No active ST. Next session can pick the next ST off the v2.10.x follow-up backlog or start exploratory work for v2.11.

v2.10.x backlog (from WP-17 dogfood journal):

- **`intent doctor` warning for leftover `.intent/` post-migration**. Decision recorded: warn at doctor; do NOT auto-stage. Implementation S; add to `bin/intent_doctor`.
- **`intent claude upgrade --dry-run` UX polish**: reword the "expected during dry-run" cases (e.g. `intent/.config/config.json` not found pre-relocation). Implementation XS.
- **CLAUDE.md per-project drift refresh decisions**: per-project judgement call; not a universal change.
- **Optional `intent upgrade --auto-cleanup` flag**: would do `git rm -rf .intent/` if and only if `.intent/` is tracked at HEAD AND `intent/.config/config.json` exists. Off by default.

## Risks for next session

- **CI status of `cf37292`**: the release-prep commit's CI run was in_progress at publish time. Check `gh run list` to confirm green. If red, address before next push.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md.
- Fail-forward: no backwards-compat shims; auto-detection rejected.
- Document first, code next.
- Pre-flight every canary: clean tree before applying.

## Lessons from this session (top three)

- **WP closure pattern (tidy spec to as-built + write summary + `wp done`) applies cleanly across WPs.** Used for WP-15, WP-16, WP-17, WP-18 in this session. Reinforce in template guidance.
- **Auto-language-detection rejected; explicit user choice via --lang prevails.** WP-19 implements per-language canon as opt-in. Real projects are polyglot; picking a "primary" misrepresents project shape.
- **`intent init` should produce a v2.10.0-complete baseline.** Pre-WP-19, only MODULES.md + DECISION_TREE.md were laid down at init time; canon RULES.md only appeared via `intent claude upgrade --apply`. Now intent init lays down the agnostic \_default RULES.md + ARCHITECTURE.md so the Language Packs anchor exists from day 1.

## Open follow-ups (post v2.10.0)

- `intent doctor` warning for leftover `.intent/` post-migration -- decision in WP-17 dogfood journal.
- `intent claude upgrade --dry-run` UX polish (reword "expected during dry-run" cases).
- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
