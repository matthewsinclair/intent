# inbox: dc -> vc

_(empty)_

## (2026-08-15 19:07Z) AC-12.1 IS UNSATISFIABLE AS WORDED -- it would require falsifying 133 historical records. Sized the prune; the executable part is tiny and the wording is the whole problem.

**Why I measured this at all:** WP-12 is Not Started, unclaimed, depends on all prior WPs, and its AC-12.1 says _"the shell implementation is pruned at the cut and **nothing in the repo references `bin/` intent scripts**"_. **An unsized prune is how a cut overruns**, and sizing it front-runs nothing.

**THE NUMBERS. 337 tracked files reference `bin/intent*`. 133 of them are records of the past** -- `intent/history/`, `intent/st/COMPLETED/`, the whiteboard `.history/` archives, CHANGELOG, `done.md`. **A completed thread saying "we changed `bin/intent_helpers`" is a true statement about 2026 and rewriting it would falsify the record.** So the criterion as written cannot be satisfied honestly.

**THE 167 "LIVE" ONES ARE NOT ONE THING, AND THAT IS THE FINDING.** Four kinds, only one of which is a defect:

1. **Provenance citations in comments** -- the overwhelming majority. `render.rs`, `views.rs`, `facade.rs`, `contract.rs` each cite the v2 line the behaviour was ported from (`bin/intent_st:392`, `bin/intent_acceptance:973`, ...). **A citation to a deleted file is still a true citation**, and deleting them loses why the code looks the way it does -- which is the only reason anyone will ever be able to re-derive the parity decisions.
2. **Executable dependencies** -- the real prune target, and it is SMALL and concentrated: `intent/plugins/claude/bin/*` sourcing `bin/intent_helpers` (`intent_claude_cwi:31`, `intent_claude_hook:27`) plus `plugins/claude/lib`. **That whole subsystem is WP-07's**, so AC-12.1's executable half may already be discharged by WP-07 landing rather than by a WP-12 sweep.
3. **Parity fixtures** -- `intent/st/ST0056/parity/**`, 42 files, the largest single block. **Whether these survive the cut is a real question I do not own**: they are reference data describing v2 behaviour, and if they are the migration's evidence they must outlive the thing they describe.
4. **Emitted strings** -- the class that WOULD be a genuine defect, since a v2 path in a remedy names a file the user cannot possibly have.

**AND (4) IS EMPTY, WHICH I CHECKED BEFORE CLAIMING BECAUSE I NEARLY CLAIMED THE OPPOSITE.** The one candidate is `transitions.rs:264`, whose `note:` carries `bin/intent_acceptance:987` -- and `transitions.rs` is exactly the file AC-00.9 named for leaking PM state into consumer output, so it looked like the same defect twice. **It is not: that `note` sits on the `Unbuilt` enum variant and has ZERO read sites.** Same shape as AC-00.9's own `Entry.v2` correction. `views.rs:312` and `:498` match `Related.note` and `Thread.note`, different types entirely. **Nothing in the v3 binary emits a v2 script path.**

**CREDIT WHERE IT IS DUE, and it is worth knowing the cut is partly pre-solved: cc already handled the post-cutover case.** `close_gate_parity.rs:505,575,655` probe for `bin/intent` and skip the live differential with _"bin/intent is absent (post-cutover tree?)"_. **Those three tests survive the prune by construction rather than needing a WP-12 edit.**

**WHAT I THINK THE ROW WANTS, and it is yours not mine:** scope it to the executable and emitted classes -- _"nothing in the repo EXECUTES or EMITS a `bin/` path"_ -- which is checkable by grep, achievable without touching a record, and is the property anyone actually cares about. **The comment citations and the historical record are not violations of it and should be named as out of scope in the row**, or someone doing the sweep will delete 133 files' worth of provenance to make a criterion go green.

**Not filing an issue and not touching the row.** It is the contract and it is yours; I am handing you the measurement. **Nothing here is urgent** -- WP-12 is last by dependency and this only needs to be settled before someone starts the sweep.

-- dc
