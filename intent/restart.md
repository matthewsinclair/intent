# Claude Code Session Restart -- narrative state

## Current state (2026-04-27, end of session -- ST0036 closed; ST0035 13 of 19 Done)

**Intent v2.10.0 in progress. ST0035 (Canonical LLM Config + Fleet Rollout) 13 of 19 Done; WP14-WP19 pending. ST0036 (Directory relocation) 9 of 9 Done; closed and moved to COMPLETED.**

### ST0035 shape

- **Done (13)**: WP01-WP13.
- **Not Started (6)**: WP14, WP15, WP16, WP17, WP18, WP19.

WP19 was added this session as a Phase 0 spec (`a7c27c3`) for the explicit-language canon command. Replaces the auto-detection approach that was tried-and-rejected during WP08 surfacing. Sized M; lands when fleet rollout context is sufficient.

Critical path remaining: `ST0035/WP14 -> WP15 -> WP16 -> WP17`. WP18 (`intent/usr/*.md` audit) runs in parallel with WP15/WP16; must land before WP17. WP19 (per-language canon) is independent; can land anywhere after WP14.

### ST0036 shape

- **Done (9)**: WP01 (`4dcccce`), WP02 (`5369afd` + fix `33a99d0`), WP03 (`777c5b0`), WP04 (`5f8b61e` + earlier `f04db11`), WP05 (`b62ea58`), WP06 (`32df058`), WP07 (`1debc03`), WP08 (`5c782b3`), WP09 (`1497885`).
- ST0036 closed via `intent st done ST0036` -- moved to `intent/st/COMPLETED/ST0036/`.

### Progress this session (5 commits + ST0036 close)

In commit order:

1. `01159ff` -- **WP-01 dispatcher fix** (surfaced by WP-08). `bin/intent_upgrade` early-exit + `needs_v2_10_0_upgrade` shortcut + new `2.10.0` case arm. Layout-keyed idempotence catches projects stamped 2.10.0 before relocation.
2. `ebd6620` -- **WP-11 canon-installer fix** (surfaced by WP-08). PROJECT_NAME from `intent/.config/config.json` (was `basename "."` -> `.`); always-`_default` templates (was hard-coded Elixir templates installed for Intent's bash CLI). New `intent/plugins/agents/templates/_default/` with three generic templates.
3. `a7c27c3` -- **WP-19 Phase 0 spec**. New ST0035 WP for explicit-language canon: `intent lang init <lang>` + `intent init --lang ...`. Multi-language is the rule, auto-detection rejected.
4. `5c782b3` -- **WP-08 Intent self-apply**. Reverted WP-05 diagnostic mv; `intent upgrade` ran all three phases of `migrate_v2_9_0_to_v2_10_0` cleanly. Git rename detected (`R .intent/config.json -> intent/.config/config.json`). 781/781 BATS green; doctor clean.
5. `1497885` -- **WP-09 cross-thread coordination**. ST0035 WP15/WP16/WP17 info.md gained 12-point checklist (10 ST0035 + 2 ST0036) + version flips (v2.9.1 -> v2.10.0; .intent/ -> intent/.config/). ST0036/impl.md finalised with closing notes.

Plus ST0036 closed (commit pending).

### Lessons worth keeping (this session)

- **WP08's "moment of truth" worked as designed.** The manual-mv diagnostic in the prior session bypassed the dispatcher and canon-installer entirely. WP08's proper run via `intent upgrade` surfaced two real bugs and spawned one needed feature (WP19). The "file gap, fix in place, ship cleaner" rhythm was natural; the user direction "fix this properly here now fully, and then fail forward" turned the surfacing into a productive remediation sweep rather than a halt.
- **Auto-detection of project language is a dead end.** Real projects are polyglot (Elixir + Swift + Rust + Lua + Bash + HTML/CSS/JS). Picking a single "primary" misrepresents the project shape and just shifts the wrong-shape problem. Replaced with explicit user choice (`intent lang init <lang>`) per WP19 spec.
- **Layout-keyed idempotence beats stamp-keyed.** Mid-ST stamp retargets can leave a project at a target stamp but on the wrong layout. Any check that asks "are we already at the target?" must inspect both axes. Three coordinated changes were needed in `bin/intent_upgrade` + `bin/intent_helpers` to make this work end-to-end.
- **PROJECT_DIR resolution matters.** `basename "."` returns `.`, which becomes `# .` as a regenerated CLAUDE.md title. Resolve to absolute path before basename, and prefer the canonical `project_name` from `intent/.config/config.json`.
- **WP05 diagnostic mv was great.** It proved the path-probe layer was correct without committing anything. Cheap experiment, sharp signal. Worth budgeting for similar "what if I just did the thing" probes in future mechanical sweeps.
- **Linter cooperates.** Markdown linter auto-aligns tables on save; commits naturally include linter touch-ups. No friction.

### Open follow-ups (outside ST0035)

- `docs/blog/_drafts/####-shell-critic-inception.md` -- blog draft; publication gated on real dogfood runs.
- WP07 follow-ups from ST0034: align Diogenes fixture-context handling across critic agent.md files; tighten IN-RS-CODE-005 carve-out for teaching fixtures.
- `git tag -d wp08-pre-relocate` -- backup tag at `69069eca`. Discardable next session.

### Resume target -- ST0035/WP14

WP08 already executed Phase 3 (canon-apply) on Intent during the relocation. WP14 becomes a verification sweep: re-run `intent claude upgrade` (no `--apply`); confirm all canon artefacts UP TO DATE / PRESENT; intent doctor; full BATS; verify hooks fire; mark Done. See `.claude/restart.md` for the step-by-step.

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
