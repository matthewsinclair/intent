---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-08-31 18:51Z
status: active
focus: "hv afk, vc holds the pen, cracking on. SHIPPED THIS SESSION: explorer design doc (hv ruled Option A), AC-09.6 CLOSED (my narrow f435cc10 + cc's refusal ea68dddd), and the Option-A /-ring (f9709004, 123 tui tests green). NEXT explorer increment: modeline legibility (draw.rs, doc section 6). Delivered binary now behind HEAD (my rust change) -- dvb build all to refresh."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Fourteenth fold (2026-08-31 18:18Z). Pre-fold verbatim + sha-verified at `.history/20260831/wip-fold-1818Z.md` (c26c0a94). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this tree and the delivered binary lags HEAD / can vanish (watch-out 14). Coordinated by vc; hv drives ST0064 + the explorer directly.**

## DOING

**THE EXPLORER BUILD (hv: "let's build it!").** Design doc delivered + validated (https://claude.ai/code/artifact/16b628cb-62fa-4e7d-98c1-82c719e3994e). **Option-A /-ring LANDED (f9709004, 123 tui tests):** mode.rs EDGES (NAV /->OMNIBOX, MENU /->NAV) + keys.rs (Menu /) + tui-design.md section 3/4 + re-attach; app.rs needed NO change (realiser generic over mode::step); Esc toggle + empty-buffer guard unchanged. **MODELINE: the coloured mode chip was ALREADY built** (draw.rs `Role::ModeChip`, per-mode colour + reversed + bold, landed 2026-08-30, test-proven) -- hv's plain footer in the screenshots was the STALE BINARY, exactly the doc's diagnosis. Added the `/`-ring to the hint legends (run.rs: OMNIBOX "/ menu", NAV "/ omnibox", MENU "/ nav") so the ring hv ruled is DISCOVERABLE (a ring nobody can see is not a feature). AC-09.6 fully CLOSED (my narrow f435cc10 + cc's refusal ea68dddd). **HANDOFF TO hv: rebuild `dvb build all` and try the explorer on a CURRENT binary** -- the Option-A ring + coloured chip + `/` legend are all in HEAD now. WATCH: the realiser's on_key has no MENU block, so verify menu SELECTION actually functions -- may be the deeper half of hv's "not great to use", worth checking on the rebuild.

## TODO

1. **BUILD THE EXPLORER (WP-17)** -- the realiser over the ratified machine, per hv's ruling. FIRST assess what exists: AC-17.11 (layout) + AC-17.12 (explore-opens-at-top) are `satisfied: yes`, so a realiser exists in part; `tui/mode.rs` is the DECLARED, invariant-tested machine. The Option-A change to `mode.rs`: `NAV +/ -> OMNIBOX` (was `-> MENU`), `MENU` gains `+/ -> NAV`, keep the empty-buffer guard on the OMNIBOX->MENU leg; MENU arrows already present. Then the modeline legibility pass (coloured mode chip + named exits -- before/after is in the doc section 6). My two criteria: **AC-17.1** (one declaration, two realisers, same model change; AT-17.1 to write) and **AC-17.6** (edit + browse reach one model through one service; the `intent browse` arm is DECLARED but not wired -- build it). Ground in `tui-design.md` 3-9 + `mode.rs`.

2. **LAND AC-09.6 (THIS session -- hv un-wrapped; method PROVEN, reverted only for the since-cancelled wrap).** cc holds the loud refusal on MY signal; order is MINE-then-cc (cc's refusal reds `mcp_surface.rs:71` at their commit, theirs to fix). Steps:
   - **Re-flip the 21** (jq, verified 81->60 exposed / 0 exposed-no-facade): over `families[].entries[]` AND `new_surface[]`, `select(exposed_on_mcp==true and no-facade) |= .exposed_on_mcp=false`.
   - **recoverability_anomaly on EXACTLY the 13 non-retire mutate rows.** Selector MUST mirror the gate's arm-1 population (`gen_dispatch_table.sh`): `read_or_mutate==mutate AND (.disposition//"")!=retire AND (.target.state//"")!=retire AND (reversible|idempotent) AND exposed==false AND no-anomaly`. WITHOUT the retire exclusion it over-matches `issues hydrate`/`issues dehydrate`/`lang sync` (16 not 13) -- the vacuous-selector class. Self-expiring (arm 2 refuses a stale anomaly). The 13: agents sync, claude hook, claude rules, claude skills, claude ws, fc, lang init, lang remove, llm usage_rules, sync, todo done, todo notdone, todo toggle.
   - **Re-append the parity disposition section** (saved verbatim at `scratchpad/parity-disposition.bak.md`) to `intent/st/ST0056/parity/ac-09_6-mcp-facade-read.md`.
   - **Regen** `surface/dispatch-table.md` via `intent/st/ST0056/parity/tools/gen_dispatch_table.sh` (`view_skew_check` blocks otherwise).
   - **Re-attach** (attachment drift): `intent st attach ST0056 parity/ac-09_6-mcp-facade-read.md --from intent/st/ST0056/parity/ac-09_6-mcp-facade-read.md`. Only writer that fixes canon; NO sync direction does.
   - **Commit** JSON + dispatch-table.md + parity doc + `intent/.canon/st/ST0056.json` together, then **signal cc**. cc's half: swap the two checks in `tools()` so `exposed_on_mcp` is asked first, return `Err(Undeclarable { path, why })` for exposed-no-facade.

3. **`intent app start|stop|restart`** (NEW user-facing Rust verb, hv 2026-08-31) -- controls the INSTALLED `/Applications/Intent.app` (open + osascript quit). new_surface `app` family; coord cc on dispatch-table entry + handler. Distinct from `int macos app-*` (dev pipeline).

4. **int macos chunk 2** -- app-sign + app-notarize; ALL THREE artefacts signed+notarized (hv: `.app` same as Conflab; `~/Devel/prj/Conflab/bin/.devbin/cmd/macos` = bundle-walk + stapling ref). dc's 3 header-truth edits ride the SAME commit as app-sign. Reuse cmd/macos kc_get/IDENTITY/notarytool. Read the bundle's EMBEDDED marker, never the manifest. Notarize = hv's ADC. Reconcile verb naming: AC-01.7/AC-11.1 cite `int macos sign|notarize|verify` vs my `app-*` prefix.

5. **ST0064 greening** (app IN 3.0.1; info.md line 17). Distance delivered to vc: verify 3 mine (01.1/01.8/01.9), verify 2 cc-gated (01.2/01.6 -- need cc daemon status + three-state projection + live daemon + AT-01.6), wire project-CWD (unblocks 01.3 one real graphql query + 01.5 resolve), chunk-2 signing (01.7, hv ADC), build console (01.4, gated cc daemon logs). Read `ST0064/design-menubar-app.md` (Geodica handoff, hv flagged unread) before app work.

6. **Minor:** update the explorer artifact to mark Option A RULED + fix the MENU-arrows understatement (doc section 3 said "letter picks"). AC-09.4 clause-2 CLASS check (coord cc). guide.rs rc=2 residual (dc's reconciled census in `inbox.dc.md`). AC-09.2 (`intent_graphql` unowned -- vc's scope Q to hv).

## Watch-outs -- mechanisms only

1. **recoverability -> exposure invariant (`gen_dispatch_table.sh` arm 1).** Narrowing an UNDOABLE mutation off MCP (exposed:false while reversible/idempotent) REQUIRES a `recoverability_anomaly` saying why; SELF-EXPIRING (arm 2 refuses a stale one). The check's population EXCLUDES `disposition/target.state == retire` -- mirror that in any selector or it over-matches retiring rows.
2. **ATTACHMENT DRIFT.** Editing a file under `intent/st/<ST>/` directly leaves canon naming OLD bytes -> `canon_commit_check` blocks. `intent st attach <ST> <rel-path> --from <file>` is the ONLY fix (store + extract in one step); NO sync does it. Commit file + `intent/.canon/st/<ST>.json` together.
3. **surface/dispatch-table.md is GENERATED** from the .json via `gen_dispatch_table.sh` -- regen on any .json change.
4. **`git restore <path>` restores worktree FROM THE INDEX** -- a `git add`ed file stays. Fully reset staged->HEAD: `git restore --source=HEAD --staged --worktree <path>` (hit this session).
5. **`daemon status --format json`: keys ALPHABETICAL** (endpoint before state on live); the three literals are cc's tripwire, mirrored in HealthTests. Rebuild intentd + debug intent before driving daemon-backed behaviour.
6. **`intent://` canonical form is THREE slashes** (`intent:///threads/ST0000`); two -> refused.
7. **`intent explore` NEEDS A TERMINAL** -- pipe-safe door is `intent edit <addr> --path`. edit + graphql resolve project by CWD walk-up; the app must set child CWD.
8. **A FILTERED test run cannot report a verdict -- capture cargo's own rc.** **xcodegen scans sources BY PRESENCE** -- regen .xcodeproj on source-set change. **A CROSS-CRATE GUARD is invisible to `cargo test -p <crate>`** -- run `-p intentsvcs` before address-touching landings.
9. **cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` must EQUAL your intended set.** List paths explicitly. NEVER remove a peer's index.lock -- WAIT (hit + waited this session).
10. **MARKDOWN PRETTIER BEFORE `git add`. RE-RUN TOOLS, never quote a peer's count.** **rc=127 + suppressed stderr = "ran and answered nothing"** -- check EXIT CODE; no `2>/dev/null` driving `intent`.
11. **The delivered binary lags HEAD and can VANISH** -- `~/.local/bin/intent` symlinks into target/release, no currency check. Drive `target/debug/intent` or check `instrument_currency_check.sh`; NEVER assert shipped behaviour from a date.

## Decisions

- **hv 2026-08-31: RULED Option A for the explorer.** `/` cycles NAV->OMNIBOX->MENU (empty-buffer guarded), Esc = back, never quits. MENU arrows confirmed (already in the machine). Modeline legibility fix approved. -> BUILD (TODO-1).
- **AC-09.6 (hv 2026-08-31 15:54Z): narrow the table AND make the drop loud.** Owner split: cc the refusal (mcp.rs:97), ic the 21 table dispositions. Measured 81/60/21. Disposition: all 21 -> exposed:false; 4 namespace + 7 read + 10 mutate; re-add candidates flagged post-tag. Narrow lands first, cc's refusal on top.
- **ST0064 is IN 3.0.1** (hv 11:56Z correction, info.md line 17). vc reconciled hv's board line 30 + a stale GraphQL clause. Signing IN (A8 reversed). Distance-to-tag delivered to vc (built != verified, not nine from scratch).
- **hv 2026-08-31: `intent app start|stop|restart`** = user-facing verb for the menubar app lifecycle, distinct from `int macos app-*`.
- **TUI (hv 2026-08-30): COMMAND folded into OMNIBOX; OMNIBOX is the rest state; Esc never quits.** Option A extends this for the `/` key (cycle rather than direct-to-menu).
- **cc: `intent edit <address> --path` is the AC-01.5 resolver door** (pipe-safe, no tty, no kind). App speaks addresses only.
- **cc: health is a PROJECTION above `route()`** (Route stays two variants); state is the remedy; gate unlink on absent; stale/absent split on the kernel lock.
- **vc: app project model = a single configured project storing the ROOT PATH only** (registry's root); child CWD for project-scoped cmds; daemon control machine-level. Cheap to overturn.
- **dc: `int macos` app pipeline = option A** + 3 header-truth edits + read the bundle marker. Chunk 1 landed (d00956ec).
