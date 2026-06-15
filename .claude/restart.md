# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only; no whiteboard in THIS project.)
2. **Read this file + `intent/wip.md`.**

## State: v2.12.0 SHIPPED + field-validated -- no active arc

ST0043 + ST0045 both Completed (closed through their own gates: ST0043 8/8, ST0045 9/9), relocated to `intent/st/COMPLETED/`. Tag `v2.12.0` (commit `4e5ac15`) on both remotes + GitHub release; post-tag wrap `5f8dace` (config.json `intent_version` -> 2.12.0 + history header finalised) pushed. Full suite green (matts). First fleet upgrade (Lamplight 2.11.13 -> 2.12.0) ran clean through the new orchestrator -- `intent doctor` green on 2.12.0.

There is **no active steel thread**. Next work comes from the backlog in `wip.md`.

## What shipped (context only -- closed)

- **ST0043** -- `intent upgrade` convergent orchestrator (~150 lines): detect -> semver sanity (downgrade refusal, v2.9.0 floor, no "Unknown version") -> verified backup -> state-probed `LEDGER` walk -> single `intent claude upgrade --apply` -> stamp once, last. New upgrade-only `bin/intent_migrations`; `bin/intent_helpers` pruned 2026 -> 369 lines (all sub-v2.9.0 migration code, fail-forward); canon engine lost `VERSION_BUMP` + BSD `sed -i ''` (Linux-safe). Detail: `intent/st/COMPLETED/ST0043/`.
- **ST0045** -- Whiteboard Protocol 3.0: per-node dirs + single-writer `wip.md`/`inbox.<sender>.md` + `hv` node. Skill completeness + reference-vs-skill drift closed (`in-session`/`in-finish` + `working-with-llms.md`); guard `tests/unit/whiteboard_protocol_3_guard.bats`. Detail: `intent/st/COMPLETED/ST0045/`.
- **Close-gate (in `bin/intent_acceptance`)** -- F1: malformed/non-numeric AC/AT lines block loudly (was silent drop). F6: missing `acceptance.md` deliberately left opt-in-by-presence (gate stays open; matts ruling) -- do NOT re-add "missing must block".

## v2.12+ backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap); cosmetic: `intent_claude_upgrade` Phase-1 prints "(run 'intent upgrade' to bump)" even mid-upgrade; `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items (field evidence only).

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. NEVER `scripts/release --no-confirm`.
