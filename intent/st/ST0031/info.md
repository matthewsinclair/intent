---
verblock: "09 Apr 2026:v0.1: matts - Initial version"
intent_version: 2.4.0
status: WIP
slug: tca-suite-hardening-post-lamplight-st0121
created: 20260409
completed:
---

# ST0031: TCA suite hardening (post-Lamplight ST0121)

## Objective

Harden Intent's `in-tca-*` skill family and `intent/docs/total-codebase-audit.md` to prevent the provisioning and close-out failure modes that manifested during the Lamplight ST0121 TCA run (2026-04-08/09). Make weak guidance enforceable through documented invariants, mechanical pre-flight guards, and required template sections — so the next TCA cannot deviate into the same antipatterns even when the operator is eager to declare victory.

## Context

The Lamplight project ran its first full Total Codebase Audit on ST0121 ("Gen 3.0 Architecture Rollout") on 2026-04-08/09 using Intent's `in-tca-*` skill family. The audit itself worked — 50 source files audited, 17 raw violations found, 10 fixed, 5 false positives, 0 deferred, 4925 tests green, credo strict clean. But the process exposed 8 corrections that need to propagate back to Intent before the next TCA runs anywhere.

The root failure was that the Lamplight operator deviated from the `in-tca-init` skill text. Instead of creating a dedicated steel thread via `intent st new`, the TCA was provisioned as WP-24 inside ST0121 (the steel thread whose code was being audited). This triggered a cascade: sub-WPs were nested inside WP-24 (which the intent CLI does not support), the TCA phase structure collided with the feature-WP template vocabulary, and the audit's close-out became a single checkbox blocking the entire ST0121 close-out. Commit 75706c18 wrote "ST0121 complete" into `wip.md`, `intent/restart.md`, `.claude/restart.md`, and `impl.md` before `feedback-report.md` existed. A 24-hour window of lying docs followed. Commit 98616a0c reconciled the state.

The skills themselves were textually correct — they already use `intent st new` and create a flat WP layout. The gap is that the rules were weak guidance rather than enforceable invariants. An operator eager to finish can skip guidance but cannot bypass a bash guard that exits 1 when preconditions are unmet. This steel thread replaces guidance with guards wherever the failure modes allow it.

The feedback report that drives this work is at:
`/Users/matts/Devel/prj/Lamplight/intent/st/COMPLETED/ST0121/WP/24/feedback-report.md`

## Work Packages

- WP-01: Doc rules — `intent/docs/total-codebase-audit.md` additions (Provisioning Invariants, FP benchmark, dedup KPI framing)
- WP-02: Init hardening — `in-tca-init` callout, False Positive Guidance as REQUIRED, `tca-init.sh` provisioning guards
- WP-03: Finish guard — `tca-report.sh` `--check-only` mode + three guards, `in-tca-finish` skill restructure, canonicalize `$TCA_DIR/feedback-report.md`
- WP-04: Rename `--st-dir` → `--tca-dir` across 6 files, audit metadata line in `in-tca-audit`, `chains_to:` on all 5 TCA skills

## Related Steel Threads

- ST0028: TCA v3.0 — the skill suite this work hardens
- ST0030: Superpowers cherry-picks — introduced `chains_to:` semantics used in WP-04
- Lamplight ST0121: the source of this feedback (external repo)

## Known Unknowns (forensic findings)

- **ReasoningTool circular dependency documentation gap**: The Lamplight feedback report credits the audit with discovering a circular dependency between `lamplight` and `aigency`, but forensic review of the phase1-03-agents-aigency.md audit file finds no such entry. The remediation is real (commit 4147038d moved `ReasoningTool` to `Aigency.Tools`), but the discovery path is undocumented. Not fixable in Intent skill updates — it is a Lamplight-side bookkeeping issue worth noting for process discipline ("every finding in the feedback report must be traceable to a specific phase audit entry").

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
