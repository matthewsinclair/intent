---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 15:47Z
status: active
focus: "**AC-12.2 + AC-12.3 ARE RUNNING.** Four files vc did not touch (working-with-llms.md, README, CHANGELOG) + the 3.0.1 release notes. **ON THE CRITICAL PATH: hv is holding the release so the docs ship WITH it.** The README voice call is MINE and vc deliberately did not make it."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING -- RUNNING NOW

**THE HOLD IS OVER. hv ruled the four ST0068 decisions at `7a2f205f`; vc confirmed my unit unmoved and unoverlapping.**

### `AC-12.2` -- four files, NONE of which vc touched

`intent/docs/working-with-llms.md` (the canon narrative, 55KB) | `README.md` | `CHANGELOG.md`

**hv ruled `intent/docs/` STAYS INTERNAL CANON and the public set links across to it** -- so the narrative is mine **by that ruling, not by default.** README and CHANGELOG are root files, also mine.

**THE MEASUREMENT THAT DEFINES THE JOB** (controls run first -- 34 and 40 hits on a certain word, 0 on nonsense): `working-with-llms.md` has **`v3` 0, `3.0.0` 0, `v2` 15, `database`/`store`/`SSOT` 0.** README the same, its only version reference a v2.10.0 migration-guide link. **CHANGELOG already carries a correct `[3.0.0]` entry** -- the release note knows, the narrative does not.

### `AC-12.3` -- the 3.0.1 RELEASE NOTES

**hv holding the release for the doc task is what made this satisfiable at all** -- it was self-contradictory while the docs shipped afterwards.

### THE README VOICE CALL IS MINE AND vc DELIBERATELY DID NOT MAKE IT

README is **emoji-led with v2-framed examples**. The new site design system **forbids emoji as interface**; house rules forbid vanity metrics. **Those are SITE and PROSE rules, not README rules, and vc refused to extend them by fiat.** But a README in one voice and a site in another is **a convergence failure inside `AC-12.2`'s own word.** My call to make, not to inherit.

### WHAT THE RELEASE NOTES MUST CARRY (vc's, and each is a measured fact)

1. **`ac new` on an existing id DESTROYS in the shipped tool and there is no `ac edit`. 3.0.1 does NOT fix it** -- ruled out of the cut. **So the docs are where that hazard gets stated.**
2. **THE CORRECTION TO `0133`'s FRAMING.** Its body still says "ingest damage" and "parser" when it is a **representable-state regression**, and **`0090` means the body cannot be rewritten.** The release notes are fresh, so the correct framing lands there.
3. **THE PUBLISHED v3.0.0 KEG SHIPS NO RULE LIBRARY AND NO SKILLS.** `0112b8c1` is in HEAD and **NOT an ancestor of `80d8b2ca`**. So **`brew install intent` then `intent claude rules list` FAILS today**, and any doc teaching that sequence against the published build is wrong.
4. **`st repair` SHIPPED IN v3.0.0 AND IS RETIRED AT HEAD** -- the tag declares 117 commands including it, HEAD declares 118 with it retired. **The only removal in that direction.** It works in the release people are on and vanishes at the next one.

### ASSIGNMENT 2 -- UNPARKED

**cc's `ac new` / `at new` refuse-and-edit package is DELIVERED.** Surface-review it. **The ping was vc's message, not a future one.**

## HOW I FOUND THE HOLD WAS STALE -- THE KEEPER

**I measured before folding instead of banking "holding" as my state.** hv's ruling had been made and acted on **for over an hour**; cc's package had landed; **my vc inbox's newest entry was still the previous day.** vc's own words: _a node doing the right thing on a channel nobody was reading_ -- the same failure that produced **two design systems**, written independently by cc and vc on separate hv briefs, in the tool whose founding rule is that there is only one.

**A HOLD WHOSE CONDITION HAS ALREADY BEEN MET IS NOT A HOLD, IT IS A NODE SITTING STILL.** And **a stale claim of OWNERSHIP is the same defect as a stale measurement** -- I offered to delete AC-12.2 from my board rather than carry it unexamined, which is what made the question answerable.

**ST0068 was UNCLAIMED with an EMPTY CONTRACT while two real docs commits landed against it.** vc is claiming it and drafting the contract. **A thread that gates a release cannot itself be ungated.**

## OTHER STATE

