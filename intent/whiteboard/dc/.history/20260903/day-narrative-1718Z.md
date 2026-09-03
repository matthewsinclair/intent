# dc, 2026-09-03 -- narrative for the 17:18Z aggressive localfold

Second fold of the day. Companion to `wip-prefold-1718Z.md` (sha `12a4a0ec`, 49,082 bytes, `cmp`-verified). The morning's fold and its narrative are `wip-prefold-1446Z.md` / `day-narrative-1446Z.md`; this covers the afternoon, which had one subject.

## The afternoon had one subject: a ruling that never arrived, and what driving it cost

Cold pickup after the compact at 14:55Z. **The whole afternoon came out of reading hv's board rather than my own.**

## The find that started it

hv ruled the `AC-02.3` fixture-coverage census on 2026-08-31 14:41Z with **OWNER dc**. It had never been on my board and appeared in no inbox I hold, back to `20260815`. **Not a ruling I dropped -- one that never arrived.**

**The general form is the keeper: a pickup reads your own board and your own inboxes, so work assigned to you that never arrived is invisible to both. Yours is the one place in the estate where its absence IS the defect.** No amount of reading my own board finds it. vc banked it as `9t` and owns the remedy -- a sweep of hv's board for `OWNER:` lines against what was actually routed.

## The census, and the answer that retired an option instead of pricing it

**Zero fixtures, and the reason is structural.** A disposition is a claim about REACHABILITY, not a REPRODUCTION. The checker verifies `stated` against page BYTES and `not-reader-reachable` against a written REASON. **Neither arm reproduces anything**, so no environment can be on the critical path of either.

hv's four candidates each fell to one call: the keg SHIPS `intentd` and the docs carry `reference/intent-daemon.md`; the keg ships NO skills (`0112b8c1` not an ancestor); `0071` already carried its own drive; nothing wanted a bucketed thread.

**Then the correction, and it is the one worth keeping.** `docs/known-defects.md` opens by promising **every entry was DRIVEN against v3.0.0**. So a `stated` row is a DRIVE, not a paragraph, and I had priced it as a paragraph. **And the page's `## What this page does not cover` names hv's four fixtures VERBATIM** -- a migrated v2 estate, a bucketed thread, an installed skill, a running daemon. **hv did not invent the fixture framing. They read it off a page I wrote.** So the remainder IS the fixture set, and the census's real product is knowing WHICH members need one.

## Driving inverted four titles, which is the whole argument for the page policy

`0207` -- **no keg `at` verb accepts `--note` at all**, HEAD's does; the flag is post-cut. `0215` -- `llm usage_rules` refuses rc=2 unimplemented, so the symptom is not exhibited. `0063` -- **`WpStatus` carries `Cancelled` at the tag AND at HEAD**, so the title's leading premise is false. `0224` -- reproduces; `init` writes a 3KB Elixir decision tree with `languages []`.

## `0063` is the row of the day and it caught both of us

**A false premise sitting on top of a real defect is the one shape no reading can resolve.** vc, reading it, would have migrated the refuted premise into the body and made it look specified. I, reading the same evidence an hour later, dispositioned it `not-exhibited` and would have discarded a live defect. **Same reading, opposite destructions, and the only thing that separated them was running the verb.**

Driven: a WP whose only criterion is descoped **cannot be closed**; the remedy printed underneath -- _satisfy or formally descope the remaining criteria_ -- **cannot be followed**, because the refusal fired precisely because nothing remains; and the escape it names, `acceptance: exempt`, **has no door at WP scope**. The two blocking paths are DISTINCT: `in_scope.is_empty()` PASSES via the WP-lenient rollup while `active == 0` over a non-empty scope BLOCKS -- so a reader reproducing on the wrong shape concludes not-exhibited, which is exactly what happened.

Seeded `0227` (`acceptance: exempt` has a read path and no writer) in someone else's hand.

## `0223`, and an artefact that became a scar

`intent st new help` / `intent issues add help` accept a bare subcommand name as a title and write permanent state at rc=0. **Reproduces on the keg.** `0095` and `0096` were never undecidable -- they are ARTEFACTS of it, with ST0061/62/63 from the same `9d717901` batch.

**Then `0225` and `ST0071` appeared LIVE at 16:39:02 while I documented it**, untracked, and neither mine nor vc's nor cc's -- all three of us excluded on evidence rather than memory. I checked my own six fixtures because I had run `st new help` all afternoon and could not honestly disclaim otherwise.

**And vc found the correction that made the page entry better: `ST0061` was ADOPTED** -- 602 bytes of objective, `wip`. **Origin does not determine disposition.** Titles are write-once on v3.0.0, so genuine in-progress work is permanently stuck with a bare subcommand name. **An adopted artefact argues for `0223` harder than five pieces of rubbish could, because it shows the cost surviving the cleanup.** The count came out of the page rather than being decremented.

## `0206` and `0213`, each with a control that changed the finding

`0206` -- three concurrent `ac new` each print `ok` at rc=0 and one is lost. **Control: sequential survives 3 of 3**, so it is contention, not the verb. **Second control, which matters to cc more than to me: a RAPID SEQUENTIAL burst also survives** -- that is `0216`'s shape without a daemon, so `0216`'s mechanism really is the watcher.

`0213` -- a `kind: test` row with no `--file`, driven green, is reported `1 AT row(s) conform`. **The control landed better than expected: `at new` citing a MISSING file is REFUSED at rc=1.** So the tool checks the path when given one and checks nothing when given none. **Citing a wrong file is caught; citing no file is blessed.** Seeded `0229`, which is the same fault in the CLOSE GATE -- worse, because a linter misleads a reader and a gate closes work.

## Five instrument defects in one session, and the pattern is sharper than "same breath"

`$?` read after a pipe through `head`, so every rc read 0. `jq -r` appending a newline -- **twice**, the second time inside the check investigating the first. Backticks in a double-quoted commit message eaten by zsh. `head -c -1` being GNU, so a `shasum` returned `e3b0c442...`, **the hash of the empty string, which no eye distinguishes from a real digest.**

**Three of the five were the SAME defect recurring with the rule already written down.** Knowing it did not stop me, because the knowledge lived in a watch-out and not in the shape of the command. **A rule you have to remember at the keyboard is not a control.** I positive-controlled the drive harness and skipped it on the counting one-liners because they looked too small to need it -- **and those were the ones that were wrong.**

vc's response to that is the sharper generalisation and it indicts the whole practice: **a watch-out is a STAGING AREA, not a destination.** The estate's guards -- clock, header, shared-artefact, append-only -- are each a discipline that stopped being remembered and became a mechanism, and every one was built after the discipline failed.

## Two premises that died while I was writing the final report

**`intent issues edit` EXISTS at HEAD** (cc built it), so `0090`/`0151` are no longer true of HEAD and `0063`'s migration is startable. The keg still lacks it, so the shipped page entry stands.

**There is no built-pair lag.** `currency ok`. vc was about to put a rebuild window in front of hv for a lag that does not exist.
