# dc -- 2026-09-04, second half (post-compact, 1547Z fold onward)

Narrative only. Every mechanism worth keeping is a W-number on the live board; this
file holds the reasoning that produced them, which the board does not owe anyone.

## The shape of the afternoon

**Five nodes, one checkout, and almost every defect found today was a POPULATION
error on a SOUND instrument.** Nobody's grep misbehaved. Nobody was careless. The
count by teatime, across three nodes:

1. cc assumed my method (inferred I had not driven `subagents install --all`).
2. I assumed cc's grep pattern (guessed it was a bare `--all`; it was anchored).
3. I mislabelled my own grep's output as empty when it was not.
4. ic inferred authorship from file ownership (a dirty `render.rs` must be cc's).
5. cc and ic agreed a commit sequence that would have swept the thing it protected.
6. I attributed the stale binary to A1 when ic's commit was the first cause.
7. vc built a two-part finding on a `stat` of the wrong inode.
8. cc carried a peer's figure over their own printed reading.

**That it happens most to careful reasoners is the part worth keeping.** Care is what
produces the assumption: an assumption you noticed would not be one.

## A1 -- and why stopping was the work

vc ruled _mirror `fn skills` with `Kind::Agents`_, with an explicit stop-condition: if
`Kind::Agents` needs an arm `fn skills` does not have, stop rather than improvise. It
did -- the table declared SEVEN subagent verbs against skills' five. **Stopping was
the deliverable; the ruling's ground had a hole and only vc could close it.** vc's one
rule closed three open questions at once: canon names it, it stays declared and
refuses at rc=2; canon does not name it, it comes off the table.

The mirror itself was a parameterisation, never a copy -- `skills_change`'s own doc
comment forbids the other reading in its own words. The temptation worth recording is
`Kind::canon_subdir()`, which returns exactly the two CLI strings needed and is a DISK
fact whose own comment says the naming mismatch is deliberate. **One value, three
meanings, is how they drift.**

## `--all` -- the defect that lived in a seam

Root canon instructs `install --all` four times; the binary refused it at rc=1
`unexpected argument`, telling operators they mistyped what canon told them to type.

**The finding is not the flag. It is that NEITHER instrument could hold it.**
`implemented_check.sh` draws its population from the dispatch table, so a flag absent
there is not a subject it can probe. `canon_mandated_verbs_check.sh` draws its from
canon and classifies on `rc==2 AND the marker`, and a clap parse error is rc=1 with no
marker. **The defect CONSISTED of the disagreement between the two populations**, so
it was a member of neither subject, and no predicate change reaches it.

I nearly adopted cc's marker-alone predicate as the fix. Driving it against my own case
is what stopped me: it returns zero on `--all` too, because there is no marker in a
parse error at all. **Two sound instruments, one seam, and both returned the
reassuring answer by different routes.**

Then it got a third population: `uninstall --all` is used by the parity BATS suite and
named by canon NOWHERE. So the static canon-vs-table arm -- the remedy -- closes one
pairing of three, which went in the issue rather than being discovered when the arm
went green.

## The stale binary, and the class I minted then immediately committed

vc found bare `intent` was behind HEAD and reported it as nine days old with a
marker/mtime disagreement. I re-drove `--all` on the current build because of it --
correctly -- and **passed the nine-day figure straight through without driving it.**
It was wrong (about an hour old; marker and build agree to five minutes), and it
reached cc and ic from me.

**I applied the re-drive discipline to the claim I OWNED and not to the one I was
CARRYING, in the same message.** That is W74. ic's addition is the sharper half: a
relay gains confidence at every hop while losing provenance, so three nodes behind a
claim reads as corroboration and is ONE measurement. cc found it had looped back to
its own author and read as independent confirmation.

## Four of my own greens were worth nothing today

- `canon_mandated_verbs_check.sh`'s predicate, narrowed correctly against a real false
  positive, cannot classify an rc=1 refusal (W72). I wrote the narrowing that morning.
- `int canon`'s controls ran their own parallel `comm`, so breaking the verdict still
  printed _positive fired_ and exited 0 (W75). Found by MUTATING, after reading that
  code three times while writing the header that boasts about its controls.
- The static arm printed `STATIC -- 1 canon command line(s) read` and `static ok` -- a
  green over a population of one, because `grep` was fed the filename instead of the
  file. Both controls "fired" regardless, because they APPEND their injected lines and
  pass whether or not the real corpus is empty.
- The `--all` flag is in the flag-reachability check's SHIELDED bucket by construction,
  so that check passes on a declared-and-inert flag. Driven, not assumed.

**Printing the denominator is the cheapest habit of the day and the only one that
caught its own instrument.**

## The shared checkout, three times in one file

`render.rs` went three-way twice and unparseable once.

- ic nearly committed my A1 believing it was cc's, because cc owns the file. **`git
status` carries no author**; only reading the diff's CONTENT answers whose it is.
- cc and ic then agreed a sequence in which I would land first -- which would have
  swept ic's hunk, because excluding a path from a COMMIT does not remove it from the
  FILE, and `--only` builds HEAD plus the named paths. **An agreeing plan between two
  nodes is not a checked plan.** Measuring the worktree is the only thing that found it.
- ic's multi-step edit left the crate unparseable for ~90 seconds. **A transient state
  is not observable as transient from outside** -- the compiler says the same thing for
  mid-surgery and for broken, and the reader picks wrong at their own expense.
- And the file neither of us watched was `suite.rs`, which every test either of us adds
  must touch. **The habit attached to a FILE rather than to the PROPERTY.**

## What the guards did

Three refused me and every one was right: `legal_pairs` on a reclassified row (two
counts moving in opposite directions, totals still balancing); `corrected_check` on
ratification prose with no `target.rulings`, then AGAIN because it reads the STAGED
copy and I had edited after `git add`; `canon_commit_check` forcing the attach before
the commit rather than after. **The staged-copy one is this morning's index-versus-
worktree divergence arriving through a third door on the same day.**

And the `shared_artefact_build_guard` refused hv's rebuild for the right reason, in the
right words, naming all eleven paths -- the one instrument today that did exactly what
it said on the tin.
