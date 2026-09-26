    bin/devbin doctor [<check>]

Checks this install and reports what it finds, then runs the project's own checks in `bin/.devbin/cmd/doctor.d/`. It exits 1 if anything failed. With `<check>`, it runs that one project check alone.

## What it checks

In this order:

    dispatcher      bin/devbin exists and is executable
    runtime         devbin's runtime library is present; names the version
                    running and whether it is vendored
    config          bin/.devbin/config.yaml validates: a missing
                    project.name, an unknown key, a yes-or-no key holding
                    any other word, a command line with no words in it,
                    and a release: value the block cannot take each fail
    languages       which languages project.languages arms devbin's
                    language-scoped built-ins for; a note when none
    release         a note when a release: block is declared and release
                    is not enabled
    log             every log: key sits where a run reads it: beside an
                    option's own run:, or on a command whose own line runs;
                    one that nothing reads fails, naming what runs instead
    devbin_version  the running devbin satisfies the project's
                    devbin_version; a note when none is declared
    manifest        every vendored file matches its recorded checksum, and
                    one devbin cannot read fails by name, as does a manifest
                    it cannot read or one that is missing; a file in the
                    vendored runtime that the manifest does not own is a note
    commands        how many commands are offered; fails only when there
                    are none, and notes any offered with nothing to run
    handlers        every file in bin/.devbin/cmd/ is reachable: each is
                    executable, and each <name>.d/ has an executable
                    <name> beside it (doctor.d/ excepted)
    shadowing       project names standing where devbin ships its own, as
                    a whole command or as one option; overriding is
                    supported, so each is a note
    launcher        bin/devbin is executable; a bin/<project.name> link to
                    devbin is noted as a retired alias, and a
                    project.alias shortcut chained through that link is
                    noted with the command that repoints it
    source          whether the devbin this project was vendored from has
                    moved: level, behind, ahead, differs or unknown, each
                    said apart; only behind suggests an upgrade
    bash            which bash is running, against the >= 5 floor

## Three outcomes, not two

    ok      the check passed
    FAIL    broken; the run exits 1
    note    true, worth saying, and not a fault

The note is the load-bearing one. devbin offers `build` everywhere by default, as a name for the project to fill with its own `run:` or handler, and until it does, doctor notes it as offered with nothing to run, and says for each name what would make it run. That is the design working, and calling it a fault would turn doctor red on a correct install; a tool whose clean state is red trains people to ignore it.

## The count

    <n> checks, <f> failed, <m> notes

A note is not a check: it is reported and not counted. A run that counts no checks at all fails on that alone rather than exiting 0 with a clean bill, because doctor is the easiest place in the framework to write a checker that examines nothing and pronounces the patient well.

The count moves from project to project -- a `project.alias` shortcut pointing straight at `bin/devbin`, a source that is level, and every project check each add to it -- so no number is written here.

A config that does not parse is refused by `bin/devbin` itself, naming the line, before doctor starts. One that parses but fails validation fails the config check, which names each problem beneath it, and the rest of the list still runs and is counted.

A vendored file devbin cannot read, such as one at mode 000, fails the manifest check the same way: it is named as `unreadable`, never as edited, since nobody can tell whether it was, and the rest of the list still runs and is counted. So does the manifest itself when devbin cannot read it: it fails by name rather than reading as a manifest with nothing in it. Make each readable before anything else: `bin/devbin upgrade` refuses to run while one is not.

An install with no manifest at all -- `bin/.devbin/lib/` present and `bin/.devbin/manifest.sha256` not -- fails the same check: no vendored file can be checked, so whether one was edited here cannot be told, and it is never read as clean. `devbin_version` fails beside it, running `unknown`, since the runtime's version is read from the manifest and never from the project's own VERSION. The fix replaces every vendored file, an edit included: `bin/devbin upgrade --from <dir> --force`, or a re-install with `install --into <project> --force`. A plain `upgrade` refuses it, so devbin_version's own fix points at the manifest's rather than at `upgrade`. A project devbin was never installed in has no `bin/.devbin/lib/` and is not this case: a first install writes the manifest.

## Your project's own checks

Every executable in `bin/.devbin/cmd/doctor.d/` runs after the built-in checks, in name order, from the project root with no stdin, and is counted into the same summary. `bin/devbin doctor <check>` runs one alone and skips the built-in checks; a name that is not an executable there is refused.

A line whose first word is `ok`, `FAIL` or `note` counts exactly as doctor's own do, under the check's name, and any other line it prints, stderr included, is shown beneath as detail. The exit status is read as well, and neither hides the other:

    a FAIL line                    fails, even when the check exits 0
    a non-zero exit, no FAIL line  fails, naming the exit status
    exit 0, no ok or FAIL line     one pass, by the check's name

A file there that is not executable, or a directory, is a note and is not run. A check is not time-limited, so one that reaches another machine should bound itself.

## The upgrade offer

At a terminal, `bin/devbin doctor` and bare `bin/devbin` first ask whether to upgrade, and only when the devbin this project was vendored from has moved forward past it -- the `behind` state above. A source that is behind the project, or differs in no known direction, is never offered: nothing says it is newer, and an upgrade from an older source can take the project backwards. Only `y` or `yes`, in any case, runs `bin/devbin upgrade`; any other answer, or none, leaves the project as it is. `DEVBIN_NO_UPDATE_NOTICE=1` turns the offer off, along with devbin's once-a-day update notice.

## What it does not check

Whether `project.languages` matches the languages another tool records for the project -- Intent's list, for one. They are different facts and are not meant to match: `project.languages` is gate scope, the languages devbin arms its language-scoped built-ins for here, while Intent's list records what the source is written in.

## See also

    bin/devbin help --why       the listing, plus what is not here and why
    bin/devbin help upgrade     re-syncing the vendored runtime
    bin/devbin help vendor      comparing the vendored runtime with its source
