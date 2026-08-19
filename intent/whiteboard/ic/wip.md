---
node: ic
name: Interface Claude
role: interface
session_id: 7c9b8dad-5c1f-49af-a9fd-9dbd287fc26d
heartbeat_at: 2026-08-19 20:26Z
status: paused
focus: "**LOCALFOLDED FOR A COMPACT. SEVEN ROWS GREEN TODAY, ALL MUTATION-PROVEN; EVERYTHING OF MINE COMMITTED.** AT-03.14, AT-03.16, AT-03.13, AT-08.1, AT-02.4, AT-02.2 (rebuilt after hv replaced the design), AT-08.4's POST limb. **~32 mutation arms, 31 red; every survivor named and predicted, none counted as covered.** **NEXT: AC-05.2 -- the lifecycle verbs edit the list, and they WARN rather than refuse (vc corrected their own premise at `9b887765`).**"
claims: [ST0057/02, ST0057/05, ST0057/07, ST0057/08, ST0056/03]
---

# Interface Claude (ic)

## DOING

**NOTHING IN FLIGHT. Everything of mine is committed.** The only dirt in the tree is `intent/events.jsonl` (written by verbs, shared) and peers' files.

## ON RESUME -- read this first

1. **READ AC-05.2 AT HEAD BEFORE PLANNING IT.** It changed twice in the last hour. **The closing verbs do NOT delete files** -- vc traced their own premise and retracted it at `9b887765`: `organize.rs:695` is the ONLY line in the tool that removes an estate file. So `st done`/`st cancel` **WARN, naming the paths**, and are not a second authority over deletion. **A second gate would be worse than a divergent copy: it answers a different question than the realiser's gate** -- _are there unsynced bytes_ versus _can the store reproduce these bytes_ -- so the two would disagree BY CONSTRUCTION rather than by drift.
2. **`.intentfiles` IS DURABLE STATE, NOT "AUTHORED".** Commands write it routinely -- `st new` adds, `st done` removes, `hydrate`/`dehydrate` do it directly. **What no longer happens is RECOMPUTATION.** vc corrected me on that word mid-build; do not regress to "nothing writes this file".
3. **DO NOT REBUILD ANY OF TODAY'S WORK.** Seven rows are green and committed. The design changed under two of them and they were REBUILT, not patched.
4. **OPEN AND MINE, unruled while hv is out:** the `st edit` fork (declared, unimplemented, and hv's generated-view ruling makes its DEFAULT argument a thing it must refuse). Flagged to vc; not ruled.
5. **OPEN AND NOT MINE TO CLOSE:** the 6-of-43 gate-visibility ask in `hv/inbox.ic.md`. hv answered exactly the question in front of him and this was in the same inbox. **A ruling arriving as "resolved" is easy to read as clearing the queue it sat in.**

## TODO -- LIVE ONLY

1. **AC-05.2: the lifecycle verbs edit the list.** `st new` adds; `st new --dehydrate` does not; `st done`/`st cancel` remove; `st reopen`/`st reinstate` add back; `st done --keep` closes without removing. **WARN on unsynced attachment bytes; `Facade::sync_uncommitted` (cc's, built today) answers exactly that question.**
2. **`hydrate` DISPATCH ROW IS dc's AND THEY ASKED ME ONE QUESTION I HAVE NOT ANSWERED: `<address>` or `<id>` for the argument.** The address grammar is mine. **Answer it.**
3. **AT-08.4's row moves green when vc verifies the POST limb** -- `Facade::post` is in at `0543f64b`, 8 tests, 5 arms all red.
4. **`issues dehydrate` is in 0 buckets** -- the one workspace red at my last run. dc's row, ahead of its accounting.

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
