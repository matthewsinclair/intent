---
node: cc
name: Control Claude
role: control
session_id: dd0650f6-a3a7-4513-99da-3842c2c1373e
heartbeat_at: 2026-08-15 00:06Z
status: active
focus: "Folded for a compact mid-session. WP-06 building. On wake FIX AC-10.7 FIRST -- v3 reports an empty estate with exit 0 on any unmigrated project, which is every project's first contact with v3."
claims: []
---

# Control Claude (cc)

## DOING

- **Folded for a compact, not for the day.** `status` stays `active`: a compact does not end a session (protocol invariant 6). The day's detail is in `.history/20260814/` and `.history/20260815/`.
- **Estate green at `f66622a`**: 204 tests, fmt and clippy clean. Five code commits this session -- `f0d6e64` (third-level surface + search + schema), `ab351a2` (doctor), `729a657` (ac/at/wp tail), `945e099` (store read-back), `f66622a` (D29 corpus rule + D28 WP prose + the fast path).

## TODO -- on wake, in this order

1. **AC-10.7 -- FIX THIS FIRST. A No Silent Errors defect in my code, at the worst possible moment.** vc measured it: in a 2.19.0 project with real threads, `intent st list` exits **0 with zero bytes on both streams**. v3 reports an empty estate, confidently, with a success code, the first time anyone runs it. Even for a genuinely empty estate v2 prints a header. `doctor` shares the root and inverts it into a false RED -- both its findings are view-skew from rendering an EMPTY model against real files, so a user reads it as v3 declaring their `steel_threads.md` corrupt. **The contract gap underneath: AC-10.1 covers pre-2.19.0 refusal and AC-00.8/AC-10.3 cover the migration, but the state BETWEEN was unspecified -- and it is the state every project on earth is in when v3 first runs.** AT-10.7 is `crates/intentsvcs/tests/unmigrated_project.rs`, to-write. Every command that reads the model must detect and NAME an unmigrated project rather than answering from an empty one.
2. **AT-06.5 is WRITTEN but vc has not seen it.** vc verified AC-06.5's behaviour independently (five faces byte-identical by `cmp`; `faces.rs` has zero filesystem reaches, so the byte-identity is a real drift check rather than a `cat` tautology) and reported AT-06.5 as still `to-write`. It landed at `f0d6e64` as `crates/intent-cli/tests/schema_command.rs`. **Tell vc so the AC can be flipped** -- test-backed satisfaction is computed from green ATs, so it cannot flip until they see it.
3. **WP-03's status is MINE to decide.** vc filed D29 as AC-03.7 inside WP-03 rather than as a convenient new WP-06 AC, which reopens a Done WP: `intent ac gate ST0056/03` exits 1, BLOCKED 6/7. The fix and `AT-03.7` (`ignored_paths_corpus.rs`) both landed at `f66622a`, so the gate should now pass -- **run it before deciding whether the WP status returns to wip.** vc deliberately did not touch wp state.
4. **`bin/intent_init:257` seeds the wrong gitignore paths, and it is the only place a project `.gitignore` is ever written.** The heredoc carries the v2 `intent/.config/cache/` + `intent/.config/backup/` and no `intent/.cache/`, so every project `intent init` creates fails to ignore the v3 DB -- which D29 now depends on, since the corpus rule keys on ignored. vc landed `intent/.cache/` in THIS repo's `.gitignore` (D21 had been ratified and never implemented anywhere). **There is no convergence path for existing projects, so AC-10.3's "gitignore converged" has no v2 antecedent and the migrator must write it outright.** The good news: the same heredoc already carries `.DS_Store`, so D29 fixes the macOS blocker in every properly-seeded project with no junk list.
5. **WP-06 remaining surface.** `st edit`, `st repair`, `st zero`; the `issues` and `todo` families; `info`, `version`, `config`, `init`, `bootstrap`; then the tool-infrastructure families (`claude`, `agents`, `lang`, `ext`, `plugin`, `modules`, `llm`, `learn`, `critic`, `treeindex`, `fileindex`). Plus **AC-06.6 (`export`)** -- vc gave it an AC rather than a deferral: it must round-trip AND refuse an unsupported format BY NAME rather than emitting lossily, which is the `at lint --fix` scar applied before the tool exists. And **AC-06.7** -- `intent search` must find a phrase appearing only in a WP body; the index carries WP prose as of `f66622a`, so this wants its test.
6. **Before porting `st repair`**: `bin/intent_st:1231` is `[0-9]+)`, and in a `case` glob `+` is a literal, so only the 4-digit form has ever worked. Marked `pending-hv` as unconstructible in clap -- a forced fix, not a free choice.
7. **The todo watermark still needs a durable home** (`DONE:<T>` is DATA, carried today as `RenderContext::todo_watermark`); v2's "else start-of-today UTC" fallback cannot survive the no-clock law.

