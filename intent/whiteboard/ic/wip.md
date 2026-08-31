---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 13:23Z
status: active
focus: "Building ST0064 (the M-port menubar app, hv's 3.0.1 addition). This turn: the scaffold spine LANDED (5d915bfe + ce03a6d3 fix), typecheck-clean vs the macOS 14 SDK; AC-17.12 reason-fold applied (a571feda, vc-ratified). cc LANDED the machine-readable daemon status (ae8dc4d4) in my shape, so the health seam (AC-01.2/06) is UNBLOCKED -- the health display is my immediate next build. Dependencies all coordinated: vc ruled nav-internal + the bare-number ladder; dc ruled int-macos option A with 3 header conditions; cc built + shaped the health projection."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold verbatim at `.history/20260831/wip-fold-1213Z.md` (eleventh fold). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree. Coordinated by vc (hv, 2026-08-31).**

## DOING

**ST0064 -- THE MENUBAR APP. Scaffold spine LANDED; the health seam just UNBLOCKED; building the health display next.**

- **LANDED this turn:** `5d915bfe` the scaffold (`native/macos/Intent`: IntentApp + LoginShell + IntentCLI + project.yml + plists + LoginShellTests + tracked .xcodeproj), faithful ports of Geodica, typecheck-clean vs the macOS 14 SDK -> **AC-01.1 + AC-01.9 spine**. `ce03a6d3` fixed the intent:// handler (it wrongly shelled `intent explore`, which needs a terminal a .app cannot give; now a gated stub). `a571feda` AC-17.12 reason-fold (vc-ratified, row stays green). WP-01 started, port-design attached (`287955df`).
- **NEXT, UNBLOCKED -- the health display (AC-01.2 / AC-01.6 / AC-01.8-derivation).** cc landed `intent daemon status --format json` at `ae8dc4d4` in my shape: `{"state":"live|stale|absent","endpoint":<iff live>,"pid":<iff stale>}`. Build DaemonService (shells the verb, decodes), the Health model, wire the menubar icon + Start/Stop/Restart menu; the unlink/socket affordance gated on `state=="absent"`, NEVER on stale (AC-01.6 remedy). Mirror cc's 3-literal tripwire in a Swift decode test. Then MINT AT-01.6 with cc's bound-but-silent-socket fixture -- construct stale END TO END, never wait for the 1-in-300 race.

## TODO

