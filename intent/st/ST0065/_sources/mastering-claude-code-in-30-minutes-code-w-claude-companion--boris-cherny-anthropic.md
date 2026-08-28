# Mastering Claude Code in 30 Minutes

**Boris Cherny, Anthropic — Code with Claude, San Francisco, 22 May 2025**
Video: <https://www.youtube.com/watch?v=B_KAEqiC-0Q>

---

## Editor's note

This is an edited companion to the recorded talk. The source is a machine-generated
transcript with substantial speech-recognition damage: "Claude Code" appears variously
as "quad code", "cloud code" and "cla code"; "bash" as "batch"; "tmux" as "T-Mo" and
"T-Max"; "REPL" as "ripple"; "incantation" as "indentation"; "in parallel" as "in perl".
Quoted passages have been cleaned of these errors and lightly trimmed for readability.
Where the original wording could not be recovered with confidence, the point is
paraphrased rather than quoted. Timestamps refer to the video.

Editorial comments appear in marked blocks and are not part of the talk.

---

## The premise: a different class of coding tool

Cherny opened by placing Claude Code against the tools most of the audience already
used.

> "Claude Code is a new kind of AI assistant. There have been different generations of
> AI assistants for coding — most of them have been about completing a line at a time,
> a few lines of code at a time. Claude Code is not for that." _(1:25)_

The claim is about scope rather than quality. Completion tools operate inside the unit
of a line or a block; Claude Code is aimed at the unit of a task — a feature, a
function, a file, a bug.

The second claim is about integration. Claude Code is a terminal program, so it
inherits whatever environment the terminal is already in: VS Code, Xcode, JetBrains,
Vim, a remote SSH session, a tmux pane. Nothing needs to be swapped out.

The third property is the awkward one, and Cherny named it himself: the tool is general
purpose, which makes it hard to start with.

> "You open it up and you just see a prompt bar, and you might wonder: what do I do
> with this? What do I type in?" _(2:26)_

His answer is deliberate. Anthropic does not push users down a prescribed workflow,
because engineers' workflows differ. The rest of the talk is an attempt to fill that
gap with convention rather than constraint.

> **Comment.** This is a real design tension, not a rhetorical one. A blank prompt
> maximises reach and minimises discoverability, and Cherny is trading the second for
> the first. The talk exists because the trade has a cost — the product needs a
> conference session to explain how to hold it. Whether that is a virtue depends
> entirely on how quickly the model absorbs the missing scaffolding.

---

## First five minutes: setup

Four things to run once, on installation _(2:57)_:

| Command               | Effect                                                                                                |
| --------------------- | ----------------------------------------------------------------------------------------------------- |
| `/terminal-setup`     | Binds Shift+Enter to newline, instead of escaping with backslashes                                    |
| `/theme`              | Light, dark and daltonised (colour-blind-friendly) themes                                             |
| `/install-github-app` | Installs the GitHub app announced that morning; lets you @-mention Claude on issues and pull requests |
| Allowed-tools config  | Pre-approves the tools you keep approving by hand                                                     |

The allowed-tools point is the one that compounds. Anything you find yourself approving
repeatedly should be added to the allow-list; the friction otherwise accumulates across
every session.

Cherny then offered a habit that has nothing to do with Claude Code itself:

> "For a lot of my prompts, I won't hand-type them into Claude Code." _(3:40)_

On macOS, he enables dictation under Accessibility, double-taps the dictation key, and
speaks the prompt. The argument is that specific prompts work better than terse ones,
and speech makes specificity cheap.

> **Comment.** The dictation tip is the most transferable thing in the first five
> minutes, and it points at something the talk never states outright: the binding
> constraint on these tools is how much context the user can be bothered to supply.
> Dictation attacks the cost of supplying it. Most of the later advice — CLAUDE.md,
> slash commands, MCP configuration — attacks the same cost from the other direction,
> by making context persistent so it need only be supplied once.

---

## Start with questions, not edits

Asked where a new user should begin, Cherny was unambiguous: codebase Q&A. It is what
Anthropic teaches its own new hires on their first day of technical onboarding — install
the tool, then immediately start asking it questions about the code.

> "At Anthropic, onboarding used to take about two or three weeks for technical hires.
> It's now about two or three days." _(4:56)_

Two design decisions make this work. There is no indexing step: no remote database of
your code, no upload, no wait before first use. Claude Code explores the repository at
question time using ordinary file and search tools. Anthropic does not train generative
models on the code.

The examples he gave show what "exploring" buys you over grep:

- _How is this class instantiated?_ — Claude looks for real call sites rather than
  matching a string, so the answer resembles documentation rather than a search result.
- _Why does this function have fifteen arguments, and why are they named this way?_ —
  Claude reads the git history to find when each argument arrived, who added it, and
  which issues the commits reference.
