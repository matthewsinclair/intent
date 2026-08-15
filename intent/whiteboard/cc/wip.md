---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 01:16Z
status: active
focus: "WP-06 is the only blocked gate. Built tonight: AC-10.7, AC-03.7, AC-03.8, AC-06.4, AC-06.7, st list + wp list to v2's table. Next: AC-06.6 (export), then AC-06.1's surface tail."
claims: []
---

# Control Claude (cc)

## DOING

- **Estate at `86b5951`: 234 tests, fmt and clippy clean.** Twelve code commits this session. **Gates 01-05 are all green; WP-06 is the only blocked one and it is mine** (2/7).
- WP-03 closed 8/8 -- AC-03.7 (machine-independent corpus) and AC-03.8 (canon round-trip) both landed and vc verified them.

## TODO -- in this order

1. **AC-06.6 -- `intent export --format <fmt>`.** Mine, unbuilt, and the last WP-06 AC that is purely mine. D03's mechanism: "YAML/md/anything else are `intent export --format` projections", which is how v3 refuses YAML canon without refusing YAML users. **It must round-trip -- what it emits re-ingests to byte-identical canon -- OR refuse the format BY NAME rather than emitting lossily.** That is the `at lint --fix` scar applied before the tool exists: a tool that cannot finish a job must not start it.
2. **AC-06.1 -- the full surface.** The WP's whole remit. Remaining: `st edit`, `st repair`, `st zero`; the `issues` and `todo` families; `info`, `version`, `config`, `init`, `bootstrap`; then the tool-infrastructure families (`claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `treeindex`, `fileindex`). Note the AC's own clause: **`intent config` lands a conformance test BEFORE its behaviour is designed**, or the `undefined` ruling on it is unverifiable by construction.
3. **Post vc a list of built-but-unflipped ATs** rather than feeding them one at a time -- they have found three of mine stale tonight while in the middle of their own verification passes. AT-06.2 (`doctor_checks.rs`, 18 tests, live since `ab351a2`) is outstanding right now.
4. **`bin/intent_init:257` seeds v2 gitignore paths and no `intent/.cache/`.** Every project `intent init` creates fails to ignore the v3 DB, and D29 depends on ignored status. No convergence path, so AC-10.3's migrator must write it outright.
5. **Before porting `st repair`**: `bin/intent_st:1231` is `[0-9]+)`, and `+` is literal in a `case` glob, so only the 4-digit form has ever worked. `pending-hv` as unconstructible in clap -- a forced fix, not a free choice.
6. **The todo watermark still needs a durable home** (`DONE:<T>` is DATA, carried today as `RenderContext::todo_watermark`); v2's "else start-of-today UTC" fallback cannot survive the no-clock law.

## Watch-outs

- **A mutation that does not apply is indistinguishable from a test that legitimately caught nothing.** Mine did tonight -- `\n\n` inside a `python -c` double-quoted string became real newlines, the needle never matched, the assert fired, the shell carried on, and the suite reported ok. Heredoc plus an explicit non-zero exit, every time.
- **`cmd | head; echo $?` reports HEAD's exit.** Three times tonight, with it already on this board. ic's `burn.sh` guard is the habit: `out="$(cmd 2>&1)"; rc=$?` and only then filter.
- **`git commit --only <paths>` does NOT protect a file two nodes are editing.** `intent wp start` left `acceptance.md` dirty with vc's prose and I nearly swept it; reading the diff first is the only thing that saved it.
- **A test written from the same misreading as the code cannot catch the misreading.** `both_spellings_of_sync_are_wired_and_agree` passed and confirmed nothing but its own premise. The incumbent's behaviour is the independent check.
- **`intent at green` checks EXISTENCE, never tracked-ness** (vc). A green AT can cite a file present in one working tree and absent from a fresh clone. Filed as a v3 requirement.
- **NEVER mutate `bin/**` in place while anyone else is live** -- `~/.local/bin/intent` symlinks INTO this repo. Sacrificial `git worktree` only.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **The markdown formatter is a second writer and it wins.** `TableMode::Markdown` keeps a minimum column width of three for that reason; `Terminal` does not.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp.**
- **v3 REFUSES in this repository**, correctly -- it is an unmigrated 2.19.0 project. BATS fixtures declare 3.0.0 via `INTENT_FIXTURE_VERSION`.

## Decisions -- this session

The ratified ones are `design.md` D22-D30; these are the working lessons.

- (2026-08-15) **A guard on reads is not a guard.** AC-10.7 looked like a bad-message defect until I mutated the guard away and ran it: `intent st new` on an unmigrated project SUCCEEDED and rendered a stub over an existing v2 thread's authored `info.md`. Reads lie; mutations destroy.
- (2026-08-15) **Two independent signals cover for each other, and that is also how one of them hides being broken.** The declaration caught the data-loss case after the unguarded run destroyed the evidence -- and separately covered for an evidence scan seeing 1 of this repo's 56 threads. A redundant check makes a partial check look whole.
- (2026-08-15) **The corpus must be a property of the repository, not the machine** (vc). The ignore walker honoured the operator's global gitignore, so what counted as canon differed per developer -- and AC-10.2 turns that into a migration that blocks for one and not the other.
- (2026-08-15) **A fixture that cannot fail proves nothing, and the second attempt is where you find out.** The first global-ignore test used `*.sql`, which is in my own real global. It now uses an extension no real global carries AND asserts `git check-ignore` agrees before asserting Intent disagrees.
- (2026-08-15) **Classify by the exit code, not the printed text.** Judging commands on what they printed meant adjudicating whether "`ST0001` is not a work package" counted as guarded, and every such judgement widens the pass condition until the test agrees with the code.
- (2026-08-15) **Derive the sweep from the table, never a hand-list.** A hand-list is complete the day it is written and silently incomplete from the next verb on.
- (2026-08-15) **Interleave and take the minimum, or do not report the number.** My first cost measurement made the UNCHECKED build look slower; spawn noise exceeds the effect.
- (2026-08-15) **Port the incumbent's ONE function, not its output.** v2's width algorithm is a single `render_table` shared by three commands "so the two tables cannot drift apart" -- reproducing the look would have passed the tests and drifted at the first change. Porting it made `wp list` free.
- (2026-08-15) **"As observed" cannot mean reproducing an absence of behaviour.** v2 reads `scope:` as free text and this repo carries ten spellings for six sizes; there is no observed behaviour to reproduce, which is exactly what modelling the field fixes.
- (2026-08-15) **A confident claim about a corpus you do not own is a defect handed to someone else.** "Tempdir fixtures are unaffected" was true of my Rust fixtures and false of the BATS estate; ic measured instead of believing me.
- (2026-08-15) **Check whether an apparatus you depend on has the hole you would expect, before it costs anything** (vc's framing). I found the `to-write` case closed and stopped; vc found the tracked-ness case beside it open.
- (2026-08-15) **A query must prove it matched something before it is allowed to report nothing.** My built-but-unflipped AT list returned ZERO and I nearly sent it: the regex captured `covers` as `([^-]+)`, which stops at the hyphen in `AC-06.2`, so it matched no rows at all and printed a clean result. Five rows were outstanding, one of them AT-10.7. **Third instance of this shape across the three of us tonight** -- vc's `*.sql` fixture, ic's header needle, my regex -- and every one was a check answering confidently about a set it never looked at. Assert the parse is non-empty, and print the count so the coverage is arguable.
