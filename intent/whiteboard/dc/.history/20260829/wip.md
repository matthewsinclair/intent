# dc archive -- 2026-08-29

**The ST0066 build, verbatim. The session crossed midnight, so the assignment and the first rulings are stamped 2026-08-28 and the build that followed is stamped here.**

## ST0066 -- `intent fc`, the fiat close. Assigned by vc, taken at `d692730e`

### What was ruled, and by whom

- hv ruled the POSTURE 2026-08-28 14:38Z and the PACKAGE 14:54Z, recorded at `b17048d3`. The body of `intent/.canon/st/ST0066.json` is the spec; vc instructed explicitly that no summary of it substitutes, and that a build resembling one of the DECLINED alternatives is a ruling to re-open rather than a detail to settle at the keyboard.
- **The premise is accepted with the ruling and the build may never contradict it:** the LLM shares hv's uid and shell, so hard prevention is not achievable on this machine. Enforcement is DETECTION + ATTRIBUTION. AC-00.6 forbids the deliverable from claiming otherwise.
- **Both of my proposed shapes carried.** Fiat sits BESIDE the status with one required composer; the extract schema string does NOT bump. AC and AT were never put on the menu because the measurement settled them -- a variant each, zero DDL.
- **`0123` ruled: the contract is the authority**, and AC-00.4 gets REWRITTEN by vc rather than built to its letter, because I showed it would pass vacuously for the WP kind.
- **STILL OPEN AND FIRST ON hv's BOUNCE: is un-fiat a `reinstatement`?** The fiat exit edge.

### What I built

`FiatRecord` + `Invoker`; `AcState::Fiat` as a sixth variant with `permitted_for` true for both kinds; the gate arithmetic (`Resolved::Fiat`, fiat inside `active`, pass when `satisfied + fiat == active`, tally `N/M satisfied, K fiat-closed`); the view render; the GraphQL projection and its flattened fields; `fiat_marker` as the composer; `Invoker.tty` classified `Immutable`/`Authored` in `transitions.rs` with both edgeless arms DRIVEN; `fiat` declared an orphan with its evidence; faces regenerated, `SCHEMA_JSON_VER` 11->12 and `SCHEMA_SDL_VER` 9->10, hashes re-pinned in the same commit as the guard demands. **1426 passing, 1 failing, and the one is the blocked ruling.**

### The finds, in the order they mattered

- **`contract::resolve` MATCHES ON KIND BEFORE STATE.** The `_ if criterion.kind == AcKind::Test` arm sits above the state arms and resolves a test-backed criterion from its ATs, ignoring the recorded state -- deliberately, because hand-authored canon can carry a `satisfied` the gate must not believe. **Any variant added below it is silently discarded for every test-backed criterion**, which is exactly the population a fiat close is for. Found by reading before writing. vc: _you found a green that would have looked right; that is the whole job._
- **THE ENTRY EDGE IS NOT SEPARABLE FROM THE EXIT, AND THIS WAS MEASURED.** vc asked me to shape `ac.fc` so the exit could be a later addition. Adding entry alone turns `fiat` from an unreachable orphan into a TRAP STATE, which `no_state_can_be_entered_and_not_left` exists to refuse. So the instruction asked for something the invariant forbids, and the honest answer was to stop and say so.
- **THE EXIT PROBABLY NEEDS NO NEW VERB.** `ac.reinstate` already exists and already means bring a requirement closed-without-being-met back into play, landing on `AcState::entry(kind)`. So hv chooses between EXTEND AN EXISTING VERB BY TWO EDGES and MAKE FIAT TERMINAL AND CHANGE THE MODELLING -- not between a whole second verb and that. **I had told vc the expensive version and corrected it before it reached hv.**
- **`0123`: AC-00.4 WOULD HAVE PASSED VACUOUSLY FOR THE WP KIND.** The thread close gate reads criteria and never `WorkPackage.status`, so "distinguishes fiat in the same line that counts it" would be satisfied because nothing counts WPs at all. The blind-instrument class arriving in an acceptance criterion instead of a test.
- **A HIGHLANDER VIOLATION IN MY OWN PATH.** `facade.rs` carried a private `state_name` duplicating `AcState::name()`, whose own doc claims that vocabulary was consolidated. I would have added the sixth arm to both. Deleted and delegated.
- **THE NEWTYPE RISK WAS REAL AND IS NOW MEASURED.** `AcState::Fiat(FiatRecord)` is the enum's first newtype variant under `tag = "is"` + `deny_unknown_fields`. Whether strictness survives through it is a question about serde, not about this code, and the failure would have been silent in both directions. `an_unknown_field_is_still_refused_inside_a_fiat_state` was written BEFORE anything was built on the type, with its accept-half beside it. It holds.

