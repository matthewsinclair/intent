# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate.
2. **Verify the working tree.** `git status` should be clean. `git log --oneline -5` should show the v2.11.4 release commit at the top.
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-04-30, end of session — v2.11.4 docs patch shipped)

v2.11.3's strict-proxy fix verified in field via Conflab smoke test. Previously-misfiring patterns clear (`IN-EX-CODE-004` no longer flags single-step `case ... do`; `IN-EX-TEST-003` no longer flags compliant `use ExUnit.Case, async: true`). Gate still produces real signal on genuine violations. v2.11.4 cut as a docs-only patch capturing the field verification and clarifying the critic-runner code locality (runner + canon rules load from `$INTENT_HOME`, not per-project — fixes apply across the fleet the moment Intent updates).

## Resume target — next session

No active steel thread. Optional follow-on, in order of return:

1. **Per-project canon refresh** for fleet members on stale Intent versions (Anvil v2.10.0, Lamplight v2.11.0, MeetZaya v2.10.0, MicroGPTEx v2.10.0, Conflab v2.11.0). Hygiene only — runner + rules already current via `$INTENT_HOME`. `intent claude upgrade` per project, single coherent commit per project.
2. **`/in-review` Elixir fleet sweep** — Tranche C of the post-v2.11.3 plan. Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab. Catalogue findings per project; surface to user before any remediation.
3. **Conflab pre-existing test findings** surfaced during the smoke (`IN-EX-TEST-001` / `005` / `007`, 7 hits across 5 test files) — worth folding into Conflab's own backlog. Not v2.11.3-introduced.
4. **Deferred v2.11.x backlog**: `intent claude upgrade` Phase-2 CLAUDE.md substitution audit; Homebrew tap; `scripts/release` v2 polish; `$N`-in-SKILL.md trap audit on remaining `in-*` skills; `docs/blog/_drafts/####-shell-critic-inception.md` blog draft (now has v2.10.1 + v2.11.3 + v2.11.4 as second / third / fourth dogfood datapoints).

## What landed this session (newest first)

- **v2.11.4 docs patch** — `intent/docs/critics.md` "Code locality" note added under "Headless runner": clarifies that the runner (`bin/intent_critic` + `critic_runner.sh`) and the canon rule library load from `$INTENT_HOME` (the Intent install on `$PATH`), not from each project's plugin tree. CHANGELOG, wip.md, restart.md refreshed.
- **Tranche A (Conflab smoke test)** — verified v2.11.3 fix in the canonical field witness. Gate clears on previously-misfiring patterns; gate still produces real signal on genuine violations (7 pre-existing findings on `TEST-001` / `005` / `007` across 5 test files, none of which are v2.11.3 regressions). Conflab tree untouched — no upgrade, no checkout, no commit, no `--no-verify` replay.

## Lessons from this session (top three)

- **Verify your architecture before you plan a fleet upgrade.** Tranche B (per-project upgrade prerequisite) was scoped on the assumption that each project carried its own runner copy. The actual architecture — runner + canon loaded from `$INTENT_HOME`, not per-project — collapses Tranche B into pure hygiene work. Trace the resolution path of whichever script the gate actually runs, end to end, before sizing fleet work.

- **A clean smoke is a smoke that finds something real.** The Conflab broad sweep returned genuine pre-existing findings. That's the right shape for "the gate works": misfiring patterns clear, and real signal still surfaces. A smoke that returns "exit 0, 0 findings everywhere" is indistinguishable from a smoke that did nothing.

- **Docs patches are legitimate.** v2.11.4 changes no behaviour but corrects shipped-as-broken docs (the runner-locality detail was missing). The patch / minor / major decision is semantic; "fixed shipped-as-broken docs" qualifies as a patch under the same framing as ST0039.

## Risks for post-cut

- Per-project canon copies (`intent/llm/RULES*.md`, `.claude/skills/`, `pre-commit.intent`) on the fleet are still on whatever Intent version each project last ran `intent claude upgrade` against. The runtime gate uses the central `$INTENT_HOME` copies, so gate behaviour is unaffected; but a developer reading their project's `intent/llm/RULES-elixir.md` may see proxy blocks that have since been stripped from canon. A per-project refresh sweep closes the gap.
- Conflab's surfaced `TEST-001` / `005` / `007` findings are genuine and pre-existing. They were hidden by gate _noise_, not gate silence — the over-broad proxies fired so loudly that real findings were lost in the false positives. Now that the noise is gone, the signal needs triage.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, restart.md (vanity metrics).
- Fail-forward: no backwards-compat shims; no deprecation stubs; migrations actively prune.
- Document first, code next, with a hard review gate after design.
- Pre-flight every canary: clean tree before applying.
- SKILL.md inline bash with `$N` positional fields gets mangled by the skill renderer. Use a script file invoked by path.
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes (including shipped-as-broken docs) are patches regardless of engineering scope.
