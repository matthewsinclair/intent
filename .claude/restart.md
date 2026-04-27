# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Now folded with orientation: reads restart files + project rules + `intent st list` first, then loads `/in-essentials`, `/in-standards`, plus per-language skills, then releases the `UserPromptSubmit` strict gate via the per-project sentinel. (Standalone `/in-start` still exists for orientation-only.)
2. **Verify the working tree.** `git status` should be clean if release engineering completed; otherwise check `intent/wip.md` for the in-flight commit.
3. **Read `intent/wip.md` and `intent/restart.md`** for narrative state.
4. **Read CHANGELOG.md v2.10.0 entry** for the shipped surface.

## State (2026-04-27, end of session -- ST0035 + ST0036 Completed; v2.10.0 ready to publish)

**Intent v2.10.0 is feature-complete. ST0035 19 of 19 Done; ST0036 9 of 9 Done; both moved to `intent/st/COMPLETED/`. CHANGELOG v2.10.0 entry finalised. Tests 810/810 green. `intent doctor` clean.**

This session post-second-compact: WP-18 retire intent/usr/\*.md (3 files canon-stale, replaced README + blog + migration cross-refs). WP-17 spec tidy + 14-row verification matrix + dogfood journal + decision on user-manual upgrade gotcha. WP-19 per-language canon (`intent lang` + `intent init --lang` + per-language stub templates + intent_init lays down agnostic \_default RULES.md). ST0035 marked complete via `intent st done ST0035`. CHANGELOG v2.10.0 flipped from "in progress" to release date.

Pending user-visible release acts (deferred until user confirms):

1. `git tag v2.10.0` (local; latest existing tag is `v2.9.0`).
2. Push tag + main to `local` (Dropbox; safe).
3. Push tag + main to `upstream` (GitHub; confirm first).
4. `gh release create v2.10.0 --notes-file <CHANGELOG section>` (confirm first).

After publish lands: update user's MEMORY.md Active Work section + final session wrap commit.

- **VERSION**: `2.10.0`.
- **Layout**: `intent/.config/`.
- **Tests**: 810/810 green.
- **Doctor**: clean.
- **Tag**: `v2.10.0` not yet created (waiting on user confirmation for the publish flow).

## What landed this session (newest first)

- (uncommitted in working tree) -- CHANGELOG v2.10.0 finalisation + ST0035 close + this/restart/wip update; one final local commit wraps these before tag/push.
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

## Resume target -- v2.10.0 release publish

If user confirms the publish flow:

```bash
# All from /Users/matts/Devel/prj/Intent
git tag v2.10.0
git push local main && git push local v2.10.0          # Dropbox; safe
git push upstream main && git push upstream v2.10.0    # GitHub; user must confirm
gh release create v2.10.0 --title "Intent v2.10.0" --notes-file <(awk '/^## \[2.10.0\]/,/^## \[2.9.0\]/{ if(!/^## \[2.9.0\]/) print }' CHANGELOG.md)
```

After publish: update MEMORY.md (drop ST0035 from Active Work; add v2.10.0 shipped marker) + final session wrap commit.

If user defers the publish: status quo is "v2.10.0 ready; tag/push deferred". Next session can pick this up directly.

## Risks for next session

- **Tag conflict** if v2.10.0 tag already exists locally or on remote. Mitigation: `git tag -l v2.10.0` before tagging; `gh release view v2.10.0` to check upstream state.
- **Push to upstream may surface CI issues** that didn't appear locally. Mitigation: monitor `gh run list` after push; address any red runs.
- **GitHub release notes formatting**: the awk extraction above stops at the next "## [" marker; verify the extracted slice renders correctly before the gh release create.

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
