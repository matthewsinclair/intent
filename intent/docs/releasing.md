# Releasing Intent

This page is the durable home for the steps a cut runs that no script runs for it. It exists because one of them — regenerating `docs/reference` — survived only in commit messages, and a step whose only home is a commit message is a step the next cut does not run. Everything here is either a command or a constraint on when a command is run; nothing here is a state, and nothing here is a count. Where a line would have to carry a number, it carries the command that answers it instead.

## Who fires a cut

hv does, in hv's own terminal. The release confirmation reads `/dev/tty`, so a tool session cannot answer it, and no tool session passes `--no-confirm`. The push gate stays human. "Get the release ready" is not "release it".

**And a cut is not reversible up to the push.** The driver commits, tags and pushes in the same run — see the next section — so there is no state in which the tag exists locally and a mistake can be quietly undone. By the time the command has returned, `main` and the tag are on both remotes. That is why the confirmation is a human's, and it is also why the driver cannot be invoked at all from a tool session while NO PUSH stands: it has no mode that stops short of publishing. Anyone planning a cut as a sequence they can back out of partway is planning something the driver does not offer.

## The ordering the tag depends on, which is the reason this page exists

**`bin/devbin build release` commits the version bump, tags it, and pushes — and the one gap in that sequence is a trap rather than an opportunity.** It commits the version bump, refuses to tag a dirty tree, creates the tag at `HEAD` — the commit it has just made — then asks for a confirmation and pushes `main` and the tag to both remotes. The whole sequence is in `bin/.devbin/cmd/build.d/release`, tag block and push block adjacent; read it there rather than trusting this paragraph, and `--dry-run` narrates every step without doing any of it.

**There IS a window between the tag and the push, and it cannot be used to insert the regeneration.** The push sits behind a human confirmation, and declining it exits 2 with the tag already created locally — the driver says so itself: `user aborted before push -- tag <TAG> exists locally; re-run when ready`. That looks like a chance to commit the regeneration and re-run. It is not. On the re-run the driver finds the tag pointing at a different sha from the new `HEAD` and **aborts rather than force-moving it** (`tag <TAG> exists at <sha> but HEAD is <sha> -- refusing to force-move`). The only ways out are deleting the tag by hand or shipping a tag that does not contain the regeneration, **which is exactly what v3.0.3 did.**

So the conclusion is not that the driver is atomic; it is that every route which regenerates after the tag ends either in a refusal or in a wrong tag.

**So the regeneration is committed BEFORE the driver runs.** The driver's version-bump commit lands on top of it and the tag is created there, which makes the regeneration an ancestor of the tag, and the tag then contains its own reference pages. This is the only arrangement that is self-consistent, and the reason is that the pages name a revision rather than a version: each one carries `Revision this describes: <sha>`. A tag's sha does not exist until the commit it points at exists, so "regenerate at the tag" cannot be satisfied by generating pages that name the tag. The tag has to point at the regeneration, not the other way round.

**What happens when the order is reversed is on the record.** At the v3.0.3 cut the tag is `46599d145` and the regeneration is `a87a94a4a`, twenty-four minutes later:

```
  git merge-base --is-ancestor a87a94a4a v3.0.3    # exits non-zero: the tag does not contain it
```

So v3.0.3 ships the previous cut's reference pages, while the pages on `main` name the v3.0.3 tree. Anyone checking out the tag reads the wrong surface, and nothing reports it.

## The generators take two revision inputs, and neither default can be trusted

The reference set has two halves and the pages have two revision inputs. Both inputs are passed explicitly at a cut; neither is left to its default.

| Input        | What it means                                               | Renders on the page as                 |
| ------------ | ----------------------------------------------------------- | -------------------------------------- |
| `--rev`      | the revision the pages DESCRIBE; the register is read at it | `Revision this describes`              |
| `--baseline` | the released revision that presence is MEASURED AGAINST     | `Release presence is reported against` |

`--rev` defaults to `HEAD`, which is right between cuts and right at a cut once the ordering above is honoured.

**`--baseline` defaults to `v3.0.0` and that default has never moved.** It drives the `In <version>` column on every command row and the "newer than, so not in an installed copy" list, so a regeneration that omits it reports presence against the first cut of the line. Read the default rather than trusting this sentence:

```
  grep -n 'BASELINE=' intent/st/ST0056/parity/tools/gen_reference.sh
  grep -rh 'reported against' docs/reference/ | sort | uniq -c
```

A baseline is usable only where the register exists at it. Check before passing one:

```
  git show "v3.0.3:surface/dispatch-table.json" > /dev/null && echo usable
```

## The baseline is the PREVIOUS RELEASE TAG

**Ruled by hv on 2026-09-17 (decision 25), and the rule is a command rather than a judgement:**

```
  git describe --tags --abbrev=0
```

**`--tags` IS LOAD-BEARING, AND THIS PAGE SHIPPED WITHOUT IT.** `git describe` considers ANNOTATED tags only unless `--tags` is passed, and every annotated tag in this repository is a v1 or v2 one, the newest being `v2.3.2`, with the whole v2.1x line and all of v3 tagged lightweight. So the bare form answers `v2.3.2`, and it answers it for every cut until somebody annotates a release tag — **it does not age out, because there is no next annotated tag for it to find.** Read the tags rather than trusting that sentence:

