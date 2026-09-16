---
node: ic
name: Interface Claude
role: interface
session_id: 7f9cf959-3635-42f2-bfdd-d88cdad6a90a
heartbeat_at: 2026-09-16 18:00Z
status: active
focus: "2026-09-15: localfolded for hv's compact. 0334 and 0396 are closed at 6be7545b4, and issue 0400 is filed for hv with reading (4) held. Resume at the IN FLIGHT doing row: ST0075 WP-02, green in wt-st0075 and banked WIP at refs/bank/ic/st0075-wp02-wip. NO RELEASE, NO PUSH."
claims: [ST0075, ST0075/03]
---

# Interface Claude (ic)

## DOING

- **RESUME HERE after hv's compact, 2026-09-16 night: ic's queue is EMPTY; nothing banked, nothing in flight.** Landed today on vc's GO, verified by vc: 0419, the /projects cursor (fe2471b17, closed 17345a14e); ST0075 WP-02 and WP-03 done on hv's word (7a0a6ae30), and ST0075 then marked done by vc on hv's word (dcf3bc3c1); 0421 and 0422, the in-place edit's caret as an overlay and the list scrolling against its drawn height (4df530f36, closed e30459a36), which hv drove on dcf3bc3c and called working; 0423, app start and restart waiting for the launch to register against `macapp::LAUNCH_DEADLINE` (15 s) and reporting what was measured rather than "did not start" (f330805da, closed 3c5fbc917); 0425, hv's "ok: a", `intent daemon logs` printing both logs as one list merged by each line's stamp, with `--follow` resuming each log where that list stopped (8c1252f39, closed c8a8915fd; the first landing commit was refused by the markdown gate because filling the issue number shortened a padded MODULES.md row, fixed by prettier and rebanked as 449cc7e56). Waiting on hv: the rebuild vc sends (0423, 0425, dc's 0424), then `intent app restart` and the Console's time-ordered backlog. vc ruled ST0075 AC-01.1 stays as written (a DONE thread's record; 0425 records the change). Issue 0420 (store write locks) is vc's, with hv's Console rows from 2026-09-15T22:42Z onwards added. NO RELEASE, NO PUSH.

## TODO

_(none)_

## Holds

_(none)_

## Watch-outs

- **Drive a surface as a USER before calling it done.** Every face was internally consistent and passing, and two still withheld what the reader came for. No test asserting the envelope can see that, because the envelope was right.
- **AN INSTALLED PAYLOAD CANNOT CITE THIS PROJECT'S TRACKER, AND I HAVE NOW DONE IT TWICE.** The whiteboard skill last night and `intent_claude_cwi` today, both refused by `no_pm_state_in_output` (AT-00.17) for ST/WP/AC ids a consumer cannot open. Name the version and the behaviour. **Run that arm on anything under `intent/plugins/` or `lib/templates/` before landing, rather than remembering harder.**
- **THE REGISTER IS TEXT WITH RULES, NOT A FILE TO REWRITE.** Locate an edit by its ROW, never by a pattern, and verify by parsing back which row carried it; insert as text, never through a serialiser; regenerate dispatch-table.md in the same commit (a formatter fixed point, so no `*emphasis*` in register prose). A new row moves `populations.{declared,shipped,probeable}`, `legal_pairs`' `n` and `census_note`, and the new-surface family count in `dispatch::tests`; a family needs `flags: []` on its own row; a new key is classified in `key_classes`. A shipped mutation needs `recoverability` (plus `recoverability_anomaly` when withheld from MCP while recoverable) and sits in exactly one bucket of `write_moves_only_what_changed`; an exact command literal in shipped source is declared in `command_rosters_are_derived_or_declared`. A retirement is `disposition` plus `target.state`, never a code deletion: `Entry::is_shipped()` fails open and `legal_pairs` allows only (retire, retire). Before landing a `transitions.rs` State change run `parity/tools/machine_table_check.sh`, before any `target.state` change `corrected_check.sh`, and attach any file under `intent/st/` before committing it.
- **JUDGE FROM A FRESH READ, AND DRIVE THE REFUSAL RATHER THAN REASON ABOUT IT.** A shared aggregator moves between the read and the judgement, and the stale read looks exactly like a finding (`wb archive`'s enum gained its fifth value at c9f40c79e under my written finding). Read the whole row, not the field you were sent to: the stale claim is usually beside it. Verify a producer in your own shell before landing the consumer that calls it, with a rebuild between the two commits when the consumer calls `intent` from PATH.
- **EVERY REPLACE ASSERTS ITS MATCH COUNT, AND NO WRITE IS EVER `|| true`.** A blind swap hit two verdict matches and turned a refusal into a silent clean seal; a perl substitution fed from an unset variable emptied a design section while `|| true` swallowed the die. Pass data by file, report to STDERR, read a diff before trusting a count over it, and write a red-run mutation against the FORMATTED text with a driver that says DID NOT APPLY.
- **NEVER EDIT BYTES A RUNNING BUILD IN THAT WORKTREE IS COMPILING**: the run then judges code you are not landing (twice in one hour, both menubar app-tests), and a test's comment about what it can fail on gets the same check as the test. **A GATE REFUSAL LEAVES YOUR PATHS STAGED ON THE SHARED INDEX**: unstage your own at once, fix in the worktree, re-prove the one arm, re-issue the same path list; capture the commit's output to a file and read rc, `git log -1` and the head of the refusal. Before any commit read dc's board DOING for an announced window: the window's message can arrive after your commit.
- **A LANDING SILENTLY FALSIFIES THE DOCS THAT TELL A READER TO RUN WHAT IT MOVED, AND NOTHING REPORTS IT.** WP-22 made an Upgrading paragraph wrong; the hydrate refusal sent a known-defects remedy to a refusal; on 2026-09-15 `~/.intent/home` was still named by restart.md and two boards after 2f29401b6 had moved the pointer to `~/.local/share/intent/home`. When a verb's behaviour or a path moves, go and read the pages that name it.
- **A SUITE RUN WRITES THE ESTATE IT RUNS IN, AND /private/tmp DOES NOT SURVIVE A REBOOT.** Every run: a private detached worktree, its own in-tree target dir, `HOME=<scratch>` written as a command, `cargo build -p intentd` first; afterwards `~/.local/share/intent/home` still names this tree. On 2026-09-15 a reboot took every worktree and scratchpad, and only the session transcript under `~/.claude/projects/` still held the notes (heredoc writes in the command text, large reads under `tool-results/`): anything that must outlive a session goes on this board, into a commit, or into the store.
- **A GREEN AT IS COVERAGE, NOT SATISFACTION.** A non-test AC stays unsatisfied beside a green AT until `intent ac satisfy --evidence` names what met it, and its AT stays `n/a`; an AT citing a file counts only if the FILE carries the row's literal id; a completed thread's rows are corrected forward with a note naming where the arm went and at which sha, never withdrawn (vc, citing hv 2026-08-21).
- **BOARDS ARE STORE ROWS: EVERY WRITE IS A `wb` VERB WITH `--node ic`, AND A HAND EDIT IS OVERWRITTEN.** Any store write can re-render a PEER's stale view on disk: those files are theirs, never in my `--only` paths. A peer inbox at 20 refuses `wb ask` (the socket is then the only channel), and my watch-outs stop at 40.
- **A GUARD ON EXIT CODES PASSED A RUN THAT CHANGED NOTHING** (2026-09-15). A prior-art check refused before the heredoc that would have written an edit script; the next command transformed that missing file into an empty script, python ran it at rc 0, and fmt, both suites, clippy and the build all went green on the unchanged code. Guard on the EFFECT: after an edit script, check each new symbol is in its file before anything builds, and read the new test's name in the run's log.
- **DRIVING A TUI CHANGE THROUGH THE REAL BINARY WRITES THE LIVE STORE** (2026-09-15): hv's drive of an in-place edit saved a stray "s" into issue 0398's title. Drive edits on a scratch estate (`intent init` in the scratchpad under an isolated HOME, `explore intent:///issues/0001`, a pty for scripted keys), and say twice that saves are real before handing hv the live one. And a worktree has no gate shim (`.githooks/pre-commit.intent` is gitignored installer canon), so a bank there is a patch blob under `refs/bank/`, never a gate-less commit.
- **A DRY RUN THAT EXITS BEFORE THE WRITES PROVES NOTHING ABOUT THE WRITES** (2026-09-16). land-wp02.sh passed dry twice and stopped half-written: `at green --note` refuses a note that does not CONTAIN the row's existing note (`at edit --note` is the replacing door), so a green after a red is "<red note>; <green note>". Peers sat held on the open chain while I resumed. Before any landing, read each write verb's refusal rules and exercise them in the dry pass, and write every step to read the row's state and skip what is done, so a re-run is safe (vc's ruling).
- **A SUITE LAUNCHED WITH `nohup ... &` IGNORES SIGINT, AND A DAEMON-FAMILY TEST THEN HANGS WITH NO OUTPUT** (2026-09-16): intent-cli's no_tail_survives_follow_however_it_ends sent INT to a verb that inherited SIG_IGN and waited 30 minutes. Launch detached suites through `perl -e '$SIG{INT}="DEFAULT"; exec @ARGV'` or run_in_background, and on a silent stall read the stack and the launch style before theorising (shared memory feedback_background_job_ignores_sigint).
- **A PLACEHOLDER SWAP CHANGES WIDTHS AND BREAKS WHAT WAS ALIGNED AROUND IT** (2026-09-16, 0425). A MODULES.md row padded to the table width while it held `ISSUE_MERGE` came out 7 characters short once `0425` went in, and the pre-commit markdown gate refused the landing mid-chain. Fill issue numbers BEFORE padding or rendering anything aligned, or run `prettier --check` on every touched markdown file in the re-prove after the fill, not only on the first build.

## Decisions

_(none)_

---

_Generated by Intent v3.0.3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
