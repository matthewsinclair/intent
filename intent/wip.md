---
verblock: "28 Aug 2026:v1.25: vc - the bounce: four rulings discharged, option 1 landed, the day folded"
intent_version: 3.0.0
---

# Work In Progress

**Current as at `dcb425f1`, 2026-08-28. This heading names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

## Where the project is

**v3.0.0 is shipped, tagged, and Intent is SELF-HOSTED on it. THE PORT IS CLOSED: all seventeen estates are on v3.** Conflab, the last, was hoisted on 2026-08-28 -- migration commit `b02b93c4`, prose carry `7652c9b4`, verified by five nodes across three estates against a pre-hop census (st 123 / wp 531 / ac 133 / at 141, all matched), a whole-tree manifest (deleted 0, every bucketed file byte-identical), and a two-arm guard control on its live hook (a stamp without `Z` refused, with `Z` committed). `Intentv2` stays on 2.19.0 by hv's standing rule; it is the tool tree, not a project. Full record: `intent/whiteboard/vc/cutover-runbook.md`.

**THE POST-HOIST RULINGS ARE DISCHARGED (hv, 2026-08-28 13:26Z, recorded `wb(hv)` at `3d5a710e`):** AC-00.8 amended to the ruled behaviour (`1098ac0f`); the `Superseded -> Cancelled` status mapping landed (`4479264f`, hv's D3 option 1 -- the `Deferred` question and the structural `status_legacy` fix are deliberately separate and sequenced); the two Conflab WP-02 fiat-closes routed to conflab-vc on hv's word. cc's 0110 renderer fix has its first slice in (`f02fb55f`); **both fixes are source-only until the next pair rebuild.**

    intent ac status ST0057   -> 66/66 satisfied, 3 withdrawn -- PASS
    intent ac status ST0056   -> 64/133 satisfied, 2 withdrawn -- BLOCKED
    intent doctor             -> 0 findings across 64 threads, 78 issues, 288 views, 1105 files

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now.

## The delivered pair, and how to tell whether it is current

**Read it off the binary -- `intent --version` for the marker, `shasum -a 256` on the resolved target for the identity -- never off this page.** Delivery is a symlink from `~/.local/bin` into `native/rust/target/release`, so **a build IS the delivery** and there is no install step.

**THE MARKER IS PROVENANCE, NOT AN IDENTITY. Never compare it to `HEAD`** -- that is true after every board commit and says the alarming thing on the healthy case. The deciding test is:

    git diff --name-only <marker>..HEAD -- native/rust surface    # empty == CURRENT

**A rebuild deletes both binaries for ~60s**, during which every estate on this machine has no `intent` and every gate fails open, so one node rebuilds and announces it by properties.

## What the 2026-08-27 rollout delivered

hv's instruction was _"coordinate dc, ic and cc to roll out the fixes"_. Landed: the v3 porter's **two** citation defects (`e935734d`, `eff618e8`); the clock guard's tolerance 0 and its third stamp surface (`3463f784`, `27b13f93`); AC-11.6 with the estate's first pty harness (`102af78f`); AC-14.7 as one transaction (`05222011`); the `sync --to-disk` remedy **and its false premise** (`04bc607f`); `--severity` enforcement (`8174de80`); `doctor`'s commit-gate check (`3805f359`); the doc-link gate (`6c380e09`); (A2)'s critic-guard body **landed inert** (`3b0063f3`); (B)'s carrier install and its can-it-RUN report (`f8a78e05`, `22a75509`); `--dehydrate` retired (`d395a5b5`); and `st list` showing the title (`b4d63b44`).

**WP-11 and WP-14 are both closed.**

## The finding that outranks the deliveries

**THE SWEEP hv RESERVED DOES NOT EXIST.** No v3 code path wrote `pre-commit.intent`, so ruling 4's refusing arm sat in the one layer nothing ships -- positive-controlled at 1 against the template and **0 against every estate carrier including this tree's**.

**Guard BODIES and the ROSTER propagate live from `INTENT_HOME`; only the CARRIER was frozen at install.** dc established by `cmp` that eleven estates carried **Intentv2's** template byte-for-byte, and Conflab's was installed from a template already a month stale. **The installer ran -- it read the wrong source.**

> **The guard bodies move with no ceremony available; the carrier cannot move at all. Both halves are one asymmetry.** (devbin-vc)

hv ruled **both (A2) and (B)**, (A2) first, then re-ruled that **(A2)'s body lands inert and its roster line waits on (B)** -- because installed carriers still run the critic themselves, so rostering it would double every finding in fifteen estates until the carriers refresh.

## Open, by owner

**hv:** the Lamplight hop-2 re-run; whether `publish_home` should refuse a temp root; the vacuous-gate remedy in `doctor`; **the `Deferred` ruling** (option 2 of the 0100 menu, deliberately separate from the shipped mapping -- vc holds the context and puts it when hv wants it); the standing-directives LIVE/SPENT split (vc's draft awaiting hv's approval); the parked devbin set (info barrier, `issues` verbs SSOT, vendor spread -- hv: _"a devbin problem that I will fix with devbin next"_).

**dc:** the (A2) roster line, string agreed, blocked on (B) being deployable; issues 0106 + 0109.

**cc:** 0110's closure (first slice `f02fb55f`; the oscillation is dead only when apply and prettier agree on the same bytes, verified through both writers); the pair rebuild that delivers `4479264f` + `f02fb55f`; 0103's 28-vs-71 split when hv turns to it; the sequenced structural status fix (0100 option 3 -- model design proposed to hv BEFORE building, hv's 12:49Z direction); the bootstrap guard that makes the `$HOME`-write class unreachable at authoring time.

**vc:** no `wp` verb writes a work package's **objective** (canon carries the field, `wp new` takes only a title); conflab-vc's `ingest` finding, where one `n-a` AT poisons every test-backed AC it covers.

**conflab-vc:** the two WP-02 fiat-closes (ruled, routed 2026-08-28, completion owed back to intent-vc); the 50 dateless completed threads (hv: _"leave that to conflab"_).

## The Lamplight re-run criterion, because an aggregate would certify a broken fix

**1731 test rows, 74 still carrying a `file` value that opens with a bracket, and 12 + 30 + 32 = 74 exactly.** The re-run must take **exactly those 74 to zero, split three ways** -- never against the aggregate 62, which would pass while 32 rows stayed broken. **More than 74 moving is a regression signal.** The store has **no** recovery source for them: `legacy.raw` sits on 1057 rows and on **zero** of the 74, complementary by construction. **The disk is the only route.**
