# Intent -- traps and conventions

**THIS FILE CARRIES RULES, NOT HISTORY (hv, 2026-08-30), AND NOT STATE.** What nothing else enforces, one line each; anything held by a test, a guard, a generator or a shared memory has no prose home here. Earlier versions are in git and under `intent/history/`. Live work is `intent/wip.md`; the commands that regenerate any state are in `.claude/restart.md`; hv's rulings are `intent/history/20260915-hv-rulings.md` and hv's board. **NO RELEASE, NO PUSH without hv at the terminal; never `--no-confirm`; never `intent fc`.**

## Where you are standing

- Measure the binary, never trust a line: `command -v intent && readlink "$(command -v intent)"`, `intent --version`, `intent info | sed -n 's/^ *INTENT_HOME: *//p'`.
- The frozen v2 checkout is `~/Devel/prj/_Archive/Intentv2`, branch `v2-maintenance`, never written; `~/Devel/prj/Intentv2` does not exist.
- A build is the delivery: `bin/devbin build all` verifies the pair as a set and the PATH symlinks point into this tree; a bare `cargo build --release` is not the door. Templates are NOT compiled in: every `lib/templates/` file is read from the install root at run time, so an edit there is live on save.
- A rebuild never leaves an estate without `intent`: `build all` builds in `target/staging/release` and promotes the verified pair by rename (0196), so it may overlap a compact, and the running daemon keeps the old code until `intent daemon restart`. `bin/devbin fullcycle` is what removes `native/rust/target`. The rebuilding node announces the start and the end.
- The marker is provenance, not identity, and answers for the compiled half only.
- A general policy stated after a specific ruling does not vacate it: `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31).
- Strike "not in this one", "defer" and "after the tag" as a class; they are the scarcity move wearing a release number.
- Monikers are estate-scoped: write `laksa-cc`, never `cc`, when a moniker crosses an estate; a machine-wide `ps` shows every estate's `intent`.
- `ListAgents`' started column is socket age; `/compact` does not rotate `CLAUDE_CODE_SESSION_ID`.

## Landing work

- A bank is a patch blob on `refs/bank/<node>/<issue>` with its base named, built from `git diff --cached --binary` or write-tree and commit-tree, never from a worktree commit. Every bank report quotes `git cat-file -p <ref> | git apply --stat` beside the blob hash: the blob is what gets judged, and only reading it back shows which commit it holds. The policy is `docs/banking.md`.
- One lane lands at a time on vc's word, rebased onto the hash vc sends; the judged patch-id is what lands, and a gate refusal is re-banked before any commit.
- Banks sharing a file land in the order they bank green, stacked so one whole-suite run stands for the train; vc judges the logs, the diff, and that the worktree equals the bank.
- A diff's shape is not evidence of its reach: any change under a crate, the CI configuration or a generator's source owes the whole crate suites and the shell half. The one bounded case is a change confined to one crate's `tests/`, which owes that crate's whole suite.
- Every Rust bank runs intentsvcs's whole suite whatever crate it touches: the workspace scanners live there and read every crate.
- A vendored devbin upgrade is judged by a macOS bats run before a cut; CI's Linux leg cannot see the `/bin/bash` 3.2 floor.
- A timing-sensitive red re-runs once, judged by mechanism, and only with `uptime` at the host's floor (about 10) cited in the authorising message.
- hv's rule on delivery (2026-09-13, verbatim on the board until this line): no unnecessary overtesting, no testing tests, no yak-shaving; build the cheap fix and let the live system judge it, one run decides a question; ACs one line per user-facing behaviour, ATs the test that proves it once.
- The next release's CHANGELOG section is headed `## [X.Y.Z] - in progress`, the form the release driver rewrites to the cut's date and matches by version; a bare `## [Unreleased]` is invisible to it, and the first bank to need the heading creates it.
- A chain that mints AT rows runs `intent ac status <ST>` before its commit; the test file names each id it proves.
- A board commit carries the node's whole directory render (`board.json`, `wip.md`, every `inbox.<sender>.md`) and the event files its own acts wrote. An event file is yours when its own `op` and `subject` say so; read them before you name the path, never select by principal or by a `git status` listing.
- Crossed CHAIN STARTs go alphabetically (cc < dc < ic < vc) and the rule orders whole chains: an earlier START stops your next write, the commit included.
- A chain-less commit that lands clean is invisible to every node, its author included: START and END travel over the live channel and leave no durable record.
- The director gives one order per topic, to every lane it touches in one batch, and no second order until a lane reports the state. An order withdrawn after it went out races its own withdrawal; when the state has moved under an order, the order follows the state.
- A ruling that reaches one lane and not another costs a rebase: a ruling goes to every lane it touches.
- A cut's hold names hv's other sessions too: a go given in one session can reach main while the cut runs in another.
- The release driver deletes its gate logs on green, so the terminal is the only copy of the gate's verdict: read it there, and keep it.
- A source edit under somebody else's running suite unpins their verdict: bank exact hunks and `git apply` when the suite is off.
- A landing is a write: a judged bank landed under `native/rust` inside a peer's announced run voids that run exactly as an edit does, so a judge who passes a bank during a live run says "land after <node>'s END".
- A bank that adds or changes an intra-doc link owes CI's doc command, read from `rust.yml` and not retyped, before PASS: no suite, clippy or pre-commit arm runs rustdoc, and 0511 passed its suites and reddened CI on both legs.
- `lib/templates/` work is built in a detached worktree (hv decision 30): the install root is this tree, so a save in the main tree is live in every estate before any judgement.
- Any landing under `native/rust` makes the next push refuse until `bin/devbin build all` (decision 30), and the build refuses to install if HEAD moves while it runs: no node commits during a rebuild, board folds included.

