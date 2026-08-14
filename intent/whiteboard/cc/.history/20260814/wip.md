# cc -- archived 2026-08-14

v2.19.0 finished: Unit 7, vc's five audit findings, the docs pass, ten issues closed, and three consumer-reported repairs to `at lint --fix`. Handed to hv for the suite and the cut.

## DONE

- **U7 -- 0010** (`827ab43`). `warn_unedited_objective` + two placeholder constants in helpers; `st done`/`wp done` warn and never block; the drift guard asserts each constant still matches BOTH templates and BOTH no-template fallback heredocs.
- **vc F1 + F2** (`9a74b4d`). Arm-aware refusal in `cmd_at_set` before any write; `ac_refuse_if_offscope` given one home and three callers; the write verifier stopped claiming the mechanism it does not check.
- **vc F3 + F4 + F5, and two more** (`69c93bc`). Probe the mv failure before blaming a collision; todo's done walk repointed; the guard pointer made checkable from both ends. The mechanical guard written for F4 then found `intent info` and `intent organize` still hand-rolling the enumeration -- missed by U3 AND by the audit.
- **Docs pass** (`87815be`). working-with-llms D11 was teaching the retired `path::name` form outright; D1 gained the declared-languages rule; the whiteboard section called the header block "frontmatter" and never described its format. usage-rules.md had no `ac`/`at` section at all. DEPRECATIONS.md records the retired forms.
- **Issues 0009-0017 closed with Resolutions** (`a96fc29`). Both vc corrections verbatim; judgement calls named as judgement; four mistakes recorded.
- **0018 folded in on hv's direction** (`409ace5`). 87 files untracked + ignored; the rule reaches consumers through the canon `.gitignore` seam; a tracked consumer cache is REPORTED with the exact `git rm`, never untracked for them.
- **`at lint --fix`, three consumer-reported repairs.** `be24f23`: a full-tree scan per row on a 65GB estate (4s a row), a glob-injection through `find -name`, and exclusions that filtered without pruning. `6f70d4e`: it was LOSSY -- four rows lost a second real cited file, ~17 lost the `::"name"` that was the only statement of which test covered the AC. The root cause was the SUGGESTION, not the fixer: the lint line named one file and silently dropped the rest, so a human following it lost the same data.
- **`4f3b2cd`** -- corrected `output_width.bats` after bisecting hv's suite failure to vc's `ba52339`. Their change is right; the test asserted an incidental scope coupling.

## Decisions archived (all now permanently recorded in issue Resolutions + CHANGELOG)

- Three holes in 0017's proposed grammar, found by running the arms against real rows (non-test arm, the template's own parenthetical, `path::name`).
- An AC contract emptied entirely by descope/withdraw is REFUSED, not passed -- not in 0013; the refusal names the `acceptance: exempt` escape.
- Node and bats deliberately stay on filesystem probes (0009), for two different reasons.
- hv added the AC-withdrawal verb by direct instruction, overtaking vc's `struck` deferral.
- The not-YAML fork for the whiteboard header block (0012).
- The 0010 scope discriminator: `## Objective` in `info.md` only, because a sweep for any placeholder fires on most threads and gets switched off.

## Rulings closed today

- **Tracked absolute home paths** -- answered by hv, filed by vc as issue 0018, fixed in this release. 42 tracked files -> 24, all remaining ones historical prose deliberately left as the record.
- **The `warning()` capital voice** -- flagged in 0010's Resolutions, taken by someone else in `8aba5ab`.

## Inbox

vc's work order (2026-08-13 20:59) and their U1-U5 audit (22:36) are both fully handled and can be cleared next session.
