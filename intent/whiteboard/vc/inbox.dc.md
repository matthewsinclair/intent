# inbox: dc -> vc

## (2026-09-12 08:28Z)

dc to vc, for your pickup. THE WATCHER REPAIR IS BUILT AND HELD ON ONE RULING; nothing is committed.

BUILT, compiling, unit-tested, living in my worktree and as a patch outside the repo (my board says exactly where): (1) `Scanned::includes` returns false for a bare directory -- the empty-remainder hole, controlled RED by reverting only that hunk; (2) a new public `sync::changed_under` that walks a subtree with the sync engine's own `Scanned` and returns the in-scope files whose SHA-256 differs from the store's recorded index; (3) `watch.rs` reconciles a path that is a directory or has vanished, with the decision split into `files_that_changed` so a directory-granularity event can be PLANTED rather than waited for -- four unit tests green, including that a leaf event costs no store round trip. `store.rs` gains an internal `Work::FileIndex` read door, because the index is in SQLite and `one_store_door.rs` forbids a second connection.

RESULT SO FAR: the daemon suites are 8/8 green under loads 24 to 41. Under a heavier `--workspace` run at loads 50 to 87 the pair still fails -- AND THE SHAPE HAS CHANGED, which is the finding rather than a persistence. The delivered path is no longer a directory. It is `.prettierignore`, a root file, and the store's index is NOT empty (probed: 23 entries). So the reconcile is reporting a REAL difference: `.prettierignore` genuinely differs from what the store recorded, because `converge_formatter_exclusion` appends to it after the index was taken.

THE RULING I NEED, AND I AM NOT GUESSING IT: may a directory-granularity event publish a difference that PRE-DATES the event? My reconcile answers *what differs now*. The test's `feed.settle()` assumes *what changed since*. Both are defensible. If the answer is that it may not, the reconcile needs a baseline taken at settle rather than the store's index, and that is a different design from the one you ruled. Both assertions in the tests are correct as written and I have not touched them.

Also banked for you: a new dependency needs its rationale in the WORKSPACE manifest, not the crate's -- `dependency_rationale` reds three ways otherwise, and two of the three are its own controls failing. And `git stash` is shared across every worktree of one repo; a peer's WIP was in the stack when I used it for a control, and I will not use it again here.

Nothing landed in your dark window from me. NO RELEASE, NO PUSH.

## (2026-09-12 14:14Z)

dc banking and folding for hv's compact. **The quiet window's step 1 is DONE and reported; step 2, WP-22, is HALF BUILT AND NOT COMMITTED.** Durable copy because you are dark.

STEP 1, MEASURED AND ANSWERED. The single bounded-ingest arm alone six times: 0 red of 6 at load 14.8-15.7. The full daemon targets five times: 1 red of 5 at load 11.2-14.8, the same arm, under the parallel target run. Against load ~40 earlier today where the same arm alone red 3 of 6. **It is load, shown across the range this box can reach, and there is no change of character.**

AND THE WINDOW CANNOT DELIVER AN IDLE HOST. My script waited 600s for a one-minute load under 4 and gave up at 15.74. Not cc or ic, who held throughout: `fileproviderd` 101%, iTerm 44%, App Tamer 29%, three CoreSimulator processes. **The floor on this host is a one-minute load of 10 to 15 with every node silent**, so no measurement any of us took today was on an idle host, and a future "run it quiet" hits the same floor. The simulators are hv's to stop, not ours.

STEP 2, WP-22, WHERE IT STANDS. Banked at `scratchpad/wp22-BANKED.patch` (313 lines, four files) and live in `scratchpad/wt-dc` at `034bf8f54`. BUILT: `Op::Search` and `Response::Search`, the envelope's `Deserialize`, the roster CONSTRUCTOR you ruled, and the daemon handler calling the same `search_all` the daemonless path calls. NOT BUILT: `--no-reconcile` on the search row, the daemonless reconcile-then-query through `index_refresh(None)`, and the parity arm. **It does not land half-built: a declared `Op::Search` no daemon answered would be dead surface.**

THREE DECISIONS INSIDE IT, stated so they are not re-litigated. `Response::Search` carries `serde_json::Value` rather than the typed envelope, because a hit holds `score: f64`, `Response` derives `Eq`, and a raw float cannot satisfy `Eq` while `serde_json::Number` can -- the same reason `Response::Graphql` already carries a value. `IndexFreshness` gains a HAND-WRITTEN `Deserialize` that RECOMPUTES `complete` instead of reading it, so a peer cannot send `complete: true` beside a non-empty `stale`; the wire forced the question and the answer is stronger than what the type had before. And `Response::search` lives in `wire.rs` rather than in the daemon, because `intentd` has no `serde_json` and adding one would buy a new manifest dependency under AC-08.10 plus a second place deciding how the envelope becomes JSON.

NOTHING WAITING ON A RULING FROM YOU. Steps 3, 4 and 5 are as you ordered and I have them on my board with your one-re-run rule for the rehearsal recorded verbatim. NO RELEASE, NO PUSH.
