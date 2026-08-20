---
node: dc
name: DevX Claude
role: worker
session_id: baf3a3a8-2d05-4e9a-8170-c1bdf1f0753c
heartbeat_at: 2026-08-20 17:11Z
status: active
focus: "**BOTH OF TODAY'S GATE ADMISSIONS ARE RULED, EVIDENCED AND UNSTARTED** -- nothing written to `cmd/precommit`. **(1) `canon_commit_check.sh`, ST0057 AC-03.6, cc's ask and my roster.** My 190-of-286 reservation is retired STRUCTURALLY, not by argument: INHERITED is never examined in narrowed mode. Denominator measured, which cc did not have -- **39 of the last 40 commits rc=0, and the one refusal IS the defective commit**. **It cannot wedge**, which is the question that decides admission: prettier writes the WORKTREE and is idempotent, so a refused attempt leaves the tree already holding the bytes its own remedy asks for. Dispatch UNCONDITIONAL -- a path trigger would be a second copy of the tool's own narrowing, and a skipped arm and a clean arm are both green. **TWO deliverables, never bundled (cc): the remedy edit makes the instruction TRUE, only `--staged` makes it UNNECESSARY.** **(2) `thread_view_skew_check.sh` answered `268 view(s) match` rc=0 for me against a binary NINE of its own inputs out of date** -- so `lib_binstale.sh` is an EXTRACTION of `surface_check.sh`'s refusal and never a copy: that reach list has already been wrong once."
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

**Nothing in flight. Tree clean of my work; `main` builds; 139 targets / 971 pass at `81b52de2` measured in a CLEAN detached worktree; clippy clean under `-D warnings`.**

## TODO

### 1. Next, and it is ruled rather than queued

- **ADMIT `thread_view_skew_check.sh` TO `cmd/precommit`, CONDITIONAL.** cc had been carrying this as _wiring `doctor`'s view-skew detection into the gate_; **it is mine**, the tool already exists, and its own MODULES row ends _awaiting dc's admission_. **cc's denominator came back zero because they checked the SHIPPED roster** (`lib/templates/hooks/`, 4 guards, live from `INTENT_HOME`) **and this belongs to the REPO-LOCAL one** (`cmd/precommit`, 11). Two rosters, two populations. Driven: `268 generated view(s) match the model`, rc=0, 0.33s -- **and the drive found what reading would not: `BIN=` at line 84 with NO staleness guard.** The binary it read was an hour older than HEAD. **The condition is one existing pattern, not a new mechanism:** `surface_check.sh` already refuses a binary staler than its inputs.

- **ADMIT `canon_commit_check.sh` TO `cmd/precommit` -- ST0057 AC-03.6, cc's ask, my roster. TWO DELIVERABLES, NEVER BUNDLED (cc): the remedy edit makes the instruction TRUE; only `--staged` makes it UNNECESSARY.** **MY 190-of-286 RESERVATION IS RETIRED AND STRUCTURALLY, NOT BY ARGUMENT** -- INHERITED is never examined in narrowed mode, because narrowing is by path-this-commit-touched and an inherited divergence is by definition elsewhere; the tool prints that on every run. **Denominator measured, which cc did not have: last 40 commits, 39 rc=0, 1 rc=1, and the refusal IS the defective commit** -- zero false positives, and being right about the broken commit says nothing about the other 39. **IT CANNOT WEDGE, which is the question that decides admission:** prettier writes the WORKTREE and is idempotent (both driven), so a refused attempt leaves the tree already holding the formatted bytes its own remedy asks for, and the retry is one `sync --to-store`. **Dispatch is UNCONDITIONAL -- ruled against my own first instinct.** A path trigger would be a second copy of the tool's own narrowing rule and its failure is SILENT, since a skipped arm and a clean arm are both green; arm 2 is not precedent, it always runs the tool and only shapes the argv.

### 2. Still recorded, still not built

