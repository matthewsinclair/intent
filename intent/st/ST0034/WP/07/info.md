---
verblock: "22 Apr 2026:v0.2: matts - Detailed plan"
wp_id: WP-07
title: "Critic subagent family"
scope: Large
status: Not Started
---

# WP-07: Critic subagent family

## Objective

Create four Critic subagents (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`) each with `code` and `test` modes. Each critic enumerates rules from agnostic + language packs, applies Detection heuristics against target files, and produces a stable machine-parseable violation report. Wire `in-review` stage-2 to dispatch to the right critic based on project language indicators.

## Context

WP07 is the main join point for WP03 (skill rationalisation) through WP06 (language rule packs). Without critics, the rules are documents with no enforcer. Critics are the consumers of the rule library.

The critic contract was locked in WP01. This WP implements it: prompt engineering for four subagents with uniform structure, fixture-based validation, and integration with `in-review`.

Each critic is a **thin orchestrator**: it reads rules at runtime, applies their Detection heuristics, and collects findings. No rule logic is embedded in the critic prompt; all logic lives in RULE.md files. Thin-Coordinator discipline applied to the critic family itself.

## Deliverables

### Four Critic subagents

```
intent/plugins/claude/subagents/
  critic-elixir/
    agent.md
    metadata.json
  critic-rust/
    agent.md
    metadata.json
  critic-swift/
    agent.md
    metadata.json
  critic-lua/
    agent.md
    metadata.json
```

### Global agents manifest update

- `intent/plugins/claude/subagents/.manifest/global-agents.json` — four new entries

### `in-review` dispatcher (coordinated with WP03)

- `intent/plugins/claude/skills/in-review/SKILL.md` stage-2 invocation snippets for each critic

### `.intent_critic.yml` schema

- `intent/docs/critics.md` (expanded from WP01 draft) documents the per-project config file
- Sample `.intent_critic.yml` with disabled rule list + severity threshold

### Critic contract docs (migrated from WP01 draft)

- `intent/docs/critics.md` — full contract reference: invocation, mode semantics, rule loading order, report format, config file

### Fixtures

```
tests/fixtures/critics/
  elixir/
    code/
      would-catch/
        sample.ex          # contains IN-EX-CODE-001 violation
      would-miss/
        clean.ex           # no violations
    test/
      would-catch/
        sample_test.exs    # contains IN-EX-TEST-001 violation
      would-miss/
        clean_test.exs
  rust/
    code/
      would-catch/
        sample.rs          # contains IN-RS-CODE-001 violation
      would-miss/
        clean.rs
    test/
      would-catch/
        sample_test.rs
      would-miss/
        clean_test.rs
  swift/
    ... (same structure)
  lua/
    ... (same structure)
```

### Manual verification plan

Not BATS (critics are Claude-side, not shell-side). Instead:

- `tests/fixtures/critics/<lang>/<mode>/<category>/*.md` with expected-violation cheat sheets
- Manual invocation: `Task(subagent_type="critic-<lang>", prompt="<mode> <fixture-path>")`
- Verify report mentions expected rule IDs

## Approach

1. **Design the agent prompt template.** One shared template, parameterised by language. Sections:
   - Role description ("You are a code critic for <language>...")
   - Mode handling ("Parse the invocation prompt to determine mode: `code` or `test`")
   - Rule loading ("Enumerate these directories and read each `RULE.md`: `rules/agnostic/*`, `rules/<lang>/<mode>/*`, `rules/<lang>/common/*` if present")
   - Detection workflow ("For each rule, apply its Detection heuristic to target files")
   - Report format (exact Markdown structure)
   - Per-project config ("Check for `.intent_critic.yml` in project root; apply disabled-rule list and severity threshold")
   - `diogenes` handoff (Elixir only, test mode)
   - Elixir-test-critic interop ("If elixir-test-critic plugin is installed, also load its rules; dedupe by `upstream_id`")

2. **Write `critic-elixir` first.** Most rules available (WP05 is largest pack). Iterate on prompt length, clarity, report format. Target: under 300 lines in agent.md.

3. **Clone for Rust/Swift/Lua.** Substitute language name, rule pack path, language-specific tooling hints (e.g. Cargo for Rust, xcodebuild for Swift).

4. **Write fixtures.** For each critic × mode × category, create one "would-catch" file with at least one known violation and one "clean" file with none.

5. **Populate global-agents.json.** Four entries with `name`, `description`, `category`, `version`.

6. **Update `in-review` stage-2.** Coordinate with WP03. Add precise invocation strings for each critic.

7. **Manual verification.** Invoke each critic against its fixtures; verify violations are reported or absent as expected. Document the procedure in `intent/docs/critics.md`.

8. **Document `.intent_critic.yml` format.**

   ```yaml
   disabled:
     - IN-EX-CODE-007 # moduledoc too noisy for this project
   severity_min: warning # show warning and above only
   ```

9. **MODULES.md registrations.**

## Acceptance Criteria

### Subagents

- [ ] Four subagent directories exist with `agent.md` + `metadata.json`
- [ ] Each `agent.md` frontmatter has: `name`, `description`, `tools` (Read, Grep, Bash at minimum)
- [ ] Each `agent.md` body follows the shared template (role, mode dispatch, rule loading, detection, report, config)
- [ ] Each critic's prompt stays under 300 lines (thin orchestrator)
- [ ] `global-agents.json` contains 4 new entries
- [ ] `intent claude subagents list` shows all 4 critics

### Mode dispatch

- [ ] `Task(subagent_type="critic-elixir", prompt="review lib/x.ex")` triggers code mode
- [ ] `Task(subagent_type="critic-elixir", prompt="test-check test/x_test.exs")` triggers test mode
- [ ] Ambiguous prompts fall back to code mode with warning

### Rule loading

- [ ] Each critic reads `rules/agnostic/*/RULE.md` as a first pass
- [ ] Each critic reads `rules/<lang>/<mode>/*/RULE.md`
- [ ] elixir-test-critic interop: if the upstream plugin is installed in `~/.claude/`, critic-elixir also loads upstream rules; dedupes by `upstream_id`
- [ ] Critic does not hard-fail if a rule file is malformed; logs a warning and continues

### Detection and reporting

- [ ] Report format is stable and machine-parseable:

  ```
  ## Critic Report: critic-<lang> <mode> <target>

  CRITICAL
  - <id> (<slug>) <file>:<line>
    <violation description>
    <suggested fix summary>

  WARNING
  - ...

  Summary: N critical, N warning, N recommendation, N style.
  Rules applied: N agnostic, N language-specific.
  ```

- [ ] Severity tiers respected: default shows CRITICAL and WARNING only; RECOMMENDATION and STYLE shown only when requested or configured
- [ ] Findings cite rule ID with slug in parentheses
- [ ] Findings include file:line and a short quoted snippet or description

### Per-project config

- [ ] Critic reads `.intent_critic.yml` from project root if present
- [ ] `disabled` list suppresses matching rule IDs
- [ ] `severity_min` filters output
- [ ] Config file absent → critic uses defaults (all rules, warning+ severity)

### `in-review` integration

- [ ] `in-review/SKILL.md` stage-2 dispatches to right critic by language indicator
- [ ] Dispatcher tested for all four languages

### Fixtures

- [ ] 4 languages × 2 modes × 2 categories (catch/miss) = 16 fixture directories under `tests/fixtures/critics/<lang>/{known-violating,known-clean}/`
- [ ] Each would-catch fixture contains at least one unambiguous rule violation
- [ ] Each would-miss fixture contains zero violations
- [ ] Each fixture directory has a `manifest.txt` listing expected rule-ID triggers (or explicitly "no triggers")
- [ ] Manual verification documented: each critic run on its fixture produces expected findings

### Tests to add

See `intent/st/ST0034/design.md` §Testing Strategy §WP07.

- [ ] `tests/unit/critic_dispatch.bats` — `in-review` stage-2 language detection via filesystem probes (`mix.exs` → elixir, `Cargo.toml` → rust, `Package.swift` → swift, `.lua` files → lua); verifies correct Critic is selected
- [ ] `tests/unit/critic_report_format.bats` — stable report shape across all four Critics (severity grouping, rule-ID citation format, suggested-fix line, summary line)
- [ ] `tests/unit/critic_config.bats` — `.intent_critic.yml` honoured (disabled list, severity_min); invalid config reported not silently ignored
- [ ] Optionally: `tests/unit/critic_fixture_smoke.bats` — loads each `tests/fixtures/critics/<lang>/known-violating/` file and checks the Critic's rule-loading logic would find the expected rule (structural test; does not run the Claude subagent)

### Tests to update

- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

### Documentation

- [ ] `intent/docs/critics.md` covers contract, invocation, rule loading order, report format, `.intent_critic.yml` schema, integration with `in-review`, `diogenes` handoff
- [ ] `.intent_critic.yml` sample file committed to `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`

## Dependencies

- **WP03** (rationalisation): `in-review` stage-2 must be ready to integrate.
- **WP04** (agnostic rules): critics read these as first pass.
- **WP05** (Elixir pack): critic-elixir's primary rule source.
- **WP06** (Rust/Swift/Lua packs): the other three critics' primary sources.

## Implementation Notes

### Agent prompt template (skeleton)

```markdown
---
name: critic-<lang>
description: Code critic for <Language>. Enforces Intent rule library against code and test files.
tools: Read, Grep, Bash
---

You are a Critic subagent specialised in <Language>. You enforce Intent's rule
library against code and test files. You do not write or modify code; you
identify violations and suggest fixes.

## Modes

Parse the invocation prompt for a mode keyword:

- "review <path>" or "<path>" → code mode
- "test-check <path>" → test mode

If ambiguous, default to code mode and note this in the report.

## Rule loading

Enumerate these directories and read each `RULE.md`:

1. `$INTENT_HOME/intent/plugins/claude/rules/agnostic/*/RULE.md` (agnostic first-pass)
2. `$INTENT_HOME/intent/plugins/claude/rules/<lang>/<mode>/*/RULE.md` (mode-specific)
3. Optional: if elixir-test-critic is installed, load its rules and dedupe by `upstream_id`

## Detection

For each loaded rule, apply its Detection heuristic to target files. Detection
may be:

- A grep pattern (use the Grep tool)
- A structural signal (use Read + pattern matching)
- A behavioural observation (harder; may require looking at surrounding code)

## Project config

Check for `.intent_critic.yml` in the project root. If present, apply:

- `disabled`: skip rules in the list
- `severity_min`: include only findings at or above this tier

## Report format

Output must be exactly this shape:

\`\`\`

## Critic Report: critic-<lang> <mode> <target>

CRITICAL

- <id> (<slug>) <file>:<line>
  <violation description>
  <suggested fix summary>

WARNING

- <id> (<slug>) <file>:<line>
  ...

RECOMMENDATION

- ...

STYLE

- ...

Summary: N critical, N warning, N recommendation, N style.
Rules applied: N agnostic, N language-specific.
\`\`\`

If no violations: emit the heading + "Summary: 0 critical, 0 warning, 0 recommendation, 0 style."

## Diogenes handoff (critic-elixir, test mode only)

If invoked in test mode and the file appears to lack a specification, you may
hand off to the `diogenes` subagent for Socratic test-spec generation. Do not
duplicate diogenes' work.
```

The `<lang>` token gets substituted per critic; Elixir gets the diogenes-handoff section, others skip it.

### Rule-loading performance

Reading 30-40 RULE.md files per invocation adds latency. Document this in `critics.md`. Optimisation paths deferred to a future ST:

- Index-based loading (use `rules/index.json` to decide which rules to load)
- Per-session rule caching
- Parallel reads

For v2.9.0, accept the latency. Critics are invoked from `in-review` or explicitly by the user; not per-keystroke.

### Report format rationale

Stable Markdown output enables:

- Easy human reading
- Simple post-processing (grep for `IN-EX-`)
- Possible future automation (auto-fix tooling reading the report, rejected for v2.9.0 per D5)

### Elixir-test-critic interop detection

```
If file exists: ~/.claude/plugins/elixir-test-critic/rules/ (or similar)
Then: also load rules from there; merge with Intent's elixir rules; dedupe by upstream_id.
```

Exact detection path depends on upstream's install layout; research during WP07.

### Diogenes handoff mechanic

In critic-elixir test mode, if the critic detects that a test file lacks a spec document (e.g. `test/x_test.spec.md` does not exist), the critic can emit a recommendation: "Run `diogenes` to generate a test spec first; then re-run critic-elixir test-check."

Critic does not call `diogenes` itself; it suggests in the report.

### Detection heuristic calibration

Rules' Detection sections say things like "grep: `assert is_struct`". Critic interprets this as a starting hint, not a definitive rule. Critic is expected to verify using surrounding context (e.g. grep match might be in a comment, not actual code).

## Risks and Edge Cases

### R6: Noisy reports

Default severity threshold is `warning`, filtering out recommendation and style. Users can override with `.intent_critic.yml`. Critic reports summary counts always; detail is gated.

### Critic prompt bloat

300-line cap. Any rule-specific logic must live in RULE.md, not the prompt. If a language needs special handling, it goes in language-specific `rules/<lang>/common/*/RULE.md` as meta-guidance, not in the prompt.

### False positives

Critics work on heuristics; false positives are inevitable. Mitigation: `.intent_critic.yml` disabled list; project-specific noise handled at project level, not in canon rules.

### Rule loading fails

Malformed RULE.md or disk I/O error. Critic logs warning, skips that rule, continues. Fail-forward: one broken rule doesn't kill the whole report.

### Concurrent critic invocations

Two critics running on different files simultaneously. No shared state in critic itself (each reads from disk). Concurrent readers of `.intent_critic.yml` is fine (no writes).

### elixir-test-critic install path shifts

Upstream changes its install location. Intent's detection fails silently (doesn't load upstream rules). Acceptable; document in `critics.md`.

### in-review dispatch ambiguity

Polyglot project (mix.exs + Cargo.toml). `in-review` prompts user per WP03. Critic itself does not handle language detection.

## Testing Approach

### Fixture-based manual verification

For each critic × mode × category:

1. Create a would-catch fixture with at least one documented violation.
2. Invoke the critic: `Task(subagent_type="critic-<lang>", prompt="<mode> <path>")`.
3. Verify the report mentions the expected rule ID.
4. Create a would-miss fixture with no violations.
5. Invoke the critic; verify summary counts are zero.

Document the procedure in `critics.md` under "Verification".

### Schema checks

- `intent claude subagents list` shows all 4 critics.
- `intent claude subagents install critic-elixir` works.

### Report format consistency

- Run each critic on its would-catch fixture; capture report.
- Diff against expected-format template. Formatting must match.

### No mistaken auto-fix

- Invoke critic and inspect its output; confirm no files have been modified.
- Tools list includes Read, Grep, Bash but critic prompt explicitly forbids Edit/Write.

### `in-review` dispatcher

- Create sandbox projects with only mix.exs, only Cargo.toml, only Package.swift, only .lua. Invoke `in-review`; verify correct critic is chosen.
- Polyglot sandbox (mix.exs + Cargo.toml): verify user is prompted.

## Size and Estimate

- **Size**: L (Large, 4-6 sessions).
- **Session 1**: Prompt template design; critic-elixir v1.
- **Session 2**: critic-elixir iteration; fixture authoring for Elixir.
- **Session 3**: critic-rust + fixtures.
- **Session 4**: critic-swift + fixtures.
- **Session 5**: critic-lua + fixtures.
- **Session 6**: `in-review` integration, `.intent_critic.yml` schema, docs pass, manual verification across all 4.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] Four critics function against fixtures
- [ ] Reports format-consistent across all four
- [ ] `in-review` dispatcher verified for all four languages + polyglot case
- [ ] `.intent_critic.yml` schema documented; sample committed
- [ ] `critics.md` is the authoritative reference
- [ ] No rule-specific logic in critic prompts (thin orchestrator invariant)
- [ ] Registered in MODULES.md
- [ ] global-agents.json updated
