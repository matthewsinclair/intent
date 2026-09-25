# Rule Schema

This document is the authoritative reference for the shape of a `RULE.md` file in Intent's rule library. Every rule in `intent/plugins/claude/rules/**` conforms to this schema. `intent claude rules validate` enforces the machine-checkable part: required keys present, no top-level key outside the two field tables, every `id` well formed and unique, every id in `references`/`concretised_by`/`related_rules`/`conflicts_with` resolving, and an `_attribution/` row for every `upstream_id`. It does not check enum values, the `concretised_by` obligation, the H1, the body sections or the example files.

The schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, 2026 Manuel Zubieta, pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`). Intent rules use upstream's frontmatter shape plus Intent-specific fields (one of them, `language`, required). Upstream rules do not drop into Intent's discovery unchanged: they have no `language:` and their `ETC-` ids fail the id check.

See `id-scheme.md` for the `IN-<LANG>-<CAT>-<NNN>` format, `attribution-policy.md` for when to use `upstream_id:` and MIT notices, and `critic-contract.md` for how Critics consume rules.

## File structure

Each rule lives in its own directory:

```
rules/<lang>/<category>/<slug>/
├── RULE.md          # required — the rule itself
├── good_test.exs    # runnable example (test-category rules)
├── bad_test.exs     # runnable example (test-category rules)
├── good.exs         # code-category rules, non-test
└── bad.exs          # code-category rules, non-test
```

- Agnostic rules (`rules/agnostic/<slug>/`) omit example files entirely. A **pattern** rule -- one governing a code shape -- cites `concretised_by:` language-specific rules; a **procedural** rule -- one governing an ACTION -- has no language-specific concretisation to point at and carries none. `IN-AG-RED-CONTROL-001` and `IN-AG-FIAT-001` are procedural members.
- Elixir `code` and `test` rules have runnable `.exs` examples, run under standalone `elixir` by `tests/unit/rule_pack_elixir_runnable.bats` in CI; `ash`, `phoenix` and `lv` rules are inline-only (they need a Mix project).
- Rust, Swift, Lua, shell and prose-pack (`prose`, `author`, `content`) rules have textual examples embedded in `RULE.md` (see `CI-LIMITATIONS.md`).

## Frontmatter

YAML frontmatter at the top of every `RULE.md`, between `---` delimiters. All top-level fields are scalars, strings, or flat arrays. No nested maps: the reader (`rules::parse_front`) takes only column-zero `key: value` scalars and block or inline lists, so anything nested is not read.

### Required fields

| Field          | Type         | Purpose                                                                                                                                                                                                                  |
| -------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `id`           | string       | Rule identifier. Format: `IN-<LANG>-<CAT>-<NNN>`. See `id-scheme.md`.                                                                                                                                                    |
| `title`        | string       | Human-readable one-line name. Matches the H1 heading in the body.                                                                                                                                                        |
| `language`     | enum         | One of `agnostic`, `elixir`, `rust`, `swift`, `lua`, `shell`, `prose`, `author`, `content`. Drives rule-pack location.                                                                                                   |
| `category`     | string       | Kebab-case category slug (`code`, `test`, `ash`, `phoenix`, `lv`, `architecture`, etc.).                                                                                                                                 |
| `severity`     | enum         | One of `critical`, `warning`, `recommendation`, `style`.                                                                                                                                                                 |
| `summary`      | string       | One or two sentences. YAML multiline with `>` encouraged. Printed with the whole file by `intent claude rules show <id>`; `list` does not show it.                                                                       |
| `principles`   | list[string] | One or more principle short-names. Intent's agnostic principles plus upstream's for Elixir rules. A rule citing an agnostic rule in `references` names its principle here too (`highlander` for `IN-AG-HIGHLANDER-001`). |
| `applies_when` | list[string] | Natural-language circumstances under which the rule applies. For humans and Claude.                                                                                                                                      |

### Optional fields

| Field                 | Type         | Purpose                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| --------------------- | ------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `upstream_id`         | string       | Intent-specific. Slug of the upstream elixir-test-critic rule this borrows from. Required when principle or detection is lifted from upstream. See `attribution-policy.md`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `applies_to`          | list[glob]   | Intent-specific. Machine-readable glob patterns. Used by tooling to narrow file sets. Example: `["test/**/*_test.exs"]`. The headless critic matches each glob suffix-anchored (`lib/**/*.ex` also matches `apps/x/lib/foo.ex`). An absent `applies_to` means every file.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `references`          | list[id]     | Intent-specific. Cross-rule citations by Intent ID. Example: `[IN-AG-HIGHLANDER-001]`. Distinct from `related_rules`: `references` implies "this rule concretises or depends on"; `related_rules` is a softer suggestion.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `concretised_by`      | list[id]     | Required on a PATTERN agnostic rule (one governing a code shape), which lists every language-specific rule whose `references` cite it, and at least 2; forbidden on language rules. **A PROCEDURAL agnostic rule -- one governing an action rather than a code pattern -- carries none, because a prohibition on doing something has no language-specific concretisation to point at.** `IN-AG-RED-CONTROL-001` and `IN-AG-FIAT-001` are that category. **THE ANTI-VAGUENESS OBLIGATION DOES NOT LAPSE, IT CHANGES FORM: a procedural rule discharges it through `applies_when`, which must name SITUATIONS rather than virtues, and one with an empty or aspirational `applies_when` has failed the requirement exactly as a pattern rule with no `concretised_by` would.** |
| `aliases`             | list[string] | Previous slugs, or previous ids when the rule moved bucket (`IN-PR-STYLE-001` carries `IN-AU-STYLE-001`). A record for readers: nothing resolves an alias. Empty array `[]` by default.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      |
| `tags`                | list[string] | Discovery keywords. No enforced vocabulary.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  |
| `related_rules`       | list[id]     | Softer cross-reference than `references`. Rules that are worth reading together but do not imply dependency.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `sources`             | list[url]    | URLs to supporting docs, blog posts, conference talks, library docs.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `conflicts_with`      | list[id]     | Rule IDs that contradict this one. Rare; typically indicates an opinionated style split.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `does_not_apply_when` | list[string] | Natural-language exceptions. Content mirrors the `## When This Does Not Apply` Markdown section; frontmatter version is for tooling filters.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                 |
| `status`              | enum         | `active` (default), `draft`, `deprecated`. The headless critic arms only `active` rules; the critic subagents do not read `status`. Defaults to `active` if omitted.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `version`             | integer      | Rule-content version. Bump on breaking changes to Detection or Problem framing. Starts at `1`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| `critic_tool`         | string       | Intent-specific. Names an EXTERNAL tool whose findings this rule is expressed through (`shellcheck`, `clippy`). **READ, not decorative** -- `classify` in `intentsvcs/src/critic.rs` reads it and the headless runner dispatches on it. A rule carrying it is enforced by that tool rather than by a greppable proxy.                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `critic_tool_context` | enum         | Intent-specific. How the external tool is invoked: `per-file` (the default when the key is absent) or `workspace`. Any value other than `per-file` reports the rule `not-run:out-of-context` in a per-file run. Only meaningful alongside `critic_tool`.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `critic_tool_codes`   | list[string] | Intent-specific. The tool's own diagnostic codes this rule claims, inline (`[SC2086, SC2046]`) or as a block list. `run` in `critic.rs` reads it to narrow the tool's output to this rule's subject. Without it the rule reports nothing: `shellcheck_findings` returns no findings for an empty code list, while the census still counts the rule as asked.                                                                                                                                                                                                                                                                                                                                                                                                                 |

