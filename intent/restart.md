# Intent -- traps and conventions

**Current as at 2026-09-15 22:45Z: THE CLOSE-OUT IS HELD OVERNIGHT AT hv's WRAP (hv, 2026-09-15): every open issue and thread closes on evidence or by ruling, and each lane resumes at its own board's IN FLIGHT todo.** v3.0.3 is shipped (tag 46599d145); the keg is unlinked and pinned; `intent` on PATH is the dev tree; the delivered pair is at `9cd639e46` and the live intentd runs it, which is behind HEAD's code by 0375 until vc's rebuild after 0331 (b) lands. hv's rulings: `intent/history/20260915-hv-rulings.md`. Lanes and hv's items: `intent/wip.md`. Open issues: `intent issues list`. Landing rules: a bank is a patch blob on `refs/bank/<node>/<issue>` with its base named; ready banks ride a train, vc stacking them on HEAD for one judging run (intentsvcs, intent-cli and intentd `--no-fail-fast`, workspace clippy, rustfmt), and a train of one runs in its lane's warm worktree; lanes land in stack order, rebased onto the hash vc sends, each closing its issues with `organize --apply` and ending its turn with the report SENT; a timing-sensitive control runs under a host hold vc announces; an event-wait family red re-runs once and is judged by mechanism; a register row with no canon behind it stops every landing. NO RELEASE, NO PUSH without hv at the terminal; never `--no-confirm`.

**THIS FILE CARRIES RULES, NOT HISTORY (hv, 2026-08-30), AND ON 2026-09-14 hv HAD IT LEANED.** What was here before is verbatim in `intent/history/20260914-restart-prefold.md` (and `202608-restart-prefold.md` before it). What survives is what nothing else enforces, one line each; anything encoded in a test, a guard, a generator or a shared memory has no prose home here.

## Where you are standing

- Measure the binary, never trust a line: `command -v intent && readlink "$(command -v intent)"`, `intent --version`, `intent info | sed -n 's/^ *INTENT_HOME: *//p'`.
- `~/Devel/prj/Intentv2` is frozen and never written.
- A build is the delivery: `bin/devbin build all` verifies the pair as a set and the PATH symlinks point into this tree; templates are compiled in, so a `lib/templates/` edit reaches `init` only after a rebuild. A bare `cargo build --release` is not the door.
- During a rebuild no estate on this machine has `intent`, and commits block; the node rebuilding announces the start AND the end.
- The marker is provenance, not identity: currency is `git diff --name-only <marker>..HEAD` over `DIRT_SCOPE` in `native/rust/build-support/source_commit.rs`, empty meaning current.
- A general policy stated after a specific ruling does not vacate it: `config`, `ext` and `learn` ship declared-and-unbuilt (hv, 2026-08-31); building one reverses a ruling.
- Strike "not in this one", "defer" and "after the tag" as a class; they are the scarcity move wearing a release number.
- Monikers are estate-scoped and nothing marks them: write `laksa-cc`, never `cc`, when a moniker crosses an estate; a machine-wide `ps` shows every estate's `intent` processes.
- `ListAgents`' started column is socket age; `/compact` does not rotate `CLAUDE_CODE_SESSION_ID`.

## Measuring anything here

- `int suite` measures HEAD in a single-writer clone: commit first, then measure.
- Output to a file, then count: `> file 2>&1`; a negative from a partial read is not a result.
- An exit code taken through a pipe is the pipe's: `cmd > f 2>&1; rc=$?`; zsh has `pipestatus`, not `PIPESTATUS`.
- `--no-fail-fast` always; `cargo test --workspace` otherwise stops at the first failing target.
- Every suite runs from a private worktree's own build, with an in-tree `CARGO_TARGET_DIR`, under an isolated HOME written as a command (`HOME=<dir>`), after `cargo build -p intentd`; a test run writes the estate it lives in, so check `~/.local/share/intent/home` after any peer run (`~/.intent/home` is gone since ST0074 WP-05).
- A stale artefact is not a regression: rebuild before diagnosing; a detached worktree carries a STAGED copy across `git checkout --detach`.
- Name the artefact you examined in every claim: `intent --version` answers for the compiled half only; `/bin/bash` is 3.2 while PATH bash is 5; `.claude/restart.md` and `intent/restart.md` are different files with opposite jobs.
- A zero from your own instrument is a claim about the instrument; the population is the claim; a gate and any n-of-m is computed by a verb; silence and success are identical unless something distinguishes them; the failure path is the one a green run never exercises; a CI run's subject is the push; a change that would green your own work is the one to route; the revision, clock and dirty count are part of every finding.
- A red arm must predate the fix and postdate the defect; a green on the fix counts only beside a red on the baseline; a fixture that discards what it is handed passes for a row its caller cannot produce.
- Run bats through `tests/run_tests.sh`.

