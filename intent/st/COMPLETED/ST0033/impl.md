# Implementation - ST0033

## Changes

### `bin/intent`

After the `PROJECT_ROOT` validation block, before the project-command `exec`:

```bash
export INTENT_ORIG_CWD="$(pwd)"
cd "$PROJECT_ROOT" || { echo "error: cannot enter project root: $PROJECT_ROOT" >&2; exit 1; }
```

### `bin/intent_treeindex`

Before the "Verify TARGET_DIR exists" check, resolve relative `TARGET_DIR` against `INTENT_ORIG_CWD` when it does not resolve against cwd:

```bash
case "$TARGET_DIR" in
  /*) ;;
  *)
    if [ -n "${INTENT_ORIG_CWD:-}" ] && [ ! -d "$TARGET_DIR" ] && [ -d "$INTENT_ORIG_CWD/$TARGET_DIR" ]; then
      TARGET_DIR="$INTENT_ORIG_CWD/$TARGET_DIR"
    fi
    ;;
esac
```

### `bin/intent_fileindex`

Same shape, applied to `STARTDIR` before its validation block.

## Tests

`tests/unit/subdir_invocation.bats` covers:

1. `st list` from `intent/st/`.
2. `st list` from a deeply nested `intent/docs/a/b/c/`.
3. `st new` from `intent/docs/` — creates ST at project root, not at cwd.
4. `wp list` from a subdirectory.
5. `doctor` from a subdirectory.
6. Outside any project: commands fail with "not in an Intent project" and create nothing.
7. `INTENT_ORIG_CWD` propagates to subcommands (via a `treeindex` relative-path assertion).

## Verification

- Full BATS suite: 461 tests pass (454 pre-existing + 7 new).
- Manual smoke tests from `intent/st/ST0033/` and `/tmp/intent-outside-test/`: behavior as specified.

## Shipped in

v2.8.2 (2026-04-15), alongside ST0032.
