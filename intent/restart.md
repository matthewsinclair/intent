# Claude Code Session Restart -- narrative state

## Current state (2026-06-11, execution session -- ST0042 WP execution in flight, mid-arc compact point)

ST0042 execution is underway in this session. All gate decisions are taken; nothing is pushed; everything rides the next release (a patch, after the remaining WPs land).

### Decisions locked in this session

- `intent audit` (T6): **retire** (Highlander; critics are the canonical engine). Executes in WP-06.
- WP-06 scope: **excludes** upgrade-subsystem dead scaffolding (F-UPG-9) -- ST0043 owns all upgrade deletions.
- Release cadence: patch after ST0042 WPs complete; ST0043 then targets a minor (v2.12.0).
- WP directory mapping: slate WP1-7 = `WP/01`-`07`; slate WP8 = ST0043; slate WP9 = `WP/08`; slate WP10 = `WP/09`.

### Landed so far (all committed on main, NOT pushed)

1. `1fc4180` docs: session-wrap docs from the review session.
2. `d0e2b1d` docs: WP-01..09 scaffolded + populated; gate decisions recorded in design.md.
3. `554fc0e` **WP-09 part A** (F-TEST-1/9): `setup_fake_home`/`teardown_fake_home` promoted into `tests/lib/test_helper.bash`; all seven fake-HOME files converted; full suite verified to leave real `~/.claude` untouched. WP-09 stays WIP (part B = vacuous tests + coverage gaps, lands LAST).
4. `d959a9b` + `3c6db54` **WP-01 DONE**: config eval eliminated; `read_config_field` (jq field-wise) replaces `parse_json`+`eval`; regression test (red-phase-proven) in `tests/unit/config.bats`.
5. `2736431` **WP-05 part A** (live bug): single `canonical_status` synonym table in `bin/intent_st`; `normalise_status` re-expressed on it; repair/organize inline `wip`->"In Progress" tables deleted; regression test pins `repair` writing `status: WIP`. WP-05 stays WIP (part B = version-fallback literal ~14x, config parsing x3, find_project_root x3, ST-dir resolver x3, ext-dir walk x5; SKIP update_config_version inlines -- ST0043 owns).
6. `a3807b7` + `2e4abea` **WP-03 DONE**: four silent-success paths fixed (st new legacy template -> error; agents init template fallback to generated AGENTS.md; intent_init -> `intent claude subagents install intent`; upgrade backup verified with `error()` abort). Regression tests in st_commands/intent_agents/init_commands/intent_upgrade_dispatcher bats.
7. `899c7cf` + `bafa78a` **WP-04 DONE**: renderers read project `.claude/` + `$HOME/.claude/` deduped; divergent `templates/_default/AGENTS.md` DELETED (init `--template _default` falls back to generated content, passes validate); `intent_agents.bats` HOME-sandboxed; root AGENTS.md regenerated (now lists real installed skills).

### Remaining execution order

WP-05 part B -> WP-02 (rules-path drift + mechanical guard) -> WP-07 (MODULES.md) -> WP-08 (canon docs + `intent st cancel`) -> WP-06 (prune dead code incl. `intent audit` retire) -> WP-09 part B (vacuous tests + coverage for survivors) -> patch release via `scripts/release --patch` (interactive confirm; NEVER --no-confirm) -> mark ST0042 Completed, propose ST0041 closure -> then ST0043 in its own arc.

Incident note (resolved): two WP-04 test fixtures briefly leaked into the real `~/.claude` (`skills/home-skill`, `agents/home-agent.md`) when a setup edit raced the test run; both removed, file now sandboxed, write-watch confirmed clean.

---

## Previous state (2026-06-11, review session -- ST0042 review complete, ST0043 opened, audit deferred)

This session shipped a small fix and produced a comprehensive architectural review of Intent. **No release was cut; the tree is dirty and mid-flight.**

### What landed

1. **`intent st new` version-stamp fix -- commit `f359917`, not pushed.** New steel threads were stamped with a hardcoded `intent_version: 2.4.0` (template path) / `2.0.0` (no-template fallback heredoc), frozen when those were last hand-edited. A Highlander violation: the unified source is `get_intent_version` -> `$INTENT_HOME/VERSION` (`2.11.11`). Fixed both paths -- the template now carries an `[Intent Version]` placeholder substituted in the directory-structure sed block (`bin/intent_st`), and the fallback heredoc uses a `__VERSION__` placeholder substituted from `get_intent_version`. Regression test added to `tests/unit/st_commands.bats`, red-phase-verified against the pre-fix code (it failed with exactly the `2.4.0` symptom). Suite green; live `intent st new` stamps `2.11.11`. ST0041/ST0042 frontmatter repaired 2.4.0 -> 2.11.11. CHANGELOG `[Unreleased]` entry added. Commit `f359917` on `main`, not pushed -- rides the next release.

