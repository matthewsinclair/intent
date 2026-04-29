---
verblock: "29 Apr 2026:v0.1: matts - Initial scope and design pointer"
intent_version: 2.11.2
status: WIP
slug: critic-gate-strict-proxies-and-rule-library-cleanup
created: 20260429
completed:
---

# ST0039: Critic gate: strict proxies and rule-library cleanup

## Objective

Stop the headless pre-commit critic gate from emitting WARNING findings derived from greppable proxies the runner cannot honour. Make the runner refuse complex proxies with a loud stderr diagnostic instead of silently degrading, and strip the proxies from rules whose detection cannot be expressed as a single-file regex.

## Context

Field report `~/Downloads/intent-v2.11.2-critic-gate-false-positives.md` from a Conflab session post-upgrade-to-v2.11.0: the pre-commit gate emitted 23 WARNINGs on a 3-file diff, on lines unrelated to the change. Three commits had to ship `--no-verify`. Two rules misfired:

- **IN-EX-CODE-004** (`with-for-railway`): every `case ... do` line flagged. Rule body explicitly excludes single-step `case`. 22 false hits across two LiveView files.
- **IN-EX-TEST-003** (`async-by-default`): the _compliant_ line `use ExUnit.Case, async: true` flagged. The documented proxy uses `grep -L | xargs grep -l` (find files lacking the literal); the runner extracts the first quoted argument and runs `grep -nE` for it forward, matching the compliant line.

ST0038 (v2.11.0) addressed a similar class of issue by stripping the greppable proxy from IN-EX-CODE-002 and IN-EX-CODE-006 — both rules whose detection could not be expressed as a single-file regex. ST0039 applies the same disposition consistently across the remaining over-broad proxies and tightens the runner so future rule authors cannot accidentally write proxies the runner will silently degrade.

User direction: "fix this PROPERLY, no half-measures."

## Diagnosis

Three classes of bug in `intent/plugins/claude/lib/critic_runner.sh`:

1. **`critic_pattern_from_grep_command` is naive.** It extracts the first quoted argument from a proxy bash block and runs `grep -nE` with it. This silently inverts `grep -L` semantics, drops `xargs grep -l` second-stage filters, drops `| grep -v <filter>` filters, treats counter heuristics (`| wc -l`) as detectors, and runs only the first line of multi-line proxy blocks.
2. **The "Critic confirms by reading body" step the rules document is bypassed.** The runner is mechanical-only by design (cannot shell out to an LLM in a pre-commit hook). 17 of 48 rules carry proxies; for the proxy+body majority, the rule's actual structural signal requires LLM body inspection. The mechanical layer treats the proxy regex as authoritative.
3. **9 of the 17 proxies are not headless-runnable.** Either inverted (IN-EX-TEST-003, IN-EX-ASH-002), counter-only (IN-EX-CODE-004), filter-pipe-dependent (IN-EX-CODE-003, IN-EX-LV-001, IN-EX-TEST-004, IN-EX-TEST-007), awk-based (IN-EX-LV-003, IN-EX-PHX-001), or multi-line composite (IN-EX-ASH-001).

## Scope

T-shirt: **M**. Two work packages, ships v2.11.3 (patch — corrects shipped-as-broken behaviour, no new feature surface).

### WP01 — runner correctness

Replace the parser in `intent/plugins/claude/lib/critic_runner.sh`:

- Add `critic_proxy_is_simple <grep_line>` predicate. Accepts `grep [-rn][E]?` with optional `--include=...`, single quoted pattern, path args. Rejects pipes, `-L`, `-v`, `-B/-A`, `xargs`, `wc`, `awk`, `sed`, multiple commands per line.
- Replace `critic_pattern_from_grep_command` with `critic_patterns_from_grep_block`: walks the block line-by-line, emits one pattern per simple grep line, emits stderr `note: skipping <rule_id> (proxy not headless-runnable)` per refused line.
- In `critic_apply_rule`, loop over the union of patterns; dedupe findings on `(file, line)`.
- Keep the runner LLM-free, headless, bash 3.x. 2-space indentation. No `declare -A`.

### WP02 — rule-library cleanup

Strip the Greppable proxy block from these 9 RULE.md files. Detection prose stays. Add the ST0038-style note ("No greppable proxy is authoritative for this rule ... Apply via the LLM-driven `critic-elixir` subagent during `/in-review`") to each:

- `intent/plugins/claude/rules/elixir/code/with-for-railway/RULE.md` (IN-EX-CODE-004)
- `intent/plugins/claude/rules/elixir/test/async-by-default/RULE.md` (IN-EX-TEST-003)
- `intent/plugins/claude/rules/elixir/code/impl-true-on-callbacks/RULE.md` (IN-EX-CODE-003)
- `intent/plugins/claude/rules/elixir/lv/two-phase-mount/RULE.md` (IN-EX-LV-001)
- `intent/plugins/claude/rules/elixir/lv/thin-liveviews/RULE.md` (IN-EX-LV-003)
- `intent/plugins/claude/rules/elixir/phoenix/thin-controllers/RULE.md` (IN-EX-PHX-001)
- `intent/plugins/claude/rules/elixir/ash/code-interfaces-only/RULE.md` (IN-EX-ASH-001)
- `intent/plugins/claude/rules/elixir/ash/actor-on-query/RULE.md` (IN-EX-ASH-002)
- `intent/plugins/claude/rules/elixir/test/start-supervised/RULE.md` (IN-EX-TEST-004)

Keep proxies for: IN-EX-TEST-001, IN-EX-TEST-002, IN-EX-TEST-005, IN-EX-LV-002, IN-EX-CODE-001 (single grep, authoritative), IN-EX-CODE-005, IN-EX-TEST-006, IN-EX-TEST-007 (multi-pattern union, now unioned correctly by WP01).

### Tests

New `tests/unit/critic_runner_proxies.bats`:

- `critic_proxy_is_simple` predicate: positive + negative cases (`grep -L`, `xargs grep -l`, `| grep -v`, `wc`, `awk`).
- Multi-pattern union on IN-EX-CODE-005 fixture.
- Stripped-proxy regression: assert the 9 stripped rules carry no `Greppable proxy` block.
- IN-EX-CODE-004 false-positive reproduction: single-step `case ... do ... end` produces no finding.
- IN-EX-TEST-003 false-positive reproduction: file with `use ExUnit.Case, async: true` produces no finding.
- IN-EX-TEST-002 positive control: `Process.sleep(` in `*_test.exs` still emits at `critical`.

Extend `tests/unit/intent_critic.bats`: stderr diagnostic + `--format json` validity. Extend `tests/unit/pre_commit_hook.bats`: single-step `case` does not block.

### Out of scope

- No `proxy_authoritative` frontmatter field. ST0038 established "strip the proxy" as the disposition.
- No per-project migration. Rules ship with Intent.
- No back-compat shim for the old "first quoted regex" behaviour. Loud > silent.
- No second runner mode (strict / lenient). One contract.
- Blog drafts: capture as follow-up.

## Related Steel Threads

- ST0038 — established the "strip the proxy from rules whose detection cannot be expressed as a single-file regex" disposition. ST0039 extends that disposition to the remaining 9 over-broad proxies and tightens the runner contract.
- ST0034 — original critic family (v2.9.0). The greppable proxies were authored as headless-fast-path companions to the LLM critic; ST0038 + ST0039 together complete the contract on which proxy forms are safe to ship.

## Context for LLM

Read `design.md` for the runner contract and the new helper signatures. The scope above is the plan; design.md captures the implementation specifics.

### How to update this document

1. Update status as work progresses (use `intent st` commands).
2. Mark completion date when finished.
