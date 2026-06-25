# Claude Code Session Restart -- narrative state

## Current state (2026-06-25)

**v2.13.0 is SHIPPED (ST0047).** Tag `v2.13.0` (commit `c6b8f70`) on both remotes + GitHub release; Intent self-upgraded clean (2.12.0 -> 2.13.0, `intent doctor` green) -- the Phase 8 canary. `intent claude start` + `intent claude ws` -- the MAAC whiteboard launcher + workstream lifecycle -- is first-class in Intent, closed through its own gate (18/18). Remaining: the fleet `intent upgrade` sweep across the other `~/Devel/prj` members. Detail: `intent/st/COMPLETED/ST0047/` + `.claude/restart.md`.

**v2.12.0 (prior) is SHIPPED + field-validated.** Tag `v2.12.0` (commit `4e5ac15`) on both remotes + GitHub release; post-tag wrap `5f8dace` (config.json `intent_version` -> 2.12.0 + history header finalised) pushed. ST0043 + ST0045, both through the ST0044 five-step with matts as verifier and closed through their own gates. First fleet upgrade (Lamplight, 2.11.13 -> 2.12.0) ran clean: state-probed ledger no-op'd both already-satisfied steps, single stamp last, `intent doctor` green.

## ST0043 -- Rethink `intent upgrade` (COMPLETE, v2.12.0)

`intent upgrade` rewritten from a 524-line version-case ladder into a ~150-line convergent orchestrator: detect -> semver sanity (downgrade refusal, v2.9.0 fleet floor, no "Unknown version") -> verified backup -> state-probed `LEDGER` walk (`step_<id>_needs/_run/_verify`) -> single `intent claude upgrade --apply` -> stamp once, last. All sub-v2.9.0 migration code pruned fail-forward (`bin/intent_helpers` 2026 -> 369 lines); the two surviving steps + `intent_relocate_dotintent` live in the new upgrade-only `bin/intent_migrations`. Canon engine lost `VERSION_BUMP` + BSD `sed -i ''` (Linux-safe); orchestrator is sole stamper. 8/8 ACs. Detail: `intent/st/COMPLETED/ST0043/`.

## ST0045 -- Whiteboard Protocol 3.0 (COMPLETE, v2.12.0)

Per-node `<node>/` dirs + single-writer `wip.md` / `inbox.<sender>.md` + the `hv` hypervisor node, superseding the 2.0 flat-file model (ST0040). Skill body was rewritten pre-contract; ST0045 added the AC/AT contract, the four skill-completeness corners (inbox init + `_(empty)_` sentinel, `.history/.gitkeep`, hv variant, message-entry fields), and closed reference-vs-skill drift (`in-session` / `in-finish` + `working-with-llms.md`); guard `tests/unit/whiteboard_protocol_3_guard.bats`. 9/9 ACs. Detail: `intent/st/COMPLETED/ST0045/`.

## Close-gate hardening (v2.12.0, in `bin/intent_acceptance`)

F1: malformed / non-numeric AC/AT lines now block the gate loudly (was a silent drop -> vacuous green). F6: a missing `acceptance.md` is deliberately left opt-in-by-presence (gate stays OPEN; matts ruling) -- do NOT re-add "missing must block". Guards in `acceptance_close_gate.bats`.

## Where detail lives

- `.claude/restart.md` -- next-session focus.
- `intent/st/COMPLETED/ST0043/` + `intent/st/COMPLETED/ST0045/` -- closed thread docs.
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.12.0.md` -- shipped-work ledger / narrative.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full test suite externally (single-file bats fine); matts is the acceptance verifier; never `scripts/release --no-confirm`.