```
  git describe --abbrev=0                                   # v2.3.2 -- the wrong answer, and a stable one
  git describe --tags --abbrev=0                            # the ruled value
  git for-each-ref --format='%(objecttype) %(refname:short)' refs/tags | grep '^tag '
```

The one merciful thing about this failure is that it is loud: the register does not exist at `v2.3.2`, so a regeneration against it refuses rather than reporting presence against a Bash-era release. **Whether release tags SHOULD be annotated is a separate question and an open one** — the practice stopped after `v2.3.2` with no decision on record, and annotated tags carry a tagger, a date and a message that lightweight ones do not. `--tags` is correct either way and does not depend on the answer.

That is what `--baseline` is passed at a cut. It was chosen over "the previous release a user can install" precisely because it cannot drift with anyone's reading of what counts as installable — the two pick different values in a case this project has already produced, and only one of them is answerable by a command.

**The case it was ruled with in view, because it is a precedent rather than a hypothetical.** v3.0.2 was tagged and its GitHub release created, and its artefacts were never published — which is why v3.0.3 exists at all, and CHANGELOG's own 3.0.3 entry says so. Under this rule the v3.0.3 cut would have measured presence against a release no reader could install. It does not bite the 3.1.0 cut, because the previous tag is v3.0.3 and that one shipped. If it ever bites again, the tag rule still applies and the anomaly belongs in that release's notes; the rule is not re-litigated at the cut.

**How this went wrong before the rule existed, kept because it is the reason the rule is written down.** The tool's default is the first cut of the line and has never moved. The last two regenerations both passed `v3.0.1` by hand — `a87a94a4a` at the v3.0.3 cut and `d21550ae5` on 2026-09-17 — the second of them because the value was copied from the previous invocation rather than derived from a rule, there being no rule to derive it from. A register exists at every 3.0.x tag, so nothing refused; the pages simply reported presence against a release two cuts back and said so accurately in a line nobody re-read.

## Running the generators

**They resolve the revision in the CURRENT WORKING DIRECTORY's repository, not in the tree they live in.** Run from anywhere else they exit non-zero with an empty log, which reads as a crash with no cause. Run them from the tree they describe, write to a scratch `--out` first, and read the diff before writing the tree.

```
  cd <the tree being described>
  BASE=$(git describe --tags --abbrev=0)
  intent/st/ST0056/parity/tools/gen_reference.sh    --rev <sha> --baseline "$BASE" --out <scratch>
  intent/st/ST0056/parity/tools/gen_cut_surface.sh  --rev <sha> --baseline "$BASE" --out <scratch>/cut-surface.md
  diff -ru docs/reference <scratch>
```

**`--out` MEANS DIFFERENT THINGS TO THE TWO GENERATORS, AND GETTING IT WRONG IS SILENT.** `gen_reference.sh` takes a DIRECTORY and writes twenty pages into it. `gen_cut_surface.sh` takes a FILE — its own usage says "write here instead of stdout", and it ends in `cp "$TMP/aligned.md" "$OUT"`. Hand it the scratch DIRECTORY and it copies its output in under the name `aligned.md`, exits 0, and prints a confident `ok: wrote <scratch>`. The `diff` on the next line then reports `cut-surface.md` as deleted and `aligned.md` as added, and an operator who syncs the scratch over `docs/reference` drops `cut-surface.md` from the cut and ships a stray file in its place. **Name the file in `--out`.**

**AND `gen_cut_surface.sh` HAS ITS OWN `--baseline`, DEFAULTING TO `v3.0.0`.** Pass it the same `$BASE` as the other half. Omitting it is the same class of silent wrong: the twenty command pages report presence against the ruled baseline while `cut-surface.md` reports against the tool's default, which breaks the one-commit-one-revision rule below on the dimension nobody thinks to check, and the page says so accurately in a line nobody re-reads.

Neither generator runs a binary; both read the register at the revision with `git show`, and they write only where `--out` points. `gen_reference.sh` writes the command pages; `gen_cut_surface.sh` writes `cut-surface.md`, whose input is `ALL_VARIANTS` in `native/rust/crates/intentsvcs/tests/error_remedies.rs` read at the same revision. Both halves go in ONE commit at ONE revision, because a set whose halves name different revisions is not a set anyone can check.

**`cut-surface.md` has a second trigger that has nothing to do with a cut:** any landing that adds a refusal variant makes it stale the moment it lands. Regenerate it with that landing rather than waiting for the release.

## The cut, in order

1. Land everything the cut carries. Confirm the estate is clean: `intent doctor`.
2. Regenerate both halves at `HEAD` against `--baseline "$(git describe --tags --abbrev=0)"`, read the diff, and commit them together. Do this BEFORE step 3 or the cut ships without them.
3. Run `bin/devbin build release --patch` or `... vX.Y.Z`, ideally `--dry-run` first. It makes the version-bump commit, tags it — so the tag now contains step 2 — asks for a confirmation, and pushes. hv fires this.
4. `int macos prepare` AT THE TAG, then `formula`, `publish`, `smoke --reinstall`.

