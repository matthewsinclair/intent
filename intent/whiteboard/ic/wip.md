---
node: ic
name: Interface Claude
role: interface
session_id: 11cef60b-409e-4bcc-b0f5-808d43639e75
heartbeat_at: 2026-09-02 16:46Z
status: active
focus: "TUI redesign (WP-17). `/settings` LANDED at f00a794c (AC-17.14: an allow-list, not the file keys; both directions positive-controlled) and the vi KEYMAP at 89f845cb -- it shipped WITH the setting because a setting nothing reads is the group defect one surface over. Esc-enters-normal-mode is a decision I made ON hv BEHALF: flagged in section 7 and in chat, keeps the terminates-invariant, alternative named. LEFT: ONE vc window of four (section 2 prose, AC-17.11 reword, AC-17.13 AT row, AC-17.14 AT row). hv holds O4 status segments + the Esc ratification. 227 cli + 170 svcs pass at 89f845cb. RE-MEASURE EVERY FIGURE AT PICKUP."
claims: [ST0065, ST0056/09, ST0056/17, ST0064]
---

# Interface Claude (ic)

**Localfold 2026-09-02 13:28Z, then built through 16:46Z (hv: agreed the redesign, then localfold+compact, then implement on the bounce -- DONE: `/settings` + vi). Pre-fold board verbatim in git at 07baadc1. RE-MEASURE EVERY FIGURE AT PICKUP -- five nodes write this hot tree and the delivered binary lags HEAD.**

## DOING

**TUI REDESIGN (WP-17). `/settings` AND THE vi KEYMAP ARE BOTH BUILT AND LANDED; WHAT REMAINS IS ONE vc CANON WINDOW AND hv's TWO OPEN RULINGS.** Accepted design: `intent/st/ST0056/tui-redesign-proposal.md` (7b08e8b1). **hv's GOVERNING DIRECTIVE: change any AC/AT needed, DO NOT yak-shave old guards or tests -- rewrite them to reality** (fail-forward; [[feedback_fail_forward_intent]]).

**LANDED (re-measure before citing):** `a8981480` mode collapse + section 3/4/5. `2f0ba5e8` the COMMAND PALETTE. `52b5acf3` vc minted `AC-17.13`; `95abeab5` its second side; `de5401c3` the dead-key invariant fixed. `02e802ae` the framed composer (O1). `94cd1047` repaint + dropdown rule. `4be902e1` composer editing keys. **`f00a794c` `/settings` (AC-17.14). `89f845cb` the vi keymap.**

**`/settings` LANDED AT `f00a794c` -- what it actually is, so nobody re-derives it:**

