---
verblock: "24 Feb 2026:v0.2: matts - Completed"
intent_version: 2.5.0
status: Completed
slug: add-work-packages-as-first-class-citizens-within
created: 20260224
completed: 20260224
---

# ST0024: Add work packages as first class citizens within a steel thread

## Objective

Add a top-level `intent wp` command for managing work packages within steel threads. Work packages break a steel thread into smaller, trackable units of work, each with its own directory and info.md.

## Context

Steel threads needed a way to decompose into smaller deliverables. Previously, task tracking was done in a flat tasks.md. Work packages provide structured subdirectories (STXXXX/WP/NN/info.md) with frontmatter-based status tracking, auto-numbering, and specifier shorthand syntax.

## Related Steel Threads

- ST0022: st new hardening (shared `escape_sed_replacement()` and `normalise_st_id()`)
- ST0025: Fix Highlander Violations (shared helpers extracted during this work)

## Deliverables

- `bin/intent_wp` -- new command script (345 lines)
- `lib/templates/prj/st/WP/info.md` -- WP template
- `lib/help/wp.help.md` -- help text
- `tests/unit/wp_commands.bats` -- 29 BATS tests
- Documentation updates across README.md, CLAUDE.md, user_guide.md, reference_guide.md
