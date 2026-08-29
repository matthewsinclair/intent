---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 14:26Z
status: paused
focus: "FOLDED HARD 14:23Z (23274 -> aggressive). **HOLDING FOR hv's ST0068 INSTRUCTIONS VIA vc -- DO NOT BEGIN THE NARRATIVE.** My AC-12.2+12.3 unit is ON THE CRITICAL PATH: hv held the release so the docs ship WITH v3.0.1. Landed today: probe fixed (d3dbeafa), 0141 filed. ON THE BOUNCE: hold, take the shape from vc."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING

**NOTHING. HOLDING FOR hv's INSTRUCTIONS, ROUTED BY vc. Nothing of mine dirty.**

**DO NOT BEGIN THE NARRATIVE BEFORE THE SHAPE ARRIVES.** vc's reason, better than my impatience: **the shape hv wants may not be the shape vc and I scoped.**

**WHAT hv IS ACTUALLY DECIDING (four, and TWO ARE ONE-WAY -- which is why nothing has moved):** whether the releases move to `docs/v2/` alongside the blog; what happens to the moved posts' **canonical URLs**; whether the doc set describes **3.0.1-as-shipped or 3.0.1-plus-coming**; and where utilz-vc's **upgrade-cost synthesis** lands. **The third one decides my unit's voice and I must not guess it.**

## MY UNIT

**`AC-12.2` + `AC-12.3` (ST0056), homed in the new `ST0068` "Update Intent's docs for v3".** Both are `n-a` ATs -- **non-test criteria closed by NAMED EVIDENCE at review, so the evidence quality IS the deliverable.**

**hv MOVED IT ONTO THE CRITICAL PATH: the documentation comes BEFORE the release and ships WITH v3.0.1.** The release is HELD. **That also DISSOLVED the contradiction I had flagged** -- `AC-12.3` says "release docs written BEFORE the cut", which holding the docs would have made unsatisfiable by construction.

**`AC-14.10` IS PARKED on `AC-14.12`**, which rules the file-based `claude ws` family for deletion. **Documenting a protocol scheduled for deletion is work done twice.**

**THE MEASUREMENT BEHIND THE UNIT: `working-with-llms.md` (55KB) has `v3` 0, `3.0.0` 0, `v2` 15, `database`/`store`/`SSOT` 0. README the same.** Controls first (34 and 40 on a certain word, 0 on nonsense). **CHANGELOG already carries a correct `[3.0.0]` entry -- the release note knows and the narrative does not.**

## v3.0.0 IS PUBLISHED AND SHIPS THE `0133` DEFECT -- WITH hv, RULED

`v3.0.0` -> `80d8b2ca`, both remotes; **`04cf6f18` NOT an ancestor**; tagged tree line 1070 is `Unsatisfied,`, a unit variant; release **not a draft, not a prerelease**, published 2026-08-26, assets downloaded. **vc re-drove all four independently, and measured the clause I could not: the tap formula is LIVE and pins `3.0.0`, so `brew install intent` HANDS YOU THE DESTROYING BUILD.**

**`AC-12.4` is the WORSE reading: all three clauses measurably DONE and the criterion still recorded UNSATISFIED. The release went out complete, past the criterion that records it, and nothing noticed.**

**hv RULED: v3.0.1 is the next release act; warn the fleet NOW naming the PROPERTY to test (does the build carry `04cf6f18`) rather than a version to trust** -- my `01afa12f` method-doc correction is the shape the warning goes out in.

## THE PROBE IS FIXED (`d3dbeafa`) -- 9/30 BEFORE, 30/30 AFTER

vc authorised it to jump the hold: **the hold is on the RELEASE and the NARRATIVE, not a freeze on the estate**, and **the harm was LIVE rather than pending.** vc **declined my cheap alternative on principle** -- warning three estates is **a social mitigation where a mechanical one was available.**

1. **THE CONTROL WAS A COIN FLIP AND HAD BEEN SINCE I WROTE IT.** `%ct` is whole seconds; the old sort fell through to comparing **SHA strings**. Fixed with `git log --all --topo-order` (removes the tie rather than breaking it better) and **explicit fixture dates** so it models the timeline it claims to. **I found it the WEAK way** -- one failure while editing -- and would have shipped a fix for a bug I thought I had just introduced. **vc ran it N times and found it had been flaky all along**, including every run I cited as evidence the probe discriminates.
2. **LAMPLIGHT-1: the exit-3 guard was BINARY**, firing only at zero, so 152/358 exited 0 with 206 threads unaccounted. Now **per-thread, partition ASSERTED, exit code GRADED** (3 not-measured, 4 incomplete).
3. **LAMPLIGHT-2: `*/acceptance.md` is a filename glob, not a thread predicate.** Cross-checked against the estate's own canon; **absent oracle SAYS SO.** Mutating it off leaks a non-thread row straight into `EXPOSED`.

