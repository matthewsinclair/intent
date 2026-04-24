# Working with LLMs in Intent

This document is Intent's canonical explanation of its LLM-facing configuration surface. It is the "why" behind the files, hooks, skills, subagents, rules, and critics that ship with every Intent project.

If `usage-rules.md` is the DO / NEVER contract and `AGENTS.md` is the auto-generated project index, this doc is the living reference that explains the system — how the pieces fit together, what decisions shaped them, and how to configure or extend them.

The doc is deliberately opinionated: the canon is already decided, and the decisions are recorded below as `D1`–`D10` for cross-reference against `intent/st/ST0035/design.md`.

## Table of contents

- Overview
- The three-file architecture
- D1–D10 — the canon decisions
- Session hook architecture
- Critic cadence
- Skills and /in-session auto-load
- Extensions at ~/.intent/ext/
- Socrates vs Diogenes FAQ
- For Elixir projects: mix usage_rules.sync interop
- Troubleshooting
- See also

## Overview

Every Intent project has four layers of LLM-facing configuration:

1. **Three root-level canon files** — tool-agnostic, Claude-specific, prescriptive-contract.
2. **Internal enforcement files** under `intent/llm/` — module registry and code-placement flowchart.
3. **Skill and subagent packs** under `~/.claude/skills/` and `~/.claude/agents/`.
4. **Session hooks and pre-commit gate** wired by `.claude/settings.json` and `.git/hooks/pre-commit`.

The first two are _content_: what LLMs read when they open the project. The third is _capability_: what commands, skills, and subagents are available. The fourth is _enforcement_: when those capabilities actually fire during a session or commit.

Each layer has one job. Deciding where a new rule or piece of information belongs is almost always a question of asking which of these four layers it fits — and the rest of this doc makes those boundaries explicit.

## The three-file architecture

Root-level LLM config on every Intent project:

```
my-project/
├── AGENTS.md          ← Tool-agnostic. Auto-generated. Read by every agentic tool.
├── CLAUDE.md          ← Claude-specific overlay. Imports AGENTS.md. Hand-edited.
└── usage-rules.md     ← Prescriptive DO / NEVER. Elixir-ecosystem-aligned.
```

Layering:

```
            ┌────────────────────────────────────────┐
            │   agentic tool opens the project       │
            └────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        ▼                       ▼                       ▼
  ┌──────────┐           ┌──────────┐           ┌──────────────┐
  │AGENTS.md │           │CLAUDE.md │           │usage-rules.md│
  │(navigate)│◄──imports─┤(overlay) │           │  (contract)  │
  └──────────┘           └──────────┘           └──────────────┘
        │                       │                       │
        └──────────── also reads ┼───────────────────────┘
                                ▼
                ┌───────────────────────────────┐
                │  intent/llm/                  │  ← internal
                │    MODULES.md                 │    enforcement
                │    DECISION_TREE.md           │
                └───────────────────────────────┘
                                │
                ┌───────────────────────────────┐
                │  intent/docs/                 │  ← narrative
                │    working-with-llms.md       │    (this file)
                │    rules.md                   │
                │    critics.md                 │
                │    writing-extensions.md      │
                └───────────────────────────────┘
```

Each file answers a different question:

| File                               | Answers                                              | Audience             |
| ---------------------------------- | ---------------------------------------------------- | -------------------- |
| `AGENTS.md`                        | How do I navigate this project?                      | Any LLM tool         |
| `CLAUDE.md`                        | What Claude-specific setup applies here?             | Claude Code sessions |
| `usage-rules.md`                   | What must I do / never do when working here?         | LLMs and humans      |
| `intent/llm/MODULES.md`            | Where does concern X live? Is there already a home?  | Intent developers    |
| `intent/llm/DECISION_TREE.md`      | Where should this new piece of code go?              | Intent developers    |
| `intent/docs/working-with-llms.md` | Why is it set up this way? How does it fit together? | Anyone (once)        |

If you're unsure where a piece of information belongs, follow the decision flow:

