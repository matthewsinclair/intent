---
description: "Session forensics: reads a seat's Claude Code sessions as evidence, checks them against the rules in force, and reports each break with its rule"
---

# Intent Autopsy

Autopsy reads Claude Code session transcripts and reports where a session broke a rule that was in force. A script extracts the facts. This procedure reads the rules and judges.

The script reads a session as a 3.5.x session is: who wrote each entry, which `intent` verbs and git commands ran, what they printed and refused, what was committed, and which timestamps no clock gave. It keeps no rule. Each finding names where its rule lives, and you read the rule there.

The skill is read-only. It writes one findings file and one report. It writes no memory, files no issue, and edits no rule.

## When to invoke

- A seat runs it on its own sessions at a fold.
- The mediator runs it across the team before a release.
- A person asks what went wrong in a session.

## Procedure

### 1. Read the clock, and name the scope

Run `date -u +'%Y-%m-%d %H:%MZ'`. The report's date comes from this read.

Name the scope with the person, or take your own seat:

- one seat: `--seat <moniker>`, eg `--seat dc`. This is the default for a seat.
- one session: `--session <id>`.
- every session of the project: `--all`. The mediator uses this.

Name the window: `--days N`, or `--since` and `--until` with UTC stamps.

### 2. Read the rules

Read the rules live, in this order. Do not work from what you remember of them.

1. Memory: `MEMORY.md` and the files it links, under `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<project-key>/memory/`.
2. The global files: `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/CLAUDE.md` and every file it imports.
3. The project's `CLAUDE.md` and `AGENTS.md`.
4. The installed skills: `/in-whiteboard`, `/in-essentials`, and `/in-finish`.
5. The terms: `intent wb profile terms`.
6. The boards: `intent wb show hv --all` for directives and rulings, and the seat's own board for watch-outs and decisions.
7. The guide: `intent llm guide`, for what a verb is for and when it refuses.

Read a rule from the rule library only when a finding names it: `intent claude rules show <id>`.

### 3. Write the project values and the pattern file

Some checks need a value that the project sets. Read each value. Do not guess one. A check that lacks its value does not run, and the output says so.

| Argument     | What it is                                           | Where to read it                                                                      |
| ------------ | ---------------------------------------------------- | ------------------------------------------------------------------------------------- |
| `--checkout` | The shared checkout's path                           | The first line of `git worktree list`                                                 |
| `--views`    | A glob for paths that only a verb writes             | `/in-whiteboard`, "The files", eg `intent/whiteboard/**`                              |
| `--heavy`    | A regular expression for a command that needs a slot | `AGENTS.md`, "Build and Test Commands", eg `^bin/devbin (build\|gate\|test\|release)` |
| `--mediator` | The seat through which hv is reached                 | `intent wb profile terms`, or the roster in `intent wb status`                        |

Give `--views` and `--heavy` once for each value.

Some rules have a mechanical form, eg a banned word, `--no-confirm`, or a direct `cargo` call. Write each one as a line of a pattern file in your scratch folder:

```
# scope :: pattern :: source
prose :: \babsolutely\b :: house-rules.md, House style
command :: --no-confirm :: MEMORY.md, Never --no-confirm
command :: ^(cargo|mix) :: AGENTS.md, Build and Test Commands
```

The scope is `prose` for what the seat wrote, or `command` for a shell command. The pattern is a regular expression, and it ignores case. The source names where the rule lives. Write a line only for a rule you read in step 2.

### 4. Run the script

The script is beside this file:

```bash
elixir "${CLAUDE_CONFIG_DIR:-$HOME/.claude}/skills/in-autopsy/scripts/autopsy.exs" \
  --seat dc --days 7 \
  --checkout /path/to/checkout \
  --views 'intent/whiteboard/**' \
  --heavy '^bin/devbin (build|gate|test|release)' \
  --mediator vc \
  --patterns <scratch>/patterns.txt \
  -o <scratch>/autopsy-findings.json
```