- _What did I ship this week?_ — a Monday standup habit. Claude reads the log, knows
  the username, and produces a summary to paste into a document. _(7:02)_

On the git-history case, Cherny made a point he returned to twice more:

> "There's nothing in the system prompt about looking through git history. It knows
> because the model is good." _(6:38)_

His onboarding advice for teams follows from this:

> "Don't start by using fancy tools. Don't start by editing code. Just start by asking
> questions about the codebase." _(7:20)_

Q&A teaches prompting cheaply, and it teaches the boundary — what can be one-shot, what
needs two or three attempts, and what needs interactive supervision.

> **Comment.** Two things deserve scrutiny here.
>
> The onboarding figure — three weeks to three days — is self-reported, unmeasured, and
> comes from a company whose engineers are unusually motivated to make the tool work.
> Treat it as a direction of travel, not a benchmark. The underlying mechanism is
> plausible and more modest than the number: Q&A shifts load from senior engineers'
> attention to compute.
>
> The "no indexing" claim is presented purely as a privacy benefit, which understates
> it. It is a genuine trade-off. No index means no setup and no code leaving the
> machine; it also means every question pays for exploration in tokens and latency, and
> that cost scales with repository size. On a large monorepo the difference is
> noticeable. The privacy framing is true, but it is not the whole argument.

---

## Editing code, and asking for a plan first

Claude Code has a small tool set — edit files, run bash commands, search files — and
chains them itself. Cherny's advice is not to specify the chain.

> "You don't have to prompt it to use this tool and this tool. Just say 'do this thing',
> and it'll figure out how to do it." _(8:24)_

The recommendation he stressed most in this section is to interpose thinking before
code. He described the common failure: someone asks for an enormous feature in one shot,
and Claude builds something coherent that is not what they wanted.

> "The easiest way to get the result you want is to ask it to think first. Brainstorm
> ideas, make a plan, run it by me, ask for approval before you write code." _(9:03)_

No special mode was required to do this in May 2025. Plain instruction was enough.

He also offered his standard shorthand for shipping — an instruction along the lines of
"commit, push, PR". Claude reads the recent log to infer commit message conventions,
creates the branch, commits, pushes and opens the pull request without further
explanation. Again, not system-prompted.

> **Comment.** "The model is good, we didn't prompt it" is the talk's recurring motif,
> and it is doing double duty: a genuine engineering observation, and a marketing claim
> about model quality over harness quality. The observation is worth taking seriously —
> most of the behaviour is emergent, and the harness is thin by design. The claim is
> weaker than it sounds. The system prompt, the tool descriptions, the permission model
> and the context-injection rules all shape behaviour heavily, and Cherny spends the
> back half of this talk explaining how to configure exactly those things. A harness
> that needs a conference session to explain is not a neutral conduit for model
> capability.

---

## Give it your team's tools

Two categories, both taught by description rather than integration work _(9:56)_:

**CLI tools.** Tell Claude the command exists and to run `--help` to learn it. Cherny
used an invented `barley` CLI for the example. If you use it often, record it in
CLAUDE.md so the knowledge survives the session.

**MCP servers.** Add the server, describe what it is for, and Claude starts using it.

> "When you start to use Claude Code on a new codebase, you can just give it all the
> tools your team already uses, and Claude Code can use them on your behalf." _(10:45)_

---

## Feedback loops: the highest-leverage pattern in the talk

Cherny sketched three workflows and was explicit that two of them matter more than the
first. Exploration and planning is the baseline. The other two depend on Claude having
some way to check its own output — unit tests, a Puppeteer screenshot, an iOS simulator
capture.

> "If you give it a mock and say 'build this web UI', it'll get it pretty good. But if
> you let it iterate two or three times, often it gets it almost perfect." _(11:17)_

The general form, stated in his own summary: whatever the domain, give it a way to see
its result, and it will iterate without further instruction.

> **Comment.** This is the load-bearing idea, and it is the one most likely to survive
> every product change since. Everything else in the talk is context plumbing.
> Verification changes the shape of the problem: it converts a one-shot generation
> task, where quality is capped by the model's first guess, into a search over
> candidates with a fitness function. The practical implication is uncomfortable for
> most teams — the value you extract from an agentic tool is bounded by the quality of
> your test suite and your ability to make results machine-checkable. Codebases with
> fast, meaningful tests get compounding returns. Codebases without them get a
> confident intern.

---

## Context: CLAUDE.md and the hierarchy

The more context Claude has, the better its decisions, because an engineer's working
knowledge of a system is mostly undocumented. `CLAUDE.md` is the simplest way to
transfer some of it. _(12:17)_

**How it loads:**

- `CLAUDE.md` in the project root is read into context at the start of every session,
  as part of the first user turn.