- "How to do thing X as a user of this project"? → `AGENTS.md` (or surface via `intent agents sync`).
- "What Claude must do that's different from other tools"? → `CLAUDE.md`.
- A terse must / never rule? → `usage-rules.md`.
- Where a given concern (auth, email, caching) _lives_ in the codebase? → `MODULES.md`.
- The decision flow for where new code should go? → `DECISION_TREE.md`.
- Narrative, rationale, or FAQ content? → `intent/docs/<topic>.md` (here or a topic-specific doc).

The ten decisions recorded in `intent/st/ST0035/design.md` define Intent's current LLM canon. D1 through D10 below restate them in living-doc form — what the state is now, with enough context to understand it.

## D1. AGENTS.md is the primary LLM config file

Every Intent project ships a root-level `AGENTS.md`. Auto-generated by `intent agents sync`, written as a real file (not a symlink), and the first file every LLM-facing tool reads.

Why: AGENTS.md became the de facto standard in 2025–2026. Adopted by Anthropic, OpenAI, Google, Cursor, GitHub Copilot, Codex, Aider, Continue, Cline, Gemini CLI, and VS Code. 60k+ repositories ship one. The Linux Foundation Agentic AI Foundation governs the spec. Aligning with the community is free; deviating costs compatibility with tooling downstream users already rely on.

Why a real file, not a symlink: symlinks work for tools that follow them, but some CI pipelines, web-rendered docs, and `find | cat` patterns treat them opaquely. A real file removes a class of bug and matches community norm.

Regenerate with `intent agents sync`; never edit manually. Manual edits are overwritten on the next sync.

## D2. CLAUDE.md is a Claude-specific overlay

`CLAUDE.md` at the project root imports `AGENTS.md` by reference and adds Claude-specific directives — memory directory pointers, file mappings, `/in-session` auto-load wiring, pointer to `.claude/settings.json` hooks. It never duplicates `AGENTS.md` content.

Why: Anthropic positions `CLAUDE.md` as complementary, not competing. If both files exist, `CLAUDE.md` wins for Claude sessions only. Treating the two as parallel sources of truth guarantees divergence over time; treating `CLAUDE.md` as an overlay removes that risk.

Rule of thumb: if a directive applies to any LLM tool, it goes in `AGENTS.md`. If it's specific to Claude Code (slash commands, hook stanzas, memory files), it goes in `CLAUDE.md`.

## D3. usage-rules.md stays and is honoured

Root `usage-rules.md` is kept and refreshed. An Intent-authored template lands at `lib/templates/llm/_usage-rules.md` and is planted in downstream projects by `intent claude upgrade --apply`.

Why: the Elixir ecosystem has a mature per-package `usage-rules.md` convention. Each package ships a `usage-rules.md` at the dep root; `mix usage_rules.sync` discovers them and gathers them into the project-level `AGENTS.md`. Intent runs Elixir fleet projects (Lamplight, Laksa, Anvil, the Arca trio, Conflab, MeetZaya, MicroGPTEx, Multiplyer, Prolix) that already benefit from this pattern. Removing `usage-rules.md` would break alignment with Elixir's most mature LLM-guidance convention for no gain. No equivalent convention exists yet for Python, Node, Ruby, Rust — but Intent's approach is forward-compatible.

Split with `AGENTS.md`: `usage-rules.md` = terse DO / NEVER contract. `AGENTS.md` = broad navigation and state. They co-exist cleanly.

## D4. intent/llm/ keeps MODULES.md and DECISION_TREE.md only

`intent/llm/` is the internal-enforcement layer: `MODULES.md` (Highlander module registry) and `DECISION_TREE.md` (code-placement flowchart). No `AGENTS.md` here anymore — the real `AGENTS.md` lives at root (D1). The vestigial `_llm_preamble.md` template is gone from `lib/templates/llm/`.

Why: `intent/llm/` serves Intent developers (internal). Root files serve downstream users and LLM tooling (external). Keeping them in separate directories clarifies responsibility.

## D5. working-with-llms.md is the narrative tech note

This file. Owns the long-form narrative: the three-file canon, hook mechanics, critic cadence, Socrates vs Diogenes FAQ, and how everything fits together.

