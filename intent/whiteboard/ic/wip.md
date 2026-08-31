---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 20:40Z
status: active
focus: "Post-compact, WORKING (hv 'crack on', vc pen). LANDED THIS SESSION: AC-09.6 closed (satisfy rode vc's 337d6451), ST0064 AC-01.1+01.8 verified+satisfied (9cad4780). ON: ST0064 greening -- 01.9 needs a minimal-env bundle launch, then cc-gated 01.2/01.6, project-CWD 01.3/01.5, signing 01.7. Explorer Option-A still awaits hv's rebuild."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Working post-compact (2026-08-31 20:40Z). hv said crack on + is afk (#dietyspeed); vc holds the pen. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree and it is hot; the delivered binary lags HEAD.**

## DOING

**ST0064 greening (app IN 3.0.1).** Read the Geodica design handoff (design-menubar-app.md) -- the load-bearing rule is the app holds NO product logic, every control shells an `intent` verb, the daemon is the authority on itself. Verified the built Swift app (9 sources) against my ACs:

- **AC-01.1 (no product logic) -- SATISFIED (9cad4780).** Mechanical absence sweep clean (no log parser / pidfile assembly / config read); every control routes to an intent verb; only spawns are LoginShell's PATH zsh + IntentCLI's run paths.
- **AC-01.8 (turtle from the predicate) -- SATISFIED (9cad4780).** MenuBarIcon = the brand turtle rasterised from docs/design/intent-logo.svg by `int macos app-icons` (d00956ec, template, visually confirmed); tint computed at paint time from the one health predicate, no cache. Fixed menuBarImage()'s stale "asset not landed" comment in the same commit.
- **AC-01.9 (login-shell PATH) -- NOT YET.** Mechanism is built + unit-tested (LoginShell/IntentCLI, one home, PATH handed to every child), but the row's OWN stated proof is a launch of the BUILT bundle with a deliberately minimal environment. That runtime check is unrun -- a static read cannot satisfy this row. NEXT.

## TODO

1. **AC-01.9 proof:** build the bundle (`int macos app-build`), launch it with a stripped env (no dev PATH), confirm it resolves `intent` via the captured login shell -- then satisfy. This is the row's stated instrument; the read-only mechanism is already confirmed.
2. **cc-gated ST0064 (01.2/01.6):** health predicate = the CLI routing predicate (one path), three states live/stale/absent as a PROJECTION above route(), STALE!=ABSENT in the remedy. Swift side is present + correct-looking (Health.swift, DaemonService reads cc's connect-then-lock order, never recomputes). Needs cc's live daemon + the two false-positive states CONSTRUCTED (socket-residue + inherited descriptor), not waited for. Coord cc.
3. **project-CWD keystone (unblocks 01.3 + 01.5):** the app must launch children with CWD = the configured project root so `intent edit <addr> --path` (cc's resolver door) and `intent graphql` resolve by CWD walk-up. 01.3 also needs one REAL graphql query exercised through `intent graphql` (else the row is vacuous). 01.5 = the intent:// handler is a CLIENT of ST0057 WP-07, never a second resolver (openAddress already does this).
4. **int macos chunk 2 (01.7 signing):** app-sign + app-notarize all three artefacts (dc owns the devbin pipeline; dc's 3 header-truth edits ride the same commit). Notarize = hv's ADC (human step). Reconcile verb naming: AC-01.7/AC-11.1 cite `int macos sign|notarize|verify` vs `app-*`; Geodica uses one `geodica app` verb for the whole lifecycle.
5. **AC-01.4 console (cc-gated):** the tailing console on cc's `intent daemon logs`; the tail-orphan trap is a CLI-side fix (process-group teardown on stdin close), verified vs SIGTERM/SIGINT/SIGKILL SEPARATELY before any UI. Not built (no Console/ files yet).
6. **`intent app start|stop|restart`** (NEW user-facing verb, hv 2026-08-31): controls the INSTALLED /Applications/Intent.app; new_surface `app` family; coord cc on dispatch entry + handler. Distinct from `int macos app-*` (dev pipeline).
7. **Explorer residuals (hv-driven):** hv rebuild `dvb build all` + `intent explore` on a current binary (Option-A ring + chip + `/` legend all in HEAD; delivered binary lags). Open Q: does the Lotus menu SELECTION function (on_key has no MENU block)? Update the explorer artifact to mark Option A RULED+LANDED.

## Watch-outs -- mechanisms only

1. **A non-test AC closes via `intent ac satisfy --evidence <ref> <ST> <AC>`, NEVER an AT.** A non-test row is never computed from covering ATs, so writing an AT closes nothing (dc's "no covering AT" on AC-09.6 had the right conclusion, wrong mechanism). Before satisfying: RE-DRIVE the property with your own instrument and POSITIVE-CONTROL it (inject a fake violation, confirm the counter moves) -- a 0 from an instrument that cannot fail is decoration.
2. **A `.canon/st/*.json` write can land intermixed with a peer's uncommitted edit of the SAME file.** `--only` is path-scoped not hunk-scoped and `git add -p` is unavailable here, so you cannot split it. Do NOT commit -- coordinate with the pen holder, keep HANDS OFF THE INDEX, let their commit carry both (vc's 337d6451 bundled three nodes' rows because canon cannot be split). A satisfy is a one-verb replay if it must be redone.
3. **RUSTFMT BEFORE `git add` on any rust** -- the gate REFUSES an unformatted rust file. `rustfmt --edition 2024 <files>`, then add.
4. **recoverability -> exposure invariant (`gen_dispatch_table.sh` arm 1).** Narrowing an UNDOABLE mutation off MCP (exposed:false while reversible/idempotent) REQUIRES a self-expiring `recoverability_anomaly`; population EXCLUDES `disposition/target.state == retire`.
5. **ATTACHMENT DRIFT.** Editing a file under `intent/st/<ST>/` directly leaves canon naming OLD bytes -> `canon_commit_check` blocks. `intent st attach <ST> <rel-path> --from <file>` is the ONLY fix; commit file + `intent/.canon/st/<ST>.json` together.
6. **surface/dispatch-table.md is GENERATED** from the .json via `gen_dispatch_table.sh` (IN/OUT env) -- regen on any .json change.
7. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your intended set.** NEVER remove a peer's index.lock -- WAIT. RE-RUN TOOLS, never quote a peer's count (cc read 61 vs my 60 once; I re-drove 138/60/0 myself this session and it matched). The delivered binary lags HEAD + can vanish.
8. **The macOS app is `native/macos/Intent/` (Swift, xcodegen from project.yml).** IntentCLI = the one shell-out home; LoginShell = the PATH capture; Health.decode consumes `daemon status --format json`; Theme.menuBarTint is the paint-time tint. The devbin `macos` verb (`int macos app-*`) builds/icons; dc owns it. ST0064 builder is ic.

## Decisions

- **AC-09.6 CLOSED (row satisfied in store, landed in vc's 337d6451).** cc caught that the CODE landed (f435cc10 + ea68dddd) but the non-test ROW was never marked, while both boards said CLOSED. hv had lifted the hold, WP-09 is my claim, so I re-drove the property myself (138 non-flag rows / 60 exposed / 0 exposed-without-facade, instrument positive-controlled) and ran `intent ac satisfy ST0056 AC-09.6`. It landed intermixed with vc's canon edits; vc's commit carried all three nodes' rows.
- **ST0064 AC-01.1 + AC-01.8 SATISFIED (ic, 9cad4780).** Verified against the built Swift app + the turtle asset provenance. AC-01.9 stays open until the minimal-env bundle launch runs (its stated proof).
- **hv 2026-08-31: RULED Option A + it LANDED.** `/` cycles NAV->OMNIBOX->MENU->NAV (empty-buffer guarded), Esc = the non-cyclic way back, never quits. mode.rs + keys.rs + tui-design.md = f9709004; `/` hint legend = ec9e03b9. 123 tui tests. Coloured mode chip already built 2026-08-30; hv's plain footer was a stale binary.
- **ST0064 is IN 3.0.1** (hv 11:56Z, info.md line 17). Signing IN (A8 reversed). Daemon is ST0056 WP-08's, not this thread's.
- **hv 2026-08-31: `intent app start|stop|restart`** = user-facing verb for the menubar app lifecycle, distinct from `int macos app-*`.
- **cc: `intent edit <address> --path` is the AC-01.5 resolver door** (pipe-safe, no tty). App speaks addresses only; openAddress already routes through it.
- **cc: health is a PROJECTION above `route()`** (Route stays two variants); state is the remedy; STALE never unlinks (AC-08.12).
