---
verblock: "11 Jun 2026:v0.1: matts - Initial version"
intent_version: 2.11.11
status: Completed
slug: explore-mfic-mechanically-falsifiable-independent
created: 20260611
completed: 20260611
---

# ST0041: Explore 'MFIC — Mechanically-Falsifiable Independent Control'

## LLM Preamble

The following is from a Slack conversation where this idea was discussed:

```
MFIC — Mechanically-Falsifiable Independent Control
How to trust code or data when a fallible author (LLM or human) could quietly corrupt it. TDD's red phase proves a test can fire — that is the F alone; MFIC adds the other three axes. Framing: internal control over LLM-directed production, the COSO/Sarbanes-Oxley control model with the LLM as the new untrusted actor.
Definition: an independent harness at a handoff boundary that derives its check from the contract or the data itself — never from the producer's account of it — turns each handoff into a refutation that bites, and holds authority to reject, halt, or steer the flow.
Litmus: if the same agent wrote both the check and the thing checked, could it pass with wrong work? Yes → gameable, not MFIC. No → MFIC.
Each word is load-bearing (drop one, name what leaks in):
Mechanically — cases/oracle machine-swept, not hand-picked. ¬M → omission bugs.
Falsifiable — each case is a biting refutation, a verdict you can't pre-arrange. ¬F → vacuous greens, or weak oracles ("doesn't crash" hiding wrong output).
Independent — truth source causally independent of the producer (≡ audit segregation of duties: maker ≠ checker). ¬I → collusive tests, the LLM trap; the axis TDD leaves open.
Control — a baseline to deviate from + authority to fail/steer (scientific control + process control + control-theory feedback). ¬C → telemetry that observes but never blocks.
Two regimes:
Static (verify code, in CI) — exhaustion, differential, metamorphic, mutation/shotgun, property-based, perf/leak gates. Detective.
Dynamic (verify state in flight, in prod) — stage-boundary contracts, key/schema preservation, bounce-back loops. Preventive (reject before next step) + Corrective (bounce to previous).
Reach for the strongest available, cheapest first: finite domain → exhaust it; inverse exists → round-trip; reference exists → differential; invariant survives a transform → metamorphic; can corrupt known-good → mutation/shotgun (yields a number, not a bool); else → property-based + shrink. Prefer the oracle the author didn't write; oracle-free (metamorphic, shotgun, invariant-derived-from-t0) beats authored-oracle.
Guardrails: ordinary validation isn't MFIC — it usually fails I (same agent defines the schema and produces against it) or C (logs only). A shotgun needs a paired specificity corpus (all known-good must pass) or a reject-everything checker scores 100%; bucket bits into must-detect vs may-ignore (padding/CRC-excluded don't count). Evidence is constitutive— a control with no trail didn't run; version measurement history by commit + hardware. Statistical members give reasonable assurance, not proof — the recognized standard, not a deficiency.
Prior art: mutation/metamorphic/differential/property-based testing, fault injection (software V&V); segregation of duties, preventive/detective/corrective controls, audit trail, test of controls, reasonable assurance (COSO / Sarbanes-Oxley).
```

Please use that as input context for this steel thread. Ask whatever questions are necessary.

## Objective

Determine what of MFIC (Mechanically-Falsifiable Independent Control) Intent should adopt, empirically rather than speculatively: run ST0042 (the Fable 5 review of Intent) as the first deliberate MFIC exercise, observe which axes hold and which leak, and harvest what works back into Intent as doctrine, mechanism, or both.

## Context

Rubber-duck outcome (2026-06-11):

- **Exploration is empirical.** Rather than write MFIC doctrine in the abstract, ST0042 is structured as an MFIC instance: reviewer causally independent of the producing sessions, mechanical sweep over the codebase, findings as falsifiable claims with file:line evidence and refutation conditions, and the human review gate as the control point. The write-up of what leaked feeds this thread.
- **The I-axis bar is accepted as bounded.** By the nature of agentic coding, full independence is unattainable in practice; fresh-context same-model checking via the VC/CC/IC triumvirate and the whiteboard protocol is about as good as it gets. Failure modes remain correlated (same model biases); oracle-free checks (metamorphic, round-trip, mutation) are preferred wherever the oracle ladder allows, because they dodge the authored-oracle collusion problem entirely.
- **The C axis stays human-adjudicated for now.** Intent's advisory-v0 philosophy (ST0040) holds: controls surface and the user actuates. Whether any control should gain blocking authority is a question this exploration may answer, not a premise.
- **Intent's current stack has every axis somewhere and all four nowhere.** Pre-commit critic gate: real C, weak M (regex proxies; ST0039 was an F-axis repair). `/in-verify`: evidence-is-constitutive, but producer-audits-self (not I). Whiteboard Verifier role (v2.11.10): first genuine I, deliberately not C. Bats suite: authored by the producing sessions (fails the litmus).

The originating Slack framing is preserved in the LLM Preamble above.

## Related Steel Threads

- ST0042 -- Fable 5 review of Intent; runs as MFIC exercise #1, its leak write-up is this thread's primary input
- ST0039 -- critic-runner strict-proxy contract; precedent F-axis fix (checks that cannot bite are refused)
- ST0040 -- whiteboard protocol; carrier of the Verifier role and the advisory-v0 C-axis stance

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
