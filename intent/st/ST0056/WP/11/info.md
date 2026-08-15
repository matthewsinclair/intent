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
- ~~INTENT_HOME retired to a documented dev override~~ **STRUCK 2026-08-15 (dc measured, vc reworded): there is nothing to retire.** v3 has ZERO `env::var("INTENT_HOME")` call sites -- the only runtime environment read in the whole binary is `COLUMNS` -- so AC-11.3 is satisfiable by construction rather than by a retirement. The "documented dev override" this named is **rust-embed's read-templates-from-disk mode, which is WP-07's, not distribution's**. Note for whoever evidences AC-11.3: `strings <binary> | grep INTENT_HOME` returns 3 hits and is **100% false-positive** -- they come from `surface/dispatch-table.json`, compiled in via `include_str!`, as parity prose describing v2. Presence in the binary is not a read
- install / upgrade docs
- Release mechanics for the Rust workspace (the bin/release successor decision)

## Acceptance

Acceptance Criteria for this work package live in the steel thread's `acceptance.md`, under the `WP-11` heading (single source of truth). Do not restate ACs here.

## Dependencies

- WP-06 (a surface worth shipping).