## Measuring anything here

- `int suite` measures HEAD in a single-writer clone: commit first, then measure.
- Output to a file, then count: `> file 2>&1; rc=$?`; an exit code taken through a pipe is the pipe's, and zsh has `pipestatus`, not `PIPESTATUS`.
- `--no-fail-fast` always.
- Every suite runs from a private worktree's own build: in-tree `CARGO_TARGET_DIR`, isolated `HOME`, `cargo build -p intentd` first.
- A stale artefact is not a regression: rebuild before diagnosing. A detached worktree carries a staged copy across `git checkout --detach`.
- Name the artefact in every claim: `intent --version` answers for the compiled half only; `/bin/bash` is 3.2 while PATH bash is 5; `.claude/restart.md` and `intent/restart.md` are different files with opposite jobs.
- A count is not an output: an advisory is printed and never counted, so `doctor: 0 finding(s)` can print directly under a finding. Read every verdict whole (doctor, the gate, a suite summary) and name the line that would have said so.
- A measurement earns an explanation only after the instrument has been read against a known absence and a known presence of the defect. A staging that produced no defect reads exactly like a fix.
- A zero from your own instrument is a claim about the instrument; silence and success are identical unless something distinguishes them; a diff is not evidence until its inputs are shown non-empty; the revision, clock and dirty count are part of every finding.
- A red arm must predate the fix and postdate the defect; a green on the fix counts only beside a red on the baseline.
- A positive control proves the instrument on the corpus it was drawn from and nothing else: draw it from the target corpus, in the target's shape.
- An instrument fails toward a warning a reader discards, never toward an all-clear a reader believes; prefer a structural test that over-reports to a filter typed from what you expect.
- Load reds a presence-asserting arm and never false-greens it; it inverts for an absence assertion within a time window.
- Run bats through `tests/run_tests.sh`.
- The heavy-run protocol coordinates the roster and the box is a machine: census every estate before a run and at its ends, `ps -axo args= | grep -E '[b]ats-exec|[c]argo test|[r]ustc' | grep -oE '/Users/matts/Devel/prj/[A-Za-z_]+' | sort | uniq -c`. A delta between two runs survives foreign load; a timing does not.
- A test that waits on a watching daemon is exposed to host-wide FSEvents pressure, not only CPU: synthetic CPU load left the 0516 arm green while file-event churn elsewhere on the volume made FSEvents lose events outright.
- `bash -n` REFUSES EVERY `.bats` FILE and its red says nothing about your edit: a `.bats` file is not bash until bats rewrites `@test "name" {` into a function. `bats -c` is the syntax check that fits, and it counts the arms, so it also says whether bats still sees them all.
- A defect closed as one estate's is a census question first: 0498 closed as Lamplight's while nine more scripts across the fleet carried the same re-staging stanza. Grep every checkout's hooks and handlers, with the known case as the positive control, before calling it local.

