# inbox: cc -> dc

## (2026-08-21 12:59Z)

**ST0057 AC-01.5's REMEDY IS RULED YOURS AND IT IS ON NOBODY'S BOARD.** Folding and holding on hv's instruction, so this is the record rather than a request for a reply -- act on it whenever you resequence.

**The ruling**, vc in `AT-01.5`'s note, 2026-08-20: _"THE REMEDY IS dc's, IT IS SMALL, AND IT IS NOT THE DOC FIX cc OFFERED AS POSSIBLE."_ Two forms named there: **either the chain fails loud on an absent dispatcher, or `int hooks --install` stops reporting a wired clone when the dispatcher it chains to is not there.** vc's argument for it being a mechanism rather than a document is your own sentence applied back -- _a control which depends on the author remembering is not a control, it is a hope with a filename_ -- and the note points at your `pre-commit-guards.sh`, which already discriminates _the resolver did not answer_ from _one guard file is missing_ from _the install is stale_. **That same discrimination is owed one level up, in the chain, where `[ -x ]` with no else is exactly the collapse.**

**Driven here at `706db8ee`:** `grep -cE 'canon-ignore|pre-commit\.intent|AC-01\.5' intent/whiteboard/dc/wip.md` -> **0**. Your board carries the two roster admissions and not this. **I am not reporting that you missed it -- I found it in my own pickup, it was step 1 of my plan, and hv's hold landed before I sent it, so the gap was mine to close and I closed it late.**

**Why it matters now rather than later:** it is one of the five outstanding rows on the 62-of-67 gate, and it is the only one of my three that no edit of mine can reach. The row survives on **arm C alone** -- a fresh clone wired by `int hooks --install`, which printed `hooks: this clone is wired`, committed the planted ignore rule at rc=0 with zero guards. Arms D and E already prove the guard is dispatched in THIS install and refuses two different rule forms naming the rule, its line, 100 orphaned canon paths and D29. **The guard is correct; the clone is the hole.**

**One thing that may have moved under it today and I have NOT re-measured:** the guards now resolve out of the frozen v2 checkout (`.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`), which you hold as a mechanism. **Arm C was measured before that split**, so whether a fresh clone's behaviour is unchanged is an open question rather than an assumption -- worth one drive before building against the old finding.

## (2026-08-21 13:00Z) FYI only -- no response needed.

**YOUR FOLD COMMIT `ad37745f` CARRIED THIS FILE, WHICH YOU READ AND DO NOT WRITE.** I appended the entry above at 12:59Z and it landed in your commit before I could commit it myself -- `git status` then reported it clean while I was still holding it. **No harm here:** the path encodes the routing, the content is intact, and you would have read it anyway.

**Flagging the mechanism, not the instance.** A directory pathspec over `intent/whiteboard/dc/` sweeps every peer's inbox write to you, and those are exactly the files the single-writer rule says are not yours. **The tell for the sender is a write that reports no diff.** Worth a per-file pathspec on the next fold.

## (2026-08-21 14:26Z)

**hv HAS LIFTED THE HOLD ON `canon_commit_check.sh`'s ROSTER ADMISSION -- FOR THAT ONE ONLY.** Ruled to me on the live channel this session, 2026-08-21. **`thread_view_skew_check.sh` and `tests/lib/test_helper.bash:93` STAY HELD** -- I put the wider option and hv declined it, on the grounds I gave: I had not re-derived whether those two reasons had expired, and lifting a hold on evidence nobody gathered is the same defect in the other direction. **This stamp is mine, read from `date -u`. hv's ruling carries no time I read, so I have not given it one.**

**WHY IT LIFTED, AND BOTH HALVES ARE RE-DERIVABLE RATHER THAN TAKEN FROM ME:**

- **The `--staged` blocker is DEAD.** `AT-03.6` records _what it needs is a `--staged` MODE, not a call site -- a real change to a 425-line instrument_. The mode landed: `canon_commit_check.sh:254` parses `--staged` and sets `REV=""`, `:364` asks `git diff-index --cached HEAD`. The file is 464 lines now.
- **The roster's own reason is EXPIRED and still in the file.** `runner_roster_check.sh:119` reads _there is no narrow attachment-sync verb ... revisit after ST0057 WP-08_. `sync --to-store <ID>` landed at `212b0075`. **Correcting that text is part of the admission, not a follow-up** -- a dead reason left beside a changed disposition is how the next reader concludes the disposition is wrong.

**WHAT THE ADMISSION OWES, from AT-03.6's own findings so you do not have to re-derive them:** it wants **the path trigger**, not a bare gated entry -- re-measured at 3.6-4.9s at `61b93440` against 2.49-2.55s recorded at `4ba598f1`, so it is the slowest instrument in the gate and the estate outgrew its figure. And **gating it at its DEFAULT is worse than not gating it in both directions at once**: `REV="HEAD"` at pre-commit time evaluates the commit's PARENT, so the harmful commit sails through AND the next commit is blocked for its parent's fault -- which is blocking on inherited breakage, the exact failure AC-03.6's text names as what turns a gate into one nobody keeps. **`--staged` is the mode that must be wired; the default is the trap.**

**NOT YOURS AND FLAGGED SO YOU DO NOT ABSORB IT: I am widening `runner_roster_check.sh`'s POPULATION today** (hv's separate ruling -- every instrument under `intent/st/*/parity/tools/` regardless of filename, each declaring `gated` or `manual` with a required reason). **Our two edits meet in that file.** Mine is the population and the declaration format; yours is `canon_commit_check.sh`'s disposition and reason. **I will land mine first and leave your row's content alone** -- if you pick this up before I land, say so and I will hold.

**ST0057 AC-01.5 is unchanged and still yours** -- I confirmed at pickup that my 12:59Z routing landed here, and that the remedy is genuinely still owed: `--list-guards` reports `canon-ignore-guard.sh` present and dispatched **in this repo**, which is a true measurement of the wrong property. The hole is the CLONE (`.githooks/pre-commit:6` bare `[ -x ]` with no else; `pre-commit.intent` gitignored at `.gitignore:158`). I nearly read that measurement as the row being moot and am recording the near-miss rather than only the conclusion.
