# Critic Subagents

Critic subagents are thin orchestrators: they read the Intent rule library at invocation time, apply each rule's Detection heuristic to target source files, and emit a machine-parseable report grouped by severity. They do not refactor, autofix, execute tests, or shell out to external linters.

One Critic per target language. The family as of v2.9.0:

| Critic          | Language   | File extensions                              | Modes      |
| --------------- | ---------- | -------------------------------------------- | ---------- |
| `critic-elixir` | Elixir     | `.ex`, `.exs`                                | code, test |
| `critic-rust`   | Rust       | `.rs`                                        | code, test |
| `critic-swift`  | Swift      | `.swift`                                     | code, test |
| `critic-lua`    | Lua        | `.lua`                                       | code, test |
| `critic-shell`  | bash + zsh | `.sh`, `.bash`, `.zsh` (+ shebang detection) | code only  |

## Contract

### Invocation signatures

Every Critic accepts two commands:

```
Task(subagent_type="critic-<lang>", prompt="review <targets>")
Task(subagent_type="critic-<lang>", prompt="test-check <targets>")
```

Targets are one or more files, directories, or globs. `critic-shell` supports only `review` since shell-test rules are a later addition.

### Mode semantics

- `code` mode loads agnostic + language code rules (`rules/agnostic/*/RULE.md` and `rules/<lang>/code/*/RULE.md`, plus framework subdirectories for `critic-elixir`: `ash/`, `phoenix/`, `lv/`).
- `test` mode loads agnostic + language test rules (`rules/<lang>/test/*/RULE.md`).

Each rule's own `applies_to` glob provides further gating — Phoenix rules gate on controller paths; LiveView rules on `*_live.ex`; Rust test rules on `#[cfg(test)]` or `tests/**` context.

### Ambiguity handling

If the first whitespace-delimited token of the prompt is neither `review` nor `test-check`, the Critic falls back to `code` mode and adds a line to the report summary noting the fallback. This is a deliberate design: a single missing keyword should not stall the review — the Critic states what it did and proceeds.

## Rule loading order

Every invocation re-reads the rule files; caches are not used. The load order for one invocation:

1. **Agnostic rules**: `intent/plugins/claude/rules/agnostic/*/RULE.md`.
2. **Language rules, mode-filtered**: `intent/plugins/claude/rules/<lang>/<code-or-test>/**/RULE.md` (for `critic-elixir` in `code` mode this expands across `code/`, `ash/`, `phoenix/`, and `lv/`).
3. **Extension rules** (if present): `~/.intent/ext/*/rules/<lang>/**/RULE.md`. Extension rules override canon rules with the same `id`; the Critic prints a shadow warning at the top of the report when a collision occurs.
4. **Upstream interop** (Elixir only): if `~/.claude/plugins/elixir-test-critic/rules/` exists, its RULE.md files are loaded and deduped against Intent rules by the `upstream_id` frontmatter field. Absence is silent.

Malformed rule files never hard-fail the run. One broken RULE.md emits a single warning line at the top of the report and is skipped; the rest of the run proceeds.

## Report format

```
## Critic Report: critic-<lang> <mode> <target>

CRITICAL
- <id> (<slug>) <file>:<line>
  <violation description>
  <suggested fix summary>

WARNING
- <id> (<slug>) <file>:<line>
  <violation description>
  <suggested fix summary>

RECOMMENDATION
- <id> (<slug>) <file>:<line>
  <violation description>
  <suggested fix summary>

STYLE
- <id> (<slug>) <file>:<line>
  <violation description>
  <suggested fix summary>

Summary: N critical, N warning, N recommendation, N style.
Rules applied: N agnostic, N language-specific.
```

Parse-stable properties (maintained identically across every Critic):

- Every finding begins with a leading `- ` and names a rule id matching `^IN-[A-Z]{2,3}-[A-Z]+-\d{3}$` followed by `(<slug>)` in parentheses.
- Severity headers are uppercase bareword lines (`CRITICAL`, `WARNING`, `RECOMMENDATION`, `STYLE`).
- Sections with zero findings are omitted from the body.
- The `Summary:` line always appears, always at the end, always lists all four severities in descending order with `N <severity>` counts. Counts include severities filtered out of the body.
- The `Rules applied:` line always follows `Summary:` and breaks the rule count into `N agnostic, N language-specific`.

If there are no violations at all, the heading still appears, followed by `Summary: 0 critical, 0 warning, 0 recommendation, 0 style.` and the `Rules applied:` line. Absence of findings is a first-class outcome, not an error.

Every finding cites exactly one rule. Where two rules would both fire on the same line, the Critic names the more specific (usually the language-specific rule concretising an agnostic one) and cross-references the related id in the description.

## `.intent_critic.yml` schema

Per-project config lives at the project root as `.intent_critic.yml`. All keys are optional.

```yaml
disabled:
  - IN-EX-CODE-007 # reason: moduledoc noise not valued here
  - IN-RS-CODE-005 # reason: explicit lifetimes preferred in our domain code

severity_min: warning

# show_all: true    # uncomment to render recommendation + style in the body
```

