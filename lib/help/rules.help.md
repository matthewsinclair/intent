@short: Enumerate, show, validate, and index Intent rules

# intent claude rules

Manage Intent's rule library — the single source of truth that Critic subagents enforce against code and tests.

## Synopsis

```
intent claude rules <command> [args]
```

## Description

Intent rules live at `intent/plugins/claude/rules/<lang>/<category>/<slug>/RULE.md` (canon) and `~/.intent/ext/<name>/rules/<lang>/<category>/<slug>/RULE.md` (user extensions). Each rule is a small Markdown file with YAML frontmatter that encodes one principle or antipattern, with runnable good/bad examples where the language permits.

The `intent claude rules` command surface exposes the rule library to Bash consumers. Canonical operations:

- **Enumerate**: `list` shows every rule across canon + extensions, with provenance tags.
- **Inspect**: `show <id>` prints a single rule's Markdown body.
- **Validate**: `validate [<id>|<path>]` checks frontmatter, required sections, cross-references, and attribution presence.
- **Index**: `index` regenerates the canonical `rules/index.json` for fast jq-driven consumption.

The library is populated across the agnostic, Elixir, Rust, Swift, Lua, and Shell rule packs. Extensions can contribute additional rule packs at `~/.intent/ext/<name>/rules/<lang>/<category>/<slug>/RULE.md`.

## Commands

### list

Enumerate rules across canon + extensions. Filter with `--lang` or `--severity`.

```
intent claude rules list
intent claude rules list --lang elixir
intent claude rules list --severity critical
```

Output columns: `id  severity  language/category  provenance  title`.

### show

Print a rule's Markdown body with a provenance header.

```
intent claude rules show IN-EX-TEST-001
```

### validate

Validate rule frontmatter and required sections. With no args, validates every rule across canon + extensions. With one arg, validates a single rule by id or path.

```
intent claude rules validate
intent claude rules validate IN-EX-TEST-001
intent claude rules validate intent/plugins/claude/rules/elixir/test/my-rule/RULE.md
```

Checks performed:

- YAML frontmatter present and parseable
- Required scalar fields: `id`, `title`, `language`, `category`, `severity`, `summary`
- `id` matches `^IN-(AG|EX|RS|SW|LU)-[A-Z][A-Z0-9-]*-[0-9]{3}$`
- `severity` is one of `critical | warning | recommendation | style`
- Required H2 sections present: `Problem`, `Detection`, `Bad`, `Good`, `When This Applies`, `When This Does Not Apply`, `Further Reading`
- H1 heading present
- If `upstream_id:` set, `_attribution/elixir-test-critic.md` exists (warning if the specific slug is not yet listed)
- Every `references:` id resolves to another known rule

### index

Regenerate `intent/plugins/claude/rules/index.json` from every canon RULE.md. The result is sorted by `id` and pretty-printed with `jq -S` so running `index` twice on an unchanged tree produces byte-identical output.

```
intent claude rules index
```

Extensions are **not** included in the canon index; ext-shipped rules are discovered at runtime.

## Environment

| Variable             | Effect                                      |
| -------------------- | ------------------------------------------- |
| `INTENT_EXT_DIR`     | Override default `~/.intent/ext/` for tests |
| `INTENT_EXT_DISABLE` | Set to `1` to ignore ext rules entirely     |

## See Also

- `intent help ext` — user extensions
- `intent/docs/rules.md` — rule authoring guide (schema, IDs, validation, attribution)
- `intent/docs/critics.md` — Critic subagent contract (rules are consumed here)
- `intent/plugins/claude/rules/_schema/rule-schema.md` — authoritative frontmatter reference
- `intent/plugins/claude/rules/_schema/index-generator.md` — pipeline spec for `index` subcommand
