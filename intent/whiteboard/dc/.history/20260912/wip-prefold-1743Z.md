---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 16:09Z
status: active
focus: "RUN 6 IS RED AND I HAVE HALTED. Second consecutive red on 2239998e3, cargo gate, two intentd fsevent arms -- one of them NOT in the roster I published. No third run, no diagnosis, no fixes: the decision is hv's. Reported to vc. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058, ST0069/22, ST0069/24]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-1600Z.md`.** Everything landed is carried by its commits and the CHANGELOG, not here.

## DOING -- nothing. RUN 6 IS RED AND THE HALT IS IN FORCE

**Subject `2239998e385f590f87c84538159bb9d580a124e9`. Run 6 went RED at the cargo gate at 16:04Z and I stopped there, which is the order.** Reported to `vc/inbox.dc.md` at 16:09Z. **No third run, no diagnosis drives, no fixes to the arms, no loosened bounds. The decision is hv's.**

**GATE READING 20.97, PEAK 65.68** -- sampled every 10s, which is the figure that should have been quoted from the start. The two arms:

- `daemon_subscriptions.rs:269` `every_subscriber_receives_every_event` -- "the first subscriber was told about `.../AGENTS.md` rather than the file that changed". Also red in run 5.
- `intentd/tests/suite.rs` `a_source_edit_reaches_the_index_and_costs_canon_nothing` at :96 -- "a source edit never reached the index". **NOT red in any earlier run**, and not in the roster I published.

**gh auth PASSED** -- `GH_CONFIG_DIR` worked and preflight reached cargo, further than runs 1-3 got. **The fourteen previews still did not run in this run**, because cargo went red first.

**The Rust tree is byte-identical to run 3's subject, measured**: `git diff --stat 25af41fba 2239998e3 -- native/rust/` is empty, run 3 printed `cargo test green` on those bytes, and the newly-failing arm was present and passing there. Two reds and one green on identical bytes.

**The store moved during run 6 and the pointer did not.** Live store `16:52:35 / 29954048` before, `17:01:59 / 30068736` after; clone tree `0 dirty path(s)`. Five peer commits landed inside the cargo window, including my own fold `d282bc02d`, and each wakes the live daemon's ingest -- a sufficient innocent cause I cannot separate from the rehearsal with this run's data. **So "no side effects" is NOT measured for run 6**, unlike runs 1-5.

## TODO

- Nothing of mine is owed beyond run 6's report. Every work package I claimed is closed: WP-22 at `783b9cc82`, WP-24 at `25af41fba`.
- **CHANGELOG**: ic writes the Added lines for the search packages; my Fixed lines stay mine.

## Holds

- **vc is DARK for hv's compact.** Every landing and report goes into `intent/whiteboard/vc/inbox.dc.md` with a same-turn `date -u` stamp as well as being messaged. **Anything needing a ruling WAITS in the inbox and is not guessed.**
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD, AND SO DOES A TODO SOMEBODY ELSE HAS ALREADY DONE.** Re-drive a hold's condition when you quote it; never read it off this line. **Driven the hard way 2026-09-12**: step 4 sat in my TODO in vc's own words while cc had measured it and vc had ruled on it, and it read exactly like live work -- I announced the load to two nodes and was one command from two large dependency builds inside a quiet window. **A board records what was true when it was written and the estate moves underneath it**, so the register and the design are the subject and this file is a pointer.

## Watch-outs

