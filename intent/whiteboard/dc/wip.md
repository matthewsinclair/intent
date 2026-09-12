---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 15:22Z
status: active
focus: "Steps 1, 2, 3 and 4 are DONE. WP-22 closed at 783b9cc82, the hook landed at bd79cf605, whole workspace green. Only step 5 is left, the final rehearsal, and it waits on the HEAD vc names. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-1414Z.md`.** Everything landed today is carried by its commits and the CHANGELOG, not here.

## DOING -- nothing. Step 5 waits on vc naming the rehearsal HEAD.

**Steps 1 to 4 are done.** WP-22 landed at `e809eea8f` and CLOSED at `783b9cc82` (AT-22.1 to AT-22.3, CHANGELOG, `wp done`). The hook landed at `bd79cf605`: `post-tool-symbol-context.sh` plus the shared `index-freshness.bash`, eleven bats arms, AT-24.4 green, AC-24.4 satisfied. Step 4 was cc's and already ruled.

**Two things a rebuild would need.** The hook ships OFF BY DEFAULT, like `post-tool-advisory`, and a bats arm asserts the shipped `settings.json` does not wire it -- because `no_pm_state_in_output` holds this repository's own `settings.json` byte-identical to the template, so wiring it would switch it on for every session on this box. vc can have it on with two lines. And the freshness predicate is ONE shell function taking the prefix as an argument, so each caller supplies its own subject: for an append the paths involved are the ones the ANSWER names; for a redirect they are the ones the pattern would have reached.

## TODO -- vc's five steps, serial, in this order

- **Step 4 is CLOSED and was never mine to run.** cc measured both Local shapes under AC-23.4 at subject `3ade8dea3` (report `b70a0f97e`) -- same toolchain and release profile as my grammar table, controls firing in both directions, wall-clock and ONNX linkage recorded -- and vc ruled under the pen that no Local runtime ships in 3.0.2. Both AC-20.4 and AC-23.4 compute satisfied. **I had announced the load and was one command from running two large dependency builds inside the quiet window to re-measure a settled question**; reading the design first is what stopped it. Two notes stand for whoever revisits shape B: cc measured NEWER versions than the brief named (fastembed 6.0.3 and candle 0.11.0 / tokenizers 0.22.2, against the brief's 4 and 0.9 / 0.21 -- the lockfile's resolution, and better numbers), and cc's note says the probe referenced the code path "and nothing executed", so the table is a SIZE measurement and does not claim the runtime answers.
- **Step 5: the final rehearsal** on the last HEAD, `--dry-run`, every gate line verbatim, `intent backup` taken deliberately and the report saying why, `~/.intent/home` read before and after, loads stated. **vc's one-re-run rule**: a red confined to `daemon_watch`/`daemon_subscriptions` re-runs the WHOLE rehearsal once and both runs' gate lines are reported; a second consecutive red on that family halts to vc, and any red outside it halts on the first.
- **CHANGELOG**: ic writes the Added lines for the search packages; my Fixed lines stay mine.

## Holds

- **vc is DARK (banking and folding).** Every landing and every report goes into `intent/whiteboard/vc/inbox.dc.md` with a same-turn `date -u` stamp as well as being messaged. **Anything needing a ruling WAITS in the inbox and is not guessed.**
- **Issue 0304 is filed and its ruling is vc's**: reconcile-by-default makes a pre-existing corpus overlap the DEFAULT answer -- a document the store carries AND the disk holds is indexed by both corpora, so one line answers twice (`kind: file` and `kind: thread`, same path, same line). Reachable on 3.0.1 through `index rebuild`, so not new; WP-22 is what makes it the default. Three shapes are on the table and the third is **invert the flag before the tag**, a small edit to one default and one register row. Condition: vc answers.
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD, AND SO DOES A TODO SOMEBODY ELSE HAS ALREADY DONE.** Re-drive a hold's condition when you quote it; never read it off this line. **Driven the hard way 2026-09-12**: step 4 sat in my TODO in vc's own words while cc had measured it and vc had ruled on it, and the board read exactly the same as real work. I announced the load to two nodes and was one command from two large dependency builds inside a quiet window. **A board is a record of what was true when it was written, and the estate moves underneath it** -- so the register and the design are the subject, and this file is a pointer to them.

## Watch-outs

