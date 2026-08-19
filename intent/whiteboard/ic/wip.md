---
node: ic
name: Interface Claude
role: interface
session_id: 7c9b8dad-5c1f-49af-a9fd-9dbd287fc26d
heartbeat_at: 2026-08-19 21:10Z
status: active
focus: "**AT-02.2 AND AT-07.5 GREEN, AND `organize` NOW PRUNES THE DIRECTORIES IT EMPTIED (`dd06342a`).** hv cleared the ship gate and the estate is DEHYDRATED -- 423 removed, 0 refused, block 18 -> 14, 0 unmet. **NEXT: `st hydrate`, claimed off dc, one render arm against my `<address>` spec.**"
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**`st hydrate` -- CLAIMED OFF dc, who was redirected to the hosting gap.** One render arm; `Facade::hydrate` is built and pinned to `Mode::Apply`. **Ruled: the argument is `<address>`, and the CLI PROMOTES a bare artefact id to one** (`ST0057` -> `intent:///threads/ST0057`) because `address::parse` demands the scheme and nobody will type it. Safe because it DELEGATES -- `is_thread_id`/`is_issue_id` own the fact and `Sigil::accepts` already calls them. **A malformed argument is a USAGE error naming both forms, never "no such thread".**

## ON RESUME -- read this first

1. **THE ESTATE IS DEHYDRATED AND THE GATE IS OPEN.** hv cleared it on one word; `organize --apply` removed 423, refused 0. `intent/st` holds ST0046, ST0056, ST0057 only. **AC-03.6, AC-06.3, AC-06.4 and AC-07.5 left the DECLARED BLOCK and are NOT WITHDRAWN** -- the block is about what GATES, not what is wanted. Every one is still owed.
2. **THE ROUND TRIP IS PROVEN** (vc, `e7f00e65`): ST0001 back into the list, `organize --apply`, five files returned, all five byte-identical to git. The database regenerates the exact bytes.
3. **DO NOT REBUILD TODAY'S WORK.** AT-02.2, AT-07.5 and the prune are landed and mutation-proven.

## TODO -- LIVE ONLY

1. **`st hydrate` render arm.** Spec is in DOING and dc has accepted it whole.
2. **`Report.pruned` IS NOT RENDERED.** `render.rs` is dc's and was in flight, so the prune is reported at the API and invisible at the CLI. **A destructive act nobody can see is the defect, not the gap.** Chase it.
3. **AC-05.2: the lifecycle verbs edit the list.** They WARN on unsynced attachment bytes; `Facade::sync_uncommitted` answers exactly that. Build the warning, NOT a gate -- vc retracted the refuse clause at `9b887765`.
4. **`intent init` IS NOW HOSTING WORK, NOT GATE WORK, AND IT HAS A DECISION INSIDE IT:** a project declaring NO preconditions cannot dehydrate at all -- `organize --apply` says _0 checked of 0 declared, so nothing is proved_. Fail-closed, absence is not permission. **So a freshly initialised project is born unable to dehydrate until someone declares one.** vc says look at this next.
5. **`issues dehydrate` in 0 buckets** -- dc's, flagged, still the one crate red.

## Watch-outs

- **A MUTATION BATTERY MUST NOT RUN IN THE SHARED CHECKOUT, AND TODAY IT COULD NOT BE AVOIDED.** dc's `852/1` measured a tree my arm had broken. **A peer mid-edit gives you a tree that will not compile or fails honestly; a mutation battery gives you one that COMPILES AND LIES.** Isolation was attempted twice and failed: **it needs a consistent snapshot and there is not one** -- my work, theirs and HEAD are three trees and no two agree.
- **THE DRIVER NOW RESTORES ON SIGTERM AND CHECKS THE BASELINE COMPILES BEFORE INJECTING.** Both earned their place the same hour: a timeout once left a mutant live, and five arms once scored VOID for a peer's broken file rather than anything of mine. **A VOID from someone else's breakage is indistinguishable from a VOID from your own.**
- **`cargo test --no-fail-fast` RETURNS 101 FOR A BUILD FAILURE AND A TEST FAILURE ALIKE. The discriminator is whether ANY `test result:` line was emitted; zero lines is UNMEASURED, never green.** HEAD itself did not compile twice today and every node read it as red.
- **AN ASSERTION CAN PASS ON THE ADDRESS BEING ECHOED BACK.** `said.contains("ac")` was satisfied by the URL `/ac/AC-01.1` in the error, not by the form being named; a mutant dropping the form name survived it. **Pick a token that is not a substring of its own input.**
- **A TRUE MEASUREMENT FILED WHERE NOTHING READS IT DOES NO WORK.** I recorded _no `intentfiles::render` call in `organize.rs`_ in one row's note while another row's green sat one screen away, and it survived two validation reads. vc's general form: **two artefacts disagree and no third thing reads both.**
- **I RULED ON THE VERB THAT READS THE MANIFEST WITHOUT EVER READING THE MANIFEST.** The polarity ruling was right, and the fact that would have persuaded anyone in one sentence was twenty-six lines away, unlooked-at. **Luck wearing method's clothes.**
- **I LED WITH A CORRECTION INSTEAD OF A REPRODUCTION AND THE CORRECTION WAS WRONG.** vc's 423 was right; my model of what `organize` removes was not. **I had the confirming measurement in hand and read it as coincidence because it disagreed with a model I had just built** -- the least-tested object in the room outranking two agreeing measurements.
- **`git commit --only <path>` COMMITS THE WORKING-TREE STATE OF THAT PATH.** It defends against the INDEX, not against a peer editing the same file. dc named this after sweeping my work into their commit.
- **STILL TRUE: run the workspace not the crate; every restore path absolute; a mutant that does not compile is not a red; `stat` without `-u` prints LOCAL; the Bash tool's cwd persists between calls, so a relative `cd` is itself an anchor that can silently miss.**

## Decisions

- (2026-08-19) **AC-05.1 POLARITY, RULED AND BUILT: `intent organize` PREVIEWS, `--apply` PERFORMS.** Precedent rather than taste -- AC-03.9 already ruled the destructive direction states what it will overwrite before it costs it. v2 shipped BOTH polarities for one operation; v3 kept the acting one and dropped the preview.
- (2026-08-19) **TOP-LEVEL `intent hydrate` / `dehydrate`, NOT `st hydrate`.** hv's two sentences conflict -- he typed the spelling and required _any st **or issue**_, and `st` cannot carry issues. **The capability is the requirement; the spelling was the nearest shape he reached for.** Provisional, reversible, it is a table row.
- (2026-08-19) **hv REPLACED THE TWO-REGION DESIGN. `organize` reads the list, hydrates what is in it, dehydrates what is not, and RECOMPUTES NOTHING.** The regions existed only because the file was machine-written. **My status-rule wiring was REMOVED rather than fixed** -- I had built the thing he then took away, and the mechanism already did his a/b/c.
- (2026-08-19) **THE NEGATIVE-PIN QUESTION IS DISSOLVED, NOT FILLED.** With no function of status, _dehydrate_ is simply _remove the id from the list_. The prefix design I recommended is not needed.
