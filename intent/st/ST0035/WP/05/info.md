---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-05
title: "Implement bin/intent_critic headless critic runner"
scope: Large
status: Done
---

# WP-05: Implement bin/intent_critic headless critic runner

## Objective

Implement `bin/intent_critic` — a bash-native headless critic runner that parses rule frontmatter + Detection heuristics directly (no LLM round-trip) and emits severity-grouped findings matching the critic-\* subagent report format. Enables fast local pre-commit enforcement without invoking an LLM. This is the biggest single engineering WP in ST0035.

## Context

Today, rule enforcement happens via `Task(subagent_type="critic-<lang>")` — an LLM subagent reads rules from `intent/plugins/claude/rules/`, applies Detection heuristics, and reports. That works for `/in-review` stage 2 but is too slow and too costly for pre-commit (which fires on every commit).

The rules themselves are mechanical: YAML frontmatter + Markdown sections including a `Detection` heuristic that describes a deterministic pattern match (regex, structural, content-based). For the mechanical subset — which covers the majority of rules — no LLM is required. A bash runner can parse the same rule files and apply the same Detection logic.

WP05 ports that Detection logic to bash. The LLM subagents remain available for richer review; they produce the identical output format. `bin/intent_critic` becomes the canonical pre-commit gate (WP06) and the CI gate (future), with subagents reserved for interactive review.

## Deliverables

1. **Main binary** at `bin/intent_critic`. CLI surface:

   ```
   intent critic <lang> [--files <path> ...] [--staged] [--severity-min <level>] [--format text|json] [--rules <dir>]
   ```

   - `<lang>`: one of `elixir`, `rust`, `swift`, `lua`, `shell`.
   - `--files`: explicit file list. Default: entire project (if no files + no --staged).
   - `--staged`: read from `git diff --cached --name-only` (pre-commit use).
   - `--severity-min`: `critical`, `warning`, `recommendation`, `style`. Default: `warning`.
   - `--format`: `text` (default, human-readable severity-grouped) or `json` (machine-readable).
   - `--rules`: alternative rules directory (default: `intent/plugins/claude/rules/` discovered from project root).
   - Exit codes: `0` no findings at or above threshold; `1` findings at or above threshold; `2` invocation error.

2. **Shared library** at `intent/plugins/claude/lib/critic_runner.sh` containing:
   - `critic_load_rules(<lang>, <rules_dir>)` — reads rule frontmatter + Detection heuristics from agnostic/ + language-specific/ + any extension rules.
   - `critic_apply_rule(<rule>, <file>)` — applies a single rule's Detection to a single file, emits findings.
   - `critic_format_report(<findings...>, <format>)` — emits severity-grouped text or JSON.
   - `critic_resolve_project_root()` — cwd-resilient discovery of `.intent/config.json`.

3. **Rule parser** (sub-component of critic_runner.sh):
   - Parses YAML frontmatter using bash + sed (no YAML library — Intent is bash 3.x compatible).
   - Extracts Detection section (fenced regex patterns or structural checks).
   - For structural checks (e.g., "function X must exist"), use grep + wc.
   - For regex patterns, use grep -P where available, fall back to grep -E.

4. **BATS test suite** at `tests/test_intent_critic.bats`:
   - Known-bad fixture file → expected findings output.
   - Known-good fixture file → no findings output.
   - Exit code contract tests.
   - Format switch tests (text vs json).
   - `--staged` mode test (requires a git fixture).
   - Severity threshold tests.
   - Rule discovery tests (agnostic vs language-specific vs extension).

5. **Test fixtures** at `tests/fixtures/critic/`:
   - `<lang>/good/` and `<lang>/bad/` dirs for each of the 5 languages.
   - Each rule that has runnable examples in `rules/<lang>/` gets a corresponding `good_test.*` / `bad_test.*` fixture (reuse what already exists).

6. **Acceptance parity test**: same file + same rules — `bin/intent_critic elixir <file>` and `Task(subagent_type="critic-elixir")` produce identical findings lists. Implemented as a manual verification step recorded in as-built (fully automated parity test is out of scope for v2.9.1; documented as follow-up).

