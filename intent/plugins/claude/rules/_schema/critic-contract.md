# Critic Subagent Contract

This document is the contract that every Critic subagent in Intent must satisfy. WP07 implements concrete Critics (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`) and WP12 adds `critic-shell` — all share the contract defined here. The `in-review` skill's stage-2 dispatches to one of these Critics based on project language indicators.

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
- Language code match: `EX` → `critic-elixir`, `RS` → `critic-rust`, `SW` → `critic-swift`, `LU` → `critic-lua`, `SH` → `critic-shell`, and `PR` / `AU` / `CO` -> `critic-prose` (one critic for the prose base and both prose disciplines; its modes are `review` and `craft-check`).
- Tool loadout (declared in `agent.md` frontmatter): `Read`, `Grep`, `Glob`, `Bash`. No `Write` or `Edit` — Critics report, they do not modify.
- Subagent registration: a directory `intent/plugins/claude/subagents/critic-<lang>/` containing `agent.md`; `intent claude subagents list` enumerates it.

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

Each code Critic supports these modes, selected by the first word of the invocation prompt:

| Mode verb          | Purpose                                      | Rule dirs loaded                                                                                                                                        |
| ------------------ | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `review` (default) | Apply code-category rules to production code | every `agnostic` rule plus `<lang>` rules whose category is `code` (`critic-elixir` also takes `ash`, `phoenix`, `lv`), from `intent claude rules list` |
| `test-check`       | Apply test-category rules to test files      | every `agnostic` rule plus `<lang>` rules whose category is `test`                                                                                      |

If the first word of the prompt is not one of the above verbs, the Critic defaults to `review` mode and includes a note in the report ("defaulted to review; specify 'test-check' for test-mode").

Why one subagent per language with modes instead of one per mode: rule-loading machinery is shared across modes; invocation is cleaner; severity filtering and report format stay uniform.

## Rule loading order

On invocation, the Critic loads rules in this order (each entry additive; later-loaded rules do not override earlier ones):

1. `intent claude rules list --lang agnostic` and `intent claude rules list --lang <lang>`; select by the `category` column for the mode.
2. `intent claude rules show <id>` for each selected rule.
3. **Upstream interop** (Elixir only): if elixir-test-critic is installed as a Claude plugin in `~/.claude/`, load its rules too. Dedupe by `upstream_id`: where an Intent rule has `upstream_id: <slug>` and upstream has the matching rule, skip upstream's copy in favour of Intent's.
4. **User extensions**: none in v3. `userstate::ext_base()` answers `None`, so `rules list` serves canon only.

Intent IDs and upstream slugs do not collide: Intent rules live in the `IN-*` namespace, upstream in `ETC-*`.

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
  - IN-EX-CODE-003 # reason: <why this project opts out>
  - IN-EX-TEST-005 # we have legacy non-async tests we're not converting
severity_min: warning # show warning, critical; hide recommendation, style
show_all: false # shorthand for severity_min: style (subagents only)
post_tool_use_advisory: false # opt-in per-edit advisory
```

All fields optional. Defaults:

- `disabled`: empty list (no rules disabled).
- `severity_min`: `warning`.

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
Rules applied: N agnostic, N language-specific.
```

(`critic-prose` prints `Rules applied: N agnostic, N prose, N <discipline>.`, `Target files reviewed: N.` and `Config: .intent_critic.yml (present|absent).`)

The code critics omit a severity section with no findings; `critic-prose` prints every section and shows `(none)` under an empty one.

When the Summary reports `0 critical, 0 warning, 0 recommendation, 0 style`, the report is a clean result.

### Finding line format

```
- <id> (<slug>) <file>:<line>
  <description>
  suggested fix: <summary>
```

- `<id>` — full rule ID, eg `IN-EX-TEST-001`.
- `<slug>` — the rule's slug, eg `strong-assertions`.
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
- IN-EX-CODE-003 (impl-true-on-callbacks) lib/my_app/accounts.ex:12
  Behaviour callback missing @impl true.
  suggested fix: add @impl true above the callback.
- IN-EX-CODE-001 (pattern-match-over-conditionals) lib/my_app/accounts.ex:55
  Nested if inside case clause. Replace with multi-clause function.
  suggested fix: extract to def find(%{...}) / def find(_) clauses.

Summary: 1 critical, 2 warning, 0 recommendation, 0 style.
Rules applied: N agnostic, N language-specific.
```

### Clean report example

```
## Critic Report: critic-elixir review lib/my_app/user.ex

Summary: 0 critical, 0 warning, 0 recommendation, 0 style.
Rules applied: N agnostic, N language-specific.
```

### Severity tier defaults

Default filter: show `critical` and `warning`. `recommendation` and `style` findings are counted in `Summary:` but not rendered in the body.

To see all severities: set `severity_min: style` in `.intent_critic.yml`, or invoke the Critic with an explicit severity override in the prompt: `review --all-severities lib/x.ex`.

### Machine-parseability

The report is designed so that:

- `^## Critic Report:` marks the start of a report.
- `^(CRITICAL|WARNING|RECOMMENDATION|STYLE)$` marks a severity section.
- `^- IN-[A-Z]{2}-[A-Z0-9-]+ \(` marks the start of a finding.
- `^Summary: ` marks the summary line.

Downstream tooling (future: auto-fix, CI gates) can parse with these anchors.

## diogenes handoff (Elixir `test-check` mode only)

When `critic-elixir test-check` runs against a test file, it may detect that the test lacks a specification document (eg `test/<module>_test.spec.md` absent, or the spec exists but is stale).

On detection, the Critic adds a recommendation to the report:

```
RECOMMENDATION
- (test-spec-missing) test/my_module_test.exs:1
  Test lacks a Socratic spec. Consider running the `diogenes` subagent first:
    Task(subagent_type="diogenes", prompt="specify lib/my_module.ex")
```

The Critic does not invoke `diogenes` itself. That's the human's or top-level agent's choice. The recommendation is the handoff signal.

## Elixir-test-critic interop

On invocation, `critic-elixir` checks whether the upstream plugin is installed:

```bash
# Detection heuristic (pseudocode)
if [ -d "$HOME/.claude/plugins/elixir-test-critic" ]; then
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

Critic failures are surfaced as one-line warnings at the top of the report (eg `(warning: <id> unreadable; skipped)`), never silently. Categories:

- Rule file malformed (fails schema parse) → log warning, skip rule, continue.
- Target file unreadable → report error, set exit status non-zero, produce partial report with what could be processed.
- Rule Detection heuristic raises an exception → log warning, skip rule, continue.
- Project config `.intent_critic.yml` malformed → report error, use defaults, continue with a prominent note.

A Critic that crashes silently is worse than one that reports partial results — always prefer "here's what I got, here's what failed" over "empty report".

## Performance envelope

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
tools: Read, Grep, Glob, Bash
---

You are a Critic subagent specialised in Elixir. You enforce Intent's rule library
against code and test files. You identify violations and suggest fixes. You do not
modify code.

## Parse the invocation

- First word = mode (`review` or `test-check`). Default `review`.
- Remaining words = target paths.

## Load the rule set

1. `intent claude rules list --lang agnostic` and `intent claude rules list --lang elixir`; select by category for the mode.
2. `intent claude rules show <id>` for each selected rule.
3. Probe for elixir-test-critic plugin; if found, load its rules and dedupe by `upstream_id`.
4. Probe for user extensions; load any `<ext>/rules/elixir/**/RULE.md`.

## Apply project config

Read `<project>/.intent_critic.yml` if present. Apply `disabled`, `severity_min`, `show_all`.

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
