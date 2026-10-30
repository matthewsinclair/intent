    bin/devbin upgrade [--from <dir>] [--force]

Re-syncs a project's vendored runtime from a devbin source. It changes only devbin's half -- bin/devbin, bin/.devbin/lib/ and the manifest -- and devbin's line-ending block in .gitattributes, and never touches config.yaml, cmd/ or help/. It names its source (`Upgrading from <dir>`) before it writes.

    --from <dir>   the devbin to upgrade from (default: the tree this machine's
                   switch names -- see The source)
    --force        overwrite vendored files that were edited in place, and in an
                   install with no manifest, or one whose manifest records no
                   files, replace every one unchecked

In devbin's own source checkout it is refused, and writes nothing: a checkout publishes its opt install with `install`, and replaces it with `install --force` (`bin/devbin help install`).

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

Devbin's line-ending block in the project's `.gitattributes` (`bin/devbin help install`) is put back when it is missing or differs, and said on its own line: `wrote` when the file was not there, `added` when it was without the block, `updated` when the block differed. A block as written is kept without a word. A marker standing alone, or two blocks, fails the run after the runtime is upgraded, naming the file, with the file as it was.

The closing count reads `N added, N updated, N unchanged, N removed, N skipped`, where `unchanged` includes converged files and `skipped` includes released ones. A run that changed nothing says `Already up to date.`, and one that changed only `.gitattributes` says so; a run that changed no vendored file but moved the recorded source or commit says so, and names the field that moved.

A file devbin cannot read refuses the whole run, before anything is written. Whether it was edited cannot be told without reading it, so neither `updated` nor `skipped` can be said of it, and a run that went on around it would leave a runtime of mixed versions. Each is named as `unreadable`, with where the unreadable copy is; make each readable, eg `chmod u+r <file>`, and run it again. A manifest it cannot read refuses the run the same way, before the source is even looked for, since what it records -- the source, and which files were edited here -- is exactly what upgrade needs from it; with the manifest read as empty, every file would look new and an edit would be overwritten unrefused.

A manifest that is not there refuses the run too, before the source is looked for, and exit 1 with nothing written. Without it every file looks new, so an edit here would be overwritten at exit 0, a fresh manifest written over it, and doctor would pass the next run. `--force` accepts that: with no record of which files were edited, it replaces every vendored file unchecked, and says so once the source is known to be a devbin. It prints no direction notice: with nothing recorded the direction is unknown, and `vendor`, the notice's remedy, refuses such a tree. `install --into <project> --force` from the devbin does the same. The test is `bin/.devbin/lib/` present with no manifest beside it; a project devbin was never installed in has no `bin/.devbin/lib/`, and `install` is the verb for it. A manifest that records no files, cut to its header or to nothing, is refused and forced the same way: it vouches for no file, so every file would look new, just as with none. Its header may still record the commit, so a forced run says the direction, but it does not send the reader to `vendor`, which refuses that manifest too.

`released` exists because the honest answer to "you edited a file I have since deleted" is neither to remove your work nor to go on claiming ownership of it. --force does not reach a released file again: the manifest no longer lists it.

`converged` exists because equality with the source is asked BEFORE the edited-in-place check. A patch you carried that has since gone upstream matches the source exactly; refusing it would keep its pre-patch checksum and leave doctor calling it edited forever, with nothing upgrade could ever change. Recording its checksum again is the heal, and nothing is written to the file either way.

A file devbin renames only in case, `Foo` to `foo`, is one file on a disk that folds case, as macOS formats APFS. There it is updated under the new spelling, or skipped if you edited it, renamed to that spelling on disk, and reported as kept; it is never removed as a file devbin stopped shipping. On a case-sensitive disk the two spellings are two files, and `Foo` is retired like any other.

## Which way it is about to move

upgrade copies from whatever source it is given, so a --from pointing at an older devbin REVERTS the runtime, printing `updated` for each file it reverts, and leaves the manifest -- and doctor's check of it -- green afterwards. The manifest's `source_commit` is the only record of the difference, so before writing anything upgrade compares it with the commit the source holds, and warns, naming both commits, when the source is BEHIND this project or shares no history with it.

It warns rather than refuses. Rolling back deliberately is a real thing to do -- pinning a project while a regression is investigated is the obvious case -- and a tool that refused it would only be run with a flag nobody reads. What was missing was never permission; it was being told.

