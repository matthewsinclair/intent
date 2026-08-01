---
id: "0011"
title: "intent st list enumerates any STnnnn directory at any depth: a staging area under intent/st/ silently becomes duplicate live threads"
date: 2026-08-01
reporter: matts
status: OPEN
severity: medium
---

# 0011: intent st list enumerates any STnnnn directory at any depth: a staging area under intent/st/ silently becomes duplicate live threads

## Tags

steel-threads, st-list, st-organize, enumeration, highlander, no-silent-errors

## Summary

`intent st list` enumerates steel threads with `find "$ST_DIR" -type d -name "ST[0-9][0-9][0-9][0-9]"` -- a recursive walk with **no depth limit**, rooted at `intent/st`. The recursion is deliberate and necessary: it is how threads inside `COMPLETED/`, `NOT-STARTED/` and `CANCELLED/` are found. But because it is bounded only by the name pattern, **any directory named `STnnnn` anywhere under `intent/st/`, at any depth, is treated as a live steel thread.** So any staging, working, archive or triage area placed under `intent/st/` silently becomes live threads -- and since such areas characteristically hold _copies_ of existing threads, the result is duplicate ids in a namespace whose whole contract is that an id identifies one thread.

`intent todo` does not have this defect: `list_bucket_dirs` globs exactly one level (`"$root"/ST[0-9]*`) and its caller names the buckets explicitly. So two Intent surfaces disagree about what constitutes a steel thread, and only one of them is right.

## Reproduction

Observed in project Baize on 2026-08-01, at Intent v2.18.0. Eight steel threads had been handed to an external design process and came back edited; they were parked for triage at `intent/st/_inbox/` -- deliberately underscore-prefixed to mark them as not-live, and deliberately inside `intent/st/` to keep them beside the threads they shadow.

```
$ ls intent/st/_inbox/
README.md  ST0016  ST0017  ST0018  ST0019  ST0020  ST0021  ST0022  ST0023

$ intent st list
ID     | Slug                                            | Status | Created    | Completed
-------|-------------------------------------------------|--------|------------|----------
ST0018 | party-session-membership-and-the-host-handshake | WIP    | 2026-07-28 |
ST0016 | design-system-refresh-the-lit-pool              | WIP    | 2026-07-28 |
ST0016 | design-system-refresh-the-lit-pool              | WIP    | 2026-07-27 |   <-- the staged copy
ST0010 | perception-data-collection-and-ml-augmentation  | WIP    | 2026-06-20 |
```

The other seven staged threads are enumerated too; they simply do not appear in this view because their status is `Not Started` and the default list shows WIP.

Minimal repro in any Intent project:

```
mkdir -p intent/st/_scratch/ST0001 && cp intent/st/ST0001/info.md intent/st/_scratch/ST0001/
intent st list          # ST0001 now appears twice
```

Confirming the fix direction -- moving the directory out of `intent/st/` (to `intent/st_inbox/`) resolved it immediately, with `COMPLETED` / `NOT-STARTED` threads still resolving correctly:

```
$ intent st list | grep -c ST0016
1
$ intent st show ST0017 | head -1   # NOT-STARTED, still resolves
---
```

## Root Cause

One enumeration rule, expressed two different ways in two files, and the looser one is used by the commands that matter most.

`bin/intent_st:868` (the `list` command, `ST_DIR="intent/st"` at `:778`):

```bash
done < <(find "$ST_DIR" -type d -name "ST[0-9][0-9][0-9][0-9]" -print0)
```

`bin/intent_todo` (`list_bucket_dirs`, called with `$ST_BASE` and `$ST_BASE/NOT-STARTED`, plus a separate `COMPLETED` loop at `:185`):

```bash
for st in "$root"/ST[0-9]*; do
  [ -d "$st" ] && [ -f "$st/info.md" ] || continue
```

`intent_todo` encodes the real rule: **a steel thread lives at `intent/st/STnnnn` or `intent/st/<STATUS-BUCKET>/STnnnn`, and nowhere else.** `intent_st` approximates it with "any `STnnnn` directory below `intent/st`", which is a superset. The two agree for a clean tree and diverge the moment anything else is placed under `intent/st/`.

The same unbounded `find` appears at `bin/intent_st:939` (legacy `.md` path), `:1380` (`repair`) and `:1425` (`organize`), so the misreading is not confined to `list`.

