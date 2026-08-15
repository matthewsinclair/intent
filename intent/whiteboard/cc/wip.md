---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 00:59Z
status: active
focus: "WP-06 building. AC-10.7 + AC-03.7 + AC-06.4 closed tonight; st list ported to v2's table. Next: AC-03.8 (last blocker on WP-03), then the WP-06 surface tail."
claims: []
---

# Control Claude (cc)

## DOING

- **Estate at `f672dbd`: 224 tests, fmt and clippy clean.** Eight code commits this session -- `5463674` (AC-10.7), `3ebaf55` (AC-03.7 machine-independence), `b67a4be` (`intent sync` wired), `a4b7a34` (WP statuses), `3dfa3ba` (BATS fixture version), `54c2589` (`st list` table + the `st sync` split), `f672dbd` (AC-06.4 thread prose).
- **WP-03 is 7/8**, unsatisfied only AC-03.8. WP-05 is BLOCKED 3/4 on vc's reopened AC-05.3 (ic's to close). WP-06 is 1/7 and is where I am.

## TODO -- in this order

1. **AC-03.8 -- canon -> DB -> canon byte-identical per entity.** The last blocker on WP-03, mine, unbuilt. ic proposed it and vc homed it in WP-03 deliberately: an unreversible FIELD is cheap to change now and expensive at WP-10. `store_round_trip.rs` already proves model -> DB -> model identity, so the new part is BYTES, which also catches serialisation order and formatting. **The fixture must exercise every field** -- a field that is empty round-trips trivially, so derive the check from the schema face and fail when a property is not represented, or it will pass while blind.
2. **WP-06 surface tail.** Done: `search`, `schema`, `doctor`, `sync`, the ac/at/wp verbs, `st list`. Remaining: `st edit`, `st repair`, `st zero`; the `issues` and `todo` families; `info`, `version`, `config`, `init`, `bootstrap`; then the tool-infrastructure families (`claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `treeindex`, `fileindex`). Plus **AC-06.6 (`export`)** -- must round-trip AND refuse an unsupported format BY NAME rather than emitting lossily.
3. **`wp list` should use the ported table too.** v2 renders it through the SAME `render_table`; v3 now has that renderer (`views::table` + `TableMode::Terminal`) but `wp list` still prints its own shape. Free parity, and the whole point of porting one function.
4. **The dispatch-table `sync` note is wrong and vc owns it** -- "both spellings run it" is false; see the reply in `vc/inbox.cc.md`. Do not edit it unilaterally.
5. **`bin/intent_init:257` seeds v2 gitignore paths and no `intent/.cache/`.** Every project `intent init` creates fails to ignore the v3 DB, and D29 now depends on ignored status. No convergence path, so AC-10.3's migrator must write it outright.
6. **Before porting `st repair`**: `bin/intent_st:1231` is `[0-9]+)`, and `+` is literal in a `case` glob, so only the 4-digit form has ever worked. Marked `pending-hv` as unconstructible in clap -- a forced fix, not a free choice.
7. **The todo watermark still needs a durable home** (`DONE:<T>` is DATA, carried today as `RenderContext::todo_watermark`); v2's "else start-of-today UTC" fallback cannot survive the no-clock law.

## Watch-outs

- **`cmd | head; echo $?` reports HEAD's exit.** I hit this THREE TIMES tonight, including on `intent ac gate | tail`, with it already written on this board. ic's guard from `burn.sh` is the fix and is three lines: `out="$(cmd 2>&1)"; rc=$?` and only then filter. A note did not stop me; a habit might.
- **`git commit --only <paths>` does NOT protect a file two nodes are editing.** It takes whatever is in the working tree at those paths. Tonight `intent wp start` left `acceptance.md` dirty with vc's prose in it and I nearly swept it -- checked the diff first, which is the only thing that saved it.
- **A test written from the same misreading as the code cannot catch the misreading.** `both_spellings_of_sync_are_wired_and_agree` passed and confirmed nothing but its own premise. The incumbent's behaviour is the independent check; my own test is not.
- **NEVER mutate `bin/**` in place while anyone else is live** -- `~/.local/bin/intent` symlinks INTO this repo. Sacrificial `git worktree` only.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **The markdown formatter is a second writer and it wins.** Generated views must emit what prettier would emit; that is why `TableMode::Markdown` keeps a minimum column width of three and `Terminal` does not.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp.** The pre-commit clock guard refuses a commit whose stamp postdates it, lacks a trailing `Z`, or sends an inbox backwards.
- **v3 REFUSES in this repository now**, and that is correct -- it is an unmigrated 2.19.0 project. Fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.

## Decisions -- this session

The ratified ones are `design.md` D22-D30; these are the working lessons.

- (2026-08-15) **A guard on reads is not a guard.** AC-10.7 looked like a bad-message defect until I mutated the guard away and ran it: `intent st new` on an unmigrated project SUCCEEDED and rendered a generated stub over an existing v2 thread's authored `info.md`. Six weeks of notes to `_(not yet written)_`, exit 0, reporting `created:`. Reads lie; mutations destroy.
- (2026-08-15) **Two independent signals cover for each other, and that is also how one of them hides being broken.** The declaration caught the data-loss case after the unguarded run had destroyed the evidence -- and separately, the declaration was covering for an evidence scan that saw 1 of this repo's 56 threads because v2 RELOCATES closed threads one level down. A redundant check makes a partial check look whole.
- (2026-08-15) **The corpus must be a property of the repository, not of the machine** (vc). The ignore walker honours the operator's global gitignore and `.git/info/exclude` by default, so what counts as canon differed per developer -- and AC-10.2 turns that into a migration that blocks for one and not the other. D29's own derivation ruled it: a path excluded only by someone's personal config is one `git add` away from being committed by anyone else.
- (2026-08-15) **A fixture that cannot fail proves nothing, and the second attempt is where you find out.** My first global-ignore test used `*.sql` -- faithful to vc's report and useless, because it is in my real global too. It now uses an extension no real global carries AND asserts `git check-ignore` agrees before asserting Intent disagrees.
- (2026-08-15) **Classify by the exit code, not by the printed text.** The surface sweep's first draft judged commands on what they printed, which meant adjudicating whether "`ST0001` is not a work package" counted as guarded -- and every such judgement widens the pass condition until the test agrees with the code. The invariant is that no command SUCCEEDS on an estate it cannot see.
- (2026-08-15) **Derive the sweep from the table, never from a hand-list.** A hand-list is complete the day it is written and silently incomplete from the next verb on, which is exactly how WP-06's tail would reintroduce a defect one command at a time.
- (2026-08-15) **Interleave and take the minimum, or do not report the number.** My first attempt at costing the migration check measured the UNCHECKED build as slower; process-spawn noise is bigger than the effect. Interleaved x60, minimum: +0.61 ms on 200 threads.
- (2026-08-15) **A confident claim about a corpus you do not own is a defect you are handing to someone else.** I told ic "tempdir fixtures are unaffected, they declare 3.0.0" -- true of my Rust fixtures, false of the BATS estate, whose shared builder said 2.10.0 and put 19 keep files under a migration refusal. They measured instead of believing me.
- (2026-08-15) **Port the incumbent's ONE function, not its output.** v2's width algorithm is a single `render_table` shared by three commands "so the two tables cannot drift apart"; reproducing the look would have satisfied the tests and drifted at the first change.
