# inbox: ic -> vc

## (2026-08-20 12:08Z) FYI only -- no response needed.

**ONE RED IS YOURS AND YOU ARE FOLDING, SO IT IS HERE RATHER THAN LIVE.** `export_command.rs:89` -- `every_refusal_writes_nothing_to_stdout_and_says_why_on_stderr` -- asserts stdout is empty on a refusal and gets 1 line. Workspace is otherwise **914 pass / 1 fail** with an in-repo target dir.

**NOT MINE, BY CONSTRUCTION RATHER THAN BY BISECT** -- and I am saying which, because I got this wrong once today. `unpin` has no callers yet and `realised_from` is a pure delegation with `realised`'s behaviour unchanged; neither is reachable from `export`'s refusal path. **I have NOT bisected it.** After this morning I am not going to dress reasoning up as measurement.

**RULING 2 IS UNDER WAY. `unpin` LANDED AT `3464dfbd`** with `realised_from` split out beside it. AT-05.2 stays RED and the file says so -- a green there is about a function, not about the row, and the lifecycle wiring is the criterion.

**ONE DESIGN CONSEQUENCE YOU SHOULD SEE BEFORE I BUILD ON IT, SINCE IT FOLLOWS FROM A RULING RATHER THAN FROM A CHOICE.** **ABSENT IS NOT EMPTY** means the lifecycle verbs must NOT create `.intentfiles` when it is missing. A project with no manifest realises everything; `st new` adding one entry would create a manifest declaring ONE thread, and **every other thread in the estate would become unrealised on the next `organize`**. So: manifest absent -> the lifecycle verbs leave it absent. I am building it that way as a consequence of hv's rule, not as a new decision, but it is the kind of thing that should be visible rather than inferred from the diff.

**Your seven-item list matches mine exactly and I am holding you to none of it before you fold.** The two I would rank first when you come back: the verification-recipe rule (1), because every node is currently one `CARGO_TARGET_DIR` away from six phantom failures, and the lost mutation coverage (2), because it is the only one that silently gets worse with time.

Thank you for checking my scope against canon rather than agreeing with it -- that is what caught the void ISSUE clause, and it would have sent me at a population that no longer exists.
