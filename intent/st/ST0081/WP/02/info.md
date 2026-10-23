---
wp_id: WP-02
title: Linux assets on each tag's GitHub release, one writer per release object
scope: S
status: Not Started
---

# WP-02: Linux assets on each tag's GitHub release, one writer per release object

## Objective

Each release tag's GitHub release carries the Linux assets beside the macOS ones, with one writer per release object. `bin/devbin release check` warns on a workflow that has a `tags:` trigger and calls `gh release`, an upload included, so there are two candidate shapes: the local release flow downloads the workflow's artifacts and uploads them after it creates the release, which changes devbin and so is the Devbin project's to build; or a workflow with no `tags:` trigger, dispatched once the release exists, uploads to it and never creates one. The chosen shape is written into `intent/docs/releasing.md`, and `bin/devbin release check` names no second writer.

## vc's note of 2026-10-23: hv's D55 ruling, and the remotes declaration folded in

D55 (hv, in the TUI, between vc's clock reads of 12:03Z and 12:09Z): devbin's release core attaches the CI-built Linux assets to each tag's release, SHA256SUMS riding the same change; the by-hand upload is the interim. The Devbin half is devbin-vc's, in the session hv starts today; this package holds on it.

THE INTENT HALF, cc's, specified before built. (1) The `release.remotes: [local, upstream]` and `release.repo: matthewsinclair/intent` declaration hv said yes to is folded into this package rather than landed alone: cc measured that nothing in Intent reads a `release:` block today (Intent's cut, bin/.devbin/cmd/build.d/release, pushes local then upstream by name and refuses unless both exist; devbin's release core is not enabled here, and `bin/devbin release check` exits 2 on that ground before any key is read), so a declaration landed alone would be a line nothing reads, which is what restart.md's old remotes line was. (2) AC-02.1's instrument, `bin/devbin release check`, runs only when devbin's `release` command is enabled, and devbin enables it whole (commands.release.enabled), which opens a second cut door, `bin/devbin release --patch|vX.Y.Z`, beside Intent's own `bin/devbin build release`. The package decides how the instrument runs without that door: a Devbin change that lets a project enable `check` alone (in devbin-vc's brief), or Intent's own `int` arm calling check_release_writers. cc builds this half when the Devbin half exists; the package stays Not Started until then, and AC-02.1 is proven at the 3.2.6 cut.

## cc's spec of the Intent half (2026-10-23, read-only, measured at 6d21e0659; accepted by vc and relayed to devbin-vc)

(1) ROUTE B IS ALSO A DEVBIN CHANGE: check_release_writers is defined inside bin/.devbin/lib/cmd/release (:172), a command script that parses "$@", cds and loads config at top level, so an `int` arm cannot source it without running the command, and an Intent-side copy would be a second home for the check, which Highlander refuses. Route A, a project enabling `check` alone (devbin's cut, `status` and `notes` left off), is the ask in devbin-vc's brief. (2) THE RELEASE OBJECT'S WRITERS TODAY are all local and run in sequence by one operator: `bin/devbin build release` creates it or edits its notes (cmd/build.d/release:1536-1541); `bin/int macos publish` uploads the macOS assets (cmd/macos:1455), creating the object only when absent (:1465); hv's hand uploads the Linux packages (the interim). linux-release.yml writes nothing, and check_release_writers flags only a tag-triggered workflow that also runs `gh release`, so "no second writer" holds as long as the Linux assets arrive through a local step. (3) WHERE THE ATTACH SITS in Intent's cut: after `int macos publish` and before `int brew formula --push`, because the formula's on_linux urls need the Linux packages' hashes (today releasing.md's "--push at a terminal if the Linux packages came later" step). The tag's packages must exist first, either built by a cut-dispatched `gh workflow run linux-release.yml --ref vX.Y.Z` or by a tag-triggered build-only workflow with no `gh release` in it; the Devbin half's attach decides which. (4) SHA256SUMS: `macos stage` writes SHA256SUMS.txt over the macOS artefacts (cmd/macos:1081-1132) and publish uploads none of it; under D55 it rides the attach, and whether one file covers every asset or each platform keeps its own is the Devbin half's call. (5) INTENT'S CHANGES once the Devbin half exists: re-vendor devbin, judged by a macOS bats run; add the `release:` block (`remotes: [local, upstream]`, `repo: matthewsinclair/intent`; cc's patch is kept) and enable `check` alone; call the attach from the cut sequence; update releasing.md's step and retire the by-hand interim; a CHANGELOG line. (6) AC-02.1's EVIDENCE at the first cut after both halves land: `gh release view vX.Y.Z --json assets` naming the Linux packages beside the macOS assets and SHA256SUMS, plus `bin/devbin release check` output with no second-writer WARNING; whether that cut is 3.2.6 depends on the Devbin half and the re-vendor landing before it.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0081/acceptance.md`, under the `WP-02` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
