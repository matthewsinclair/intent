# Claude Code Session Restart -- narrative state

## Current state (as at `69a5db5e`, 2026-08-20)

**This heading names a COMMIT, not just a date, and that is deliberate.** A restart file is read as CURRENT STATE and written as a snapshot of when its author typed; nothing used to mark which, and a cold session treated a four-day-old line as the next action. **Re-stamp it when you fold, and if you cannot say what it is current as at, that is the finding.**

## First actions after `/compact` or new session

0. **IN A FRESH CLONE ONLY, RUN `int hooks` FIRST.** This repo's hooks are TRACKED, at `.githooks/`, reached by `core.hooksPath` -- but **`core.hooksPath` is repo-local config and a clone does not inherit it.** So a fresh clone has the hook bodies and runs none of them: the critic gate, the clock guard, the header guard, the canon-ignore guard, the append-only guard and all three formatters are silently inert. `int hooks` reports the state and names the fix; **`int hooks --install` is the only thing that writes.**

   **NOTHING TRIGGERS THIS AUTOMATICALLY AND NOTHING CAN.** Git runs nothing on clone, deliberately -- a clone-time hook would be remote code execution -- and a hook cannot report that hooks are off, because it would not run either. **This step is a habit, and today proved three times over that habits decay invisibly** (the guard roster, `prepush`, `check format`). What limits the damage is that CI catches the CONSEQUENCE even when nothing catches the CAUSE: unwired hooks mean unformatted code, and `cargo fmt --check` runs on every push.

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, and chains `/in-whiteboard pickup` (the board exists: `hv`, `cc`, `dc`, `ic`, `vc`). Declared languages: elixir, author, content, rust, shell. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md` + `intent/restart.md`.**

## State (as at `69a5db5e`, 2026-08-20)

**THE GATE IS 63 OF 67** Run `intent ac status ST0057` and `intent ac status ST0056` -- the verb prints `N/M satisfied, K withdrawn`. **Two hand-tallied figures in a row were wrong**, the second because its two halves counted "live" by different rules. **Never re-derive this by hand.**

**ST0056** (v3.0.0 rewrite) -- 133 criteria / 137 tests, **59 of 132 satisfied**, 1 withdrawn. **The intentdb is the DURABLE SSOT; nothing on disk is truth.** D01 was REVERSED by hv 2026-08-15; **do not reason from it.**

**ST0057** (disk as a sparse projection) -- 53 criteria / 53 tests, **47 of 51 satisfied**, 2 withdrawn. **Sparseness applies to VIEWS; canon is NEVER sparse.** WPs 02/04/06/07/09/10 Done.

**The five outstanding, with owners:** ST0057 AC-01.5, ST0057 AC-03.6, ST0056 AC-03.14 (cc); ST0057 AC-07.7, ST0057 AC-08.5 (ic). **dc holds none.** **Every row id here is thread-qualified deliberately** -- a bare `AC-03.6` resolves in ST0056 to a GREEN row about FTS prose bodies, so it reads as DONE to anyone who looks it up in the wrong thread.

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

**`.intentfiles` is DURABLE STATE.** Many writers, no recomputation -- `st new` adds an id, `st done` removes it, a human may edit it, **nothing derives it from status**. **ABSENT IS NOT EMPTY**: a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Roles (hv):** cc builds, ic runs parity/interface, dc owns DevX and distribution, vc stewards (contract, WP-close verification, hv interface; holds ST0056 + ST0057). **localfold = your own board; globalfold = project-wide docs, and it is vc's.**

## Next

1. **cc** -- ST0057 AC-01.5, ST0057 AC-03.6, ST0056 AC-03.14; ST0056 AC-10.4 built over `migrate::plan`'s write set with a **non-empty control**; AT-10.2's second citation onto `intent-cli/tests/ingest_command.rs`; AT-10.12 held on the unexplained trim asymmetry.
2. **ic** -- AC-08.5; AT-07.7, whose **red-first arm must be `AcCollection` specifically** -- the other three come from D57-8's POST clause and any test sourced from that paragraph reaches them, so a test that passes without `AcCollection` has reproduced the original defect one level down.
3. **dc** -- holds none of the gate. AT-11.6's deliverable is theirs and stays unbuilt.
4. **vc** -- `declared_but_unwired` adequacy; the heartbeat-currency note for hv; cc's eleven-copies filing.
5. **hv's standing question:** 250 files under `intent/` are not in the store at all.

## Carried from the previous fold -- NOT RE-VERIFIED at `69a5db5e`

**These were live on 2026-08-19 and this fold did not re-measure them.** Marked rather than dropped, and marked rather than asserted: **a rewrite that silently drops an item is indistinguishable from one that resolved it**, which is the class this estate spent 2026-08-20 documenting. Re-measure before acting; do not treat presence here as evidence either way.

- **cc** -- wiring `intent doctor`'s view-skew detection into the gate. **The detection exists; only the wiring is missing.** This is NOT ST0057's AC-03.6, which is about a commit containing canon that names bytes absent from that commit -- and note that ST0056 ALSO has an AC-03.6, green, about FTS prose bodies.
- **ic** -- `st hydrate`'s render arm; the `st edit` fork, unruled; the `issues dehydrate` bucket ruling that understates by four.
- **dc** -- the hosting sweep: 16 of 32 families dispatch, `intent claude` implements 1 of 8, against 230 call sites in this repo's own machinery.
- **Resolved since, verified here:** dc's AC-06.3 and AC-06.4 are both **green** (ST0057/WP-06 closed). vc's ST0057/WP-09 is **Done**.

## Traps that cost real time

- **THE GATE, AND ANY N-OF-M, IS COMPUTED BY A VERB. `intent ac status`.** Hand-tallying it produced two wrong numbers in two days, and the second was wrong because its halves used different definitions of "live".
- **NO INSTRUMENT HERE CATCHES AN EXPIRED CITATION -- ONLY A BUILDER TRYING TO SATISFY THE ROW DOES.** `at lint` exempts `to-write` from L2/L3, **correctly**, so a citation cannot be validated until it is used. **The cheap split: does the cited file carry the row's own literal id?** 2 hits means ready to green; **0 hits means the citation is wrong.**
- **A DOCUMENT CAN GO STALE AGAINST ITSELF, AND DOING THE SOURCING CORRECTLY IS WHAT DELIVERS THE WRONG ANSWER.** `at lint` checks rows against files; `doctor` checks views against canon; **a design document's clauses are checked by a reader noticing.**
- **A CHANGE THAT WOULD CONVENIENTLY GREEN YOUR OWN WORK IS THE ONE TO STOP AND ROUTE.** The tell, not the virtue.
- **THE REVISION IS PART OF THE FINDING.** Name revision, clock and dirty count on every measurement. **A suite started at T over a tree edited at T+n describes no revision.**
- **A TRUE MEASUREMENT OF A DIFFERENT PROPERTY, OFFERED AS PROOF.** The evidence being real and driven is exactly what makes it persuasive. **A background waiter's exit code is its own, never the watched process's verdict.**
- **`cargo test | tail -N` THEN COUNTING IN THE TAIL** reports the tail as the total -- 7 targets of 140. **And `$?` after a pipe is the LAST stage's rc, never cargo's.** Redirect to a file; read the rc directly.
- **ZERO FAILURES ACROSS A WORKSPACE DOES NOT PROVE A BINARY RAN.** Confirm each subject appears in the `Running` list before moving a row.
- **ISOLATE THE TARGET DIR, KEEP IT INSIDE THE CHECKOUT, AND USE AN ABSOLUTE PATH.** `install::home()` walks `current_exe()` ancestors for a marker, so an out-of-repo binary returns NotFound. A relative path under a drifted cwd built **1.2G** where gitignore hid it.
- **THE SHELL cwd PERSISTS BETWEEN CALLS**, and `&&` on a probe makes a failed probe indistinguishable from a clean estate.
- **`sync --to-store` IS DISK-AUTHORITATIVE FOR ATTACHMENTS** -- a canon-only edit to a realised attachment is discarded in silence at rc=0. Edit the FILE first. **For a typed field canon wins.**
- **`at lint` and the read verbs read the STORE, not canon.** Edit canon, `--to-store`, THEN lint. **Check the sync's rc, not its tail.**
- **`intent st list` defaults to in-progress and returns 2; `--all` is NOT a flag.** Use `st list --status all`.
- **Never `$?` after a pipe. `grep` is ugrep here and BSD grep in a `#!/bin/bash` script -- `-E` throughout. `grep -c` exits 1 on zero. The Bash tool's shell is zsh.** `cargo test` stops at the first failing target -- **`--no-fail-fast`, always.**
- **Read the clock, then PASTE -- never read, then type.**

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks; **always `git commit --only <paths>`** (a bare commit sweeps a peer's staged index). Whiteboard stamps carry a trailing `Z` read from `date -u`. matts runs the full suite externally and is the acceptance verifier. NEVER `--no-confirm` on the release. **DO NOT PUT v3 ON PATH.** **`upstream` was frozen and `prepush` now records the freeze LIFTED by hv 2026-08-20 with an empty `FROZEN_REMOTES` -- confirm with hv before any push there.** Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/int build release` date them at cut time.

## End of day -- 2026-08-20 17:38Z

**THE SUITE IS GREEN, DRIVEN ON hv's ONE-OFF AUTHORITY, AND EVERY NUMBER HERE IS FROM A COMPLETED FILE RATHER THAN A RUN IN FLIGHT.**

```
rust    fmt 0 | clippy 0 (-D warnings) | test 0    142 targets, 995 passed, 0 failed
shell   bats rc=0                                  1440 passed, 0 failed  (plan line: 1..1440)
critic  rust clean over 178 files; shell clean after `3882ffa5`
credo   N/A -- no mix.exs, no .credo.exs. Verified, not assumed.
```

**RUN THE SUITE THROUGH `tests/run_tests.sh`, NEVER `bats` DIRECTLY.** The runner exports `INTENT_FIXTURE_VERSION` from `VERSION` (`e474b419`, 2026-08-18); a direct `bats` invocation builds a **v3** fixture against the **v2** binary and every test dies on the version guard. **A direct run today produced 302 failures, 300 of them that one refusal, and none of them real.**

**STILL LIVE AND GATED ON hv -- RULED, EVIDENCED, DELIBERATELY UNSTARTED:**

- **`tests/lib/test_helper.bash:93` defaults fixtures to `3.0.0`.** The runner defends itself; the helper does not, so **every direct single-file `bats` run -- the invocation our own guidance prefers -- silently hits the trap.** dc's one-line convergent fix (default from `VERSION`) is written and justified by count: 37 files call `create_test_project`, all v2-driving; the 5 files touching the v3 binary never call it. **The default is wrong in every case where it takes effect.**
- **dc's two roster admissions**: `canon_commit_check.sh` (ST0057 AC-03.6, drives correctly, in neither roster) and `thread_view_skew_check.sh` (conditional on a staleness guard).

**FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED AND MUST NOT BE.** `bin/intent_st:1187`/`:1208` (`$LIST_ARGS`) and `bin/intent_treeindex:220` (`$prune_expr`) are **intentional word-splitting**; quoting them sends one argument where several are meant. `bin/intent_st:1353` is a fragment of a multi-line `sed` script the line-based proxy cannot parse. **A sweep driven to zero without reading each site breaks three live paths.**

**THE WHITEBOARD'S LIVE CHANNEL IS UNGUARDED AND THE HAZARD IS TRANSCRIPTION (cc, 2026-08-20 17:38Z).** The clock guard's three checks run at **commit**. A stamp sent over the live channel passes nothing -- vc sent one eight minutes ahead of UTC and nothing objected. **The danger is not the message: it is a peer quoting a live stamp into their board, where it enters the committed record laundered through them, past a guard posted at the wrong door, wearing their authorship.** The rule: **attribute a peer's live stamp, never assert it.** And: file stamps generated by `date -u` were all correct while typed message stamps drifted -- **read the clock, then PASTE.**

### The clock guard's residual risk is its ORDINARY case, not its exotic one (cc, 2026-08-20 17:41Z)

The shipped guard documents that **a stamp carrying a `Z`, landing in the past, and increasing monotonically passes all three checks** -- and files it as a smaller target rather than an empty one. **cc has named the generator that produces exactly that stamp, and it is not memory and not carelessness.**

**It is: read the clock ONCE, then advance by feel.** vc's run today went `17:36Z` (read) then `17:39Z`, `17:44Z`, `17:46Z` (arithmetic), finishing eight minutes ahead of UTC while `date -u`-generated file stamps in the same session were all correct.

**THAT SEQUENCE DOES NOT SLIP PAST CHECK C -- IT SATISFIES IT PERFECTLY.** Check C is the good one: two-sided, needs no clock, resting on the only thing that cannot be wrong -- time does not run backwards. **Increments-by-feel are monotonic BY CONSTRUCTION**, so a drifting run passes check C at every stamp, **more reliably than a careless but correct process would.**

**So the read is PER-STAMP and never per-session, and the reason is the failure mode above.** A second clock read is the only thing that catches it -- **and a second read is exactly what a session economises on once it believes it knows what time it is.** cc's own catch was a collision between two habits, not vigilance: the next read happened to land beside the line just written.

**And the live channel is unguarded entirely** -- all three checks run at COMMIT. **The hazard is transcription:** a peer quoting a live stamp into their board launders it into the committed record, past a guard at the wrong door, wearing their authorship. **Attribute a peer's live stamp; never assert it.**

**A fabricated stamp is never repaired.** You cannot recover a time you never read, and **a repaired stamp is indistinguishable from a read one -- which is the defect itself, applied a second time as a remedy.**
