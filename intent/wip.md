---
verblock: "28 Aug 2026:v1.24: vc - Conflab is on v3; the port is closed; the day it took"
intent_version: 3.0.0
---

# Work In Progress

**Current as at `013bc484`, 2026-08-28. This heading names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

## Where the project is

**v3.0.0 is shipped, tagged, and Intent is SELF-HOSTED on it. THE PORT IS CLOSED: all seventeen estates are on v3.** Conflab, the last, was hoisted on 2026-08-28 -- migration commit `b02b93c4`, prose carry `7652c9b4`, verified by five nodes across three estates against a pre-hop census (st 123 / wp 531 / ac 133 / at 141, all matched), a whole-tree manifest (deleted 0, every bucketed file byte-identical), and a two-arm guard control on its live hook (a stamp without `Z` refused, with `Z` committed). `Intentv2` stays on 2.19.0 by hv's standing rule; it is the tool tree, not a project. Full record: `intent/whiteboard/vc/cutover-runbook.md`.

    intent ac status ST0057   -> 66/66 satisfied, 3 withdrawn -- PASS
    intent ac status ST0056   -> 64/133 satisfied, 2 withdrawn -- BLOCKED
    intent doctor             -> 0 findings across 64 threads, 78 issues, 288 views, 1105 files

**RUN THESE. DO NOT TRANSCRIBE THE NUMBERS.** They have had three homes carrying three values before now.

## The delivered pair, and how to tell whether it is current

**Read it off the binary -- `intent --version` for the marker, `shasum -a 256` on the resolved target for the identity -- never off this page.** Delivery is a symlink from `~/.local/bin` into `native/rust/target/release`, so **a build IS the delivery** and there is no install step.

**THE MARKER IS PROVENANCE, NOT AN IDENTITY. Never compare it to `HEAD`** -- that is true after every board commit and says the alarming thing on the healthy case. The deciding test is:

    git diff --name-only <marker>..HEAD -- native/rust surface    # empty == CURRENT

**A rebuild deletes both binaries for ~60s**, during which every estate on this machine has no `intent` and every gate fails open, so one node rebuilds and announces it by properties.

## What tonight's rollout delivered

hv's instruction was _"coordinate dc, ic and cc to roll out the fixes"_. Landed: the v3 porter's **two** citation defects (`e935734d`, `eff618e8`); the clock guard's tolerance 0 and its third stamp surface (`3463f784`, `27b13f93`); AC-11.6 with the estate's first pty harness (`102af78f`); AC-14.7 as one transaction (`05222011`); the `sync --to-disk` remedy **and its false premise** (`04bc607f`); `--severity` enforcement (`8174de80`); `doctor`'s commit-gate check (`3805f359`); the doc-link gate (`6c380e09`); (A2)'s critic-guard body **landed inert** (`3b0063f3`); (B)'s carrier install and its can-it-RUN report (`f8a78e05`, `22a75509`); `--dehydrate` retired (`d395a5b5`); and `st list` showing the title (`b4d63b44`).

**WP-11 and WP-14 are both closed.**

## The finding that outranks the deliveries

**THE SWEEP hv RESERVED DOES NOT EXIST.** No v3 code path wrote `pre-commit.intent`, so ruling 4's refusing arm sat in the one layer nothing ships -- positive-controlled at 1 against the template and **0 against every estate carrier including this tree's**.

**Guard BODIES and the ROSTER propagate live from `INTENT_HOME`; only the CARRIER was frozen at install.** dc established by `cmp` that eleven estates carried **Intentv2's** template byte-for-byte, and Conflab's was installed from a template already a month stale. **The installer ran -- it read the wrong source.**

> **The guard bodies move with no ceremony available; the carrier cannot move at all. Both halves are one asymmetry.** (devbin-vc)

hv ruled **both (A2) and (B)**, (A2) first, then re-ruled that **(A2)'s body lands inert and its roster line waits on (B)** -- because installed carriers still run the critic themselves, so rostering it would double every finding in fifteen estates until the carriers refresh.

## Open, by owner

**hv:** the Lamplight hop-2 re-run; whether `publish_home` should refuse a temp root; the vacuous-gate remedy in `doctor`; **and the post-hoist list from 2026-08-28** -- Intent's own carrier (still the Aug-21 copy; the shim install here is one `intent claude upgrade --apply`), ST0121/WP-02 and ST0124/WP-02 recorded WIP with a passing gate, the 23 WP statuses silently defaulted to `not-started` (issue 0100), ST0056 AC-00.8 saying `settings.json` unchanged while the hop rewrites it, the whiteboard protocol having no information barrier (devbin-vc + devbin-cc, jointly), the `issues` verbs disagreeing on which source of truth they read, the devbin vendor being on three source commits fleet-wide.

**dc:** the (A2) roster line, string agreed, blocked on (B) being deployable.

**cc:** the bootstrap guard that makes the `$HOME`-write class unreachable at authoring time.

**vc:** the runbook's Learned section for the 28th is a stub and wants the day's findings folded by class; no `wp` verb writes a work package's **objective** (canon carries the field, `wp new` takes only a title); conflab-vc's `ingest` finding, where one `n-a` AT poisons every test-backed AC it covers.

## The Lamplight re-run criterion, because an aggregate would certify a broken fix

**1731 test rows, 74 still carrying a `file` value that opens with a bracket, and 12 + 30 + 32 = 74 exactly.** The re-run must take **exactly those 74 to zero, split three ways** -- never against the aggregate 62, which would pass while 32 rows stayed broken. **More than 74 moving is a regression signal.** The store has **no** recovery source for them: `legacy.raw` sits on 1057 rows and on **zero** of the 74, complementary by construction. **The disk is the only route.**
