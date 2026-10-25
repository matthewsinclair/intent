# Working with coding agents

**Intent's answer to "how does my agent know what this project is doing" is that the file the agent reads is generated from the project rather than maintained beside it.**

This page is the overview. The deep guides live at `intent/docs/` in the Intent repository and are linked from each section.

## The problem, stated precisely

An agent reading your repository sees what the code does. It cannot see what you were trying to achieve, what you rejected, or what breaks if it changes something — because none of that is in the code, and the places it usually lives are places nothing checks.

The usual fix is a context file: `AGENTS.md`, `CLAUDE.md`, a `.cursorrules`. **That works exactly until it stops matching the project**, which happens quietly and is not detectable by reading either the file or the code. A stale context file is worse than none, because the agent acts on it with confidence.

**So Intent does not ask you to maintain one.**

## The generated contract

```
  $ intent agents sync
```

This regenerates `AGENTS.md` from Intent's template and the project's configuration: its name, its author and its declared languages, each language adding its toolchain's commands. **Do not hand-edit it; the next sync overwrites you.** It is written at your repository root, or in `.claude/` when the project's `contracts` key is `dotclaude`, for a tree whose root people read rather than tools: `intent init --contracts dotclaude` sets that at birth, and `intent agents layout <root|dotclaude>` moves an existing project between the two. `CLAUDE.md` always sits beside it.

The contract is split across these files, and the split is deliberate:

| File             | What it is                                                   | Who writes it                                                                                          |
| ---------------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `AGENTS.md`      | The tool-agnostic contract, for agents that read `AGENTS.md` | `intent init`, then `intent agents sync` or `intent claude upgrade --apply`                            |
| `CLAUDE.md`      | A Claude Code overlay, adding what is Claude-specific        | `intent init`, then `intent claude upgrade --apply`, keeping what you write between its `user` markers |
| `usage-rules.md` | Terse DO / NEVER rules, an Elixir-community convention       | Seeded by `intent claude upgrade --apply` when absent, then yours                                      |

`intent claude upgrade` reports what it would write and changes nothing until you pass `--apply`. With `--apply` it also writes `.intent_critic.yml`, the pre-commit gate and the post-pull hooks (`post-merge`, `post-checkout`, `post-rewrite`, which run `intent sync --apply` for you), and it wires the Claude Code harness, session hooks in `.claude/settings.json` and the MCP server in `.mcp.json`, unless you pass `--skip-settings`, which leaves both alone. **A `CLAUDE.md` you wrote yourself, without the generated footer, is held back** rather than overwritten; `--force` overwrites it.

**`CLAUDE.md` imports `AGENTS.md`.** Claude Code reads `AGENTS.md` only where there is no `CLAUDE.md`, so a generated `CLAUDE.md` opens with an `@AGENTS.md` line, which loads it. The index of the agnostic principles lives in `AGENTS.md` alone and `CLAUDE.md` points at it; the rule bodies have one home, the rule library. The templated `.claude/settings.json` denies Claude Code edits to the generated `AGENTS.md`, in either layout.

`intent agents validate` checks that `AGENTS.md` is present and carries its required sections.

**Closing a project:** `intent deinit` lists what Intent wrote into it, the contracts, the seeds still as seeded, Intent's entries in `.claude/settings.json` and `.mcp.json`, and its git hook blocks, and writes nothing. `intent deinit --apply` removes and edits exactly that, keeping anything a person wrote, and `--all` also removes `intent/` with the store.

**Deeper:** [`intent/docs/working-with-llms.md`](../intent/docs/working-with-llms.md) is the full narrative — why the three files are split this way, how session hooks compose with them, and the decisions behind the architecture.

## The rule library

Rules are first-class objects, not prose in a context file. They are versioned, addressable, and read by three different consumers — you, your agent, and the commit gate.

```
  $ intent claude rules list --lang rust
  $ intent claude rules show IN-AG-HIGHLANDER-001
```

The agnostic principles the language packs concretise are **Highlander** (one canonical home per concern), **PFIC** (Pure Function, Impure Coordination: a deterministic core, with I/O at the boundary), **Thin Coordinator** (parse, call, render — logic lives elsewhere), and **No Silent Errors**. `intent claude rules list --lang agnostic` lists the whole agnostic pack.

Which packs load is driven by what the project declares:

```
  $ intent lang list
  $ intent lang init rust
```

**Language is a declaration, not a detection.** Intent will not infer it from the files present, because file presence is unreliable evidence and a wrong guess loads the wrong rules.

**Deeper:** [`intent/docs/rules.md`](../intent/docs/rules.md) — the rule schema, how Detection heuristics are written, and what makes a rule mechanically checkable rather than merely stated.

## Critics

A critic reads the rule library at invocation and applies it to files you name, or to the files staged for commit.

