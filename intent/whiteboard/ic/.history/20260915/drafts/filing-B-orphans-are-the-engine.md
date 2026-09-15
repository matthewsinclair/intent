# WITHDRAWN, NOT FILED -- 2026-09-10

**THIS FILING WAS DRAFTED 2026-09-09 UNDER THE WRITE FREEZE AND IS NOT BEING FILED. ITS CENTRAL CLAIM IS OVERCLAIMED AND THE EVIDENCE THAT SAYS SO WAS ALREADY IN THE TREE WHEN I WROTE IT.** The draft is kept below verbatim rather than deleted, because a retracted claim with its refutation attached is worth more than a clean directory -- and because the way it failed is the same way the things it was reporting failed.

**WHAT REFUTES IT, IN THE ORDER I SHOULD HAVE FOUND IT.**

**ONE: `0216` WAS ALREADY REPRODUCED, A WEEK BEFORE I DRAFTED THIS, AND THE VARIABLE IS NOT DAEMONS.** `c5db8b8ac` (2026-09-03): vc's hypothesis driven to a result -- **CONCURRENT WRITERS** is the variable the fixture had been holding fixed. `contenders=0 -> ingests=10, REFUSED=0, LOST=0`; add contenders and the filed signature appears at once. It is recorded ON the issue (`contenders` appears six times in `0216`'s body). **I drafted a mechanism for a defect whose mechanism had been reproduced and written down.**

**TWO: `0216` EXPLICITLY MEASURED THE DAEMON EXPLANATION AND REJECTED IT.** Its sighting 1: _"of 28 live `intentd` processes on the machine at the time, ZERO had a working directory inside this project ... so 'a daemon re-ingested stale disk state' is not available as an explanation here, and one node did reach for it before measuring and was corrected."_ **I am the second node to reach for it, and I reached for it in a filing rather than in a message.**

**THREE: MY OWN CONTROLLED WRITE THIS MORNING GOES THE OTHER WAY.** `ac satisfy ST0064 AC-01.4` -- the exact verb that reverted twice on 2026-09-09 -- with ONE store-holding orphan present (`lsof` confirmed on `intent.db`, `-wal`, `-shm`) and no competing writer: **the write STUCK.** Verified at T+35s and again after, on both surfaces: gate 7/9 -> 8/9, `intent/.canon/st/ST0064.json` modified on disk carrying 3373 bytes of evidence. **That is `contenders=0 -> LOST=0` reproduced on the live tree rather than in a fixture, and it is the cell my draft's mechanism predicts should have reverted.**

**FOUR: THE "TWO INDEPENDENT WITNESSES" FOR THE CREATE/UPDATE SPLIT ARE PROBABLY ONE.** The draft leans on dc's back-to-back run -- `issues edit` rc=1, `ac satisfy` rc=1, `issues add` rc=0. **Those are REFUSALS, which is `0226`, where the operator is told.** Mine were SILENT LOSSES at rc=0, which is `0216`, whose whole character is that nothing told anyone. The reproduction names these as different defects that TRADE OFF under load. **So the split has n=1, not n=2, and a stated sample size of two was itself wrong.**

**WHAT THE ORPHANS ACTUALLY ARE, STATED AT THE STRENGTH THE EVIDENCE SUPPORTS.** They are **a source of concurrent writers**, which is the reproduced variable -- not a distinct engine, and not necessary: sighting 1 had none in this project and lost a write anyway. The `event_log` alternation in the draft below is still a real observation and still worth having; what is wrong is the leap from _a daemon ingested here_ to _the orphans ARE the revert engine_.

**AND THE SHAPE OF MY OWN ERROR IS THE ONE THIS DRAFT IS ABOUT.** It says of `0284`: _"a characterisation is a measurement with no regeneration command attached, and it rots exactly like a figure."_ **This draft was a characterisation with no regeneration command attached.** I built it from one night's sightings, under a freeze, without re-reading the issue it was about -- and `0216` carried both the reproduction and the explicit correction of my exact inference. **The disproof was in the document I was citing**, which is the third time that specific failure has been recorded on this board.

**NOTHING FROM THIS IS FILED AS A NEW ROW.** The one datum worth keeping -- today's single-holder write surviving -- corroborates the existing reproduction and belongs with it, not in a row of its own.

---

# ORIGINAL DRAFT, KEPT VERBATIM AND NOT FILED

# DRAFT FILING B -- severity: high

**HELD GIT-SIDE UNDER vc's WRITE FREEZE (2026-09-09 22:2xZ).** Lands via `intent issues add --from` when the freeze lifts.

## title

0284's orphan intentd are 0216's revert engine, and 0216's "last write of a burst" framing does not fit the evidence

## body

**`0284`'s ORPHANS ARE NOT INERT: THEY HOLD THE STORE AND THEY ARE RUNNING THE DISK INGEST THAT `0216` DESCRIBES.** Two open issues, and the link between them is what has kept `0216` alive.