- **`intent claude upgrade --apply` IS ALL-OR-NOTHING.** Three of four actions unwanted on this tree, including **regenerating AGENTS.md from 3.0.0 DOWN to 2.19.0** on a self-hosted v3 tree. `c4aba380` now points operators at that button more precisely, so the fleet question has a sharper edge.
- **`bin/int SAYS` is back with hv** and unbuildable where he put it without forking vendored devbin.

### 3. Mine and small

- **`author`/`content` PRINT NOTHING WHERE v2 PRINTS 136 BYTES.** A clean no-op returns before any output, so a prose language is indistinguishable from one that ran and found nothing.
- **CLI-level tests for `critic`** -- the module has 18, the command surface none.
- **`critic --no-such-flag` EXITS 1 WHERE v2 EXITS 2 -- the gate BLOCKS on a typo.** Spine clap-error path plus INV-02's `critic` exception: mine and ic's together.

### 4. Standing

- **WP-07 hosting sweep**: `agents` (42 call sites), `lang` (44), `claude skills` (23) / `subagents` (25) / `ws` (13) / `prime` (10). `upgrade` (29) and `start` (6) **unmeasured**, excluded by name because they write outside the sandbox.
- **AT-11.6 RESOLVED IN MY FAVOUR (vc, 16:08Z); THE CANON EDIT IS THEIRS AND LANDS SHORTLY.** They had re-cited the row onto `prepush`; **`prepush` clones HEAD into a tempdir, never touches `native/rust/target/release/`, refuses nobody and names no paths** -- so it implements the PREVENTION mitigation that AC-11.6 explicitly declined in favour of REFUSAL. vc's own second count, independent of the first: **`bin/int` is a symlink to the dispatcher and does not contain the string `prepush`** -- the row cited the INVOCATION, not the IMPLEMENTATION. **The row's deliverable is still unbuilt and still mine.**
- **AT-11.7** -- positive control is the `intentd` fossil marker. **D37 payload sweep, `AT-00.17`**; discriminator is CITATION versus FORMAT EXAMPLE.
- `output-contracts.md`; `doctor` v3 mirror (XS).

## Watch-outs

**Today's instances are verbatim in `.history/20260820/wip.md`; 08-19's in `.history/20260819/watch-outs-full.md`. These are the CLASSES.**

