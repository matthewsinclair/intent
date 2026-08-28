---
node: ic
name: Interface Claude
role: interface
session_id: c3439256-4fb7-4499-8444-95d1f0d52bd7
heartbeat_at: 2026-08-28 16:30Z
status: active
focus: "PRIME SWEEP DELIVERED (2ab7c2e0, catalogue section 8): five findings, three MISGUIDED. THE ONE THAT MOVES OTHER PEOPLE: templates reach projects by TWO paths -- _CLAUDE.md is include_str!'d so my fix is SOURCE-ONLY until cc's rebuild, the prime template is read from disk so that one is LIVE. 'Eleven sites landed' is true of the source and per-file for delivery. NOW ON: the retirement-mechanics DRAFT (in-start + in-next + _wip.md), draft only, to vc before anything is deleted. THEN the AGENTS.md costed proposal."
claims: [ST0065, ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0057/11, ST0057/14, ST0061]
---

# Interface Claude (ic)

## DOING

**RETIREMENT-MECHANICS DRAFT -- STARTING NOW.** `in-start`, `in-next`, `_wip.md` template. Draft only; it goes to vc before anything is deleted. Installed copies exist fleet-wide, so this is a LIFECYCLE operation, not a delete. My draft lands FIRST and cc's `init.rs` disposition for `_wip.md` follows from it.

**ST0065 REWRITE LEG -- AND THE CLAIM NEEDED CORRECTING.** The 11 doc/skill MODULES.md sites, `in-finish` step 4 and `usage-rules`' two claims are all landed **in source** and re-grep-verified. **They are NOT all delivered.** `lib/templates/llm/_CLAUDE.md` is `include_str!`'d into the binary -- the shipped binary carries the OLD bytes, zero occurrences of my new wording, and a fresh `intent init` emitted the old row in front of me. That fix rides cc's rebuild batch. **So the rewrite leg needs THREE delivery actions, not one rebuild** -- a binary rebuild for the embedded templates, nothing for the read-from-disk one, and a skills sync for the four skills. `lib/templates/prime/operational-knowledge.md` is NOT embedded, so that one is live now. **Skills are a THIRD path, and I measured it rather than asserting it:** skill bodies get zero hits in the binary (control: an embedded string gets one), so they are read from disk and installed into `~/.claude/skills/` by `intent claude skills sync`. **All four skills I edited DIVERGE from their installed copies; two I did not edit are IDENTICAL** -- so the comparison discriminates and the divergence is exactly my edits. **This session is running the PRE-EDIT skills.** I have not synced: that writes outside the repo and the project is deliberately holding delivery for cc's batch, so it is vc's to sequence, not mine to take.

**PRIME SWEEP DELIVERED** at catalogue section 8 (`2ab7c2e0`). Worst finding: the payload teaches two flags the shipped binary refuses, and the bats test covering one of them drives the **v2** binary where it works -- green, and structurally unable to see v3's refusal.

**Both catalogues are the ruling record and stay live** -- `intent/st/ST0065/_catalogue-wp01.md` and `_catalogue-wp02.md`. They carry their own instrument errors inside them, which is why they can be trusted about anything else.

## TODO

- **RETIREMENT MECHANICS -- DRAFT ONLY, then to vc before anything is deleted.** `in-start`, `in-next`, `_wip.md` template. **Installed copies exist fleet-wide, so a canon-skill retirement is a LIFECYCLE operation, not a delete.** My draft lands FIRST and cc's `init.rs` disposition for `_wip.md` follows from it (vc holds cc off).
- **AGENTS.md COSTED PROPOSAL -- design-first, nothing edited, to vc then hv.** hv's direction: the injected set carries everything an agent must have, AGENTS.md becomes the honest cross-tool mirror. **Four-rules inversion is in scope.** The load-bearing question is that a mirror needs a generator or it drifts. Prior art vc named: `intent agents sync` IS the generator, dc built `agents_sync_parity` TODAY (structural-invariant, property-not-roster, two-sided control), and `parity.md`'s ratified-deviations list is the working model for a mirror contract that names its permitted divergences. **Cost against that shape.**
- **DEFERRED DELIBERATELY, not omitted:** `in-start:23` is a MODULES.md site inside a skill hv retired -- it resolves with the retirement, and I will not edit a line in a file I am drafting the deletion of.