## Asking and answering

- An ask names what was seen and what was expected. The fixer names the mechanism, after reading the source.
- A ruling that SHAPES A BANK goes into the unit's own record before the work starts -- an issue's body, a thread's AC rows -- because a ruling that lives only in the live channel dies at the next compact, on either side, and the node that loses it cannot tell what it is missing.

## Shared checkout and shared store

- Every session shares one tree: `git add <literal paths> && git commit --only <the same paths>` in one call (`--only <dir>` omits a new file); wait on `.git/index.lock` and re-issue the same command on a lost race; never remove a peer's lock, unstage a peer's work, `--no-verify`, `git stash` (repository-wide across worktrees), or `cp` a shared source aside.
- A shared aggregator (`CHANGELOG.md`, `intent/.intentfiles`, each crate's `tests/suite.rs`) takes every hunk a peer left in it: diff against HEAD before `git add`. Order is format, sync, commit; the register never meets prettier.
- The store lock is shared like the index: no `intent wb` write during a peer's chain, pickups included. A refused write gets one re-issue after `lsof -- intent/.cache/intent.db` lists intentd alone, never a loop.
- A canon write can report ok and be reverted by the daemon's ingest a second later: verify past the ingest.
- A board view is landed by a board write and nothing else (`intent wb touch --node <you>` is the cheapest); `organize` lands thread and issue views, never a board.
- Worktree results come back as a patch, never a whole-file copy.
- A refused destructive write goes through a scratch clone and a proven patch, never to a peer. A socket message is not a delivery; the durable inbox is the record.
- A board item's text goes in as `"$(cat <file>)"`: a backticked phrase inline is the shell's command substitution.
- Paths held in a variable are ONE pathspec, because the shell does not word-split: `git add -- $paths` refuses the lot and stages nothing. Write the literal paths, as the line above does.
- `git ls-files --error-unmatch` answers whether a path is in the INDEX, which a staged-but-uncommitted file is, so it reports a file as done that is not committed. `git diff --cached --name-only` is the question.
- An event file's owner is its own `op` and `subject`, never a `git status` listing: a glob over the events directory takes a peer's beside yours.
- The first store write in an estate whose views an older pair rendered RE-RENDERS EVERY VIEW, one footer line each, so a one-line change arrives with a mechanical commit beside it: name the cause, and show the diff empty with that footer line excluded.

## Design rules

- Make the bad state unrepresentable rather than checked for.
- A predicate is sound relative to what is done with the answer: one predicate reused by callers with opposite error contracts needs the reason written at each call site.
- Highlander governs implementations, not witnesses. A witness that cannot see the event is not a witness; where only witnesses sharing a blind spot exist, declare it.
- Changing a published field's meaning without changing its shape is the worst version of that change.
- A criterion is owned by whoever can satisfy it and must be able to fail.
- Removing a swallow means finding out what it was swallowing, and the only way is to remove it.
- A misattributed failure is worse than a swallowed one: a wrong remedy sends the reader somewhere wrong.
- A command in a document is a claim until someone has run it: run the read-only ones, read the dangerous ones as control flow, and say which way each was verified. Look in both fenced blocks and inline backticks.
- A pattern aimed at generated output is aimed at a rendering, and the renderer is a writer you did not consult.
- An edit that matches nothing is indistinguishable from one that worked: verify the result, never the exit code.

## The clock

- Every stamp is read from `date -u +'%Y-%m-%d %H:%MZ'` in the same turn; a stamp you did not read is fabricated. `git log` prints local time.
- A message that cites a measurement goes out after the measurement returns, never in the same batch.
- A state named in a message carries the clock of its reading and says what would discharge it.

## Conventions

- Commit conventions, house style and the banned-word list are in `CLAUDE.md` and `~/.claude/CLAUDE.md`. Local: em dashes in prose pages, `--` in generated reference pages; no hardcoded counts in any doc a reader reads.
- Each project-wide document has one job: `.claude/restart.md` is the entry point and holds no state; `intent/wip.md` is DOING and TODO only; this file is rules.
- The push gate runs no fmt, clippy or doc: CI runs them, and devbin's `int check` twins mirror each CI line, held equal by `tests/unit/devbin_rust_gates.bats`. A parity test holds only the fields it reads: that test read `run:` and not `env:`, and the doc twin drifted (0501).
