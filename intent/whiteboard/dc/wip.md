---
node: dc
name: DevX Claude
role: worker
session_id: 482cf2fc-6b49-4a0d-8d76-38b3c981924c
heartbeat_at: 2026-08-15 16:33Z
status: paused
focus: "PAUSED after a localfold. WP-11's macOS leg is COMPLETE and proven end to end -- signing happens on STAGED copies now, so the shared-target race is removed rather than backstopped, and `int macos prepare` runs the sequence as one pass. D42 absorbed: my lane is a no-op under it, because every date reach I own is devbin and devbin is not Intent. Everything left on WP-11 is WP-12 cutover."
claims: [ST0056/11]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.** No clauses. hv ruled it four times and it was reinterpreted after three of them, twice by me inside ten minutes.

- **You never ask what time it is.** Not the OS, not `date`, not the filesystem, **and not the database either.** Asking SQLite and then writing the answer is still writing a time you obtained.
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE. Read-then-write leaves a gap that two writers interleave in, so two records get stamped in the opposite order to the one they were written in.
- **hv's structural close, and it is the form to build against: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times, and every one returned was set by SQLite on a record. **Direction is not symmetric -- IN is forbidden, OUT is fine**: a returned time is evidence a record was written; an accepted time is a second clock with extra steps.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES, WHICH IS WHY THIS VERSION WILL HOLD.** Every earlier statement of D42 asked where a value came from and whether its source was legitimate -- judgement calls this estate failed three times in one day from three nodes. **A time-typed input parameter is a defect by inspection.** Asking where a caller got a value is a discussion; asking whether a signature accepts one is a grep. **A signature that accepts a time is a standing invitation that gets accepted eventually**, no matter how careful today's author is.
- **The defect is always one step earlier.** Reaching for a clock means you are about to write a time into something that is not a durable record. The fix is never a better clock; it is not writing the time.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it's just a label". The third fooled cc, vc and me independently -- **better provenance is not the absence of a confection.**
- **SCOPE, hv: devbin is NOT Intent** -- external, vendored, no db, does what it likes with time. My whole D42 directive resolved to a no-op, and reporting that beat inventing work to look responsive.
- **A board stamp is a label, not data, and nothing may read it as one.** The ordering that exists and cannot be fabricated is the **commit**. The `## (...)` heading is legacy, kept only because nodes parse the shipped format live.

## The truth model -- canon, held not restated

Ratified in `design.md` (D01 as reversed) and `data-model.md`, and deliberately not duplicated here, because a second home is how two copies drift. The three points that change what I DO: **the db is the durable SSOT and files are re-creatable secondary artefacts**; **the typed API is the only door in**, so conformance is by construction rather than by anyone checking; **migrations are normal** -- anything justified by "we can never migrate" rests on a constraint nobody made.

## DOING

- **Nothing in flight.** WP-11's macOS leg is complete; everything left is WP-12 cutover. Folded at hv's instruction.

## TODO

0. **WP-11 (Distribution) -- MINE, WIP. WHAT REMAINS IS WP-12 CUTOVER AND NOT MINE TO FORCE.**
   - **BUILT AND PROVEN: `int macos <doctor|stage|sign|notarize|verify|checksum|prepare|formula|env|store-creds>`.** Signing acts on STAGED copies in `target/dist`; `prepare` runs stage/sign/notarize/checksum as one pass. Canaried both directions at every step. The tap `matthewsinclair/homebrew-intent` is live and **deliberately carries no formula** until there is a release to point at.
   - **REMAINS:** (a) **a real version** -- the binary reports `3.0.0-dev`, and **the publish step must REFUSE to emit a formula for a dev build**; (b) **the publish step** itself.
   - **AC-11.4 stays UNSATISFIED and that is correct.** The criterion is the published hash matching the published bytes; nothing is published. **A better-built mechanism is no more a satisfied AC than a built one was** -- the trap to avoid on the bounce.
   - **HELD until WP-12: `int build release` gaining `Cargo.toml` to its sidecar sync.** Right for a v3 release, wrong today -- the Rust workspace is versioned independently at `3.0.0-dev`, so wiring it now makes a v2 release stamp its own version into `Cargo.toml`.
   - **`brew services` for intentd is BLOCKED on WP-08.** `intentd --help` says "not yet implemented" -- nothing to describe. Conflab's formula carries the `service do` block to port.
