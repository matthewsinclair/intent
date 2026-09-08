# laksa-vc's independent read of ST0068 AC-03.1 / AC-03.2

**RECEIVED 2026-09-08 17:54Z over SendMessage, in reply to vc's ping at 17:38Z. WRITTEN TO DISK BECAUSE IT ARRIVED ON A CHANNEL THAT DOES NOT SURVIVE A SESSION, AND IT IS THE EVIDENCE FOR TWO ACCEPTANCE ROWS.** Not yet acted on -- vc was told to hold. Verbatim below; vc's own assessment is NOT mixed in, deliberately, so a later reader can separate the reader's findings from the author's response.

**VERDICT: BOTH ROWS FAIL.** AC-03.1 on seven logged questions; AC-03.2 on two independent grounds.

**THE INDEPENDENCE PROPERTY HELD.** They read all 448 lines before concluding, and did not open the consolidation commits or the ST0068 notes. **The mitigation held too: every question names where they looked**, which is what makes a false fail cost one lookup instead of a revision.

**AND THEY DISCLOSED A BROKEN INSTRUMENT IN THEIR OWN READ, UNPROMPTED:** `grep -E 'licence\|license'` -- under `-E`, `\|` is a LITERAL PIPE, so the sweep silently matched nothing and they nearly logged _licence appears nowhere_ off a search that could not have found it. Re-run with real alternation it returns two hits. **The finding survived and its first evidence did not.** That is the honest-and-blind-grep family, self-caught, in a read vc is relying on -- and it is the same class vc filed against its own instrument twice today.

---

## AC-03.1 -- FAIL, on seven questions a builder would have to invent an answer to

Ordered by how much they block. Section references are laksa-vc's.

1. **WHICH TERMINAL CAPTURE GOES ABOVE THE FOLD?** Sec 7.3 requires "one real terminal capture proving the claim", calls it esbuild's move, and never says which command or output. Sec 6.2 shows `intent st new "Port the acceptance gate"` but as an illustration of the component, not as the home page's capture -- and Sec 6.2 forbids simulating one, so it cannot be invented. Looked: 6.2, 7.2, 7.3, 7.5, 12. **Sub-question in the same place: "above the fold" at what viewport?** Nothing in 5, 7.3 or 9 bounds it, and with `--t-display: clamp(2.5rem,6vw,4rem)` plus a paragraph and an install line ahead of it, whether a capture fits is a real constraint.
2. **WHICH PLATFORMS, AND WHAT ARE THEIR INSTALL LINES?** Sec 7.7 says "with the platforms named" -- plural -- and the only line anywhere is `brew install matthewsinclair/intent/intent`. Looked: 7.2, 7.7, 7's closing paragraph, and a whole-document grep for `linux|macos|darwin|curl|tar|binary` returning zero hits. **Internal tension: 7.7 says name the platforms, 7's closer says "One primary action, and it is `brew install ...`".** Either is satisfiable; both together is not obviously so.
3. **IS THE INSTALL LINE A TERMINAL BLOCK OR INLINE CODE?** It appears as a bare code span with no `$` and no output. Under 6.2 it needs a non-selectable, non-copied `$`; under 6.3 it gets no prompt and no border. Looked: 6.2, 6.3, 7.2, 7.7. **Small, but it is the page's single primary action, so a wrong call is on the install path.**
4. **WHICH HEADING IS THE `h1`?** Sec 9 requires one `h1` and no skipped levels -- a checkable budget. Sec 7's blocks are all drafted as `###`. Sec 4 gives three heading steps and says there is no `h5`. Looked: 4, 7, 9. **Nothing maps 7's blocks onto a heading level, so the builder guesses at the thing 9 will fail them on.**
5. **WHICH PASSAGE IS THE RUBRIC?** Sec 6.5 defines it, caps it at one or two per page, and says a page with four has none. Sec 7 supplies all the copy and nominates none. Looked: 6.5, all eight blocks of 7. **The document wrote its own copy, so this is squarely a question it could have answered.**
6. **IS THERE SYNTAX HIGHLIGHTING ON THIS PAGE AT ALL?** Sec 6.3 says "optional and must be build-time"; 11.E decides only the palette and opens "If build-time highlighting is used". Looked: 6.3, 11.E, whole-document grep for "highlight". **The prior question is unresolved and the register presupposes it.** Also an AC-03.2 finding -- see ground 2.
7. **WHAT DO THE LINKS POINT AT?** Sec 6.1 and 7.1 give header `docs` and `github`; 7.8 gives footer docs, GitHub, changelog, licence, author. **No URL appears anywhere.** The org is inferable from the brew tap and `docs` presumably means `docs/`, but **the LICENCE is never named** -- the corrected grep returns two hits, Sec 4's font-licence column and the footer list. **Weakest of the seven and nearly not logged.**

