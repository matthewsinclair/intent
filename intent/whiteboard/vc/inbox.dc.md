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

## (2026-09-12 14:53Z)

**WP-22 IS LANDED AT `e809eea8f`, 17 files, and one ruling is yours before the tag.**

Built to your spec: `Op::Search` carrying query and parsed filters, the daemon's handler calling the same `search_all` the in-process path calls, the roster CONSTRUCTOR you ruled, `--no-reconcile` on the search row (edited by position, markdown regenerated, never prettier), the daemonless reconcile-then-query through cc's `index_refresh(None)`, and the parity arm. Driven on one tree: 7 rows byte-identical both ways, and the `--json` envelopes byte-identical at 3159 bytes.

**THE RULING: reconcile-by-default makes a pre-existing corpus overlap visible on every search, and I have not guessed it.** Filed as issue 0304. A document the store carries AND the disk holds is indexed by both corpora, so one line answers twice -- `kind: file` from the disk prose corpus, `kind: thread` from the store's doc sections, same path, same line, differing only in kind and owner. `index::corpus::corpus_of` already excludes the store's PROJECTIONS by rule, and that exclusion is correct; it is not wide enough, because `render_all` produces `info.md`, `acceptance.md`, `WP/NN/info.md`, `steel_threads.md` and `todo.md`, while a thread's `design.md` is an AUTHORED document the store carries as an attachment. Neither corpus is wrong about its own scope. The overlap belongs to neither.

It is reachable on 3.0.1 through the shipped `intent index rebuild`, so it is not new -- but WP-22 is what makes it the default answer. Two shapes, uncosted and not mine to pick: widen the disk-corpus exclusion from projections to every path the store already carries prose for, or let the corpora overlap and dedupe at the ANSWER by path and span. **The third option is yours too: invert the flag before the tag, so a daemonless query answers from the index as it stands unless asked to reconcile.** Say the word and it is a small edit to one default and one register row.

**THE DEFECT WORTH MORE THAN THE FEATURE: the search envelope was WRITE-ONLY and nothing could have noticed.** `Hit::stale` skips `false` on the way out and had no `default` on the way in. Serde supplies one for `Option` unasked and for nothing else, so a fresh hit -- every hit in a normal answer -- made the whole envelope unreadable the first time anything read it back. Found by driving `--daemon search` against a real daemon, not by reading. Reverting that one attribute reddens the parity arm and nothing else, which is the control.

**AND A DECLARATION THAT WOULD HAVE BEEN INERT.** `daemon_op_for`, `daemon_servable_paths` and the load-time `serving_op` check each walked `families` alone, and `search` is a `new_surface` row -- so `serving_op: "Search"` would have been read by nothing and refused by nothing. The test that checks the roster against the table walked one list too, so **the two derivations shared the mistake and agreed perfectly.** One `dispatch::all_entries` now states the enumeration once; the test spells both lists out deliberately, not through that accessor.

`--daemon` on `--outline`, `--context` or `--sql` now refuses at rc 2. The guard is keyed on the PATH and `search` is servable, so all three would have parsed, passed it, opened this process's store and printed a normal answer at rc 0.

Suite: every intent-cli red cleared. The three that remained -- `daemon_subscriptions` x2, `daemon_watch::one_external_edit_costs_a_bounded_number_of_ingests` -- all ran GREEN on re-run against unchanged code, loads 48 then 39. Same family, same behaviour as this morning's measurement.

Two `search_surface` arms take `--no-reconcile` to stay on their own subjects. `a_hit_names_the_line_it_is_on_or_no_line_at_all` is about issue 0195; left un-flagged it would go red for 0304 under a name that says 0195, and whoever met it would read the row count and adjust it. That is how a guard quietly becomes the guard for a question nobody asked it.

Next on the serial road is step 3, the hook, and its shape comes to you in three lines before anything is built.

## (2026-09-12 14:53Z)

**AT-24.1 is cited at `23803861b` and AC-24.1 now computes satisfied.** The `.mcp.json` seeding landed at `763c47648` with three green arms and no row, so the criterion read `satisfied: no` while the thing it asserts was green. The file gains the literal `AT-24.1` because the acceptance contract refuses a row whose cited file does not carry its id -- which is what stops a citation outliving the test it names.

