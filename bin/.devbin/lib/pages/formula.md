    bin/devbin formula render <tag> <out> [<cask-out>] [--sums <file>]
    bin/devbin formula publish [--tap <url>]

Render this project's Homebrew formula from a release, and publish it to the project's tap. The formula is rendered, never typed: it is kept as a template in the project, and devbin fills its placeholders from git and from the release, so nobody types a tag, a commit or a checksum.

It is opt-in: absent until a project asks for it with `commands.formula.enabled: true`. A project's own `bin/.devbin/cmd/formula` handler, or a `commands.formula.run:` line, still wins over devbin's, as for any built-in, and then `bin/devbin help formula` prints the project's own page.

    render <tag> <out>     write the formula for <tag>, an annotated tag in this
                           checkout, to <out>, and the cask to <cask-out> where
                           formula.cask names one; a relative path is read from
                           the project root, where devbin runs every command
    publish [--tap <url>]  render the newest release into a fresh clone of the
                           tap, and once you say yes, commit it and push it. A
                           tap that already carries the render is left as it is,
                           and nobody is asked

## The declaration

    formula.mode      source, a formula brew builds from the tag's checkout,
                      or assets, a formula for the release's own assets;
                      default source
    formula.name      the formula's name; default project.name
    formula.template  the template, from the project root; default
                      packaging/homebrew/<name>.rb
    formula.tap       the tap's git url; default homebrew-<name> under the
                      release repository's owner, over ssh
    formula.repo      assets: the repository whose release carries the
                      assets; default the release repository
    formula.sums      assets: the sums file's name on that release; default
                      SHA256SUMS
    formula.assets    assets: the assets the templates take a sha256 of, each
                      a name on the release, which may hold {version}
    formula.bin       assets: the keg's executables, from libexec
    formula.manifest  assets: the keg's manifest, under libexec; default
                      .<name>-install
    formula.cask      assets: a cask template, from the project root; none:
                      no cask

A key the mode does not read is refused rather than ignored.

## Source mode

Below its header, the template holds exactly one `@@TAG@@` and one `@@REVISION@@`, and render fills them with the tag and the commit it names, read with `git rev-parse <tag>^{commit}`. brew checks out the tag at install and stops when its commit is not the formula's revision, so a wrong revision fails every install, and the offline `brew audit` accepts any 40 hex digits, so it does not catch one. render refuses:

- a tag this checkout does not have;
- a lightweight tag, since every release tag is annotated;
- a template without exactly one tag placeholder and one revision placeholder: with two, which one is the pin is not knowable, and with none there is nothing to fill;
- a tag holding a character a release tag never does, since it is written into a Ruby string that brew evaluates.

## Assets mode

For a formula that installs the release's assets rather than building from source. The templates hold:

    @@VERSION@@          the release's version, read from the tag through
                         release.tag; at least once
    @@SHA256:<asset>@@   the sha256 of an asset in formula.assets, read from
                         the release's sums file by its exact name; exactly
                         once for each asset, across the formula and the cask
    @@REVISION@@         the tag's commit, where the template wants it
    @@INSTALL@@          the whole install body devbin generates, on a line of
                         its own: everything the archive holds into libexec,
                         then what @@FINISH@@ adds
    @@FINISH@@           the install body's end, on a line of its own, for a
                         template that lays its assets out itself: 0755 on each
                         of formula.bin, the manifest, and a link in bin for each

The formula holds exactly one of `@@INSTALL@@` and `@@FINISH@@`, and the cask holds neither. Each line of that generated body answers a way such a keg fails. GitHub serves a release asset with no executable bit and Homebrew's install keeps the mode it is given, so without the 0755, brew installs the keg and nothing in it runs. The manifest is the dev/opt/brew contract's, T3's: `tool`, `version`, `source_commit` and `managed_by: brew` as `# key: value` lines, then one `<sha256>  <path>` line for each file under libexec, the manifest itself left out, sorted bytewise.

The sums come from the release, never from a download hashed here: `gh release download <tag> --pattern <sums>` on formula.repo, or the file `--sums` names, for a render offline. Each line is `<sha256>  <name>`, the form a sha256 check reads, and render refuses a file with any other line, one that names an asset twice, and one that does not name an asset formula.assets lists.

render also refuses:

- a formula with no url at top level, two spaces in: `brew tap` checks a formula under every simulated OS and arch, and one whose only url sits inside `on_macos` has none on Linux, so the whole tap is refused;
- a top-level `version` line when the top-level url names the version, which brew reads from it, and `brew audit --strict` refuses the second statement;
- an `@@SHA256:<asset>@@` naming no asset formula.assets lists.

The cask, where formula.cask names one, is rendered from the same release and goes to the tap in the same commit as the formula, as `Casks/<template's name>`, so the tap never holds a formula and a cask from different releases.

In both modes, a placeholder left after filling is refused by name, so a misspelt one cannot ship, and a refusal writes nothing.

## The rendered file's header

In both modes, each file that render writes opens with a header of its own, below any magic comment it keeps. The header says the file was rendered by devbin's formula built-in, names the template by its path from the project root, the tag and the commit the tag names, and says to edit the template rather than the file, which the next render replaces. A template declared with an absolute path is read under the project root too, and named without its leading slashes, so no path on the maintainer's machine reaches the tap.

The template's own header, its leading run of `#` comment and blank lines, is about the template, so it is dropped; a `=begin` block is no part of it. Only what follows the header ships, so only that is validated and filled: a placeholder written in the header is neither counted nor filled, and a template with nothing after its header is refused. A Ruby magic comment in the header, such as `# frozen_string_literal: true`, an Emacs mode line, or an encoding named on the first line, is kept at the top with a blank line after it, since Ruby reads most of them only before the first code.

## Which release publish publishes

publish is handed no tag. It takes the newest release by the rule the release core's `--patch` reads: the newest X.Y.Z among the tags that match the project's `release.tag` template, which `bin/devbin release check` prints. The core runs each `after:` command with no input and no tag in its environment, so a cut whose `after:` names `bin/devbin formula publish` publishes the release it just made, and a person running it later brings the tap up to the newest release.

It publishes only committed templates. render reads a template as it stands, which is what a person editing it wants to see, but a formula pushed to the tap from bytes no commit holds is the provenance a published install refuses to fake, so publish refuses a template with changes no commit holds.

## The yes

publish names the tap it resolved, and the release, and asks before it pushes. There is no `--yes`. Run by itself, it asks the person at this terminal. Run as a cut's `after:` step, it takes the yes the cut's confirm step was given, whose question named the `after:` commands: the release core hands its `after:` commands `DEVBIN_RELEASE_CONFIRMED`, naming the tag, whenever that step was answered, at its prompt or by `--yes`, and not when it had nothing left to ask about. publish says so when it takes it, and honours it only for exactly the tag it publishes; any other value is ignored and it asks. With nobody to ask it refuses, nothing is published, and the after step says so. A tap that already carries the render asks nothing, so an `after:` step re-run after a cut is quiet.

It writes nothing in this tree. The tap is cloned into a temporary directory that is removed when publish exits, and the clone and the push run with git's terminal prompt off and ssh in batch mode, so publish fails rather than waits for a password. The commit in the tap is `<name> <version>`, made with your own git identity.

## See also

    bin/devbin help install      what a machine install is, and how a project
                                 finds the one Homebrew placed
    bin/devbin help release      the release core, and its after: step
