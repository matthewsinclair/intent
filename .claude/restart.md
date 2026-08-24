# Claude Code Session Restart

**This file is the ENTRY POINT and nothing else. The state lives in `intent/wip.md` (current work) and `intent/restart.md` (narrative + traps + conventions).** **It was three copies of one narrative until 2026-08-24, each opening with a banner saying it superseded everything below it -- which is the tell that nobody was deleting, only prepending.** That is the gate-figure defect at document scale: three homes carrying three values. **If you find yourself writing a supersedes banner, DELETE WHAT IT SUPERSEDES INSTEAD.**

## First actions after `/compact` or a new session

0. **IN A FRESH CLONE ONLY, RUN `int hooks` FIRST.** Hooks are TRACKED at `.githooks/` and reached by `core.hooksPath` -- **which is repo-local config that a clone does not inherit.** A fresh clone therefore has every hook body and runs none of them: the critic gate, both whiteboard guards, the canon-ignore guard, the append-only guard and all three formatters are silently inert. `int hooks` reports; **`int hooks --install` is the only thing that writes.**

   **NOTHING TRIGGERS THIS AUTOMATICALLY AND NOTHING CAN.** Git runs nothing on clone, deliberately, and a hook cannot report that hooks are off because it would not run either. **It applies to BOTH checkouts** -- this one and `~/Devel/prj/Intentv2`. What limits the damage is that CI catches the CONSEQUENCE when nothing catches the cause: unwired hooks mean unformatted code, and `cargo fmt --check` runs on every push.

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, chains `/in-whiteboard pickup`. Board: `hv`, `cc`, `dc`, `ic`, `vc`. Declared languages: elixir, author, content, rust, shell. Solo unless launched as a node via `intent claude start <ws>`.

2. **Read `intent/wip.md`, then `intent/restart.md`.**

3. **Do not transcribe the gate figure from anywhere. Run the three verb calls** -- `intent/wip.md` names them.
