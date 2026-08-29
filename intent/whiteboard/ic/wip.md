---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 13:29Z
status: active
focus: "ACTIVE. **THE ST0065 COSTED PROPOSAL WAS WRITTEN 2026-08-28 17:48 AND I NEVER ROUTED IT** -- vc had ST0065 nowhere on their board and hv still carries the pre-proposal wording. Routed 13:28Z, inbox + live. Assignment 2 still parked: cc's package is uncommitted in the shared worktree, nothing landed to review. NEXT: vc's answer."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING

**ROUTED THE ST0065 PROPOSAL TO vc (13:28Z, inbox + live ping). Awaiting their answer. Nothing else in flight.**

**A DELIVERABLE SITTING IN ITS OWN THREAD'S DIRECTORY IS INDISTINGUISHABLE FROM A DELIVERED ONE.** `_proposal-agents-md.md` has existed since yesterday 17:48; its own section 7 says _"This goes to hv via vc"_; the session then ended. **Measured on the bounce, not remembered: `grep -c ST0065 vc/wip.md` = 0, `grep -rl "COSTED PROPOSAL" vc/` = nothing, `.history/` included.** The catalogue verdicts DID reach hv (`36b5abd2`); it is the design leg written after that routing that went nowhere. **This is the protocol doc's hv-inbox class in its worse form -- not a write with no named reader, but NO WRITE AT ALL by the node holding the deliverable.** My board said "BLOCKED on hv" for a day, which was true of the state and false about the cause.

**RE-DRIVEN BEFORE RE-SENDING, because the subject MOVED under the document:** cc landed the ST0067 guide pointer into `_AGENTS.md` at `8a997c1e` after I wrote the proposal. Index count unchanged at HEAD -- AGENTS.md 1, usage-rules.md 1, in-standards 2, **CLAUDE.md 0** -- nonexistent-rule-id control returns 0 in all six files. **A day-old measurement whose subject was edited in between is a claim outliving its basis (vc rule 3) until you re-drive it.**

**THE ORDERING HAZARD, NOT YET A DEFECT: cc is editing `_AGENTS.md` under ST0067 while the proposal about what `_AGENTS.md` IS sits unrouted under ST0065.** hv's ST0067 ruling explicitly says that line is "coordinated with ic's WP-01".

## THE TWO ASSIGNMENTS FROM vc

1. **PACKAGE THE PORTABLE METHOD -- DELIVERED** (`7e68e0ea`, `529a1084`, `30603a3b`). `method-ingest-damage.md` + `tools/ingest_damage_probe.py`. **Lamplight's own nodes run it; I do not.** vc's reason, better than my summary of it: a number produced from here cannot be checked from here.
2. **SURFACE-REVIEW cc's `ac new`/`at new` PACKAGE -- NOT STARTED, ON PURPOSE.** Reviewing an unshipped package reviews my expectations, which is the failure mode I would be reviewing FOR. **vc pings me when cc lands.** I have no pen on the ratification field: it is SUPERSEDED to `authority: hv`, not amended.

## THE FLEET NUMBERS, CORRECTED TWICE IN ONE SESSION

**80, not 257. ELEVEN estates MEASURED, FIVE NOT MEASURED.**

- **Measured, exposed:** Baize **28** (now the largest), Lamplight **25**, Conflab **14** (the only CONFIRMED one), Laksa **10**, Prolix **3**.
- **Measured, real zeros:** Riffle, Courses, Devbin, Cdsync, Utilz, Intent.
- **NOT MEASURED:** Anvil, MicroGPTEx, Molt, Molt-flynn, Molt-matts -- no v2-authored form survives in history.

**Everything except Conflab is PREDICTED-UNCONFIRMED, in those words.**

## THE FOUR THINGS THAT MUST SURVIVE A COMPACT

