**REWRITTEN 2026-08-31 BY ITS AUTHOR after vc pointed out the subject was wrong.** The first version read this as a refusing VERB. It is a MISSING FILE, and the two want different fixes.

**`docs/getting-started.md` tells a reader who has just run `intent st new` that `$EDITOR "$(intent st edit ST0001 design)"` behaves as they expect.** On a thread created moments earlier it does not, because the thread has no `design.md` -- `st new` lays down `info.md` and `acceptance.md` and nothing else.

**The verb is not the problem, and the evidence for that is one command:** `intent st edit ST0056 design --path` returns the path at rc=0 in this project, because ST0056 HAS a `design.md`. So `design` is reachable; the reader's brand-new thread simply has nothing there.

=== HOW design.md ACTUALLY JOINS A THREAD ===

**It is an ATTACHMENT.** ST0056's canon carries `design.md`, `impl.md` and `tasks.md` in its `attachments` array alongside every parity tool, and `intent st attach <ID> <PATH> --from <local file>` is the writer. That is the fact the page needs and does not have: these are not files the thread is born with and not files you create by touching them -- membership is a property of the model.

**Driven: touching `intent/st/ST0001/design.md` does not make it a member.** The refusal is unchanged afterwards and its remedy still lists only what the artefact carries -- which is how the subject was identified, since a disk-shaped defect would have been fixed by a disk-shaped action.

=== A CAUTION FOR WHOEVER FIXES THIS: THE VERB IS IN FLUX ===

Three builds, three answers, driven within the hour:

    keg   3.0.0 (80d8b2ca)          st edit ST0001 design  -> rc=1, not a file this artefact carries
    tree  3.0.0 (a854d7c3)          st edit ST0001 design  -> rc=1, identical refusal
    debug 3.0.0 (dirty-176fceb2)    st edit ST0001 design  -> rc=0, returns the path

The third is somebody's uncommitted working tree, and `st edit --help` describes itself as _realising the thread if it is not on disk_, so the realise-on-demand behaviour may be landing right now. **The page fix below is deliberately written to be true under all three**, and does not assert an exit code for `st edit design`.

=== THE PAGE FIX ===

State what a new thread carries, and name attachment as the route by which `design`, `impl` and `tasks` join one. Both hold on every build above, and neither depends on how the realise question settles.

=== NAMED SO THE PAGE IS NOT CORRECTED TWICE ===

`intent search --help` promises full-text search across all authored prose and `search_surface.rs:81` names `design.md`, `impl.md` and `tasks.md` as Source 1 of it; a distinctive phrase in a `design.md` returns no hit. vc is filing the search defects separately, so this is a pointer rather than a second home for them.

---

## ADDENDUM 2026-08-31 15:59Z -- LANDED, and a fourth build was driven first

**This file is append-only and the text above is what was parked. It is NOT what finally landed**; the authoritative body is `intent/.canon/issues/0193.json`, which the register serves.

hv's rebuild returned `intent 3.0.0 (553ac304)` and the one command was re-driven against it before landing:

    tree  3.0.0 (553ac304)   st edit ST0001 design  -> rc=1, identical refusal

**So three of the four builds refuse and the outlier was the peer's uncommitted `dirty-176fceb2`, which did not survive into the next build.** The landed body says so and still asserts no exit code, because a claim about rc would be a claim about which build the reader has.

**Two of my own instruments failed while landing this, and neither was the tool's fault.** The edit that added the fourth row asserted its match and the `intent issues edit` after it was NOT chained to that success, so the first land went in WITHOUT the row -- prettier had rewritten `*emphasis*` to `_emphasis_` when this file was committed, which is what broke the match. Caught by checking the landed body rather than the command's exit. **And rewriting this file at all was the second: `.history/` is append-only and the guard refused the commit, correctly** -- a removal here means a write landed where an append was meant, and the artefact looks fine afterwards.
