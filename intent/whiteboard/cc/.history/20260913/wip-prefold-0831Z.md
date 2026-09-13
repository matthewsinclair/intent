---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
commit_session_id: read it off your own last commit with git log -1 --format=%B <sha> | grep -o 'session_[A-Za-z0-9]*' -- never off this line, and never with git's trailer parser, which returns empty on every commit here
heartbeat_at: 2026-09-12 20:16Z
status: active
focus: "LOCALFOLDED 2026-09-12 20:16Z for the user's compact. The whole `intent wb` family is LANDED across five commits. UNLANDED and banked as two patches: the `Hold` kind with `wb add`, and the two board renderers. Next: land the kind+verb, then the renderers, the GraphQL fields, the search measure. NO RELEASE, NO PUSH."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

**LOCALFOLDED 2026-09-12 20:16Z for the user's compact. Everything landed is carried by its commits; the UNLANDED work is carried by the two patches named below, never by prose here.**

- **ST0069 WP-14 IS cc's LANE, vc DIRECTS.** hv: _"it's not 'done' until ST0069 is done... All non-cancelled WPs done done."_

- **THE `intent wb` FAMILY IS LANDED, FIVE COMMITS.** `87b819abd` tables + rung 24 + `wb register` + the five `board.json`; `de03d227e` reads (`status`, `show`); `dc77fc9f7` messages (`ask`, `announce`, `clear`); `28f9d4b99` items (`decide`, `claim`, `unclaim`) plus four `recoverability` corrections; `dd3e3444e` lifecycle (`pickup`, `touch`, `release`, `archive`) plus `enum_arg`. Issue `0312` and its manifest line at `190eae3bf` / `ea0bad859`.

- **TWO PATCHES ARE BANKED UNLANDED, AND THEY LAND IN THIS ORDER.** (1) `scratchpad/wb-hold-and-add.patch`, base `ea0bad859` -- `WbItemKind::Hold` as the fifth kind, `SCHEMA_JSON_VER` 19, `wb add <kind> <text>` refusing `decision` with a remedy naming `wb decide`, the `wb_item_kind` map as one home, the row, the slot, the census bucket and the provoked refusal. **vc wants this as its OWN commit so dc can rebase the migration (AC-14.9) onto it.** (2) `scratchpad/wb-views.patch`, the same base -- `views::wb_board` and `views::wb_inbox`. **The renderers are written for FOUR kinds and must gain `Hold` before they land.**

- **THEN, IN vc's ORDER.** AC-14.2: the two renderers as generated views, byte-identical on TWO RENDERS with the formatter EXCLUDED rather than a fixed point through it (vc reworded the row; `.prettierignore` already excludes the inboxes for a separate reason, and `generated_views_are_not_formatted.rs` derives its roster from `views::render_all`). Built against the FIXTURE; the live boards stay hand-authored under both guards until vc signals the cutover, so `render_all` is NOT wired yet and the doctor-skew half arrives with it. Then AC-14.7's GraphQL half -- root `board(node)` and `boards`, types already derive `SimpleObject`. Then AC-14.8, re-measured on the fixture once the views exist, never against a store a daemon holds.

- **AT ROWS ARE OWED FOR EVERY AC-14 ROW I COVER**, cited to arms that exist.

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
- **`git apply -3` IMPLIES `--index`, SO IT STAGES INTO THE SHARED INDEX, AND ITS PER-FILE LINES ARE NARRATION RATHER THAN EVIDENCE.** It prints `Applied patch to '<file>' cleanly.` for every file on `--check` and on a run that then REFUSES, so a `tail -3` of them says nothing about whether anything landed -- I read three and believed an apply that had not happened, which is my own truncation watch-out arriving from a new direction. Worse in a shared tree: for a minute my arms and a peer's uncommitted work were in one file, and either side's `commit --only` would have swept the other's in invisibly. **Check the TREE for a symbol you know you added**, and back out with `git apply -R` rather than by unstaging anything of theirs.
- **`git commit --only <paths>` TAKES THE WORKING TREE, so a path a peer is mid-commit on is not yours to name.** Find out WHOSE it is and wait; do not stash, unstage or reset around it.
- **A NUMBER DEFENSIBLE IN A SENTENCE IS WHAT A MEASUREMENT IS A CORRECTIVE TO** (ic, on my 2000-byte bound). I picked it as "a long paragraph" against the 32KB inbox in the design; ic parsed the 45 entries the bound would actually govern -- median 2423, 28 over 2000 -- so it refused the MEDIAN case. **A bound is set from the corpus it governs, and the setting records which corpus and when it was read.** The failure it would have caused is not one anybody debugs: a node finding mid-report that its escalation will not fit, whose cheapest way out is to say less.
- **CHECK WHICH OF TWO CITATIONS IS RIGHT BEFORE MAKING THEM AGREE.** Asked to reconcile two `basis` spellings, both turned out wrong: `ST0069 design.md` says of ITSELF that it is the search leg and that this leg keeps its inherited design elsewhere, and the directory I then cited holds no `design.md`. Agreement is not correctness.
- **A REGISTER THAT VALIDATES IS NOT A SURFACE THAT EXISTS.** Both `wb` rows were inserted textually inside the WRONG family's `entries[]`, and every arm of the generator passed -- the census, the pair count, the populations, the vocabularies -- because each reads ROWS and the rows were well-formed. `families` did not move, so the shape assertion passed too. The binary refused: `intent wb --help` said `unrecognized subcommand 'wb'`, because `spine::build` names a family's clap command from `family.name`. Textual insertion is the rule for that file and is also how a row lands in the wrong parent. **Type the command.**
- **`cargo test` DOES NOT CHECK FORMATTING, SO A GREEN SUITE IS NOT A COMMITTABLE TREE.** 2530 tests passed and the gate then refused four unformatted Rust files. `rustfmt --edition 2024` on the named files, rebuild, re-run the arms that touch them, re-issue the SAME command.
- **`git add` LOSES THE LOCK RACE TOO, AND ITS FAILURE SURFACES DOWNSTREAM AS SOMETHING ELSE.** A retry loop that classifies only the COMMIT's output reads the add's silent failure as `pathspec 'x' did not match any file(s) known to git` and stops on what looks like a real refusal. Retry the add and the commit as ONE unit.
- **A CLAIM NAMES THE ARTEFACT IT EXAMINED AND THE INSTRUMENT THAT READ IT**, and a green counts only beside a red the same instrument produced.
- **A CLOCK VALUE COMES FROM A `date -u` READ IN THIS TURN.** Boards are guarded; messages and file names are not -- I mis-stamped a banked file this morning and renamed it.
- **A PEER CHANNEL'S WRITE IS NOT ITS DELIVERY.** An ask carries its terminating condition, and a CODE-WRONG goes to vc rather than into my own diff.

## Decisions -- unexecuted only

- **None.** Every ruling routed to cc is executed. The lessons that were here are in `.history/20260912/wip-prelean-0606Z.md`, and the two from the doc audit are in the project memory (`feedback_shared_checkout_hazards`, `project_doc_audit_20260911`).