## Shared checkout and shared store

- Five sessions share one working tree: `git add <literal paths> && git commit --only <the same literal paths>` in ONE call (zsh does not word-split an unquoted list; `--only <dir>` omits a new file); wait on `.git/index.lock`; read `git log -1` after; on a lost race re-issue the SAME command; never remove a peer's lock, unstage a peer's work, `--no-verify`, `git stash` (repository-wide across worktrees), or `cp` a shared source aside.
- A shared aggregator (`CHANGELOG.md`, `intent/.intentfiles`, `tests/suite.rs`) takes every hunk a peer left in it: diff against HEAD before `git add`; `sync` has no unit narrower than a thread; order is format, sync, commit; the register never meets prettier.
- The store lock is shared like the index: during a peer's landing chain no other lane runs any `intent wb` write, pickups included; a refused write gets ONE re-issue after `lsof -- intent/.cache/intent.db` lists intentd alone, never a loop; after a landing commit the daemon's ingest holds the lock invisibly, wait for it to cool; attribute a holder only by lsof on THIS store's path.
- A canon write can report ok and be reverted by the daemon's ingest a second later: verify past the ingest; `sync --to-store` replaces the store from the extract; `intent edit --path` is a write; `st list --status all` (`--all` exits 1); `intent` refuses outside a project, so no `cd` into the scratchpad before a store write; a correct refusal is not a save; a remedy is a promise.
- Worktree results come back as a patch, never a whole-file copy; an attachment edited in a worktree goes through `st attach` with the worktree's own build after an unscoped `sync --to-store`. A bank is that patch as a blob on `refs/bank/<node>/<issue>` (`git add -A && git diff --cached --binary <base>`, then `git hash-object -w` and `git update-ref`), recovered with `git cat-file -p <ref> | git apply`: never a file under `/private/tmp`, which a reboot clears with every worktree in it (0338 (ii), 2026-09-15), and never a worktree commit, which the gate refuses as GATE ABSENT.
- A refused destructive write goes through a scratch clone and a proven patch, never to a peer. A socket message is not a delivery; the durable inbox is the record. FIXED is four states: worktree, index, HEAD, pushed.
- A directory-granularity file event is a question, not an answer; the rulings gate and dehydration collide when a ratified ruling cites a thread file.

## Design rules

- Make the bad state unrepresentable rather than checked for.
- Where a property belongs to a syscall or a dependency default, the outcome holds under any implementation.
- A predicate is sound relative to what is done with the answer.
- Highlander governs implementations, not witnesses: consolidating witnesses deletes the measurement.
- Changing a published field's meaning without changing its shape is the worst version of that change.
- A criterion is owned by whoever can satisfy it and must be able to fail.
- Removing a swallow means finding out what it was swallowing, and the only way is to remove it.
- A rule that catches a failure mode it was not designed for is the strongest evidence for it.
- The consolidation is the work rather than the shortening.
- Read what the binary resolves, not what the list says; a subject written down beats a discipline.

## The clock

- Every stamp is read from `date -u +'%Y-%m-%d %H:%MZ'` in the same turn; a stamp you did not read is fabricated. Three generators: arithmetic from one read, fabrication with the correct value present, a stale reference that only accuses the other party. `git log` prints local time; appending `Z` is an assertion. The commit guard catches a future stamp, a missing `Z` and an inbox going backwards; the live channel has no door.

## Conventions

- T-shirt sizing only. The intent CLI for every ST, WP, AC, AT, issue and board write; never a hand edit of a generated view or a board. Never manually wrap markdown. No Claude attribution in commits, ever; every commit ends `(C) hello@matthewsinclair.com`. `eg` only, and the banned filler is listed in `~/.claude/CLAUDE.md`; never open with "You're right"; no em dashes in skill files; 2-space indentation everywhere; no hardcoded counts in any doc a reader reads.
- Each project-wide document has one job: `.claude/restart.md` is the entry point and holds no state; `intent/wip.md` is DOING and TODO only; this file is rules plus its one state line.
- The push gate runs no fmt or clippy; CI is their only home. Em dash in prose pages; `--` in generated reference pages. A surface claim travels with what makes it checkable.
