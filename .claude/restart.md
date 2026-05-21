# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate. As of v2.11.7, also chains to `/in-whiteboard pickup` if `intent/whiteboard/` exists.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show `release: v2.11.8` at the top.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-05-21, end of session -- v2.11.8 cut)

v2.11.8 shipped today. Patch fixing a multi-session deadlock in the `/in-session` UserPromptSubmit gate, surfaced by a Lamplight bug report. With two or more Claude Code sessions open against the same Intent project, the gate blocked every prompt and `/in-session` never released it.

Root cause: asymmetric source of truth for session identity. The gate (`require-in-session.sh`) read `session_id` from its hook payload; the releaser (`release-gate.sh`) had no payload and read the id from a shared per-project state file written by `session-context.sh` on `SessionStart`, which concurrent sessions stomped. The releaser touched the wrong sentinel and the gate's real-id sentinel never appeared.

Fix: both sides now resolve identity from the single env var `$CLAUDE_CODE_SESSION_ID` that Claude Code exports into every hook and Bash tool invocation. They agree by construction; when the env var is absent both degrade to the same `unknown` sentinel (always touched by the releaser), so they still agree and the gate self-heals. The shared state file was removed entirely — `session-context.sh` no longer persists the id, `release-gate.sh` dropped its state-file and legacy reads. Verified empirically that `$CLAUDE_CODE_SESSION_ID` equals the gate's payload id. All 927 tests pass; critic-shell clean.

Files: `intent/plugins/claude/skills/in-session/scripts/release-gate.sh`, `lib/templates/.claude/scripts/require-in-session.sh`, `lib/templates/.claude/scripts/session-context.sh`, SKILL.md step 4, `working-with-llms.md` troubleshooting block; tests `release_gate_script.bats` / `require_in_session_gate.bats` / `session_context_script.bats`. Pushed to both remotes; release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.8>.

Decision: shipped as **patch** — shipped-as-broken defect, patch regardless of engineering scope.

Propagation gap surfaced: `intent claude skills sync` checksums `SKILL.md` only, so a script-only edit under `scripts/` does not propagate without `install --force`. This release also touched SKILL.md, so the fleet picks up the fixed `release-gate.sh` automatically on `intent upgrade`. Logged in wip.md backlog.

## Resume target -- next session

No active steel thread. One field smoke worth running:

1. **Concurrent-session acceptance test (Lamplight).** After `intent upgrade` on Lamplight, open 2+ Claude Code sessions, run `/in-session` in each, then a non-slash prompt in both — neither should be gated. Direct acceptance test for the v2.11.8 bug. (Already-running sessions need `/compact` or restart to pick up the new hook scripts.)

Optional follow-on, in order of return:

1. **Skill-sync script-change blind spot** (new this session). Key the sync checksum on the whole skill dir (or hash `scripts/`), not just `SKILL.md`. Cheap fix; prevents silent script drift on future hotfixes.
2. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates. Worked around manually in v2.11.5; needs a real fix before the next minor.
3. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
4. **`/in-review` Elixir fleet sweep** — still parked.
5. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab backlog.
6. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.
7. **ST0040 deferred items** (intentional out-of-scope per ST0040 design.md): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` hook for claim-scope; `intent/.config/whiteboard.json` per-project config. Each revisited only if the v0 advisory model shows brittleness in field use.

## Lessons from this session (top three)

- **Asymmetric sources of truth deadlock; one source self-heals.** The gate read identity from the payload, the releaser from a shared file — two paths that disagreed under concurrency. The fix was collapsing both onto one source (`$CLAUDE_CODE_SESSION_ID`) that degrades symmetrically, not patching the releaser's file. When two cooperating processes must agree on a key, give them the same source.

- **Verify the environment empirically before designing the fix.** The fix hinged on whether Bash tool invocations see `$CLAUDE_CODE_SESSION_ID`. One `env | grep` confirmed it; a second check proved it equalled the gate's payload id. That turned three candidate fix directions into one obvious choice.

- **Skill propagation has a script-only blind spot.** `intent claude skills sync` keys on `SKILL.md` checksum, so editing only a `scripts/` file silently fails to propagate. We got lucky (SKILL.md also changed); a pure script hotfix would have shipped to canon and never reached `~/.claude` or the fleet without `--force`.

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
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes are patches regardless of engineering scope. Additive features default to minor — opt-in-by-presence additions can be argued as patch when the behaviour change is zero for non-adopting projects.
- When a bats suite runs commands against the real project root, snapshot + restore affected files inside the test.
- For non-interactive Intent automation that spawns `claude -p`, always set `INTENT_SKIP_IN_SESSION_GATE=1` on the invocation.
- Never invoke `scripts/release` with `--no-confirm` from inside a tool-driven session — let its interactive confirmation be the human-in-the-loop checkpoint.
- A new skill arriving in canon needs: (1) `intent claude skills list` enumeration test entry, (2) auto-install hook in `bin/intent_upgrade` if it should fleet-propagate, (3) regression test for the upgrade path, (4) docs section anchor in `working-with-llms.md`, (5) CHANGELOG section, (6) `chains_to` edits in companion skills + `intent claude skills sync` propagation, (7) wip/restart bump.
- Cross-project sibling skills can cross-pollinate small conventions cheaply; wholesale dependency adoption only when the dependency footprint matches.
