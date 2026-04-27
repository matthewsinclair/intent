@short: Per-language canon: install language-specific RULES + ARCHITECTURE

# intent lang

Install language-specific canon templates (`RULES-<lang>.md`, `ARCHITECTURE-<lang>.md`) into a project, and register the language in the agnostic `intent/llm/RULES.md` Language Packs section.

## Synopsis

```
intent lang <command> [options]
```

## Description

Intent's canon-installer ships language-agnostic `_default` templates by design (one project might be Elixir, another Rust, another a polyglot mix; auto-detection is rejected per ST0035 design decision). The `intent lang` command provides explicit, user-driven per-language setup.

`intent lang init <lang>` installs the language-specific `RULES-<lang>.md` (and optionally `ARCHITECTURE-<lang>.md`) into `intent/llm/`, then appends an entry to the agnostic `intent/llm/RULES.md` Language Packs section pointing at the language's rule pack at `intent/plugins/claude/rules/<lang>/`.

Re-runs are safe (idempotent). Multi-language is the default case (`intent lang init elixir rust shell`).

## Commands

### list

List languages with available templates.

```
intent lang list
```

Enumerates the directories under `intent/plugins/agents/templates/` (excluding `_default/`), one language per line.

### show

Show what `intent lang init <lang>` installs.

```
intent lang show <lang>
```

Prints the source template directory and the destination paths for each file that would be installed.

### init

Install per-language canon. Idempotent. Multi-language.

```
intent lang init <lang> [<lang> ...]
```

For each named language:

1. Validates that `intent/plugins/agents/templates/<lang>/` exists.
2. Copies `RULES.md` to `intent/llm/RULES-<lang>.md` (skip if checksum identical).
3. Copies `ARCHITECTURE.md` to `intent/llm/ARCHITECTURE-<lang>.md` if present (skip if checksum identical).
4. Appends an entry to the `Language Packs` section in `intent/llm/RULES.md` (skip if the language is already listed). The marker pair (`<!-- intent-lang-packs:start -->` / `<!-- intent-lang-packs:end -->`) is created on first run.

Failure of one language does not abort the others (fail-forward). Summary reports per-language status.

### help

Show usage.

```
intent lang help
```

## Examples

```bash
# See what's available
intent lang list

# Preview what an install would do
intent lang show elixir

# Install one language
intent lang init elixir

# Install multiple languages
intent lang init rust shell

# Re-run is a no-op (zero diff)
intent lang init elixir
```

## Environment Variables

- `INTENT_LANG_TEMPLATES_DIR` -- override the default templates directory (used by tests).

## See Also

- `intent help agents` -- AGENTS.md generator (uses the same per-language template directories for `intent agents init --template <lang>`)
- `intent help claude` -- Claude Code plugin (canon-installer)
- `intent help init` -- project initialisation (`intent init --lang <list>` invokes `intent lang init` post-init)