Step 4 staging at the tag is what makes step 2's placement load-bearing: `prepare` stages the tag's tree, so anything committed after the tag is not in what ships.

## The pointer to this page lives in `CLAUDE.md` itself, not in the template

Worth writing down because the reflex is wrong and the cost of following it is not small. `CLAUDE.md` ends with a line saying it is generated from `lib/templates/llm/_CLAUDE.md`, so the instinct when adding an entry to its "Internal authoring docs" index is to edit the template. **The template has no such section.** What it has is a `<!-- user:start -->` / `<!-- user:end -->` region that is preserved across regeneration, and the index sits inside that region in the rendered file. So the entry goes into `CLAUDE.md` directly and survives the next render.

Following the reflex would have edited a file that does not carry the section, and would have done something worse on the way: `lib/templates/llm` is inside `DIRT_SCOPE` — the path set whose changes re-stamp the build marker, declared in `native/rust/build-support/source_commit.rs` and worth reading there rather than trusting this sentence — while the repository root is not. A documentation pointer written to the template therefore marks the installed pair as carrying unbuilt changes for every node on the project, and a pointer written to `CLAUDE.md` moves nothing built.

## How the commands on this page were verified, which is a field and not a courtesy

**A command in a document is a claim, and it is only worth more than a sentence once somebody has run it.** This page landed on 2026-09-17 carrying four command defects, every one of them written carefully and none of them driven: the baseline missing `--tags`, a `grep` whose pattern could not match its own subject, `--out` handed a directory where a file was meant, and a second generator never given the baseline the first one got. Three of the four fail silently or plausibly, which is exactly why writing them with care did not catch them. **An unrun command is a sentence in a monospace font, carrying more authority than it has earned** (dc's phrasing, 2026-09-17).

**AND "RUN YOUR COMMANDS" CANNOT BE THE WHOLE RULE, BECAUSE THE COMMANDS THAT MOST NEED TO BE RIGHT ARE THE ONES NOBODY CAN SAFELY RUN.** `git tag`, the push, the publish and the brew steps are unverifiable by execution BY CONSTRUCTION, and they are the consequential half. So there are two methods, and which one applies is decided by the command rather than by the author's confidence:

| Kind                             | Method                                        | On this page                                                     |
| -------------------------------- | --------------------------------------------- | ---------------------------------------------------------------- |
| Cheap and read-only              | RUN it, and read what it printed              | every `git describe`, `git show`, `grep`, and both generators    |
| Destructive, outward or one-shot | READ it, as CONTROL FLOW rather than as lines | `bin/devbin build release` — the tag, the confirmation, the push |

The second row is not a weaker method and it caught the harder defect. The claim that this page originally made about the release driver — that it commits, tags and pushes in one run, so no window exists after the tag — was assembled from correct `grep` hits and was wrong, and what disproved it was reading the driver's control flow: the push sits behind a human confirmation, and declining it exits 2 with the tag already created. Correct citations, different behaviour.

**So a page that records WHICH WAY each command was checked makes this class visible at review**, and that is the reason this section exists rather than a note in a commit message. The defect that shipped sat squarely in the runnable half and nobody ran it, which is a question a reviewer can ask in one line.

**ONE OF THE FOUR SHOWS WHERE AN UNRUN COMMAND COMES FROM, WHICH IS NOT CARELESSNESS.** The broken `grep` matched the presence line as it READS in prose — "reported against `v3.0.1`" — while the generated pages render it as two cells of a table with a boundary and padding between the phrase and the version. The pattern was written against a mental model of the sentence rather than against the artefact. It was checked at the three previous regenerations and matched at none of them: it never worked, rather than working and rotting. **A pattern aimed at generated output is aimed at a RENDERING, and the renderer is a writer you did not consult.**

**AND AFTER FINDING ONE BAD COMMAND, RUN THE OTHERS — BUT ESTABLISH FIRST THAT YOU CAN SEE THEM ALL.** This page was audited twice on the day it was corrected. The second audit swept for command lines with an alternation of the tools a release document was expected to contain, typed from memory; it had no entry for `grep` and none for a bare script name, so two of the five fenced blocks were invisible to it, and it reported the three it could see as clean. **An instrument that cannot exhibit the failure reports clean, and that reading was then offered as reassurance** — which is the worst direction for a narrowed population to fail in, because an undercount that produces a warning gets checked and an undercount that produces an all-clear gets believed and quoted.

**So count the FENCES, not the matches of a guess.** The enumeration that settles it is structural and takes one command: walk the file, toggle on each ` ``` `, and print every line inside. Against that census, three of the five blocks this page carried when it landed were defective. A pattern that already assumes what it is looking for cannot tell you what is there.

## What this page does not carry

No counts, no inventories and no state. Every answer above is regenerated by the command beside it, deliberately, because the failure this page was written to fix is a step that survived only as a number in somebody's commit message.
