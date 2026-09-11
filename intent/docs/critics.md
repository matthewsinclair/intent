# Critic Subagents

Critic subagents are thin orchestrators: they read the Intent rule library at invocation time, apply each rule's Detection heuristic to target source files, and emit a machine-parseable report grouped by severity. They do not refactor, autofix, execute tests, or shell out to external linters.

One Critic per code language, plus one for prose. The family:

| Critic          | Language                    | File extensions                              | Modes                               |
| --------------- | --------------------------- | -------------------------------------------- | ----------------------------------- |
| `critic-elixir` | Elixir                      | `.ex`, `.exs`                                | code, test                          |
| `critic-rust`   | Rust                        | `.rs`                                        | code, test                          |
| `critic-swift`  | Swift                       | `.swift`                                     | code, test                          |
| `critic-lua`    | Lua                         | `.lua`                                       | code, test                          |
| `critic-shell`  | bash + zsh                  | `.sh`, `.bash`, `.zsh` (+ shebang detection) | code only                           |
| `critic-prose`  | prose (`author`, `content`) | `.md`, `.mdx`, `.html`                       | review (style), craft-check (craft) |

## Contract

### Invocation signatures

Every code Critic accepts these commands:

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
3. **Extension rules**: not read in v3. `userstate::ext_base()` answers `None`, so `intent claude rules list`/`show` serve canon only and nothing under `~/.intent/ext/` is reached or can shadow a canon rule.
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

Parse-stable properties of the code critics (critic-elixir, critic-rust, critic-swift, critic-lua, critic-shell). `critic-prose` differs: it prints every severity section with `(none)` when empty, and follows `Summary:` with `Rules applied: N agnostic, N prose, N <discipline>.`, `Target files reviewed: N.` and `Config: .intent_critic.yml (present|absent).`

- Every finding begins with a leading `- ` and names a rule id matching `^IN-[A-Z]{2}-[A-Z0-9-]+-[0-9]{3}$` followed by `(<slug>)` in parentheses.
- Severity headers are uppercase bareword lines (`CRITICAL`, `WARNING`, `RECOMMENDATION`, `STYLE`).
- Sections with zero findings are omitted from the body.
- The `Summary:` line always appears, always at the end, always lists every severity (critical, warning, recommendation, style) in descending order with `N <severity>` counts. Counts include severities filtered out of the body.
- The `Rules applied:` line always follows `Summary:` and breaks the rule count into `N agnostic, N language-specific`.

If there are no violations at all, the heading still appears, followed by `Summary: 0 critical, 0 warning, 0 recommendation, 0 style.` and the `Rules applied:` line. Absence of findings is a first-class outcome, not an error.

Every finding cites exactly one rule. Where two rules would both fire on the same line, the Critic names the more specific (usually the language-specific rule concretising an agnostic one) and cross-references the related id in the description.

## `.intent_critic.yml` schema

Per-project config lives at the project root as `.intent_critic.yml`. All keys are optional.

```yaml
disabled:
  - IN-EX-CODE-003 # reason: <why this project opts out>
  - IN-RS-CODE-005 # reason: explicit lifetimes preferred in our domain code

severity_min: warning

# show_all: true    # uncomment to render recommendation + style in the body
```

| Key                      | Value                                                                           | Default   |
| ------------------------ | ------------------------------------------------------------------------------- | --------- |
| `disabled`               | List of rule IDs to suppress entirely for this project.                         | `[]`      |
| `severity_min`           | `critical` \| `warning` \| `recommendation` \| `style`. Body threshold.         | `warning` |
| `show_all`               | Shorthand for `severity_min: style`.                                            | `false`   |
| `post_tool_use_advisory` | Opt-in PostToolUse critic advisory via `.claude/scripts/post-tool-advisory.sh`. | `false`   |

Who reads which key: the headless runner (`intent critic`) reads only `disabled`; the pre-commit gate reads `severity_min` and passes it as `--severity-min`; `show_all` is honoured by the critic subagents only (the gate ignores it); `post_tool_use_advisory` is read only by `.claude/scripts/post-tool-advisory.sh`.