**THE `critic_tool*` KEYS WERE READ BY THE RUNNER AND UNDECLARED HERE UNTIL 2026-09-09**, across shipped rules -- `rust/code/{lifetime-elision-first,ownership-before-clone,result-over-panic}` and `shell/code/{no-parse-ls,quote-expansions}`. `intent claude rules validate` found them on its first run, the same day it was wired, which is the argument for that verb REFUSING unknown keys rather than warning: under a warn contract the findings would have printed and nobody would have acted.

**AND THE DANGEROUS FIX WAS THE OBVIOUS ONE.** Faced with a validator refusing those rules, the cheap repair is to delete the offending keys from the rules -- which would have silently disabled shellcheck code-narrowing and the clippy integration, because `critic.rs` reads each of them. **The rules were right and this schema was the stale document.** It is the mirror of a defect the estate already guards against in the other direction -- a surface DECLARING what nothing reads. This is code READING what nothing declared. **Both are one concern living in two homes and drifting apart, and only one of the two directions currently has a verb that reports it.**

### Forbidden in frontmatter

- Nested maps (eg `detection: { pattern: ..., severity: ... }`). Keep everything flat.
- Free-form keys outside this schema. `intent claude rules validate` rejects unknown top-level keys. If new metadata is needed, update this schema first.

