# Rule Schema (Intent v2.9.0)

This document is the authoritative reference for the shape of a `RULE.md` file in Intent's rule library. Every rule in `intent/plugins/claude/rules/**` conforms to this schema. The `intent claude rules validate` tool enforces it.

The schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, 2026 Manuel Zubieta, pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`). Upstream rules drop into Intent's discovery unchanged; Intent rules use the same frontmatter shape plus a small set of Intent-specific optional fields that upstream tools ignore.

See `id-scheme.md` for the `IN-<LANG>-<CAT>-<NNN>` format, `attribution-policy.md` for when to use `upstream_id:` and MIT notices, and `critic-contract.md` for how Critics consume rules.

## File structure

Each rule lives in its own directory:

```
rules/<lang>/<category>/<slug>/
├── RULE.md          # required — the rule itself
├── good_test.exs    # runnable example (test-category rules)
├── bad_test.exs     # runnable example (test-category rules)
├── good.exs         # code-category rules, non-test
├── bad.exs          # code-category rules, non-test
├── good.<ext>       # Rust/Swift/Lua — textual only, see CI-LIMITATIONS.md
└── bad.<ext>        # ditto
```

- Agnostic rules (`rules/agnostic/<slug>/`) omit example files entirely and cite `concretised_by:` language-specific rules.
- Elixir rules have runnable `.exs` examples validated by `mix test`.
- Rust/Swift/Lua rules have textual examples embedded in `RULE.md` (see `CI-LIMITATIONS.md`).

## Frontmatter

YAML frontmatter at the top of every `RULE.md`, between `---` delimiters. All top-level fields are scalars, strings, or flat arrays. No nested maps (keeps the bash+jq index generator trivial; see `index-generator.md`).

### Required fields

| Field          | Type         | Purpose                                                                                           |
| -------------- | ------------ | ------------------------------------------------------------------------------------------------- |
| `id`           | string       | Rule identifier. Format: `IN-<LANG>-<CAT>-<NNN>`. See `id-scheme.md`.                             |
| `title`        | string       | Human-readable one-line name. Matches the H1 heading in the body.                                 |
| `language`     | enum         | One of `agnostic`, `elixir`, `rust`, `swift`, `lua`. Drives rule-pack location.                   |
| `category`     | string       | Kebab-case category slug (`code`, `test`, `ash`, `phoenix`, `lv`, `architecture`, etc.).          |
| `severity`     | enum         | One of `critical`, `warning`, `recommendation`, `style`.                                          |
| `summary`      | string       | One or two sentences. YAML multiline with `>` encouraged. Shown in `intent claude rules list`.    |
| `principles`   | list[string] | One or more principle short-names. Intent's agnostic principles plus upstream's for Elixir rules. |
| `applies_when` | list[string] | Natural-language circumstances under which the rule applies. For humans and Claude.               |

### Optional fields

| Field                 | Type         | Purpose                                                                                                                                                                                                                   |
| --------------------- | ------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `upstream_id`         | string       | Intent-specific. Slug of the upstream elixir-test-critic rule this borrows from. Required when principle or detection is lifted from upstream. See `attribution-policy.md`.                                               |
| `applies_to`          | list[glob]   | Intent-specific. Machine-readable glob patterns. Used by tooling to narrow file sets. Example: `["test/**/*_test.exs"]`.                                                                                                  |
| `references`          | list[id]     | Intent-specific. Cross-rule citations by Intent ID. Example: `[IN-AG-HIGHLANDER-001]`. Distinct from `related_rules`: `references` implies "this rule concretises or depends on"; `related_rules` is a softer suggestion. |
| `concretised_by`      | list[id]     | Required on agnostic rules; forbidden on language rules. Lists at least 2 language-specific rule IDs that demonstrate this principle. Prevents agnostic rules becoming vague wisdom.                                      |
| `aliases`             | list[string] | Previous slugs for this rule. Supports rename without ID changes. Empty array `[]` by default.                                                                                                                            |
| `tags`                | list[string] | Discovery keywords. No enforced vocabulary.                                                                                                                                                                               |
| `related_rules`       | list[id]     | Softer cross-reference than `references`. Rules that are worth reading together but do not imply dependency.                                                                                                              |
| `sources`             | list[url]    | URLs to supporting docs, blog posts, conference talks, library docs.                                                                                                                                                      |
| `conflicts_with`      | list[id]     | Rule IDs that contradict this one. Rare; typically indicates an opinionated style split.                                                                                                                                  |
| `does_not_apply_when` | list[string] | Natural-language exceptions. Content mirrors the `## When This Does Not Apply` Markdown section; frontmatter version is for tooling filters.                                                                              |
| `status`              | enum         | `active` (default), `draft`, `deprecated`. Only `active` rules are enforced by Critics. Defaults to `active` if omitted.                                                                                                  |
| `version`             | integer      | Rule-content version. Bump on breaking changes to Detection or Problem framing. Starts at `1`.                                                                                                                            |

