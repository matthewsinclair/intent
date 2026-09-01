---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-01 18:00Z
status: active
focus: "HOLD RELEASED hv-DIRECT ('devbin-vc is done. The hold is released. Please wait for instructions from intent-vc'). Freeze off; hv delegated my sequencing to vc -- I do NOT self-start ST0064, I await vc's word. Toolchain CURRENT (re-confirmed 18:00Z after a peer rebuild produced a same-commit binary): binary @ 361eff99 == newest build input @ 361eff99 (git rev-list -1 HEAD -- native/rust surface docs/design). Next when vc directs: ST0064 app pair (01.3/01.5). Compare intent --version to the CURRENCY command, NEVER to bin/devbin --version (both print 3.0.0 (<sha>) for different quantities)."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Cold start (2026-09-01 17:07Z, aggressive checkin+localfold+compact on hv's word, holding for the devbin-vc bounce). Pre-fold verbatim + sha-verified at .history/20260901/wip-fold-1707Z.md (sha 315d1543). RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree, a devbin rollout is moving build paths, and the delivered binary can lag HEAD.**

## DOING

**HOLD RELEASED (hv-DIRECT) -- awaiting vc's instructions per hv.** hv, first-hand this session, verbatim: _"devbin-vc is done. The hold is released. Please wait for instructions from intent-vc."_ So the freeze is off AND hv has delegated my next-work sequencing to vc -- I do NOT self-start ST0064; I stand by for vc's word, which now carries hv's explicit delegation (a peer relay would NOT have, and my earlier caution held until this direct word arrived). The devbin rollout is committed (`03d40d30 Re-vendor devbin to e39fba7`); `native/rust` clean (only dirty paths are peers' canon + the devbin vendor manifest, none mine). **Toolchain re-measured myself, CURRENT**: `intent --version` @ 361eff99 == `git rev-list -1 HEAD -- native/rust surface docs/design` @ 361eff99 -- re-confirmed 18:00Z after a peer's rebuild cycle briefly removed the dev symlink (W28) and returned a same-commit binary; HEAD moved to 7f40d6f2 by wb/canon/vendor only. I drove the value rather than swallowing vc's counter-value (the same-failure-reversed rule).

## TODO

1. **ST0064 app pair -- AC-01.3 (M) + AC-01.5 (S)** (HELD by the stop: both need an app build). Wiring landed 7e84538a. 01.3: add ONE real query through `intent graphql` to the app (today it uses `daemon status --format json`) -- executor RE-DRIVEN real in-process (`intent graphql '{threads{id status}}'` -> `{"data":...}` rc=0), NOT blocked on cc's daemon. 01.5: intent:// -> `intent edit --path` resolver. ONE `bin/devbin macos app-build` + ONE bundle rig (scratchpad/ac0109_launch.sh: `defaults write com.matthewsinclair.intent.macos IntentProjectRoot <a real project>`, launch, trigger a project verb, confirm resolution + conditions (i)/(ii)) proves both. While in IntentCLI, sharpen the D07 comment to "the disagreement will never announce itself" (0206 framing). ST0064 is single-writer + separate from ST0056, so NO vc window needed.
2. **cc-gated 01.2/01.6:** Swift side present + correct (Health reads connect-then-lock; projection above route(); STALE!=ABSENT remedy). Needs cc's live daemon + both false-positive states CONSTRUCTED. Coord cc.
3. **01.7 signing (dc + hv):** app-sign/notarize; dc owns the devbin pipeline + 3 header edits; notarize = hv's ADC. Reconcile `int macos sign|notarize|verify` vs `app-*`.
4. **01.4 console (cc):** tailing cc's `intent daemon logs`; tail-orphan trap CLI-side, verified vs SIGTERM/SIGINT/SIGKILL SEPARATELY.
5. **`intent app start|stop|restart`** (hv, new user verb): controls the INSTALLED app; new_surface `app` family; coord cc.
6. **Explorer (hv-driven):** hv rebuild + `intent explore`; open Q: does the Lotus menu SELECTION function? Mark Option A RULED+LANDED in the artifact.

## Watch-outs -- mechanisms only

