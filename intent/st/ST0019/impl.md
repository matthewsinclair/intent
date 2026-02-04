# Implementation - ST0019: Treeindex

## Status

Design complete (2026-02-04). Implementation not yet started. All three work packages (WP01, WP02, WP03) are pending.

## Implementation Notes

### Claude Headless Invocation

The CLI command uses `claude -p` (print mode) for LLM summarization:

```bash
echo "$file_contents" | claude -p \
  --model sonnet \
  --no-session-persistence \
  --max-budget-usd 0.02 \
  --tools "" \
  --append-system-prompt "$FORMAT_PROMPT" \
  -- "Generate a .treeindex for $(basename "$dir")/"
```

Key flags:
- `--tools ""` disables tool use (text-in/text-out only)
- `--no-session-persistence` avoids session clutter
- `--max-budget-usd` caps cost per directory
- `--` separates flags from prompt (needed because `--tools ""` can consume next arg)

### Platform Compatibility

The fingerprint function uses `stat` which differs between macOS and Linux:
- macOS: `stat -f '%N %z'`
- Linux: `stat --printf='%n %s\n'`

The script must detect the platform and use the correct variant.

### File Reading Strategy

For summarization, read enough of each file to understand its role but not so much that we blow the context window:
- Source files: first ~80 lines (captures module doc, struct defs, public function heads)
- Config files: full content (typically small)
- Markdown: first 10 lines
- Skip binaries, lock files, generated files

## Challenges & Solutions

*To be populated during implementation.*
