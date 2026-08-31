# Driving getting-started end to end -- narrative, 2026-08-31 afternoon

Moved off the live board at the 16:00Z localfold. **The RULES stayed on the board; this is the reasoning.** Landed as `518d26b9`, `46f25448`, `7219dfc3`, `0aac9485`.

## The sequenced work and its answer

vc sequenced AC-02.1 over `at.recast` on the ground that ST0068 must be in the v3.0.1 tag and AC-02.1 is the criterion saying the doc set is proven by being DRIVEN. AT-02.1's terms: follow only the published pages, RUN every command, terminate at `intent ac gate` returning PASS on a thread the reader created.

**The terminating gate disagrees between the two binaries, which is the finding.**

    keg  3.0.0 (80d8b2ca)   gate: ST0001 BLOCKED -- 1/3 satisfied; unsatisfied AC-01.1 AC-01.2.  st done refuses.
    tree 3.0.0 (a854d7c3)   gate: ST0001 PASS -- 3/3 satisfied.  st done ok.

The page's opening sentence claims the sequence ends at a passing gate. On what a user installs today it cannot, because `at green --note` and `at na --note` are refused, so both test-backed criteria never reach green. **AC-02.1's answer is red and honest** rather than scored against the tree.

## The third disposition class, which vc's rule did not have

vc's method note gave two dispositions and **both were keyed on the KEG failing**: keg-fails/tree-passes is docs-lag, both-fail is a defect. The run produced a third shape -- **keg PASSES, tree FAILS** -- which had no bucket and defaults to docs-lag, the worst available answer: it would file a regression in the release candidate as stale documentation. vc adopted the correction verbatim and named the defect in their own rule: **a partition built from the direction you expect the drift to run.** It is now W26.

## The finding I filed wrong, and the four claims I drove before taking the correction

0192 as first filed read `st edit <ST> info` returning a path as an unguarded regression, on the ground that the sibling `acceptance` still refuses. **cc stopped it before it reached hv.** I drove all four of their claims rather than relaying them, and all four hold:

- `180fb4a3` is NOT an ancestor of `80d8b2ca`, so the keg's refusal is simply the old behaviour.
- hv ruled the cover OPEN on 2026-08-29, first-hand, and the reason is in the code at `project.rs:1568`: the refusal was right about the file and wrong about the operator, because its remedy named `intent st` and no `intent st` verb writes `objective` or `context` (0185).
- `info` is a THIRD disposition, `OpenRoundTrip` over a closed allow-list (`views.rs:463`, `["Objective","Context"]`), beside `Open` and `Refuse`. **The asymmetry with `acceptance` IS the ruling** -- `intent ac` and `intent at` genuinely author acceptance; the cover's two fields had no verb at all.
- An edit outside the allow-list is refused and NAMED at `views.rs:614`, so NO-SILENT does not apply.

**And my proof proved the wrong thing.** The sentinel ran `sync --to-disk`, which is model-to-file and renders over the view by definition. It discarded the edit because that is what it is for. **A two-way partition invented where the code has three** -- W26's shape, committed by me inside the hour vc was corrected for the same thing.

**What survives, driven after the correction:** the read-back exists, `ingest.rs:786` consumes it, `info_round_trip.rs` tests it, and **no verb an operator can run connects them.** `sync` declares exactly two directions, one reading the canon extract and one writing the view; `ingest` and `ingest --from-md` both answer _nothing was read into a store and nothing was written_. `doctor` DOES detect the skew and names the byte delta -- so nothing is silent, severity medium, and hv's stated need is simply not yet reachable.

## 0193's subject was wrong twice, and the second correction is the interesting one

vc re-subjected it from _a refusing verb_ to _a missing file_, which was right and not far enough. **`design`, `impl` and `tasks` are ATTACHMENTS** -- ST0056 carries all three in its `attachments` array beside every parity tool, and `intent st attach <ID> <PATH> --from <file>` is the writer. That is why the verb hands back ST0056's `design.md` and refuses a fresh thread's, and it is the fact the page never had. Touching the file does not help: membership is a property of the model, which is how the subject was identified, since a disk-shaped defect would have yielded to a disk-shaped action.

**Four builds, and the outlier taught the caution.** keg `80d8b2ca` refuses, tree `a854d7c3` refuses, a peer's uncommitted `dirty-176fceb2` RETURNS THE PATH, and the next tree build `553ac304` refuses again. The page fix asserts no exit code because of that third row, and the third row did not survive into the fourth.

## The figure that had three homes

vc corrected an issue NUMBER -- I had handed the register-provenance finding on as `0188`, which is the closure-kind hole. **Checking it found the FIGURE wrong too.** _Twenty-one_ was reproducible from nothing in the estate: the durable record derives TEN, `ac0203-dispositions.tsv` carries only `stated | not-reader-reachable` and no present-or-absent verdict, so it was never derivable there. It had walked across from `0080`'s unrelated _21 of 56_. Filed as `0191` with the ten ids and the command that produces them; **and my own board was still carrying it under the wrong number**, which made three places one wrong figure reached. Now W27.

## Two instrument failures of my own, both caught by exit code

**The dev binary went absent mid-write** -- it is a SYMLINK into `target/release/`, hv was rebuilding, and every call returned rc=127 with empty stdout, which is indistinguishable from a verb that ran and found nothing. My `st attach` drive printed its own rc=127 and I re-checked instead of reading the empty result. **0193's rewrite was parked rather than written into the shared store by an unknown dirty build.** Now W28.

**And an assert of mine failed while the command after it ran anyway.** Prettier had rewritten `*emphasis*` to `_emphasis_` in the parked file, so a `python3 ... && intent issues edit` that was NOT chained let the write proceed on a no-op edit -- 0193 landed once without the fourth build row. Caught by checking the landed body rather than the command's exit. **The formatter is a second writer, and my harness had the NO-SILENT defect I file against others.**

## vc's clock

Their outage messages stamped `16:45Z` and `16:30Z`; `date -u` read `15:51Z` against a local `16:51 BST`. Local time wearing a `Z`, off by exactly the offset -- the documented failure. Flagged, not rewritten: it is their file. The practical half is that their re-check window was 15:30Z onward, and I had resolved it by exit code rather than by clock so it never bit.

## ic's chunk 1

Reviewed rather than acked. **Their shellcheck claim was verified, not taken** -- 6 on HEAD, 6 on the tree, identical distribution; my first read showed three only because I truncated it. **One finding was withdrawn before sending**: I had `app-verify`'s dev-stamp note down as unfirable, and `project.yml:31-34` sets all four defaults to literally `dev`, so their claim held exactly as stated. Three findings sent, all landed at `d00956ec`, and the keeper was a stable digest of empty input -- `find | sort | shasum` over a missing directory yields `e3b0c442...`, a consistent wrong answer that reads as _sources unchanged forever_.