- **I DO NOT KNOW THE DAEMON FAMILY'S THRESHOLD, AND I SAID I DID.** I reported "one variable, three readings -- red at 82, red at 84, green at 21" and it was too clean: run 5 went RED at a gate reading of **13.19**, the lowest start-load of the day. `cargo test --workspace` drives the load ITSELF -- run 5 started at 13.19 and ended at 41.21 -- so a start-of-run reading describes the box before cargo, not what the daemon arms met minutes in. **That is a cost measurement used as a consequence measurement**, the trap already on this board, walked into with a different number. Run 6 samples every 10s and reports the PEAK, which is the figure that should have been quoted. **AND THE ROSTER I PUBLISHED WAS WRONG TOO, THE SAME WAY, THREE HOURS LATER.** I said every failure today was `daemon_subscriptions` or `daemon_watch`; run 6 reddened `a_source_edit_reaches_the_index_and_costs_canon_nothing`, which is neither and which cc had added at 12:55 that day. **Twice in one afternoon I enumerated what I had observed and published the enumeration as the population.** What still stands, stated as what it is: every failure so far is an fsevent-delivery arm inside `intentd`, no target outside `intentd` has failed once, and run 3 reached `cargo test green` on a byte-identical Rust tree -- so the family is not unconditionally broken, no threshold is known, and the arm roster is open rather than closed.
- **THIS HOST HAS NO IDLE.** With every Intent node silent the one-minute load floors around 10 to 15 -- `fileproviderd`, iTerm, App Tamer, three CoreSimulator processes, none ours to pause. "Alone" has meant one test target, never an idle host.
- **A FIELD SKIPPED ON SERIALISATION AND NOT DEFAULTED ON DESERIALISATION MAKES THE WHOLE ENVELOPE WRITE-ONLY.** `Hit::stale` skipped `false` with no `default`; serde supplies one for `Option` unasked and for NOTHING ELSE, so every hit in a normal answer made the envelope unreadable the first time anything read it back. Invisible while the type only ever went outwards. Found by DRIVING `--daemon search`, not by reading.
- **TWO DERIVATIONS THAT SHARE A MISTAKE ARE ONE DERIVATION.** `daemon_op_for`, `daemon_servable_paths` and the load-time `serving_op` check each walked `families` alone, and the test that checks the roster against the table walked one list too -- so a `serving_op` on a `new_surface` row would have been read by nothing, refused by nothing, and agreed about perfectly. A test written to be an independent derivation is independent of the FUNCTION, not of the assumption.
- **A GATE'S SUBJECT IS AS EASY TO GET WRONG AS ITS RULE, AND THE RULE BEING RIGHT HIDES IT.** My hook's first build read the correct freshness predicate against the wrong paths -- the ones the GREP searched rather than the ones its own ANSWER named -- so a grep confined to a clean directory passed and appended hits from a file that had moved. Found by driving the case; the code looked right.
- **RUN THE WHOLE REHEARSAL, AND IT CUTS BOTH WAYS.** An early refusal hides a later defect (the first rehearsal died at `intent doctor`). **The mirror cost more**: three runs stopped at the LAST line of `preflight()`, so every gate ran three times and the fourteen write-step previews ran never. A green preflight reads like a green rehearsal.
- **A TRUNCATION THAT DOES NOT ANNOUNCE ITSELF IS A SILENT NARROWING, AND THE READER CONCLUDES ABSENCE.** The notes preview was `head -30` over a 59-line section, so `### Fixed` and `### Removed` never appeared and nothing said so. Fixed at `2239998e3`. Third instance of that shape in this file family in one day, with the two test gates and cc's `tail` on a failures block -- when you cap output, print what was capped.
- **A COMMIT MESSAGE IS A CLAIM ABOUT ITS OWN DIFF, AND A FAILED EDIT DOES NOT STOP THE COMMIT.** A board edit aborted on a bad anchor before writing and the retry loop committed anyway, so `871886c45` describes an update its diff does not contain. Check `git show --stat` against the message when a scripted edit and a scripted commit run in one breath.
- **A SETUP STEP THAT SILENTLY DOES NOTHING LEAVES AN INSTRUMENT THAT STILL ANSWERS.** `intent index refresh` does not exist (`rebuild` does) and my "control" ran nothing while reading as informative; `intent init --name X` is not v3's spelling and refused at rc 1 while every `critic` run after it looked normal. Twice in one day.
- **A COST MEASUREMENT IS NOT A CONSEQUENCE MEASUREMENT.** My watch-cost numbers were right about events, cached paths and wakeups and could not have seen the ingest loop the same registration caused, because they counted paths and never ran an ingest.
- **A DEPENDENCY COMPILED BUT NEVER REFERENCED IS NOT IN THE BINARY.** `lto = "fat"` plus macOS dead-stripping drops it, so a size measurement that only adds the crate reads a real cost as ZERO. Reference it behind `env::var_os`.
- **A FILE THAT "NEVER LANDS" STILL HAS TO BE GONE.** An unregistered harness in `tests/` reddened `no_orphan_suite_member` and `one_clock` four runs of four. The guards run against the TREE, not the commit.
- **RESULTS COME BACK AS A PATCH, NEVER A WHOLE-FILE COPY** (vc, a rule of the cut). A copy silently reverts whatever landed while you were building and the diff looks exactly like your own work.
- **A TEST ASSERTS ITS CLAIM, NOT ITS CONTAINER** (vc, a rule of the cut).
- **A SECOND ENUMERATION OF A SET IS A SECOND STATEMENT OF SCOPE.** Enumerate once, decide once.
- **A TEMPLATE OR SHELL PAYLOAD EDIT IS DRIVEN WITH THE BATS SUITE**; its text is asserted there and nowhere in cargo.
- **A DISCIPLINE ON YOUR BOARD IS NOT A FLAG ON YOUR COMMAND LINE.** I wrote "under an isolated HOME" and had set none.
- **A GATE THAT READS A GITIGNORED PATH CANNOT BE REHEARSED IN A CLONE.** Take `intent backup` in the clone deliberately and say why.
- **`git stash` IS SHARED ACROSS EVERY WORKTREE OF ONE REPO.** Control a diff with `git diff > patch; git checkout -- <paths>; git apply patch`.
- **NEVER run a formatter over a file you are editing by hand.** The register is edited BY POSITION and only its markdown regenerated.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call**, and on a lock refusal re-issue the SAME command. A retry loop is worth having; peers land constantly.
- **Every suite and build from a private worktree with its IN-TREE target dir** under an isolated HOME, with `CARGO_HOME` pointed at the real one.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output.**
- **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split.** Messages and commit bodies go in a file, through `-F`.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
