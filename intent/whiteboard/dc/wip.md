---
node: dc
name: DevX Claude
role: worker
session_id: 80fa1787-174a-49f0-8ef1-c2c7b48d3fb8
heartbeat_at: 2026-08-21 12:56Z
status: paused
focus: "**FOLDED AND HOLDING on hv's instruction (relayed by vc, live channel ~12:40Z). NOTHING IN FLIGHT; no code written today and no row moved.** **THE SESSION'S ONE REAL FINDING IS A CONTRACT CONFLICT AND IT IS IN `.history/20260821/wip.md` SECTION 1: AC-11.6's green arm contradicts cc's 2026-08-20 authorship ruling, which is WHY the red-first arm was never reachable -- the discriminator it asks for cannot exist, because Protocol 3.0 invariant 3 forbids path claims outright AND cc showed a shared-tree build carries the union whoever invokes it.** Do not build AT-11.6 to the row as written; route the conflict to vc. **FOUR ASSIGNMENTS ARRIVED TODAY AND ALL FOUR ARE MINE** -- ST0057 **AC-01.5's REMEDY** (cc, 12:59Z), the INTENT_HOME guard-resolution mechanism, prune-at-fold for `target/<node>`, and the one-word `precommit:141` edit. **CORRECTION TO WHAT I REPORTED AT PICKUP: dc does NOT hold none of the gate.** AC-01.5 is one of the five outstanding rows and its remedy is ruled dc's; I read the owner off `restart.md`, which names cc, and the remedy was never on anyone's board. **The three previously-held items are UNCHANGED and still held on hv's word.** Gate: **62 of 67**, and it is ST0057's CLOSURE gate, NOT the 3.0.0 release -- ST0056 is 59/132 with seven WPs Not Started."
claims: [ST0056/07, ST0056/11]
---

# DevX Claude (dc)

**Today's reasoning, dead ends and perishable measurements are verbatim in `.history/20260821/wip.md`. Earlier days under their own dates. This file is the COLD-SESSION MINIMUM: the rules that govern the next write, and what to start on.**

## D42 -- TIME. Read this before writing anything, anywhere.

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.** The stamp is applied BY the write.
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES. NO cli or intentsvcs function TAKES a time; they may RETURN times.** A time-typed input parameter is a defect by inspection. Not exceptions: fixtures, "only reading it", "but it came from the database".
- **A board stamp is a label, not data** -- read it from `date -u` and PASTE, **per stamp, never per session.** The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The SQLite db is the durable SSOT, files are re-creatable; the typed API is the only door in; migrations are normal.**

**`intentdb` IS RETIRED AND NAMES NO COMPONENT (hv, 2026-08-21).** The crates are `intent-cli`, `intentd` and `intentsvcs`; **`intentsvcs` solely owns the db, and `intentd` is a CLIENT of it exactly as the CLI is.** The word implied a daemon-owned store. Diagram: `design.md:12-17`, unchanged for the whole rewrite. **The SUBSTANCE of D01 is untouched -- only the term was wrong.**

## The environment moved today, and the next session wakes up inside it

- **THE v2 CLI HAS LEFT THIS CHECKOUT.** `~/Devel/prj/Intentv2`, branch `v2-maintenance`, cut at `fb45e9ea` = main HEAD, **not** the `v2.19.0` tag.
- **`intent` ON PATH IS v2.19.0 AND ANSWERS FOR THE FLEET, NOT FOR THIS TREE.** Drive v3 explicitly: `./native/rust/target/debug/intent`. **v3 still does not go on PATH.**
- **`bin/` IS NO LONGER LOAD-BEARING FOR ANY OTHER PROJECT**, so v2 shell can be pruned here without breaking fifteen of them.
- **The three bindings, and the symlink is the WEAKEST of them:** `bin/intent:26` reads `if [ -z "$INTENT_HOME" ]`, so **the exported var beats symlink resolution outright** -- repointing the symlink alone changes nothing and looks exactly like success.

## DOING

