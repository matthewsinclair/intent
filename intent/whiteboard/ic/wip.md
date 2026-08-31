---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 20:16Z
status: active
focus: "Fifteenth fold 18:58Z. hv afk, vc pen, HOLDING for hv. SHIPPED: explorer design doc (hv ruled Option A), AC-09.6 CLOSED (f435cc10 + cc's ea68dddd), Option-A /-ring COMPLETE (ring f9709004 + hint legend ec9e03b9, 123 tui tests). Modeline chip was already built -- hv saw a stale binary. HANDOFF: hv rebuild `dvb build all` + try `intent explore` on a current binary."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Fifteenth fold (2026-08-31 18:58Z). Pre-fold verbatim + sha-verified at `.history/20260831/wip-fold-1858Z.md` (68d9d576). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree; the delivered binary lags HEAD (my rust landed, binary behind -- `dvb build all` refreshes). Coordinated by vc; hv drives the explorer + ST0064 directly.**

## DOING

**HOLDING for hv (afk, vc holds the pen).** The explorer's Option-A work is COMPLETE and the ball is in hv's court:

- **HANDOFF -> hv: `dvb build all`, then try `intent explore`** on a CURRENT binary. The Option-A `/`-ring, the coloured mode chip, and the `/` hint legend are all in HEAD; the delivered `~/.local/bin/intent` is behind it (my rust change). hv's "not great to use" was largely the STALE BINARY -- the doc's own diagnosis, now confirmed.
- **ONE open explorer question for the rebuild:** the realiser's `on_key` (app.rs) has NO MENU block -- menu Hotkey/Move/Enter go through the generic path, and where the menu's SELECTION state lives is unclear. Verify the Lotus menu actually FUNCTIONS (letter-picks, drilling) when hv drives it; may be the deeper half of "not great to use". Not yet investigated.

## TODO

