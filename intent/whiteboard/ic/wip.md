---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-02 16:08Z
status: active
focus: "TUI redesign (WP-17), 7 commits landed and hv has driven it twice. ON THE BOUNCE: build `/settings` -- hv RULED it, full spec in DOING (file is ~/.intent/config.json, SCOPED to an explorer: section, paths relative, refuse-as-a-spelling). Then the vi keymap behind explorer.editing.mode. Then section 2 prose + AC-17.11 + AC-17.13 AT row in ONE vc window. O4 status segments still wait on hv. 215 passed at 4be902e1. RE-MEASURE every figure at pickup."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Localfold 2026-09-02 13:28Z (hv: agreed the redesign, then localfold+compact, then implement on the bounce). Pre-fold board verbatim in git at 07baadc1. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree and the delivered binary lags HEAD.**

## DOING

**TUI REDESIGN (WP-17). hv HAS DRIVEN THE BUILD TWICE AND IT WORKS; WHAT REMAINS IS `/settings`, THEN vi, THEN THE SECTION 2 PROSE.** Accepted design: `intent/st/ST0056/tui-redesign-proposal.md` (7b08e8b1). **hv's GOVERNING DIRECTIVE: change any AC/AT needed, DO NOT yak-shave old guards or tests -- rewrite them to reality** (fail-forward; [[feedback_fail_forward_intent]]).

**LANDED (re-measure before citing):** `a8981480` mode collapse to OMNI/MENU/EDIT + section 3/4/5 + AC-17.9 reworded + AT-17.9 appended. `2f0ba5e8` the COMMAND PALETTE -- hv drove the build and found MENU was a painted mockup (no model at all; the arrows SILENTLY moved the body behind the bar; `Hotkey` was declared, emitted, reachable and consumed by nothing while every invariant passed). `52b5acf3` vc minted `AC-17.13`; `95abeab5` its second side; `de5401c3` the dead-key invariant FIXED after vc aimed 8x backwards at it. `02e802ae` the framed composer (O1). `94cd1047` repaint-after-edit + the dropdown's rule. `4be902e1` ordinary terminal editing in the composer.

**NEXT -- `/settings`, RULED BY hv 2026-09-02 AND AGREED IN FULL:**

