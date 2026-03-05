@short: Capture project learnings for future LLM sessions

# intent learn

Capture project-specific learnings that persist across LLM sessions.

## Synopsis

```
intent learn [options] "description"
```

## Description

The learn command accumulates project-specific knowledge (footguns, patterns that worked, patterns that failed) in `.intent/learnings.md`. These learnings are consumed by `intent claude prime` for injection into Claude Code's persistent memory.

## Commands

### Add a learning

```
intent learn "description"                    # Footgun (default)
intent learn --category worked "description"  # Pattern that worked
intent learn --category failed "description"  # Pattern that failed
```

### List learnings

```
intent learn --list
```

### Help

```
intent learn help
```

## Options

- `--category CAT` -- Category: footgun (default), worked, failed
- `--list` -- Show all learnings

## Storage Format

Learnings are stored in `.intent/learnings.md`:

```markdown
# Project Learnings

## Footguns

- 2026-03-04: Never use Map.get on User struct

## Patterns That Worked

- 2026-03-04: Ash bulk actions handle 10k+ rows fine

## Patterns That Failed

- 2026-03-04: Ecto.Multi nesting caused deadlocks
```

## Integration

Learnings are consumed by `intent claude prime` and injected into MEMORY.md under "Known Footguns".

## Examples

```bash
# Add a footgun
intent learn "Never use Map.get on User struct"

# Add a pattern that worked
intent learn --category worked "Ash bulk actions handle 10k+ rows fine"

# Add a pattern that failed
intent learn --category failed "Ecto.Multi nesting caused deadlocks"

# List all learnings
intent learn --list
```

## See Also

- `intent help audit` -- Automated code quality checks
- `intent claude prime` -- Pre-load Claude Code project memory
