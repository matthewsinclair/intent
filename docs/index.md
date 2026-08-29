# Intent

**Intent captures why your code exists, in the repository, in a form your team and your coding agents can both read.**

Your code says what it does. Version control says when it changed and who changed it. Neither says what you were trying to achieve, what you considered and ruled out, or what breaks if someone changes it. That reasoning usually lives in a head, a chat log, or a design doc that stopped matching the code months ago.

Intent gives it a place to live that ages with the code instead of away from it.

```
  $ brew install matthewsinclair/intent/intent
```

## Start here

| If you want to                               | Read                                                 |
| -------------------------------------------- | ---------------------------------------------------- |
| Get it installed                             | [Installation](install.md)                           |
| See it work on a real project                | [Getting started](getting-started.md)                |
| Understand the model before you commit to it | [Concepts](concepts/)                                |
| Wire it into your coding agent               | [Working with coding agents](working-with-agents.md) |
| Move a v2 project to v3                      | [Migrating from v2](migrating-from-v2.md)            |
| Look up a command                            | [Command reference](reference/)                      |

## What it actually is

A single CLI, written in Rust, that manages a small set of durable objects inside your repository.

**A steel thread** is one intention followed end to end — what you are trying to achieve and why it matters. It breaks into **work packages**, the units of work that get done. It states **acceptance criteria**, which are the conditions that decide whether the intention was met. Each criterion is backed by an **acceptance test**, so Intent can compute whether a thread is satisfied rather than asking you to assert it.

Around that sits the machinery that makes it survive contact with real projects: a store that is the single source of truth, generated views so nothing is hand-maintained twice, a rule library your coding agents can be held to, per-language critics that check work against those rules, and commit-time gates that refuse changes contradicting what the project said it was doing.

## Why it exists

**Comments rot because nothing checks them.** They sit beside the code, they are not tested, and the first person to change the code under a stale comment usually leaves it there.

**Design documents rot because nothing links them.** They sit apart from the code in a wiki or a drive, they are correct on the day they are written, and there is no mechanism anywhere that notices when they stop being true.

**And an AI coding agent cannot reconstruct any of it.** It reads what the code does and builds confidently on assumptions you would have rejected in one sentence, because the sentence was never written down anywhere it could read. Giving an agent more context each session does not fix this; the context has to come from something that cannot silently drift.

Intent's answer is to make the reasoning a tracked object with a state the tool computes. A criterion is either satisfied by a test that runs, or it is not, and the difference is visible without anyone remembering to look.

## What this documentation covers

This set describes **Intent v3**, the Rust implementation. Intent v2 was a Bash implementation whose final release was v2.19.0; it is documented separately and frozen at [`docs/v2/`](v2/). The two are not interchangeable — a v2 binary refuses a v3 project deliberately, and [Migrating from v2](migrating-from-v2.md) describes the hop.

Deeper authoring guides — writing rules, writing critics, writing extensions, and the full narrative on how Intent composes with coding agents — ship inside the tool and are documented at `intent/docs/` in this repository. This set links across to them rather than duplicating them.

---

_Intent is by [Matthew Sinclair](https://github.com/matthewsinclair). Source at [github.com/matthewsinclair/intent](https://github.com/matthewsinclair/intent)._
