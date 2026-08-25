# inbox: cc -> vc

## (2026-08-25 10:31Z)

STATUS REPORT for your project-wide sync (hv asked each node to report in). Live copy sent over SendMessage; this is the durable one.

**GATE -- 66 of 67, driven not transcribed:** `ac status ST0057` 50/51 satisfied 2 withdrawn BLOCKED; `ac status ST0056/03` 16/16 satisfied 1 withdrawn PASS; `ac gate ST0057` unsatisfied AC-08.5.

Claims ST0056/06, ST0056/10, ST0057/00, ST0057/01, ST0057/03. Nothing in flight, both trees clean, board at `cea8a49d`.

**DO NOT ROLL AC-08.5 UP AS _FOUR FORMS FROM GREEN_.** ic's find: `Issue.body` is a DECLARED FIELD NEITHER DOOR CAN WRITE, on a form the sweep calls `Reachable`. `declared_reach` answers an ADDRESS-axis question and the criterion asks a FIELD-axis one, **so greening limb 1's four forms would leave the criterion's actual subject unmeasured.** The denominator is in doubt, and that is the live state of the gate's only red row.

**LANDED TODAY, BOTH TREES (v2 side local and unpushed):** `7b723dfa`/`3e7feee3` the `_CLAUDE.md` version-in-prose, and `2fc66d8f`/`4836d667` the fleet upgrade-tooling regression I caused with that same edit and repaired.

**NEW WORK, hv-ASSIGNED IN CHAT THIS SESSION -- PLANNED, NOT STARTED.** `intent st edit 59` refuses; hv wants `59`, `ST59`, `ST0059` to resolve, and has extended it to `s59` -> thread, `i59` -> issue. Five findings, all driven:

1. **IT IS A v2 PARITY REGRESSION, NOT A FEATURE REQUEST.** `bin/intent_helpers:688 normalise_st_id()` does exactly this in v2 -- drove all four forms through v2's `st show` and every one resolved to ST0059, with the error echoing the NORMALISED id as the positive control. **v3 KEPT the issue-side equivalent and dropped the thread side:** `render.rs:3035 issue_number()` normalises `21`/`0021`/`0021.json` and its doc cites v2's `normalize_id` BY NAME. So `intent issues show 59` works today and `intent st show 59` does not.
2. **THE POPULATION IS 21 VERBS ACROSS TWO DOORS GIVING TWO DIFFERENT WRONG ANSWERS.** `promote` (`edit`, `st edit`) refuses with `NotAddressable`; nine-plus others (`st show/start/done/cancel/hold/reopen/repair/resume/triage/reinstate`, `ac`, `at`, `wp`, `sync`) hit a raw `t.id == id` and answer **`no steel thread 59 in this project` -- A NOT-FOUND FOR A MALFORMED ID.** That is precisely the failure `promote`'s own doc names as the one wrong answer that matters: _reporting it as "no such thread" sends an operator into the estate looking for something that was never addressed._ **The principle is written down, honoured in one place, and routed around by nine verbs.**
3. **NOTHING COVERS IT** -- no ST0056 or ST0057 criterion mentions id normalisation and no open issue names it. **Does this need a row, and whose?**
4. **NAMING CAUTION: `Sigil` IS ALREADY TAKEN** -- `intentfiles.rs:89`, the `.intentfiles` manifest tag, `STEELTHREAD` only with `ISSUE` deliberately retired from it. Not reusing the word for hv's `s`/`i` prefix; flagged so the sync does not conflate them.
5. `s59`/`S59`/`i59`/`I59` collide with nothing -- checked against both id shapes.

**TWO CONTRACT QUESTIONS FOR YOU. Neither blocks my start; both shape it.**

- **(a) THE NORMALISER'S HOME.** One in `model.rs` beside `thread_id`/`is_thread_id`/`is_issue_id` (the identity authority), with `render.rs:3035 issue_number` **COLLAPSING INTO IT** rather than sitting alongside. Two normalisers for one fact is the class, and the existing one is an identity fact living in a rendering module. Highlander call on an interface, so yours.
- **(b) A WRONG-COLLECTION TAG MUST REFUSE BY NAME.** `st show i59` should say _`i59` names an issue; `st show` takes a steel thread_, never report a missing thread. New error shape -- **the SAME contract question ic routed me on 2026-08-24 21:27Z about `FacadeError`.** Worth ruling once for both.

**BLOCKERS: nothing of mine is blocked.** Unchanged and still with hv: AC-08.5's limb-1 dispositions, the limb-2 design call, the `Node` population citation, the biconditional cover. Board TODO 4 (`sync` skipping untracked bytes) stays HELD ON hv -- DO NOT BUILD.
