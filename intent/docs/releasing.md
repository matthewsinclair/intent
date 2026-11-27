# Releasing Intent

This page holds the steps of a cut that no script runs for it, and the order that the tag depends on. Each line is a command or a rule about when to run one. Where an answer can change, the page gives the command that reads it, never a count or a state.

## Who fires a cut

hv fires a cut, in hv's own terminal. devbin's release core cuts Intent (ST0091). Its confirm step reads `/dev/tty`, so a tool session cannot answer it, and no tool session passes `--yes`. The push gate stays human. "Get the release ready" does not mean "release it".

The core runs its steps in this order: resolve, pre-flight, confirm, stamp, commit, legs, tag, push, object, formula and after. `bin/devbin release vX.Y.Z --dry-run` prints each step as a row, runs `release.preflight`, and writes nothing.

- Before the confirm step, the core writes nothing.
- Between the confirm and the tag, the core stamps and commits the release locally. It pushes that commit to `upstream` as `candidate/vX.Y.Z`, which starts CI.
- The core makes the tag only when every leg is green.
- After the tag, the push, the release object and the `after:` commands make the release public.

**To continue a cut that stopped, run the same cut again.** `version set` runs only where the version differs, every derived step runs again and names its paths, and the core continues from where it stopped. `bin/.devbin/cut/preflight clean` stands down while the version file differs from HEAD's, so it does not refuse the stamp's own changes. A cut that has tagged cannot be backed out. After the tag, the remedy is also the same cut, never a deletion.

## Commit the reference pages before the cut

The core's release commit goes on top of HEAD, and the core tags that commit. Commit the regenerated reference pages before the cut, so that the tag contains them.

Each page names the sha it describes in its `Revision this describes` row. A tag's sha does not exist until its commit exists, so no page can name the tag it ships in. The tag must point at a commit that already has the pages.

The cut has no step in which to regenerate them. At the legs step, `bin/devbin gate reference` (bin/.devbin/cut/gate) regenerates both halves into scratch. It refuses the release commit when the committed set does not describe it, so a cut over stale pages stops before the tag. Commit the regenerated pages, then run the same cut again. No flag lets a cut ship stale pages.

## The generators' two revision inputs

The reference set has two halves, and each generator takes two revision inputs. Pass both inputs at a cut. Do not trust either default.

| Input        | What it means                                               | Renders on the page as                 |
| ------------ | ----------------------------------------------------------- | -------------------------------------- |
| `--rev`      | the revision the pages DESCRIBE; the register is read at it | `Revision this describes`              |
| `--baseline` | the released revision that presence is MEASURED AGAINST     | `Release presence is reported against` |

**Pass `--rev` as the word `HEAD`, never as a sha.** The pages print the revision's name beside its sha: in the title of `cut-surface.md`, in the index, and in the `Revision this describes` row. The staleness check regenerates with `--rev HEAD` and masks only the generation stamp and that row. A set made with `--rev <sha>` has a different title and padding, so the check refuses it every time.

**The baseline is the previous release tag** (hv, 2026-09-17):

```
  git describe --tags --abbrev=0
```

`--tags` is necessary. Without it, `git describe` reads only annotated tags, and the tags from v2.3.3 to v3.2.8 are lightweight, so the bare form answers `v2.3.2` at every cut. devbin's core tags with `git tag -a`, so the tags from v3.3.0 are annotated. `--tags` is correct for both kinds.

**Each generator's own `--baseline` default is `v3.0.0`**, and it drives the `In <version>` column and the "newer than, so not in an installed copy" list. Read the defaults and the committed baselines:

```
  grep -n 'BASELINE=' intent/st/ST0056/parity/tools/gen_reference.sh intent/st/ST0056/parity/tools/gen_cut_surface.sh
  grep -rh 'reported against' docs/reference/ | sort | uniq -c
  grep -h 'Compared against' docs/reference/cut-surface.md
```