### Example frontmatter

Elixir test rule (Intent-original, so no `upstream_id:`):

```yaml
---
id: IN-EX-TEST-001
language: elixir
category: test
severity: critical
title: Strong assertions against concrete values
summary: >
  Shape assertions (`assert is_struct`, `assert is_map`, `refute is_nil`) pass for
  any value of the right type. They do not prove the function did what the test
  claims. Assert on concrete fields and return shapes.
principles:
  - honest-data
  - public-interface
  - highlander
applies_when:
  - "Any ExUnit test asserting on a return value of a fallible function"
  - "Assertions on struct fields, map values, or list contents"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Property-based tests that assert invariants rather than specific values"
  - "Tests where the shape itself is the contract (eg GenServer callback returns)"
references:
  - IN-AG-HIGHLANDER-001
related_rules:
  - IN-EX-TEST-002
aliases: []
status: active
version: 1
---
```

Agnostic rule (no runnable examples, must concretise):

```yaml
---
id: IN-AG-HIGHLANDER-001
language: agnostic
category: architecture
severity: critical
title: There can be only one
summary: >
  Never duplicate code paths, modules, or logic for the same concern. Duplicate
  paths drift over time, creating inconsistent behaviour and silent bugs.
principles:
  - highlander
applies_when:
  - "Validation, formatting, error handling, configuration loading, I/O wrappers"
  - "Any code concern that could be implemented in more than one place"
does_not_apply_when:
  - "Localisation files (same key, different language translations)"
  - "Test fixtures where repeated setup is clearer than extracted helpers"
concretised_by:
  - IN-EX-CODE-006
  - IN-RS-CODE-002
aliases: []
status: active
version: 1
---
```

## Markdown body

Sections appear in this fixed order. Every rule has all of them; empty sections are explicitly marked "N/A" rather than omitted, so readers can tell the author considered and rejected the section.

```markdown
# <Title matching frontmatter>

<One-line restatement of the rule. Not a section heading; sits between H1 and "## Problem".>

## Problem

<Concrete scenario of what goes wrong when the rule is violated. Name the failure mode. Cite a real incident or pattern where possible. 2-6 paragraphs.>

## Detection

<How a reviewer or Critic subagent spots a violation. Grep patterns, AST signals, structural heuristics. This is guidance for the Critic, not a prescriptive regex. 1-3 paragraphs plus a bulleted list of signals.>

## Bad

<Fenced code block showing the antipattern. Cross-link to `bad_test.exs` / `bad.exs` where the rule ships one. Keep the inline snippet short.>

## Good

<Fenced code block showing the correct pattern. Cross-link to `good_test.exs` / `good.exs` where the rule ships one.>

## When This Applies

<Expanded form of the `applies_when:` frontmatter list. Discusses nuance.>

## When This Does Not Apply

<Expanded form of `does_not_apply_when:`. Substantive — prevents Critic noise. If there are no exceptions, write "No known exceptions." and move on.>

## Further Reading

<Bulleted list of URLs, book references, other rule cross-links. Each entry is a complete citation (title + source), not a bare URL.>
```

### Section headings are load-bearing