## Watch-outs

- **`git commit --only <paths>` does NOT protect a file two nodes are both editing.** It scopes to paths and then takes whatever is in the working tree there. My `ab351a2` swept ic's uncommitted `MODULES.md` row exactly this way. Register a module in the SAME commit that creates it; never leave a MODULES.md edit sitting.
- **`cmd | head; echo $?` reports HEAD's exit, not the command's.** It fired for vc three times in one session and each time handed them a finding that did not exist -- and I made the same mistake once. Exit codes are half of what this thread's parity contract is about. Capture into a variable, then test.
- **NEVER mutate `bin/**` in place while anyone else is live** -- `~/.local/bin/intent` symlinks INTO this repo. Sacrificial `git worktree` only.
- **Do not use `git stash` in this repo** -- two pre-existing 2025 stashes; a pop once dumped 522 lines of pruned migration code into the tree.
- **The markdown formatter is a second writer and it wins.** Generated views must emit what prettier would emit. Authored `*emphasis*` is the one case the renderer cannot stabilise; AC-07.6 lands the exclusion.
- **Every timestamp is read from `date -u +'%Y-%m-%d %H:%MZ'`, per stamp.** The pre-commit clock guard refuses a commit whose stamp postdates it, lacks a trailing `Z`, or sends an inbox backwards.
- **Adding any `.bats` file now costs a register regeneration** (ic): the register is corpus-bound to the on-disk `tests/**` estate at WP close, and 98 rows against 99 files is the silent undercount `lib_corpus.sh` refuses. Rust tests under `crates/**` cost nothing.
- **8 of ic's 31 `keep` files cannot construct their fixtures under v3** -- they hand-build `intent/st/NOT-STARTED/`. They are exactly the model-facing files, so **conformance for st/wp/ac/at cannot be measured from the v2 estate as written**; those convert to mutation-based fixtures or retire.

## Decisions -- this session

The ratified ones are `design.md` D22-D29; these are the working lessons.

- (2026-08-15) **The daily driver must not touch files, and measuring said so louder than reasoning did.** Answering from the store instead of re-parsing canon saved only ~20%, because the freshness SCAN -- 244 files content-hashed -- dominated, and both paths paid it. The fix was not a cheaper check but removing the check from the command path: warm commands ~13ms -> ~5ms, `st sync` 26ms for 80 threads. **The optimisation you reason your way to is the one you can see; the one that matters is the one you measure.**
- (2026-08-15) **A trade is only acceptable if its failure mode is VISIBLE.** Trusting the store makes an out-of-band canon edit serve stale data. That is fine ONLY because `doctor` rebuilds from canon and reports the disagreement with `intent sync` as its remedy. Verified end to end rather than assumed: edit behind the tool, read stale, doctor names it, sync repairs it, doctor clean.
- (2026-08-15) **Clean by luck and clean by construction look identical in a diff.** `intent/.cache/intent.db` escaped the ingest corpus through path shape, not through any rule -- and D21, which was supposed to make it ignored, had been ratified and never implemented anywhere. D29 alone would have left the hole open and looked complete.
- (2026-08-15) **When an argument disposes of a concern, check whether it disposes of the neighbouring one too, before conceding the neighbour** (ic, against themselves). They conceded `bats_coverage` was qualified by my finding, then measured and found the same argument covered it. A wrong concession is one pickup away from becoming a peer's inherited assumption.
- (2026-08-15) **Test the seam the behaviour actually lives at.** My first AT-03.7 called `ingest::read`, which only ever opens `thread.json` by name and never sees a stray file -- three false failures. The corpus rule lives in `sync::scan`.
- (2026-08-15) **A fixture has to be able to fail.** The first corpus fixture used `\x00\x01` as "binary", which is valid UTF-8 -- so nothing was flagged and the test passed while proving nothing.
- (2026-08-14) **Two commands that look like one idea twice may be a precise pair.** `ac rescope`/`ac reinstate` read as synonyms and were about to ship as aliases; v2 refuses each other's state and names the correct verb. Reading the incumbent caught what the help text hid.
- (2026-08-14) **A surface that accepts an invented verb is a No Silent Errors failure wearing the costume of a gap** -- and it was invisible from every error-path test BECAUSE the error path is where it looked correct.
- (2026-08-14) **Two-directional guards survive their own premises changing; one-directional ones certify the status quo.** The SSOT guard caught its own premise going stale because it asserted both directions.
- (2026-08-14) **Classifying by the SHAPE of a failure is a guess that looks like a finding.** Only reading the test says why it failed.
