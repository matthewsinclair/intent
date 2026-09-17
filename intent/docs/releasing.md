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
  grep -rho 'reported against `v[0-9.]*`' docs/reference/ | sort | uniq -c
```

A baseline is usable only where the register exists at it. Check before passing one:

```
  git show "v3.0.3:surface/dispatch-table.json" > /dev/null && echo usable
```

## The baseline is the PREVIOUS RELEASE TAG

**Ruled by hv on 2026-09-17 (decision 25), and the rule is a command rather than a judgement:**

```
  git describe --abbrev=0
```

That is what `--baseline` is passed at a cut. It was chosen over "the previous release a user can install" precisely because it cannot drift with anyone's reading of what counts as installable — the two pick different values in a case this project has already produced, and only one of them is answerable by a command.

**The case it was ruled with in view, because it is a precedent rather than a hypothetical.** v3.0.2 was tagged and its GitHub release created, and its artefacts were never published — which is why v3.0.3 exists at all, and CHANGELOG's own 3.0.3 entry says so. Under this rule the v3.0.3 cut would have measured presence against a release no reader could install. It does not bite the 3.1.0 cut, because the previous tag is v3.0.3 and that one shipped. If it ever bites again, the tag rule still applies and the anomaly belongs in that release's notes; the rule is not re-litigated at the cut.

**How this went wrong before the rule existed, kept because it is the reason the rule is written down.** The tool's default is the first cut of the line and has never moved. The last two regenerations both passed `v3.0.1` by hand — `a87a94a4a` at the v3.0.3 cut and `d21550ae5` on 2026-09-17 — the second of them because the value was copied from the previous invocation rather than derived from a rule, there being no rule to derive it from. A register exists at every 3.0.x tag, so nothing refused; the pages simply reported presence against a release two cuts back and said so accurately in a line nobody re-read.

## Running the generators

**They resolve the revision in the CURRENT WORKING DIRECTORY's repository, not in the tree they live in.** Run from anywhere else they exit non-zero with an empty log, which reads as a crash with no cause. Run them from the tree they describe, write to a scratch `--out` first, and read the diff before writing the tree.

```
  cd <the tree being described>
  intent/st/ST0056/parity/tools/gen_reference.sh    --rev <sha> --baseline <release> --out <scratch>
  intent/st/ST0056/parity/tools/gen_cut_surface.sh  --rev <sha> --out <scratch>
  diff -ru docs/reference <scratch>
```

Neither generator runs a binary; both read the register at the revision with `git show`, and they write only where `--out` points. `gen_reference.sh` writes the command pages; `gen_cut_surface.sh` writes `cut-surface.md`, whose input is `ALL_VARIANTS` in `native/rust/crates/intentsvcs/tests/error_remedies.rs` read at the same revision. Both halves go in ONE commit at ONE revision, because a set whose halves name different revisions is not a set anyone can check.

**`cut-surface.md` has a second trigger that has nothing to do with a cut:** any landing that adds a refusal variant makes it stale the moment it lands. Regenerate it with that landing rather than waiting for the release.

## The cut, in order

1. Land everything the cut carries. Confirm the estate is clean: `intent doctor`.
2. Regenerate both halves at `HEAD` against `--baseline "$(git describe --abbrev=0)"`, read the diff, and commit them together. Do this BEFORE step 3 or the cut ships without them.
3. Run `bin/devbin build release --patch` or `... vX.Y.Z`, ideally `--dry-run` first. It makes the version-bump commit, tags it — so the tag now contains step 2 — asks for a confirmation, and pushes. hv fires this.
4. `int macos prepare` AT THE TAG, then `formula`, `publish`, `smoke --reinstall`.

Step 4 staging at the tag is what makes step 2's placement load-bearing: `prepare` stages the tag's tree, so anything committed after the tag is not in what ships.

## The pointer to this page lives in `CLAUDE.md` itself, not in the template

Worth writing down because the reflex is wrong and the cost of following it is not small. `CLAUDE.md` ends with a line saying it is generated from `lib/templates/llm/_CLAUDE.md`, so the instinct when adding an entry to its "Internal authoring docs" index is to edit the template. **The template has no such section.** What it has is a `<!-- user:start -->` / `<!-- user:end -->` region that is preserved across regeneration, and the index sits inside that region in the rendered file. So the entry goes into `CLAUDE.md` directly and survives the next render.

Following the reflex would have edited a file that does not carry the section, and would have done something worse on the way: `lib/templates/llm` is inside `DIRT_SCOPE` — the path set whose changes re-stamp the build marker, declared in `native/rust/build-support/source_commit.rs` and worth reading there rather than trusting this sentence — while the repository root is not. A documentation pointer written to the template therefore marks the installed pair as carrying unbuilt changes for every node on the project, and a pointer written to `CLAUDE.md` moves nothing built.

## What this page does not carry

No counts, no inventories and no state. Every answer above is regenerated by the command beside it, deliberately, because the failure this page was written to fix is a step that survived only as a number in somebody's commit message.
