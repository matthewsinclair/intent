---
verblock: "22 Apr 2026:v0.34: matts - ST0034 active"
intent_version: 2.8.2
---

# Work In Progress

## Current State

Intent v2.8.2. **ST0034 (Agentic Software Engineering Suite) active** — 11 work packages, target release v2.9.0. Plan approved; WP docs populated; coding not yet started.

## Recent

- **2026-04-15**: v2.8.2 released (tagged twice; second commit `84a3a5f` slipstreamed a fix to `bin/intent_upgrade` that previously halted the migration chain at 2.6.0). Contents:
  - ST0033 -- cwd-resilient dispatch. `bin/intent` exports `INTENT_ORIG_CWD` and `cd`s to `$PROJECT_ROOT` before `exec`ing subcommands. `intent_treeindex` and `intent_fileindex` consult `INTENT_ORIG_CWD` for relative path args. Regression tests in `tests/unit/subdir_invocation.bats`.
  - ST0032 -- Credo custom checks wired into `.credo.exs` via `lib/scripts/configure_credo.exs`. 2 broken templates removed, 4 fixed, `bracket_access_on_struct` added.
  - Upgrade chain completed through 2.6.0/2.7.0 gap. New `migrate_v2_6_0_to_v2_8_0` (pure version stamp). Every starting-version case chains through to 2.8.2. Pre-v2 fallback chain extended. `needs_v2_8_2_upgrade` accepts 2.6.0 and 2.7.0.
  - Fleet upgrade: all 16 Intent projects in `~/Devel/prj/` now at 2.8.2 (Anvil, Conflab, Intent, Laksa, Lamplight, MeetZaya, MicroGPTEx, Molt, Molt-matts, Multiplyer, Prolix, Utilz, Courses/Agentic Coding, Arca/arca_cli, Arca/arca_config, Arca/arca_notionex). A3/\* skipped per user direction.
- **2026-04-09**: v2.8.1 released -- TCA suite hardening from Lamplight ST0121 feedback. ST0031 complete. `tca-report.sh --check-only` pre-flight guard, `tca-init.sh` provisioning guards, False Positive Guidance as a REQUIRED design.md section, `--st-dir` to `--tca-dir` rename across 6 files (breaking, internal only), `chains_to:` frontmatter on all 5 TCA skills.
- **2026-04-06**: ST0031 (Agentic Coding Course, earlier numbering) migrated to `../Courses/Agentic Coding/` project and renumbered to ST0001 there. Course content removed from Intent history via `git filter-repo`.

## Active Steel Threads

- **ST0034**: Agentic Software Engineering Suite. v2.9.0 target. 11 WPs. Rules as first-class citizens, critic-`<lang>` family (elixir/rust/swift/lua) with code+test modes, `~/.intent/ext/` extension system, worker-bee pruned from canon to reference extension, elixir subagent deleted. Fail-forward design. See `intent/st/ST0034/design.md` for full architecture.

## Parked

- ST0010: Not started, in `intent/st/NOT-STARTED/`
- ST0015: Not started, in `intent/st/NOT-STARTED/`
