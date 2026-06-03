# Claude Code Session Restart -- narrative state

## Current state (2026-06-03, end of session -- v2.11.11 shipped)

**v2.11.11 shipped 2026-06-03.** Patch fixing rules-path drift in the LLM guidance Intent generates for consuming projects. Found in Baize ST0001 WP-04 and reported as a handoff (`../Baize/intent/handoff-intent-rules-path.md`, deleted this session per its own acceptance criterion). Affects every project that uses Intent with LLMs. Shipped: tag `v2.11.11` (commit `7531306`) pushed to both remotes, self-upgrade stamp `a7fca3f`.

### The bug

Generated `AGENTS.md` (via `intent agents sync`) and `CLAUDE.md` (from `lib/templates/llm/_CLAUDE.md`), plus the five `critic-<lang>` subagents, told agents the coding-rule library lives at a local `intent/plugins/claude/rules/` path. That directory exists only inside the Intent tool itself; in a consuming project the rules are reachable solely through the CLI (`intent claude rules list` / `show <id>`). A field `critic-elixir` run looked for the local dir, missed, and fell back with a confusing "rule library not installed at the expected path" diagnostic, reviewing at reduced fidelity.

### What landed (three surfaces)

1. **Generated / propagated guidance points at the CLI.** `intent agents sync` generator (`intent/plugins/agents/bin/intent_agents`), `lib/templates/llm/{_CLAUDE,_usage-rules}.md`, and the agent templates (`templates/_default/{AGENTS,RULES}.md`, `templates/elixir/AGENTS.md`, `templates/{shell,rust,lua,swift}/RULES.md`) now describe rule access as served by the installed Intent tool via the CLI — no local-directory reference. Root `AGENTS.md` regenerated; root `CLAUDE.md:36` hand-matched to the template (full `intent claude upgrade --apply` avoided because of the known Phase-2 date-rewrite hazard).
2. **Critic subagents resolve via the CLI.** All five (`intent/plugins/claude/subagents/critic-{elixir,rust,swift,lua,shell}/agent.md`) enumerate with `intent claude rules list --lang <lang>` (+ `--lang agnostic`) and read each rule with `intent claude rules show <id>`, partitioning code-vs-test mode by the `category` column the CLI emits (elixir code mode = `code`+`ash`+`phoenix`+`lv`). The CLI already merges canon + `~/.intent/ext/` rules with provenance + id-shadowing, so the per-critic extension-merge section was removed (Highlander, one enumeration path); the `elixir-test-critic` upstream probe (outside the CLI's reach) is retained in `critic-elixir`.
3. **Propagation.** `bin/intent_upgrade` now runs `intent claude subagents sync` beside the existing skills sync (failure-tolerant, no `--force`), so the corrected critics reach every machine's `~/.claude/agents/` mirror on the next `intent upgrade`. Critics are user-global, so one sync per machine covers all its projects.

### Not touched, and why

- The headless runner (`bin/intent_critic`, `intent/plugins/claude/lib/{critic_runner,rules_lib}.sh`) resolves `INTENT_HOME` centrally and was already correct.
- Intent's repo-local contributor docs (`intent/docs/{rules,critics}.md`, root `usage-rules.md`) keep the source path: it genuinely exists in the Intent repo and is not propagated to consumers.
- `/in-session` and `/in-standards` SKILL.md tables reference the same local path — the same drift class, but left out of this patch's approved scope. Logged as backlog item 8 in `wip.md`; their tests (`tests/unit/in_session_skill.bats:70-73`) still assert the path, so they pass untouched.

### Verification

New regression tests (suite green, run via `tests/run_tests.sh`): `tests/unit/claude_md_template.bats` (template routes through the CLI, no local path), `tests/unit/intent_agents.bats` (generated `AGENTS.md` Rule Library likewise), `tests/unit/intent_upgrade_dispatcher.bats` (upgrade wires + narrates the subagent sync). One existing assertion flipped from "contains the path" to "contains the CLI" in `claude_md_template.bats`; two `_default`-template markers in `intent_claude_upgrade.bats` re-keyed to the new CLI string. Live-verified end-to-end: a real `critic-elixir` run against a Baize file enumerated the agnostic + Elixir rule packs via the CLI with no fallback warning; `intent claude rules list --lang elixir` works from Baize's root; the synced `~/.claude/agents/critic-elixir.md` carries the CLI instructions with zero local-path references.

No steel thread (shipped-as-broken guidance fix, patch). No data migration.

Release-mechanics note: the release was cut concurrently while this fix was being finished, so the commit messages are non-standard for this repo. The fix landed as `7baca20 "Commit for release"` (not a conventional `fix:` commit), then `9cee9a4` (steel_threads.md), then `7531306 "release: v2.11.11"` (VERSION + AGENTS bump, tagged), then `a7fca3f "Intent upgrade"` (self-upgrade). Verified the tagged release contains the complete change set (all five critic CLI swaps, generator, templates, `intent_upgrade` subagent-sync, regression tests). Nothing to re-run. A trailing `docs: finalise wip/restart for v2.11.11` commit (this doc update) sits on top.

## Resume target -- next session

v2.11.11 is shipped (tag pushed both remotes); nothing to release. One housekeeping item: the Baize handoff deletion is an uncommitted change in the Baize repo — commit it there separately. Then, optional follow-on in order of return:

1. **Skill-level rules-path drift** (sibling of this fix, deferred). `/in-session` + `/in-standards` SKILL.md tables still point at `intent/plugins/claude/rules/<lang>/`. Swap to the CLI; update `tests/unit/in_session_skill.bats:70-73`.
2. **`/in-whiteboard verify <stream>` subcommand** (deferred from v2.11.10). Heavier than the Verifier role section warrants today; revisit if the advisory role proves it wants automation.
3. **Skill-sync script-change blind spot.** `intent claude skills sync` checksums `SKILL.md` only, so a script-only edit under `scripts/` does not propagate without `install --force`. (The subagent-sync wiring added this release is a different mechanism; the skills checksum is still the gap.)
4. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Regex sweep rewrites historical migration dates. Worked around manually in v2.11.5 and again this session (hand-edited root `CLAUDE.md` instead of running `--apply`); needs a real fix before the next minor.
5. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
6. **`/in-review` Elixir fleet sweep** — still parked. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab.
7. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) — still parked; Conflab backlog.
8. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.
9. **ST0040 deferred items** (intentional out-of-scope per ST0040 design.md): `intent st new` ST-ID allocation race; `intent whiteboard init` CLI; `PreToolUse` hook for claim-scope; `intent/.config/whiteboard.json` per-project config. Each revisited only if the v0 advisory model shows brittleness in field use.

