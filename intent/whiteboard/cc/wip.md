---
node: cc
name: Control Claude
role: control
session_id: 2fa2121a-51bb-433f-8459-97b1d78b71c9
heartbeat_at: 2026-09-14 12:09Z
status: active
focus: "dc's build done at 4c172d260 (2026-09-14 12:09Z): 0355's touch-cost fix is live (intentd pid 16233, doctor clean), so the build-window hold is archived. Awaiting vc's verdict on the live touch cost; then clear wt-cc's uncommitted patch and the scratchpad/subj worktree. NO RELEASE, NO PUSH."
claims: [ST0056/06, ST0056/10]
---

# Control Claude (cc)

## DOING

_(none)_

## TODO

- Read the lane column in `intent/wip.md`, never a copy here.

## Holds

- **THE DEFECT LIST ITEMS hv DID NOT RULE.** The mixed-proxy silent drop, the rule proxies that contradict their own rule (item 9, with the gate-blocked pair), and the usage-error exit code, which is dc's to rule. **Released when hv rules them, or vc routes one to me.**
- **POST-CUT:** `ext` x5, `learn`, `config` x3 ship declared-and-unbuilt (hv, 2026-08-31). **Released when hv opens work after the 3.0.1 cut**; `0177` is post-cut with no owner.
- **0366 LANDED AT 524f5f868; dc VERIFIES IT.** dc runs the two-arm harness (0366 through --daemon search on a fresh daemon, the unfixed pair as control) and then the full workspace suite on that checkout, every target --no-fail-fast. **Released when dc reports: green closes it, a red comes back to cc.**

## Watch-outs

- **LEANED 2026-09-12 06:07Z ON hv's INSTRUCTION.** One line each, and only what bears on work in front of cc. The families whose subject is gone or which a guard now enforces -- population and denominator, one name two artefacts, the unfelt claim, the reachable-but-unread artefact, canon past the daemon's ingest, folding this board, counting a migration's blast radius -- are in `.history/20260912/wip-prelean-0606Z.md` with their full text. Read them there before arguing with one.
- **THE SHARED CHECKOUT.** `git add <paths>` then `git commit --only <paths>` in ONE call; against a peer's index lock re-issue the SAME command, never remove the lock, and judge by `git log -1`, never by the loop.
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
- **AN ARM ON AN IN-MEMORY STORE CANNOT SEE A DEFECT THAT LIVES IN THE OPEN PATH.** AC-14.8's facade arm was green while `intent search` found nothing: the next process's open erased the board sections, `load_fresh` held no boards, and `board.json` answered as a file. Measure through the surface the criterion names, on a store that outlives the facade.
- **AFTER A THREE-WAY APPLY THE LANDING PATCH IS `git diff HEAD`.** The apply stages into the worktree's index, so a bare `git diff` is empty and the landing refused with "no valid patches"; a new file needs `git add -N` before it shows at all.
- **`reset --hard` KEEPS UNTRACKED FILES.** A script's new test file survived onto the next base without its `suite.rs` line, and `no_orphan_suite_member` refused it; a red that names a file outside the change is residue, and the file says so.
- **`Failure::Unavailable` EXITS 2, WHICH A GATE READS AS FAIL-OPEN.** A refusal that is a plain no is `Failure::Error`, exit 1; the CLI drive caught the first build of the register form getting this wrong.
- **A RULE CHANGE APPLIED TO MAIN BEFORE ITS COMMIT BLOCKS EVERY NODE.** The gate reads its guards from the working copy, so while the stamp ruling sat applied and uncommitted, arm 6c of shared_artefact_build_guard.sh, which pinned the reversed rule, refused dc's and ic's commits too. Find the guard that pins the old rule before applying, and land both in one commit. And a parity tool under intent/st/ is a thread ATTACHMENT: the daemon ingests the edit into canon, and that canon lands in the same commit or canon-commit refuses.
- **A SAMPLE CANNOT TELL A THREAD BLOCKED IN A WAIT FROM ONE SPINNING THROUGH IT.** Both show the same leaf in every sample, so the hot thread is named by ps -M CPU per row, and an inlined frame is resolved by disassembling the binary whose UUID matches the sample's Binary Images line (dwarfdump --uuid, then objdump at the return address), not by symbol names.
- **CONCURRENT intent WRITES REFUSE ON database is locked AND CHANGE NOTHING, AND A RACING SIBLING CAN PRINT overwrote bytes that were not the store's render.** A batch of set / ac edit calls against a store other nodes are writing lost a third of its writes to the lock. Re-issue only the writes whose canon still differs from the draft, then check canon against every draft past the ingest and diff the canon against HEAD for fields nobody meant to touch.
- **BUILDS AND SUITES.** Only from a private worktree's own in-tree build under an isolated `HOME`; read back the home pointer and the live store's mtime afterwards, because a test run deploys to the estate it lives in. Since dc's build done at e3c67792c (2026-09-14 07:25Z) the pointer is `~/.local/share/intent/home` (XDG), not `~/.intent/home`, and it names this tree.
- **A LOCAL DAEMON PROBE NAMES A COST; A THREAD CENSUS DOES NOT.** 0355's touch cost was attributed to a watcher thread by ps -M creation order and was the store thread, off by one; a probe-only timing line per handler and per refresh stage named replace_sections_for in one run. Run it from wt-cc's release build under an isolated HOME on a separate worktree, with XDG_RUNTIME_DIR from mktemp -d, because a socket under the scratchpad is past SUN_LEN and the daemon exits at bind. Kit: scratchpad/probe-run.sh, probe-instrument.py.

## Decisions

- vc, 2026-09-13: the generated views a verb re-renders, boards and inboxes included and every node's, land in the commit of the verb that rewrote them. Nobody owns a generated file.

---

_Generated by Intent v3.0.1 from `the whiteboard model`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
