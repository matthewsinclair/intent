# Claude Code Session Restart -- narrative state

## Current state (2026-04-29, end of session -- v2.11.3 ready to cut)

ST0039 implementation complete. The pre-commit critic gate now refuses Greppable proxies the headless runner cannot honour faithfully (pipes, `xargs`, `grep -L`, `-v` filters, `-B/-A` context, awk state machines) with a once-per-rule stderr diagnostic, instead of silently degrading to "first quoted regex from a multi-line block." Eight Elixir rules whose detection cannot be expressed as a single-file regex have had their proxy blocks stripped — the rules themselves still apply via `/in-review` (the `critic-elixir` LLM subagent does the body confirmation). One rule (IN-EX-CODE-004) had its counter line surgically removed; the legitimate `error -> error` forwarder detector remains.

BATS suite green. `intent doctor` clean. `scripts/release --dry-run --patch` previews 2.11.2 → 2.11.3 cleanly. Working tree carries the staged diff for v2.11.3.

## What this session did (chronological)

### 1. Field bug ingest

User pasted a defect report (`~/Downloads/intent-v2.11.2-critic-gate-false-positives.md`) from a Conflab session post-upgrade-to-v2.11.0: the pre-commit gate emitted 23 WARNINGs in a 3-file diff, on lines unrelated to the change. Three commits had to ship `--no-verify`. Two distinct misfires:

- **IN-EX-CODE-004** (with-for-railway): every `case ... do` line flagged. The rule's body excludes single-step `case` and explicitly marks the proxy as non-authoritative.
- **IN-EX-TEST-003** (async-by-default): the _compliant_ `use ExUnit.Case, async: true` line flagged. The documented proxy uses inverse semantics (`grep -L | xargs grep -l`) the runner could not honour.

User direction: "fix this PROPERLY, no half-measures."

### 2. Phase 1 exploration (three parallel Explore agents)

Mapped `bin/intent_critic` + `intent/plugins/claude/lib/critic_runner.sh` end-to-end. Confirmed the parser bug: `critic_pattern_from_grep_command` extracted only the first quoted argument from a proxy bash block and ran `grep -nE` with it. Silently inverted `grep -L`, dropped `xargs grep -l` filters, dropped `| grep -v` filters, treated counter heuristics (`| wc -l`) as detectors, ran only the first line of multi-line proxies. Audited every RULE.md across canon: proxy-bearing rules are all Elixir; the rest are body-only or mechanical-clean.

### 3. Phase 2 design validation (Plan agent)

Plan agent refined the recommended approach: drop the proposed `proxy_authoritative` schema field — ST0038 already established "strip the over-broad proxy" as the disposition. Instead: tighten the runner with a strict-proxy predicate + multi-pattern union, refuse complex proxies with a stderr diagnostic, and strip proxies from rules whose detection genuinely cannot be expressed as a single-file regex.

### 4. Plan-mode workflow

User correction during plan: "Version should be: v2.11.3. This is a patch to work that should have been working, but wasn't." Frame the work by _kind_ not by _engineering size_. Saved to memory: `feedback_patch_vs_minor_framing.md`. Plan accepted.

### 5. Implementation (WP01 + WP02)

ST0039 created via `intent st new`, populated `info.md` and `design.md` first (document-before-code).

**WP01** (runner): replaced `critic_pattern_from_grep_command` with `critic_proxy_is_simple` predicate + `critic_patterns_from_grep_block` walker; updated `critic_apply_rule` to run a union of accepted patterns and dedupe `(line, content)`; once-per-rule stderr diagnostic for refused lines.

**WP02** (rule cleanup): full strip from 8 rules (IN-EX-TEST-003, IN-EX-CODE-003, IN-EX-LV-001, IN-EX-LV-003, IN-EX-PHX-001, IN-EX-ASH-001, IN-EX-ASH-002, IN-EX-TEST-004) + surgical edit on IN-EX-CODE-004 (counter line removed, `error -> error` detector kept). Three additional rules (TEST-005/006/007) had their `--include='*_test.exs'` flag dropped (the `applies_to` field already scopes them).

