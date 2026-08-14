---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-07
title: "Canon and claude subsystem"
scope: L
status: Not Started
---

# WP-07: Canon and claude subsystem

## Objective

Embed the canon (templates, skills, rules, agents) in the binary and port the `intent claude` family, making the 0022 broken-install class unconstructible and keeping consumer hooks byte-compatible through the v3 swap.

## Deliverables

- Embedded canon via rust-embed: lib/templates, canon skills, the rule library, subagent definitions; INTENT_HOME demoted to a dev override
- `intent claude` family: skills install/sync/uninstall (SHA256 manifests), rules list/show, subagents, `hook <name>` byte-compatible from day one (0016 -- consumer settings.json must not notice the swap)
- The critic headless runner (`intent critic <lang>`) reading the embedded rule library, strict-proxy contract preserved (ST0039)
- `intent agents sync` generating AGENTS.md from project state
- `intent lang` family over the declared-languages config

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-07` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-04. Interleaves freely with WP-06.
