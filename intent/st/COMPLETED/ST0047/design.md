# Design - ST0047: claude_with_intent (MAAC launcher + workstream lifecycle)

## Approach

Prototype-in-Baize-then-promote, on the shoulders of Lamplight. **Lamplight (`../Lamplight/intent/whiteboard`) pioneered the whiteboard process by convention** -- five hand-run nodes -- and is the working prototype plus the reference for how MAAC actually runs; consult it for any practical question about the process. **Baize is the MVP**: the first productised version, standing the process up as a first-class Intent capability rather than by hand. Build `bin/claude_with_intent` (POSIX shell, matching the `bin/cli` + `bin/repl` house style) in Baize and dogfood it against the hand-scaffolded `hv` + `cc` + `ic` + `vc` board (the golden reference). Once proven, relocate the capability into Intent's `intent claude` command family and back-fill the siblings. Document-before-coding: this ST is the spec; no script code until it is reviewed.

All Claude Code CLI facts below were verified against the installed `claude` (v2.1.191) -- `claude --help` flag lists + the gate source -- not assumed. The one sub-agent claim that did not survive verification (a `--append-system-prompt-file` flag, and a `/in-session` "deadlock") is recorded under Alternatives.

## Design decisions

### D1 -- workstream == whiteboard node (vocabulary unification)

A "workstream" **is** the Whiteboard Protocol 3.0 "node"; `<wsid>` is the node moniker, the directory name, and the routing key. Two refinements over the original protocol wording:

- **Node ids are short-ish slugs, not fixed two-character.** `cc` / `ic` / `vc` are fine; so are `backend` / `infra` / `db`. Validate: lowercase, `[a-z0-9-]`, reasonable length.
- **`hv` is Workstream Zero** -- the always-present human node, provisioned by default whenever a project's whiteboard is first created. Working nodes are **made to order** (only on an explicit `ws new`), never assumed.

Rationale: one concept under two names is drift waiting to happen; unifying on the protocol's model is what lets the script and the skill share a format and never diverge. `ws new` is the automated form of the protocol's "Scaffolding a node".

### D2 -- script vs skill Highlander split

| Surface                         | Owns                                                                                                                                            | Why                                |
| ------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------- |
| `/in-whiteboard` **skill**      | Claude-driven protocol ops needing judgement: pickup / ask / announce / decide / claim / clear / the _semantic_ archive (deciding what is DONE) | These need a model in the loop     |
| `claude_with_intent` **script** | Deterministic lifecycle: `ws new` / `ws list` / `ws archive` / `ws hygiene`; and `start` (launch)                                               | Mechanical, idempotent, scriptable |

Both honour **one SSOT**: the Protocol 3.0 on-disk format. The skill's hand-scaffold prose points at the script (no second scaffolder grows). Highlander: there is exactly one place that knows how a node directory is shaped.

### D3 -- the launch mechanism (verified, Claude Code 2.1.191)

The single command `start` emits:

```
claude --effort max --permission-mode auto --append-system-prompt "$(compose_ctx <ws>)" "/in-session"
```

- `--effort max` -- verified flag (`low|medium|high|xhigh|max`).
- `--permission-mode auto` -- verified choice (`acceptEdits|auto|bypassPermissions|default|dontAsk|plan`); the human-intended mode (see D4).
- `--append-system-prompt "$(...)"` -- **there is no `--append-system-prompt-file` in 2.1.191**; the composed context is inlined via command substitution (multi-line safe under quotes).
- seed `"/in-session"` -- a slash command, admitted by the gate's slash-exemption (see D5), so it runs first: loads skills, releases the sentinel, and (skill step 5) chains `/in-whiteboard pickup`.
- **context** (`compose_ctx`) = the workstream identity line + `.claude/restart.md` + a standing instruction ("after /in-session, ULTRATHINK a daily plan for this workstream, then wait"). The **live board is read by pickup**, not baked into the context -- avoids snapshot drift.
- **interactive**: no `-p`, so the session seeds the first turn then waits for the human.
- **testability seam**: a `CWI_DRY_RUN` (or `--dry-run`) prints the assembled `claude ...` argv instead of exec-ing it -- the unit-test hook for the launcher and a safe preview.
- **OPEN SPIKE**: whether a bare `/in-session` seed flows straight into "show the plan" off the appended instruction (in-session step 6 = "proceed with what the user asked"), or needs one human nudge. Fallback: `INTENT_SKIP_IN_SESSION_GATE=1` + a prose seed that names `/in-session` as action 1. Settled empirically in WP-02.

