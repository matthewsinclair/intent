# Claude Code Session Restart -- narrative state

## Current state (2026-05-18, end of session -- v2.11.7 cut)

**v2.11.7 shipped 2026-05-18.** Both remotes pushed (`local` Dropbox + `upstream` GitHub); release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.7>. ST0040 (whiteboard protocol for multi-Claude sessions in the one repo) is marked **Completed**.

### The skill

**`/in-whiteboard`** — multi-session coordination protocol. Each Claude Code session running concurrently against the same Intent project belongs to a durable **stream** (eg `control`, `ia-ux`) that owns one file under `intent/whiteboard/`. Stream files carry frontmatter (`stream_id`, `current_session_id`, `session_started_at`, `heartbeat_at`, `status`, `focus`, `claimed_steel_threads`, `recent_memory_writes`) and an advisory markdown body. Cross-stream point-to-point handoffs go in shared append-only `asks.md`. Shared-platform-layer edits (eg `apps/lamplight/**` in Lamplight) coordinate via a per-project shared file (`lamplight.md`, `core.md`, etc).

Subcommand surface: `pickup` / `claim` / `unclaim` / `touch` / `ask` / `decide` / `lamplight` / `release` / `status`. Claims are by steel-thread ID only — glob-path claims rejected by design because they drift from actual edits. Heartbeat older than 7 days marks a claim reclaimable; reclaim requires explicit user acknowledgement.

### Integration shipped

- New skill canonical source at `intent/plugins/claude/skills/in-whiteboard/SKILL.md`.
- `/in-session` chains to `/in-whiteboard pickup` as new step 5 (after gate release).
- `/in-finish` chains to `/in-whiteboard release` as new step 1 (before any wip/restart/done updates).
- Both chains opt-in by presence: if `intent/whiteboard/` does not exist in the project root, the chained step skips silently.
- `bin/intent_upgrade` auto-installs `in-whiteboard` and re-syncs the canon skill mirror after the migration dispatcher completes. Idempotent + failure-tolerant; no `--force` so user customisations are never silently lost.
- New "Multi-session coordination" section in `intent/docs/working-with-llms.md` after "Skills and /in-session auto-load". Covers tense/reader/cadence vs `wip.md`, file layout, stream identity discovery, ST-only claims, shared platform layer pattern, chain integration, heartbeat semantics, Lamplight live reference.
- `asks.md` header conventions extended with optional `Re: <prior-ask-anchor>` (reply threading) and `FYI only -- no response needed.` (info-dump marker). Borrowed from the cross-project LLMsend protocol (<https://github.com/pmarreck/llmsend>); the tmux/kitty live-ping mechanism from LLMsend was considered and deliberately not adopted for the intra-project case.
- `tests/unit/skills_commands.bats` enumerates `in-whiteboard` in the canonical-roster invariant.
- `tests/unit/intent_upgrade_dispatcher.bats` gains a regression case asserting a v2.10.x → current-target upgrade lands `in-whiteboard` at `~/.claude/skills/in-whiteboard/SKILL.md` (fake-`$HOME` sandbox).
- `CHANGELOG.md` carries `## [2.11.7] - 2026-05-18`.
- Three commits on `main`: `ce38a10` skill landing (out-of-cycle to unblock Lamplight), `b85dc10` integration, `f09bb65` release.

Decision: shipped as **patch** (v2.11.7) at user direction. New-skill default is minor; the protocol is opt-in by directory presence with zero behaviour change for non-adopting projects, which makes the patch framing defensible. Document the decision so future skill additions know it's not a new pattern.

### Caveat for already-running sessions

A Claude Code session already running at upgrade time has the old `in-session` / `in-finish` prose loaded in context — the new chain only auto-fires from the next `/in-session` (after `/compact` or session restart). Manual `/in-whiteboard pickup` works in the current session. New sessions started after upgrade get the chain.

## Resume target -- next session

No active steel thread. Three optional smokes worth running once before settling:

1. **Field-side `/in-whiteboard` exercise.** Lamplight has a populated `intent/whiteboard/` with `control.md` and `ia-ux.md` streams. The next `/in-session` in Lamplight should auto-fire `pickup`, surface other-stream state, and refresh heartbeats. If the chain doesn't fire, the running session predates the canon sync — `/compact` and retry.
2. **Fleet pickup smoke.** Pick one downstream project (eg Conflab) and run `intent upgrade`. Confirm the "Ensuring in-whiteboard skill is installed..." line appears and the install completes. Confirm `intent claude skills sync` line also fires. Verify `~/.claude/skills/in-whiteboard/SKILL.md` lands.
3. **Negative smoke**: a project without `intent/whiteboard/` directory should see zero behaviour change from `/in-session` and `/in-finish`. Confirm the chain step skips silently.

Optional follow-on, in order of return:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates. Worked around manually in the v2.11.5 session; needs a real fix before the next minor.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** — still parked.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft (v2.11.7 is the seventh dogfood datapoint).
6. **ST0040 deferred items** (intentional out-of-scope per ST0040 design.md): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` hook for claim-scope; `intent/.config/whiteboard.json` per-project config. Each revisited only if the v0 advisory model shows brittleness in field use.

## Lessons from this session (top three)

- **A skill arriving from a parallel session is a multi-surface integration job.** Dropping `SKILL.md` into canon is the headline change, but the work to make it land in the formal release is broader: upgrade-path auto-install (so fleet projects pick it up without a manual step), companion skill chain edits propagated via `intent claude skills sync` (so already-installed `in-session` / `in-finish` mirrors get the new prose), test enumeration update, regression test for the upgrade flow, docs section anchor, CHANGELOG entry, ST status flip. Each touchpoint is small; missing any one is a silent gap.

- **Opt-in-by-presence is the right default for protocol additions.** ST0040's `if [ -d intent/whiteboard ]` guard in the chained skill prose means projects that don't want multi-session coordination see zero behaviour change. This made it possible to ship as a patch rather than a minor — the surface area is additive and self-gated. Worth replicating for future protocol additions where adoption is per-project.

- **Cross-project skills can cross-pollinate cheaply.** The LLMsend protocol (cross-project messaging via tmux send-keys + kitty CSI u) and in-whiteboard (intra-project file-based coordination) solve different problems on different axes, but the small conventions (`Re:` reply threading, `FYI only` marker) transfer cleanly. Worth surfacing as docs cross-references when adjacent protocols exist; not worth borrowing wholesale when the dependency footprint differs (tmux/kitty vs file-only).

## Risks for post-cut

- A Claude Code session already running at upgrade time has the old in-session/in-finish prose in context — the chain auto-fire only kicks in on `/compact` or restart. Manual `/in-whiteboard pickup` works as a workaround in the current session, but a user who upgrades mid-session and doesn't know about the workaround will wonder why coordination "isn't firing". Documented in CHANGELOG as a caveat.
- The auto-install in `intent upgrade` runs `intent claude skills install` and `intent claude skills sync` without `--force`. If a user has hand-modified their `~/.claude/skills/in-whiteboard/SKILL.md` or `~/.claude/skills/in-session/SKILL.md`, the sync will prompt to overwrite. Default `N` preserves their customisations but means they won't get the v2.11.7 chain prose until they explicitly opt in. Acceptable trade-off: never silently overwrite > always pick up latest canon.
- The `intent upgrade` short-circuit at "already at target version" skips the auto-install block entirely. Users at v2.11.7 stamp who somehow didn't get the skill installed (eg first-upgrade failed mid-flight) need to run `intent claude skills install in-whiteboard` directly. Documented in ST0040 impl.md.

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
