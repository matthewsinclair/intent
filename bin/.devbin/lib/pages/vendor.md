    bin/devbin vendor [--from <dir>] [--all]

How do these bytes compare to the devbin they came from, and what would upgrade do about each file? Reports, and writes nothing.

    --from <dir>   the devbin to compare against (default: the source the manifest records)
    --all          list every file, including the ones that are level

Exits 0 when the tree is LEVEL with its source and 1 when it is not; non-zero is an answer, not a fault. A run that cannot compare at all -- no vendored runtime here, a --from that is not a devbin -- also exits 1 and says why on stderr. devbin's own source checkout runs its runtime in place, so there is nothing vendored to compare there, and vendor says so.

## The question doctor cannot ask

doctor compares the vendored tree against the checksums recorded by the LAST INSTALL OR UPGRADE. That is an internal check, and internal is the whole limitation: a re-vendor rewrites the files and the manifest together, so nothing in the pair can tell a local patch from a fresh copy. This command asks what the tree corresponds to OUTSIDE itself.

upgrade knows the answer and only tells you by DOING it. This tells you first.

## What it reports

    level        matches the source
    behind       differs from the source and was not edited here -- upgrade
                 would replace it
    patched      edited here -- upgrade would refuse it (--force replaces)
    converged    edited here, and the edit is now upstream
    absent       the source ships it and it is not on disk here -- upgrade
                 would add it
    orphaned     no longer shipped -- upgrade would remove it
    released     no longer shipped and edited here -- upgrade leaves it with you
    unlisted     under bin/.devbin/lib/ but neither shipped nor recorded: owned
                 by nobody
    unreadable   cannot be read here, in the source, or both, so it cannot be
                 compared -- upgrade refuses to run until it can

`level` is only listed under --all; the default report is the findings, then a `tracked, divergent, unlisted` count. Every row but `level` and `unlisted` makes the tree not level. An `unlisted` file is reported and leaves the exit status alone, since no upgrade would touch it. A comparison that examined no file at all fails rather than reading as level.

An `unreadable` file is counted as divergent, and each one is also named on stderr, where the checksum tool says why. Two copies nobody can read are not two matching copies, so a tree holding one is never level, and the closing line says to make each readable before running upgrade. A manifest it cannot read is refused outright, exit 1, naming it: the rows compare this tree with it. So is an install with no manifest, `bin/.devbin/lib/` present and `bin/.devbin/manifest.sha256` not: every file would read as new, a copy matching the source as `level` and an edit as `behind`, and which files were edited here is the half of the answer nobody has. It is refused, never read as level, and names `bin/devbin upgrade --from <dir> --force` and `install --into <project> --force`, each of which replaces every vendored file, an edit included.

## converged is why this is a cross product and not two greps

Two comparisons are in play and each is blind to what the other measures: the rows are tree-vs-MANIFEST, doctor's check, and the columns are tree-vs-SOURCE, this one's.

                  source equal    source differs    source absent
    ok            level           behind            orphaned
    modified      CONVERGED       patched           released

The cell in capitals is a file somebody edited here whose edit has since been UPSTREAMED -- what carrying a local patch until devbin takes it produces every time it works. Its bytes match the source while the manifest still records the pre-patch checksum, so doctor calls it edited and upgrade has nothing left to replace. It looks like damage and it is the system working; upgrade records its checksum again instead of refusing it.

## Direction, and why unknown is a real answer

The header names, before any verdict, the source compared against, the commit these bytes were vendored FROM, the commit the source holds now, and the direction between those two:

    same        the source is exactly the commit that was vendored
    forward     the recorded commit is an ancestor of the source's commit
    backward    the source is BEHIND what was vendored -- an upgrade from it
                goes back in history (the file rows say which bytes move)
    unrelated   neither contains the other
    unknown     the question cannot be answered from what is on disk

unknown has ordinary causes: the manifest predates the `source_commit` field; the bytes were vendored from something that named no commit; they were vendored from a tree whose devbin half was uncommitted (`+dirty`) and match no commit; a checkout source does not have the recorded commit, or is not a git checkout at all; or an install source's ancestry table does not hold it. None is exotic and none may be read as "fine": upgrade syncs from its source unconditionally, so a downgrade prints `updated` per file. So upgrade says it cannot tell, out loud, whenever the direction is unknown and a vendored byte would change.

## It is not a check gate, on purpose

Carrying local patches is a SUPPORTED state -- the manifest header says an edit is detected, not overwritten -- and a gate going red on a supported state would assert a verdict devbin does not hold.

The exit status still means something a script can use, which is the other half of the split: upgrade's status answers "did I complete", and this one answers "is this tree level with its source". Neither has to be read out of the other.

## Naming the baseline

An instrument green over an unnamed baseline certifies nothing, so the header prints what was compared before it prints any verdict. The comparison is against the source's WORKING TREE -- what upgrade would actually copy -- and the source's commit is printed beside it, marked `+dirty` when devbin's half there is uncommitted, so a stale or dirty --from is visible rather than inferred.

The direction compares commits, not bytes: a dirty source can read `same` while files differ, which is why the file rows, not the direction, say what an upgrade would change.

## See also

    bin/devbin help upgrade      acting on what this reports
    bin/devbin help doctor       the install-time half of the same question
