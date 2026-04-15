# Design - ST0033: cwd-resilient intent CLI dispatch

## Approach

Single-point fix in the `bin/intent` dispatcher. `find_project_root()` in `bin/intent_config:37` already walks up from cwd looking for `.intent/config.json`; `load_intent_config` exports `PROJECT_ROOT`. The dispatcher already validates that `PROJECT_ROOT` is non-empty before dispatching project commands and errors out with "not in an Intent project directory" otherwise.

The fix is one hunk immediately before the `exec` site for project commands:

```bash
export INTENT_ORIG_CWD="$(pwd)"
cd "$PROJECT_ROOT" || { echo "error: cannot enter project root: $PROJECT_ROOT" >&2; exit 1; }
```

All subcommand scripts now execute with cwd == project root. The test suite already exercises this invariant (tests always `cd` to the project before running intent), so no subcommand-level changes are needed for the common paths.

## Design Decisions

**Dispatcher-level `cd`, not per-subcommand path fixes.** Highlander: one place, not twenty. Every subcommand already works correctly when cwd == project root — that's the tested path. Making the dispatcher guarantee the precondition is cheaper, smaller, and less likely to rot than auditing every `.intent/` and `intent/` reference across the codebase.

**Preserve `INTENT_ORIG_CWD`.** A `cd` is destructive information loss for commands that accept path arguments meaningful in the invoker's frame of reference. Exporting `INTENT_ORIG_CWD=$(pwd)` before the `cd` gives subcommands an escape hatch. Only the two subcommands that take path args consult it.

**Relative path args resolved by the subcommand, not the dispatcher.** Generically rewriting `"$@"` in the dispatcher would surprise users who pass e.g. a glob or a value that looks like a path but isn't. Fix at the point of use: `intent_treeindex` (DIR arg) and `intent_fileindex` (STARTDIR arg) each check whether their relative arg resolves against cwd (== project root); if not, and it resolves against `INTENT_ORIG_CWD`, use that. Absolute paths are untouched.

**Plugin/global commands unchanged.** Plugin commands (line 168) and global commands (line 171) exec before `load_intent_config` runs — they don't need a project root and don't get the `cd`.

## Architecture

```
intent <cmd> [args]
  │
  ├── parse command, locate script
  │
  ├── if plugin  ──> exec plugin (no cd, no project context)
  ├── if global  ──> exec global (no cd, no project context)
  │
  ├── source intent_config, load_intent_config
  │     └── find_project_root   (walks up from cwd)
  │         └── sets PROJECT_ROOT (or empty)
  │
  ├── if !PROJECT_ROOT  ──> error "not in an Intent project" + exit 1
  │
  ├── [NEW] export INTENT_ORIG_CWD=$(pwd)
  ├── [NEW] cd "$PROJECT_ROOT"
  │
  └── exec subcommand   (cwd now guaranteed to be project root)
```

## Alternatives Considered

**Per-subcommand `$PROJECT_ROOT/` prefixing.** Rejected. Touches every subcommand, every shared helper (`get_config_field`, `resolve_st_dir`, …), and requires ongoing vigilance. Fails the Highlander rule.

**Rewrite relative args in the dispatcher.** Rejected. `"$@"` parsing without command-specific knowledge produces surprising behavior for non-path args.

**Require users to `cd` to project root.** Rejected. That's the bug, not the fix.
