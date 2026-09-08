# ST0064 port design: the Intent menubar app (ic)

Authored by ic on the bounce (2026-08-31), the Intent-specific port of Geodica's menubar app. The Geodica handoff is the sibling attachment `design-menubar-app.md`; THIS records what the handoff does not -- the Intent module layout, the criterion-to-file binding, the gated seams, and the `int macos` lane decision. hv put ST0064 into the next release (the one that carries the daemon family; hv holds the number) as an M PORT; vc coordinates.

## The load-bearing rule (AC-01.1)

The app holds NO product logic. Every control is an `intent` verb it shells out to; every displayed fact comes from the daemon or the CLI. No log parser, no pidfile path, no config key in Swift. Verified by absence over the Swift sources.

## Module layout -- `native/macos/Intent/` (AS BUILT, re-synced 2026-09-08)

**EVERY PATH THIS SECTION NAMED ON 2026-08-31 WAS WRONG BY THE TIME THE APP EXISTED -- all six, not some.** It described `App/`, `Views/` and `Resources/` directories that were never created, `Health.swift` under `Services/`, and separate `AppDelegate.swift`, `MenuBuilder.swift` and `URLScheme.swift` files that do not exist. **A criterion-to-file binding is the thing a reader NAVIGATES BY, so a wrong one is worse than an absent one** -- it sends someone to a path, they find nothing, and the cheapest reading is that the work was not done. Corrected against the tree rather than against memory:

- `project.yml` + generated `Intent.xcodeproj` (tracked) -- xcodegen scans sources by presence, so a new file absent from the spec silently is not in the project.
- `Intent/IntentApp.swift` -- the menubar `NSStatusItem`, the menu construction, the lifecycle actions AND the `intent://` handler (`:212`). **At the target root, and there is no `AppDelegate.swift`, no `Views/` and no `MenuBuilder.swift`: the menu is built here.**
- `Intent/Models/Health.swift` -- the health-state consumer and its decoder. **`Models/`, not `Services/`.**
- `Intent/Services/` -- the seam to the CLI:
  - `LoginShell.swift` -- captures the login-shell PATH once (`/bin/zsh -ilc`); AC-01.9. Ported ~verbatim from Geodica.
  - `IntentCLI.swift` -- THE one shell-out wrapper: binary resolution, environment, error mapping, and both the batch (`capture`/`run`) and streaming (`stream`) forms. AC-01.1/01.9, the single home for spawning.
  - `DaemonService.swift` -- the health READ, `IntentCLI.capture(["daemon", "status", "--format", "json"])`, plus the lifecycle verbs. **Not in the 08-31 layout at all.**
  - `ProjectService.swift` -- the thread count, via `intent graphql`. **Not in the 08-31 layout at all.**
- `Intent/Utilities/` -- `ContinuousObservation.swift`, `Theme.swift`. **Not in the 08-31 layout at all.**
- `Intent/` root also holds `Info.plist` and both entitlements files (hardened runtime for notarisation). **There is no `Resources/` directory**; the icon lives in `Assets.xcassets`.
- **There is no `Console/`.** It remains ungated work, not a gated file -- see `AC-01.4` below and issue `0281`.

## Criterion-to-file binding (all on WP-01) -- AS BUILT, re-synced 2026-09-08

**DRIVE `intent ac list ST0064` FOR THE STATES; THE WORDS BELOW ARE THE BINDING, NOT THE VERDICT.** At this re-sync the gate read 7/9 with `AC-01.4` and `AC-01.7` open.

- AC-01.1 no-product-logic -- the whole layout; verified by an absence-grep over the Swift sources, now pinned by `one_daemon_predicate_across_both_trees.rs`.
- AC-01.2 health predicate == CLI routing predicate -- `Services/DaemonService.swift` shells the CLI verb; `Models/Health.swift` decodes it. **No longer gated: SATISFIED 2026-09-08 on a clause-to-arm mapping** (`daemon_status_answers_a_machine.rs` for the wire face, `daemon_health_splits_stale_from_absent.rs` for the round-trip clause, `one_daemon_predicate_across_both_trees.rs` for the one-predicate clause).
- AC-01.3 talks GraphQL/JSON via `intent graphql` -- `Services/ProjectService.swift`.
- AC-01.4 tail-orphan trap verified before any console -- **NOT a gated file: there is no `Console/` and no log source.** `IntentCLI.stream` exists and HAS NO CALLER. **Filed as `0281`**: `IntentCLI.swift:131` assigns the tail's lifecycle to _the verb's job, not the app's_, and no `log`/`tail`/`console` verb exists anywhere.
- AC-01.5 `intent://` handler through the one resolver -- **`IntentApp.swift:212`, NOT a separate `URLScheme.swift`.** The app speaks ADDRESSES only (vc ruling).
- AC-01.6 LIVE / STALE / ABSENT -- `Models/Health.swift` renders the three; the projection above `route()` is `intentsvcs::daemon`. **No longer gated: SATISFIED 2026-09-08.**
- AC-01.7 build/run/test/install devbin verb + signed and notarised -- the `int macos app-*` pipeline. **Open, and it is an EXTERNAL DEPENDENCY rather than unbuilt work: it needs Apple Developer credentials only hv can supply.** Do not read it as startable.
- AC-01.8 turtle icon, state derived at paint time with no cached `lastKnownState` -- `Assets.xcassets` + the `Health.swift` derivation.
- AC-01.9 captured login-shell PATH, every child handed that environment -- `Services/LoginShell.swift` + `Services/IntentCLI.swift`.