- `CLAUDE.local.md` holds personal notes and is not checked in.
- `CLAUDE.md` files in nested directories are pulled in on demand, when Claude works in
  those directories.
- An enterprise-level file can be pushed to every repository and every employee.

**What goes in it:** common bash commands, MCP tools in use, architectural decisions,
core files, style guide — the things a new engineer would need to know. Anthropic's own
file contains roughly that.

**What matters most:** keep it short.

> "If it gets too long, it's just going to use up a bunch of context and it's usually
> not that useful." _(13:24)_

Beyond CLAUDE.md, context arrives by several other routes _(14:17)_:

- **Slash commands** in `.claude/commands`, either in your home directory or checked
  into the project. Claude Code's own repository uses one to label incoming GitHub
  issues, run automatically by a GitHub Action, so humans do not have to.
- **@-mentioning** files and folders inline.
- **Nested CLAUDE.md files**, as above.

He recommended treating this material as a prompt to be engineered, not a README to be
written: run it through a prompt improver, and decide deliberately who each piece is
for, whether it should load every time or on demand, and whether it is a team standard
or a personal preference.

The same project / global / enterprise hierarchy applies to settings as well as
context — slash commands, permissions and MCP servers all resolve through it _(15:42)_.
The permissions case is the practical one:

- Auto-approve a command for everyone by putting it in enterprise policy. If every
  engineer runs the same test command, nobody should approve it by hand.
- Block a command or a URL at enterprise level, and no individual can override it.
- Check `.mcp.json` into the repository, and anyone running Claude Code there is
  prompted to install the servers the team uses.

Cherny conceded the resulting matrix is dense, and gave a default: start with shared
project context. One person's configuration work benefits everyone on the team.

Two commands manage the result. `/memory` lists every memory file currently in play —
enterprise policy, user memory, project CLAUDE.md, nested files — and lets you edit
them. Typing `#` followed by an instruction records it, and lets you choose which
memory file it lands in. _(17:35)_

The worked example: Anthropic's apps repository ships a Puppeteer MCP server with
`.mcp.json` checked in, so every engineer in that repository can drive end-to-end tests
and screenshot the UI without installing anything.

> **Comment.** There is an unresolved tension in this section. "Keep CLAUDE.md short,
> because context is scarce" sits directly against a four-tier hierarchy that
> encourages enterprise files, project files, personal files and nested files to
> accumulate independently, each maintained by different people with different
> incentives. Nobody owns the total. The `/memory` command exists precisely because
> that total becomes hard to see, which is a tell.
>
> The general lesson is one every configuration system eventually teaches: hierarchies
> are easy to add to and hard to prune. If you adopt this, budget the context the way
> you would budget a build time — measure the aggregate, and make someone responsible
> for it.

---

## Key bindings

Terminal UI is minimal, and Cherny noted that discoverability suffers as a result.
His reference sheet _(18:46)_:

| Binding             | Effect                                                                                                                                                         |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Shift+Tab`         | Switch to auto-accept-edits mode. Bash commands still require approval; edits do not. Use it when Claude is clearly on track, or when it is iterating on tests |
| `#`                 | Remember this. Claude writes it into CLAUDE.md, and you choose which file                                                                                      |
| `!`                 | Drop to bash. The command runs locally and enters the context window, so Claude sees the command and its output on the next turn                               |
| `@`                 | Mention a file or folder to pull it into context                                                                                                               |
| `Esc`               | Stop whatever Claude is doing. Always safe — it will not corrupt the session                                                                                   |
| `Esc` `Esc`         | Jump back in history                                                                                                                                           |
| `Ctrl+R`            | Show full output, the same view Claude has in its context                                                                                                      |
| `claude --resume`   | Resume a specific past session                                                                                                                                 |
| `claude --continue` | Continue the most recent session                                                                                                                               |

The `Esc` habit is worth isolating. Cherny described using it mid-edit — Claude proposes
a twenty-line change, nineteen lines are right, so he interrupts, corrects the one line,
and asks it to redo the edit. Interruption is a normal part of the loop, not a failure
of it.

---

## The SDK: Claude Code as a Unix utility

The `-p` flag is the SDK. It is the same SDK Claude Code itself runs on. _(20:53)_

```bash
claude -p "your prompt" \
  --allowedTools "Bash(git log:*)" \
  --output-format json
```

Output can be JSON or streaming JSON. Anthropic uses this in CI, in incident response,
and across internal pipelines.

> "Just think of it as a Unix utility. You give it a prompt, it gives you JSON. You can
> pipe into it, you can pipe out of it." _(21:46)_

The composition examples: pipe `git status` in and select the result with `jq`; pull a
large log out of a GCP bucket and ask Claude what is interesting in it; pipe data from
the Sentry CLI and have Claude act on it.

> "It's a super-intelligent Unix utility, and I think we've barely scratched the
> surface of how to use this." _(22:09)_

