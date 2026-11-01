    bin/devbin release <vX.Y.Z|--patch|--minor|--major> [--dry-run] [--yes]
    bin/devbin release status <vX.Y.Z>
    bin/devbin release notes
    bin/devbin release check
    bin/devbin release attach <vX.Y.Z> [--from <dir>]
    bin/devbin release snapshot <vX.Y.Z | --commit <rev>> <new-dir> [--no-parent]
    bin/devbin release gates [--rerun]

Cut a release: confirm once, then stamp, commit, run every leg (the gates, here and on each host release.legs names, the macOS stage where it is declared, and the candidate's CI) under one wait loop, tag and push once all are green, make the release object and run `after:`, all under that one yes. `release` is opt-in: a project enables it with `commands.release.enabled: true` and declares how it releases in the `release:` block of `bin/.devbin/config.yaml`, where every key has a default. Every `release:` key is annotated in devbin's reference config, and `bin/devbin devbin` names where that file and devbin's releasing guide are.

`release.verbs` says which of these a project offers, every one by default: a project whose own command cuts its releases offers `check` and `attach` with the cut off. A verb it leaves out is refused, exit 2, and `release --help` shows only the verbs offered.

## The steps

    resolve      the version and the tag
    pre-flight   the branch, the tree, the version, the note, every remote,
                 gh signed in when the release object needs it, tmp/ ignored
                 by git; unless ci.query is -, the candidate's ref a branch
                 name git takes, and without release.publish.url a remote
                 naming the project's repository for the candidate and a
                 push of its branch that starts release.ci.workflow
                 (ci-state --fires); no gate in all that its platforms:
                 hold back on every leg; under release.publish.url, no
                 remote it pushes to naming the publish repository, the
                 snapshot list in the tree, and the url's refs read
                 (Publishing a snapshot, below); every refusal named in one
                 run
    confirm      ask once, before anything runs, naming the tag, the release
                 commit, the candidate's push, the send to each host
                 release.legs names, the macOS stage's submission to Apple,
                 each remote's push, under release.publish.url the public
                 branch and tag, the object, its assets and the after:
                 commands, which run under the same yes
    stamp        set the version, run any derived: commands, date the note,
                 regenerate the notes views
    commit       the release commit
    legs         every leg at once, read by one wait loop: the declared
                 gates on the release commit (default: check all, test all),
                 under one seal of their own, already where a sealed green
                 stands for it on this platform; the same gates on each
                 host release.legs names, at the release commit there, their
                 seal copied back, a host pre-flight cannot reach named and
                 not run; the macOS stage where macos is enabled, app-sign
                 then app-notarize under a seal of its own; unless ci.query
                 is -, the candidate: the release commit pushed to
                 candidate/<tag> on the repository's remote, or under
                 release.publish.url the release commit's snapshot, made
                 first, pushed to candidate/<tag> at the public repository,
                 and its CI read by that ref; a red leg, or a leg other than
                 the local gates not green within ci.wait, stops the run
                 with nothing tagged and the candidate deleted
    tag          once every leg is green: every remote read again, a
                 change since the confirm stopping the run here; then an
                 annotated tag whose message is the note's body
    push         the release commit and the tag, to each remote in remotes:,
                 or to every remote git lists when none is declared, and
                 under release.publish.url the publish branch fast-forwarded
                 to the snapshot and the tag pushed at it, lightweight, each
                 read back after its push; then the candidate is deleted
    object       the release object -- made here (object: local, the
                 default), by the project's CI (object: ci), or none, under
                 release.publish.url on the public repository; under
                 object: local, then the assets assets.workflow's run built,
                 as release attach attaches them
    after        the declared after: commands

The push step reads each remote back after pushing to it, and a remote is slowest straight after a push, so a read that does not answer is asked again every 15 seconds for about 2 minutes, the bound the push itself has. A remote that answers without the release fails the step at once, as one that does not hold the release after the push. An answer that cannot be read here, such as a branch moved to a commit this checkout does not hold, fails it at once too, as a read that could not tell whether the remote holds the release, since no later ask can clear it. One that has not answered in that time fails it as a read that could not tell, never as a remote without the release: running the same command again reads it back.

Each step with a postcondition checks it first. Running the same command again after a failure therefore continues from where it stopped, and a step already done says `already`. A cut that stopped after its tag is resumed by naming the version: once the tag exists, `--patch` would name the NEXT version, so the summary prints the command to resume with. A dry run's prints the command to cut with.

A project with a `formula` command, devbin's built-in enabled or its own handler, publishes to Homebrew, and there a released cut ends on one more line, a note (a cut whose `after:` failed ends on its remedy instead): once the tap has the tag, `brew upgrade <formula>`, then `<formula> use brew`, after which `<formula> --version` reads `<formula> <version> (brew, <sha12>)` at the released version. The switch among dev, opt and brew rests at brew, and this step puts the machine back there. The core runs no brew and points no link. The formula's name is `formula.name`, or `project.name` where that is not declared.

## CI green before the tag

Unless `release.ci.query` is `-`, the opt-out, the legs step pushes the release commit to its candidate ref, `release.publish.candidate` with `{tag}` filled in (default `candidate/{tag}`), on the remote that pushes to the project's repository, `release.repo` or the repository read from the remote it falls back to, each remote judged by where a push to it goes, its push urls where it has any, else all its urls, as configured: of the remotes `release.remotes` declares, in its order, then every other remote, the first that pushes to it on its own host, whatever the case, else the first that pushes to its OWNER/NAME on any host, an ssh host alias included; pre-flight refuses a cut where no remote pushes to it, and one where remotes push to its OWNER/NAME on more than one host and none on its own, naming each remote with each host and the repository's own host to point a push url at, since a mirror would take the candidate's push where CI never runs. Under `release.publish.url` the candidate is the release commit's snapshot instead, pushed to that ref at the public repository (Publishing a snapshot, below). The push comes first: one refused, or one that does not answer, stops the run before anything else starts. The query is then asked about the release commit with `DEVBIN_RELEASE_TAG` set to the candidate ref, every 15 seconds, while the local gates run beside it; the run prints a line with the clock each time a leg's word changes, and a leg read green is not read again. The local gates are pending while their run is alive, and once it has ended its own exit status decides, the record giving the detail. A gate that opens the terminal itself to ask, as ssh, gpg or sudo can, is stopped there, since the local gates' run is not the terminal's foreground group, and the cut waits until it is interrupted: run such a gate under `release gates`. A cut run again after a red candidate pushes it again, and a red read the same before and after that push is the earlier run's, pending until CI answers anything else; a candidate a killed run left holding the release commit, read red, is deleted first, so that push makes a run of its own. The tag is made and pushed only once every leg is green. The candidate ref is deleted once the tag is pushed, as at any stop, only while it holds the release commit; one that will not go is named with the command that deletes it, and the run ends at exit 1, the release standing. A red leg stops the run with nothing tagged, the local gates' whole process group stopped and the candidate ref deleted on the same terms as after the tag's push, and so does a candidate not green within `ci.wait` of the loop's start; the local gates alone are not bounded by `ci.wait` and run to their end. Fix what the run names and run the same target again: the stamp and the commit read `already`, and a leg whose record is green for the release commit is not run again. The project's workflow must therefore run on a push of the candidate branch, and pre-flight asks `bin/devbin ci-state --fires` whether it does, refusing a cut whose push would start no run.

## Publishing a snapshot

A project that declares `release.publish.url` publishes each release as a snapshot of its product, the commit `release snapshot` makes of the release commit, and never as its development repository. Its cut pushes the workroom to `release.remotes` alone, refusing at pre-flight any remote of this project that names the publish repository by OWNER/NAME on any host or as the same path, pushed to or not, and a remote it pushes to whose url names a host and no readable OWNER/NAME, and makes the snapshot public itself, once every leg is green, in the steps a cut always has:

    pre-flight   also: release.ci.query is not -, since CI on the
                 candidate is what stands before anything is public; the
                 tree holds release.snapshot.list; release.repo names the
                 public repository where the release object needs gh; and
                 the url answers, its publish branch, candidate branch and
                 tag read
    legs         once the release commit exists, its snapshot is made in the
                 run's scratch, on the publish branch's tip, where a tip that
                 is this snapshot already is found, and a public tag at
                 another commit is refused, before anything is pushed, as is
                 a snapshot that changes release.ci.workflow, or whose push
                 would start no run of it (ci-state --fires, asked at the
                 release commit with the files the snapshot changes); the
                 candidate is the snapshot, pushed to its candidate branch at
                 the url and read by that ref
    push         once every leg is green, the public refs read again, then
                 the publish branch fast-forwarded to the snapshot and the
                 tag pushed at it, lightweight, each read back; a tag
                 another hand pushed, or a publish branch that moved while
                 CI ran, is refused by name
    object       the public repository's, as release.repo names it; the
                 assets run is read on the snapshot's commit

Nothing is forced. A red candidate, a run still going at `ci.wait`, or no verdict at all stops the run at the legs step, with nothing tagged and nothing public, and the candidate deleted. A public tag already at another commit is refused, since a tag is never moved, and so is a publish branch that moved after the snapshot was made on it: running again makes the snapshot on the new tip, a new sha. An older version run again after a later one was published is refused as public already. A cut resumed after its tag makes the snapshot again, the same commit from the same inputs, and while it is not public reads the candidate's CI on it again before the push goes on, since a snapshot made on a public branch that moved is a new commit. `release status` reads the public repository's refs too, by the read the cut's pre-flight makes, so its push row is pending while the snapshot is not public there, and an older version, its tag in the publish branch's history, reads as public already; it reads no url one of the project's remotes names, as a cut reads none.

Every call to the public repository is made from a repository of the run's own, so the url is read as the person's git reads it, through no remote of this project: a push can go through `url.<base>.pushInsteadOf` in their own git config. The candidate's CI must run the whole matrix a tag runs, so the project's workflow runs on the candidate branch. With `release.publish.url` declared, CI on the candidate is what stands before anything is public, and the built-in `formula` reads its revision from the public repository's tag.

## Choosing the version

The bump is the one judgement in a release, so nothing infers it from commits. Name the version, or use `--patch`, `--minor` or `--major`, which bump the newest tag matching `release.tag` (by default `v{version}`) -- not the version file.

## --dry-run and --yes

`--dry-run` makes no stamp, commit, tag, push or release object, and runs no gate and no `after:`. It prints each step as what it would do, naming anything it could not predict. Where it would make the release commit, it names that commit as the one the commit step makes, never HEAD, and plans its push to every remote, one already at HEAD included. Where the release commit exists already, its gates row reads the record a cut reads: `already` where a sealed green stands for that commit on this platform, and otherwise what the cut would run. When no step failed, its last line is its verdict, saying so and whether the cut stops to ask at the confirm step.

The confirm step asks a person, once, before anything runs, and everything after it runs under that answer. `--yes` answers it. The `after:` commands run under the same yes: they are handed `DEVBIN_RELEASE_CONFIRMED`, naming the tag, whenever the confirm step was answered, at its prompt or by `--yes`, so `formula publish` as an `after:` command pushes to the tap the question named. A confirm step with nothing left to ask about gives no yes. With neither a terminal nor `--yes`, the cut stops there, before anything is stamped, committed or run, and exits 1, its row saying nobody was there to ask. It never guesses consent.

Under `release.publish.url`, a dry run whose release commit exists already makes the snapshot in its own scratch, reads the public repository and the candidate on the snapshot, and pushes nothing.

## The other forms

    status <v>   the step table for that version, with each step's state and
                 evidence -- changes nothing
    notes        regenerate the notes views release.notes.kind generates:
                 CHANGELOG.md and RELEASES.md for releases, none for
                 changelog, and whatever notes.run writes for command
    check        print the release: declaration, each key marked declared or
                 default, name every workflow step that writes a release
                 object under object: local, then check that every generated
                 view is current, that the project's version has a note, and,
                 where a cut has a candidate read by the built-in query, that
                 a push of the next cut's candidate branch starts
                 release.ci.workflow (ci-state --fires)
    attach <v>   attach the files the earliest successful run of
                 release.assets.workflow at the release commit built, on
                 the candidate's branch or at the tag (under
                 release.publish.url, the commit the public tag names), to
                 its release object, and write one SHA256SUMS over every
                 asset on it;
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
    gates [--rerun]
                 run release.gates in order, in place, on the commit HEAD
                 is, under one sealed run of their own under tmp/release/,
                 as a cut's legs step runs them: the record that step
                 reads. It reads that record first, as the cut does, and
                 where a sealed green stands for the commit on this platform
                 it prints the cut's row and runs nothing; --rerun runs them
                 all the same. Refuses a tree that is not one commit, clean,
                 before anything runs, and fails a gate that writes to the
                 tree or commits, or a pass whose record does not read back

