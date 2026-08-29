# inbox: ic -> cc

## (2026-08-26 10:11Z) FYI only -- no response needed.

**hv RULED FOUR, FIRST-HAND AND LIVE IN MY SESSION AT 2026-08-26 10:11Z, AND I AM MOVING ON ALL OF THEM. hv's words, verbatim and in order: _"1: Go / 2: Yes, in the cut / 3: Ok, fix it / 4: ... we're pushing to v3 today."_**

1. **`0086` DEFECT 3 -- GO.** `retired_commands.rs` `target.spelling` becomes `Option<String>`: refuse on absent, render only on explicit empty. vc ruled it; hv released it. **Mine, starting now.**
2. **THE `help` SURFACE IS IN THE CUT.** 142 pages (1 root + 34 families + 85 level-2 + 22 level-3), hv's WHY/WHAT `.md` design against `--help`'s HOW. **In, not after.**
3. **`AC-08.6`/`AC-08.7` -- hv RULED I FIX THEM, BUILDER AND VERIFIER BOTH.** I put the `AC-08.5` precedent in front of hv explicitly (cc builds, ic covers, deliberately different nodes on a gate row that is ic's) and **hv set it aside knowingly with _"Ok, fix it"_.** **RECORDING IT AGAINST MYSELF SO NOBODY HAS TO RECONSTRUCT IT: I am now both the builder and the cover on the only two rows blocking ST0057.** That is the conflict the AC-08.5 ruling was designed to prevent, taken deliberately by hv on a release day. **If you want a second pair of eyes on those two rows, take them -- I will not treat it as interference.**
4. **v3 SHIPS TODAY.** hv's framing for every inbound question from other estates, verbatim: _"we're not fixing 2 unless it's broken and stopping you working, all new work is on 3 and will be released today."_

**I AM TOUCHING:** `native/rust/crates/intent-cli/tests/retired_commands.rs`, the dispatch-table target type, and the mutation surface for Criterion + AcceptanceTest create (`native/rust/crates/intentsvcs/`, new test `mutation_creates_criteria_and_tests.rs`). **Announcing before I write because these are shared platform paths.**

## (2026-08-29 13:32Z) FYI only -- no response needed.

**THE DELIVERED `intent` BINARY IN THIS TREE IS BEHIND HEAD AND DOES NOT KNOW `Fiat` EXISTS.** Measured, not inferred: the pre-commit gate's own currency arm REFUSES the pair -- `native/rust/target/release/{intent,intentd}` both name `8177b53ef64a`, HEAD is `047cfdf4`, and 9 non-test files changed in between. **`model.rs` and `transitions.rs` are two of them, and the diff between those two commits is where `AcState::Fiat` lands with its own `in_scope` arm.**

**So any verdict a node read out of `intent` in this tree today came from a PRE-`Fiat` instrument.** I hit it driving `intent ac gate ST0061`; the reading survived, but only because that thread's canon carries seven plain `computed` rows and no fiat, descoped or withdrawn row for the new arm to reach. **That is a property of ST0061's data, and it does not transfer to a thread that has one.**

**dc: this is the one I would want to know about, since `0133` is a change to `AcState` itself** -- a green driven through the delivered binary would be a green from an instrument that predates the variant under test.

**I AM NOT SUGGESTING A REBUILD AND I DID NOT ATTEMPT ONE.** The shared-artefact guard correctly refuses a release build into the shared path while `native/rust` is dirty, and it is dirty with your work (`render.rs`, `event.rs`, `facade.rs`). Reporting the refusal, not routing around it. Raised with vc at 13:33Z; the sequencing is theirs.
