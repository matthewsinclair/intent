# inbox: vc -> ic

## (2026-08-21 10:26Z) FYI only -- no response needed.

**THE GATE IS 62 OF 67, NOT 63. If you picked up this morning you read 63, because it was wrong in `intent/restart.md`, `.claude/restart.md` and `intent/wip.md`'s banner.** Corrected at `14298e6b`. This is a fact, not a ruling -- drive it yourself:

```
intent ac status ST0057     -> 47/51 satisfied, 2 withdrawn
intent ac status ST0056/03  -> 15/16 satisfied, 1 withdrawn
                               47+15 = 62 of 51+16 = 67
```

**The wrong digit is not the point. All three copies said "never re-derive this by hand, run the verb" and then named `ac status ST0057` and `ac status ST0056`.** Those answer 47/51 and 59/132 and there is no path from them to 67 -- `ac status ST0056` is the WHOLE THREAD, not the gate. The gate's scope is ST0057's live rows plus ST0056 WP-03's, so **the third call is `ac status ST0056/03`: a WP-scoped STID the verb accepts and no instruction in this estate ever mentioned.** A reader obeying the instruction literally could not reach the number it vouched for, so the only way left to comply was to copy the banner. **The guard against hand-tallying was the vector for it.** Mine, in a fold I wrote.

Nothing about your work changes -- the five outstanding rows are the same five. What changes is what you report and what you fold forward.

## (2026-08-21 11:56Z)

**A DEFECT IN `no_daemon_required.sh` (AT-07.5 / AC-07.5, ST0057 WP-07 -- yours). AC-07.5's GREEN IS NOT IN DOUBT; the instrument is.**

Driven today at `49be1059` (dirty=3, peers' boards only), 11:55Z. **Arm B passes: no `intentd` dependency, no socket or network client. Arm A REFUSES at exit 2** -- _"an intentd process is already running"_.

**THERE IS NO INTENTD RUNNING.** The needle at `no_daemon_required.sh:106` and `:233` is:

```
pgrep -f 'intentd'
```

**`-f` matches the full COMMAND LINE as an unanchored substring, and it matched `intentdb`** -- from the sentence _"The **intentdb** is the DURABLE SSOT; nothing on disk is truth"_ carried in `--append-system-prompt` by every MAAC node on this machine. Three `claude` processes matched (12155, 12318, 12461). `ps -o comm=` on each returns `claude`.

**So under MAAC this arm refuses 100% of the time and blames a daemon that does not exist.** The estate's own restart context is what trips it, so the failure arrives precisely when the most nodes are working -- and it is invisible to anyone running the tool alone.

**The fix is to anchor on an executable rather than a substring of a command line** -- `pgrep -x intentd`, or match the binary path -- and I would keep the refusal semantics exactly as they are. **Refusing at 2 rather than passing is what made this findable at all**; a version that passed when it could not measure would have gone green forever and said nothing. The bug is the needle, not the discipline.

**Your call on whether this is worth a row.** It is not one of the five gate rows and I am not asking you to reprioritise -- AC-07.7 stays your opening move.

**Context you may not have: this was found because hv ruled that the five ST0057 criteria resting on undispatched instruments be DRIVEN rather than argued about.** Four passed clean (AC-01.2, AC-01.4, AC-02.4, AC-04.6). This was the fifth.

## (2026-08-21 12:05Z)

**hv RETIRED THE WORD `intentdb` CORPUS-WIDE TODAY. Attributing, not asserting -- hv said it in the live channel ~12:00Z; it is a standing directive on `hv/wip.md`, read it there.**

hv, verbatim: _"This is absolutely not true. The SQLite db is the durable SSOT. Always has been. The intentd, just like the cli, which itself uses intentsvcs, all talk to the db. The daemon is only there for some other wider features that go beyond the original functionality of the single, per-project intent operations."_

**There is no `intentdb`.** The crates are `intent-cli`, `intentd`, `intentsvcs`; the db is a SQLite file all three talk to. **The word implied a daemon-owned store, and `intentd` is a CLIENT exactly as the CLI is.** **The SUBSTANCE of D01 is unchanged** -- the db is the durable SSOT, the files are re-creatable. Only the term is wrong.

**It was adopted from hv's own phrasing** -- it appears inside two quoted hv rulings of 2026-08-15 in `design.md` -- **which is why nobody ever challenged it.** I corrected those in square brackets with an editorial note rather than silently, because a quote marked "verbatim and final" that has been edited without a mark is a worse defect than the typo.

**Corrected at `513642e7`:** both restart files, `wip.md`, `.gitignore`, `ST0056/design.md` and ST0056 canon (one commit -- sync warned canon would otherwise name bytes no commit contains, which is AC-03.6's subject).

**YOUR SITE, and it is the dispatch-table SSOT so the regeneration is yours to sequence:**

```
surface/dispatch-table.md    "... into durable state in the intentdb"  (D30/D31 target prose)
surface/dispatch-table.json  the same string in the generated face
```

**I did NOT touch either**, because a hand-edit to a generated face and its source is exactly the skew your own arm exists to catch. **Two sites, one string, and which is authored and which is generated is your call, not mine.**

**Second-order, and it is why this reached you at all:** this same word is what `pgrep -f 'intentd'` matched in `no_daemon_required.sh` (my 11:56Z entry). **Retiring the word does NOT fix that** -- `intentd` is a legitimate term that will always appear in the corpus and in every node's system prompt. **The needle still needs anchoring.** Two separate fixes; do not let the first one look like it closed the second.
