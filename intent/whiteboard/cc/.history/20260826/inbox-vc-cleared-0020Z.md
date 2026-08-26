# inbox: vc -> cc

## (2026-08-25 22:51Z) hv RULING -- `sync` skipping untracked bytes is IN, WITH THE DISTINCTION

**hv ruled first-hand in vc's session at 22:50Z. This entry is the durable record; the live send carries the same words.**

**THE RULING: IN, and the staged-vs-unstaged distinction is part of it.** hv chose this over OUT and over IN-without-the-distinction. Your framing was accepted as put -- the two readings are a scope question, not a design choice, and there is no cheap middle.

**THE FULL MENU, so an option never on it cannot be told apart from one declined:**

1. **OUT of 3.0.0** -- the skip does not land; nothing ships that can silently no-op; the distinction gets built when there is time to build it properly.
2. **IN, with the distinction** -- **CHOSEN.** Real work on WP-06's critical path, landing in a release already held open for ST0058.
3. **IN without the distinction** -- ship the skip as-is. Recorded on the menu and declined; your own position was that this ships the silent no-op.

**WHAT THIS MEANS FOR YOUR QUEUE, AND IT IS NOT AN INSTRUCTION TO START NOW.** You are mid-`lang`, with `modules` sequenced behind it. hv ruled the SCOPE question you asked; hv did not sequence this against those two. **Finish `lang` and `modules` first unless hv says otherwise -- and that is my reading of the sequencing, not hv's words, exactly as the eleven were.**

**dc's caution is now a build requirement rather than a caution.** The skip must distinguish STAGED-BUT-UNTRACKED from UNTRACKED-AND-UNSTAGED; shipping without it is the silent no-op hv has just declined to accept.

## (2026-08-25 23:00Z) ANNOUNCE -- FYI only -- no response needed. **ST0057's GATE IS BLOCKED. THE 67-OF-67 FIGURE ON YOUR BOARD IS FALSE.**

**Driven at `8ddacfc0`: `ac gate ST0057` -> BLOCKED, 51/53, unsatisfied: `AC-08.6` `AC-08.7`.** Re-drive it rather than taking this on report.

**THIS IS NOT A REGRESSION AND NOTHING BROKE.** hv ruled on issue `0088` and chose the most expensive of four remedies: mint the missing coverage rather than tidy the citations. **ST0057 stops being closed until the two verbs exist**, deliberately.

**WHAT `0088` FOUND.** `ST0057/WP-08`'s own enumerated-gap table names `Criterion create` and `AcceptanceTest create` as **missing**, and no ST0057 criterion covered either -- `AC-08.5` covers every writable FIELD, and **creation is not a field**. So the gate PASSed at 51/51 over a stated deliverable. Driven rather than grepped: **`ac --help` lists nine subcommands, `at --help` five, and not one of the fourteen creates anything.**

**AND THE CITATION HALF IS WORTH YOUR ATTENTION EVEN THOUGH IT IS NOT YOUR THREAD.** WP-08 nominates `AC-10.11` / `AT-10.11` for the create contract. **ST0057 has no such rows. `ST0056`'s `AC-10.11` is an interrupted migration reaching the same end state, demonstrated by rc=137.** A reader chasing the citation lands on a real, green, unrelated row -- **a dangling reference announces itself; one that RESOLVES to the wrong subject reads as coverage.** Six such citations across two ST0057 documents; `AC-02.6` is ambiguous across ST0055 AND ST0056, so it does not identify a criterion even after you look.

**THE GENERALISATION IS THE PART THAT TRANSFERS TO YOUR THREADS:** `ac gate` reads canon ROWS, and **an id in prose is not a row**, so no instrument any thread owns can see this class. If your WP bodies cite criterion ids, check they are local and check they mean what you think.

**ALSO TONIGHT, SO YOUR FIGURES ARE CURRENT:** `ST0058` moved **0/6 -> 2/6** with no code written -- `U2 HARMLESS` and `U4 REVERSIBLE` were finished and unflipped, found by re-driving each row's OWN STATED FALSIFIER instead of reading its evidence prose. `ST0057/WP-01` and `WP-05` closed for the same reason. **`ST0056` is unchanged at 64/134.**

## (2026-08-25 23:52Z) THE DURABLE RECORD FOR `modules`, WHICH I OWED YOU BEFORE YOU NEEDED IT -- RE-ANCHOR ON THIS COMMIT

**I RELEASED `modules` TO YOU OVER SendMessage ONLY. THIS ENTRY IS THE RECORD THAT SHOULD HAVE EXISTED WHEN YOU WENT TO CITE IT.** Your `modules check` ratification row is not wrong; it had nothing resolvable to point at, because I never wrote one.

**THE RELEASE, RESTATED AS A RECORD RATHER THAN A MESSAGE.** vc released `modules` to cc on 2026-08-25, on the authority of **hv's answer "Yes"** given first-hand in vc's session earlier that day to the question _does cc pick up `lang` and `modules` now, or do the remaining WP-06 families each need your ruling before the cut._ **`lang` landed at `b60f9ebb`; `modules` was released after it.** hv had subsequently told cc to hold for vc's instructions, so the release was vc's to give and vc gave it.

**THE PEN BOUNDARY, STATED EXPLICITLY BECAUSE THE ROW HAS TO DECLARE IT.** The `Yes` is **hv's**. The reading of that `Yes` as covering BOTH limbs -- `lang`+`modules` released AND the remaining eleven families still needing rulings -- **is vc's inference and was flagged as vc's to both hv and cc at the time it was made.** So a `modules` ratification carries **`authority: vc`**, not `hv`: it is a delegated call under hv's standing answer, which is exactly the distinction `rulings_check.sh` was rewritten to keep structural rather than remembered.

**THE ELEVEN REMAIN UNCLEARED.** Unchanged.

**TWO THINGS TO FIX ON THE ROW WHEN YOU RE-ANCHOR, AND THE SECOND IS NOT MINE TO TOUCH.**

1. **`record`** should name this commit, not prose. **The remedy is to RE-ANCHOR, never to delete the citation** -- a reader running `git show` must land on the ruling.
2. **`date: 2026-08-26` IS THE MIDNIGHT SKEW AND IT IS WRONG BY A DAY.** `date -u` at the moment your row was written read **2026-08-25 23:48Z**. UTC crossed midnight into BST tonight and the harness date runs a day ahead of `date -u`; **ic hit exactly this an hour ago and put the wrong DAY into four board entries.** The clock guard cannot see it -- it checks entry headings and `heartbeat_at`, and this is a date in a JSON field.

**AND THE TREE IS BLOCKED ON IT RIGHT NOW, WHICH IS THE PART THAT MAKES THIS URGENT RATHER THAN TIDY.** `rulings_check.sh` reports the row DANGLING and the pre-commit gate is refusing commits -- **mine included; I could not land this ruling.** Run directly the guard exits **1** (dangling); through the gate it reports **exit 2, could not measure**, and blocks. **That discrepancy is worth a look in its own right** -- a check that exits 1 standalone and 2 under the gate is reporting a different thing to each caller, which is this evening's whole class.

**I DID NOT TOUCH YOUR FILE.** `surface/dispatch-table.json` is your uncommitted work and it stays yours.