**WHAT THEY DELIBERATELY DID NOT COUNT, AND THE REASONING IS THE VALUABLE PART:** the placeholder accent (3, 11.A) and the thread figure's composition (11.G) are declared open decisions. Building requires resolving them, but the document says so, **and AC-03.1 is about questions the thread could have answered -- not decisions it deliberately routed outward. Counting those would have made the row unfailable in one direction and meaningless in the other.**

## AC-03.2 -- FAIL, on two independent grounds

### Ground 1: limb completeness (decision / constraint / what-breaks)

```
A accent        D ok  C ok (1 inheritance)        B ok (error leaves red, or dE 13.8 collision)  PASS
B search        D ok  C ok (9 zero-off-origin)    B ok (both branches, and the timing argument)  PASS
C wordmark      D ok  C ok (no logo exists)       B MISSING                                      FAIL
D versioned     D ok  C ok (hv 2026-08-29)        B ok (not recoverable once reindexed)          PASS
E highlighting  D ok  C ok (6.3 rules out client) B ok (two colour systems)                      PASS
F --note        D ok  C ok (must not compete)     B ok (a colour doing no work is worse)         PASS
G figure        D ok  C ok (6.6)                  B = "nothing structural"                       PASS, noted
H toggle        D ok  C ok (3 both palettes, 8)   B ~ cost, not breakage                         marginal
```

**C IS THE FAIL.** Its "what breaks either way" says the placeholder may be right permanently and that a drawn mark must earn its place against Sec 8. **That is a hurdle facing one branch, not a consequence of taking either. Nothing is stated to break.** Two of three is a fail.

**G PASSED DELIBERATELY AND WAS FLAGGED RATHER THAN BURIED:** "Nothing structural -- this is a genuine drawing decision and belongs to whoever draws it." **An honestly filled limb -- "nothing" is a real answer.** It makes G a task rather than a decision, **and it is the model C should have followed: if "nothing breaks" is acceptable, C could say so plainly and does not.**

**H PASSED MARGINALLY, AND laksa-vc NAMED IT AS THE ONE THEY EXPECT TO BE ARGUED WITH ON.** Its consequence limb is what a toggle COSTS (wire `[data-theme]` both ways, JS as progressive enhancement) rather than what breaks. **Restating a constraint in the consequence slot is a weaker form of C's defect.**

### Ground 2: completeness

The criterion says EVERY unresolved design decision reaches the agent. **Sec 6.3 leaves "is there syntax highlighting at all" open by calling it optional, and Sec 11 has no entry for it** -- 11.E presupposes the answer and decides only the palette. **So an unresolved decision sits outside the register, and the register entry that touches it is downstream of a question nobody asked.** Sec 12 says "eight of its decisions are open (11)", which reads as exhaustive and, on this one, is not.

## What laksa-vc said in the document's favour, recorded because a fail is not the whole read

Sec 11's three-limb format is doing what it claims -- **five of eight entries are genuinely decidable from what is written**, and D in particular tells the reader that the tidy answer is the destructive one, which is the thing a design agent would otherwise get wrong. Sec 1's self-correction about the curated prefix list and Sec 3's note about WCAG being blind to hue are both the document arguing against its own earlier draft in public. **Their summary: the failures are gaps in Sec 7's specificity, not defects in the system.**
