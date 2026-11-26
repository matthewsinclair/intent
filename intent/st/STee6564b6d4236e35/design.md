# /in-autopsy for the 3.5.x era: Phase 0 design

This is an analysis and a design draft. It changes no code. The measurement ran on 2026-11-26 at 09:31Z by `date -u`, on main at `30a9d2da6`, with `intent 3.5.4` from brew.

## What the skill must find

A 3.5.x session is one seat of a Claude Team. Most of its entries are tool calls, tool results, and messages from other seats. A person types few of them. The skill must read all of these as evidence.

### Who wrote each entry

The skill must name the source of every `user` entry before it reads the entry's words. The sources are:

- a person, who typed the words;
- a seat, in a `<cross-session-message>` wrapper that carries `from-name`;
- a subagent's hand-back, in an `<agent-message>` wrapper;
- a background task, in a `<task-notification>` wrapper;
- a compaction summary;
- a skill's body or a slash command's wrapper;
- a system reminder;
- a tool result.

The skill must also name the seat a session ran in. The transcript carries it: the `agent-name` entry holds the session name, eg `intent-dc`.

### The whiteboard

Source of the rules: `/in-whiteboard`, and the terms that `intent wb profile terms` prints.

- A write with another seat's moniker: `--node <moniker>` differs from the session's own seat.
- A hand edit of a rendered view: an `Edit`, a `Write`, or a shell write to `intent/whiteboard/<node>/wip.md`, an `inbox.<sender>.md`, or a `board.json`.
- `intent wb clear <sender>` with no answer and no action after the pickup that showed the message. The script gives the sequence. The reading is a judgement.
- `intent wb release` followed by more work in the same session, or a release before a compact.
- `intent wb add hold` with no condition in its text. This is a judgement.
- A message that must be acted on, sent to the inbox only, with no `SendMessage` beside it.
- A ruling or a done claim that a seat took from a board and did not check against the artefact.

### The seats and the route to hv

Source of the rules: `intent/plugins/claude/teams/TERMS.md`, and the roster in `intent/whiteboard/README.md`.

- A seat other than the mediator runs `intent wb ask hv`. The tool refuses it, and the refusal is evidence that the seat did not know the route.
- hv rules to a seat directly, and the seat does not pass the ruling to the mediator.
- A seat waits idle on hv.
- A message to a seat is longer than the terms allow. A report to hv is longer than the terms allow. The script gives each message's length. The limit is read from the terms.
- A seat repeats a check that a builder already ran. This is a judgement on the Desk and command events.

### The Desk

Source of the rules: the `intent estate slots` entry of `intent llm guide`, `/in-essentials` rule 10, and `AGENTS.md`.

- A heavy run with no open slot: a `bin/devbin` build, gate, test, or release verb with no `intent estate slots start` before it.
- A slot that was started and never released.
- A START message with no END message for the same request id.
- A `start` whose `--named` set differs from the set of the `ask`. The tool refuses it, and the refusal is evidence.
- A heavy run that began with no read of `intent estate status`.

### Banks and worktree folds

`refs/bank/<seat>/...` and `refs/attic/<node>/<worktree>` are this project's practice. The rules are on the boards, eg vc's watch-out of 2026-10-27 on attic refs. No shipped skill defines them. The script must report the events as facts:

- every `git update-ref` and every `git worktree add` or `git worktree remove`, with its ref or path;
- a `git worktree remove` with `--force`, which `/in-whiteboard` forbids;
- a `git worktree remove` with no attic ref written before it.

The procedure reads the board's watch-outs, decisions, and directives, and it judges the events against them.

### Acceptance tests

Source of the rules: the `intent at red`, `intent at green`, and `intent ac satisfy` entries of `intent llm guide`, and `/in-finish` step 2.

- `intent at red` with no failing test run before it in the same turn.
- `intent at green` with no passing test run before it.
- `intent ac satisfy` on a criterion whose work was not done. This is a judgement. The script gives each call and the commands before it.
- A refusal from `at green`, `st done`, or `wp done`, and what the seat did next.
- Any call of `intent fc`. `IN-AG-FIAT-001` makes this verb the person's alone.

### `wrote:` lines and commits

Source of the rules: `/in-essentials` rule 9, and `/in-whiteboard` invariants 9 and 10.

