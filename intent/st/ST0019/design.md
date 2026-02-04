# Design - ST0019: Treeindex

## Status

Design complete. WP01 implemented 2026-02-04. See as-built notes below for deviations from original design.

## .treeindex File Format

```markdown
<!-- treeindex v1 fingerprint:a7b3c9f2 generated:2026-02-03T14:30:00Z -->

# accounts/

User identity and authentication domain. Manages user registration, credential
storage, and session lifecycle through Ash resources and domain actions.

## Dirs

- `notifications/` -- Email delivery for auth-related events
- `tokens/` -- Token generation and validation for email confirmation and password reset

## Files

- `accounts.ex` -- Domain module; public API code interface for all account operations
- `user.ex` -- Ash resource; user schema, registration and login actions
- `credential.ex` -- Ash resource; credential storage, multiple auth strategies
```

### Format Rules

- Line 1: HTML comment -- format version, fingerprint (8 hex chars), ISO timestamp
- H1: leaf directory name with trailing `/`
- Summary: 2-3 sentences about purpose and responsibility
- `## Dirs`: alphabetical, one-liner per subdirectory. Omit section if no subdirs
- `## Files`: ordered by architectural importance, one-liner per file
- Max 60 lines. For 20+ files, group by naming pattern

## Fingerprint Design

Based on filenames + file sizes in the immediate directory. Computed by bash:

```bash
treeindex_fingerprint() {
  local dir="$1"
  {
    find "$dir" -maxdepth 1 -type f ! -name '.treeindex' ! -name '.DS_Store' \
      -exec stat -f '%N %z' {} \; 2>/dev/null | sed "s|${dir}/||" | sort
    find "$dir" -maxdepth 1 -type d ! -path "$dir" \
      -exec basename {} \; 2>/dev/null | sort | sed 's/$/ d/'
  } | shasum -a 256 | cut -c1-8
}
```

**Properties**: no mtime dependency (stable across git clones), detects adds/removes/size changes, 8-char compact hash, fast (single `find` + `shasum` pipeline).

**Scope**: immediate contents only. A child file change does not change the parent fingerprint.

## Generation Algorithm

Bottom-up. Leaf directories first, then parents.

1. Walk target directory to specified depth, collect all directories
2. Sort by depth (deepest first)
3. For each directory:
   a. Compute fingerprint; skip if `.treeindex` exists and matches
   b. Read source files (head ~80 lines each for context)
   c. Read child `.treeindex` files for subdirectory context
   d. Call `claude -p` with format instructions and file contents
   e. Prepend fingerprint header to Claude's output
   f. Write `.treeindex`
4. Report progress and summary

## Claude Invocation

Uses `claude -p` (headless print mode) with:

- `--model haiku` -- Claude Haiku 4.5, fast and cost-effective for summarization
- `--no-session-persistence` -- no session clutter
- `--max-budget-usd 0.50` -- safety cap per directory (original $0.02 was too low)
- `--tools ""` -- disable tool use (text-in/text-out only)
- `--append-system-prompt` -- format instructions (stored as top-level single-quoted variable for bash 3.2 compat)

## .treeindexignore

Added during implementation. A gitignore-style file at `intent/.treeindex/.treeindexignore` that controls which directories and files are excluded from indexing:

- Lines ending with `/` match directory names (pruned from find)
- Other lines match file names (glob patterns passed to find `-name`)
- Comments (`#`) and blank lines are ignored
- Auto-created with sensible defaults on first run if missing
- Never overwritten if already present

## Consumption Strategy

CLAUDE.md instructions tell Claude to check `.treeindex` before exploring unfamiliar directories. Not deterministic, but practical -- Claude follows these instructions most of the time. A stale `.treeindex` is still better than no index.

## Alternatives Considered

- **MCP server**: Rejected. Overkill for file-based summaries. Still requires Claude to choose the MCP tool over built-in Read/Glob.
- **Claude Code hooks**: Rejected. Hooks cannot intercept or redirect built-in tool calls.
- **Skill/slash command**: Rejected. Skills are for workflows, not persistent artifacts. The CLI command is the right entry point for generation.
- **Pure CLI (no LLM)**: Rejected. Summarization requires understanding code semantics. AST parsing would be language-specific and shallow.

## As-Built Deviations

| Aspect        | Original Design                  | As-Built                                 | Reason                                              |
|---------------|----------------------------------|------------------------------------------|-----------------------------------------------------|
| Storage       | Inline `.treeindex` in each dir  | Centralized shadow at `intent/.treeindex/`| Keeps source tree clean                             |
| DIR arg       | Optional, defaults to `.`        | Mandatory                                | Prevents accidental project-root indexing            |
| Default depth | 1                                | 2                                        | More useful default coverage                        |
| Model         | sonnet                           | haiku                                    | Cost-effective, Haiku 4.5 handles summarization well|
| Budget cap    | $0.02/dir                        | $0.50/dir                                | $0.02 too low, caused false failures                |
| Ignore config | Hardcoded lists                  | `.treeindexignore` file                  | User-configurable exclusions                        |
| Scope         | Global command                   | Requires project context                 | Shadow dir needs `intent/.treeindex/`               |
| Bash compat   | Not specified                    | Bash 3.2 required                        | macOS `/bin/bash` is 3.2.57                         |
