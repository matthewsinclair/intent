# Claude Code Session Restart -- narrative state

## Current state (2026-06-11, v2.11.12 SHIPPED)

Tag `v2.11.12` (commit `574b015`) pushed to both remotes (`local` Dropbox + `upstream` GitHub); GitHub release published. The release carries the whole ST0042 + ST0041 arc: all nine ST0042 WPs (Fable 5 architectural review of Intent), the ST0041 MFIC harvest, and the done-work archive split. Both STs are **Completed** in `intent/st/COMPLETED/`. Post-release wrap (this file, `.claude/restart.md`, `wip.md`, memory, config self-stamp to 2.11.12) committed after the tag.

Nothing is in flight. The tree should be clean and the next work is a fresh arc.

## Next arc: ST0043 -- Rethink `intent upgrade`

WIP, not started, targets **v2.12.0 minor**, own session. `intent st show ST0043` / `intent/st/ST0043/info.md` carries the full Architecture-B design (convergent end-state + structural-step ledger), confirmed defects, and the delete/keep migration path. It owns: the two-installer collapse (~1800 lines), `update_config_version` inlines, dead migration scaffolding (`needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, `create_project_backup`), and the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit.

## Where the detail lives

- `intent/wip.md` -- current-state summary + Next Up backlog.
- `intent/done.md` -- terse DONE ledger; verbose release narratives at `intent/history/v2.11.*.md` (v2.11.12 narrative includes the full WP-by-WP arc summary).
- `intent/st/COMPLETED/ST0042/` -- review findings (design.md), MFIC leak write-up + execution notes (impl.md).
- `intent/st/COMPLETED/ST0041/` -- MFIC harvest: adopted practice vs deferred candidates (impl.md).
- `CHANGELOG.md [2.11.12]` -- the shipped release notes.

## Session conventions (carry forward)

T-shirt sizing only; compact at ~200-250k; ALWAYS use the intent CLI for ST/WP operations; NEVER manually wrap markdown; NO Claude attribution in commits; no vanity metrics; fail-forward (no stubs/shims); the user runs the full test suite externally (pause and ask -- single-file bats runs are fine); refresh BOTH restart files on every release.
