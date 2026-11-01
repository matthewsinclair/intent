    bin/devbin ci-state <sha>
    bin/devbin ci-state --fires <ref> <sha> [<path>...]

Read CI's verdict on a commit: one line, `<verdict><TAB><detail>`, the verdict one of `green`, `failed`, `pending`, `none` or `unknown`, and the detail naming the run it read or the reason it could not. It reads the project's workflow runs on GitHub through `gh`, and changes nothing anywhere.

It is the release core's own words, so `bin/devbin ci-state` is a `release.ci.query` as it stands: the core runs `<query> <sha>` with no input, with a cut's candidate branch, such as `candidate/v1.2.3`, in `DEVBIN_RELEASE_TAG`, and reads the first line. It exits 0 whenever it answered, whatever the verdict, and 1 on a usage error or a config that does not validate. Read the verdict from the line, never from the exit status. The sha is a full one, as the core passes: `gh` matches a run's commit by its whole sha, so a short sha or a branch name is refused as a usage error rather than read as no run.

## Which runs count

    DEVBIN_RELEASE_TAG set     only that ref's runs: a tag's, or a branch's such
                               as candidate/v1.2.3; none until one exists
    DEVBIN_RELEASE_TAG unset   every run on the sha; a sha with no run of its own
                               is read on the commit before it, while each commit
                               between changes only what the workflow's push
                               trigger ignores

The ref is matched against each run's head branch, which for a tag's push is the tag and for a branch's push the branch. A push to `main` on the same commit carries fewer legs than a release's tag or candidate, so its green is no verdict on the release. Without a ref, a push that moved only coordination state starts no run, so a commit like that is read on the commit before it. The push trigger's `paths-ignore` is read from the workflow as committed at the asked sha, never the working tree's copy, and a `pull_request` trigger's list is not the push's. An entry that is not a directory's whole tree (`<dir>/**`), or a list written inline, is `unknown`, never read as something else.

A commit this repository does not hold, such as a release's public snapshot, is `none` when it has no run: CI may yet run there, and no commit before it can be read. The walk back ends `none` at a commit that changes what the workflow reads, or at the first commit, and the detail says which; a shallow clone that holds no commit before the one the walk reached is `unknown`.

## Whose verdict a run's is

    release.ci.summary declared   that job's: a workflow whose summary job does
                                  not wait for a leg that blocks nothing reads
                                  green when that leg alone is red
    release.ci.summary undeclared the run's own status and conclusion

A summary job that is absent, skipped, cancelled or still going is not green: a finished run with none is `failed`, naming that. Among several runs on one commit the worst decides: `failed`, any conclusion but success, a cancelled one included, over one still going over `green`.

A `failed` verdict names the deciding run's failed jobs inside its parenthesis, as `run <id> (<conclusion>; failed jobs: <name>, <name>)`, each a job that finished in anything but success, skipped or neutral. It says `no job ran` when none ran, and `no job failed` when jobs ran and none failed, as when the run failed around them. A job list `gh` cannot give is said in the same place, and the verdict stays `failed`.

## Whether a push starts the workflow

    bin/devbin ci-state --fires <ref> <sha> [<path>...]

Answers whether a push of the branch `<ref>` starts `release.ci.workflow`, read as committed at `<sha>`, never the working tree's copy: one line, `<word><TAB><reason>`, exit 0 whenever it answered. `<path>...` are the files the pushed commit changes, which the push trigger's `paths` and `paths-ignore` filters are read against. A release asks it about the candidate branch a cut will push, with the files the release commit changes, before anything moves.

    fires     a push of <ref> starts the workflow
    quiet     it does not: no push trigger, a branches list nothing of which
              matches <ref>, a branches-ignore entry that does, a push trigger
              filtering tags alone, or, with <path>... given, a paths list
              matching none of them or a paths-ignore leaving none of them out
    missing   the tree at <sha> holds no such workflow file
    unknown   a trigger written in a form this does not read, a push trigger
              filtering by paths: with no <path> given, a
              release.ci.query that is not this built-in, `-` among them,
              or a git that cannot answer

Patterns are matched as GitHub matches them: `*` within a path segment, `**` across them, `**/` as any number of directories, none included, `?` and `+` after the character they repeat, a `[]` of letters, digits and ascending ranges of them, and the entries in order, a `!` entry excluding what an entry before it matched. A trigger written with YAML's anchors, aliases, tags, explicit keys or escapes is `unknown`, never read as something else. With no `<path>` given, a `paths-ignore` of whole directories (`<dir>/**`) leaves the answer to the branch filter, and a `fires` answer names the premise it rests on, that the release commit's files sit outside the ignored trees; any other entry, or a list written inline, is `unknown`. The reason names the file and, for `quiet`, the entry to add. It asks `gh` nothing.

## The declaration

    release.repo          the repository, OWNER/NAME; undeclared, read off the
                          remote a release pushes to first, else origin, else
                          the only remote
    release.ci.workflow   the workflow file in .github/workflows whose runs are
                          the verdict; default tests.yml
    release.ci.summary    the job whose verdict is its run's; default none

Declaring `release.ci.workflow` or `release.ci.summary` is refused unless `release.ci.query` is `bin/devbin ci-state`, its default and the one query that reads them; `-`, the opt-out, reads none.

## When it cannot read

A `gh` that is not installed, a `gh` call that fails, a list of runs as long as the limit `gh` was asked for, and a `git` that cannot answer about a commit the walk reached are each `unknown`, carrying the words of what failed: a verdict nobody could read is never read as anything else. No call is bounded here; the release core bounds the whole query, as it bounds every reader it asks.