1. **The file is `~/.intent/config.json`** (NOT the project config): this is a global explorer setting. `intentsvcs::userstate::global_config()` already resolves the path -- **there is no new config system to build**, only a reader/writer. Nothing writes that file today.
2. **`/settings` is SCOPED TO AN `explorer:` SECTION and shows/edits only that.** The reason is not tidiness: the file holds `intent_version` (a MIGRATION MARKER) and `intent_dir` (structural), so an unscoped settings view would render rows whose edit must not happen -- the palette's own rule (offer only what can be acted on) one surface over, and worse, because those rows DO have a realiser and it breaks the install.
3. **Paths resolve RELATIVE to `explorer.`** -- `/settings editing.mode`, never `/settings explorer.editing.mode`. One resolution rule, not two.
4. **`/settings author` must be REFUSED AS A SPELLING** (section 8's existing rule), saying what was tried and that `/settings` governs the explorer section -- which teaches the scope instead of reading as broken.
5. **The first setting is `explorer.editing.mode`: `emacs` (default) or `vi`.** The section is not in the file yet -- it is created on first write.
6. **TWO THINGS FLAGGED TO hv AS POSSIBLY AWKWARD, surface rather than guess if they bite:** the palette has NO ARGUMENT SUPPORT (commands are name-only, so `/settings x.y` is a real extension to the vocabulary), and a settings row is **the first thing in the TUI that writes OUTSIDE the store**, so it needs its own write path rather than borrowing the facade's.

**THEN: the vi keymap, behind that setting.** `keys::edit` is the emacs map and is what the default resolves to. **`set -o vi` IS NOT DETECTABLE FROM A CHILD PROCESS -- measured, not assumed:** `SHELLOPTS` is bash-only and absent under zsh, nothing in the environment carries it, `~/.inputrc` is readline's file which zsh never reads. Declared-not-detected is ST0037's ruling one surface over.

**THEN: the section 2 half.** `tui-design.md` section 2 prose, `AC-17.11` reworded, and `AC-17.13`'s AT row -- **ALL THREE IN ONE vc WINDOW, on request**. The AT row must say the evidence lives in TWO tests in two files (`every_trigger_the_machine_answers_is_acted_on_by_the_realiser` and `every_offered_command_is_reachable_by_its_name_and_actually_does_something`), because the property is two-sided and a row citing one looks covered. **The STATUS SEGMENTS (O4) still wait on hv's data-source ruling** -- branch, diff, gate and binary-currency have no source the TUI can reach; vc recommends a facade seam over shell-outs in `run.rs`.

**COUPLING, MEASURED:** `mode.rs`'s transcription test READS `tui-design.md` section 3 FROM DISK at test time, so section 3 and EDGES are gate-locked into one commit. `AC-17.11`'s tests do NOT read the doc -- they assert an in-test shape -- so section 2 and `layout.rs`/`draw.rs` are coupled only by the criterion's prose. **The composer's editing keys needed NO canon window at all**: they are all `Typing`, which changes the buffer and not the mode, so the machine table was untouched.

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

7. **A MARKDOWN ATTACHMENT HAS TWO WRITERS AND THEY RUN AT DIFFERENT TIMES -- ORDER: prettier FIRST, then let the store settle, THEN verify, THEN commit.** Hit live landing `tui-design.md` at a8981480: I wrote the file, the running intentd ingested it (store == disk), then **the pre-commit gate REFUSED on unformatted markdown** and prettier rewrote the file AFTER the store had taken my version -- the [[project_formatter_is_a_second_writer]] divergence arriving in the window between sync and commit. The watcher re-ingested and I verified THREE ways before committing (committed extract == store == disk sha). **AND RE-DRIVE ANY TEST THAT PARSES THE FILE: prettier ALIGNS markdown tables**, so it rewrote the very §3 table `parse_ratified` reads. It survived only because the parser trims each cell -- "it happened to still parse" is a thing to measure, not assume.

8. **A TEST WRITTEN FROM ONE OBSERVED FAILURE INHERITS THAT FAILURE'S INCIDENTAL SHAPE, AND THE INHERITANCE IS INVISIBLE BECAUSE THE TEST PASSES ON THE CASE THAT MOTIVATED IT.** Live twice today. The dead-key invariant was built from `Hotkey`, whose edge is a SELF-LOOP; a planted dead trigger declaring a MODE CHANGE sailed through, because `on_key`'s tail applies the transition and that alone satisfied "the app changed". **8x generalised (mine, vc's board): ANY STATE THE HARNESS ITSELF MOVES IS NOT EVIDENCE THAT THE THING UNDER TEST MOVED IT** -- and the framework case is the invisible one, since the bookkeeping is nowhere near the code under test. **Cure: a baseline that ALREADY CONTAINS the bookkeeping.** Cheap check: plant a second instance of a DIFFERENT shape and see whether the check still fires. **And the move that finds it is applying a fresh class BACKWARDS to what you wrote before it** -- rare because you only look when you have nothing to celebrate.

## Decisions

- **TUI REDESIGN ACCEPTED by hv 2026-09-02** (all 4 recs; proposal 7b08e8b1). Driving posture RULED: change any AC/AT to match, no yak-shaving old guards/tests. This is the WP-17 divergence rework. Implementation on the bounce -- see DOING.
- **AT-00.11 GREEN, landed 83c512c2** (mode 1 of AC-00.11). Note APPENDED not replaced: original 4028-char guidance verbatim at offset 0 + green evidence -> 8462; vc verified independently at 83c512c2^. Both attachments present, roster carries the row. WP-09 = 6/6 stays.
- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv 2026-09-01, via vc). Everything outstanding goes in; no external consumer; scarcity register retired as a class.
- **TOOLCHAIN CURRENCY: binary 361eff99 BEHIND HEAD** (peer AC-06.11); rebuild owed, NOT mine; routed to vc for hv.
- **ST0064 WP-01 = 5/9 (ic).** 01.1/01.3/01.5/01.8/01.9 done; 01.2/01.4/01.6/01.7 cc/dc/hv-gated. project-CWD wiring; vc RULED per-app-instance root, D07 registry UNBUILT.
- **WP-17 STOOD DOWN status is SUPERSEDED by the redesign** -- 17.1 was unsatisfiable-as-written in 3.0.x (D56 equalises downward, EmptyMutation); the redesign reopens the WP-17 rows under the new design, so re-measure WP-17 against the reworded AC-17.9/17.11 once landed. 17.6 still blocked on cc's WP-08 browser-open stub.
- **ST0065 has ZERO ACs** (empty contract, hv-owned). WP-14 (whiteboard+inboxes in the store, L) MAY return from ST0069 -- hv's, not mine.