- **THIS HOST HAS NO IDLE, AND "ALONE" MEANS ONE TARGET RATHER THAN AN IDLE HOST.** Measured in the window: with every Intent node silent, the one-minute load floors around 10 to 15 -- `fileproviderd` at 101%, iTerm, App Tamer, three CoreSimulator processes, none of them ours to pause. The daemon family reds ~3 of 6 alone at load 40 and 0 of 6 alone at load 15, so it is load and not a defect; and no measurement any node took today was on an idle host.
- **A FIELD SKIPPED ON SERIALISATION AND NOT DEFAULTED ON DESERIALISATION MAKES THE WHOLE ENVELOPE WRITE-ONLY.** `Hit::stale` skips `false` and had no `default`; serde supplies one for `Option` unasked and for NOTHING ELSE, so a fresh hit -- every hit in a normal answer -- made the envelope unreadable the first time anything read it back. Invisible for as long as the type only ever went outwards. Found by DRIVING `--daemon search`, not by reading.
- **TWO DERIVATIONS THAT SHARE A MISTAKE ARE ONE DERIVATION.** `daemon_op_for`, `daemon_servable_paths` and the load-time `serving_op` check each walked `families` alone, and the test that checks the roster against the table walked one list too -- so a `serving_op` on a `new_surface` row would have been read by nothing, refused by nothing, and agreed about perfectly. A test written to be an independent derivation is only independent of the FUNCTION, not of the assumption.
- **A SETUP STEP THAT SILENTLY DOES NOTHING IS STILL THE SAME TRAP** -- I ran `intent index refresh` as a control and it does not exist (`rebuild` does); its rc came from a pipe and the control read as informative while running nothing. Second time today's class has bitten.
- **A GATE'S SUBJECT IS AS EASY TO GET WRONG AS ITS RULE, AND THE RULE BEING RIGHT HIDES IT.** My hook's first build read the correct freshness predicate against the wrong paths -- the ones the GREP searched rather than the ones its own ANSWER named -- so a grep confined to a clean directory passed the gate and appended hits from a file that had moved. Found by driving the case, not by reading the code, which looked right.
- **A COST MEASUREMENT IS NOT A CONSEQUENCE MEASUREMENT.** My watch-cost numbers were right about events, cached paths and wakeups, and could not have seen the ingest loop the same registration caused, because they counted paths and never ran an ingest.
- **A DEPENDENCY COMPILED BUT NEVER REFERENCED IS NOT IN THE BINARY.** `lto = "fat"` plus macOS dead-stripping drops it, so a size measurement that only adds the crate reads a real cost as ZERO. Reference it behind `env::var_os` and make the control that it ANSWERS, not that it compiled.
- **THE ARITHMETIC IS AN INSTRUMENT CHECK.** Five grammar deltas summed to MORE than a shared-runtime model allows, which is impossible -- and that impossibility is what revealed they excluded the tree-sitter runtime.
- **A FILE THAT "NEVER LANDS" STILL HAS TO BE GONE.** My measurement harness sat unregistered in `tests/` and reddened `no_orphan_suite_member` and `one_clock` four runs out of four. The guards run against the TREE, not the commit.
- **RESULTS COME BACK AS A PATCH, NEVER A WHOLE-FILE COPY** (vc, a rule of the cut). A copy silently reverts whatever landed on main while you were building, and the diff looks exactly like your own work -- mine nearly deleted cc's `suite.rs` registration.
- **A TEST ASSERTS ITS CLAIM, NOT ITS CONTAINER** (vc, a rule of the cut). Twice in one day: a bats test pinned the sentence around its claim, and `carrier_is_installed_beside_the_block` pinned a LIST's length around its claim.
- **A SETUP STEP THAT FAILS SILENTLY LEAVES AN INSTRUMENT THAT STILL ANSWERS.** `intent init --name X` is not v3's spelling; it refused at rc 1 and every `intent critic` run after it looked normal, because the rule library resolves from the INSTALL ROOT.
- **`Op::Registry` LISTS the daemon's projects and does not REGISTER one**, and a watch only starts when a project-scoped op routes there. My arm's first run reported the dispatch broken while the dispatch was fine.
- **A SECOND ENUMERATION OF A SET IS A SECOND STATEMENT OF SCOPE**, and this thread paid for it twice. Enumerate once, decide once.
- **RUN THE WHOLE REHEARSAL, NOT THE STEP YOU EXPECT TO FAIL.** The first rehearsal refused at `intent doctor` and never reached the test gate, so a real committed defect sat red on main behind an unrelated refusal.
- **A TEMPLATE OR SHELL PAYLOAD EDIT IS DRIVEN WITH THE BATS SUITE**; its text is asserted there and nowhere in cargo.
- **A DISCIPLINE ON YOUR BOARD IS NOT A FLAG ON YOUR COMMAND LINE.** I wrote "under an isolated HOME" and had set none; the phrase came off this board rather than off the command.
- **`git stash` IS SHARED ACROSS EVERY WORKTREE OF ONE REPO.** Control a diff with `git diff > patch; git checkout -- <paths>; git apply patch`.
- **NEVER run a formatter over a file you are editing by hand.** The register is edited BY POSITION and only its markdown regenerated.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, and on a lock refusal re-issue the SAME command. Peers land often; a retry loop is worth having.
- **Every suite and build from a private worktree with its IN-TREE target dir** under an isolated HOME, with `CARGO_HOME` pointed at the real one.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output.**
- **A GATE THAT READS A GITIGNORED PATH CANNOT BE REHEARSED IN A CLONE.** Take `intent backup` in the clone deliberately and say why.
- **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split.** Messages go in a file, through `-F`.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
