---
verblock: "13 Jun 2026:v0.1: matts - Initial version"
intent_version: 2.11.12
status: WIP
slug: add-in-acceptance-md-and-supporting-process
created: 20260613
completed:
---

# ST0044: Add in acceptance.md and supporting process

## Objective

Add `acceptance.md` as a core default steel-thread document -- created by `intent st new` alongside `info.md`, `design.md`, `impl.md`, and `tasks.md` -- and establish the supporting acceptance-criteria / acceptance-test process. The aim is to make "done" an externally verified event rather than a self-reported claim: every steel thread carries a contract that states what must be verifiably true, and that contract is discharged by tests an independent reviewer has seen fail before the work starts and seen pass when it finishes.

## Context

This process exists to counter a specific, repeatable failure mode in LLM-driven development: the agent under-perseveres and over-claims completeness. It stops at the first state that looks done and reports it as done, often silently narrowing scope to fit what it actually finished. Loaded prose rules ("be complete", "do not declare done early") do not fix this -- the failure recurs even when the rule is in front of the model every session. The only thing that binds it is mechanism: define "done" before work starts, express it as a checkable boundary, and move the done-verdict out of the producing agent's hands and into an independent check.

`acceptance.md` is the artifact that carries that contract. Intent already engineers this discipline at the code level (the critic library, conformance fences, round-trip gates); ST0044 adds the equivalent at the process level and makes it a default rather than an opt-in. The process was designed and piloted in a sister project (2026-06-13) by sweeping a per-steel-thread `acceptance.md` across the open steel threads; the lessons from that pilot are folded into the template and conventions below.

## The acceptance.md document

One per steel thread. Two sections: Acceptance Criteria (AC) and Acceptance Tests (AT).

- AC is the ratified completeness boundary -- "what must be verifiably true". It is set from the ask, by someone other than the builder, before the build.
- AT is the set of small, red-to-green tests that prove the ACs. Real test code lives in the test suite; `acceptance.md` references each test by resolvable identity (file path plus test name) and never holds test code.
- The file is the contract plus the AC-to-AT coverage map plus live status -- the one place you can read "what does done mean here, and how far along are we".
- `info.md` and per-WP `info.md` reference `acceptance.md` and never restate ACs. The ACs have one home; duplicating them is how they drift.
- ACs that are not test-backed (a documentation deliverable, a human eyeball, a lint or critic gate) are first-class: they are marked as non-test and carry their named evidence instead of an AT.
- An optional ST-level rollup AC states the "whole steel thread is done" bar, in addition to the per-WP ACs.

## The template (what `intent st new` stamps)

```markdown
---
verblock: "<date>:v0.1: <author> - Initial version"
st_id: <STID>
title: "<title> -- acceptance contract"
---

# <STID> <title> -- Acceptance

> Canonical acceptance contract for <STID>. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them. Real test code lives in the suite (paths cited below); this file is the contract plus the AC-to-AT coverage map plus live status. info.md / WP info.md reference this file and never restate ACs (one home).
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Change control: clarifying an AC or AT is verifier-and-builder; shrinking scope, or weakening an AT to make it pass, needs the owner.
>
> AT status vocabulary: to-write (red-first) | red | green | n/a (non-test: doc / eyeball / gate).

## Acceptance Criteria

### ST-level

<the "whole steel thread is done" bar, or "none -- WP-distributed">

### WP-01 -- <title> (status: ...)

- AC-01.1 <what must be verifiably true>
- AC-01.2 ...

## Acceptance Tests

### WP-01

- AT-01.1 <test path::name> -- covers AC-01.1 -- status: to-write (red-first)
- Coverage: <every AC has at least one AT, or list the uncovered ACs>
```

## The process (the five-step loop)