- **A CROSS-CHECK RECONCILES WHEN BOTH SIDES SHARE AN ERROR (ic, on my number).** I reported HEAD at 139/978; it is **971**, and the extra seven were ic's uncommitted files in the shared tree. My check was _974 + 4 = 978, 138 + 1 = 139_ -- **both true, and it felt like verification precisely because the same contaminated term sat on both sides.** The number was right and the SUBJECT was wrong. **And it self-heals**: it became true the moment ic committed. **Correct FORWARD, never force-push** -- a pointer from the wrong figure to the record contradicting it is what breaks the healing. **Measure in a detached worktree with an isolated `CARGO_TARGET_DIR` before a count goes into a message.**
- **A TRUE MEASUREMENT OF A DIFFERENT PROPERTY, OFFERED AS PROOF (vc, on themselves, AT-11.6).** `prepush` really did run on the push path and really did print `ok:` twice; vc watched it. **None of it was about the property the criterion names.** The companion line, from my own board and quoted back at me: _a green is only ever about the question the instrument asks._
- **NO INSTRUMENT WE OWN CATCHES AN EXPIRED CITATION -- ONLY A BUILDER TRYING TO SATISFY THE ROW DOES** (vc's generalisation, 16:08Z). Every instance today was found that way: me on AT-11.6 by failing to make the red-first arm red, cc twice on AT-10.14 and AC-10.2. **`at lint` exempts `to-write` from L2/L3 correctly**, so a citation is only ever validated by someone using it.
- **A COUNT MIXES _NOT BUILT_, _BUILT AND UNVERIFIED_, AND _VERIFIED AND UNMOVED_, AND ONLY THE FIRST IS WORK** (ic). Of seven rows I reported outstanding, `AC-05.3` is a row nobody moved: its only cover is `n-a` with `file: null`, so no test can ever move it.
- **DELETE THE BINDING, DO NOT SHIM IT.** Changing `plan` to take `&Realised` surfaced a SECOND lookup reading had missed, loud only because the old binding was gone. **The compiler is a population oracle and a shim blinds it.**
- **THE PROBE PATTERN EXCLUDES THE ANSWER AND NEVER SAYS SO.** `grep 'organize::plan('` found 3 sites, the compiler found 7. And ic's: `cargo test | tail -200 > log` then counting `test result:` counts **what survived the tail**. Write the whole log, count after. **AND IT RUNS THE OTHER WAY TOO, WHICH IS THE HALF THIS BULLET WAS MISSING: A NAME SEARCH RETURNS A FACT ABOUT THE SEARCH** (cc's framing, 2026-08-20). Sweeping which tools read the release binary, `grep -l 'target/release/intent'` answered FOUR and I was one sentence from reporting that a GATING arm was unguarded -- the fourth match was PROSE inside `runner_roster_check.sh`, the roster row describing a different tool. True population three, one guarded. **A MENTION READ AS A SUBJECT, three files from the header that documents the distinction.** The exclusion direction fails quietly; this one fails LOUDLY and in your favour, which is worse, because an over-count reads as diligence.
- **POLARITY BELONGS IN THE ASSERTION'S SHAPE, NOT IN WHOEVER READS THE OUTPUT.** `assert_eq!(after, before)` prints `after` as `left`, and I read a failure backwards -- calling four ADDED files a silent removal and saying the report was lying when it was telling the truth.
- **CITE THE IMPLEMENTATION, NEVER THE INVOCATION.** vc's AT-11.6 cited `bin/int`, a symlink to a dispatcher that does not contain the command's name; mine were three comments and a doc link naming `Facade::apply_manifest_edit`, a function actually called `edit_list`. **Same hop, two nodes, one day.**
- **AN INSTRUMENT WHOSE PREMISE EXPIRED IS A LIVE DEFECT.** Five documents on 2026-08-20 stated a world the estate had left, three of them vc's own rows. **Only a narrow slice is mechanical**, so name the class and do NOT build a detector for it.
- **MARK PROVENANCE PER CLAIM, NOT PER MESSAGE -- driven, read, or inferred.** The cost lands on the READER, which is why the writer never feels it.
- **VERIFY THE RETRACTION, NOT JUST THE CLAIM.** A self-correction is still a report, from the one person already wrong about that subject, arriving with the momentum of having just been careful.
- **`git commit --only <paths>` IS PATH-SCOPED, NOT HUNK-SCOPED.** It defends against a peer's STAGED index and does nothing about their UNSTAGED edits. **THE THREE REMEDIES ARE NOT INTERCHANGEABLE:** building the workspace first FAILS BY CONSTRUCTION; `git diff --cached -U0` needs a human to recognise a stranger's hunk; **a detached worktree catches it MECHANICALLY and is the only one that does.** `int prepush --force` is that check on demand.
- **A FIX THAT CHANGES A TYPE LEAVES EVERY USE UNVERIFIED, and the ones that still compile are exactly the ones nothing reports.**
- **SILENCE ON SUCCESS IS INDISTINGUISHABLE FROM NOT RUNNING.** Three instruments existed with no trigger in one day. **A WRITE SURFACE WITH NO RECORD IS THE SAME SHAPE** (cc): they answered my blocking question live, said so, and the durable inbox held nothing.
- **SYNTHESISE THE INSTANCE _AND_ KNOW THE LIVE POPULATION.** A red-first arm needs a SYNTHETIC instance or the estate is not free to fix its own defect; a green over synthetics alone cannot say whether the feature has ever RUN.
- **THE EXIT CODE IS NOT WHERE YOU THINK.** A pipe eats it; `grep -c` exits 1 on zero; **`die` calls `exit` and a redirection does not contain one** -- use a subshell.
- **MEASURE IN THE SHELL THAT WILL RUN THE CODE.** Mine is **zsh**; hooks run under **bash**, which strips NUL bytes in command substitution.
- **TWO SHELL TRAPS, THE SECOND FRIGHTENING.** `local a=$1 b=$a` does not see `a`. And an EXIT trap firing after a `local tmp` has left scope cleans up against an unbound name -- **its harmless and catastrophic forms differ by one character**, `rm -rf ""` versus `rm -rf "$tmp"/`, and the harmless one is what you get on the day you test it.
- **A CLAIM ABOUT A MUTABLE SUBJECT MUST NAME ITS REVISION**, and a timing figure must name its LOAD: one build measured 41s under peer load and 18s idle.
- **STANDING CONSTRAINTS.** Push `local` only unless hv says otherwise; `upstream`'s freeze is a **CI/CD BUDGET** freeze, so pushing spends what it protects. NEVER `git pull --rebase`; a peer `.git/index.lock` means WAIT. Timestamps READ FROM `date -u`; `git log` prints LOCAL time. **The markdown formatter is a second writer** -- one MODULES.md row cost 41 lines of column re-padding.

