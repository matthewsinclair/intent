# Claude Code Session Restart -- narrative state

## Current state (2026-04-30, end of session -- v2.11.4 docs patch cut)

v2.11.3's strict-proxy fix verified in the field via a Conflab smoke test. Previously-misfiring patterns clear: `IN-EX-CODE-004` no longer flags single-step `case ... do`; `IN-EX-TEST-003` no longer flags compliant `use ExUnit.Case, async: true`. The gate still produces signal on real violations (e.g. `IN-EX-TEST-001` weak assertions, `IN-EX-TEST-005` control flow in tests, `IN-EX-TEST-007` literal-on-the-right asserts) — proving the gate is not silently passing everything. No stderr `note: skipping` diagnostics — expected, since the rules whose proxies were stripped no longer carry proxy blocks for the runner to refuse.

The smoke also surfaced an architectural clarity gap. The headless runner (`bin/intent_critic` + `intent/plugins/claude/lib/critic_runner.sh`) and the canon rule library load from `$INTENT_HOME` — the Intent install resolved relative to whichever `intent` is on `$PATH` — not from each project's plugin tree. A fix to the runner, or a strip / edit of a canon rule, applies to every Intent project the moment Intent itself updates. v2.11.4 documents this in `intent/docs/critics.md`.

v2.11.4 is a docs-only patch: behaviour is unchanged from v2.11.3.

## What this session did (chronological)

### 1. `/in-session` bootstrap

Project on `main`, working tree clean, ST0039 already moved to COMPLETED, v2.11.3 already cut. `/in-essentials` + `/in-standards` loaded; UserPromptSubmit gate released for the session.

### 2. Plan mode + Tranche A scope

User pasted the prior session's restart prompt and asked for a plan for any remaining work, then "wait for instructions." Plan written to `/Users/matts/.claude/plans/jiggly-inventing-penguin.md`: three tranches (A: Conflab smoke; B: fleet upgrade; C: `/in-review` Elixir fleet sweep). User selected Tranche A.

### 3. Tranche A -- Conflab smoke test

Initial assumption (per the plan) was that each fleet member needed a per-project canon refresh to v2.11.3 before the gate fix would apply. The smoke surfaced the actual architecture: `bin/intent_critic` resolves `INTENT_ROOT` from its own location and sources `$INTENT_ROOT/intent/plugins/claude/lib/critic_runner.sh`; the runner reads canon rules from `$INTENT_HOME/intent/plugins/claude/rules/`. The `intent` symlink on `$PATH` points at the Intent install, so v2.11.3's fix is already live in every project.

Smoke run: `intent critic elixir --files <80 lib + test files> --severity-min recommendation`. Exit 0 on the targeted trigger files (`lib/conflab/runtime_config.ex`, `test/conflab/endpoints_test.exs`); exit 1 on the broad sweep with 1 critical + 6 warnings, all genuine pre-existing findings on rules whose proxies survived the strip (`TEST-001` / `005` / `007`). None of the eight stripped rules or the surgically-edited `IN-EX-CODE-004` produced findings on Conflab's `case` or `async:` patterns. Stderr empty.

Conflab tree untouched -- no upgrade, no checkout, no commit, no `--no-verify` replay. The smoke proves the fix on the live working tree.

### 4. v2.11.4 docs patch

User asked to check in and commit, then mint a patch release. v2.11.4 cut:

- `intent/docs/critics.md` "Code locality" note added under "Headless runner": runner + canon rules load from `$INTENT_HOME`, not per-project; per-project canon refresh stays useful for keeping `intent/llm/RULES*.md` and `.claude/skills/` copies in sync but is not on the critical path for runner / rule fixes.
- `CHANGELOG.md` `[2.11.4]` section: docs + verification entries.
- `intent/wip.md`, `intent/restart.md`, `.claude/restart.md` refreshed.
- `scripts/release --patch` cut the release.

## Resume target -- next session

No active steel thread. Optional follow-on, in order of return:

1. **Per-project canon refresh** for fleet members on stale Intent versions. Hygiene -- the runner + rules are already current via `$INTENT_HOME`. `intent claude upgrade` per project. Targets: Anvil (2.10.0), Lamplight (2.11.0), MeetZaya (2.10.0), MicroGPTEx (2.10.0), Conflab (2.11.0).
2. **`/in-review` Elixir fleet sweep** -- Tranche C of the post-v2.11.3 plan. Catalogue findings per project; surface to user before any remediation.
3. **Conflab pre-existing test findings** surfaced during the smoke (`TEST-001` / `005` / `007`, 7 hits) -- worth folding into Conflab's own backlog.
4. **Deferred v2.11.x backlog** -- `intent claude upgrade` Phase-2 CLAUDE.md substitution audit; Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft.

## Lessons from this session

- **Verify your architecture before you plan a fleet upgrade.** Tranche B was scoped on the assumption that each project carried its own runner copy. The actual architecture -- runner + canon loaded from `$INTENT_HOME`, not per-project -- collapses Tranche B into pure hygiene work. Five minutes of `grep INTENT_ROOT bin/intent_critic` would have caught this before the plan went out. Lesson: when the plan touches per-project canon, trace the resolution path of whichever script the gate actually runs, end to end, before sizing the work.

- **A clean smoke is a smoke that finds something real.** The Conflab broad sweep returned 7 findings -- not v2.11.3 regressions, but pre-existing genuine violations. That's the right shape for "the gate works": misfiring patterns clear, and real signal still surfaces. A smoke test that returns "exit 0, 0 findings everywhere" wouldn't have been meaningfully different from a smoke test that did nothing.

- **Docs patches are legitimate.** The runner-locality clarification doesn't change behaviour, but it changes the mental model anyone reads the docs with. v2.11.4 ships docs-only because the docs were genuinely behind the architecture. The patch / minor / major decision is semantic; "fixed shipped-as-broken docs" qualifies as a patch.

## Risks for post-cut

- The fleet's per-project canon copies (`intent/llm/RULES*.md`, `.claude/skills/`, `pre-commit.intent`) are still on whatever Intent version the project last ran `intent claude upgrade` against. The runtime gate uses the central `$INTENT_HOME` copies, so this doesn't affect gate behaviour, but it does mean any developer reading their project's `intent/llm/RULES-elixir.md` may see proxy blocks that have since been stripped from canon. A per-project refresh sweep closes the gap.

- Conflab's surfaced `TEST-001` / `005` / `007` findings are genuine and have been present in the codebase. They were hidden by gate _noise_, not gate silence -- the over-broad proxies fired so loudly that real findings were lost in the false positives. Now that the noise is gone, the signal needs triage.

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
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes (including shipped-as-broken docs) are patches regardless of engineering scope.
