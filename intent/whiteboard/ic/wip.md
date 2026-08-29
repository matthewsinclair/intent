---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 17:58Z
status: paused
focus: "EOD 1757Z. **AC-12.2 + AC-12.3 DONE AND SATISFIED IN CANON BY vc, who swept independently rather than accepting my report.** Nothing claimed, nothing blocked, nothing owed. On the bounce: no assignment -- ask vc. **Holding `intent` invocations until vc says the rebuilt pair is current.**"
claims: []
---

# Interface Claude (ic)

## DOING

**Nothing. Both criteria closed.** `AC-12.2` (working-with-llms.md, README, CHANGELOG) and `AC-12.3` (`docs/releases/3.0.1/RELEASE_NOTES.md`), satisfied in canon at `6cdbbc5f` **by vc's independent sweep, not by my report.**

**ON THE BOUNCE: ASK vc FOR THE NEXT UNIT.** Do not infer one. **Assignment 2 (surface-review cc's refuse-and-edit package) was unparked and I never got to it** -- raise it rather than assume it lapsed.

**`intent` INVOCATIONS ARE HELD** until vc confirms the rebuilt pair is current. During their window there is no shared binary at all, so anything driven at it **fails rather than answers** -- which is the good failure, but do not meet it unwarned. **Source and git plumbing work throughout and are how I verified `st edit` during the window.**

## THE FOUR THAT COST ME TODAY

**1. A CLAIM WHOSE _BASIS_ IS WITHDRAWN READS EXACTLY LIKE ONE THAT STILL HOLDS.** I was handed "3.0.1 does not fix `ac new`" and carried it on my own board twice. It was true when given; hv's "fold everything into 3.0.1" removed the constraint under it. **No symptom, and the entry looked settled -- settled is what makes a thing stop being checked.**

**2. A RULING AND A LANDED CHANGE ARE INDISTINGUISHABLE IN A MESSAGE.** Two of four surface changes I was handed did not exist (`issues edit` absent from register, source AND `--help`; `st edit` still the path-printer). **Fix: a surface claim travels with what makes it checkable.** **AND THE FIX HAS A LIMIT I FOUND THE HARD WAY** -- "it is in the register" covers a NEW verb and **cannot cover a BEHAVIOUR CHANGE to an existing one.** `st edit` was an unchanged register row before and after `8aa83dd6` while what it does changed completely. **The register answers _does this exist_, not _does it still do what the last doc said_.** For that class the check is the source or the test.

**3. FIXING THE INSTANCE IS NOT FIXING THE CLASS.** The `intent doctor` claim: **vc swept and found three; I reasoned about the document in front of me and fixed one of two.** My README kept the false sentence while my release notes had the correction. **The sweep is the check, not the care.**

**4. I DID NOT DERIVE A WRONG ANSWER -- I APPLIED A CORRECT GENERAL CONVENTION.** `user/tap/formula` is the shape everyone has seen; this project is `matthewsinclair/intent/intent` and says so in the file that publishes the formula. **Three of us wrote three different wrong spellings of a rule already written down.** A rule living where no author of the affected text would look is not three carelessnesses.

## Watch-outs

**POSITIVE-CONTROL THE INSTRUMENT, AND A CONTROL THAT PASSES UNDER THE BROKEN INSTRUMENT IS DECORATION.** My emoji test returned a clean `0` and its control **failed** -- the zero was blind. Working instrument: 28 at HEAD, 0 after. **Same shape as the register limit above: an instrument answering confidently where it structurally cannot see.**

**A FALSE SENTENCE PROPAGATES BY BEING BELIEVED, AND EVERY HOP LOOKS BETTER SOURCED** (vc's). Their doctor claim reached my release notes before I checked it. **The docs failure mode that code does not have.** And **a wrong claim of your OWN is the most trusted source you have**, because you know where it came from -- vc wrote it once then leaned on it twice as a premise.

**LATEST COMMIT AND INTRODUCING COMMIT ANSWER DIFFERENT QUESTIONS.** `git log -1 -- <path>` says _has this changed since the tag_; `--diff-filter=A ... | tail -1` says _did it ship in the tag_. I used the first for the second and got a different answer for one of four items.

**THE GENERATED-VIEW BANNER IS EVIDENCE IN ONE DIRECTION ONLY.** Presence means generated; **absence does not mean authored** -- one WP `info.md` carries none while 38 siblings do. `intent doctor` is the check. **Hand over the check, not the table.**

**REPORT, DO NOT EDIT, ANOTHER NODE'S FILE -- AND DO NOT WRITE INTO A THREAD THEY CLAIM.** I left AC-12.2/12.3 unsatisfied for vc even though I did the work; `doctor` was already reporting a status-gate disagreement on ST0056, which was a second reason.

**AN INDEX LOCK MEANS A PEER IS MID-COMMIT. NEVER DELETE IT** -- wait in a bounded loop. **`--only` SEPARATES FILES, NOT FACTS** and drops a peer's staged sibling.

**A HOLD WHOSE CONDITION HAS ALREADY BEEN MET IS NOT A HOLD, IT IS A NODE SITTING STILL.** Measure before folding; do not bank "holding" as state.

**A STAMP TYPED BY FEEL IS FABRICATION AND THE MESSAGE CHANNEL HAS NO GUARD** (`0099`). Cite the commit, never invent a better-looking minute. **zsh: an unquoted `--include=*.rs` ABORTS the command. A SCRIPT THAT ABORTS MID-WAY WRITES NOTHING AND THE LINES BELOW IT STILL PRINT SUCCESS.**

## Decisions

- **(hv) v3.0.1 IS THE NEXT RELEASE ACT; DOCS PRECEDE THE CUT AND SHIP WITH IT.** Scope: ST0056 and its feeders. **The cut is downstream of `8aebe2ce` by construction**, which is what makes "3.0.1 fixes `ac new`" true.
- **(hv) `intent/docs/` STAYS INTERNAL CANON**; `docs/` is the v3 set, `docs/v2/` the frozen archive, v2 blog canonicals preserved at their old locations.
- **(hv) The number stays `3.0.1`** over vc's `3.1.0` recommendation, cost stated. **The notes do not editorialise about it.**
- **(ic) THE README VOICE CALL WAS MINE: no emoji.** Not the site rule applied by fiat -- vc was right to refuse that -- the same judgement for the README's own reason.
- **(ic) A DOCUMENT WRITTEN AHEAD OF ITS SUBJECT CARRIES A PROVENANCE PARAGRAPH** naming what it was verified against and stating that ruled-but-unlanded items are deliberately absent. **It then needs no rewrite when they land** -- proved when `st edit` landed and the paragraph stayed true.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
