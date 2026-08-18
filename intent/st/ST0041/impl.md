# Implementation - ST0041: Explore 'MFIC — Mechanically-Falsifiable Independent Control'

## Implementation

The "implementation" of this exploration is the harvest: what MFIC observations from the ST0042 exercise become Intent practice. Source data: `intent/st/COMPLETED/ST0042/impl.md` (review-phase leak write-up + execution-phase notes).

## Harvest — adopted as working practice (proven in the exercise)

1. **Red-phase against the pre-fix tree.** Guard tests are proven red against HEAD (`git grep` on the committed pre-fix state) before the fix lands. The t0 tree is a causally independent oracle the author cannot pre-arrange — partial I recovered cheaply, F made non-negotiable. Used in WP-02 and WP-08; should be the default for every defect-class guard.
2. **Every defect-class fix ships a mechanical guard.** Fix the instances, then pin the class: `rules_path_guard.bats` (dead rules path + unsubstitutable placeholders), `docs_completeness.bats` phantom pins (commands/hook-keys/paths that never existed), the version-fallback and ext-root Highlander pins in `config.bats`. A class that regressed three releases running (T2) stops regressing when a grep can refuse it.
3. **Untested module = unfalsified claim.** First-time behavioural coverage for `intent llm` / `intent organize` / `intent claude prime` immediately exposed two commands that had been dead for releases. Coverage priority should follow "which modules carry zero falsification", not line-count metrics.
4. **Gate integrity is a maintenance obligation.** `intent modules check` had three permanent false stales; a gate that always fails is ignored and stops being a control (C-axis erosion). Treat persistent false positives in any gate as a defect with the same priority as a missing gate. Same lesson as ST0039's strict-proxy contract, now observed on a second gate.
5. **Vacuous-test detection is a real review dimension.** "Does this test ever invoke the product?" caught three whole files asserting on self-defined data (T10). Candidate critic heuristic for the test rule packs.

## Harvest — candidates deferred (revisit on evidence)

- **Promoting the suite-resident guards to an independent CI gate** (an all-four-axes control: mechanical sweep, biting refutation, not authored by the generator it checks, blocking authority). Deferred until CI exists as a distinct surface from the local suite.
- **Cross-session test authorship** (tests written by a different session than the code, via the whiteboard Verifier role). The exercise's executor authored its own guards — bounded I accepted for now; the Verifier-role discipline is the upgrade path if authored-oracle collusion is observed in the field.
- **Blocking authority for any advisory control.** No evidence yet that human adjudication is insufficient; advisory-v0 holds.

## Challenges & Solutions

- The strict I-litmus ("same agent wrote both the check and the thing checked") fails for most of what one session can do alone. Resolution: distinguish authored-oracle checks (fail the litmus) from oracle-free checks (pattern pins, t0-tree red-phase, external contracts) — the latter retain refutation power even with a single author, and the exercise repeatedly showed them biting their own author.
