---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 14:06Z
status: active
focus: "ST0064 (the M-port menubar app). This turn: the health display LANDED (94ea6126 -- daemon status drives the tinted tortoise + state-gated menu, AC-01.2/06 + the 01.8 derivation) and AC-01.5 LANDED (2832a7fe -- wired to cc's resolver door 9508788, verified end to end). Also fixed a red mcp_stdio test + the protocol defect under it (176fceb2, mine from AC-09.5). Six of nine criteria are built + typecheck-clean; the big remaining piece is the int macos app-* build/sign verb (AC-01.7, dc's option A) that makes the app actually build/run/test."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Pre-fold at `.history/20260831/wip-fold-1213Z.md`. RE-MEASURE EVERY FIGURE AT PICKUP. Coordinated by vc (hv, 2026-08-31).**

## DOING

**ST0064 -- the menubar app. BUILT + typecheck-clean this turn; the build/sign verb is the next major piece.** All Swift is faithful ports of Geodica, `native/macos/Intent`, typechecks against the macOS 14 SDK.

Criterion status (code built; gate-satisfaction via evidence/AT comes with the build verb + AT-01.6):

- **AC-01.1** no product logic -- built (verify by an absence-grep at the end).
- **AC-01.2 / 01.6** health -- BUILT (`94ea6126`): DaemonService polls `daemon status --format json` (cc's projection, the one predicate via the CLI), decodes {state,endpoint?,pid?}; the menu gates Start/Stop/Restart on it, unlink NEVER on stale (pid named, investigate). HealthTests mirror cc's 3-literal tripwire.
- **AC-01.8 derivation** -- BUILT: the tortoise tint is computed at paint time from the state, no cache. **Asset half TODO** (rasterise intent-logo.svg).
- **AC-01.5** intent:// handler -- BUILT + VERIFIED (`2832a7fe`): hands the whole address to `intent edit <addr> --path` (cc's door), opens the path, parses nothing. Canonical form is `intent:///` (THREE slashes).
- **AC-01.9 / 01.1** login-shell + one shell-out -- BUILT (`5d915bfe`), LoginShellTests green-by-design.

## TODO

1. **AC-01.7 -- the `int macos` app-\* build/sign verb. THE next major build; it makes the app actually build/run/test.** dc's option A: extend `cmd/macos` (dc's file) with app build/run/test/install, reusing the credential/notary flow. In the SAME commit as app-sign, dc's 3 header-truth edits (strike the "five short subcommands" ground; rewrite the .app disclaimer + sharpen the stapling asymmetry; name kc_get/require_identity/notarytool as the shared unit) + read the BUNDLE'S embedded marker, not the manifest. **PING dc before editing `cmd/macos` (W4, one hand); dc reviews the diff.** Notarise = hv's ADC.
2. **AT-01.6 -- mint with cc's bound-but-silent-socket fixture** (cc offered; ping when at it). Construct stale END TO END, never wait for the race; assert the two remedies differ.
3. **AC-01.8 asset** -- rasterise `docs/design/intent-logo.svg` (rsvg-convert) into the asset catalog as MenuBarIcon + AppIcon.
4. **AC-01.4 + console** -- IN for 3.0.1; port Geodica's ConsoleRunner + ConsoleWindowController; the tail-orphan gate is live work (verify SIGTERM/SIGINT/SIGKILL separately). GATED on cc's `intent daemon logs` verb.
5. **AC-01.3** -- coherence Q to vc: does daemon-status satisfy it (CLI channel talks GraphQL/JSON to intentd) or does the app owe one real `intent graphql` query? Not blocking.
6. **AC-09.6** -- 21-row flip, vc routes when dc's dispatch-table.json lands. **Parity hardening** (mine, not tag-blocking). **AC-06.8** row red, not mine.

## Watch-outs -- mechanisms only

1. **`daemon status --format json`: keys ALPHABETICAL** (endpoint before state on live). The three literals are cc's tripwire (a renamed variant compiles + serialises but SILENTLY stops decoding in Swift) -- mirrored in HealthTests. **REBUILD intentd + the debug intent before driving daemon-backed behaviour** (cc's commits land faster than the release pair).
2. **The `intent://` canonical form is THREE slashes** (`intent:///threads/ST0000` -- empty authority = this project). Two slashes -> `threads` parses as authority -> refused. Four of us wrote the two-slash form from memory because nothing emits the right one.
3. **`intent explore` NEEDS A TERMINAL** -- never shell it from the app; the pipe-safe door is `intent edit <addr> --path` (cc's 9508788).
4. **A FILTERED test run cannot report a verdict; only the unfiltered cargo exit code can** -- a suite piped through grep reports grep's rc 0 over a red suite (how the mcp_stdio red hid). Capture cargo's own rc.
5. **xcodegen scans sources BY PRESENCE** -- regenerate the .xcodeproj on any source-set change (I track it).
6. **A CROSS-CRATE GUARD is invisible to `cargo test -p <changed-crate>`** -- `cargo test -p intentsvcs` before address-touching landings.
7. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your intended set.** List paths explicitly (zsh `git add $var` is one pathspec). **NEVER remove a peer's index.lock** -- wait (hit dc's this turn).
8. **THE STORE IS SHARED BEFORE THE TREE IS; NEVER write a shared extract a peer is mid-edit on** -- coordinate the window (did for AC-17.12 on ST0056.json). An attachment does not realise to disk on `st attach` -- realise deliberately, never `sync --to-disk` (pulls peers' pending).
9. **MARKDOWN PRETTIER BEFORE `git add`. THE FIGURES ROT AND THE PROPERTY DOES NOT** -- re-run tools, never quote a peer's count.

## Decisions

- **2026-08-31 cc: `intent edit <address> --path` is the AC-01.5 resolver door** (`9508788`), pipe-safe, no terminal, no kind; the app opens what it returns. `intent explore` needs a tty (declined).
- **2026-08-31 cc: health is a PROJECTION above `route()`** (Route stays two variants); state IS the remedy (no `removable` field), the app gates unlink on absent; stale/absent split on the kernel lock; connect-then-lock order is load-bearing.
- **2026-08-31 vc/hv: the bare-number ladder** (family verb resolves in-family, ladder is fallback, family-agnostic verb lists candidates); the resolver can return >1 -> a VIEW if the app takes a bare id. **nav singular is INTERNAL; the app speaks addresses only.**
- **2026-08-31 dc: `int macos` app pipeline = option A** + 3 header-truth conditions + read the bundle marker (TODO 1). (B)'s DEVBIN_LIB refactor waits for the tag.
- **2026-08-31 hv: console IN for 3.0.1** via cc's `intent daemon logs` verb (a direct logfile tail was declined -- Swift would learn the log location). ST0064 is an M PORT; criteria bind to WP-01.
- **2026-08-31 AC-17.12 reason-fold applied** (`a571feda`); the mcp resources/list refusal is INTERNAL_ERROR, and a stale test named a live method (`176fceb2`).