7. **Documentation**: `intent/docs/pre-commit-hook.md` (WP06 authors) references this binary. `intent/docs/critics.md` gets an "also available as bin/intent_critic" note.

8. **MODULES.md registration** for the new binary and library.

## Approach

### Phase 1: Rule parser (bash-native YAML frontmatter + Detection extraction)

1. Study rule file structure: `intent/plugins/claude/rules/elixir/code/pattern-matching/RULE.md` as exemplar.
2. Extract frontmatter fields needed for triage: `id`, `severity`, `applies_when`, `does_not_apply_when`, `categories`, `concretised_by` (for agnostic rules).
3. Extract Detection section: what structural / regex patterns count as a match.
4. Build `critic_parse_rule()` that returns a structured bash associative-array-esque dict via env-var expansion.

### Phase 2: Detection runner

5. Build `critic_apply_rule()` that takes a parsed rule + a file path and emits 0 or more findings.
6. Finding shape: `{severity, rule_id, file, line, excerpt, message}`. Text format:
   ```
   [CRITICAL] IN-EX-TEST-001 at lib/foo.ex:42
     Shape assertion without a value check. Use pattern-match against expected shape + values.
     > assert is_list(result)
   ```
7. JSON format: same fields, one finding per object, wrapped in an array.

### Phase 3: Rule discovery + layering

8. Discover rules: agnostic pack → language pack → extension packs (`~/.intent/ext/*/rules/`). Mirror the subagent's discovery order.
9. Handle `concretised_by:` — agnostic rules defer to language-specific ones; don't double-report.
10. Respect `.intent_critic.yml` (WP07): disabled rule IDs are filtered out; severity threshold applied.

### Phase 4: CLI surface

11. Build the main `bin/intent_critic` script. Arg parsing follows Intent's existing pattern in `bin/intent_*`.
12. `--staged` mode: `git diff --cached --name-only`, filter by language extension.
13. Exit code contract.

### Phase 5: Tests

14. Author BATS test suite.
15. Author fixtures.
16. Run `tests/run_tests.sh`; all tests green.

### Phase 6: Parity verification

17. For each of the 5 languages, pick a known-bad file. Run `bin/intent_critic <lang> <file>` and `Task(subagent_type="critic-<lang>", prompt="review <file>")`. Diff outputs; document any divergence as a follow-up ticket (expected: minor ordering differences only).

### Phase 7: Integration

18. Register in MODULES.md.
19. Update `intent/docs/critics.md` with the "headless runner" note.
20. Commit.

## Acceptance Criteria

- [ ] `bin/intent_critic` exists and is executable.
- [ ] `bin/intent_critic --help` shows full CLI surface.
- [ ] `bin/intent_critic elixir tests/fixtures/critic/elixir/bad/` produces expected findings (documented in fixtures).
- [ ] `bin/intent_critic elixir tests/fixtures/critic/elixir/good/` produces no findings.
- [ ] `bin/intent_critic shell bin/intent_critic` (self-critique) produces ≤ 0 CRITICAL findings (the runner must not itself violate shell rules critically).
- [ ] `--format json` produces valid JSON (`jq . < output` exits 0).
- [ ] `--severity-min critical` filters out warnings and below.
- [ ] `--staged` reads from git staging area correctly.
- [ ] Exit codes: 0 on no findings-above-threshold, 1 on findings, 2 on invocation error.
- [ ] BATS tests pass in CI.
- [ ] Parity verification manually performed for each of 5 languages; divergences (if any) documented in `as-built`.
- [ ] MODULES.md registers `bin/intent_critic` and `intent/plugins/claude/lib/critic_runner.sh`.
- [ ] `intent/docs/critics.md` updated.
- [ ] Runs in < 2 seconds on a single file for any language (performance budget).
- [ ] Runs in < 10 seconds on a 100-file project.

### Tests to add

- **BATS suite**: ≥ 20 tests covering each CLI flag, each language, fixture pass/fail, exit codes, format switch, staged mode, severity threshold.

### Tests to update

- `tests/run_tests.sh` — no update needed (auto-discovers new BATS files).

## Dependencies

- **Blocks**: WP06 (pre-commit hook invokes this binary), WP07 (`.intent_critic.yml` consumed by this binary), WP11 (upgrade installs this binary into projects via `intent critic` dispatch).
- **Blocked by**: None. WP05 can start any time after Phase 0 approval.

