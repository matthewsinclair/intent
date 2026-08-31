---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 14:47Z
status: active
focus: "ST0064 (the M-port menubar app), 6 of 9 criteria BUILT + typecheck-clean (spine, health display, intent:// handler). NEXT: the int macos app-* build/sign verb (dc's option A, dc's file) -- SCOPED; draft chunk 1 (no-creds: doctor/verify/build/run/test/icons) as a diff for dc's review, then chunk 2 (sign/notarize + dc's 3 header edits). Plus the project-model CWD wiring (AC-01.5 runtime + AC-01.3; vc ruled store-root-path). Every dependency is coordinated + ruled."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Twelfth fold. Pre-fold verbatim at `.history/20260831/wip-fold-1428Z.md` (sha-verified). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree. Coordinated by vc.**

## DOING

**ST0064 -- the menubar app. 6/9 criteria BUILT + typecheck-clean vs the macOS 14 SDK** (`native/macos/Intent`, faithful Geodica ports; the port design is attached to ST0064).

- BUILT: **AC-01.1** (no product logic), **01.9** (login-shell PATH + one shell-out), **01.2/06** (health display -- DaemonService polls `daemon status --format json`, the tinted tortoise + state-gated menu, unlink NEVER on stale), **01.8-derivation** (tint at paint time, no cache), **01.5** (intent:// -> `intent edit <addr> --path`, verified). Commits: `5d915bfe` spine, `94ea6126` health, `2832a7fe`+`ce03a6d3` handler, `176fceb2` the mcp red-test/protocol fix.
- _*NEXT -- the int macos app-* build verb, chunk 1 (no-creds)._* Reference: Geodica's `bin/geodica_app` (511-line ELIXIR), TRANSLATE to bash into dc's `cmd/macos`, reuse its `kc_get`/`IDENTITY`/`notarytool` unit. Chunk 1: app-doctor + app-verify (refusal arms) + **app-icons** (= AC-01.8 asset, `rsvg-convert docs/design/intent-logo.svg`) + app-build/run/test/install (xcodebuild; SYMROOT/OBJROOT off the Dropbox tree; stamps as build settings; exec-not-`open`). **PING dc first (W4, one hand); draft the diff; dc reviews.** Chunk 2: app-sign/notarize + dc's 3 header-truth edits + the bundle-embedded-marker read.

**RESOLVED at pickup (2026-08-31 14:47Z):** the `show.rs` emit-partner LANDED and is not mine to touch. hv ruled the emit half REFUSAL-ONLY for 3.0.1; the refusal landed in `3388d0fe` (`address::parse` names the two-slash cause, inherited by `edit`/`sync`/`nav`/MCP -- so my `intent edit` door gets the clearer refusal for free). A contested TEXT `address:` line (st/wp/issues show via `Address::to_url`, `show.rs:79/98/126`) landed as a SELF-REVERTING standalone commit `a854d7c3` pending hv -- also not mine. My URI-split emit assignment is discharged.

## TODO

1. _*The int macos app-* verb_* -- chunk 1 then chunk 2 (above). THE piece that makes the app build/run/test. Notarise arm = hv's ADC.
2. **Project-model CWD wiring (AC-01.5 runtime + AC-01.3)** -- vc RULED: store the project ROOT PATH only (a setting = the daemon registry's `root`); IntentCLI sets the child CWD to it for project-scoped commands (edit, graphql); daemon control stays machine-level (comment where they diverge). Then AC-01.3's one real `intent graphql` query -- thread count in the menu (vc's discharge, refutable).
3. **AT-01.6** -- mint with cc's bound-but-silent-socket fixture (cc offered; ping when at it). Construct stale END TO END, never wait for the race; assert the two remedies differ.
4. **AC-01.4 + console** -- IN for 3.0.1; port Geodica's ConsoleRunner + ConsoleWindowController; the tail-orphan gate is LIVE WORK (SIGTERM/SIGINT/SIGKILL separately). GATED on cc's `intent daemon logs` verb.
5. **WP-09 (ST0056/09) -- re-surfaced by vc/dc since the fold; all mine:**
   - **AC-09.6 -- vc's SEQUENCED critical-path drive** (a row of its own now, not "when dispatch-table lands"). Minted as AGREEMENT: _exposed implies servable_. Read the ~94 MCP-exposed arms, report the fraction with ONE servable door -- the output is a DECISION for hv (BUILD the missing methods OR NARROW `exposed_on_mcp`), NOT a backlog. **Do not build ~54 facades because the row seems to ask.** Facade gaps become their own rows/issues (`fc` = 0171). Naive brace-matching cannot find an arm body in `render.rs` (format-literal braces) -- the cost is real and vc put that reason in the criterion.
   - **AC-09.4 clause 2 -- CLASS check** (vc ruled estate-scoped, row open). `SERVED_BY_DAEMON` becomes a DECLARED exception with a meetable discharge condition (cc corrected mine: "daemon serves the surface" is UNMEETABLE for version/info/init/lang; cc's meetable version lives at `render.rs:7729`). MOVE the scan primitives into `tests/common/mod.rs` (cc's shared-apparatus home -- do NOT fork); coordinate with cc; do NOT close on the one guarded instance.
   - **guide.rs rc=2** -- the "two closed causes" framing is ALREADY rewritten (`:152` now reads _not a closed list_). RESIDUAL only: dc's census (inbox 16:55Z) found G (`init` refusing an existing project) + H (`--limit` non-number) exit **2** for what `:152/:154` assign to **1** -- a refusal wearing the fail-open code. VERIFY at HEAD on the DEBUG build, then reconcile; non-tag-blocking (neither is gate-run).
   - **AC-09.2** -- the `intent_graphql` escape hatch is UNOWNED in practice (vc: `mcp.rs:86` doc-comment only; no executable query in any binary). Awareness; hv is scoping (may descope to ST0069 with AC-00.4's clause).
6. **Held for hv (my other claims):** **WP-17 / ST0056-17 (TUI)** -- hv: _quite different to what we agreed_; rework coming, hv scopes it directly; do NOT spend on section-9 vocabulary until then. **ST0065** -- hv taking it directly (vc's routing debt discharged). **parity hardening** (mine, not tag-blocking); **AC-06.8** (row red, not mine).

## Watch-outs -- mechanisms only

1. **`daemon status --format json`: keys ALPHABETICAL** (endpoint before state on live). The three literals are cc's tripwire (a renamed variant compiles + serialises but SILENTLY stops decoding in Swift) -- mirrored in HealthTests. **REBUILD intentd + the debug intent before driving daemon-backed behaviour.**
2. **The `intent://` canonical form is THREE slashes** (`intent:///threads/ST0000` -- empty authority = this project). Two slashes -> `threads` parses as authority -> refused.
3. **`intent explore` NEEDS A TERMINAL** -- never shell it from the app; the pipe-safe door is `intent edit <addr> --path`. Both `edit` and `graphql` resolve the project by CWD walk-up -- the app must set the child CWD (project-model wiring).
4. **A FILTERED test run cannot report a verdict; only the unfiltered cargo exit code can** -- a suite piped through grep reports grep's rc 0 over a red suite. Capture cargo's own rc.
5. **xcodegen scans sources BY PRESENCE** -- regenerate the .xcodeproj on any source-set change (I track it).
6. **A CROSS-CRATE GUARD is invisible to `cargo test -p <changed-crate>`** -- `cargo test -p intentsvcs` before address-touching landings.
7. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your intended set.** List paths explicitly. **NEVER remove a peer's index.lock** -- wait for it to clear.
8. **THE STORE IS SHARED BEFORE THE TREE IS; NEVER write a shared extract a peer is mid-edit on** -- coordinate the window. An attachment does not realise to disk on `st attach` -- realise deliberately, never `sync --to-disk`.
9. **MARKDOWN PRETTIER BEFORE `git add`. THE FIGURES ROT AND THE PROPERTY DOES NOT** -- re-run tools, never quote a peer's count.

## Decisions

- **cc: `intent edit <address> --path` is the AC-01.5 resolver door** (`9508788`), pipe-safe, no terminal, no kind. `intent explore` needs a tty (declined).
- **cc: health is a PROJECTION above `route()`** (Route stays two variants); STATE IS THE REMEDY (no `removable` field), gate unlink on absent; stale/absent split on the kernel lock; connect-then-lock order is load-bearing.
- **vc: the app project model is (a) a single configured project storing the ROOT PATH only** (the registry's `root`); child CWD for project-scoped commands, daemon control machine-level; a later picker fills the SAME field. Cheap to overturn.
- **vc/hv: the bare-number ladder** (family verb resolves in-family, ladder is fallback, family-agnostic verb lists candidates; can return >1 -> a VIEW). **nav singular is INTERNAL; the app speaks addresses only.**
- **vc: AC-01.3 divides cleanly from AC-01.6, neither yields** -- liveness from the query (positive), the kernel lock names WHICH kind of stopped (negative). The row now REQUIRES one real `intent graphql` query, else it is vacuously green.
- **dc: `int macos` app pipeline = option A** + 3 header-truth conditions + read the bundle marker. (B)'s DEVBIN_LIB refactor waits for the tag.
- **hv: console IN for 3.0.1** via cc's `intent daemon logs` verb (a direct logfile tail was declined). ST0064 is an M PORT; criteria bind to WP-01.
