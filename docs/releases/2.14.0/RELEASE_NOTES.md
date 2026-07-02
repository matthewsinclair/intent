# Intent v2.14.0 Release Notes

**Release Date**: 2026-07-02

## Overview

Intent v2.14.0 adds **`intent todo`** -- a flat DOING / TODO / DONE view of every steel thread and work package (ST0050) -- and fixes a generated-file width bug (ST0051).

`intent todo` answers "what's in flight right now?" without leaving the terminal. It is a _projection_: `intent/todo.md` is derived from each unit's real `status:` and its status-directory placement, never authored by hand, so it cannot drift from `intent/st/**`. The same discipline that makes steel threads trustworthy -- one source of truth -- is what makes the board trustworthy.

This is a minor, not a patch: it adds a new command surface. It requires no migration.

## What's new: `intent todo`

A nested GitHub-Flavored-Markdown checklist, bucketed by real status:

```
## DOING
- [-] ST0007: rate-limited cache
  - [x] 01: token bucket
  - [-] 02: eviction policy
## TODO
- [ ] ST0009: SSO for enterprise
## DONE:2026-07-02T00:00:00Z
- [x] ST0006: audit logging
```

| Command                      | Does                                                                  |
| ---------------------------- | --------------------------------------------------------------------- |
| `intent todo` / `todo list`  | Print `intent/todo.md` (generating it on first use)                   |
| `intent todo update`         | Regenerate from current ST / WP status                                |
| `intent todo --json`         | Emit the board as keyed-by-bucket JSON (each thread carrying its WPs) |
| `intent todo done <ST[/NN]>` | Mark done via `intent st` / `wp done` -- inherits the acceptance gate |
| `intent todo notdone <...>`  | Reopen to WIP                                                         |
| `intent todo toggle <...>`   | Flip done / not-done                                                  |
| `intent todo done --flush`   | Clear the DONE view (advance the `## DONE:<T>` watermark)             |
| `intent todo done --prune`   | Emit the DONE items to stdout (for archiving), then flush             |

Two properties matter:

- **It cannot lie.** Every checkbox glyph is derived from real status; there is no separately-stored checkbox state. The mutation verbs change _real_ status by wrapping `intent st` / `intent wp` and regenerating -- they never hand-edit a checkbox. So `intent todo done` inherits the ST0048 acceptance close-gate: a BLOCKED contract is refused, never bypassed.
- **DONE is watermarked.** The heading is `## DONE:<T>`, where `<T>` is the last-flush instant, and DONE lists completions at or after it. `--flush` advances `<T>` (clearing the view without touching the record in `COMPLETED/`); `--prune` emits the items being cleared for you to archive (eg `intent todo done --prune >> intent/done.md`). Between flushes the watermark is sticky, so the file stays prettier-stable.

## Also fixed: generated-file width (ST0051)

`intent st sync --write` hard-coded an 80-column width, clipping the slug column of the generated `steel_threads.md`. Generated files now size to a new `dft_width` config field (`intent/.config/config.json`, default `120`); interactive stdout stays at the terminal width; an explicit `--width N` overrides both. `intent init` seeds `dft_width` for new projects.

## Compatibility -- `completed:` is now an ISO 8601 timestamp

To make the DONE flush exact, `intent st done` now stamps the frontmatter `completed:` field as an ISO 8601 UTC timestamp (`2026-07-02T21:36:06Z`) instead of a bare `YYYYMMDD` date. **No migration is required.** Existing threads keep their `%Y%m%d` stamps; every place that reads `completed:` -- the DONE bucket membership test and the `steel_threads.md` render -- tolerates both forms (a legacy date is read as that day's `00:00:00Z`, and the index shows just the date part). You will simply see full timestamps on newly-closed threads and dates on older ones.

## Upgrade

```bash
intent upgrade --apply
```

`intent todo` and the width fix are available immediately -- both are plain CLI paths served centrally from `$INTENT_HOME`, so no per-project rollout and no session restart are needed. `dft_width` is read with a built-in `120` default, so projects that predate the field get the correct width without editing their config.

## Why

A project's real state already lives in `intent/st/**`; what was missing was a fast, honest way to _see_ it. A hand-maintained TODO list is exactly the kind of parallel record that drifts -- the thing steel threads exist to prevent. `intent todo` refuses that trade: it is a pure projection, cheap to regenerate, safe to script (`--json`), and its only mutations go through the same gated `intent st` / `wp` verbs everything else does. You get the convenience of a checklist with none of the drift.