### Corrections against myself, four of them, all in-session

1. **`AcState::in_scope()` drives nothing -- it has NO CALLERS.** I told vc it keeps the denominator honest. Positive-controlled the empty grep (`.is_pass()` returns 16 under the same pattern) so the zero was the fact rather than the instrument. The value stands; the claimed consequence was wrong.
2. **Minutia 3 was the wrong layer.** I leaned a structured half-field; `0116`'s own closing sentence says it and ST0066 are two halves of one mechanism, so it is a SEQUENCING question and the field would be a second home for what `0116` will model. vc: _it replaces the question I was going to put to hv._
3. **I overstated my own escalation's cost** -- see `ac.reinstate` above. vc: _correcting your own escalation before it reaches the principal, when the error made your ask look bigger rather than smaller, is the direction nobody audits._
4. **"Exactly one failing test in the whole workspace" was FALSE when I said it.** See Family 9.

### The instrument failures, three of them, same family two different ways

- **`D42` in a `///` on a modelled type**, twenty minutes after reading the rule that says `///` is published into the JSON Schema and the SDL. Caught by `no_pm_state_in_output`.
- **`ST0066` in a shipped STRING LITERAL** in my orphan reason. Caught by that test's sibling, which reads literals rather than doc comments. **Third PM-state leak of the day, second in an hour.**
- **`rc=0` from the rules validator was `head`'s status through a pipe.** Re-measured bare: **rc=2**. My board's own top trap, and reporting it would have been a second false defect filed against cc's code on the same mechanism.

---

## AFTERNOON -- 2026-08-29 12:00Z onward: the increment LANDED

**`b7a3e771` -- 1454 passing, ZERO failing, `--no-fail-fast`.** 16 files. The AC half of ST0066 complete: `FiatRecord`/`Invoker`, `AcState::Fiat`, gate arithmetic, view render, GraphQL projection, `fiat_marker`, `Invoker.tty` classified with both arms driven, `fiat` an orphan, faces regenerated and hashes re-pinned.

### hv ruled the exit: TWO EDGES ON `ac.reinstate`, no new verb

My reading carried in my own terms -- _renders distinctly forever_ is a claim about RENDERING, not irreversibility, and the estate's own sentence settles it: **the HISTORY is the event log, not this field.** The terminal option was declined **on the cost I identified**: a terminal value in a `State` field is what `no_state_can_be_entered_and_not_left` exists to refuse, so taking it would have meant weakening a guard protecting every other machine in the estate. vc: _your correction of my framing is what made this ruleable in one pass._

### The guard defect landing it exposed

`the_transition_table_transcribes_the_ratified_machines_edge_for_edge` looked edges up by `(verb, to)` -- **unique BY ACCIDENT until `ac.reinstate` gained a second source.** It then returned the first match and compared a ratified edge against a DIFFERENT declared one: reporting a divergence that did not exist while silently not checking the edge it was looking at. Key widened to `(verb, to, from)`. **`IN-AG-RED-CONTROL-001`'s class exactly -- a property holding because another mechanism happened to make its key unique.**

### The second-witness gap, routed not taken