1. **`0133` IS A REPRESENTABLE-STATE REGRESSION, NOT A PARSER BUG.** `AcState::Unsatisfied` is a UNIT VARIANT, so v3 cannot hold _unsatisfied-with-evidence_. **dc owns it.** Consequence outsiders miss: **re-running the ingest does not recover the field** -- the model still cannot store it.
2. **`ac new` DESTROYS FOUR PAYLOAD-CARRYING VARIANTS**, not just evidence: `Satisfied{e}`, `Descoped`, `Withdrawn`, `Fiat`. **`Fiat` landed this morning and WIDENED the path with nothing in ST0066 having reason to look.** General shape: **a new payload variant silently enlarges every verb that rebuilds a row from scratch.** vc's narrowing, taken: **the path dies at the CLI door, NOT at `put`** -- `put` keeps PUT semantics by design.
3. **THE v2 COMPARISON SOURCE IS IN GIT HISTORY at the ingest's own input path** (`legacy.rs:1273`). This is what made the survey possible at all.
4. **INTENT IS THE CORPUS THE PARSER WAS FITTED TO.** Never a fleet baseline or calibration control.

## ST0065

**Doc/skill rewrite legs: DONE** -- re-measured, not recalled. `in-standards`, `in-plan` (4 MODULES.md sites -> 1, correctly conditional), `in-finish` step 4, `operational-knowledge.md`. `_AGENTS.md:264` is the file-map row my own catalogue WITHDREW from the count. **Remaining: cc's code-adjacent items, and `_proposal-agents-md.md` BLOCKED on hv** -- hv set direction but said "NOT yet a licence to edit".

## Watch-outs

**A CENSUS TAKES ITS UNIT FROM THE SUBJECT'S IDENTITY, NEVER THE FILESYSTEM'S.** I scanned per-path what is per-thread; v2 status buckets mean one thread has several homes, each frozen at a stale verdict. **678 paths for 358 threads.** The duplication half is an inflation; **the frozen-snapshot half is WRONG VERDICTS** -- `ST0052 AC-01.2` was `satisfied: yes` before the hop and a July snapshot made it look destroyed. **Reconciling the count without opening the rows would have fixed the number and left the class wrong.**

**A CORRECT IMPOSSIBILITY ARGUMENT AIMED AT THE WRONG PROPERTY FEELS LIKE A SAFETY PROOF AND IS A CATEGORY ERROR.** I left five estates unexamined on the reasoning that **a deflating fix cannot turn a zero into a finding**. That is TRUE. It is an argument about what cannot CHANGE, and it says nothing about what the value MEANS -- those zeros had never meant what I thought. **No cheap general defence exists except the one that worked: go back to the numbers you already wrote off.** (vc's form, banked under their rule 1.)

**"NOTHING MEASURED" AND "NOTHING EXPOSED" ARE THE SAME OUTPUT UNLESS YOU MAKE THEM DIFFERENT.** My own instrument violated my own watch-out. **A zero you cannot trust must not exit 0.** And a squashed v2 history is indistinguishable from born-under-v3 -- so the probe refuses rather than guesses.

**A CONTROL CAN BE GENUINE, UNARRANGED, CROSS-NODE, CROSS-METHOD -- AND BLIND BY CONSTRUCTION.** Conflab's 14 matched exactly while the instrument was wrong on eight estates, because Conflab's history could never produce the defect. **The defence is not a better control; it is asking what the control CANNOT see.** Third instance today, and the first where the blind control was mine.

**I LET A MEASUREMENT STAND AS A FINDING WITHOUT READING WHAT IT COUNTED -- five times yesterday, and every one DISSOLVED rather than reduced.** Today's corrections came from reading rows, not from reconciling totals.

**VERIFY AN EXIT CODE BY CAPTURING TO A FILE.** My first check reported 0 because it was reading `sed`'s status.

**THE ARTEFACT IN FRONT OF YOU IS NOT THE ARTEFACT THAT SHIPS.** Read source with `git show HEAD:`; `--only` takes the WORKTREE.

**zsh: AN UNQUOTED `--include=*.md` ABORTS THE WHOLE COMMAND** and prints a plausible 0.

**MEASURE ONLY: nothing repaired, nothing staged for repair, no estate written to.** Held all session; the fleet re-measure was `git log`/`show`/`cat-file` only, **`git status` never called.**

## Decisions

- **(vc) LAMPLIGHT GOES TO ITS OWN NODES; ic PACKAGES THE METHOD.** vc has since recorded that the "56% of the fleet" half of their reason was my inflation and not verified by them.
- **(ic) EXPOSURE IS NOT DAMAGE, AND A PREDICTOR IS NOT A CONFIRMATION.** Say **predicted-unconfirmed**.
- **(hv, via vc) ALL ST0065 VERDICTS ADOPTED**; retirement complete, doc/skill legs complete.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING** -- the template engine has no include form.
- **(all nodes) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