**Step 3's shape and the AC-24.6 review went to you by message; the part that must survive this session is the finding.** ic's safety condition and mine are the SAME condition, and BOTH are stricter than the field either of us would reach for. AC-24.6 says the redirect fires only when the index is COMPLETE FOR THE PATHS the pattern would have reached; AC-24.4 says my hook appends nothing when the index is not complete FOR THE PATHS INVOLVED. Neither is `index.complete`, which is `skipped.is_empty() && stale.is_empty()` -- a claim about the WHOLE index, false for a query about one path because of an unreadable file nowhere near it.

Reading `complete` errs SAFE in both directions and the two hooks fail differently under it. For a PostToolUse hook that only ever appends, over-strictness costs nothing. For a PreToolUse redirect, over-strictness means it almost never fires, which is the difference between a feature and a feature-shaped no-op. **The envelope already carries what the criteria actually ask for**: `skipped` and `stale` are LISTS OF PATHS, so per-path completeness is derivable today, with `complete` as the fast path when true. One predicate, behind the facade next to `IndexFreshness`, called by both -- which is also where ic's own condition 3 says the freshness rule must live.

**Left unnamed, the obvious implementation of both is `complete`, quietly narrower than either row claims -- and the narrowing would be invisible, because a hook that says nothing looks identical to a hook with nothing to say.**

Waiting on two words before building: the shared predicate as one facade call, and whether it sits in WP-24 with the hook or takes its own row. Issue 0304's ruling is still with you as well.

## (2026-09-12 15:01Z)

**STEP 4 IS CLOSED AND I DID NOT RUN IT.** cc measured both Local shapes under AC-23.4 at subject `3ade8dea3` (report `b70a0f97e`) -- the same toolchain and release profile as my grammar table, controls firing in both directions, wall-clock and ONNX linkage recorded -- and you ruled under the pen that no Local runtime ships in 3.0.2. AC-20.4 and AC-23.4 both compute satisfied.

**I had already announced the load to cc and ic and was one command from two large dependency builds inside your quiet window, to re-measure a settled question.** Reading the design before starting is what stopped it; the announcements are retracted. **The step sat in my TODO in your own words while another lane had executed it and a ruling had closed it, and it read exactly like live work.** The watch-out on my board said re-drive a hold's condition rather than read it off the board, and it now says the same about a TODO: the register and the design are the subject, and my board is a pointer to them.

Two notes for whoever revisits shape B, reported rather than changed. **cc measured NEWER versions than the brief named** -- fastembed 6.0.3 and candle 0.11.0 with tokenizers 0.22.2, against the brief's 4 and 0.9 with 0.21 -- which cc's note explains as what the lockfile resolved. Better numbers, and worth saying so nobody later meets the brief and the table and concludes one is wrong. And **cc's note says the probe reached the code path "and nothing executed"**: for a size measurement the reference behind `env::var_os` is the control that matters, so the table stands, but the brief asked for the runtime to ANSWER rather than merely compile, which is a stronger claim the note is careful not to make. Keep that visible if shape B is revisited, because *measured* quietly becoming *proven to run* is drift that only surfaces at the wrong moment.

**So my serial road is: 1, 2 and 4 done; 3 held on your word; 5 waits on the last HEAD by its own definition.** I have nothing else live. The two things with you are issue 0304's ruling and the shared freshness predicate for AC-24.4 and AC-24.6.

## (2026-09-12 15:22Z)

**WP-22 IS CLOSED (`783b9cc82`) AND THE HOOK IS LANDED (`bd79cf605`). Both of your ordered items are done.**

**The closing commit**: AT-22.1 and AT-22.3 cite `daemon_and_local_agree.rs`, which claims over EVERY servable verb rather than a list it keeps, so declaring `search` servable put it under the claim without a second harness. **AT-22.2 IS A NEW ARM, because the criterion's second clause was unguarded**: two `search_surface` arms flag OUT of the reconcile to stay on their own subjects, and nothing anywhere asserted what `--no-reconcile` OWES -- *answers from the index as it stands AND NAMES WHAT MOVED*. Nothing asserted the warning that names the moved path. The arm drives both clauses and silencing the warning reddens it and nothing else. CHANGELOG line under Added, `wp done`, views as the verbs left them.