The install template is `lib/templates/_intent_critic.yml`; `intent claude upgrade --apply` seeds it only when the project has no `.intent_critic.yml`, and overwrites an existing one only with `--force`. A worked sample with example `disabled:` entries lives at `intent/plugins/claude/rules/_schema/sample-intent-critic.yml`.

Behaviour under edge conditions:

- **Absent file**: apply defaults silently. No warning, no indicator.
- **Malformed YAML**: print one top-of-report warning line (`(warning: .intent_critic.yml is malformed; using defaults)`) and proceed with defaults. Never hard-fail on parse errors.
- **Unknown rule id in `disabled`**: tolerated silently — rule ids vanish from the pack as rules are renamed or retired, and a hard failure on stale config is disproportionate.

## Headless runner (`intent critic`)

The same rule library is also enforceable without an LLM round-trip via `intent critic <lang>`. The runner parses each rule's YAML frontmatter, extracts the Greppable proxy fenced bash block from the Detection section, and applies the grep regex to target files. Text output opens with a census line (`critic: <lang> -- <asked> of <total> rule(s) ASKED of this run; <armed> armed in total.`), then lines naming declared, undeclared, unrunnable, out-of-context and tool-absent rules, then either `ok: no <lang> findings at severity >= <min> across <n> file(s)` or findings grouped as `== CRITICAL (<n>) ==` with `[CRITICAL] <id> at <file>:<line>` and `  > <line>`. It is not the subagent report format. A parallel `--format json` is available for CI. The JSON document is `{language, asked, armed, total, findings: [{rule, severity, file, line, text}], census: [{rule, arming, disposition, by}], refused: [...], disabled: [...]}` on stdout. `arming` is `armed|declared|unrunnable|undeclared`; `disposition` is `ran|not-run:tool-absent|not-run:out-of-context|n-a`.

```
intent critic <lang> [--files <path> ...] [--staged] [--severity-min <level>] [--format text|json] [--rules <dir>]
```

`intent critic --languages` prints the headless roster (elixir, rust, swift, lua, shell). `intent critic author` and `intent critic content` exit 0 and print nothing: prose has no headless runner. Any other language (including `prose` and `agnostic`) is refused at exit 2.

Use cases:

- **Pre-commit gate** (ST0035/WP-06): `intent critic <lang> --staged --severity-min warning` on the staged file list. Blocks the commit on findings at or above the threshold. Runs in well under a second on a typical staged slice.
- **CI gate**: the same invocation from a GitHub Actions step or equivalent.
- **Fast local sanity check**: `intent critic elixir --files lib/foo.ex` while iterating, before asking the LLM subagent for the fuller review.

Exit codes:

| Exit | Meaning                                                                                                                                                                                                                     |
| ---- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `0`  | Rules were asked and none produced a finding at or above `--severity-min` (also `author`/`content`, which run nothing).                                                                                                     |
| `1`  | Findings at or above `--severity-min`.                                                                                                                                                                                      |
| `2`  | The runner could not answer: unknown language, bad `--severity-min`, `--rules` not a directory, an unreadable file, `git` unavailable for `--staged`, or an empty rule library for the language. The gate fails open on it. |
| `3`  | Refused: a rule armed on an external tool (`critic_tool`, eg shellcheck) whose tool is not on this machine. The gate blocks on it.                                                                                          |

**Mechanical subset only**: only rules that publish a Greppable proxy block, or name an external tool with `critic_tool:` (shellcheck per-file; clippy is `workspace` context and not run per file), are runnable by the headless runner. Rules whose Detection is purely prose (eg "any function body longer than 50 lines") are not run, and the census names them on every run: `declared` when the rule says `No greppable proxy is authoritative`, `undeclared` when it says nothing. The LLM subagent (`Task(subagent_type="critic-<lang>")`) remains the canonical path for those.

