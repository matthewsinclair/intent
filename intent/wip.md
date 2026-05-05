---
verblock: "05 May 2026:v0.70: matts - v2.11.5 cut + fleet upgrade + STP cleanup"
intent_version: 2.11.5
---

# Work In Progress

## Current State

**v2.11.5 shipped 2026-05-05.** Behavioural patch fixing three latent bugs surfaced by a Conflab session:

1. `intent treeindex` reported "empty response from Claude" for every directory inside any v2.10.0+ Intent project. The spawned `claude -p` session inherited the project's `UserPromptSubmit` hooks; the strict gate fired, exited 2, and `claude -p`'s non-bare path swallowed the stderr and emitted exit 0 with empty stdout. Fix: `INTENT_SKIP_IN_SESSION_GATE=1` env-var bypass in `require-in-session.sh`; `bin/intent_treeindex` sets it on every `claude -p` call.
2. `intent agents generate` produced a stripped AGENTS.md (empty project name, no language scaffolding, no installed-skill enumeration, no conditional resource links) when invoked via the dispatcher. The dispatch path skipped `load_intent_config`, leaving `PROJECT_ROOT` empty and every per-project detection silently failing. Fix: `intent_agents_generate_content` self-loads project context. Latent since the dispatcher was first added 2025-08-20.
3. `migrate_v2_10_x_to_v2_11_0` hard-coded the resulting stamp to `"2.11.0"` instead of the live target. Fix: stamp `get_intent_version`. Field impact muted by `needs_v2_11_0_upgrade`'s short-circuit but the bug existed.

Also folded in: test-isolation fix to `docs_completeness.bats` (the `agents_sync_idempotent` case ran `intent agents sync` against the real Intent root and never restored AGENTS.md, leaving the tree dirty and blocking subsequent `scripts/release` pre-flights).

After the release, fleet upgrade across `~/Devel/prj/*`. Nine projects upgraded by this session — Anvil, Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz — each with one `chore: upgrade to Intent v2.11.5` commit. Conflab and Lamplight handled by user; Intent self-stamp committed.

STP cleanup pass: stripped the "(formerly STP)" parenthetical and "## Migration Notes" sections from CLAUDE.md on Anvil / Laksa / MeetZaya / Multiplyer; fixed `lib/templates/prj/_wip.md` to use `intent st new` instead of "STP commands"; deleted three orphan STP-era templates from canon (`lib/templates/usr/_reference_guide.md`, `_deployment_guide.md`, `lib/templates/prj/st/_steel_threads.md`). `grep -rln STP lib/templates/` now returns nothing.

## Next Up

No active steel thread. Optional follow-on, in order of return:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Its regex sweep rewrites the historical migration date in any CLAUDE.md it touches (`migrated from STP to Intent vX.Y.Z on YYYY-MM-DD` becomes `migrated to vCURRENT`). Worked around in this session by reverting CLAUDE.md after `intent upgrade` and editing the top-of-file version line manually. Permanent fix: scope the substitution to current-state lines only, leave historical lines verbatim.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template (no consumer in `bin/` or `intent/plugins/`). Not STP-tainted, so survived the orphan-deletion sweep, but it's still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** -- still parked from the post-v2.11.3 plan. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project before any remediation.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`, 7 hits) -- still parked; folds into Conflab's own backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish (`--rollback`, log-to-file mirror); `$N`-in-SKILL.md trap audit on remaining `in-*` skills; `docs/blog/_drafts/####-shell-critic-inception.md` blog draft (v2.11.5 is the fifth dogfood datapoint).

## Recent

- **2026-05-05**: v2.11.5 cut. Gate bypass for non-interactive `claude -p` automation; `intent agents generate` self-load fix; migration stamp uses live target. Fleet upgrade across 10 projects (8 directly + Intent self-stamp + the user's own Conflab/Lamplight). Canon STP cleanup: 3 orphan templates deleted, 1 live template fixed, 4 fleet projects' CLAUDE.md de-STP-ified.
- **2026-04-30**: v2.11.4 cut. Docs patch capturing v2.11.3 field verification + critic-runner code-locality clarification.
- **2026-04-29**: ST0039 / v2.11.3 cut. Strict-proxy contract in `critic_runner.sh`.
- **2026-04-28**: ST0037 / v2.11.0 cut (`languages` config field), then v2.11.1 length-guard hotfix and v2.11.2 `intent upgrade` dispatcher hotfix.

## Parked

_(None.)_