Run it from the project's root, or give `--project <name>`. `--subagents` also reads each session's subagent transcripts. `--help` lists every argument.

The output is one JSON document:

- `sessions`: each session's seat, its window, and who wrote each `user` entry.
- `events`: the protocol events, eg an `intent` verb, a commit, a ref, a worktree act, a heavy run, a sent message, and what a verb printed.
- `findings`: each with its check, session, line, a quote, and the `source` of its rule.
- `checks_run`: each check, the sessions it ran on, and the reason where it did not run.
- `signals`: the corrections, each with who made it: a person, a seat, the tool, or the seat itself.
- `patterns`: each occurrence of a line of the pattern file.
- `unread`: each transcript line the script could not read.

### 5. Judge each finding

A finding is a lead. For each one:

1. Read the rule at the finding's `source`.
2. Read the transcript lines around the finding's line.
3. Keep the finding if the rule covers the act. Drop it if the rule does not, and say why in the report.

The script reads a command's text. It does not see inside a script that the command runs. A wrapper script that books a slot, or a stamp that a wrapper printed, can make a lead false.

### 6. Read the events for what no check can judge

Some rules need a judgement. Read the events for these:

- `intent wb clear` on a message that was not answered or actioned.
- `intent wb add hold` with no condition in its text.
- `intent ac satisfy` on work that was not done.
- A check that a seat ran again after a builder ran it.
- The project's own practice on the board, eg bank and attic refs.
- A ruling that hv gave to a seat, and that the seat did not pass to the mediator.

### 7. Class each finding

- **Enforcement failure**: the session broke a rule that was in force.
- **Rule gap**: a correction has no rule behind it.
- **Tool gap**: the tool could refuse the act and does not, or the remedy in its refusal did not help.
- **Stale rule**: no session in the window touched the rule.

### 8. Write the report

Write the report to `intent/autopsy/YYYYMMDD.md`, with the date from the clock read in step 1. Use the format below. The report has no totals.

### 9. Send the report's path to the mediator

Send the path to the mediator in at most five lines. The mediator puts it to hv. On a board with no team, give the path to the person.

Propose a tool gap as an issue for hv to rule on. Propose a rule gap as a memory note only when it is a standing rule of hv. The mediator is the one writer of memory.

## Report format

```markdown
# Autopsy report: YYYYMMDD

Scope: <seat, session, or team>. Window: <from> to <to>, UTC. Clock read: <stamp> by `date -u`.

## Enforcement failures

### <what happened, in one line>

- Evidence: <session id>, line <n>: "<quote>"
- Rule: <the rule's source, and its text>
- Proposed fix: <one fix>

## Rule gaps

## Tool gaps

## Stale rules

## Corrections

By a person, by a seat, by the tool, and by the seat itself. Each with its session, line, and quote.

## Checks that ran and found nothing

<names>

## Checks that did not run

<name>: <reason>

## Findings dropped

<check>, <session> line <n>: <why the rule does not cover it>

## Leads

Events that this session could not judge.
```

## Important notes

- The script needs Elixir 1.18 or later (`elixir --version`). It uses Elixir's own `JSON` module, so it needs no network.
- Session files are at `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/projects/<project-key>/*.jsonl`.
- The script reads the files and never changes them. It writes only the file that `-o` names.
- The script reads one session at a time.
- A report proposes. It applies nothing.

## Red flags

| Rationalisation                              | Reality                                                                        |
| -------------------------------------------- | ------------------------------------------------------------------------------ |
| "I know the rules, so I can skip step 2."    | A rule changes between sessions. Read it at its source.                        |
| "The script found it, so it is a failure."   | A finding is a lead. Read the rule and the transcript before you keep it.      |
| "No finding means the rule held."            | Only for a check in `checks_run` that ran. Report the checks that did not run. |
| "I will add the memory note for this gap."   | The skill writes no memory. The mediator does, when hv rules.                  |
| "A summary of totals makes a better report." | A count is not evidence. Give each finding with its quote.                     |
