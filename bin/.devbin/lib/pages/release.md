    bin/devbin release <vX.Y.Z|--patch|--minor|--major> [--dry-run] [--yes]
    bin/devbin release status <vX.Y.Z>
    bin/devbin release notes
    bin/devbin release check

Cut a release: stamp, commit, gate, confirm, tag, push, make the release object, then run `after:` and read the CI verdict. `release` is opt-in: a project enables it with `commands.release.enabled: true` and declares how it releases in the `release:` block of `bin/.devbin/config.yaml`, where every key has a default. Every `release:` key is annotated in devbin's reference config, and `bin/devbin devbin` names where that file and devbin's releasing guide are.

## The eleven steps

     1  resolve      the version and the tag
     2  pre-flight   the branch, the tree, the note, every remote, gh signed in
                     when the release object needs it, and tmp/ ignored by git;
                     every refusal named in one run
     3  stamp        set the version, run any derived: commands, date the note,
                     regenerate the notes views
     4  commit       the release commit
     5  gates        the declared gates, on the release commit (default:
                     check all, test all)
     6  confirm      ask once, naming the tag, the commit and what others will see
     7  tag          an annotated tag whose message is the note's body
     8  push         the release commit and the tag, to each remote in remotes:,
                     or to every remote git lists when none is declared
     9  object       the release object -- made here (object: local, the
                     default), by the project's CI (object: ci), or none
    10  after        the declared after: commands
    11  ci           ask ci.query for the release tag's CI verdict, if it is
                     declared

Each step with a postcondition checks it first. Running the same command again after a failure therefore continues from where it stopped, and a step already done says `already`. A cut that stopped after its tag is resumed by naming the version: once the tag exists, `--patch` would name the NEXT version, so the summary prints the command to resume with.

## Choosing the version

The bump is the one judgement in a release, so nothing infers it from commits. Name the version, or use `--patch`, `--minor` or `--major`, which bump the newest tag matching `release.tag` (by default `v{version}`) -- not the version file.

## --dry-run and --yes

`--dry-run` makes no stamp, commit, tag, push or release object and runs no `after:`. It does run the gates, for real, on HEAD, as a prediction, so it takes as long as they do. Everything else it prints as what it would do, naming anything it could not predict.

Step 6 asks a person. `--yes` answers it. With neither a terminal nor `--yes`, the cut stops there and exits 1, its row saying nobody was there to ask. It never guesses consent.

## The other forms

    status <v>   the step table for that version, with each step's state and
                 evidence -- changes nothing
    notes        regenerate the notes views release.notes.kind generates:
                 CHANGELOG.md and RELEASES.md for releases, none for
                 changelog, and whatever notes.run writes for command
    check        print the release: declaration, each key marked declared or
                 default, then check that every generated view is current and
                 that the project's version has a note

## Exit codes

    0    released; for a dry run, the gates passed and the rest was predicted
    1    a step failed, a dry run's gates included, or the arguments were
         refused; a failed step is named, and a re-run continues from it.
         Step 6 fails this way when nobody is there to ask, as when a
         person declines, and its row says which

A real cut is itself a sealed gate: its full log goes to a file under `tmp/release/` rather than being left to scroll past. A dry run makes no release, so it seals no log of its own; the gates it runs still seal theirs.

## Never moved, never deleted

The core never moves or deletes a tag, and neither should a person. A defect found after the push is fixed forward: fix it, write the next patch's note, cut again.
