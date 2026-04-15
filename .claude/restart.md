# Claude Code Session Restart

## WIP

No active steel threads. Intent v2.8.2 is stable. All 16 managed Intent projects in `~/Devel/prj/` at 2.8.2.

## Recent

- **2026-04-15**: v2.8.2 released (two commits under one tag). ST0033 (cwd-resilient dispatch) + ST0032 (Credo check wiring) + upgrade-chain gap fix (`migrate_v2_6_0_to_v2_8_0`, every case chains through to 2.8.2). Tag force-moved once after the slipstream. Fleet of 16 external projects brought to 2.8.2.
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
