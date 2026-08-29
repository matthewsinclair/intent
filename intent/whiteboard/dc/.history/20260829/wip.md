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