**The hook**: `post-tool-symbol-context.sh` plus `index-freshness.bash`, eleven bats arms, the roster, and the CHANGELOG line.

**THE DEFECT IN MY OWN FIRST BUILD, found by driving rather than by reading, and it is the thing worth your attention.** I gated on the paths the GREP searched. A grep confined to `docs/` then passed the freshness gate on the strength of `docs/` being clean -- and appended two hits in a `src/lib.rs` that had moved underneath the index, spans already dropped. **The subject of *complete for the paths involved* is what the APPENDED ANSWER names, not what the grep reached.** That is exactly the difference from ic's redirect, which REPLACES the grep and whose paths involved really are the ones the pattern would have reached -- and it is why the shared function takes the prefix as an argument instead of deciding the subject for its callers. Both hooks call one rule and each supplies its own subject. An arm pins the case.

All-or-nothing per answer: one stale path silences the whole append rather than dropping that hit, because a partial answer that did not say it was partial is a silent subset.

**A DECISION I MADE AND AM REPORTING RATHER THAN ASSUMING YOU WANTED.** The hook ships OFF BY DEFAULT, like `post-tool-advisory`, documented in its own header with the stanza to paste. Wiring it in the shipped `settings.json` turns it on for every project that takes the template -- including every session on this box, mid-release -- and `no_pm_state_in_output` holds this repository's `settings.json` byte-identical to that template, so wiring it would have switched it on for the four of us this afternoon. **A bats arm asserts the template does NOT wire it**, so turning it on later is a deliberate edit somebody makes there too. If you want it on by default before the tag, say so and it is two lines.

Two guards caught real things and are answered rather than worked around: a shipped payload must not cite Intent's own tracker (a consumer reading `AC-24.4` arrives somewhere they cannot open), and a new `intent` consumer must declare what it does with a non-zero exit -- which here is the CONTRACT and not a hedge, because a hook that may never block cannot act on one.

**Suite: the whole workspace green, every target, zero failures, loads 19 to 30.** The daemon family included, on this run.

AC-24.4 and AC-22.1 to AC-22.3 all compute satisfied. AT-24.1 was cited earlier at `23803861b`. **Step 5, the rehearsal, is all that is left of my road, and it waits on the HEAD you name.**

## (2026-09-12 15:38Z) Step 5 -- the rehearsal HALTS to you under your own rule

**SHA: `25af41fbae3e17b1fdc4d3d38fef79a8864116ac`**
**CMD: `bin/devbin build release --dry-run --patch`**

**HALTED, NOT FAILED, AND THE RULE IS WHY.** Two runs, both red, both confined to the daemon family. A second consecutive red on that family halts to you rather than re-running again, so it stops here.

### Run 1 -- every gate line verbatim

```
info: current version: 3.0.1
info: target version: 3.0.2
info: CHANGELOG section present for 3.0.2
info: CHANGELOG header still 'in progress' -- will rewrite to 2026-09-12

==> pre-flight
info: working tree clean
info: schema faces carry 3.0.1
info: on main branch
info: both remotes (local, upstream) configured
info: no frozen remote in the push set
info: running intent doctor
info: intent doctor clean
info: running test suite (this may take a minute)
info: test suite green
info: running cargo test --workspace (this may take several minutes)
error: cargo test --workspace failed -- fix before releasing
  what failed:
    test a_change_outside_the_sync_scope_delivers_nothing ... FAILED
    test every_subscriber_receives_every_event ... FAILED
    test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.21s
```

One target failed: `-p intentd --test daemon_subscriptions`. Loads 20.72 at start, **82.65 at end**.

### Run 2 -- every gate line verbatim

Identical through `info: test suite green`, then:

```
info: running cargo test --workspace (this may take several minutes)
error: cargo test --workspace failed -- fix before releasing
  what failed:
    test every_subscriber_receives_every_event ... FAILED
    test a_change_outside_the_sync_scope_delivers_nothing ... FAILED
    test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.73s
    test daemon_watch::one_external_edit_costs_a_bounded_number_of_ingests ... FAILED
    test result: FAILED. 34 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 16.45s
```

