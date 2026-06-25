# Tasks - ST0047: claude_with_intent (MAAC launcher + workstream lifecycle)

Tracking is by work package (`intent wp list ST0047`); this mirrors their state. The thread is **WIP -- design elaborated, build not started (pending hv review)**.

## Doing

- [ ] **Phase 1 -- elaborate this ST** (info / design / acceptance + 4 WPs). IN PROGRESS. Pause for hv review before any script code (document-before-coding).

## To do (work packages)

- [ ] **WP-01 -- Whiteboard provisioner: `ws new` + `ws list`.** The deterministic node scaffolder (Protocol 3.0 shape, any short-ish id, hv=Workstream Zero) + the roster read. Dogfood: regenerate the Baize `hv`+`cc`+`ic`+`vc` skeleton and diff against the hand-built golden reference. Selects the shell-test harness. (S)
- [ ] **WP-02 -- Session launcher: `start|st`.** The launch (verified `--effort max --permission-mode auto --append-system-prompt` + `/in-session` seed), `compose_ctx`, the `CWI_DRY_RUN` test seam, and provision-if-absent. Resolves the proceed-to-plan spike + confirms the `auto` posture. (S) [dep WP-01]
- [ ] **WP-03 -- Lifecycle: `ws archive` + `ws hygiene`.** Retire-with-history; mechanical structural lint + tidy (never the semantic archive). (S) [dep WP-01]
- [ ] **WP-04 -- Promote to `intent claude` + back-fill.** Relocate into the Intent namespace, wire the one format SSOT (skill <-> script), back-fill Laksa + Lamplight + Intent. (S) [dep WP-01..03 green + Baize dogfood]

## Task Notes

Prototype lives in Baize `bin/claude_with_intent` (POSIX shell, `bin/cli` house style); Baize's hand-scaffolded whiteboard is the golden reference. Document-before-coding: no script code until this ST is reviewed. The capability is an Intent tool, not a Baize one -- Baize is only the first proving ground, so the ST is kept here and the Baize `intent/` docs stay clean of it.

## Dependencies

- WP-01 gates WP-02 (`start` provisions via `ws new`) and WP-03 (archive/hygiene operate on `ws new` output).
- WP-04 gates on WP-01..03 green + the Baize dogfood pass.
- External: Claude Code >= 2.1.191 (the verified `--effort` / `--permission-mode` flags); the `/in-whiteboard` skill (format SSOT); `require-in-session.sh` (the gate slash-exemption the seed relies on).