2. **ST0042 -- Fable 5 review of Intent: review COMPLETE (WPs not executed).** Deliberately structured as the first MFIC exercise (see ST0041): mechanical-sweep coverage, falsifiable findings with file:line evidence + refutation conditions, reviewers causally independent of the producing sessions, human review gate as the control. Eight reviewer dimensions ran in parallel -- architecture, templates/generation, test suite, docs-canon, plugin surface, shell-critic, upgrade/migration, plus an upgrade-rethink design dimension added at the user's request. Inline mechanical sweeps (placeholder drift, hardcoded versions, shellcheck, bash-3.2 compat, size outliers) ran first. Load-bearing findings were independently re-verified against the code before graduating to proposed WPs; the worst (config-eval) was demonstrated by a benign, reverted PoC. Full findings (themes T1-T12) and a 10-WP proposed slate are in `intent/st/ST0042/design.md`; the MFIC leak write-up is in `intent/st/ST0042/impl.md`.

   **Audit purpose -- IMPORTANT for the next session:** architectural integrity, Highlander violations, and poor design/implementation. **NOT security.** The one security finding (T1, the `load_intent_config` eval -> arbitrary code execution from a checked-in `config.json`) is recorded because it surfaced and is real, but it is reframed as an incidental robustness defect, not a security workstream. Do not over-index on it; the lens is design quality.

3. **ST0043 -- Rethink `intent upgrade`: OPENED, design populated, not started.** Review WP8 spun out to its own thread (design-level; ~1800 lines across `bin/intent_upgrade`, the migrate/needs layer in `bin/intent_helpers`, and `intent_claude_upgrade`; L+). `info.md` carries the full Architecture-B design: converge on a declared end-state ("make it so", probing on-disk state) with an ordered structural-step ledger reserved for genuine one-way transforms (day-one ledger: `stp_to_intent`, `prune_backlog`, `relocate_config`, `languages_field`); `intent_upgrade` = orchestrator + sole stamper, `intent_claude_upgrade` = sole canon engine; numeric semver replaces glob enumeration; downgrade hard-errors before mutation; verified backup; stamp once, last. Confirmed defects it must kill: F-UPG-3 (mid-chain stamps the live target, so an interrupted chain silently half-migrates and claims success), F-UPG-1/2 (hard-fail on future versions after mutation begins), the BSD-only `sed -i ''` (Linux breakage), and the unanchored CLAUDE.md version-sed (historical-date corruption). Delete/keep migration path is enumerated in `info.md`.

### Gate decisions (2026-06-11)

- WP8 `intent upgrade` rethink -> spun out to **ST0043**.
- WP9 `st cancel` -> **add the command.** `intent st cancel <ID>` does what it says: moves the thread to CANCELLED (status + relocation). The docs (`usage-rules.md`) already promise it and forbid the manual alternative; the WP makes the docs true.
- **Pending:** `intent audit`'s fate (T6). Its Credo checks R2/R6/R11 overlap rule-library concerns (thin-coordinator, module-highlander, `IN-EX-CODE-003` @impl) but via a different mechanism (compiled Credo AST vs LLM/grep critic heuristics). Decide: retire the overlap, or keep `intent audit` as the precise Elixir-toolchain path and document a division of labour with the critics. Deferred to the new session.

### Resume target -- next session

The user will run the **audit / WP execution in a new session**, framed as architectural integrity / Highlander / design quality (not security). Steps: (1) read `intent/st/ST0042/design.md` (findings + WP slate) and `intent/st/ST0043/info.md` (upgrade rethink); (2) create the approved WPs via `intent wp new`; (3) settle `intent audit`'s fate; (4) execute highest-value-first (the live `wip`->`WIP` vs `wip`->`In Progress` status-normalisation bug in WP5 and the real-`~/.claude` test pollution in WP10 are cheap and concrete). Nothing this session is pushed; `f359917` rides the next release.

### Uncommitted working-tree state at handoff

`f359917` committed (st-new fix). Uncommitted: `intent/st/ST0041/info.md` (MFIC objective/context), `intent/st/ST0042/{info,design,impl,tasks}.md` (review output), `intent/st/ST0043/info.md` (upgrade rethink), `intent/wip.md`, `intent/restart.md`, `.claude/restart.md`. The user asked to document state and wait -- holding before any commit of the doc changes.

---

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
