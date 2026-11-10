---
st_id: ST0091
title: Intent releases through devbin's core: its driver, kovacs preflight and pre-push build retire
status: WIP
created: 2026-10-31
completed:
---

# ST0091: Intent releases through devbin's core: its driver, kovacs preflight and pre-push build retire

## Objective

Intent cuts its releases with devbin's release core instead of its own driver, so one command takes the yes to a brew-installable artefact and no suite or build runs twice for one commit. This is Devbin ST0011's WP12, Intent's line in that thread's per-project remediations. devbin-vc ruled on 2026-10-31, under hv's pen, that this thread in Intent's store is the one place that work is written and claimed; Devbin's WP12 points here.

Concretely:

1. `bin/.devbin/cmd/build.d/release` retires into the core's cut. `release.verbs` gains `cut`; `derived:` carries the schema bless, the lock refresh, `agents sync`, `claude upgrade --apply` and `sync --to-disk`; and a live intentd refuses the cut and its dry run at the core's pre-flight, before anything is stamped. Intent's own checks are declared in `release.gates` rather than made core steps: the bats suite, the cargo gate with 0481's daemon-family re-run, the ST0056 contract and reference checks, the provenance check, and the pair build at the release commit (WP-01).
2. `bin/.devbin/preflight-linux.sh` and `bin/.devbin/cmd/shared/preflight.lib` retire into `release.legs`, with kovacs as the leg. `bin/.devbin/cmd/prepush` loses its preflight cover and its clone-and-build, and keeps its frozen-remote refusal (WP-02).
3. `cmd/brew`'s render and publish retire into the built-in formula, which renders from templates under `packaging/homebrew/`. Intent keeps its own `cmd/macos`, its asset names and its packaging: the vendored core's macOS publish ships other assets and signs before the tag, so it is not adopted until a devbin stages after the tag. The cut publishes through one `after:` command that builds the pair at the tag, stages, publishes the macOS assets and then the formula, stopping at its first failure, and Intent's brew smoke follows it (WP-03).
4. `tests.yml`, `rust.yml` and `linux-release.yml` fire on `candidate/` refs and not on tags, so every leg, the macOS legs included, is green on the commit that ships before it is tagged. The five hand commands after the tag stop, and `intent/docs/releasing.md` and `.github/workflows/README.md` describe the cut as built (WP-04).
5. Each package is judged and landed on top of the vendor commit that brings the new core into Intent's tree. hv cuts the first release through the core, and that cut's RELEASE seal is the re-read of the 3.2.8 baseline.

## Context

**Why.** Devbin's ST0011 diagnosed the release cycle of every project that vendors devbin's release core (hv, 2026-10-31), and found the time goes to repetition rather than to the core's own steps. Intent's 3.2.8 cut on 2026-10-30 took 61m44s from the candidate's commit to the tap. The Ubuntu suite ran three times on GitHub, once on kovacs and once more in the driver on this Mac, and GitHub ran the release commit's workflows twice, once for the push to main and once for the tag. The tap commit went out 2m26s before the shipping commit's macOS CI finished, so no macOS verdict existed for that commit when the formula shipped.

**What was measured, read-only, on 2026-10-31.** dc inventoried every path the remediation names, with its callers, at 7ff46b458, and vc checked the inventory against the tree; it went to devbin-vc at 10:46Z by date -u. vc's baseline of the 3.2.8 cut, read from GitHub, the git reflogs and file times, went at 10:44Z. Intent has no RELEASE seal to read, because the driver writes none.

**devbin-vc's rulings, 2026-10-31, under hv's pen.** This thread is the one place Devbin's WP12 work is written and claimed in Intent; dc claims its packages, and Devbin's WP12 points here. The vendor commit is Devbin's WP09 sweep, committed by vc in Intent's tree, and these packages sit on top of it, so none starts before the sweep, and nothing slow or duplicated is fixed before it either. Of what must survive: 0634's cask sha and the brew smoke go to the built-in formula (Devbin WP08); the candidate is pushed to upstream only, found by its GitHub URL and never as the first remote (Devbin WP02b); the frozen-remote refusal stays in prepush; and 0481's re-run, the contract and reference checks and the provenance check are Intent's own gates, declared in `release.gates`.

**What the core as built changed, 2026-11-10.** dc read the packages again against the vendored devbin 0.1.20 core, vc checked the read in the source, and devbin-vc answered from devbin's main. The core runs no project command before the stamp, so the refusal of a live intentd needs a pre-flight seam. Devbin's next release carries one (devbin 0327), and Intent's first cut through the core waits for that release to be swept in. The same release carries a declared trailer for the release commit's message (devbin 0328), which Intent declares so that the commit keeps its (C) line. hv ruled both that day in devbin-vc's window, as devbin-vc relayed them to vc. The cask's sha256 is filled from the release's sums file, across formula and cask, and that fill stands for issue 0634's check. The brew smoke was never built into the core, so it stays Intent's, as an `after:` command that reports after publication and gates nothing. The pair build is a gate and not a derived step, because a derived step runs before the release commit exists. And the core's formula step refuses a lightweight tag, so Intent's release tags are annotated from this cut on.

**The core's shape** is Devbin ST0011's design.md: the recommended shape (section 5), Intent's line in the per-project remediations (section 7), the leg contract (section 11) and decisions D57 to D68 (section 12), among them D57 (a sealed green for a commit and platform is not run again), D62 (remote legs) and D64 (test workflows fire on `candidate/` and not on tags). This thread does not restate them.

## Work Packages

| WP    | Title                                                                                | Size | Status |
| ----- | ------------------------------------------------------------------------------------ | ---- | ------ |
| WP-01 | The cut runs through devbin's core, and Intent's driver retires                      | L    | WIP    |
| WP-02 | kovacs runs as the cut's remote leg, and prepush keeps only its push gates           | S    | WIP    |
| WP-03 | The tap and the macOS assets publish through the built-ins, with prepare as a shadow | S    | WIP    |
| WP-04 | The workflows fire on the candidate, and the docs describe the cut as built          | S    | WIP    |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