## Impact

Bounded -- no data loss, verified rather than assumed -- but it makes the thread registry state a falsehood, and one of the affected commands reports a success it did not achieve.

1. **Duplicate ids in `intent st list`.** The id namespace's one guarantee is that an id names one thread. Observed above.

2. **`steel_threads.md` inherits it.** `intent st sync --write` shells out to `intent st list --markdown` (`bin/intent_st:1156`) and writes the result between the index markers, so a `sync --write` persists the duplicate row into the project's canonical, committed index. Nothing downstream re-checks it.

3. **`intent st organize --write` reports a move that did not happen.** It uses the same unbounded `find`, reads each found directory's `status:`, computes the bucket that status implies, and runs `mv "$dir" "$CORRECT_PARENT/"`. For a staged copy whose live counterpart already exists, that target is occupied. Verified behaviour:

   ```
   $ mv st/_inbox/ST0016 st/
   mv: rename st/_inbox/ST0016 to st/ST0016: Directory not empty
   $ echo $?
   1
   ```

   **The live thread survives intact and the staged copy is left where it was** -- `mv` refuses to merge non-empty directories, so the destructive reading of this does not hold. But `bin/intent_st:1480-1482` runs the `mv` and then echoes unconditionally, with no exit-code check:

   ```bash
   mv "$dir" "$CORRECT_PARENT/"
   echo "  Moved $ID to $CORRECT_PARENT"
   ```

   so `organize --write` prints `Moved ST0016 to intent/st` for every collided thread while `mv`'s error goes to stderr and is never acted on. Same family as 0003 and 0004: a command asserting an outcome it did not verify.

4. **The genuinely lossy case is the one with no collision.** A staged copy whose id has _no_ live counterpart at the target bucket moves cleanly -- silently promoting a stale, not-yet-triaged copy into the live namespace, where it is thereafter indistinguishable from a real thread. In the observed project all eight had live counterparts, so all eight collided; that was luck, not design.

5. **The workaround is not discoverable.** Underscore-prefixing the directory reads as the obvious way to mark it not-live, and has no effect. Nothing warns; the tree simply grows a second thread with the same id.

## Proposed Fix

**Give the two surfaces one enumerator, and take the rule from `intent_todo`, which already has it right** (Highlander -- the definition of "what is a steel thread" should not exist twice, and today the looser copy is the one `list` / `sync` / `organize` / `repair` depend on).

1. Add a shared helper -- `list_st_dirs <base>` in `bin/intent_helpers`, beside the existing `resolve_st_dir` / `resolve_wp_dir` -- that enumerates exactly the canonical locations: `<base>/ST[0-9]*` plus `<base>/{COMPLETED,NOT-STARTED,CANCELLED}/ST[0-9]*`, each one level, each requiring `info.md`. This is what `intent_todo` does today, generalised over all four buckets.
2. Repoint `bin/intent_st` `:868`, `:1380` and `:1425` at it, and `bin/intent_todo` too so the rule has exactly one home.
3. Independently of the enumeration fix, **check `mv`'s exit status at `:1481` before claiming the move** -- a failed `mv` must report the failure and exit non-zero rather than printing `Moved`. That line is wrong even with a clean tree, for any cause of `mv` failure (permissions, full disk, a case-insensitive-filesystem collision).

An explicit bucket allowlist is preferable to "skip `_`-prefixed directories": it makes the canonical layout the rule rather than blacklisting one naming convention, and it fails closed for _any_ future non-thread directory placed under `intent/st/`, however named.

Worth considering alongside: `intent doctor` could assert that no two enumerated threads share an id, so a duplicate is reported as a defect rather than rendered as a table row.

## Related

- Surfaced in project Baize, 2026-08-01, while triaging steel threads returned from an external design round. Fixed there by moving the staging area to `intent/st_inbox/` (outside the find root) -- a project-side workaround, not a fix for the tool.
- Same project, 2026-07-31: an unanchored `_inbox/` rule in `intent/.gitignore` also silently matched `intent/st/_inbox/`, leaving the README explaining the triage untracked. `intent/st/` accumulating things it was not designed to hold has now caused two separate accidents.
- 0003 and 0004 -- same No-Silent-Errors family: a command reporting an outcome it did not verify. Item 3 of the proposed fix is a direct instance.

## Resolutions

{{TBC}}
