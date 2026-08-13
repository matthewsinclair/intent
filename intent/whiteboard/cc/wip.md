---
node: cc
name: Control Claude
role: control
session_id: 347f2808-51b6-4c3c-90a5-3b43d41f5ecf
heartbeat_at: 2026-08-13T22:35Z
status: paused
focus: "v2.19.0 in progress: 6 of 7 units of vc's 0009-0017 work order landed (U1-U6). U7 (0010, warn on template Objective) then close-out. Nothing pushed; no tag. vc reviewing U1-U5."
claims: []
---

# Control Claude (cc)

## DOING

- **U7 -- 0010: warn at close (XS).** `st done` + `wp done` WARN, never block, when `## Objective` is still the verbatim template placeholder from `lib/templates/prj/st/ST####/info.md:14`. Placeholder string defined once with a drift-guard test against the template. Scope discriminator is the point: ST-level `## Objective` in `info.md` ONLY -- a sweep for any placeholder anywhere hits 72% of threads and would be disabled within a day. WP-level equivalent warns too.
- **Close-out (task 8).** Fill each issue's Resolutions (judgement calls + the two vc corrections verbatim), `intent issues close` each, CHANGELOG heading is already `## [2.19.0] - in progress`, hand to hv for the full suite + release cut. NEVER `bin/release --no-confirm`.

## TODO

- **Expect vc findings on U1-U5** -- hv said vc is reviewing them. Route: fixes land under the same issues, before close-out.
- **Consumer sweep (hv, separate repos):** `intent upgrade` per project. After v2.19.0 the same pass ALSO sweeps AT grammar, converges AGENTS.md, and rewrites settings.json to the portable hook form. Utilz / Lamplight / Baize.
- **Lamplight contract sweep (hv, separate repo):** ST0276 (11 bolded `**green`), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`, plus its 314 AT rows through `at lint --fix`.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- AT-name traceability deferral: SUPERSEDED by 0017 L3 (the id is the traceable token, not the `@test` name). Retire the entry at close-out.

## Watch-outs

- **The gate blocks unswept estates from the day v2.19.0 ships.** Every named row was already contributing no coverage, silently; `at lint --fix` does the mechanical half. The CHANGELOG says so explicitly -- do not soften it.
- **`intent upgrade` short-circuits when the project is already at the target version** (`intent_upgrade:107`, version equal AND no ledger step needs work). So "the fix reaches consumers" is true because v2.19.0 IS a version boundary for everyone, NOT because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe, or it will not reach a converged project.
- **AGENTS.md convergence must stay AFTER the canon apply** in `bin/intent_upgrade`, never as a ledger step. Canon creates `usage-rules.md`, which AGENTS.md's own file map lists; regenerating first leaves it stale exactly when canon changed something. Verified by running it, not by reading.
- **`bin/release` stamps all five sidecars BEFORE the tag** -- no manual post-tag wrap. Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. If it aborts on a dirty tree, read the list it prints.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case; a `claude` subcommand needs an explicit arm in `bin/intent`. Register in MODULES.md FIRST.
- **Do not use `git stash` for a before/after comparison in this repo.** It carries two pre-existing 2025 stashes; a pop conflicted and dumped 522 lines of long-pruned v2.2.1 migration code plus a stale `.intent/config.json` into the tree. Use `git show HEAD:<file>` instead. No work was lost, and both old stashes were left untouched -- they are hv's.

## Decisions

- (2026-08-13) **Mutation-test every guard before believing it.** Four checks written today would have guarded nothing: one asserted a substring that matched with the delimiter still present; one called a bats helper inside `bash -c`, where the function does not exist; one grepped for a word the fixture prose already contained; and one -- in the very file guarding issue 0016 -- used an INVALID ERE, so grep errored, `|| true` swallowed it, and it could never fail. That last is the exact defect class this whole release is about, written into the test meant to prevent it. Every guard in v2.19.0 has since been mutation-checked: break the behaviour, confirm the right test fails, restore.
- (2026-08-13) **Run the real path in a sacrificial copy; the dry-run path diverges.** Three defects surfaced only by exercising: `at lint --fix` compared the repaired path to the raw reference instead of the canonical form, so the commonest migration (a correct path merely lacking backticks) was skipped; the AGENTS.md ledger step ran before the canon apply and left the file stale; and the first upgrade convergence test was invalid because the fixture was already at the target version. Same standing lesson as v2.18.0, three more instances.
- (2026-08-13) **Three holes in issue 0017's proposed grammar, found by running the arms against real rows.** vc caught `path::name` (delta a). Measurement found two more: the parenthetical status note the shipped TEMPLATE itself taught, and -- structurally -- non-test rows, since `n/a` is documented in every contract preamble as the doc/eyeball status while the one-armed grammar demanded a path on every row, giving those rows no legal form to migrate to. Hence two arms and the enforced biconditional (n/a IFF non-test). Recorded in 0017's Resolutions.
- (2026-08-13) **An AC contract emptied entirely by descope/withdraw is REFUSED, not passed.** Not in issue 0013. Passing on an empty set would make the new verbs a trivial gate bypass; the refusal points at the existing `acceptance: exempt` declaration instead. ST0048's rule is that an exemption is announced, never inferred from emptiness, and a contract emptied one withdrawal at a time is still emptiness. Reversible in one line if hv prefers.
- (2026-08-13) **Node and bats deliberately stay on filesystem probes** (issue 0009). Node because Intent's declared vocabulary has no name for it, so gating on a declaration a project cannot make would delete the line forever. Bats because it is a test RUNNER, not a language -- `.bats` files are the right evidence, and declaring `shell` should not claim a project needs bats.
- (2026-08-13) hv added an AC-withdrawal verb by direct instruction, overtaking vc's `struck` deferral (deferred for want of field evidence; the request is the evidence). Each state-changing verb now carries its own audit payload: satisfy needs `--evidence`, descope needs `--to`, withdraw needs `--reason`.
- (2026-07-30) Two tooling fixes taken directly rather than filed, on hv's "just fix it": `bin/release` (every published tag was self-inconsistent) and `intent upgrade` (the orchestrator did not converge the tool-managed Language Packs block). Both verified by EXERCISING them in sacrificial copies, which turned up defects a dry run could not show.

## Open -- hv rulings owed

- **Redundant hook script copies.** After U5 the per-project `.claude/scripts/{session-context,require-in-session}.sh` are unreferenced: settings.json calls the installed tool. Editing a local copy expecting it to take effect now silently does nothing -- the same trap class this release removes. Pruning them deletes files from consumer trees, which is hv's call, so it is recorded and not actioned.
- **Tracked absolute home paths outside the guard's scope.** 42 tracked files carry `/Users/matts`; most are `intent/.treeindex/**` (written by `intent treeindex` by design) plus CHANGELOG/analysis prose. The U5 guard covers config that FUNCTIONS (`lib/templates/.claude/`, `.claude/`, tracked only). A tracked machine-specific cache in a public repo is a real question and hv's to answer.
- **A `javascript` language pack** would complete 0009 (see the Node exception above).
- **Issue 0004 item 4** (`ac status` exit code) -- premise does not reproduce; `status` = reporter, `gate` = gate. Own issue if hv wants it changed.