`data-model.md`'s Machine 3 lists EIGHT edges; my transcription declares ten. They are deliberately **second witnesses to each other** and hv's ruling is in only one. `data-model.md` is ST0056 canon and vc's surface, so I did not edit it -- **and until it catches up the two disagree and NOTHING DETECTS IT**, because the check reads only the transcription. Named in the artefact beside the new rows, because a note in the code outlives a message.

### cc committed two of my hunks by accident, and the mechanism corrects my board

`ccfefe2b` carried my `state_name` deletion and its call-site rename. **I told cc to KEEP them**: they are the Highlander fix, they deliver no behaviour, and my boundary was about an unshipped fiat close reading as delivered. Verified rather than assumed -- `git diff -- facade.rs` is empty, so cc committed exactly what I had.

**THE MECHANISM IS THE KEEPER AND IT CORRECTS MY OWN WATCH-OUTS.** My board carries `git commit --only <paths>` as the safe move. True for what it claims -- it stops a bare commit sweeping a peer's whole file. **It does NOT commit a SUBSET OF HUNKS: it takes the worktree version of the path, wholesale.** cc measured that first, then built a blob with `git update-index --cacheinfo`, verified it correct, and **the pre-commit hook refreshed the index from the working tree and replaced it** -- a `--cacheinfo` entry carries zeroed stat data, so the next porcelain command reads it as racily-clean and re-reads the file. **The verification was correct when it ran and stopped being true before the commit landed.** The technique that works is committing from a DETACHED WORKTREE, where disk IS the version you want.

### `0133` folded in by hv

**Not an ingest defect -- a REPRESENTABLE-STATE REGRESSION.** v2 could express _unsatisfied, and here is what was measured_; `AcState::Unsatisfied` is a UNIT VARIANT, so v3 cannot represent it and migration destroys it BY CONSTRUCTION. No parsing fix could ever have reached it. Scope: `Unsatisfied` gains optional evidence, and `legacy.rs:1707`'s wildcard populates it. **NOT mine: `ac new`, which is cc's.**

**Checked rather than asserted that my shape does not foreclose it:** every site is a one-line widening, and the GraphQL projection needs NOTHING because `AcStateView.evidence` already exists. **Two coverage gaps handed over rather than discovered late:** the projection test and the sample table both enumerate cases BY HAND, so an `Unsatisfied` gaining evidence passes both while never exercising the field.

### The question I raised and did not settle

**If `ac unsatisfy` keeps clearing evidence, the only way to reach `unsatisfied`-with-evidence is authored canon -- NO VERB can produce the state the model now admits.** That is orphan-shaped in exactly the sense `transitions.rs` polices, so it is not only semantic: it decides whether the fix leaves a hole the machine must declare. The case the other way is real too -- evidence justifying a revoked satisfaction leaves a claim standing behind its own retraction. Ruling, not implementation detail.

---

## AFTERNOON II -- 0133 BUILT, NOT LANDED. Branch `dc/0133-unsatisfied-note`

**The generator fix landed on main at `14879fc5`. 0133 itself is COMPLETE and sits on a branch because `facade.rs` is held by cc mid-edit.**

### The generator fix -- a hole in my OWN landed fiat work

`model_laws.rs`'s `ac_state()` feeds `criterion() -> thread() -> thread_round_trips` and had ZERO `Fiat`. So `b7a3e771` landed the sixth variant with the estate's GENERAL round-trip law never generating it. **Its doc read "Every recorded AC state, including both in-scope ones" -- true when written, my commit made it false, nothing failed.** `prop_oneof!` is a LIST, not a match.

**Driven to both verdicts rather than accepted on a green:** `thread_round_trips` passed before AND after, so its green cannot tell the two apart. A temporary probe asserting zero fiat reported **342/2000, `inherited_from` present in 177 and absent in 165** -- the `skip_serializing_if` field exercised both ways.

