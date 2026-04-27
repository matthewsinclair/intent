---
verblock: "24 Apr 2026:v0.1: matts - Phase 0 skeleton"
---

# Implementation Log — ST0035

This file fills out as WPs complete. Each WP's implementation notes are captured here; per-WP acceptance verification lives in each `WP/NN/info.md` under its `As-Built` section.

## Implementation notes by WP

### WP01 — Self-upgrade + cancel ST0010/ST0015

_Not started._

### WP02 — Refresh root usage-rules.md

_Not started._

### WP03 — Write working-with-llms.md

_Not started._

### WP04 — .claude/settings.json template

_Not started._

### WP05 — bin/intent_critic runner

_Not started._

### WP06 — .git/hooks/pre-commit template

_Not started._

### WP07 — .intent_critic.yml template

_Not started._

### WP08 — Root AGENTS.md generator

_Not started._

### WP09 — Root CLAUDE.md overlay template

_Not started._

### WP10 — Delete deprecated artefacts

_Not started._

### WP11 — Extend intent claude upgrade

_Not started._

### WP12 — Socrates/Diogenes FAQ cross-refs

_Not started._

### WP13 — Update Intent own CLAUDE.md

_Not started._

### WP14 — Self-apply canon to Intent (dogfood)

_Not started._

### WP15 — Canary rollout (Conflab, Lamplight, Laksa)

_Not started._

### WP16 — Fleet rollout (13 projects)

_Not started._

### WP17 — Verification sweep + dogfood journal

_Not started._

## Cross-WP technical concerns

Tracked here as they surface:

- **User-section preservation markers** (CLAUDE.md, settings.json, usage-rules.md). Pick one marker convention (HTML comment), document, reuse across WP09, WP11, WP04. Flag anyone who uses a different marker.
- **`bin/intent_critic` and subagent parity**: WP05 must ship a parity-verification report (same file, same rules, identical output). If not identical, document divergences and whether they're acceptable.
- **`intent claude upgrade --apply` idempotence**: WP11 must pass two consecutive `--apply` with zero diff. Test in WP14.

## Code examples

(To be filled from WP implementations. Aim: 1–3 illustrative snippets per WP where non-obvious.)

## Challenges and solutions

(To be filled. Candidate challenges from design.md risk register: parity-divergence in bin/intent_critic; hook-reminder compliance; user-marker preservation edge cases.)
