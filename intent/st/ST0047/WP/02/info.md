---
verblock: "25 Jun 2026:v0.2: matts - Scope the launcher WP"
wp_id: WP-02
title: "Session launcher: start|st with provision-if-absent"
scope: Small
status: Done
---

# WP-02: Session launcher: start|st with provision-if-absent

## Objective

The headline command: `start <ws>` launches a Claude Code session bound to a workstream with the verified flag combo and a composed context, lands it interactive, and waits. Includes the provision-if-absent flow and the `CWI_DRY_RUN` test seam.

## Deliverables

- `start|st <ws>`: assembles `claude --effort max --permission-mode auto --append-system-prompt "$(compose_ctx <ws>)" "/in-session"` (the verified 2.1.191 form).
- `compose_ctx <ws>`: identity line + `.claude/restart.md` + the standing daily-plan-then-wait instruction (board excluded -- pickup reads it live).
- `CWI_DRY_RUN` / `--dry-run`: print the assembled argv instead of exec (the unit-test hook + safe preview).
- Provision-if-absent: absent `<ws>` -> report + prompt; `n` exits clean, `y` -> `ws new` then launch.
- Resolve the proceed-to-plan spike (does the `/in-session` seed flow into the plan, or need a nudge); confirm the `auto` posture against the TUI status line + lock the value.
- AT-02.1..02.3 (dry-run-backed) + AC-02.4/02.5 live-launch evidence.

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-02` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-01 (`ws new` -- the provision-if-absent path calls it; `start` targets a provisioned workstream).
- External: Claude Code >= 2.1.191; `require-in-session.sh` slash-exemption (the seed relies on it).