### 6. Tests

New `tests/unit/critic_runner_proxies.bats` covering predicate cases, multi-pattern union, stripped-proxy regressions, false-positive reproductions, and positive controls. Extensions to `tests/unit/intent_critic.bats` (stderr/JSON separation via `run --separate-stderr` plus `bats_require_minimum_version 1.5.0`). Extensions to `tests/unit/pre_commit_hook.bats` (single-step `case` and compliant async test no longer block). BATS suite green.

### 7. Docs + version

`intent/docs/critics.md` "Mechanical subset only" paragraph rewritten to document the strict-proxy contract. `CHANGELOG.md` `[2.11.3]` section written. `wip.md` and `restart.md` updated. VERSION stays at 2.11.2 in tree; `scripts/release --patch` will bump.

## Resume target -- v2.11.3 cut

User asked for a clean check-in before publishing. Workflow:

1. `git add` the staged diff (runner change, 12 RULE.md edits, 3 BATS files, docs, CHANGELOG, ST0039 directory, wip/restart).
2. `git commit` with a single coherent message; pre-commit hook runs critic-shell on the runner change.
3. `scripts/release --patch` to cut v2.11.3.

## Lessons from this session

- **The runner refuses what it cannot reproduce.** `grep -L`, `xargs grep -l`, `| grep -v`, `-B/-A` context, awk state machines — none of these survive single-file headless invocation. Honouring them in a runner means encoding shell semantics that were never the runner's job. ST0039's contract: accept only what a single grep invocation can detect; refuse the rest with a loud stderr note.

- **Severity is in the rule, not the runner.** ST0039 deliberately did not introduce a `proxy_authoritative` frontmatter field. Severity is per-rule and stays in YAML frontmatter; the proxy's authoritativeness is structural (presence/absence of a Greppable proxy block) — same disposition ST0038 took with IN-EX-CODE-002 and IN-EX-CODE-006.

- **stderr is not stdout.** The runner's diagnostics belong on stderr; JSON / text findings belong on stdout. Tests that capture both (BATS `run` default) need `run --separate-stderr` (BATS 1.5+, requires `bats_require_minimum_version`).

- **The patch / minor decision is semantic, not size.** A multi-file runner refactor with rule-library cleanup ships as a patch when the work is correcting shipped-as-broken behaviour. Engineering scope doesn't gate the version bump.

## Risks for post-cut

- Conflab fleet member: re-run the original failing commits without `--no-verify` to confirm the gate clears. Repro fixtures (single-step `case`, compliant async test) verified clean during this session against a temp file; the real-world Conflab paths should be smoke-tested next session.
- Other fleet members on `languages: ["elixir"]` may carry committed code that _was_ hit by the over-broad proxies and is now silently un-flagged at the gate level. Those rules still apply via `/in-review` — but a "first run after upgrade" sweep with `/in-review` in any actively-edited Elixir project (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab) is worth queuing.

## Session conventions (carry forward)

- T-shirt sizing only (XS / S / M / L / XL / XXL).
- Compact/refresh at ~200-250k tokens.
- ALWAYS use `intent` CLI for ST/WP operations.
- NEVER manually wrap lines in markdown.
- NO Claude attribution in commits.
- NEVER report test/skill/subagent counts in release notes, CHANGELOG, wip.md, restart.md (vanity metrics).
- Fail-forward: no backwards-compat shims; no deprecation stubs; migrations actively prune.
- Document first, code next, with a hard review gate after design.
- Pre-flight: clean tree before applying any canary.
- SKILL.md inline bash with `$N` positional fields gets mangled by the skill renderer. Use a script file invoked by path.
- **NEW (2026-04-29)**: Patch / minor / major decision is semantic, not size. Shipped-as-broken fixes are patches regardless of engineering scope.