**A wrong baseline does not fail.** Where the register does not exist at the baseline, each generator prints a `note:`, exits 0, and writes pages that report presence against nothing. `gate reference` refuses that set at the legs step. Check a baseline before you pass it:

```
  git show "$(git describe --tags --abbrev=0):surface/dispatch-table.json" > /dev/null && echo usable
```

## Running the generators

Run the generators from the tree they describe, because they resolve the revision in the current directory's repository. From another directory they exit non-zero with an empty log. Write to a scratch `--out` first, and read the diff before you write the tree.

```
  cd <the tree being described>
  BASE=$(git describe --tags --abbrev=0)
  intent/st/ST0056/parity/tools/gen_reference.sh    --rev HEAD --baseline "$BASE" --out <scratch>
  intent/st/ST0056/parity/tools/gen_cut_surface.sh  --rev HEAD --baseline "$BASE" --out <scratch>/cut-surface.md
  diff -ru docs/reference <scratch>
```

- **`gen_reference.sh` takes a directory in `--out`, and `gen_cut_surface.sh` takes a file.** Give `gen_cut_surface.sh` a directory and it writes `aligned.md` into it, exits 0 and prints `ok`. A sync of that scratch over `docs/reference` then drops `cut-surface.md`.
- **Give both generators the same `$BASE`.** Otherwise the two halves report presence against different releases.
- Neither generator runs a binary. Both read the register at the revision with `git show`, and they write only where `--out` points. `gen_cut_surface.sh` reads `ALL_VARIANTS` in `native/rust/crates/intentsvcs/tests/error_remedies.rs` at the same revision.
- Commit both halves in ONE commit at ONE revision.
- **A landing that adds a refusal variant makes `cut-surface.md` stale.** Regenerate it in that landing.

`intent/st/ST0056/parity/tools/reference_current_check.sh` regenerates both halves into scratch against the previous release tag, and diffs them with the committed set. It masks only the generation time and the revision row, and it writes nothing. It exits 0 when the set is current, 1 when the set is stale (it prints the diff and the commands that fix it), and 2 when it cannot give a verdict. `bin/devbin gate reference` runs it at the release commit. Run it yourself before step 3.

## The cut, in order

0. Stop the daemon: `intent daemon stop`. The cut builds the pair, and that replaces the binary a running daemon was started from (issue 0468). The cut does not stop or start the daemon, and `bin/devbin preflight daemon` refuses while intentd runs. `intent use brew` in step 4 starts it again.
1. Land everything the cut carries, then run `intent doctor`. The core runs `release.preflight` at its pre-flight, and a dry run does too. `bin/devbin preflight all` runs the same checks by hand, all but `defects`:
   - `clean`: no dirty or untracked path.
   - `daemon`: no intentd runs.
   - `faces`: the published schema faces at HEAD carry HEAD's version.
   - `spill`: `claude upgrade --apply` writes nothing outside the release's own files (issue 0482).
   - `schema`: a cut that moves the store's schema runs with the release's build first on PATH.
   - `defects`: the H1 of `docs/known-defects.md` names the release. It needs the version, so step 2 runs it.

   **A cut that moves the store's schema.** `bin/devbin preflight schema` says whether the cut moves it, and from which schema to which. The stamp migrates this checkout's store, one way, and the release commit's hook then runs `intent doctor` with the `intent` on PATH. So run the cut as `PATH="$PWD/native/rust/target/release:$PATH" bin/devbin release vX.Y.Z`. After the cut, only an Intent at the release's schema can read this checkout. No seat writes in it until every machine that opens it runs the release: from brew, or from the opt tree while the tap is held (below).

