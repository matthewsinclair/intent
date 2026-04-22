# Writing Intent Extensions

**Status**: WP02 skeleton. Full worked example (`worker-bee` as a user extension) lands in WP10 once WP08 extracts worker-bee from canon.

## What is an extension?

A user extension is a self-contained directory at `~/.intent/ext/<name>/` that contributes subagents, skills, or rule packs into Intent without modifying canon. Extensions are content-only — no executable code is loaded. Discovery is layered: canon remains the default; user extensions override by name with a visible shadow warning. Setting `INTENT_EXT_DISABLE=1` reverts to canon-only for the duration of a command.

## When to build an extension

Build an extension when you want to:

- Add a subagent or skill that is only useful in your projects, your organisation, or a specific domain.
- Override a canon subagent or skill with your own version without forking Intent.
- Ship a rule pack (e.g. language-specific patterns that Intent does not cover) that a Critic subagent can consume.

If you want the change to be part of Intent itself, open an issue or PR against the canon repository rather than writing an extension.

## Anatomy

```
~/.intent/ext/<name>/
  extension.json              # required manifest
  README.md                   # recommended
  LICENSE                     # optional but strongly recommended
  subagents/<name>/           # zero or more
    agent.md
    metadata.json
  skills/<slug>/              # zero or more
    SKILL.md
  rules/<lang>/<slug>/        # zero or more
    RULE.md
    good_test.exs             # Elixir only
    bad_test.exs              # Elixir only
```

The `extension.json` manifest is the single source of truth for what the extension contributes. Only paths declared in `contributes` are exposed to discovery — stray files are ignored.

## Manifest

See `intent/plugins/claude/ext-schema/extension.schema.json` for the full JSON Schema. Required top-level fields:

- `schema` — must be the literal `"intent-extension/v1"`.
- `name` — lowercase, digits, hyphens. Must match the directory basename under `~/.intent/ext/`.
- `version` — semantic version (MAJOR.MINOR.PATCH).

Recommended fields:

- `description`, `author`, `license`, `homepage`
- `intent_compat: { min, max }` — version bounds for the Intent versions this extension supports
- `contributes: { subagents: [...], skills: [...], rules: [...] }` — array of `{ name, path }` objects declaring each contributed item
- `checksums` — optional per-file SHA for tamper detection

## Scaffolding

Use `intent ext new` to generate a valid skeleton:

```bash
intent ext new my-agent --subagent
intent ext new my-skill --skill
intent ext new my-rules --rule-pack

intent ext validate my-agent
```

The generated extension passes `intent ext validate` immediately.

## Discovery and precedence

Discovery order (highest precedence first):

1. `$INTENT_EXT_DIR` — env override (used by tests)
2. `~/.intent/ext/*/` — user extensions
3. `$INTENT_HOME/intent/plugins/claude/{subagents,skills,rules}/` — canon

When an extension ships a subagent or skill with the same name as a canon entry, the extension wins but a shadow warning is emitted on every `intent claude subagents list|show|install` (and the equivalent for skills). No silent shadowing.

## Safety

- `intent ext validate` rejects absolute paths and any `..` segment in `contributes.*.path` before touching the filesystem.
- The manifest must parse as JSON and pass the schema check; malformed manifests are tagged in `intent ext list` but do not crash discovery.
- Unknown top-level manifest keys are rejected (schema is strict).
- Intent never executes extension code in v2.9.0; extensions are content-only.

## Commands

- `intent ext list` — enumerate installed extensions with status
- `intent ext show <name>` — manifest + contributions + shadow warnings
- `intent ext validate [<name>]` — schema + traversal + compat + contribution-existence checks
- `intent ext new <name> --subagent | --skill | --rule-pack` — scaffold a valid skeleton

See `intent help ext` for full usage.

## Worked example

**Lands in WP10.** The canonical worked example is `worker-bee`: WP08 extracts the current canon `worker-bee` subagent into `lib/templates/ext-seeds/worker-bee/`, and WP09's migration seeds `~/.intent/ext/worker-bee/` on upgrade. This document will then walk through the full lifecycle — scaffolding, authoring, validation, installation, shadow detection — using worker-bee as the reference.

## See also

- `intent help ext` — `intent ext` command reference
- `intent/plugins/claude/ext-schema/extension.schema.json` — manifest JSON Schema
- `intent/plugins/claude/rules/_schema/rule-schema.md` — rule authoring schema (for `--rule-pack` extensions)
- `intent/docs/rules.md` — rule authoring guide (WP10)
- `intent/docs/critics.md` — Critic subagent contract (WP10)
