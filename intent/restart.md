# Claude Code Session Restart -- narrative state

## Current state (2026-06-11, end of ST0042+ST0041 execution arc -- tree release-ready)

The whole arc is done. All nine ST0042 WPs executed and committed (one fix commit + one `wp done` commit each); ST0042 and ST0041 marked **Completed** via `intent st done` and relocated to `intent/st/COMPLETED/`. Full suite green at every step; the final full run was executed by the user externally and confirmed green. **Nothing is pushed** -- everything rides the imminent patch release.

### Immediate next step (user-run)

```bash
bash scripts/release --patch    # v2.11.11 -> v2.11.12; interactive confirm at the push step
```

NEVER `--no-confirm` from a tool-driven session. After the release ships: refresh this file, `.claude/restart.md`, and `intent/wip.md`'s Current State block (release convention), and update memory's Active Work.

### What landed in the arc (all on main, in commit order)

WP-09a test HOME isolation (`1fc4180`+), WP-01 config eval (`d0e2b1d`), WP-05a canonical_status (`554fc0e`), WP-03 silent successes (`d959a9b`+`3c6db54`), WP-04 AGENTS.md generation (`899c7cf`+`bafa78a`), compact-point docs (`b236240`), WP-05b Highlander consolidation (`cc04497`+`a401430`), WP-02 rules-path drift + guard (`98cb98e`+`46f3612`), WP-07 MODULES.md + `file::function` checker fix (`47d0acb`+`285bbd3`), WP-08 canon docs + `intent st cancel` (`7f3db94`+`0e02fad`), WP-06 prune incl. `intent audit` retirement (`1d468aa`+`992aaff`), WP-09b vacuous-test rewrites + coverage (+ `intent organize` dispatch fix, `intent llm usage_rules` path fix), ST closures + this archive/wrap commit.

### Decisions executed (gate, 2026-06-11)

- `intent audit` retired (Highlander; critics canonical; credo templates survive via `intent st zero`).
- WP-06 excluded upgrade-subsystem dead scaffolding -- ST0043 owns all upgrade deletions.
- Patch now; ST0043 targets v2.12.0 minor, fresh session, own arc.

### After the release: next arc is ST0043

`intent st show ST0043` / `intent/st/ST0043/info.md` carries the full Architecture-B design (convergent end-state + structural-step ledger), confirmed defects, and the delete/keep migration path. It owns: the two-installer collapse (~1800 lines), `update_config_version` inlines, dead migration scaffolding (`needs_migration`, `show_migration_summary`, `count_migration_files`, `update_version_in_frontmatter`, `create_project_backup`), and the `intent claude upgrade` Phase-2 CLAUDE.md substitution audit.

### Where the detail lives

- `intent/wip.md` -- current-state summary + Next Up backlog (pruned of completed items).
- `intent/done.md` -- terse DONE ledger (this arc + release history); verbose release narratives at `intent/history/v2.11.*.md` (split out of wip.md this session).
- `intent/st/COMPLETED/ST0042/` -- review findings (design.md), MFIC leak write-up + execution notes (impl.md).
- `intent/st/COMPLETED/ST0041/` -- MFIC harvest: adopted practice vs deferred candidates (impl.md).
- `CHANGELOG.md [Unreleased]` -- the release-notes source: Added (`intent st cancel`), Fixed (eval, status canon, silent successes, AGENTS.md generation, Highlander consolidation, rules-path drift, MODULES.md gate, canon docs, organize/llm resurrections, critic-test rewrites), Removed (intent_main, intent_minimal, stp, intent audit, orphan templates).

### Session conventions (carry forward)

T-shirt sizing only; compact at ~200-250k; ALWAYS use the intent CLI for ST/WP operations; NEVER manually wrap markdown; NO Claude attribution in commits; no vanity metrics; fail-forward (no stubs/shims); the user runs the full test suite externally (pause and ask -- single-file bats runs are fine); refresh BOTH restart files on every release.