1. **Issues 0030 / 0031 (`intent upgrade`) -- filed, DEFERRED under DEFAULT-DEFER, not forgotten.** 0030: backup dirs stamped in LOCAL time, so oldest-first retention deletes the newer artefact across a DST fall-back -- **latent only because nothing sweeps `.backup/` root.** 0031: `--backup-dir` basenames straight into `.backup/`.
2. **Release mechanics sequenced behind WP-10.** Not front-running it.
3. Open for others: **`intent/.cache/` contradicts the model** (cc, D21); **`core.hooksPath`** (deferred -- it collides with `intent claude upgrade`, a v2 `bin/**` change); the **`bin/` boundary** (hv).

## Watch-outs

Facts about this estate, not reminders. Everything amounting to "remember to" is worthless here -- three nodes broke rules they had personally written, on the day they wrote them.

- **THE ONLY macOS CHECK THAT MEANS ANYTHING IS A QUARANTINED COPY.** `stapler` reports no ticket on a bare Mach-O -- correct, there is nowhere to put one. `spctl -a -t exec` says "does not seem to be an app" -- correct, that policy is for bundles. And **`codesign --verify --strict` RETURNS 0 ON AN AD-HOC BINARY WE DID NOT SIGN**, because an ad-hoc signature is a valid signature. Use `spctl -a -t open --context context:primary-signature` on a copy carrying `com.apple.quarantine`.
- **`target/release/` IS SHARED MUTABLE STATE -- WHICH IS WHY NOTHING SIGNS THERE ANY MORE.** A peer's `cargo build --release` silently replaced a Developer ID signature with the linker's ad-hoc one and de-notarised a shipped binary inside the hour, with no signal and every artefact of the proof still reading as valid. Fixed structurally: `stage` copies to `target/dist` first. **Never sign anything in a directory someone else writes.**
- **SIGNING MUTATES THE BINARY; NOTARISATION DOES NOT. SIGN BEFORE YOU CHECKSUM.** A sha256 taken one step early **does not fail for us -- it fails for every `brew install`**, where we have the least visibility.
- **A CANARY PROVES NOTHING UNTIL YOU CONFIRM THE FIXTURE REACHED THE BRANCH.** I planted a stale sums file on already-notarised artefacts, so the command correctly PASSED and my check reported "BUG" about a branch that never ran. **A red-looking result from a green run reads exactly like a real defect** -- the direction nobody watches, because it feels like diligence working. Canary both ways.
- **`--date=format:` IGNORES `TZ`; `--date=format-local:` RESPECTS IT.** `TZ=UTC git log --date=format:` prints LOCAL time with a `Z` appended -- wrong by exactly the offset and looking perfect. `git log` is local by default and is the usual source of that error.
- **MY BOARD IS A MEMO; THE AC IS THE CONTRACT.** AC-11.4 already ordered the staged-copy restructure, conditional on hv ruling the matrix -- which hv did. I re-read my board instead and rebuilt an agreed case from scratch. **A deferral recorded in two places has its precondition met in only one, and the copy you re-read on a bounce is the wrong copy.**
- **A GUARD MUST BLOCK ON WHAT THE COMMIT ADDS, NOT ON THE WORKING TREE.** `provenance_check.sh` globbed the tree, so one node's untracked mid-generation file froze every node's commits on paths they had never touched. **A guard that must be bypassed is a guard nobody keeps.** Hold the commit and diagnose; never reach for `--no-verify`.
- **`--only` COMMITS WHAT YOU NAME, AND `--amend` IGNORES IT ENTIRELY.** A move is two facts; naming the new path leaves the deletion staged. And `--amend` with no pathspec re-commits the WHOLE INDEX -- ic's swept 19 files including peer boards. **A sweep does not move a file, it SPLITS A CHANGE**: a method and its test are one unit, each half reads as finished alone, and HEAD stopped building while every worktree stayed green. **Verify at HEAD with `git ls-tree`, and read `git status --short`, not the diff.**
- **NEVER mutate `bin/**` or `tests/**` in place.** `~/.local/bin/intent` AND `~/bin/intent` both symlink to `bin/intent`; several sessions are live. Sacrificial worktree only. **`bin/.devbin/**` and `native/**` are safe** -- and devbin is vendored external code, not Intent's.
- **NEVER `git pull --rebase` IN THIS SHARED TREE.** Push; if rejected as non-fast-forward, coordinate. A peer's `.git/index.lock` means a peer is running git: WAIT, never remove it.
- **THIS REPO IS A v2 PROJECT AND THE v3 BINARY REFUSES HERE BY DESIGN.** Every measurement taken with the v3 binary inside this tree measures the refusal path. `int dogfood` exists so there is a v3 project to measure against.
- **A control refuses; documentation reminds; only one is load-bearing.** Anything I can obey only by concentrating is an unfixed defect, not a discipline.
- **The repository is PUBLIC**, and there are two remotes, `local` and `upstream`. Push both; never enumerate them through `head`.