1. **Explorer residuals (after hv tries the rebuild):** the menu-selection question above; and any UX gaps hv finds on a current binary. The Option-A machine + chip + hint legend are landed + tested; further work is hv-directed from real use.
2. **`intent app start|stop|restart`** (NEW user-facing Rust verb, hv 2026-08-31) -- controls the INSTALLED `/Applications/Intent.app` (open + osascript quit). new_surface `app` family; coord cc on the dispatch-table entry + handler. Distinct from `int macos app-*` (dev pipeline).
3. **int macos chunk 2** -- app-sign + app-notarize; ALL THREE artefacts signed+notarized (hv: `.app` same as Conflab; `~/Devel/prj/Conflab/bin/.devbin/cmd/macos` = the bundle-walk + stapling ref). dc's 3 header-truth edits ride the SAME commit as app-sign. Reuse cmd/macos kc_get/IDENTITY/notarytool. Read the bundle's EMBEDDED marker, never the manifest. Notarize = hv's ADC (human step). Reconcile verb naming: AC-01.7/AC-11.1 cite `int macos sign|notarize|verify` vs my `app-*` prefix.
4. **ST0064 greening** (app IN 3.0.1; info.md line 17). Distance delivered to vc: verify 3 mine (01.1/01.8/01.9), verify 2 cc-gated (01.2/01.6 -- need cc daemon status + three-state projection + live daemon + AT-01.6), wire project-CWD (unblocks 01.3 one real graphql query + 01.5 resolve), chunk-2 signing (01.7, hv ADC), build console (01.4, gated cc daemon logs). Read `ST0064/design-menubar-app.md` (Geodica handoff, hv flagged unread) before app work.
5. **Minor:** update the explorer design artifact to mark Option A RULED + LANDED (it currently reads "recommended"; https://claude.ai/code/artifact/16b628cb-62fa-4e7d-98c1-82c719e3994e). AC-09.4 clause-2 CLASS check (coord cc). guide.rs rc=2 residual (dc's reconciled census in `inbox.dc.md`). AC-09.2 (`intent_graphql` unowned -- vc's scope Q to hv).

## Watch-outs -- mechanisms only

1. **RUSTFMT BEFORE `git add` on any rust** -- the gate REFUSES an unformatted rust file (rewrites nothing). `rustfmt --edition 2024 <files>`, then add + commit. Hit this session (a doc-comment edit tripped it).
2. **recoverability -> exposure invariant (`gen_dispatch_table.sh` arm 1).** Narrowing an UNDOABLE mutation off MCP (exposed:false while reversible/idempotent) REQUIRES a `recoverability_anomaly` saying why; SELF-EXPIRING (arm 2 refuses a stale one). The check's population EXCLUDES `disposition/target.state == retire` -- mirror that in any selector or it over-matches retiring rows.
3. **ATTACHMENT DRIFT.** Editing a file under `intent/st/<ST>/` directly leaves canon naming OLD bytes -> `canon_commit_check` blocks. `intent st attach <ST> <rel-path> --from <file>` is the ONLY fix (store + extract in one step); NO sync does it. Commit file + `intent/.canon/st/<ST>.json` together. (tui-design.md + the parity doc are both ST0056 attachments.)
4. **surface/dispatch-table.md is GENERATED** from the .json via `intent/st/ST0056/parity/tools/gen_dispatch_table.sh` (IN/OUT env) -- regen on any .json change.
5. **`git restore <path>` restores worktree FROM THE INDEX** -- a `git add`ed file stays. Fully reset staged->HEAD: `git restore --source=HEAD --staged --worktree <path>`.
6. **The TUI is generic over `mode::step`** (app.rs:381 `arm(steps(...))`) -- a mode-graph change in `mode.rs` EDGES + `tui-design.md` section 3 (transcription test) drives the realiser with NO app.rs change. keys.rs::trigger only NAMES the trigger. The two `/` guards (empty-buffer in app.rs:273, pane focus) are app-side, not machine.
7. **`daemon status --format json`: keys ALPHABETICAL**; three literals are cc's tripwire, mirrored in HealthTests. Rebuild intentd + debug intent before driving daemon-backed behaviour.
8. **`intent://` canonical form is THREE slashes**; **`intent explore` NEEDS A TERMINAL** (pipe-safe door `intent edit <addr> --path`); edit + graphql resolve project by CWD walk-up so the app must set child CWD.
9. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your intended set.** NEVER remove a peer's index.lock -- WAIT (hit + waited twice this session). **A FILTERED test run cannot report a verdict** -- capture cargo's own rc. **`cargo test -p intent-cli` in a private CARGO_TARGET_DIR fails daemon_and_local_agree/dual_path_conformance** unless `cargo build -p intentd` into the same dir (cc's tip); scope to `tui::` to dodge it.
10. **RE-RUN TOOLS, never quote a peer's count** (cc's independent census read 61 vs my correct 60 -- a walk that counts a flag as a command row). **rc=127 + suppressed stderr = ran-and-answered-nothing.** **The delivered binary lags HEAD + can vanish** -- drive `target/debug/intent` or check `instrument_currency_check.sh`.

## Decisions

- **hv 2026-08-31: RULED Option A + it LANDED.** `/` cycles NAV->OMNIBOX->MENU->NAV (empty-buffer guarded), Esc = the non-cyclic way back (toggle unchanged), never quits. mode.rs EDGES + keys.rs + tui-design.md 3/4 = `f9709004`; the `/` hint legend (OMNIBOX "/ menu", NAV "/ omnibox", MENU "/ nav") = `ec9e03b9`. 123 tui tests. MENU arrows already in the machine. The coloured mode chip was already built (2026-08-30); hv's plain footer was a stale binary.
- **AC-09.6 CLOSED (hv ruled narrow + loud, 15:54Z).** ic narrowed the 21 to exposed:false (`f435cc10`, 13 recoverability anomalies, parity doc re-attached); cc landed the loud refusal (`ea68dddd`, driven on a planted victim). 0 exposed rows lack a facade; mcp_surface stayed green because the narrow landed first.
- **ST0064 is IN 3.0.1** (hv 11:56Z, info.md line 17). vc reconciled hv's board line 30 + a stale GraphQL clause. Signing IN (A8 reversed). Distance-to-tag delivered to vc (built != verified, not nine from scratch).
- **hv 2026-08-31: `intent app start|stop|restart`** = user-facing verb for the menubar app lifecycle, distinct from `int macos app-*`.
- **cc: `intent edit <address> --path` is the AC-01.5 resolver door** (pipe-safe, no tty, no kind). App speaks addresses only.
- **cc: health is a PROJECTION above `route()`** (Route stays two variants); state is the remedy; gate unlink on absent; stale/absent split on the kernel lock.
- **vc: app project model = a single configured project storing the ROOT PATH only** (registry's root); child CWD for project-scoped cmds; daemon control machine-level. Cheap to overturn.
- **dc: `int macos` app pipeline = option A** + 3 header-truth edits + read the bundle marker. Chunk 1 landed (d00956ec).
