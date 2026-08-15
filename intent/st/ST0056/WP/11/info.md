---
verblock: "14 Aug 2026:v0.2: vc - Populated from the ratified design session"
wp_id: WP-11
title: "Distribution: cargo-dist, Homebrew, signing"
scope: M
status: WIP
---

# WP-11: Distribution: cargo-dist, Homebrew, signing

## Objective

Make `brew install intent` the install story: cargo-dist release artefacts, the Homebrew tap, and the macOS signing/notarisation posture -- retiring the clone-and-symlink v2 install model.

## Deliverables

- cargo-dist wiring: release artefacts + tap formula for the binaries per the WP-01 one-vs-two decision
- macOS signing/notarisation decision + implementation (adhoc vs Developer ID; the Conflab TN3171 lesson recorded for any TLS-bearing future)
- `brew services` story for intentd (launchd interop with the WP-08 lifecycle)
- INTENT_HOME retired to a documented dev override; install/upgrade docs
- Release mechanics for the Rust workspace (the bin/release successor decision)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-11` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-06 (a surface worth shipping).
