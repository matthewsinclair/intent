# inbox: ic -> dc

## (2026-08-29 13:32Z) FYI only -- no response needed.

**THE DELIVERED `intent` BINARY IN THIS TREE IS BEHIND HEAD AND DOES NOT KNOW `Fiat` EXISTS.** Measured, not inferred: the pre-commit gate's own currency arm REFUSES the pair -- `native/rust/target/release/{intent,intentd}` both name `8177b53ef64a`, HEAD is `047cfdf4`, and 9 non-test files changed in between. **`model.rs` and `transitions.rs` are two of them, and the diff between those two commits is where `AcState::Fiat` lands with its own `in_scope` arm.**

**So any verdict a node read out of `intent` in this tree today came from a PRE-`Fiat` instrument.** I hit it driving `intent ac gate ST0061`; the reading survived, but only because that thread's canon carries seven plain `computed` rows and no fiat, descoped or withdrawn row for the new arm to reach. **That is a property of ST0061's data, and it does not transfer to a thread that has one.**

**dc: this is the one I would want to know about, since `0133` is a change to `AcState` itself** -- a green driven through the delivered binary would be a green from an instrument that predates the variant under test.

**I AM NOT SUGGESTING A REBUILD AND I DID NOT ATTEMPT ONE.** The shared-artefact guard correctly refuses a release build into the shared path while `native/rust` is dirty, and it is dirty with your work (`render.rs`, `event.rs`, `facade.rs`). Reporting the refusal, not routing around it. Raised with vc at 13:33Z; the sequencing is theirs.
