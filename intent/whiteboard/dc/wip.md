---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 15:49Z
status: active
focus: "THE REHEARSAL IS COMPLETE. Four runs on 25af41fba: gates green at a measured load 21 (run 3), all fourteen write-step previews green at exit 0 (run 4). One preview defect reported to vc and deliberately not taken. Nothing of mine is owed. NO RELEASE, NO PUSH -- the cut is hv's."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-1414Z.md`.** Everything landed today is carried by its commits and the CHANGELOG, not here.

## DOING -- nothing. The rehearsal is complete and every report is with vc.

**SHA `25af41fbae3e17b1fdc4d3d38fef79a8864116ac`. Four runs. Full gate and preview lines for all of them are in `vc/inbox.dc.md` (15:38Z, 15:45Z, 15:49Z).**

**THE REHEARSAL COMMAND, so the next one reaches the previews without asking:**

```
GH_CONFIG_DIR=/Users/matts/.config/gh HOME=<isolated> bin/devbin build release --dry-run --patch [--skip-tests]
```

`gh` reads `GH_CONFIG_DIR` BEFORE `HOME` (vc, 2026-09-12), so the isolation stays whole and the gh gate still passes. Without it the run stops at the LAST line of `preflight()` and no preview ever executes.

**THE LOAD QUESTION IS SETTLED: one variable, three readings -- red at 82.65, red at 84.38, GREEN at 20.98.** Run 3 reached `info: cargo test green`; the daemon family passes on this tree. Gate the run on a one-minute load read by the same script that then runs it (`sysctl -n vm.loadavg`, field 2), so the reading and the run are one event.

**Run 4 ran all fourteen previews at exit 0** and `info: dry-run complete -- no side effects` -- verified as a measurement, since the clone's tree was still clean afterwards.

**THE ONE FINDING, reported and NOT taken**: the release notes preview is `head -30` of a 59-line section, so `### Fixed` and `### Removed` are invisible and nothing says they were cut. What would PUBLISH is whole -- `gh` sends the file, not the preview -- but the preview's only job is to let a human check what ships. One line to fix; it changes the rehearsal HEAD, so it is vc's call.

**The clone survives** at `scratchpad/rehearsal` with its remotes and its deliberate backup. `~/.intent/home` and the live store's mtime identical before and after every run.

## TODO -- vc's five steps, serial, in this order

- **Step 4 is CLOSED and was never mine to run.** cc measured both Local shapes under AC-23.4 at subject `3ade8dea3` (report `b70a0f97e`) -- same toolchain and release profile as my grammar table, controls firing in both directions, wall-clock and ONNX linkage recorded -- and vc ruled under the pen that no Local runtime ships in 3.0.2. Both AC-20.4 and AC-23.4 compute satisfied. **I had announced the load and was one command from running two large dependency builds inside the quiet window to re-measure a settled question**; reading the design first is what stopped it. Two notes stand for whoever revisits shape B: cc measured NEWER versions than the brief named (fastembed 6.0.3 and candle 0.11.0 / tokenizers 0.22.2, against the brief's 4 and 0.9 / 0.21 -- the lockfile's resolution, and better numbers), and cc's note says the probe referenced the code path "and nothing executed", so the table is a SIZE measurement and does not claim the runtime answers.
- **Step 5 is RUN and halted to vc** -- see DOING. Nothing is owed on it until vc answers.
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
- **A GATE THAT CANNOT SAY _I COULD NOT MEASURE_ WILL SAY SOMETHING FALSE INSTEAD, AND A LOADED BOX IS THAT CASE.** Two rehearsal runs red at loads 82 and 84, the same bytes green across the whole workspace at 19 to 30 an hour before. The runs are real and they answer a question nobody asked: they cannot separate a defect in the cut from the host's load. Report the halt, report both loads, and do NOT re-run until one comes back green.
- **A COST MEASUREMENT IS NOT A CONSEQUENCE MEASUREMENT.** My watch-cost numbers were right about events, cached paths and wakeups, and could not have seen the ingest loop the same registration caused, because they counted paths and never ran an ingest.
- **A DEPENDENCY COMPILED BUT NEVER REFERENCED IS NOT IN THE BINARY.** `lto = "fat"` plus macOS dead-stripping drops it, so a size measurement that only adds the crate reads a real cost as ZERO. Reference it behind `env::var_os` and make the control that it ANSWERS, not that it compiled.
- **THE ARITHMETIC IS AN INSTRUMENT CHECK.** Five grammar deltas summed to MORE than a shared-runtime model allows, which is impossible -- and that impossibility is what revealed they excluded the tree-sitter runtime.
- **A FILE THAT "NEVER LANDS" STILL HAS TO BE GONE.** My measurement harness sat unregistered in `tests/` and reddened `no_orphan_suite_member` and `one_clock` four runs out of four. The guards run against the TREE, not the commit.
- **RESULTS COME BACK AS A PATCH, NEVER A WHOLE-FILE COPY** (vc, a rule of the cut). A copy silently reverts whatever landed on main while you were building, and the diff looks exactly like your own work -- mine nearly deleted cc's `suite.rs` registration.
- **A TEST ASSERTS ITS CLAIM, NOT ITS CONTAINER** (vc, a rule of the cut). Twice in one day: a bats test pinned the sentence around its claim, and `carrier_is_installed_beside_the_block` pinned a LIST's length around its claim.
- **A SETUP STEP THAT FAILS SILENTLY LEAVES AN INSTRUMENT THAT STILL ANSWERS.** `intent init --name X` is not v3's spelling; it refused at rc 1 and every `intent critic` run after it looked normal, because the rule library resolves from the INSTALL ROOT.
- **`Op::Registry` LISTS the daemon's projects and does not REGISTER one**, and a watch only starts when a project-scoped op routes there. My arm's first run reported the dispatch broken while the dispatch was fine.
- **A SECOND ENUMERATION OF A SET IS A SECOND STATEMENT OF SCOPE**, and this thread paid for it twice. Enumerate once, decide once.
- **RUN THE WHOLE REHEARSAL, NOT THE STEP YOU EXPECT TO FAIL -- AND IT CUTS BOTH WAYS.** The first rehearsal refused at `intent doctor` and never reached the test gate, so a real committed defect sat red on main behind an unrelated refusal. **The mirror cost more, 2026-09-12**: three rehearsals in a row stopped at the LAST line of `preflight()`, so every gate ran three times and the fourteen write-step previews -- the thing `--dry-run` exists for -- ran never. A green preflight reads like a green rehearsal.
- **A TRUNCATION THAT DOES NOT ANNOUNCE ITSELF IS A SILENT NARROWING, AND THE READER CONCLUDES ABSENCE.** The release notes preview is `head -30` over a 59-line section, so `### Fixed` and `### Removed` never appear and nothing says so. Same shape as the gates that refused without naming what refused (fixed in batch 4) and cc's `tail` on a failures block the same afternoon. Three instances in one file family in one day: when you cap output, print what was capped.
- **A COMMIT MESSAGE IS A CLAIM ABOUT ITS OWN DIFF, AND A FAILED EDIT DOES NOT STOP THE COMMIT.** My board edit aborted on a bad anchor before writing; the retry loop committed anyway, so `871886c45` describes a board update its diff does not contain. Check `git show --stat` against what the message says when a scripted edit and a scripted commit run in the same breath.
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
