---
verblock: "21 Aug 2026:v1.14: vc - the v2/v3 split; intentdb retired; gate scope corrected"
intent_version: 2.19.0
---

# Work In Progress

## Current State (as at `69a5db5e`, 2026-08-20)

**This heading names a COMMIT, not a date.** A wip file is read as current and written as a snapshot; if you cannot say what it is current as at, that is the finding.

**THE GATE IS 62 OF 67, AND IT TAKES THREE VERB CALLS RATHER THAN THE TWO THIS LINE USED TO NAME.** The scope is _all of ST0057's live rows plus all of ST0056 WP-03's_: `intent ac status ST0057` (47/51), `intent ac status ST0056/03` (15/16), and `intent ac gate ST0057` for the outstanding ids. **`intent ac status ST0056` answers 59/132 and is NOT this number's denominator** -- that is the whole thread, not the gate. **The third call is the one nobody wrote down:** `ST0056/03` is a WP-scoped STID and the verb accepts it. **Three figures in this file have now been wrong, and the third was wrong for a NEW reason** -- not stale, not double-counted, but because the line forbade hand-tallying while naming an instrument that could not reach the number, so the only way left to obey it was to copy the banner. **Run the three calls. Do not add them up from memory.**

**THE DISK MODEL IS RUNNING, NOT DESIGNED.** `intent organize --apply` removed 423 files at `e7f00e65`; `intent/st/` holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database. **Proven reversible by measurement**: ST0001 rehydrated to five files byte-identical to git, a fence-heavy pair to fifteen, and all 282 attachments verify against their own `sha256`.

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2. **DO NOT PUT v3 ON PATH** -- the pre-commit gate works _because_ it runs v2, whose version guard is scoped to writes. On PATH, `intent critic` answers 2 in all five declared languages, which is the code the gate fails open on, here and in the other 15 Intent projects on this machine.

### The two threads

**ST0056 -- the v3.0.0 rewrite.** Architecture in `design.md`. **The SQLite db is the DURABLE SSOT; nothing on disk is truth.** **There is no `intentdb` -- that word was a TYPO that propagated corpus-wide, and it names no component.** The crates are `intent-cli`, `intentd` and `intentsvcs`; **`intentd` is a CLIENT of the db exactly as the CLI is, and exists only for wider features beyond single-project operations. It is not the SSOT and no read requires it** (hv, 2026-08-21). D01 was REVERSED by hv 2026-08-15 -- do not reason from it. **133 criteria / 137 tests, 59 of 132 satisfied, 1 withdrawn.** WPs 01/02 Done; 03/04/05/06/07/10/11 WIP; 08/09/12-16 Not Started.

**ST0057 -- disk as a sparse projection.** **Sparseness applies to VIEWS; canon is NEVER sparse.** **53 criteria / 53 tests, 47 of 51 satisfied, 2 withdrawn.** WPs 02/04/06/07/09/10 Done; 01/03/05/08 WIP.

### THE GATE: 62 OF 67, AND THE FIVE THAT ARE LEFT

All of ST0057's live rows (47/51) plus all of ST0056 WP-03's (15/16). **Outstanding, with owners:**

| row      | thread       | owner | why it is not green                                           |
| -------- | ------------ | ----- | ------------------------------------------------------------- |
| AC-01.5  | ST0057       | cc    | red                                                           |
| AC-03.6  | ST0057       | cc    | red                                                           |
| AC-03.14 | ST0056 WP-03 | cc    | AT-03.15 red                                                  |
| AC-07.7  | ST0057       | ic    | **newly minted 2026-08-20, unbuilt**                          |
| AC-08.5  | ST0057       | ic    | red -- the pin is a measurement and its measured set is empty |

**dc holds none of the five.**

### THE ARCHITECTURE hv RULED, replacing the two-region manifest design

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds an id, `st done` removes it, a human may edit it. **Nothing derives it from status.** **ABSENT IS NOT EMPTY** -- a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

## What changed on 2026-08-21 -- THE v2/v3 SPLIT

**THE v2 CLI LEFT THIS CHECKOUT. `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` -- main HEAD, NOT the `v2.19.0` tag.** Every other Intent project on this machine now resolves there, at byte-identical behaviour to what it was already running.

**The branch point is the part worth keeping.** The old symlink resolved into the working tree, so the fleet had never run the tag; branching there would have reverted **2027 commits** across every project on this machine while presenting as a symlink move. **A released tag is evidence about a release, never about a deployment -- ask what the consumer is ACTUALLY running before choosing what to freeze.**

