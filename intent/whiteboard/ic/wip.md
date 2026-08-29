---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 16:39Z
status: active
focus: "**AC-12.2 AND AC-12.3 ARE BOTH COMPLETE** -- narrative, README, CHANGELOG, and the v3.0.1 release notes, all committed. **Left unsatisfied in canon deliberately: ST0056 is vc's claim and they have the evidence.** Two of vc's four surface changes are rulings not code and are deliberately absent from both documents."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING -- AC-12.2 AND AC-12.3 BOTH COMPLETE

**`AC-12.2`** `ead0399f` + `39303df4` working-with-llms.md | `76a98089` + `f3769b87` README.md | `884950f9` + `c8df89c8` CHANGELOG.md
**`AC-12.3`** `25157b2c` `docs/releases/3.0.1/RELEASE_NOTES.md`

**NOT SATISFIED IN CANON BY ME.** ST0056 is **vc's claim**; I have handed them the evidence and left the sequencing where it belongs. `intent doctor` already reports a status-gate disagreement on that thread, which is a second reason not to write into it uninvited.

### WHAT I LEFT OUT OF BOTH DOCUMENTS, DELIBERATELY

**vc handed me FOUR surface changes; TWO are rulings, not code.** `issues edit` is absent from the register, from `intent-cli/src`, and from `intent issues --help` -- **three independent surfaces, all negative.** `st edit` is still the path-printer (`Usage: intent st edit <ID> [FILE]`, no `--editor`, no `--path`). `ac edit` and `at edit` both verify. **Documenting the other two would put verbs in a release note that the release does not contain** -- the exact defect the README was just fixed for. The notes carry a provenance paragraph saying so, and the structure takes them when they land.

**A RULING MADE THIS HOUR AND A CHANGE LANDED THIS HOUR READ IDENTICALLY IN A MESSAGE.** That has now cost me twice today: vc's retired fact (which vc caught) and this. **Not avoidable by care at the sending end** -- the fix is that a surface claim travels with what makes it checkable, which for a verb is just "it is in the register".

### THREE CLAIMS I INHERITED AND ONE I SHIPPED

**`intent doctor` does NOT check the support tree.** I took that from vc's `install.md` into my own upgrade instructions **before** checking it -- so it had already propagated once. No rules-or-skills check exists in `doctor.rs`; its finding kinds are `criterion`, `pre-commit`, `rev-parse`, `unchanged`, `nonsense`. **The direct test is `intent claude rules list`**, which reads the library out of the install and so fails on the actual fault rather than proxying for it. Reported to vc, not edited.

**I SHIPPED A BREW LINE THAT WOULD INSTALL NOTHING.** `matthewsinclair/tap/intent`; that repo does not exist (`gh` verified, nonsense control fails identically). Correct is **`matthewsinclair/intent/intent`** -- `bin/.devbin/cmd/macos:194` rules the fully-qualified form deliberately. **I did not derive a wrong answer; I applied the correct GENERAL convention (`user/tap/formula`) to a project that deliberately does not follow it.** Three of us wrote three different wrong spellings of a rule already written down in the file that publishes the formula. **And I was the one who had just told vc their two pages disagreed on that same command.**

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
