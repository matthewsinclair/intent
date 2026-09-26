    bin/devbin upgrade [--from <dir>] [--force]
    bin/devbin upgrade [--prefix <dir>]          (in devbin's own source checkout)

Two modes, chosen by the tree you run it in.

    UPGRADE          in a project: re-sync its vendored runtime from a devbin
                     source. Changes only devbin's half -- bin/devbin,
                     bin/.devbin/lib/ and the manifest -- and never touches
                     config.yaml, cmd/ or help/.
    PUBLISH-UPGRADE  in devbin's own source checkout: replace the machine install
                     that projects vendor from. Refuses on a dirty tree, and
                     refuses when there is no install yet -- use install.

    --from <dir>   UPGRADE only: the devbin to upgrade from (default: the source
                   the manifest records)
    --force        UPGRADE only: overwrite vendored files that were edited in place,
                   and in an install with no manifest, replace every one unchecked
    --prefix <dir> PUBLISH-UPGRADE only: the install to replace (default:
                   install.prefix in that checkout's bin/.devbin/config.yaml).
                   There is no built-in default: an unset prefix is refused

A flag from the other mode is refused rather than ignored. PUBLISH-UPGRADE prints a `mode:` line before it writes anything; UPGRADE names its source (`Upgrading from <dir>`) before it writes.

## The refusal is the feature

The manifest records a checksum for every file devbin owns. A vendored file whose checksum has moved was edited in place, so when it also differs from the source, upgrade REPORTS it, REFUSES it and leaves it alone.

An upgrade that always wins is one people stop running. Someone who patched a vendored file had a reason, and silently reverting it teaches them that upgrading is dangerous; reporting it starts the conversation instead. Pass --force when the edit was a mistake.

A refused file keeps the checksum it had before the edit. Re-checksumming a file upgrade had just declined to overwrite would record the edit as canonical: doctor would call it intact one line after upgrade called it edited, and the next upgrade would find nothing to refuse -- two correct-looking steps composing into a false green.

## What it reports

    added        the source ships a file that is not on disk here
    updated      re-vendored: it differed from the source and was not edited
                 here, or it was and --force was given
    converged    you edited it, and the edit is now upstream -- tracked again
    skipped      edited in place and still different from the source: REFUSED
                 (use --force)
    removed      devbin no longer ships it, and your copy was not edited here
                 (or --force was given)
    released     devbin no longer ships it and your copy was edited: without
                 --force it is left on disk and dropped from the manifest, so
                 it is now yours
    unreadable   devbin cannot read it, here or in the source: nothing is
                 written, and the run exits 1

The closing count reads `N added, N updated, N unchanged, N removed, N skipped`, where `unchanged` includes converged files and `skipped` includes released ones. A run that changed nothing says `Already up to date.`; a run that changed no vendored file but moved the recorded source or commit says so, and names the field that moved.

A file devbin cannot read refuses the whole run, before anything is written. Whether it was edited cannot be told without reading it, so neither `updated` nor `skipped` can be said of it, and a run that went on around it would leave a runtime of mixed versions. Each is named as `unreadable`, with where the unreadable copy is; make each readable, eg `chmod u+r <file>`, and run it again. A manifest it cannot read refuses the run the same way, before the source is even looked for, since what it records -- the source, and which files were edited here -- is exactly what upgrade needs from it; with the manifest read as empty, every file would look new and an edit would be overwritten unrefused.

A manifest that is not there refuses the run too, before the source is looked for, and exit 1 with nothing written. Without it every file looks new, so an edit here would be overwritten at exit 0, a fresh manifest written over it, and doctor would pass the next run. `--force` accepts that: with no record of which files were edited, it replaces every vendored file unchecked, and says so once the source is known to be a devbin. It prints no direction notice: with nothing recorded the direction is unknown, and `vendor`, the notice's remedy, refuses such a tree. `install --into <project> --force` from the devbin does the same. The test is `bin/.devbin/lib/` present with no manifest beside it; a project devbin was never installed in has no `bin/.devbin/lib/`, and `install` is the verb for it.

`released` exists because the honest answer to "you edited a file I have since deleted" is neither to remove your work nor to go on claiming ownership of it. --force does not reach a released file again: the manifest no longer lists it.

`converged` exists because equality with the source is asked BEFORE the edited-in-place check. A patch you carried that has since gone upstream matches the source exactly; refusing it would keep its pre-patch checksum and leave doctor calling it edited forever, with nothing upgrade could ever change. Recording its checksum again is the heal, and nothing is written to the file either way.

A file devbin renames only in case, `Foo` to `foo`, is one file on a disk that folds case, as macOS formats APFS. There it is updated under the new spelling, or skipped if you edited it, renamed to that spelling on disk, and reported as kept; it is never removed as a file devbin stopped shipping. On a case-sensitive disk the two spellings are two files, and `Foo` is retired like any other.

## Which way it is about to move

upgrade copies from whatever source it is given, so a --from pointing at an older devbin REVERTS the runtime, printing `updated` for each file it reverts, and leaves the manifest -- and doctor's check of it -- green afterwards. The manifest's `source_commit` is the only record of the difference, so before writing anything upgrade compares it with the commit the source holds, and warns, naming both commits, when the source is BEHIND this project or shares no history with it.

It warns rather than refuses. Rolling back deliberately is a real thing to do -- pinning a project while a regression is investigated is the obvious case -- and a tool that refused it would only be run with a flag nobody reads. What was missing was never permission; it was being told.

When the source is a machine install, the direction is looked up, not computed. An install is not a git repository, so each publish writes into the install's manifest `ancestry` lines for the earlier commits the install has held, each placed against the commit it holds now, and upgrade finds the commit this project recorded in that table. A recorded commit the table does not hold -- eg bytes vendored from a checkout at a commit the install never held, or from a different install -- reads `unknown`; that is the table declining to guess, and it heals on the next upgrade from the install.

`unknown` is not silent: when the direction cannot be established and a vendored file would change, upgrade says it cannot tell whether the source is ahead or BEHIND before it writes anything. `bin/devbin vendor` compares the two trees file by file and shows exactly what would change, whatever the direction says.

## Being told there is an update

At most once a day, and only when stderr is a terminal, `bin/devbin` says on stderr that the source this project vendored from has moved, and which way. A source that moved forward is an update and names `upgrade`; a source BEHIND this project, or different in no known direction, names `vendor` and is never offered as an update. A source that is not on this machine says nothing.

Bare `bin/devbin` and `bin/devbin doctor`, typed at a terminal, go one step further and ask whether to upgrade now -- only when the source moved forward, and anything but a yes leaves the project as it is. Any non-empty `DEVBIN_NO_UPDATE_NOTICE` (eg `DEVBIN_NO_UPDATE_NOTICE=1`) silences the notice and the offer.

## The exit status is a ruling

It stays 0 when files were refused, because this command's status answers "did I complete", and a run that reported every refusal and wrote a correct manifest did. Whether the tree is level with its source is a different question with its own command, `bin/devbin vendor`, whose exit status means only that; upgrade points at it whenever it skipped a file. A run that cannot start -- no vendored runtime here, no source recorded or named, a --from that is not a devbin, a file here or in the source or a manifest that devbin cannot read, a manifest missing without --force -- exits 1 before writing anything.

## The source

With no --from, upgrade uses the source recorded in the manifest, which install writes and every upgrade rewrites -- so after `bin/devbin upgrade --from <dir>`, that directory is the default. That is usually right and occasionally not, eg a checkout that has moved, or a project seeded from another project that has since been deleted. --from names a different one, and it is checked for a `bin/devbin` with a runtime beside it before anything is written.

## PUBLISH-UPGRADE: replacing the machine install

It shares install's publish writer and its clean-tree gate: the install's whole value is that its bytes came from a committed tree, so an upgrade cut from a dirty one would destroy exactly what it exists to provide.

An install has no author, so nothing in it is preserved: every file the source ships is copied over it, and an edited file is NAMED before it is overwritten, since a file silently replaced is a change nobody can find afterwards. A file in the install that devbin cannot read is named the same way and replaced unread, and an install with no manifest, which cannot name an edit, says that every file is being replaced unchecked; one in the source it cannot read refuses the publish, and the install is left as it was. A file the source has stopped shipping is removed from the install, and named as it goes, so no project vendored from the install receives it afterwards. A file devbin renamed only in case is kept, under the new spelling, on a disk that folds case, where the two spellings are one file.

Replacing the install moves no project: each keeps the bytes it has until it is upgraded.

## Uninstalling

There is no uninstall command. To take devbin out of a project it was installed into, remove devbin's half:

    rm bin/devbin
    rm -rf bin/.devbin/lib
    rm bin/.devbin/manifest.sha256

What is left -- config.yaml, cmd/ and help/ -- is your half of the seam, intact.

## See also

    bin/devbin help install      what gets vendored, and what does not
    bin/devbin help vendor       what an upgrade would change, before it does
    bin/devbin help doctor       checking the result
