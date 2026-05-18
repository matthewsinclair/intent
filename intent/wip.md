---
verblock: "18 May 2026:v0.72: matts - v2.11.7 cut (ST0040 whiteboard protocol)"
intent_version: 2.11.7
---

# Work In Progress

## Current State

**v2.11.7 shipped 2026-05-18.** Additive patch shipping the multi-session coordination protocol designed in a parallel Lamplight session and live-tested in `/Users/matts/Devel/prj/Lamplight/intent/whiteboard/`. ST0040 captures the design, alternatives, and deliberate deferrals; this release rolls it into formal canon. The protocol is opt-in by directory presence — projects without `intent/whiteboard/` see zero behaviour change.

Integration: new `/in-whiteboard` skill at `intent/plugins/claude/skills/in-whiteboard/` with subcommands `pickup` / `claim` / `unclaim` / `touch` / `ask` / `decide` / `lamplight` / `release` / `status`. `/in-session` step 5 auto-fires `pickup`; `/in-finish` step 1 auto-fires `release`; both opt-in by directory presence. `bin/intent_upgrade` now auto-installs `in-whiteboard` and re-syncs the canon skill mirror after the migration dispatcher completes (idempotent; failure-tolerant; no `--force`, so user customisations are never silently lost). Regression test in `tests/unit/intent_upgrade_dispatcher.bats` asserts the install lands on a v2.10.x → current-target upgrade. New "Multi-session coordination" section in `intent/docs/working-with-llms.md`. Optional `Re:` and `FYI only` header conventions on the `asks.md` `ask` subcommand, borrowed from the cross-project LLMsend protocol (in-whiteboard is the intra-project sibling). Three commits on `main`: `ce38a10` skill landing, `b85dc10` integration, `f09bb65` release. Pushed to both remotes; release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.7>.

Decision worth carrying forward: shipped as **patch** at user direction. The new-skill default is "minor", but the protocol is opt-in by directory presence with zero behaviour change for non-adopting projects — the patch framing is defensible on those grounds. Re-confirm at the time of the next skill addition; do not assume from this case.

Fleet pickup: `intent upgrade` in any v2.11.x project auto-installs `in-whiteboard` and re-syncs `in-session` / `in-finish`. Already-running sessions in those projects see the new chain only on `/compact` or session restart (the loaded skill prose is from session-start, not re-read per turn). Manual `/in-whiteboard pickup` works in the current session as a workaround.

**v2.11.6 shipped 2026-05-15.** Additive patch shipping one new Lua coding rule — **IN-LU-CODE-006 — Dispatch table over if-chain for value dispatch** — surfaced during Lamplight ST0163 WP-04 (Murder mechanic hook authoring). The rule formalises the table-of-functions idiom Lua uses in place of pattern-matched function heads / multi-head dispatch. Concretises IN-AG-PFIC-001; sister rule IN-EX-CODE-001 (Elixir multi-head dispatch). Enforcement via the `critic-lua` subagent (prose Detection, no Greppable proxy — matches existing Lua-pack convention). Fleet pickup automatic (rules load from `$INTENT_HOME`).

## Next Up

No active steel thread. Optional follow-on, in order of return:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Its regex sweep rewrites the historical migration date in any CLAUDE.md it touches. Worked around in v2.11.5 by reverting CLAUDE.md after `intent upgrade`. Permanent fix: scope the substitution to current-state lines only, leave historical lines verbatim.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template (no consumer in `bin/` or `intent/plugins/`). Not STP-tainted, still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** — still parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project before any remediation.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab's own backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish (`--rollback`, log-to-file mirror); `$N`-in-SKILL.md trap audit on remaining `in-*` skills; `docs/blog/_drafts/####-shell-critic-inception.md` blog draft (v2.11.7 is the seventh dogfood datapoint).
6. **ST0040 deferred items** (per ST0040 design.md, deliberately out-of-scope for v0): `intent st new` ST-ID allocation race (flock + atomic stub write); `intent whiteboard init` CLI subcommand; `PreToolUse` hook for claim-scope enforcement; `intent/.config/whiteboard.json` per-project config. Revisit each only if the v0 advisory model proves brittle in field use.

## Recent

- **2026-05-18**: v2.11.7 cut. Multi-session coordination protocol (ST0040): new `/in-whiteboard` skill, chain integration into `/in-session` and `/in-finish`, auto-install in `intent upgrade`, new docs section, `Re:` / `FYI only` conventions cross-pollinated from the LLMsend protocol. ST0040 marked Completed.
- **2026-05-15**: v2.11.6 cut. Single new Lua coding rule (IN-LU-CODE-006 dispatch-table-over-if-chain) surfaced during Lamplight ST0163 WP-04. Shipped as patch at user direction.
- **2026-05-05**: v2.11.5 cut. Gate bypass for non-interactive `claude -p` automation; `intent agents generate` self-load fix; migration stamp uses live target. Fleet upgrade across 10 projects (8 directly + Intent self-stamp + the user's own Conflab/Lamplight). Canon STP cleanup: 3 orphan templates deleted, 1 live template fixed, 4 fleet projects' CLAUDE.md de-STP-ified.
- **2026-04-30**: v2.11.4 cut. Docs patch capturing v2.11.3 field verification + critic-runner code-locality clarification.
- **2026-04-29**: ST0039 / v2.11.3 cut. Strict-proxy contract in `critic_runner.sh`.
- **2026-04-28**: ST0037 / v2.11.0 cut (`languages` config field), then v2.11.1 length-guard hotfix and v2.11.2 `intent upgrade` dispatcher hotfix.

## Parked

_(None.)_