When the source is a machine install, the direction is looked up, not computed. An install is not a git repository, so each publish writes into the install's manifest `ancestry` lines for every commit the publishing checkout's HEAD contains, as `forward`, and for the earlier commits the install has held, each placed against the commit it holds now, and upgrade finds the commit this project recorded in that table. The whole history is there because an install Homebrew places is a new install at every version, with no earlier publish to carry a table from: without it, every project vendored from one release would read the next as a difference rather than an update. A recorded commit the table does not hold -- eg bytes vendored from a checkout at a commit the install never held, or from a different install -- reads `unknown`; that is the table declining to guess, and it heals on the next upgrade from the install.

`unknown` is not silent: when the direction cannot be established and a vendored file would change, upgrade says it cannot tell whether the source is ahead or BEHIND before it writes anything. `bin/devbin vendor` compares the two trees file by file and shows exactly what would change, whatever the direction says.

## Being told there is an update

At most once a day, and only when stderr is a terminal, `bin/devbin` says on stderr that the source this project vendored from has moved, and which way. A source that moved forward is an update and names `upgrade`, unless no vendored file changed, when nothing is said; a source BEHIND this project, or different in no known direction, names `vendor` and is never offered as an update. A source that is not on this machine says nothing.

Bare `bin/devbin` and `bin/devbin doctor`, typed at a terminal, go one step further and ask whether to upgrade now -- only when the source moved forward and changed a vendored file, and anything but a yes leaves the project as it is. Any non-empty `DEVBIN_NO_UPDATE_NOTICE` (eg `DEVBIN_NO_UPDATE_NOTICE=1`) silences the notice and the offer.

## The exit status says whether it completed

It stays 0 when files were refused, because this command's status answers "did I complete", and a run that reported every refusal and wrote a correct manifest did. Whether the tree is level with its source is a different question with its own command, `bin/devbin vendor`, whose exit status means only that; upgrade points at it whenever it skipped a file. A run that cannot start -- no vendored runtime here, no source named and none recorded that this machine can place, a --from that is not a devbin, a file here or in the source or a manifest that devbin cannot read, a manifest missing without --force -- exits 1 before writing anything.

## The source

With no --from, upgrade reads the tree this machine's switch names. The manifest records the KIND of tree its bytes came from, never where that tree is: `install`, `checkout` or `project`. The manifest is tracked in every project that vendors devbin, and a path in it named one machine's layout in all of them. The kind says where the bytes came from, never where the next upgrade goes: for every kind, the source is `DEVBIN_INSTALL_PREFIX` when it is set, else the tree the link `~/.local/bin/devbin` leads to, read by the link's own path, so cron and a hook with no login PATH still find it, else the install Homebrew placed, at `$HOMEBREW_PREFIX/opt/devbin/libexec`. The link is what `devbin use` writes (`bin/devbin help use`), so `use dev` makes the next upgrade vendor from the checkout, which the manifest then records as `checkout`, with `+dirty` when that tree is dirty. When nothing names a source here, upgrade says what is missing and asks for --from, with the remedy for the cause: export `HOMEBREW_PREFIX` as `brew shellenv` exports it when it is unset or relative, or install devbin with Homebrew when it names a prefix with no devbin under it, and a link that leads nowhere, or to a tree that is neither an install nor devbin's checkout, or whose manifest cannot be read, is named as such rather than read as absent. A project that records no source at all asks for --from. A source DEVBIN_INSTALL_PREFIX places is taken as given, so a refusal of that tree, as no devbin or as one that cannot say what kind of tree it is, says the variable named it. Vendoring from a tree other than the one the switch names says so as it writes, since a later upgrade without --from reads the switch's tree. A source that is no checkout and has no manifest to say what it is is refused, since recording `project` for it would be a guess. A manifest written before this recorded a path, which still reads, and the kind replaces it only at an upgrade this runtime runs: an upgrade is run by the project's own vendored runtime, so the upgrade that installs this runtime over an older one is run by the older one and records the path again. A project upgraded from such a runtime needs a second upgrade, run by the new one, before its manifest records the kind. --from names a different source, and it is checked for a `bin/devbin` with a runtime beside it before anything is written. A relative --from is read from the directory you run upgrade in, as your shell reads it there: an exported CDPATH is never searched for it, and `--from -` names a directory called `-` there, never the one you were in before.

## Uninstalling

There is no uninstall command. To take devbin out of a project it was installed into, remove devbin's half:

    rm bin/devbin
    rm -rf bin/.devbin/lib
    rm bin/.devbin/manifest.sha256

and the lines from `# >>> devbin: line endings >>>` to `# <<< devbin: line endings <<<` in `.gitattributes`.

What is left -- config.yaml, cmd/ and help/ -- is your half of the seam, intact.

## See also

    bin/devbin help install      what gets vendored, and what does not
    bin/devbin help use          the switch an upgrade reads its source from
    bin/devbin help vendor       what an upgrade would change, before it does
    bin/devbin help doctor       checking the result
