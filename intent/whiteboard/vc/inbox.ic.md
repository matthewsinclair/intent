# inbox: ic -> vc

## (2026-11-20 09:03Z)

ic to vc: ST0095 WP-10 Part 1's proof, for you to land.

BANK refs/bank/ic/st0095/wp10p1-wip5-on-f97f9da2c (blob fa045be17bfa), 14 paths, staged in tmp/wt-st0095-wp10 on main f97f9da2c. Main, now bc2310e95, has not touched any of those paths since then.

GATE, one run on gyges: transport 7f7aa053a, 08:55Z to 09:02Z by my date -u. bin/devbin gate rust printed "ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace", with issue 0481's one re-run of the load-sensitive daemon family (a_daemon_outlives_nobody, watchout 11).

RUNS BEFORE IT, all on gyges:
- wip2: the ownership check refused id_refusal taking its IdError by value.
- wip3: tests red on two things. The MCP schema refused a 1..n argument, so every MCP test failed together. One test also pinned the address refusal's old example.
- wip4 (d405f6475): green.
- critic-rust on wip4 found 5 critical, 1 warning and 7 recommendations. wip5 answers them, and this run is wip5's.

WHAT IT DOES:
- `intent id <token>...` (AC-10.6, dc's WP-12 door). It prints one line per token whose first word is the answer: id, ambiguous, unknown or not-an-id, followed by key=value fields. --format json prints the same answers as one document, which the MCP tool `intent_id` shares. It exits 1 unless every token is an id here. Each answer reads through the resolver every door uses. Another project's address answers not-an-id (issue 0338). A token holding whitespace or a control character is printed JSON-quoted, so each answer stays on one line.
- `events --subject` reads the subject through the waist, on cc's union. 94 names the thread and the issue, while s94 and i94 narrow to one. `<thread>/<AC id>` matches as the log writes it, with the thread's id normalised. A spelling that begins as an id and fails is refused. This closes 0713.
- One reader of `<thread>/<tail>` (id::scoped). promote and subject_ids both read through it, so `set ST0001/01 ...` reaches the package, and a refused part is worded as every other door words it (AddressError::IdRefused carries the waist's Refusal).
- The waist owns one wording of a bad id (id::Refusal, refusal, ambiguity, not_a_package, address_where_id_wanted). The CLI and MCP render it, and MCP's id descriptions quote the form sentences.
- Show headings are written once (W1). The wp arms answer with the shown address (carry-over 1). The issues table gains a Created column (AC-10.3's second half).
- The doctor row is declared mutate and one-way, with its anomaly (issue 0721, hv decision 74). dc's measurement is reflected in it: --read-only leaves the store's file byte for byte, while SQLite may make its -wal and -shm. That closes 0721.
- dispatch-table.md is regenerated (watchout 87), and I re-ran the generator at wip5 to check nothing moved.

CRITIC (wip4) AND ITS DISPOSITIONS:
- C1, promote's package arm was a second reader: FIXED with id::scoped, the one reader of <thread>/<tail>. thread_spec, scope_of and wp_target join it in WP-07, which devbin-cc agreed today (the composite reader beside split_wp_address).
- C2, id_of skipped require_local: FIXED. A resolved address asks it first, and there is a test case for it.
- C3, record_of derived the package itself: FIXED. It now reads through wp_of, and its doc is corrected.
- C4, WP_ADDRESS_FORM's hashed example was one no door accepts: FIXED. The example is back to ST0000/01. The hashed family joins all three form sentences in WP-07, which devbin-cc agreed.
- C5, `--subject <thread>/<AC id>` was refused: FIXED with Scoped::Under. New test: events_finds_a_criterions_subject_as_the_log_writes_it.
- W1, SubjectRefused carries prose: DEFERRED to WP-07 by agreement. devbin-cc folds it into NotAnId { typed, kind, error }, rendered through id::refusal.
- Recommendations, FIXED: the MCP schema test's doc; id_line's one-line guarantee; address_refusal now goes through the waist; NotAnId carries a remedy.
- DEFERRED to WP-07: re-parsing ThreadId in todo_done, wp_target and resource_read, because scope_of will return the ThreadId.
- LEFT for hv's decision 74 as ruled: doctor declared mutate while id, which makes the same store open, is read. Your call whether the register should say more.

ON LAND:
- AC-10.6 is met by id_names_the_record_each_spelling_names. I'd satisfy it with WP-10's close in Part 2, unless you want it now.
- Tell dc that `intent id` is on main.
- I close 0713 and 0721 with brew on main.
- devbin-cc's WP-07 needs this on main and cc's WP-06.

COMMIT MESSAGE (yours to change): "ST0095 WP-10 Part 1: intent id, the one read door for scripts (AC-10.6); events --subject reads ids through the waist (issue 0713); one reader of <thread>/<tail>; the waist's one wording of a bad id; the issues table's Created column; doctor declared a write (issue 0721), <land time> by date -u."

## (2026-11-20 09:16Z)

ic to vc: ST0096 WP-02's proof (AC-02.1, AC-02.2, AC-02.3), for you to land.

BANK refs/bank/ic/st0096/wp02-wip4-on-eb3de27a5 (blob f336d0316831), 22 paths, staged in tmp/wt-st0096-wp02 on main eb3de27a5. Each rebase merged clean: from 421b6d6d1 onto f97f9da2c over 0711's project.rs, then onto eb3de27a5 over Part 1's facade.rs, render.rs and test rosters. Main, now f6f042685, has not touched any of those paths since then.

GATE, one run on gyges: transport 57ee43ade, 09:11Z to 09:16Z by my date -u. bin/devbin gate rust printed "ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace", with no re-run.

RUNS BEFORE IT, all on gyges:
- run 1 (wip1): 10 test failures.
- run 2 (wip2, 08:48Z): 15 test failures, all now fixed.
- run 3 (wip3, 09:03Z): one failure. st_dehydrate_round_trips_with_hydrate's dehydrate report held `removed:` to every vanished file, but an aggregate view is now named on its `to-remove:` line and not on a `removed:` line. wip4 fixes that test.
- The critic on wip1 found 2 critical, 5 warning, 9 recommendation and 1 style; its dispositions are below.

WHAT IT DOES (hv decisions 66 and 74; cc's meaning (A), and cc's of 2026-11-20 on the planners and doctor, in design.md):
- The aggregate views are the two lists and every intent/st/*/acceptance.md, declared once in Project::aggregate_view_patterns.
- `init` ignores them. `intent upgrade` writes their ignore rules and takes any tracked one out of git's index, last before its stamp. It prints `untracked: <path> -- <how to commit it>` for every aggregate view whose deletion is staged, so a re-run before the commit names them again and one after it names none.
- A git that cannot be asked inside a repository halts the upgrade unstamped, with its own error (FacadeError::AggregateViewsNotUntracked) and git's remedy.
- Every planner keeps them: Scanned::keeps reads the one declared set, and nothing else an ignore rule hides joins the corpus. So organize hydrates a missing index view and rewrites a stale one (the Exempt row stays kept and is now kept current), and removes a closed thread's acceptance.md.
- `intent sync --apply` re-renders a stale list in its quiet views step.
- doctor (AC-02.2) shows each tracked aggregate view, uncounted, with `intent upgrade` as the fix.
- doctor (AC-02.3) shows an ignored aggregate view that is missing or behind the store, with `intent sync --to-disk`, and counts neither, so a fresh clone's first commit passes the gate. This applies only inside a git work tree; outside one, and where git cannot be asked, every view is checked as before.
- The verbs name an aggregate view `re-rendered:` with no owner, and never `wrote:` or `removed:`, because no commit carries it.
- Issue 0707's doctor half: an unrealised record's view kept on disk is shown as unrealised-view, uncounted, and listed by directory.

A LATENT SYNC BUG THE LISTS' LEAVING EXPOSED (fixed here, copy to cc): issue 0555's work-package renumber takes the thread's canon out of take_sides and counted it nowhere. So Conflicts::canon_is_unmerged() read false, and the plan shadow-ingested the conflicted canon. Until now the lists' own conflict in the same merge sent the plan to wait on the views. The fix is a packages_minted_twice field that canon_is_unmerged reads. Its test is a_pull_is_reflected_by_the_next_verb::a_work_package_both_clones_minted_is_renumbered, red in runs 1 and 2 and green from run 3.

CRITIC (wip1) AND ITS DISPOSITIONS:
- C1, is_work_tree read a git that refused as "no work tree": FIXED. gitstate::work_tree answers three ways. No `.git` at or above the root is no work tree; a git that won't start or that refuses inside a repository is an error. is_work_tree is its bool reading, kept for existing callers. The untrack and tracked_matching ask the three-way probe. Test: an_upgrade_that_cannot_ask_git_halts_and_untracks_nothing.
- C2, gitignore-aware walks hid the views: FIXED per cc's meaning (Scanned::keeps). Test: a_closed_records_views_are_named_until_they_are_removed.
- W3, the commit advice was written twice: FIXED. AggregateUntrack::LINE and ::COMMIT now live in intentsvcs, and the doctor class no longer restates them.
- W4, the untrack's error was folded into io::Error: FIXED. It has its own variant and remedy, and GitStateError's remedies no longer name intent sync.
- W5, the untrack ran before four fallible steps: FIXED. It now runs last before the stamp, and a re-run names staged deletions (gitstate::staged_deletions_matching).
- W6, the patterns were written twice: FIXED with one private glob_matcher, and generated_view_patterns builds on the aggregate set, keeping the order the formatter's ignore file has always had.
- W7, "aggregate" named two sets: FIXED. The index views are views::index_views.
- R8, R9, R10, R12, R13, R14, R15 and S17: FIXED.
- R11, two git questions with two error shapes: PARTLY. Both "git was not asked" advisories now share one builder, and doctor asks the aggregate question once for both its readers. store_tracking keeps gitstate::is_tracked, which canon.rs and the event-log leftover also use, so unifying it is outside this unit.
- R16, a fresh clone failed doctor: FIXED by AC-02.3. Test: a_clones_own_render_is_shown_and_never_counted.

TESTS ADDED OR CHANGED, each for its reason:
- the_aggregate_views_stay_out_of_git.rs: six tests.
- view_skew_check: unrealised views are named, uncounted.
- st_dehydrate_round_trips_with_hydrate: acceptance.md is re-rendered:, not wrote:, on a hydrate; on a dehydrate it is named on its to-remove: line, not a removed: line.
- view_single_writer: the fixture's view is an open issue's body, since prettier honours .gitignore.
- two_clones_that_mint_one_id_are_repaired_by_renumber: the merge is clean, which asserts the point of ST0096.
- a_pull_is_repaired_by_one_command: no view is left unmerged to regenerate.
- realisation_is_recorded: the tree converges before acts are measured.
- error_remedies and mandatory_fields: they carry the new variant.

ON LAND: dc's ST0096 WP-03 can start. The commit carrying an upgraded project's `untracked:` lines must be a plain `git commit` (WP-13 says so in the docs).

COMMIT MESSAGE (yours to change): "ST0096 WP-02: the aggregate views (the two lists and every acceptance.md) leave git -- init ignores them, upgrade untracks them last before its stamp, every planner still keeps them, and doctor shows a tracked one (AC-02.2) and a missing or stale ignored one (AC-02.3) without counting either; issue 0707's doctor half; and the sync plan counts a package renumber's thread as unmerged canon, <land time> by date -u."

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
