---
id: "0031"
title: intent upgrade --backup-dir basenames into .backup/, so --backup-dir db lands a rollback artefact inside the D35 snapshot namespace
date: 2026-08-15
reporter: matts
status: OPEN
severity: low
---

# 0031: intent upgrade --backup-dir basenames into .backup/, so --backup-dir db lands a rollback artefact inside the D35 snapshot namespace

## Tags

backup, upgrade, d35, namespace

## Summary

`intent upgrade --backup-dir DIR` takes the basename of its argument and joins it to `.backup/` (`bin/intent_upgrade:119-121`), so the caller controls a directory name directly under `.backup/`. D35 reserves `.backup/db/` for rolling DB snapshots with schedule-based retention. `intent upgrade --backup-dir db` therefore writes an upgrade rollback artefact into the namespace the snapshot sweep ages out.

This is the one collision that survives the namespace layout, because the layout confines _mechanisms_ to directories and this flag lets a user put one mechanism's artefact inside another's directory.

## Reproduction

```
intent upgrade --backup-dir db
# writes .backup/db/, which D35's retention owns
```

Also reachable by accident with any argument whose basename collides -- `--backup-dir /somewhere/else/db` basenames to `db` too, which is easier to type than it looks.

## Root Cause

The flag was designed when `.backup/` had exactly one occupant, so an unconstrained name was harmless. D35 makes `.backup/` a namespaced directory with more than one retention policy in it; the flag was never revisited against that.

## Impact

Low. It needs a user to pass a specific value, and the consequence is that one rollback artefact is aged out on the snapshot schedule instead of being kept until the upgrade is trusted. But it is a backup deleting a backup, which is the exact failure the namespacing exists to prevent, so it should not be left open on the grounds that it is unlikely.

## Proposed Fix

Reserve the namespace names under `.backup/` -- currently `db` and `upgrade` -- and refuse rather than accept: `error "--backup-dir: '<n>' is a reserved backup namespace"`. Refusing beats silently rewriting the name, because a silently relocated backup is a backup the operator cannot find when they need it.

The reserved list should be derived from the namespaces the tool itself writes rather than hand-maintained, so a third namespace cannot be forgotten -- the same enumerate-do-not-hardcode rule that fixed `pr-checks.yml`.

Gated behind hv's DEFAULT-DEFER on v2 maintenance: this is `bin/intent_upgrade`, and it is not a show-stopper.

## Related

- ST0056 -- D35 defines the `.backup/db/` namespace this collides with
- 0030 -- the other half of the same `.backup/` namespace work

## Resolutions

{{TBC}}
