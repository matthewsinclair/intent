---
id: "0030"
title: intent upgrade stamps its backup directory in LOCAL time, so a retention sweep cannot order it
date: 2026-08-15
reporter: matts
status: OPEN
severity: medium
---

# 0030: intent upgrade stamps its backup directory in LOCAL time, so a retention sweep cannot order it

## Tags

backup, clock, upgrade, d35, latent

## Summary

`intent upgrade` names its rollback directory `backup-$(date +%Y%m%d-%H%M%S)` -- LOCAL time, no zone marker (`bin/intent_upgrade:117`). A local stamp does not sort chronologically across a DST transition, so any retention policy that picks "oldest by name" can delete the newer artefact and keep the older one. This is the same class the whiteboard clock guard exists for, one artefact over.

Latent today only because nothing sweeps `.backup/`. D35 introduces the first sweep, which is what makes it worth filing now rather than when it bites.

## Reproduction

At the end of BST, run an upgrade at 02:30 BST and another at 01:30 GMT thirty minutes later. The second is stamped `...-0130...` and the first `...-0230...`, so a lexical sort puts the later run first. Any `ls | head -n` or `sort | head` retention picks the wrong victim.

Direct evidence without waiting for October: `date +%Y%m%d-%H%M%S` and `date -u +%Y%m%dT%H%M%SZ` on this machine differ by the local offset and only the second sorts monotonically.

## Root Cause

`date` without `-u` reads the local clock. The project has hit this exact class twice already on the whiteboard -- entry headings stamped in BST sorting below correctly-stamped UTC ones -- which is why `lib/templates/hooks/whiteboard-clock-guard.sh` exists. The backup path was written before that lesson and never revisited.

## Impact

None today: no code sweeps `.backup/`, so nothing orders these names. The impact arrives with D35's rolling snapshot retention if anyone extends the sweep to `upgrade/` artefacts, at which point a backup deletes the wrong backup -- precisely the loss the mechanism exists to prevent.

The dc namespace ruling (2026-08-15) contains the blast radius in the meantime: **nothing ever sweeps `.backup/` root**, and upgrade artefacts live there. So this is a precondition of extending retention, not a live defect.

## Proposed Fix

Stamp `date -u +%Y%m%dT%H%M%SZ`, which sorts lexically and chronologically at once, and write into the `.backup/upgrade/<UTC>/` namespace at the same time (one change, one place). Existing `backup-<LOCAL>/` directories at `.backup/` root are left exactly where they are -- they are user rollback data and no sweep can reach them under the namespace rule.

Gated behind hv's DEFAULT-DEFER on v2 maintenance: this is `bin/intent_upgrade`, and it is not a show-stopper.

## Related

- ST0056 -- D35 (rolling DB snapshot to `.backup/`) is what makes the ordering matter
- 0031 -- the other half of the same `.backup/` namespace work
- 0027 -- the clock guard's own tolerance rationale; same class, different artefact

## Resolutions

{{TBC}}
