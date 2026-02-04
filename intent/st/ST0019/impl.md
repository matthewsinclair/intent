# Implementation - ST0019: Treeindex

## Status

All work packages resolved (2026-02-04). WP01 implemented, WP02 skipped, WP03 implemented.

## Implementation Notes

### Claude Headless Invocation

The CLI command uses `claude -p` (print mode) for LLM summarization:

```bash
echo "$prompt" | claude -p \
  --model "$MODEL" \
  --no-session-persistence \
  --max-budget-usd "$MAX_BUDGET" \
  --tools "" \
  --append-system-prompt "$FORMAT_SPEC" 2>/dev/null
```

Key flags:
- `--tools ""` disables tool use (text-in/text-out only)
- `--no-session-persistence` avoids session clutter
- `--max-budget-usd 0.50` caps cost per directory (bumped from $0.02 which was too low)
- `--append-system-prompt` passes the format spec as a system prompt addition
- Prompt is piped via stdin, not passed as a positional argument

### Centralized Shadow Directory

Treeindex files are stored in a shadow tree at `intent/.treeindex/` that mirrors the project structure:

```
$PROJECT_ROOT/lib/my_app/accounts/  ->  intent/.treeindex/lib/my_app/accounts/.treeindex
$PROJECT_ROOT/bin/                  ->  intent/.treeindex/bin/.treeindex
$PROJECT_ROOT/intent/st/ST0019/     ->  intent/.treeindex/intent/st/ST0019/.treeindex
```

The `treeindex_shadow_path()` function resolves any directory (relative or absolute) to its shadow path by stripping the `$PROJECT_ROOT/` prefix and appending `.treeindex`.

### Platform Compatibility

The fingerprint function uses `stat` which differs between macOS and Linux:
- macOS: `stat -f '%N %z'`
- Linux: `stat --printf='%n %s\n'`

The script detects the platform once at startup via `detect_stat_format()`.

### Bash 3.2 Compatibility

macOS ships `/bin/bash` 3.2.57. The script must work under this version. Key constraints:

- **No `mapfile`**: Use `while IFS= read -r line; do arr+=("$line"); done < <(...)` instead
- **No `[[ ]]` for path checks**: Use `[ "${dir#/}" != "$dir" ]` instead of `[[ "$dir" = /* ]]`
- **No heredocs inside `$()`**: The format spec uses single quotes in content (eg "directory's"), which causes bash 3.2 parse failures inside `$()`. Solved by assigning `FORMAT_SPEC` as a top-level single-quoted variable
- **No `+=` for string concat in some contexts**: Use `var="$var addition"` pattern

### .treeindexignore

Gitignore-style config file at `intent/.treeindex/.treeindexignore`. The `load_treeindexignore()` function parses it into:
- `IGNORE_DIRS` (space-separated string) -- used by `collect_directories()` for find prune expressions
- `EXCLUDE_FILE_ARGS` (bash array) -- used by `gather_files()`, `treeindex_fingerprint()`, and `has_content()` as find exclusion arguments

The array approach (`"${EXCLUDE_FILE_ARGS[@]}"`) properly handles glob patterns like `*.beam` without shell expansion.

### File Reading Strategy

For summarization, read enough of each file to understand its role but not so much that we blow the context window:
- Source files: first ~80 lines (captures module doc, struct defs, public function heads)
- All file types treated equally (no special handling for config/markdown)
- Skip files matching `.treeindexignore` patterns

## Challenges & Solutions

### Shadow Path Leakage

**Problem**: When `abs_dir` equals `PROJECT_ROOT` exactly, `${abs_dir#$pr_prefix}` doesn't strip the prefix (because `pr_prefix` has trailing `/` but `abs_dir` doesn't). The full absolute path leaks into the shadow tree, creating paths like `intent/.treeindex/Users/matts/Devel/prj/Intent/...`.

**Solution**: (1) Made DIR mandatory -- no defaulting to `.`. (2) Reject project root as DIR with explicit error. (3) `collect_directories()` only accepts paths strictly under `$PROJECT_ROOT/*`. (4) Safety check in `treeindex_shadow_path()` that errors if prefix stripping fails.

### Self-Referencing Shadow Directory

**Problem**: Running `intent treeindex intent/` would index the `intent/.treeindex/` shadow directory itself, creating recursive paths and a write failure ("Is a directory") when the shadow path for `intent/` collided with the shadow directory.

**Solution**: Added `.treeindex/` to the default `.treeindexignore` patterns so the shadow directory is always excluded from indexing.

### Budget Cap Too Low

**Problem**: Original `--max-budget-usd 0.02` caused Claude invocations to fail with cost-exceeded errors on directories with many files, even though the account had plenty of credit.

**Solution**: Bumped to `$0.50` per directory. A typical Haiku summarization costs well under $0.05.

### Empty IGNORE_DIRS Handling

**Problem**: When `.treeindexignore` is empty or contains no directory patterns, `IGNORE_DIRS` is empty. The find expression `\( \)` (empty group) causes a syntax error.

**Solution**: Check `[ -n "$prune_expr" ]` and conditionally include the prune clause in the find command.

## Files

| File                                 | Lines | Description           |
|--------------------------------------|-------|-----------------------|
| `bin/intent_treeindex`               | 612   | Main CLI command      |
| `tests/unit/treeindex_commands.bats` | ~580  | 38 bats tests         |
| `intent/.treeindex/.treeindexignore` | 27    | Default ignore patterns|
