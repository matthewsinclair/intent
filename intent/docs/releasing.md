# Releasing Intent

This page is the durable home for the steps a cut runs that no script runs for it. It exists because one of them — regenerating `docs/reference` — survived only in commit messages, and a step whose only home is a commit message is a step the next cut does not run. Everything here is either a command or a constraint on when a command is run; nothing here is a state, and nothing here is a count. Where a line would have to carry a number, it carries the command that answers it instead.

## Who fires a cut

hv does, in hv's own terminal. The release confirmation reads `/dev/tty`, so a tool session cannot answer it, and no tool session passes `--no-confirm`. The push gate stays human. "Get the release ready" is not "release it".

**And a cut is not reversible up to the push.** The driver commits, tags and pushes in the same run — see the next section — so there is no state in which the tag exists locally and a mistake can be quietly undone. By the time the command has returned, `main` and the tag are on both remotes. That is why the confirmation is a human's, and it is also why the driver cannot be invoked at all from a tool session while NO PUSH stands: it has no mode that stops short of publishing. Anyone planning a cut as a sequence they can back out of partway is planning something the driver does not offer.

## The ordering the tag depends on, which is the reason this page exists

**`bin/devbin build release` commits, tags and pushes in one run.** It commits the version bump (`bin/.devbin/cmd/build.d/release:1171`), tags `HEAD` — the commit it has just made (`:1221`) — and pushes `main` and the tag to both remotes (`:1240`, `:1242`). There is therefore no window between the tag being created and the tag being published into which a later commit can be inserted.

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

## OPEN, FOR hv: which release is presence measured against?

This is not settled and this page does not invent an answer. The facts, each re-readable by the commands above: the tool's default is the first cut of the line; the last two regenerations both passed `v3.0.1` by hand, `a87a94a4a` at the v3.0.3 cut and `d21550ae5` on 2026-09-17; v3.0.2 and v3.0.3 were both tagged on 2026-09-14 and both carry release notes; and a register exists at every 3.0.x tag, so any of them would work as a baseline.

**Under every reading of the convention, today's pages are at least one release behind**, because they report presence against `v3.0.1` while later releases have shipped. What is not established is the rule: "the previous release tag" and "the previous release a user can install" pick different baselines here, and only hv can say which the column is meant to answer. Whichever hv rules, it is written into this section as the rule, and the `--baseline` line of the cut command below is changed to match.

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
2. Regenerate both halves at `HEAD` against the ruled baseline, read the diff, and commit them together.
3. Run `bin/devbin build release --patch` or `... vX.Y.Z`. It makes the version-bump commit, tags it — so the tag now contains step 2 — and pushes. hv fires this.
4. `int macos prepare` AT THE TAG, then `formula`, `publish`, `smoke --reinstall`.

Step 4 staging at the tag is what makes step 2's placement load-bearing: `prepare` stages the tag's tree, so anything committed after the tag is not in what ships.

## The pointer to this page lives in `CLAUDE.md` itself, not in the template

Worth writing down because the reflex is wrong and the cost of following it is not small. `CLAUDE.md` ends with a line saying it is generated from `lib/templates/llm/_CLAUDE.md`, so the instinct when adding an entry to its "Internal authoring docs" index is to edit the template. **The template has no such section.** What it has is a `<!-- user:start -->` / `<!-- user:end -->` region that is preserved across regeneration, and the index sits inside that region in the rendered file. So the entry goes into `CLAUDE.md` directly and survives the next render.

Following the reflex would have edited a file that does not carry the section, and would have done something worse on the way: `lib/templates/llm` is inside `DIRT_SCOPE` — the path set whose changes re-stamp the build marker, declared in `native/rust/build-support/source_commit.rs` and worth reading there rather than trusting this sentence — while the repository root is not. A documentation pointer written to the template therefore marks the installed pair as carrying unbuilt changes for every node on the project, and a pointer written to `CLAUDE.md` moves nothing built.

## What this page does not carry

No counts, no inventories and no state. Every answer above is regenerated by the command beside it, deliberately, because the failure this page was written to fix is a step that survived only as a number in somebody's commit message.
