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