**AND A CLASSIFIER I BUILT, DROVE AND WITHDREW.** The banner split put 44 of 67 Intent threads in "residue" -- a large, publishable number. **I opened ONE instead: `ST0001`'s acceptance file is created BY the v3 hoist and carries no banner because the early renderer emitted none.** Timestamps failed too (**the hoist wrote acceptance files ~25h BEFORE the first `.canon` record**). Underneath both: **v3's GENERATED `acceptance.md` uses the SAME `-- satisfied:` syntax as v2 authored**, so the banner is **SUFFICIENT, NOT NECESSARY.** Withdrawn to one `UNCLASSIFIED` bucket naming both causes.

**THE CRACK THAT FOLLOWS, FLAGGED NOT BURIED: if v3-generated files satisfy the v2-authored test, `recovered` may be CONTAMINATED on any estate and the exposure figure is computed over it.** Intent reads 0 exposed -- **a fact about Intent, not evidence about Baize, Laksa or Prolix.** **No estate is called measured on the strength of a green.**

## `0141` FILED -- AND WHAT ALMOST WENT IN INSTEAD

**`set_ac_state`'s short-circuit is payload-inclusive**, so a **self-loop on a payload-carrying enum** turns a safe no-op into a **silent overwrite reported as a movement**.

**Guarded on the FOUR ratified machines** -- vc planted the hazard and `machine_table_check.sh` reds at exit 1, expanding an empty from-list to `(any)` which matches no ratified row **by construction**, catching self-loops by the same property. **A guard built for transcription drift closes it BY ACCIDENT.**

**UNGUARDED on `AcceptanceTest.status` and `WorkPackage.scope`** -- the two `Disposition::State` fields `data-model.md` deliberately does not table.

**WATCH `AcceptanceTest.status`: `to-write | red | green | n-a`, whose semantics `data-model.md` calls "the most operational subtlety and the least written down".** Safe today for **one reason only: the enum carries no payload.** **An `n-a` reason or a red's failure text is exactly what someone will add** -- and the wildcard becomes the erasing write the day it lands. **Here on the board because an issue is not a thing anyone reads before making the change that triggers it.**

**HAD I FILED TWENTY MINUTES EARLIER I WOULD HAVE CLAIMED A GENERAL GAP THAT IS THREE-QUARTERS CLOSED. Right reasoning, real hazard, WRONG BLAST RADIUS, and confident.**

## PARKED / WAITING

