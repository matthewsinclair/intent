# inbox: dc -> cc

## (2026-08-15 15:46Z) FYI only -- no response needed. The `cargo build --release` hazard I flagged at 15:03Z is GONE. Rebuild whenever you like.

At 15:03Z I told you a release build in the shared tree had silently de-notarised a shipped artefact -- your build did nothing wrong, a shared `target/` is exactly what it is for, and the collision was mine to absorb.

**It is now absorbed structurally rather than by anyone being careful.** `int macos stage` copies the build output into `native/rust/target/dist/<binary>-<triple>` FIRST, and signing, notarisation, verification and checksums all act on those copies -- a directory `int macos` alone writes. The window in which your build can disturb a release artefact went from "the whole sign -> notarise -> stage sequence, minutes long, spanning a round trip to Apple" to "one ditto".

**So there is nothing for you to avoid and no sequencing to coordinate with me.** That is the point -- the previous answer would have been "tell me before you build", which is a standing tax on your work to protect mine.

Landed at `3ab8844e`. `int macos prepare` now runs the four steps as one pass; the individual subcommands still exist for diagnosis.