## Decisions

- (2026-08-20) **A REPORTER FAILS OPEN; AN ACTOR REFUSES -- TWO DOORS ON ONE MODEL, AND `intentfiles.rs` HAD SAID SO BEFORE EITHER EXISTED.** Its own words: _the grammar's real refusal belongs on the verbs that read the manifest deliberately._ `realised_from` answers `Unreadable` for a manifest that will not parse, which realises everything -- correct for `doctor`, catastrophic for a verb about to remove files. **The filter they share is extracted**, so the sigil space cannot change in one door and not the other.
- (2026-08-20) **ABSENT IS DECIDED AT THE FILESYSTEM, ONCE, BY THE CALLER THAT TOUCHES IT.** `realised_for_action` takes TEXT and has no opinion about absence -- inferring it from an empty string would collapse it with a manifest declaring NONE, an opposite state.
- (2026-08-20) **A WORKSPACE-WIDE CHECK BELONGS WHERE THERE IS ONE WRITER.** fmt and clippy refuse on a peer's half-finished file in the SHARED tree; the prepush clone carries nobody's uncommitted work and is the tree CI compiles. **And a fault that can only escape on a push belongs to a PUSH gate.**
- (2026-08-20) **TWO MECHANISMS ENFORCING DIFFERENT PROPERTIES ARE NOT TWO COPIES OF ONE** (vc, retracting their own Highlander argument on AT-11.6). Prevention and refusal are different criteria; declining to build the second because the first exists is not Highlander, it is a gap.
- (2026-08-20) **THE ROSTER IS THE RUNNER'S TO ANSWER, AND IT ANSWERS BEFORE IT DISPATCHES.** `int hooks` was burned once by a probe with a side effect -- it grepped `prepush` for a string, matched a COMMENT, and cloned the repository every time anyone asked what was wired. **Never ask a question by running the thing.**
- (2026-08-20) **THE COPIED FILE NAMES NO GUARD AND HOLDS NO ROSTER.** Roster lives in `pre-commit-guards.sh`, read live, never copied. Adding a guard is one line and reaches every consumer with no reinstall.
- (2026-08-20) **HOOKS ARE TRACKED VIA `core.hooksPath`, NOT VIA AN INSTALLER THAT COPIES** (hv). `.githooks/` is tracked; `pre-commit.intent` is **gitignored inside it** because it is canon the installer writes.
- (2026-08-20) **A TEMPLATE IS EMBEDDED BECAUSE `init` WRITES IT; A GUARD IS READ LIVE BECAUSE THE GATE DISPATCHES IT.**
- (2026-08-20) **A REFUSAL WHOSE REASON EXPIRED IS A LIVE DEFECT, NOT AN OWED CAPABILITY.**
- (2026-08-20) **AN INSTRUMENT THAT REPRODUCES THE DEFECT IT WAS BUILT TO CATCH IS WORSE THAN ONE THAT UNDER-REPORTS AND SAYS SO.** The two errors are not comparable, so the conservative instrument wins and its false negatives get named in the file.
