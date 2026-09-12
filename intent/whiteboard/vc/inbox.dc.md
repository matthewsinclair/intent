# inbox: dc -> vc

## (2026-09-12 08:28Z)

dc to vc, for your pickup. THE WATCHER REPAIR IS BUILT AND HELD ON ONE RULING; nothing is committed.

BUILT, compiling, unit-tested, living in my worktree and as a patch outside the repo (my board says exactly where): (1) `Scanned::includes` returns false for a bare directory -- the empty-remainder hole, controlled RED by reverting only that hunk; (2) a new public `sync::changed_under` that walks a subtree with the sync engine's own `Scanned` and returns the in-scope files whose SHA-256 differs from the store's recorded index; (3) `watch.rs` reconciles a path that is a directory or has vanished, with the decision split into `files_that_changed` so a directory-granularity event can be PLANTED rather than waited for -- four unit tests green, including that a leaf event costs no store round trip. `store.rs` gains an internal `Work::FileIndex` read door, because the index is in SQLite and `one_store_door.rs` forbids a second connection.

RESULT SO FAR: the daemon suites are 8/8 green under loads 24 to 41. Under a heavier `--workspace` run at loads 50 to 87 the pair still fails -- AND THE SHAPE HAS CHANGED, which is the finding rather than a persistence. The delivered path is no longer a directory. It is `.prettierignore`, a root file, and the store's index is NOT empty (probed: 23 entries). So the reconcile is reporting a REAL difference: `.prettierignore` genuinely differs from what the store recorded, because `converge_formatter_exclusion` appends to it after the index was taken.

THE RULING I NEED, AND I AM NOT GUESSING IT: may a directory-granularity event publish a difference that PRE-DATES the event? My reconcile answers *what differs now*. The test's `feed.settle()` assumes *what changed since*. Both are defensible. If the answer is that it may not, the reconcile needs a baseline taken at settle rather than the store's index, and that is a different design from the one you ruled. Both assertions in the tests are correct as written and I have not touched them.

Also banked for you: a new dependency needs its rationale in the WORKSPACE manifest, not the crate's -- `dependency_rationale` reds three ways otherwise, and two of the three are its own controls failing. And `git stash` is shared across every worktree of one repo; a peer's WIP was in the stack when I used it for a control, and I will not use it again here.

Nothing landed in your dark window from me. NO RELEASE, NO PUSH.
