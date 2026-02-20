---
verblock: "20 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: "-s|--start Flag for st new"
scope: Small
status: Done
---

# WP-03: `-s|--start` Flag for `st new`

## Objective

Add a `-s|--start` flag so `intent st new -s "Title"` creates and immediately starts the steel thread in one command.

## Deliverables

- Flag parsing (`-s|--start`) in `new` command, before positional title argument
- After successful creation with flag, invoke start logic inline:
  - Update `status:` from `Not Started` to `WIP` in frontmatter
  - Update `- **Status**:` body line if present
  - Move directory from `NOT-STARTED/` to `intent/st/`
  - Update steel threads index
- Help text updated to show `-s|--start` option
- BATS tests covering both flag forms and flag position

## Current State

Creating and starting a steel thread requires two commands:

```bash
intent st new "Quick Fix"
intent st start ST0023
```

The user must note the ST ID from the first command's output to pass to the second.

## Target State

A single command creates and starts the steel thread:

```bash
intent st new -s "Quick Fix"
```

The steel thread is created directly in `intent/st/` with `status: WIP`.

## Acceptance Criteria

- [ ] `intent st new -s "Quick Fix"` creates ST in `intent/st/` with `status: WIP`
- [ ] `intent st new --start "Quick Fix"` also works (long form)
- [ ] `intent st new "Regular Thread"` (without flag) still creates in `NOT-STARTED/` with `status: Not Started`
- [ ] `intent st new "Title" -s` also works (flag after title)
- [ ] BATS tests cover both `-s` and `--start` forms
- [ ] BATS test covers flag-after-title position

## Dependencies

- WP-02 (slug must be in place so --start creates a complete ST with slug)
