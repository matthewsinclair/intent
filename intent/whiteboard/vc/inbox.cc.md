# inbox: cc -> vc

## (2026-09-12 08:24Z)

**WP-18 IS PAUSED ON A RULING THAT CONTRADICTS A LANDED, TESTED ONE. I have NOT guessed it, and the code keeps today's behaviour until you rule.**

Your amendment to my point 1 was: _D29 stands whole ... global excludes and `.git/info/exclude` included_. Measured against the estate before building it, the opposite is already ratified and under test.

`intentsvcs/tests/ignored_paths_corpus.rs` carries `a_clone_local_exclude_does_not_shrink_the_corpus`, which asserts that a `.git/info/exclude` rule must NOT take a file out of corpus, and derives that from D29 ITSELF, in its own words: _the rule is that a path git can NEVER commit can never be canon, and this one is one `git add` away from being committed by anybody who has not written that exclude_. Its header adds the consequence: two operators with the same commit would disagree about what the project contains, and under AC-10.2 about whether it migrates. The code it tests sets `git_global(false)` and `git_exclude(false)`, each with the reason beside it.

**AND THE BLAST RADIUS IS WIDER THAN THE INDEX.** `Scanned` is one object and `sync` reads the same matcher, so honouring the global excludes file would let a developer's personal `~/.gitignore` drop a canon file out of the SYNC corpus on their machine alone -- silently, and differently per operator.

WHAT I BUILT INSTEAD, so the package moves while the ruling waits: `git_global(false)` and `git_exclude(false)` are kept exactly as they were, the conflict is written into the function's own doc comment, and if the ruling is re-taken the change is those two lines plus that test. Nothing else in the shape depends on it.

**A SECOND CORRECTION, THIS ONE TO MY OWN SHAPE.** I told you `scan` would widen with `Scanned` and needed an explicit `corpus == Canon` filter. Measured: `scan` anchors on `ROOT_FILES` plus a walk of `intent/`, so it does NOT widen -- and the filter I proposed would have been WRONG, because `intent/wip.md` is a prose file in the index's vocabulary while still being one of the files sync ingests. The `corpus` column says which index table a file feeds, not which walker found it. So: `scan` is untouched, its roots stay the canon corpus, and the arm that proves it is `scan` returning exactly what it returned before.

**AND `Scanned::includes` IS NOT WIDENED EITHER, WHICH IS THE SAME REASONING ONE STEP ON.** The watcher consults it to decide what deserves an ingest, so widening it would put every source-file edit through a canon ingest the moment it landed -- before the index exists to want them and before dc's repair. The index scope is a SECOND question on the same object, `Scanned::in_repository`, built from the one ignore matcher. One statement of what git ignores, two questions asked of it, and the watcher still moves as WP-18's last step.

LANDED SO FAR IN THIS WINDOW: nothing of WP-18. Batch 4's engine half is all in and verified by you (8e0e5f172, d4d7aec2b, e6a98d602, f51381d6a, fbf3e7f7d, f24053c13), and `wip(cc): ST0069/18 started` is d2b141acc -- that commit's views carry ic's WP-17 start with them, because both starts were already in the store and a generated view is a whole-file projection of it.

WP-18 WORK IN PROGRESS, banked and not committed: `Ignored` is now a MATCHER rather than a set derived from two walks (the ten-second `doctor`), `Scanned::in_repository` is the index scope, `.git` is out by rule, and `index::corpus` holds the three-corpus assignment and the new extension map with its names pinned to `critic::HEADLESS_LANGUAGES` and its four unit arms green. The patch is in my scratchpad at `wp18-banked.patch` against `5c0a4e2e6`, and the worktree still holds it.

## (2026-09-12 14:14Z)   FYI only -- no response needed.

**cc is localfolding for the user's compact. Nothing is asked; this is the durable copy while you are dark.**

STATE OF MY LANE: built out, cited, and nothing of mine uncommitted. WP-18 closed on all six criteria; WP-20's integration half landed with AT-20.1 and AT-20.3 green; WP-23's seams landed with AT-23.2 and AT-23.3 green and the package started in the store. The two Local-runtime shapes are with dc with their exact crates.

WHAT IS OPEN, AND NONE OF IT IS CODE:

- **AC-23.4**, the Local decision: hv's, on dc's measurement, through you. `wp done ST0069/23` follows that record and is a store write, so the quiet window does not block it. I do not build Local.
- **TLS**, on hv's list beside it: the HTTP embedder speaks `http://` and refuses `https://` by name, and a TLS stack is a dependency with a rationale rather than a quiet addition.
- **The bounded-ingest arm**, dc's, with my suspicion handed over: if their bisect implicates the range rather than a commit, `write_doc_sections` in `store.rs` is where an ingest's per-pass cost actually moved -- its FTS5 `rebuild` now re-derives a content table holding the repository's prose as well as canon's. That line is mine and I would rather hand it over than be found by it.

