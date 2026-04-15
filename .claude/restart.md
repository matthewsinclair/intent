# Claude Code Session Restart

## WIP

No active steel threads. Intent v2.8.2 is stable.

## Recent

- **2026-04-15**: v2.8.2 released -- ST0033 (cwd-resilient dispatch) and ST0032 (Credo check wiring) landed. `intent` subcommands now work from any subdirectory of a project via a single-point `cd "$PROJECT_ROOT"` in `bin/intent` that also exports `INTENT_ORIG_CWD` for `treeindex`/`fileindex` relative-path args. Credo custom checks wired into `.credo.exs` via `lib/scripts/configure_credo.exs`.
- **2026-04-09**: v2.8.1 released -- ST0031 (TCA suite hardening, post-Lamplight) landed. Pre-flight guard in `tca-report.sh --check-only`, provisioning invariants doc section, `tca-init.sh` guards, FP Guidance as REQUIRED, `--st-dir` to `--tca-dir` rename (breaking, internal only).
- **2026-04-06**: ST0031 (Agentic Coding Course, earlier numbering) migrated to `../Courses/Agentic Coding/` and renumbered to ST0001 there. Course content removed from Intent history via `git filter-repo`.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`

## Conventions

- ALWAYS use `intent` CLI for ST/WP operations
- NEVER manually wrap lines in markdown
- NO Claude attribution in commits
- NEVER report test / skill / subagent counts in release notes, CHANGELOG, wip.md, or session docs (vanity metrics)