1. **AC-01.7 -- the `int macos` app-\* verb.** dc RULED option A (extend `cmd/macos`, dc's file) with 3 header edits in the SAME commit as app-sign: strike the expired "five short subcommands" ground; rewrite the bundle disclaimer to what is TRUE (bare Mach-O for the CLI binaries, .app for the app, bundle walk + entitlements in app-\* arms, keep + sharpen the stapling asymmetry); name `kc_get`/`require_identity`/notarytool as the shared Highlander unit. Read the BUNDLE'S embedded marker for traceability, not the manifest (dc's stage-staleness gap, the 0187 class). PING dc the moment I edit `cmd/macos` (W4: one hand, one file); dc reviews the diff. Notarise arm = hv's ADC.
2. **AC-01.5 -- the intent:// handler wire-in.** BLOCKED on the DOOR, not on resolution semantics (vc's correction): both doors fail -- `edit intent://... --path` swallows the URL into <KIND>, `explore` needs a tty. vc routed the edit-narrow-door to cc to pull forward ahead of consumerless sites. The app inherits the ST->issue->WP ladder + multi-candidate for free (never reimplements it). Not a promote() call site (Swift shell-out). If a jump-to/search box ever takes a bare id, the candidate list is a VIEW to design.
3. **The console -- IN for 3.0.1 (hv ruled).** cc builds `intent daemon logs` (thin: intentd.log + intentd.err.log exist, userstate.rs:293/308 resolves both); I port Geodica's ConsoleRunner.swift + ConsoleWindowController.swift. AC-01.4's tail-orphan gate is LIVE WORK -- verify against SIGTERM/SIGINT/SIGKILL SEPARATELY before building the console on it. Wait for cc's verb, like the health face.
4. **AC-01.8 -- the turtle asset.** Rasterise `docs/design/intent-logo.svg` with rsvg-convert into the asset catalog; the state-derivation half wires with the health display.
5. **AC-09.6** -- the 21-row exposed=>servable flip; vc routes when dc's `dispatch-table.json` lands.
6. **AC-06.8** -- row red by vc's note; not mine to close.
7. **Parity-tool hardening** (mine, unblocked, not tag-blocking): conservation_check.sh refiled + UNACCOUNTED 5-way; gen_dispatch_table.sh rot.
8. **GraphQL** -- RESOLVED: vc landed AT-00.3, AC-00.3 closed (ST0056 108/135); AC-09.2 was a stale note, re-measured against HEAD.

## Watch-outs -- mechanisms only

1. **`daemon status --format json`: keys are ALPHABETICAL** (endpoint before state on live -- serde BTreeMap), Codable-irrelevant. The three literals (live/stale/absent) are cc's tripwire: a renamed `Health` variant compiles + serialises but SILENTLY stops decoding in Swift, in my repo where cc cannot see it -- MIRROR the tripwire in a Swift decode test. **REBUILD intentd + the debug intent before driving daemon status** (the running release pair predates today's commits).
2. **`intent explore` NEEDS A TERMINAL** -- never shell it from the app with captured stdout; it opens a TUI and refuses when stdout is not a tty. The pipe-safe address door is the URI-uniformity work.
3. **xcodegen scans sources BY PRESENCE** -- a new Swift file absent from a regenerated .xcodeproj silently is not in the build; regenerate on any source-set change.
4. **A CROSS-CRATE GUARD IS INVISIBLE TO `cargo test -p <changed-crate>`** -- run `cargo test -p intentsvcs` before any address-touching landing.
5. **`git add $P` WITH A SPACE-JOINED VAR DOES NOT WORD-SPLIT UNDER zsh** -- one pathspec, matches nothing, commits NOTHING. List paths explicitly.
6. **cc's POST-VERIFY IS STANDING: after every commit, `git show --name-only HEAD` must EQUAL your intended path set.**
7. **THE STORE IS SHARED BEFORE THE TREE IS** -- an `st attach`/`ac edit`/`wp new` realises into peers' canon on their sync; land canon writes in the same breath; NEVER write a shared extract (ST0056.json) a peer is mid-edit on -- coordinate the window (did this turn for AC-17.12).
8. **NEVER remove a peer's `.git/index.lock`** -- wait for it to clear (hit dc's lock this turn; retried clean).
9. **An ATTACHMENT does not realise to disk on `st attach`** -- the canon is SSOT; realise deliberately (disk = exact canon bytes) rather than `sync --to-disk`, which pulls peers' pending attachments into your extract.
10. **MARKDOWN PRETTIER-FORMATTED BEFORE `git add`.** THE FIGURES ROT AND THE PROPERTY DOES NOT -- re-run tools, never quote a peer's count.

## Decisions

- **2026-08-31 cc: health is a PROJECTION above `route()`** (`Health { Live(Endpoint), Stale{pid}, Absent }`, `daemon::health()`), Route stays two variants. AC-01.2 true by construction: app -> CLI verb -> projection -> route(). **STATE IS THE REMEDY** (vc's AC-01.6 ruling) -- no `removable` field; the app gates the unlink affordance on `state=="absent"`. STALE/ABSENT split on the KERNEL LOCK, not connect(); an orphan is ABSENT. Order is load-bearing (connect-then-lock); the app reads the answer, never recomputes it.
- **2026-08-31 vc/hv: the bare-number ladder** -- a family verb (`st show 55`) resolves in-family outright; the ST->issue->WP ladder is a FALLBACK when the family has no match; only a family-agnostic verb collects candidates. The resolver can return >1 candidate -> a VIEW if the app ever takes a bare id. Lives in `address::promote`; the app inherits it, reimplements nothing.
- **2026-08-31 vc: nav singular grammar is INTERNAL** (no Display/to_path/renderer -- parse-only, never emitted); the app speaks ADDRESSES only (AC-01.5).
- **2026-08-31 dc: `int macos` app pipeline = option A** (extend `cmd/macos`), with the 3 header-truth conditions in the app-sign commit + read the bundle's embedded marker (see TODO 1). (B)'s DEVBIN_LIB refactor waits for the tag (a dev-tree change with no shipped consequence, same shape hv ruled once).
- **2026-08-31 hv REVERSED ST0064 INTO 3.0.1 as an M PORT** (via vc). Criteria bind to WP-01 by AC-01.N prefix.
- **2026-08-31 AC-17.12 reason-fold RATIFIED + applied** (`a571feda`): the view grammar has no emitter, a floor under the second-resolver refusal. Row stays green.
