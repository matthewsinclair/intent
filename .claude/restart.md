# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate. As of v2.11.7, also chains to `/in-whiteboard pickup` if `intent/whiteboard/` exists.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show `f09bb65 release: v2.11.7` at the top.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-05-18, end of session -- v2.11.7 cut)

v2.11.7 shipped today. Multi-session coordination protocol (ST0040): new `/in-whiteboard` skill at `intent/plugins/claude/skills/in-whiteboard/` with subcommands `pickup` / `claim` / `unclaim` / `touch` / `ask` / `decide` / `lamplight` / `release` / `status`. Each Claude Code session running concurrently against the same Intent project belongs to a durable **stream** that owns one file under `intent/whiteboard/`; cross-stream handoffs go in shared append-only `asks.md`; shared-platform-layer edits coordinate via a per-project shared file (`lamplight.md`, `core.md`, etc). Claims are by steel-thread ID only — glob-path claims rejected. Heartbeat older than 7 days marks a claim reclaimable.

Integration: `/in-session` chains to `pickup` as step 5; `/in-finish` chains to `release` as step 1. Both chains opt-in by directory presence — projects without `intent/whiteboard/` see zero behaviour change. `bin/intent_upgrade` auto-installs `in-whiteboard` and re-syncs the canon skill mirror after the migration dispatcher completes. Regression test at `tests/unit/intent_upgrade_dispatcher.bats` (test #4). New "Multi-session coordination" section in `intent/docs/working-with-llms.md`. `asks.md` header conventions extended with optional `Re:` and `FYI only` markers — borrowed from the cross-project LLMsend protocol (in-whiteboard is the intra-project sibling).

Three commits on `main`: `ce38a10` skill landing, `b85dc10` integration, `f09bb65` release. Pushed to both remotes; release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.7>. ST0040 marked Completed.

Decision worth carrying forward: shipped as **patch** at user direction. New-skill default is minor; the protocol is opt-in by directory presence with zero behaviour change for non-adopting projects, which makes the patch framing defensible. Re-confirm at the time of the next skill addition; do not assume from this case.

## Resume target -- next session

No active steel thread. Three optional smokes before settling:

1. **Field-side `/in-whiteboard` exercise.** Lamplight has a populated `intent/whiteboard/` with `control.md` and `ia-ux.md` streams. Next `/in-session` in Lamplight should auto-fire `pickup`. If the chain doesn't fire, the running session predates the canon sync — `/compact` and retry.
2. **Fleet pickup smoke.** Pick one downstream project (eg Conflab) and run `intent upgrade`. Confirm the "Ensuring in-whiteboard skill is installed..." line appears and `~/.claude/skills/in-whiteboard/SKILL.md` lands.
3. **Negative smoke**: a project without `intent/whiteboard/` should see zero behaviour change from `/in-session` / `/in-finish` (chain step skips silently).

Optional follow-on, in order of return:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates. Worked around manually in v2.11.5; needs a real fix before the next minor.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** — still parked.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft (v2.11.7 is the seventh dogfood datapoint).
6. **ST0040 deferred items** (intentional out-of-scope per ST0040 design.md): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` hook for claim-scope; `intent/.config/whiteboard.json` per-project config. Each revisited only if the v0 advisory model shows brittleness in field use.

## Lessons from this session (top three)

- **A skill arriving from a parallel session is a multi-surface integration job.** Dropping `SKILL.md` into canon is the headline change, but the work to make it land in the formal release is broader: upgrade-path auto-install, companion-skill chain edits propagated via `intent claude skills sync`, test enumeration update, regression test for the upgrade flow, docs section anchor, CHANGELOG entry, ST status flip. Each touchpoint is small; missing any one is a silent gap.

- **Opt-in-by-presence is the right default for protocol additions.** ST0040's `if [ -d intent/whiteboard ]` guard in the chained skill prose means projects that don't want multi-session coordination see zero behaviour change. This made it possible to ship as a patch rather than a minor — the surface area is additive and self-gated. Worth replicating for future protocol additions where adoption is per-project.

- **Cross-project skills can cross-pollinate cheaply.** LLMsend (cross-project messaging via tmux send-keys + kitty CSI u) and in-whiteboard (intra-project file-based coordination) solve different problems on different axes, but the small conventions (`Re:` reply threading, `FYI only` marker) transfer cleanly. Worth surfacing as docs cross-references when adjacent protocols exist; not worth borrowing wholesale when the dependency footprint differs (tmux/kitty vs file-only).

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
