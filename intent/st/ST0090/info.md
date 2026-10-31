---
st_id: ST0090
title: The index rebuild says what happened, in the CLI's report and Intent.app's menu
status: WIP
created: 2026-10-31
completed:
---

# ST0090: The index rebuild says what happened, in the CLI's report and Intent.app's menu

## Objective

Rebuilding the search index tells the person who ran it what happened, in the CLI's report and in Intent.app's menu, so that a clean run never reads as a crash (issue 0664).

Concretely:

1. One function in intentsvcs names a language's level-3 state from its stored run and the paths gone stale since: `stale` when a run stored and some of what it stored has gone stale, the extractor's move included; the run's own `missing` or `failed` whatever is stale; `current` only when a run stored and nothing it stored has gone stale. The search envelope's `ResolutionState::of` and intent-cli's `resolution_lines`, which `intent index status`, `intent index rebuild` and `intent index resolve` all print through, both ask it, so no face can name one run two ways (WP-01, cc).
2. `intent index status --json` and `intent index rebuild --json` name that same state for each language, and the text still lists every stale path, as its deliberate rule says (WP-01, cc).
3. `intent index status` and `intent index rebuild`, text and JSON, name when the index was last reconciled, read from the store's own stamp and never from a clock at render (WP-01, cc).
4. Intent.app's Rebuild Search Index row copies Gtools' Refresh Search row: its second line says how long ago the index was reconciled, in Gtools' age words, from the CLI's stamp; a rebuild that fails raises an alert as Gtools' refresh does; every run is still recorded in the app's log, and a rebuild no longer opens the log by itself (WP-02, ic).
5. Each package lands alone with its own judge run, one build carries both, vc tells hv which build that is, and the release stays hv's.

## Context

hv filed issue 0664 on 2026-10-31, at 07:08:12Z by its event's stamp: Rebuild Search Index in Intent.app's menu "causes a silent crash only visible when viewing the log", with the log's block pasted into vc's session. vc measured it the same morning, and nothing crashed:

- Every rebuild the app has logged since 2026-10-22, six of them, exited 0 in 21 to 24 seconds.
- No crash report exists for Intent.app or intentd. Both processes were the ones that had run since the evening before, and macOS's log holds no error or fault from the app across the last three rebuilds.
- The block's last line, its exit line, is the verdict: exit 0 in 23.8 seconds. Everything above it is the rebuild's report of what the index holds, and 243 of its lines listed the Rust files changed since rust-analyzer last resolved references, on 2026-09-17.

What made a clean run read as a failure is two defects and one design.

- **The report contradicted itself.** It named Rust's level 3 `current` on the line directly above `stale: 243`, while every search answer named the same run `stale`. Measured live before vc's resolve run: `intent search --json` gave `"state": "stale"` with `"stale_total": 243`, and `intent index status` printed `resolution: rust  current  rust-analyzer`. `resolution_lines` in intent-cli's `render.rs` prints the stored run's `state`, which records what the last run did; `ResolutionState::of` in intentsvcs' `search.rs` derives `stale` for a run that stored and has since gone stale. ST0076's AC-07.1 made one function own the search faces' level-3 words, and the index report was left outside it.
- **Intent.app says nothing about a finished run.** `runLogged` in `IntentApp.swift` records the block in Intent.app.log and then opens that log in the macOS Console, so the report is the only feedback, and the reader has to infer the outcome from its last line. Gtools' Refresh Search row, which hv's rule of 2026-10-22 makes the model for Intent's menu, opens nothing: its second line says how long ago the catalogue was built, and a failure raises an alert.
- **The report lists every stale path by design.** `resolution_lines`' doc comment says the stale paths are listed, never counted, because a reader needs to know whether the file in front of them is one of them. Six weeks without a resolution run made that list most of the output. vc refreshed Rust's references on hv's word at 07:28Z (run 2, stored at 07:28:53Z, joined 575 of 575 files, stale none), so today's list is empty. Any committed change to a Rust file makes that file stale again, and until WP-01 lands the report will again say `current` above it.

hv's ruling, in vc's session and read before 07:26Z by date -u: "Ok, do all of that. Raise it as a new steel thread, assigned to CC (core) and IC (UX/UI), elaborate all of the necessary details and instructions, and then check all of that in and hold." "That" was vc's plan: the report says `stale` whenever files are stale, the menu states each run's outcome, cc builds the Rust and ic the Swift, vc judges each landing without claiming it, a fix hv asked for lands alone with its own run and build, and the release stays hv's.

Out of scope until hv rules: shortening the stale list, which reverses the deliberate rule above; a remedy line under `stale` naming `intent index resolve`; and any change to Run Doctor, which keeps opening the log as Gtools' doctor shows its console.

Issue 0664 closes when both packages are Done and hv has seen the row on the build that carries them. Its title says crash, and retitling it is hv's call as its reporter.

## Work Packages

| WP    | Title                                                                          | Size | Status |
| ----- | ------------------------------------------------------------------------------ | ---- | ------ |
| WP-01 | One level-3 state on every face, and the index report names its reconcile time | S    | Done   |
| WP-02 | Intent.app's Rebuild Search Index row copies Gtools' Refresh Search row        | S    | WIP    |

## Acceptance

Acceptance Criteria and Acceptance Tests are RENDERED into `acceptance.md`, which is a GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in this thread's model, and the verbs write it: `intent ac new` and `intent at new` mint a row, `intent ac edit` and `intent at edit` reword or re-cite one, and `intent ac satisfy|unsatisfy|descope|rescope|withdraw|reinstate` and `intent at green|red|na` move its state. This cover never restates them.

## Related Steel Threads

- ST0076: ST0076's AC-07.1 made one function own the search faces' level-3 words; ST0090 brings intent index status, rebuild and resolve under it

---

_Generated by Intent v3 from the thread canon. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
