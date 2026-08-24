---
verblock: "24 Aug 2026:v1.16: vc - gate 66 of 67, one row left; the five-estate config sweep; Intentv2 frozen"
intent_version: 2.19.0
---

# Work In Progress

## Current State (as at `50417c83`, 2026-08-24)

**This heading names a COMMIT, not a date.** A wip file is read as current and written as a snapshot; if you cannot say what it is current as at, that is the finding.

**THE GATE IS 66 OF 67 AND ONE ROW IS LEFT: ST0057 `AC-08.5`.** Driven at `50417c83`, 0 dirty, 2026-08-24, all three calls -- `intent ac status ST0057` (**50/51**, 2 withdrawn), `intent ac status ST0056/03` (**16/16, PASS**), `intent ac gate ST0057` for the ids. **`intent ac status ST0056` answers 61/132 and is NOT this number's denominator** -- that is the whole thread, not the gate. **The third call is the one nobody wrote down:** `ST0056/03` is a WP-scoped STID and the verb accepts it.

**THIS FILE HELD THE FIGURE TWICE AND THE TWO COPIES DISAGREED WITH EACH OTHER -- 65 here and 62 further down, in one document, both stale.** That is the finding, and it is not arithmetic: **a number with more than one home drifts in every home, and nothing here compares them.** Across the estate it had three homes and three values -- this file 65, `intent/restart.md` 62, `.claude/restart.md` 62 and untouched since 08-21. **Highlander applies to a number in prose exactly as it applies to code.** The gate figure now has ONE home in this file and the three verb calls above are how you get a current one; **anything typed here is stale from the moment it is typed.**

**CONTROLLED, BECAUSE THE CHEAP WAY TO FAKE A RISING FRACTION IS A SHRINKING DENOMINATOR:** 51 and 16 both held, and the withdrawn counts (2 and 1) both held. Four rows greened. **Cross-checked across two binaries** -- `intent3` (release) and the debug build return identical figures -- **which certifies the READ PATH is not divergent between builds and certifies nothing about whether the store agrees with canon.** Two readings of one store are one reading counted twice.

**THE DISK MODEL IS RUNNING, NOT DESIGNED.** `intent organize --apply` removed 423 files at `e7f00e65`; `intent/st/` holds `ST0046`, `ST0056`, `ST0057` and `steel_threads.md`. Fifty-two completed and two cancelled threads live only in the database. **Proven reversible by measurement**: ST0001 rehydrated to five files byte-identical to git, a fence-heavy pair to fifteen, and all 282 attachments verify against their own `sha256`.

**Intent is SELF-HOSTED on v3.** `bin/intent` (v2, 2.19.0) and `native/rust` (v3, 3.0.0-dev) coexist; a v2 binary REFUSES a v3-declared tree at exit 2. **~~DO NOT PUT v3 ON PATH~~ RETIRED 2026-08-22 ON BOTH OF ITS OWN GROUNDS, DRIVEN (ST0058).** **(1)** v3 is on PATH as **`intent3`**, a DISTINCT NAME, so `intent` is still v2 and the gate is untouched by construction rather than by care -- **this paragraph is the strongest argument for the distinct name and nobody cited it while choosing.** **(2)** The stated hazard no longer exists anyway: **`intent critic` under v3 answers rc=0 in all five declared languages** (elixir 9-of-19 rules asked, rust 1-of-7, shell 2-of-6, author and content clean), not 2. **The pre-commit gate still runs v2 and still works, and the sentence survives only as the reason the name had to be different.**

### THE LOCAL CUTOVER IS DONE AND v3 IS USABLE ACROSS THE ESTATE (ST0058, 2026-08-22)

**hv asked for _"useable by me across the wider estate here locally"_ -- a DIFFERENT BAR from the 3.0.0 release gate, and it is met.** Four gates: **U2 HARMLESS closed both directions, U4 REVERSIBLE driven, U1 INSTALLABLE built** (`intent3` + `int local`), **U3 DAILY-COMPLETE open** as five Rust builds. **Full procedure, migration floor, canary order and live findings: `intent/restart.md` and `intent/st/ST0058/design.md`.**

**Four commands:** `int local build`, `int local status`, `cd <project> && intent3 upgrade`, and back via `git checkout . && git clean -fd && rm -rf intent/.cache`. **The way back is `git`, so a project with uncommitted work has no way back.**

**THE MIGRATION FLOOR IS 2.19.0 AND 11 OF 16 PROJECTS ARE BELOW IT** -- they need a v2 `intent upgrade` first, verified on copies. **The canary order INVERTS: everything small and dormant is below the floor, so the first switchable projects are the large active ones. Baize first. `Intentv2` NEVER** -- every census finds it and it is the tool fifteen projects run.

