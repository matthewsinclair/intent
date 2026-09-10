# inbox: cc -> hv

## (2026-09-03 17:21Z) FOLDED ON YOUR INSTRUCTION -- EVERYTHING BELOW IS A LIVE DECISION AND NOTHING ELSE

**19,134 bytes of handled entries archived VERBATIM and cmp-verified to `cc/.history/20260903/hv-inbox-cc-prefold-1718Z.md` (sha `3f307d11`).** Nothing is lost; what is gone from here is discharged, and it went because you should not have to read nine dead items to find seven live ones.

**DISCHARGED AND REMOVED, so you do not go looking:** `sync` skipping untracked bytes (the code took a third option I told you did not exist); the `~/.intent/home` dead-worktree blocker (resolves, `state: OK`); `autotests = false` (all three crates carry it); the test-target consolidation (landed); `Node`'s model and the two FYI entries. **And four items I escalated as YOURS have since been ruled by vc under the pen, so they are my work now rather than your decisions:** the migrator-commit contradiction, the `0086`/`0063`/`0095`/`0096` disposition, `0192`'s in-or-out, and `0205`'s vendored block. **`0214` is CLOSED**, which kills the cut decision I asked you for on it.

**SEVEN THINGS NEED A WORD FROM YOU. Each is a question you can answer with one.**

**[1] Should `at green` run the L3 arm?** (i) warn, do not refuse (ii) refuse (iii) leave as is. **REC (i).** Refusing breaks the legitimate write-then-cite order -- greening a row before the citing test exists -- **which is the order that produced this morning's gate outage**, so refusing would have converted a self-inflicted stoppage into a permanent one.

**[2] `INTENT_BIN` flip and the re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip (iii) neither this cut. **REC (i).** The estate's default `INTENT_BIN` is `bin/intent`, **the v2 SHELL SCRIPT**, and it is THREE binaries not two (`test_helper.bash:21` v2 shell, `run_v2_suite.bash:55` v3 debug, `~/.local/bin/intent` v3 release). Re-baselining first pays the wall time twice.

**[3] `burn.sh` re-run, or accept `AC-06.1`'s coverage half red into 3.0.1?** (i) run it (ii) accept red and say so on the row (iii) descope the half. **REC (i), and it is yours because full-suite runs are yours** -- a double full-estate sweep that hung 3.5h once. `coverage_map.sh` refuses to publish and is right to: the burn TSV no longer covers the estate.

**[4] `WP-08` -- endorse vc's hold or override it?** vc ruled HELD under the pen, on a falsifiable condition: **conformance coverage exists for the daemon, or the gap is explicitly accepted on the record by you.** `wp done` is an XS away and I am not taking it. **REC endorse.** I built one piece of that coverage today and I am explicitly not arguing it discharges my own blocker.

**[5] `config` bare resolves to `target: undefined`. What should it do?** (i) print the resolved config (ii) print help (iii) refuse with a remedy. **REC (i) -- FLAGGED: this rests on my reading of the surface, NOT on a census of what other bare noun verbs do.** If that consistency claim is load-bearing for you, it wants driving first.

**[6] `agents` bare is recorded `pending-hv`. Same question, same options.** **REC: whatever you rule for [5], for the same reason** -- two bare nouns answering differently is the defect, not either answer.

**[7] Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** (i) clear the class then gate (ii) stay report-only (iii) gate now. **REC (i)** -- the class is one-token path repairs, since five of six targets I once recorded as absent exist under another name. **PREMISE MOVED AND YOU SHOULD HAVE IT BEFORE RULING: the account lived in `0214`, and `0214` is now CLOSED.** The lint half is dc's.

**AND ONE THING THAT IS NOT A DECISION, RECORDED BECAUSE IT WOULD OTHERWISE DIE.** The daemon-lock race I escalated as having no issue at all **may already be filed as `0210`** (shared-thread canon commits deadlocking under intentd auto-ingest). Adjacent ground. **I have not compared them.** If it is a duplicate my item dies; if not, it still has no record.

## (2026-09-08 20:11Z)

**ROUTED ON YOUR INSTRUCTION. MY INBOX FROM YOU IS EMPTY -- there was nothing of yours to clear, so this is the other direction: everything of mine that needs a word from you, in one place.**

**READ THIS BEFORE THE 2026-09-03 ENTRY ABOVE. FOUR OF ITS SEVEN ARE DEAD AND I RE-DROVE ALL SEVEN JUST NOW RATHER THAN RESTATING THEM.** A five-day-old decision list is not a backlog, it is a mix, and working it top-down would have spent your time on settled items.

**DISCHARGED -- DO NOT SPEND A WORD ON THESE:**

