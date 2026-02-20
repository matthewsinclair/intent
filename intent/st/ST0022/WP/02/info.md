---
verblock: "20 Feb 2026:v0.1: matts - Initial version"
wp_id: WP-02
title: Slug Generation System
scope: Medium
status: Not Started
---

# WP-02: Slug Generation System

## Objective

Auto-generate a slug from the title when creating a steel thread, store it in frontmatter, and display it in listings instead of the full title.

## Deliverables

- `slugify()` function in `bin/intent_st` (lowercase, replace non-alnum with hyphens, collapse runs, strip leading/trailing hyphens, max 50 chars)
- `slug:` field added to template `lib/templates/prj/st/ST####/info.md`
- `slug:` field populated in both template-sed and heredoc paths of `st new`
- `st list` shows "Slug" column instead of "Title" column
- `st sync --write` writes slug to steel_threads.md index
- Slug extracted from frontmatter in list/sync code path
- Fallback to title for pre-existing STs without `slug:` field

## Slug Algorithm

```
1. Lowercase the entire string
2. Replace any run of non-alphanumeric characters with a single hyphen
3. Strip leading and trailing hyphens
4. If length > 50, truncate at the last hyphen before position 50
   (if no hyphen found, hard-truncate at 50)
5. Strip any trailing hyphen from truncation
```

## Acceptance Criteria

- [ ] `intent st new "My Cool Feature"` creates frontmatter with `slug: my-cool-feature`
- [ ] `intent st new "Ash/Ecto Database Layer"` creates `slug: ash-ecto-database-layer`
- [ ] Title longer than 50 chars produces slug truncated to <= 50 chars
- [ ] `intent st list --status all` shows slug column instead of title column
- [ ] `steel_threads.md` index uses slug instead of title after `st sync --write`
- [ ] Pre-existing STs without `slug:` field show title as fallback in listings
- [ ] BATS tests cover slugification edge cases (special chars, long titles, Unicode)
- [ ] BATS tests cover slug display in `st list`

## Dependencies

- WP-01 (escape function must be in place for sed substitution of slug)