- **ST0065 has ZERO ACCEPTANCE CRITERIA** -- catalogued in two WPs and argued in a costed proposal with **no ratified boundary behind any of it.** I lean **`acceptance: exempt`**. Batched to hv with the proposal.
- **THE COSTED PROPOSAL recommends OPTION 2, TESTED DUPLICATION**: the 575-byte index in both `_CLAUDE.md` and `_AGENTS.md` via `claude upgrade --apply`, plus one byte-identity arm. **A test, not a generator -- the template engine has NO include form.** Limit stated: **`usage-rules.md` cannot join** (seeded-once, user-owned), so three homes -> two tested + one declared exception, **not zero**.
- **TWO THREADS PASS THEIR OWN GATE AND SIT WIP** -- ST0057 66/66, ST0061 7/7. vc sequences after hv rules the release.
- **ASSIGNMENT 2** (surface-review cc's `ac new`/`at new`) -- **vc pings me; I do not watch.**

## THE FLEET

**80 exposed. 11 estates measured, 5 not.** Baize **28**, Lamplight **25**, Conflab **14** (only CONFIRMED), Laksa **10**, Prolix **3**. **Everything except Conflab is PREDICTED-UNCONFIRMED, in those words.** **Lamplight returned 25 blind against a predicted 25 -- their NUMBER stands, its WARRANT is withdrawn**, because it rested on a single 8/8 from a control now known to pass ~30% of the time. Re-run over N requested.

## Watch-outs

**REPORT THE REASONING AT THE RESOLUTION YOU ACTUALLY MEASURED IT** (dc's, the day's best). **It is not that we wrote things down -- it is that we wrote down the parts that could be WRONG.** A board of verified conclusions gives nobody a fixture to plant against. **Every correction today landed on a claim whose reasoning was visible; the claims with no reasoning attached are still standing and I do not know whether they are right.**

**vc's INVERSION, WHICH IS SHARPER THAN MY VERSION: five corrections in an afternoon is NOT evidence the estate is well-checked -- it is evidence that EXPOSED REASONING GETS CHECKED.** The corrections are **a biased sample of the errors**, biased toward the ones somebody made checkable. **So a bare verdict is not merely unfalsifiable by a reader -- it is INVISIBLE TO THE PROCESS THAT PRODUCED EVERY ONE OF TODAY'S FIXES, which means the safest-looking entries on any board are the ones nothing has ever tested.**

**A FILING IS A RELAY OF YOUR OWN REASONING, SO IT CARRIES A BLAST-RADIUS CLAIM WHETHER OR NOT YOU MAKE ONE** (vc's limb on relaying-is-authoring). **Confidence tracks the reasoning; the SCOPE is a separate measurement nobody takes.**

**BORN STALE IS NOT "A CLAIM OUTLIVING ITS BASIS" AND THAT RULE CANNOT REACH IT.** `90988faf` carried a false header AND its own refutation twenty lines below, over a mechanism predating both. **There was never a moment it was true**, so "re-check when the subject moves" cannot catch it. What did: **cc wrote a precondition on the sentence they were relying on.**

**A BINARY GUARD ON A CONTINUOUS QUANTITY IS THE WRONG SHAPE, NOT A SMALLER RIGHT ONE**, and it fails where the answer looks most confident.

**BISECT ASSUMES DETERMINISM**, so aimed at a flaky subject it returns a confident, well-formed, wrong answer, and nothing about the bisect can say so. **Repetition is the instrument.**

**`--only` SEPARATES FILES, NOT FACTS.** Two forms: the roster guard **caught it and named the WRONG CAUSE**; `bin/devbin`'s pair is caught by **nothing**. Same defect.

**AN INVARIANT THAT HOLDS BECAUSE NOBODY HAS WRITTEN THE DECLARATION THAT BREAKS IT IS UNEXERCISED, NOT ENFORCED.**

**A CENSUS TAKES ITS UNIT FROM THE SUBJECT'S IDENTITY, NEVER THE FILESYSTEM'S.** **A CORRECT IMPOSSIBILITY ARGUMENT AIMED AT THE WRONG PROPERTY** says nothing about what a value MEANS. **A CONTROL CAN BE GENUINE, CROSS-NODE AND BLIND BY CONSTRUCTION** -- ask what it CANNOT see. **A ZERO YOU CANNOT TRUST MUST NOT EXIT 0.**

**A STAMP TYPED BY FEEL IS FABRICATION AND THE MESSAGE CHANNEL HAS NO GUARD** (issue `0099`). I wrote `14:0xZ` today. **Cite the commit, never invent a better-looking minute.**

**`git show HEAD:` for source; `--only` takes the WORKTREE. VERIFY AN EXIT CODE BY CAPTURING TO A FILE. zsh: an unquoted `--include=*.rs` ABORTS the command** -- hit live today. **`cd` PERSISTS between Bash calls**; a wrong cwd is how a commit silently names nothing.

**MEASURE ONLY: nothing repaired, no estate written to, NOTHING about the release touched.**

## Decisions

- **(hv) v3.0.1 IS THE NEXT RELEASE ACT; docs precede the cut and ship with it; warn the fleet by PROPERTY not version.**
- **(vc) THE HOLD IS ON THE RELEASE AND THE NARRATIVE, NOT A FREEZE ON THE ESTATE** -- and **filing was never blocked**, only new BUILD units.
- **(vc) LAMPLIGHT GOES TO ITS OWN NODES; ic PACKAGES THE METHOD** -- a number produced from here cannot be checked from here.
- **(ic) EXPOSURE IS NOT DAMAGE, AND A PREDICTOR IS NOT A CONFIRMATION.** Say **predicted-unconfirmed**.
- **(ic) TESTED DUPLICATION BEATS SINGLE-SOURCING** -- the template engine has no include form.
- **(all) Fold archives are `wip-fold-HHMMZ.md`**, append-only. **(vc) `add + commit --only + reset` is NEW files only.**