### Forbidden in frontmatter

- Nested maps (e.g. `detection: { pattern: ..., severity: ... }`). Keep everything flat.
- Free-form keys outside this schema. `intent claude rules validate` rejects unknown top-level keys. If new metadata is needed, update this schema first.

### Example frontmatter

Elixir test rule (Intent rule borrowing upstream principle):

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
applies_when:
  - "Any ExUnit test asserting on a return value of a fallible function"
  - "Assertions on struct fields, map values, or list contents"
applies_to:
  - "test/**/*_test.exs"
does_not_apply_when:
  - "Property-based tests that assert invariants rather than specific values"
  - "Tests where the shape itself is the contract (e.g. GenServer callback returns)"
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
  - IN-EX-CODE-001
  - IN-RS-CODE-002
aliases: []
status: active
version: 1
---
```

## Markdown body

Sections appear in this fixed order. Every rule has all nine; empty sections are explicitly marked "N/A" rather than omitted, so readers can tell the author considered and rejected the section.

```markdown
# <Title matching frontmatter>

<One-line restatement of the rule. Not a section heading; sits between H1 and "## Problem".>

## Problem

<Concrete scenario of what goes wrong when the rule is violated. Name the failure
mode. Cite a real incident or pattern where possible. 2-6 paragraphs.>

## Detection

<How a reviewer or Critic subagent spots a violation. Grep patterns, AST signals,
structural heuristics. This is guidance for the Critic, not a prescriptive regex.
1-3 paragraphs plus a bulleted list of signals.>

## Bad

<Fenced code block showing the antipattern. Cross-link to bad_test.exs / bad.exs
/ bad.<ext> for runnable or textual form. Keep the inline snippet under 15 lines.>

## Good

<Fenced code block showing the correct pattern. Cross-link to good_test.exs /
good.exs / good.<ext>.>

## When This Applies

<Expanded form of the `applies_when:` frontmatter list. Discusses nuance.>

## When This Does Not Apply

<Expanded form of `does_not_apply_when:`. Substantive — prevents Critic noise.
If there are no exceptions, write "No known exceptions." and move on.>

## Further Reading

<Bulleted list of URLs, book references, other rule cross-links. Each entry is
a complete citation (title + source), not a bare URL.>
```

### Section headings are load-bearing

- Use exact H2 headings (`## Problem`, `## Detection`, etc.) — no variations. The validator greps for these verbatim.
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

**Both `good_*.exs` and `bad_*.exs` must exit 0 when run.** This is a deliberate upstream convention: rule violations are detected by the Critic reading the source, not by runtime failure. The `bad` example demonstrates what a Critic would flag; ExUnit itself is not the enforcer.

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

The files use `Mix.install([])` + `ExUnit.start(autorun: true)` and run standalone — no surrounding Mix project required. `mix test <path>` also works when the caller is inside a Mix project, but standalone `elixir` is the reference form because it is what `intent claude rules validate` (WP02) will invoke.

