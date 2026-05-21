# Claude Code Session Restart -- narrative state

## Current state (2026-05-21, end of session -- v2.11.8 cut)

**v2.11.8 shipped 2026-05-21.** Patch fixing a multi-session deadlock in the `/in-session` UserPromptSubmit gate, surfaced by a Lamplight bug report (3 concurrent streams in one project). Both remotes pushed; release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.8>.

### The bug

With two or more Claude Code sessions open against the **same** Intent project, the gate blocked every prompt and `/in-session` never released it. Root cause: asymmetric source of truth for session identity.

- `require-in-session.sh` (gate) read the real `session_id` from its hook payload and checked `/tmp/intent/in-session-<session_id>.sentinel`.
- `release-gate.sh` (releaser, run by `/in-session`) had no payload, so it read the id from a shared per-project state file `/tmp/intent-claude-session-current-id-<key>` written by `session-context.sh` on `SessionStart`.
- Concurrent sessions all wrote that one file; it held some other session's id; the releaser touched the wrong sentinel; the gate's real-id sentinel never appeared; re-running `/in-session` re-read the poisoned file. Infinite loop.

### The fix

Both sides now resolve identity from the single env var `$CLAUDE_CODE_SESSION_ID`, which Claude Code exports into every hook and Bash tool invocation. Verified empirically this session: the env value matched the gate's payload id and the state file exactly. They agree by construction with no shared mutable file. When the env var is absent (older Claude Code) both degrade to the same `unknown` sentinel (always touched by the releaser), so they still agree and the gate self-heals — no combination leaves gate and release expecting different sentinels.

- `release-gate.sh`: rewritten to `sid="${CLAUDE_CODE_SESSION_ID:-}"`; touch that sentinel + always `unknown`; cksum/project_key/state-file/legacy reads deleted.
- `require-in-session.sh`: `session_id="${CLAUDE_CODE_SESSION_ID:-unknown}"`; payload now parsed only for `prompt` (slash-command passthrough).
- `session-context.sh`: `capture_session_id()` and the state-file write removed entirely; project/git/wip emission unchanged.
- Each script gained an adjacent rationale comment on its `set -u` line (why `-e`/`-o pipefail` are deliberately omitted) — clears IN-SH-CODE-003.
- SKILL.md step 4 and the `working-with-llms.md` troubleshooting block rewritten to the single-source model; concurrent sessions documented as supported.
- Tests: `release_gate_script.bats` rewritten (env-driven, plus a regression case proving a poisoned shared file no longer diverts the release); `require_in_session_gate.bats` switched to `CLAUDE_CODE_SESSION_ID` injection + a payload-decoy test; `session_context_script.bats` asserts no shared state file is written. All 927 pass; critic-shell clean.
- No data migration: sentinels and state files are ephemeral `/tmp`; stale `current-id-*` files are inert.

Decision: shipped as **patch** (v2.11.8) — shipped-as-broken defect, patch regardless of engineering scope.

### Propagation gap surfaced (added to backlog)

`intent claude skills sync` checksums `SKILL.md` only, so a script-only edit under a skill's `scripts/` dir does **not** trigger a re-copy — must force with `intent claude skills install <name> --force`. This release also touched `in-session`'s SKILL.md, so the fleet picks up the fixed `release-gate.sh` automatically on `intent upgrade`. Real gap for future script-only skill hotfixes; logged in wip.md backlog.

## Resume target -- next session

No active steel thread. One field smoke worth running:

1. **Concurrent-session acceptance test (Lamplight).** After `intent upgrade` on Lamplight, open 2+ Claude Code sessions in the project, run `/in-session` in each, then type a non-slash prompt in both. Neither should be gated. This is the direct acceptance test for the v2.11.8 bug. (Already-running sessions need `/compact` or restart to pick up the new hook scripts — the loaded hook is from session start.)

Optional follow-on, in order of return:

1. **Skill-sync script-change blind spot** (new this session). `intent claude skills sync` should key its checksum on the whole skill dir (or hash `scripts/`), not just `SKILL.md`. Cheap fix; prevents silent script drift on future hotfixes.
2. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates. Worked around manually in the v2.11.5 session; needs a real fix before the next minor.
3. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
4. **`/in-review` Elixir fleet sweep** — still parked.
5. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab backlog.
6. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.
7. **ST0040 deferred items** (intentional out-of-scope per ST0040 design.md): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` hook for claim-scope; `intent/.config/whiteboard.json` per-project config. Each revisited only if the v0 advisory model shows brittleness in field use.

## Lessons from this session (top three)

- **Asymmetric sources of truth deadlock; one source self-heals.** The gate read identity from the payload, the releaser from a shared file — two paths that could disagree, and under concurrency did. The fix wasn't "patch the releaser to read a better file"; it was collapsing both sides onto one source (`$CLAUDE_CODE_SESSION_ID`) that degrades symmetrically. When two cooperating processes must agree on a key, give them the same source, not two sources you hope stay in sync.

- **Verify the environment empirically before designing the fix.** The whole fix hinged on whether Bash tool invocations even see `$CLAUDE_CODE_SESSION_ID`. One `env | grep` confirmed it, and a second check proved it equalled the gate's payload id and the state file. That turned three candidate fix directions into one obvious choice. Cheap empirical checks beat reasoning about what the harness "probably" exposes.

- **Skill propagation has a script-only blind spot.** `intent claude skills sync` keys on `SKILL.md` checksum, so editing only a `scripts/` file silently fails to propagate. We got lucky here (SKILL.md also changed), but a pure script hotfix would have shipped to canon and never reached `~/.claude` or the fleet without `--force`. Worth fixing at the sync layer.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact / refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, restart.md (vanity metrics).
- Fail-forward: no backwards-compat shims; no deprecation stubs; migrations actively prune.
- Document first, code next, with a hard review gate after design.
- Pre-flight every canary: clean tree before applying.
- SKILL.md inline bash with `$N` positional fields gets mangled by the skill renderer. Use a script file invoked by path.
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes (including silent failures) are patches regardless of engineering scope. Additive features default to minor — opt-in-by-presence additions can be argued as patch when the behaviour change is zero for non-adopting projects.
- When a bats suite runs commands against the real project root, snapshot + restore affected files inside the test.
- For non-interactive Intent automation that spawns `claude -p`, always set `INTENT_SKIP_IN_SESSION_GATE=1` on the invocation.
- Never invoke `scripts/release` with `--no-confirm` from inside a tool-driven session — let its interactive confirmation be the human-in-the-loop checkpoint.
- A new skill arriving in canon needs: (1) `intent claude skills list` enumeration test entry, (2) auto-install hook in `bin/intent_upgrade` (if it should fleet-propagate), (3) regression test for the upgrade path, (4) docs section anchor in `working-with-llms.md`, (5) CHANGELOG section, (6) `chains_to` edits in any companion skills + `intent claude skills sync` propagation, (7) wip/restart bump.
- Cross-project sibling skills (eg LLMsend ↔ in-whiteboard) can cross-pollinate small conventions cheaply; wholesale dependency adoption only when the dependency footprint matches (file-only ↔ file-only is fine; file-only ↔ tmux/kitty is not).
