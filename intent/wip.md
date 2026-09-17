---
verblock: "2026-09-17:v1.77: vc - ST0076 WP-03 to WP-06 in build, 0432 on hv's capture, ST0056 on the gyges run"
intent_version: 3.0.3
---

# Work In Progress -- closing out Intent's open issues and threads

## DOING

- The close-out (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling. The rulings are in `intent/history/20260915-hv-rulings.md` and on hv's board (decision 18 adds devbin's whiteboard defects to this release); what is open is `intent issues list` and `intent st list`. One lane lands on the tree and the store at a time, on vc's word; a bank is a patch blob on `refs/bank/<node>/<issue>`; ready banks ride a train with one judging run. Each lane resumes when hv lifts its hold, at its own board's IN FLIGHT todo.
- ST0076, the typed symbol index for Rust and Elixir (hv, 2026-09-16), closing on its twelve ACs (`intent ac gate ST0076`): dc builds WP-03 (Elixir qualified references, AC-03.1, level 2 only where a module is written, aliases expanded within the do-block) on cc's extractor-shapes bank (refs/bank/cc/st0076-extractor-shapes), and the two land as one train on vc's judgement of the combined gate; ic builds WP-04 (surfaces, AC-04.1 and AC-04.2: `--kind` for hit kinds, a separate `--subkind` from a derived roster, `--in`, filters honoured by `--context` and `--outline`, one field name across SQL, JSON and MCP); cc measures then builds WP-05 (Rust resolved references through rust-analyzer SCIP, AC-05.1); dc then WP-06 (the Elixir tracer, AC-06.1, AC-06.2 on Laksa). AC-00.1 is hv's drive after the rebuild.
- Open issues otherwise: 0432 (hv's blank explorer after nvim): not reproduced by ic in a real iTerm window, nor on a clone of the Intent store with a live intentd and the pre-0420 binary; it waits on hv's `script -q ~/explore-blank.log intent explore` capture, with the view and row, how nvim was left, and whether a key or a resize restores the screen.
- vc: closing the quiet window with hv; judging every bank, train and landing.

## TODO

- The quiet window's remainder, now: hv runs `bin/devbin build all`, `intent daemon restart` and `bin/int macos app-install`, then drives `intent app restart` (0423, 0431), the Console's merged order (0425), `intent wb register` (0424), a declared guard (0426), and a fresh Claude session asked where a symbol is defined, recording whether it reaches the index's MCP tool (0428) and whether a keyword ToolSearch ranks it (0430), plus ST0076 AC-00.1 once WP-04 lands; the store-lock fix (0420) takes effect with the rebuild. Then `intent/st/ST0056/parity/tools/gen_reference.sh` regenerates `docs/reference/` whole as its own commit.
- The all-estate board trawl, on vc's word once the quiet window closes (hv, 2026-09-16): a plain `intent wb migrate` CARRIES FOR REAL on a board with nothing uncarried, so dc runs it only in scratch clones, one per estate (every registered estate, Conflab included, covering every board not yet in the store: a board with no registered node is registered in the clone first, so the worklist shows the real path, register then migrate): the store copied with `sqlite3 .backup` (never `cp`, every store is WAL), the estate's working `intent/whiteboard/` files copied over the clone so uncommitted board edits count, an isolated HOME and no daemon. dc records each node's exit status and its `uncarried:`, `coerced:` and `left in place:` lines, and names already-migrated nodes as skipped. vc sends hv one worklist; hv rules restructure or drop per unit or by class; dc carries in the live estates, doctor runs, vc checks each estate, and each estate gets one mechanical commit in its own repo. NO PUSH.
- The guards adoption pass, beside the trawl (vc, 2026-09-16, on 0426): per estate, declare the project's own guards in config.json `guards`, remove the calls it duplicates from the hand-wired or tracked hook, and run doctor clean; coordinated with devbin for the repos whose `.githooks/` and `bin/int` it manages (Intent, Gtools, Laksa); one commit per estate. NO PUSH.
- hv: the prepared `gh release edit v3.0.2` annotation.
- hv chooses the release version at the close-out.
- ST0056's clean install (AC-00.5, AC-11.1) and 0344's never-tapped leg: after the rebuild and drive pass here, hv runs `intent/st/ST0056/gyges-brief.md` on gyges and hands back `~/intent-clean-install.log`; vc judges it, dc satisfies AC-00.5 and AC-11.1 on it, and WP-11 and ST0056 close on hv's word.
- ST0060 (`intent vault`) stays open as the next line's first thread (hv, 2026-09-15).
