---
verblock: "25 Jun 2026:v0.2: matts - Scope the provisioner WP"
wp_id: WP-01
title: "Whiteboard provisioner: ws new + ws list"
scope: Small
status: Done
---

# WP-01: Whiteboard provisioner: ws new + ws list

## Objective

The deterministic node scaffolder and roster read -- the automated form of the Protocol 3.0 "Scaffolding a node". `ws new <wsid>` provisions a workstream to spec (any short-ish slug; `hv` as Workstream Zero on first creation); `ws list` reports the roster from frontmatter. This is the foundation `start` (WP-02) and the lifecycle ops (WP-03) build on.

## Deliverables

- `bin/claude_with_intent` skeleton (POSIX sh, `bin/cli` house style, subcommand dispatch) with `ws new` + `ws list`.
- `ws new`: scaffolds `<wsid>/wip.md` (valid frontmatter), `<wsid>/.history/.gitkeep`, per-peer inboxes (header + `_(empty)_`); idempotent-safe (refuse-on-exist); id validation (No-Silent).
- `hv` provisioned as Workstream Zero on first whiteboard creation; working nodes made-to-order.
- `ws list`: one line per node from `wip.md` frontmatter, read-only.
- The shell-test harness selection (bats vs plain-sh-runner) + AT-01.1..01.5.
- Dogfood: regenerate the Baize `hv`+`cc`+`ic`+`vc` skeleton, diff against the golden reference (AC-01.6).

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-01` heading (single source of truth). Do not restate ACs here.

## Dependencies

- None (foundation WP). Reads the Protocol 3.0 format (the `/in-whiteboard` skill / README) as the scaffold spec.