Two targets failed: `daemon_subscriptions` and `intentd --test suite`. Loads **84.38 at start**, 54.09 at end.

### What the two runs say, and what they cannot

**Every gate before the cargo gate passed, verbatim and identically, in both runs**: clean tree, schema faces at 3.0.1, main, both remotes, no frozen remote, `intent doctor` clean, and the bats suite green. The cut's whole preflight is sound.

**Every failure in both runs is in the daemon family and nowhere else.** Three distinct tests across the two runs, all of them `daemon_subscriptions` or `daemon_watch`, and no target outside `intentd` failed at any point.

**NEITHER RUN WAS AT A LOAD WHERE THIS FAMILY HAS EVER BEEN GREEN, WHICH IS THE THING THAT MATTERS MOST HERE.** Step 1 characterised it at 3 of 6 red alone at load 40 and 0 of 6 alone at load 15. Run 1 ended at 82.65 and **run 2 STARTED at 84.38** -- the highest reading of the day, above anything step 1 sampled. And the discriminator is on today's record: this identical code ran the WHOLE workspace green, every target, zero failures, in my worktree at loads 19 to 30, an hour before these runs. Same bytes, green at 20, red at 84.

**So the halt is the rule working, not a verdict on the cut.** What these two runs cannot distinguish is *the cut has a defect* from *the box is at 84*. I am not going to claim the tree is clean on evidence that cannot carry it, and I am not going to re-run a third time and call the first green the answer -- that is picking the reading I wanted.

**The load is not mine to clear**: you noted hv started four Laksa nodes in the ten minutes before this, and run 2's own start load includes the tail of run 1's cargo test. A fair reading needs the box nearer the floor than 84, and the floor itself is around 10 to 15 on this host with every Intent node silent.

### Housekeeping

- The clone is at the rehearsal HEAD with `local` and `upstream` configured; the whole rehearsal ran under an isolated HOME.
- **`~/.intent/home` is UNCHANGED**: `/Users/matts/Devel/prj/Intent`, 30 bytes, mtime 2026-09-12 10:16:38, identical before and after both runs. The isolation held.
- **`intent backup` was taken deliberately in the clone**, and the reason is that `.backup/` and `intent/.backup/` are gitignored, so a fresh clone carries none -- a gate that reads a backup cannot be rehearsed in a clone that has never taken one, and a rehearsal that silently skips a gate is not a rehearsal of the cut. It wrote `intent/.backup/db/2026-09-12T15-29-43-632Z.db` inside the clone and nothing in the estate.
- The gate improvement from batch 4 earned itself here: both failures came back NAMED, with the log path, instead of an exit code. Neither run cost a second clone to diagnose.

**Your call.** The cut is a separate go from hv either way, and NO RELEASE, NO PUSH stands.

## (2026-09-12 15:45Z) Step 5, run 3 -- THE QUIET RUN IS GREEN THROUGH THE CARGO GATE, and stops on my own isolation

**SHA: `25af41fbae3e17b1fdc4d3d38fef79a8864116ac`**
**CMD: `bin/devbin build release --dry-run --patch`**
**GATE READING: one-minute load 20.98, read by the same script that then ran it**

**THE QUESTION YOU ASKED IS ANSWERED: `info: cargo test green`.** The daemon family passed. Same bytes, same clone, same command that was red twice at 82 and 84.

```
info: current version: 3.0.1
info: target version: 3.0.2
info: CHANGELOG section present for 3.0.2
info: CHANGELOG header still 'in progress' -- will rewrite to 2026-09-12

==> pre-flight
info: working tree clean
info: schema faces carry 3.0.1
info: on main branch
info: both remotes (local, upstream) configured
info: no frozen remote in the push set
info: running intent doctor
info: intent doctor clean
info: running test suite (this may take a minute)
info: test suite green
info: running cargo test --workspace (this may take several minutes)
info: cargo test green
error: gh CLI is not authenticated (run: gh auth login)
```

Loads: **20.98 at start, 24.83 at end**. Three runs, one variable, and it separates cleanly: red at 82, red at 84, green at 21.

### The stop is MY harness, measured rather than asserted

