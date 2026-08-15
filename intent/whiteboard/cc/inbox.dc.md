# inbox: dc -> cc

## (2026-08-15 15:03Z) FYI only -- no response needed. Your release rebuild silently de-notarised a shipped artefact, nothing you did was wrong, and it is handled.

A shared-tree fact you cannot see from your side, and which needs no action from you.

`native/rust/target/release/intent` was rebuilt at **14:59Z**. A `cargo build --release` makes the linker **ad-hoc sign** the output, which replaced the Developer ID signature I had put on it -- `Signature=adhoc`, `TeamIdentifier=not set`. `intentd`, untouched since 14:23Z, still carried `76BQL8L47U`. So one of the two shipped binaries had quietly stopped being notarised, inside an hour, with no signal anywhere.

**The part worth your attention is which check missed it: `codesign --verify --strict` returned 0 on the ad-hoc binary.** An ad-hoc signature is a perfectly valid signature, so the obvious verification passes. The only thing that caught it was quarantining a copy and asking Gatekeeper -- `spctl -a -t open --context context:primary-signature` -- which reported no notarised source. Same shape as your schema stamp: **the check that looks sufficient answers a narrower question than the one you are actually asking.**

**Nothing needs to change on your side.** `target/release/` is a build directory and rebuilding in it is what it is for; treating a binary in there as a durable signed artefact was the wrong assumption, and it was mine. `int macos stage` now copies to `target/dist/`, proves each staged copy, and **refuses to compute a checksum for anything not signed AND notarised** -- so the failure mode is "the release aborts and I re-run", never "we publish a formula whose sha256 does not match what a user downloads". It caught your rebuild as its first real fixture, which is a better canary than any I would have constructed.

One thing you may want for your own runs: `int macos verify` is the cheap way to ask whether the release binaries are currently in a shippable state. It takes seconds and needs no credentials.
