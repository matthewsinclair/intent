---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 13:29Z
status: active
focus: "ACTIVE. Routed the ST0065 proposal vc had never been told existed. **FIAT CAVEAT RETRACTED -- ZERO fiat rows in the STORE, so every stale-binary gate reading today is sound, on every thread.** TWO THREADS PASS THEIR OWN GATE AND SIT WIP (0057, 0061); **ST0065 HAS NO CONTRACT AT ALL.** Fixed a header of mine that shipped BORN STALE and cost cc real work. Awaiting vc on all three."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING

**ROUTED THE ST0065 PROPOSAL TO vc (13:28Z, inbox + live ping). Awaiting their answer. Nothing else in flight.**

**A DELIVERABLE SITTING IN ITS OWN THREAD'S DIRECTORY IS INDISTINGUISHABLE FROM A DELIVERED ONE.** `_proposal-agents-md.md` has existed since yesterday 17:48; its own section 7 says _"This goes to hv via vc"_; the session then ended. **Measured on the bounce, not remembered: `grep -c ST0065 vc/wip.md` = 0, `grep -rl "COSTED PROPOSAL" vc/` = nothing, `.history/` included.** The catalogue verdicts DID reach hv (`36b5abd2`); it is the design leg written after that routing that went nowhere. **This is the protocol doc's hv-inbox class in its worse form -- not a write with no named reader, but NO WRITE AT ALL by the node holding the deliverable.** My board said "BLOCKED on hv" for a day, which was true of the state and false about the cause.

**RE-DRIVEN BEFORE RE-SENDING, because the subject MOVED under the document:** cc landed the ST0067 guide pointer into `_AGENTS.md` at `8a997c1e` after I wrote the proposal. Index count unchanged at HEAD -- AGENTS.md 1, usage-rules.md 1, in-standards 2, **CLAUDE.md 0** -- nonexistent-rule-id control returns 0 in all six files. **A day-old measurement whose subject was edited in between is a claim outliving its basis (vc rule 3) until you re-drive it.**

**THE ORDERING HAZARD, NOT YET A DEFECT: cc is editing `_AGENTS.md` under ST0067 while the proposal about what `_AGENTS.md` IS sits unrouted under ST0065.** hv's ST0067 ruling explicitly says that line is "coordinated with ic's WP-01".

**ST0061 (dehydrate) IS COMPLETE BY ITS OWN RATIFIED BOUNDARY AND STILL SITTING WIP.** `intent ac gate ST0061` -> PASS 7/7, seven ATs green. **Not closing it: my claim, vc's sequencing.** Offered to vc 13:33Z.

**AND THE INSTRUMENT THAT SAID SO IS BEHIND HEAD.** The pre-commit gate's own currency arm refuses the delivered pair -- `target/release/{intent,intentd}` name `8177b53ef64a`, HEAD is `047cfdf4`, 9 non-test files between them **including `model.rs` and `transitions.rs`, where `AcState::Fiat` lands with its own `in_scope` arm.** Every node driving `intent` in this tree today is reading a PRE-`Fiat` instrument; announced to dc and cc 13:32Z because `0133` is a change to `AcState` itself.

**THE VERDICT SURVIVES ON A PROPERTY OF ST0061'S DATA, NOT ON AN IMPOSSIBILITY ARGUMENT.** Its canon carries seven `{"is": "computed"}` rows and zero fiat/descoped/withdrawn, so the new arm is unreachable **for this thread**. "A change like that could not flip a pass" is yesterday's category error in a clean shirt, and it is not what I checked.

**I DID NOT REBUILD AND DID NOT ASK ANYONE TO.** The shared-artefact guard correctly refuses a release build while `native/rust` is dirty, and it is dirty with cc's and dc's live work. **Reporting a refusal is not routing around it.**

## **v3.0.0 IS PUBLISHED AND SHIPS THE `0133` DEFECT**

**Every step measured:** `v3.0.0` -> `80d8b2ca`, on **both** remotes, same sha. **`04cf6f18` is NOT an ancestor of it.** `model.rs` in the TAGGED tree line 1070 is `Unsatisfied,` -- a bare unit variant. `gh release view`: **not a draft, not a prerelease**, published **2026-08-26T13:49:37Z**, three assets, **`downloadCount` 3 each.**

**The tag cannot be moved** -- published, two remotes, downloaded. **The fix is v3.0.1**; sequencing is hv's. Escalated to vc live and to `hv/inbox.ic.md` durably, because releases are hv's and this is what that inbox is for.

**THE THING I CANNOT RESOLVE FROM HERE: `AC-12.4` is recorded UNSATISFIED and its first two clauses are measurably DONE** (tagged both remotes, release published). **Either the criterion is stale, or the release went out ahead of the criterion meant to gate it** -- and the second reading means the gate did not gate. I cannot separate them.

**AND IT RE-AIMED MY OWN SHIPPED METHOD DOC (`01afa12f`).** It told four estates not to re-run the ingest expecting recovery. **dc's fix did not make one word of that false** -- it turned it from a fact about Intent into a fact about **which build you are standing on**, and for everyone actually running v3.0.0 the original advice still holds. **A paragraph can go stale without any word in it becoming false.** The doc now hands over the PROPERTY to test (does your build carry `04cf6f18`) instead of the verdict to trust.

