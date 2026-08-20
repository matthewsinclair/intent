---
node: dc
name: DevX Claude
role: worker
session_id: baf3a3a8-2d05-4e9a-8170-c1bdf1f0753c
heartbeat_at: 2026-08-20 15:58Z
status: active
focus: "**FIVE LANDED. AC-04.7 (`3661f288`/`67a84577`), AC-04.6 (`8db6b67c`), `int hooks` reports the four shipped guards it was silent about (`c4aba380`), and fmt+clippy now run over the prepush CLONE (`1005fc2a`) -- the one workspace in the estate with a single writer, which is why the workspace-wide obstacle does not apply there.** **ST0057/WP-04 IS 7/7 AND WP-06 IS CLOSED, so neither of my ST0057 WPs has an outstanding row. GATE 59 OF 66; NONE OF THE SEVEN OUTSTANDING ARE MINE.** **NEXT: admit `thread_view_skew_check.sh` (1 gated triple -> 268 views) CONDITIONAL on it gaining `surface_check.sh`'s staleness refusal -- it reads a release binary at line 84 with no mtime guard, so its green is about whatever was built last, not HEAD.**
claims: [ST0056/07, ST0056/11, ST0057/04, ST0057/06]
---

# DevX Claude (dc)

## D42 -- TIME. Read this before writing anything, anywhere.

**DB records have a timestamp field. That is the source of truth for time. Nothing else. Ever.**

- **You never ask what time it is** -- not the OS, not `date`, not the filesystem, **and not the database either.**
- **The stamp is applied BY the write**, at INSERT/UPDATE/UPSERT/DELETE.
- **hv's structural close: NO cli or intentsvcs function TAKES a time.** Functions may RETURN times. **IN is forbidden, OUT is fine.**
- **THE RULE IS ABOUT SIGNATURES, NOT VALUES.** A time-typed input parameter is a defect by inspection. **Caught me on 2026-08-20 in `init.rs`** -- `fn substitute(.., date: &str, ..)` directly under a module comment claiming no time enters. Now a closure over the store's returned stamp.
- **Not exceptions:** test fixtures; "only reading it"; **"but it came from the database"**; "it is just a label".
- **A board stamp is a label, not data.** The ordering that cannot be fabricated is the **commit**.

## The truth model -- canon, held not restated

`design.md` (D01 as reversed) + `data-model.md`. **The db is the durable SSOT, files are re-creatable; the typed API is the only door in; migrations are normal.**

## DOING

**Nothing in flight. Tree clean of my work; `main` builds; clippy clean under `-D warnings`; 964 pass / 0 fail.**

## TODO

### 1. Recorded, NOT built -- two of the four landed today

- **`int check format` AND CLIPPY: DONE (`1005fc2a`).** They now run over the **prepush clone**, and the obstacle I had recorded was real but scoped to the wrong tree: workspace-wide refuses on a peer's half-finished file **in the SHARED checkout**, and the clone is a `git clone --local` of the pushed revision with one writer and no uncommitted work. **The gate that catches a fault which can only escape on a push is a PUSH gate.** Driven both ways -- planted divergence REFUSED, same tree clean PASSES. Cost, with its conditions: fmt 0.42s and no compile, clippy 15.4s on a build just finished, whole run 34.7s.
- **`int hooks` UNDER-REPORTED BY FOUR: DONE (`c4aba380`).** `--list-guards` on the shipped runner, `intent info` to resolve the install, three states driven (present / not-applicable / MISSING, with MISSING winning) and three absences kept apart. **`guards: 4 ran` on the very next commit was the positive control that adding the arm did not break the dispatch.**
- **`intent claude upgrade --apply` IS STILL ALL-OR-NOTHING.** On this tree three of four actions are unwanted, including **regenerating AGENTS.md from 3.0.0 DOWN to 2.19.0** on a self-hosted v3 tree. `c4aba380` makes `int hooks` point at that button more precisely, so it is now a fleet question with a sharper edge.
- **`bin/int SAYS` IS BACK WITH hv** and unbuildable where he put it without forking vendored devbin.

### 1b. THE SKEW ADMISSION -- RULED TODAY, ONE CONDITION OUTSTANDING

**cc asked whose the `doctor` view-skew wiring is. IT IS MINE**, and it was an owed decision of mine wearing their label: the tool exists, and its own MODULES row ends _awaiting dc's admission_. **cc's denominator came back zero because they checked the SHIPPED roster (`lib/templates/hooks/`, 4 guards, live from `INTENT_HOME`) and this belongs to the REPO-LOCAL one (`cmd/precommit`, 11).** Two rosters, two populations.

