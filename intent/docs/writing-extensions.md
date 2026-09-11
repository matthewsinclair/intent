# Writing Intent Extensions

User extensions let you add subagents, skills, or rule packs to Intent without forking it. Extensions are content-only — no executable code is loaded. This document is the authoring guide.

## What is an extension?

An extension is a self-contained directory at `~/.intent/ext/<name>/` that contributes subagents, skills, or rule packs into Intent. Each extension declares its contributions in an `extension.json` manifest. Discovery is layered: canon remains the default; user extensions override by name with a visible shadow warning. Setting `INTENT_EXT_DISABLE=1` reverts to canon-only for the duration of a command.

## When to build an extension

Build an extension when you want to:

- Add a subagent or skill that is only useful in your projects, your organisation, or a specific domain.
- Override a canon subagent or skill with your own version without forking Intent.
- Ship a rule pack (e.g. language-specific patterns that Intent does not cover) that a Critic subagent can consume.

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

The `extension.json` manifest is the single source of truth for what the extension contributes. Only paths declared in `contributes` are exposed to discovery — stray files are ignored.

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

## Discovery and precedence

Discovery order (highest precedence first):

1. `$INTENT_EXT_DIR` — env override (used by tests).
2. `~/.intent/ext/*/` — user extensions.
3. `$INTENT_HOME/intent/plugins/claude/{subagents,skills,rules}/` — canon.

When an extension ships a subagent or skill with the same name as a canon entry, the extension wins but a shadow warning is emitted on every `intent claude subagents list|show|install` (and the equivalent for skills). No silent shadowing.

For rules, the same shadowing applies by `id:` rather than directory name — an extension rule with `id: IN-EX-CODE-006` overrides the canon rule with the same ID, and the Critic subagent prints a shadow warning at the top of the report.

## Safety

- `intent ext validate` rejects absolute paths and any `..` segment in `contributes.*.path` before touching the filesystem.
- The manifest must parse as JSON and pass the schema check; malformed manifests are tagged in `intent ext list` but do not crash discovery.
- Unknown top-level manifest keys are rejected (schema is strict).
- Intent never executes extension code in v2.9.0; extensions are content-only.

## Commands

- `intent ext list` — enumerate installed extensions with status.
- `intent ext show <name>` — manifest + contributions + shadow warnings.
- `intent ext validate [<name>]` — schema + traversal + compat + contribution-existence checks.
- `intent ext new <name> --subagent | --skill | --rule-pack` — scaffold a valid skeleton.

See `intent help ext` for full usage.

## Creating your own extension

### Scaffold

```bash
intent ext new my-agent --subagent
```

Generates:

```
~/.intent/ext/my-agent/
├── extension.json
├── README.md
└── subagents/
    └── my-agent/
        ├── agent.md
        └── metadata.json
```

The generated extension passes `intent ext validate` immediately. The same flag applies for `--skill` and `--rule-pack`:

```bash
intent ext new my-skill --skill
intent ext new my-rules --rule-pack
```

### Author the contribution

Edit the generated `agent.md` (or `SKILL.md`, or `RULE.md`) and update the manifest's `description` to something meaningful. For subagents and skills, the same authoring guidance as canon applies — see `intent/docs/creating-custom-agents.md` and the existing canon `SKILL.md` files for examples. For rule packs, follow `intent/docs/rules.md`.

### Validate

```bash
intent ext validate my-agent
```

What this checks:

- Manifest parses as JSON and conforms to the schema.
- `name` matches the directory basename.
- `version` is a valid semver.
- Every `contributes.*.path` exists and contains no `..` segment.
- No absolute paths in `contributes`.
- `intent_compat.min` is satisfied by the running Intent version.

A clean run prints `ok:` and exits 0. Failures print `error:` lines and exit non-zero.

### Install

For subagents and skills, install via the existing plugin commands — extensions are transparent:

```bash
intent claude subagents install my-agent
intent claude skills install my-skill
```

Rule-pack contributions don't need explicit install — they participate in Critic discovery automatically. Verify with:

```bash
intent claude rules list
intent claude rules validate
```

The list output tags ext-supplied rules with `[ext:<name>]` provenance.

## Debugging extensions

### Manifest doesn't validate

Run `intent ext validate <name>` to see the specific failure. Common causes:

- `schema` not the literal `"intent-extension/v1"`.
- `name` doesn't match the directory basename.
- `intent_compat.min` is higher than the running Intent version.
- `contributes.*.path` points at a directory that doesn't exist.

### Extension visible in `ext list` but not in `subagents list`

Check the manifest's `contributes` block. Only paths declared there are exposed to discovery — a `subagents/foo/` directory on disk that isn't listed in `contributes.subagents` is ignored.

### Shadow warning is unexpected

When you see `(shadow: canon entry exists at <path>)`, it means your extension and canon both ship a subagent or skill with the same name. The extension wins, but the warning is permanent — there's no way to "accept" a shadow and silence the warning. If you want to take over the canon name, that's the working state. If you didn't intend to shadow, rename your extension's contribution.

### Extension ignored entirely

Check `INTENT_EXT_DISABLE`. If set to `1`, all extensions are suppressed and Intent operates canon-only:

```bash
echo $INTENT_EXT_DISABLE
unset INTENT_EXT_DISABLE
```

## Publishing (deferred)

In v2.9.0, extensions are local-only. There is no registry, no `intent ext install <url>`, no push/pull workflow. Distribution is entirely user-managed: copy `~/.intent/ext/<name>/` between machines, or commit a directory to a personal Git repo and clone it under `~/.intent/ext/`.

A registry, signing, and remote-install workflow remains future work (not yet scheduled). The schema's `checksums` and `homepage` fields exist in anticipation — recommended now, enforced later.

## See also

- `intent help ext` — `intent ext` command reference
- `intent/plugins/claude/ext-schema/extension.schema.json` — manifest JSON Schema
- `intent/docs/rules.md` — rule authoring guide (for `--rule-pack` extensions)
- `intent/docs/critics.md` — Critic subagent contract (rules are consumed here)
- `intent/docs/creating-custom-agents.md` — authoring guide for canon and extension subagents
