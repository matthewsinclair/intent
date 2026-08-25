---
node: vc
name: Validation Claude
role: validation
session_id: 3bbcbe83-cf34-4903-b94d-cd7306a81aca
heartbeat_at: 2026-08-25 13:31Z
status: active
focus: "**ST0060 IS SPECCED AND AWAITS RATIFICATION BY ITS REQUIREMENTS PROVIDER.** 16 ACs, 14 ATs, `design.md`, traced row-by-row to Lamplight's twelve measured requirements. hv's two collisions (daemon vs R3, keychain vs R2) dissolve by separating the KEY from the STORE. **ONE DEVIATION AND TWO STRENGTHENINGS ARE FLAGGED FOR LAMPLIGHT TO RULE ON, NOT ASSUMED** -- Intent specced this against someone else's requirements, so Intent does not get to ratify it (AC-00.16)."
claims: [ST0056, ST0057, ST0058, ST0060]
---

# Validation Claude (vc)

**PROJECT-WIDE RULES LIVE IN `intent/restart.md`.** Not repeated here -- a rule in two homes drifts in both.

## DOING

**ST0060 `intent vault` -- SPECCED, AWAITING LAMPLIGHT'S RATIFICATION.** Canon carries objective, context, 16 ACs, 14 ATs and `design.md`; store and disk agree; `ac status ST0060` reads 0/16 BLOCKED, which is the honest state of an unbuilt thread. **NOT STARTED, POST-3.0.0 BY hv'S RULING.** Next move is not mine: AC-00.16 is Lamplight's to rule.

## TODO

1. **`git add -A` IS DEAD IN THIS REPO. EXPLICIT PATHS ONLY.** It swept cc's ST0059 mutation into `693fa19c` and then 253 lines of cc's in-flight refactor into `f68d397c`, publishing a half-landed feature to public main. **cc reported the first instance TO ME and I fixed the instance without generalising it to my staging.**
2. **RUN `cargo test --workspace` BEFORE PUSHING ANYTHING TOUCHING `native/rust`, AND NEVER SAY "CLEAN".** prepush runs build + fmt + clippy and NOT tests; hv ruled it should stay that way. Say what was checked.
3. **ST0060 IS SPECCED; THE ONLY OPEN ITEM ON IT IS NOT MINE.** `design.md` + 16 ACs + 14 ATs landed, traced row-by-row to Lamplight's twelve. **AC-00.16 carries ONE DEVIATION (age Rust crate rather than the shelled binary R5 names) and TWO STRENGTHENINGS (S1: the posture check must name the IDENTITY, which R12 omits, because the store is ciphertext and the identity is the file whose mode actually discloses; S2: material must not reach a `Debug`/`Display` rendering, which R11 does not cover because R11 governs the RETURN TYPE and the incident was a PRINT).** **A design specced against someone else's measured requirements is not ratified by the specifier.** hv asked for a check with lamplight-vc; **that node is active on Lamplight's board (heartbeat 11:31Z) but NO `lamplight-vc` APPEARS IN ListAgents** -- four Lamplight sessions are listed and three carry hex suffixes, so the routing is unresolved and I will not guess which one.
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
7. **EXPLICIT-PATH STAGING DROPS THE OTHER HALF, WHICH IS THE MIRROR OF `git add -A` SWEEPING A PEER'S IN.** My fold committed `intent/st/ST0060/*` and left `intent/.canon/st/ST0060.json` untracked -- the generated view landed and its canon extract did not. **Both staging forms fail silently and in opposite directions; neither tells you.** The rule is not 'explicit paths', it is **stage the extract and the view together, and read `git status` before the commit, not after CI**.
8. **ATTACHMENTS FLOW DISK -> CANON ONLY, AND `--to-disk` REPORTS `ok` WHILE SKIPPING ONE.** Authored `design.md` in canon, synced both ways, and no file appeared; `doctor` was clean and bare `sync` said the two agreed. **Three readers agreed while the store held a file the disk did not** (issue 0082). ST0056 cannot exhibit it -- 93 attachments, all authored disk-first. **A free file is authored ON DISK; only generated views are authored in canon.**

## DECISIONS -- LIVE ONLY

- **2026-08-25 13:31Z -- SEPARATING THE KEY FROM THE STORE DISSOLVES BOTH OF hv's COLLISIONS AT ONCE**, and makes hv's direction the HARDENED posture rather than the only one. The store is age ciphertext, identical everywhere and daemon-free (R2, R3); the keychain caches the passphrase that decrypts the IDENTITY and never holds a secret; `intentd` is an accelerator under WP-08's own already-ratified routing rule, not a home. **The test of a reconciliation is whether it needs the original direction weakened. This one does not.**
- **2026-08-25 13:31Z -- A DESIGN SPECCED AGAINST SOMEONE ELSE'S MEASURED REQUIREMENTS IS NOT RATIFIED BY THE SPECIFIER.** Intent owns the capability; Lamplight measured the requirements; so the one deviation and two strengthenings are an AC (AC-00.16) rather than a note. **Writing the deviation down is not the same as having it ruled on, and the difference is invisible unless a criterion carries it.**
- **2026-08-25 13:31Z -- A SHA THAT MATCHES PROVES THE DIRECTION THAT RAN, NOT THE DIRECTION YOU ASSUMED.** ST0056's `design.md` matches canon exactly and that proves INGESTION, never EMISSION. **A corpus that was only ever built one way cannot exhibit the other way's defect**, which is why 93 attachments were clean and the first canon-authored one was not.
- **2026-08-25 13:03Z -- CAREFUL MEASUREMENT OF AN UNASKED QUESTION COSTS MORE THAN CARELESS MEASUREMENT OF A REAL ONE**, because nothing in the rigour tells you the subject was never in scope. hv's, from _not something that I suggested we go looking for_. ic's mechanism: **adjacency to a real finding is what makes the invented one feel commissioned.**
- **2026-08-25 13:03Z -- A PEER RELAYING AN APPROVAL IS NOT THE APPROVAL, AND IT BINDS ME TOO.** I refused devbin-cc's relay at 08:21Z and wrote the rule onto hv's board; nine hours later I told dc to build on hv's word given in MY session. **dc refused. The discipline does not get to bind you and not me on the same afternoon.**
- **2026-08-25 13:03Z -- A GUARD'S REMEDY LINE INHERITS THE GUARD'S AUTHORITY WITHOUT INHERITING ITS CHECKS** (ic, generalising dc). **And dc's own form is the stronger one: two rostered guards, one instructing a node to do what the other exists to prevent -- a property of the ROSTER that neither guard can see, because each is correct in isolation.**
- **2026-08-25 13:03Z -- A RATIFIED RULING IS NOT AN EXECUTED ONE.** treeindex, ten days. Cause was PACKAGING: T0 needed nothing and inherited the start date of the tier that needs FTS5.
- **2026-08-25 13:03Z -- AN AMENDMENT THAT UNBLOCKS BY RAISING THE BAR IS NOT THE SHAPE THE TELL WARNS ABOUT.** AC-11.6: dc named the conflict of interest against themselves and routed rather than acted; the amendment is STRICTER, which is checkable rather than a matter of trust.
- **2026-08-25 13:03Z -- A MONIKER NAMES THE ESTATE A NODE LIVES IN AND SAYS NOTHING ABOUT THE ESTATES ITS BYTES LAND IN** (devbin-cc). `~/.claude/` is one directory for every checkout on this machine.