**DRIVEN rather than ruled from the header: `268 generated view(s) match the model`, rc=0, 0.33s.** And the drive found what reading would not: **`BIN=` at line 84 with NO staleness guard** -- no `mtime`, no `newer`. The binary it read was built at 15:01 against a HEAD several commits back. **A gated green about whatever was built last is the class this estate has pulled out of four files today.** Admit it, conditional on the staleness refusal `surface_check.sh` already carries. Mine, next.

### 1a. ST0057 IS OUT OF MY HANDS

**WP-04 is 7/7 and WP-06 is CLOSED.** AC-04.6 was built to vc's REWRITTEN clause, and the rewrite was load-bearing: the live estate comes back **EQUAL at 3 on disk and 3 declared on the first run**, so a tool built to the original clause would have reported a green that meant nothing. Synthetic red, live population printed, five self-test arms -- including _remove the planted file and go green again_, which arms 1 and 2 alone cannot tell from a tool that always reds. **WP-04 close verification is with vc.**

**AT-11.6 I HAVE NOT MOVED, AND WILL NOT UNTIL vc RULES.** They re-cited it from an unbuilt shell tool onto `prepush`, correctly refusing a second guard for a property one already enforces. But **AC-11.6 rules for the REFUSAL mitigation in those words** -- _refusing a dirty cross-owner rebuild prevents it AND TELLS SOMEONE WHY_ -- and `prepush` implements the PREVENTION one: clone HEAD, build clean, refuse nobody, name no paths. **I cannot drive the red-first arm against it, so I cannot green it honestly.** The `stale_at_check` arm cc relayed is telling the truth.

### 2. Mine and small

- **`author`/`content` PRINT NOTHING WHERE v2 PRINTS 136 BYTES.** My clean no-op returns `Ok(())` before any output, so a prose language is indistinguishable from one that ran and found nothing.
- **CLI-level tests for `critic`** -- the module has 18; the command surface has none.
- **`critic --no-such-flag` EXITS 1 WHERE v2 EXITS 2 -- the gate BLOCKS on a typo.** Issue 0043 on the git side. **Not a one-token fix**: the spine's clap-error path plus INV-02's `critic` exception, so mine and ic's together.
- **`--languages` ARITY IS A PROPOSAL TO ic, NOT MY EDIT.** Fix site is the TABLE (`"arity": "1"` -> `"0..1"`), which is ic's SSOT.

### 3. Owed decisions I said I would rule once

- **`canon_commit_check.sh` ADMISSION.** cc has driven `--staged` and both recorded objections are dead: the narrow arm is 3.7s, the inherited clause is structural, `--exhaustive` sees what narrowed excludes. **The number I want to think about is 190 pre-existing divergent attachments of 286** -- a gate whose whole-set mode would refuse two thirds of the estate is one whose ADDS-only narrowing is load-bearing rather than a convenience, and that changes what admitting it means.
- **`thread_view_skew_check.sh` ADMISSION** -- gated skew stays 1 of 269 until I rule. The roster it was blocked on is now one live home, so the question has changed shape.
- **FIVE INHERITED FLAGS from cc's AT-06.8.** `st bootstrap --audit-only` / `--dry-run` / `--deliverable` are mine when `bootstrap` lands. The ratchet is equality, so wiring one reds the test and forces the list to shrink -- that is the mechanism, not a nuisance.

### 4. Standing