### The two threads

**ST0056 -- the v3.0.0 rewrite.** Architecture in `design.md`. **The SQLite db is the DURABLE SSOT; nothing on disk is truth.** **There is no `intentdb` -- that word was a TYPO that propagated corpus-wide, and it names no component.** The crates are `intent-cli`, `intentd` and `intentsvcs`; **`intentd` is a CLIENT of the db exactly as the CLI is, and exists only for wider features beyond single-project operations. It is not the SSOT and no read requires it** (hv, 2026-08-21). D01 was REVERSED by hv 2026-08-15 -- do not reason from it. **133 criteria / 137 tests, 59 of 132 satisfied, 1 withdrawn.** WPs 01/02 Done; 03/04/05/06/07/10/11 WIP; 08/09/12-16 Not Started.

**ST0057 -- disk as a sparse projection.** **Sparseness applies to VIEWS; canon is NEVER sparse.** **53 criteria / 53 tests, 50 of 51 satisfied, 2 withdrawn.** WPs 02/03/04/06/07/09/10 Done; 01/05/08 WIP.

### THE GATE: 66 OF 67, AND THE ONE THAT IS LEFT

All of ST0057's live rows (50/51) plus all of ST0056 WP-03's (16/16). **Outstanding:**

| row     | thread | owner     | why it is not green                                                                                                      |
| ------- | ------ | --------- | ------------------------------------------------------------------------------------------------------------------------ |
| AC-08.5 | ST0057 | cc builds | the pin measures ONE entity through ONE door; the row says it will not green on an empty set over an unstated population |

**Closed since this table last named five:** ST0057 `AC-01.5`, ST0057 `AC-03.6`, ST0056 `AC-03.14` (cc) and ST0057 `AC-07.7` (ic), all satisfied 2026-08-22/23. **dc held none of the five and holds none of the one.**

**WHAT ACTUALLY BLOCKS AC-08.5 IS NOT THE PIN, IT IS THREE SURVIVING BURNING CASES**, and every one of them is a claim that a CAPABILITY IS ABSENT: `ST0011.completed` is a THREAD field with no setter; an attachment's canon record has no setter narrower than a thread; **no CLI verb creates an AC or an AT at all.** **The row's own history is four such claims refuted or narrowed the moment somebody finally checked** -- `at green` was said to destroy notes and does not in v3; `sync` was said to have no operation smaller than 57 threads and takes IDs; a pin asserted no creator existed while `put` created both thirty lines away in the same file. **Re-drive all three before building against them.** The class is not a wrong measurement; it is reasoning from an absence nobody looked for.

### THE ARCHITECTURE hv RULED, replacing the two-region manifest design

> **`.intentfiles` is DURABLE STATE -- the record of which database artefacts also have a realised form on disk.**
> **Realisation is driven from `.intentfiles`; commands change `.intentfiles`; `organize` realises it.**

**Many writers, no recomputation.** `st new` adds an id, `st done` removes it, a human may edit it. **Nothing derives it from status.** **ABSENT IS NOT EMPTY** -- a missing manifest keeps everything, a manifest declaring nothing keeps nothing.

**Three layers, and confusing them is the recurring error:** canon (`intent/.canon/st/<ID>.json`, committed, never sparse) / store (`intent/.cache/intent.db`, gitignored, the durable SSOT) / views (`info.md`, `acceptance.md`, committed, generated). **`acceptance.md` is a GENERATED VIEW -- a row authored there is discarded.**

## END OF DAY 2026-08-21 -- ALL FOUR NODES FOLDED, HOLDING FOR A BOUNCE

**Folded and paused, claims intact, tree clean:** cc `855a5e4e`, dc `084a683b`, ic `34530c41`, vc this commit. Globalfold is vc's; no peer touched a project-wide file, driven rather than assumed.

**THE ~13:30Z BOUNCE DID NOT TAKE, AND THAT IS MEASURED RATHER THAN INFERRED.** All four sessions fired `SessionStart:resume`, kept this morning's conversation, and **no board's `session_id` changed after its ~09:29Z pickup** -- driven from `git log -p` over each board. Each id changed exactly ONCE today, at that pickup, and not since.

**VERIFICATION FOR THE NEXT BOUNCE IS ONE FIELD: a genuinely fresh session's `$CLAUDE_CODE_SESSION_ID` DIFFERS from the one recorded on its board.** Same id means it resumed again. **DO NOT use `ListAgents`' `started` column -- it is SOCKET age, not SESSION age.** A topology change re-registers every peer, so everyone looks freshly started to everyone else; **all four nodes read it, all four reported "three of four bounced, but not me", and all four were wrong.** Each could observe the discriminating field only about ITSELF, so agreement between four nodes was worth nothing. **Consensus is not corroboration when every node used the same broken instrument.**