- A commit in the shared checkout with no `--only`. The script reads the entry's `cwd` and the command, so a commit in a worktree is a different case.
- `git add -A`, `git add .`, or `git add --all` in the shared checkout.
- A committed path that no verb printed on a `wrote:` or `removed:` line in the session.
- A printed `wrote:` path that the session never committed. This is a lead, because a pickup alone is not committed.
- A commit of a path that a verb printed as `re-rendered:`, which belongs to another seat.
- A commit message with an AI attribution line.

### Fabricated timestamps

Source of the rule: `/in-whiteboard`, "Every timestamp is READ FROM A CLOCK".

Every transcript entry carries its own `timestamp`, which the machine wrote. This gives two checks.

- A stamp ahead of the clock: a seat writes a time that is later than the entry's own time. No clock gave that value.
- A stamp with no read: a seat writes a stamp, and no `date -u` result in the same turn holds that value.

A third case is a stamp with no trailing `Z`. The script checks the stamps a seat writes into `SendMessage` bodies, `intent wb` texts, commit messages, and file writes.

### What the tools said

The v2 skill reads no tool result. In 3.5.x the tool is the first to correct a seat. The skill must find:

- every refusal and every `error:` line from an `intent` verb, with the command and the next command;
- the same refusal met more than once, which shows a gap in the guide or in a skill;
- a non-zero exit that the seat did not mention or act on;
- a guard's refusal at commit, and what the seat did next.

### Corrections

- A person's correction, read only from entries a person typed.
- A seat's correction of another seat, and a ruling of hv that a seat relayed.
- A seat's own correction, eg a line that opens with "CORRECTION".
- A correction after a compaction, which the v2 skill already marks.

### Rules from memory and the global files

The skill reads `MEMORY.md` and its linked files, the global `CLAUDE.md`, `house-rules.md`, and `AGENTS.md`, and the project's `CLAUDE.md` and `AGENTS.md`. Some of those rules have a mechanical form, eg `--no-confirm`, a truncated `git remote -v`, a direct `cargo` or `mix` call, or a bulk `sed`. The procedure writes these forms to a pattern file for one run, and the script scans commands and prose with it.

## What it finds today and what it misses

### The measurement

The current script ran read-only with this command:

```bash
elixir intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs --days 1 --project "Devel-prj-Intent" -o <scratchpad>/findings-1d.json
```

It read the eight sessions of the last day under `/Users/matts/.config/claude/projects/-Users-matts-Devel-prj-Intent/`. Four are the seats' sessions of 2026-11-25 and 2026-11-26: `38176daf` (dc), `7094e76d` (ic), `ef5e3583` (cc), and `f6d27012` (vc). It ran with no build and no mix project. `Mix.install` found Jason in the local cache.

Its summary:

```
Corrections: 3
Frustration signals: 2
User flags: 12
Deferrals: 0
Banned matches (raw): 4
  authored here     : 1
  ACTIONABLE        : 0  <- the number to act on
```

### What it reported is wrong about who spoke

A person typed twelve entries in the eight sessions. The script attributed none of its findings to those entries.

- Every one of the twelve user flags is text that no person typed. Eight are compaction summaries that open with `<artifact-content-authored-by-others/>`. Two are subagent hand-backs. Two are messages from a seat. The pattern `\bflag\b` matches any use of the word, and the script does not ask who wrote it.
- Both frustration signals are messages from a seat. One is ic's message to cc that opens "please agree the meaning of the A3(a) re-cite text". The other is dc's "GO taken" report to vc.
- Two of the three corrections pair a seat's status report with an entry that corrected nothing. One entry is a Desk message from devbin-vc. The other is a task notification.
- One correction is real: vc's "I was wrong, and I'm writing the correction" at `f6d27012` line 7592. The script names the user as its source. The source was hv's refutation, relayed by devbin-vc.

The 3.5.4 fix removed skill bodies from this count. Peer messages, hand-backs, task notifications, and compaction summaries are still counted as the user's words.

### What it missed

I read the same transcripts with my own throwaway scripts and by eye. The current script reports none of the following.

