# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate. As of v2.11.7, also chains to `/in-whiteboard pickup` if `intent/whiteboard/` exists.
2. **Verify the working tree.** v2.11.11 is shipped: tag `v2.11.11` at `7531306`, `main` pushed to both remotes at `a7fca3f "Intent upgrade"`, with a trailing `docs: finalise wip/restart for v2.11.11` commit on top. `git status` clean. Nothing to release.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; `intent/restart.md` carries the full narrative; this file's "Resume target" says what to do next.

## State (2026-06-03, end of session -- v2.11.11 shipped)

v2.11.11 fixes rules-path drift in the LLM guidance Intent generates for consuming projects. Found in Baize ST0001 WP-04 (handoff `../Baize/intent/handoff-intent-rules-path.md`, deleted this session per its own acceptance criterion). Affects every project that uses Intent with LLMs. Patch, no steel thread. **Shipped:** tag `v2.11.11` (`7531306`) pushed to both remotes; self-upgrade `a7fca3f`. The release was cut concurrently with the fix, so its commits carry non-standard messages ("Commit for release" / "Intent upgrade") — the tagged release was verified to contain the full change set. See `intent/restart.md` for the mechanics note.

**The bug:** generated `AGENTS.md` (via `intent agents sync`), `CLAUDE.md` (from `lib/templates/llm/_CLAUDE.md`), and the five `critic-<lang>` subagents told agents the rule library lives at a local `intent/plugins/claude/rules/` path. That directory exists only inside the Intent tool; in a consuming project the rules are reachable solely via the CLI (`intent claude rules list` / `show`). A field `critic-elixir` run missed the local dir and fell back with a confusing "rule library not installed" diagnostic.

**Three surfaces fixed:**

- **Generated guidance → CLI.** `intent/plugins/agents/bin/intent_agents` generator, `lib/templates/llm/{_CLAUDE,_usage-rules}.md`, and agent templates (`templates/_default/{AGENTS,RULES}.md`, `templates/elixir/AGENTS.md`, `templates/{shell,rust,lua,swift}/RULES.md`) now describe rule access via the CLI, no local path. Root `AGENTS.md` regenerated; root `CLAUDE.md:36` hand-matched (avoided `intent claude upgrade --apply` because of its known Phase-2 date-rewrite hazard).
- **Critics → CLI.** All five `critic-{elixir,rust,swift,lua,shell}/agent.md` enumerate via `intent claude rules list --lang <lang>` and read each rule with `show <id>`, partitioning code-vs-test mode by the `category` column. Per-critic extension-merge section removed (the CLI already merges canon+ext with provenance; Highlander); `elixir-test-critic` upstream probe kept.
- **Propagation.** `bin/intent_upgrade` now runs `intent claude subagents sync` beside the skills sync, so the corrected critics reach each machine's `~/.claude/agents/` on next `intent upgrade`.

**Not touched:** `bin/intent_critic` + `rules_lib.sh` (resolve `INTENT_HOME` centrally, already correct); Intent's repo-local contributor docs + root `usage-rules.md` (path is real there, not propagated). **Same-drift follow-up:** `/in-session` + `/in-standards` SKILL.md tables still cite the local path — left out of scope, logged in `wip.md`.

**Verified:** new regression tests across `claude_md_template.bats`, `intent_agents.bats`, `intent_upgrade_dispatcher.bats` (one existing assertion flipped, two `_default` markers re-keyed in `intent_claude_upgrade.bats`); full suite green via `tests/run_tests.sh`. Live: a real `critic-elixir` run against a Baize file enumerated the agnostic + Elixir packs via the CLI with no fallback; the synced `~/.claude/agents/critic-elixir.md` has zero local-path refs.

## Resume target -- next session

v2.11.11 is shipped (tag pushed both remotes); nothing to release. Housekeeping: the Baize handoff deletion is an uncommitted change in the Baize repo — commit it there separately. Then, optional follow-on:

1. **Skill-level rules-path drift** (sibling of this fix, deferred). `/in-session` + `/in-standards` SKILL.md tables still point at `intent/plugins/claude/rules/<lang>/`. Swap to the CLI; update `tests/unit/in_session_skill.bats:70-73`.
2. **`/in-whiteboard verify <stream>` subcommand** (deferred from v2.11.10). Revisit if the advisory Verifier role wants automation.
3. **Skill-sync script-change blind spot.** `intent claude skills sync` checksums `SKILL.md` only; a script-only edit under `scripts/` needs `install --force`. (The subagent-sync wiring added this release is a separate mechanism.)
4. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates; worked around again this session. Needs a real fix before the next minor.
5. **`lib/templates/usr/_user_guide.md`.** Orphan template; delete or repurpose.
6. **`/in-review` Elixir fleet sweep** — parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
7. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — parked; Conflab backlog.
8. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.
9. **ST0040 deferred items** (per ST0040 design.md): `intent st new` ST-ID race; `intent whiteboard init` CLI; `PreToolUse` claim-scope hook; `intent/.config/whiteboard.json` per-project config. Revisit only if the v0 advisory model proves brittle.

## Lessons from this session (top three)

- **Fix the generating source, not the artefact.** The drift lived in the agents-sync generator, the LLM templates, and the critic agent.md files — editing root `AGENTS.md`/`CLAUDE.md` alone would have been overwritten on the next sync. The grep-gate after regenerating proved the source fix flowed through.

- **An LLM-guidance bug needs an LLM-level fix.** The critics were the actual failure, not just the docs. Routing their rule discovery through the `intent claude rules` CLI fixed the bug and let three enumeration paths collapse into one (the CLI already merges canon+ext, and its `category` column already encoded the code-vs-test split — no CLI change needed).

- **Verify in the consuming project, not just the tool.** The bug only manifests where the project is not Intent. A real `critic-elixir` run against a Baize file is what demonstrated the fallback was gone; a green Intent suite alone could not, because Intent's own repo has the path.

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
- Refresh BOTH restart files (`.claude/restart.md` + `intent/restart.md`) on every release, not just `wip.md` — they drift silently otherwise and their git-log anchors go stale.
- A new skill arriving in canon needs: (1) `intent claude skills list` enumeration test entry, (2) auto-install hook in `bin/intent_upgrade` if it should fleet-propagate, (3) regression test for the upgrade path, (4) docs section anchor in `working-with-llms.md`, (5) CHANGELOG section, (6) `chains_to` edits in companion skills + `intent claude skills sync` propagation, (7) wip/restart bump.
- Cross-project sibling skills can cross-pollinate small conventions cheaply; wholesale dependency adoption only when the dependency footprint matches.