## Implementation Notes

- **Bash 3.x constraint**: macOS default bash is 3.2. No `declare -A`. Use env-var prefix patterns or line-delimited pseudo-arrays.
- **YAML parsing in bash**: exists via `awk` / `sed`. Intent already has a pattern somewhere (check `bin/intent_helpers` for any existing YAML extraction). Reuse if present; otherwise keep it minimal (support only the frontmatter fields we need).
- **Rule category handling**: agnostic rules with `concretised_by:` defer to their language-specific implementations. The runner must know not to double-report the agnostic rule when a language-specific one fires on the same finding.
- **Detection heuristic flexibility**: some rules describe Detection in prose ("any function with more than 50 lines"); these are hard to mechanise in bash. Ship the runner supporting the mechanical subset (regex/structural) and emit a WARN for rules whose Detection can't be bash-applied ("advisory mode — LLM subagent required for full check"). Document this clearly.
- **Performance**: avoid spawning bash subshells per rule × per file. Batch where possible. Cache parsed rules once per run.
- **Extension rules**: at this WP, just support `~/.intent/ext/*/rules/<lang>/`. Shadowing-by-rule-id (extension wins) matches the subagent contract.
- **Cwd resilience**: follow the `INTENT_ORIG_CWD` pattern from ST0033; don't require user to be at project root.

## Risks and Edge Cases

- **Risk**: Bash regex doesn't support some patterns the subagent handles. **Mitigation**: document the mechanical subset; flag non-mechanical rules as advisory; recommend `/in-review` stage 2 for full coverage.
- **Risk**: Output format diverges from subagent output. **Mitigation**: lock the format spec in `intent/docs/critics.md` (already exists — just extend); unit-test the exact format in BATS.
- **Risk**: Performance unacceptable on large projects. **Mitigation**: performance budget in acceptance criteria; if exceeded, investigate subshell spawning. Last-resort: a `--parallel` flag.
- **Risk**: False positives from bash-native detection. **Mitigation**: rule-level disable via `.intent_critic.yml` (WP07); severity threshold lets users tune.
- **Edge**: Windows / PowerShell users. **Out of scope** — Intent is already bash 3.x POSIX. Document.
- **Edge**: Files with non-UTF-8 content. **Mitigation**: grep handles binary; skip binary files (use `file --mime-encoding`).
- **Edge**: Empty staging area in `--staged` mode. Exit 0, emit "No staged files" notice.

## Verification Steps

1. `bin/intent_critic --help` — CLI surface matches spec.
2. `bats tests/test_intent_critic.bats` — all green.
3. Run against a real Intent-managed Elixir project (e.g., Conflab); confirm plausible findings.
4. Run against Intent's own bash (`bin/intent_critic shell bin/intent*`); document findings baseline in `intent/st/ST0035/WP/05/parity-report.md`.
5. Parity verification: same file, same rules, `bin/intent_critic` vs `Task(subagent_type="critic-<lang>")` — diff outputs for each of 5 languages.
6. Performance: `time bin/intent_critic elixir lib/` on a medium Elixir project — confirm < 10s.
7. MODULES.md audit: both new modules listed.

## Size and Estimate

- **Size**: L (Large). 4–6 sessions. This is the biggest single engineering WP in ST0035.
- Session 1: Rule parser (frontmatter extraction).
- Session 2: Detection runner for Elixir (exemplar language).
- Session 3: Extend to Rust/Swift/Lua/Shell (ports of the Elixir machinery with language-specific tweaks).
- Session 4: CLI surface, `--staged`, `--format json`, exit codes.
- Session 5: BATS suite, fixtures.
- Session 6: Parity verification, MODULES.md, docs, commit.

## Exit Checklist

- [ ] All acceptance criteria met.
- [ ] BATS suite passes.
- [ ] Parity verification documented per language.
- [ ] Performance budget met.
- [ ] `bin/intent_critic shell` on Intent's own bash runs clean (zero critical/warning findings — or documented exemptions in `.intent_critic.yml`).
- [ ] MODULES.md, critics.md updated.
- [ ] Committed cleanly.