## Watch-outs

**THE DAY'S CLASS, FIVE INSTANCES, AND IT IS NOT THE ONE I FIRST NAMED: I LET A MEASUREMENT STAND AS A FINDING WITHOUT READING WHAT IT COUNTED.**

- **The count moved four times and BOTH DIRECTIONS.** First three were UNDERCOUNTS from a pattern fitted to the surface form I had already seen (path-qualified, when most instances were bare). The fourth was an OVERCOUNT from a pattern broad enough to catch everything, **whose hits I never opened** -- it swept up a correctly-guarded hook, a prose description, and a file-map row. **So "widen the pattern" is the WRONG lesson and is exactly what produced instance four. The grep was the finding instead of the start of one.**
- **A CONTROL IS ONLY A CONTROL ONCE IT HAS FIRED.** My staleness sweep returned a clean zero across seven files. Run against a known-positive it ALSO returned zero -- I had rewritten the same path-qualified pattern minutes after correcting it. **A green from an instrument that has not been seen to fire is not evidence.**
- **ONE POPULATION IS NOT THE CORPUS, AND THIS ONE REACHED hv BEFORE I CAUGHT IT.** I catalogued `usage-rules`' `.git/hooks/pre-commit` as FALSE. It is TRUE for a default project -- the canon installer writes exactly there. It is false only where `core.hooksPath` is set, **which is this repo, the self-hosted one I measured on.** hv adopted the verdict in that form. **Measure the thing the document ships to, not the thing you are standing in.**
- **A COMMIT MESSAGE CAN ASSERT A CHANGE THE COMMIT DID NOT MAKE.** A patch script asserted on a table row the markdown formatter had realigned; the assert fired, the commit went ahead with six files instead of seven, and **the message described the intent rather than the tree.** Same defect as the whole thread: a document asserting something the mechanism did not do. **Read what landed, not what you told it to land.**

**PEER FINDINGS FIND YOURS.** dc's 0113 sweep retired my hypothesis EMPTY -- correct outcome, and their single false positive (a hit on prose DESCRIBING a guard) is what made me re-read my own twenty and find three that were never instructions. **A peer's null result carried a discriminator I needed.**

**A RETIREMENT RETIRES ITS INSTRUCTIONS** (hv's ruling, adopted as the one-decision principle). Three retirements had each been fixed on one side only: `init` stopped writing MODULES.md, the 08-24 fold collapsed the restart files, the gate took over session entry. **The artefact was repaired and the instruction that regenerates it kept running every time.**

**AN ISSUE STORE WITH NO WRITE PATH CANNOT HOLD A CORRECTED FINDING.** 0122 stands mis-scoped (it names a guarded hook) because `issues` has no edit verb. vc ruled it stays unpatched as live evidence; **the catalogue carries the work order and 0122 cannot point back at it** -- a one-directional pointer, structurally the same as the AGENTS.md loop. When the edit verb is built, **0122's correction is its first drive.**

## Decisions

- **(hv, 2026-08-28, via vc) ALL ST0065 VERDICTS ADOPTED** under the one-decision principle. `in-next` RETIRES (hv took the call I declined to make). `DECISION_TREE` gates behind `intent lang init elixir`. **AGENTS.md is DESIGN-FIRST** -- costed proposal before any edit.
- **(vc, 2026-08-28) FILE OWNERSHIP, each file touched once:** cc takes ALL of `_DECISION_TREE.md` (its three MODULES lines ride the relocation) and both archetype templates and 0122's CLI strings. **My 17 resolved as 11 mine, 1 deferred, 5 cc's.**
- **(ic, 2026-08-28) A DEFERRAL IS RECORDED AS DELIBERATE OR IT READS AS AN OMISSION** -- `in-start:23` is on the record as deferred into the retirement, not missed.
- **(all nodes, 2026-08-28) EVERY FOLD ARCHIVE IS `wip-fold-HHMMZ.md`, never plain `wip.md`.** My near-miss was real on two boards independently -- vc's morning fold wrote plain `wip.md` too. Existing files stay; history is append-only and nothing is renamed.
