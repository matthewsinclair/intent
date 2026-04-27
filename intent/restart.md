# Claude Code Session Restart -- narrative state

## Current state (2026-04-27, end of session -- ST0035 16 of 19; WP-15 + WP-16 closed)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) 16 of 19 Done. WP-15 + WP-16 both closed this session post-compact.** All in-scope fleet projects on canon: 8 absorbed into WP-15 canary, 5 user-manually upgraded + verified this session, Pplr out of scope. Three `.intent/` cleanup commits in the fleet (Multiplyer, MeetZaya, Courses/Agentic Coding) -- legacy directories were tracked at HEAD with stale config; user-manual `intent upgrade` runs created the new layout but didn't auto-clean the legacy. Plus housekeeping in `~/.claude` global repo (folded `/in-start` into `/in-session`, expanded `.gitignore`, checked in months of curated agents + skills + config; pushed to `matthewsinclair/cfg-claude`). Tests **791/791 green**; doctor clean.

### ST0035 shape

- **Done (16)**: WP01-WP16.
- **Not Started (3)**: WP17, WP18, WP19.

WP-15 closed via `intent wp done ST0035/15` (`300334d`) with aggregate `canary-summary.md`. WP-16 closed via `intent wp done ST0035/16` (`216edc5`) with aggregate `fleet-summary.md` documenting the as-built disposition (8 canary + 5 user-manual + 1 Pplr OOS).

Critical path remaining: `WP-17 (verification + dogfood journal) || WP-18 (intent/usr/*.md audit)` in parallel; then `WP-19 (per-language canon)` independent.

### Progress this session (two commits + one in Anvil)

In commit order:

1. `d5b9203` -- **canon-installer: LEGACY single-file pre-commit migration**. Detects when `pre-commit` is the canon body verbatim and `pre-commit.intent` is absent (legacy install pattern from before chaining); auto-migrates by mv canon body -> `pre-commit.intent` + write fresh chain stub at `pre-commit`. `INSTALL_PRE_COMMIT` also updated so fresh installs produce the chained architecture from the start (was producing legacy state). +3 new BATS scenarios; fresh-install test asserts chained layout. 788/788 green (was 785).
2. `0724f88` -- **Anvil canary report** at `intent/st/ST0035/WP/15/canary-reports/anvil.md`.
3. `39c63bd` (in **Anvil**) -- **`Intent upgrade to 2.10.0`** (user-authored single commit covering canon application + flybys: lazy_html `:only` removal so `lucide_icons` resolves; `Anvil.Projects.create -> create_project` in 4 policy tests for Ash 3.24 compat). mix test 192/192. Pushed to `local`.

### Earlier this day (six commits + one in Laksa)

1. `9a6387b` -- **WP-14 Intent self-dogfood verification**. Dry-run + apply + 12-point verification + reports under `intent/st/ST0035/WP/14/`. Confirmed WP-08 already ran Phase 3 (canon-apply); WP-14 is a verification sweep + idempotence proof (MD5 sanity).

2. `9315bb6` -- **canon-installer rough edges fixed** (surfaced by WP-14):
   - `intent_agents:506-509` -- AGENTS.md generator footer emits a blank line after `---` (eliminates the prettier-vs-generator oscillation).
   - `intent_claude_upgrade:488` -- RULES.md count uses ERE (`grep -Ec '^(###|[0-9]+\.)'`); dropped the `|| echo 0` fallback (was producing `(0\n0 rules/sections)`).
   - `intent_claude_upgrade:486-505` -- REVIEW warnings for RULES/ARCHITECTURE only fire when content is verbatim the `_default` template (`cmp -s` check).
   - `intent_claude_upgrade:404-475` -- new helpers `canon_chain_block_present` + `canon_emit_chain_block` + `canon_insert_chain_block`. Marker pair (`intent-chain-block:start/end`) makes re-application idempotent.
   - `intent_claude_upgrade:642-665` -- chain detection rewritten: CHAINED only when chain block present; new `CHAIN_REQUIRED` state when `pre-commit.intent` is installed but block missing; new `CHAIN_PRE_COMMIT_BLOCK` action.
   - `intent_claude_upgrade:1078-1098` -- Phase 3 `CHAIN_PRE_COMMIT` and `CHAIN_PRE_COMMIT_BLOCK` actions auto-insert the block (no more manual-paste snippet).
   - +4 new BATS scenarios (785/785 total).