**NODES CANNOT SELF-COMPACT.** `/compact` is hv-triggered. A fold instruction reaches "folded to files" and stops there. Before a bounce the compact is redundant -- the bounce discards what a compact would summarise.

**OPEN FOR hv, AND IT IS A PROTOCOL GAP RATHER THAN A SLIP: MONIKERS ARE ESTATE-SCOPED AND NOTHING MARKS THEM.** `ic`, `cc` and `vc` exist in Intent, Lamplight AND Laksa simultaneously. `/in-whiteboard` says _"the 2-letter moniker is the directory name, the routing key, and the handle"_ and defines the roster per-project -- correct inside an estate, and **silent about crossing one.** Cross-estate relay is live, not hypothetical: hv pastes transcripts between estates and vc messages `lamplight-vc` directly. **It fired today: vc read `lamplight-ic`'s `$path`-clobber incident out of a pasted transcript and instructed `intent-ic` to fold it as its own.** `intent-ic` refused, verified its own PATH, and kept the mechanism while declining the ownership. **Had it complied, an incident ic never had would be permanent in ic's record, and the next ic reads a board as lived experience with no way to tell.** That is the `intentdb` class arriving INSIDE A FOLD -- the worst site, because **a fold is where an unchallenged claim becomes permanent.** Contained at one (checked both directions: whether it landed on cc's or dc's boards, AND whether it was ever sent). **Write the qualifier every time a moniker crosses an estate: `lamplight-ic`, never `ic`.**

**ic's RULE, THE GENERAL CASE:** _a fold instruction is not a trusted source about your own history; a peer telling you what only you could know is telling you what THEY know._

## What changed on 2026-08-21 -- THE v2/v3 SPLIT

**THE v2 CLI LEFT THIS CHECKOUT. `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` -- main HEAD, NOT the `v2.19.0` tag.** Every other Intent project on this machine now resolves there, at byte-identical behaviour to what it was already running.

**The branch point is the part worth keeping.** The old symlink resolved into the working tree, so the fleet had never run the tag; branching there would have reverted **2027 commits** across every project on this machine while presenting as a symlink move. **A released tag is evidence about a release, never about a deployment -- ask what the consumer is ACTUALLY running before choosing what to freeze.**

**And the binding was never the symlink.** Three routes reached this checkout -- the `~/.local/bin` symlink, an exported `INTENT_HOME`, and `$INTENT_HOME/bin` on PATH -- and `bin/intent:26` self-resolves only `if [ -z "$INTENT_HOME" ]`, so **the exported var beats the symlink outright.** Changing the symlink alone would have succeeded at rc=0 and moved nothing. `env -u VAR <cmd>` is the one-line test that tells an override from a resolution defect.

**WHAT IT UNLOCKS: `bin/` is no longer load-bearing for anyone else**, so v2 shell can be pruned here without breaking fifteen projects, and "DO NOT PUT v3 ON PATH" has lost the constraint that motivated it. **WHAT IT COSTS:** this repo's commit guards now resolve out of the frozen v2 checkout; identical today, drifting from the next guard change. **Routed to dc as a mechanism** -- hv declined direnv (covers a prompt, not automation) and hand-refresh (an advisory is not a control).

**Also 2026-08-21:** the word `intentdb` retired corpus-wide -- it names no component, and the architecture is `intentd` and `intent-cli` BOTH as clients of `intentsvcs`, which solely owns the SQLite db (`design.md:12-17`, unchanged for the whole rewrite). **The gate's scope corrected: the figure is ST0057's CLOSURE gate, not the 3.0.0 release gate** -- the release is WP-12, dependent on all prior WPs, with seven WPs Not Started. **The SCOPE ruling is what this entry records and it still stands; the figures it carried (62 of 67, ST0056 at 59/132) were true ON 2026-08-21 and are not current** -- see the Current State section above, or run the three verb calls. A 190-row AT citation sweep came back clean at one anomaly (`AT-00.6`). `runner_roster_check.sh` was found to have a population bounded to ST0056 and to `*_check.sh`, leaving ST0057's seven parity instruments undeclared while the guard reports clean on every commit; **hv ruled the population widens and all declare.** The five ST0057 criteria resting on undispatched instruments were DRIVEN: four confirmed, and `no_daemon_required.sh` refused at exit 2 on a needle defect -- `pgrep -f 'intentd'` matching `intentdb` inside every node's own system prompt.

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
