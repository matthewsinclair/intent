---
description: "Session bootstrap: auto-load coding skills for the detected project language after context reset or compact"
chains_to:
  [
    "in-essentials",
    "in-standards",
    "in-elixir-essentials",
    "in-elixir-testing",
    "in-ash-ecto-essentials",
    "in-phoenix-liveview",
    "in-author-essentials",
    "in-content-essentials",
    "in-whiteboard",
  ]
---

# Session Bootstrap

Thin coordinator that loads the right coding skills for the current project. Invoke once after `/compact`, context reset, or at the start of a coding session. One command replaces the manual skill-reload list.

## When to invoke

- Immediately after `/compact` or any context reset.
- At the start of a coding session before touching code.
- Whenever the user says "reload the skills" or equivalent.

## Procedure

### 1. Load universal skills

Invoke these skills via the Skill tool, unconditionally:

- `/in-essentials` -- Intent workflow rules (use the CLI, never edit generated files)
- `/in-standards` -- agnostic coding discipline (Highlander, PFIC, Thin Coordinator, No Silent Errors)

### 2. Read the project's declared languages

Languages-in-use is a configuration decision, not a filesystem detection. Read the `languages` array from `intent/.config/config.json`:

```bash
jq -r '(.languages // []) | .[]' intent/.config/config.json
```

For each language listed, invoke the matching essentials skill if one exists. Elixir and `author` have a per-language essentials skill; the other languages get their coding rules via the rule library served by the installed Intent tool (`intent claude rules list --lang <lang>`, `intent claude rules show <id>`) plus the `critic-<lang>` subagent applied on demand.

| Language  | Skills to invoke (if listed in config)                                           |
| --------- | -------------------------------------------------------------------------------- |
| `elixir`  | `/in-elixir-essentials`, `/in-elixir-testing`; plus Ash and LiveView per step 3  |
| `rust`    | (no essentials skill; `intent claude rules list --lang rust`, critic-rust)       |
| `swift`   | (no essentials skill; `intent claude rules list --lang swift`, critic-swift)     |
| `lua`     | (no essentials skill; `intent claude rules list --lang lua`, critic-lua)         |
| `shell`   | (no essentials skill; `intent claude rules list --lang shell`, critic-shell)     |
| `author`  | `/in-author-essentials`; prose + courseware, `critic-prose` on demand (not code) |
| `content` | `/in-content-essentials`; web content, `critic-prose` on demand (not code)       |

If the array is empty or missing, no language-specific essentials skills load. The user can declare languages with `intent lang init <lang>` (or remove with `intent lang remove <lang>`).

### 3. Elixir dep-based fan-out

When `mix.exs` is present, read it and invoke by dependency:

- Depends on `:ash` or `:ash_postgres` -> invoke `/in-ash-ecto-essentials`
- Depends on `:phoenix_live_view` -> invoke `/in-phoenix-liveview`

### 4. Release the UserPromptSubmit gate

Intent projects ship a strict `UserPromptSubmit` hook (`require-in-session.sh`) that blocks the first prompt until `/in-session` has been run. Releasing the gate is cooperative: this skill writes a per-session sentinel that the hook looks for.

Run the helper script (idempotent, fast, ~30 lines):

```bash
bash "$HOME/.claude/skills/in-session/scripts/release-gate.sh"
```

It is extracted from this prose so any pipeline survives skill-renderer token-stripping. The earlier inline form leaked positional-field syntax through the renderer, which silently emptied it, producing a no-op gate release. The canonical source is `intent/plugins/claude/skills/in-session/scripts/release-gate.sh`.

How it resolves the session_id: a single authoritative source, `$CLAUDE_CODE_SESSION_ID`, the env var Claude Code exports into every Bash tool invocation. The gate (`require-in-session.sh`) resolves the same env var, so the release path and the check path agree by construction -- no shared state file in between. When the env var is absent (an older Claude Code build), both sides fall back to the same `unknown` sentinel, which the script always touches, so they still agree and the gate self-heals.

This replaced an earlier design where the releaser read a shared per-project state file written by `SessionStart`. Concurrent Claude Code sessions in one project all wrote that one file, the releaser touched the wrong session's sentinel, and the gate deadlocked. The shared file is gone.

If after running `/in-session` the gate is **still** firing on the next prompt -- visible as `Expected sentinel: /tmp/intent/in-session-<UUID>.sentinel` in the hook output -- copy the UUID from the gate's error message and run `touch /tmp/intent/in-session-<UUID>.sentinel`.

### 5. Pickup the whiteboard

If `intent/whiteboard/` exists in the project root, invoke `/in-whiteboard pickup`. This reads your node's board + inboxes, surfaces peer-node state + any inbound asks, and touches this session's heartbeat. If the directory doesn't exist, skip silently -- the whiteboard is opt-in per project.

The whiteboard is the live cross-session coordination channel; `wip.md` is the post-session snapshot. See `/in-whiteboard` for protocol details.

### 6. Confirm and proceed

Report the skill set loaded in one line so the user can spot an unexpected match. Do not wait for further instructions; proceed with whatever the user was already asking for.

## Persistent reminders

Hold these regardless of which language skills get loaded above. They do not expire at the end of `/in-session`; they apply for the rest of the session.

- Be diligent for any Highlander, Thin Coordinator, and PFIC violations (adjust for language-specifics, ie Elixir, Rust, Lua, and Swift, as appropriate).
- NEVER MANUALLY WRAP .MD FILES.

## Why this exists

After `/compact`, the conversation summary is regenerated but skill invocations are not replayed. Without a bootstrap, the user has to paste the skill list manually every reset. This skill is a Thin Coordinator: parse the project, call the right skills, done. One command instead of six.

## Red Flags

| Rationalisation                                | Reality                                                                 |
| ---------------------------------------------- | ----------------------------------------------------------------------- |
| "The skills loaded once already this session." | After `/compact` they are not active. Re-invoke via `/in-session`.      |
| "I can read each rule file on demand."         | Skills carry procedural orchestration, not just rule prose. Load them.  |
| "Elixir-only -- no need for agnostic rules."   | Every language concretises the agnostic pack. Always load in-standards. |
