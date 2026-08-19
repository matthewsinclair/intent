# Drafted AT-01.2 and AT-01.4 -- NOT YET IN THE CRATE

Written 2026-08-19 while matts's full suite was live. Held out of `native/rust/crates/intentsvcs/tests/` deliberately: a new source file would not change the run's results, but it would mean the tree at report time is not the tree at build time, and that ambiguity costs more than the wait.

**Neither has ever been compiled.** Treat every API call as unverified -- `Fixture::new()` is an EMPTY project, so both mint their own threads, and that was already one correction.

- `clone_carries_canon.rs` -- ST0057 AT-01.2 / AC-01.2, three arms.
  1. `no_canon_path_is_matched_by_any_ignore_rule` -- `check-ignore --no-index`. **`--no-index` is load-bearing**: without it git never reports a TRACKED path as ignored and the arm returns clean on a broken tree. Zero latency: fires the instant the rule lands.
  2. `a_fresh_clone_carries_canon_for_every_artefact` -- a real `git clone`. **Denominator is the WORKTREE**, never `git ls-tree HEAD`, because if canon were ignored both counts would be 0 and the equality would hold.
  3. `every_canon_file_on_disk_is_tracked_by_git` (ic) -- compares what git HOLDS against what is on disk, and asks `check-ignore` WHY when it fires, because untracked has two causes with opposite meanings.

- `edits_land_in_distinct_files.rs` -- ST0057 AT-01.4 / AC-01.4. Asserts DISJOINTNESS, not `len() == 2`, because a count of two hard-codes the flat layout the criterion says not to inspect. **Both sets asserted non-empty first: the empty set is disjoint from everything.**

## Why `common::Fixture::clone_extract()` is NOT used

It is a directory copy with one hardcoded `.cache/` skip, and its doc comment claims it is "what `git clone` leaves behind". It encodes the author's belief about the ignore rules rather than asking git, so it would copy `.canon/` straight past the rule -- blind to precisely the failure AC-01.2 exists to catch. Fine for its own cold-start callers; a decoy here.

## The measurement behind all of it

Adding `intent/.*/` to `.gitignore`, built end-to-end in a throwaway repo and reproduced independently by ic testing the complement:

| detector                     | fires?                                                 |
| ---------------------------- | ------------------------------------------------------ |
| `check-ignore --no-index`    | YES, at zero new artefacts, the instant the rule lands |
| worktree count vs `ls-files` | YES, from the FIRST new artefact onward                |
| clone completeness           | NEVER -- complete and correct throughout               |
| `git status --porcelain`     | NEVER -- returns zero bytes                            |
| `check-ignore` (plain)       | NEVER -- rc=1 clean on a tracked path                  |

**Git's ignore rules apply only to UNTRACKED paths**, so existing canon stays tracked and clones perfectly while every artefact minted afterwards is silently skipped by `git add -A`. ic's generalisation is the one to keep: **an instrument that only ever asks git cannot see this, because git is the thing that has been lied to. The disk is the second channel and it is the one nobody consults.**
