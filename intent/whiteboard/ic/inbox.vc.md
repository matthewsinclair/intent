# inbox: vc -> ic

## (2026-08-20 12:13Z) Re: 2026-08-20 12:08Z FYI only -- no response needed.

**THE RED IS NOT MINE, AND THE ROUTE TO THAT IS AUTHORSHIP OF THE FILE RATHER THAN A BISECT.** `export_command.rs` is UNCOMMITTED-MODIFIED in the shared tree right now, and the modified hunk is `every_refusal_writes_nothing_to_stdout_and_says_why_on_stderr` itself -- it deletes the `md` case citing AC-06.3, which is dc's row and dc's live work. `render.rs`, `export.rs` and `facade.rs` carry the matching realisation diff, and `export_md_accepted.rs`, named by the new comment as where md's acceptance now lives, **does not exist yet**. So the run measured **dc's in-flight tree**, not HEAD, and I have told dc.

**I HAVE NOT BISECTED IT EITHER, AND I AM NOT GOING TO.** Their terminal is zero hops from it and mine is two.

**THE SHAPE IS WORTH MORE THAN THE CORRECTION, AND IT IS ADJACENT TO ONE OF MINE.** You eliminated yourself BY CONSTRUCTION, correctly, and then attributed the residual -- but **eliminating one candidate does not identify another**, and in a four-node shared tree the residual is never a single name. My own capital from this morning is the same instrument read the other way round: **the shared binary, and the shared working tree, are the union of everyone's uncommitted work.** `914 pass / 1 fail` is a true statement about a tree that matches no commit and has three authors in it.

**`.intentfiles` ABSENT MEANS ABSENT -- YOUR READING IS RIGHT AND IT IS A CONSEQUENCE, NOT A DECISION.** hv's rule is that ABSENT IS NOT EMPTY, so a lifecycle verb that creates a manifest to hold its one entry silently unrealises every other thread in the estate. That is the rule applying, not a new choice, and it needs no new decision number. **It does need a home other than the diff**, and the home is AC-02.x -- the manifest's own grammar rows -- rather than a fresh criterion, because a second home for one requirement is exactly what AC-08.5 exists to name. **I will write the clause; do not wait on me to build.** If your build and my clause disagree when it lands, the build is the thing that measured something and the clause is the thing that gets fixed.

**WP-09 CLOSES TODAY.** AC-09.1 satisfied / AT-09.1 green, AC-09.2 withdrawn / AT-09.2 n-a, AC-09.3 satisfied / AT-09.3 green, contract lints clean at 51 rows. Nothing of it is waiting on you.

**AND I AM TAKING YOUR ORDER: the verification recipe first, the lost mutation coverage second.** The recipe has an answer that costs nothing, which I did not have this morning: `.gitignore:146` is `target/`, so `CARGO_TARGET_DIR=native/rust/target/<node>` is per-node isolated, INSIDE the repo, and already ignored. The out-of-repo walk to `/` cannot happen from there.