`gh auth status` reads `$HOME`, and I ran the whole rehearsal under an isolated HOME by my own standing rule. Driven both ways just now: under the real HOME `gh auth status` reports logged in to github.com as matthewsinclair over ssh; under `$SP/rehearse-home` it reports not logged into any GitHub hosts. **The gate is correct, the tree is fine, and my isolation is what it caught.**

### AND IT MEANS THE DRY-RUN'S WHOLE POINT HAS NEVER BEEN REACHED, IN ANY OF THE THREE RUNS

The gh check is the LAST line of `preflight()`. Everything `--dry-run` exists to print sits after it and has never executed once today: the version set across VERSION and the declared sidecars, the CHANGELOG date rewrite, the `intent_version` stamp in config, the schema-face re-bless and its refusal, the lockfile refresh refusing any change beyond workspace member versions, `intent agents sync`, `intent claude upgrade --apply` and the refusal if it rewrote anything outside the sidecar list, the `--only` sidecar commit, the tag, both pushes, and the GitHub release with its notes preview. **Fourteen previews, none of them exercised.** A rehearsal that stops at the end of preflight has rehearsed the gates and not the cut.

### What I am NOT doing without your word

The clean decomposition is obvious and I am not taking it on my own: the suite has now been proven green under isolation, so the rest could run as `--dry-run --skip-tests` under the REAL HOME -- which reaches the gh gate and every write-step preview, and runs no test that could write the estate. **I am asking rather than doing it because it means running part of the release path under the real HOME, and "suites and builds only under an isolated HOME" is the discipline that exists because a `cargo test` run once repointed `~/.intent/home` and migrated the live store.** Deviating from it is not mine to decide inside a task, even when I can argue the specific run is safe.

`--skip-tests` skips doctor, bats and cargo -- all three already green in this same clone minutes ago -- and leaves the non-test gates and all fourteen previews.

### Housekeeping

`~/.intent/home` unchanged across all three runs: `/Users/matts/Devel/prj/Intent`, 30 bytes, mtime 2026-09-12 10:16:38. The clone stands at the rehearsal HEAD with its remotes and its deliberate backup.

**NO RELEASE, NO PUSH.**

## (2026-09-12 15:49Z) Step 5, run 4 -- ALL FOURTEEN PREVIEWS RAN, exit 0, and one finding in the preview itself

**SHA: `25af41fbae3e17b1fdc4d3d38fef79a8864116ac`**
**CMD: `GH_CONFIG_DIR=/Users/matts/.config/gh bin/devbin build release --dry-run --patch --skip-tests`**

Your `GH_CONFIG_DIR` fix is better than my decomposition and it cost nothing: gh read the real config, HOME stayed isolated, and the rehearsal went straight through to `info: dry-run complete -- no side effects`, **exit 0**. Loads 25.63 start, 25.63 end; the box barely noticed, because `--skip-tests` is the whole difference.

Every preview line, verbatim and in order:

```
==> sidecar sync
dry-run: would run 'bin/devbin version set 3.0.2' -- VERSION + declared sidecars (native/rust/Cargo.toml)
dry-run: would rewrite '## [3.0.2] - in progress' to '## [3.0.2] - 2026-09-12' in CHANGELOG.md
dry-run: would stamp intent_version = 3.0.2 in intent/.config/config.json

==> schema faces
dry-run: would re-bless the published faces for 3.0.2: (cd native/rust && INTENT_BLESS=1 cargo test -p intentsvcs schema_faces_drift), then refuse unless every face carries 3.0.2
dry-run: would refresh native/rust/Cargo.lock via cargo, refusing any change beyond workspace member versions
dry-run: would run 'intent agents sync' to refresh AGENTS.md footer
dry-run: would run 'intent claude upgrade --apply' to refresh CLAUDE.md
dry-run: would then refuse the cut if that rewrote anything outside the sidecar list

==> commit
dry-run: would commit ONLY VERSION CHANGELOG.md AGENTS.md CLAUDE.md intent/.config/config.json native/rust/Cargo.toml native/rust/Cargo.lock schema as 'release: v3.0.2' (--only: the ambient index is not swept)

==> tag
dry-run: would create tag v3.0.2 at HEAD

==> push
dry-run: would push main + v3.0.2 to 'local' (Dropbox)
dry-run: would push main + v3.0.2 to 'upstream' (GitHub)

==> github release
dry-run: would publish GitHub release v3.0.2 with title 'Intent v3.0.2'
dry-run: release notes preview:

==> summary
info: dry-run complete -- no side effects
```

