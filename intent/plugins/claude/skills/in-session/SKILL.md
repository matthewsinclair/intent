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

### 2. Detect the project's primary language

Probe the project root in this order. First hit wins for primary language; multiple can apply for polyglot repos.

| Probe                                      | Language | Skills to invoke                                                                |
| ------------------------------------------ | -------- | ------------------------------------------------------------------------------- |
| `mix.exs` exists                           | Elixir   | `/in-elixir-essentials`, `/in-elixir-testing`; plus Ash and LiveView per step 3 |
| `Cargo.toml` exists                        | Rust     | `/in-rust-essentials` (ships in WP06)                                           |
| `Package.swift` exists                     | Swift    | `/in-swift-essentials` (ships in WP06)                                          |
| `.luarc.json` or `.lua`-dominated tree     | Lua      | `/in-lua-essentials` (ships in WP06)                                            |
| `bin/` or `scripts/` has bash/zsh shebangs | Shell    | `/in-shell-essentials` (ships in WP12)                                          |

### 3. Elixir dep-based fan-out

When `mix.exs` is present, read it and invoke by dependency:

- Depends on `:ash` or `:ash_postgres` -> invoke `/in-ash-ecto-essentials`
- Depends on `:phoenix_live_view` -> invoke `/in-phoenix-liveview`

### 4. Release the UserPromptSubmit gate

Intent projects ship a strict `UserPromptSubmit` hook (`require-in-session.sh`) that blocks the first prompt until `/in-session` has been run. Releasing the gate is cooperative: this skill writes a per-session sentinel that the hook looks for.

Run the following Bash command now (it is idempotent and fast):

```bash
project_dir="${CLAUDE_PROJECT_DIR:-$PWD}"
project_key="$(printf '%s' "$project_dir" | cksum 2>/dev/null | awk '{print $1}')"
mkdir -p /tmp/intent
sid="$(cat "/tmp/intent-claude-session-current-id-${project_key}" 2>/dev/null || true)"
sid_legacy="$(cat /tmp/intent-claude-session-current-id 2>/dev/null || true)"
[ -n "$sid" ] && touch "/tmp/intent/in-session-${sid}.sentinel"
[ -n "$sid_legacy" ] && [ "$sid_legacy" != "$sid" ] && touch "/tmp/intent/in-session-${sid_legacy}.sentinel"
touch /tmp/intent/in-session-unknown.sentinel
echo "released sentinels: per-project=${sid:-(none)} legacy=${sid_legacy:-(none)} +unknown"
```

How it resolves the session_id (in priority order):

1. **Per-project state file** `/tmp/intent-claude-session-current-id-${project_key}` -- written by the v2.10.0+ `SessionStart` hook (`session-context.sh`). Per-project scoping prevents cross-session contamination when multiple Claude Code sessions run concurrently in different Intent projects.
2. **Legacy shared state file** `/tmp/intent-claude-session-current-id` -- written by the pre-v2.10.0 `SessionStart` hook. Used as a best-effort fallback so sessions that started before the per-project hook landed can self-rescue without a Claude Code restart.
3. **`unknown` fallback** -- always touched. Covers the case where the gate's own `session_id` extraction also fell back to `unknown` (no jq, empty payload, etc.).

If after running `/in-session` the gate is **still** firing on the next prompt -- visible as `Expected sentinel: /tmp/intent/in-session-<UUID>.sentinel` in the hook output -- it means the running Claude Code session predates the per-project hook and the legacy file got stomped by another project's SessionStart. Two recovery paths:

- **Quick:** copy the UUID from the gate's error message and run `touch /tmp/intent/in-session-<UUID>.sentinel`.
- **Proper:** restart the Claude Code session (so `SessionStart` fires with the new hook and writes the per-project state file).

### 5. Confirm and proceed

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