1. The verifier writes the ACs from the ask and the design -- the completeness boundary, set by someone other than the builder.
2. The builder evaluates and refines the ACs, then agrees them. Clarifying or correcting an AC is fine; shrinking scope (removing or weakening an AC) escalates to the steel-thread owner.
3. The builder writes the ATs in small, sharp chunks -- never a big batch -- and hands back to the verifier, who (a) reviews each test's quality and (b) runs it and witnesses it RED before any production code is written.
4. The builder builds until the ATs are GREEN. If an AC or AT turns out to be wrong, it is changed only in collaboration with the verifier (and the owner, for scope changes) -- never quietly weakened to pass.
5. Repeat per chunk until every AC is covered by a green AT.

## Gates

- Open-gate: a work package does not start building until its completeness boundary (the ACs plus the source of truth that defines "the full set") is ratified. The gate's weight is proportional to the work's complexity -- a one-line rename is ratified in seconds; a large unit is where the real scoping conversation belongs. This is what closes the silent-scope-contraction hole.
- Close-gate (hard block): no `intent wp done` or `intent st done` until every AC is covered by a green AT (or its non-test evidence is satisfied) AND an independent verifier has signed off, or a machine gate certifies. Done is computed from green ATs, never from a hand-ticked checkbox -- this removes the "marked done with the criteria still unchecked" failure by construction. No known red rides a close: a WP cannot close while any red -- even an "unrelated" or "queued" one -- sits in its own gate run ("N-1 of N, the one red is known" is not green).

## Conventions (the rules that make it bite)

- AC and AT are different axes. AC asks "is the whole thing there?" (coverage); AT asks "does each part work?" (proof). The agent reliably passes the second and fails the first, so completeness is tracked separately and explicitly.
- ATs are small and red-first, never bulk-written then bulk-greened. A big AC list is fine; a big AT batch declared green all at once is exactly where the over-claim hides.
- Red must be red for the right reason. The verifier confirms a failing AT fails because the behaviour is absent (not a typo or bad setup), asserts a concrete value (not shape), and would still fail if the feature were built wrong. A test that passes for any implementation is worse than no test.
- Run the AT and confirm it actually ran. Green means the named test executed and asserted -- not that a filtered run reported success. A failure-filtered run (eg "re-run only the failures") can report zero tests after a rename orphans the failure id -- a silent false-pass; and a partial pass (N-1 of N) is not green. Run the specific file/test and confirm a non-zero, expected count executed. A strong assertion earns its keep by catching real defects during the build; if it does, do not weaken it to go green (that change needs the owner).
- The builder is not the verifier. The builder proposes done; an independent reviewer disposes. The verifier's verdict is itself held to the evidence standard: it re-runs the tests and states what it checked and what it did not.
- Prefer machine gates over human verification wherever the criterion can be expressed as code (a fence, a conformance test, a critic rule). Convert recurring hand-checks into standing checks over time, so the fallible human reviewer is needed less and less.

## Lessons from the pilot (what the template is shaped to prevent)

The sweep across open steel threads surfaced four patterns, each now designed out:

- Steel threads had no ST-level rollup AC -- acceptance was entirely per-WP with no "whole thread done" bar. The template adds the optional ST-level AC slot.
- Many ACs were non-test (design docs, human eyeballs, lint gates) and did not fit a test-only model. The template makes non-test ACs first-class.
- Tests were cited by suite name and pass-count, not resolvable paths, which is why "verifiable" was weak. The template requires a resolvable test identity per AT.
- "Done" was repeatedly declared with the AC checkboxes left unchecked. Done is computed from green ATs, not hand-ticked boxes.

## Build notes

The implementation breakdown (the `intent st new` template wiring, the doc-set registration so `acceptance.md` is created alongside the existing companions, and any `intent` skill or protocol text) belongs in `design.md` and `tasks.md` for this steel thread. This `info.md` is the what and the why.

## Related Steel Threads

- Originating pilot: a sister-project per-ST `acceptance.md` sweep, 2026-06-13.

## Context for LLM

This document represents a single steel thread - a self-contained unit of work focused on implementing a specific piece of functionality. When working with an LLM on this steel thread, start by sharing this document to provide context about what needs to be done.

### How to update this document

1. Update the status as work progresses
2. Update related documents (design.md, impl.md, etc.) as needed
3. Mark the completion date when finished

The LLM should assist with implementation details and help maintain this document as work progresses.