**Nothing in flight. No code written today. The only files I touched are my own board, my own archive and my own inbox.**

## TODO

### 1. ST0057 AC-01.5's REMEDY IS MINE, IT IS ON THE 62-OF-67 GATE, AND IT WAS ON NOBODY'S BOARD (cc, 12:59Z)

**I TOLD matts AT PICKUP THAT dc HOLDS NONE OF THE GATE. THAT IS WRONG AND THIS IS THE CORRECTION.** I took the owner from `restart.md`, which assigns AC-01.5 to cc -- true of the ROW, false of the REMEDY. **cc drove it: `grep -cE 'canon-ignore|pre-commit\.intent|AC-01\.5' dc/wip.md` -> 0.** No edit of cc's can reach this row.

**The ruling** (vc, in `AT-01.5`'s note, 2026-08-20): _THE REMEDY IS dc's, IT IS SMALL, AND IT IS NOT THE DOC FIX cc OFFERED AS POSSIBLE._ **Two forms named: either the chain FAILS LOUD on an absent dispatcher, or `int hooks --install` stops reporting a wired clone when the dispatcher it chains to is not there.** vc's warrant is my own sentence turned back on me -- _a control which depends on the author remembering is not a control, it is a hope with a filename_.

**THE SHAPE IS ALREADY BUILT ONE LEVEL DOWN.** `pre-commit-guards.sh` discriminates three absences -- _the resolver did not answer_, _one guard file is missing_, _the install is stale_. **That same discrimination is owed one level UP, in the chain, where `[ -x ]` with no `else` is exactly the collapse.**

**The row survives on ARM C alone: a fresh clone wired by `int hooks --install` printed `hooks: this clone is wired`, then committed a planted ignore rule at rc=0 with ZERO guards running.** Arms D and E already prove the guard is correct and dispatched in THIS install. **The guard is not the hole; the clone is.**

**DRIVE ARM C AGAIN BEFORE BUILDING AGAINST IT (cc's flag, and it is a good one).** Arm C was measured BEFORE today's `INTENT_HOME` split, and the guards now resolve out of the frozen v2 checkout -- **which is the mechanism assignment below.** Whether a fresh clone still behaves that way is an OPEN QUESTION, not an assumption. **The two assignments are the same subject from two ends; sequence them together.**

### 2. THREE ASSIGNMENTS hv ROUTED TO ME TODAY -- all new, all mine, detail live in `inbox.vc.md`

- **THE GUARD-RESOLUTION MECHANISM (12:35Z) -- the real one, and hv picked me over two cheaper answers on purpose.** With `INTENT_HOME` now pointing at `Intentv2`, **this repo's own commit guards resolve out of the frozen v2 checkout**: `.githooks/pre-commit` -> `pre-commit.intent` -> `intent info` -> `$INTENT_HOME/lib/templates/hooks/`. Byte-identical today; **drifting from the next guard change.** This is the frozen-roster failure already on our record -- `cmd/precommit:94-99` describes it in our own words. **hv DECLINED direnv (git hooks do not reliably inherit it, so it is green where you look and absent where it matters) and DECLINED hand-refresh (an advisory that requires remembering is not a control).** Two constraints: **a wrong answer must be LOUD**, and **`bin/` is the one genuine cc/dc collision -- get hv's word before landing there.**
- **PRUNE-AT-FOLD (11:40Z).** **Every node prunes its own `target/<node>` at fold**; shared `target/debug` survives. Disposal joins creation in the same ritual. **Measured at `706db8ee`: `native/rust/target` is 66G across 1,336,417 files** against 1,481 tracked -- `debug` 33G, `cc` 18G, `ic` 15G, `release` 927M. **CORRECT COMPLIANCE PRODUCED THE DUPLICATION** -- our own rule says isolate the target dir inside the checkout, and says nothing about removing it. Two hazards: **a gitignored artefact is invisible to every instrument we own**, and **never delete a live node's dir** (board headers carry the live session ids; an orphan's owner appears on no live board). **There is no `target/dc` -- my own compliance is vacuous today.** `target/tmp/` and `target/dist/` match no moniker, so **the per-node rule does not reach them and nothing else does either.**
- **`bin/.devbin/cmd/precommit:141` -- one word.** `the intentdb is per-machine durable truth` -> `the SQLite db is per-machine durable truth`. **The claim is exactly right and only the noun is wrong.** vc deliberately did not do it: `bin/` is open for hv, not assumed by either node.

### 3. The three HELD items -- ruled, evidenced, ready, and still matts' call. Unchanged.

- **`tests/lib/test_helper.bash:93`** reads `${INTENT_FIXTURE_VERSION:-3.0.0}`; make it default from `VERSION`, the same source `tests/run_tests.sh:61` has read since `e474b419`. **The runner defends itself; the helper does not**, so a direct single-file `bats` run -- the invocation our own guidance prefers -- silently builds a v3 fixture and drives a v2 binary at it. **37 files call `create_test_project`, all v2-driving; the 5 touching the v3 binary never call it, so the default is wrong in every case where it fires.**
- **ADMIT `canon_commit_check.sh` TO `bin/.devbin/cmd/precommit` (ST0057 AC-03.6).** Exists, 464 lines, **carries the literal `AC-03.6` 3x -- ready, not a wrong citation.** **TWO DELIVERABLES, NEVER BUNDLED: the remedy edit makes the instruction TRUE; only `--staged` makes it UNNECESSARY.** Dispatch **unconditional** -- a path trigger would be a second copy of the tool's own narrowing.
- **ADMIT `thread_view_skew_check.sh`** (149 lines; cc built it, dc rosters), **CONDITIONAL on a staleness refusal, and the condition was LIVE at `706db8ee`** -- release binary 2h56m older than `intentsvcs/src/{migrate,facade}.rs` (measurement in the archive; **destroyed by the next release build**). **Build `lib_binstale.sh` as an EXTRACTION of `surface_check.sh`'s refusal, never a copy** -- that reach list has already been wrong once. **It is ungated, and building it is what converts this held item from _conditional_ to _awaiting only hv's word_ without taking his decision.**

### 4. AT-11.6 is BLOCKED ON A CONTRACT CONFLICT -- do not build it to the row as written

**`shared_artefact_build_guard.sh` is ABSENT (driven).** Full argument in `.history/20260821/wip.md` section 1. In one line: **AC-11.6's green arm mandates the very arm that produces an unattributable union binary**, so the criterion's green arm reintroduces the defect it exists to prevent -- and that is why the red-first arm was never reachable. **Route to vc as contract steward; hv if vc bounces it. The amendment would conveniently unblock my own row, which is the reason to route it, not the reason to make it.**

**AND THE ESCALATION NOW CARRIES A LIVE, UNPLANTED INSTANCE, surfaced by the gate on my own fold commit `5f8d5b7d` and confirmed independently (archive section 7): `target/release/intent` and `intentd` carry DIFFERENT `dirty-` markers -- two binaries in one shared path from two different trees, invoked as a matched pair.** It is a sharper statement of the criterion than the criterion's own founding episode, and **it refutes the ownership discriminator outright: neither marker names an owner because there is none to name.** **Perishable -- the next release build destroys it; capture the sha256 pair, never the markers.**

**AT-11.7 `provenance_fields_check.sh` is ABSENT and is buildable NOW** -- its spec is sound and its positive controls are on record (the currency failure `f2e4d1f9005d0334`, and the `intentd` fossil marker). Reuses `self_provenance_check.sh`'s artefact reading; **not a Highlander violation -- vc has ruled that two mechanisms enforcing different properties are not two copies of one.**

### 5. Recorded, not built

- **The WP-07 hosting sweep needs a DRIVEN re-measure and the disposition route is a DEAD END** -- see archive section 4. `disposition` is about v2->v3 porting, not implemented-vs-stub, and the denominators (27 table families / 32 carried / 34 declared) do not reconcile. **Drive it through `render.rs:495`'s unbuilt-verb reporter, not off the table.**
- **`intent claude upgrade --apply` IS ALL-OR-NOTHING** -- three of four actions unwanted here, including regenerating AGENTS.md from 3.0.0 DOWN to 2.19.0 on a self-hosted v3 tree.
- **`author`/`content` PRINT NOTHING WHERE v2 PRINTS 136 BYTES** -- a clean no-op returns before any output, so a prose language is indistinguishable from one that ran and found nothing.
- **CLI-level tests for `critic`** (the module has 18, the command surface none); **`critic --no-such-flag` EXITS 1 WHERE v2 EXITS 2**, so the gate blocks on a typo -- mine and ic's together.
- **D37 payload sweep / `AT-00.17`**; `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

**Today's instances are verbatim in `.history/20260821/wip.md`; earlier days under their own dates. These are the CLASSES that will bite the next write.**

- **A GREEN IS ONLY EVER ABOUT THE QUESTION THE INSTRUMENT ASKS.** The family, all measured: a cross-check that reconciles because **both sides share an error**; a **true measurement of a different property** offered as proof; a count that mixes _not built_, _built and unverified_ and _verified and unmoved_ when only the first is work; **a zero from an instrument never shown able to produce a non-zero**.
- **AN EXISTENCE TEST COMES BEFORE AN ID TEST, AND MY OWN SPLIT OMITTED IT.** _0 hits means the citation is wrong_ **holds only for a file that EXISTS**; for an absent file 0 hits is trivially true and means **UNBUILT** -- the opposite state, one being work owed and the other a defect in the row. Both rows I touched today were absent.
- **A NAME SEARCH RETURNS A FACT ABOUT THE SEARCH, IN BOTH DIRECTIONS.** The pattern can EXCLUDE the answer (`grep 'organize::plan('` found 3 sites, the compiler found 7) or INCLUDE a non-answer (a MENTION inside prose read as a SUBJECT). **The over-count is worse: it reads as diligence.**
- **AN INSTRUMENT OR CITATION WHOSE PREMISE EXPIRED IS A LIVE DEFECT, NOT STALE CONTEXT.** **No instrument we own catches it -- only a builder trying to satisfy the row does**, and `at lint` exempting `to-write` is CORRECT, which makes this structural rather than an oversight.
- **A GITIGNORED ARTEFACT IS INVISIBLE TO EVERY INSTRUMENT WE USE** (vc, 2026-08-21, on the 66G). Whatever gets built, **its output must surface where a human actually looks**, or it joins the built-correct-rostered-and-dispatched-by-nothing class.
- **SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING**, and a skipped gate arm from a clean one. **A write surface with no named reader is the same shape.**
- **SYNTHESISE THE INSTANCE _AND_ KNOW THE LIVE POPULATION.** A red-first arm needs a SYNTHETIC instance; a green over synthetics alone cannot say whether the feature has ever RUN. **A rig tests the states its author enumerated** -- ten arms covered absent-binary and not stale-binary.
- **DELETE THE BINDING, DO NOT SHIM IT.** The compiler is a population oracle and a shim blinds it. **A fix that changes a type leaves every use unverified, and the ones that still compile are exactly the ones nothing reports.**
- **POLARITY BELONGS IN THE ASSERTION'S SHAPE, NOT IN WHOEVER READS THE OUTPUT** -- `assert_eq!(after, before)` prints `after` as `left`. **And a POSITIVE assertion cannot notice a retired string coming back; pair it with a `refute`.**
- **CITE THE IMPLEMENTATION, NEVER THE INVOCATION** (a symlinked dispatcher does not contain the command's name). **MARK PROVENANCE PER CLAIM -- driven, read or inferred.** **VERIFY THE RETRACTION, NOT JUST THE CLAIM.** **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION**, and a timing figure its LOAD.
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it; `grep -c` exits 1 on zero so a `||` fallback fires on a true zero; **`die` calls `exit` and a redirection does not contain one** -- use a subshell. **MEASURE IN THE SHELL THAT WILL RUN THE CODE**: mine is zsh, hooks run bash. **`local a=$1 b=$a` does not see `a`.**
- **STANDING CONSTRAINTS.** **`git commit --only <paths>` is PATH-scoped, not HUNK-scoped** -- it defends against a peer's STAGED index and does nothing about their UNSTAGED edits; **only a detached worktree catches that class mechanically**. Push `local` only; **confirm with hv before any `upstream` push**. **DO NOT PUT v3 ON PATH.** NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. **The markdown formatter is a second writer.** **Run the suite through `tests/run_tests.sh`, never `bats` directly.**
- **FOUR SHELL CRITIC FINDINGS ARE DELIBERATELY NOT FIXED AND MUST NOT BE.** `bin/intent_st:1187`/`:1208` (`$LIST_ARGS`) and `bin/intent_treeindex:220` (`$prune_expr`) are **intentional word-splitting**; `bin/intent_st:1353` is a fragment of a multi-line `sed` script the line-based proxy cannot parse. **A sweep driven to zero without reading each site breaks three live paths.**

## Decisions

**Today's full set is in the fold archive. These are the ones that govern the next build.**

- (2026-08-21) **THE GATE'S SCOPE IS ST0057's CLOSURE, NOT THE 3.0.0 RELEASE.** 62 of 67 = `ac status ST0057` (47/51) + **`ac status ST0056/03` (15/16)** -- a **WP-scoped STID** the verb accepts and no instruction in this estate ever mentioned. **The release is ST0056 WP-12, and ST0056 is 59/132 with seven WPs Not Started**, so 62/67 read as release progress says 93% where the thread is at 45%. **`ac gate` NAMES the unsatisfied rows; `ac status` gives only N/M.**
- (2026-08-21) **THE GUARD AGAINST HAND-TALLYING WAS THE VECTOR FOR IT** (vc, on their own fold). All three copies said _never re-derive this by hand, run the verb_ and then named two verbs that **cannot reach the number they vouched for**. **A reader obeying the instruction literally was left with copying the banner as the only way to comply.** An instruction that names an insufficient procedure is worse than one that names none.
- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES -- two doors on one model, and the filter they share is EXTRACTED.** **Absence is decided at the filesystem, once, by the caller that touches it** -- inferring it from an empty string collapses it with a manifest declaring NONE, an opposite state.
- (2026-08-20) **A WORKSPACE-WIDE CHECK BELONGS WHERE THERE IS ONE WRITER** (the prepush clone, not the shared tree), **and a fault that can only escape on a push belongs to a PUSH gate.**
- (2026-08-20) **THE ROSTER IS THE RUNNER'S TO ANSWER, AND IT ANSWERS BEFORE IT DISPATCHES.** **The copied file names no guard and holds no roster**; roster lives in `pre-commit-guards.sh`, read live from `INTENT_HOME`, so a new guard reaches every consumer with no reinstall. **A template is embedded because `init` writes it; a guard is read live because the gate dispatches it.**
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc, retracting their own Highlander argument). Prevention and refusal are different criteria; declining the second because the first exists is a gap, not Highlander.
- (2026-08-20) **AN INSTRUMENT THAT REPRODUCES THE DEFECT IT WAS BUILT TO CATCH IS WORSE THAN ONE THAT UNDER-REPORTS AND SAYS SO.** The conservative instrument wins and its false negatives get named in the file. **A refusal whose reason expired is a live defect, not an owed capability.**
- (2026-08-20) **`CARGO_TARGET_DIR` FIXES FREQUENCY, NOT AUTHORSHIP** (cc, correcting my reshape). A legitimate publish build **carries the union identically**; the target-dir split makes union binaries RARER but cannot make any one of them attributable. **Only requiring a CLEAN TREE reaches authorship** -- and the prize is that `dirty-<sha>` becomes `<sha>`. **This is what AT-11.6 collides with; see TODO 3.**
