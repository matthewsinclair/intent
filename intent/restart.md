# Claude Code Session Restart -- narrative state

## Current state (2026-04-27, end of session -- ST0035 + ST0036 Completed; v2.10.0 ready to publish)

**Intent v2.10.0 is feature-complete. ST0035 (Canonical LLM Config + Fleet Rollout) 19 of 19 Done. ST0036 (Directory Relocation) 9 of 9 Done. Both moved to `intent/st/COMPLETED/`.** CHANGELOG v2.10.0 entry finalised (release date 2026-04-27). Tests 810/810 green. `intent doctor` clean.

Pending user-visible release acts (deferred until user confirms):

1. `git tag v2.10.0` (local; latest existing tag is `v2.9.0`).
2. Push tag + main to `local` (Dropbox; safe).
3. Push tag + main to `upstream` (GitHub; confirm first).
4. `gh release create v2.10.0 --notes-file <CHANGELOG section>` (confirm first).

After publish lands: update user's MEMORY.md Active Work section + final session wrap commit.

### Progress this session (4 commits in Intent + 3 commits in fleet projects + 5 commits in ~/.claude)

In commit order (this session, both compact halves):

1. `300334d` -- ST0035/WP-15 spec tidy + canary aggregate summary + status flip.
2. `216edc5` -- ST0035/WP-16 spec tidy + fleet summary + cleanup leftover .intent/ in 3 fleet projects (Multiplyer, MeetZaya, Courses/Agentic Coding) + status flip.
3. `54c6ea9` -- session wrap (between WP-16 closure and second compact).
4. `329e9f3` -- ST0035/WP-18 retire intent/usr/\*.md (3 files canon-stale at v2.6.0; substantially duplicated by README + working-with-llms + intent help; cross-refs updated).
5. `92e1ab7` -- ST0035/WP-17 spec tidy + 14-row verification matrix (feedback-report.md) + dogfood journal + decision on user-manual upgrade gotcha (warn at doctor, do NOT auto-stage; filed as v2.10.x follow-up).
6. `6c1f41e` -- ST0035/WP-19 per-language canon: new `intent lang` command (list/show/init) + `intent init --lang` flag + per-language stub templates for rust/swift/lua/shell + intent_init lays down agnostic \_default RULES.md/ARCHITECTURE.md (so fresh init has Language Packs anchor). +19 BATS scenarios; tests 791 -> 810.

Plus ~/.claude global repo housekeeping (5 commits, pushed to matthewsinclair/cfg-claude on GitHub) -- see `.claude/restart.md` for detail.

### Lessons worth keeping (this session)

- **Closure pattern (tidy spec to as-built + write summary + `wp done`) applies cleanly across WPs.** Used for WP-15, WP-16, WP-17, WP-18 in this session. Reinforce in template guidance somewhere; the pattern is recurring across STs.
- **WP-17 spec drift was caught by closure pattern.** Spec said "10-point matrix" but ST0036 added 2 more checks; spec said "17 projects" but as-built was 14; spec bundled CHANGELOG/tag/release/ST-close into deliverables but those moved out post-WP-19. **Lesson**: when a verification matrix grows, sweep all WPs that reference it.
- **Canon-installer hardening was load-bearing for fleet rollout.** Three new actions (`MIGRATE_LEGACY_PRE_COMMIT`, `CHAIN_PRE_COMMIT` auto-insert, `NORMALIZE_GITIGNORE`) all surfaced by canary projects, baked into Intent, then re-applied across remaining canaries. Cheap because each canary is small + isolated; expensive if surfaced after fleet rollout.
- **Auto-language-detection rejected; explicit user choice via --lang prevails.** WP-19 implements the per-language canon as opt-in (`intent lang init <lang>` or `intent init --lang <list>`). Real projects are polyglot; picking any single "primary" misrepresents project shape. Marker-based Language Packs section in agnostic RULES.md is the single anchor; per-language RULES-<lang>.md and ARCHITECTURE-<lang>.md sit alongside.
- **`intent init` should produce a v2.10.0-complete baseline.** Pre-WP-19, `intent init` only laid down MODULES.md + DECISION_TREE.md; canon RULES.md only appeared via `intent claude upgrade --apply` later. Now `intent init` lays down agnostic \_default RULES.md + ARCHITECTURE.md so the Language Packs anchor exists from day 1.

### Open follow-ups (outside ST0035/ST0036)

- `intent doctor` warning for leftover `.intent/` post-migration -- decision recorded in WP-17 dogfood journal; v2.10.x follow-up.
- `intent claude upgrade --dry-run` UX polish (reword "expected during dry-run" cases).
- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

### Resume target

After v2.10.0 publish acts complete: no active ST. Next session can pick the next ST off the backlog or start exploratory work for v2.11.

### Session conventions (carry forward)

- T-shirt sizing only (XS/S/M/L/XL/XXL) -- never clock-time estimates.
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, restart.md.
- Fail-forward: no backwards-compat shims, no deprecation stubs.
- Auto-detection of language/etc. rejected; use explicit user choice.
- Document first, code next, with a hard review gate after Phase 0.
- Pre-flight: reset stale state on canary projects before applying.
