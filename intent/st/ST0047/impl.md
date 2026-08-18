# Implementation - ST0047: Add claude_with_intent script to Intent to support multi-agent agentic coding (MAAC)

## Implementation

Built and proven in Baize as `bin/claude_with_intent` (WP-01..03), then promoted to first-class Intent as `intent claude start|ws` (WP-04) and the Baize prototype retired (Highlander -- no divergent second copy).

As-built home: `intent/plugins/claude/bin/intent_claude_cwi` (bash, `set -u` only -- not `-e`/pipefail, the `require-in-session.sh` precedent, because the body is `[ test ] && action` guards; every error path surfaces via `error()` from `intent_helpers`). Dispatched from the `bin/intent` claude branch (`start|ws` -> `intent_claude_cwi`, deliberately no `shift` so the script's own `case` consumes the subcommand). Help in `lib/help/claude.help.md`.

### Surface

- `intent claude start <ws>` -- compose context + launch `claude --effort max --permission-mode auto --append-system-prompt "$(compose_ctx)" /in-session`; provision-if-absent prompt; `CWI_DRY_RUN` seam.
- `intent claude ws new|list|archive|hygiene` -- the deterministic node lifecycle; `CWI_WB` seam.

### Key promotion adaptations (from the Baize prototype)

- Root resolution: `dirname/..` -> `find_project_root` -- the command operates on the CURRENT project's whiteboard, and plugin commands exec before `bin/intent` enters `PROJECT_ROOT`. Served centrally from `$INTENT_HOME`, so it is available in every project with no per-project install (the "back-fill" is smoke, not a rollout).
- `die()` -> `error()` (from `intent_helpers`; the canonical No-Silent surface).
- Dropped the `st` alias (it collides with top-level `intent st`).
- No-Silent hardening surfaced by Intent's critic-shell: guarded the load-bearing filesystem mutations in `ws new` (mkdir + the wip.md / inbox writes) and `ws archive` (the destructive `mv`) with `|| error` -- the Baize review missed these, and under `set -u`-only there is no `-e` net, so a failed `mv` would otherwise `rm` peer inboxes and report false success.

### SSOT convergence (AC-04.2)

The `/in-whiteboard` skill's "Scaffolding a node" prose now points at `intent claude ws new`, and the lazy-inbox drift ("never pre-seeded") was reconciled to the script's eager bidirectional pre-seed (ratified by AC-01.1 + the golden board); `ask` / `announce` keep on-demand creation as a self-healing fallback for hand-added nodes.

### Tests

`tests/unit/claude_with_intent.bats` -- the WP-01..03 ATs ported from the Baize `cwi_test.sh` plus the WP-04 dispatch + skill-SSOT guards, driven through the real `intent claude` dispatch via the `CWI_WB` / `CWI_DRY_RUN` seams (never spawns a real claude, never touches a real board).

### Intent whiteboard (AC-04.3)

Intent now dogfoods MAAC: `intent/whiteboard/` is stood up with `hv` + `cc` + `vc` (no `ic` -- Intent is CLI plus data, no UX surface) plus a roster README. Back-fill smoke confirmed the centrally-served command against Lamplight (its live 5-node board, read-only `ws list` / `ws hygiene`) and Laksa (graceful no-board error).

## Provenance

Lamplight pioneered the whiteboard by convention (the prototype and the operational reference for how MAAC actually runs); Baize was the first productised use (the MVP); ST0047 makes it a first-class Intent capability.

## Challenges & Solutions

- **Tool vs project root.** Sibling plugin scripts resolve `INTENT_ROOT` (the tool home) 4 levels up; the whiteboard needs the project root. Resolved via `find_project_root` from the invoker's cwd (plugin commands run there, before `bin/intent` cd's into the project).
- **Test harness across the promotion.** Baize used a dependency-free `cwi_test.sh`; Intent is bats. The ATs were re-homed to `tests/unit/claude_with_intent.bats` (driving the real dispatch), so the surviving suite matches Intent house style and the Baize test could be retired with the prototype.
