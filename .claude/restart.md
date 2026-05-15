# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show `2780611 release: v2.11.6` at the top.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-05-15, end of session -- v2.11.6 cut)

v2.11.6 shipped today. Single-rule additive patch: a new Lua coding rule (**IN-LU-CODE-006 — Dispatch table over if-chain for value dispatch**) surfaced during a parallel Lamplight session (ST0163 WP-04, Murder mechanic hook authoring). Lua has no pattern matching and no multi-head function definitions; the idiomatic substitute is a table-of-functions keyed by the discriminating value with a single lookup + invoke at the call site. Concretises IN-AG-PFIC-001; sister rule IN-EX-CODE-001 (Elixir multi-head dispatch). Enforcement via the `critic-lua` subagent (prose Detection, no Greppable proxy — matches existing Lua-pack convention).

Integration: rule file in canon at `intent/plugins/claude/rules/lua/code/dispatch-table-over-if-chain/RULE.md`; registered in `tests/unit/rule_pack_lua.bats` `lua_rules()` heredoc; `tests/fixtures/critics/lua/code/would-catch/sample.lua` extended with a `perturbation.tag` dispatch chain; `manifest.txt` lists IN-LU-CODE-006; `intent/plugins/claude/rules/index.json` regenerated. Two commits on `main`: `a5f3d54` integration, `2780611` release. Pushed to both remotes; release at <https://github.com/matthewsinclair/intent/releases/tag/v2.11.6>.

Decision worth carrying forward: shipped as **patch** at user direction, overriding the project's stated "rule additions are minor" precedent (v2.9.0 added the Lua pack as minor). Re-confirm patch-vs-minor at the time of the next rule addition; do not assume from this case.

Fleet pickup is automatic — `critic-lua` and the headless runner load rules from `$INTENT_HOME`, not from per-project plugin trees. No per-project upgrade step needed.

## Resume target -- next session

No active steel thread. Optional smoke: invoke `critic-lua` from a fleet project against a Lua file containing a tag dispatch chain (eg one of the Lamplight Murder mechanics files pre-refactor) to confirm IN-LU-CODE-006 fires field-side.

Optional follow-on, in order of return:

1. **`intent claude upgrade` Phase-2 CLAUDE.md substitution audit.** Its regex sweep rewrites historical migration dates. Worked around manually in the v2.11.5 session; needs a real fix before the next minor.
2. **`lib/templates/usr/_user_guide.md`.** Orphan template, not STP-tainted but still cruft. Delete or repurpose.
3. **`/in-review` Elixir fleet sweep** -- still parked.
4. **Conflab pre-existing test findings** (`IN-EX-TEST-001` / `005` / `007`) -- still parked; Conflab backlog.
5. **Deferred v2.11.x backlog**: Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft (v2.11.6 is the sixth dogfood datapoint).

## Lessons from this session (top three)

- **A new rule arriving from a parallel session is a small but real integration job.** Dropping `RULE.md` into canon is necessary but not sufficient: `rule_pack_lua.bats` has a hardcoded `lua_rules()` enumeration that fails the count/presence/validator invariants until updated; `index.json` must be regenerated via `intent claude rules index`; the would-catch fixture is the natural place to add a violating sample. Each touchpoint is small; missing any one of them is a silent gap.

- **Auto-mode classifier saved a `--no-confirm` mistake.** Cutting a public release with `--no-confirm` was blocked even though the plan was pre-approved — and rightly so. Plan approval covers the path, not bypasses of the release script's own safety gates. Run interactive cuts interactively (or surface the dry-run for human review) rather than asking the harness to swallow a confirmation prompt.

- **Patch-vs-minor framing is user-call, not auto-derived.** Project memory says rule additions are minors (precedent: v2.9.0). User chose patch for this single-rule addition. Don't argue with the call; recommend the framing, accept the override, document the decision in restart.md so future sessions know it's not a new pattern.

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
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes (including silent failures) are patches regardless of engineering scope. Additive features (eg new rules) default to minor — but user can override case by case.
- When a bats suite runs commands against the real project root, snapshot + restore affected files inside the test.
- For non-interactive Intent automation that spawns `claude -p`, always set `INTENT_SKIP_IN_SESSION_GATE=1` on the invocation.
- Never invoke `scripts/release` with `--no-confirm` from inside a tool-driven session — let its interactive confirmation be the human-in-the-loop checkpoint.
- A new rule arriving in canon needs: (1) test-pack enumeration entry, (2) would-catch fixture sample + manifest entry, (3) `intent claude rules index` regen, (4) CHANGELOG section, (5) wip/restart bump.
