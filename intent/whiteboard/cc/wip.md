---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 18:12Z
status: active
focus: "WP-14 commit TWO is LANDED: 87b819abd, on a green workspace suite at peak load 122.14. Next is vc's standing order two -- the rest of the intent wb family, commit per verb group, row landing with its verb. The live store stays at 23 until vc rebuilds and broadcasts. NO RELEASE, NO PUSH."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

- **ST0069 IS REOPENED TO THE END BY hv, AND cc's LANE IS WP-14.** hv, verbatim: _"it's not 'done' until ST0069 is done... All non-cancelled WPs done done."_ vc directs and has the pen.

- **WP-14 COMMIT ONE `b9f2aec74` AND COMMIT TWO `87b819abd` ARE BOTH LANDED.** Commit two carries the three tables and rung 24, `SCHEMA_VERSION` 24, both pins, the projection, the restore door, `register_nodes`/`register_roster`, the `wb` family and `wb register` register rows, the render arm, the five `intent/whiteboard/<node>/board.json`, and the four data-model.md items. `wp start ST0069/14` ran before it, so the WP reads in flight. The landing report with the file list and the peak is in `vc/inbox.cc.md` at 18:12Z.

- **NEXT IS vc's STANDING ORDER TWO: the rest of the `intent wb` family.** One commit per verb group, the register row landing WITH its verb (vc's amendment, for WP-14 only), fixture board only, views NOT switched, both guards untouched, no `cwi` and no skill. ic reviews and corrects the rows in their own commit after each lands.

- **THE LIVE STORE STAYS AT 23 UNTIL vc REBUILDS THE PAIR AND BROADCASTS.** No binary built from commit two or later reads it; vc migrates it to 24 by their own hand. Worktree only. NO RELEASE, NO PUSH.

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.

## Holds -- mine, with the CONDITION that releases each

- **RELEASED 2026-09-12:** the D29 pause. vc withdrew the amendment at `2866a40ed`; the corpus is the committed `.gitignore` rules and `ignored_paths_corpus.rs` is the record. The code never moved, so nothing had to be unwound.

- **THE DEFECT LIST ITEMS hv DID NOT RULE.** hv ruled batches 2 and 3 to cc on 2026-09-12 06:16Z and both are landed. **The mixed-proxy silent drop stays held:** vc's two briefs disagreed and vc settled it -- the first stands, so it is not batch 3's and waits with item 9. **Still held, unruled:** the rule proxies that contradict their own rule (item 9) -- the elixir proxies firing on what their Good prescribes, swift and lua UNDECLARED -- and the usage-error exit code, which is dc's to rule. **Released when hv rules them, or vc routes one to me.** The gate-blocked pair (`strong-assertions/bad_test.exs`, `test-highlander-shared-setup/good_test.exs`) is part of item 9 and waits with it.

- **POST-CUT (culled from the 3.0.1 loop 2026-09-11):** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; none is 3.0.1 work. **Still held (2026-09-11 19:49Z):** hv opened the doc audit, not these, and `intent/wip.md` lists `0177` as post-cut with no owner.

## Watch-outs

**LEANED 2026-09-12 06:07Z ON hv's INSTRUCTION.** One line each, and only what bears on work in front of cc. The families whose subject is gone or which a guard now enforces -- population and denominator, one name two artefacts, the unfelt claim, the reachable-but-unread artefact, canon past the daemon's ingest, folding this board, counting a migration's blast radius -- are in `.history/20260912/wip-prelean-0606Z.md` with their full text. Read them there before arguing with one.