Why: `usage-rules.md` is too terse for "why" content. `AGENTS.md` is auto-generated and can't carry opinion. `CLAUDE.md` is Claude-specific. A separate tech note is the natural home for system-level explanation. `intent/docs/` already holds authoring guides (`rules.md`, `critics.md`, `writing-extensions.md`); this doc fits the same slot.

## D6. Socrates and Diogenes are two agents, two domains

Intent ships both the `socrates` subagent and the `diogenes` subagent. They are disjoint — different domains, different personas, different artefacts. Neither is being renamed, consolidated, or split further.

See the dedicated FAQ section below for the forensic history and the clearest short answer to the common "weren't they the same agent?" question.

## D7. Session hooks inject reminders

`.claude/settings.json` ships three hooks for every Intent project:

- `SessionStart` (matchers: `startup`, `resume`, `clear`, `compact`) — injects a brief project context reminder so Claude recognises that an Intent project is active.
- `UserPromptSubmit` (strict gate) — blocks the first user prompt until `/in-session` has run in the conversation. This enforces loading coding-standards skills before any code discussion starts.
- `Stop` (after every assistant turn when no prompt is queued) — injects a `/in-finish` reminder at session end.

Why strict on `UserPromptSubmit`: soft reminders had low observed compliance in multi-turn sessions. A hard gate catches the case where the user forgets to load the session. The choice is explicit in ST0035's resolved decisions — the user will reassess intrusiveness post-rollout; if the gate is too noisy in practice, the template carries both strict and soft variants, and flipping is a one-line config change.

Why soft on `SessionStart` and `Stop`: these fire automatically, not in response to a user action. Blocking at these points would be surprising. A reminder injection has proven sufficient.

Why no `PostToolUse` hook by default: it would fire on every `Write|Edit` during multi-step work — too noisy, too expensive in tokens. A helper script ships for opt-in via `.intent_critic.yml post_tool_use_advisory: true`, but the default `.claude/settings.json` stanza omits the hook entirely.

## D8. Critics run via git pre-commit hook

The primary critic cadence is a `.git/hooks/pre-commit` that runs `bin/intent_critic <lang> <staged-files>` and blocks the commit when findings meet or exceed the configured severity threshold. Per-project override via `.intent_critic.yml`.

Why pre-commit: local, deterministic, offline, zero-latency feedback. Every developer sees violations on their own machine before pushing.

Why a headless bash runner (`bin/intent_critic`) rather than invoking a Claude subagent for the gate: pre-commit runs dozens of times a day. The rules' Detection heuristics are mechanical (YAML frontmatter + Markdown patterns), parseable in bash, deterministic. LLM round-trip for mechanical checks is slow and wasteful. The LLM-based `critic-<lang>` subagents remain available for richer reviews via `/in-review` stage 2.

Secondary cadences:

- GitHub Actions on push / PR runs the same `intent_critic` binary — catches `--no-verify` bypasses and agents that push without committing locally.
- The `Stop` hook's end-of-turn reminder can suggest `/in-review` for deeper analysis when appropriate.

## D9. Fail-forward — no backwards-compat shims

Deprecated artefacts are deleted now. Migrations actively prune. Examples from v2.9.1:

- `intent/llm/AGENTS.md` → deleted. Root `AGENTS.md` is the only location.
- `lib/templates/llm/_llm_preamble.md` → deleted.
- ST0010 (MCP exploration) and ST0015 (Enhanced ST templates) → cancelled, moved to `intent/st/CANCELLED/`, annotated with deprecation one-liners.

Why: Intent's stated posture is fail-forward. Backwards-compat shims accumulate, decay, and hide bugs. Users who need legacy behaviour pin to an older Intent version.

## D10. Phase 0 review gate before implementation

ST0035 (and any non-trivial steel thread) uses a "document first, code next" discipline. Phase 0 populates all ST and WP docs without changing production files. Phase 0 output is committed and reviewed. Implementation begins only after approval.

Why: this locks scope and acceptance criteria before any implementation decision can drift. It gives the reviewer a single review surface (`info.md` + `design.md` + per-WP `info.md` files) rather than a stream of commits that keep moving the target.

