---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 08:28Z
status: active
focus: "The watcher repair is BUILT and UNCOMMITTED, and it is HELD: the reconcile is now reporting a real pre-existing difference and whether that may be published needs vc. Then the mid-course rehearsal, then the grammar measurement. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-0827Z.md`.** Batches 1 and 4 landed and are carried by their commits and the CHANGELOG, not here.

## DOING -- the watcher repair, on vc's settled ruling. BUILT, UNCOMMITTED, HELD.

**THE DIFF IS NOT IN GIT.** It is `scratchpad/watcher-repair-WIP.patch` in this session's scratchpad, and the same changes are live in the worktree `scratchpad/wt-dc`. If both are gone, rebuild from this section: it carries the whole design.

Three parts, all written and compiling, `cargo build -p intentd` clean:

1. **`sync.rs::includes` returns false for a bare directory.** An empty remainder meant `components.pop()` was a no-op, the `descends` loop ran zero times, and `intent/` itself fell through to `true` -- so the skip list never got a chance to speak. Unit test added and CONTROLLED RED by reverting only that hunk.
2. **`sync.rs::changed_under(root, under, previous)`**, new and public: walks `under` with the same `Scanned` the sync engine uses, compares SHA-256 against the store's recorded index, returns the in-scope files whose bytes differ. Scope and policy stay in `sync.rs` so the watcher is not a second statement of scope.
3. **`watch.rs::files_that_changed`**, split out of `on_batch` so a directory-granularity event can be PLANTED rather than waited for; a path that is a directory or no longer exists is reconciled, a leaf is handled as before. Four unit tests, all green, including that a leaf event costs no store round trip. `store.rs` gains an internal `Work::FileIndex` read door -- the index lives in SQLite and `one_store_door.rs` forbids a second connection.

**WHAT IS HELD, AND IT IS A RULING NOT A BUG.** The daemon suites ran 8/8 green under loads 24-41. Under a heavier `--workspace` run at load 50-87 the pair still fails, and the delivered path is no longer a directory -- it is `.prettierignore`, a root file, with the store's index NON-empty (probed: 23 entries). So the reconcile is reporting a REAL difference: `.prettierignore` genuinely differs from what the store recorded, because `converge_formatter_exclusion` appended to it after the index was taken.

**The question for vc: may a directory event publish a difference that PRE-DATES it?** My reconcile answers _what differs now_; the test's `feed.settle()` assumes _what changed since_. Both are defensible and the choice is not mine to guess. Do not soften either assertion; they are correct as written.

## TODO

- **The mid-course rehearsal**, once the watcher lands: `--dry-run` of `build release --patch` in a clone with the gates live and `intent backup` taken deliberately first, every gate line reported. Not the run that counts; it proves batch 4 left the release path whole and that preflight is deterministic once the watcher stops answering itself. Drive the daemon suite more than once, load stated.
- **Then WP-20's grammar measurement, AC-20.4**: one tree-sitter grammar crate at a time (rust, elixir, swift, lua, bash), release pair built with the in-tree target dir, binary-size delta per grammar with the exact command and the toolchain in force. No estimate, no rounding into prose. Swift is the expected outlier. A report to vc and hv, who sets the line; the crates do not land from this step. Read `intent/st/ST0069/design.md` FIRST.
- **Tell cc when the watcher lands** -- WP-18 widens this watcher and builds on the fix.
- **CHANGELOG lines** from cc for its last two items (fbf3e7f7d, f24053c13) and ic's output lines, as they reach me.

## Holds

