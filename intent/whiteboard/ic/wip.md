---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-02 14:10Z
status: active
focus: "IMPLEMENTING the TUI redesign (WP-17). MODE-MODEL HALF BUILT AND GREEN: 195 passed, 1 failed, and the one red is the DESIGNED coupling -- mode.rs EDGES is ahead of tui-design.md section 3 and goes green when the design lands. NAV folded into the composer; OMNI/MENU/EDIT. UNCOMMITTED, 7 files. vc GRANTED the canon window 14:09Z and RATIFIED my section-3 spelling correction. NEXT: section 2-5 design write + AC-17.9/AC-17.11 in the window, then commit 1; then the lower-bar half. RE-MEASURE every figure at pickup."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Localfold 2026-09-02 13:28Z (hv: agreed the redesign, then localfold+compact, then implement on the bounce). Pre-fold board verbatim in git at 07baadc1. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree and the delivered binary lags HEAD.**

## DOING

**ON THE BOUNCE: implement the TUI redesign as ONE coordinated change.** The accepted design is `intent/st/ST0056/tui-redesign-proposal.md` (committed 7b08e8b1). hv accepted all four open calls: **O1** framed composer (the one bordered element), **O2** keep `:q`/`:w`/`:q!` as hidden aliases advertising only `/`, **O3** Esc-on-empty = no-op, **O4** the binary-currency status segment stays in. **hv's GOVERNING DIRECTIVE: change any AC/AT needed to drive this forward, and DO NOT spend time yak-shaving around old guards or tests -- rewrite them to match the reality of what we are doing** (fail-forward; [[feedback_fail_forward_intent]]). The change, three coupled parts:

