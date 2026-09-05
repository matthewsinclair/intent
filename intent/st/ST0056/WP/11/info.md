---
wp_id: WP-11
title: Distribution: Homebrew tap, signing, notarisation
scope: M
status: WIP
---

# WP-11: Distribution: Homebrew tap, signing, notarisation

## Objective

Make `brew install intent` the install story: cargo-dist release artefacts, the Homebrew tap, and the macOS signing/notarisation posture -- retiring the clone-and-symlink v2 install model.

**TITLE CHANGED 2026-09-05 (dc), and the old one is recorded here so a reader who remembers it can find out why.** It was `Distribution: cargo-dist, Homebrew, signing` until hv's D40 deferral (2026-08-15) was reconciled into this list. **The body carried the correction and the TITLE did not** -- and the title is what `intent wp list ST0056` renders, so the index went on advertising the deferred approach while the detail underneath said otherwise. Same fact, three homes, one corrected, and the stale one had the widest readership. Caught by vc.

**WHY THE STALE LINE DID NOT LOOK STALE, WHICH IS THE PART WORTH KEEPING:** the three sibling bullets below all carried strike-throughs with reasons, so the convention was VISIBLY IN USE. That made the one unstruck line read as a deliberate exception rather than as an omission. **A convention followed by its neighbours makes the line that breaks it look intentional** -- which is why reading `design.md` before building was the only thing that stopped an afternoon spent wiring a tool hv had ruled out three weeks earlier. (vc's sharpening, adopted.)

## Deliverables

- ~~cargo-dist wiring~~ **DEFERRED 2026-08-15 by hv's D40, and this line went stale against it.** D40 put the tap at `matthewsinclair/homebrew-intent` with artefacts on the source repo's own releases, and records cargo-dist as deferred **as a consequence rather than as a judgement on the tool**: macOS-only leaves no matrix for it to manage, and it cannot notarise regardless -- measured, zero hits for `notarytool`/`stapler`/`xcrun`. What ships instead is `int macos formula`, which generates the formula from the STAGED artefacts and reads the version from the staged binary itself, the only source that cannot disagree with what a user actually runs. **Recorded here 2026-09-05 (dc) because the deliverable list still read as if cargo-dist were the plan, three weeks after hv ruled otherwise -- the same status-versus-reality gap found on WP-12 the same day.**
- macOS signing/notarisation decision + implementation (adhoc vs Developer ID; the Conflab TN3171 lesson recorded for any TLS-bearing future)
- ~~`brew services` story for intentd~~ **DELIVERED 2026-09-05 (dc): the formula now emits a `service do` block.** It was held out by a comment in the generator giving its own precondition -- *there is no start verb and no log path, so any block here would be a guess at an interface that does not exist* -- and **both halves were driven false on the delivered pair**: `intent daemon start|stop|status|run` all answer, `start` is idempotent and reports its pid, and the log path exists and is being written. The block runs `intentd` DIRECTLY rather than `intent daemon run`, on the daemon's own words in `main.rs` (*serving is what this binary does with NO arguments*, and *`intent daemon run` execs this binary*) -- going through the CLI would give launchd a process to supervise in front of the one that matters. The generated formula is ruby `Syntax OK`; **that checks Ruby syntax and NOT brew DSL semantics, which needs a real `brew install` and is outward.**
- ~~INTENT_HOME retired to a documented dev override~~ **STRUCK 2026-08-15 (dc measured, vc reworded): there is nothing to retire.** v3 has ZERO `env::var("INTENT_HOME")` call sites -- the only runtime environment read in the whole binary is `COLUMNS` -- so AC-11.3 is satisfiable by construction rather than by a retirement. The "documented dev override" this named is **rust-embed's read-templates-from-disk mode, which is WP-07's, not distribution's**. Note for whoever evidences AC-11.3: `strings <binary> | grep INTENT_HOME` returns 3 hits and is **100% false-positive** -- they come from `surface/dispatch-table.json`, compiled in via `include_str!`, as parity prose describing v2. Presence in the binary is not a read
- ~~install / upgrade docs~~ **WRITTEN 2026-08-16 (dc): `intent/st/ST0056/install.md`.** Marked at the top as describing a path that does not work yet -- the tap deliberately carries no formula. Leads with the fact that `brew install` SHADOWS a v2 install rather than replacing it (brew prefix at PATH position 1, the v2 symlinks at 17 and 19, measured), names issue 0036's unreachable remedy as a do-not-publish-before, and defers all migration detail to `migration.md` rather than restating it. Becomes user-facing at the WP-12 cutover. **No AC covers this deliverable** -- it completes the WP's list without closing a criterion
- Release mechanics for the Rust workspace (the bin/release successor decision) -- **RECORDED 2026-08-16 (dc), and finding a defect was what recorded it.** The successor is not one command but three composed, and **the composition was impossible until today**: `int build release` cuts the version, stamps sidecars, commits, tags, pushes AND creates the GitHub release (`gh release create`, build.d/release:531); `int macos publish` then required the tag to exist (the cut owns tagging) while refusing outright if a release for that tag existed. **Tag present and release absent is a state the only thing that creates the tag cannot leave behind.** Each command individually correct; nothing could cut a v3 release. Fixed by making the refusal test what its own comment protects -- ASSETS, not existence: a release carrying zero assets has published no bytes and no hash describes anything, so publish attaches to it (`gh release upload`); one carrying assets is refused exactly as before; an unreadable count fails closed. The sequence is therefore: `int build release vX.Y.Z` -> `int macos prepare` -> `int macos publish`. **UNEXERCISED, and it cannot be rehearsed here:** `staged_version` reports `3.0.0-dev`, so the dev-version refusal fires before the new branch is reachable, and making it reachable is the held `Cargo.toml`-sidecar item below. Branch selection canaried over all nine input shapes; `gh` confirmed to return `0` for a real assetless release. **Worth a D-number if vc agrees** -- it is a composition rule, not an implementation detail

## Dependencies

- WP-06 (a surface worth shipping).

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0056/acceptance.md`, under the `WP-11` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0056.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `the thread canon`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