## Lessons from this session (top three)

- **Fix the source of generated content, not the artefact.** The drift lived in three propagating sources (the agents-sync generator, the LLM templates, the critic agent.md files) plus their downstream regenerated copies. Editing root `AGENTS.md`/`CLAUDE.md` alone would have been silently overwritten on the next sync. The grep-gate after regenerating (`grep intent/plugins/claude/rules AGENTS.md CLAUDE.md`) is what proved the source fix actually flowed through.

- **A behavioural bug in LLM guidance needs an LLM-level fix, not just a doc edit.** The critics were the actual failure (the confusing fallback), not just the docs. Swapping their rule-discovery to the `intent claude rules` CLI — and leaning on the CLI's existing canon+ext merge to delete the redundant per-critic merge section — both fixed the bug and collapsed three enumeration paths into one (Highlander). The CLI's `category` column already encoded the code-vs-test partition the critics needed, so no CLI change was required.

- **Verify in the consuming project, not just the tool.** The bug only manifests in a project that is not Intent. Running a real `critic-elixir` against a Baize file (not a unit fixture) is what actually demonstrated the "not installed" fallback was gone — a green Intent test suite alone could not, because Intent's own repo has the path.

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
- Refresh BOTH restart files (`.claude/restart.md` + `intent/restart.md`) on every release, not just `wip.md` — they drift silently otherwise and their git-log anchors go stale.
- A new skill arriving in canon needs: (1) `intent claude skills list` enumeration test entry, (2) auto-install hook in `bin/intent_upgrade` if it should fleet-propagate, (3) regression test for the upgrade path, (4) docs section anchor in `working-with-llms.md`, (5) CHANGELOG section, (6) `chains_to` edits in any companion skills + `intent claude skills sync` propagation, (7) wip/restart bump.
- Cross-project sibling skills (eg LLMsend ↔ in-whiteboard) can cross-pollinate small conventions cheaply; wholesale dependency adoption only when the dependency footprint matches.
