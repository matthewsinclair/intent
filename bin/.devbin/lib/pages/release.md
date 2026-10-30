    bin/devbin release <vX.Y.Z|--patch|--minor|--major> [--dry-run] [--yes]
    bin/devbin release status <vX.Y.Z>
    bin/devbin release notes
    bin/devbin release check
    bin/devbin release attach <vX.Y.Z> [--from <dir>]
    bin/devbin release snapshot <vX.Y.Z | --commit <rev>> <new-dir> [--no-parent]
    bin/devbin release publish <vX.Y.Z> [--expect <sha>] [--dry-run] [--yes]

Cut a release: stamp, commit, gate, confirm, tag, push, make the release object, then run `after:` and read the CI verdict. `release` is opt-in: a project enables it with `commands.release.enabled: true` and declares how it releases in the `release:` block of `bin/.devbin/config.yaml`, where every key has a default. Every `release:` key is annotated in devbin's reference config, and `bin/devbin devbin` names where that file and devbin's releasing guide are.

`release.verbs` says which of these a project offers, every one by default: a project whose own command cuts its releases offers `check` and `attach` with the cut off. A verb it leaves out is refused, exit 2, and `release --help` shows only the verbs offered.

## The steps

    resolve      the version and the tag
    pre-flight   the branch, the tree, the version, the note, every remote,
                 gh signed in when the release object needs it, tmp/ ignored
                 by git, under ci.base CI green on the base commit, and under
                 release.publish.url no remote it pushes to naming the publish
                 repository; every refusal named in one run
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
    snapshot     release publish's, with candidate and publish: skipped
                 without release.publish.url, and with it, pending on release
                 publish, where a cut stops (Publishing a snapshot, below)
    candidate    the snapshot on its candidate branch, and CI green on it
    publish      the publish branch and the tag at the snapshot
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

## Publishing a snapshot

A project that declares `release.publish.url` publishes each release as a snapshot of its product, the commit `release snapshot` makes of the tag, and never as its development repository. Its cut pushes to `release.remotes` alone, refusing one that names the publish repository by OWNER/NAME on any host or as the same path, or whose url names a host and no readable OWNER/NAME, and stops after the push step: the snapshot, candidate and publish rows read `pending` on `release publish <v>`, and the object, after and ci rows read `not reached`, since each reads the public repository, which holds nothing of the release yet. A project that does not declare it reads those three rows as `skipped`, and cuts as it always has.

`release publish <v>` makes the tag's snapshot public. Each step reads its postcondition first, so running it again continues where it stopped:

    resolve      the tag is here: the commit the snapshot is made of
    pre-flight   no remote of this project names the publish repository, by
                 OWNER/NAME on any host, an ssh host alias included; the
                 tag's commit carries the release and holds the list;
                 ci.query is declared, and release.repo where the release
                 object needs gh; and the url answers, its publish branch,
                 candidate branch and tag read. Every refusal is named, and
                 nothing is written
    snapshot     the snapshot, made in the run's scratch on the public tip,
                 where a tip that is this snapshot already is found; --expect
                 refuses any other sha, and a public tag or a candidate
                 branch that holds another commit is refused here, before
                 anything is pushed
    confirm      ask once, naming the snapshot and what others will see
    candidate    push the snapshot to its candidate branch
                 (release.publish.candidate, default candidate/{tag}), read
                 it back, then ask ci.query about its sha, with no tag, for
                 up to ci.wait; only green goes on
    publish      fast-forward the publish branch to the snapshot, then push
                 the tag at it, lightweight, each read back; then delete the
                 candidate branch and read back that it is gone
    object       as a cut's, against release.repo, the public repository
    after        as a cut's
    ci           as a cut's, about the snapshot's sha

Nothing is forced. A red verdict, a run still going at `ci.wait`, or no verdict at all stops the run at the candidate step, and the publish branch and the tag stay where they were: a red candidate stays as the record of it, and the fix is the next version, while a run still going is read again by running the same command. A public tag already at another commit is refused, since a tag is never moved, and so is a publish branch that moved after the snapshot was made on it: running again makes the snapshot on the new tip, a new sha.

`--expect <sha>` takes the snapshot's whole sha, as `release snapshot` prints it, and refuses a snapshot of any other, so what goes public is the commit a person approved. The same inputs give the same snapshot, so the sha read before the run is the sha the run makes.

Every call to the public repository is made from a repository of the run's own, so the url is read as the person's git reads it, through no remote of this project: a push can go through `url.<base>.pushInsteadOf` in their own git config. The candidate's CI must run the whole matrix a tag runs, so the project's workflow runs on the candidate branch. With `release.publish.url` declared, `release.ci.base` is refused, since CI on the candidate is what stands before anything is public, and the built-in `formula` reads its revision from the public repository's tag.

## Choosing the version

The bump is the one judgement in a release, so nothing infers it from commits. Name the version, or use `--patch`, `--minor` or `--major`, which bump the newest tag matching `release.tag` (by default `v{version}`) -- not the version file.

## --dry-run and --yes

`--dry-run` makes no stamp, commit, tag, push or release object and runs no `after:`. It does run the gates, for real, on HEAD, as a prediction, so it takes as long as they do. Everything else it prints as what it would do, naming anything it could not predict. Where it would make the release commit, it names that commit as the one the commit step makes, never HEAD, and plans its push to every remote, one already at HEAD included. When no step failed, its last line is its verdict, saying so and whether the cut stops to ask at the confirm step.

The confirm step asks a person. `--yes` answers it. With neither a terminal nor `--yes`, the cut stops there and exits 1, its row saying nobody was there to ask. It never guesses consent.

`release publish` takes both, the same way. Its dry run makes the snapshot in its own scratch, reads the public repository, and pushes nothing.

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
    snapshot <v> <new-dir> [--no-parent]
                 make the tag's public snapshot in <new-dir>, which must not
                 exist yet and must be outside the project: one commit of its
                 tracked files over release.snapshot.list, read in its own
                 tree, made under release.snapshot.author with its date, never
                 signed, so the same inputs give the same commit. It is made on
                 the publish branch's tip at release.publish.url, whose history
                 must be snapshots, or with no parent under --no-parent or when
                 there is no tip. --commit <rev> in place of <v> makes it of any
                 commit. Prints its tree and commit; a refusal writes nothing
    publish <v> [--expect <sha>]
                 make the tag's snapshot public, under release.publish.url:
                 Publishing a snapshot, above

## Exit codes

    0    released, or under release.publish.url cut and pushed, or made
         public; for a dry run, no step failed: its gates passed, and it
         predicted the rest or listed what it could not
    1    a step failed, a dry run's gates included, or the arguments were
         refused; a failed step is named, and a re-run continues from it.
         The confirm step fails this way when nobody is there to ask, as when a
         person declines, and its row says which
    2    the verb is one this project's release.verbs leaves out, refused
         before anything was read

A real cut is itself a sealed gate: its full log goes to a file under `tmp/release/` rather than being left to scroll past, and so is a real `release publish`. A dry run makes no release, so it seals no log of its own; the gates it runs still seal theirs.

## Never moved, never deleted

The core never moves or deletes a tag, and neither should a person. A defect found after the push is fixed forward: fix it, write the next patch's note, cut again.