| Key            | Value                                                                   | Default   |
| -------------- | ----------------------------------------------------------------------- | --------- |
| `disabled`     | List of rule IDs to suppress entirely for this project.                 | `[]`      |
| `severity_min` | `critical` \| `warning` \| `recommendation` \| `style`. Body threshold. | `warning` |
| `show_all`     | Shorthand for `severity_min: style`.                                    | `false`   |

A canonical sample lives at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`. Copy it to the project root and edit; annotate every `disabled` entry with a trailing `# reason: ...` comment.

Behaviour under edge conditions:

- **Absent file**: apply defaults silently. No warning, no indicator.
- **Malformed YAML**: print one top-of-report warning line (`(warning: .intent_critic.yml is malformed; using defaults)`) and proceed with defaults. Never hard-fail on parse errors.
- **Unknown rule id in `disabled`**: tolerated silently — rule ids vanish from the pack as rules are renamed or retired, and a hard failure on stale config is disproportionate.

## Integration with `/in-review`

The two-stage review skill (`intent/plugins/claude/skills/in-review/SKILL.md`) dispatches to the right Critic at Stage 2. Stage-2 detection probes the project root in this order:

| Probe                                       | Dispatches to   |
| ------------------------------------------- | --------------- |
| `mix.exs`                                   | `critic-elixir` |
| `Cargo.toml`                                | `critic-rust`   |
| `Package.swift`                             | `critic-swift`  |
| `.luarc.json` or `.lua`-dominant tree       | `critic-lua`    |
| `bin/` or `scripts/` with bash/zsh shebangs | `critic-shell`  |

For each match, `/in-review` issues one `review` call for the code targets and one `test-check` call for the test targets, then reports the union. Polyglot projects (e.g., `mix.exs` and `Cargo.toml` at the same root) prompt the user for which language to review rather than dispatching every Critic blindly.

## Test-spec handoff (Diogenes)

In `test-check` mode, if a target test file has no adjacent spec document (e.g., `test/accounts_test.exs` without `test/accounts_test.spec.md`), the Critic emits a RECOMMENDATION citing `diogenes` as the handoff:

```
RECOMMENDATION
- (test-spec-missing) <path>:1
  No adjacent spec file (<expected-spec-path>).
  Run `Task(subagent_type="diogenes", prompt="specify <path>")` for Socratic spec generation; re-run critic-<lang> test-check afterward.
```

The Critic never invokes `diogenes` itself — the handoff is an advisory the user acts on. Absence of a spec is not a rule violation; it is a handoff opportunity.

Note: the `diogenes` subagent as implemented in v2.9.0 is Elixir-specialised. The Critic-side handoff pattern is deliberately language-agnostic — generalising `diogenes` across Rust, Swift, and Lua test stacks is a separate concern for a future steel thread.

## Architectural escalation (Socrates)

When a finding depends on a non-local architectural call — cross-module Highlander collapse, genuinely ambiguous Detection, competing design principles — the Critic emits a RECOMMENDATION citing `socrates`:

```
RECOMMENDATION
- (architectural-review) <path>:<line>
  Finding <id> turns on an architectural call: <short description>.
  Consider `Task(subagent_type="socrates", prompt="review <decision>")` for CTO-level dialog before acting.
```

Same constraint: recommend, never invoke. Reserve the advisory for genuinely cross-cutting cases; do not tag every finding with it.

## Verification

To run a Critic by hand against a fixture:

```
Task(subagent_type="critic-elixir", prompt="review tests/fixtures/critics/elixir/code/would-catch/sample.ex")
Task(subagent_type="critic-elixir", prompt="test-check tests/fixtures/critics/elixir/test/would-catch/sample_test.exs")
```

Fixtures live under `tests/fixtures/critics/<lang>/{code,test}/{would-catch,would-miss}/`. Each directory contains a sample source file plus a `manifest.txt` listing the expected rule IDs (or `no violations` for the `would-miss` variants).

Expected outcomes:

- `would-catch` runs should surface every rule id listed in that directory's `manifest.txt`. Additional findings beyond the manifest are acceptable so long as they are legitimate.
- `would-miss` runs should emit the bare `Summary: 0 critical, 0 warning, 0 recommendation, 0 style.` line with no findings in the body.

Interpreting the report:

- Read severity headers top-down. Critical always comes first; fix those before touching warnings.
- Each finding's `(<slug>)` lets you open the rule file directly: `intent/plugins/claude/rules/<lang>/<mode-or-subdir>/<slug>/RULE.md`.
- The `Rules applied:` line tells you whether any rules were filtered out by `.intent_critic.yml` — if the count is unexpectedly low, check the config.

## Non-goals

- **No autofix.** Critics report only. They never modify source files.
- **No external lint shelling.** Critics never call `credo`, `cargo clippy`, `swiftlint`, `luacheck`, `shellcheck`, or any other tool. Rules can reference external tool lints in their Detection prose, but the Critic enforces by reading the rule and applying the heuristic — not by running the tool.
- **No test execution.** Critics are static reviewers.
- **No rule authoring from inside the Critic.** New or amended rules go into `rules/<lang>/` via a normal edit, validated by `intent claude rules validate`.
- **No caching across invocations.** Every run re-reads the rule library to keep detections aligned with the current state of the rules.