1. **`tui-design.md` §2-§5** -- §2 foot becomes framed composer + status-segment row + mode/hint line (retire two-rules-no-borders; KEEP the two-aligned-columns alignment guarantee and the pure-data seam); §3 mode machine becomes OMNI/MENU/EDIT (kill the 3-way `/` ring AND the Esc toggle; fold NAV to a buffer-empty focus guard; `/` opens MENU in one press; Esc = back-to-composer); §4 keys; §5 one-press `/`.
2. **Code** -- `mode.rs` (Mode enum drops Nav; EDGES = the proposal's new table; invariant + transcription tests rewritten), `keys.rs` (no ring, `/`-one-press, Esc semantics, buffer-guarded arrows), `layout.rs` (FOOT grows; status-segment section; composer-box allowance; degradation order re-examined), `draw.rs` (box-border role + palette; relax the no-border assertion to only-the-composer-is-framed), `omnibox.rs` (`/` -> command palette; `:` aliases).
3. **Criteria via vc's canon window (GRANTED for the bounce, on request not queued -- PING vc when there).** Reword **AC-17.11** (five-sections/two-rules -> composer+segments+mode-line) and **AC-17.9** (5-mode + Esc-toggle -> OMNI/MENU/EDIT + Esc-to-home + one-press `/`), plus their AT rows/tests. **vc WINDOW CONDITIONS (13:29Z), all bounce-binding:** (a) every reworded row KEEPS its superseded wording INLINE, naming the ratified design that replaced it -- AC-17.11 already does exactly this (its `THE FOOT IS THE RATIFIED DESIGN'S, NOT THIS ROW'S ORIGINAL WORDING` line); the discriminator is design-premise-MOVED (legit, what this is) vs implementation-FAILED (laundering), indistinguishable after the fact from the row alone, and it composes badly with B2 (a green is a stored string nothing re-checks) unless each row carries that one sentence. (b) AC-17.9's `ESC is total` invariant NARROWS -- carry EMBED IN as the named exemption, never drop `total` silently (that retires a safety property by omission). (c) O3 is a behaviour RETIREMENT -- state it AFFIRMATIVELY (`Esc on an already-empty composer is a no-op because you are home`), not as an absent clause. (d) any AT `--note`: APPEND never replace (Watch-out 1). AC-17.4 / 17.10 / 17.12 UNAFFECTED; **17.1 UNAFFECTED but STILL LIVE + unruled (hv B1) -- do NOT read unaffected as closed.**

**Coupling watch:** `mode.rs`'s transcription test parses §3, so §3 and `mode.rs` EDGES land together; `AC-17.11`'s tests pin §2's shape, so `layout.rs`/`draw.rs` land with the reworded criterion. Pure code files commit directly (mine); `tui-design.md` + criteria go through vc's canon window (vc informed, expecting it). Size ~L. Nothing else of mine in flight.

## TODO

1. **IMPLEMENT THE TUI REDESIGN** -- above. Ping vc for the canon window when at the design/criteria writes.
2. **ST0064 remaining (5/9), not mine to start:** 01.2/01.6 cc-gated; 01.7 dc pipeline + hv's ADC signing; 01.4 cc console.
3. **Explorer/Lotus menu SELECTION (hv item, A1 on vc's list)** -- needs hv at the keys; not answerable here.
4. **`intent app start|stop|restart`** (hv, new user verb): controls the INSTALLED app; new_surface `app` family; coord cc.

## Watch-outs -- mechanisms only

1. **Non-test AC closes via `intent ac satisfy`, NEVER an AT; AT rows close via `at green --note`.** Re-drive + positive-control before satisfying. **`at green --note` REPLACES the note WHOLESALE and silently** -- read the WHOLE existing note first and APPEND (0207/8c-bis; it ate AT-00.12's 7803 bytes once, and AT-00.11 would have been the second time this session had I not re-measured). Source the existing note from the JSON extract (`jq -rj`), never `acceptance.md` (prettier is a second writer + the row carries a 114-char prefix). Assert contains-original-verbatim AND longer AFTER the write.
2. **Canon on SHARED threads: SERIALISE ST0056 writes through vc; gather evidence OUTSIDE the window, make the window ONE coherent commit.** 0206 (concurrent facade verbs clobber), 0210 (a dirty parity/tools file blocks EVERY node's canon commit via the extract union; the guard TRIAD declared_kind/runner_roster/stale_at has NO passing partial state for a new parity file, and it is REPO-WIDE -- restoring a file to disk mid-landing red-gated the whole tree this session until the coherent commit cleared it), 0212 (watcher ingest reverts a store write when disk lags; verify DISK not store before commit).
3. **The macOS app is `native/macos/Intent/` (Swift, xcodegen, folder-GLOBBED).** `bin/devbin macos app-build` unsigned Debug off-tree; `app-test` the suite. IntentCLI = the one shell-out home. NOT `dvb test rust`.
4. **A value/claim carried ONE STEP past what it supports is the recurring class, and it is MINE too.** Hand/take PROPERTIES not values; re-measure at pickup; a retraction carries the same burden as the claim it retracts.
5. **`git commit --only <explicit paths>` is the only safe write** on this 5-writer tree (git add new files first; committed exactly my paths twice this session while cc/dc/vc boards sat staged in the index -- do NOT sweep them). NEVER remove a peer's index.lock (WAIT+retry). cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` == your set. Every stamp from a `date -u` read THIS turn.
6. **`intent --version` (commit the BINARY was built from) vs currency (`intent --version` vs `git rev-list -1 HEAD -- native/rust surface docs/design`).** Binary 361eff99 is BEHIND HEAD; the gate's currency arm REFUSES; rebuild owed, NOT mine (racing sessions is 0196/ST0058). A display filter over a tool's own diagnostic is a population filter.

## Decisions

- **TUI REDESIGN ACCEPTED by hv 2026-09-02** (all 4 recs; proposal 7b08e8b1). Driving posture RULED: change any AC/AT to match, no yak-shaving old guards/tests. This is the WP-17 divergence rework. Implementation on the bounce -- see DOING.
- **AT-00.11 GREEN, landed 83c512c2** (mode 1 of AC-00.11). Note APPENDED not replaced: original 4028-char guidance verbatim at offset 0 + green evidence -> 8462; vc verified independently at 83c512c2^. Both attachments present, roster carries the row. WP-09 = 6/6 stays.
- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv 2026-09-01, via vc). Everything outstanding goes in; no external consumer; scarcity register retired as a class.
- **TOOLCHAIN CURRENCY: binary 361eff99 BEHIND HEAD** (peer AC-06.11); rebuild owed, NOT mine; routed to vc for hv.
- **ST0064 WP-01 = 5/9 (ic).** 01.1/01.3/01.5/01.8/01.9 done; 01.2/01.4/01.6/01.7 cc/dc/hv-gated. project-CWD wiring; vc RULED per-app-instance root, D07 registry UNBUILT.
- **WP-17 STOOD DOWN status is SUPERSEDED by the redesign** -- 17.1 was unsatisfiable-as-written in 3.0.x (D56 equalises downward, EmptyMutation); the redesign reopens the WP-17 rows under the new design, so re-measure WP-17 against the reworded AC-17.9/17.11 once landed. 17.6 still blocked on cc's WP-08 browser-open stub.
- **ST0065 has ZERO ACs** (empty contract, hv-owned). WP-14 (whiteboard+inboxes in the store, L) MAY return from ST0069 -- hv's, not mine.
