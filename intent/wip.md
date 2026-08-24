---
verblock: "24 Aug 2026:v1.18: vc - globalfold; two of three burning cases re-driven and dead"
intent_version: 2.19.0
---

# Work In Progress

**Current as at `1e8e3666`, 2026-08-24. This heading names a COMMIT, not a date** -- a wip file is read as current and written as a snapshot, and if you cannot say what it is current as at, that is the finding.

## The gate: 66 of 67

**RUN THESE. DO NOT TRANSCRIBE THE NUMBER.** It has had three homes carrying three values, and one document held it twice disagreeing with itself.

    intent ac status ST0057      -> 50/51 satisfied, 2 withdrawn -- BLOCKED
    intent ac status ST0056/03   -> 16/16 satisfied, 1 withdrawn -- PASS
    intent ac gate ST0057        -> unsatisfied: AC-08.5

**The scope is all of ST0057's live rows plus all of ST0056 WP-03's.** `ac status ST0056` answers 61/132 and is **NOT** this number's denominator. `ST0056/03` is a WP-scoped STID and the verb accepts it -- the third call is the one nobody wrote down, and omitting it is how the second wrong figure was produced.

**IT IS ST0057's CLOSURE GATE, NOT THE 3.0.0 RELEASE GATE.** The release is ST0056 WP-12, whose dependency line reads _"All prior WPs"_ against **seven Not Started**. Read as release progress it says nearly done, where ST0056 stands at 61 of 132.

## The one row left: ST0057 AC-08.5 -- built, verified, still red, and UNOWNED

**cc built `Facade::set` at `7926cfae` and deliberately did not green it. vc verified at `d38ecbe0`: the row stays red.** The four field-setter gaps are closed and limb 2 is now an **invariant of the verb** -- `splice_one_field` re-serialises and refuses if any key but the addressed one moved.

**WHAT ACTUALLY BLOCKS IT, from the instrument's own printed output rather than from this file:**

- **Limb 1 -- four entity forms have no write path through any door:** `intent:///issues`, `.../wp`, `.../ac`, `intent:///nodes/ic`. (The sweep prints **5**; it drives `put` only, and `Wp` gained a door through `set`. Confirmed by three independent routes.)
- **Limb 2 -- `put`'s thread door still clears 8 of 8** unasked fields on a minimal legal body. **This is a DESIGN call, not a build:** `put` already grafts four children while replacing nine scalars, so it is already a hybrid and nobody has asked which it is meant to be. **Exposure is zero** -- no CLI `put`, 16 call sites, all tests -- which is what makes refusing a partial body viable.
- **The sweep's door set** -- ic's file, ruled to be over the **UNION** of doors. Corrects the worklist 5 -> 4 and **greens nothing.**
- **The biconditional cover** -- hv's, unbuilt. ic must not build the instrument deciding whether their own gate row is green.

**NOBODY HOLDS ANY OF IT.** All three peer boards read clean. **A clean board is a statement about ASSIGNMENT, not about completion, and when every lane is clean at once the remaining work has no owner -- which looks identical to having no existence.**

## The three burning cases: RE-DRIVEN 2026-08-24, and TWO ARE DEAD

The previous revision of this file said three survived and told the next node to re-drive them. Driven:

1. **`ST0011.completed` -- DEAD, both halves.** The value is `2025-06-03`, repaired at `608e9721` (2026-08-20). And it is settable: Thread's unsettable set is `schema`/`id`/`status` only.
2. **An attachment's canon record -- DEAD for text attachments.** `put`'s attachment arm (`facade.rs:4251`) builds the row from the content and **replaces exactly that one attachment's canon row**, which is narrower than a thread. **Remainder: bytes-carried attachments are refused BY NAME, and there is no CLI verb either way.**
3. **No CLI verb creates an AC or an AT -- STANDS.** The only create arms in the CLI are `st new`, `wp new` and `issues add` -- established by finding which command OWNS each arm, because counting names is the exact trap this row's own history records.

**That is six absence claims on this row refuted the moment somebody checked.** The class is not a wrong measurement: **it is reasoning from an absence nobody looked for.** Re-drive before building against any of them.

## The two threads, driven 2026-08-24

**ST0056 -- the v3.0.0 rewrite.** 133 criteria / 138 tests, **61 of 132 satisfied**, 1 withdrawn. WPs 01/02/03 Done; 04/05/06/07/10/11 WIP; 08/09/12/13/14/15/16 Not Started.

**ST0057 -- disk as a sparse projection.** 53 criteria / 53 tests, **50 of 51 satisfied**, 2 withdrawn. WPs 02/03/04/06/07/09/10 Done; 01/05/08 WIP. **Sparseness applies to VIEWS; canon is NEVER sparse.**

**ST0058 has ZERO acceptance criteria** -- `ac status ST0058` refuses with _empty contract_. Define them or declare `acceptance: exempt`.

## Next, per node

1. **Nobody, and that is the finding.** The four AC-08.5 items above are unassigned. Two are builds, one is ic's file, one is hv's.
2. **cc** -- ST0056 AC-10.4 over `migrate::plan`'s write set with a **non-empty control**; AT-10.2's second citation onto `intent-cli/tests/ingest_command.rs`; AT-10.12 held on the unexplained trim asymmetry.
3. **ic** -- the `issues`-surface paper for hv: `--kind` vs `--status`, four words for one concept, **and the absence behind it -- v3 `issues` has no body setter at all.** One paper, not two.
4. **dc** -- holds none of the gate. AT-11.6's deliverable is theirs and unbuilt. **Not closed by `0075`:** 9 of 12 rostered guards do not run under `set -e`, and six arms still assert a repository finding on any non-zero exit.
5. **vc** -- `ST0057/WP-01` and `WP-05` are recorded **WIP while their gates PASS**; that is WP-close verification. Then `declared_but_unwired` adequacy and the marker's per-crate staleness.
6. **hv's standing question:** **199** files under a thread are not carried by the store -- _"not all of that should be in the db, but certainly some of it should."_ It was ~250; the number moves, so drive it.

## Sitting with hv

- **AC-02.6's SECOND JOB is uncovered.** `intent/st/ST0057/design.md:270` assigns the GET/PUT round trip to AC-02.6, which lives in **ST0056** and is green on its FIRST job only. **It unsatisfies nothing, and that is the defect.** Minting coverage changes a denominator.
- **What re-reads a criterion's instruments when a facade grows an arm?** Nothing does. The suite ran green over 808 tests while an instrument silently understated the criterion it serves. **Re-running proves nothing when an instrument and its expectation drift together, in step, away from the thing they are about.**
- **`intent#0076`** -- the Elixir critic's comment-blind proxies; the fix belongs in the RUNNER.
- **`intent#0073`** -- six swift rule dispositions. Shipped surface, so it needs hv before it needs an editor.
- **`intent#0074`** -- whether the elixir pack should cover `.heex` at all. That is WORK, not a fix; the misleading MESSAGE is a defect regardless.
- **`intent#0071`** -- needs a CHANGELOG entry and a v2 heading that does not exist. Release policy.
- **dc's routing question 2** -- the frozen-`$INTENT_HOME` mechanism. Detector half CLOSED; **routing NOT discharged, and a reason expiring is not a routing being discharged.**

## Open issues

**Twelve:** `0063` `0064` `0065` `0066` `0067` `0068` `0069` `0071` `0072` `0073` `0074` `0076`. **Driven, not transcribed:** `intent issues list`.