This is a _process_ decision, not a _content_ decision — it applies to how Intent develops Intent, and is recommended (not enforced) for downstream projects.

## Session hook architecture

`.claude/settings.json` at the project root wires the session hooks. Template source: `lib/templates/.claude/settings.json` (ships in ST0035/WP-04).

Ship shape (strict-gate default):

```json
{
  "hooks": {
    "SessionStart": [
      {
        "matchers": ["startup", "resume", "clear", "compact"],
        "hooks": [
          {
            "type": "command",
            "command": "$INTENT_HOME/lib/hooks/session_start.sh"
          }
        ]
      }
    ],
    "UserPromptSubmit": [
      {
        "matchers": ["*"],
        "hooks": [
          {
            "type": "command",
            "command": "$INTENT_HOME/lib/hooks/user_prompt_submit_strict.sh"
          }
        ]
      }
    ],
    "Stop": [
      {
        "matchers": ["*"],
        "hooks": [
          {
            "type": "command",
            "command": "$INTENT_HOME/lib/hooks/stop_reminder.sh"
          }
        ]
      }
    ]
  }
}
```

How it works:

- Claude Code reads `.claude/settings.json` at session start.
- When a matching lifecycle event fires, the shell command runs. Stdout on exit 0 is injected into the conversation as a `<system-reminder>` — the only mechanism by which a hook can influence the conversation.
- Claude reads the reminder and acts on it.

The strict `UserPromptSubmit` hook uses a sentinel file to track whether `/in-session` has run in the current conversation. First-prompt flow:

1. User types a message. `UserPromptSubmit` hook fires.
2. Script checks for `/tmp/intent-session-<SESSION_ID>.sentinel`. Absent → inject a blocking system-reminder: _"`/in-session` must run before any coding work. The gate releases once the session bootstrap completes."_
3. User (or Claude, reading the reminder) invokes `/in-session`. The skill's script creates the sentinel.
4. Next prompt: sentinel present → hook exits silently. User's prompt reaches Claude normally.

Why this design: Claude Code's hook API does not let a hook directly invoke a slash command. Injected system-reminders are the only reliable influence mechanism. Sentinel-file tracking lets the hook fire every turn cheaply (one stat call) while only blocking until the session is bootstrapped.

If the strict gate proves too intrusive, flip to soft-reminder mode by replacing the `UserPromptSubmit` command with `$INTENT_HOME/lib/hooks/user_prompt_submit_soft.sh`. The template ships both modes; the switch is one line of JSON.

## Critic cadence

Critics enforce the rule library at two distinct points:

1. **Pre-commit (mechanical, headless, bash).** `.git/hooks/pre-commit` runs `bin/intent_critic <lang> <staged-files>`. Severity threshold from `.intent_critic.yml`. Blocks the commit when any finding meets or exceeds the threshold.
2. **On-demand (LLM-based, richer).** `Task(subagent_type="critic-<lang>", prompt="review <targets>")` invokes the corresponding LLM subagent. Same rules, deeper judgement, slower.

The split exists because these are different use cases:

| Use case                      | Tool                     | Speed   | Depth                                   |
| ----------------------------- | ------------------------ | ------- | --------------------------------------- |
| Block broken code at commit   | `bin/intent_critic`      | < 1 sec | Mechanical: Detection heuristic matches |
| Review code quality pre-merge | `critic-<lang>` subagent | ~30 sec | Judgement: reads context, weighs rules  |

Both read the same rule files under `intent/plugins/claude/rules/` and produce the same report format. The bash runner is a faithful reimplementation of the Detection-heuristic engine — parseable YAML frontmatter plus pattern matching on source code. No LLM round-trip required for mechanical checks.

### `.intent_critic.yml` per-project config

Lives at the project root:

```yaml
severity_min: warning
disabled:
  - IN-EX-CODE-007 # reason: moduledoc noise not valued here
  - IN-RS-CODE-005 # reason: explicit lifetimes preferred in our domain code
# show_all: true    # uncomment to render recommendation + style in the body
```