### D4 -- auto-mode + multi-session safety

`--permission-mode auto` is the hv-intended posture: **not** `acceptEdits` ("accept edits on", ruled too permissive), **not** `bypassPermissions`. The exact semantic difference between `auto` and `acceptEdits` is not documented in `--help`; confirm behaviourally on first launch (TUI status line) and lock the value in WP-02. N concurrent sessions share one umbrella `_build` + dev DB: the per-project **namespace split** + **announce-first** on shared layers mitigate edit/build races; per-workstream git worktrees are a deferred hardening (they fragment the shared DB + the shared `intent/whiteboard`).

### D5 -- the gate interaction (no deadlock)

`require-in-session.sh` exempts slash commands (`case "$prompt" in /*) exit 0`) and honours `INTENT_SKIP_IN_SESSION_GATE` (for `claude -p` automation). The launcher deliberately does **not** set the bypass -- it _wants_ `/in-session` to run -- and relies on the slash-exemption for the seed. SessionStart runs before any prompt; UserPromptSubmit fires on the seed but passes it (slash). The naive "paste prose first" manual form fails precisely because prose is not slash-exempt; `start` sidesteps it.

### D6 -- provision-if-absent (`start`)

`start <ws>`: if `<ws>` exists -> launch. Else -> stop, report "workstream `<ws>` does not exist", prompt "create it? [y/N]". `n` -> exit, no mutation. `y` -> `ws new <ws>` then launch. No silent auto-create.

### D7 -- promotion home

The existing `intent claude` family (it already serves `rules` + `skills`) is the home -> `intent claude start|ws ...`. The Baize `bin/claude_with_intent` prototype maps 1:1. `intent claude`'s dispatch is not a `bin/intent_claude` file (it routes elsewhere) -- confirm its location in WP-04 before relocating.

## Architecture

### CLI surface

```
claude_with_intent start|st <ws>      launch a workstream session (provision-if-absent prompt)
claude_with_intent ws new <wsid>      scaffold a new workstream (== protocol node)
claude_with_intent ws list            list workstreams + status (reads each wip.md frontmatter)
claude_with_intent ws archive <wsid>  retire a workstream, keep .history/
claude_with_intent ws hygiene [<ws>]  mechanical whiteboard lint + tidy
```

### Layout + dispatch

- Baize prototype: `bin/claude_with_intent` (POSIX `sh`; `cd` to the umbrella root like `bin/cli`; a `case` subcommand dispatch with `start|st` and `ws|workstream <sub>`).
- Manages: `intent/whiteboard/<node>/{wip.md,inbox.*.md,.history/}` per Protocol 3.0.
- Promotion: relocate into the Intent `intent claude` namespace; share the format SSOT with the `/in-whiteboard` skill.

### compose_ctx(ws)

Emits to stdout: the identity line (`YOU ARE: <ws> (<Name>) ...`), then `.claude/restart.md`, then the standing daily-plan-then-wait instruction. Consumed by `--append-system-prompt "$(compose_ctx <ws>)"`. The board is intentionally excluded (pickup reads it live).

## Alternatives considered

- **pbcopy / clipboard context** -- rejected for the script (manual paste step; clobbers the user clipboard; the pasted prose hits the in-session gate). Acceptable only for a throwaway interim manual command.
- **`--append-system-prompt-file <path>`** -- does not exist in 2.1.191 (a sub-agent hallucinated it). Inline via `--append-system-prompt "$(cat ...)"` instead.
- **headless `-p`** -- rejected; MAAC needs an interactive session that waits for the human after the seeded turn.
- **per-node git worktrees** for isolation -- deferred; they fragment the shared dev DB and the shared `intent/whiteboard`.
- **A `Session` resource / bespoke launch state** -- unnecessary; the whiteboard already is the durable state, the launcher is stateless.