1. **`intentsvcs/src/settings.rs` is the allow-list AND the only reader/writer of `~/.intent/config.json`'s `explorer` section.** Registered in MODULES.md before it was written. `DECLARED` is one `Setting { path, label, blurb, values }`; `find()` IS the allow-list, so exactly one place decides whether a spelling is a setting.
2. **`View::Settings` went into the SHARED contract (`intentsvcs::nav`)**, because a `View` is what the stack holds -- a settings screen that was not one needs a parallel navigation model. Its path segment `/settings` is RESERVED, which costs an entity kind of that name; a test holds the reservation against the REAL declaration so that collision fails the suite rather than silently unaddressing a collection.
3. **A setting is PICKED, not typed.** `Enter` cycles to the next declared value, so a settings row takes the DOOR arm of the `OMNI + Enter` triple (`BY_ROW_KIND` gained `("setting", Omni)`; no edge changed, so section 3's table was untouched and stayed gate-locked). This is the picker section 7 records as the `select` row's next step, arriving first where the value set is declared.
4. **BOTH DIRECTIONS POSITIVE-CONTROLLED, and the second control is the one worth keeping:** with the allow-list lookup removed, the app emits `SetSetting { path: "intent_version", value: "emacs" }` -- the criterion's own defect, on demand.
5. **ONE WRITER FOR ONE FILE.** `bootstrap.rs` hand-rendered `config.json` because a serialiser is a second writer nobody declared; `/settings` made a second writer inevitable, so the two were made ONE (`settings::render_doc`). **`intent bootstrap --force` now KEEPS the settings section it did not author** -- that was a real destruction path I would have shipped.
6. **DRIVEN AGAINST A COPY OF THE REAL `~/.intent/config.json`**, not only fixtures: the three existing lines survive byte-for-byte, the section is created, read-back returns `vi`, and both refusals fire with the teaching message.

**THE vi KEYMAP LANDED AT `89f845cb`, AND IT SHIPPED WITH THE SETTING RATHER THAN AFTER IT ON PURPOSE** -- see watch-out 9. **`Esc` IS THE ENTRY TO NORMAL MODE AND THAT IS A DECISION I MADE ON hv's BEHALF; IT IS FLAGGED IN SECTION 7 AND IN CHAT, NOT SMUGGLED.** Esc is load-bearing in the ratified machine and vi needs it. The resolution keeps the invariant rather than contradicting it: section 3 requires that repeated Esc TERMINATES, not that it does so in ONE press, and normal mode is one step closer to rest. The escape corpus now runs under BOTH keymaps with the allowance DERIVED from the keymap -- widening it to cover both would have stopped saying anything about emacs. **The alternative hv may prefer is a different entry key, leaving Esc single-purpose; it costs every vi user's muscle memory.**

**WHAT IS LEFT, AND IT IS ONE vc WINDOW OF FOUR THINGS:** `tui-design.md` section 2 prose, `AC-17.11` reworded, `AC-17.13`'s AT row, and **`AC-17.14`'s AT row (now buildable -- the code exists)**. `AC-17.13`'s row must say the evidence lives in TWO tests in two files (`every_trigger_the_machine_answers_is_acted_on_by_the_realiser` and `every_offered_command_is_reachable_by_its_name_and_actually_does_something`), because the property is two-sided and a row citing one looks covered. `AC-17.14`'s row has its own two-sidedness: `a_setting_row_the_allow_list_does_not_carry_is_refused_rather_than_written` (app.rs, with its own control arm) and `the_settings_view_renders_the_declared_set_and_not_what_the_file_holds` (views.rs) -- the allow-list being right is no evidence the RENDERER asked it.

**hv's TWO OPEN ITEMS:** the `Esc` entry above (ratify or replace), and **the STATUS SEGMENTS (O4)** -- branch, diff, gate and binary-currency have no source the TUI can reach; vc recommends a facade seam over shell-outs in `run.rs`.

**COUPLING, MEASURED:** `mode.rs`'s transcription test READS `tui-design.md` section 3 FROM DISK at test time, so section 3 and EDGES are gate-locked into one commit -- **but only the TABLE**: `BY_ROW_KIND` and section 3's prose are not transcribed, which is why both landed without a canon window. `AC-17.11`'s tests do NOT read the doc, so section 2 and `layout.rs`/`draw.rs` are coupled only by the criterion's prose.

## TODO

1. **ASK vc FOR THE ONE CANON WINDOW OF FOUR** -- section 2 prose, `AC-17.11` reword, `AC-17.13` AT row, `AC-17.14` AT row. Gather the evidence OUTSIDE the window and make it ONE coherent commit (watch-out 2).
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

9. **A THING THAT IS DECLARED, WRITABLE, VISIBLE AND READ BY NOTHING IS THE RECURRING DEFECT ON THIS SURFACE, AND I HAVE NOW SHIPPED TWO OF THEM.** `Hotkey` was hv's find (declared, emitted, reachable, consumed by nothing, every invariant green). **`Command::group` was MINE** -- I wrote the paragraph condemning `Hotkey` and added a dead field to the same module in the same commit; found only by grepping for its consumers while adding `Act::Settings`, not by any test. **And `explorer.editing.mode` would have been the third**: a setting an operator can flip while nothing changes is a menu of errors one surface over, which is why the vi keymap shipped WITH the setting rather than after it. **The cheap check is a grep for consumers of every field you declare, at the moment you declare it** -- no invariant catches this class, because a field with no reader breaks nothing.

## Decisions

- **TUI REDESIGN ACCEPTED by hv 2026-09-02** (all 4 recs; proposal 7b08e8b1). Driving posture RULED: change any AC/AT to match, no yak-shaving old guards/tests. This is the WP-17 divergence rework. Implementation on the bounce -- see DOING.
- **AT-00.11 GREEN, landed 83c512c2** (mode 1 of AC-00.11). Note APPENDED not replaced: original 4028-char guidance verbatim at offset 0 + green evidence -> 8462; vc verified independently at 83c512c2^. Both attachments present, roster carries the row. WP-09 = 6/6 stays.
- **RELEASE IS v3.0.1, FEATURE COMPLETE, NUMBER CLOSED** (hv 2026-09-01, via vc). Everything outstanding goes in; no external consumer; scarcity register retired as a class.
- **TOOLCHAIN CURRENCY: binary 361eff99 BEHIND HEAD** (peer AC-06.11); rebuild owed, NOT mine; routed to vc for hv.
- **ST0064 WP-01 = 5/9 (ic).** 01.1/01.3/01.5/01.8/01.9 done; 01.2/01.4/01.6/01.7 cc/dc/hv-gated. project-CWD wiring; vc RULED per-app-instance root, D07 registry UNBUILT.
- **WP-17 STOOD DOWN status is SUPERSEDED by the redesign** -- 17.1 was unsatisfiable-as-written in 3.0.x (D56 equalises downward, EmptyMutation); the redesign reopens the WP-17 rows under the new design, so re-measure WP-17 against the reworded AC-17.9/17.11 once landed. 17.6 still blocked on cc's WP-08 browser-open stub.
- **ST0065 has ZERO ACs** (empty contract, hv-owned). WP-14 (whiteboard+inboxes in the store, L) MAY return from ST0069 -- hv's, not mine.
