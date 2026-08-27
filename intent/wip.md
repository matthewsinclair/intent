---
verblock: "27 Aug 2026:v1.23: vc - the rollout hv ordered, and the sweep that turned out not to exist"
intent_version: 3.0.0
---

# Work In Progress

**Current as at `a8f8ed3c`, 2026-08-27. This heading names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

## Where the project is

**v3.0.0 is shipped, tagged, and Intent is SELF-HOSTED on it.** Sixteen of seventeen estates are ported; **Conflab is the exception and its migration is REFUSED, not pending** -- `intent ingest` finds 7 blocking residue rows, all in `intent/st/ST0121/acceptance.md`. Repairing them is hv's call and is complicated by the remedy naming v2 tooling that estate does not have installed.

    intent ac status ST0057   -> 66/66 satisfied, 3 withdrawn -- PASS
    intent ac status ST0056   -> 64/133 satisfied, 2 withdrawn -- BLOCKED
    intent doctor             -> 0 findings across 64 threads, 78 issues, 288 views, 1105 files

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now.

## The delivered pair, and how to tell whether it is current

**`d395a5b5`, sha256 `b5c956f57ac0a800` / `15edb1544bc14578`.** Delivery is a symlink from `~/.local/bin` into `native/rust/target/release`, so **a build IS the delivery** and there is no install step.

**THE MARKER IS PROVENANCE, NOT AN IDENTITY. Never compare it to `HEAD`** -- that is true after every board commit and says the alarming thing on the healthy case. The deciding test is:

    git diff --name-only <marker>..HEAD -- native/rust surface    # empty == CURRENT

**The pair is 8 compiled inputs behind as of this writing and a rebuild is owed** -- it carries none of tonight's `st list` work. **A rebuild deletes both binaries for ~60s**, during which every estate on this machine has no `intent` and every gate fails open, so one node rebuilds and announces it by properties.

## What tonight's rollout delivered

hv's instruction was _"coordinate dc, ic and cc to roll out the fixes"_. Landed: the v3 porter's **two** citation defects (`e935734d`, `eff618e8`); the clock guard's tolerance 0 and its third stamp surface (`3463f784`, `27b13f93`); AC-11.6 with the estate's first pty harness (`102af78f`); AC-14.7 as one transaction (`05222011`); the `sync --to-disk` remedy **and its false premise** (`04bc607f`); `--severity` enforcement (`8174de80`); `doctor`'s commit-gate check (`3805f359`); the doc-link gate (`6c380e09`); (A2)'s critic-guard body **landed inert** (`3b0063f3`); (B)'s carrier install and its can-it-RUN report (`f8a78e05`, `22a75509`); `--dehydrate` retired (`d395a5b5`); and `st list` showing the title (`b4d63b44`).

**WP-11 and WP-14 are both closed.**

## The finding that outranks the deliveries

**THE SWEEP hv RESERVED DOES NOT EXIST.** No v3 code path wrote `pre-commit.intent`, so ruling 4's refusing arm sat in the one layer nothing ships -- positive-controlled at 1 against the template and **0 against every estate carrier including this tree's**.

**Guard BODIES and the ROSTER propagate live from `INTENT_HOME`; only the CARRIER was frozen at install.** dc established by `cmp` that eleven estates carried **Intentv2's** template byte-for-byte, and Conflab's was installed from a template already a month stale. **The installer ran -- it read the wrong source.**

> **The guard bodies move with no ceremony available; the carrier cannot move at all. Both halves are one asymmetry.** (devbin-vc)

hv ruled **both (A2) and (B)**, (A2) first, then re-ruled that **(A2)'s body lands inert and its roster line waits on (B)** -- because installed carriers still run the critic themselves, so rostering it would double every finding in fifteen estates until the carriers refresh.

## Open, by owner

**hv:** Conflab's 7 residue rows; the Lamplight hop-2 re-run; whether `publish_home` should refuse a temp root; the vacuous-gate remedy in `doctor`.

**dc:** the (A2) roster line, string agreed, blocked on (B) being deployable.

**cc:** the bootstrap guard that makes the `$HOME`-write class unreachable at authoring time.

**vc:** the runbook's 172KB section from the 26th, still unfolded; no `wp` verb writes a work package's **objective** (canon carries the field, `wp new` takes only a title); conflab-vc's `ingest` finding, where one `n-a` AT poisons every test-backed AC it covers.

## The Lamplight re-run criterion, because an aggregate would certify a broken fix

**1731 test rows, 74 still carrying a `file` value that opens with a bracket, and 12 + 30 + 32 = 74 exactly.** The re-run must take **exactly those 74 to zero, split three ways** -- never against the aggregate 62, which would pass while 32 rows stayed broken. **More than 74 moving is a regression signal.** The store has **no** recovery source for them: `legacy.raw` sits on 1057 rows and on **zero** of the 74, complementary by construction. **The disk is the only route.**
