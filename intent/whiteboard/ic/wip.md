---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 20:59Z
status: active
focus: "Working (hv 'crack on'/afk, vc pen). ST0064 WP-01 = 3/9: AC-01.1+01.8+01.9 all SATISFIED with real evidence (9cad4780, d0cec969). Built the app (clean) AND the launch/inspect rig (minimal-env bundle launch + os_log, orphan-safe) -- 01.9 proven by its STATED instrument, not a substitute. NEXT: project-CWD (01.3/01.5) reuses that rig but needs a project-root discovery decision (app has no way to read the registry root yet). AC-09.6 closed earlier (rode vc's 337d6451)."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Working (2026-08-31 20:59Z). hv said crack on + is afk (#dietyspeed); vc holds the pen. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree and it is hot.**

## DOING

**ST0064 greening (app IN the next release).** Verified the built Swift app against my three directly-checkable ACs -- WP-01 now 3/9:

- **AC-01.1 (no product logic) -- SATISFIED (9cad4780).** Mechanical absence sweep clean; every control an intent verb; only spawns are LoginShell's PATH zsh + IntentCLI's run paths.
- **AC-01.8 (turtle from the predicate) -- SATISFIED (9cad4780).** MenuBarIcon = the brand turtle from docs/design/intent-logo.svg via `int macos app-icons` (d00956ec, template, visually confirmed); tint at paint time, no cache.
- **AC-01.9 (login-shell PATH) -- SATISFIED (d0cec969) BY THE STATED INSTRUMENT.** Built the Debug bundle, launched it under a deliberately minimal env (env -i HOME PATH=/usr/bin:/bin); os_log showed `launched; intent: .../target/release/intent (login shell: ~/.local/bin/intent)` -- resolved via the CAPTURED login shell, not the launch env. Positive+negative controlled. NOT a static read.

**Built two things worth reusing:** the app builds clean (`bin/devbin macos app-build`, unsigned Debug, off-tree under ~/.local/state), and the launch/inspect RIG (scratchpad/ac0109_launch.sh: stream os_log, launch bundle under a controlled env, capture, orphan-safe kill). vc's insight: 01.3/01.5 (project-CWD) reuse this same rig -- PATH-to-child and CWD-to-child are one harness.

## TODO

1. **project-CWD (01.3/01.5) -- NEEDS A DISCOVERY DECISION FIRST.** The app has NO project-root surface (only the IntentBinary override), `intent edit`/`intent graphql` take no project flag (pure CWD walk-up), and IntentCLI does NOT set the child's currentDirectoryURL -- so children inherit the app's `/` and resolve no project. vc's model = "a single configured project storing the registry's root path." The open point: HOW does the app read that root? There is no machine-level `intent` command that reports a registered project root (daemon is machine-level; edit/graphql are CWD-scoped). Settle discovery, then it is: store the root + IntentCLI sets child CWD + reuse the rig to prove children get it. 01.3 also needs one REAL graphql query through `intent graphql` or the row is vacuous.
2. **cc-gated ST0064 (01.2/01.6):** Swift side present + correct (Health reads cc's connect-then-lock order, projection above route(), STALE!=ABSENT remedy). Needs cc's live daemon + the two false-positive states CONSTRUCTED, not waited for. Coord cc.
3. **int macos chunk 2 (01.7 signing):** app-sign + app-notarize; dc owns the devbin pipeline (dc's 3 header edits ride the same commit); notarize = hv's ADC. Reconcile verb naming (AC-01.7/AC-11.1 `int macos sign|notarize|verify` vs `app-*`; Geodica uses one `app` verb).
4. **AC-01.4 console (cc-gated):** the tailing console on cc's `intent daemon logs`; the tail-orphan trap is a CLI-side fix verified vs SIGTERM/SIGINT/SIGKILL SEPARATELY before any UI. Not built (no Console/ files).
5. **`intent app start|stop|restart`** (NEW user-facing verb, hv 2026-08-31): controls the INSTALLED /Applications/Intent.app; new_surface `app` family; coord cc. Distinct from `int macos app-*`.
6. **Explorer residuals (hv-driven):** hv rebuild `dvb build all` + `intent explore`; open Q: does the Lotus menu SELECTION function (on_key has no MENU block)? Update the explorer artifact to mark Option A RULED+LANDED.

## Watch-outs -- mechanisms only

1. **A non-test AC closes via `intent ac satisfy --evidence <ref> <ST> <AC>`, NEVER an AT.** Before satisfying: RE-DRIVE the property with your own instrument and POSITIVE-CONTROL it. And USE THE ROW'S STATED INSTRUMENT -- AC-01.9 names a minimal-env BUNDLE launch, so a component-level capture test would have been a substitute; the built bundle was launched and its os_log read.
2. **A `.canon/st/*.json` write can land intermixed with a peer's uncommitted edit of the SAME file.** `--only` is path-scoped not hunk-scoped and `git add -p` is unavailable. Coordinate with the pen holder, keep HANDS OFF THE INDEX, let their commit carry both (vc's 337d6451 bundled three nodes' rows). A satisfy is a one-verb replay.
3. **The macOS app is `native/macos/Intent/` (Swift, xcodegen).** `bin/devbin macos app-build` builds unsigned Debug to ~/.local/state (off-tree). app-run execs the binary directly + prints the `log stream` predicate. IntentCLI = the one shell-out home; LoginShell = the PATH capture; Health.decode consumes `daemon status --format json`. dc owns `int macos`; ic is the ST0064 builder.
4. **A dated RULING RECORD is superseded, never edited.** info.md keeps hv's verbatim "3.0.1"; only my own prose (port-design.md) was named by property. If hv moves to 3.1.0, the superseding note is vc's under the pen.
5. **RUSTFMT BEFORE `git add` on any rust.** recoverability->exposure invariant needs a self-expiring anomaly (pop excludes retire). Attachment edits: `intent st attach <ST> <rel> --from <file>` writes store+canon (never the disk file); a running intentd auto-ingests disk edits (disk->store is directional). dispatch-table.md is generated.
6. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your set.** NEVER remove a peer's index.lock. RE-RUN TOOLS, never quote a peer's count. The delivered binary lags HEAD.

## Decisions

- **ST0064 WP-01 = 3/9 (ic).** AC-01.1 + AC-01.8 (9cad4780), AC-01.9 (d0cec969, stated instrument: minimal-env bundle launch, os_log confirms login-shell resolution). The launch/inspect rig is built and reusable for project-CWD.
- **AC-09.6 CLOSED (rode vc's 337d6451).** cc caught the row open-in-store while both boards said closed; I re-drove 138/60/0 (positive-controlled), satisfied it, kept hands off vc's index; vc's commit carried three nodes' rows.
- **The release is NOT 3.0.1 in fact** (vc: +13 commands/+30 flags, daemon answers on a socket; recommends 3.1.0). hv holds the number. My docs name it by property; hv's ruling record stays verbatim.
- **hv 2026-08-31: RULED Option A + it LANDED.** `/` cycles NAV->OMNIBOX->MENU->NAV; f9709004 + ec9e03b9; 123 tui tests. Coloured chip already built; hv saw a stale binary.
- **ST0064 is IN the next release** (hv 11:56Z). Daemon is ST0056 WP-08's. Signing IN (A8 reversed).
- **cc: `intent edit <address> --path` is the AC-01.5 resolver door** (pipe-safe); openAddress already routes through it. Health is a PROJECTION above `route()`; STALE never unlinks (AC-08.12).
- **hv 2026-08-31: `intent app start|stop|restart`** = user-facing menubar lifecycle verb, distinct from `int macos app-*`.
