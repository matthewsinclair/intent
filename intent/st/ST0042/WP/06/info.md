---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-06
title: "Prune dead and legacy code"
scope: S
status: WIP
---

# WP-06: Prune dead and legacy code

## Objective

Delete dead and legacy surfaces (theme T6) per the fail-forward convention: no stubs, no deprecation shims. Includes retiring `intent audit` (gate decision 2026-06-11: the Credo overlap with the rule-library critics is a Highlander violation; critics are the canonical engine).

## Evidence

- `bin/intent_main` -- dead second dispatcher, diverged (relative-path `chmod` bug at `:98`), zero runtime callers (F-ARCH-3/9).
- `bin/intent_minimal` -- alpha-versioned (`VERSION="2.0.0-alpha"`) Phase-1 stub, no caller (F-ARCH-4).
- `bin/stp` -- symlink preserving the retired STP command a year post-rebrand (F-UPG-10).
- `intent audit` -- Credo checks (R2/R6/R11) overlap rule-library critics; two parallel Elixir engines (F-ARCH-5). RETIRE.
- Orphan templates: `lib/templates/eng/tpd/` set (heredoc used instead), `lib/templates/usr/_user_guide.md` (F-TPL-6/7).

## Scope exclusions

- Dead migration scaffolding (`needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, `create_project_backup` -- F-UPG-9): upgrade-subsystem code, ST0043 owns those deletions (gate decision 2026-06-11).

## Deliverables

- Listed files/symlinks deleted; `intent audit` dispatch case removed along with its implementation and docs/help references.
- MODULES.md rows for deleted modules removed (registry stays truthful).
- Any docs/help text referencing deleted surfaces cleaned (coordinate with WP-08).
- Grep proof: no remaining references to deleted entry points from live code.

## Acceptance Criteria

- [ ] `intent audit`, `stp`, `intent_main`, `intent_minimal` gone; no dangling references in `bin/`, plugins, docs, or help output.
- [ ] Full bats suite green (tests for deleted modules removed, not skipped).
- [ ] MODULES.md reflects the post-prune inventory.

## Dependencies

- Best after WP-08 (canon docs) is drafted or alongside it -- docs mention `intent audit`. Execution order places WP-06 after WP-08.