**The comment is not the fix,** so `every_ac_state_variant_has_a_generator_arm` is a never-called exhaustive match. **Its firing point was MEASURED:** a seventh variant produces FIVE errors first, all in `src`, and the lib failing means the test file is never compiled -- so the author fixes those five, the lib goes green, **and that is the moment they believe they are done.** Exactly what happened with `Fiat`. Verified live by deleting the canary's own arm: E0004 at `model_laws.rs`.

**Two siblings do NOT fire and are named in the doc:** `in_scope` is a `matches!` and `evidence` ends in `_ => None`.

### 0133 as hv ruled it, both halves

- **`Unsatisfied` gets its OWN payload, `note`, NOT `evidence`.** Evidence means proof a criterion was MET, on `Satisfied` and nowhere else.
- **`ac unsatisfy` KEEPS CLEARING** -- and ruling 1 is what makes ruling 2 cheap: nothing to orphan, because the note is a different field from the one the verb clears.
- **vc's measurement that moved ruling 1:** the `evidence` at `legacy.rs:1707` is a COMPOSITE of the v2 evidence AND note fields, so on an unsatisfied row the destroyed text is predominantly the NOTE. Naming it `evidence` would have been a naming lie in a PUBLISHED face.
- **My answer to the ingest question: FUSE, do not split** -- the satisfied path already fuses identically, and splitting only one path would make two ingest paths disagree with nothing detecting it. vc accepted it over their own flag.

### THE STRICTNESS ARM FOUND A SHIPPED DEFECT BEFORE THE TYPE MOVED

```
computed    (unit):    ACCEPTED -- unknown field swallowed
unsatisfied (unit):    ACCEPTED -- unknown field swallowed
satisfied/descoped/withdrawn/fiat: REFUSED (correct)
```

**`deny_unknown_fields` is a property of the variant's SHAPE, not of the enum.** The two that swallow are the two ENTRY states. **The widening closes `unsatisfied` as a side effect -- and a fix nothing names is invisible the day someone reverts it**, which is why the arm stays in the file permanently. `computed` stays open; filed separately, NOT bundled. **vc measured Intent's own exposure: 416 `AcState` objects, 333 unit variants, ZERO stray keys -- and Intent is the estate LEAST likely to carry it.** The fleet is unmeasured and a row accepted today starts REFUSING after the widening.

### The mechanical edit that compiled perfectly and destroyed prose

A context-aware bulk rename across the tests hit **three DOC COMMENTS**, and one became self-refuting: _"`AcState::Unsatisfied { note: None }` and `AcState::Unsatisfied { note: None }` are different syntax for the same value."_ **The compiler cannot see it and no test covers it.** Same family as vc's find that `b7a3e771` falsified three prose claims in three files -- arriving from a third direction: **a mechanical rename falsifies the prose that describes the OLD shape.**

### How the suite was actually measured

**Diffed against its own baseline in the same worktree.** Baseline 1434 passing / 20 failing; mine 1436 / 23. The three were the faces guard, expected. After re-pinning: **`comm` of the failing-name sets shows my change introduces ZERO new failures.** The ~20 red CLI tests are environmental and fail identically at baseline. **A raw pass/fail count would have said "23 failures" and been useless.**

### Faces, measured not assumed

`thread.schema.json` and `schema.graphql` MOVED; `issue`/`event` moved only because they carry the JSON version; **`ddl.sql` did not move at all.** JSON 12->13, SDL 10->11, **DDL left at 11 as a measured result.** Hashes re-pinned from the guard's own reported values.

### Why a worktree, and what it cost

`facade.rs` carries cc's 102 uncommitted insertions and calls a method that exists in neither the tree nor HEAD -- **cc mid-edit, not a defect, and not mine to touch.** All of 0133 was built and driven in a detached worktree. **The worktree has its OWN index**, which is what made it safe: cc staged a file into the SHARED index during this session and my worktree commits could not sweep it.

**The commit gate is ABSENT in a fresh worktree** (`.githooks/pre-commit.intent` is gitignored, so no clone receives it) -- copied it in rather than bypassing. It then **refused for the right reason**: no release binary, so every generated thread view would go UNCHECKED and exit 0 would report that as a pass.
