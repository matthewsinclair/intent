# Intent v2.17.0 Release Notes

**Release Date**: 2026-07-10

## Overview

Intent v2.17.0 adds **`intent issues`** -- a first-class, lightweight issue tracker built into the CLI (ST0055). It formalises the ad-hoc `intent/issues/` convention that had grown organically across the fleet into a supported command with five verbs. An issue is the sub-steel-thread unit: a bug, follow-up, or observation too small for a full steel thread but worth tracking in-tree rather than leaving as prose drift.

This is a minor, not a patch: it adds a new command surface. It is additive -- projects that never run `intent issues` see no behaviour change and need no migration; the issues tree is created lazily on first use.

## What's new

```
intent issues [list] [--kind open|closed|all]   # list issues (default: open)
intent issues add [--severity SEV] TITLE         # add an issue, print its ID:TITLE  (alias: new)
intent issues show ID [--json]                   # show one issue, optionally as JSON
intent issues close ID                           # OPEN -> CLOSED (aka done)
intent issues open ID                            # CLOSED -> OPEN
```

| Aspect      | Design                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------ |
| On disk     | Directory-per-issue: `intent/issues/{OPEN,CLOSED}/NNNN/NNNN-slug.md`                       |
| Status      | The bucket directory (OPEN vs CLOSED) is authoritative; frontmatter `status:` mirrors it   |
| IDs         | Next zero-padded 4-digit integer, `max(existing)+1` across both buckets                    |
| Template    | Intent-owned (`lib/templates/issues/_ISSUE.md`), stamped on `add` -- not vendored per repo |
| Scaffolding | Lazy -- the first `add` creates `intent/issues/{OPEN,CLOSED}/`                             |

### Directory-per-issue

An issue is a directory, not a lone file. That lets an issue carry its satellites -- attachments, source material, generated PDF/JSON, or (in future) a per-issue work-package subtree -- alongside its `NNNN-slug.md`. `close` and `open` move the whole `NNNN/` directory between the two bucket directories and mirror the frontmatter `status:` field. There are exactly two states; a legacy `RESOLVED` frontmatter is normalised to CLOSED on read, so listings never surface a third state.

### Intent owns the template

The issue template lives once, in Intent's `lib/templates/`, and is stamped into each new issue by substitution. Projects no longer vendor a `_templ/0000-issue-title.md`; there is a single source for the issue shape across the whole fleet.

## Companion fix: pipes in titles no longer corrupt tables

A `|` in a steel-thread, work-package, or issue title used to corrupt every markdown table the title landed in (`steel_threads.md`, `intent wp list`, `intent todo`), because the shared table renderer splits rows on `|`. v2.17.0 adds `sanitize_title` (in `bin/intent_helpers`) and calls it at the input boundary of every command that accepts a new title -- `intent st new`, `intent wp new`, `intent issues add` -- replacing `|` with `/` so a pipe can never enter a stored title. Filed and closed as Intent's own first `intent issues` record (issue 0001).

## Under the hood

- `bin/intent_issues` is a thin coordinator, auto-dispatched by `bin/intent`; no dispatcher change was needed.
- `slugify` was promoted from `bin/intent_st` into `bin/intent_helpers` so `st`, `wp`, and `issues` share one slugifier (Highlander).
- The command is registered in `intent/llm/MODULES.md` and listed in `intent help`.

## Fleet adoption

Fleet members pick up `intent issues` on their next `intent upgrade`. Projects that already keep an ad-hoc `intent/issues/` tree (Utilz, Conflab, Lamplight) are being normalised to the directory-per-issue canon as a post-release rollout.

## Upgrading

`intent upgrade` from any v2.1x. The new command is additive; there is nothing to migrate. Run `intent doctor` to confirm a clean install.