1. **Non-test AC closes via `intent ac satisfy`, NEVER an AT; use the ROW'S STATED INSTRUMENT.** AC-01.9 named a minimal-env BUNDLE launch, so a component test would be a substitute. Re-drive + positive-control before satisfying.
2. **Canon on SHARED threads has TWO+ hazards.** (a) 0206 (HIGH): two facade verbs each write the whole record back, second's stale snapshot SILENTLY overwrites first -- announce + commit in one breath. (b) 0210 (HIGH, mine): the `.canon/st/<ST>.json` extract names the STORE's union of every dirty-and-ingested attachment; with intentd running, one node's dirty parity/tools file blocks EVERY node's canon commit on that thread, and committing the attachment WITHOUT the extract fails the other way -- coherent-with-git-across-all-nodes or no commit; the guard names the FILE not the culprit. **DISCIPLINE (vc, until fixed): ST0056 attachment writes SERIALISED through vc -- announce before touching parity/tools.** Even `issues add` regenerates the extract and can dirty it into whoever holds the window.
3. **Attachment drift:** files under intent/st/<ST>/ are ST0056 attachments; edit on disk -> `intent st attach <ST> <rel> --from <file>` re-syncs store+canon (the gate's own prescribed writer) BEFORE commit, then commit the file + `.canon/st/<ST>.json` together in a coherent window. disk->store directional; running intentd auto-ingests. `bash -n` is NOT a check on a notes/table block (0205) -- safe form = quoted-heredoc.
4. **The macOS app is `native/macos/Intent/` (Swift, xcodegen).** `bin/devbin macos app-build` = unsigned Debug to ~/.local/state (off-tree). IntentCLI = the one shell-out home (binary + env + child CWD); LoginShell = PATH capture. dc owns `int macos`; ic is the builder. A NEW swift file regenerates the tracked xcodeproj -- prefer same-file.
5. **A dated RULING RECORD is superseded, never edited.** info.md's "3.0.1" is CORRECT AS WRITTEN (hv closed the number at v3.0.1). RUSTFMT before `git add` on rust. dispatch-table.md is generated. cc's POST-VERIFY IS STANDING: `git show --name-only HEAD` == your set. NEVER remove a peer's index.lock (wait); `git commit --only <paths>` is the only safe write; RE-RUN TOOLS, never quote a peer's count; every stamp from a `date -u` read this turn.
6. **A hold leg built on a peer's VALUE is a stale-value trap (mine: 817eb93c, retracted 4680810d).** I made vc's 7-min-stale "binary gone" load-bearing without measuring. PESSIMISTIC-direction staleness self-perpetuates: "the tool is down" makes a hold the rational move, so nobody checks -- an overstatement gets caught, an understatement recruits standing-still. Hand/take PROPERTIES not values; and measure a peer's COUNTER-value too, don't swallow it (that is the same failure reversed).
7. **`intent --version` and `bin/devbin --version` print an IDENTICAL `3.0.0 (<sha>)` for DIFFERENT quantities (devbin-vc, 625f9e20).** `intent --version` = the commit the BINARY was built from; `bin/devbin --version` = what the CHECKOUT is at NOW (+dirty flag). A version comparison BETWEEN the two re-manufactures the #6 stale-toolchain error and looks like evidence. Currency = `intent --version` vs the CURRENCY command (`git rev-list -1 HEAD -- native/rust surface docs/design`), NEVER vs devbin's. Verified 2026-09-01 17:57Z: 361eff99 vs 04f31ad2, both correct.

## Decisions

- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv first-hand 2026-09-01 08:32Z, 3183990a via vc). Everything outstanding goes in; cost not a constraint; no external consumer. The 3.0.1-vs-3.1.0 conditional does NOT fire -- info.md's three 3.0.1 occurrences correct as written. SCARCITY-REGISTER shape ("not in this cut" / "after the tag" / "ship it red") is retired as a class.
- **TOOLCHAIN RE-MEASURED CURRENT 2026-09-01 17:57Z (post-rollout):** binary @ 361eff99 == newest build input @ 361eff99 (`git rev-list -1 HEAD -- native/rust surface docs/design`); HEAD 04f31ad2 ahead by wb/canon/devbin-vendor only; `native/rust` clean. The devbin rollout committed at 03d40d30 did NOT move the currency instrument's path set (devbin-vc reported, verified here). Still a VALUE -- re-drive on any native/rust change. The "broken toolchain" leg (817eb93c) stays retracted; do NOT re-derive it.
- **LANDED this session:** 0205 harness refactor (three tables -> quoted-heredoc; rode vc's combined 11528266, three-way ST0056 canon coherence), estate_corpus re-pin retirement (035114c9; all 4 members declare intent_version 3.0.0), coherence-trap issue 0210 (d7744332, HIGH).
- **ST0064 WP-01 = 3/9 (ic).** 01.1+01.8 (9cad4780), 01.9 by the bundle-launch instrument (d0cec969). project-CWD WIRING landed (7e84538a): IntentCLI sets child CWD to a validated configured root (ProjectConfig), condition (i) loud-refuse + (ii) interim-for-D07. vc RULED (a): per-app-instance configured root, NOT a machine registry (D07's registry UNBUILT).
- **AC-09.6 CLOSED** (rode vc's 337d6451, 138/60/0 positive-controlled). ST0056/09 = 6/6 green; ST0056/17 = 10/12 (17.1/17.6 red on builds not mine + hv's TUI-divergence design conversation, unstarted).
- **ST0065 has ZERO ACs** (empty contract, hv-owned, hv taking it directly). WP-17 rework hv-gated. WP-14 (whiteboard+inboxes in the store, L) MAY return from ST0069 -- vc's reading to hv, ownership hv's, not mine.
- **cc: `intent edit <address> --path` is the AC-01.5 door**; openAddress routes through it. Health is a PROJECTION above route(); STALE never unlinks (AC-08.12).