- **THE SHARED CHECKOUT.** `git add <paths>` then `git commit --only <paths>` in ONE call; against a peer's index lock re-issue the SAME command, never remove the lock, and judge by `git log -1`, never by the loop.
- **BUILDS AND SUITES.** Only from a private worktree's own in-tree build under an isolated `HOME`; read back `~/.intent/home` and the live store's mtime afterwards, because a test run deploys to the estate it lives in.
- **THE GATE.** Never `--no-verify`. Capture a commit's WHOLE output and read `rc` and `git log -1` before believing it landed; a filtered refusal reads exactly like success. The gate lints the rule library's own bad examples, so that pair cannot be committed at all.
- **THIS SHELL IS ZSH.** Unquoted `$var` does not word-split, an unmatched glob aborts the call, a bash script's functions sourced here run as zsh (drive them from a `bash drive.sh`), and an exit code that IS the finding never goes through a pipe.
- **`cargo test --workspace` STOPS AT THE FIRST FAILING TARGET**, so a run with one red measures nothing after it -- I claimed a green twice today over a suite the run never reached, with a real red behind it. `--no-fail-fast`, always, and the flaky daemon pair is exactly the target that hides the rest.
- **`alone` MEANS ONE TEST TARGET, NOT AN IDLE HOST.** I reported a red as a change of character from a two-of-two on a box three nodes were hammering; dc's caveat was right and the claim had to be narrowed to what was measured.
- **`cargo test -p intent-cli` DOES NOT BUILD `intentd`, AND THE DAEMON TESTS SPAWN IT FROM THE TARGET DIR.** In a target dir that has never built the sibling it is ABSENT, not stale, and 37 tests across daemon, web, graphql, routing and edit-and-browse fail deterministically at any load. `cargo build -p intentd` first, then the suite: 614 passed, 0 failed. **This is not the load-sensitive daemon family** -- that is a real and separate thing, three arms, dc's characterisation. A stable 37 means a missing precondition; a moving two or three means the box.
- **A `tail -n` ON A FAILURES BLOCK IS A SILENT NARROWING, AND I READ ONE AS A COMPLETE LIST.** `tail -40` cut three names off a 37-name block; comparing that against a later full capture produced a "composition shift" I reported to vc as the signature of a load-flaky family. The set was IDENTICAL every run -- deterministic, not flaky -- which is the opposite character. Capture a failures block WHOLE, or say the capture was bounded when quoting it.
- **`git stash` IS A REPOSITORY-WIDE STACK, INCLUDING FROM A WORKTREE.** I used it to lift a diff for a control; if a peer had stashed in that window my pop would have taken theirs. It popped clean and nothing was at risk because the main checkout held the edits throughout, but the instrument for lifting a diff is a PATCH FILE, which I had.
- **"MEASURED" MUST NOT DRIFT INTO "PROVEN TO RUN"** (dc, on my own Local table). A reference behind an opaque condition is the right control for a SIZE measurement -- it defeats the dead-strip -- and it is a weaker claim than the runtime answering, which is what dc's brief asked for. The table says reached-and-not-executed and must keep saying it.
- **A SCHEMA BUMP MIGRATES THE WORKTREE'S OWN STORE, AND REVERTING THE BUMP STRANDS IT.** Building at `SCHEMA_VERSION` 24 migrated `wt-cc/intent/.cache/intent.db` to 24; reverting to 23 left the binary refusing its own store, which surfaced as THIRTY red daemon tests that read exactly like the absent-sibling signature and were nothing of the kind. There is no downgrade rung by design. Delete the worktree's cache and let it rebuild. The live store was untouched and still read 23, which is the isolation rule doing its job.
- **NEVER RESERIALISE `surface/dispatch-table.json`; INSERT TEXTUALLY.** A `json.dumps` round trip to add two rows reformatted unrelated entries, re-escaped `\u0027` back to `'`, and moved a whole `daemon restart` row -- 127 insertions against 60 deletions for a 31-line addition. The register keeps its own formatting and is never run through a reformatter. Its generator also refuses a hand-edited `populations` block and compares ORDER, and it refuses an idempotent verb withheld from MCP unless `recoverability_anomaly` says why.
- **schemars LIFTS `///` INTO THE PUBLISHED FACE, so a criterion id in a doc comment SHIPS.** `no_pm_state_in_output` caught this twice in one commit -- first in the DDL comments, which are published in `ddl.sql`, then again in `board.schema.json` after I had fixed the first. A consumer holding that face has no AC to look up. Ids belong in `//` comments, which do not ship.
- **THE GATE REFUSES UNFORMATTED MARKDOWN AND THE DAEMON RE-INGESTS THE FORMATTED BYTES UNDER YOUR STAGE.** A refused commit, `prettier --write`, and the canon went `MM` while my index already held the old bytes. Settle past the ingest, then re-stage; committing the half you had is the failure available here.
- **A REGISTER THAT VALIDATES IS NOT A SURFACE THAT EXISTS.** Both `wb` rows were inserted textually inside the WRONG family's `entries[]`, and every arm of the generator passed -- the census, the pair count, the populations, the vocabularies -- because each reads ROWS and the rows were well-formed. `families` did not move, so the shape assertion passed too. The binary refused: `intent wb --help` said `unrecognized subcommand 'wb'`, because `spine::build` names a family's clap command from `family.name`. Textual insertion is the rule for that file and is also how a row lands in the wrong parent. **Type the command.**
- **`cargo test` DOES NOT CHECK FORMATTING, SO A GREEN SUITE IS NOT A COMMITTABLE TREE.** 2530 tests passed and the gate then refused four unformatted Rust files. `rustfmt --edition 2024` on the named files, rebuild, re-run the arms that touch them, re-issue the SAME command.
- **`git add` LOSES THE LOCK RACE TOO, AND ITS FAILURE SURFACES DOWNSTREAM AS SOMETHING ELSE.** A retry loop that classifies only the COMMIT's output reads the add's silent failure as `pathspec 'x' did not match any file(s) known to git` and stops on what looks like a real refusal. Retry the add and the commit as ONE unit.
- **A CLAIM NAMES THE ARTEFACT IT EXAMINED AND THE INSTRUMENT THAT READ IT**, and a green counts only beside a red the same instrument produced.
- **A CLOCK VALUE COMES FROM A `date -u` READ IN THIS TURN.** Boards are guarded; messages and file names are not -- I mis-stamped a banked file this morning and renamed it.
- **A PEER CHANNEL'S WRITE IS NOT ITS DELIVERY.** An ask carries its terminating condition, and a CODE-WRONG goes to vc rather than into my own diff.

## Decisions -- unexecuted only

- **None.** Every ruling routed to cc is executed. The lessons that were here are in `.history/20260912/wip-prelean-0606Z.md`, and the two from the doc audit are in the project memory (`feedback_shared_checkout_hazards`, `project_doc_audit_20260911`).
