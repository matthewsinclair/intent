# Critic Subagent Contract

This document is the contract that every Critic subagent in Intent must satisfy. WP07 implements four concrete Critics (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`) and WP12 adds a fifth (`critic-shell`) — all share the contract defined here. The `in-review` skill's stage-2 dispatches to one of these Critics based on project language indicators.

Draft lives here under `_schema/` for WP01. At WP10 this content migrates to `intent/docs/critics.md` as the user-facing reference; the copy here remains as the canonical schema-side definition.

## What a Critic is

A Critic subagent is a **thin orchestrator** that:

1. Reads the rule library for its language at invocation time.
2. Applies each rule's Detection heuristic to the target file(s).
3. Produces a stable, machine-parseable violation report grouped by severity.

A Critic is **not** a fixer, generator, or editor. It reports; it does not modify.

A Critic's prompt (the `agent.md` body) contains only orchestration logic: mode dispatch, rule loading order, report format. No rule content lives in the Critic prompt — that's in RULE.md files.

## Identity

Each Critic has:

- Name: `critic-<lang>` where `<lang>` matches the language segment of rule IDs (`elixir`, `rust`, `swift`, `lua`, `shell`).
- Language code match: `EX` → `critic-elixir`, `RS` → `critic-rust`, `SW` → `critic-swift`, `LU` → `critic-lua`, `SH` → `critic-shell`.
- Tool loadout (declared in `agent.md` frontmatter): `Read`, `Grep`, `Bash`. No `Write` or `Edit` — Critics report, they do not modify.
- Subagent registration: in `intent/plugins/claude/subagents/.manifest/global-agents.json`.

## Invocation

Invoked via Intent's existing subagent mechanism:

```
Task(
  subagent_type="critic-elixir",
  prompt="<mode-verb> <target-path> [additional-paths...]"
)
```

Alternatively invoked by `in-review` stage-2, which composes the invocation based on project-language detection.

## Modes

Each Critic supports two modes, selected by the first word of the invocation prompt:

| Mode verb          | Purpose                                      | Rule dirs loaded                                                                             |
| ------------------ | -------------------------------------------- | -------------------------------------------------------------------------------------------- |
| `review` (default) | Apply code-category rules to production code | `rules/agnostic/*` + `rules/<lang>/code/*` + any shared (`rules/<lang>/common/*` if present) |
| `test-check`       | Apply test-category rules to test files      | `rules/agnostic/*` + `rules/<lang>/test/*` + any shared                                      |

If the first word of the prompt is not one of the above verbs, the Critic defaults to `review` mode and includes a note in the report ("defaulted to review; specify 'test-check' for test-mode").

Why one subagent per language with modes instead of two per language: rule-loading machinery is shared across modes; invocation is cleaner; severity filtering and report format stay uniform. Rejected alternative documented in `intent/st/ST0034/design.md` D5.

## Rule loading order

On invocation, the Critic loads rules in this order (each entry additive; later-loaded rules do not override earlier ones):

1. `intent/plugins/claude/rules/agnostic/*/RULE.md` — cross-language principles first.
2. `intent/plugins/claude/rules/<lang>/<mode>/*/RULE.md` — language + mode specific (e.g. `rules/elixir/test/*` for `critic-elixir test-check`).
3. `intent/plugins/claude/rules/<lang>/common/*/RULE.md` — optional. Language-specific rules shared across modes (empty in v2.9.0; reserved).
4. **Upstream interop** (Elixir only): if elixir-test-critic is installed as a Claude plugin in `~/.claude/`, load its rules too. Dedupe by `upstream_id`: where an Intent rule has `upstream_id: <slug>` and upstream has the matching rule, skip upstream's copy in favour of Intent's.
5. **User extensions**: for every extension discovered via `plugin_get_source_roots` (see WP02), load any `<ext>/rules/<lang>/**/RULE.md` files contributed.

Precedence on collision (same rule ID, different source): last-loaded wins within the lang / mode combination, but a warning is emitted in the report. In practice Intent IDs and upstream slugs do not collide — Intent rules live in `IN-*` namespace, upstream in `ETC-*`.

After loading, the Critic applies optional filters:

- `.intent_critic.yml` project config (see below): disabled rules dropped from the active set.
- `status:` filter: only `active` rules are enforced. `draft` and `deprecated` rules are loaded but marked inactive.

## Rule content interpretation

For each active rule, the Critic reads:

- `id`, `title`, `severity` — for the report.
- `summary` — for human readability if reported.
- `applies_to` (glob list) — narrows which target files the rule applies to. If target path doesn't match any glob, skip the rule for this target.
- `## Detection` section — the primary enforcement heuristic. Interpreted as guidance (grep patterns, AST signals, structural descriptions), not prescriptive regex.

The Critic **may** use `Grep` for pattern-based Detection signals. It **must** use `Read` to confirm context — grep hits inside comments or in excluded sections should not trigger violations.

## Per-project config

Critics read an optional `.intent_critic.yml` from the project root at invocation time.

Schema:

```yaml
# .intent_critic.yml — optional per-project configuration for Intent Critic subagents
disabled:
  - IN-EX-CODE-007 # moduledoc noise; not a concern for this project
  - IN-EX-TEST-005 # we have legacy non-async tests we're not converting
severity_min: warning # show warning, critical; hide recommendation, style
include_categories:
  - code
  - test
exclude_paths:
  - "lib/generated/**"
  - "test/fixtures/**"
```

All fields optional. Defaults:

- `disabled`: empty list (no rules disabled).
- `severity_min`: `warning`.
- `include_categories`: all categories in scope for the current mode.
- `exclude_paths`: empty list.

Absent config file → defaults apply uniformly.

## Report format

Output is a stable Markdown block with a fixed structure. Critics compose the report after applying all filters and loading all rules. The format is machine-parseable (a line starting with `- ` under a severity heading is always a finding).

### Structure

```
## Critic Report: critic-<lang> <mode> <target>

CRITICAL
- <id> (<slug>) <file>:<line>
  <one-line violation description>
  suggested fix: <short summary>

WARNING
- <id> (<slug>) <file>:<line>
  <description>
  suggested fix: <summary>

RECOMMENDATION
- <id> (<slug>) <file>:<line>
  <description>

STYLE
- <id> (<slug>) <file>:<line>
  <description>

Summary: N critical, N warning, N recommendation, N style.
Rules applied: N agnostic, N language-specific, N upstream (deduped), N user-ext.
Target files reviewed: N.
Config: .intent_critic.yml (present|absent).
```

Every severity section always appears, even when empty. An empty section shows `(none)` on the next line.

When the Summary reports `0 critical, 0 warning, 0 recommendation, 0 style`, the report is a clean result.

### Finding line format

```
- <id> (<slug>) <file>:<line>
  <description>
  suggested fix: <summary>
```

- `<id>` — full rule ID, e.g. `IN-EX-TEST-001`.
- `<slug>` — the rule's slug, e.g. `strong-assertions`.
- `<file>:<line>` — relative path from project root + line number of the violation.
- Description — one line, max 120 chars.
- `suggested fix:` line optional for `style` severity, required for `critical` and `warning`.

### Example report (full, realistic)

```
## Critic Report: critic-elixir review lib/my_app/accounts.ex

CRITICAL
- IN-AG-NO-SILENT-001 (no-silent-errors) lib/my_app/accounts.ex:42
  `case Repo.get(User, id) do _ -> :ok end` swallows not-found and returns :ok.
  suggested fix: pattern-match on {:ok, user} / {:error, :not_found} and surface the error.

WARNING
- IN-EX-CODE-001 (pattern-match-over-conditionals) lib/my_app/accounts.ex:55
  Nested if inside case clause. Replace with multi-clause function.
  suggested fix: extract to def find(%{...}) / def find(_) clauses.

RECOMMENDATION
(none)

STYLE
- IN-EX-CODE-007 (moduledoc-public-modules) lib/my_app/accounts.ex:1
  Public module missing @moduledoc.

Summary: 1 critical, 1 warning, 0 recommendation, 1 style.
Rules applied: 4 agnostic, 11 language-specific, 0 upstream (deduped), 0 user-ext.
Target files reviewed: 1.
Config: .intent_critic.yml (absent).
```

### Clean report example

```
## Critic Report: critic-elixir review lib/my_app/user.ex

CRITICAL
(none)

WARNING
(none)

RECOMMENDATION
(none)

STYLE
(none)

Summary: 0 critical, 0 warning, 0 recommendation, 0 style.
Rules applied: 4 agnostic, 11 language-specific, 0 upstream (deduped), 0 user-ext.
Target files reviewed: 1.
Config: .intent_critic.yml (absent).
```

### Severity tier defaults

Default filter: show `critical` and `warning`. `recommendation` and `style` sections still appear in the report structure but may contain `(none)` when filtered out.

To see all severities: set `severity_min: style` in `.intent_critic.yml`, or invoke the Critic with an explicit severity override in the prompt: `review --all-severities lib/x.ex`.

### Machine-parseability

The report is designed so that:

- `^## Critic Report:` marks the start of a report.
- `^(CRITICAL|WARNING|RECOMMENDATION|STYLE)$` marks a severity section.
- `^- IN-[A-Z]{2}-[A-Z0-9-]+ \(` marks the start of a finding.
- `^Summary: ` marks the summary line.

Downstream tooling (future: auto-fix, CI gates) can parse with these anchors.

## diogenes handoff (Elixir `test-check` mode only)

When `critic-elixir test-check` runs against a test file, it may detect that the test lacks a specification document (e.g. `test/<module>_test.spec.md` absent, or the spec exists but is stale).

On detection, the Critic adds a recommendation to the report:

```
RECOMMENDATION
- IN-EX-TEST-XXX (<slug>) test/my_module_test.exs:1
  Test lacks a Socratic spec. Consider running the `diogenes` subagent first:
    Task(subagent_type="diogenes", prompt="specify lib/my_module.ex")
```

The Critic does not invoke `diogenes` itself. That's the human's or top-level agent's choice. The recommendation is the handoff signal.

## Elixir-test-critic interop

On invocation, `critic-elixir` checks whether the upstream plugin is installed:

```bash
# Detection heuristic (pseudocode)
if [ -d "$HOME/.claude/plugins/elixir-test-critic" ] || \
   [ -d "$HOME/.intent/ext/elixir-test-critic" ]; then
  UPSTREAM_AVAILABLE=1
fi
```

(Exact detection path confirmed in WP07 after verifying upstream's install layout.)

If available:

- Load upstream rules from the discovered path.
- Dedupe against Intent rules by `upstream_id` match.
- Include findings from both sets in the report.
- Annotate upstream findings: `(upstream)` appears after the ID in the finding line.

If not available: Critic runs with Intent rules only. No warning — upstream is optional depth, not a dependency.

## Error handling

Critic failures are surfaced as `## Critic Error:` blocks, not silent. Categories:

- Rule file malformed (fails schema parse) → log warning, skip rule, continue.
- Target file unreadable → report error, set exit status non-zero, produce partial report with what could be processed.
- Rule Detection heuristic raises an exception → log warning, skip rule, continue.
- Project config `.intent_critic.yml` malformed → report error, use defaults, continue with a prominent note.

A Critic that crashes silently is worse than one that reports partial results — always prefer "here's what I got, here's what failed" over "empty report".

## Performance envelope

- Rule loading: 30-50 RULE.md files per invocation. Accepted latency for v2.9.0.
- Target files: arbitrary. Intent's typical invocation is single-file or single-directory.
- Detection: grep + Read. Single target file review should complete in seconds; recursive directory reviews may take tens of seconds.

Performance optimisations (rule caching, index-based loading, parallel Read) are deferred to a future ST. v2.9.0 accepts the straightforward-implementation latency.

## What a Critic does not do

- Does not write, edit, or suggest edits that modify files. Output is read-only.
- Does not invoke other subagents (except soft handoff via report recommendation, as with `diogenes`).
- Does not publish, commit, or push anything.
- Does not run project tests (`mix test`, `cargo test`, etc.).
- Does not compile code.
- Does not cache across invocations. Each run is independent.

## Sample `agent.md` skeleton (for WP07 implementation)

```markdown
---
name: critic-elixir
description: Critic for Elixir code and test files. Enforces Intent's rule library.
tools: Read, Grep, Bash
---

You are a Critic subagent specialised in Elixir. You enforce Intent's rule library
against code and test files. You identify violations and suggest fixes. You do not
modify code.

## Parse the invocation

- First word = mode (`review` or `test-check`). Default `review`.
- Remaining words = target paths.

## Load the rule set

1. Read `$INTENT_HOME/intent/plugins/claude/rules/agnostic/*/RULE.md`.
2. Read `$INTENT_HOME/intent/plugins/claude/rules/elixir/<mode>/*/RULE.md`.
3. Read `$INTENT_HOME/intent/plugins/claude/rules/elixir/common/*/RULE.md` if dir exists.
4. Probe for elixir-test-critic plugin; if found, load its rules and dedupe by `upstream_id`.
5. Probe for user extensions; load any `<ext>/rules/elixir/**/RULE.md`.

## Apply project config

Read `<project>/.intent_critic.yml` if present. Apply `disabled`, `severity_min`,
`exclude_paths`, `include_categories`.

## Detect violations

For each loaded rule whose `applies_to:` matches the target path:

- Apply the Detection heuristic from the `## Detection` section.
- Verify context via Read (grep hits inside comments or strings do not count).
- Collect findings.

## Generate report

Output the report in the contract-specified format, including summary and rule-applied counts.

## Soft handoffs

In `test-check` mode, if the target test lacks a spec, add a diogenes recommendation.
```

## Future work

- Auto-fix: deferred. Critics stay read-only in v2.9.0.
- Parallel review (review an entire directory in parallel): deferred.
- Incremental review (only re-run on changed files): deferred.
- Pre-commit hook integration: deferred; documentation after Critics prove out.
