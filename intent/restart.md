# Intent -- traps and conventions

**THIS FILE CARRIES RULES, NOT HISTORY (hv, 2026-08-30), AND NOT STATE.** What nothing else enforces, one line each; anything held by a test, a guard, a generator or a shared memory has no prose home here. Prior contents are verbatim under `intent/history/`. Live work is `intent/wip.md`; the commands that regenerate any state are in `.claude/restart.md`; hv's rulings are `intent/history/20260915-hv-rulings.md` and hv's board. **NO RELEASE, NO PUSH without hv at the terminal; never `--no-confirm`; never `intent fc`.**

## Where you are standing

- Measure the binary, never trust a line: `command -v intent && readlink "$(command -v intent)"`, `intent --version`, `intent info | sed -n 's/^ *INTENT_HOME: *//p'`.
- The frozen v2 checkout is `~/Devel/prj/_Archive/Intentv2`, branch `v2-maintenance`, and is never written. It is NOT at `~/Devel/prj/Intentv2`, which two boot documents named until 2026-09-17 and which does not exist.
- A build is the delivery: `bin/devbin build all` verifies the pair as a set and the PATH symlinks point into this tree; templates are compiled in, so a `lib/templates/` edit reaches `init` only after a rebuild. A bare `cargo build --release` is not the door.
- During a rebuild no estate on this machine has `intent`, and commits block; the node rebuilding announces the start AND the end.
- The marker is provenance, not identity, and it answers for the compiled half only; `.claude/restart.md` carries the currency measurement and its scope.
- A general policy stated after a specific ruling does not vacate it: `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31); building one reverses a ruling.
- Strike "not in this one", "defer" and "after the tag" as a class; they are the scarcity move wearing a release number.
- Monikers are estate-scoped and nothing marks them: write `laksa-cc`, never `cc`, when a moniker crosses an estate; a machine-wide `ps` shows every estate's `intent` processes.
- `ListAgents`' started column is socket age; `/compact` does not rotate `CLAUDE_CODE_SESSION_ID`.

## Landing work

- A bank is a patch blob on `refs/bank/<node>/<issue>` with its base named, recovered with `git cat-file -p <ref> | git apply`: never under `/private/tmp`, which a reboot clears, and never a worktree commit, which the gate refuses as GATE ABSENT.
- One lane lands at a time on vc's word, rebased onto the hash vc sends, closing its issues with `organize --apply` and ending the turn with the report SENT. **The judged patch-id is what lands**; a gate refusal is re-banked before any commit. A register row with no canon behind it stops every landing.
- Banks sharing a file land in the order they bank green, stacking so one run stands for the landing. One whole-suite judging run per train, in the warm worktree holding its stack, START and END to every lane; vc judges the logs, the diff, and that the worktree equals the bank.
- **A "comments only" or "config only" change does not waive the suites** (vc, 2026-09-18: 0451's doc-comment edits drifted the committed schema faces and its CI line failed devbin's twin test, both red on main until the next lane's run). Any change under a crate, the CI configuration or a generator's source owes the whole crate suites and the shell half; the diff's shape is not evidence of its reach.
- A timing-sensitive control runs under a host hold vc announces; an event-wait red re-runs once and is judged by mechanism, never by a failing set.
- **Two orders from the pen in flight cross exactly like two CHAIN STARTs, and the lanes cannot resolve it because each is obeying** (vc, 2026-09-18: cc and ic each yielded to the other twice and the host sat idle). One order per topic, to every lane it touches in one batch, and no second order until a lane reports the state; when the state has moved under the order, the order follows the state (the run that exists stands). Decision 23 governs the lanes; this governs the director.
- **A source edit under somebody else's running suite silently unpins their verdict** (dc, 2026-09-17): the tree its verdict describes stops being the tree it started on. Bank exact hunks; `git apply` when the suite is off.

## Measuring anything here

- `int suite` measures HEAD in a single-writer clone: commit first, then measure.
- Output to a file, then count: `> file 2>&1`; a negative from a partial read is not a result.
- An exit code taken through a pipe is the pipe's: `cmd > f 2>&1; rc=$?`; zsh has `pipestatus`, not `PIPESTATUS`.
- `--no-fail-fast` always; `cargo test --workspace` otherwise stops at the first failing target.
- Every suite runs from a private worktree's own build, in-tree `CARGO_TARGET_DIR`, isolated `HOME=<dir>`, after `cargo build -p intentd`; a run writes the estate it lives in, so check `~/.local/share/intent/home` after any peer run.
- A stale artefact is not a regression: rebuild before diagnosing; a detached worktree carries a STAGED copy across `git checkout --detach`.
- Name the artefact you examined in every claim: `intent --version` answers for the compiled half only; `/bin/bash` is 3.2 while PATH bash is 5; `.claude/restart.md` and `intent/restart.md` are different files with opposite jobs.
- A zero from your own instrument is a claim about the instrument; the population is the claim; a gate and any n-of-m is computed by a verb; silence and success are identical unless something distinguishes them; the failure path is the one a green run never exercises; a CI run's subject is the push; a diff is not evidence until its inputs are shown to be non-empty, empty against empty reading as agreement at exit 0; a change that would green your own work is the one to route; the revision, clock and dirty count are part of every finding.
- A red arm must predate the fix and postdate the defect; a green on the fix counts only beside a red on the baseline; a fixture that discards what it is handed passes for a row its caller cannot produce.
- **A positive control proves the instrument on the corpus you controlled against and says NOTHING about the corpus you point it at next** (dc, 2026-09-17). Draw the control from the target corpus and check it shares the target's SHAPE: a fence walk validated against a document built of fenced blocks reported clean over three documents that have none, every command in them inline.
- **An instrument must fail toward a warning a reader discards, never toward an all-clear a reader believes** (ic, 2026-09-17). Prefer a structural test that over-reports to a filter typed from what you expect to find. The shared mechanism is NARROWED BY WHAT ITS AUTHOR EXPECTED and it wears several shapes -- a guessed alternation, an assumption that commands live in fences, a normaliser collapsing one run of characters but not another -- so a rule naming any one shape lets the rest through.
- Run bats through `tests/run_tests.sh`.

## Shared checkout and shared store

- Five sessions share one tree: `git add <literal paths> && git commit --only <the same literal paths>` in ONE call (`--only <dir>` omits a new file); wait on `.git/index.lock`; re-issue the SAME command on a lost race; never remove a peer's lock, unstage a peer's work, `--no-verify`, `git stash` (repository-wide across worktrees), or `cp` a shared source aside.
- A shared aggregator (`CHANGELOG.md`, `intent/.intentfiles`, and each crate's own `native/rust/crates/*/tests/suite.rs`) takes every hunk a peer left in it: diff against HEAD before `git add`; `sync` has no unit narrower than a thread; order is format, sync, commit; the register never meets prettier.
- The store lock is shared like the index: during a peer's landing chain no other lane runs any `intent wb` write, pickups included; a refused write gets ONE re-issue after `lsof -- intent/.cache/intent.db` lists intentd alone, never a loop; after a landing commit the daemon's ingest holds the lock invisibly, wait for it to cool; attribute a holder only by lsof on THIS store's path.
- A canon write can report ok and be reverted by the daemon's ingest a second later: verify past the ingest. `sync --to-store` replaces the store from the extract; `intent edit --path` is a write; `st list --status all` (`--all` exits 1); no `cd` into the scratchpad before a store write; a correct refusal is not a save; a remedy is a promise.
- Worktree results come back as a patch, never a whole-file copy; an attachment edited there goes through `st attach` with the worktree's own build after an unscoped `sync --to-store`.
- A refused destructive write goes through a scratch clone and a proven patch, never to a peer. A socket message is not a delivery; the durable inbox is the record. FIXED is four states: worktree, index, HEAD, pushed.
- A directory-granularity file event is a question, not an answer; the rulings gate and dehydration collide when a ratified ruling cites a thread file.
- A board item's text goes in as `"$(cat <file>)"`, never inline in double quotes: a backticked phrase in item prose is the shell's command substitution.

## Design rules

- Make the bad state unrepresentable rather than checked for.
- Where a property belongs to a syscall or a dependency default, the outcome holds under any implementation.
- A predicate is sound relative to what is done with the answer. **One predicate reused by two callers with opposite error contracts needs the reason written AT the call site**, or the next reader tidies them into agreement and silently reinstates the defect.
- Highlander governs implementations, not witnesses: consolidating witnesses deletes the measurement. **And a witness that cannot see the event is not a witness** (the 0442 detector, 2026-09-17): prove a second instrument can observe the fault before counting it, because independence from a failure mode is sometimes exactly what blinds it. Where only witnesses sharing a blind spot exist, declare that; never add a decoy to make the principle read as met.
- Changing a published field's meaning without changing its shape is the worst version of that change.
- A criterion is owned by whoever can satisfy it and must be able to fail.
- Removing a swallow means finding out what it was swallowing, and the only way is to remove it.
- A misattributed failure is worse than a swallowed one: silence gives the reader nothing, a wrong remedy gives them somewhere wrong to go and every attempt confirms the wrong diagnosis.
- A rule that catches a failure mode it was not designed for is the strongest evidence for it.
- The consolidation is the work rather than the shortening.
- Read what the binary resolves, not what the list says; a subject written down beats a discipline.
- **A COMMAND IN A DOCUMENT IS A CLAIM AND BEATS A SENTENCE ONLY ONCE SOMEBODY HAS RUN IT** (ic and dc, 2026-09-17, after a release page that landed hours earlier was found to carry four defects spread across most of its command blocks). Read-only ones get RUN; ones nobody can safely run -- `git tag`, a push, a publish -- get READ as control flow. **Say which way each was verified.** Enumerate them structurally in BOTH homes, fenced blocks and inline backticks: the documents every node reads at boot carry all of their commands inline and not one fenced block.
- A pattern aimed at generated output is aimed at a RENDERING, and the renderer is a writer you did not consult.
- An edit that matches nothing is indistinguishable from one that worked: verify the result, never the exit code.

## The clock

- Every stamp is read from `date -u +'%Y-%m-%d %H:%MZ'` in the same turn; a stamp you did not read is fabricated. Three generators: arithmetic from one read, fabrication with the correct value present, a stale reference that only accuses the other party. `git log` prints local time; appending `Z` is an assertion. The commit guard catches a future stamp, a missing `Z` and an inbox going backwards; the live channel has no door.
- **A message that cites a measurement goes out AFTER the measurement returns, never in the same batch** (vc, 2026-09-18: "they hash identical" was sent beside the comparison, which printed DIFFER; a second comparison happened to agree). A claim typed before its read is the clock fault in another field.
- **A state named in a message carries the clock of its reading and says what would discharge it.** A warning that has expired still does its work if it provokes a measurement; the form to avoid is one that invites belief rather than a check.

## Conventions

- Commit conventions, house style and the banned-word list are in `CLAUDE.md` and `~/.claude/CLAUDE.md`, which every session receives; they are not restated here. What is local: em dashes in prose pages and `--` in generated reference pages; no hardcoded counts in any doc a reader reads; a surface claim travels with what makes it checkable.
- Each project-wide document has one job: `.claude/restart.md` is the entry point and holds no state; `intent/wip.md` is DOING and TODO only; this file is rules.
- The push gate runs no fmt or clippy; CI is their only home.
