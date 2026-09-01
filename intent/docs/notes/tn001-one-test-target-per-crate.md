---
verblock: "01 Sep 2026:v0.9: dc - The regenerating commands land against 4afef84; what hv's cycle did and did not verify"
---

# TN001 -- One test target per crate: the ruling Intent made and did not apply

**Status: RULED (hv, 2026-08-27, estate-wide). APPLIED IN INTENT AT `71a96213` (dc, 2026-09-01), AND THE FULL CYCLE HAS BEEN RUN AND WATCHED BY hv.** Every figure below carries the command that regenerates it, because a technote is read at boot and a figure in one goes stale silently.

_This line read "APPLICATION IN FLIGHT" through v0.4 and was true when written. It is kept as a correction rather than overwritten silently, because a status is a live claim and the note's whole subject is claims that expire._

Written by vc at hv's direction, on dc's census and dc's build. Circulated to every Intent-using project so each can make its own version rather than inherit ours.

## The defect in one line

**Under cargo's default `autotests = true`, every `.rs` file in a crate's `tests/` directory becomes its own test target -- a separate compile and a separate FULL LINK of the crate and all its dependencies.** A crate with 166 test files links 166 binaries to run its tests. Nothing warns you; it is the default, it is invisible in the output, and it grows one file at a time.

## Why this is a technote and not a ticket

**Intent ruled this estate-wide on 2026-08-27 and then did not do it.**

| estate     | what happened, and when                                                                                         |
| ---------- | --------------------------------------------------------------------------------------------------------------- |
| Laksa      | adopted **2026-08-27**, the day of the ruling; its `Cargo.toml` quoted the ruling verbatim                      |
| Lamplight  | consolidated **2026-08-27**, its one crate that has tests, zero orphans                                         |
| **Intent** | ruled **2026-08-27**, applied **2026-09-01** at `71a96213` -- **five days, in the estate that made the ruling** |

**THIS TABLE HELD A STATUS COLUMN UNTIL v0.6 AND THE COLUMN IS THE ONE PLACE THIS NOTE BROKE ITS OWN OPENING RULE** (raised by devbin-vc, adjudicated by vc, blessed by hv). Every figure here carries the command that regenerates it -- except a per-estate status, which carries none and cannot. **A DOCUMENT HAS A REVISION AND A MESSAGE DOES NOT, SO THE MESSAGE IS THE COPY THAT ROTS -- AND A PER-ESTATE STATUS TABLE IS A MESSAGE WEARING A DOCUMENT'S CLOTHES.** It looks checkable, and what it asserts is true only at a moment nothing records.

**IT HAD ALREADY ROTTED TWICE, AND THE SECOND ONE WAS STILL SITTING HERE WHEN THIS EDIT WAS MADE.** The Lamplight row read `1 of 10` through v0.1 and v0.2 -- wrong by a factor of ten, in the direction that punishes compliance, corrected while hv was ruling this note fleet canon. And the Intent row read `NO -- not one crate` from v0.1 to v0.5, which stopped being true at `71a96213` and **stayed on the page for the rest of that day**, contradicted by this note's own status line thirteen lines above it.

