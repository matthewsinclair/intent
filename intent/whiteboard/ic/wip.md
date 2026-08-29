---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-29 16:01Z
status: active
focus: "**AC-12.2 IS LANDED -- narrative, README, CHANGELOG all converged and committed.** AC-12.3 (3.0.1 release notes) + the 3.0.1 CHANGELOG entry are BLOCKED on ONE question asked twice and not yet answered: **is 3.0.1 cut from HEAD, or from before `8aebe2ce`?** Release-note fact 1 moved under me -- `ac new` REFUSES at HEAD now."
claims: [ST0065, ST0061]
---

# Interface Claude (ic)

## DOING -- AC-12.2 LANDED, AC-12.3 BLOCKED ON ONE QUESTION

### `AC-12.2` -- THREE FILES DONE

`ead0399f` + `39303df4` working-with-llms.md | `76a98089` README.md | `884950f9` CHANGELOG.md

**The narrative's gap was MISGUIDED, not stale** -- it said nothing about the store, so a reader following it hand-edits `acceptance.md` and loses it silently. Added the v3 preamble with the generated-vs-authored split **measured**, handing over the PROPERTY (read the banner in the file in front of you) not the table. **The banner is evidence in ONE direction:** `ST0056/WP/15/info.md` has none while all 38 siblings do; `intent doctor` is the check. **D1-D11 NOT re-verified and the stamp says so.**

**I CORRECTED MY OWN TABLE AN HOUR LATER.** I called `.canon` "the record"; the store is `intent/.cache/intent.db` and `.canon` is the **committed extract**. Telling a reader the JSON is the record invites the same silent loss one layer up. **The CHANGELOG's 3.0.0 entry had it right all along.**

**README was FALSE IN THE FIRST FIVE MINUTES:** clone-and-PATH hands them the v2 line, which refuses a v3 tree; `intent help` **refuses in the build on PATH.** **It now carries NO command list** and points at the generated reference -- drift removed structurally, not by care. **My own draft failed my own check** (`--non-test`; the flag is `--kind`).

**VOICE CALL MADE, IT WAS MINE:** emoji go. 28 -> 0. **Not the site rule applied by fiat** (vc was right to refuse that) -- the same judgement for the README's own reason.

### `AC-12.3` + the 3.0.1 CHANGELOG entry -- BLOCKED ON ONE QUESTION

**IS 3.0.1 CUT FROM HEAD, OR FROM BEFORE `8aebe2ce`?** Asked vc twice; not answered yet.

**RELEASE-NOTE FACT 1 MOVED UNDER ME AND I CAUGHT IT BY MEASURING.** "3.0.1 does not fix `ac new`" was measured **before cc's package landed**. At HEAD `ac_new` returns `CriterionExists` (`facade.rs:4574`) and its remedy names `intent ac edit` (`:942`). `8aebe2ce`, 13:31Z, **NOT an ancestor of `80d8b2ca`**. The shipped-tool half still stands; the 3.0.1 half is now conditional. **Writing it either way without the answer puts a false sentence in the one document where that is least recoverable.**

**The other three facts re-verified and hold:** keg ships no rule library (`0112b8c1` not an ancestor); `st repair` in `shipped` at the tag (117) and gone at HEAD (118), read off BOTH dispatch tables; 0133's framing unaffected.

### REPORTED TO vc, NOT EDITED -- their file

**`docs/index.md` says `brew install intent`; `docs/install.md` says `brew install matthewsinclair/tap/intent`.** The bare form resolves against homebrew-core, not the tap. **Two pages written in the same hour disagree on the first command every reader types.**

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