## THE CANON NARRATIVE AND THE README HAVE HAD **ZERO** v3 CONVERGENCE

**Not partial. Zero -- and `AC-12.2` names them first.** Instrument positive-controlled before I believed it, because a zero from a grep is exactly what I got wrong yesterday.

- **`intent/docs/working-with-llms.md`** (55272 bytes, last touched `861fa66c` 2026-08-25): control `Intent` -> 34, control `zzqqxx` -> 0. **`v3` -> 0. `3.0.0` -> 0. `v2` -> 15. `database`/`the store`/`SSOT` -> 0.**
- **`README.md`** (17207 bytes, same commit): control 40 / 0. **`v3` 0, `database` 0, `SQLite` 0, `brew` 0** -- its ONLY version reference is a link to the v2.9.0 -> v2.10.0 migration guide.
- **CHANGELOG is the outlier the GOOD way:** a written `[3.0.0]` entry dated 2026-08-26 whose opening line states DB-as-SSOT correctly. **The release note knows; the narrative does not.**

**THE TWELVE THAT ACTUALLY REMAIN ON ST0056** (the gate's 66 mixes stored-unsatisfied with test-backed-not-yet-green): `AC-00.5` brew on a clean machine, `AC-00.6` prune `bin/` at the cut, `AC-06.3` deviation register complete, `AC-10.6` rollback exercised on the canary, `AC-11.1` tap formula, `AC-11.4` checksum matches downloaded bytes, `AC-12.2`, `AC-12.3`, `AC-12.4` tag/release/formula, `AC-13.9` T3+T4 staged, `AC-14.10`, `AC-14.12`.

**THREE ARE INTERFACE-SHAPED AND OFFERED TO vc: `AC-12.2`, `AC-12.3`, `AC-14.10`.** All three are backed by `n-a` ATs -- non-test criteria closed by NAMED EVIDENCE at review, so **the evidence quality IS the deliverable**.

**NOT STARTED, AND ONE OF THEM SHOULD PROBABLY WAIT: `AC-14.10` is entangled with `AC-14.12`**, which rules the file-based `claude ws` family must GO. **Updating `/in-whiteboard` to document a file protocol scheduled for deletion is work against the grain** -- if `ws` goes DB-based the skill should follow the new mechanism rather than be written twice. That ordering is vc's or hv's, not mine to guess.

## THE WIP GATE SWEEP, AND THE HOLE IN MY OWN THREAD

**ST0057 PASS 66/66 (3 withdrawn) | ST0061 PASS 7/7 | ST0056 BLOCKED 67/133 | ST0058 BLOCKED 2/6 | ST0066 BLOCKED 0/6 | ST0065 EMPTY CONTRACT.** Two threads pass their own gate and are still WIP.

**ST0065 HAS ZERO ACCEPTANCE CRITERIA AND IT IS MY CLAIM.** The thread hv ruled on, that I catalogued across two WPs and wrote a costed proposal against, **has never had a ratified boundary.** Not minting ACs unilaterally -- ratification is the open-gate and the pen is not mine to assume. **I lean `acceptance: exempt`** (a review-and-catalogue thread's deliverable IS the catalogue plus hv's verdicts) and said so to vc; the sequencing is theirs and the ratification hv's.

**THE PRE-`Fiat` CAVEAT IS RETRACTED AS TOO NARROW.** I told vc the ST0061 reading survived on a property of that thread's data. **Measured in the store since: ZERO fiat rows exist anywhere** -- 306 computed, 78 satisfied, 27 unsatisfied, 5 withdrawn. So every gate reading through the stale binary is sound on every thread. **The pattern was LOOSE and I demonstrated it over-matches** -- the same `LIKE` caught a `satisfied` row merely mentioning "withdrawn" in prose -- **which is what makes the zero sound, and it cost nothing next to the rebuild it replaced.**

## BORN STALE IS NOT RULE 3, AND IT NEEDS A DIFFERENT DEFENCE

**`89cdaffc`, header only.** `mutation_creates_criteria_and_tests.rs` claimed its fixture _"ALREADY carries L2 findings before this file writes anything"_. **False by construction:** `Fixture::write_thread` creates every cited file and writes the row's own id into it (`common/mod.rs:160-164`), so L2 and L3 are both unreachable there.

**`git log -S` puts the false claim and the body comment refuting it IN THE SAME COMMIT (`90988faf`), and the mechanism underneath predates both by months (`a1a949cf`).** Nothing decayed; **there was never a moment the header was true**, and the file shipped with its own refutation twenty lines below the claim. **Rule 3 is written around a claim OUTLIVING its basis, which presumes a basis existed -- so "re-check when the subject moves" cannot reach this: the subject never moved.** What caught it is what cc did: **write a precondition on the sentence you are relying on.** I did the archaeology to separate born-stale from ordinary carelessness before reporting it as a class.

**IT COST A PEER REAL WORK, WHICH IS THE ONLY REASON IT SURFACED.** cc believed my sentence, wrote the precondition, and it fired -- without it their own arm would have passed proving nothing. **The arm of mine they flagged is NOT vacuous and I left it alone**: it plants `AT-03.4` covering a nonexistent criterion, controls the finding is non-empty BEFORE the create, and asserts it survives AFTER, so a silent repair cannot pass it for the wrong reason.

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
