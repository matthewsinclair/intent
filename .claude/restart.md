# Claude Code Session Restart

**This file is the ENTRY POINT and nothing else. The state lives in `intent/wip.md` (current work) and `intent/restart.md` (narrative + traps + conventions).** **It was three copies of one narrative until 2026-08-24, each opening with a banner saying it superseded everything below it -- which is the tell that nobody was deleting, only prepending.** That is the gate-figure defect at document scale: three homes carrying three values. **If you find yourself writing a supersedes banner, DELETE WHAT IT SUPERSEDES INSTEAD.**

## FIRST THING IN ANY SESSION, AND IT IS THREE MEASUREMENTS BEFORE ANY WORK

**Nothing below is a state you may trust. Each numbered line is the command that regenerates it -- AND THIS FILE DELIBERATELY NO LONGER PRINTS WHAT THAT COMMAND LAST RETURNED.** It printed all three until 2026-09-08, beside the instructions to run them, and every one had rotted: the pair distance said _nine_ and was zero, the gate said _eleven_ and was twenty-eight, and a node described as having nothing startable had three items. **A number offered next to its own regeneration command gets read INSTEAD of running the command** -- which is what happened, to the node that wrote the numbers.

1. **ASK THE OTHER NODES; DO NOT READ THEIR STATE OFF ANY BOARD.** Their headers say what they wrote at their own fold. **This went wrong twice on 2026-09-04 -- once invented, once decayed -- and both times a peer corrected it and nothing in the tooling would have.**
2. **MEASURE HOW FAR THE SHIPPED PAIR IS BEHIND THE TREE, BEFORE BELIEVING ANY BEHAVIOUR.** `intent --version` for the marker, then `git rev-list --count <marker>..HEAD -- native/rust surface`. A node standing on a stale binary sees the OLD behaviour for everything COMPILED into it. **Non-zero is the hazard; zero is the ordinary case and is not a reason to skip the check next time.**

   **THIS ANSWERS FOR THE COMPILED HALF ONLY, AND SAID _everything_ UNTIL 2026-09-08.** Note the scope in the command -- `-- native/rust surface` -- which excludes `intent/plugins/`. Anything served from a SCRIPT is read live from the tree (`intent claude ws hygiene` -> `intent/plugins/claude/bin/intent_claude_cwi`, resolved at `intentsvcs/src/install.rs:361`), so a fix there is in effect at commit whatever the marker says. **Witnessed the day this line was written: the pair named `b070c158`, the fix was `cb04fe7d0`, `--is-ancestor` said NO, and the new behaviour was live** -- a peer nearly reported the fix as undelivered on the version string's word. **ASK THE BEHAVIOUR: for a part-compiled, part-scripted tool the version is confidently wrong about the scripted half rather than silent.**

3. **RUN THE GATE, DO NOT TRANSCRIBE IT.** `intent ac gate ST0056` and `intent ac gate ST0068`. **SIX rows cannot move until hv cuts, and NAMING them is the durable half:** `ST0056` AC-00.5, AC-07.7, AC-11.1, AC-11.4, AC-12.4 and `ST0068` AC-04.2 -- unfalsifiable without a published artefact, not work anybody is withholding. **Every other unsatisfied row is engineering somebody can start, and how many there are is a state. Run it.**

**WHAT IS WAITING ON hv, HEADED BY THE ONE THAT UNBLOCKS A NODE:** `ST0065`'s three rulings -- ic takes `WP-01` the moment they land. Then `AC-11.7`'s re-mint, `0175` vs `unmigrated_surface`, `0267`'s strong form, `AT-10.5`'s re-weighed ask, the four canon rows, and the push. **The full ordered list is `intent/whiteboard/hv/inbox.vc.md` and the `## hv items` section of `intent/whiteboard/vc/wip.md`.** **DO NOT INFER FROM THIS LIST THAT A NODE WAITING ON A RULING IS IDLE -- ask them.** That inference was written into this file as a fact and was wrong.

**THE FOUR CANON ROWS ARE NOT FOUR UNFINISHED JOBS.** `0205` and `0272` are DELIVERED and waiting on a disposition; only `0268` and `0271` are filings awaiting a word. **Reading that list as a backlog puts someone on work that is already done.**

## First actions after `/compact` or a new session

0. **IN A FRESH CLONE ONLY, RUN `bin/int hooks` FIRST.** Hooks are TRACKED at `.githooks/` and reached by `core.hooksPath` -- **which is repo-local config that a clone does not inherit.** A fresh clone therefore has every hook body and runs none of them: the critic gate, both whiteboard guards, the canon-ignore guard, the append-only guard and all three formatters are silently inert. `bin/int hooks` reports; **`bin/int hooks --install` is the only thing that writes.**

   **NOTHING TRIGGERS THIS AUTOMATICALLY AND NOTHING CAN.** Git runs nothing on clone, deliberately, and a hook cannot report that hooks are off because it would not run either. **It applies to BOTH checkouts** -- this one and `~/Devel/prj/Intentv2`. What limits the damage is that CI catches the CONSEQUENCE when nothing catches the cause: unwired hooks mean unformatted code, and `cargo fmt --check` runs on every push.

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, chains `/in-whiteboard pickup`. Board: `hv`, `cc`, `dc`, `ic`, `vc`. Declared languages: elixir, author, content, rust, shell, swift. Solo unless launched as a node via `intent claude start <ws>`.

2. **Read `intent/wip.md`, then `intent/restart.md`.**

3. **Do not transcribe the gate figure from anywhere. Run the verb calls `intent/wip.md` names** -- and note it names them without a count, deliberately: **a pointer that carries a number rots when the list grows, and this line said _three_ while the list held five.**
