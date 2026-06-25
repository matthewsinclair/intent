# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) **Intent now HAS a whiteboard** (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.13.0 SHIPPED (ST0047) -- fleet sweep remaining

`intent claude start` + `intent claude ws new|list|archive|hygiene` -- the MAAC whiteboard launcher + workstream lifecycle -- shipped first-class in v2.13.0. Tag `v2.13.0` (commit `c6b8f70`) on both remotes + GitHub release; ST0047 closed through its own gate (18/18) and relocated to `intent/st/COMPLETED/`. Intent self-upgraded clean (2.12.0 -> 2.13.0; ledger no-op'd the satisfied steps, `intent doctor` green) -- the Phase 8 canary. Post-ship wrap committed (config.json 2.13.0 + canon refresh + wip/done/history).

### Remaining: Phase 8 fleet sweep

Each other `~/Devel/prj` Intent project -> `intent upgrade` -> `intent doctor` green -> commit. Confirm the project list with matts first. Excludes Pplr, Sites-in-Laksa, llm-tropes. (Intent itself is already done -- the canary.)

## What ST0047 shipped

- **`intent_claude_cwi`** (`intent/plugins/claude/bin/`) -- dispatched from the `bin/intent` claude branch (`start|ws`, no shift); resolves the CURRENT project via `find_project_root`, served centrally from `$INTENT_HOME` (no per-project install). `set -u` only (No-Silent via `error()`); critic-shell-hardened (guarded the `ws archive` `mv` + `ws new` writes). Help in `lib/help/claude.help.md`.
- **`/in-whiteboard` skill** -- "Scaffolding a node" repointed at `intent claude ws new`; lazy-inbox drift reconciled to the eager bidirectional pre-seed (Highlander SSOT).
- **Intent whiteboard** -- `intent/whiteboard/` stood up with hv+cc+vc (no ic) + roster README; Intent dogfoods MAAC.
- **Tests** -- `tests/unit/claude_with_intent.bats` (ATs ported from the retired Baize `cwi_test.sh` + WP-04 dispatch/SSOT guards).
- **Baize** -- prototype retired (`cc2438f`, in that repo).
- Provenance: Lamplight pioneered by convention (the operational reference), Baize first productised (the MVP), Intent now first-class.

## v2.13+ backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