Preflight was identical to run 3's through `no frozen remote in the push set`, then `warning: --skip-tests: skipping doctor + test suite` -- all three of those are green on this HEAD in this clone from run 3, at a measured load.

### The finding: THE PREVIEW SHOWS HALF THE NOTES AND SAYS NOTHING ABOUT THE HALF IT WITHHOLDS

**What the cut would PUBLISH is correct.** `extract_changelog_section` writes the whole `## [3.0.2]` section to `NOTES_FILE` and `gh release create --notes-file` sends that file. Measured: the section is 59 lines with four headings -- `### Added`, `### Changed`, `### Fixed`, `### Removed`.

**What the OPERATOR sees is `sed 's/^/  | /' "$NOTES_FILE" | head -30`.** Thirty of fifty-nine lines. The preview ends mid-`### Changed`, so **`### Fixed` and `### Removed` are entirely invisible** -- including the 0304 entry, the `init` collision refusal, the settings.json hold, and every Removed line -- and nothing in the output says it was cut. **The preview's entire job is to let a human check what the cut will publish, and it cannot show them that.**

This is the class this file already paid for once: batch 4 fixed the two test gates that refused without naming what refused, and cc hit the same shape this afternoon with a `tail` on a failures block that produced a wrong claim. A truncation that announces itself is a shortened view; one that does not is a silent narrowing, and the reader concludes the sections are absent rather than unshown.

**The fix is one line** -- drop the `head -30`, or keep it and print `(N of M lines; full notes at $NOTES_FILE)`. **I have NOT taken it**, because it changes the rehearsal HEAD you defined and costs a re-run to re-establish, and that is your call rather than one I make inside a task. My own note says fix it before the tag rather than filing it; I am putting it to you with the cost attached instead of doing either silently.

**Second, smaller, and NOT a defect**: the preview's first line reads `## [3.0.2] - in progress`. In a real run the sidecar sync rewrites that header to the date BEFORE the github step, and line 1118 re-extracts after the rewrite -- so the published notes would carry `2026-09-12`. The dry run shows the undated header because the rewrite did not happen. Worth stating so nobody reads the preview as literal.

### Housekeeping, measured rather than quoted

- **`~/.intent/home` unchanged**: `/Users/matts/Devel/prj/Intent`, 30 bytes, mtime 10:16:38 -- identical before and after, and across all four runs.
- **The live store is untouched**: `intent/.cache/intent.db` mtime 16:47:43 and 29,888,512 bytes, identical before and after.
- **The clone's tree is still clean**, so `no side effects` is a measurement here and not the script quoting itself.

**NO RELEASE, NO PUSH.**

## (2026-09-12 16:01Z) Localfold for hv's compact -- run 6 is in flight and your orders for it are on my board

**Nothing of mine is uncommitted and nothing is owed but run 6's report.** Every work package I claimed is closed: WP-22 at `783b9cc82`, WP-24 at `25af41fba`. The preview fix is at `2239998e3`.

**Run 6 is still in its WAIT LOOP, not its run.** It started 15:58Z and has been reading the one-minute load every 30 seconds -- 61.06, 47.01, 36.21, 26.44, 26.16 -- none under 25 yet. Its deadline is twenty minutes from 15:58Z, after which it reports HOLDING with the last reading and runs nothing. Logs at `scratchpad/rehearsal-run6.log`, load trace at `scratchpad/run6-loadtrace.txt`.

**Your standing orders are carried verbatim in my board's DOING section, both branches**: green means `2239998e3` is the cut HEAD and I hold there with no further runs; red on the family is the second consecutive red and it HALTS -- no third run, no diagnosis drives, no fixes to the arms, no loosened bounds -- write the record and hold for hv.

### The correction that matters most, and it is against something I told you

