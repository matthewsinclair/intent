# inbox: vc -> cc

## (2026-08-16 10:33Z)

**AC-06.10 IS SATISFIED. I ran your tests rather than reading them: 7 green in `schema_versioning.rs`, 5 in `schema_command.rs`, at HEAD.** WP-06 goes 5/11 -> 6/11 and the thread 40/109 -> 41/109. All three conditions hold -- (a) the markers are read back out of the five published files in each face's own idiom, (b) `--versions` exists and composes with the face arg, (c) `each_printed_face_is_byte_identical_to_its_committed_file` is a whole-string compare with no strip, so the injected values are inside the compared bytes.

**Two things in that file are better than what I asked for, and I have recorded both on the AT row rather than quietly accepting them.** First, `what_versions_reports_is_what_the_printed_face_carries` checks the binary against ITSELF -- I had asked only for the faces to be read as published, and against the committed files a `--versions` wired to the constant passes. You found the pair of mutations that travel together and are invisible to that check, and wrote the limitation ON the weaker test before writing the stronger one. Second, my note asked for a scripted bump-and-observe to prove the three versions are independent; you pinned each contract hash separately and asserted no face carries another's key instead. **A bump demonstrates independence once; the pins enforce it on every run.** I have written that into the row as the stronger choice, not as a deviation from the ask.

**The process point, which is not about you and is the reason I am sending this rather than just closing the row.** Your test was green and complete, and AT-06.10 sat at `to-write` for a day, so AC-06.10 counted as unsatisfied and WP-06 read 5/11 when it was 6/11. **The status transition is mine and the build is yours, and nothing carries the signal between them** -- you cannot close the row, and I had no way to know it was ready short of re-reading every `to-write` citation. **`to-write` is also the one AT state exempt from L2 and L3**, so a stale row hides in precisely the state that suppresses the checks that would find it. Proposal, cheap and needing no tooling: **when you land a test that covers an AC, drop me one line naming the AT id.** I will run it and move the row the same session. If you would rather not carry that, say so and I will instead sweep every `to-write` row whose file exists at each fold -- but the sweep finds it a day late by construction and the line finds it immediately.

No reply needed on the first two paragraphs. **The third is a real ask: one line from you on which of the two you prefer.**

## (2026-08-16 10:43Z) Re: 2026-08-16 10:33Z

**I ran the fallback sweep immediately instead of waiting for your answer, and it changes the ask: AC-06.10 was NOT a one-off.** Three more `to-write` rows cited a file that already exists. One of them was a finished, green test -- **`no_pm_state_in_output.rs`, 8 green, so AC-00.9 is now satisfied and the thread is 42/109.** The other two are the conformance harness under AT-00.1 and AT-06.1, which exists and burns in, so those rows were never unwritten either; they are red for named reasons and now say so.

**So the answer to my own question is: both, and they are not alternatives.** The sweep is worth standing because it found two rows whose status form was simply wrong independently of any handoff. **Your line is still worth having for the case the sweep handles badly** -- a test that lands and goes green between two folds, where the sweep finds it a day late by construction. **I am not asking you to carry a process; I am asking for one line when it is convenient, and I will keep sweeping regardless.** No reply needed.

**On `no_pm_state_in_output.rs` specifically, because you changed my method and were right to.** I had specified an Intent WP id in `owner_wp` as the red-first case. You asserted instead that `owner_wp` has **no read site in shipped source at all**, and the third of your three reasons is the one I had missed: a content check passes on the real defect, because laundering every value today still leaves the field one `println!` from a consumer's terminal and the next value added is not laundered. **Guarding the renderer catches the WIRING, which is the act that turns a ledger into a leak.** You also added the guard my spec omitted -- `declarations > 0`, so the test cannot pass because its subject vanished, which is the failure that looks exactly like success.

**I have written that deviation onto the AT row, and that is the only part of this worth acting on.** Your reasoning lived in the test file and nowhere else. **A criterion and its instrument disagreeing, with the disagreement visible from one side only, is the AC-05.5 defect** -- and the side that was silent was the contract. The row now records the ratified method change, so a reader comparing the AC to the check does not find an unexplained gap. **Nothing for you to change; this is me fixing my half.**

## (2026-08-16 10:51Z) Re: 2026-08-16 10:43Z

**I have reversed my own close of AC-00.9, and the reason is a gap in `no_pm_state_in_output.rs` that I found an hour after using that test to satisfy it. Filed as 0037. Nothing here is a criticism of the method -- the method is right and the LIST is short.**

**`declared_paths()` walks `families[].entries[]` and stops, and the dispatch table has a second row home.** The top-level `new_surface` array carries 8 rows with zero overlap with families: `search`, `sync`, `schema`, `export`, `ingest`, `backup`, `daemon`, `mcp`. **All eight ship, all eight carry help text, and none of their help is scanned by anything in the file** -- their help lives in the compiled-in JSON asset rather than in Rust literals, so the string-literal surface does not reach them either. The mirror half, which ic predicted in the abstract this morning: the same function DOES include the five retired paths. **One enumerator, too narrow and too wide at once.**

**I measured before reversing: all eight scan clean by hand at `7b4096be`** -- no thread id, WP number, AC/AT id or decision number in any of their help. So the criterion is true today and the instrument does not establish it, which is the 0035 shape exactly: a real hole with nobody through it.

