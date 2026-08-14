---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-14 22:19Z
status: active
focus: "Folded for a compact mid-session. WP-03 and WP-04 CLOSED, WP-05 3/4. hv: START BUILDING on wake -- do not wait on vc. Next is WP-06, work-list below."
claims: []
---

# Control Claude (cc)

## DOING

- **Folded for a compact, not for the day.** `status` stays `active`: a compact does not end a session (protocol invariant 6). The day's detail is in `.history/20260814/`.
- **Estate: v3 builds and runs.** WP-03 CLOSED 6/6, WP-04 CLOSED 5/5, WP-05 3/4 at `9a45340`. 158 tests / 25 targets, fmt and clippy clean. `intent st new` writes canon + four views + DB; `ac gate` puts its verdict on stdout and exits 1; `st done` refuses through the gate.

## TODO -- on wake, in this order

**hv's standing instruction (2026-08-14, at the fold): START BUILDING. Do not wait on vc.** WP-05's remaining AC-05.3 is ic's register and is not a blocker on cc's hands -- vc gated WP-06 behind it, but hv has overridden that sequencing for cc. Build; claim as usual; let vc verify behind.

1. **WP-06 -- the CLI parity long tail.** Opening work-list, measured rather than guessed (`tests/conformance/BASELINE.md` bucket 3b): **`st repair`, `st sync`, `st edit`, `wp show`** are dispatched by the spine and not wired to the facade. Then the remaining 21 families, which currently parse and answer `not yet wired to the facade (WP-06)`.
2. **AC-06.2: doctor**, rebuilt as model/DB integrity queries plus file checks -- it consumes the WP-03 primitives that already exist (`views::skew`, `sync::scan`'s unparsed state, `ingest::refresh_index`).
3. **Contract rows are vc's, not mine.** Do not hand-edit `acceptance.md` and do not run `intent at set` into a file a peer has open. Claim, and let them land the rows.
4. **The WP-10 residue risk wants a MEASUREMENT before any policy ruling**: an estate whose ATs cite files that have since moved BLOCKS on L2/L3 at migration. `intent at lint` over each corpus member at its named revision gives the number cheaply. vc has deliberately not pre-ruled the carry policy until that number exists.
5. **The todo watermark needs a durable home.** `DONE:<T>` is DATA (`bin/intent_todo:20-21`, read back at `:159`, advanced only by `done --flush`/`--prune`), carried today as `RenderContext::todo_watermark`. Materialise it in project config; v2's "else start-of-today UTC" fallback cannot survive the no-clock law.
6. **`installed-agents.json` is untracked AND unignored** -- `intent/plugins/claude/subagents/.manifest/` tracks its sibling but not it, and `.gitignore` names neither, so `intent claude subagents install` leaves a permanent `??` holding absolute machine paths. Pre-existing. Wants an issue; check the consumer estate first, since a rule that only fixes this repo is the wrong shape.
7. **Push the two local-only fleet commits**: Utilz `0171297`, Lamplight `7058fd3a8`. **Re-verify both are still unpushed at the moment of acting.**

## PARKED -- v2 maintenance is default-defer (hv, 2026-08-14)

Show-stoppers only; **0025-class suite-blockers are the whole exception**, and that is the test a candidate must pass -- not "is it small". Decided, not forgotten; do not re-raise as new findings.

- `Error:` on STDOUT in the three plugin bins (0023's Resolutions) and `intent_claude_prime:212`. It changes what callers CAPTURE, not merely what they read.
- The inert per-project `.claude/scripts/*.sh` copies; a `javascript` pack to finish 0009's Node exception; issue 0004 item 4 (`ac status` exit code -- the premise does not reproduce, so it wants a close ruling, not work).
- The dead `CREATED` block in `intent_st`'s in-progress arm. Anchor on the comment `# Extract created date for index update`, never a line number.

## Watch-outs

- **NEVER mutate `bin/**` in place while anyone else is live.** `~/.local/bin/intent` symlinks INTO this repo, so every project on this machine runs whatever state the file is in at that instant. Sacrificial `git worktree` only, and pass `INTENT_HOME` explicitly or the harness silently measures the live tree instead.
- **Announce before holding the repo-root `tests/` estate**, with a duration; ic's register sweep runs there. `crates/**`, `schema/**` and `surface/**` are cc's lane and need no announce.
- **`git commit --only <paths>`, never bare, never `-A`.** Three nodes share this tree and the pre-commit formatter stages its own rewrites.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes, and a pop once dumped 522 lines of long-pruned migration code into the tree. Use `git show HEAD:<file>` or a throwaway worktree.
- **The markdown formatter is a second writer and it wins.** `prettier --write` runs on staged `.md` in pre-commit. Generated views must emit what it would emit -- column-padded tables, no trailing space, one blank line before the banner. Authored `*emphasis*` is the one case the renderer cannot stabilise (it becomes `_emphasis_`); asserted as a named gap until AC-07.6 lands the formatter exclusion.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp**, including the second one in a turn. Board stamps are not a cross-node ordering -- use commits. cc entry headings before `2026-08-14 14:37Z` are unverifiable and deliberately not retro-corrected.
- **The v3 binary is `target/debug/intent`**; the conformance harness defaults to it and burns in against `/usr/bin/false` first. A conformance file that stays green with a broken binary never reached `INTENT_BIN` and its result is not evidence.

## Decisions -- working method for WP-06 and after

The rest of the day's decisions are in `.history/20260814/`; the ones already ratified as canon are `design.md` D22-D27.

- (2026-08-14) **Classifying by the SHAPE of a failure is a guess that looks like a finding.** I read 35 conformance failures as "not yet built" because the assertion was `assert_success`, put it in a delivered baseline, and sent it. Reading the tests showed most were the ratified manual-edit-workflows class -- 26 of 54 and 23 of 30 hand-build a v2 estate in their own body. **Only reading the test says why it failed.** And checking it surfaced the defect that had CAUSED the misclassification: the renderer answered an unwired verb with "a command is required" when a command had plainly been given. **A misleading message produces wrong analysis downstream; the defect sits upstream of the error, not beside it.**
- (2026-08-14) **Test against the incumbent, not against your memory of it.** Eleven fixture tests agreed with each other perfectly and were all built on what I believed v2 does. One test that RAN v2's binary found two enforced gate rules v3 had no defence against. **A fixture asserts what you believe; a differential asserts what is true, and only the second catches a thing you never knew existed.** Directly applicable to every WP-06 verb.
- (2026-08-14) **Running it beats testing it, when the suite has only ever driven one half of the behaviour space.** 157 green tests, every CLI test on an error path, so the binary had never been asked to succeed. Running it by hand found three defects including a fresh project's FIRST command failing.
- (2026-08-14) **Two rules blocking the same case is fine; two rules that cannot be told apart is a defect.** Assert the DIAGNOSIS, not just the block. Same-text-for-different-causes is a bug at every layer, not only in the error type where an AC names it.
- (2026-08-14) **When the contract and the narrative disagree, the contract governs; when two ratified statements disagree, the reading that makes another one UNSATISFIABLE is the wrong reading.** A narrative is written before the work; a contract is written to be checkable.
- (2026-08-14) **Determinism has two independent enemies.** INSIDE: the renderer reaching a clock -- killed structurally. OUTSIDE: a second writer downstream (the formatter). Both end as "regenerate-and-diff is non-empty", and a check that cries wolf is a check nobody reads.
- (2026-08-14) **A purity law holds because the impurity has ONE NAMED HOME at the edge**, never because everyone remembers to be pure. `today()` at the CLI's outermost layer is why D23's no-clock renderer is enforceable rather than aspirational.
- (2026-08-14) **Mutation discipline, in four clauses.** (1) A test that passes is not a test that works -- break it, watch the RIGHT test fail, restore. (2) Write the expectation BEFORE the run; an unexpected green is investigated as hard as an unexpected red. (3) **Applied is not reached** -- the canary must come from the same fixture and branch the test drives. (4) **A battery needs controls that stay GREEN**, or it has only proved the mutations broke something. Corollary: the harness must hard-fail when a substitution matches nothing.
- (2026-08-14) **A record names what it covers -- the commit, the subject, the revision -- never "HEAD".** A stale green is cheap while it is redundant and expensive at the single moment it is not.
- (2026-08-14) **Verify the premise at the moment you act, not when you queued it**, and **a peer confirming your finding by re-running your path is not corroboration** -- that is a second execution of the first route, not an independent one. Both fired today; both were caught.
- (2026-08-14) **When a path moves, the unit of work is every reference in the ESTATE, not every reference you were told about.** vc's move list had three items; there were five. A `grep -rn` over the old string is the only thing that closes the set.
- (2026-08-14) **A tool that cannot finish a job must not start it.** `at lint --fix` half-migrated rows. A lossy fixer damages what it touches; a lossy suggestion damages everything touched after it. Carries straight into the v3 migrator.
- (2026-08-14) **Grep for a Highlander rule; do not read for it.** A guard scoped to what is already clean certifies the status quo -- widen the needle until it would have caught the bug, then fix what it finds.
