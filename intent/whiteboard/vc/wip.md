---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 13:03Z
status: active
focus: "**FOLDED 2026-08-25 13:03Z FOR A COMPACT. hv HOLDS THE NEXT MOVE AND ST0060's DESIGN RUBBER-DUCK IS THE NEXT SESSION.** Day record in `.history/20260825/wip-fold-1301Z.md`; this board carries only what is still live. **THREE CI REDS TODAY, ALL MINE, AND TWO OF THEM ARE ONE FAILURE: I acted on a tree four nodes share as though it were mine alone.** `git add -A` published cc's in-flight refactor after cc had reported the SAME sweep to me that morning. **hv's ruling is the day's biggest finding: careful measurement of an unasked question costs more than careless measurement of a real one.**"
claims: [ST0056, ST0057, ST0058]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

Nothing in flight. Tree clean, refs level at `3c2f50d6`. **CI was in progress on cc's fix when I folded -- CONFIRM IT GREEN BEFORE REPORTING IT GREEN.**

## TODO

1. **`git add -A` IS DEAD IN THIS REPO. EXPLICIT PATHS ONLY.** It swept cc's ST0059 mutation into `693fa19c` and then 253 lines of cc's in-flight refactor into `f68d397c`, publishing a half-landed feature to public main. **cc reported the first instance TO ME and I fixed the instance without generalising it to my staging.**
2. **RUN `cargo test --workspace` BEFORE PUSHING ANYTHING TOUCHING `native/rust`, AND NEVER SAY "CLEAN".** prepush runs build + fmt + clippy and NOT tests; hv ruled it should stay that way. Say what was checked.
3. **ST0060 -- `intent vault` -- DESIGN RUBBER-DUCK WITH hv, NEXT SESSION. NOT TO BE STARTED.** Post-3.0.0. hv's direction: put it in `intentd`, master password in an OS-specific store, **invent NO new crypto or auth lanes**. Lamplight's requirements are at `Lamplight:intent/st/NOT-STARTED/ST0358/requirements.md`, twelve requirements from MEASURED consumers. **THE TWO CONSTRAINTS I WOULD BRING TO THE TABLE:** (a) `intentd` is WP-08, **Not Started, XL, a 49-line `main.rs` stub** -- so the daemon is a dependency, not a home; (b) Lamplight's R7 says **CI must never require the vault**, and there is **NO universal Linux equivalent to macOS Keychain** -- `secret-tool` needs a D-Bus keyring daemon, which fails exactly where servers and CI live.
4. **FIVE WITH hv, NONE MINE TO DECIDE:** A1 the deciding check (a commit from a session started after 12:03 settles whether `includeCoAuthoredBy` reaches the trailer); A3 WP-15 timing; A4 `fileindex`; A5 `--force` version mismatch; A6 ST0058's contract; A7 **TODO 8 ordering -- rule BEFORE 0077's wiring, or the two-writer shape arrives by construction**; A8 dc's three. **A2 is ruled and waits on hv's word to dc IN dc's OWN SESSION.**
5. **`~/.claude/skills/` IS ONE MACHINE-GLOBAL DIRECTORY UPSTREAM OF 15 COMMITTED `AGENTS.md` FILES.** Six estates still carry the stale description. **`skills sync --force` FROM A v2 ESTATE REGRESSES ALL OF THEM** -- I broadcast that flag as a remedy and devbin-cc caught it. **This reshapes what WP-15 IS: not a tidy-up of 25 files in one repo.**
6. **CARRIED FROM BEFORE TODAY:** AC-08.5's denominator is in doubt on the FIELD axis (my ruling, cc's escalation, ic's 0077, my 0080 -- three instances, three directions). ST0058 is WIP with ZERO ACs. `declared_but_unwired` adequacy. The marker's per-crate staleness.

## WATCH-OUTS -- vc's OWN

1. **THE WRONG TREE IS REACHABLE BY DEFAULT FROM FOUR UNRELATED DIRECTIONS.** `$INTENT_HOME` resolution, a line number off v2's `--help`, `git log --all` crossing into `v2-maintenance`, and devbin-cc hitting the same instrument the opposite way. **Four instances across two estates in one day. Not carelessness four times.**
2. **ZSH DOES NOT WORD-SPLIT AN UNQUOTED `$var`.** Four times today. It told me a built verb was unbuilt, a real hazard was not real, and 8 carriers were 0. **cc hit the mirror image: an unquoted path list arrived as ONE pathspec and the commit silently did not happen.** The safe form and the unsafe form fail in opposite directions and only one tells you.
3. **AN EMPTY POPULATION REPORTS CLEAN.** `--since` matched zero commits and my loop returned `0 with, 0 without`. **Print the size before reading the verdict.**
4. **ONLY INVOCATION IS EVIDENCE.** Family bare reads healthy; leaf `--help` renders full clap usage; leaf invoked returns rc=2. **`claude ws` looked fine at two of three levels.**
5. **A TRUE REPORT ABOUT A STATE THE READER CREATED** (cc's). `sync` told me store and extract agreed -- true, because I had destroyed my own edit with `--to-disk` first -- and I had a high-severity issue against it half-written.
6. **NEVER HAND-EDIT A GENERATED VIEW.** Twice today. `view_skew_check` caught `dispatch-table.md` at commit; it does NOT cover WP `info.md`, so that one passed the gate and an unrelated `st hold` reverted it hours later.

## DECISIONS -- LIVE ONLY

- **2026-08-25 13:03Z -- CAREFUL MEASUREMENT OF AN UNASKED QUESTION COSTS MORE THAN CARELESS MEASUREMENT OF A REAL ONE**, because nothing in the rigour tells you the subject was never in scope. hv's, from _not something that I suggested we go looking for_. ic's mechanism: **adjacency to a real finding is what makes the invented one feel commissioned.**
- **2026-08-25 13:03Z -- A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.** I refused devbin-cc's relay at 08:21Z and wrote the rule onto hv's board; nine hours later I told dc to build on hv's word given in MY session. **dc refused. The discipline does not get to bind you and not me on the same afternoon.**
- **2026-08-25 13:03Z -- A GUARD'S REMEDY LINE INHERITS THE GUARD'S AUTHORITY WITHOUT INHERITING ITS CHECKS** (ic, generalising dc). **And dc's own form is the stronger one: two rostered guards, one instructing a node to do what the other exists to prevent -- a property of the ROSTER that neither guard can see, because each is correct in isolation.**
- **2026-08-25 13:03Z -- A RATIFIED RULING IS NOT AN EXECUTED ONE.** treeindex, ten days. Cause was PACKAGING: T0 needed nothing and inherited the start date of the tier that needs FTS5.
- **2026-08-25 13:03Z -- AN AMENDMENT THAT UNBLOCKS BY RAISING THE BAR IS NOT THE SHAPE THE TELL WARNS ABOUT.** AC-11.6: dc named the conflict of interest against themselves and routed rather than acted; the amendment is STRICTER, which is checkable rather than a matter of trust.
- **2026-08-25 13:03Z -- A MONIKER NAMES THE ESTATE A NODE LIVES IN AND SAYS NOTHING ABOUT THE ESTATES ITS BYTES LAND IN** (devbin-cc). `~/.claude/` is one directory for every checkout on this machine.
