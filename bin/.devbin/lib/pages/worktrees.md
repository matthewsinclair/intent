    bin/devbin worktrees [report] [--repo <dir>] [<path>...]
    bin/devbin worktrees remove-landed <path>...

Report which linked worktrees of this repository hold work its branch already has, and remove the ones that provably do. A unit built in a worktree of its own is held there until it lands, and nothing ends the hold after: the landing reads the bank, not the worktree. So landed worktrees pile up, each with its own build target, and removing one by hand is risky, since a session standing in it loses its directory and `--force` destroys unlanded work whenever the measure was wrong.

    report          one block per linked worktree, or only the ones named:
                    its verdict and the evidence for it; writes nothing
    --repo <dir>    report on the repository at <dir>, with this project's keys
    remove-landed   remove each named worktree that reads landed, refusing
                    every other, naming why

`report` is the default, so a bare `bin/devbin worktrees` reports on every linked worktree, then on the plain directories beside them. It reads the list from the repository the worktrees share, so it answers the same from the main worktree or a linked one. `report` exits 0 when it read every worktree, whatever the verdicts, and 1 when a read failed, naming the worktree and git's words, or a path named is no linked worktree. Outside a git checkout `worktrees` is not available, exit 2. `--repo` naming a path that is no directory, or a directory that is no git checkout, is refused as an operand error, exit 1. A project root that is no repository itself but lies inside one reads that repository's worktrees.

## The evidence

Each block opens with the worktree's path, from the main worktree when it lies inside it, and its verdict, then a `why` line for every reason found, then one line per piece of evidence:

    head        its HEAD, its branch or detached, and whether the branch holds it
    base        the merge-base of its HEAD and the branch
    commits     its own commits, by git cherry: - has an equivalent on the
                branch by patch-id, + has none; with any +, the content test
                over base..HEAD as well, so both landed tests are shown
    staged      the diff of HEAD to the index: its files, its patch-id, the
                branch commit or the bank with the same change, and the
                content test
    unstaged    the diff of the index to the working tree, the same fields
    untracked   each untracked file, linked build targets aside
    ignored     each ignored file or directory, with its size and no reason:
                a plain git worktree remove deletes them, --force or not
    linked      each untracked link that leads out of the worktree to a
                directory: a build target linked in from another tree
    stash       each stash entry naming it
    processes   each process whose working directory is the worktree or under it
    locked      git's lock, and its reason
    index       its index's last write, in UTC

The content test asks whether the branch holds a diff. In a scratch index of the branch's tree, made outside the project and removed when the run ends, the diff is reverse-applied: when it applies, the branch holds its content; when it applies forward instead, the branch lacks it; when it does neither, it conflicts. The change test asks whether a branch commit or a bank is the same change, by `git patch-id --stable`: the branch's commits from the worktree's base to its tip, never a span of dates, and every bank. A bank that is a blob is read as a patch, and one that is a commit as its diff against its first parent. The two are different questions, and a unit landed with edits can pass the first and fail the second. Every diff they read is git's own: no external diff, no textconv, no colour and git's `a/` and `b/` prefixes, whatever the repository's or your config says, and `GIT_EXTERNAL_DIFF` is not followed.

A stash records the branch it was made on, never the worktree: a worktree on a branch is named by a stash made on that branch, and a detached one by a stash made detached whose first parent is its HEAD, so detached worktrees at one commit are named by one another's stash, and each reads held: the safe failure, since a held worktree is only left standing. The process check reads each process's directory with `lsof`, or from `/proc` on Linux where there is no `lsof`; with neither it cannot tell, and says so. The report's own processes are left out, and every other is named, the caller's own shell included. A process of another user that this one cannot read is not seen.

## The verdicts

Every reason found is printed, and the verdict is the strongest of them: held, then unknown, then landed.

- **landed** only when every check holds: nothing unstaged; nothing untracked but linked build targets; no stash entry names it; no process stands in it, and the process check could tell; every own commit is on the branch by ancestry or has an equivalent there by patch-id; and a staged diff, when there is one, is content the branch holds and a change a branch commit or a bank carries.
- **held** when a check fails over content the branch lacks: own commits with no equivalent whose content the branch lacks; a staged or unstaged diff that no branch commit and no bank carries and whose content the branch lacks; a staged diff a branch commit made whose content the branch no longer holds; a stash entry naming it.
- **unknown**, with its evidence, where only the author can say: a staged diff a bank has and the branch does not hold, since only the unit knows whether that bank is live or superseded; a staged diff the branch holds that no branch commit and no bank carries, or one that conflicts with the branch, which may have landed with edits; own commits that read `+` whose content the branch holds; landed content left unstaged; untracked files, which may be real work; a process standing in it, a live session or a stale shell alike; a process check that could not tell.

A worktree kept for later reads landed, since no check can tell it from a finished one. Mark one with `git worktree lock --reason <why>`: the report shows the lock and its reason, and `remove-landed` refuses it. A worktree whose directory is gone reads **prunable**, with no other evidence; `git worktree prune` clears its record.

## Plain directories

A directory that holds a linked worktree and lies below the main worktree is looked in, and every directory there that git lists as no worktree is reported with its size and no verdict. The main worktree itself is never looked in, so a worktree made directly inside it does not list the project's own directories, and nothing outside it is looked at. With paths named, only those worktrees are reported, and no plain directory.

## remove-landed

Each path named is taken in order, and the evidence is read again for it, never taken from an earlier report:

1. It is refused, naming why, when it is not a linked worktree of this repository, when it is the main worktree, when its directory is gone, when it is locked, when you are standing in it or under it, or when the run itself is inside it: the project, its repository or the devbin running.
2. Anything but landed is refused, with its reasons.
3. Each linked build target is removed as the link alone, with `rm --` on the link's own path, and the evidence is read again: it must read landed with nothing untracked.
4. A staged diff, which the content test has just shown the branch holds, is dropped with `git -C <path> reset --hard`.
5. A plain `git worktree remove <path>`, never `--force`. A refusal by git is reported in git's words as a finding, and the run goes on to the next path.

Those are its only writes, but the remove deletes the worktree's ignored files with it, which the report lists. There is no form that removes every landed worktree at once: each is removed by its author, who names it. It exits 0 when every path named was removed, and 1 when any was refused. `--repo` is report's alone, and `remove-landed` refuses it: removal stays with the author, in the author's own project.

## Configuration

    worktrees:
      branch: main        # the branch landed work reaches
      banks:              # ref patterns whose refs are banks; none by default
        - refs/bank

`worktrees.branch` is the branch landed work reaches, read as `refs/heads/<branch>`; a project whose main branch is not `main` sets it. `worktrees.banks` lists ref patterns as `git for-each-ref` reads them: `refs/bank` matches every ref beneath it, at any depth, where `refs/bank/*` matches one level only. A bank is a commit, read as its diff, or a blob, read for every patch it holds: one that carries several, each opened by a `commit <sha>` or `From <sha>` line as `git log -p` and `git format-patch` write them, matches each as that bank's, and the report's banks line names it with its count. With none declared, the change test asks the branch's commits alone.