**And the binding was never the symlink.** Three routes reached this checkout -- the `~/.local/bin` symlink, an exported `INTENT_HOME`, and `$INTENT_HOME/bin` on PATH -- and `bin/intent:26` self-resolves only `if [ -z "$INTENT_HOME" ]`, so **the exported var beats the symlink outright.** Changing the symlink alone would have succeeded at rc=0 and moved nothing. `env -u VAR <cmd>` is the one-line test that tells an override from a resolution defect.

**WHAT IT UNLOCKS: `bin/` is no longer load-bearing for anyone else**, so v2 shell can be pruned here without breaking fifteen projects, and "DO NOT PUT v3 ON PATH" has lost the constraint that motivated it. **WHAT IT COSTS:** this repo's commit guards now resolve out of the frozen v2 checkout; identical today, drifting from the next guard change. **Routed to dc as a mechanism** -- hv declined direnv (covers a prompt, not automation) and hand-refresh (an advisory is not a control).

**Also 2026-08-21:** the word `intentdb` retired corpus-wide -- it names no component, and the architecture is `intentd` and `intent-cli` BOTH as clients of `intentsvcs`, which solely owns the SQLite db (`design.md:12-17`, unchanged for the whole rewrite). **The gate's scope corrected: 62 of 67 is ST0057's CLOSURE gate, not the 3.0.0 release gate** -- the release is WP-12, dependent on all prior WPs, with ST0056 at 59/132 and seven WPs Not Started. A 190-row AT citation sweep came back clean at one anomaly (`AT-00.6`). `runner_roster_check.sh` was found to have a population bounded to ST0056 and to `*_check.sh`, leaving ST0057's seven parity instruments undeclared while the guard reports clean on every commit; **hv ruled the population widens and all declare.** The five ST0057 criteria resting on undispatched instruments were DRIVEN: four confirmed, and `no_daemon_required.sh` refused at exit 2 on a needle defect -- `pgrep -f 'intentd'` matching `intentdb` inside every node's own system prompt.

## What changed on 2026-08-20

**D57-8 STOPPED CONTRADICTING ITSELF** (`c5320329`). Its fenced list enumerated nine forms, every one an entity; its READ/WRITE clause required three COLLECTION addresses by name; its under-addressing clause wrote out a fourth in full. Four collection forms were mandated in prose, implemented in `address.rs`, and absent from the only place a reader -- or a test -- goes to enumerate the grammar. **The cost was paid by the node who sourced it correctly**: `d57_8_forms()` was built from the DESIGN rather than from `address.rs`, on the sound ground that a denominator read out of the implementation agrees with it by construction, and it came back four short. **AC-07.1 is NOT reopened** -- its population is _every ENTITY form_ and against nine it is faithful. **AC-07.7 / AT-07.7 minted** for collection resolution.

**FOUR ROWS GREENED AND ONE CRITERION ATTESTED BY EVIDENCE** (`8d20dc49`), driven at `28b3610b` in a clean detached worktree: 140 targets, 985 passed, 0 failed. **ST0057/WP-04 closed on 7/7 verified rather than accepted.** WP-06, WP-09 and WP-10 also closed this cycle.

**AT-11.6's RE-CITATION WITHDRAWN AND THE DELIVERABLE RESTORED.** `prepush` clones HEAD to a temp dir and builds there -- it never touches the shared release path, refuses nobody, names no paths. AC-11.6 weighed both mitigations and ruled FOR refusal, and the row had been re-cited onto the one it declined. Separately, `bin/int` is a symlink to the dispatcher and contains no `prepush` at all.

## Next

1. **cc** -- ST0057 AC-01.5, ST0057 AC-03.6, ST0056 AC-03.14; ST0056 AC-10.4 built over `migrate::plan`'s write set with a non-empty control; AT-10.2's second citation onto `intent-cli/tests/ingest_command.rs`; AT-10.12 still held on the unexplained trim asymmetry.
2. **ic** -- AC-08.5; AT-07.7, whose red-first arm **must be `AcCollection` specifically**, because the other three come from the POST clause and any test sourced from that paragraph reaches them.
3. **dc** -- holds none of the gate. AT-11.6's deliverable is theirs and stays unbuilt.
4. **vc** -- `declared_but_unwired` adequacy; the heartbeat-currency note for hv; the eleven-copies filing (one v2-estate builder now in `common/mod.rs`, nine pre-existing named and left).
5. **Everyone, hv's standing question:** 250 files under `intent/` are not in the store at all -- `docs/`, `llm/`, `history/`, `eng/`, `plugins/`, and the project-level `done.md` / `wip.md` / `restart.md` / `todo.md`. hv: _not all of that should be in the db, but certainly some of it should._

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