- Use exact H2 headings (`## Problem`, `## Detection`, etc.) -- no variations. The headless critic finds a proxy only under an exact `## Detection` line; `intent claude rules validate` does not check body sections.
- `## Detection` is also where the headless critic looks for a proxy: a line naming a `Greppable proxy` followed by a fence opened as ` ```bash ` arms the rule, `No greppable proxy is authoritative for this rule` declares that it has none, and a `critic_tool:` in the frontmatter takes precedence over both. What a proxy line may contain is the Strict-proxy contract in `intent/docs/critics.md`.
- `## Bad` / `## Good` use the short form (upstream convention). Not `## Bad Example` or `## The Bad Pattern`.
- `## When This Applies` / `## When This Does Not Apply` match upstream verbatim (not `## When It Applies` with different wording — exact match). Note the frontmatter fields stay `applies_when:` / `does_not_apply_when:` (Intent's tooling names); only the Markdown section headings match upstream.

## Runnable example contract (Elixir)

For Elixir rules, runnable examples live in sibling files to `RULE.md`:

### Test-category rules

Files: `good_test.exs`, `bad_test.exs`

Template:

```elixir
# EXPECTED: passes
Mix.install([])

ExUnit.start(autorun: true)

defmodule RuleSlugGoodTest do
  use ExUnit.Case, async: true

  test "demonstrates the correct pattern" do
    assert ...
  end
end
```

`bad_test.exs` is identical in shape but demonstrates the antipattern:

```elixir
# EXPECTED: passes
# BAD PRACTICE: <one-line description of the antipattern>.
#   The Critic subagent detects this by reading the source; ExUnit itself does not fail.
Mix.install([])

ExUnit.start(autorun: true)

defmodule RuleSlugBadTest do
  use ExUnit.Case, async: true

  test "demonstrates the antipattern" do
    # Antipattern inline — see the RULE.md Detection section for the signal the Critic uses.
    assert is_struct(user)
  end
end
```

### Code-category rules

Files: `good.exs`, `bad.exs`

For rules about production code (not tests). Examples are standalone scripts without ExUnit:

```elixir
# EXPECTED: passes
Mix.install([])

defmodule RuleSlug.GoodExample do
  def do_thing({:ok, value}), do: {:ok, transform(value)}
  def do_thing({:error, _} = err), do: err
  defp transform(v), do: v * 2
end

IO.inspect(RuleSlug.GoodExample.do_thing({:ok, 21}))
```

### Exit code contract

**Both `good*.exs` and `bad*.exs` must exit 0 when run.** This is a deliberate upstream convention: rule violations are detected by the Critic reading the source, not by runtime failure. The `bad` example demonstrates what a Critic would flag; ExUnit itself is not the enforcer.

This matters because:

- Critics work statically (Read + Grep), not by invoking tests.
- Many antipatterns compile and pass ExUnit while still being wrong (shape assertions, `Process.sleep` for synchronisation, missing `async: true`).
- Requiring runtime failure would force awkward contrivances.

Rules where the antipattern would actually fail to compile or run are rare. In those cases, put the broken snippet in the `## Bad` section of `RULE.md` as a fenced block rather than a `bad.exs` file.

### Validation

Both files must exit 0 when executed. The canonical invocation is:

```bash
elixir <rule-dir>/good_test.exs
elixir <rule-dir>/bad_test.exs
```

The files use `Mix.install([])` + `ExUnit.start(autorun: true)` and run standalone — no surrounding Mix project required. `mix test` cannot run them: `Mix.install/2` refuses inside a Mix project.

The first non-empty line of each file must be `# EXPECTED: passes` (upstream convention; not checked by `intent claude rules validate`; CI asserts it in `tests/unit/rule_pack_elixir_runnable.bats`). Other permitted values (`failure`, `flaky`) are reserved for upstream-style rules where runtime failure is intentional; Intent rules use `passes`.

## Runnable example contract (Rust / Swift / Lua)

**Textual only.** No `good.rs` / `bad.rs` / etc. files. Examples are fenced code blocks inside the `## Bad` and `## Good` Markdown sections.

```markdown
## Bad

\`\`\`rust
fn load(id: u32) -> User {
  let user = db.find(id).unwrap(); // panics on missing
  user
}
\`\`\`

## Good

\`\`\`rust
fn load(id: u32) -> Result<User, Error> {
  db.find(id).ok_or(Error::NotFound)
}
\`\`\`
```

See `CI-LIMITATIONS.md` for the rationale. Runnable examples for these languages are a future-work item; they'd require a Rust / Swift / Lua CI environment in the Intent repo.

## Formatting invariants

Two-space indentation is mandatory throughout the rule library, regardless of what the target language's ecosystem conventionally uses.

- **All fenced code blocks** in `RULE.md` use 2-space indentation — Elixir, Rust, Swift, Lua, Python, Bash, YAML, JSON, everything. Rust (conventionally 4), Swift (conventionally 4), Lua (conventionally 4), Python (conventionally 4): reformat to 2 before committing.
- **All runnable `.exs` files** (`good_test.exs`, `bad_test.exs`, `good.exs`, `bad.exs`) use 2-space indentation. Elixir's own convention is already 2-space, so no reformatting is needed for Elixir — but always verify when adapting upstream snippets.
- **All YAML frontmatter** uses 2-space indentation (matches YAML's standard).
- **All tables** in `RULE.md` use consistent column alignment; the markdown linter may adjust spacing around pipes — that's fine and expected.

The rationale is consistency across the Intent repo, not fidelity to any given language's style guide. The scope is Intent-internal: external sources quoted verbatim in "Further Reading" need not be reformatted.

When authoring a new rule, check indentation by step: a line indented more than two columns deeper than the line above it is a violation, except a continuation aligned under an opening construct (`with` clauses, wrapped arguments). `grep -nE '^    [^ ]'` is not that check: it also matches correctly nested code two levels deep.

## Field consumers (every field must have a consumer)

Anti-bloat invariant: every field should have a named consumer; one with none is a candidate for removal. The consumers are:

| Field                 | Claude reads | `intent claude rules` reads |       Critic subagent reads       |       `intent critic` reads       |
| --------------------- | :----------: | :-------------------------: | :-------------------------------: | :-------------------------------: |
| `id`                  |      ✓       |              ✓              |                 ✓                 |                 ✓                 |
| `title`               |      ✓       |              ✓              |                 ✓                 |                --                 |
| `language`            |      ✓       |              ✓              |           ✓ (dispatch)            |       ✓ (selects the pack)        |
| `category`            |      ✓       |              ✓              |                 ✓                 |                --                 |
| `severity`            |      ✓       |              ✓              |            ✓ (filter)             | ✓ (filter; unknown value refused) |
| `summary`             |      ✓       |             --              |                --                 |                --                 |
| `principles`          |      ✓       |   ✓ (validate: agreement)   |                 ✓                 |                --                 |
| `applies_when`        |      ✓       |             --              |                 ✓                 |                --                 |
| `upstream_id`         |      --      |    ✓ (attribution check)    |        ✓ (upstream dedupe)        |                --                 |
| `applies_to`          |      --      |             --              |          ✓ (file filter)          |          ✓ (file filter)          |
| `references`          |      ✓       |   ✓ (resolves, agreement)   |                 ✓                 |                --                 |
| `concretised_by`      |      ✓       |   ✓ (resolves, agreement)   |                --                 |                --                 |
| `aliases`             |      ✓       |             --              |                --                 |                --                 |
| `tags`                |      --      |             --              | ✓ (`critic-shell` dialect filter) |                --                 |
| `related_rules`       |      ✓       |    ✓ (validate resolves)    |                --                 |                --                 |
| `sources`             |      ✓       |             --              |                --                 |                --                 |
| `conflicts_with`      |      ✓       |    ✓ (validate resolves)    |                --                 |                --                 |
| `does_not_apply_when` |      ✓       |             --              |            ✓ (filter)             |                --                 |
| `status`              |      --      |             --              |                --                 |    ✓ (non-active never fires)     |
| `version`             |      --      |             --              |                --                 |                --                 |
| `critic_tool`         |      --      |             --              |                --                 |           ✓ (dispatch)            |
| `critic_tool_context` |      --      |             --              |                --                 |  ✓ (per-file or out of context)   |
| `critic_tool_codes`   |      --      |             --              |                --                 |   ✓ (narrows the tool's output)   |

Fields with zero ✓s are candidates for removal.

## Adding a new rule (quick reference)

1. Identify the rule pack. Use `language` + `category` to find the directory. See `intent/llm/DECISION_TREE.md` (Step 3) for placement.
2. Assign an ID. Use the next free `IN-<LANG>-<CAT>-<NNN>`. Never reuse a numeric suffix, including for deleted rules.
3. Copy any existing rule directory as a template. `rules/elixir/test/strong-assertions/` is the canonical exemplar (runnable examples + full schema); a simpler starting point is any agnostic rule under `rules/agnostic/`.
4. Fill frontmatter per this schema.
5. Write the Markdown sections.
6. Author runnable examples (Elixir) or textual examples (Rust/Swift/Lua). For an Elixir rule with runnable examples, add its slug to `runnable_code_rules` or `runnable_test_rules` in `tests/unit/rule_pack_elixir_runnable.bats`; that list is hand-kept.
7. Run `intent claude rules validate <id>`.
8. Ensure skills that cite the rule have the new ID in their `rules:` list; critic subagents pick it up from `intent claude rules list` with no wiring.
9. If the rule borrows from upstream, add `upstream_id:` and update `_attribution/elixir-test-critic.md` per `attribution-policy.md`.

## Schema evolution

- **Adding optional fields**: minor version bump; backwards compatible.
- **Adding required fields**: major version bump; migration needed. Extremely rare.
- **Removing fields**: major version bump; must migrate existing rules first.
- **Renaming sections**: forbidden without a validator update and a migration pass.

Schema changes are proposed via a new ST, not a WP inside an existing ST.