- **A fabricated stamp.** At `38176daf` line 5702 dc sent a message that opens "dc to vc, 2026-11-25 18:12Z by date -u". The entry's own time is `2026-11-25T18:05:43Z`. No `date -u` ran in that turn. The same class is in all four seats' sessions, eg `f6d27012` line 6581: "2026-11-26 06:05Z" written at `05:57:40Z`.
- **A seat's own correction of a stamp.** At `ef5e3583` line 936 cc wrote: "A CORRECTION: the live message that said this gave its time as 16:0xZ, which no clock gave". No acknowledgment pattern matches it.
- **A seat's correction of another seat.** At `38176daf` line 5437 vc wrote to dc: "ruling changed. The one-pass count goes into this bank." At `7094e76d` line 6052 vc wrote to ic: "Your reading is right, and my brief was wrong".
- **The tools' refusals.** At `38176daf` line 7586 `intent estate slots ask` answered "error: an ask names at most one preferred lane". At line 7418 a runner answered "refused: main's HEAD is 5e7b1116d... so this run would judge another commit". At `f6d27012` line 1862 the answer was "error: unrecognized subcommand 'list'". The script reads no tool result.
- **Every protocol event.** The dc session holds `intent wb` writes, `intent estate slots` calls, and START and END messages. It holds `refs/bank/` and `refs/attic/` writes, worktree adds and removes, `intent ac satisfy` calls, and commits. The script reports none of them, so it cannot say whether a rule held or broke.
- **Subagent transcripts.** Each session has a `subagents/` folder beside it. The script reads only the top-level `.jsonl` files.

### Why it finds too little

- **The house style removes its trigger.** The correction detector waits for an apology phrase from the assistant. The house style bans "You're right" and STE removes the others. A seat that corrects itself in house style is not seen.
- **The person rarely speaks.** The frustration and flag detectors read the person's words. In a team session the person types a few lines, and a seat or a tool delivers most corrections.
- **It reads words only.** It scans tool inputs for banned words. It never reads a command as an act, and it never reads a tool result.
- **Its banned list measures a rule that holds.** The list found no actionable match in the eight sessions.

### Where the rules held

My scripts found every commit in the shared checkout made with `--only`, in all four seats' sessions. They found no hand edit of a rendered view. The rewrite must be able to say this as "no finding" for a check that ran, so a clean answer and an unrun check differ.

### What I could not measure

- The check of committed paths against `wrote:` paths. Paths often reach `git commit --only` through a shell variable or a file, so a throwaway script cannot resolve them. The rewrite must read the commit's own answer for the paths.
- START with no END. My script named three request ids with a START message and no END message. I did not verify them by eye, so they are leads.
- A stamp with no `date -u` read in the turn. My script found many, but its turn boundary is coarse and a runner script can read the clock. Only the stamps ahead of the entry's own time are verified.
- `intent wb clear` with nothing handled. It needs a judgement on each message.
- The large sessions of 2026-11-24 and earlier. I did not run the script on them, because the eight sessions gave enough evidence.

## What stays and what goes

### Stays

- The two stages. A script extracts facts. The procedure in `SKILL.md` reads the rules and judges.
- Elixir, as one `.exs` script that ships in the skill's folder. The reasons are in "The shape of the rewrite".
- The read-only contract: the script writes only its output file.
- The record per occurrence, with `turn_kind` on each record, and the quoted and negated marks.
- The compaction boundary and the post-compaction mark.
- The classes "memory gap" and "enforcement failure", and the stale-memory pass.
- The report file under `intent/autopsy/`, and the rule that the report proposes and applies nothing.
- `skill_content?/1` as an idea. It becomes one case of the source classifier.

### Goes

- `banned-words.txt`. It is a second copy of the house style, which lives in `house-rules.md` and `IN-PR-STYLE-001`. Its AI-ism, hedging, identity, and filler lines cite no rule in force.
- `Mix.install` and Jason. Elixir's own `JSON` module reads and writes the data, so the first run needs no network.
- The acknowledgment patterns as the one route to a correction.
- The frustration and flag patterns over every `user` entry. They run only over entries a person typed.
- The deferral patterns and the "legitimate" patterns in the script. The global rule on deferrals is in `CLAUDE.md`, and the procedure writes its mechanical form to the pattern file.
- `--days` by file modification time as the only selector, and the silent choice of the newest project folder when a name matches several.
- The report's summary line of totals. The memory rule on vanity metrics forbids it.
- The step that tells the session to propose `MEMORY.md` text for each gap. `MEMORY.md` says vc is its one writer and that one incident's procedure is never a memory note.
- The silent drops: a JSON line that does not parse, an unknown flag, and a missing pattern file are each reported.
- The detector arms of `tests/unit/test_autopsy.bats` that compare against a count written by hand, and the arms that require `banned-words.txt`.

