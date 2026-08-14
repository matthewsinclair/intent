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

---

## Third fold, 2026-08-14 15:11Z (EOD/EOW) -- aggressive lean-out

Folded at hv's instruction to cut hard for end of day and end of week. What follows was live on the board and is retired here; the live board keeps only what the v3 build era actually needs.

### DOING, retired

- **Suite GREEN at `46c6601`** (`bin/int test all`, 2026-08-14 14:42Z): 1244 bats, 0 failing; 12 rust, 0 failing; exit 0. The `+4` over the 1240 hv measured at `3563ff4` is exactly `ambient_project_root_guard.bats`, so the 0025 fix cost nothing elsewhere.
- **AC-02.1 SATISFIED** on the first green rust CI run (`31812129560`, macOS + Linux, `fmt --check` + `clippy -D warnings` + tests, 1m47s, on `736033d`). vc verified and recorded it. WP-02 went to 5/6.
- **0024 review closed by vc: sound, close stands.** Both notes actioned -- archive re-cited to `e685e90`, and `at_lint_wp_scope.bats:74`'s `grep -qv` replaced with a bare negated match at `8b7d382`, mutation-proven both ways.
- **WP-02 close claimed to vc** (`94dd922`). Gate reads `ST0056/02 BLOCKED -- 5/6 satisfied; unsatisfied: AC-02.6`, which is the state vc predicted; the AC-02.6 renumber into WP-04's group is theirs, since contract changes route through vc.

### The Lamplight sweep program, retired outright

The whole item is dead and is kept here only so nobody re-derives it. The estate was already at 2.19.0 with ~40 files in flight from a sweep their own nodes ran and four of their sessions live in it; their hv ruled AT remediation dead outright (`aaf4d3b2b`, widened at `7f5c0bd9a`), so the residue is the PERMANENT state of that estate. hv here then ruled closed-thread lossless-by-carrying as policy (`migration.md`), which supersedes the sweep question entirely for v3.

- **The baseline keeps its value as v3 migrator input, not as a pre-sweep snapshot**: `intent/analysis/20260814-lamplight-at-sweep-baseline.md` (vc, Lamplight `15dbccc92`) -- 1639 AT rows, ~1158 (70%) in shapes `--fix` must refuse: 975 `::name`, 508 multi-file `+`, 325 both. WP-10's fixture cannot be "the post-sweep trees", because there will not be any.
- **The after-conditions method, if any estate-rewriting sweep ever runs**: row count must not fall; `::name` counts may fall only where the name survived into a trailing note; the backticked-reference count must not fall at all. AT-row-scoped (`^- AT-\d`) via a Python pass, NOT line greps. Two-pass split with vc: cc's counts immediately after as the stop condition, vc's independently afterwards as the record.
- **Lamplight's own backlog, theirs to sequence**: ~1158 rows needing the two-ended migration; and the bad-status item is only **9** AT rows out of vocabulary (`green.` x8 with a trailing full stop, one `:degraded`). ST0298 / ST0270 / ST0198 carry `BUILT` / `Done` / `WIP` on **AC** rows, not AT rows -- a different state model, outside `at lint` L1, and not to be reported to them as an AT problem.
- **Utilz and Baize were never verified.** If either is ever picked up, taking the baseline is part of the sweep, not a preliminary to skip because the estate looks small.

### Watch-outs retired with the v2.19.0 release episode

- **A sweep that rewrites the estate must be measured against git before it is trusted.** The U1 `--fix` sweep destroyed 87 test names in this repo's own contracts and nobody noticed for a day; recoverable only because `f28938c^` still held them.
- **The gate blocks unswept estates from the day v2.19.0 ships.** Every named row was already contributing no coverage, silently. The CHANGELOG says so explicitly -- do not soften it when a consumer complains.
- **`intent upgrade` short-circuits when the project is already at the target version** (`intent_upgrade:107`). The fix reached consumers because v2.19.0 IS a version boundary, NOT because upgrade re-provisions canon unconditionally. Any future canon-only correction needs a ledger step with a real state probe.
- **AGENTS.md convergence must stay AFTER the canon apply** in `bin/intent_upgrade`, never as a ledger step. Canon creates `usage-rules.md`, which AGENTS.md's own file map lists. Verified by running it, not by reading.
- **The full timestamp finding**, kept for the record now that the live board carries only its one-line rule: I first blamed clock skew between sessions; vc identified the real mechanism as local BST stamped with a `Z`. I then recorded that my stamps "came from `date -u` and were true" -- right for my heartbeats and **wrong for my inbox entry headings**. At the 14:37Z fold, true UTC read `14:37Z` while that afternoon's headings read 12:55 / 13:05 / 14:05 / 14:30: BST, an hour ahead, unmarked. Same failure as vc's, in the file next door, written while I was describing vc's. **Not retro-corrected and must not be** -- every cc entry heading before `2026-08-14 14:37Z` is unverifiable for ordering.

### Decisions absorbed into compressed entries or retired

- **A number in a note decays exactly like a green claim.** "314 AT rows" was wrong by 5x; "Full suite GREEN at HEAD" was wrong by three commits. Both were true when measured and neither carried what it was measured against. A measured figure must name its subject and revision or it is a rumour with a decimal point. Complement from vc: an unscoped grep returning an implausible number ("30+ distinct statuses", counting AC rows and prose) is a finding to CHECK, not a finding. _(Absorbed into "a record names what it covers".)_
- **A mutation that fails to produce an expected red is itself the finding.** 0024's test 5 asserted a negative as `grep -q "..." && false || true`, whose trailing `|| true` swallows the failure so it can never go red. It passed the baseline and then passed M1, a mutation that should have killed it. _(Absorbed into the single mutation-discipline entry.)_
- **A test that passes is not a test that works.** Seven guards this release would have guarded nothing, every one caught by mutation and none by review: an invalid ERE whose error `|| true` swallowed; a bats helper called inside `bash -c`; an assertion matching the fixture's own prose; a scope test whose decoy could never have been selected; a probe matching an unrelated comment; a probe hitting usage text instead of code; and one asserting the defective behaviour AS the contract. _(Absorbed into the single mutation-discipline entry.)_
- **A verifier of results may not state conclusions about mechanism.** `assert_written` checks the post-state and then claimed "the file was NOT updated" -- false in the case that mattered. Its pre-write permission siblings keep those words, because a write refused before it began genuinely did not happen. _(Retired: the fix landed; the rule is now in the code's own comments.)_
- **Report the findings you killed, not just the ones you kept.** Checking `RELEASE_NOTES.md:7` and saying so cost three lines and saved vc a re-derivation; a finding already refuted is worth as much to a peer as one that survived, and it stops the same line arriving again from a third node. Reciprocal with vc. _(Retired: now routine practice on both boards.)_
