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

This regenerates `AGENTS.md` at your repository root from the project's actual state — its declared languages, its installed skills and subagents, its rules. **Do not hand-edit it; the next sync overwrites you.**

Three files make up the contract, and the split is deliberate:

| File             | What it is                                             | Who writes it                                                                                          |
| ---------------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `AGENTS.md`      | The tool-agnostic contract. Read first by any agent    | `intent init`, then `intent agents sync`                                                               |
| `CLAUDE.md`      | A Claude Code overlay, adding what is Claude-specific  | `intent init`, then `intent claude upgrade --apply`, keeping what you write between its `user` markers |
| `usage-rules.md` | Terse DO / NEVER rules, an Elixir-community convention | Seeded by `intent claude upgrade --apply` when absent, then yours                                      |

`intent claude upgrade` reports what it would write and changes nothing until you pass `--apply`. With `--apply` it also writes `.intent_critic.yml` and the pre-commit gate, and wires Claude Code session hooks into `.claude/settings.json` unless you pass `--skip-settings`. **A `CLAUDE.md` you wrote yourself, without the generated footer, is held back** rather than overwritten; `--force` overwrites it.

**One index is stated twice, on purpose.** `CLAUDE.md` and `AGENTS.md` both carry the short index of the four agnostic rules, because `AGENTS.md` is not a file Claude Code reads. The rule bodies have one home, the rule library, and the two indexes are held identical by a test rather than by care.

`intent agents validate` checks that `AGENTS.md` is present and carries its required sections.

**Deeper:** [`intent/docs/working-with-llms.md`](../intent/docs/working-with-llms.md) is the full narrative — why the three files are split this way, how session hooks compose with them, and the decisions behind the architecture.

## The rule library

Rules are first-class objects, not prose in a context file. They are versioned, addressable, and read by three different consumers — you, your agent, and the commit gate.

```
  $ intent claude rules list --lang rust
  $ intent claude rules show IN-AG-HIGHLANDER-001
```

Four agnostic rules underpin the language packs: **Highlander** (one canonical home per concern), **PFIC** (Pure Function, Impure Coordination: a deterministic core, with I/O at the boundary), **Thin Coordinator** (parse, call, render — logic lives elsewhere), and **No Silent Errors**. Each language pack concretises them.

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

Every run opens by saying how many of the language's rules it could apply and which it could not, so a clean result is read against what was asked.

Inside Claude Code the same critics are subagents:

```
  Task(subagent_type="critic-rust", prompt="review src/parser.rs")
```

**Critics read the library rather than embedding it**, so a rule fixed once is fixed for every critic that applies it — including the headless runner the commit gate uses.

**One limit worth knowing before you rely on them.** The headless runner honours only rules whose detection is a simple, greppable pattern. A rule whose real check needs judgement is not silently approximated by a weaker grep — the runner refuses it and says so. **A mechanical check standing in for a judgement it cannot make is worse than no check**, because it reports clean.

**Deeper:** [`intent/docs/critics.md`](../intent/docs/critics.md) — the critic contract and how to write one.

## The commit gate

`intent claude upgrade --apply` installs a pre-commit gate. It runs the critics for your declared languages over the staged files, and four guards, each only where its subject exists: a whiteboard timestamp that did not come from a clock, a whiteboard header written as escaped YAML, an ignore rule that would hide `intent/.canon/`, and lines removed from an append-only path.

**They are backstops on specific failures, not a review.** Passing them means you did not do one of a short list of known-bad things. The guard bodies are read from the installed tool at commit time, so a fixed guard reaches your project on the next `intent upgrade` without reinstalling the hook.

**Deeper:** [`intent/docs/pre-commit-hook.md`](../intent/docs/pre-commit-hook.md).

## Skills and custom agents

For Claude Code, Intent installs procedural skills — a session bootstrap that loads the right rule packs, a review pipeline, a verification pass.

```
  $ intent claude skills install in-standards
  $ intent claude skills sync
```

**Never hand-copy a skill into `.claude/skills/`.** They are tracked by checksum and a hand-placed copy diverges silently.

**Deeper:** [`intent/docs/creating-custom-agents.md`](../intent/docs/creating-custom-agents.md) for project-specific agents. User extensions at `~/.intent/ext/` are declared and not built: `intent ext` answers every subcommand with _a known command that is not implemented yet_.

## What this buys, and what it does not

**It buys a context file that cannot silently drift**, because it is generated from something that would have to change first, and rules with one home that three consumers read.

**It does not buy correctness.** An agent with perfect context can still write bad code. The claim is narrower and worth stating plainly: the agent is working from what the project actually says about itself, rather than from what somebody wrote down once and nobody has checked since.

---

Back to [the documentation index](index.md).
