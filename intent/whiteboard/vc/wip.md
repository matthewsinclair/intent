---
node: vc
name: Validation Claude
role: validation
session_id: bc2638fd-2828-428f-bdf9-aa7f9a99540d
heartbeat_at: 2026-09-16 19:47Z
status: active
focus: "THE CLOSE-OUT (hv, 2026-09-15), folded for hv's compact 2026-09-16: every bank through 0425 landed and verified, ST0075 and ST0057 done; resume at todo 20: judge dc's 0426, then hv's rebuild and drive, docs/reference, then the trawl and the guards adoption pass. NO RELEASE, NO PUSH."
claims: [ST0056, ST0057, ST0060]
---

# Validation Claude (vc)

## DOING

- THE CLOSE-OUT (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. Lanes and rules in intent/wip.md, rulings in intent/history/20260915-hv-rulings.md. vc: judge every bank, train and landing; impl.md and the ST0056 provisional markings; AC-07.6 reworded in the 0338 (i) landing; the quiet window on the word of hv. NO RELEASE, NO PUSH.

## TODO

- IN FLIGHT (2026-09-16). Landed and verified against their banks since todo 19: dc's 0412 (b8db00fa9) and migrate (292414661, closing 0403, 0404 and 0406-0409 at 61a88ae21), cc's 0331-stale (f2f25c57d; vc ruled the claude rules row a read), ic's 0419 cursor (fe2471b17), ic's 0421 and 0422 explorer fixes (4df530f36), cc's 0402 canon normalisation (4802c70d9: intentd had already written it and the banked check judged it 3/3/10), ic's 0423 app start/restart deadline (f330805da), dc's 0424 register refusal (fe982f2e1), and ic's 0425 merged log backlog (8c1252f39). ST0075 and ST0057 are done on hv's word (dcf3bc3c1); ST0057 stays declared in .intentfiles because ST0056's parity tools read its parity/tools. Filed: 0420 (the store write lock, with captures and intentd's own refusals from 2026-09-15T22:42Z onwards). Open: 0344 (the clean install), 0420, and 0426. RESUME: (1) dc is building 0426 (project guards declared as `guards` in config.json and run by the live gate; a missing declared body blocks; --list-guards gains a 5th column; doctor Advisories for a hand-wired hook and an unset core.hooksPath beside a tracked hook dir; adoption is a separate estate pass) under vc's five rulings: judge the bank, GO, verify. (2) Then hv's rebuild and drive: `bin/devbin build all`, `intent daemon restart`, `bin/int macos app-install`, then drive `intent app restart` (0423), the Console order (0425), `wb register` (0424) and 0426's gate. (3) Then gen_reference.sh regenerates docs/reference whole, as its own commit. (4) Then on vc's word the board trawl (dc's trawl.sh in scratch clones, register-then-migrate for every board not in the store) and the guards adoption pass, and vc sends hv one worklist. NO RELEASE, NO PUSH.

## Holds

_(none)_

## Watch-outs

- **A parent build cannot read a newer build's fixture** (store 18 vs 17). Build each arm's fixture with its own binary.
- **Count `intentd` by executable**, `ps -axo pid=,command=` on argv[0]'s basename.
- A renderer change (a footer, a view's format) skews every generated view that organize does not touch: the whiteboard boards and inboxes, steel_threads.md and todo.md. Doctor refuses every commit until they match, intent sync --to-disk refuses while intentd watches, and the restarted intentd re-renders them itself, so after the rebuild and organize --apply re-read git status and commit those views with the rest (37e961bdb, 2026-09-15).

## Decisions

- **NO OVERTESTING, NO YAK-SHAVING** (2026-09-12, on handing vc the pen): build what the STs need; tests are the AC rows; never a test that tests a test. Applies to every lane and to the director.
- **WATCH THE RUST FOR HIGHLANDER, THIN COORDINATOR AND PFIC on every review.** A posture, not a gate. **PFIC is _Pure Function, Impure Coordination_** -- deterministic core, I/O at the boundary.
- (2026-09-13) hv, verbatim: NO UNNECESSARY OVERTESTING, NO TESTING TESTS, NO YAK-SHAVING. Expeditious delivery of working code; do not relitigate the speed of light for every decision; be RUTHLESS. New code, good code, good tests, done. Applied: build the cheap fix and let the live system judge it; one run decides a question; never a positive control of an instrument; ACs one line per user-facing behaviour, ATs the test that proves it once.
- (2026-09-15) vc, under the close-out pen: after 0321, every line intentd writes to intentd.log or intentd.err.log, remedy: lines included, opens with an RFC 3339 UTC timestamp with a Z suffix, then one space, then the line byte-identical to today, through one writer module whose clock read is one_clock.rs's single exemption, its reason recorded once in ST0056 design.md's D42; ST0075's ConsoleLine skips exactly that token, when present, before it classifies a line from either log (in WP-02's bank), so 0321 lands with no Swift change. The intentd notices that carry a remedy: line but no severity token get intentd's existing warning: or error: token in dc's 0321 bank, which closes ic's new issue with 0321 (the critic advisory's option (iii); the Console does not guess). Two banks that share a file (MODULES.md, dispatch-table.json and its view) stack in the order they bank green, nobody holds a bank for the other, and the second proves its bank on top.
- (2026-09-16) vc, under the pen, on dc's costing read from the source at d24487079: devbin's migrate carry defects are fixed by a LOUDER MIGRATOR, not a richer model. The model is unchanged: no store rung, no face change, no new item verbs, because every new field would be data only migrate creates and no verb changes, which is devbin letter I recreated. wb migrate carries the pre-migration wip.md verbatim as a snapshot through the existing snapshot path, so nothing is lost; anything it cannot carry (a ### sub-heading, a table, a heading's text after the kind word, a board lead) is named as a unit BEFORE any write and the carry is REFUSED, the way WbSendersNotRegistered already refuses, unless an explicit drop flag is passed; prose it carries as an item is reported on a coerced: line and never silently; a .history file left in place gets its own word, not uncarried:. Devbin letter C (empty sections emitted, fixed section order) closes as intended: both are documented renderer rules in views.rs, and its real harm is the pickup gap, fixed separately. Size M, one crate.
- (2026-09-16) vc, under the pen, on cc's grouping of devbin letters J, O, N (store and audit) and H, I, M (identity): (1) a node's name and role are corrected by `intent wb register <moniker> --name --role --correct`, a flag on the one identity door rather than a new verb: registered monikers only (it never creates a node), name and role only, rc 0 reporting the node unchanged when nothing differs, WbRegisteredDifferently's remedy names it with both values, recoverability as register's, its dispatch row, legal pairs and table regen in the same change; hv may respell it before release. (2) Every whiteboard write logs an event, wb touch included, through one store door that stamps updated_at and appends the event in the same transaction; events are not carried into intent/.canon (issues/, project.json, st/), so a logged heartbeat dirties no tracked file.
- (2026-09-16) vc, under the pen, answering cc's replace_boards question for bank 1 (0411, 0415, 0414): sync --to-store APPLIES THE DIFFERENCE BY NATURAL KEY and never carries id or updated_at, so board.json and schema/board.schema.json do not move. replace_boards becomes delete-missing, update-changed, insert-new, the precedent replace_file_index already sets: wb_node keyed on moniker, wb_item on (node, kind, seq), wb_message on (sender, recipient, recorded_at, body) counted as a multiset so two identical rows stay two. An unchanged row keeps its id and updated_at; a changed row is updated through the one store door vc decision 21 (2) rules, stamping updated_at; an inserted row takes the next id in board.json order, which is today's order on a full restore. sync_overwrite lists the whiteboard differences (a node whose fields differ, an item or message added, changed or removed) beside threads and issues, so the no-overwrite line is printed only when it is true, and a run that differs names what it replaced. Size M with decision 21 (2), one crate.
- (2026-09-16) vc, under the pen: when two CHAIN STARTs cross, the moniker earlier in the alphabet goes first (cc < dc < ic < vc) and the other waits for its CHAIN END, with no further messages about order. ic's and vc's starts crossed repeatedly on 2026-09-16 and both sides yielded each time.

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
