# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only; no whiteboard in THIS project.)
2. **Read this file + `intent/wip.md`.**

## State: v2.12.0 STAGED + COMMITTED -- awaiting `scripts/release --minor`

ST0043 + ST0045 are BOTH Completed (closed through their own gates: ST0043 8/8, ST0045 9/9) and relocated to `intent/st/COMPLETED/`. Full suite green (matts). All work is committed; the tree is clean for the release pre-flight.

**The ONE remaining action is matts's:** `bash scripts/release --minor`. The script (read 2026-06-15) requires a clean tree (pre-flight aborts otherwise), then bumps `VERSION` 2.11.14 -> 2.12.0, runs `intent agents sync`, commits `release: v2.12.0` (VERSION + CHANGELOG.md + AGENTS.md), tags, pushes `local` + `upstream`, cuts the GH release. NEVER pass `--no-confirm`. After the tag, the manual post-release wrap bumps `config.json` `intent_version` to 2.12.0 (a release tag carries the prior config version by design).

If a NEW session starts after the release is cut: verify the tag exists (`git tag | grep v2.12.0`), confirm `config.json` `intent_version` == 2.12.0, then this arc is fully done -- move to the standing backlog in `wip.md`.

## What shipped (for context only -- already built + closed)

- **ST0043** -- `intent upgrade` convergent orchestrator. `bin/intent_upgrade` (~150 lines): detect -> semver sanity (downgrade refusal, v2.9.0 floor, no "Unknown version") -> verified backup -> state-probed `LEDGER` walk -> single `intent claude upgrade --apply` -> stamp once, last. New upgrade-only `bin/intent_migrations` (relocate_config + languages_field steps + `intent_relocate_dotintent`). `bin/intent_helpers` pruned 2026 -> 369 lines (all sub-v2.9.0 migration code; fail-forward). Canon engine (`intent/plugins/claude/bin/intent_claude_upgrade`) lost `VERSION_BUMP` + BSD `sed -i ''` (Linux-safe); orchestrator is sole stamper. Detail: `intent/st/COMPLETED/ST0043/`.
- **ST0045** -- Whiteboard Protocol 3.0. `in-whiteboard/SKILL.md` gained inbox-file init (`# inbox:` header + single-writer + `_(empty)_` sentinel + creation on first `ask`), `.history/.gitkeep`, the `hv` node variant, message-entry required-vs-recommended. Drift closed in `in-session`/`in-finish` SKILL.md + the `working-with-llms.md` whiteboard section. Guard: `tests/unit/whiteboard_protocol_3_guard.bats`. 3.0 skills synced to `~/.claude`. Detail: `intent/st/COMPLETED/ST0045/`.
- **Close-gate (release-scoped, in `bin/intent_acceptance`)** -- F1: malformed/non-numeric AC/AT lines block loudly (was silent drop). F6: missing `acceptance.md` deliberately left opt-in-by-presence (gate stays open; matts ruling). Guards in `acceptance_close_gate.bats`.
- Release docs: `CHANGELOG.md` `[2.12.0]`, `intent/history/v2.12.0.md`, `intent/done.md` ledger + Releases bullet.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. NEVER `scripts/release --no-confirm`.