## Build order -- refusal arms first (vc)

1. Scaffold + `IntentApp` / `LoginShell` / `IntentCLI` (AC-01.9/01.1) -- the spine.
2. The `int macos` app pipeline REFUSAL ARMS first -- `verify` / `doctor` decline with no creds loaded; an unsigned local build is the most anyone can drive.
3. Status via `intent graphql` (AC-01.3), the turtle icon asset (AC-01.8).
4. WIRE-IN as the gated seams land: **the health display is DONE (2026-09-08, AC-01.2/01.6 satisfied) and the status line is itself the affordance that opens intentd's web face**; the console is still waiting on the log-source ruling AND on `0281`; the `intent://` emit half sits with the URI-uniformity work.

## The `int macos` lane decision (dc's file -- WP-11 / ST0056/11)

`bin/.devbin/cmd/macos` signs BARE Mach-O CLI binaries and disclaims `.app` bundles by construction -- no inside-out bundle walk, no entitlements, no stapling. A menubar `.app` needs all three (and a `.app` CAN be stapled, unlike a bare binary). AC-11.1's own rule governs: the AC names the outcome, the WP owns the mechanism. The credential/notary flow (ADC login keychain, notarytool, team 76BQL8L47U) is SHARED and must stay Highlander. Two options, dc's call on dc's file:

- (A) Extend `cmd/macos` with `app-build` / `app-run` / `app-sign` / ... subcommands, reusing its credential functions in place. One file grows; the credential lookup stays put.
- (B) Give the app pipeline its own home (`cmd/macos.d/app` or `cmd/app`) sourcing dc's credential/notary functions -- which first requires factoring those into `DEVBIN_LIB` so the split does not strand them (the file's own header names that hazard).

**RESOLVED AS (A), AND THE DOC SAT ON THE QUESTION AFTER IT WAS ANSWERED.** `bin/.devbin/cmd/macos` carries `app-doctor`, `app-icons`, `app-build`, `app-run`, `app-test`, `app-install` and `app-verify` in place, reusing the credential functions where they already lived. **What remains for `AC-01.7` is not the lane but the credentials**, which is hv's to supply and nobody's to start.

## Health display constraints (AC-01.6, vc's repair at `dcf92a1f`)

Three states is not enough on its own -- the row is satisfied only if STALE and ABSENT carry DIFFERENT remedies, because the prohibition is met by either candidate discriminator and a build cannot be read as compliant just because its vocabulary matches the sentence.

- STALE -- a holder is alive and not serving: investigate the pid, do NOT unlink (`AC-08.12`).
- ABSENT -- nothing owns the endpoint: the residue is safe to clear. An orphaned listening descriptor has no holder to investigate, so it is ABSENT, not STALE -- rendering it STALE declares a remedy it cannot carry out and fails the row.

So the DISPLAY shape follows the remedy, not just the label: the app MUST NOT offer an unlink affordance on STALE, and MAY on ABSENT. Have this before wiring against cc's machine face, not after.

**THIS PARAGRAPH USED TO SAY _mint `AT-01.6` when wiring the display_, AND THAT INSTRUCTION WAS WRONG -- IT IS REWRITTEN RATHER THAN TICKED, BECAUSE A READER WHO ACTED ON IT WOULD HAVE BUILT THE WRONG THING.** Driven 2026-09-08 off `criteria[]` in the canon: **all NINE ST0064 criteria are `kind: non-test`**, so the thread having zero ATs is structurally correct and not a coverage gap. **A non-test row closes by `intent ac satisfy --evidence`, never by minting an AT** -- and minting one for a non-test criterion is the same error as leaving a test criterion uncovered, in the other direction.

**WHAT SURVIVES FROM THE OLD PARAGRAPH IS THE TESTING ADVICE, WHICH WAS ALWAYS RIGHT:** construct both STALE and ABSENT, never wait for the 1-in-300 race, and assert the two DIFFERENT REMEDIES -- a suite that passes because the race did not fire has measured nothing. That is now built in `daemon_health_splits_stale_from_absent.rs`, and the arm that claimed the remedy clause turned out to be an enum-variant tautology that could not fail; it is replaced by two arms asserting what an operator may safely DO. The evidence for `AC-01.6` is the clause-to-arm mapping recorded on the criterion itself.
