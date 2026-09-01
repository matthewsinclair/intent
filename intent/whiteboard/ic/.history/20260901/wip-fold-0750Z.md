---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 21:19Z
status: active
focus: "Working (hv 'crack on'/afk, vc pen). ST0064 WP-01 3/9 satisfied (01.1/01.8/01.9, real evidence). project-CWD WIRING landed (7e84538a, compiles clean) under vc ruling (a) + conditions (i) loud-refuse and (ii) interim-for-D07. NEXT: 01.3/01.5 satisfaction = one real `intent graphql` query in the app + the rig-proof (set root, launch, confirm children resolve THIS project). AC-09.6 closed earlier (rode 337d6451). App builds via bin/devbin macos app-build; launch rig at scratchpad/ac0109_launch.sh."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Working (2026-08-31 21:09Z). hv said crack on + is afk (#dietyspeed); vc holds the pen. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree and it is hot.**

## DOING

**ST0064 greening (app IN the next release).** WP-01 = 3/9 satisfied; project-CWD WIRING landed:

- **AC-01.1 (no product logic) -- SATISFIED (9cad4780).** Absence sweep clean; every control an intent verb; one spawn home.
- **AC-01.8 (turtle from the predicate) -- SATISFIED (9cad4780).** MenuBarIcon = brand turtle via `int macos app-icons`; tint at paint time, no cache.
- **AC-01.9 (login-shell PATH) -- SATISFIED (d0cec969) BY THE STATED INSTRUMENT.** Built bundle launched under env -i; os_log confirms login-shell resolution. Positive+negative controlled.
- **project-CWD WIRING -- LANDED (7e84538a), compiles clean.** IntentCLI sets child currentDirectoryURL to a VALIDATED configured root (ProjectConfig, UserDefaults IntentProjectRoot, isIntentProject on intent/.config/config.json). Condition (i): configured-but-invalid = loud refusal (projectRootInvalid); unconfigured stays quiet (CLI's own not-in-project error). Condition (ii): marked in-code as INTERIM for D07's unbuilt registry -- reads it and goes away when D07 lands, never a parallel home (0204).

## TODO

1. **Satisfy 01.3/01.5 (the wiring is landed; these are the proofs):**
   - 01.3 needs ONE REAL query through `intent graphql` exercised BY the app (today it uses `daemon status --format json`, not graphql) -- a small behaviour addition, design point: what query, where.
   - Rig-proof (reuse ac0109_launch.sh): `defaults write com.matthewsinclair.intent.macos IntentProjectRoot <a real project>`, launch the bundle, trigger a project verb (intent:// -> `intent edit --path`), confirm it resolves THIS project's paths -- and that unconfigured/invalid behave per conditions (i)/(ii).
2. **cc-gated ST0064 (01.2/01.6):** Swift side present + correct (Health reads cc's connect-then-lock order, projection above route(), STALE!=ABSENT remedy). Needs cc's live daemon + both false-positive states CONSTRUCTED. Coord cc.
3. **int macos chunk 2 (01.7 signing):** dc owns the devbin pipeline (+ 3 header edits ride it); notarize = hv's ADC. Reconcile verb naming (`int macos sign|notarize|verify` vs `app-*`).
4. **AC-01.4 console (cc-gated):** tailing console on cc's `intent daemon logs`; tail-orphan trap is a CLI-side fix verified vs SIGTERM/SIGINT/SIGKILL SEPARATELY. Not built.
5. **`intent app start|stop|restart`** (NEW user verb, hv): controls the INSTALLED app; new_surface `app` family; coord cc.
6. **Explorer residuals (hv-driven):** hv rebuild + `intent explore`; open Q: does the Lotus menu SELECTION function (on_key has no MENU block)? Mark Option A RULED+LANDED in the artifact.

## Watch-outs -- mechanisms only

1. **A non-test AC closes via `intent ac satisfy`, NEVER an AT; USE THE ROW'S STATED INSTRUMENT.** AC-01.9 named a minimal-env BUNDLE launch, so a component capture test would have been a substitute -- the built bundle was launched + its os_log read. Re-drive + positive-control before satisfying.
2. **parity/tools harness: `bash -n` is NOT a check on the notes/OVERRIDES block** (cc, 2026-08-31). Embedded double quotes inside a double-quoted string close it and turn prose into shell -- syntactically VALID, so bash -n passes while meaning something else. SOURCE it and read stderr; and keep a live pre-existing row as a control (an empty result read as data is the trap; a control that would pass under the broken instrument is decoration).
3. **Concurrent canon writes -- TWO distinct hazards on SHARED threads (ST0056; ST0064 is safe, single writer + thread-level race via apply_envelopes).** (a) GIT level: a `.canon/st/*.json` write lands intermixed with a peer's UNCOMMITTED edit -- `--only` is path-scoped not hunk-scoped, `git add -p` unavailable, so coordinate with the pen, HANDS OFF THE INDEX, let their commit carry both (vc's 337d6451 carried three nodes' rows). (b) STORE level (0206, HIGH): two facade verbs each load a canon snapshot, mutate one field, write the WHOLE record back -- the second's stale snapshot SILENTLY overwrites the first, both writes succeed, NO TRACE. INTERIM DISCIPLINE til hv rules a fix: before a canon verb on a shared thread, ANNOUNCE on the board + COMMIT in the same breath. A satisfy is a one-verb replay if lost.
4. **The macOS app is `native/macos/Intent/` (Swift, xcodegen).** `bin/devbin macos app-build` = unsigned Debug to ~/.local/state (off-tree). IntentCLI = the one shell-out home (binary + env + NOW child CWD); LoginShell = PATH capture; Health.decode consumes `daemon status --format json`. dc owns `int macos`; ic is the builder. Adding a NEW swift file regenerates the tracked Intent.xcodeproj (xcodegen scans by presence) -- prefer same-file where sane.
5. **A dated RULING RECORD is superseded, never edited.** info.md keeps hv's verbatim "3.0.1"; only my prose was named by property. Superseding note is vc's under the pen.
6. **RUSTFMT BEFORE `git add` on rust.** Attachment edits: `intent st attach <ST> <rel> --from <file>` writes store+canon (disk->store is directional; a running intentd auto-ingests). dispatch-table.md is generated. cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` == your set. RE-RUN TOOLS, never quote a peer's count.

## Decisions

- **project-CWD: vc RULED (a) (2026-08-31) -- a per-app-instance configured root, NOT a machine registry.** D07 ratifies a registry ("one intentd/machine, N projects, per-project DBs, REGISTRY") but it is UNBUILT; building it is release scope (hv's), and a second resolution path beside CWD walk-up is 0204's shape. Conditions: (i) validate on set + loud refuse; (ii) mark the store INTERIM for D07 in code. Wiring landed 7e84538a.
- **ST0064 WP-01 = 3/9 (ic).** 01.1+01.8 (9cad4780), 01.9 (d0cec969, stated instrument). Launch/inspect rig built + reusable.
- **AC-09.6 CLOSED (rode vc's 337d6451).** Re-driven 138/60/0, positive-controlled; hands off vc's index; vc's commit carried three nodes' rows.
- **The release is NOT 3.0.1 in fact** (vc: +13 commands/+30 flags, daemon answers on a socket; recommends 3.1.0). hv holds the number; docs name by property; hv's ruling record verbatim.
- **hv 2026-08-31: Option A RULED + LANDED** (f9709004 + ec9e03b9, 123 tui tests). Coloured chip already built; hv saw a stale binary.
- **ST0064 IN the next release** (hv 11:56Z); daemon is ST0056 WP-08's; signing IN (A8 reversed). **`intent app start|stop|restart`** = user verb, distinct from `int macos app-*`.
- **cc: `intent edit <address> --path` is the AC-01.5 door**; openAddress routes through it. Health is a PROJECTION above route(); STALE never unlinks (AC-08.12).
