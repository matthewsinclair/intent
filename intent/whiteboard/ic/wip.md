---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 12:17Z
status: active
focus: "ST0065 COMPLETE, verified. SURVEY Intent leg DELIVERED AND THEN LARGELY DEVALUED BY ITS OWN BEST FINDING: INTENT IS THE CORPUS THE PARSER WAS FITTED TO (legacy.rs born 2026-08-16 debugged ON this estate; hop 08-19; every acceptance-path fix 08-26/27 for OTHER estates), so a clean Intent proves nothing about the fleet and Intent MUST NOT be the baseline. Highest-value next: which estates hopped in the 08-19 to 08-26 window -- a DATE comparison, not a sweep."
claims: [ST0065, ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** ST0065's three legs are delivered and its step 1 is committed. Next work is vc's survey, below.

## RESOLVED -- the two held writes, and how

**hv RAN BOTH THEMSELVES** (menu: hv runs them here, sync first -- CHOSEN). The authorisation chain was DISSOLVED rather than lengthened, which was the point of the hold. Nothing was executed by me and nothing needed to be.

**vc's sequencing catch, which my draft got wrong:** `sync` INSTALLS FROM CANON, so a sync AFTER the uninstall reinstalls both. hv ran **sync first, then uninstall**. My draft called steps 2 and 3 order-free because uninstall does not read canon -- true, and not the whole picture once a sync joins the sequence. **The draft is now wrong on disk and should say so.**

**vc corrected the cost figure DOWNWARD and told me unprompted:** the delivery gap was ONE skill, not four -- three were already settled and my `in-standards` repair had already landed at `818d27a8`. So "every session is loading the pre-edit skill" was false when vc relayed it to hv. **A relay carries the measurement's TIMING as well as its scope.** My hold was still right, for the reason it was made and not for the cost.

## DELIVERED -- fleet survey, INTENT LEG (report: `survey-ingest-damage.md`, tools in `tools/`)

**THE REACH FINDING: the v2 comparison source is in GIT HISTORY on any git-tracked estate.** `legacy.rs:1273` reads `dir.join("acceptance.md")` -- the ingest's OWN input path, which the v3 generated view then overwrote. Conflab's `acceptance.v2.md` was a hand-preserved COPY, not the parser's input. **Measured on ONE estate; it fails wherever pre-hop history was squashed and that is checked NOWHERE.**

**INTENT: 615 authored rows, 615 matched, ZERO confirmed class (i).** Measured against canon **AS INGESTED** at the hop (`16048f82`). **The same check against TODAY's canon returns 112, 98 of them in ST0057, which returns 0 at the hop** -- eleven days of authoring reading as damage. **An instrument pointed at the current store measures the AUTHORS, not the ingest.**

**UNRESOLVED, attributed to nothing:** `ST0056/AC-00.10`, authored 7182 chars, canon-at-hop 15215. NOT the splice -- longest repeated span at a 100-char floor is none. ST0056 was under edit across the hop boundary, so it is **CONFOUNDED** and reported as such.

**`0127` IS CLOSED** -- conflab-vc reconciled 114 authored notes against 114 canon notes. The class does not exist. **`0126` is a two-capture ROTATION, not 3x duplication**, so my same-field duplication test is aimed at the OVERLAP SUBSET rather than the class.

**16 estates UNMEASURED -- not clean.** Next in value order: confirm the recovery method on a SECOND estate (the whole reach claim rests on one layout); give the splice detector its structural discriminators before any fleet run.

## STILL OPEN -- residue from the retirement, needs an out-of-repo write

`~/.claude/skills/in-start/` and `in-next/` are **empty directories** left by the uninstall. The tool's own listing is correct (both absent, no orphans), but a presence check on the directory reads them as installed -- **my own count was fooled by exactly that before I looked inside.** Removing them writes outside the repo, so it is flagged, not done.

## SUPERSEDED -- vc's assignment as originally briefed

**MEASURE ONLY. REPAIR NOTHING.** No markdown-to-store route (`0097`), no issue edit verb (`0090`) -- a repair today is a hand edit the next sync reverts, and it **consumes the evidence**. Read-only across estates; announce before touching a shared tree; do not disturb a live checkout.

**Seventeen estates went through the v2 ingest; sixteen have never been checked.** Conflab caught it only because it preserved `acceptance.v2.md`. **This repo retains no v2 buckets** -- vc looked.

Four filed classes, kept SEPARATE on purpose (**a survey reporting one "prose damage" number sends someone to fix one and believe they fixed both**):

| issue  | class                                  | shape                                                                                                                                  | remedy needs                                                           |
| ------ | -------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `0124` | prose BETWEEN two recognised fields    | discarded; survivor reads grammatical and complete. **Genuinely lossy -- it has no home**                                              | a RESTORE, so only where a v2 bucket survives                          |
| `0126` | SPLICE                                 | **NOT lossy.** One authored span read from THREE offsets and concatenated, so the row is **scrambled and INFLATED** (+297, +193 chars) | a dedup RE-PARSE -- works on EVERY estate, no comparison source needed |
| `0127` | the note FIELD dropped entirely        | leaves a `legacy.raw` stub a delta reads as a surviving short statement                                                                | --                                                                     |
| `0129` | authored full stop rewritten into `--` | latent; the parser's own delimiter                                                                                                     | --                                                                     |

**THE PREDICTOR (conflab-vc's), and it needs no comparison source:** count rows whose AUTHORED note carries a `--` after the status value. Control shape to copy: **2-for-2 damaged carry it, 3-for-3 clean do not** -- both arms populated. **A predictor with no clean arm proves nothing.**

**THE PREDICTOR PREDICTS; IT DOES NOT CONFIRM.** Where a comparison source exists, confirm. Where it does not, report **predicted-unconfirmed, in those words**. This is the 157-versus-4 error -- the 157 counted the generator legitimately appending its own `-- covers` / `-- satisfied:`, ie it **measured the tool working correctly**.

**THE SINGLE MOST VALUABLE THING THE SURVEY CAN SETTLE:** whether class (i) is lossy across its POPULATION. It is measured on two specimens and unmeasured on the other 18 rows, and the answer decides **whether the fleet needs a restore path at all.**

**Instrument warning that governs the design: a LENGTH delta is blind to (ii) and (iv) by construction, and (iii) is an ABSENT field rather than a short one.** conflab-vc's length scan had a properly discriminating control and is still blind. **Where a class cannot be measured, say UNMEASURED -- never zero.**

**If the survey is larger than S, STOP and give vc the shape rather than pushing through.**

## Watch-outs

**MY REFINEMENT WAS ANTI-CORRELATED WITH THE DAMAGE, AND ITS ZERO WAS BY CONSTRUCTION.** conflab-vc measured the mechanism: the ingest DISCARDS the evidence clause on every criterion authored UNSATISFIED and KEEPS it on every one authored SATISFIED. I narrowed a detector to `satisfied AND no evidence` -- ie to the immune population -- reasoning that unsatisfied rows carry no evidence _because they are not satisfied yet_. **THAT READING IS WHAT THE DAMAGE MANUFACTURES.** vc ran the same detector and got the same zero, so two nodes agreed with each other while both measured the protected set. **A damage class that supplies its own innocent explanation defeats agreement between instruments; only the authored source settles it.**

**"UNMEASURABLE" AND "NOT EXPOSED" ARE DIFFERENT FACTS AND I CONFLATED THEM TWICE TODAY.** 16 rows first read as unmeasurable were post-hop threads that never met the parser -- checked, not assumed. Same shape as reading an absent file as an absent banner.

**THE CLASS, SEVEN INSTANCES: I LET A MEASUREMENT STAND AS A FINDING WITHOUT READING WHAT IT COUNTED.** Both directions -- undercounts from a pattern fitted to a form I had seen, then an overcount from a broad pattern **whose hits I never opened**. So "widen the pattern" is the WRONG lesson; it produced instance four. Instances 6 and 7 (the four principles are readable nowhere; the generator renders all three root files) were both taken from a docstring or a constant instead of the thing itself, and **both were caught in-session before hv. That is the discipline working, not the class being gone.**

**A CONTROL IS ONLY A CONTROL ONCE IT HAS FIRED** -- and one that MOVES is still a control (`in-session` 94 to 95, because my own commit added one).

**MEASURE WHAT THE DOCUMENT SHIPS TO, NOT WHAT YOU ARE STANDING IN.** The hook-path overstatement reached hv this way; dc's Family 8 is the same shape -- **the estate's own configuration hides its bugs from it.**

**FOUR DELIVERY PATHS; "LANDED" DIFFERS ON EACH.** Embedded templates need the rebuild; the prime template is live from disk; skills need a sync; **`usage-rules.md` is seeded once and NEVER synced (`canon.rs:316`) so that fix reaches no existing project, ever. A commit is not a delivery.**

**THE GATE READS THE WHOLE STAGED INDEX** -- a peer's in-flight work blocks everyone. Unstage only your own path. Cleared on retry both times.

## Decisions

- **(hv, via vc) ALL ST0065 VERDICTS ADOPTED.** `in-next` RETIRES. `DECISION_TREE` gating is **measured NOT YET LANDED** -- bare `intent init` still writes it. cc's.
- **(ic) A RETIREMENT'S DELETE IS ITS LAST STEP.** `sync` never prunes a vanished canon source (`skills.rs:690-697`) and `upgrade`'s skill step IS that sync, so a canon-first delete strands the skill everywhere **while it keeps loading into sessions**.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING.** The template engine has no include form -- three tokens, unknown ones refused -- so AGENTS.md gets duplicate-plus-drift-test, which is `agents_sync_parity`'s own argument.
- **(all nodes) Fold archives are `wip-fold-HHMMZ.md`**, append-only, nothing renamed. **(vc) `add + commit --only + reset` is NEW files only.**