- **[1] `at green` L3 arm.** Settled by `0270`, CLOSED 2026-09-05. `VerdictCitesAbsentFile` now REFUSES and its remedy names the consequence (the finding refuses every commit in the repo, not just yours). **Ruled (ii); my REC was (i) and was overridden. Recording that rather than re-arguing it.**
- **[4] `WP-08` -- endorse vc's hold or override?** `intent wp show ST0056/08` reads **Done**. Gone.
- **[5] `config` bare resolves to `target: undefined`.** No longer true. `intent config` -> **rc=2**, `error: config is a known command that is not implemented yet`, with a remedy. That is option (iii), implemented.
- **[6] `agents` bare, same question.** Premise gone. `intent agents` -> **rc=0** and prints help. The two bare nouns still answer differently, and **the difference now tracks implementation status rather than nothing** -- `config` genuinely is not implemented, `agents` is. That is a defensible divergence, so there is no longer a defect to rule on.

**STILL LIVE, AND EACH IS ONE WORD:**

- **[2] `INTENT_BIN` flip and re-baseline -- which order?** (i) flip then re-baseline (ii) re-baseline then flip. **REC (i), AND IT IS NOW DERIVED RATHER THAN PREFERRED.** ic supplied the mechanism today: `burn.sh` measures coverage as the DELTA between a default-`INTENT_BIN` run and an `INTENT_BIN=/usr/bin/false` run. Under (ii) the baseline arm has no defined subject and the number it produces cannot be used. **There is one answer, not a favourite.**
- **[3] `burn.sh` re-run, or accept `AC-06.1`'s coverage half red?** (i) run (ii) accept red on the row (iii) descope. **REC (i), and it is yours because full-suite runs are.** **CHEAPER THAN THIS LIST HAS BEEN SAYING:** ic corrected their own earlier answer to me -- `burn.sh` exists, works, and carries the per-file timeout it earned off a 3.5h hang. It is a RUN, not a build. What is stale is the BASELINE: `coverage_map.sh` refuses right now over 98 rows against 116 files, 19 never measured, and refusing is correct. **[2] is a precondition of this, not a sibling.**
- **[7] Flip `RUSTDOCFLAGS="-D rustdoc::broken_intra_doc_links"` to a gate?** **REC (i)'s PRECONDITION IS NOW MET -- I drove it: the workspace has ZERO broken intra-doc links, and nothing gates it anywhere.** So the question collapses to: gate it now, or leave it report-only. **REC: gate it.** The class is clear, so a gate costs nothing today and stops it reopening.

**NEW SINCE THAT ENTRY -- TODAY'S, AND THE FIRST TWO ARE THE ONES THAT BLOCK OTHER PEOPLE:**

- **`0217` IS THE CHEAPEST THING ON THIS LIST.** It is vc's row-shape call, and it converts `AC-06.13` from red-on-a-ratchet into a closeable row. My instrument is written and driven; nothing else is needed.
- **DISPOSITIONS ON FIVE OPEN ISSUES, and two of them are DELIVERED work waiting on a word, not unfinished work:** `0205` and `0272` are delivered; `0268` and `0271` are filings; **`0175`'s symptom is FIXED and I re-drove it tonight** (`intent agents` -> rc=0 with help, which is what it was filed against) **and the issue is still OPEN. REC on `0175`: close as fixed.**
- **THE VERSION SCOPE FOR THE NEXT CUT -- 3.0.1 OR LARGER.** vc measured that all eighteen command paths added since `v3.0.0` declare `disposition: new-surface`; not one is a pre-existing surface finally declared. **NO RECOMMENDATION FROM EITHER OF US -- it is a release-scope call. HONEST LIMIT: neither of us built the v3.0.0 binary and drove it, so the argument is structural rather than observational.** The mechanism no longer blocks either answer.
- **THE CHANGELOG SAYS `- unreleased`; THE RELEASE HANDLER EXPECTS `- in progress`.** Driven: `int build release --dry-run --patch` stops, and only `--allow-stale-date` gets past it, on a message that misdescribes the cause. (i) CHANGELOG adopts `in progress` (ii) handler learns `unreleased`. **REC (ii).**
- **`0285` (marker provenance) and `0287` (the guard blind to generated embeds) ARE FILED AND UNRULED.** Both change shipped or gate behaviour. `0285`'s fix routes to me on your word; **on `0287`, note vc's constraint before assigning it -- the guard lives at `intent/st/ST0056/parity/tools/`, which a standing ruling says does not outlive ST0056, so the home is a decision that precedes the fix.**

**NOT A DECISION, CARRIED BECAUSE IT WOULD OTHERWISE DIE -- AND IT IS UNCHANGED SINCE 2026-09-03, WHICH IS ITSELF THE POINT.** My unfiled daemon-lock race may already be `0210`. `290261fc` fixed ONE lock race (`stop` returning while the daemon still held its advisory lock). **Whether that is the same thing as `0210`, or as the race I never filed, is STILL NOT COMPARED.** Nobody has picked it up in five days.

**ONE LOOSE END THAT IS NOBODY'S.** `bin/.devbin/manifest.sha256` is modified in the tree -- a devbin re-vendor, `source_commit 086fce66 -> 65895b59`. It was dirty before vc's session and before mine; vc declined it, ic checked the bytes and declined it, I did not write it. **Three nodes have looked and none of us owns it, and it survived the global fold uncommitted.** Flagging because the next fold may sweep it in without asking whose it is.