## The shape of the rewrite

### Language

Elixir stays. The script reads Claude Code's transcript format, which is outside Intent's model, and it ships as a skill asset that `intent claude skills install` copies. A Rust verb would add a row to the dispatch table, the guide, the MCP list, and the release's reference set for a patch release. The script needs Elixir 1.18 or later for the `JSON` module. `SKILL.md` says so, and the script refuses an older Elixir with that remedy. No Python is added.

### Modules

One file, `scripts/autopsy.exs`, with pure modules and one impure coordinator (PFIC, Thin Coordinator).

| Module               | Kind   | What it does                                                                                                                                                                                   |
| -------------------- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Autopsy.Transcript` | pure   | Turns one JSONL line into a typed entry. It is the one reader of the wrapper markers, and it names each entry's source and the session's seat.                                                 |
| `Autopsy.Turns`      | pure   | Groups entries into turns. It joins each tool result to its tool call, and it carries the entry's time, `cwd`, and compaction index.                                                           |
| `Autopsy.Events`     | pure   | Reads each tool call and its result as an event: an `intent` verb with its arguments, exit, and printed lines; a git commit, ref, or worktree act; a clock read; a file write; a sent message. |
| `Autopsy.Checks`     | pure   | Runs each mechanical check over the events of a session. Each check returns findings, or the fact that it ran and found none.                                                                  |
| `Autopsy.Signals`    | pure   | Finds corrections by source: a person, a seat, a tool, or the seat itself.                                                                                                                     |
| `Autopsy.Patterns`   | pure   | Scans prose and commands with the pattern file of the run. It keeps the per-occurrence record and the quoted and negated marks.                                                                |
| `Autopsy.CLI`        | impure | Reads arguments and files, calls the modules, and writes the output. Every failure is reported with its remedy, and the exit is non-zero.                                                      |

`Autopsy.Events` reads any `intent <noun> <verb>` call by its shape, so a new verb needs no change to the script. Only the checks name verbs.

### The checks

Each check is one function with one name, and each finding carries that name. The first set:

- `stamp_ahead_of_clock`, `stamp_without_read`, `stamp_without_z`
- `commit_without_only`, `add_all`, `commit_path_not_printed`, `printed_path_not_committed`, `commit_of_rerendered`
- `write_as_other_seat`, `hand_edit_of_view`, `release_then_work`, `ask_hv_past_mediator`
- `heavy_run_without_slot`, `slot_not_released`, `start_without_end`
- `at_red_without_run`, `at_green_without_run`, `fiat_close`
- `worktree_remove_forced`, `worktree_remove_without_attic`
- `refusal_repeated`, `exit_not_acted_on`

A check holds the shape it looks for. It holds no rule text. Each finding carries a `source` field that names where the rule lives, eg `/in-essentials rule 9`, so the procedure reads the rule there.

Three of these checks need a value that a project sets: the paths of rendered views, the commands that count as a heavy run, and the shared checkout's path. The script takes them as arguments. The procedure reads them from `AGENTS.md`, `/in-whiteboard`, and `git worktree list`.

### Inputs

- `--project <name>`: refused with the list of matches when the name matches more than one folder.
- `--session <id>`, `--seat <moniker>`, `--since <UTC stamp>`, `--until <UTC stamp>`: choose the sessions and the window. `--days` stays as a short form.
- `--subagents`: also read each session's `subagents/` folder.
- `--patterns <file>`: the pattern file of the run. Each line has a scope (`prose` or `command`), a pattern, and the source of its rule. With no file the pattern scan does not run, and the output says so.
- `--views`, `--heavy`, `--checkout`: the three project values above.
- `-o <file>`: the output file.

The script reads one session at a time and writes its records before the next, so memory use follows the largest session.

### Outputs

One JSON document with these parts:

- `run`: the arguments, the script's version, and the time read from the system clock.
- `sessions`: for each session its id, seat, time window, compaction boundaries, and the sources of its `user` entries.
- `events`: the protocol events, each with session, line, time, and `cwd`.
- `findings`: each with its check, session, line, a short quote as evidence, and its `source`.
- `checks_run`: every check that ran, with the sessions it ran on. A check that could not run is listed with the reason.
- `signals`: the corrections, each with who made it.
- `unread`: every line the script could not parse, with its session and line.

The summary on stderr names each check and whether it found something. It prints no grand total.

### Where the rules come from

The script keeps no rule. The procedure reads the rules live at each run, in this order:

1. Memory: `MEMORY.md` and its linked files, under `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<project-key>/memory/`.
2. The global files: `CLAUDE.md` and every file it imports.
3. The project's `CLAUDE.md` and `AGENTS.md`.
4. The installed skills: `/in-whiteboard`, `/in-essentials`, `/in-finish`.
5. The terms: `intent wb profile terms`.
6. The boards: `intent wb show hv --all` for directives and rulings, and the seat's own board for watch-outs and decisions.
7. The rule library: `intent claude rules list`, then `intent claude rules show <id>` for a rule a finding names.
8. The guide: `intent llm guide`, for what a verb is for and when it refuses.

From these the procedure writes the pattern file and the three project values to its scratch folder. It then runs the script.

### The procedure in SKILL.md

1. Read the clock with `date -u`. Name the seat and the window with the person, or take the session in hand.
2. Read the rules, as above.
3. Write the pattern file and run the script.
4. Read the findings. For each one, read the rule at its `source` and the transcript lines around it. Drop a finding the rule does not cover, and say why.
5. Read the events for what no check can judge. Examples are a `clear` with nothing handled, a hold with no condition, and an `ac satisfy` on work not done. Others are a repeated proof, and the bank and attic practice on the board.
6. Class each finding:
   - **enforcement failure**: a rule in force was broken;
   - **rule gap**: a correction with no rule behind it;
   - **tool gap**: the tool could refuse the act and does not, or its refusal's remedy did not help;
   - **stale rule**: a rule that no session touched in the window.
7. Write the report to `intent/autopsy/YYYYMMDD.md`, with the date from the clock read. Each finding has its evidence, its rule's source, and one proposed fix.
8. Send the report's path to the mediator in at most five lines. The mediator puts it to hv. The skill writes no memory, files no issue, and edits no rule.

A tool gap is proposed as an issue for hv to rule on. A rule gap is proposed as a memory note only when it is a standing rule of hv.

### The report

- **Findings by class**, each with: what happened, the quote, session and line, the rule's source, and the proposed fix.
- **Checks that ran and found nothing**, as a list of names.
- **Checks that did not run**, each with the reason.
- **Corrections**, by who made them.
- **Leads**: events the session could not judge.

The report has no summary of totals.

### Files

| Path                                                               | Change                                                                              |
| ------------------------------------------------------------------ | ----------------------------------------------------------------------------------- |
| `intent/plugins/claude/skills/in-autopsy/SKILL.md`                 | rewritten                                                                           |
| `intent/plugins/claude/skills/in-autopsy/scripts/autopsy.exs`      | rewritten                                                                           |
| `intent/plugins/claude/skills/in-autopsy/scripts/banned-words.txt` | removed                                                                             |
| `tests/unit/test_autopsy.bats`                                     | the detector arms and the `banned-words.txt` arms replaced; the lifecycle arms kept |
| `intent/docs/skills-triage.md`                                     | the `in-autopsy` row updated                                                        |
| `CHANGELOG.md`                                                     | one entry under 3.5.5                                                               |

No new module needs a row in `intent/llm/MODULES.md`: the script is a skill asset and has no row today. No Rust changes, so the Desk run is the shell test leg only.

## Tests

New tests are needed, because the detectors are new product behaviour. They stay in `tests/unit/test_autopsy.bats` and replace the detector arms there. No new test file and no new guard is added.

- **One fixture session**, written as real transcript lines in the test, as the file does today. It holds a seat's `agent-name` entry and one line or one short turn for each check.
- **One arm for each check**, with two assertions. The first asserts that the finding with that check's name is present at the fixture line that breaks the rule. The second asserts that it is absent at a line that keeps the rule. The second is the control: it fails if the check fires on everything.
- **One arm for sources**: the fixture has a peer message, a hand-back, a task notification, a compaction summary, and a skill body. Each carries the word "flag", and none is reported as a person's flag. A typed entry with the same word is reported.
- **One arm for No Silent Errors**: a line that does not parse is named in `unread`, and an unknown argument exits non-zero with a remedy.
- **One arm for the refused cases**: a project name with two matches is refused, and each match is named.

No arm compares against a count written by hand. An arm selects a finding by check name and fixture line, and it asserts presence or absence. Where an arm needs a number, it derives it from the fixture, eg from the lines the fixture marks.

The proof of the rewrite as a whole is one layer: the builder runs the new script once on the four seats' sessions of 2026-11-25 and 2026-11-26. It reports whether the script finds the stamp at `38176daf` line 5702 and the refusal at line 7586. It reports whether the script finds cc's correction at `ef5e3583` line 936. It reports whether any entry that no person typed is given as a person's words. That run needs no Desk slot. The shell test leg rides the release's own gate.

## Size

One steel thread with three work packages.

| Work package                                                                                                                                             | Size |
| -------------------------------------------------------------------------------------------------------------------------------------------------------- | ---- |
| WP 1: the reader. `Transcript`, `Turns`, `Events`, `CLI`, the sources, the seat, the new selectors, `JSON` in place of Jason, and the reported failures. | M    |
| WP 2: the checks, the signals, and the pattern scan, with their arms in `tests/unit/test_autopsy.bats`.                                                  | M    |
| WP 3: `SKILL.md`, the report format, the removal of `banned-words.txt`, the triage row, the CHANGELOG entry, and the one proof run.                      | S    |

The thread as a whole is L. WP 2 depends on WP 1. WP 3 depends on both.

## Open questions for hv

### 1. Elixir script or a Rust verb

- **(a) Elixir, as designed (recommended).** The skill stays a skill with a script. It costs the thread's L and needs Elixir 1.18 or later on the machine. dc builds it.
- **(b) A Rust verb, eg `intent autopsy`.** The reader joins the library crate and every project gets it with no Elixir. It costs XL: a dispatch row, guide text, the MCP decision, Rust tests, and a cargo slot on the Desk. cc builds the reader and ic owns the surface. It does not fit a patch release.

### 2. How much of the check set ships in 3.5.5

- **(a) All the checks listed (recommended).** The thread is L, and hv's examples are all covered. dc builds it.
- **(b) The reader, the sources, the stamps, the commits, and the tools' refusals only.** WP 2 drops to S and the thread to M. The Desk, AT, and worktree checks move to a later release, and until then the procedure judges those events by reading. dc builds it.

### 3. The banned-words list

- **(a) Remove it, and scan with the pattern file that the procedure builds from the live rules (recommended).** One home for the house style. A project with no house-style rule gets no word scan. It is in WP 3 at no added size.
- **(b) Keep a shipped default list.** A project with no rules still gets the AI-ism scan. It keeps a second copy of the house style, which the Highlander rule forbids, and its lines cite no rule. No added size.

### 4. Who receives the report and who acts on it

- **(a) The report goes to the mediator, and the skill writes nothing else (recommended).** vc is memory's one writer, and hv rules on each issue, so this option fits both rules. On a `solo` board the mediator is gc, so the same rule holds. No added size.
- **(b) The skill also files each tool gap with `intent issues add`.** hv gets issues with no ruling step. It adds XS, and it writes to the store from a skill that is read-only today.

### 5. Scope of one run: a seat or the team

- **(a) One run reads one seat's sessions by default, and `--seat` or `--project` widens it (recommended).** A seat runs it at its fold on its own work. The mediator runs it across the team before a release. No added size.
- **(b) Every run reads the whole team.** One report for each window, written by the mediator only. A seat cannot check its own session. No added size.

### 6. Subagent transcripts

- **(a) Read them only with `--subagents` (recommended).** A subagent holds no seat and writes no board, so most checks do not apply to it. No added size.
- **(b) Always read them.** The stamp and commit checks also cover delegated work. Runs take longer on a large corpus, and WP 1 stays M.

### 7. The old reports

`intent/autopsy/` holds three reports in the v2 format, the last from 2026-04-04.

- **(a) Leave them (recommended).** They are records. No cost.
- **(b) Remove them, under the rule that v2 no longer exists.** vc removes them in one commit, XS. hv's rule on retiring v2 things may already decide this.