```
  $ intent critic rust --files src/parser.rs
  $ intent critic rust --staged
```

Every run opens by saying how many of the language's rules it could apply and how many it could not, naming each rule it could have run and did not, so a clean result is read against what was asked.

Inside Claude Code the same critics are subagents:

```
  Task(subagent_type="critic-rust", prompt="review src/parser.rs")
```

**Critics read the library rather than embedding it**, so a rule fixed once is fixed for every critic that applies it — including the headless runner the commit gate uses.

**One limit worth knowing before you rely on them.** The headless runner honours only rules whose detection is a greppable pattern or a named tool it can run, such as `shellcheck` or `clippy`, and it names a tool-armed rule it did not run. A rule whose real check needs judgement is not silently approximated by a weaker grep — the runner refuses it and says so. **A mechanical check standing in for a judgement it cannot make is worse than no check**, because it reports clean.

**Deeper:** [`intent/docs/critics.md`](../intent/docs/critics.md) — the critic contract and how to write one.

## The commit gate

`intent claude upgrade --apply` installs a pre-commit gate. It runs the critics for your declared languages over the staged files, and the guards on Intent's roster, each only where its subject exists: a whiteboard timestamp that did not come from a clock, a whiteboard header written as escaped YAML, an ignore rule that would hide `intent/.canon/`, lines removed from an append-only path, and staged files that do not match the formatters the project declares. It also runs `intent doctor`, refusing a commit whose views disagree with the store, and any guards the project declares under `guards` in `intent/.config/config.json`.

**They are backstops on specific failures, not a review.** Passing them means you did not do one of a short list of known-bad things. The guard bodies are read from the installed tool at commit time, through the install pointer `intent bootstrap` records, so a fixed guard reaches your project as soon as the Intent on this machine carries it, without reinstalling the hook.

**Deeper:** [`intent/docs/pre-commit-hook.md`](../intent/docs/pre-commit-hook.md).

## Skills and custom agents

For Claude Code, Intent installs procedural skills — a session bootstrap that loads the right rule packs, a review pipeline, a verification pass.

```
  $ intent claude skills install in-essentials
  $ intent claude skills sync
```

**Never hand-copy a skill into `${CLAUDE_CONFIG_DIR:-$HOME/.claude}/skills/`**, where Intent installs them. They are tracked by checksum: `intent claude skills sync` names a copy it did not write, and holds a skill modified since it was installed, rather than overwriting either.

**Deeper:** [`intent/docs/creating-custom-agents.md`](../intent/docs/creating-custom-agents.md) for project-specific agents. User extensions at `~/.local/share/intent/ext/` are declared and not built: `intent ext` answers every subcommand with _a known command that is not implemented yet_.

## Several sessions in one checkout

When more than one agent works the same checkout, the index, the stash and the reflog are shared objects, and a finished change that cannot land yet needs a home that is none of them. [Banking work](banking.md) is the policy: the change is a patch stored as a git blob under `refs/bank/`, judged at one patch-id, and landed on the director's word with `git commit --only`.

## Every session on this machine

`intent estate status` lists every Claude Code session running on this machine, whatever project it is in: its project and whiteboard node, whether it is busy, idle, in a heavy run, paused or gone, how long it has run, its last heartbeat, its claims and what it is doing. A session that is a whiteboard node shows its board's focus. A session outside any whiteboard can say what it is doing with `intent estate wip "<line>"`, and `--clear` removes the line. `--format json` prints the same answer for an agent, which reads it before starting something that contends with the others, such as a heavy build.

`intent estate status --browser` opens the same answer as a page served by a running `intentd`, at `/estate/status`. It shows the machine's load, the last hour of load and running slots, plan usage, the queue of requests for a slot, the lanes and the sessions, and it reads the estate again every few seconds while it is open. Each panel folds, and a folded panel's heading keeps its one-line reading. In the explorer, `/estate` shows the sessions as a list, as [the explorer guide](explorer.md) describes.

The census on that page counts the process classes the machine's controller declares, and nothing is built in: `intent estate watch <class> <pattern>` declares a class, or changes its pattern, as a regular expression matched against each process's command line, and `intent estate unwatch <class>` removes it. Plan usage comes from the account's usage endpoint when the daemon can read it; the fallback is the statusline step, where a Claude Code statusline script pipes the JSON it is handed to `intent estate statusline || :`. That verb prints nothing on stdout, so the status line stays the script's own.

## What this buys, and what it does not

**It buys a context file that does not silently drift**, because it is generated from something that would have to change first and `intent doctor` says when it is behind, and rules with one home that three consumers read.

**It does not buy correctness.** An agent with perfect context can still write bad code. The claim is narrower and worth stating plainly: the agent is working from what the project actually says about itself, rather than from what somebody wrote down once and nobody has checked since.

---

Back to [the documentation index](index.md).
