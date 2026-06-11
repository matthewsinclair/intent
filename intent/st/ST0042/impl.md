# Implementation - ST0042: Fable 5 Review of Intent codebase

## MFIC leak write-up (input to ST0041)

The review was run as the first deliberate MFIC exercise. What held and what leaked, per axis:

### Mechanically (M) -- held, with a known gap

Coverage was enumerated up front (eight dimensions over a declared surface) rather than hand-picked, and the mechanical sweeps did their job: the `[[LANG]]` placeholder leak was found by an exhaustive placeholder-vs-substitution diff, the same heuristic that would have caught this morning's `2.4.0` bug. Gap: verification was not mechanically exhaustive. Of ~60 findings, the load-bearing (WP-determining) ones were independently re-verified; the long tail of LOW findings is single-reviewer `reported`, not `confirmed`. An honest M would either verify all or declare the sample -- this write-up declares it.

### Falsifiable (F) -- held

Every finding carried a refutation condition, and the format forced it. Two findings were strengthened by it: the eval-RCE claim graduated from "reading the eval path" to a live PoC (benign marker file, reverted), and the AGENTS.md misreport from a code claim to a live `grep` of the generated artefact showing "No skills installed" on a provisioned machine. The first reviewer-reported eval finding (F-ARCH-1) was itself an unproven read until the PoC fired -- the F discipline is what closed it.

### Independent (I) -- held at the accepted bounded bar, and it bit

This is the axis the exercise was really testing. The reviewers were fresh-context sessions causally independent of the sessions that wrote the code (and of the v2.11.11 author), and that independence found real bugs the producing sessions shipped: the nine-skill rules-path drift is the exact class v2.11.11 fixed and missed on the skill surface; the test suite pollutes the real `~/.claude`. Corroboration appeared without coordination -- two dimensions (architecture, shell-critic) independently reached the config-eval RCE by different routes. The bar's ceiling also showed: the reviewers share the model's biases, so this is reasonable assurance, not proof, exactly as the VC/CC/IC framing predicted. Notably, a producer-audits-self check (the green Intent test suite) could not have surfaced most of these -- the bugs live in the gap between "self-consistent" and "correct".

### Control (C) -- held as designed (human-adjudicated)

Nothing executed from the review. Findings became a proposed WP slate; the user adjudicates at the gate. This is C with the human as actuator -- the advisory-v0 stance (ST0040) held deliberately. The open question for ST0041: is human-adjudicated C sufficient, or does the recurrence of the same drift class across three releases argue for a mechanical control with blocking authority (the T2 "mechanical guard" WP is a candidate first instance -- a check that fails CI when any propagated artefact cites the dead path)?

### Harvest candidates for ST0041

1. The T2 mechanical guard is a concrete MFIC instance worth building regardless of WP sequencing: oracle-free (greps for a forbidden string in generated output), independent (not authored by the generator), and a real C if wired into the pre-commit/CI gate. It would be Intent's first all-four-axes control.
2. The test-suite findings (T10) are the evidence that Intent's bats layer fails the I-litmus -- authored by the producing sessions. ST0041 should consider whether "tests written by a different session/model than the code" is a discipline Intent can adopt cheaply via the whiteboard Verifier role.
3. The "reviewer independence found what the green suite could not" result is the empirical core: it justifies the I axis as load-bearing for LLM-directed production, not theoretical.

## Review methodology notes

- Eight reviewer agents (seven planned + upgrade-rethink), each constrained to a falsifiable-finding output format, run against a declared coverage map with explicit scope exclusions.
- Mechanical sweeps (placeholder drift, hardcoded versions, shellcheck, bash-3.2 compat, size outliers) run inline ahead of the agents.
- Load-bearing findings re-verified against the code before graduating to proposed WPs; the eval-RCE additionally demonstrated by PoC.