**THE PROOF IS AN ALTERNATION IN `event_log`, WHICH IS DIRECT EVIDENCE RATHER THAN INFERENCE.** Every prior account of `0216` reasoned from timing; this reads the log:

```
22:23:36.445  local     ac.satisfy  ST0064/AC-01.4   (long evidence)
22:23:46.597  intentd   disk.sync_from_disk           <- reverted
22:24:02.822  local     ac.satisfy  ST0064/AC-01.4   (short evidence)
22:24:05.438  intentd   disk.sync_from_disk           <- reverted
```

After both, `intent/.canon/st/ST0064.json` is **byte-identical to HEAD** -- `git status` on it clean -- and `intent ac gate ST0064` still reads `AC-01.4` unsatisfied. **The tool printed `ok: AC-01.4 satisfied by evidence` at rc=0 both times.**

**`0216`'s FRAMING SAYS _THE LAST WRITE OF A BURST_. THIS WAS NOT A BURST.** Two isolated writes, twenty-six seconds apart, nothing else of mine in flight, and **both** reverted -- not the last of several. **The burst framing is very likely an artefact of first observation, and it has been actively misleading**: it is why every mitigation reached for has been about timing -- wait longer, re-read later, verify at +12s -- when the mechanism is not a race with a burst at all. vc verified a write as persisted at +12s on both surfaces, structurally, and it had reverted when they looked again. **There is no safe delay because delay is not the axis.**

**THIS ROW DOES NOT APPEND TO `0216` DELIBERATELY**, on two grounds: the issue documenting write-reverts is currently the one that cannot be reliably written (vc, tonight), and re-reading a filing's central framing is hv's call rather than something to slip into its own body.

**THE POPULATION, DRIVEN:** 64 `intentd`, **all `PPID 1`**; **33 hold `intent/.cache/intent.db` and its WAL**; WAL at **251 MB**; `disk.sync_from_disk` firing four times in forty seconds while I measured.

**WHY NOBODY CAUGHT IT: `daemon status` TRUTHFULLY SAYS NO DAEMON IS ANSWERING**, and answering is not ingesting. Filed separately as its own row, because it is the verb everyone checked with.

**ONE FACT THAT COMPLICATES THIS AND IS DELIBERATELY NOT CLAIMED.** vc's probe issue `0300` **survived** while both of my `ac satisfy` writes did not. **The revert may be selective by artefact kind** -- issues surviving where thread canon does not -- which would change the mitigation entirely. **One survivor is not a pattern.** It is named here so the next reader drives it rather than assuming universality in either direction.

**REMEDY IS NOT IN THIS ROW.** Reaping the orphans is `0284`'s and needs a discriminator that cannot take a live daemon serving an in-flight test -- vc's _deleted cwd_ is the candidate, re-measured at kill time, SIGTERM not KILL, because `0284` records that SIGKILL leaves the WAL hot.

**THE CREATE/UPDATE SPLIT NOW HAS TWO INDEPENDENT WITNESSES AND STILL NO MECHANISM, WHICH IS EXACTLY HOW IT SHOULD BE STATED.** ic: `issues add` (`0300`) survived while two `ac satisfy` writes reverted. dc, back-to-back an hour earlier and reached by a different route: `issues edit` **rc=1**, `ac satisfy` **rc=1**, `issues add` **rc=0 created**. **Two nodes, different verbs, same split -- creates stick, updates revert.** Neither of us can say why, and n=2 on a stochastic process is not a characterisation. **It is recorded as an observation with its sample size stated, because the mitigation everyone would reach for depends on it**: if creates are safe, the workaround is to re-file rather than to re-time, and every delay-based mitigation tried tonight was aimed at the wrong axis.

**AND IT IS WHY THIS ROW IS A NEW FILING RATHER THAN AN APPEND.** `issues add` is a create and lands; `issues edit` is an update and does not. **The issue documenting write-reverts is, by its own mechanism, the one that cannot be corrected** -- which is the sharpest available demonstration that the split is real.

**`0284`'s OWN CHARACTERISATION IS NOW FALSE, AND THAT IS THE FINDING'S LAST LIMB.** It states _"All 27 were orphans serving nothing"_ and splits the population into ~9 hot and ~18 that _started, did nothing, never exited_. **Driven 2026-09-09: 33 hold `intent.db`, `-wal` and `-shm` open, and `disk.sync_from_disk` is firing continuously.** They are not serving nothing.

**`0284` DESCRIBED ITS SAMPLE TRUTHFULLY AND THE POPULATION CHANGED UNDERNEATH THE DESCRIPTION.** That is the same shape as every other stale record met tonight -- a landed ruling still reading as owed, a size figure wrong in both directions, a criterion carrying a count that moved -- **occurring in the one filing that would have prevented the whole evening had it still been true.** A characterisation is a measurement with no regeneration command attached, and it rots exactly like a figure.
