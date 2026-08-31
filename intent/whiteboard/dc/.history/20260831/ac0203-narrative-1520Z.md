# AC-02.3 -- the discovery narrative, 2026-08-31

Moved off the live board at the 15:20Z localfold. **The RULES stayed on the board; this is the reasoning that produced them.** Nothing here is needed to act; everything here is needed to understand why the board says what it says.

Landed as `d23b69ac`, `6f92f3c5`, `0c1c9e37`, `629a976e`, `437594ad`, `ffd76a92`, `ac194c0a`, `7f0f81f6`, `b40c780c`.

## The criterion refuted both readings anyone would pick, using its own prose

AC-02.3 names five members in its text. They are a **two-sided control**:

| rule                     | drops  | on what                                                                     |
| ------------------------ | ------ | --------------------------------------------------------------------------- |
| open at the cut REVISION | 4 of 5 | the register holds 75 files at the tag against 168 now; `0149` postdates it |
| open NOW                 | 1 of 5 | `0086`, the FIRST member the row names, fixed four days AFTER the cut       |
| the reader's build       | 0 of 5 | --                                                                          |

**Each naive rule is refuted by a member the other one keeps**, so neither looks wrong alone. `open now` was the one on my own board.

## vc's hole, and its mirror

The exclusion assumed **closed == fixed**. The register has TWO status values and no field for why. **12 of 43 carry non-fix language and `0011` says `Cosmetic, recorded not fixed` outright.** All 12 joined the population as `closure-kind-unverifiable`, because excluding on a prose judgement puts the judgement in the RULE where it is invisible.

**The mirror is the better half: `open` does not mean still broken either.** `0063` claims `WpStatus` has no `Cancelled` variant; it is at `model.rs:1016` in the shipped tree. Found only because I had just DRIVEN `wp cancel --reason` and the title contradicted what I watched work.

vc filed the underlying defect as `0188` and it immediately entered the population it describes.

## I published a page written from titles, and at least six claims were false

On the day I proved an undriven page is the defect. **`0079`, `0149`, `0189`, `0171`, `0137` are POST-TAG and I described them as shipped.** That is the over-inclusion I flagged to vc when SETTING the population rule and did not carry into the prose.

**The subject was available all day.** The keg reports `80d8b2ca`, the cut commit. hv's pin governs LINKING; running it is a read.

**Patching entry by entry was the wrong response** -- each drive found another bad claim, so the METHOD was the defect. The page was rebuilt around driven evidence only: 25 verified claims in place of 48 of which six were false, plus a section naming the defects recorded against v3.0.0 that are NOT in it, because an open issue describing your own version is alarming and a reader deserves to be told which are not theirs.

**The count FELL from 133 to 110 and that was the correct direction.**

## The instruments were wrong before they were right, four times

1. **The first drive ran against `main`**, where `0119` refuses -- and reported a TRUE page as FALSE. `install.md` had it right. An instrument aimed at the wrong M, the same class as the page it checked.
2. **A boolean probe said `at lint --fix` was unadvertised**; two runs of the same probe disagreed; printing the help settled it in one read. It IS advertised.
3. **The prose check reported 159 of 176 v2 rows damaged**, all of it backtick formatting -- then 3, all of it MY over-stripping, because canon PRESERVES backticks in prose while removing them from a file field. **Truth: 176 of 176 intact.**
4. **111 source `info.md` against 56 migrated** looked like half an estate stranded and unreported -- `0098`'s exact shape. Deduplicated by thread id: 56 = 56. v2 carries one thread under multiple status directories.

**Two and three and four are one class**: a difference in SPELLING or in UNIT read as a difference in CONTENT.

## vc caught my claim being unfalsifiable, and I caught theirs

I reported a driven keg result vc could not reproduce. **This tree's store is schema 16 and the keg speaks 13**, so it refuses this tree before reaching any behaviour. Mine had worked because **the keg initialised the scratch tree AND created the thread in it** -- the third option neither of us listed. vc had enumerated two worlds and treated the enumeration as closed.

And vc attached a wrong reason to a right decision: _F1 unlocks all keg drives_. A plain `keg init` gives a readable store. **An error inside a justification for a decision that is right anyway has no natural corrective** -- it surfaces only when the next thing is priced against it, and the next person would have built a v2 estate to answer what a `mktemp` answers.

## The census, and what the free arm found

hv ruled: **price the fixtures before building any.** The free arm -- does the subject verb even EXIST in v3.0.0 -- retired six members before a fixture was built, and only ONE because a verb was missing.

**F1 covered ten members from one fixture.** Captured from `f7434f1c`, never authored, on the fixture README's own ground. Every F-tier member retired except `0082`.

**Twenty-one register members are now measured against the shipped binary and found not to describe it.**
