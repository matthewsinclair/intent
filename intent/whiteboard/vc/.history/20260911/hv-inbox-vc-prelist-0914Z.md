# inbox: vc -> hv

## (2026-09-11 08:51Z) THE FINISH LINE FOR 3.0.1 -- everything you are asked, in order, and everything that was culled to get here

The 93 entries this file carried are archived verbatim at `vc/.history/20260911/hv-inbox-vc-precull-0849Z.md`, cmp-verified. Every ask in them is either answered below, culled today, or dead. `intent/wip.md` carries the same text and is the one home; this entry exists so the durable surface is not empty.

## What ships 3.0.1

These rows are satisfied BY the cut. There is nothing to build for any of them.

| row | what it asserts |
| --- | --- |
| ST0056 AC-00.5 | `brew install` on a clean machine yields `intent` and its daemon |
| ST0056 AC-00.6, AC-12.1 | the 26 v2 shell files are deleted and nothing executes a `bin/intent*` path |
| ST0056 AC-07.7 | every `claude` door is reachable in the installed keg |
| ST0056 AC-11.1, AC-11.4 | a tag yields artefacts and a tap formula; sign before checksum |
| ST0056 AC-12.4 | tagged on both remotes, release published, formula live |
| ST0058 AC-00.1 | the installed pair is coherent: one build, one commit |
| ST0068 AC-04.2 | the docs are in the same tag |

ST0056 WP-07, WP-11 and WP-12 close with those rows; ST0056, ST0058 and ST0068 close with them.

## Decisions for hv, in order

None of these is a test. Each is one word, and the word unblocks the item.

1. **Push.** `git rev-list --count @{u}..HEAD` says how many.
2. **The store migration 13 -> 17 is irreversible and nothing shipped says so.** A CHANGELOG line, a refusal in `migrate`, or a backup step. Recommend: the CHANGELOG line plus a backup sentence in the migration docs. Docs only.
3. **Three doors the canon orders every agent to call answer "not implemented yet"**: `st bootstrap`, `agents template`, `claude prime` (ST0058 AC-00.3). Build them, or strike them from the dispatch table and the CLAUDE.md/AGENTS.md templates. Recommend: strike. XS each, fail-forward.
4. **The 16 `collapsible_if` lints in intentsvcs.** One mechanical edit at 16 sites. Say go. Until it lands, CI's `rust` workflow stops before `test` and nothing in the suite is verified on a runner.
5. **The one red test, `mutation_completeness`, has no criterion behind it** (AC-04.6 withdrawn). Delete it, or ratify `AcceptanceTest.kind` as a sixth machine. Recommend: delete.
6. **The v2 bats suite dies with the trunk at the cut.** 186 of CI's 216 `Intent Tests` failures are v2 tests of pruned v2 doors; 15 more are v2 tests pointed at v3. Confirm: prune, no port. That workflow stays red until the cut and goes green by deletion.
7. **ST0064 AC-01.7: sign and notarise the menubar app.** Your Apple credentials, nobody else's. Or drop the app from 3.0.1, which reverses your 08-31 ruling.
8. **ST0057 WP-12 and WP-13** (issues get a realised file; the v2 `COMPLETED/` bucket is ingested-then-removed or marked stale). Two S features with no ruling. Build, or descope to ST0069 and close ST0057. Recommend: descope.
9. **ST0068 AC-03.1, AC-03.2** (the design-system doc for the Laksa site). The site is not in the Intent tag. Descope to ST0069? Recommend: yes.
10. **Two data-loss defects in intentd: ship or block.** `0216`/`0212` (the disk ingest reverts a completed canon write under contention) and `0206` (canon read-modify-write with no compare-and-swap). Recommend: ship 3.0.1 with both in `docs/known-defects.md`; fix in 3.0.2. Releases are yours.

Done under the pen, no word needed: `CHANGELOG.md`'s heading corrected from `unreleased` to `in progress`, which is the release handler's grammar. `int local status` lands after the cut; no criterion names it.

## Culled 2026-09-11

- 11 ST0056 criteria withdrawn (AC-00.1, 00.2, 00.8, 00.10, 00.14, 00.16, 04.6, 06.1, 10.5, 15.2, 15.3): instrument, register, baseline and conservation-proof rows. Their records stay; they no longer gate.
- ST0056 WP-04, 05, 06, 10, 15, 17 done; WP-13, 14, 16 cancelled (already descoped to ST0069).
- ST0061, ST0065 done. ST0046, ST0059, ST0062, ST0063 cancelled.
- 90 issues closed: 85 whose subject was a guard, gate, register, parity tool, proxy or baseline; 5 discharged. The ids are in commit `6918a2e5`.
- Every hv inbox is one pointer entry. The dated ruling record and the unruled-questions list on hv's board are archived verbatim under `hv/.history/20260911/`.

## Out of 3.0.1, on the record

- ST0060 (vault), ST0069 (post-cut), ST0070 (LLM config): untouched.
- `config`, `ext`, `learn` ship declared-and-unbuilt (hv, 2026-08-31).
- 91 open issues, every one a product defect. None blocks the cut unless item 10 says so. `intent issues list`.
- The burn baseline, the AC-06.1 re-baseline, flip-then-burn, the `INTENT_BIN` flip, AT-07.5's daemon-free window, the `overhead` meter (W46), the floating-lint policy, the obliged-reader-of-CI question: all apparatus, all dropped with the loop.

## Known and shipping as-is unless ruled otherwise

- `a_daemon_outlives_nobody.rs` flakes roughly 1 in 20 in parallel; mechanism unconfirmed.
- A SIGTERM'd daemon does not checkpoint its WAL; a 559 MB WAL was truncated by hand on 2026-09-10.
