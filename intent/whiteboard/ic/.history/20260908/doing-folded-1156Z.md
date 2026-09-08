# ic -- executed work folded 2026-09-08 11:56Z

Narrative half of the localfold. Verbatim pre-fold board beside this as `wip-prefold-1156Z.md` (134,463 bytes, prettier-clean at source, `cmp`-verified before the first edit, sha256 opens `2bb3167bb3b96cca`). **Status stayed `active` throughout** -- a fold is not a session ending (`/in-whiteboard` invariant 6).

## The menubar reaches intentd's web face (ST0064)

hv asked inline for a menu item titled with intentd's local URL that opens it, then refined it from a screenshot. Delivered in two cuts.

**Cut one, `071113a0`.** The address goes in `daemon status --format json` rather than being read from `intentd.addr` by the app. The deciding argument was not the app's architecture but the verb's own help, which already promised _the address it answers on_ while a daemon answers on two -- the unix socket every client routes through, and the loopback port. So this closed a gap between promise and behaviour rather than adding a field for a consumer. `answering_loopback_under` + `loopback_base_url` went into `intentsvcs::daemon` because `browse --browser` had the same search inline and this would have been the second copy.

**Cut two, `ac1e92ba`.** hv saw the shipped build and the menu said the same thing twice: a greyed `intentd is answering` caption above an item whose whole title was an address. The status line became the affordance -- reads `intentd is active`, opens the page, no URL text anywhere. `answering` was a borrowed word: it is the CLI's term for a routing question (does a round trip complete) that rode into a menu telling an operator whether their daemon is up.

**The token trap, killed and then pinned.** cc flagged that the HTTP face requires a bearer token. Measured: an unauthenticated `GET /` answers 200 with the real status page; the secret gates `/op` and entity views. A test now drives a raw GET and asserts 200, so the day `/` starts demanding the token is the day a test fails rather than an operator meets a login wall.

## The 3.0.1 scope contradiction, closed as a class

hv's third statement that everything is in 3.0.1 was met by a sweep rather than another apology. One sentence at `hv/wip.md:73` was the only live assertion; `restart.md:11` had already diagnosed it in prose and left it standing. vc struck it (`fc77dd090`), then the corrections themselves turned out to carry the error -- five homes, then six, the last in ST0056 canon (`AT-09.2`'s note) and a seventh in shipped Rust (`wire.rs`). vc closed all of it at `b7383453` and `2019f0e7`.

## What the day cost, in three lessons

**A correction that quotes its error keeps the error alive.** Four copies of the correction lost to one contradicting sentence, and then the copies themselves became instances, because a reader arriving by grep gets the string and not the frame around it. My own board did it; so did two of vc's corrections, an hour after agreeing the rule.

**A class sweep is bounded by the paths it was pointed at, and that bound is invisible in its own output.** A clean result reads identically whether the corpus was complete or not. vc declared closed off boards and boot docs; my sweep found a sixth home only because it included `intent/st/` by habit. The honest form, now owed by both: name the searched paths, or the close is a claim nobody can check.

**Commit, then build.** An install from a dirty tree stamps a commit that does not contain its source. cc's framing is sharper than mine was: blast radius is recoverable by a rebuild, a false self-description is not, because anything that recorded the marker keeps the claim after the artefact is replaced.
