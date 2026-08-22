# Claude Code Session Restart -- narrative state

## Current state (as at `68296b8e`, 2026-08-22)

**This heading names a COMMIT, not just a date, and that is deliberate.** A restart file is read as CURRENT STATE and written as a snapshot of when its author typed; nothing used to mark which, and a cold session treated a four-day-old line as the next action. **Re-stamp it when you fold, and if you cannot say what it is current as at, that is the finding.**

## First actions after `/compact` or new session

0. **IN A FRESH CLONE ONLY, RUN `int hooks` FIRST.** This repo's hooks are TRACKED, at `.githooks/`, reached by `core.hooksPath` -- but **`core.hooksPath` is repo-local config and a clone does not inherit it.** So a fresh clone has the hook bodies and runs none of them: the critic gate, the clock guard, the header guard, the canon-ignore guard, the append-only guard and all three formatters are silently inert. `int hooks` reports the state and names the fix; **`int hooks --install` is the only thing that writes.**

   **NOTHING TRIGGERS THIS AUTOMATICALLY AND NOTHING CAN.** Git runs nothing on clone, deliberately -- a clone-time hook would be remote code execution -- and a hook cannot report that hooks are off, because it would not run either. **This step is a habit, and today proved three times over that habits decay invisibly** (the guard roster, `prepush`, `check format`). What limits the damage is that CI catches the CONSEQUENCE even when nothing catches the CAUSE: unwired hooks mean unformatted code, and `cargo fmt --check` runs on every push.

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the prompt gate, and chains `/in-whiteboard pickup` (the board exists: `hv`, `cc`, `dc`, `ic`, `vc`). Declared languages: elixir, author, content, rust, shell. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md` + `intent/restart.md`.**

## END OF DAY 2026-08-21 -- ALL FOUR NODES FOLDED, HOLDING FOR A BOUNCE

**Folded and paused, claims intact, tree clean:** cc `855a5e4e`, dc `084a683b`, ic `34530c41`, vc this commit. Globalfold is vc's; no peer touched a project-wide file, driven rather than assumed.

**THE ~13:30Z BOUNCE DID NOT TAKE, AND THAT IS MEASURED RATHER THAN INFERRED.** All four sessions fired `SessionStart:resume`, kept this morning's conversation, and **no board's `session_id` changed after its ~09:29Z pickup** -- driven from `git log -p` over each board. Each id changed exactly ONCE today, at that pickup, and not since.

**VERIFICATION FOR THE NEXT BOUNCE IS ONE FIELD: a genuinely fresh session's `$CLAUDE_CODE_SESSION_ID` DIFFERS from the one recorded on its board.** **THAT IS HALF A TEST AND THE HALF THAT FAILS IS THE ONE YOU WILL READ (vc + ic, both measured across their own compacts 2026-08-21): `/compact` DOES NOT ROTATE THE ID.** So **CHANGED proves a fresh session** -- sound, a compact cannot manufacture it -- while **UNCHANGED means resumed OR compacted**, two causes with opposite remedies and one reading. **hv triggers compacts deliberately and every node here has now done at least one**, so the ambiguous branch is the common case rather than the exotic one. **A discriminator that fails only in the direction you expected to read is worse than one that fails both ways, because nobody re-checks the reading they expected.** Ask the node, or read its hook mode; the id alone cannot separate them. **DO NOT use `ListAgents`' `started` column -- it is SOCKET age, not SESSION age.** A topology change re-registers every peer, so everyone looks freshly started to everyone else; **all four nodes read it, all four reported "three of four bounced, but not me", and all four were wrong.** Each could observe the discriminating field only about ITSELF, so agreement between four nodes was worth nothing. **Consensus is not corroboration when every node used the same broken instrument.**

**NODES CANNOT SELF-COMPACT.** `/compact` is hv-triggered. A fold instruction reaches "folded to files" and stops there. Before a bounce the compact is redundant -- the bounce discards what a compact would summarise.

**OPEN FOR hv, AND IT IS A PROTOCOL GAP RATHER THAN A SLIP: MONIKERS ARE ESTATE-SCOPED AND NOTHING MARKS THEM.** `ic`, `cc` and `vc` exist in Intent, Lamplight AND Laksa simultaneously. `/in-whiteboard` says _"the 2-letter moniker is the directory name, the routing key, and the handle"_ and defines the roster per-project -- correct inside an estate, and **silent about crossing one.** Cross-estate relay is live, not hypothetical: hv pastes transcripts between estates and vc messages `lamplight-vc` directly. **It fired today: vc read `lamplight-ic`'s `$path`-clobber incident out of a pasted transcript and instructed `intent-ic` to fold it as its own.** `intent-ic` refused, verified its own PATH, and kept the mechanism while declining the ownership. **Had it complied, an incident ic never had would be permanent in ic's record, and the next ic reads a board as lived experience with no way to tell.** That is the `intentdb` class arriving INSIDE A FOLD -- the worst site, because **a fold is where an unchallenged claim becomes permanent.** Contained at one (checked both directions: whether it landed on cc's or dc's boards, AND whether it was ever sent). **Write the qualifier every time a moniker crosses an estate: `lamplight-ic`, never `ic`.**

**ic's RULE, THE GENERAL CASE:** _a fold instruction is not a trusted source about your own history; a peer telling you what only you could know is telling you what THEY know._

## THE v2/v3 SPLIT -- 2026-08-21. READ THIS BEFORE THE STATE BELOW; IT CHANGES WHERE YOU ARE STANDING

**THIS CHECKOUT IS v3 ONLY NOW. The v2 CLI the fleet runs lives in a SEPARATE checkout and is no longer served from here.**

- **`~/Devel/prj/Intentv2`**, branch `v2-maintenance`, cut at `fb45e9ea` -- **main HEAD, NOT the `v2.19.0` tag.** Every other Intent project on this machine now runs that.
- **~~All THREE bindings moved~~ THERE ARE FOUR AND THE FOURTH IS ON NO LIST ANYWHERE (dc, 2026-08-21, driven): `~/bin/intent` at PATH 19, a SECOND symlink to the same target, made in the same minute. Deleting `~/.local/bin/intent` as "the" binding hands resolution to it, and it still answers v2.** The three that were known, and the symlink was the weakest of them: `INTENT_HOME=/Users/matts/Devel/prj/Intentv2`, `~/.local/bin/intent -> Intentv2/bin/intent`, and `$INTENT_HOME/bin` on PATH. **`bin/intent:26` is `if [ -z "$INTENT_HOME" ]`, so the exported var BEATS symlink resolution** -- repointing the symlink alone would have returned 0 and moved nothing.
- **The branch point was main and not the tag on purpose.** The old symlink resolved into the working tree, so the fleet had NEVER run `v2.19.0`. Branching at the tag would have rolled **2027 commits** back across every project on this machine -- `intent_critic -94`, `intent_acceptance -144` -- **while presenting as a symlink move.** A released tag is evidence about a release, never about a deployment.

**WHAT IT UNLOCKS, AND IT IS THE POINT: `bin/` IS NO LONGER LOAD-BEARING FOR ANYONE ELSE.** "DO NOT PUT v3 ON PATH" existed because ONE checkout served the fleet and the rewrite at once. **That constraint is gone.** Pruning v2 shell here breaks nobody. Whether v3 goes on PATH is now hv's call on its merits rather than a hazard to fifteen other projects. **~~`intent` on PATH is v2.19.0 and answers for the FLEET -- to drive v3, use the explicit path.~~ HALF STILL TRUE, 2026-08-22: `intent` IS still v2 and still answers for the fleet -- and v3 is now on PATH as `intent3`, a wrapper in this repo's own `bin/` (dc, ST0058/U1). The explicit path is no longer the only route and is no longer the recommended one.**

**~~WHAT IT COSTS -- LIVE, UNSOLVED~~ CLOSED BY dc AND VERIFIED IN THE GATE'S OWN OUTPUT, 2026-08-22: every commit now prints `intent gate: guards read from THIS repository (/Users/matts/Devel/prj/Intent/lib/templates/hooks), not from INTENT_HOME`.** The account below is the HISTORY of the defect, not its present state -- and it is left standing because the SIBLING is still open (see the local cutover section). This repo's commit guards USED TO resolve out of the FROZEN v2 checkout: `.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`. **All five guard files are identical today; drift starts the moment anyone improves a guard here**, and that is the frozen-roster failure already on this estate's record. **hv declined both cheap answers**: direnv covers an interactive prompt and not automation, because git hooks do not reliably inherit it; and refreshing the frozen copy by hand is an advisory, and an advisory that requires remembering is not a control.

**STEP 0 NOW APPLIES TO TWO CHECKOUTS.** `Intentv2` was a fresh clone, inherited no `core.hooksPath`, and has been wired. Clone either again and `int hooks` is the first thing you run.

**THE WORD `intentdb` IS RETIRED AND NAMES NO COMPONENT** (hv, 2026-08-21). The crates are `intent-cli`, `intentd`, `intentsvcs`; the db is a SQLite file all three talk to. **The architecture, inviolable and unchanged for the whole rewrite** -- `intentd` and `intent-cli` are BOTH clients of `intentsvcs`, which solely owns `intent/.cache/intent.db`. `intentd` is not the SSOT and no read requires it. Diagram: `intent/st/ST0056/design.md:12-17`.

**AND THE GATE'S SCOPE: 62 of 67 is ST0057's CLOSURE gate -- all ST0057 live rows plus all ST0056 WP-03 rows. IT IS NOT THE 3.0.0 RELEASE GATE.** The release is ST0056 WP-12, whose dependency line reads _"All prior WPs"_, and **ST0056 stands at 59/132 with SEVEN WPs Not Started** (08 intentd XL, 09 MCP, 12 cutover, 13 search XL, 14 coordination, 15 skills, 16 contract drift). Read as release progress, 62/67 says 93% where ST0056 is at 45%.

## THE LOCAL CUTOVER -- v3 IS USABLE ACROSS THE ESTATE NOW (ST0058, 2026-08-22)

**hv's AIM, VERBATIM: _"Not necessarily releasable to the public, but useable by me across the wider estate here locally."_ That is a DIFFERENT BAR from the 3.0.0 release gate and it is met.** ST0056 WP-12 is the public release -- tag, brew, shell pruned -- and its dependency line reads _"All prior WPs"_ with seven Not Started. **Full detail, 685 lines with every measurement attributed: `intent/st/ST0058/design.md`.**

### What you type

    int local build                     # coherent binary pair, VERIFIED -- never bare `cargo build --release`
    int local status                    # which `intent` wins on PATH, before changing anything (read-only)
    cd <project> && intent3 upgrade     # the switch: explicit, per project, one at a time
    git checkout . && git clean -fd && rm -rf intent/.cache      # the way back

**THE ONE PRECONDITION IS THE WHOLE RISK: THE WAY BACK IS `git`, SO A PROJECT WITH UNCOMMITTED WORK HAS NO WAY BACK.** Commit or stash first.

**`intent3` IS A WRAPPER IN THIS REPO'S `bin/` AND MUST `exec`, NEVER BE COPIED** (dc, `99168a8f`). A bare copy has no `lib/templates/` marker above it, so `install::home()` fails, every hook refuses at **exit 1**, and **Claude Code blocks on 2 and NOT on 1 -- so the strict `/in-session` prompt gate would silently stop enforcing in every project at once.** ic found it; it was closed before packaging rather than after.

**`cargo build --release` DOES NOT RELIABLY PRODUCE A COHERENT PAIR and that is deliberate** -- `build-support/source_commit.rs` omits `rerun-if-changed` on purpose, because emitting any would REPLACE cargo's default of re-running on package change and make the embed stale on CODE changes, silently, in the worse direction. **Nobody "fixes" it. `int local build` forces both crates and verifies the set.**

### The migration floor -- 11 of 16 projects cannot convert directly

**THE FLOOR IS EXACTLY 2.19.0**, driven on copies of real projects: Anvil 2.13.0 and Riffle 2.18.0 both REFUSED, Baize 2.19.0 migrated. v3 refuses below it cleanly and names the remedy. **The two-step is VERIFIED rather than asserted: v2 `intent upgrade` reaches the floor in ONE hop, then `intent3 upgrade` migrates** (Riffle 5 threads/34 files, Anvil 6 threads/21 files, all statuses and dates intact).

    AT THE FLOOR (5)      Baize | Conflab | Lamplight | Laksa | Intentv2
    BELOW IT (11)         Cdsync Devbin Riffle Utilz 2.18.0 | Courses 2.14.0
                          Anvil MicroGPTEx Molt Prolix 2.13.0 | Molt-flynn Molt-matts 2.11.5

**THE CANARY ORDER INVERTS FROM THE OBVIOUS ONE.** Everything small and dormant is BELOW the floor, so the first switchable projects are the LARGE ACTIVE ones. **Baize first** -- smallest that can migrate directly, clean, exercises the session-hook path, no live session. Then Conflab, then Lamplight. **Laksa only after its dirty paths are committed.**

**`Intentv2` MUST NEVER BE MIGRATED.** It carries `intent/.config/config.json`, so **every census finds it and it looks like the ideal canary -- 3 threads, clean, at the floor.** It is the v2 CLI fifteen projects RUN. A census that finds projects by config presence cannot tell a consumer from the tool.

### Why it is safe to try -- driven in BOTH directions

**v3 refuses any project whose `config.json` does not declare a v3 version**, on read verbs and write verbs alike, rc=1, nothing written. Driven across every version string in the estate plus 2.9.0, 1.0.0 and absent -- **with `3.0.0-dev` as the POSITIVE CONTROL at rc=0 creating a store, without which eight refusals prove nothing.** Zero-thread projects refuse; v3 canon under a v2 declaration refuses.

**And the mirror: v2 INSIDE a v3 project permits READS and refuses WRITES**, naming both versions. So `intent` staying on PATH in a switched project is safe. **`intent claude skills sync` must be run with v2 and that is CORRECT** -- skills live in `~/.claude/`, not project state, so the project-version gate never consults it.

**The session machinery survives the switch:** `claude hook session-context` and `require-in-session` both rc=0 under BOTH binaries, tree unchanged. **6 of 17 projects fire those on every session** -- Baize, Conflab, Intent, Intentv2, Laksa, Lamplight.

### LIVE AND UNFIXED -- read before driving this repo

- **`intent edit` AND `intent st edit` WRITE ON THEIR rc=1 REFUSAL PATH.** `st edit` is declared `read_or_mutate: read` and its help promises a path print. They mutate the store and append to **TRACKED** `intent/.intentfiles`, putting a realisation-policy diff into your next commit that you never made. **THE PRECONDITION IS THE FINDING: unrealised CLEAN, realised-without-manifest CLEAN, manifest-present WRITES** -- and two nodes independently swept the two conditions that hide it and both published clean. **A VERB IS NOT READ-ONLY; IT IS READ-ONLY IN A CONDITION.** Affected population is exactly one project **and it is this one** (4 STEELTHREAD rows). A migrated project never acquires a manifest through ordinary use. cc's build, not a cover.
- **EVERY ROUTE TO `claude skills sync` SOURCES SKILLS FROM THE FROZEN Intentv2 CHECKOUT** -- including running the dispatcher that lives in THIS tree, because the exported `INTENT_HOME` beats self-resolution at `bin/intent:26`. Three hops: `INTENT_HOME` picks the dispatcher, the dispatcher's location fixes `INTENT_ROOT`, `INTENT_ROOT` fixes `SKILLS_SOURCE_DIR`. **NOT YET ARMED -- 0 skill commits since the split, 0 files differing -- and the first skill edit reverts silently while the sync reports success.** Route B works today (`env -u INTENT_HOME /Users/matts/Devel/prj/Intent/bin/intent claude skills sync`) and **is an advisory with an expiry: it runs the v2 shell dispatcher that WP-12 prunes.** The real fix is v3 implementing `claude skills sync`.
- **U3 IS A CONTRACT GAP AND IT IS NARROWER THAN IT LOOKS.** Five verbs are mandated in canon and unimplemented in v3 -- `claude skills`, `lang`, `plugin`, `ext`, `version` -- **all dispositioned `keep`, so all are UNBUILT rather than retired**, from `surface/dispatch-table.json`'s own `disposition` field. **`treeindex` is the only RETIREMENT of ours and the canon still mandates it in 3 files.** **DO NOT EDIT THE CANON: every one of those verbs WORKS in v2, so the mandates are correct for 16 of 17 projects. The canon is not defective, it is not VERSION-AWARE.** The treeindex edit belongs at the cutover, when it becomes false everywhere.

### Measuring anything here

**`int suite` RUNS THE SUITE IN `prepush`'s SINGLE-WRITER CLONE AND IS ATTRIBUTABLE BY CONSTRUCTION** (dc, `5173a220`). It prints `DESCRIBES=<sha>` and cannot be perturbed by any node editing the tree mid-run -- **demonstrated by accident: dc edited `cmd/suite` INSIDE its own run window and the figure was unaffected.** It measures HEAD, not the working tree, and without `--with-build` it does not cover `tests/conformance/run_v2_suite.bash`; the output says so every run.

**WHILE ANYTHING IS UNCOMMITTED, NO SUITE FIGURE CAN NAME A REVISION BY CONSTRUCTION** -- the clone cannot contain what HEAD does not. matts' own harness printed `THIS VERDICT DESCRIBES NO COMMIT` over uncommitted vc paths on 2026-08-22. **Commit first, then measure.**

## State (as at `69a5db5e`, 2026-08-20)

**THE GATE IS 62 OF 67, AND IT TAKES THREE VERB CALLS.** `intent ac status ST0057` (47/51) **plus `intent ac status ST0056/03` (15/16)** -- the scope is all of ST0057's live rows plus all of ST0056 WP-03's -- and `intent ac gate ST0057` names the outstanding ids. **`ac status ST0056` answers 59/132 and is NOT this number's denominator.** The `ST0056/03` call is a WP-scoped STID; the verb accepts it and no instruction here ever said so. **Three figures have been wrong now, and the third was wrong because this line said do not hand-tally while naming calls that could not reach the figure** -- so obeying it meant copying the banner. **Run all three.**

**ST0056** (v3.0.0 rewrite) -- 133 criteria / 137 tests, **59 of 132 satisfied**, 1 withdrawn. **The SQLite db is the DURABLE SSOT; nothing on disk is truth.** **There is no `intentdb` -- that word was a TYPO that propagated corpus-wide, and it names no component.** The crates are `intent-cli`, `intentd` and `intentsvcs`; **`intentd` is a CLIENT of the db exactly as the CLI is, and exists only for wider features beyond single-project operations. It is not the SSOT and no read requires it** (hv, 2026-08-21). D01 was REVERSED by hv 2026-08-15; **do not reason from it.**

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
