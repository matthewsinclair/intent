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

---

# Second fold, 2026-08-14 (post-tag) -- archived from the live board

The release shipped during this session. Everything below was live DOING or a closed
question at the time of the fold; it is here because it is finished, not because it
stopped mattering. Watch-outs and Decisions deliberately stayed on the live board.

## DOING (all complete -- v2.19.0 is cut, tagged and pushed)

- **Commit the board so the tree is clean for the cut.** Landed as `e709149`. The
  hazard behind it: `bin/release` runs its leftover-dirt check at `:437-447`, AFTER
  stamping and committing the five sidecars, so anything dirty outside them costs a
  `release: vX.Y.Z` commit with no tag rather than an early abort. Every node pickup
  writes a heartbeat, which makes the board itself the likeliest offender. Kept as a
  standing watch-out.
- **Reported to vc before the cut: `intent/wip.md` was half-swept by `e1e2300`.** The
  count moved to fifteen and the enumeration did not -- it named eleven and stopped at
  0021 -- and "0020 and 0021 were both called in by hv" had become four. vc's reading of
  the second point is the better one: two reads as an exception, four reads as the
  batching principle.
- **The sharper half of the same report: "Full suite GREEN at HEAD (post-0020)" was
  false at HEAD**, with three code commits (0021, 0022, 0023) postdating the cited run.
  It turned out to be in FOUR documents, not one -- `intent/wip.md`, `intent/restart.md`,
  `.claude/restart.md` and `intent/done.md`. Fixed at `dde7b59`; all four now name the
  commit the run covered and say what stands behind HEAD. Generalised into a Decision.
- **One finding checked and dropped rather than filed:** `RELEASE_NOTES.md:7` ("fourteen
  other fixes") is fifteen issues minus the 0017 centrepiece, and `e1e2300` did touch the
  file -- so the number is the swept one and it is correct. Reported as a negative so vc
  would not re-derive it.

## Closed by vc under hv's pre-cut batching (three of the four in cc's lane)

- **0020** `2769c40`: `st list --status all` membership through `normalise_status`; ten
  literals collapse to five canonical tokens; unplaced rows emitted last and named on
  stderr; exit stays 0, because escalating would break index regeneration on exactly the
  estates that have the problem. Guard mutation-proven M1-M5.
- **0021** `3949f56`: `st zero` D5a removed -- Intent had been shipping a second, dead
  Elixir enforcement mechanism (six custom Credo checks, copied unconditionally, wired
  best-effort, usually loaded by nothing). `doctor` check 4e reports consumer residue in
  three states and quotes the `elixirc_paths` lines, because deleting the directory alone
  breaks their build.
- **0022** `08ef2f5`: both no-template fallback heredocs DELETED rather than corrected --
  this board's "adjacent, not fixed" item, executed. Correcting them restores two copies
  and buys another year of drift. Consequence: the two 0010 drift guards are now inverted,
  from "the constant still matches the second generator" to "there is no second generator".
- **0023** `e1e2300`: `error()` and its 25 imitators speak the documented lowercase voice
  -- the other half of `8aba5ab`, and the worse half, because the one function whose whole
  job is to give failures a single voice was setting the wrong example. 26 sites, six
  files. The twelve test assertions pinning the old string were found by sweeping for them
  BEFORE the change rather than by watching them fail. **Named and deliberately left:**
  every `Error:` echo in the three plugin bins goes to STDOUT, which changes what callers
  capture rather than merely what they read -- same class as the 0019 silent-sync failure,
  now queued beside `intent_claude_prime:212`.
- **Release docs written PRE-cut** `86cdbe1` + `62e8e24`: `intent/history/v2.19.0.md` and
  `docs/releases/2.19.0/RELEASE_NOTES.md`. Both practices had lapsed (history after
  v2.16.0, releases after 2.17.0), resumed here and NOT backfilled.
- **"vc's residual 1" refuted** by vc and by cc independently, same day, same conclusion:
  0019 removed the mechanism, so no Created value travels from the `st done` call site at
  all. vc's method settled it (a scratch `st new` -> `start` -> `done` showing the row comes
  out right); cc's only showed the value cannot travel.