## Exit codes

    0    released; for a dry run, no step failed, and it predicted the rest
         or listed what it could not
    1    a step failed, or the arguments were refused; a failed step is
         named, and a re-run continues from it.
         The confirm step fails this way when nobody is there to ask, as when a
         person declines, and its row says which
    2    the verb is one this project's release.verbs leaves out, refused
         before anything was read

A real cut is itself a sealed gate: its full log goes to a file under `tmp/release/` rather than being left to scroll past. A dry run makes no release and runs no gate, so it seals nothing. Every sealed log says when it ran, a `start:` line and an `end:` line in UTC, and which host it ran on, as information only; a cut's own log adds one `leg:` line for each of its legs once they have started: the leg, its last word, the commit it ran on, the seconds from the legs step's start to its last change of word, and its detail.

A real cut's gates run under one seal of their own, `tmp/release/<stamp>.GATES.{out,errors}`, whose log names the commit the tree stood on when the run opened and again before its seal. A later cut of the same release commit on this platform (the operating system, its release and architecture, and bash's version, as the log's `platform:` line records them) reads the newest such pair, and where it is green and names that commit at both lines, with no `+dirty`, the local gates are `already` and are not run again. A pair whose tree was dirty, or moved before its seal, stands for no commit, and so do two pairs of one commit with one modification time, since neither is the newer: the gates run again, and the row says why. A record that cannot be read is named, with what could not be read: `release status` reports it as `UNREADABLE`, and a cut runs the gates over it. `release gates` reads the same pair first and makes it, and `release status` reads it.

The cut, `release status` and `release gates` each work in the repository at the project root. Git's repository variables an outer command left set, such as `GIT_DIR` in a hook, are cleared at the start, and each one cleared is named, with the repository used instead.

## Never moved, never deleted

The core never moves or deletes a tag, and neither should a person. A defect found after the push is fixed forward: fix it, write the next patch's note, cut again.