| Key            | Value                                                  | Default   |
| -------------- | ------------------------------------------------------ | --------- |
| `severity_min` | `critical` \| `warning` \| `recommendation` \| `style` | `warning` |
| `disabled`     | List of rule IDs to suppress                           | `[]`      |
| `show_all`     | Shorthand for `severity_min: style`                    | `false`   |

Default `severity_min` is `warning`: both CRITICAL and WARNING findings block a commit (the "warnings-are-errors" posture). Per-project tuning is expected; a per-rule `# reason:` comment on each `disabled` entry is required discipline so downstream reviewers know why the rule was silenced.

Full contract in `intent/docs/critics.md`.

## Skills and /in-session auto-load

Skills install to `~/.claude/skills/<name>/SKILL.md` and auto-load into every Claude Code session.

The session-bootstrap flow handled by the `/in-session` skill:

1. Unconditionally invoke `/in-essentials` and `/in-standards` (universal workflow rules).
2. Detect the project's primary language by probing the project root:
   - `mix.exs` → Elixir → `/in-elixir-essentials`, `/in-elixir-testing`.
   - `Cargo.toml` → Rust → `/in-rust-essentials`.
   - `Package.swift` → Swift → `/in-swift-essentials`.
   - `.luarc.json` or `.lua`-dominant tree → Lua → `/in-lua-essentials`.
   - `bin/` or `scripts/` with bash/zsh shebangs → Shell → `/in-shell-essentials`.
3. Elixir-specific dep fan-out:
   - `:ash` or `:ash_postgres` in `mix.exs` → `/in-ash-ecto-essentials`.
   - `:phoenix_live_view` in `mix.exs` → `/in-phoenix-liveview`.
4. Report the loaded skill set in one line so the user can spot an unexpected match.

Why auto-load: after `/compact`, conversation context is regenerated but skill invocations are not replayed. Without `/in-session`, the user has to paste the skill list manually every reset. The `/in-session` skill is a thin coordinator that parses the project and calls the right skills in one command.

To extend:

- **New language pack** — add a probe row to `in-session/SKILL.md`, plus the corresponding language skill under `intent/plugins/claude/skills/`. For critic coverage in the new language, add a dispatch entry in `intent/docs/critics.md` and ship rules under `intent/plugins/claude/rules/<lang>/`.
- **New framework fan-out** — add a dep-based branch in `/in-session` step 3 (Elixir-specific today, extensible to other ecosystems).

## Extensions at ~/.intent/ext/

Extensions let you add subagents, skills, or rule packs without forking Intent. Each extension is a self-contained directory at `~/.intent/ext/<name>/` with an `extension.json` manifest declaring contributions. Discovery is layered: canon is the default; user extensions override by name with a visible shadow warning.

Brief commands:

```bash
intent ext list                       # Enumerate installed extensions
intent ext show <name>                # Manifest + contributions
intent ext validate [<name>]          # Schema + traversal + contribution-existence checks
intent ext new <name> --subagent | --skill | --rule-pack
```

Precedence order:

1. `$INTENT_EXT_DIR` (env override, used by tests).
2. `~/.intent/ext/<name>/` (user extensions).
3. `$INTENT_HOME/intent/plugins/claude/{subagents,skills,rules}/` (canon).

When an extension ships a subagent, skill, or rule with the same name / rule-id as a canon entry, the extension wins — but a shadow warning is emitted on every relevant `intent` command. `INTENT_EXT_DISABLE=1` suppresses extensions entirely for a single invocation (useful for debugging).

Reference extension: `worker-bee` at `~/.intent/ext/worker-bee/`. It was seeded automatically by the v2.8.2 → v2.9.0 migration. Install via `intent claude subagents install worker-bee` to actually use it.

Full authoring guide: `intent/docs/writing-extensions.md`.

## Socrates vs Diogenes FAQ

**Question.** Weren't Socrates and Diogenes the same agent once? Why are there two?

**Answer.** They were never the same agent. Git log settles it:

