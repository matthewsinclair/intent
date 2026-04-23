@short: Manage user extensions at ~/.intent/ext/

# intent ext

Manage Intent user extensions.

## Synopsis

```
intent ext [command] [options]
```

## Description

User extensions live under `~/.intent/ext/<name>/` and contribute subagents, skills, or rule packs into Intent without modifying canon. Each extension is self-contained and declares its contributions in an `extension.json` manifest.

Discovery order (highest precedence first):

1. `$INTENT_EXT_DIR` — env override (used by tests)
2. `~/.intent/ext/*/` — user extensions
3. `$INTENT_HOME/intent/plugins/claude/{subagents,skills}/` — canon

When an extension ships a subagent or skill with a name that also exists in canon, the extension wins but a shadow warning is emitted on every `list`, `show`, and `install`. Setting `INTENT_EXT_DISABLE=1` suppresses extension discovery entirely and reverts to canon-only.

## Commands

### list

Enumerate installed extensions with name, version, and validity status.

```
intent ext
intent ext list
```

An absent `~/.intent/ext/` is not an error — `list` reports the empty-set status and exits 0.

### show

Print manifest fields, contribution counts, and per-contribution shadow warnings.

```
intent ext show <name>
```

### validate

Validate extension manifests against the JSON schema, reject path-traversal attempts, and check that every declared contribution path exists.

```
intent ext validate              # validate every extension
intent ext validate <name>       # validate one
```

### new

Scaffold a new extension skeleton with a valid manifest and placeholder contribution. The generated extension passes `intent ext validate` immediately.

```
intent ext new <name> --subagent
intent ext new <name> --skill
intent ext new <name> --rule-pack
```

## Environment

| Variable             | Effect                                             |
| -------------------- | -------------------------------------------------- |
| `INTENT_EXT_DIR`     | Override default `~/.intent/ext/` (tests use this) |
| `INTENT_EXT_DISABLE` | Set to `1` to suppress ext discovery entirely      |

## Examples

```bash
# List extensions
intent ext

# Show details for the worker-bee extension
intent ext show worker-bee

# Install an ext subagent via the subagents plugin (ext wins over canon)
intent claude subagents install worker-bee
```

## See Also

- `intent help claude` — subagents and skills consume extensions through the claude plugin
- `intent help plugin` — the plugin architecture ext builds on
- `intent/docs/writing-extensions.md` — full extension authoring guide with worker-bee worked example
- `intent/plugins/claude/ext-schema/extension.schema.json` — manifest JSON Schema
