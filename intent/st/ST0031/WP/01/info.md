---
verblock: "09 Apr 2026:v0.1: matts - Initial version"
wp_id: WP-01
title: "Doc rules: provisioning invariants and FP benchmarks"
scope: Small
status: Done
---

# WP-01: Doc rules: provisioning invariants and FP benchmarks

## Objective

Add the architectural rules and empirical benchmarks that the rest of ST0031 enforces. Documentation lands first so the subsequent skill/script work has written-down invariants to reference in code review and commit messages.

Addresses Lamplight feedback items P0.1 (TCA is its own ST), P0.2 (WPs are flat), and P4 (low dedup rate is a positive KPI). Also captures the P1 empirical benchmark (R8 FP rate dropped from ~82% to 0% with pre-classification) as a data point the rest of the process doc can point at.

## Deliverables

1. New `## 0.0 Provisioning Invariants` section in `intent/docs/total-codebase-audit.md`, inserted between the `# Phase 0: Provisioning` heading and the existing `## 0.1 Define the Rule Set`. Four numbered invariants: TCA is its own ST, WPs are flat, last WP is synthesis, rank components by later-pain impact.
2. New empirical-benchmark paragraph after the "General principle" rule-precision bullet, documenting the Lamplight R8 pre-classification data point.
3. Sharpened framing of the dedup-rate benchmark paragraph to explicitly state that low dedup on newly-authored code is a positive KPI, with the Lamplight 12% data point.

## Acceptance Criteria

- [x] `## 0.0 Provisioning Invariants` section exists in `total-codebase-audit.md` with four numbered invariants
- [x] Each invariant includes a "why it matters" justification with concrete failure modes
- [x] Lamplight ST0121 incident is referenced (commits 75706c18 to 98616a0c, 2026-04-08)
- [x] Correct-layout tree for a fresh TCA is shown under Invariant 2
- [x] R8 pre-classification benchmark paragraph is present after the "General principle" bullet
- [x] Dedup-rate paragraph explicitly treats low dedup on new code as a positive signal
- [x] No manual line wrapping in any edited paragraph
- [x] No Claude attribution in commit messages

## Dependencies

None. WP-01 is the leaf of the dependency graph — docs land first.