**Strict-proxy contract** (ST0039, v2.11.3+): the runner accepts only proxy lines of the form `grep [-r|-n|-E|-rn|-rE|-nE|-rnE|--include=GLOB ...] '<pattern>' [<path>...]` — single grep invocation, no pipes, no `xargs`, no `-L` / `-v` / `-B` / `-A` flags, no awk/sed. Multi-line proxy blocks are accepted as a union of simple lines. A rule whose every proxy line falls outside the contract is census `unrunnable`: it is listed on stdout as `critic: <lang> -- UNRUNNABLE proxy, present but outside the runner contract: <ids>` and in JSON `refused`, and does not change the exit code. Only the single-quoted pattern is used. The line's flags and path arguments are discarded, every pattern runs as an extended regex, and it is applied to each `--files`/`--staged` file that the rule's `applies_to` admits. Globs match suffix-anchored, so `lib/**/*.ex` matches `apps/x/lib/foo.ex`, and a rule with no `applies_to` applies to every file. Rules whose detection cannot be expressed as a simple grep (inverse semantics, filter pipelines, awk state machines, cross-file or callsite-scope reasoning) ship with no Greppable proxy block at all and apply only via `/in-review`.

The runner reads the canon library shipped with the install (`intent/plugins/claude/rules/`), or the directory given with `--rules`, which replaces canon. User extension packs are not read in v3. Agnostic rules are intentionally skipped (they are concretised by language rules and would double-report). Per-project opt-out of specific rule IDs flows through `.intent_critic.yml disabled:` — see the schema section above.

**Code locality** (clarified v2.11.4): the headless runner is `intent critic`, compiled into the v3 `intent` binary since the v2 shell runner was removed, and the canon rule library it reads is the one shipped with the install that the running `intent` binary resolves from its own location (`$INTENT_HOME` is not read), _not_ each project's plugin tree. A fix to the runner, or a strip / edit of a canon rule, applies to every Intent project the moment Intent itself updates; no per-project `intent claude upgrade` is required for the gate's behaviour to change. Per-project `intent claude upgrade --apply` refreshes the project's canon files (`.claude/settings.json`, `CLAUDE.md`, `AGENTS.md`, the seeded `usage-rules.md` and `.intent_critic.yml`) and the `pre-commit.intent` carrier and chain block, but it is not on the critical path for runner-or-rule fixes.

## Integration with `/in-review`

The two-stage review skill (`intent/plugins/claude/skills/in-review/SKILL.md`) dispatches to the right Critic at Stage 2. Dispatch reads the project's declared `languages` array from `intent/.config/config.json` (ST0037 replaced filesystem-marker probing — file presence is unreliable evidence of intent). Each declared language maps directly to its Critic:

| `languages` entry | Dispatches to   |
| ----------------- | --------------- |
| `elixir`          | `critic-elixir` |
| `rust`            | `critic-rust`   |
| `swift`           | `critic-swift`  |
| `lua`             | `critic-lua`    |
| `shell`           | `critic-shell`  |
| `author`          | `critic-prose`  |
| `content`         | `critic-prose`  |

For each declared language, `/in-review` issues one `review` call for the code targets and one `test-check` call for the test targets, then reports the union. Polyglot projects (more than one entry in `languages`) dispatch to each matching Critic with a target glob narrowed to its own subtree; array order is the explicit declaration, and the first entry is the primary where one is needed. An empty `languages` array runs no language Critic — only the agnostic checklist applies.

## Test-spec handoff (Diogenes)

In `test-check` mode, if a target test file has no adjacent spec document (eg `test/accounts_test.exs` without `test/accounts_test.spec.md`), the Critic emits a RECOMMENDATION citing `diogenes` as the handoff:

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

## Operational note: subagent registration freezes per session

Claude Code reads the subagent registry once at session start. Subagents installed mid-session — including the Critic family on a fresh upgrade — are not visible to `Task()` until the next session starts. After running `intent claude subagents install critic-elixir` (or any other Critic), close the current Claude Code session and start a new one before invoking the Critic; otherwise the `Task(subagent_type="critic-elixir", ...)` call resolves to "subagent not found".

This is a Claude Code constraint, not an Intent behaviour. The registration freeze applies equally to canon subagents, extension subagents, and any subagent installed by hand.

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