**THE TEST THAT REPLACED THE COLUMN IS TENSE, NOT IMPORTANCE** (vc's amendment, and it is the half a reader can apply): a PRESENT-TENSE claim about another estate's STATE rots the moment that estate edits anything, silently; a DATED claim about an EVENT survives whatever happens next. `Laksa's Cargo.toml quotes the ruling verbatim` was the first kind. `Laksa adopted it on 2026-08-27, quoting the ruling verbatim` is the second, and nothing Laksa does can falsify it.

**THE REGENERATING COMMANDS, WITH THEIR DISPOSITIONS ATTACHED.** Two instruments, and which one ADJUDICATES is not a matter of taste -- item 1 below already ruled it, and a note that grew a second rule contradicting its first would be the exact defect it is about.

```
bin/devbin check autotests                                      # DIAGNOSES, per-crate, runs anywhere
cargo test --workspace --no-run 2>&1 | grep -c 'Executable tests/'   # ADJUDICATES
```

**THE GATE STATES ITS OWN POPULATION ON EVERY PATH INCLUDING PASSES** -- `N crate(s) with tests/, M in breach (pruned: ...)` -- which is what makes it citable at all rather than merely available. Its own comment gives the reason: an empty walker is silent and an empty parser is loud, and only one of those is safe. **A gate that reported a verdict without its population would be the instrument this note warns about, wearing a uniform.**

**ITS CRITERION IS `is discovery controlled`, NOT ARITHMETIC, AND THIS ESTATE IS WHY** (devbin-vc, `4afef84` in devbin's repo -- `git show 4afef84:lib/cmd/check`). An earlier version made more than one declared target a breach. **`intent-cli` and `intentd` each declare two DELIBERATELY, for the reasons this note gives below -- so the literal rule reddened the estate that authored the ruling.** Declared blocks now pass at any count with the count reported. The one refusal left is the genuinely silent case: **discovery off, nothing declared, `.rs` files sitting in `tests/` -- compiled by nobody and run in no suite.**

**THE LAMPLIGHT ROW READ "1 crate of 10" IN v0.1 AND v0.2 OF THIS NOTE AND IT WAS WRONG** (corrected by lamplight-vc, measured with this note's own instrument). Lamplight has three crates and a workspace root; **exactly one has any test files, and it was consolidated on 2026-08-27 with zero orphans.** That is 100% of the crates that have tests, reported as 10%. The correction arrived while hv was ruling at Lamplight that this note becomes fleet canon -- **so the wrong figure was one step from propagating into every estate.**

Intent is the estate that made the ruling. **201 of its test files landed in the same month the ruling was made**, so the cost was accruing fastest exactly where the decision had been taken. A ruling that is not applied where it was authored is not a slow rollout; it is a ruling nobody is enforcing, and the author is the last to notice because they remember deciding it.

**That is the transferable half of this note. The Cargo settings below are mechanical. The failure was that a decision and its application were separately tracked, and only one of them was.**

## The mechanism

Cargo's `autotests` defaults to `true`. Each file directly under `tests/` becomes a `[[test]]` target. A target is not a cheap thing: it is a full link of the crate under test plus its whole dependency graph, and linking dominates the cost -- so the total is roughly linear in FILE COUNT, not in test count or in code volume.

Adding a test file is therefore not a marginal cost. It adds a link.

## The fix, in four parts

Three are per-crate; the fourth is per-repo.

**1. Turn off automatic discovery and declare one target.** In each crate's `Cargo.toml`:

```toml
autotests = false

[[test]]
name = "suite"
path = "tests/suite.rs"
```

**2. Make the former targets modules of the one target.** `tests/suite.rs` becomes a list of `#[path]` module declarations, one per file that used to be its own target. The test bodies do not move and do not change.

**3. Guard against orphans, because step 2 is hand-maintained and will drift.** A file added to `tests/` after the switch is compiled by nobody and runs in no suite -- **it passes by not existing.**

**THE FAILURE MODE INVERTS AND STAYS SILENT, WHICH IS THE WHOLE ARGUMENT** (laksa-vc). Before the change, a stray file silently became an extra binary. After it, a stray file silently becomes nothing. **Neither announces itself, so consolidation without a guard trades a LOUD WASTE for a QUIET HOLE.** That is why the guard is not optional and not a follow-up.

**Four subtleties, all from Laksa's implementation, and a guard built from a summary would miss three of them:**

- **KEY ON `#[path = "..."]`, NEVER THE `mod` NAME.** The two are free to diverge and only the path decides what compiles. A mod-name detector agrees with a path detector on every line of a healthy file and diverges the first time somebody renames one without the other -- **which is exactly when you needed it.**
- **PLANT A REAL FILE; A SYNTHETIC CONTROL IS NOT ENOUGH.** A synthetic control proves the SUBTRACTION works. It cannot prove the DIRECTORY READER reaches the right place -- and a walker that returns an empty list on a `read_dir` error finds no files, therefore no orphans, and goes **GREEN** when aimed at a moved or renamed directory. Laksa planted a real `zz_orphan_probe.rs`, got a red naming it, removed it, got green, in one atomic command with the tree verified clean after. **Both instruments must also assert their own corpus is non-empty, because an empty walker is silent while an empty parser is loud, and only one of those is safe.**
- **ONLY THE UNDECLARED DIRECTION NEEDS GUARDING.** A path DECLARED with no file behind it is a `#[path]` compile error -- loud, immediate, and needing nothing. Stated so nobody adds the other half later thinking it was missed.
- **THE GUARD CANNOT CATCH ITS OWN OMISSION, AND NO TEST CLOSES THIS.** Dropped from `suite.rs` it stops being compiled and stops reporting -- its own defect applied to itself. **Its declaration line is load-bearing and must not be tidied away.** Say this in your own note; do not let it be discovered five estates later.

**AND THE TRAP IN PROVING THE GUARD, WHICH CAUGHT dc ON THE FIRST ATTEMPT AND GENERALISES WELL PAST THIS JOB: `cargo test <filter>` EXITS 0 WHEN THE FILTER MATCHES NOTHING.** dc's first proof filtered `--exact` on a bare test name; the test lives in a module, so it matched nothing, ran zero tests, and exited 0 -- **reporting the guard as decoration.** The baseline arm added as a control passed the same vacuous way, because **exit 0 over 28 filtered-out tests is byte-identical to exit 0 over one pass.**

**SO EVERY ARM MUST ASSERT THE COUNT, NOT THE EXIT CODE.** The general form: **any harness that selects by name and checks only `$?` is green from the moment the name drifts** -- and name drift is silent, routine, and exactly what refactoring does.

**4. Cut debuginfo, once, at the workspace root.**

```toml
[profile.dev]
debug = "line-tables-only"
```

`cargo test` inherits `dev`, so one key here reaches every test target rather than needing a `[profile.test]` beside it. `line-tables-only` keeps the file and line in a backtrace -- what a person actually reads off a failure -- and drops the type and variable detail that only a debugger consumes.

**And separately: pass `--no-fail-fast` at every `cargo test` call site.** With one target, the first failure otherwise stops the whole suite.

**THIS IS NOT A TIDINESS FLAG AND LAMPLIGHT HAS THE SHARP VERSION: CONSOLIDATION MAKES A BARE `cargo test` STRICTLY WORSE THAN IT WAS.** Their `ci.yml:241` ran bare before and still does -- and on 2026-08-27, **17 independently-failing targets became one, so the first failure now hides the other sixteen.** Before consolidation a bare run reported every failing target; after it, one. **You do not lose the flag's benefit by omitting it; you lose CI information you previously had.** Add it in the same commit as `autotests = false`, not after.

## Do not move the files, and on today's Intent that is a hard constraint

**Use `#[path]` so that NO FILE MOVES.** Laksa did this deliberately and it is the reason thirteen green acceptance-test rows citing those files by path never went stale.

**AND ON THE PUBLISHED RELEASE THERE IS NO WAY BACK FROM MOVING THEM.** Driven, both artefacts, 2026-09-01:

| binary                 | `intent at edit --file`              |
| ---------------------- | ------------------------------------ |
| published `v3.0.0` keg | **`unrecognized subcommand 'edit'`** |
| unreleased dev HEAD    | present                              |

**So on every estate running the published release today, a row's cited file cannot be retargeted by any verb.** A consolidation that moved files would put contract prose and green statuses at risk in order to fix a path -- **real damage, to avoid a cosmetic problem.** The constraint dissolves at the next release, and nobody should plan against that until it is cut.

**AND THE PRICE OF THE OTHER CHOICE IS MEASURED, NOT ARGUED** (lamplight-vc, who paid it). Lamplight consolidated by MOVING all 17 files from `tests/*.rs` into `tests/main/*.rs`. **That staled 31 AT citations across 5 threads -- ST0290 25, ST0286 3, ST0264 / ST0315 / ST0351 one each -- undetected for five days.** It compounds with Intent issue `0015`, under which **a GREEN AT whose citation does not resolve still holds a gate up**.

**IT ALSO MANUFACTURED A FALSE FINDING, WHICH IS THE MORE USEFUL HALF.** On 2026-08-27 Lamplight filed that `ST0264`'s `AT-16.4` cited a file _"GENUINELY absent -- a real finding"_. It was not absent; it had moved at 12:22:47Z that morning, and the symptom was filed at 18:44Z without anyone looking for a cause. **THE REMEDY WAS WHAT WAS AT RISK: _absent_ points at voiding a green AT whose test exists and passes; _moved_ points at a path update.** Retracted by its author.

**So "no file moves" is worth roughly 31 stale citations and one wrong finding per crate of that size.** `#[path]` costs nothing and sidesteps the whole question. Take it even after `at edit` ships.

## The trap in measuring it, which cost us a wrong scope

**Test FILES are not cargo TARGETS. They are equal only where `autotests` is default.**

A census that counts files and reports targets will be right for every unconsolidated crate and wrong for every consolidated one -- so it systematically overstates the remaining work, and it overstates it MOST for the estates that already did what you asked. Laksa's 16 test files compile to ONE target; a file count reports Laksa as the second-worst offender in the fleet when it is the only estate that is finished.

**On this fleet, a six-tree scope was taken off a file-count column and the correct scope was three.** Two of the six needed nothing -- one already compliant, one with no test files at all -- and a third was a deliberately frozen fallback checkout whose Rust is a pre-cut snapshot. The caveat had been stated at the top of the census and was not applied to the reading below it.

**AND THERE IS A SECOND HEAD OF THE SAME CAVEAT, WHICH PRODUCED THE WRONG LAMPLIGHT FIGURE ABOVE** (lamplight-vc). **On a fleet using worktree isolation, a `find`-based crate census counts every crate once per checkout.** Lamplight holds 14 `Cargo.toml`: four are theirs, **eight are those same four seen again in `.worktrees/cc` and `.worktrees/ic`** -- git worktrees of the same repository -- and two are gitignored dependency NIFs. A census that walks the tree sees fourteen crates where there are four.

**BOTH HEADS PUSH THE SAME DIRECTION, AND IT IS THE DIRECTION THAT PUNISHES COMPLIANCE.** Files-not-targets overstates the work for whoever consolidated; worktrees-as-crates overstates it for whoever isolates. **The estates doing the right thing measure as the worst offenders**, and neither error announces itself.

**So: state the caveat AND apply it, or do not state it. A caveat that sits above a table nobody re-reads is decoration.**

## What each project should do

1. **Count your real targets by asking cargo, not by grepping for them.** `cargo test --workspace --no-run 2>&1 | grep -c 'Executable tests/'` is the only count that is not a proxy. The two greps -- `grep -c '^\[\[test\]\]' */Cargo.toml` and `grep -rn 'autotests' --include=Cargo.toml .` -- still DIAGNOSE, and they must not ADJUDICATE: read together with "absent `autotests` means `true`" they do reach the answer, but only by an inference the reader has to make correctly. Intent's own `testkit` shows the cost of getting it wrong -- `grep -c '^\[\[test\]\]'` returns **0** for it while cargo reports **1**, because a single test file with no `autotests` key is auto-discovered. **An estate reading the grep alone sees a crate reporting zero targets, concludes it needs consolidating, and consolidates something already correct.** In a note about instruments that answer confidently about what they never looked at, the measuring instruction must be the direct one.
2. **Apply the four parts above** to every crate whose test files outnumber its declared targets.
3. **Build the orphan guard before you consolidate, not after.** It is the only thing standing between a consolidated suite and a silently shrinking one.
4. **Write your own version of this note.** Not a copy -- your crate layout, your counts, your call sites. A note that describes someone else's estate is read once and never checked.

   **FILE IT WHERE YOUR TECHNOTES ALREADY LIVE, AND NUMBER IT NEXT IN YOUR OWN SEQUENCE. THE PATH AND THE NUMBER ARE YOURS, NOT INTENT'S TO CARRY ACROSS.** This paragraph replaces an instruction that said "if `intent/docs/notes/` does not exist yet, this will be your first" -- **which was wrong and would have done real damage** (found by conflab-vc, after it had already been circulated to five estates). Conflab's technotes live at `docs/notes/`, not `intent/docs/notes/`, and its `tn001` is taken by an unrelated note; followed literally, the instruction creates a SECOND notes tree containing a SECOND document numbered TN001 about a different subject. **Conflab's is `tn005`.** **And the failure is silent, because creating a directory always succeeds** -- nothing would have reported the duplicate.

   The instruction generalised Intent's own layout from a sample of two estates that happened to agree. **Before filing, look for the notes tree you already have.**

5. **If you own no Rust, check the PATH before you change the declaration.** Two estates declare `rust` in `intent/.config/config.json` while owning none -- Baize, whose only `Cargo.toml` files are dependency NIFs, and Gtools, which has **zero** `Cargo.toml` anywhere.

   **THIS ITEM ORIGINALLY SAID A DECLARED-BUT-UNOWNED LANGUAGE "ARMS A CRITIC OVER CODE YOU DID NOT WRITE". THAT IS AN ASSERTED CONSEQUENCE AND IT WAS NEVER TRACED** (corrected by baize-vc, who measured the path instead of accepting the claim). In Baize it does not hold, and structurally rather than by luck: `.gitignore` carries `/deps/`, so every Rust file there is ignored and can never be staged; and `check critic` defaults to `--staged`, reporting NOT APPLICABLE when it scans nothing. **The only Rust that exists cannot enter the only corpus the gate reads, so the declaration is INERT, not armed.** Gtools is inert for a simpler reason still -- no crates at all. **Two estates, one asserted hazard, zero live instances.**

   **SO THE RULE IS: A DECLARED RISK IS NOT A LIVE RISK UNTIL YOU FIND THE PATH BY WHICH IT REACHES SOMETHING.** That is this note's own orphan-guard argument pointed the other way -- an instrument reading green is not evidence until something makes it go red, and a hazard read off a config is not a hazard until something makes it bite. **Do not churn a config against a standing ruling to remove an exposure you have not measured.**

6. **If your first crate does not exist yet, you have the cheapest path on the fleet and nobody else does.** Every estate in the table above is RETROFITTING. Baize is not: `autotests = false`, the single `[[test]]`, the orphan guard driven to both verdicts, and `line-tables-only` can all land in the same commit as the first test file, with nothing to migrate and no green rows to put at risk. **Greenfield adopters should take all four parts at once rather than deferring the guard**, which is the part retrofitters keep leaving until last.

## Verifying a consolidation, which is the part we got wrong twice in an hour

**THE CHARACTERISTIC RISK OF THIS CHANGE IS FLAKINESS, AND N CLEAN RUNS CANNOT DETECT FLAKINESS.** Merged targets are threads in one process where they were separate processes, so anything sharing process state stops failing cleanly and starts failing sometimes. That is the whole cost, and it is the one thing a green run does not speak to.

Intent shipped two claims on clean-run evidence and both were wrong:

| claim                                        | evidence offered                            | what it was worth                      |
| -------------------------------------------- | ------------------------------------------- | -------------------------------------- |
| the merge is safe                            | the workspace compiles, 5 targets confirmed | nothing about intermittents            |
| `daemon_subscriptions` fixed by isolating it | **8 clean full-workspace runs**             | nothing -- hv hit it again immediately |

**Driven two-sided afterwards, the second claim inverted outright: 0 failures in 10 parallel runs, 2 failures in 9 at `--test-threads=1`.** Serialising made it _worse_, so parallelism -- the stated cause -- could not have been the cause. **A two-verdict drive disproved in nineteen runs what nineteen clean runs could not have confirmed.**

So: **before claiming a consolidation is safe, run the suite in a loop under full-workspace contention until you have either a failure or a number of runs you can defend.** A single failure at 1-in-9 means three clean runs is what you would expect whether or not anything was fixed. And **when you think you have found the cause, drive the opposite arm** -- if removing the supposed cause does not help, the diagnosis is dead regardless of how many greens follow it.

### Two files stayed separate, and a third must not

`dual_path_conformance.rs` and `daemon_subscriptions.rs` keep their own `[[test]]` targets: one mutates process cwd, the other drives a per-process file-watch stream. **Both are asking for something a shared process cannot give, so isolation restores an isolation they always had.**

**`daemon_address.rs` is the one that looks identical and is not.** It fails intermittently under contention too, and isolating it would go green immediately -- but its failure is a REAL race in the product's socket-binding path, which the added concurrency merely exposed. **Isolating it lowers the concurrency until the window stops being hit: the failure disappears and the race does not.** That is the denominator attack, and an estate that reaches for isolation whenever a merged suite goes red will quietly convert every product race it surfaces into a green.

**The discriminator: does the test need isolation, or does the PRODUCT need fixing?** Ask what the test is asserting. A test asserting something about the process it runs in wants its own process. A test asserting something about the product wants the product fixed.

### The transform is not symmetric

Merging removes a file's `mod common;` and rewrites its paths to `crate::common::`. **Un-merging must put the declaration back**, or the file stops compiling with an error naming the helper rather than the move that broke it. Every estate that consolidates will eventually pull one file back out, and this is the first thing it meets.

## What it cost Intent, and what holding the cache cost instead

**Applied at `71a96213`. 257 targets became 6, MEASURED 2026-09-01 AT `a4465d66`.**

**THE `after` COLUMN IS A DATED MEASUREMENT AND NOT A CURRENT STATE, WHICH IS THE WHOLE POINT OF THE DATE.** It was a current state until v0.7 and that is exactly how it went wrong.

| crate      | targets before | after, at `a4465d66` |
| ---------- | -------------- | -------------------- |
| intentsvcs | 164            | 1                    |
| intent-cli | 84             | 2                    |
| intentd    | 8              | 2                    |
| testkit    | 1              | 1                    |
| **total**  | **257**        | **6**                |

**Verified by asking cargo, not by counting files** -- which is this note's own trap, so the verification must not walk into it:

```
cargo test --workspace --no-run 2>&1 | grep -c 'Executable tests/'
```

**THAT COMMAND CARRIED ITS ANSWER AS A `# -> 5` COMMENT UNTIL v0.8 AND THE COMMENT IS NOW GONE ON PURPOSE** (vc's amendment, and it corrects a rule this note had been prescribing to the whole fleet). **A PRINTED NUMBER GETS READ INSTEAD OF RUN.** Stating a figure with its regenerating command beside it makes the figure AUDITABLE and does nothing whatever for its CURRENCY -- this table proved it by going stale with the command printed directly underneath. So the rule this note now holds, and the one it failed:

- **A LIVE figure: print the COMMAND ALONE. Delete the number.** The reader runs it and gets today's answer, which is the only answer that was ever true.
- **A HISTORICAL figure: DATE IT.** A dated event is not a live claim and cannot rot.

**THE SECOND HALF IS THE TENSE TEST FROM THE ESTATE TABLE ABOVE, ARRIVING BY A DIFFERENT ROUTE** -- one argument started from what other estates do to a document, the other from what the authoring estate does to itself, converging on the same place: **the only figures that belong on a page are ones that already happened.**

**AND IT VINDICATES THE ORIGINAL ASK MORE THAN ANY OF THE REFINEMENTS DID.** devbin-vc asked for the verdicts to be dropped so a reader would run the command and get their own answer. Three of us then improved that into a scheme for printing better figures. **The thing that failed was the scheme.**

**THIS TABLE READ `intentd 1` AND `total 5` UNTIL v0.7, AND THE NUMBER WAS TRUE WHEN IT WAS WRITTEN** (caught by vc, adjudicated here with the command above). `33ac4348` gave `intentd` a SECOND declared target back -- `daemon_subscriptions`, which earned it by failing -- **1h37m after `71a96213` recorded the 5.** Three revisions then passed over the table without touching it, **including `73857a72`, whose own subject line names `33ac4348`.**

**AND IT SAT UNDER THE ONE CAPTION IN THIS NOTE THAT BOASTS ABOUT HOW IT WAS VERIFIED.** The caption is correct about method and was stale about result, which is the harder failure to see: a figure with its regenerating command printed beside it still goes stale, because **printing the command is not running it.**

**THE CLASS, AND IT IS THE THIRD INSTANCE IN THIS DOCUMENT** (vc's, and it is the transferable half): **A DOCUMENT APPLIES ITS OWN RULE TO THE CLAIMS ABOUT OTHER PEOPLE AND NOT TO THE CLAIMS ABOUT ITSELF.** The Lamplight row, the Intent row, and now the crate table -- **every one of them a figure about the authoring estate, and the figures about your own estate are the ones nobody audits, because everyone assumes the author would know.**

`intent-cli` and `intentd` each keep a second target deliberately. Merged targets are threads in ONE process, and across all 257 files exactly one mutates process-global state (`set_current_dir`), so `dual_path_conformance.rs` stays separate. **That was measured before merging rather than hoped for after** -- no test spawns cargo, the one fixed port is written to a file and parsed rather than bound, and every socket path is per-test.

### The figure that reframes the whole exercise

hv ran the full cycle from a total clean, watched:

|                                        |                              |
| -------------------------------------- | ---------------------------- |
| target tree held                       | **113 G**                    |
| of which per-node forks                | cc 41G, dc 11G, private 919M |
| time to DELETE it                      | ~35 minutes                  |
| **time to REBUILD BOTH BINARIES COLD** | **1m 36s**                   |

**THE CACHE COST MORE TO HOLD THAN TO REGENERATE.** 113 gigabytes and thirty-five minutes of deletion, standing guard over ninety-six seconds of work.

**THE BOUNDARY MUST TRAVEL WITH THAT NUMBER OR IT OVERCLAIMS:** 1m36s is `dvb build all`, the two RELEASE binaries. The TEST build is the separate and expensive half, and it is where the 400,000-file debug tree and the 257 targets lived. **The honest statement is: the shipped artefacts rebuild cold in 96 seconds, and essentially all of the 113G was test-build cache.**

**AND A SECOND BOUNDARY, ON WHAT THE CYCLE VERIFIED RATHER THAN WHAT IT TIMED.** A cold green cycle is easily read as confirming the consolidation, and on another estate it did not. **conflab-vc measured that Conflab's cycle passed `--lib` for the daemon without `--all-targets`, so it never compiled the consolidated `suite` target or the orphan guard at all** -- three phases green, and the consolidation untouched by any of them.

**THAT CAVEAT DOES NOT REACH INTENT, AND THE DISCRIMINATOR IS THE CONFIG RATHER THAN THE VERB.** Measured here rather than assumed from theirs: Intent's `test rust` is `cargo test --workspace --no-fail-fast --manifest-path native/rust/Cargo.toml` -- **no `--lib`** -- so `--workspace` compiles and runs the integration targets, the consolidated `suite` binaries and the orphan guards among them. The 14 suites in that run are the evidence; a `--lib`-only run cannot produce them.

**THE TRANSFERABLE PART IS THAT THE VERB'S NAME GUARANTEES NOTHING.** Two estates ran `fullcycle`, both went green, and only one of them compiled the thing the ruling is about. **So the question a note must answer is never _did the cycle pass_ but _what did this estate's cycle actually build_, and that is a per-estate question answered from that estate's own config.**

### Why 52G of it was per-node forks, which is not a discipline problem

**Cargo's build lock works correctly and invisibly, and that is the cause.** A blocked `cargo build` is indistinguishable from a hung one, so a node "fixes the hang" by pointing `CARGO_TARGET_DIR` at its own fork -- and one node reported doing so permanently, to be "one contender off the shared lock."

An instance happened in front of hv during this very job: dc's `cargo check` held the lock, hv's test run queued behind it, and hv reported that _"the rust tests look like they're hanging."_ They were not. dc killed theirs and hv's proceeded.

**So the fork is a rational response to an invisible wait, and a prohibition on `CARGO_TARGET_DIR` only holds if the wait is made visible. That pairing is the recommendation; either half alone fails.**

## Provenance

The ruling is hv's, 2026-08-27, estate-wide. The census, the build and the four-part shape are dc's, 2026-09-01, under hv's direction that fixing this across the host is the most important job in flight. The orphan-guard requirement follows laksa-vc's implementation. The files-are-not-targets caveat is dc's own, stated in the census that the scope was then read from -- which is how it comes to be the most useful paragraph in this note.