- **ST0065 has ZERO ACs too.** Batched to hv with the costed proposal (**Option 2, TESTED DUPLICATION**: the 575-byte index in both `_CLAUDE.md` and `_AGENTS.md` via `claude upgrade --apply`, plus a byte-identity arm; **a test, not a generator -- the engine has no include form**; limit: **`usage-rules.md` cannot join**, seeded-once/user-owned).
- **TWO THREADS PASS THEIR OWN GATE AND SIT WIP** -- ST0057 66/66, ST0061 7/7. vc sequences.
- **`0141` FILED** -- self-loop on a payload-carrying enum. **Guarded on the four ratified machines** (`machine_table_check.sh` catches it BY ACCIDENT), **unguarded on `AcceptanceTest.status` and `WorkPackage.scope`.** Watch `AcceptanceTest.status`: safe today **only because the enum carries no payload**, and an `n-a` reason or a red's failure text is exactly what someone will add.
- **PROBE FIXED (`d3dbeafa`, `48c3af2a`)** -- 9/30 before, 30/30 after. Lamplight ran it **12/12, reported BY BLOB SHA**, reproduced their hand partition exactly.

## THE FLEET

**80 exposed. TWO estates now have VERIFIED loss:** Conflab **14 confirmed**; Lamplight **1 of 25** (`ST0358`, evidence ABSENT), **24 unswept and deliberately not assumed.** Baize **28**, Laksa **10**, Prolix **3** remain **PREDICTED-UNCONFIRMED, in those words.** **The confirmed pair is hv's argument for the release, not mine.**

## Watch-outs

**REPORT THE REASONING AT THE RESOLUTION YOU ACTUALLY MEASURED IT** (dc's). **It is not that we wrote things down -- it is that we wrote down the parts that could be WRONG.** vc's inversion: **five corrections in an afternoon is not evidence the estate is well-checked, it is evidence that EXPOSED REASONING GETS CHECKED** -- the corrections are **a biased sample of the errors**, so **the safest-looking entries on any board are the ones nothing has ever tested.**

**LUCK WEARING THE SHAPE OF JUDGEMENT.** I withdrew the date classifier because the BANNER was insufficient; Lamplight later showed the date form fails too, for a reason I never found. **Right outcome from an argument that did not reach the real defect** -- the counter-shape to a true result from a blind instrument, and nothing polices it.

**DEAD CODE THAT STILL MAKES A CLAIM IS A FALSE CLAIM.** `port` was computed, threaded through `classify()`, and printing a warning implying it classified -- all three false, nothing changed underneath them.

**BORN STALE IS NOT "A CLAIM OUTLIVING ITS BASIS."** `90988faf` carried a false header AND its refutation twenty lines below. What caught it: **cc wrote a precondition on the sentence they were relying on.**

**COMMIT DATE IS NOT CONTENT DATE WHEN RENAMES ARE IN PLAY** (`18000b4cf`: 3723 files, all R100). **A BINARY GUARD ON A CONTINUOUS QUANTITY IS THE WRONG SHAPE.** **BISECT ASSUMES DETERMINISM** -- repetition is the instrument. **`--only` SEPARATES FILES, NOT FACTS.** **AN INVARIANT NOBODY HAS WRITTEN THE DECLARATION TO BREAK IS UNEXERCISED, NOT ENFORCED.** **A ZERO YOU CANNOT TRUST MUST NOT EXIT 0.**

**A STAMP TYPED BY FEEL IS FABRICATION AND THE MESSAGE CHANNEL HAS NO GUARD** (`0099`). **Cite the commit, never invent a better-looking minute.**

**`git show HEAD:` for source; `--only` takes the WORKTREE. CAPTURE AN EXIT CODE TO A FILE. zsh: an unquoted `--include=*.rs` ABORTS the command. `cd` PERSISTS between Bash calls.** **A SCRIPT THAT ABORTS MID-WAY WRITES NOTHING AND THE LINES BELOW IT STILL PRINT SUCCESS.**

## Decisions

- **(hv) FOUR ST0068 DECISIONS RULED:** `docs/` is the v3 set, `docs/v2/` the frozen archive, **`intent/docs/` stays INTERNAL CANON**, v2 blog canonicals **preserved pointing at their old locations** (the one-way half; `docs/v2/README.md` records why, so nobody "fixes" them).
- **(hv) v3.0.1 IS THE NEXT RELEASE ACT; DOCS PRECEDE THE CUT AND SHIP WITH IT; warn the fleet by PROPERTY (does the build carry `04cf6f18`) not by version.**
- **(vc) `ac new`'s DESTRUCTION IS RULED OUT OF THE 3.0.1 CUT** -- the docs are where the hazard gets stated instead.
- **(ic) EXPOSURE IS NOT DAMAGE, AND A PREDICTOR IS NOT A CONFIRMATION.** Say **predicted-unconfirmed**.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING** -- the engine has no include form.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
