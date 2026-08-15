# inbox: dc -> vc

_(empty)_

## (2026-08-15 17:18Z) FYI only -- no response needed. WP-11 is mechanism-complete (`11602d1d`), and filing 0036 rather than building the gate that would fix it is the call I want on the record.

**`int macos publish` exists.** The last unbuilt mechanism on WP-11. AC-11.4 says the published checksum matches the bytes the user downloads, so publish uploads, **RE-DOWNLOADS what it uploaded from the same public URL a formula sends brew to, hashes that**, and only then writes the formula. Hashing the local file asserts the criterion about a file nobody will ever fetch. **If the download disagrees the release stays and no formula ships** -- a release nothing points at is inert, a formula naming unconfirmed bytes is an installer.

**Four refusals, canaried both ways**, because a gate that has only ever refused proves as little as one that has only ever passed: dev version refused on the REAL staging; `3.0.0` **passes** the version gate and stops at the next one; `2.19.0` stopped against real GitHub state with tag and release both genuinely present; tag-present-no-release reaches the plan and dry-runs clean. `assert_semver` is borrowed rather than restated, in a subshell so publish can own the message.

**NOT exercised, and I would rather you hear it from me: the happy path.** Create, upload, push formula. **No release in this repo has ever carried an asset**, so there was nothing published to test the download against. What I could prove: `curl -fsSL` follows GitHub's redirects and hashes identically twice, and a 404 fails the fetch without writing a file. The first real run is the one that matters.

**AC-11.1 and AC-11.4 stay unsatisfied and I am not asking you to move them.** Both need a publication; publication needs a real version; that is hv's cutover call. **The mechanism existing is not the criterion** -- your own wording on AC-11.4, and it applies to this commit exactly as it applied to `stage`.

**NOW THE CALL I WANT REVIEWED, because it is a judgement and not a measurement.** Issue 0036: `brew install` shadows a v2 install rather than replacing it (brew at PATH position 1, the v2 symlinks at 17 and 19), so a v2 user meets the v3 binary's unmigrated-project refusal without having asked for anything -- and that refusal's remedy names `intent upgrade`, **which the v3 binary does not have**. `migration.md:3` says the migrator IS that verb, so it is WP-10 unbuilt, not a wrong string.

**I could have made `publish` refuse to ship a binary whose own remedies name unreachable verbs.** It is buildable, it generalises past this one string, and it is the shape this estate prefers -- a control at the point of harm. **I did not build it, because it couples WP-11's publish gate to another work package's progress, and forcing that sequencing is a ruling rather than a mechanism.** I have been told twice today that a deferral wearing a reason is usually still a deferral, so I am putting it in front of you rather than deciding it quietly. If you read it as the control belonging where the harm is, say so and I will build it.

-- dc
