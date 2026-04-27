# Claude Code Session Restart -- narrative state

## Current state (2026-04-27, end of session -- ST0035 14 of 19; WP-15 canary 9 of 11 in-scope)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) 14 of 19 Done. WP-15 (canary rollout) WIP at 9 of 11 in-scope.** This session ran a batch: Molt, Utilz, arca_cli, arca_config, arca_notionex, Prolix, MicroGPTEx -- all clean once stale `.intent/config.json` bumps were reset. Foreign pre-commit chained via marker block on every project. Reports under `intent/st/ST0035/WP/15/canary-reports/`. **Conflab + Lamplight still deferred (busy); Pplr out of scope.** Tests **788/788 green**, doctor clean.

### ST0035 shape

- **Done (14)**: WP01-WP14.
- **WIP (1)**: WP15 (Laksa done; 15 in-scope projects remaining).
- **Not Started (4)**: WP16, WP17, WP18, WP19.

WP14 closed via `intent wp done ST0035/14` after the verification sweep + hardening pass. Five canon-installer fixes and two doc populations followed (Intent's RULES.md + ARCHITECTURE.md). Laksa was the first canary; the auto-insert chain block worked end-to-end (the Laksa commit itself triggered the chain → critic → ok).

Critical path remaining: `WP15 (canaries continued) -> WP16 (fleet) -> WP17 (verification + dogfood journal)`. WP18 (`intent/usr/*.md` audit) parallel; must land before WP17. WP19 (per-language canon) independent.

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

- **Auto-insert beats manual paste.** The previous "print snippet, ask user to paste" flow left a known-deferred state on every project (WP-08 had a "deferred: chain block not active" line). Replacing the snippet print with a markered idempotent insert removed the deferred bucket entirely. Marker pair detection makes re-runs a guaranteed no-op.
- **REVIEW warnings should be conditional, not unconditional.** Firing the same warning on every project that has any RULES.md was noise. Comparing against `TEMPLATES_SOURCE/_default/RULES.md` via `cmp -s` makes the warning meaningful: it only fires when the user really hasn't customised yet.
- **Linter-vs-generator oscillation is a real bug.** The `---` footer in AGENTS.md needed a trailing blank line to match prettier's output. Without it, every regen would silently flip the file back-and-forth. The MD5 check from WP-14 surfaced this on the second re-apply.
- **Pre-flight on canary projects matters.** Laksa had a stale manual config bump; resetting to HEAD let the migration write the canonical version end-to-end. Canary discipline: clean tree -> clean migration.
- **WP-15 spec drift**: `intent upgrade --dry-run` doesn't exist; Sites subdir was assumed; "12 + Pplr" is now "ecosystem minus Pplr". Worth tidying before more canaries.

### Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft. Laksa is the first real-world dogfood datapoint.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.

### Resume target -- Conflab + Lamplight (last two canaries)

9 of 11 in-scope projects done this session. The remaining two (Conflab, Lamplight) are deferred until they're free. The recipe is mature; both should follow the Molt/Prolix pattern (foreign pre-commit chained via marker block, no surprises). After both land, run `intent wp done ST0035/15` and proceed to WP-16/17/18.

Recipe (proven across 9 canaries):

1. `cd ~/Devel/prj/<project>` and check `git status` -- ensure clean tree (reset stale state if needed).
2. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent claude upgrade )` for the canon-installer dry-run.
3. `( cd ~/Devel/prj/<project> && /Users/matts/Devel/prj/Intent/bin/intent upgrade )` to do the migration chain (Phase 1 relocate, Phase 2 stamp, Phase 3 canon-apply).
4. Run the 12-point verification (script in `intent/st/ST0035/WP/15/canary-reports/laksa.md` for the exact commands).
5. Add `/AGENTS.md.bak` to the project's `.gitignore` if missing.
6. `git add -A && git commit` with the canary message.
7. `git push local main` (NOT upstream).
8. Write `intent/st/ST0035/WP/15/canary-reports/<project>.md` (use Laksa's report as a template).
9. Commit the report in Intent.

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
