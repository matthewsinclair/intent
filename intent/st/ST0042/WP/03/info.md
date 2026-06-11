---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-03
title: "Kill reports-success-while-doing-nothing"
scope: S
status: Not Started
---

# WP-03: Kill reports-success-while-doing-nothing

## Objective

Fix four No-Silent-Errors violations (theme T3) where a command prints success after a guarded operation silently did nothing. Each path either does the work or errors via `error()` -- never lies.

## Evidence

- F-TPL-4 (MEDIUM): `intent st new` legacy single-file path references `lib/templates/prj/st/_ST####.md` which does not exist; the `if [ -f ]` guard creates no file yet `created:` prints (`intent_st:491,502,505-506`).
- F-TPL-9 (MEDIUM): `intent agents init --template {rust,lua,shell,swift}` prints "Created AGENTS.md at project root." while the guarded copy created nothing -- those template dirs have no AGENTS.md (`intent_agents:176-179,204-205`).
- F-TPL-11 (MEDIUM): `intent_init` interactive agent install invokes `"$SCRIPT_DIR/intent_agents"`, a path that does not exist (real location `intent/plugins/agents/bin/intent_agents`), so the "Y" branch can never succeed (`intent_init:283`).
- F-shell (CRITICAL): upgrade backup uses `cp -r ... 2>/dev/null || true` then prints "Backup created successfully" unconditionally, immediately before destructive migration (`intent_upgrade:140-144`). Scope note: ST0043 redesigns backup; this WP only makes the message truthful (verify the copy, `error()` on failure) -- minimal change, no redesign.

## Deliverables

- Each of the four paths either performs the action and reports truthfully, or surfaces failure via `error()` from `bin/intent_helpers`.
- Regression test per fix (bats) demonstrating the failure mode is gone.

## Acceptance Criteria

- [ ] No success message prints on any of the four paths unless the action verifiably happened.
- [ ] Backup failure during upgrade aborts before any destructive step.
- [ ] Full bats suite green; critic-shell clean on touched scripts.

## Dependencies

- WP-09 part A (fake-HOME helper) first, since upgrade-path tests are involved.