2. Regenerate both reference halves at `HEAD` against `--baseline "$(git describe --tags --abbrev=0)"`, read the diff, and commit them together. Do this and the three items below BEFORE step 3, so that the tag carries them.
   - **The known-defects page.** Re-drive `docs/known-defects.md` WHOLE against the pair built from this `HEAD`. `intent --version` names the pair's commit; if it names an older one, run `bin/devbin build all` first. Drive each entry the way the page was made, in a fresh scratch project under an isolated `HOME`. Replace each transcript with the new run's, and do not edit it. An entry that no longer reproduces leaves the page, and the commit that removes it names the release that fixed it. Retitle the page for the release, commit it, and check the title with `DEVBIN_RELEASE_VERSION=X.Y.Z bin/devbin preflight defects`. Do not re-drive one entry against a development build between cuts: the page's preamble pins every entry to the build a release is cut from. `git for-each-ref 'refs/bank/*/kd/*'` lists the scripts and logs of earlier re-drives.
   - **The explorer's screenshots.** Run `intent/st/ST0056/parity/tools/gen_explorer_shots.sh`. It builds the Rust example that takes the shots with cargo first, and its header names what else it needs. Judge each PNG that `git status -- docs/images/explorer` lists as modified against its committed copy: `git show HEAD:<png> > /tmp/committed.png && magick compare -metric AE -fuzz 10% /tmp/committed.png <png> null:` prints on stderr how many pixels differ by more than anti-aliasing, and exits 1 when any do. Commit a PNG only when that number is not zero, and restore the others with `git checkout -- <png>`. The status alone is not the test: runs on different days change edge pixels that no reader can see (issue If074a59).
   - **The CHANGELOG.** Each user-facing issue closed since the previous tag has an entry in the release's section. List the closed issues:

     ```
       SINCE=$(git log -1 --format=%cs "$(git describe --tags --abbrev=0)")
       jq -r --arg d "$SINCE" 'select(.status == "closed" and .closed >= $d) | "\(.id)\t\(.closed)\t\(.title)"' intent/.canon/issues/*.json
     ```

     Read each issue's landing commit, not its title. A change that a user can see gets an entry: a verb, a refusal or its remedy, a shipped guard's output, intentd, an MCP tool, or the formula and the cask under `packaging/homebrew/`. A change to CI, to tests or to `bin/.devbin` gets none, and so does an issue closed with no change. The list starts on the previous tag's day, so an issue closed that day can belong to the previous section: `git merge-base --is-ancestor <commit> "$(git describe --tags --abbrev=0)"` says which. Head the section `## [X.Y.Z] - in progress`, never with a typed date. The core's stamp dates the section, and the release object's notes are that section.

3. Run `bin/devbin release vX.Y.Z --dry-run` and read every row. Then hv runs `bin/devbin release vX.Y.Z` and answers its one confirm step. The steps after the confirm, as the dry run names them:
   - **stamp**: devbin's `version set`, then each `release.derived` line, `bin/devbin stamp config|lock|schema|cli|agents|upgrade|views` (bin/.devbin/cut/stamp). Each one prints the paths it wrote, and the release commit can hold them. A refusal gives a forward remedy: commit or set aside what it names, then run the same cut again.
   - **commit**: `release: vX.Y.Z`.
   - **legs**: the core pushes the release commit to `upstream` as `candidate/vX.Y.Z`. That starts `tests.yml` (and through it `rust.yml`) and `linux-release.yml`. The pre-push hook asks a candidate push for no currency verdict (ST0091 WP-02). At the same time `release.gates` (`gate all`, `build all`, `test shell`) run in place in this checkout, and a run fails if the tree or HEAD moves under it. They also run on each `release.legs` host (kovacs), in a worktree there at the release commit. A host that pre-flight cannot reach is named and recorded as not run. `gate all` (bin/.devbin/cut/gate) runs the contract and reference checks, every cargo step of `rust.yml` in a private worktree with issue 0481's one re-run of intentd's load-sensitive family, and on macOS the pair built at the release commit, its embedded source commit and its `intent doctor`. The core also runs `bin/devbin macos leg` at the release commit, which builds, signs, notarises and stages the pair and the app.
   - **tag, push**: an annotated `vX.Y.Z` at the release commit, then `main` and the tag to `local` and `upstream` only (`release.remotes`).
   - **object**: the GitHub release, with the Linux packages that `linux-release.yml` built at the release commit, and `SHA256SUMS` over every asset.
   - **after**: `bin/devbin cut publish`, then `bin/devbin brew smoke --at-tag`. These are step 4.
4. The core runs its `after:` commands. `bin/devbin cut publish` (bin/.devbin/cut/publish) runs three steps in order and **stops at the first failure**. It builds and signs nothing.
   1. `bin/devbin macos publish` uploads the macOS assets that the macOS leg staged, then downloads them again and checks them against `SHA256SUMS.txt`.
   2. `bin/devbin release attach vX.Y.Z` uploads nothing that the release holds, and writes `SHA256SUMS` again, over the macOS assets too.
   3. `bin/devbin formula publish --pull` renders the formula and the cask from `packaging/homebrew/`, with each sha256 read from `SHA256SUMS` (issue 0634). It pushes them to the tap under the cut's yes, and brings this machine's tap forward.

   Then `bin/devbin brew smoke --at-tag` installs cold from the tap and checks the keg, once the tap carries the release.

   - **A failed publish.** The release, its tag and its Linux assets are public. The tap is unchanged, because the formula step is last and a failure stops the run before it. `brew smoke --at-tag` then says "the tap is not at X.Y.Z: the publish step did not finish" and smokes nothing. **The remedy is the same cut again**, or `bin/devbin cut publish` by hand at the tag, because each step reads what is done first. A release that holds some macOS assets and not others is refused, and the refusal prints the `gh release delete-asset` command for each asset to delete before you run the cut again.
   - **A held tap (hv decision 77).** `bin/devbin cut publish --hold-tap` runs the macOS and sums steps and stops before the formula. The tap still serves the previous release, so no `brew upgrade` reaches the new one. To hold a cut, set `release.after` in `bin/.devbin/config.yaml` to `bin/devbin cut publish --hold-tap` alone. On hv's word, run `bin/devbin cut tap` at the tag. It refuses a release that does not hold every macOS asset. It runs `bin/devbin formula publish --pull`, then `bin/devbin brew smoke --at-tag`, and stops at the first failure. A second run is safe, because the formula step does nothing when the tap already carries the render. Run by hand, it has no cut's yes, so `formula publish` asks at the terminal. After `cut tap`, commit `release.after` back to `bin/devbin cut publish` and `bin/devbin brew smoke --at-tag`, so the next cut is not held. Until the tap moves, our own machines run the release from the opt tree (below).
   - **The cut ends on brew** (hv, 2026-10-25). Once the tap carries the release, run `brew upgrade matthewsinclair/intent/intent`, then `intent use brew`. It removes the links in `~/.local/bin`, so Homebrew's `intent` answers, and it restarts the daemon through that binary. Then read `intent --version`, which names the brew tree, and run `intent doctor`. `intent use dev` is the way back to work.
   - **The app on this machine.** Step 4 does not install the cut's Intent.app here: the formula does not carry the app (`docs/install.md`). `intent app start` launches `/Applications/Intent.app` when one exists, so the menubar keeps running the previous release. Swap in the stapled bundle that the macOS leg staged, then read the version off the bundle, not off the menu:

     ```
       intent app stop
       D=$(mktemp -d) && ditto -x -k native/rust/target/dist/Intent.app.zip "$D"
       spctl -a -vv "$D/Intent.app" && xcrun stapler validate "$D/Intent.app"
       mv /Applications/Intent.app "$D/Intent.app.previous"
       ditto "$D/Intent.app" /Applications/Intent.app
       /System/Library/Frameworks/CoreServices.framework/Frameworks/LaunchServices.framework/Support/lsregister -f /Applications/Intent.app
       intent app start
       intent app status
       plutil -p /Applications/Intent.app/Contents/Info.plist | grep -E 'CFBundleShortVersionString|IntentCommit'
     ```

     `intent app status` exits 0 when the app runs, and names the path it runs from. The two plist keys must read the release's version and the commit that the tag points at, `git rev-parse --short=9 'vX.Y.Z^{commit}'`. The bare tag name gives the tag object's own sha, which no plist carries. Run `intent app stop` and `intent app start` apart, not `intent app restart`, because the swap goes between them. The previous bundle stays in `$D` until you delete it.

5. After `brew smoke` passes, put the release on the public site. `intent.laksa.io` shows the shipped version beside its wordmark, read from `db/release.json` in the site repository (`~/Devel/prj/Sites/intent`). Write that file with the new `version`, the `released` date and the release `url`, and commit it there. Publish with `bin/devbin sites gitpush --sites=intent` from the Laksa checkout, `--dry-run` first. The push is the deploy: GitHub's webhook syncs the site. Then fetch `https://intent.laksa.io/` and check that it shows the new version and not `laksa-render`, which appears only on the fallback page. A missing or unreadable `db/release.json` gives a header with no version and no error, so a skipped step shows the OLD version: read the number.

The reference pages, the known-defects page and the CHANGELOG reach a reader only through the tag and the release notes that the core writes from it. A commit made after the tag is in neither, so step 2 goes before step 3.

## Our machines while the tap is held

Users install from the tap, so while a cut holds it, they stay on the previous release. Our own machines (rhadamanth, gyges and kovacs) run the new release from the opt tree of the dev, opt and brew contract. A release asset unpacked by hand is not a tree Intent recognises, because only the formula writes its `.intent-install` manifest. On each machine, once, after the tag:

1. In a clean worktree of Intent at the tag, run `bin/devbin build all`, which builds the release pair.
2. From that pair, run `intent install --prefix ~/.local/opt/intent`, which publishes the opt tree with its manifest.
3. Run `intent use opt --prefix ~/.local/opt/intent`. It points `~/.local/bin/intent` and `intentd` at the opt tree, and restarts the daemon through it.
4. In each project, run `intent upgrade` once, then `intent claude upgrade --apply`.

The links are per user, so the switch moves every estate on the machine at once. It governs login shells only (`docs/install.md`). A session started without the login PATH still runs Homebrew's previous release for its hooks and MCP server, so start each seat again from a login shell after the switch. Intent.app comes from the cask and stays on the previous release until the tap moves.

When the tap moves, on each machine: run `intent use brew`, then `brew update` and `brew upgrade matthewsinclair/intent/intent matthewsinclair/intent/intent-app`, then `intent --version`, which names the brew tree. You can then remove the opt tree.

## The pointer to this page is in CLAUDE.md, not in its template

`CLAUDE.md`'s "Internal authoring docs" index is inside its `<!-- user:start -->` / `<!-- user:end -->` region, which a render keeps. The template, `lib/templates/llm/_CLAUDE.md`, has no such section, so edit the pointer in `CLAUDE.md`. An edit to the template is also an edit under `DIRT_SCOPE` (`native/rust/build-support/source_commit.rs`), which marks the installed pair as behind the tree.

## How each command on this page is checked

A command in a document is a claim. Check each one in the way its kind allows, and keep this list true when you change the page.

- **Cheap and read-only: run it, and read what it printed.** Every `git describe`, `git show`, `git for-each-ref` and `grep`, both generators into scratch, `reference_current_check.sh`, the CHANGELOG listing, `bin/devbin preflight` and the dry run.
- **Destructive, outward or one-shot: read it as control flow, not as lines.** `bin/devbin release vX.Y.Z`, `bin/devbin cut publish` and `cut tap`, the brew and app steps, and the site push.

A pattern that reads generated output must match the rendered page, not the sentence as it reads in prose: the pages render a value as a table cell, with padding. Before you call a sweep of this page clean, list every line inside a code fence, so that the sweep sees each command.
