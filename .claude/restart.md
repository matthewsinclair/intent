# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) **Intent now HAS a whiteboard** (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: ST0047 COMPLETE -- v2.13.0 ready to ship (NOT yet shipped)

`intent claude start` + `intent claude ws new|list|archive|hygiene` -- the MAAC whiteboard launcher + workstream lifecycle -- is built, tested, and gate-GREEN (18/18), promoted from the Baize prototype to first-class Intent. The ST is complete but **NOT yet closed or shipped**: the close + release are hv-driven.

### Remaining (hv-driven, in order)

1. **Full suite** (matts, external) -- on green:
2. `intent st done ST0047` (gate green; relocates to `intent/st/COMPLETED/`).
3. **Commit** the v2.13.0 work to Intent main (the cwi command + `bin/intent` dispatch + `/in-whiteboard` skill + help + bats + the new `intent/whiteboard/` + ST docs + CHANGELOG + `intent/history/v2.13.0.md`).
4. `scripts/release --minor` -> v2.13.0 tag + push + gh release; then the post-tag wrap (config.json `intent_version` -> 2.13.0 + history header finalised). NEVER `--no-confirm`.
5. **Fleet `intent upgrade` sweep** (Phase 8): each `~/Devel/prj` Intent project -> `intent upgrade` -> `intent doctor` green -> commit. Confirm the project list with matts first. Excludes Pplr, Sites-in-Laksa, llm-tropes.

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