## Decisions

Standing only. The day's full set is in `.history/20260815/wip.md`.

- (2026-08-15) **A STRUCTURE THAT CANNOT FAIL BEATS A CONTROL THAT CATCHES THE FAILURE, AND "RECORDED AS THE BETTER SHAPE, DELIBERATELY UNBUILT" IS USUALLY DEFERRAL WEARING A REASON.** I shipped the refusal and left the design written down. A refusal detects the race only at the END, after a notarisation round trip has been spent on bytes that no longer exist. Building it took under an hour.
- (2026-08-15) **THE DEFECT IS OFTEN THE GAP BETWEEN CORRECT STEPS, NOT ANY STEP.** Four individually-correct macOS subcommands, run by hand with a multi-minute wait in the middle. A note saying "run these as one sequence" is a reminder; `int macos prepare` is the sequence.
- (2026-08-15) **REFUSING TO SETTLE BY INFERENCE IS NOT A RESTING STATE -- IT OBLIGES YOU TO GO AND GET THE ANSWER** (vc's). An open question parked across three rulings is a decision made by default. **Three independent stops is one alarm, not three data points.**
- (2026-08-15) **MEASURE BEFORE YOU DESIGN, AND LOOK FOR THE ESTATE'S EXISTING ANSWER FIRST.** A well-formed-feeling question can rest on an unruled premise and name a tool that cannot do the job. Conflab had shipped the answer for four months.
- (2026-08-15) **A SUFFICIENT-LOOKING CHECK THAT ANSWERS A NARROWER QUESTION, AND FAILS GREEN.** Ask what a check actually proves, not what it is conventionally used for.
- (2026-08-15) **A WRONG ARTEFACT IS NOT A NEUTRAL PLACEHOLDER -- IT MAKES A CONFIDENT FALSE STATEMENT.** A formula pointing at a nonexistent release reads as "the tap is broken" rather than "the release is not out yet". **An empty tap says the true thing.**
- (2026-08-15) **AN AC NAMES THE OUTCOME; THE MECHANISM BELONGS IN THE WORK PACKAGE** (vc's). A criterion naming a tool can be invalidated by a measurement of that tool while the thing the project wanted is still achievable.
- (2026-08-15) **THE CONTROL GOES WHERE THE HARM IS.** A dev-version formula printed to a terminal harms nobody; the same bytes in a tap install a build nobody meant to ship.
- (2026-08-15) **A PEER CANNOT AUTHORISE WHAT A HARNESS REFUSED**, and a peer performing it on your behalf launders the refusal.
