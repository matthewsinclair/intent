---
st_id: ST0066
title: Add 'fiat close' as a feature to close STs, WPs, ACs, and ATs
---

# ST0066: Add 'fiat close' as a feature to close STs, WPs, ACs, and ATs -- Acceptance

> **THIS FILE IS A GENERATED VIEW, AND A ROW AUTHORED HERE IS DISCARDED BY THE NEXT SYNC.** The acceptance contract is canon in the thread model; this file renders it. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 (non-test) `intent fc <target> --because "<text>"` exists and fiat-closes any of the four kinds (ST, WP, AC, AT); invoked without `--because` it refuses on stderr with a nonzero exit and writes nothing. -- evidence: driven 2026-08-30 (dc) on a tree-hash instrument positive-controlled BOTH ways first -- a pure read leaves it unmoved, a real write moves it. REFUSAL ARM: no --because refuses at rc=1 writing nothing, for all four kinds, via clap; --because "" and "   " refuse at rc=1 writing nothing via THE MODEL (`st.fc` requires a reason and was given none), which is the arm that proves the tool and not the parser -- clap is satisfied by an empty string. SUCCESS ARM: rc=0 on a FRESH FIXTURE PER KIND because fc on an ST cascades and closes the rest -- ST (gate BLOCKED 0/2 -> PASS 0/2 satisfied, 2 fiat-closed), WP (-> BLOCKED 0/2, 1 fiat-closed, correctly scoped to its own AC), AC, AT. ST and WP driven from `wip`: fc lands where done lands, so triage/not-started are pre-work states with no close to make -- a first drive from the fixture's default states refused, and that was the fixture, not the verb. -- satisfied: yes
- AC-00.2 (non-test) The fiat record lives IN THE MODEL on the closed entity -- by, date, because, and invoker evidence (tty-or-not, timestamp, environment fingerprint) -- and survives the store-to-extract-to-store round-trip byte-faithfully. -- satisfied: no
- AC-00.3 (non-test) FC on an ST or WP fiat-closes its open children, and every cascaded child carries an inherited-fiat marker naming the ancestor FC it descends from; no cascaded child renders as ordinarily closed. -- satisfied: no
- AC-00.4 (non-test) **A FIAT CLOSE IS COUNTED WHEREVER THE THING IT CLOSED WAS COUNTED, AND IT IS DISTINGUISHABLE FROM AN ORDINARY SATISFACTION IN THE SAME LINE THAT COUNTS IT.** The close gate counts a fiat-closed requirement as closed -- unblocking is FC's purpose -- and every gate and status render marks it as fiat rather than as satisfied, in the line carrying the count and not in a footnote beside it.

**REWRITTEN 2026-08-29 (vc) UNDER hv's `0123` RULING, ON dc's VACUITY FINDING, AND THE REWRITE IS THE FINDING.** Original text: *The close gate counts a fiat-closed requirement as closed -- unblocking is FC's purpose -- and every gate and status render distinguishes fiat-closed from ordinarily-satisfied in the same line that counts it.* Its first half is TRUE OF THE WP KIND FOR THE WRONG REASON: **the thread gate never counts WPs at all**, so *the gate counts a fiat-closed WP as closed* holds because the gate counts zero WPs either way, and the strongest-sounding half of the FC contract would have passed on the kind it governs by that kind being absent from the instrument.

**hv RULED THE BEHAVIOUR CORRECT AND THE RECORD MISSING** (`0123`, menu: contract is the authority -- CHOSEN | `done` not assertable over Not Started | park it): an AC-free WP's contract IS the thread's per the ratified rollup, so a passing thread contract leaves that WP nothing outstanding, and `status` is work-tracking rather than a second contract. **So this row is rewritten to what is actually checkable per kind rather than built to a letter that passes vacuously** -- the AC-11.1 / AC-07.5 precedent, where a row named a mechanism that made it unsatisfiable and the fix was to state the outcome.

**WHAT EACH KIND OWES.** For AC and AT, the gate counts the row directly and the fiat close moves the number. For ST and WP, the claim is about the ROLLUP and not about a per-WP tally the gate does not keep: a fiat-closed ST or WP leaves no unsatisfied requirement attributed to it, and its fiat provenance renders wherever its closure does.

**AND THE ROW CARRIES ITS OWN POSITIVE CONTROL, WHICH IS THE GENERAL RULE IT ADDS.** `X is counted as closed` is unfalsifiable unless the instrument can be shown counting X at all, so the evidence is a PAIR of runs across the fiat close -- the count before and the count after -- and a kind whose number cannot be made to move is not thereby passing, it is unmeasured. **A criterion satisfied by an instrument that could not have reported otherwise is the dominant defect class on this estate**, and this row was very nearly its cleanest instance: it would have gone green on the strength of the gate having no opinion about WPs. -- satisfied: no
- AC-00.5 (non-test) doctor renders a fiat-closed row distinctly wherever it reports one, and never proposes a remedy that would convert a fiat close into an ordinary one. -- evidence: driven 2026-08-30 (dc), both halves, after a vacuity check run BEFORE any build found the row had a live subject and was FAILING. FIRST HALF: doctor rendered a fiat-closed scope byte-identically to an ordinarily satisfied one (diff clean, same fixture shape, one state apart) while the close gate distinguished the same two states in its own tally -- so the distinction was computed and doctor was the surface not asking. Fixed at 8541c59c by carrying the fiat count on Verdict::Pass beside the detail, the arrangement `unsatisfied` already uses; Verdict is not a serialised face so no schema moved. SECOND HALF: the shared remedy said "either satisfy them or take them out of scope", which over a fiat row is the conversion this row forbids; remedy is per-CLASS so it was split to FindingClass::StatusGateDisagreementOverFiat at c9c48440. The new remedy was itself DRIVEN, not written -- its first version named `wp done`, which is refused from Not Started, the very state the finding fires on; corrected, and following it exactly clears the finding while `ac list` still reads fiat-closed. REACH: the status-gate path is the only one of doctor's 21 finding classes whose finding is a claim about a row's CLOSURE; the other 20 were enumerated and none reports closure state. Census arm compares class AND prose, driven red on the pre-fix branch. -- satisfied: yes
- AC-00.6 (non-test) The LLM-side prohibition ships in the rules library and the relevant skills, so an LLM's own loaded contract forbids invoking `fc`; the rule's Detection section states plainly what is mechanically checkable about an FC invocation and what is not -- enforcement is detection plus attribution, never a claimed impossibility. -- satisfied: no

## Acceptance Tests

### ST-level

_(no tests in this group)_

---

_Generated by Intent v3.0.0 from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
