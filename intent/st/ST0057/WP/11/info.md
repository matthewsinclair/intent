---
wp_id: WP-11
title: Default disposition realises WIP threads only: organize --default writes .intentfiles; init, migration and upgrade share the function
scope: S
status: Not Started
---

# WP-11: Default disposition realises WIP threads only: organize --default writes .intentfiles; init, migration and upgrade share the function

## Objective

**A project realises WIP steel threads only, by default and by declaration, and `intent organize --default [--force]` is the verb that writes the default.** `organise` is already an accepted spelling (a spine alias, accepted and never shown) and stays one.

**THIS COVER SAID "OPEN" UNTIL 2026-08-27 AND hv HAD ALREADY OVERRULED IT.** The superseded wording, kept visible rather than deleted because a corrected sentence reads exactly like one that was never wrong: *"hv, 2026-08-26, revising the WIP-only wording of an hour earlier: 'default means that the only things in .intentfiles are OPEN'. OPEN = every status except Completed and Cancelled (WIP, Triage, Not Started, On Hold)."*

**THE RULING THAT STANDS, TRANSCRIBED FROM ITS SOURCE RATHER THAN SUMMARISED.** `intent/whiteboard/hv/wip.md`, entry dated 2026-08-26 19:48Z, carrying its own provenance caveat verbatim -- *"recorded by vc: hv FIRST-HAND IN lamplight-vc's SESSION, relayed verbatim by lamplight-vc; hv's own stamp not read"*. hv's words on seeing a 57-thread realised set: **"Now it has NOT STARTED STs!??!"** and **"It should ONLY HAVE WIP STs!!!!!"**

So `.intentfiles`'s rule *"OPEN means every status except Completed and Cancelled"* was WRONG -- **a definition by exclusion, which acquires members by accident**: it swept in planned threads nobody is working on. `--default` declares exactly the threads whose status is WIP; Not Started, Triage, Hold, Completed and Cancelled live in the store, in full, and `intent st hydrate <ID>` realises any of them on demand.

## The verb

- `intent organize --default` -- `.intentfiles` ABSENT: write it from status (header + one `STEELTHREAD:<ID>` line per open thread) and exit 0. PRESENT: change nothing, say so (`present, declares N; --force to regenerate`), exit 0.
- `intent organize --default --force` -- PRESENT: regenerate from status after a y/N read from the tty (`hydrate` / `dehydrate` customisations are lost, which is what the confirmation is for); no tty, no `--force` write.
- `--default` on its own writes the DECLARATION only and never removes a file: removal stays behind `organize --apply` and the dehydration preconditions, so the next preview reports every undeclared realised thread as `to remove (blocked)` and nothing moves. Declaring and dehydrating are two steps by design.
- **`--default --force` ANSWERED `y` ON A TTY IS THE EXCEPTION, AND THIS COVER USED TO DENY IT EXISTED.** hv, 2026-08-26, first-hand, quoted at AC-11.6: `--default` never removes a file *"unless it is used with --force, which does remove files, after a confirm"*. That arm applies the regenerated declaration in the same run, dehydrating each undeclared realised thread ONLY where every declared precondition holds for it, and removing not one file of a thread whose preconditions are unmet. AC-11.6 is the contract; **the confirm text is owed a rewrite alongside it, because it currently promises "it removes no files", which under `--force` is false.**

## One function, three callers

The default declaration from status is ONE function in `intentsvcs::intentfiles`. Callers: the verb; `intent init` (a fresh project gets the file present and empty -- "keep nothing" until a thread exists); the migration (hop 2) and `intent upgrade` when the file is ABSENT. `upgrade` never touches a present file: a write there is a change to state, never a regeneration (the file's own header). That is how every migrated project gets its declaration at its next `intent upgrade` -- the 3.0.1 re-stamp -- without a hand sweep.

## Why

Every migrated project is fully realised (Laksa: 110 of 110) because none has an `intent/.intentfiles` and the contract says ABSENT means everything stays; `migrate.rs:474-488` consults the file and nothing writes it. Intent's own tree is 8 of 63 because hv wrote the file by hand.

## Not here

- Issues: hv's default is "open STs and ISSUES", and the grammar has exactly one sigil; `ISSUE` was retired 2026-08-20 because a declared issue realised nothing. WP-12 gives issues a realised form first; until then `--default` declares open threads only.
- Dehydrating the fleet's realised trees: waits on one `.canon` emitter (attachments in canon), the preconditions moving into the tool (`organize.rs:261`), and `st dehydrate` (ST0061); then one `organize --apply` per project.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0057/acceptance.md`, under the `WP-11` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0057.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `the thread canon`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