- **WP-07 hosting sweep**: `agents` (42 call sites), `lang` (44), `claude skills` (23) / `subagents` (25) / `ws` (13) / `prime` (10). `upgrade` (29) and `start` (6) **unmeasured**, excluded by name because they write outside the sandbox.
- **AT-11.6 / AT-11.7** still `to-write`. **AT-11.6 takes a row citing `prepush --force` and NO new code** -- the mechanism existed all along and I reimplemented it worse. 11.7's positive control is the `intentd` fossil marker.
- **D37 payload sweep, `AT-00.17`** at `no_pm_state_in_payload.rs`, minted `to-write`. Discriminator is CITATION versus FORMAT EXAMPLE; cuts ~80 sites to ~15 with no per-site judgement. Two unplanted positive controls verified at zero hops.
- `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

- **A CROSS-CHECK RECONCILES WHEN BOTH SIDES SHARE AN ERROR (ic, on my number, and it is the sharpest thing said to me today).** I reported HEAD at 139/978; **HEAD alone is 139/971** and the extra 7 were ic's uncommitted files in the shared tree. My check was _974 + 4 = 978, 138 + 1 = 139_ -- **both true, and it felt like verification precisely because the same contaminated term sat on both sides.** The number was right and the SUBJECT was wrong. **And it self-heals**: it became true the moment ic committed, so nothing afterwards could show it had been false. Corrected FORWARD in `8db6b67c` rather than force-pushed -- a pointer from the wrong figure to the record contradicting it is what breaks the healing; a rewritten message erases the evidence the class exists. **Measure in a detached worktree with an isolated `CARGO_TARGET_DIR` before a count goes into a message.**
- **A COUNT MIXES _NOT BUILT_, _BUILT AND UNVERIFIED_, AND _VERIFIED AND UNMOVED_, AND ONLY THE FIRST IS WORK** (ic). Of the seven rows I reported outstanding, `AC-05.3` is a row nobody moved: its only cover is `n-a` with `file: null`, so no test can ever move it.

- **`assert_eq!(after, before)` PRINTS `after` AS `left`, AND I READ A FAILURE BACKWARDS BECAUSE OF IT** -- calling four ADDED files a silent removal, and saying out loud that the report was lying when it was telling the truth. **The polarity now lives in the assertion's SHAPE (a subset check over a named `removed` list) rather than in whoever reads the output**, which is the only version that cannot be misread. A suite that contains `organize_preview_polarity.rs` is one that already knew.
- **DELETE THE BINDING, DO NOT SHIM IT.** Changing `plan` to take `&Realised` surfaced a SECOND declared-set lookup I had not found by reading. It was loud only because the old binding was gone; a compatibility shim would have kept the old semantics at the one site nothing reports. **The compiler is a population oracle and a shim blinds it.**
- **THE PROBE PATTERN EXCLUDES THE ANSWER AND NEVER SAYS SO -- twice today, same shape.** `grep 'organize::plan('` found 3 call sites; the compiler found 7, because tests import the symbol unqualified. And ic's: `cargo test | tail -200 > log` then counting `test result:` lines counts **what survived the tail**, not what ran -- 18 targets / 94 tests reported for a run of 138 / 974. **Write the whole log, count after.**
- **`str.replace` HAS NO COUNT ARGUMENT.** `ic/inbox.dc.md` carried TWO committed `_(empty)_` sentinels from my own fold, so one announce filled both. **The defect was mine in two layers -- the doubled sentinel and the unbounded replace -- and ic found it at zero hops in their own file.**

- **MARK PROVENANCE PER CLAIM, NOT PER MESSAGE -- driven, read, or inferred.** cc sent four claims in one voice; three were driven in code and one read off a directory listing, and nothing said which, so I spent a careful check on the wrong one. **The cost lands on the READER, which is why the writer never feels it.** True of most of my messages today -- _the guard is perfect and has nowhere to run_ went to hv in the same register as things I had driven. **Reasoning wearing a measurement's clothes.**
- **VERIFY THE RETRACTION, NOT JUST THE CLAIM.** A self-correction is still a report, from the one person already wrong about that subject, and it arrives with the momentum of having just been careful.
- **`git commit --only <paths>` IS PATH-SCOPED, NOT HUNK-SCOPED.** It defends against a peer's STAGED index and does nothing about their UNSTAGED edits in a file you also touched. It put ic's caller at HEAD without its callee and **`main` did not compile**. **THE THREE REMEDIES ARE NOT INTERCHANGEABLE:** building the workspace first FAILS BY CONSTRUCTION (the shared tree compiles either way); `git diff --cached -U0` catches it and needs a human to recognise a stranger's hunk; **a detached worktree at the named revision catches it MECHANICALLY and is the only one that does.**
- **A FIX THAT CHANGES A TYPE LEAVES EVERY USE OF THAT VALUE UNVERIFIED, and the ones that still compile are exactly the ones nothing reports.** One edit produced two silent survivors: `.ok()` swallowing clap's `UnknownArgument`, and `&author` outliving the `String` it was written for.
- **AN INSTRUMENT KEYED ON A CODING IDIOM DEPENDS ON NOBODY IMPROVING THE CODE** (cc's name for it). One level below ST0039's _substring standing in for a syntactic fact_: the syntactic fact was itself an accident of style. My loop avoided duplicating a string -- right in that file -- and made the id invisible to a scanner keyed on literals in another.
- **SYNTHESISE THE INSTANCE _AND_ KNOW THE LIVE POPULATION.** A red-first arm needs a SYNTHETIC instance or the estate is not free to fix its own defect (vc). **But a green suite over synthetics alone cannot tell you whether the feature has ever RUN** -- five green tests, live population zero.
- **A POPULATION CAN GO OUT FROM UNDER A CHECKER, AND A NAME CAN SPAN POPULATIONS.** Four instances in a day: a dispatch test blind to the two guards that never ran; `emitting_names`/`refused_names` losing a format to a new variant; `error_remedies.rs` provoking a refusal with a format that stopped refusing; a `withheld > 0` guard whose subject shipped. **And `grep -c` on a field name with three subjects is a count of nothing until you say which level.**
- **SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING.** A guard that passes prints nothing; so does a runner nothing dispatched. **THREE INSTRUMENTS EXISTED WITH NO TRIGGER IN ONE DAY**: `canon-ignore-guard.sh` (no roster), `prepush` (nobody reached it), `check format` (no dispatcher).
- **THE PROBE POPULATION CAN EXCLUDE THE ANSWER AND NEVER SAYS SO.** `cargo test` stops at the first failing binary; a `head -20` hides the twenty-first; a failing assertion hides the ones after it. **Read the target list, not the last `test result:` line.**
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it. **`grep -c` exits 1 on zero and a pipe eats THAT** -- empty output reads as absence, and it nearly reversed a correct correction. **Drop the `-c` and read the lines.**
- **MEASURE IN THE SHELL THAT WILL RUN THE CODE.** My shell is **zsh**; the hooks run under **bash**, which strips NUL bytes in command substitution and says so. A `-z` pipeline measured clean in zsh and was broken in bash.
- **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION.** Name revision, clock and dirty count on every measurement.
- **STANDING CONSTRAINTS.** Push `local` only unless hv says otherwise; `upstream` runs the only CI and its freeze is a **CI/CD BUDGET** freeze, so pushing spends the thing it protects. NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. Timestamps READ FROM `date -u`; `git log` prints LOCAL time. The markdown formatter is a second writer.

## Decisions

- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES -- TWO DOORS ON ONE MODEL, AND `intentfiles.rs` HAD ALREADY SAID SO BEFORE EITHER EXISTED.** Its own words: _the grammar's real refusal belongs on the verbs that read the manifest deliberately._ `realised_from` answers `Unreadable` for a manifest that will not parse, which realises everything -- correct for `doctor`, catastrophic for a verb about to remove files. `realised_for_action` returns the parse error instead. **The filter they share is extracted, so the sigil space cannot change in one door and not the other.**
- (2026-08-20) **ABSENT IS DECIDED AT THE FILESYSTEM, ONCE, BY THE CALLER THAT TOUCHES IT.** `realised_for_action` takes TEXT and has no opinion about absence -- inferring it from an empty string would collapse it with a manifest declaring NONE, which is a real and opposite state.

- (2026-08-20) **THE COPIED FILE NAMES NO GUARD AND HOLDS NO ROSTER.** The bodies resolved live and the array naming them did not, so canon gained guards and consumers did not. Roster lives in `pre-commit-guards.sh`, read live, never copied. **Adding a guard is one line and reaches every consumer with no reinstall.**
- (2026-08-20) **HOOKS ARE TRACKED VIA `core.hooksPath`, NOT VIA AN INSTALLER THAT COPIES** (hv). The copy mechanism is the disease just cured. `.githooks/` is tracked; `pre-commit.intent` is **gitignored inside it** because it is canon the installer writes, and tracking it would be a second home for canon.
- (2026-08-20) **`export --format md` DELEGATES TO `Facade::realise()`** (hv). **The premise that expired is the POPULATION, not the fact** -- markdown still cannot be read back. A verb returning a destination for tree-shaped output and a document for document-shaped output is describing reality, not compromising.
- (2026-08-20) **A TEMPLATE IS EMBEDDED BECAUSE `init` WRITES IT; A GUARD IS READ LIVE BECAUSE THE GATE DISPATCHES IT.** Two mechanisms, two populations, stated rather than implied -- which is why `lib/templates/hooks/**` is deliberately NOT in the embed.
- (2026-08-20) **THE ENV READ BELONGS IN `bootstrap`, NOT `init`.** v2 already puts it there. Removing it beat getting an ALLOWED row: **no ruling needed is a better outcome than a ruling.**
- (2026-08-20) **A REFUSAL WHOSE REASON EXPIRED IS A LIVE DEFECT, NOT AN OWED CAPABILITY.**
- (2026-08-20) **AN INSTRUMENT THAT REPRODUCES THE DEFECT IT WAS BUILT TO CATCH IS WORSE THAN ONE THAT UNDER-REPORTS AND SAYS SO.** The two errors are not comparable, so the conservative instrument wins and its false negatives get named in the file.
