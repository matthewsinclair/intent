    bin/devbin install [--into <dir>] [--name <n>] [--alias <a>] [--lang <l>] [--force]
    bin/devbin install [--prefix <dir>] [--managed-by brew] [--force]   (in devbin's own source checkout)

Installs devbin into a project or, run from devbin's own source checkout, publishes a machine install for projects to vendor from. A user's install is the one Homebrew places (see An install Homebrew placed, below), and a project is seeded from it with `devbin install --into <project>`; publishing is for the maintainer of a dev checkout. The mode depends on which `bin/devbin` you ran and on whether --into was given, not on the directory you stand in, and it is printed before anything is written, so a misdetection lands in the output rather than in the filesystem.

    VENDOR    install devbin into a project. Always the mode when --into is
              given, and the only mode outside devbin's own source checkout.
    PUBLISH   run from devbin's own source checkout with no --into: publish
              that checkout's runtime as its opt install, and with --force
              replace the one there, the one way to re-publish.

    --into <dir>   the project to install into (default: the current directory)
    --name <n>     the project name, lower case (default: the target directory's
                   name, lower-cased)
    --alias <a>    a short name for the project, lower case. RECORDED ONLY --
                   devbin creates nothing for it
    --lang <l>     a language the project declares; repeatable or comma-separated
    --force        VENDOR: re-vendor over an existing install. PUBLISH:
                   replace the opt install at the prefix, which must be one
                   devbin wrote
    --prefix <dir> PUBLISH only: where to publish (default: install.prefix in that
                   checkout's bin/.devbin/config.yaml, a leading ~ expanded).
                   There is no built-in default: an unset prefix is refused
    --managed-by brew
                   PUBLISH only: record that Homebrew owns this install, so
                   devbin's own install and upgrade refuse to write into it.
                   The Homebrew formula passes it

A flag that means nothing in the mode you are in is refused rather than ignored, and so is an upper-case --name or --alias.

## VENDOR: what it writes, and what it will not touch

This is the seam the whole install and upgrade design rests on:

    bin/devbin                   DEVBIN'S -- copied from the source
    bin/.devbin/lib/             DEVBIN'S -- every file the source ships
    bin/.devbin/manifest.sha256  DEVBIN'S -- a checksum of each of those files
    bin/.devbin/config.yaml      YOURS -- a starter is written only if there is none
    bin/.devbin/cmd/             YOURS -- created if missing, never written into
    bin/.devbin/help/            YOURS -- created if missing, never written into
    .gitattributes               YOURS, but for DEVBIN'S line-ending block (below)

Your config, your handlers and your authored help are never overwritten, with or without --force. The flag re-vendors devbin's half; it has no power over yours.

Within devbin's half the flag is blunt: it copies every file the source ships over what is there, edited or not, without naming the edits. It also removes each file the previous manifest recorded that the source no longer ships, edited or not, and names each one. A file devbin renamed only in case is not one of them on a disk that folds case, where the two spellings are one file: it is kept, renamed to the new spelling, and said so. A file under `bin/.devbin/lib/` that no manifest recorded is left where it is, since it may be yours, and `bin/devbin doctor` names it. Without --force, install refuses an existing install and names both `--force` and `bin/devbin upgrade`, which is the verb that keeps local edits and reports them, and hands you an edited file devbin no longer ships rather than removing it.

Every file the source ships is read before one is copied, and one devbin cannot read refuses the install, named, with nothing written. A copy that comes out unreadable here, as a umask that strips the owner's read bit makes every copy, refuses the manifest instead: the files are in place, and no manifest is written short of one it cannot checksum.

## Line endings: devbin's block in .gitattributes

A checkout that converts line endings breaks the runtime where nothing points at the cause: git on Windows checks text out with CRLF unless the repository says otherwise, a script whose first line then reads `bash\r` does not run, and every converted file reads as edited in place. So install writes one marked block into the project's root `.gitattributes`, creating the file when there is none:

    # >>> devbin: line endings >>>
    ...
    bin/devbin text=auto eol=lf
    bin/.devbin/** text=auto eol=lf
    # <<< devbin: line endings <<<

The rule is in the repository, so every clone gets it, rather than in each clone's git settings. `text=auto` leaves a binary alone, such as a compiled handler under `bin/.devbin/cmd/`. What is between the two markers is devbin's: install writes it and `bin/devbin upgrade` puts it back when it is missing or differs. Every line outside them is yours and is kept byte for byte, and another tool's block beside devbin's is that tool's. A marker standing alone, or two blocks, is refused by name with nothing written, since which lines are devbin's cannot be told. `bin/devbin doctor` asks git whether the rule holds, with `git check-attr eol`, so a later line that undoes it is found too.

Commit the file with the install. A file that already carries CR bytes is not converted by the rule; doctor names it, with the cause.

## Re-running install never renames a project

A project that already has a config.yaml keeps it exactly as it is, so --name, --alias and --lang only shape the starter config, and re-running install can never rename a project, drop its alias or change its languages -- the one destructive thing an idempotent installer could plausibly do by accident.

The starter records the name, the alias when one is given, and the --lang languages (`languages:` is left commented out when none was given), and sets `devbin_version` to `^` plus the installing version, a constraint `bin/devbin doctor` enforces.

## What a machine install is, and why it cannot be run

PUBLISH writes a project-shaped tree with the project half missing:

    bin/devbin                   the launcher
    bin/.devbin/lib/             the runtime
    bin/.devbin/manifest.sha256  checksums, plus kind: install, the commit it was
                                 cut from, ancestry lines for every commit before
                                 it (see bin/devbin help upgrade), and
                                 managed_by: brew when Homebrew placed it
    (no config.yaml, no cmd/, no help/)

That absence is the mechanism rather than tidiness. In a tree with no config.yaml whose manifest says `kind: install`, `bin/devbin` answers only `install`, `shell-init`, `use` and its version (`version`, `--version`, `-v`); every other verb is refused with exit 5, naming how to seed a project and how to upgrade one. So an install can only ever be vendored FROM, never worked IN, and there is no check to disable and no flag to pass.

    <install>/bin/devbin install --into <project>          seed a project from it
    cd <project> && bin/devbin upgrade --from <install>    move a project onto it

Because an install is project-shaped, `bin/devbin upgrade --from` and `bin/devbin vendor --from` work on it unchanged, and its manifest is the ordinary one.

A publish records nothing outside the install. Which of devbin's trees the machine reads -- the dev checkout, its opt install or the install Homebrew placed -- is chosen by `devbin use` alone (`bin/devbin help use`), whose one record is the link `~/.local/bin/devbin`. A project records only the kind of tree its bytes came from (`install`, `checkout` or `project`), never where, and its next upgrade reads the tree the switch names: `DEVBIN_INSTALL_PREFIX` when it is set, else the tree that link leads to, else the install Homebrew placed (below). `bin/devbin doctor`, run in the project, names that tree, by the line `--version` prints, and what chose it. So a publish, to `install.prefix` or to any other --prefix, moves no project until `devbin use opt` points the link at it.

The install's manifest header carries `tool: devbin`, `version` and `managed_by` (`devbin`, or `brew` for a keg) beside devbin's own keys, so every tool released through Homebrew reads its installs alike. An install whose manifest predates those keys is named by doctor with its remedy: an opt install fails doctor until `install --force` from the checkout replaces it, and a keg is a note naming `brew upgrade devbin`, since only a release that writes the new form can fix it.

## An install Homebrew placed

devbin's Homebrew formula makes a machine install like any other: at the release tag it runs devbin's own PUBLISH into the keg's `libexec`, with `--managed-by brew`, and links `bin/devbin` into Homebrew's `bin` as `devbin`. So `devbin install --into <project>` seeds a project from it, as from any install.

A project finds it at `$HOMEBREW_PREFIX/opt/devbin/libexec` when no `DEVBIN_INSTALL_PREFIX` and no `~/.local/bin/devbin` link names another tree. Those two are read first, so a machine whose link names a checkout or its opt install keeps it, and installing the formula moves no project; `devbin use brew` removes the link, and from then on the keg answers. The path is `opt/devbin`, which Homebrew moves to each new keg, never the versioned keg under `Cellar/devbin/`, which `brew upgrade` deletes; `shell-init` run from the keg bakes the `opt` path into `dvb` for the same reason. `HOMEBREW_PREFIX` is what `brew shellenv` exports, and nothing else is read: no `brew` is run and none is looked for on PATH. A process started without the login profile -- cron, a hook a GUI app launched, a bare ssh command -- has no `HOMEBREW_PREFIX`, so there the default is off, and the message that says no install was found names the variable.

Homebrew replaces a keg whole at each version and deletes the one before, so bytes devbin wrote into a keg would be lost at the next upgrade. An install whose manifest records `managed_by: brew` is therefore refused by `install` and `upgrade` alike, before anything is written, and the refusal names `brew upgrade devbin`.

## PUBLISH: why an install is cut only from a clean tree

A project vendored straight from a working tree can carry bytes that match no commit, and two vendors taken from the same dirty tree at the same commit record the same provenance line over different files, so nothing can tell them apart. So publishing refuses a dirty tree, and there is no --force for it: an install cut from a dirty checkout would launder those bytes through one more directory and give them the LOOK of provenance.

Clean means the whole tree: anything `git status --porcelain` lists refuses the publish. devbin's own half -- `bin/devbin` and the runtime beside it -- is asked harder than that: an ignored file, a symlink, or a file git was told not to watch there refuses too, reported as `seam: <state>`. A tree that is not a git repository is refused, since the commit the install would claim cannot be established, and so is a repository git cannot read, most often one another user owns, which git refuses unless `safe.directory` names it; the refusal says which of the two, and the second carries git's own first line.

`install` publishes a NEW install and refuses to overwrite one unless given --force (below); `upgrade` in the checkout publishes nothing and names `install --force`.

## PUBLISH --force: replacing the opt install

`install --force` in devbin's own checkout is the one way to re-publish. It uses the publish writer and the clean-tree gate a new publish does: the install's whole value is that its bytes came from a committed tree, so a replace cut from a dirty one would destroy exactly what it exists to provide. Only a tree devbin wrote is replaced: its manifest must say `tool: devbin`, or carry the older install form, which the replace brings up to date; anything else at the prefix is refused, and without --force an existing install is refused, naming `install --force`.

An install Homebrew placed, whose manifest records `managed_by: brew`, is refused before the gate reads anything, and the refusal names `brew upgrade devbin`: Homebrew replaces the keg whole at each version, so bytes written into it would be lost at the next one.

An install has no author, so nothing in it is preserved: every file the source ships is copied over it, and an edited file is NAMED before it is overwritten, since a file silently replaced is a change nobody can find afterwards. A file in the install that devbin cannot read is named the same way and replaced unread, and an install with no manifest, which cannot name an edit, says that every file is being replaced unchecked; one in the source it cannot read refuses the publish, and the install is left as it was. A file the source has stopped shipping is removed from the install, and named as it goes, so no project vendored from the install receives it afterwards. A file devbin renamed only in case is kept, under the new spelling, on a disk that folds case, where the two spellings are one file.

Replacing the install moves no project: each keeps the bytes it has until it is upgraded.

## Provenance is transitive

A project vendored from an install records the commit the install was cut from, not "not a git checkout": an install is not a git repository, so re-deriving the commit at the second hop would answer nothing, and the install's own stamp is forwarded instead.

## No launcher named after your project

install creates no `bin/<name>` or `bin/<alias>` link to `bin/devbin`: a launcher named after the project would shadow a binary the project builds under that name. `bin/devbin` is the invocation everywhere.

## See also

    bin/devbin help upgrade      keeping a vendored runtime current
    bin/devbin help use          choosing which of devbin's trees this machine runs
    bin/devbin help vendor       comparing it with the devbin it came from
    bin/devbin help doctor       checking an install
    bin/devbin devbin            where devbin's long-form guides are kept,
                                 including migrating an existing launcher
