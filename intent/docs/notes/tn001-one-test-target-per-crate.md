---
verblock: "01 Sep 2026:v0.1: vc - First technote; the consolidation ruling, its mechanism, and the trap in measuring it"
---

# TN001 -- One test target per crate: the ruling Intent made and did not apply

**Status: RULED (hv, 2026-08-27, estate-wide). APPLICATION IN FLIGHT IN INTENT AT TIME OF WRITING (dc, 2026-09-01, under hv's authority of the same morning). This note is the pattern and the reasoning; it is NOT a report that Intent is finished.** Every figure below carries the command that regenerates it, because a technote is read at boot and a figure in one goes stale silently.

Written by vc at hv's direction, on dc's census and dc's build. Circulated to every Intent-using project so each can make its own version rather than inherit ours.

## The defect in one line

**Under cargo's default `autotests = true`, every `.rs` file in a crate's `tests/` directory becomes its own test target -- a separate compile and a separate FULL LINK of the crate and all its dependencies.** A crate with 166 test files links 166 binaries to run its tests. Nothing warns you; it is the default, it is invisible in the output, and it grows one file at a time.

## Why this is a technote and not a ticket

**Intent ruled this estate-wide on 2026-08-27 and then did not do it.**

| estate     | consolidated, counting CRATES THAT HAVE TESTS          |
| ---------- | ------------------------------------------------------ |
| Laksa      | YES -- and its `Cargo.toml` quotes the ruling verbatim |
| Lamplight  | **YES -- 1 of 1**                                      |
| **Intent** | **NO -- not one crate**                                |

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

1. **Count your real targets, not your files.** `grep -c '^\[\[test\]\]' */Cargo.toml` and `grep -rn 'autotests' --include=Cargo.toml .` together tell you where you actually are. If `autotests` is absent, it is `true`.
2. **Apply the four parts above** to every crate whose test files outnumber its declared targets.
3. **Build the orphan guard before you consolidate, not after.** It is the only thing standing between a consolidated suite and a silently shrinking one.
4. **Write your own version of this note.** Not a copy -- your crate layout, your counts, your call sites. A note that describes someone else's estate is read once and never checked.

   **FILE IT WHERE YOUR TECHNOTES ALREADY LIVE, AND NUMBER IT NEXT IN YOUR OWN SEQUENCE. THE PATH AND THE NUMBER ARE YOURS, NOT INTENT'S TO CARRY ACROSS.** This paragraph replaces an instruction that said "if `intent/docs/notes/` does not exist yet, this will be your first" -- **which was wrong and would have done real damage** (found by conflab-vc, after it had already been circulated to five estates). Conflab's technotes live at `docs/notes/`, not `intent/docs/notes/`, and its `tn001` is taken by an unrelated note; followed literally, the instruction creates a SECOND notes tree containing a SECOND document numbered TN001 about a different subject. **Conflab's is `tn005`.** **And the failure is silent, because creating a directory always succeeds** -- nothing would have reported the duplicate.

   The instruction generalised Intent's own layout from a sample of two estates that happened to agree. **Before filing, look for the notes tree you already have.**

5. **If you own no Rust, check the PATH before you change the declaration.** Two estates declare `rust` in `intent/.config/config.json` while owning none -- Baize, whose only `Cargo.toml` files are dependency NIFs, and Gtools, which has **zero** `Cargo.toml` anywhere.

   **THIS ITEM ORIGINALLY SAID A DECLARED-BUT-UNOWNED LANGUAGE "ARMS A CRITIC OVER CODE YOU DID NOT WRITE". THAT IS AN ASSERTED CONSEQUENCE AND IT WAS NEVER TRACED** (corrected by baize-vc, who measured the path instead of accepting the claim). In Baize it does not hold, and structurally rather than by luck: `.gitignore` carries `/deps/`, so every Rust file there is ignored and can never be staged; and `check critic` defaults to `--staged`, reporting NOT APPLICABLE when it scans nothing. **The only Rust that exists cannot enter the only corpus the gate reads, so the declaration is INERT, not armed.** Gtools is inert for a simpler reason still -- no crates at all. **Two estates, one asserted hazard, zero live instances.**

   **SO THE RULE IS: A DECLARED RISK IS NOT A LIVE RISK UNTIL YOU FIND THE PATH BY WHICH IT REACHES SOMETHING.** That is this note's own orphan-guard argument pointed the other way -- an instrument reading green is not evidence until something makes it go red, and a hazard read off a config is not a hazard until something makes it bite. **Do not churn a config against a standing ruling to remove an exposure you have not measured.**

6. **If your first crate does not exist yet, you have the cheapest path on the fleet and nobody else does.** Every estate in the table above is RETROFITTING. Baize is not: `autotests = false`, the single `[[test]]`, the orphan guard driven to both verdicts, and `line-tables-only` can all land in the same commit as the first test file, with nothing to migrate and no green rows to put at risk. **Greenfield adopters should take all four parts at once rather than deferring the guard**, which is the part retrofitters keep leaving until last.

## Re-deriving Intent's own state

**Do not read a number off this note.** At the time of writing, Intent's four crates hold 263 files under `crates/*/tests/`, `autotests = false` is set in three of them, and `[[test]]` blocks declare four targets -- but the suite files and the call-site flags were still uncommitted work in progress, so the tree did not yet build in that configuration.

To get the current answer:

```
find native/rust/crates/*/tests -maxdepth 1 -name '*.rs' | wc -l    # files
grep -c '^\[\[test\]\]' native/rust/crates/*/Cargo.toml             # declared targets
grep -rn 'autotests' --include=Cargo.toml native/rust               # discovery off?
```

## Provenance

The ruling is hv's, 2026-08-27, estate-wide. The census, the build and the four-part shape are dc's, 2026-09-01, under hv's direction that fixing this across the host is the most important job in flight. The orphan-guard requirement follows laksa-vc's implementation. The files-are-not-targets caveat is dc's own, stated in the census that the scope was then read from -- which is how it comes to be the most useful paragraph in this note.