- **The watcher repair is held for vc's ruling on the pre-dating-difference question above.** Condition: vc answers. vc is compacting; the report is in `vc/inbox.dc.md` as well as sent.
- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **`git stash` IS SHARED ACROSS EVERY WORKTREE OF ONE REPO.** I used it for a control and a peer's WIP was sitting in that stack; push/pop raced against them and only luck kept it straight. Control a diff with `git diff > patch; git checkout -- <paths>; git apply patch` instead, which touches nothing shared.
- **A NEW DEPENDENCY NEEDS ITS RATIONALE IN THE WORKSPACE MANIFEST, NOT THE CRATE'S.** `dependency_rationale` reds three ways when a dep `intentd` declares has no comment block above its pinned version in `native/rust/Cargo.toml`. Two of the three reds are the check's own controls failing, which reads as a catastrophe.
- **The test daemon's stderr is `Stdio::null()`.** An `eprintln!` probe inside `intentd` can never reach the test output; write to a file if you need to instrument it, and delete the probe before banking.
- **`cargo test -p intent-cli` ALONE reds a couple of dozen daemon-dependent tests that `--workspace` passes** (cc): a single-package run does not build the `intentd` binary they need. Match the release gate's own `--workspace` spelling, not a narrower one.
- **A red that MOVES between tests across runs of the same bytes is timing, not state.** Elimination reaches "setup"; only repetition reaches which.
- **A red that CHANGES SHAPE after your fix is a new finding, not the old one persisting.** The delivered path went from a directory to a root file; reading that as "still broken" would have hidden that the mechanism had changed.
- **D42: a clock value goes into a board or a message only from a `date -u` read in the same turn's output, pasted.** `git log`, `stat` and `ls -la` print LOCAL; appending `Z` is an assertion.
- **A GATE THAT READS A GITIGNORED PATH CANNOT BE REHEARSED IN A CLONE.** `intent doctor` reports `backup-stale` on `intent/.backup/db`, which `.gitignore` excludes, so a fresh clone fails it by construction. Take `intent backup` in the clone deliberately and say so.
- **A VERSION PINNED INTO A GENERATED FILE ROTS AT EVERY RELEASE.** Equal byte counts on both sides of a skew report are the tell that the difference is a same-width substring, not an edit.
- **Doctor's refusal is ONE exit code over SEVERAL residues.** Read every residue before attributing the refusal to the one you expected.
- **`--only` is not ceremony: a peer's work can arrive INSIDE your commit window.** A plain `git commit` sweeps their staged files into yours, silently.
- **A green at a commit that is no longer the subject is a claim about a tree nobody is releasing.** Re-drive at the new HEAD before committing, however unrelated the intervening commit looks.
- **NEVER run a formatter over a file you are editing by hand.** `prettier --write` reflowed the whole command register inside a one-line change, hiding the line and landing bytes nothing had tested. Edit the register by position and regenerate only its markdown.
- **A working-tree deletion under `intent/st/` is not data loss on its own.** The canon carries each attachment's text and hash; check the canon and the event log before calling it one.
- **Shared checkout: `git add <paths> && git commit --only <paths>` in ONE call.** On a lock refusal re-issue the SAME command.
- **Every suite and build from a private worktree with its IN-TREE target dir** under an isolated HOME. An out-of-tree `CARGO_TARGET_DIR` puts the binary where no `lib/templates/` sits above it and reds the install-root tests (vc, corrected 2026-09-12).
- **The Bash tool's shell is zsh: unquoted `$var` does NOT word-split** -- a path list in a variable reaches `git add` as one argument. It caught me again today. Messages go in a file, through `-F`.
- **Homebrew's `post_install` cannot write the user's HOME** (sandbox temp HOME, EPERM; driven). `~/.intent/home` is written by `intent bootstrap` alone, and bootstrap REPOINTS an existing pointer.

## Decisions

- **devbin `0047` (hv, 2026-09-01): option 3, the split.** `fullcycle`'s clean phase forces only the blocked-binaries arm; `_clean_confirm`'s removal prompt stays. Relayed to devbin-vc, who own `bin/.devbin/lib/`; the vendored copy here is overwritten on upgrade, so it is never mine to implement. UNEXECUTED, and not dc's.