The first non-empty line of each file must be `# EXPECTED: passes` (upstream convention — enforced by the validator). Other permitted values (`failure`, `flaky`) are reserved for upstream-style rules where runtime failure is intentional; Intent rules use `passes`.

## Runnable example contract (Rust / Swift / Lua)

**Textual only in v2.9.0.** No `good.rs` / `bad.rs` / etc. files. Examples are fenced code blocks inside the `## Bad` and `## Good` Markdown sections.

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

## Field consumers (every field must have a consumer)

Anti-bloat invariant: no field in this schema exists without a named consumer. The consumers are:

| Field                 | Claude reads | `intent claude rules` reads | `intent claude rules index` reads | Critic subagent reads  |
| --------------------- | :----------: | :-------------------------: | :-------------------------------: | :--------------------: |
| `id`                  |      ✓       |              ✓              |                 ✓                 |           ✓            |
| `title`               |      ✓       |              ✓              |                 ✓                 |           ✓            |
| `language`            |      ✓       |              ✓              |                 ✓                 |      ✓ (dispatch)      |
| `category`            |      ✓       |              ✓              |                 ✓                 |           ✓            |
| `severity`            |      ✓       |              ✓              |                 ✓                 |       ✓ (filter)       |
| `summary`             |      ✓       |          ✓ (list)           |                 —                 |           —            |
| `principles`          |      ✓       |              —              |                 ✓                 |           ✓            |
| `applies_when`        |      ✓       |              —              |                 —                 |           ✓            |
| `upstream_id`         |      —       |    ✓ (attribution check)    |                 ✓                 |  ✓ (upstream dedupe)   |
| `applies_to`          |      —       |              —              |                 ✓                 |    ✓ (file filter)     |
| `references`          |      ✓       |    ✓ (validate resolves)    |                 ✓                 |           ✓            |
| `concretised_by`      |      ✓       |   ✓ (validate invariant)    |                 ✓                 |           —            |
| `aliases`             |      ✓       |         ✓ (lookup)          |                 ✓                 |           —            |
| `tags`                |      —       |              —              |                 ✓                 |           —            |
| `related_rules`       |      ✓       |              —              |                 ✓                 |           —            |
| `sources`             |      ✓       |              —              |                 —                 |           —            |
| `conflicts_with`      |      ✓       |        ✓ (validate)         |                 ✓                 |           —            |
| `does_not_apply_when` |      ✓       |              —              |                 —                 |       ✓ (filter)       |
| `status`              |      —       |         ✓ (filter)          |                 ✓                 | ✓ (skip if not active) |
| `version`             |      —       |              ✓              |                 —                 |           —            |

Fields with zero ✓s are candidates for removal. None currently.

## Adding a new rule (quick reference)

1. Identify the rule pack. Use `language` + `category` to find the directory. See `DECISION_TREE.md` (post-WP10) for placement.
2. Assign an ID. Use the next free `IN-<LANG>-<CAT>-<NNN>`. Never reuse a numeric suffix, including for deleted rules.
3. Copy the archetype at `_schema/archetype/strong-assertions/` as a template.
4. Fill frontmatter per this schema.
5. Write the nine Markdown sections.
6. Author runnable examples (Elixir) or textual examples (Rust/Swift/Lua).
7. Run `intent claude rules validate <id>` (once WP02 lands).
8. Ensure skills or Critic subagents that reference the rule have the new ID in their `rules:` list.
9. If the rule borrows from upstream, add `upstream_id:` and update `_attribution/elixir-test-critic.md` per `attribution-policy.md`.

## Schema evolution

- **Adding optional fields**: minor version bump; backwards compatible.
- **Adding required fields**: major version bump; migration needed. Extremely rare.
- **Removing fields**: major version bump; must migrate existing rules first.
- **Renaming sections**: forbidden without a validator update and a migration pass.

Schema changes are proposed via a new ST, not a WP inside an existing ST.