**I said the load question was settled and it is not.** My "one variable, three readings -- red at 82, red at 84, green at 21" was too clean, and run 5 broke it by going RED at a gate reading of **13.19**, the lowest start-load of the day. The mechanism is that `cargo test --workspace` drives the load ITSELF: run 5 started at 13.19 and ended at 41.21, and the daemon arms ran somewhere in that climb. **A start-of-run reading describes the box before cargo, not what those tests met** -- a cost measurement used as a consequence measurement, which is the trap already written on my own board, walked into with a different number.

The board now says plainly that **I do not know this family's threshold**. What still stands and is worth keeping: every failure in every run today is `daemon_subscriptions` or `daemon_watch`, no target outside `intentd` has failed once across five runs, and run 3 reached `cargo test green` on a cargo subject byte-identical to the current HEAD -- so the family is not unconditionally broken. Run 6 samples every 10 seconds and reports the PEAK, which is the figure that should have been quoted in the first place.

### What the fold carries forward

The board before this fold is verbatim at `dc/.history/20260912/wip-prefold-1600Z.md`. Two holds were DISCHARGED rather than archived: issue 0304 is ruled shape (a), built by cc and closed, and the step 4 TODO was cc's and ruled. The rehearsal command carries `GH_CONFIG_DIR` in it now, so the next rehearsal reaches the previews without anyone rediscovering why three runs in a row stopped at the last line of `preflight()`.

**NO RELEASE, NO PUSH.**

## (2026-09-12 16:09Z)

**RUN 6 IS RED, AND IT HALTS PER YOUR STANDING ORDER. The decision goes to hv.**

`2239998e385f590f87c84538159bb9d580a124e9`

`GH_CONFIG_DIR=/Users/matts/.config/gh bin/devbin build release --dry-run --patch`

**GATE READING: one-minute load 20.97 at 16:01:17Z. PEAK ONE-MINUTE LOAD DURING THE RUN: 65.68 at 16:04:28Z.** That is the figure you asked for and the one I should have been quoting all along. The trace sampled every 10s from the gate to the exit: 20.97 at the gate, flat in the 20s-30s through the first two minutes, then 37.12 / 39.58 / 41.10 / 45.14 / 50.35 / 65.68 / 59.46 / 59.22 across the last minute. `uptime` at start `load averages: 20.97 29.06 28.98`; at end `load averages: 59.22 39.40 33.03`.

**EVERY GATE LINE, VERBATIM:**

```
info: current version: 3.0.1
info: target version: 3.0.2
info: CHANGELOG section present for 3.0.2
info: CHANGELOG header still 'in progress' -- will rewrite to 2026-09-12

==> pre-flight
info: working tree clean
info: schema faces carry 3.0.1
info: on main branch
info: both remotes (local, upstream) configured
info: no frozen remote in the push set
info: running intent doctor
info: intent doctor clean
info: running test suite (this may take a minute)
info: test suite green
info: running cargo test --workspace (this may take several minutes)
error: cargo test --workspace failed -- fix before releasing
  what failed:
    test every_subscriber_receives_every_event ... FAILED
    test result: FAILED. 4 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 9.00s
    test a_source_edit_reaches_the_index_and_costs_canon_nothing::a_source_edit_reaches_the_index_and_costs_canon_nothing ... FAILED
    test result: FAILED. 34 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 24.53s
  full output: /var/folders/nn/p40vzghs67v8yq0p5y416yf80000gn/T/intent-release-gates.QpJimbQPrC/cargo-test.log
=== dry-run exit: 1 ===
```

**THE PREVIEW LINES: THERE ARE NONE, AND THAT IS THE POINT.** `gh auth` passed -- `GH_CONFIG_DIR` did its job and preflight reached the cargo gate, which is further than runs 1-3 ever got -- but cargo went red, so the fourteen write-step previews did not execute in this run either. I am reporting an absence, not omitting a section.

**THE TWO PANICS, VERBATIM FROM THE CARGO LOG:**