## (2026-09-10 17:20Z)

**CI HAS BEEN RED ON EVERY RUN SINCE 2026-08-29 AND THE DECISION IS NOT ABOUT THE FAILURES.** Both workflows, twelve days, including this morning's 255-commit push. Two decisions below; the second is small. The first is not about CI at all.

### THE CLAIM, WHICH IS ABOUT WHAT OUR EVIDENCE HAS BEEN WORTH

`view_single_writer.rs` refuses to report a verdict it did not measure: when prettier is absent it PANICS rather than skipping, because *a skip here is a green that means nothing, on a criterion about instruments that pass without measuring*. It was right, on every push, for twelve days. Neither GitHub runner ships prettier. **This machine has it from Homebrew, so those five arms measured HERE AND ONLY HERE.**

**EVERY GREEN ANY NODE REPORTED THIS FORTNIGHT IS A macOS-WITH-HOMEBREW GREEN.** My `538/538` an hour before I found this INCLUDED the arms that fail in CI. dc's `app-test REAL_RC=0`, ic's `8/8`, my suite -- none of them says anything about Linux, and the estate manufactures the gap by construction: `bash` 3.2.57 beside Homebrew 5.3.15, shellcheck on ubuntu and not macOS, prettier on neither runner and on every dev box.

That is not a CI defect. It is a statement about the evidence base under every claim of done on this board, and it is bigger than the three causes under it.

### THE DECISION: CI DISPATCHES A CORRECT VERDICT INTO A QUEUE WITH NO READER

On 2026-08-29 you ruled the fmt and lint arms out of `prepush`. **THAT RULING IS SOUND AND I AM NOT ASKING YOU TO REVISIT IT.** They were a second home; CI runs the identical commands on both platforms on every push; 925 commits between pushes turns an early warning into a wall at the worst possible moment. `cmd/prepush` records the reasoning and it holds.

It distinguishes itself from the 2026-08-20 defect on exactly one point, in its own words: *a check nothing dispatches. One that something else dispatches is a different thing, and the distinction is the whole ruling.*

**THE DISPATCHER EXISTS. NOBODY READS ITS VERDICT.** Twelve days, ten consecutive reds, four nodes reasoning about a green estate. **This is the whiteboard protocol's own finding one level over: a write surface with no named reader is a queue, not a channel, and nothing observable distinguishes the two.** The protocol fixed that for `hv/inbox.*` by naming vc as the obliged reader. Nobody applied it to the dispatcher we already had.

The ruling also names its own escape condition -- *a local arm earns its keep again once a red `upstream/main` starts costing something, which it will once 3.0.0 publishes for real*. We are pre-tag with a red main, so that condition is arriving rather than arrived.

**THE ASK: name an obliged reader of CI's verdict.** (i) a named node checks `gh run list` at pickup and routes a red, the way vc is obliged for your inbox -- **REC**, it is the fix that matches the diagnosis and costs one command per boot; (ii) re-add the local arm, which re-answers the cadence question you already settled and reinstates the wall; (iii) accept red main until the tag. **(i) and (ii) are not exclusive and (i) does not need (ii).**

### THE SECOND DECISION: DOES THIS ESTATE CHASE A FLOATING LINT?

19 clippy errors. **I fixed the three that are real on any toolchain** (`d28510c7`) and deliberately did not touch the other 16. Those are `collapsible_if`, fired by let-chain stabilisation in rustc 1.98.1 -- exactly what `bin/.devbin/config.yaml:286` predicts: *`fmt --check` and `clippy -D warnings` float on @stable, so CI can go red with zero code change when a new stable lands*, with no `rust-toolchain.toml` on the deliberate ground that a pin which does not bind is worse than none.

**THE FLOAT DOES NOT EXPLAIN THE ONSET AND I NEARLY FILED IT AS THE CAUSE.** 1.98.1 is dated 2026-09-01; the red starts 2026-08-29. It is the second layer, not the first.

(i) fix all 16 -- they are mechanical, and the next stable brings more; (ii) `#![allow]` at the workspace root with a written reason and a review date; (iii) pin the toolchain and accept that the pin binds CI only. **NO RECOMMENDATION -- this is a policy about how much this estate pays to track a moving lint set, and greening it on one node's read is how a gate quietly becomes decoration.**

### WHAT IS ALREADY DONE, SO THIS IS NOT A LIST OF ASKS

`45434c39` installs prettier on both runners. `d28510c7` clears the three real lints. Neither is pushed -- vc holds the push and is putting it to you. **Neither makes CI green**: the 16 stand, and `Intent Tests` is red from 2026-08-30 on a third, unrelated cause routed to dc -- `125f601d8` deleted six v2 plugin scripts while `bin/intent` still dispatches to seven, so **five of v2's seven `claude` subcommands are broken in this tree.** That last one has a consequence worth your attention: **the v2 side of any `INTENT_BIN`-flip comparison is itself broken**, which makes `AC-06.1`'s re-baseline suspect until it is fixed.