- **Socrates debuted 2025-08-05 (commit `7f4529e`)** as CTO Review Mode. Personas: Socrates (CTO) + Plato (Tech Lead). Domain: architectural and strategic decisions. It never had a testing role.
- **Diogenes debuted 2026-02-20 (commit `37a0ed0`)** as a separate new agent. Personas: Aristotle (Empiricist) + Diogenes (Skeptic). Domain: test-specification dialog and test-quality validation. It was not carved out of Socrates — it is a new concern that needed new personas.

Both use Socratic-dialog methodology. Both have Greek-philosopher names. That's the source of the confusion, and it's cosmetic. The agents solve disjoint problems:

- Reach for `socrates` when you need to talk through an architectural or strategic decision.
- Reach for `diogenes` when you need to generate or validate test specifications (currently Elixir-specialised; generalising to other test stacks is a future ST).

They're not variants of the same agent. Renaming one (e.g., `socrates-test`) would falsely imply they share a domain, and they don't. Consolidating into "Socrates with modes" would force unrelated personas — Socrates + Plato vs Aristotle + Diogenes — into one agent and muddy both. The clean separation stays; documentation is the fix.

## For Elixir projects: mix usage_rules.sync interop

Elixir projects using Intent inherit a useful interop with the Ash Framework's `usage_rules` Hex package (`mix usage_rules.sync`).

How `mix usage_rules.sync` works: it scans the project's dependencies for `deps/<dep>/usage-rules.md` files and gathers them into a project-level `AGENTS.md`. The project's own root `usage-rules.md` is included in the gathered set.

How Intent plays with it: Intent's templated root `usage-rules.md` is hand-authored Markdown, not generated. `mix usage_rules.sync` reads it as-is. No Intent-specific tooling is required — if you're already running `mix usage_rules.sync` for your deps, adding Intent's `usage-rules.md` to the mix works without configuration.

Two practical consequences:

1. **AGENTS.md generation source of truth.** Intent's `intent agents sync` and Ash's `mix usage_rules.sync` can coexist, but pick one as the source of truth for the final `AGENTS.md` and disable the other's generation step. Intent's is generally preferred for Intent-using projects; deps' `usage-rules.md` content can be surfaced via Intent's own discovery if needed.
2. **Deps' rules are additive, not competitive.** Your project's root `usage-rules.md` describes project rules. `deps/ash/usage-rules.md`, `deps/phoenix_live_view/usage-rules.md`, and so on describe package rules. They aren't in conflict — they describe different domains. Both are useful.

For non-Elixir projects, `usage-rules.md` is still useful as a prescriptive DO / NEVER file, but the `mix usage_rules.sync` interop does not apply.

## Troubleshooting

### SessionStart hook does not fire

Symptom: opening a fresh Claude Code session in the project does not inject the project reminder.

Checks:

- Is `.claude/settings.json` present at the project root?
- Does it parse as valid JSON? (`jq . .claude/settings.json` must succeed.)
- Are the `matchers` correct for the event type? `SessionStart` accepts `startup`, `resume`, `clear`, `compact`.
- Is the hook script executable? `test -x "$INTENT_HOME/lib/hooks/session_start.sh"`.

If the script runs but nothing is injected, remember the contract: stdout on exit 0 is the injection payload. Confirm the script writes to stdout (not stderr) and exits 0.

### Strict UserPromptSubmit gate blocks every prompt, not just the first

Symptom: every user turn shows the "`/in-session` must run" reminder even after `/in-session` has been invoked.

Check: the sentinel file. The gate uses `/tmp/intent-session-<SESSION_ID>.sentinel` to track that `/in-session` has run. If `/in-session`'s script is not writing the sentinel, the gate keeps firing.

Fixes:

- Verify `/in-session` installation: `intent claude skills show in-session`.
- Verify the sentinel directory is writable and that `$SESSION_ID` resolves correctly inside the hook.
- If the strict gate is more friction than it's worth, flip it to soft-reminder mode by editing `.claude/settings.json`'s `UserPromptSubmit` command path from `user_prompt_submit_strict.sh` to `user_prompt_submit_soft.sh`.

### Pre-commit hook blocks on a rule you don't care about

Symptom: `git commit` fails with a critic finding you consider a false positive or a rule that isn't valuable in this codebase.