## Record correction made in passing

The dead `CREATED` block is at `bin/intent_st:696`, not the `730-743` this board carried
nor the `731-741` in 0020's Resolutions. Both were read before `2769c40` and `08ef2f5`
shifted the file. Both records now anchor on the comment string instead of the number.

---

## Archived at fold, 2026-08-14 14:37Z -- the afternoon session (devbin + 0024 + 0025 + the SDL face)

Everything below was DOING and is now done. Kept for the record; not reloaded on pickup.

- **0025 CLOSED, fixed properly.** `resolve_project_root` in `intent_helpers` is THE project-root authority -- it ASSIGNS from the filesystem, overwriting anything inherited, and is registered in MODULES.md as the seam every reader comes through. `require_project_root` now resolves before refusing rather than testing a variable. The three plugin bins that never resolved (`subagents`, `prime`, `upgrade`) do so at load, which matters because `bin/intent:187` execs plugin commands BEFORE loading config. `bin/intent` clears an inherited value at entry so a future reader that forgets to resolve fails SAFE (empty -> honest refusal) rather than dangerous (a stranger's tree).
- **The mutation matrix is on the issue, including the one that killed nothing.** Removing the dispatcher scrub alone reds NOTHING -- every reader that exists today also resolves -- so the scrub is deliberate fail-safe cover for readers not yet written, and the record says so rather than implying it was proven. Only removing BOTH mechanisms reproduces the original defect.

- **devbin adopted (`bin/int`), and `bin/release` is now `bin/int build release`.** `bin/intent` is untouched and cannot be touched: devbin's `link_alias` refuses to replace a real file. `bin/in` was the estate-consistent alias and is impossible -- `in` is a bash reserved word, a syntax error as a command in bash while working in zsh. The three commands hv asked for all work: `test all`, `build cli`, `build release`.
- **Suite green: `bin/int test all` -> 1240 passing, 0 failing, exit 0, at `3563ff4`** (rust + shell legs). Named against the commit, not "HEAD" -- the same run before the 0025 fix had 72 failures by test 830.
- **The rename half is already committed, in vc's `072d277`** -- they used `git add` + bare `git commit`, which takes the whole index including what I had staged. History reads oddly; the rest of devbin is in my own commit. Nothing of mine was lost.

- **WP-02 SDL face DONE (`732affa`).** The third committed face, exported from the same master -- model types carry SimpleObject/Enum derives beside their schemars ones, so a new field reaches the SDL with nobody remembering. One unavoidable projection (`AcScopeView`: GraphQL cannot express a tagged enum with per-variant fields), guarded from both ends and mutation-proven four ways. AT-02.2 green; AT-02.3/4/5 flipped from a stale `to-write` to green. **Next: WP-03 (ingest, views, sync engine).**
- **ST0056 build lane, taken from vc per hv (their 13:05Z).** vc stewards the thread and holds the contract; ic has the parity deep pass; cc writes the code. WP-01 is Done and hv-ratified; WP-02's foundation landed at `5e4b766` (cargo workspace, model types as the authored master, store with D01 as law, committed faces + INTENT_BLESS drift workflow, CI, four mutation-proven guards). **Mine next: the SDL face** -- a minimal async-graphql schema over the types, added to `faces()` + the drift test. Then AC-02.1 flips on the first green CI run after push, and AC-02.6 (event-log envelope per mutation) needs a call at review: its AT stays red until WP-04, or the AC descopes there. Then WP-03.
- **0024 CLOSED (`e685e90`)** -- fixed, guarded, mutation-proven, CHANGELOG'd as 2.19.1 in progress. Announced to vc before touching `bin/**` per the standing agreement. _(Corrected 2026-08-14 from `1f5e354`, on vc's finding: that was the pre-amend twin -- tree-identical, same parent, same date, never pushed and unreachable from main, so the citation resolved nowhere for anyone reading from GitHub. Nothing substantive changed; the hash did. Recorded rather than silently swapped, same rule as the fabricated stamps above.)_
