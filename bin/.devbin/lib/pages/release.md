    bin/devbin release <vX.Y.Z|--patch|--minor|--major> [--dry-run] [--yes]
    bin/devbin release status <vX.Y.Z>
    bin/devbin release notes
    bin/devbin release check
    bin/devbin release attach <vX.Y.Z> [--from <dir>]

Cut a release: stamp, commit, gate, confirm, tag, push, make the release object, then run `after:` and read the CI verdict. `release` is opt-in: a project enables it with `commands.release.enabled: true` and declares how it releases in the `release:` block of `bin/.devbin/config.yaml`, where every key has a default. Every `release:` key is annotated in devbin's reference config, and `bin/devbin devbin` names where that file and devbin's releasing guide are.

`release.verbs` says which of these a project offers, every one by default: a project whose own command cuts its releases offers `check` and `attach` with the cut off. A verb it leaves out is refused, exit 2, and `release --help` shows only the verbs offered.

## The steps

    resolve      the version and the tag
    pre-flight   the branch, the tree, the note, every remote, gh signed in
                 when the release object needs it, tmp/ ignored by git, and
                 under ci.base CI green on the base commit; every refusal
                 named in one run
    stamp        set the version, run any derived: commands, date the note,
                 regenerate the notes views
    commit       the release commit
    gates        the declared gates, on the release commit (default:
                 check all, test all)
    confirm      ask once, naming the tag, the commit and what others will see
    tag          an annotated tag whose message is the note's body
    push         the release commit and the tag, to each remote in remotes:,
                 or to every remote git lists when none is declared, each
                 read back after its push
    object       the release object -- made here (object: local, the
                 default), by the project's CI (object: ci), or none;
                 under object: local, then the assets assets.workflow's
                 run built, as release attach attaches them
    after        the declared after: commands
    ci           ask ci.query for the release tag's CI verdict, if it is
                 declared

The push step reads each remote back after pushing to it, and a remote is slowest straight after a push, so a read that does not answer is asked again every 15 seconds for about 2 minutes, the bound the push itself has. A remote that answers without the release fails the step at once, as one that does not hold the release after the push. An answer that cannot be read here, such as a branch moved to a commit this checkout does not hold, fails it at once too, as a read that could not tell whether the remote holds the release, since no later ask can clear it. One that has not answered in that time fails it as a read that could not tell, never as a remote without the release: running the same command again reads it back.

Each step with a postcondition checks it first. Running the same command again after a failure therefore continues from where it stopped, and a step already done says `already`. A cut that stopped after its tag is resumed by naming the version: once the tag exists, `--patch` would name the NEXT version, so the summary prints the command to resume with. A dry run's prints the command to cut with.

A project with a `formula` command, devbin's built-in enabled or its own handler, publishes to Homebrew, and there a released cut ends on one more line, a note (a cut whose `after:` failed or whose CI is not green ends on its remedy instead): once the tap has the tag, `brew upgrade <formula>`, then `<formula> use brew`, after which `<formula> --version` reads `<formula> <version> (brew, <sha12>)` at the released version. The switch among dev, opt and brew rests at brew, and this step puts the machine back there. The core runs no brew and points no link. The formula's name is `formula.name`, or `project.name` where that is not declared.

## CI green before the cut

With `release.ci.base: true` the order is: push the branch, wait for CI green on it, then cut; with `release.ci.tag: false` as well, the cut does not wait for the tag's run, so read that run once the cut is done and file what it finds. Pre-flight asks `ci.query` once about the base commit, the one the release commit is made on, and anything but green -- a failed run, one still going, none at all, or no answer -- refuses the cut, naming the commit and what the query said, before any stamp, commit or gate; a run still going says to wait for it and cut again. Once HEAD is the release commit, the base is the commit before it, which stands for it only while the release commit changes nothing outside what the release owns.

## Choosing the version

The bump is the one judgement in a release, so nothing infers it from commits. Name the version, or use `--patch`, `--minor` or `--major`, which bump the newest tag matching `release.tag` (by default `v{version}`) -- not the version file.

## --dry-run and --yes

`--dry-run` makes no stamp, commit, tag, push or release object and runs no `after:`. It does run the gates, for real, on HEAD, as a prediction, so it takes as long as they do. Everything else it prints as what it would do, naming anything it could not predict. Where it would make the release commit, it names that commit as the one the commit step makes, never HEAD, and plans its push to every remote, one already at HEAD included. When no step failed, its last line is its verdict, saying so and whether the cut stops to ask at the confirm step.

The confirm step asks a person. `--yes` answers it. With neither a terminal nor `--yes`, the cut stops there and exits 1, its row saying nobody was there to ask. It never guesses consent.

## The other forms

    status <v>   the step table for that version, with each step's state and
                 evidence -- changes nothing
    notes        regenerate the notes views release.notes.kind generates:
                 CHANGELOG.md and RELEASES.md for releases, none for
                 changelog, and whatever notes.run writes for command
    check        print the release: declaration, each key marked declared or
                 default, name every workflow step that writes a release
                 object under object: local, then check that every generated
                 view is current and that the project's version has a note
    attach <v>   attach the files the earliest successful run of
                 release.assets.workflow at the tag built to its release
                 object, and write one SHA256SUMS over every asset on it;
                 never makes the object, never dispatches a run, reads no run
                 once SHA256SUMS agrees with the object, and a re-run
                 continues where it stopped
    attach <v> --from <dir>
                 attach the files under <dir> instead, built on this machine,
                 through the same checks, upload, SHA256SUMS and readback: the
                 one way an asset reaches a release, which the built-in macos
                 takes for its app. With release.assets.workflow declared, the
                 run's attach must have finished first. A relative <dir> is
                 read from the project root

## Exit codes

    0    released; for a dry run, no step failed: its gates passed, and it
         predicted the rest or listed what it could not
    1    a step failed, a dry run's gates included, or the arguments were
         refused; a failed step is named, and a re-run continues from it.
         The confirm step fails this way when nobody is there to ask, as when a
         person declines, and its row says which
    2    the verb is one this project's release.verbs leaves out, refused
         before anything was read

A real cut is itself a sealed gate: its full log goes to a file under `tmp/release/` rather than being left to scroll past. A dry run makes no release, so it seals no log of its own; the gates it runs still seal theirs.

## Never moved, never deleted

The core never moves or deletes a tag, and neither should a person. A defect found after the push is fixed forward: fix it, write the next patch's note, cut again.
