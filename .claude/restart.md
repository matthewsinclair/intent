# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show `9d926dc chore: delete orphan STP-era templates` at the top.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-05-05, end of session -- v2.11.5 cut + fleet upgraded + STP cleanup)

v2.11.5 shipped today. Three behavioural fixes for latent bugs surfaced by a Conflab session:

1. **Gate bypass for non-interactive `claude -p`** -- `INTENT_SKIP_IN_SESSION_GATE=1` short-circuits `require-in-session.sh` so `intent treeindex` and any other `claude -p` automation bypasses the strict gate. Treeindex was silently failing per directory because the non-bare `claude -p` swallows the hook's exit 2 and emits empty stdout.
2. **`intent agents generate` PROJECT_ROOT self-load** -- `intent_agents_generate_content` now loads project context if the dispatcher branch did not. Latent since the dispatcher was first added 2025-08-20.
3. **`migrate_v2_10_x_to_v2_11_0` stamps live target** -- previously hard-coded to `"2.11.0"`; now uses `get_intent_version`. Field impact muted by the `needs_v2_11_0_upgrade` short-circuit but the bug existed.

Plus: test-isolation fix to `docs_completeness.bats:agents_sync_idempotent` (was leaving Intent's AGENTS.md dirty post-suite, blocking subsequent `scripts/release` pre-flights).

Post-release: fleet upgrade across `~/Devel/prj/*` -- nine projects upgraded directly (Anvil, Laksa, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz), plus Intent self-stamp; user handled Conflab and Lamplight. Fleet is now uniformly on v2.11.5.

STP cleanup: stripped "(formerly STP)" + "## Migration Notes" from CLAUDE.md on the four projects that had them (Anvil, Laksa, MeetZaya, Multiplyer); fixed `lib/templates/prj/_wip.md` (live `intent_init` seed); deleted three orphan STP-era templates from canon. `grep -rln STP lib/templates/` now returns nothing.

## Resume target -- next session

No active steel thread. Optional follow-on:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Its regex sweep rewrites historical migration dates. Worked around manually this session; needs a real fix before the next minor.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** -- still parked.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) -- still parked; Conflab backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.

## Lessons from this session (top three)

- **The non-bare `claude -p` swallows hook stderr.** A blocking hook produces silent success: exit 0, empty stdout. Every `claude -p` wrapper that runs against an Intent project needs `INTENT_SKIP_IN_SESSION_GATE=1`. Today only treeindex does. A periodic grep audit is worth adding to catch new wrappers.

- **A bats test that runs commands against the real project root pollutes the tree for every downstream operation.** `docs_completeness.bats:agents_sync_idempotent` had been leaving AGENTS.md dirty for an unknown number of releases. Suspect a leaky test before suspecting the release script when the pre-flight reports a phantom dirty file.

- **Latent bugs hide behind short-circuits.** `migrate_v2_10_x_to_v2_11_0`'s hard-coded `"2.11.0"` stamp was wrong since v2.11.0 cut, but `needs_v2_11_0_upgrade` short-circuited the migration on most projects. When a guard predicate skips a code path, the skipped path still runs sometimes. Make the inside of the guard correct on its own.

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
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes (including silent failures) are patches regardless of engineering scope.
- When a bats suite runs commands against the real project root, snapshot + restore affected files inside the test.
- For non-interactive Intent automation that spawns `claude -p`, always set `INTENT_SKIP_IN_SESSION_GATE=1` on the invocation.
