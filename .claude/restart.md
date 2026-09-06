# Claude Code Session Restart

**This file is the ENTRY POINT and nothing else. The state lives in `intent/wip.md` (current work) and `intent/restart.md` (narrative + traps + conventions).** **It was three copies of one narrative until 2026-08-24, each opening with a banner saying it superseded everything below it -- which is the tell that nobody was deleting, only prepending.** That is the gate-figure defect at document scale: three homes carrying three values. **If you find yourself writing a supersedes banner, DELETE WHAT IT SUPERSEDES INSTEAD.**

## FIRST THING ON 2026-09-06, AND IT IS THREE MEASUREMENTS BEFORE ANY WORK

**The estate folded at 2026-09-06 00:25-00:32Z with all four boards cut for a COLD reader. Nothing below is a state you may trust -- each line is the command that regenerates it.**

1. **ASK THE OTHER NODES; DO NOT READ THEIR STATE OFF ANY BOARD.** Their headers say what they wrote at their own fold. **This went wrong twice on 2026-09-04 -- once invented, once decayed -- and both times a peer corrected it and nothing in the tooling would have.**
2. **MEASURE HOW FAR THE SHIPPED PAIR IS BEHIND THE TREE, BEFORE BELIEVING ANY BEHAVIOUR.** `intent --version` for the marker, then `git rev-list --count <marker>..HEAD -- native/rust surface`. **It read NINE at the fold.** A node standing on that binary sees the OLD behaviour for everything landed since it was built.
3. **RUN THE GATE, DO NOT TRANSCRIBE IT.** `intent ac gate ST0056` and `intent ac gate ST0068`. **Eleven rows were unsatisfied at the fold and SIX of them cannot move until hv cuts** -- they are unfalsifiable without a published artefact, not work anybody is withholding.

**WHAT IS WAITING ON hv, HEADED BY THE ONE THAT UNBLOCKS A NODE:** `ST0065`'s three rulings -- ic takes `WP-01` the moment they land and has had nothing startable for most of two days. Then `AC-11.7`'s re-mint, `0175` vs `unmigrated_surface`, `0267`'s strong form, `AT-10.5`'s re-weighed ask, the four canon rows, and the push. **The full ordered list is `intent/whiteboard/hv/inbox.vc.md` and the `## hv items` section of `intent/whiteboard/vc/wip.md`.**

**THE FOUR CANON ROWS ARE NOT FOUR UNFINISHED JOBS.** `0205` and `0272` are DELIVERED and waiting on a disposition; only `0268` and `0271` are filings awaiting a word. **Reading that list as a backlog puts someone on work that is already done.**

## First actions after `/compact` or a new session

0. **IN A FRESH CLONE ONLY, RUN `bin/int hooks` FIRST.** Hooks are TRACKED at `.githooks/` and reached by `core.hooksPath` -- **which is repo-local config that a clone does not inherit.** A fresh clone therefore has every hook body and runs none of them: the critic gate, both whiteboard guards, the canon-ignore guard, the append-only guard and all three formatters are silently inert. `bin/int hooks` reports; **`bin/int hooks --install` is the only thing that writes.**

   **NOTHING TRIGGERS THIS AUTOMATICALLY AND NOTHING CAN.** Git runs nothing on clone, deliberately, and a hook cannot report that hooks are off because it would not run either. **It applies to BOTH checkouts** -- this one and `~/Devel/prj/Intentv2`. What limits the damage is that CI catches the CONSEQUENCE when nothing catches the cause: unwired hooks mean unformatted code, and `cargo fmt --check` runs on every push.

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, chains `/in-whiteboard pickup`. Board: `hv`, `cc`, `dc`, `ic`, `vc`. Declared languages: elixir, author, content, rust, shell, swift. Solo unless launched as a node via `intent claude start <ws>`.

2. **Read `intent/wip.md`, then `intent/restart.md`.**

3. **Do not transcribe the gate figure from anywhere. Run the verb calls `intent/wip.md` names** -- and note it names them without a count, deliberately: **a pointer that carries a number rots when the list grows, and this line said _three_ while the list held five.**
