---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 11:50Z
status: active
focus: "BOOTED POST-COMPACT AND WAITING ON vc, whose OWN board queues my survey THIRD (dc FIAT-EXIT ruling to hv first, then AC-00.4). NOTHING IN FLIGHT, nothing of mine dirty. TWO ACTIONS STILL HELD FOR THE USER OWN VOICE -- the skills uninstall and the skills sync -- so ST0065 committed canon edits remain DELIVERED TO NOBODY. NEXT WHEN vc CALLS IT: the fleet ingest-prose-damage survey, MEASURE-ONLY."
claims: [ST0065, ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT.** ST0065's three legs are delivered and its step 1 is committed. Next work is vc's survey, below.

## HELD -- needs THE USER, not a peer and not hv-via-vc

**Both are authorized on hv's board and I have run NEITHER, deliberately.**

- **`intent claude skills uninstall in-start in-next`** (retirement step 2) -- authorized at `b17048d3`, menu recorded with declined options.
- **`intent claude skills sync`** -- authorized in the same class, and without it today's canon edits reach nobody including our own sessions.

**WHY THEY ARE STILL HELD.** Both write OUTSIDE the repo into `~/.claude/skills/` and `~/.intent/`. I put the uninstall to the user directly; **the harness reported that no genuine user input had been received and that the answer must not be treated as consent.** hv's board is maintained BY vc, and the protocol's own words are that an hv attribution written by vc is _unverifiable by construction_ -- the same sentence the parity gate prints about `authority: hv` with no record. **So the chain is well-formed and still not the user's own voice.** A peer cannot supply that and neither can a board.

**Consequence to state plainly rather than let someone infer:** ST0065's canon edits are committed and **delivered to nobody**. Any session reading `in-standards` today still gets the retired MODULES.md imperative.

## NEXT -- vc's assignment: FLEET INGEST PROSE DAMAGE SURVEY (not started)

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
