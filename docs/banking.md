# Banking work: landing changes from a shared checkout

**When more than one session works in one checkout, a finished change that is not yet allowed to land needs a home that is not the index, not the stash, not a temporary directory and not a commit. That home is a bank: a patch stored as a git blob and named by a ref under `refs/bank/`.**

This page is policy, for Intent's own repository and for any project where several coding agents, or agents and people, share a checkout under the whiteboard node model. It was a convention the nodes kept in memory until 2026-09-18; every rule below was measured before it was written down, and the dates say when.

## What a bank is

A bank is the change as a patch, `git diff --binary` against the commit it was made on, written into the repository's object store with `git hash-object -w` and named by a ref: `refs/bank/<node>/<topic>/<name>`. The ref points at a **blob**, not at a commit and not at a branch.

That shape is the whole point:

- It lives in `.git`, so it survives a worktree's removal, a scratch directory's cleanup and a reboot. It lives in this repository's `.git` only. A clone does not fetch `refs/bank/*`, so a bank is lost with that `.git`, and another clone gets it only by fetching or pushing it by name.
- It is not a branch and not a commit, so it never appears in `git branch`, `git status` or `git log`, and `git log --all` keeps working over it (measured 2026-09-15).
- It is not sent by `git push`, `git push --all` or `git push --tags`, which move branches and tags. `git push --mirror` does send it, and so does a remote configured with `mirror = true` or a push refspec covering `refs/*`.
- It carries no index state and no stash state, so it touches nothing a peer's commit gate reads.
- A bank keeps no reflog. Git logs `refs/heads/`, `refs/remotes/`, `refs/notes/` and `HEAD`, not `refs/bank/`, and deleting a ref deletes any reflog it had. So `git update-ref -d` on a bank leaves only an unreachable blob. `git fsck --unreachable` still finds it until `git gc` prunes it, after `gc.pruneExpire` (two weeks by default) or at once with `--prune=now`. Delete a bank only after the landing that carries its patch-id.

`git for-each-ref refs/bank/` lists them, each with its object type `blob`. A repository that has run `git gc` holds most of them in `.git/packed-refs` rather than as loose files under `.git/refs/bank/`; both forms are the same ref.

Every bank names its **base**, the commit the patch applies to, in the message that announces it or in a note banked beside it. A patch without a base is a patch nobody can apply.

## The recipe

In the private worktree where the change was built, with `<base>` the commit it was built on:

```
git add -A
git update-ref refs/bank/<node>/<topic>/<name> "$(git diff --cached --binary <base> | git hash-object -w --stdin)"
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply -R --check
git cat-file -p refs/bank/<node>/<topic>/<name> | git patch-id --stable
```

The third line checks that the bank reverses cleanly on the tree it was taken from, so every change it carries is in that tree. It cannot see a change the tree holds and the bank lacks, such as a file `git add -A` skipped as ignored. For equality, apply the bank to `<base>` in a scratch index (`git read-tree <base>`, then `git apply --cached`, both under `GIT_INDEX_FILE`) and compare `git write-tree` with the real index's. The fourth line is the id the change is judged by.

To read, list and apply one:

```
git for-each-ref refs/bank/ --format='%(objecttype) %(refname)'
git cat-file -p refs/bank/<node>/<topic>/<name>
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply --check
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply
```

`git apply` on the base leaves the change in the working tree and not in the index, which is what a landing wants.

## Why the alternatives are refused

Each of these was tried and each lost work or nearly did.

- **A temporary directory.** The host rebooted on 2026-09-15 and `/private/tmp` went with it, taking every scratch directory, every worktree kept in one, and a judged, unlanded patch that then had to be regenerated from a transcript.
- **A worktree commit.** In Intent's own repository the pre-commit hook lives in the tracked `.githooks/` and its shim `pre-commit.intent` is ignored, so a worktree's checkout has the hook and not the gate, and the hook refuses the commit as `GATE ABSENT` rather than committing unguarded; copying the shim in then meets the doctor gate on views the commit does not carry. Nobody bypasses the gate.
- **`git stash`.** A worktree's stash is the whole repository's stash, shared with every session and holding entries months old. It also refuses a tree with intent-to-add entries, and the `pop` that follows a refused `stash` then reaches for somebody else's entry (2026-09-17).
- **The shared index.** `git add` publishes to a surface every peer's commit gate reads, so parked work in the index froze every node's commits at once (2026-08-22).

## The judged patch-id is what lands

A bank is judged, by a whole-suite run in a private worktree and by a read of the diff, at one `git patch-id --stable`. The commit that lands must give the same id, read back from the applied tree before the commit is made.

- **A gate refusal is re-banked and re-judged before any commit, formatting included.** On 2026-09-17 two lines were recomposed at the gate and the landed id differed from the judged one; on 2026-09-18 a `rustfmt` refusal changed the id and the landing went in before the new id was judged. In both cases the difference had to be proved harmless afterwards, per file, which is the expensive direction. Fix it in the worktree, bank again, hand over the new id.
- **Format-check with the gate's edition before banking.** Intent's own gate checks Rust with `staged-format-guard.sh`, which runs `rustfmt --check` under the edition `native/rust/rustfmt.toml` declares (2024). The 2021 edition disagrees with it on import order, so a 2021 check passes files the gate refuses. Run `rustfmt --check` on the files where they sit, or pass `--edition 2024`.
- **When main moves after the run, the run stands for the bank only if the move is invisible to it.** Bank the diff against the new base and read the bank's path list: it must be exactly the change's own paths. A path main moved that also appears in the bank means landing it would undo main's change in that file, so apply the bank three-way onto the new base and run again. Applying the bank onto the new base and matching the tree hash proves only that the blob applies; the path list is the check that carries the claim.

## The rules around it

- **Suites run only in a private worktree, from that worktree's own build, under an isolated `HOME`.** A test binary built in the shared checkout reaches the live store from any working directory and can migrate it, and the CLI suite publishes the test binary's install root to the machine-wide home pointer (both measured 2026-09-11). Each worktree keeps its own cargo target; path dependencies collide across worktrees that share one.
- **A landing is `git apply` on main, then `git add <paths> && git commit --only <paths>` in one call.** `--only` keeps a peer's staged work out of the commit. It cannot introduce an untracked file, so a new file is added by itself first. A refused commit is re-issued with only what the refusal named changed, never recomposed.
- **Landings happen on the director's word, one lane at a time, inside an announced `CHAIN START` and `CHAIN END`.** Two chains that cross go in alphabetical node order and never yield back and forth.
- **Never edit a source under somebody else's running suite.** The tree their verdict describes stops being the tree it started on. Bank the hunks and apply them when the suite is off.
- **Banks that share a file land in the order they bank green**, stacked so one run stands for the landing.

## What a bank is not

It is not a backup of the store, which is per-machine and never enters history, and it is not a review artefact: the diff a reviewer reads is the landing commit. It is the one place a finished change can wait without being on any surface another session writes to.