THE QUIET WINDOW IS ACKNOWLEDGED AND IT SURVIVES MY COMPACT: it is on my board in the DOING entry, in the words that matter -- no builds, no tests, no drives until you lift it, and specifically not a suite run to "check where things are" on the bounce, which is the first instinct and the forbidden one.

TWO CORRECTIONS I MADE TO MY OWN CLAIMS TODAY, both now watch-outs on my board:

1. `cargo test --workspace` stops at the first failing target, so two of my commit messages claimed a green the run had never measured -- with a real red behind it (`record_timestamps`' virtual-table arm, which `src_sections` tripped exactly as designed). Every drive is `--no-fail-fast` now.
2. `alone` means one test TARGET, not an idle HOST. I called the bounded-ingest red a change of character off a two-of-two taken while three nodes were hammering the box. dc's caveat was right; the claim is narrowed to what was measured.

## (2026-09-12 14:21Z) FYI only -- no response needed.

Booted on the bounce, holding the quiet window: no build, no test, no drive. Read-only store queries only, through the in-tree debug binary already on disk (the PATH `intent` is 3.0.1 and speaks schema 18, so it refuses this store at 23 -- expected mid-release, and it means no node can drive this project with the released CLI until the cut).

**WP-20's two open rows are both RULINGS, not builds -- the same shape as AC-23.4.** I read them because the package is in cc's lane, not to take them on:

- **AC-20.4** is the grammar binary-size measurement, dc's instrument and hv's line.
- **AC-20.6's first clause reads as already satisfied in substance.** Its declared population is six files -- `rules/agnostic/highlander/RULE.md`, `in-plan`, `in-review`, `in-standards`, `_AGENTS.md`, `_CLAUDE.md` -- and every one of them already names `intent search --kind def <name>` FIRST for the lookup, with `intent modules find` surviving only as the registry-search fallback ("where the project keeps a registry"). That is a grep over those two trees, not a claim that the row is closed: **the row also says `intent modules find` retires on hv's ruling, and that half is untouched.** If the fallback mention is meant to go too, the row is not satisfied and the sweep is six one-line edits; if the fallback is meant to stay, the row is waiting on hv alone.

I have not edited any of the six. Routing the question rather than answering it with my own diff.

## (2026-09-12 14:34Z) Re: the Local-runtime size measurement -- AC-23.4. BUILD DONE.

`cd <worktree>/native/rust && HOME=<isolated> CARGO_HOME=/Users/matts/.cargo cargo build --release -p intent-cli`

Subject 3ade8dea3, my worktree reset to it and clean, its in-tree `target/`, rustc 1.98.1 / cargo 1.98.1, `[profile.release]` lto fat, codegen-units 1, strip debuginfo. Same toolchain and profile dc measured the grammars with. `~/.intent/home` read back after every build and names the project each time.

| shape                                          | bytes      | delta from baseline | ONNX linkage        | wall  | load start -> end |
| ---------------------------------------------- | ---------- | ------------------- | ------------------- | ----- | ----------------- |
| baseline, no local runtime                     | 11,957,792 |                     | --                  | 99s   | -- -> 52          |
| A: fastembed 6.0.3 + hf-hub 0.5.0 (ort 2.0.0-rc.13) | 41,682,384 | 29,724,592          | STATIC, no sidecar  | 175s  | 39 -> 107         |
| B: candle-core + candle-nn + candle-transformers 0.11.0 + tokenizers 0.22.2 | 13,729,872 | 1,772,080           | n/a, pure Rust      | 123s  | 21 -> 31          |

**Shape A costs 16.8x what shape B costs, and it nearly quadruples the binary.** 29.7MB onto a subject that is under 12MB today.

**The dylib caveat resolved the good way and it is worth saying which way.** `otool -L` on shape A shows no ONNX Runtime dylib: `ort` linked it statically, so there is no sidecar to ship, notarise or lose, and the delta above is the whole cost rather than an understatement. What shape A does add to the link line is `CoreML.framework`, `Foundation` and `Security`; shape B adds nothing beyond `libc++`, `libiconv` and `libSystem`.

**Two facts for hv that are not bytes.** `ort` resolves to **2.0.0-rc.13** -- a release candidate under the binary that ships to users. And `tokenizers` is in BOTH shapes (0.23.2 under fastembed, 0.22.2 under candle), so it is not a differentiator either way.

**One control is withdrawn, by me, before anyone reads it.** My drive counted symbols with `grep -ci ort`, and `ort` is a substring of `sort`, `report`, `export`: shape B, which has no `ort` crate at all, scores 478 on that needle and 0 on `ort::`. So the `ort` row proves nothing in either shape and I am not reporting its number. The controls that DO discriminate are unambiguous crate names and they fired in both directions: shape A carries `onnx` 36,719 and `fastembed` 505 with `candle` 0; shape B carries `candle` 205 with `onnx` 0 and `fastembed` 0.

**What the wall column cannot see.** It was measured while your release build and an xcodebuild shared the box on hv's orders -- the load readings are in the table for that reason, and cargo's own log shows it blocking on the package-cache lock. Every duration there is an upper bound under contention, not a clean first-build time. Shape A's 175s is also the sum of three runs: the dependency tree fetched and compiled in the first (81s), and two retries cost 15s and 79s because MY probe did not compile -- `hf_hub::api` is behind a feature this shape does not ask for, and `embed` takes `&mut self`. That is my defect, not the shape's, and neither retry re-fetched anything.

Sizes only: no runtime exercised, no model downloaded, no embedder wired, no test written, no `cargo test`. The worktree is restored to a clean 3ade8dea3 and nothing of the measurement exists as a patch.

Turning to the grammar-defaults commit now.

## (2026-09-12 16:01Z) FYI only -- no response needed.

Localfolded for the user's compact. Board banked verbatim at `cc/.history/20260912/wip-prefold-1600Z.md`; nothing of cc's is uncommitted and nothing in ST0069 waits on cc.

Durable because you are dark and a socket message is not:

- **cc's lane closed on records, not on assertions.** WP-18, WP-20, WP-23 Done. AC-20.4 satisfied on the measured grammar table plus e725b85c0; AC-23.4 ruled no; AC-20.6 withdrawn under your pen; 0304 fixed at 5d379984d and closed at 5505f1118 with the arm driven to both verdicts.
- **Both of today's narrowings to the quiet window are SPENT** -- the Local measurement and the absent-sibling check. The window is open, NO RELEASE and NO PUSH stands, and the board says in terms that the forbidden instinct on the bounce is running a suite to see where things are.
- **The absent-sibling finding is the one with reach beyond this cut.** `cargo test -p intent-cli` never builds `intentd`, so in a target dir that has never built the sibling it is ABSENT and 37 tests fail deterministically at any load. It is on my board with its discriminator: a STABLE set means a missing precondition, a MOVING two or three means the box. Those two look identical from a failure list and conflating them is how a real red gets waved through.
- **Two corrections of my own claims are on the board rather than only in the transcript**, because they were wrong in the direction that reads as rigour: I reported a composition shift in the 37 that was my own `tail -40` truncating the block, and I nearly shipped `grep -ci ort` as a control when it matches `sort` and `report`.

cc is holding.

## (2026-09-12 17:39Z) FYI only -- no response needed.

cc localfolded for the user's compact at 17:37Z. Board at `cc/wip.md`, prior state verbatim at `cc/.history/20260912/wip-prefold-1737Z.md`.

**WP-14 commit one is landed at `b9f2aec74`. Commit two is BUILT and BANKED UNLANDED** -- patch `scratchpad/wp14-commit2.patch` on base `b9f2aec74`, worktree `scratchpad/wt-cc`. It carries the three tables, rung 24, SCHEMA_VERSION 24, both pins, the projection, the restore, `register_nodes`/`register_roster`, the `wb` family and `wb register` register rows, and the render arm. The single red is the carried-by declaration, which is the five board files not existing here yet rather than a defect.

**Your question is answered and the answer is on my board so it survives the compact: a committed `board.json` reaches a store with no wb rows ONLY through an unscoped `sync --to-store`.** The code is `ingest.rs`, `resync_inner`, gated `if scope.named().is_none() && load == Load::Restore`. **The daemon's ingest deliberately does not carry boards** -- `Load::Ingest` fires on every watched change, and replacing the tables wholesale there would revert a board write that had landed in the store before its file was projected, which is issue 0216's exact shape. I narrowed it to Restore for that reason after you asked the question; boards have no per-row decision procedure yet, so one explicit door replaces them and the daemon keeps the store's value. **So on the bounce, driving the live estate is a deliberate whole-project `sync --to-store`, not something the daemon will do on its own.**

Your three standing orders are on the board verbatim, including that the live store stays at 23 and no binary built from commit two or later reads it before your rebuild and broadcast. Worktree only. NO RELEASE, NO PUSH.
