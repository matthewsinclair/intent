    bin/devbin install [--into <dir>] [--name <n>] [--alias <a>] [--lang <l>] [--force]
    bin/devbin install [--prefix <dir>]          (in devbin's own source checkout)

Installs devbin into a project or, run from devbin's own source checkout, publishes a machine install for projects to vendor from. The mode depends on which `bin/devbin` you ran and on whether --into was given, not on the directory you stand in, and it is printed before anything is written, so a misdetection lands in the output rather than in the filesystem.

    VENDOR    install devbin into a project. Always the mode when --into is
              given, and the only mode outside devbin's own source checkout.
    PUBLISH   run from devbin's own source checkout with no --into: publish
              that checkout's runtime as a machine install.

    --into <dir>   the project to install into (default: the current directory)
    --name <n>     the project name, lower case (default: the target directory's
                   name, lower-cased)
    --alias <a>    a short name for the project, lower case. RECORDED ONLY --
                   devbin creates nothing for it
    --lang <l>     a language the project declares; repeatable or comma-separated
    --force        VENDOR only: re-vendor over an existing install
    --prefix <dir> PUBLISH only: where to publish (default: install.prefix in that
                   checkout's bin/.devbin/config.yaml, a leading ~ expanded).
                   There is no built-in default: an unset prefix is refused

A flag that means nothing in the mode you are in is refused rather than ignored, and so is an upper-case --name or --alias.

## VENDOR: what it writes, and what it will not touch

This is the seam the whole install and upgrade design rests on:

    bin/devbin                   DEVBIN'S -- copied from the source
    bin/.devbin/lib/             DEVBIN'S -- every file the source ships
    bin/.devbin/manifest.sha256  DEVBIN'S -- a checksum of each of those files
    bin/.devbin/config.yaml      YOURS -- a starter is written only if there is none
    bin/.devbin/cmd/             YOURS -- created if missing, never written into
    bin/.devbin/help/            YOURS -- created if missing, never written into

Your config, your handlers and your authored help are never overwritten, with or without --force. The flag re-vendors devbin's half; it has no power over yours.

Within devbin's half the flag is blunt: it copies every file the source ships over what is there, edited or not, without naming the edits. It also removes each file the previous manifest recorded that the source no longer ships, edited or not, and names each one. A file devbin renamed only in case is not one of them on a disk that folds case, where the two spellings are one file: it is kept, renamed to the new spelling, and said so. A file under `bin/.devbin/lib/` that no manifest recorded is left where it is, since it may be yours, and `bin/devbin doctor` names it. Without --force, install refuses an existing install and names both `--force` and `bin/devbin upgrade`, which is the verb that keeps local edits and reports them, and hands you an edited file devbin no longer ships rather than removing it.

Every file the source ships is read before one is copied, and one devbin cannot read refuses the install, named, with nothing written. A copy that comes out unreadable here, as a umask that strips the owner's read bit makes every copy, refuses the manifest instead: the files are in place, and no manifest is written short of one it cannot checksum.

## Re-running install never renames a project

A project that already has a config.yaml keeps it exactly as it is, so --name, --alias and --lang only shape the starter config, and re-running install can never rename a project, drop its alias or change its languages -- the one destructive thing an idempotent installer could plausibly do by accident.

The starter records the name, the alias when one is given, and the --lang languages (`languages:` is left commented out when none was given), and sets `devbin_version` to `^` plus the installing version, a constraint `bin/devbin doctor` enforces.

## What a machine install is, and why it cannot be run

PUBLISH writes a project-shaped tree with the project half missing:

    bin/devbin                   the launcher
    bin/.devbin/lib/             the runtime
    bin/.devbin/manifest.sha256  checksums, plus kind: install, the commit it was
                                 cut from, and ancestry lines for the earlier
                                 commits it has held (see bin/devbin help upgrade)
    (no config.yaml, no cmd/, no help/)

That absence is the mechanism rather than tidiness. In a tree with no config.yaml whose manifest says `kind: install`, `bin/devbin` answers only `install`, `shell-init` and its version (`version`, `--version`, `-v`); every other verb is refused with exit 5, naming how to seed a project and how to upgrade one. So an install can only ever be vendored FROM, never worked IN, and there is no check to disable and no flag to pass.

    <install>/bin/devbin install --into <project>          seed a project from it
    cd <project> && bin/devbin upgrade --from <install>    move a project onto it

Because an install is project-shaped, `bin/devbin upgrade --from` and `bin/devbin vendor --from` work on it unchanged, and its manifest is the ordinary one.

## PUBLISH: why an install is cut only from a clean tree

A project vendored straight from a working tree can carry bytes that match no commit, and two vendors taken from the same dirty tree at the same commit record the same provenance line over different files, so nothing can tell them apart. So publishing refuses a dirty tree, and there is no --force for it: an install cut from a dirty checkout would launder those bytes through one more directory and give them the LOOK of provenance.

Clean means the whole tree: anything `git status --porcelain` lists refuses the publish. devbin's own half -- `bin/devbin` and the runtime beside it -- is asked harder than that: an ignored file, a symlink, or a file git was told not to watch there refuses too, reported as `seam: <state>`. A tree that is not a git repository is refused, since the commit the install would claim cannot be established.

`install` publishes a NEW install and refuses to overwrite one; `upgrade` replaces an existing one and refuses when there is none. Whichever you reach for, the wrong one names the right one.

## Provenance is transitive

A project vendored from an install records the commit the install was cut from, not "not a git checkout": an install is not a git repository, so re-deriving the commit at the second hop would answer nothing, and the install's own stamp is forwarded instead.

## No launcher named after your project

install creates no `bin/<name>` or `bin/<alias>` link to `bin/devbin`: a launcher named after the project would shadow a binary the project builds under that name. `bin/devbin` is the invocation everywhere.

## See also

    bin/devbin help upgrade      keeping a vendored runtime current
    bin/devbin help vendor       comparing it with the devbin it came from
    bin/devbin help doctor       checking an install
    bin/devbin devbin            where devbin's long-form guides are kept,
                                 including migrating an existing launcher
