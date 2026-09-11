# Writing Intent Extensions

User extensions let you add subagents, skills, or rule packs to Intent without forking it. Extensions are content-only — no executable code is loaded. This document is the authoring guide.

## What is an extension?

An extension is a self-contained directory at `~/.intent/ext/<name>/` that contributes subagents, skills, or rule packs into Intent. Each extension declares its contributions in an `extension.json` manifest.

**`ext` is declared and not built.** `intent ext` and its subcommands `list`, `show`, `validate` and `new` are listed by `intent --help` and `intent ext --help`, and every one refuses at exit 2 with ``error: `ext` is a known command that is not implemented yet``. No command in this build reads `~/.intent/ext/`: rules, skills and subagents resolve from the install's canon only, and `intent claude rules validate` says so on stderr (`note: extension rule packs were NOT validated ...`). `$INTENT_EXT_DIR` and `$INTENT_EXT_DISABLE` are read by nothing. The layout and manifest below are what `intent/plugins/claude/ext-schema/extension.schema.json` defines; nothing in this build validates against it.

## When to build an extension

Build an extension when you want to:

- Add a subagent or skill that is only useful in your projects, your organisation, or a specific domain.
- Override a canon subagent or skill with your own version without forking Intent.
- Ship a rule pack (eg language-specific patterns that Intent does not cover) that a Critic subagent can consume.

If you want the change to be part of Intent itself, open an issue or PR against the canon repository rather than writing an extension.

## Anatomy of an extension

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
  rules/<lang>/<cat>/<slug>/  # zero or more
    RULE.md
    good_test.exs             # Elixir only (per CI-LIMITATIONS.md)
    bad_test.exs              # Elixir only
```

The `extension.json` manifest is the single source of truth for what the extension contributes.

## The manifest

See `intent/plugins/claude/ext-schema/extension.schema.json` for the full JSON Schema. Required top-level fields:

- `schema` — must be the literal `"intent-extension/v1"`.
- `name` — lowercase letters, digits, hyphens. 2–64 characters. Must match the directory basename under `~/.intent/ext/`.
- `version` — semantic version (`MAJOR.MINOR.PATCH`, optional pre-release suffix).

Recommended fields:

- `description` — under 280 chars.
- `author`, `license`, `homepage`.
- `intent_compat: { min, max }` — version bounds for the Intent versions this extension supports. `max` accepts `2.x` / `3.x` style ranges.
- `contributes: { subagents: [...], skills: [...], rules: [...] }` — array of `{ name, path }` objects declaring each contributed item.
- `checksums` — optional per-file SHA for tamper detection (currently informational; enforcement is future work, not yet scheduled).

Unknown top-level keys are rejected by the schema. If you need new metadata, open an issue against canon — the schema is intentionally strict.

## Commands

`intent ext --help` lists `list`, `show <name>`, `validate [<name>]` and `new <name> --subagent | --skill | --rule-pack`. Each refuses at exit 2 in this build: ``error: `ext` is a known command that is not implemented yet``.

See `intent ext --help` for full usage.

## Install

`intent claude subagents install` and `intent claude skills install` resolve canon only in this build, so an extension's subagent or skill cannot be installed through them.

Rule packs under `~/.intent/ext/` are not reached in this build: `intent claude rules list` prints `canon` in its `prov` column for every rule, and `intent claude rules validate` notes on stderr that extension packs were not validated.

## Publishing (deferred)

There is no registry, no `intent ext install <url>`, and no push/pull workflow.

A registry, signing, and remote-install workflow remains future work (not yet scheduled). The schema's `checksums` and `homepage` fields exist in anticipation — recommended now, enforced later.

## See also

- `intent ext --help` -- `intent ext` command reference
- `intent/plugins/claude/ext-schema/extension.schema.json` — manifest JSON Schema
- `intent/docs/rules.md` — rule authoring guide (for `--rule-pack` extensions)
- `intent/docs/critics.md` — Critic subagent contract (rules are consumed here)
- `intent/docs/creating-custom-agents.md` — authoring guide for canon subagents
