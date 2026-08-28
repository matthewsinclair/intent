---
verblock: "28 Aug 2026:v1.27: vc - D2 complete on Conflab, the pair delivers 4479264f, a window that recruits"
intent_version: 3.0.0
---

# Work In Progress

**Current as at `d68f0395`, 2026-08-28. This heading names a COMMIT, not a date** -- a wip file is read as current and is only ever true of a tree.

## Where the project is

**v3.0.0 is shipped, tagged, and Intent is SELF-HOSTED on it. THE PORT IS CLOSED: all seventeen estates are on v3.** Conflab, the last, was hoisted on 2026-08-28 -- migration commit `b02b93c4`, prose carry `7652c9b4`, verified by five nodes across three estates against a pre-hop census (st 123 / wp 531 / ac 133 / at 141, all matched), a whole-tree manifest (deleted 0, every bucketed file byte-identical), and a two-arm guard control on its live hook (a stamp without `Z` refused, with `Z` committed). `Intentv2` stays on 2.19.0 by hv's standing rule; it is the tool tree, not a project. Full record: `intent/whiteboard/vc/cutover-runbook.md`.

**THE POST-HOIST RULINGS ARE DISCHARGED (hv, 2026-08-28 13:26Z, recorded `wb(hv)` at `3d5a710e`):** AC-00.8 amended to the ruled behaviour (`1098ac0f`); the `Superseded -> Cancelled` status mapping landed (`4479264f`, hv's D3 option 1 -- the `Deferred` question and the structural `status_legacy` fix are deliberately separate and sequenced); the two Conflab WP-02 fiat-closes routed to conflab-vc on hv's word. cc's 0110 renderer fix has its first slice in (`f02fb55f`); **both fixes are source-only until the next pair rebuild.**

**THREE THREADS PRIMED ON hv's FIVE-ITEM DIRECTIVE (`e28ff02d`), work queued behind the hoist tail:** ST0066 (fiat close -- human-only force-close with the why on the record), ST0067 (`intent llm` -- the LLM's discovery surface, hv: asap), ST0065 (root-and-branch review of the canon .md set + the /in-* skills, two WPs, Boris Cherny transcript vendored at `_sources/`).

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

**hv:** the Lamplight hop-2 re-run (GATED, per cc's measured re-premise of 0111, by the MISSING markdown-to-store route -- 0097's family, new work, costed menu owed to hv; falsifier pending with lamplight-vc); whether `publish_home` should refuse a temp root; the vacuous-gate remedy in `doctor`; **Laksa's `SUPERSEDED` THREAD status** (cc's flag: the identical vocabulary gap in `thread_status`, on the arm that BLOCKS rather than carries, and by cc's census the single finding blocking that estate's migration -- widening it is an hv ruling because it unblocks a migration as a side effect); ic's Phase 4 instruments placement (node scratch vs project apparatus -- the three census tools died with the session, designs survive in ic's history); the parked devbin set (info barrier, `issues` verbs SSOT, vendor spread -- hv: _"a devbin problem that I will fix with devbin next"_).

**dc:** ASSIGNED (hv 13:51Z): the message-mechanism family -- 0106, 0109, 0112, 0113 (four defects, one class: the message is not the mechanism). Standing: the (A2) roster line, string agreed, blocked on (B) being deployable.

**cc:** the rebuild is DONE and verified by properties (pair names `4479264f`, `intent` `fd5e785d`, `intentd` `f9140c0b`; 0110's renderer fix verified through the PUBLISHED binary, both sets converge). 0110's third consequence survives with an independent cause -- report mode prints the ROSTER, not a diff -- filed by cc as **0115**. 0111 STOPPED by cc's own measured refutation of its cause (the verb is not blind; both homes hold the truncation) -- falsifier with lamplight-vc, the surviving message contradiction is cc's to fix now, re-premise after the answer. `Deferred` is RULED wait-for-structural (13:51Z) -- no interim mapping; the sequenced structural fix (0100 option 3) still routes its model design to hv before building. Standing: 0103's 28-vs-71 split when hv turns to it; the bootstrap guard for the `$HOME`-write class.

**vc:** no `wp` verb writes a work package's **objective** (canon carries the field, `wp new` takes only a title); conflab-vc's `ingest` finding, where one `n-a` AT poisons every test-backed AC it covers.

**conflab-vc:** the two WP-02 fiat-closes are DONE (hv's word given in their session; Conflab `39ad847c`; disagreements 2 -> 0 READ FROM doctor's decomposition, `status-gate-disagreement` absent entirely; panel-survival half recorded ACCEPTED UNVERIFIED, provenance naming the relay). CONFLAB IS CLOSED OUT at `3ae2e8cb`, nodes paused, nothing dispatched: doctor 51, NONE Conflab-actionable -- 50 blocked on Intent 0118 (recovered input ready in Conflab issue 0010), 1 is the 0105-family false positive awaiting dc's fix DELIVERY. Their four contract-prose edits are untouched pending hv's carried-over sequencing call: drive 0119's guarded byte-faithful procedure, or wait for a real edit verb. The inexpressible-half defect is Intent **0116**.

## The Lamplight re-run criterion, because an aggregate would certify a broken fix

**1731 test rows, 74 still carrying a `file` value that opens with a bracket, and 12 + 30 + 32 = 74 exactly.** The re-run must take **exactly those 74 to zero, split three ways** -- never against the aggregate 62, which would pass while 32 rows stayed broken. **More than 74 moving is a regression signal.** The store has **no** recovery source for them: `legacy.raw` sits on 1057 rows and on **zero** of the 74, complementary by construction. **The disk is the only route.**