3. `d0d0dc6` -- **populate Intent's RULES.md + ARCHITECTURE.md**. No longer verbatim `_default` stubs. RULES.md cites the four agnostic principles + six Intent dev rules + bash 3.x constraints + markdown/testing/commit discipline. ARCHITECTURE.md describes the 12 core directories + 6 key patterns (thin coordinator, plugin callbacks, single template source, layout-keyed idempotence, three-phase canon-apply, steel thread lifecycle) + hook architecture + critic dispatch + migration history.

4. `f5d9df9` -- **housekeeping**. `.claude/settings.local.json` untracked (per-developer noise; paths and tool allowlists vary). `/AGENTS.md.bak` gitignored (regen safety net). Backup tag `wp08-pre-relocate` deleted.

5. `a729ec64` (in **Laksa**) -- **`chore: apply ST0035 + ST0036 canon (v2.10.0 rollout canary)`**. 2.8.2 -> 2.10.0 chain in one `intent upgrade`. Canon installed; chain block auto-inserted between `set -euo pipefail` and the existing prettier/mix-format body. CLAUDE.md preserved (user-authored). Pushed to `local` (Dropbox); `upstream` deferred per WP-15 protocol.

6. `2e90556` -- **Laksa canary report**. `intent/st/ST0035/WP/15/canary-reports/laksa.md`. 12-point verification all green; three benign observations documented.

### Lessons worth keeping (this session)

- **`/in-start` and `/in-session` should be one command.** Different lifecycle moments but almost every post-compact session needed both. SessionStart hook only nudged `/in-session`, so `/in-start` got forgotten. Folded the orientation steps into `/in-session` step 1; standalone `/in-start` retained for the rare orientation-only case. Pattern: when two commands "almost always go together", fold them and keep the rare-case one separate.
- **WP-15 spec tidy is a closure pattern.** When a WP's plan diverges from execution (scope expanded, references go stale), the cleanest closure is: tidy spec to reflect as-built, write aggregate summary, mark done. Don't try to re-litigate the original plan -- preserve as a "scope as built" note and move on.
- **Canon-installer issues caught early via canary discipline.** Three new actions (`MIGRATE_LEGACY_PRE_COMMIT`, `CHAIN_PRE_COMMIT` auto-insert, `NORMALIZE_GITIGNORE`) were all surfaced by canary projects, baked into Intent, then re-applied across remaining canaries. Cheap because each canary is small and isolated; expensive if surfaced after fleet rollout.
- **Personal repos accumulate noise without `.gitignore` discipline.** `~/.claude` had 1.6GB of session transcripts as untracked because no gitignore covered `projects/`. Always `.gitignore` runtime dirs early; check periodically that the repo's working-tree noise reflects actual user-intent additions, not Claude Code runtime droppings.

### Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

### Resume target -- WP-17 (verification sweep + dogfood journal) || WP-18 (`intent/usr/*.md` audit)

WP-15 + WP-16 closed. Next: WP-17 + WP-18 can run in parallel (per WP-17 spec, WP-18 must land before WP-17 closes).

**WP-17 (S)**: per-project dogfood journal -- 13 in-scope projects \* 12-point matrix. Capture observations from canary + user-manual rollouts. Worth flagging:

- `.claude/` was overly-broad-gitignored in three projects pre-NORMALIZE_GITIGNORE (Utilz, arca_notionex, MicroGPTEx). Now uniform fleet-wide.
- CLAUDE.md user sections preserved across all canaries (correct behaviour); note per-project drift between user content and canon date stamps.
- **User-manual upgrade gotcha**: leftover `.intent/` directory not auto-cleaned by user commit. Worth deciding in WP-17 whether `intent upgrade` should warn or auto-stage the deletion.

**WP-18 (M)**: review/update or retire `intent/usr/*.md`. Independent of WP-17 mechanically; coordination point is "WP-18 must close before WP-17 closes".

**WP-19** (independent): per-language canon (`intent lang init` + `intent init --lang`). Phase 0 elaborated; ~2-3 sessions.

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
