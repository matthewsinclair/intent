---
node: dc
name: DevX Claude
role: worker
session_id: b9e78c72-479d-4984-9df9-ac1bedfe7f2d
heartbeat_at: 2026-09-12 09:22Z
status: active
focus: "The watcher repair is LANDED (da08b4a18), reshaped to vc's ruling. Now: the confirming drive under a genuinely isolated HOME, which is also the experiment for whether a test repoints `~/.intent/home` or whether that was my own missing isolation. Then the mid-course rehearsal, then WP-20's grammar measurement. NO RELEASE, NO PUSH."
claims: [ST0056/07, ST0056/11, ST0056/12, ST0058]
---

# DevX Claude (dc)

**The board before this fold is verbatim at `.history/20260912/wip-prefold-0827Z.md`.** Batches 1 and 4 landed and are carried by their commits and the CHANGELOG, not here.

## DOING -- the confirming drive, and the HOME question it answers

**The watcher repair LANDED at `da08b4a18`.** Six files. `candidates` is factored out of `scan` and is the ONE enumeration of the corpus; `changed_under` calls it and filters by the event's path; `includes` refuses a bare directory. vc verified it by file list. cc is told; WP-18 widens it.

**WHAT IS OPEN IS A CLAIM I MADE AND DID NOT CHECK.** I reported those drives as being "under an isolated HOME" and I set no HOME at all -- the phrase came off my own board as a description of how I work rather than off the command I ran. vc found `~/.intent/home` repointed at my worktree and restored it. The drive running now sets `HOME` to a scratch dir and `CARGO_HOME` to the real one, and reads the REAL pointer with its mtime before the run and after each of two runs. **If the real pointer still moves, a test reaches past the isolation and I name it; if it does not, there is no test defect to chase and vc's `dual_path_conformance` suspicion is unproven rather than confirmed.**

**Also on the record and not yet explained:** a confirming run taken while the box was still at load 53 with `fseventsd` pinned near 100% showed two reds -- `an_external_edit_delivers_both_d20_events_in_layer_order` and `daemon_watch::one_external_edit_costs_a_bounded_number_of_ingests`. Neither is the scope pair, and both are _no event arrived in time_ rather than _the wrong thing arrived_. Not called environmental until the quiet run says so.

## TODO

- **The mid-course rehearsal**, next in vc's order: `--dry-run` of `build release --patch` in a clone with the gates live and `intent backup` taken deliberately first, every gate line reported. Not the run that counts; it proves batch 4 left the release path whole and that preflight is deterministic once the watcher stops answering itself. Drive the daemon suite more than once, load stated.
- **Then WP-20's grammar measurement, AC-20.4**: one tree-sitter grammar crate at a time (rust, elixir, swift, lua, bash), release pair built with the in-tree target dir, binary-size delta per grammar with the exact command and the toolchain in force. No estimate, no rounding into prose. Swift is the expected outlier. A report to vc and hv, who sets the line; the crates do not land from this step. Read `intent/st/ST0069/design.md` FIRST.
- **CHANGELOG lines** from cc for its last two items (fbf3e7f7d, f24053c13) and ic's output lines, as they reach me.

## Holds

- **The tap formula commit `9987a93` is local and unpushed.** Condition: hv approves that push, as its own action.
- **A HOLD WHOSE STATED CAUSE IS WRONG STILL READS AS A HOLD.** Re-drive a hold's condition when you quote it; never read it off this line.

## Watch-outs

- **A DISCIPLINE ON YOUR BOARD IS NOT A FLAG ON YOUR COMMAND LINE.** I wrote "under an isolated HOME" in a commit message and to vc, and I had set no HOME: the phrase came off this board as a description of how I work. A claim sourced from your own standing practice reads exactly like a checked one. Read the command back before you describe it.
- **A SETUP STEP THAT FAILS SILENTLY LEAVES AN INSTRUMENT THAT STILL ANSWERS.** `intent init --name X` is not v3's spelling; it refused at rc 1, I did not read the code, and every `intent critic` run after it looked perfectly normal because the rule library resolves from the INSTALL ROOT, not the project. Read the setup's exit code before you trust the measurement.
- **A SECOND ENUMERATION OF A SET IS A SECOND STATEMENT OF SCOPE.** `walk(dir)` and `Scanned::includes(file)` answered differently about one file depending on which door the event came through. Enumerate once, decide once; `scan` and `changed_under` now share `candidates`.
- **SYNTHETIC LOAD SATURATES `fseventsd`, AND A WATCHER TEST THEN TIMES OUT FOR A REASON THAT IS NOT THE CODE.** Read `ps aux | grep fseventsd` beside the load average before attributing a watcher red.
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
