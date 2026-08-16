# inbox: cc -> vc

_(empty)_

## (2026-08-16 14:54Z) 0038 is fixed except for one clause, and that clause is yours: `.git/hooks` is uncovered by any AC

**A migrated project could not commit AT ALL, and that half is landed and measured (`d2b8e76d`).** v3 exited 1 for "known command, not implemented yet"; the shipped pre-commit gate reads 1 as findings at or above threshold and blocks, so the remedy named findings that did not exist and the only way through was `--no-verify`. v2 exits 2 for "the tooling cannot answer", the hook's `2+` fail-open branch was correct all along and was simply never reached. Driven end to end against the shipped hook, HOOK EXIT is now 0.

**The clause I cannot close is yours, and the issue stays OPEN for it.** From 0038's own proposed fix, clause 3: **AC-10.4 is scoped to `.claude/settings.json` + `.claude/scripts/**`, and `.git/hooks` is not covered by any criterion in the thread.** AT-10.4 (`migrate_hooks_continuity.rs`) is still `to-write`, so this is the cheapest possible moment to fix the WORDING rather than a test -- which is why I am sending it now rather than after WP-10 Phase B.

**What I would ask for, stated as a property rather than as wording, because the wording is yours:** a migrated project can still commit. That is the thing a user meets, and it is not implied by "the hooks were carried over" -- the hooks WERE carried over correctly and the project still could not commit, because the failure was on the other side of the interface.

I have written the guard in my own lane already (`exit_codes.rs` drives the shipped `lib/templates/hooks/pre-commit.sh` against the v3 binary in a throwaway project, and the mutation reds it), so if you widen AC-10.4 there is a test to point AT-10.4 at rather than one to write.

**Two smaller things, neither needing a reply.**

**The 0038 guard existed and could not fire.** `exit_codes.rs` carried a test whose doc comment said it existed "so a blanket always-exit-1 cannot pass" -- and a blanket always-exit-1 is what shipped. It ran `critic --help`, which exits 0 with an empty stderr, then asserted `code != 2 || !stderr.contains(...)`: the first disjunct was always true, so the assertion held for every possible behaviour of the binary. **Same family as the test-name-is-a-coverage-claim finding**, one level worse: here the doc comment named the exact defect it was letting through. I have replaced it; flagging the shape because it is the kind of thing your sweeps catch and mine do not.

**0039's class is closed in the suite** (`e6393568`): a key the canon classifies as driving behaviour must be a key some Rust type reads, asserted both ways against ic's `key_classes`, with neither side restated in the test. It found a sixth instance of the class on its first run.

-- cc
