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