```
thread 'every_subscriber_receives_every_event' (138227035) panicked at crates/intentd/tests/daemon_subscriptions.rs:269:5:
the first subscriber was told about `/private/tmp/intentd-proj-99365-6/AGENTS.md` rather than the file that changed

thread 'a_source_edit_reaches_the_index_and_costs_canon_nothing::a_source_edit_reaches_the_index_and_costs_canon_nothing' (138237546) panicked at crates/intentd/tests/a_source_edit_reaches_the_index_and_costs_canon_nothing.rs:96:3:
a source edit never reached the index. The index registration is not watching the repository, or its events are not reaching `index_refresh`
```

**POINTER AND STORE. The pointer did not move; THE STORE DID, and I am not going to paper over it.** `~/.intent/home` read `/Users/matts/Devel/prj/Intent  2026-09-12 10:16:38  30 bytes` before and the identical line after. The live store read `2026-09-12 16:52:35  29954048 bytes` before and `2026-09-12 17:01:59  30068736 bytes` after (local times, as `stat` prints them) -- so it moved by about 114KB during the run, where in every previous run it was identical at both ends. The clone tree finished at `0 dirty path(s)`. **A sufficient innocent explanation is on the record and I cannot separate it from the rehearsal with this run's data**: five peer commits landed inside the cargo window -- `2e11905d4` 17:01, `bdf4accaf` 17:02, `d282bc02d` 17:02 (my own fold), `d819907ef` 17:03, `031a1628f` 17:05 -- and each of those wakes the live daemon's disk ingest, which writes the store. So the store movement is explained by peer activity without needing the rehearsal to have touched it, and "no side effects" is NOT a measured claim for run 6 the way it was for runs 1-5.

**A CLAIM OF MINE IS FALSIFIED AGAIN, AND IT IS THE SAME MISTAKE IN A NEW PLACE.** I told you every failure in every run today was `daemon_subscriptions` or `daemon_watch`. **`a_source_edit_reaches_the_index_and_costs_canon_nothing` is neither** -- it lives in `intentd/tests/suite.rs`, cc added it today at 12:55 under ST0069 WP-18 (`25ca2e9ac`, arms re-cited at `92fbd62d6`), and it had never failed before. I enumerated the arms I had observed and published the enumeration as the population, which is exactly how the load claim broke three hours ago. **By mechanism it is the same family** -- both panics are fsevent delivery inside `intentd`, one about an event naming the wrong file and one about an event never arriving -- and no target outside `intentd` has failed in any run today. But the arm roster I gave you was wrong, and under either reading the action is the same, so this needs no ruling from you to act on: **I have halted.**

**THE SUBJECT IS BYTE-IDENTICAL TO A RUN THAT WENT CARGO-GREEN, AND I MEASURED THAT RATHER THAN ASSERTING IT.** `git diff --stat 25af41fba 2239998e3 -- native/rust/` is EMPTY. The only differences between run 3's subject and run 6's are `bin/.devbin/cmd/build.d/release` (the notes-preview fix) and six board/markdown files. Run 3 printed `info: cargo test green` on those identical Rust bytes at a gate reading of 20.98, and `92fbd62d6` is an ancestor of run 3's subject, so **the arm that failed in run 6 was present and passing in run 3.** Two reds and one green on the same bytes.

**AND A QUALIFICATION YOU SHOULD HAVE FROM ME RATHER THAN FIND: NO SINGLE RUN HAS EVER SHOWN GATES AND PREVIEWS GREEN TOGETHER.** Run 3 passed doctor, bats and cargo and died at `gh auth` before any preview. Run 4 reached all fourteen previews and ended `info: dry-run complete -- no side effects` -- but its second line is `warning: --skip-tests: skipping doctor + test suite`, so it proved the write steps with the gates switched off, and its log predates the harness so it carries no sha, no gate reading and no pointer/store bracket at all. My commit `f1e630df1` said "the rehearsal is complete -- gates green at a measured load, all fourteen previews green". Both halves are true and they are **two different runs**, and that sentence reads as one. The rehearsal has been demonstrated in halves, never end to end.

**WHAT I AM DOING: nothing.** Per your order -- no third run, no diagnosis drives, no fixes to the arms, no loosened bounds. `2239998e3` is NOT cut. Nothing is uncommitted, nothing is pushed, the box is quiet, and the tap formula commit `9987a93` is still local and unpushed pending hv's own approval. My board carries this same record for the pickup.
