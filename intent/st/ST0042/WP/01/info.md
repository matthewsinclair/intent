---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: "Eliminate config eval"
scope: S
status: Not Started
---

# WP-01: Eliminate config eval

## Objective

Remove the `eval` of jq-interpolated config values from `load_intent_config` (theme T1, confirmed by PoC). A value containing `$(...)`, backticks, or `$VAR` in a checked-in `intent/.config/config.json` (or `~/.config/intent/config.json`) currently executes arbitrary shell on the next project-scoped `intent` command. Recorded as a robustness/design defect per the audit framing -- the fix is to stop evaling, not to sanitise.

## Evidence

- `bin/intent_config:35` -- jq `to_entries` interpolation builds `key="value"` strings from raw JSON values.
- `bin/intent_config:84` and `:96` -- `eval` of that output.
- Reached from `bin/intent:181-183` for every non-global command.
- PoC (run + reverted during review): `"author": "$(touch /tmp/intent_eval_pwned)"` created the marker on `intent st list`.

## Deliverables

- `load_intent_config` reads each known config field with a field-wise `jq -r '.field'` read (the `get_config_field` approach already in `intent_helpers`), assigning variables directly with no `eval` anywhere in the path.
- Regression test: a config value containing `$(...)` / backticks is loaded verbatim into the variable and executes nothing.
- Existing config-loading behaviour preserved (defaults, local-over-global precedence).

## Acceptance Criteria

- [ ] No `eval` remains in `bin/intent_config` (or anywhere on the config-load path).
- [ ] PoC config value no longer executes; the literal string lands in the variable.
- [ ] Regression test red-phase-proven against the pre-fix code where practical, green after.
- [ ] Full bats suite green; critic-shell clean on touched scripts.

## Dependencies

- WP-09 part A (fake-HOME test helper) should land first so suite runs are safe; otherwise none.