---

## Parallelism

The last section covered the most advanced usage Cherny observes internally — and he
was clear it is not how he works himself.

> "I'm sort of a Claude normie. I'll usually have one Claude running at a time, maybe a
> few terminal tabs for a few different repos." _(22:38)_

Power users at Anthropic run SSH sessions and tmux into their Claude sessions, keep
multiple checkouts of the same repository so several instances can work in it at once,
or use git worktrees for isolation. He noted Anthropic was working on making this
easier, and that in the meantime the ceiling is high: run as many sessions as you want.

---

## Questions from the floor

**What was the hardest part to build?** _(24:00)_

Making bash safe. Bash changes system state in unexpected ways, but requiring manual
approval of every command destroys productivity — and the solution has to work across
codebases with very different practices, since not everyone runs their code in a
container. What Anthropic landed on has three parts: a set of commands classified
read-only; static analysis to determine which commands can be safely combined; and a
tiered permission system supporting allow-lists and block-lists at several levels.

**Is Claude Code multimodal?** _(25:05)_

Yes, and it has been from the start — the terminal just makes it hard to discover. Drag
and drop an image, give a file path, or paste it. Cherny's own use: drop in a mock, ask
for an implementation, give Claude a browser-automation server so it can screenshot and
iterate against the mock.

**Why a CLI rather than an IDE?** _(25:48)_

Two reasons. First, Anthropic engineers use a wide spread of editors — VS Code, Zed,
Xcode, Vim, Emacs — and the terminal is the common denominator. Second, and more
pointed:

> "We see up close how fast the model is getting better, and I think there's a good
> chance that by the end of the year people aren't using IDEs any more." _(26:11)_

The strategic conclusion: avoid over-investing in UI layers that model progress may
render redundant.

**Do researchers use it for ML work?** _(26:42)_

Yes. Roughly 80% of technical staff at Anthropic used Claude Code daily at the time of
the talk, researchers included, with a notebook tool for editing and running notebooks.

> **Comment.** The IDE prediction is the most falsifiable statement in the talk, and it
> was wrong — see the note below. It is worth reading it charitably anyway. Cherny was
> not predicting that editors would vanish; he was justifying a resource-allocation
> decision under uncertainty about model capability. That decision looks sound even
> though the prediction failed, which is a useful separation to keep in mind when
> evaluating anyone's roadmap reasoning.

---

## What has changed since May 2025

This talk predates a great deal. Read it for the principles, not the API surface. As of
August 2026, the main shifts:

- **Plan mode is now a first-class mode.** Cherny's advice to ask for a plan in plain
  English still works, but `Shift+Tab` now cycles through permission modes rather than
  simply toggling auto-accept.
- **The Claude Code SDK was renamed the Claude Agent SDK**, and has gained first-class
  support for subagents and hooks. The `-p` composition patterns above still hold.
- **Subagents and hooks** now cover delegation and lifecycle interception, which the
  talk handles only through parallel sessions and prompting.
- **Checkpointing** saves code state before each change; `Esc` `Esc` or `/rewind`
  restores it. Checkpoints cover Claude's edits, not user edits or bash commands.
- **Skills** provide reusable, packaged instructions — a more structured answer to the
  context problem than nested CLAUDE.md files.
- **Claude Code runs well outside the terminal**, including editor extensions and the
  desktop app. The prediction that IDEs would be gone by the end of 2025 did not hold;
  Anthropic built for both.
- **Cherny is now Head of Claude Code at Anthropic.**

The parts that have aged well: start with codebase Q&A; ask for a plan before code;
give the model a way to check its own work; write context once and share it; treat the
CLI as a Unix utility. The parts that have aged badly are, without exception,
predictions about the shape of the interface.

---

## Appendix: one-page reference

**Setup**

```
/terminal-setup        # Shift+Enter for newlines
/theme                 # light / dark / daltonised
/install-github-app    # @-mention Claude on issues and PRs
/memory                # list and edit active memory files
```

**In-session keys**

```
Shift+Tab   auto-accept edits
#           remember this
!           run a bash command (enters context)
@           mention a file or folder
Esc         stop safely
Esc Esc     jump back in history
Ctrl+R      show full output
```

**Resuming**

```
claude --resume     # pick a past session
claude --continue   # continue the most recent one
```

**Context files**

```
./CLAUDE.md            checked in, loaded every session
./CLAUDE.local.md      personal, not checked in
./**/CLAUDE.md         loaded on demand, per directory
enterprise policy      pushed to all repos and users
.claude/commands/      slash commands, personal or checked in
.mcp.json              MCP servers, shared with the team
```

**Scripting**

```
claude -p "prompt" --allowedTools "..." --output-format json
git status | claude -p "summarise" --output-format json | jq ...
```
