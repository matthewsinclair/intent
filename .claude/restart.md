# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Reads restart files + CLAUDE.md + MODULES.md, surveys steel threads, loads `/in-essentials` and `/in-standards`, releases the UserPromptSubmit gate.
2. **Verify the working tree.** `git status` should be clean if v2.11.3 has been cut. `git log --oneline -5` should show the v2.11.3 release commit at the top once shipped (or the ST0039 implementation commit if the cut hasn't happened yet).
3. **Read `intent/wip.md` and `intent/restart.md`.** The wip "Current State" line is the operative status; the restart "Resume target" section says what to do next.

## State (2026-04-29, end of session — ST0039 implementation done; v2.11.3 ready or just cut)

ST0039 implementation complete. The pre-commit critic gate now refuses Greppable proxies the headless runner cannot honour faithfully (pipes, `xargs`, `grep -L`, `-v` filters, `-B/-A` context, awk state machines) with a once-per-rule stderr diagnostic instead of silently degrading. Eight Elixir rules whose detection cannot be expressed as a single-file regex have had their proxy blocks stripped; the rules apply via `/in-review` only. IN-EX-CODE-004 had its counter line surgically removed, keeping the legitimate `error -> error` forwarder detector. BATS suite green. `intent doctor` clean.

Defect fix to behaviour shipped as broken in v2.11.0; ships as v2.11.3 (patch).

## Resume target — verify v2.11.3 ship state

If v2.11.3 has not yet shipped:

1. `git status` — confirm working tree carries the staged ST0039 diff.
2. `scripts/release --dry-run --patch` — verify the bump path 2.11.2 → 2.11.3.
3. `scripts/release --patch` — cut the release.

If v2.11.3 has shipped:

1. Smoke-test the fix in a downstream Elixir project (Conflab is the canonical witness) by re-running one of the originally-failing commits without `--no-verify`. Expected: gate clears.
2. Queue a `/in-review` sweep across actively-edited Elixir fleet members (Anvil, Lamplight, MeetZaya, MicroGPTEx) — proxies that the gate over-fired on are now stripped, but the rules themselves still apply via the LLM critic-elixir subagent. Code that _should have been_ flagged but wasn't (because the gate was over-firing on unrelated lines) may surface during these sweeps.

## What landed this session (newest first)

- **ST0039 / v2.11.3** — strict-proxy contract in `intent/plugins/claude/lib/critic_runner.sh`. New `critic_proxy_is_simple` predicate + `critic_patterns_from_grep_block` walker; `critic_pattern_from_grep_command` deleted (no back-compat shim). Greppable proxy stripped from 8 RULE.md files (IN-EX-TEST-003, IN-EX-CODE-003, IN-EX-LV-001, IN-EX-LV-003, IN-EX-PHX-001, IN-EX-ASH-001, IN-EX-ASH-002, IN-EX-TEST-004) and surgically edited in IN-EX-CODE-004. New `tests/unit/critic_runner_proxies.bats` plus extensions to `intent_critic.bats` and `pre_commit_hook.bats`.
- **Field bug ingest** — `~/Downloads/intent-v2.11.2-critic-gate-false-positives.md` (Conflab session report; 23 false-positive WARNINGs in a 3-file diff; three commits required `--no-verify`).

## Lessons from this session (top three)

- **The runner refuses what it cannot reproduce.** ST0039's contract: the headless mechanical critic accepts only proxies of the form `grep [-rnE|--include=...] '<pattern>' [<path>...]`. Anything else (pipes, `xargs`, `grep -L`, `grep -v` filters, `-B/-A` context, awk) is refused with a stderr diagnostic. Loud > silent.

- **Prose-level discomfort can surface system-level regressions.** The bug report came from a developer who noticed the gate was over-firing; tracing it back surfaced two distinct runner bugs (silent first-quoted-arg extraction, no honouring of `-L` / `xargs grep -l` / pipeline filters) plus eight rules whose proxies were never expressible in single-file headless form.

- **The patch / minor decision is semantic, not size.** A multi-file runner refactor with rule-library cleanup ships as a patch when the work is correcting shipped-as-broken behaviour. Engineering scope doesn't gate the version bump.

## Risks for post-cut

- Conflab and other Elixir fleet members may carry committed code that _was_ hit by the over-broad proxies and is now silently un-flagged at the headless gate level. Those rules still apply via `/in-review`, but the gate no longer pre-emptively catches them. A "first-run-after-upgrade" `/in-review` sweep on Anvil / Lamplight / MeetZaya / MicroGPTEx / Conflab is worth queuing.
- The strict-proxy predicate accepts only a narrow grammar. Future rule authors writing a proxy must follow the grammar or accept that their rule is LLM-only via `/in-review`. The stderr diagnostic is the feedback loop.

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
- Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes are patches regardless of engineering scope.
