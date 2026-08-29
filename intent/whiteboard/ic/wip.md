---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 13:13Z
status: active
focus: "vc ASSIGNED 2 THINGS. (1) PACKAGE THE PORTABLE METHOD -- Lamplight`s own nodes run it, not me. DONE: method-ingest-damage.md + tools/ingest_damage_probe.py, 8 self-test arms, each verified capable of failing. (2) SURFACE-REVIEW cc`s ac/at package AFTER it lands -- NOT STARTED, waiting on cc. **MY 257 WAS WRONG: IT IS 80. Lamplight 25 not 145, Baize now largest at 28.** Per-path scanning counted one thread once per status bucket."
claims: [ST0065, ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. Nothing of mine dirty.**

## DONE THIS SESSION

- **ST0065 retirement COMPLETE.** Step 3 canon delete `ae6a83ce`; step 4 verified -- canon 23, both skills absent from the tool's listing, no orphans, zero live references, `in-session` control at 124. hv ran the two out-of-repo commands themselves.
- **Fleet ingest-damage survey.** `survey-ingest-damage.md` (committed) + `tools/{splice_scan,ingest_conserve,fleet_exposure}.py`.

## THE ONLY THINGS FROM THE SURVEY THAT MUST SURVIVE A COMPACT

**The report holds everything else. Read it before acting on any of this.**

1. **`0133` is a REPRESENTABLE-STATE REGRESSION, not a parser bug.** `AcState::Unsatisfied` is a UNIT VARIANT (`model.rs:1233`), so v3 cannot represent _unsatisfied-with-evidence_, which v2 could author. The wildcard at `legacy.rs:1707` destroys it. **Three paths destroy it and only `ac unsatisfy` tells the user** -- `ac new` and the migration are silent (`IN-AG-NO-SILENT-001`).
2. **The v2 comparison source is in GIT HISTORY at the ingest's own input path** (`legacy.rs:1273` reads `acceptance.md`, which the v3 generated view then overwrote). Conflab's `acceptance.v2.md` was a preserved COPY. **This is what made the whole survey possible.**
3. **FLEET EXPOSURE 257 real rows; LAMPLIGHT 145 of them.** All PREDICTED-UNCONFIRMED except Conflab, where my predictor returned exactly the 14 conflab-vc had measured. **Lamplight is not ours to survey; routing is vc's.**
4. **INTENT IS THE CORPUS THE PARSER WAS FITTED TO** (`legacy.rs` born 2026-08-16 debugged ON this estate; hop 08-19). **Never use Intent as a fleet baseline or calibration control.**

## THE 257 IS WRONG: IT IS 80 (2026-08-29 13:0xZ, found while PACKAGING the method)

**I SCANNED PER-PATH WHEN THE UNIT IS PER-THREAD -- 3.2x INFLATION.** v2 status buckets were collapsed into a flat layout before the hop, so one thread has several historical paths each frozen at a stale verdict. **Lamplight: 678 paths, 358 threads, 155 multi-path.** `ST0206 AC-01.2` counted 3x byte-identical; `ST0052 AC-01.2` read `satisfied: no` at a July snapshot and `satisfied: yes` post-collapse -- **satisfied BEFORE the hop.**

**Lamplight 145 -> 25. Baize 43 -> 28 and is now the LARGEST. Riffle/Courses/Devbin 29 rows -> ZERO, entirely artifact. FLEET 257 -> 80.** All still PREDICTED-UNCONFIRMED.

**THE CONFLAB CONTROL PASSED IN FULL WHILE EIGHT ESTATES WERE WRONG** -- unchanged at 14, because its history produced no stale bucket snapshots. **The control was genuine, unarranged, and blind to the broken axis. Third instance today; first where the blind control was mine.**

## CORRECTION TO MY OWN SURVEY (re-measured at HEAD ccf52f68, sent to vc 12:57Z)

**`ac new` destroys FOUR payload-carrying states, not just evidence.** `facade.rs:4481` builds a FRESH `Criterion` and `put`s it; `state:` comes from `kind` alone and the existing row is never read. At HEAD `AcState` has six variants and four carry a payload -- `Satisfied{evidence}`, `Descoped{..}`, `Withdrawn{..}`, `Fiat(..)`. **`Fiat` is dc's from this morning (`b7a3e771`): landing a new payload variant silently WIDENED this path, and nothing in ST0066 had reason to look at `ac_new`.**

**THE FIX PRECEDENT IS ONE FUNCTION BELOW AND WAS APPLIED TO AT ROWS ONLY.** `at_new` carries `note`/`legacy` off `st_show` (`8daca5f1`), on the rule **"a verb with no way to express a value has no way to express erasing one either, so absence can only mean not-saying"** -- written after six ST0061 notes died to one re-cite. Same argument, verbatim, unapplied to `ac_new`. `at_new` also contract-checks the prospective row; `ac_new` checks nothing.

**cc's ruled package (refuse a taken key in `ac new`) would close this BY CONSTRUCTION** -- the destroying call IS the call on an existing id. Not new work; it raises what cc already holds from ergonomics to a live silent-destruction fix. **Not claimed by me, nothing filed.**

## NEXT

**Take instruction from vc.** Offered and not started: **confirming Lamplight's 145 rather than predicting them** -- the method is established and per-estate. vc may prefer it go to Lamplight's own nodes.

## Watch-outs

**THE CLASS, AND TODAY IT COST ME FIVE: I LET A MEASUREMENT STAND AS A FINDING WITHOUT READING WHAT IT COUNTED.** 23 "lost segments" that were all my own parser artifacts; 12 splice hits that were authored repetition; 30 evidence hits that were healthy unsatisfied rows; 98 findings that were later authoring; 136 fleet rows that were template scaffold. **Every single one dissolved on being read. None survived.**

**A DETECTOR CAN BE ANTI-CORRELATED WITH THE DAMAGE, AND THEN ITS ZERO IS BY CONSTRUCTION.** I refined to `satisfied AND no evidence` -- the population the ingest PROTECTS -- reasoning that unsatisfied rows lack evidence _because they are not satisfied yet_. **That is the story the damage manufactures.** vc made the identical error independently and reported it to hv as clean. **AGREEMENT BETWEEN TWO INSTRUMENTS IS NOT EVIDENCE WHEN BOTH SHARE THE BLIND SPOT.**

**A FALSE PREMISE CARRYING URGENCY REORDERS SOMEONE ELSE'S QUEUE BEFORE ANYONE CHECKS IT.** I claimed the fleet was "queued to receive" the damage. **The port is closed; all 16 are on 3.0.0.** I inferred it and never ran the one-command ledger check. **Worst-shaped error of the day -- not because it was wrong but because it was urgent AND wrong.**

**THE ARTEFACT IN FRONT OF YOU IS NOT THE ARTEFACT THAT SHIPS.** Read source with `git show HEAD:`, never the worktree -- dc's uncommitted ST0066 work sits in `facade.rs` and cc committed two of its hunks by accident (`0134`) precisely this way. **`--only` takes the WORKTREE.** Three sub-cases: a version string is not a binary identity; a generated view is CURRENT and still not authoritative; a dirty tree is not HEAD.

**"UNMEASURABLE" AND "NOT EXPOSED" ARE DIFFERENT FACTS.** Also "absent file" vs "absent banner", and "empty directory" vs "installed skill". **A check that cannot separate two states reports the wrong one silently.**

**zsh: AN UNQUOTED `--include=*.md` ABORTS THE WHOLE COMMAND** and the pipeline prints a plausible 0. Quote every glob.

**MEASURE ONLY meant nothing was repaired and nothing staged for repair.** Held all session across every estate.

## Decisions

- **(hv, via vc) ALL ST0065 VERDICTS ADOPTED**; retirement complete and verified.
- **(ic) A RETIREMENT'S DELETE IS ITS LAST STEP** -- `sync` never prunes a vanished canon source (`skills.rs:690-697`). **And a `sync` in the sequence forces sync-BEFORE-uninstall**, which my draft got wrong.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING** -- the template engine has no include form, so AGENTS.md gets duplicate-plus-drift-test.
- **(ic) EXPOSURE IS NOT DAMAGE, AND A PREDICTOR IS NOT A CONFIRMATION.** Say **predicted-unconfirmed** in those words.
- **(all nodes) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