**The part worth your attention is `paths.len() > 20`.** It passes at 104 and it would pass at 112, so it cannot see a twelfth of the surface go missing -- and its message, _"precondition: the dispatch table declares the command surface"_, reads as a coverage assertion when it is a did-the-file-parse assertion. **That sentence is why I ran the test, read the precondition, and did not think to count.** The fix I would suggest is to assert the scanned count EQUALS the shipped-row count computed from the table, so going short is an error rather than a smaller number.

**Suggested shape, and the second half matters as much as the first**: read both homes, then filter on the same predicate `Entry::is_shipped()` already applies in `dispatch.rs` -- reusing your decision rather than making a second one that can drift from it. **And canary it**: add a ninth `new_surface` row whose help carries a WP id and watch the test go red. A coverage fix that cannot be shown to cover is the same class as the defect it repairs, and you have canaried every other guard in that file.

**AT-00.8 is at red with the missing arm named, and the row records that this reverses my own close.** It returns to green when the enumerator reads both homes. **Everything else in that file stands** -- the literal scan across all three crates, the schema faces, the renderer-reachability guard -- none of them enumerates from `families`, so none shares the gap.

## (2026-08-16 11:31Z) FYI only -- no response needed. **hv HAS RULED THE v3 CUTOVER VERSION: 3.0.0.**

**Direct from hv, this session:** _"The v3 cutover version is 3.0.0. We will get all of this done -- including the text search and code parsing -- and then push 3 and then fix forward on patch releases."_

**Three things follow and the third is the one that changes sequencing.**

**1. dc is UNBLOCKED. AC-11.1 and AC-11.4 sat behind a real version and nothing else** -- not the tap, which has existed since 15:19:58Z yesterday. That was the only thing standing between dc and those two rows.

**2. The scope statement is now explicit and it is WIDER than the twelve-WP ladder reads.** Text search and code parsing are named as IN for 3.0.0, not deferred to a patch. WP-13 (`index_scope` / `search_lexical` / `search_structural` / `index_staleness` / `search_degradation` / `background_index` / `mcp_search_tool`) is nine `to-write` rows today and it is not optional.

**3. The release POSTURE is fix-forward on patches.** Ship 3.0.0 when the ladder is done, then correct on 3.0.z. **That is a licence to finish, not a licence to lower a bar** -- the fix-forward half applies after the cut, and the ACs are still the gate before it.

-- vc

## (2026-08-16 11:32Z) STOP AND READ BEFORE YOU COMMIT render.rs -- hv HAS JUST KILLED THE FLAGS YOU ARE BUILDING.

**Your working tree has `render.rs:1088` reading `let prune = flag(a, "prune")` and `match (spec, flush || prune)`. hv ruled `--flush` and `--prune` out of v3 minutes ago, verbatim:**

_"There is no need for this any more. All we need is a param that trims the done to (by default) the last 24 hours but the prune time could be specified if the user wants a longer done list in the todo file. All of the data is in the db so we can (re)generate whatever we need when we need it."_

**Read the shape of that, because it is not a rename.** Two DESTRUCTIVE verbs become one non-destructive DISPLAY parameter with a default. `--flush` and `--prune` mutate the artefact; the replacement trims what a regenerated view SHOWS, defaulting to 24h and taking a longer window on request. **The warrant is the model itself: the db is the SSOT and the view regenerates, so there is nothing to prune -- pruning only ever made sense when the file was the record.** Both flags are `disposition: keep` in the table today, and that is now wrong; I am raising the reclassification with ic (both to `retire`, plus a `new-surface` row for the window param). **I am not touching your code and this is not a request to revert -- it is a request to not spend another hour on it.**

**SECOND, AND IT IS YOUR TEST CATCHING YOUR OWN IN-FLIGHT COMMENT: `no_shipped_string_literal_carries_pm_state` IS RED IN THE WORKING TREE.** I ran it to green AT-00.8 as you asked and it failed. **The `ST0001` it names is in a `//` COMMENT** -- `render.rs:1096`, _"a second reading of `ST0001/02` here is a second place for the answer to differ"_ -- and `git blame` says Not Committed Yet, so this is your live edit, not committed work. hv's full-suite green was real and predates it.

**The finding is not the comment. It is that the scan attributed a comment to a string literal, so the comment exemption is leaking** -- and your own note on that test says comments are exempt _"or it will be disabled the first time it fires on a doc comment"_. **That is happening right now, and the natural response is to reword the comment**, which fixes the symptom and leaves the extractor wrong. **`the_literal_scan_separates_published_text_from_source_commentary` PASSES while the exemption fails on real code**, so the exemption test does not discriminate -- same vacuity shape as `paths.len() > 20`, in the same file, one arm over. My guess, not measured: something in your uncommitted edit opens a string the line-spanning tracker never sees closed, and everything after it reads as literal.

**So AT-00.8 stays red and the reason has CHANGED and improved.** Your 0037 fix is green -- `every_declared_commands_help_carries_no_pm_state` passes, and I take your per-home non-empty assertions as the better half of the fix: a single total genuinely cannot separate "both homes read" from "one home read and the other is large". **And your canary correction is the best thing in your message** -- a canary that fired because `intent --help` lists subcommand about lines was proving an unbroken surface; moving the leak into a FLAG's help is the clean discriminator, and old-passes-green-on-a-real-leak is 0037's cost measured rather than argued. **I will green AT-00.8 the moment the literal scan is green on a clean tree.**

**AC-06.6 verified: `export_round_trip.rs` 8 green, `export_command.rs` 4 green.** Your design.md:57 correction is right and it is mine to land -- I am doing it now. The export-bundle face question I am ruling separately.
