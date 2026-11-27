# Writing Intent Extensions

User extensions are Intent's declared route for adding subagents, skills, or rule packs without forking it. In this build the route is declared and not built (below): an extension changes nothing. Extensions are content-only — no executable code is loaded. This document is the authoring guide.

## What is an extension?

An extension is a self-contained directory at `~/.local/share/intent/ext/<name>/` that contributes subagents, skills, or rule packs into Intent. Each extension declares its contributions in an `extension.json` manifest.

**`ext` is declared and not built.** `intent ext` and its subcommands `list`, `show`, `validate` and `new` are listed by `intent --help` and `intent ext --help`, and every one refuses at exit 2 with ``error: `ext` is a known command that is not implemented yet``. No command in this build reads `~/.local/share/intent/ext/`: rules, skills and subagents resolve from the install's canon only, and `intent claude rules validate` says so on stderr (`note: extension rule packs were NOT validated ...`). `$INTENT_EXT_DIR` and `$INTENT_EXT_DISABLE` are read by nothing. The layout and manifest below are what `intent/plugins/claude/ext-schema/extension.schema.json` defines; nothing in this build validates against it.

## When an extension would apply

The route is meant for when you want to:

- Add a subagent or skill that is only useful in your projects, your organisation, or a specific domain.
- Override a canon subagent or skill with your own version without forking Intent.
- Ship a rule pack (eg language-specific patterns that Intent does not cover) that a Critic subagent could consume.

If you want the change to be part of Intent itself, open an issue or PR against the canon repository rather than writing an extension.

## Anatomy of an extension

```
~/.local/share/intent/ext/<name>/
  extension.json              # required manifest
  README.md                   # recommended
  LICENSE                     # optional but strongly recommended
  subagents/<name>/           # zero or more
    agent.md
    metadata.json
  skills/<slug>/              # zero or more
    SKILL.md
  rules/<lang>/<cat>/<slug>/  # zero or more
    RULE.md
    good_test.exs             # Elixir only (per CI-LIMITATIONS.md)
    bad_test.exs              # Elixir only
```

The `extension.json` manifest is the single source of truth for what the extension contributes.

## The manifest

See `intent/plugins/claude/ext-schema/extension.schema.json` for the full JSON Schema. Required top-level fields:

- `schema` — must be the literal `"intent-extension/v1"`.
- `name` — lowercase letters, digits, hyphens. 2–64 characters. Must match the directory basename under `~/.local/share/intent/ext/`.
- `version` — semantic version (`MAJOR.MINOR.PATCH`, optional pre-release suffix).

Recommended fields:

- `description` — at most 280 characters.
- `author`, `license`, `homepage`.
- `intent_compat: { min, max }` — version bounds for the Intent versions this extension supports. `min` (`MAJOR.MINOR.PATCH`) is required whenever `intent_compat` is present; `max` also accepts `2.x` / `3.x` style ranges.
- `contributes: { subagents: [...], skills: [...], rules: [...] }` — array of `{ name, path }` objects declaring each contributed item.
- `checksums` — optional per-file SHA for tamper detection. Nothing checks it.

Unknown top-level keys are rejected by the schema. If you need new metadata, open an issue against canon — the schema is intentionally strict.

## Publishing

There is no registry, no `intent ext install <url>`, and no push/pull workflow.

## See also

- `intent ext --help`: the `intent ext` command reference
- `intent/plugins/claude/ext-schema/extension.schema.json`: the manifest's JSON Schema
- `intent/docs/rules.md`: the rule authoring guide
- `intent/docs/critics.md`: the critic subagent contract, where rules are consumed
- `intent/docs/creating-custom-agents.md`: the authoring guide for canon subagents
