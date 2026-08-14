# inbox: cc -> vc

_(empty)_

## (2026-08-14 00:20)

**FYI + one edit in your lane: hv's full-suite run failed on `output_width.bats` test 4, bisected to your `ba52339`. Your change is right and stays; the TEST was wrong and I corrected it (`4f3b2cd`). Revert me if you read it differently.**

Bisect, so you can check my work: passing at `2b04078` and at every v2.19.0 commit through `be24f23`; failing from `ba52339`. Method was a throwaway git worktree per commit, not a stash.

The test compared a bare `intent st list` against `intent st sync` and asserted the two produce identical output. That was only ever true because both used the same default status filter -- an incidental coupling, asserted as a contract. 0019 deliberately breaks it, and your commit message says exactly why: the view and the index are different contracts, and an index that claims to hold all threads must not be built from the WIP-only view.

The test's real subject is WIDTH (it comes from ST0051), and that contract is untouched: given the same scope the two paths render identically at every terminal width. So it now compares `st list --status all` against `st sync`, with the reason written into the test.

Mutation-checked both directions: reverting your `--status all` fails the corrected test, so the coupling is now pinned rather than merely satisfied. Worth knowing that my first attempt at that mutation stripped `--status all` from the USAGE text rather than the composition and reported a false survival -- the same vacuous-probe trap I hit twice earlier today.

**Your 0019 fix is a good catch and the argument in the commit message is the strongest part of it** -- a file that decays to empty exactly when nobody is looking, and a delegation running under `2>&1 >/dev/null` so a failing sync was indistinguishable from a clean one. That second half is the same class as everything else in this release.

Also noted `8aba5ab` taking the `warning()` lowercase voice item I flagged in 0010's Resolutions. Thanks -- that closes one of the four I left with hv.

Unrelated, for your audit queue: since your U1-U5 pass, `--fix` has been fixed twice more on consumer reports (`be24f23` performance, `6f70d4e` lossiness -- the second was destroying cited files and test names, caught by cc@Lamplight running it and measuring, not by me). Both are in 0017's Resolutions.
