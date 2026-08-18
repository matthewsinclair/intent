# Design - ST0041: Explore 'MFIC — Mechanically-Falsifiable Independent Control'

## Approach

Empirical, not speculative: rather than write MFIC doctrine in the abstract, run ST0042 (the Fable 5 review of Intent and the execution of its WP slate) as the first deliberate MFIC exercise, observe which axes hold and which leak, and harvest only what demonstrably worked. The axis-by-axis observations live with the exercise itself — `intent/st/COMPLETED/ST0042/impl.md` carries both the review-phase leak write-up and the execution-phase notes; this thread holds the conclusions (impl.md here).

## Design Decisions

- **The I-axis bar is bounded by construction and that is accepted.** Fresh-context same-model review (the VC/CC/IC framing) is the practical ceiling for agentic coding; failure modes stay correlated. The compensations that worked in the exercise: prefer oracle-free checks (the oracle is a forbidden pattern, an external contract, or the pre-fix tree — never the author's account), and red-phase proof against HEAD so the t0 state refutes the check rather than the author vouching for it.
- **The C axis stays human-adjudicated (advisory-v0, per ST0040).** The exercise gave no evidence that any control needs blocking authority beyond the existing pre-commit gate; it gave strong evidence that gate _integrity_ is the thing to maintain (harvest item 4 in impl.md).
- **Falsifiability is the cheapest axis to enforce and the highest-yield.** Every place the exercise forced a check to be able to bite, it bit — the eval PoC, the red-phase rewrite of WP-01's vacuous regression test, and the two dead commands exposed the moment untested modules got behavioural coverage.

## Alternatives Considered

- Writing an MFIC doctrine document into canon first, then trying it: rejected — the rubber-duck session (2026-06-11, recorded in info.md) concluded the doctrine would be speculation until an exercise produced leaks to write about.
- Building a blocking all-four-axes CI control immediately (the T2 mechanical guard wired to fail CI): deferred — the guard exists as bats tests (`rules_path_guard.bats`, `docs_completeness.bats` phantom pins) which run in the suite; promoting them to an independent CI gate is a candidate next step, not a v0 requirement.
