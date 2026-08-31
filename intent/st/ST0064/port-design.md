# ST0064 port design: the Intent menubar app (ic)

Authored by ic on the bounce (2026-08-31), the Intent-specific port of Geodica's menubar app. The Geodica handoff is the sibling attachment `design-menubar-app.md`; THIS records what the handoff does not -- the Intent module layout, the criterion-to-file binding, the gated seams, and the `int macos` lane decision. hv put ST0064 into the next release (the one that carries the daemon family; hv holds the number) as an M PORT; vc coordinates.

## The load-bearing rule (AC-01.1)

The app holds NO product logic. Every control is an `intent` verb it shells out to; every displayed fact comes from the daemon or the CLI. No log parser, no pidfile path, no config key in Swift. Verified by absence over the Swift sources.

## Module layout -- `native/macos/Intent/`

- `project.yml` + generated `Intent.xcodeproj` (tracked) -- xcodegen scans sources by presence, so a new file absent from the spec silently is not in the project.
- `Intent/App/` -- `IntentApp.swift` (menubar `NSStatusItem`, lifecycle), `AppDelegate.swift`.
- `Intent/Services/` -- the seam to the CLI:
  - `LoginShell.swift` -- captures the login-shell PATH once (`/bin/zsh -ilc`); AC-01.9. Ported ~verbatim from Geodica (priced 96 lines).
  - `IntentCLI.swift` -- THE one shell-out wrapper: binary resolution (login-shell PATH + symlink fallback + a Locate override), environment, error mapping. AC-01.1/01.9, the single home for spawning.
  - `Health.swift` -- the health-state consumer. GATED on cc's machine-readable `daemon status` + three-state projection (AC-01.2/01.6). A wire-in point only until cc lands.
  - `URLScheme.swift` -- the `intent://` handler. Hands the URL to the resolver via `intent explore` / `intent edit`; the app speaks ADDRESSES only (vc ruling, nav singular is internal). AC-01.5.
- `Intent/Views/` -- `MenuBuilder.swift` (the menu). `Console/` is GATED: `intent daemon` has no logs/tail verb, so there is no log source yet (scope question to hv).
- `Intent/Resources/` -- `Info.plist`, entitlements (hardened runtime for notarisation), the turtle icon rasterised from `docs/design/intent-logo.svg` with `rsvg-convert`, never `qlmanage` (which flattens transparency to white).

## Criterion-to-file binding (all on WP-01)

- AC-01.1 no-product-logic -- the whole layout; verified by an absence-grep over Swift sources.
- AC-01.2 health predicate == CLI routing predicate -- `Health.swift` calls the CLI verb, never a Swift probe. GATED on cc.
- AC-01.3 talks GraphQL/JSON via `intent graphql`, status is a query and `stopped` = query fails -- `Health.swift` / `IntentCLI.swift`. Buildable now: the executor is in HEAD (dbfc1eb1).
- AC-01.4 tail-orphan trap verified before any console -- gated on the log-source scope question.
- AC-01.5 `intent://` handler through the one resolver -- `URLScheme.swift`.
- AC-01.6 LIVE / STALE / ABSENT -- `Health.swift`; GATED on cc (`Route` stays two variants, the third state is a projection above `route()` keyed on the kernel lock).
- AC-01.7 build/run/test/install devbin verb + signed and notarised -- the `int macos` app pipeline (lane decision below).
- AC-01.8 turtle icon, state derived at paint time with no cached `lastKnownState` -- `Resources/` asset + the `Health.swift` derivation (the derivation half is gated).
- AC-01.9 captured login-shell PATH, every child handed that environment -- `LoginShell.swift` + `IntentCLI.swift`.

## Build order -- refusal arms first (vc)

1. Scaffold + `IntentApp` / `LoginShell` / `IntentCLI` (AC-01.9/01.1) -- the spine.
2. The `int macos` app pipeline REFUSAL ARMS first -- `verify` / `doctor` decline with no creds loaded; an unsigned local build is the most anyone can drive.
3. Status via `intent graphql` (AC-01.3), the turtle icon asset (AC-01.8).
4. WIRE-IN as the gated seams land: the health display (cc), the console (log-source ruling), the `intent://` emit half (the URI-uniformity work).

## The `int macos` lane decision (dc's file -- WP-11 / ST0056/11)

`bin/.devbin/cmd/macos` signs BARE Mach-O CLI binaries and disclaims `.app` bundles by construction -- no inside-out bundle walk, no entitlements, no stapling. A menubar `.app` needs all three (and a `.app` CAN be stapled, unlike a bare binary). AC-11.1's own rule governs: the AC names the outcome, the WP owns the mechanism. The credential/notary flow (ADC login keychain, notarytool, team 76BQL8L47U) is SHARED and must stay Highlander. Two options, dc's call on dc's file:

- (A) Extend `cmd/macos` with `app-build` / `app-run` / `app-sign` / ... subcommands, reusing its credential functions in place. One file grows; the credential lookup stays put.
- (B) Give the app pipeline its own home (`cmd/macos.d/app` or `cmd/app`) sourcing dc's credential/notary functions -- which first requires factoring those into `DEVBIN_LIB` so the split does not strand them (the file's own header names that hazard).

Flagged to dc; the pipeline itself waits on this answer. The Swift scaffold and the spine do not.

## Health display constraints (AC-01.6, vc's repair at `dcf92a1f`)

Three states is not enough on its own -- the row is satisfied only if STALE and ABSENT carry DIFFERENT remedies, because the prohibition is met by either candidate discriminator and a build cannot be read as compliant just because its vocabulary matches the sentence.

- STALE -- a holder is alive and not serving: investigate the pid, do NOT unlink (`AC-08.12`).
- ABSENT -- nothing owns the endpoint: the residue is safe to clear. An orphaned listening descriptor has no holder to investigate, so it is ABSENT, not STALE -- rendering it STALE declares a remedy it cannot carry out and fails the row.

So the DISPLAY shape follows the remedy, not just the label: the app MUST NOT offer an unlink affordance on STALE, and MAY on ABSENT. Have this before wiring against cc's machine face, not after.

AC-01.6 has NO covering AT today (vc's 69-thread sweep, 2026-08-31). A non-test row a gate cannot turn green. Mint AT-01.6 when wiring the display: CONSTRUCT both STALE and ABSENT (never wait for the 1-in-300 race), and assert the two different remedies -- a suite that passes because the race did not fire has measured nothing.
