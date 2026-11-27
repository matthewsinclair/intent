# Banking work: landing changes from a shared checkout

**When several sessions work in one checkout, a finished change that may not land yet is kept in a bank: a patch stored as a git blob and named by a ref under `refs/bank/`.** The index, the stash, a temporary directory and a commit each lose work in this case (below).

This page is policy for Intent's own repository, and for any project where several coding agents, or agents and people, share one checkout under the whiteboard node model.

## What a bank is

A bank is the change as a patch, `git diff --binary` against the commit it was made on, written into the repository's object store with `git hash-object -w` and named by a ref: `refs/bank/<node>/<topic>/<name>`. The ref points at a **blob**, not at a commit and not at a branch.

- It lives in `.git`, so it survives the removal of a worktree, the cleanup of a scratch directory and a reboot. It lives in this repository's `.git` only. A clone does not fetch `refs/bank/*`, so a bank is lost with that `.git`, and another clone gets it only by fetching or pushing it by name.
- It is not a branch and not a commit, so it never appears in `git branch`, `git status` or `git log`, and `git log --all` still works.
- `git push`, `git push --all` and `git push --tags` do not send it. `git push --mirror` does, and so does a remote configured with `mirror = true` or a push refspec that covers `refs/*`.
- It carries no index state and no stash state, so it touches nothing that a peer's commit gate reads.
- A bank keeps no reflog. Git logs `refs/heads/`, `refs/remotes/`, `refs/notes/` and `HEAD`, not `refs/bank/`. So `git update-ref -d` on a bank leaves only an unreachable blob, which `git gc` prunes after `gc.pruneExpire` (two weeks by default), or at once with `--prune=now`. **Never delete a bank ref, spent or not** (vc's ruling): once `git gc` prunes its blob, nothing brings it back.

`git for-each-ref refs/bank/` lists the banks, each with its object type `blob`. After `git gc`, most of them are in `.git/packed-refs`, not loose under `.git/refs/bank/`. Both forms are the same ref.

Every bank names its **base**, the commit the patch applies to, in the message that announces it or in a note banked beside it. Nobody can apply a patch that has no base.

## The recipe

In the private worktree where the change was built, with `<base>` the commit it was built on:

```
git add -A
git update-ref refs/bank/<node>/<topic>/<name> "$(git diff --cached --binary <base> | git hash-object -w --stdin)"
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply -R --check
git cat-file -p refs/bank/<node>/<topic>/<name> | git patch-id --stable
```

The third line checks that the bank reverses cleanly on the tree it was taken from, so every change it carries is in that tree. It cannot see a change that the tree holds and the bank lacks, eg a file that `git add -A` skipped as ignored. To prove that they are equal, apply the bank to `<base>` in a scratch index (`git read-tree <base>`, then `git apply --cached`, both under `GIT_INDEX_FILE`) and compare `git write-tree` with the real index's. The fourth line prints the id that the change is judged by.

To list, read and apply one:

```
git for-each-ref refs/bank/ --format='%(objecttype) %(refname)'
git cat-file -p refs/bank/<node>/<topic>/<name>
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply --check
git cat-file -p refs/bank/<node>/<topic>/<name> | git apply
```

`git apply` on the base puts the change in the working tree and not in the index, which is what a landing needs.

## Why the alternatives are refused

- **A temporary directory.** A reboot clears `/private/tmp`, with every scratch directory and every worktree kept in it.
- **A worktree commit.** The gate runs in a linked worktree and judges its index (issue 0644). In Intent's own repository the thread view skew check then refuses a commit from a worktree with no build of its own, until `bin/devbin build all` runs in it. A commit that passes lives only as long as its worktree: once the worktree goes, nothing refers to a detached commit. Nobody bypasses the gate, so a bank stays the place to park.
- **`git stash`.** A worktree's stash is the whole repository's stash, which every session shares. It also refuses a tree with intent-to-add entries, and the `pop` after a refused `stash` then takes somebody else's entry.
- **The shared index.** `git add` writes to a surface that every peer's commit gate reads, so work parked in the index stops every seat's commits.

## The judged patch-id is what lands

A bank is judged at one `git patch-id --stable`, by a whole-suite run in a private worktree and by a read of the diff. The commit that lands must give the same id, read back from the applied tree before the commit is made.

- **After a gate refusal, bank again and judge again before any commit, formatting included.** Fix the change in the worktree, bank it again, and hand over the new id. A landing whose id differs from the judged one must then be proved harmless file by file, which costs more.
- **Format-check with the gate's edition before you bank.** Intent's gate checks Rust with `staged-format-guard.sh`, which runs `rustfmt --check` under the edition that `native/rust/rustfmt.toml` declares (2024). The 2021 edition orders imports differently, so a 2021 check passes files that the gate refuses. Run `rustfmt --check` on the files where they are, or pass `--edition 2024`.
- **When main moves after the run, the run stands for the bank only if the move cannot touch it.** Bank the diff against the new base and read the bank's path list: it must hold the change's own paths and no others. If a path that main moved is also in the bank, landing the bank would undo main's change in that file, so apply the bank three-way onto the new base and run again. A matching tree hash proves only that the blob applies. The path list is the check that carries the claim.

## The rules around it

- **A bank is judged in a private worktree, from that worktree's own build.** Run the suite through `bin/devbin`, and where `intent estate status` shows that the estate has a controller, ask it for a slot first (`intent estate slots ask`). Each worktree keeps its own cargo target, because path dependencies collide across worktrees that share one.
- **A landing is `git apply` on main, then `git add <paths> && git commit --only <paths>` in one call.** `--only` keeps a peer's staged work out of the commit. It cannot add an untracked file, so add a new file by itself first. Re-issue a refused commit with only what the refusal named changed; never recompose it.
- **Work lands on the mediator's word.** Changes that do not touch each other are stacked, proved in one run and landed together (`intent wb profile terms`). Git's index lock orders the commits.
- **Never edit a source under somebody else's running suite.** The tree that their verdict describes would no longer be the tree their run started on. Bank the hunks, and apply them when the suite has ended.
- **Banks that share a file land in the order they bank green**, stacked so that one run stands for the landing.

## What a bank is not

A bank is not a backup of the store, which is per machine and never enters history. It is not a review artefact: the diff a reviewer reads is the landing commit. It is the one place where a finished change can wait without being on a surface that another session writes to.