Fix order (by preference):

1. **Fix the code.** Most critic findings are real. Read the rule file (`intent claude rules show <id>`) and consider the fix.
2. **Tune severity.** If the rule is valid but too loud in this codebase, demote via `.intent_critic.yml` `severity_min: critical` (only block on CRITICAL, not WARNING).
3. **Disable the specific rule.** In `.intent_critic.yml`'s `disabled` list, with a mandatory `# reason:` comment explaining why. Unannotated entries fail review.
4. **Bypass this commit.** `git commit --no-verify`. Use sparingly — shared-branch policies should flag `--no-verify` commits for explicit review.

### `intent claude upgrade --apply` refuses to overwrite a hand-edited file

Symptom: running `intent claude upgrade --apply` reports that a file (e.g. `CLAUDE.md`) will not be overwritten because it has been hand-edited.

This is by design. Intent does not clobber human-curated content silently. Two paths:

- If you want to preserve your edits: accept the report and move on. The upgrade completes for other files.
- If you want Intent's current template: move your edits aside (`mv CLAUDE.md CLAUDE.md.bak`), re-run `intent claude upgrade --apply`, then merge any edits from the backup back in.

### `intent agents sync` writes to `intent/llm/AGENTS.md` instead of root

Symptom: after upgrade, `intent agents sync` still writes `AGENTS.md` in the old internal location.

Cause: the project may still be stamped at an earlier Intent version, or a stale config is being read. Check `.intent/config.json`'s `intent_version` field; run `intent upgrade` if it reads below `2.10.0`. The v2.10.0 migration flips the default output path and deletes the legacy `intent/llm/AGENTS.md`.

### New subagent installed mid-session is invisible to `Task()`

Symptom: after `intent claude subagents install <new-agent>`, calling `Task(subagent_type="<new-agent>")` returns "subagent not found".

Cause: Claude Code reads the subagent registry once at session start. Mid-session installs are not picked up until the next session starts.

Fix: close the current Claude Code session and start a new one. This applies equally to canon subagents, extension subagents, and any subagent installed by hand — it is a Claude Code constraint, not an Intent behaviour.

### `bin/intent_critic` produces different findings than the `critic-<lang>` subagent

Symptom: pre-commit passes and the subagent fails (or vice versa) on the same file + same rule set.

First: confirm both are running the same rules. Both read `intent/plugins/claude/rules/<lang>/**/RULE.md`. Extensions at `~/.intent/ext/*/rules/` also participate for both — if `INTENT_EXT_DISABLE=1` is set in one context but not the other, rule sets diverge.

Second: bash-parser parity is a known risk area. `bin/intent_critic` is intended to produce identical findings to the subagent; divergence is a bug. Report with a minimal reproducing file plus the rule ID and the expected versus observed output.

### `intent claude skills sync` reports mismatched checksums on a skill you edited

Symptom: a skill you edited locally now shows a checksum mismatch on `intent claude skills status`, and `sync` wants to overwrite your edits.

This mirrors the upgrade-doesn't-clobber contract. If the edit is intentional, either move it into an extension (`~/.intent/ext/<name>/skills/<slug>/`) so it shadows canon cleanly, or accept that `sync` will revert it on next run. Mid-fleet drift on canon skills creates more problems than it solves.

## See also

- `AGENTS.md` at the project root — navigation, build/test commands, installed skills and subagents.
- `usage-rules.md` at the project root — DO / NEVER contract.
- `intent/docs/rules.md` — rule authoring guide.
- `intent/docs/critics.md` — Critic subagent contract and report format.
- `intent/docs/writing-extensions.md` — authoring guide for extensions at `~/.intent/ext/`.
- `intent/llm/MODULES.md` — Highlander module registry.
- `intent/llm/DECISION_TREE.md` — code-placement flowchart.
- `intent/st/ST0035/design.md` — the decision log that drove this canon.

---

_Document stamp: Intent v2.9.1 canon, authored for ST0035/WP-03, 2026-04-24. Expected to be kept current across v2.9.x; significant canon changes should update both this doc and `intent/st/<ST>/design.md`._
