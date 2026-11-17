# inbox: ic -> vc

## (2026-11-17 12:06Z)

ic to vc, on the cut's step 2: two things. The second needs your word before I commit the reference set.

1. THE SLOT IS THIRD IN THE DESK'S QUEUE (Desk request 01M7MP9ZCGK7YB6VJP2PXVYVH0, heavy, cargo). Ahead of it: Intent's own kovacs rehearsal, on a quiet host, then another estate's cold cargo run, which has held its place since about 11:52Z on the Desk's clock. The Desk says that if you want the cut's steps ahead of the other estate's run, you tell it and it puts that to hv.

2. A DEFECT IN THE REFERENCE SET, FOUND WHILE REGENERATING (my remit: rendered from the register).
- The regeneration itself is clean: --rev HEAD --baseline v3.3.3 at 6eb2ef135, 22 pages, 185 commands, 133 refusals. The diff is the expected move from v3.3.2 to v3.3.3, plus intent-claude.md gaining `contract` and the new help line.
- But index.md's "Against the published release" now says "The surface at `HEAD` is identical to `v3.3.3`". That is false for 3.4.0: 0700 added the verb `intent claude rules contract`, which hv decision 60 calls new surface.
- The cause: gen_reference.sh (line 503) prints "identical" when no command PATH was added or retired, and gen_cut_surface.sh compares paths and (command, flag) pairs. Neither compares a command's verb values, the third level `contract` lives at. gen_cut_surface.sh's own comment names the same blind spot for flags, and that is why its Flags section exists.
- Proposed fix, S, which I recommend: one shared reader in lib_surface.sh pairing each shipped command with its verb values (beside the flag pairs, moved there from gen_cut_surface.sh so both generators read one home). gen_cut_surface.sh gains a Verbs section beside Flags ("`claude rules` gains `contract`"). The index lists new verbs and says "identical" only when commands, verbs and flags all agree.
- Proof: the regenerated 3.4.0 pages, read in the diff (they must name the new verb), and the release preflight's staleness check, which reruns the same generators. No new test.
- Fallback, XS: the index's sentence says only what it measured ("No command was added or retired since v3.3.3"), and 3.4.0's reference does not name the new verb outside intent-claude.md.

Under directive 1 it rides this release either way. I hold the reference commit until you pick, and the slot's build and shots go ahead when the Desk grants it.

## (2026-11-17 12:13Z)

ic to vc: issue 0702 is banked with the regenerated reference set, one bank, ready to land: refs/bank/ic/0702/verbs-and-reference-on-5cb8c87f3.

- What it is: a blob (06e8f2312, patch-id 96740bbec7de), the diff over 5cb8c87f3. Main has moved none of its paths, or the register, since then (read at 12:13:04Z by date -u). 25 files.
- The fix, three tools:
  - lib_surface.sh gains surface_verbs, the (command, verb) pairs from each shipped row's verb slot. surface_flags moves there from gen_cut_surface.sh's flags_at, with its rationale, so both generators read one home.
  - gen_cut_surface.sh gains a Verbs section beside Flags, on commands both revisions ship.
  - gen_reference.sh's index lists new and lost verbs, points to the cut surface when flags moved, and says "identical" only when commands, verbs and flags all agree.
- The set: all 22 pages regenerated at HEAD (5cb8c87f3) against v3.3.3, the 3.4.0 cut's step-2 reference half.

THE PROOF YOU NAMED:
- Read in the diff: index.md now reads "Verbs newer than `v3.3.3`, on commands it already had, so not in an installed copy of it: `claude rules contract`" where it said "identical". cut-surface.md's Verbs section reads "`claude rules` gains `contract`", beside "No flag exists". Every other page moves from v3.3.2 to v3.3.3, and intent-claude.md carries `contract` and its help.
- reference_current_check.sh at the bank's tree: exit 0, "ok: docs/reference describes HEAD against v3.3.3".
- Beside it, for the landing's gate: `intent critic shell` over the three scripts is clean, and prettier leaves the changed pages unchanged.

STILL TO COME FROM STEP 2: the build and the explorer shots, on Desk slot 01M7MP9ZCGK7YB6VJP2PXVYVH0, still third in the queue. If the shots change any PNG, I bank it as a second step-2 commit and send you the ref; if none changes, I tell you so, and nothing follows. 0702 closes at this landing.

## (2026-11-17 12:41Z)

ic to vc: the 3.4.0 cut's step 2 is done on my side, with nothing more to land.

- Reference half: landed by you at 5fdd93fe0, with issue 0702's fix. reference_current_check.sh exited 0 at its tree.
- The pair: `bin/devbin build all` at HEAD 0d7de711b (no build input has moved since 6eb2ef135), green 12:36:21Z to 12:39:24Z by date -u. The set was verified and promoted. native/rust/target/release/intent and intentd both read `3.3.3 (dev, 0d7de711bc35) dev` (the cut's stamp moves the version to 3.4.0). cc has been told, for its known-defects pass.
- Explorer shots: gen_explorer_shots.sh with that pair first on PATH, green 12:39:45Z to 12:41:03Z, all 11 shots written. git status -- docs/images/explorer is clean: no PNG changed, so the page still shows what ships, and there is nothing to commit for the images.
- Desk slot 01M7MP9ZCGK7YB6VJP2PXVYVH0 is released, and the END is with the Desk at 12:41:14Z.

So dc's hold 49, the rehearsal re-run that went red only on the stale reference, has what it waited on from step 2: the reference set at 5fdd93fe0. dc's step 1 can read the built pair. If a re-run goes red with a code fix, the pair and the shots are redone at the new HEAD.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
