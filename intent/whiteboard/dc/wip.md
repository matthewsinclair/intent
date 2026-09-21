---
node: dc
name: DevX Claude
role: worker
session_id: dfe2e58c-f5c6-4ebc-b5d4-bfea6e74dac1
heartbeat_at: 2026-09-21 07:54Z
status: active
focus: "Issues line (vc 07:31Z): 0494 STOPPED on D42, awaiting hv (a/b/c); 0495 built in wt-issues, whole suites to read then bank and report; 0496 walk banked, goes to vc with the 0495 report. Carries (doing 81) deferred past the cut. NO PUSH, NO RELEASE."
claims: [ST0056/11]
---

# DevX Claude (dc)

## DOING

- **The whiteboard carry of dc's three estates** -- Conflab (cc dc ic vc), Prolix (cc hv vc), arca_cli (cc vc). Replaces doing 80, whose after-steps ("then doctor, then organize") fell short of vc's decision 57. ic carries Baize, Molt, Riffle and arca_config, Riffle first as the rehearsal. WAITS ON ic's Riffle report and vc's word. hv's intentd restart is DONE (2026-09-21, running 510409ccc). **MIGRATE ORDER (decision 54):** hv first wherever an hv node exists, otherwise alphabetical: Prolix hv, cc, vc; Conflab cc, dc, ic, vc; arca_cli cc, vc. **hv's BOARD ON PROLIX IS CARRIED BY dc's HAND, NEVER `--drop-uncarried` ON hv's ROWS.** An uncarried or coerced hv row: carry everything else, commit nothing that drops an hv row, send the report to hv through vc with the rows quoted; hv decides. On every other node READ the uncarried report before `--drop-uncarried`, an empty one included, and keep dropped rows in that node's pre-migration copy (decision 24 Part B). **BEFORE, PER ESTATE:** re-read `git status` there; NEVER `intent wb status` first (`intent/.cache/` is gitignored, so on a store-less copy the verb writes a db); the discriminator is `intent/.canon/whiteboard/`, read-only; hash the rendered boards. Doctor-clean before a carry is NOT evidence (`views.rs:2203` skips an unmigrated board). **THE CARRY:** argument-free `intent wb register`, then `intent wb migrate <node>` in the order above. **AFTER, decision 57's three checks for two holes:** `intent wb show <node>` against the rendered file item by item (the only check that reads the store); the rendered boards hashed again against the before; doctor's COUNTED skew now the boards are migrated, the output read whole, not its count. **NOT CHECKS, and the report says so:** doctor-clean before, and doctor's store-stale line (its dump carries no whiteboard rows, 0495). **Organize is no door for a board**: a stale board view is landed by a board write, `intent wb touch --node <n>` the cheapest. Then one commit by literal paths in that repo, `git status` read after as well as before. NO PUSH. ic's board on Conflab is dc's to carry. arca_cli has no whiteboard README: nobody invents one, the commit names the absence, and the roster question goes to hv with the report.
- **RESUME HERE -- dc's issues line for 2026-09-21, on vc's order of 07:31Z (hv's plan: close the open issues and the last thread, then cut).** Worktree `scratchpad/wt-issues` (detached at f7d476d1b; its own in-tree target; HOME `scratchpad/wt-issues-home`; env in `scratchpad/wt-issues-env.sh`). NO PUSH, NO RELEASE; landings and rebuilds only on vc's word. **0494: STOPPED, disposition is hv's.** The forward note is at f7d476d1b and vc judged it clean: the ask collides with D42 (ST0056 design.md:393, :418, :426; one_clock.rs), and the Lamplight evidence does not fit (a 2.148s lock wait, but ceab7404a held the handled render 11.7s before the event was minted, with board.json `live` against inbox `(handled)` in one commit, cause unexplained). vc put (a) close as D42-by-design and re-file the anomaly, (b) keep it for the anomaly alone, or (c) reverse D42 to hv, with (a) recommended by both; the anomaly issue is filed only after hv chooses. **0495: BUILT, NOT YET JUDGED.** doctor's `db_checks` compares each migrated board in the store against board.json through `board_changes` and names the node in the store-stale advisory, which stays uncounted; it reuses `facade::BOARD_RERENDER_REMEDY`, now `pub(crate)`; CHANGELOG gains `## [Unreleased]` / `### Fixed`. The new arm `doctor_reports_a_board_row_its_file_lacks.rs` is green, and red at :66 with src reverted after both preconditions held; fmt (2024) and clippy are clean. WIP banked at `refs/bank/dc/0495-wip` e5e38e342 (base f7d476d1b). NEXT: read the whole-suite logs `scratchpad/0495-intentsvcs.log` and `0495-intent-cli.log` (script `0495-suites.sh`, loads logged), then bank `refs/bank/dc/0495` against the then-current main, read its path list, and report the blob, patch-id and every verdict line whole to vc. **0496: THE WALK IS DRAFTED, NOT SENT.** It is at `refs/bank/dc/0496/walk.md` dd16fe7c8. CLAUDE.md is stale TODAY (template e12e071d1 after the render at 11ce80ad5; doctor 0); AGENTS.md is in step but held by nothing (validate checks only its shape); only dispatch-table.md is held by a failing mechanism. The arm is `canon::apply` in report mode as a doctor ADVISORY, sized S for the arm and its test, M with a new FindingClass plumbed through. It goes to vc WITH the 0495 bank report, and hv rules whether it builds or closes on the walk. The `agents_sync_parity.rs:498` near-coverage sentence goes into 0496's body in the same chain as its first real change.

## TODO

- **hv's to push, nobody else's.** The tap README commit 0341f61 is ahead 1 in `/opt/homebrew/Library/Taps/matthewsinclair/homebrew-intent` (not under ~/Devel); it is hv's call, or it rides the next `int macos publish`. Prodinfra is ahead 3: 28044b9 (usage-rules.md names the project), 1ede442 (fleet trawl to 3.1.0) and 66e6ead (devbin runtime 0.1.2). dc does not claim the second two. All re-verified 2026-09-21 07:06Z.

## Holds

- **ST0056 AC-00.5 and AC-11.1** (claim ST0056/11). Released when hv has run intent/st/ST0056/gyges-brief.md on gyges and vc has judged `~/intent-clean-install.log`. Then dc runs `ac satisfy` for both.
- **The guards adoption pass.** Released when `bin/int hooks` reports a hook wired through the canon `.githooks/pre-commit.intent` shim AS WIRED, driven on one estate after its guards pass. It gates nothing in this release. Ownership of the hook fix is hv's call and vc is carrying it.

## Watch-outs

_(none)_

## Decisions

_(none)_

---

_Generated by Intent v3.1.0 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
