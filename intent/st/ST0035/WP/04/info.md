---
verblock: "24 Apr 2026:v0.2: matts - Phase 0 forensic detail"
wp_id: WP-04
title: "Ship .claude/settings.json template with session hooks"
scope: Medium
status: Not Started
---

# WP-04: Ship .claude/settings.json template with session hooks

## Objective

Author the canonical `.claude/settings.json` template at `lib/templates/.claude/settings.json` that every Intent project inherits. It wires two primary Claude Code hooks: `SessionStart` (matchers: `startup`, `resume`, `clear`, `compact`) that injects "Run /in-session" into the conversation, and `Stop` that injects "Run /in-finish" as a wrap-up reminder. Template is installed into each project's `.claude/settings.json` by `intent claude upgrade --apply` (WP11).

## Context

Fleet audit showed `.claude/` is universally empty across all Intent projects — the hook capability exists in Claude Code but no project uses it. WP04 makes hooks the default.

Claude Code hooks are configured in `.claude/settings.json` (project-level) and/or `.claude/settings.local.json` (user-local overrides). Hook commands emit text on stdout that Claude reads as a system-reminder. Hooks cannot directly invoke slash commands; the reminder is injected and the model is trained to act on critical-skill notices.

Open decisions resolved 2026-04-24:

- **#2 (enforcement strictness)** = **strict**. Hard `UserPromptSubmit` gate that blocks the first prompt until `/in-session` has been invoked in the conversation. SessionStart + Stop reminders stay as a belt-and-braces layer. User will reassess intrusiveness after rollout.
- **#4 (PostToolUse advisory critic)** = **off by default**. Too noisy (every intermediate edit during multi-step work would fire) and too costly in tokens (every tool use would inject advisory findings). Pre-commit gate catches everything at the canonical checkpoint. Helper script `post-tool-advisory.sh` still ships so users can opt in via `.intent_critic.yml post_tool_use_advisory: true` + adding the PostToolUse stanza to their `.claude/settings.local.json`, but the **default** `.claude/settings.json` template **omits the PostToolUse stanza**.

WP04 ships the template only — installation into projects is WP11. Coordinates with WP05 (intent_critic must support single-file low-latency invocation for the opt-in PostToolUse path, even though it's off by default).

## Deliverables

1. **Template file** at `lib/templates/.claude/settings.json`. Shape (final syntax verified against Claude Code hook spec during WP04 kickoff):

   ```json
   {
     "hooks": {
       "SessionStart": [
         {
           "matcher": "startup|resume|clear|compact",
           "hooks": [
             {
               "type": "command",
               "command": "/path/to/session-context.sh"
             }
           ]
         }
       ],
       "UserPromptSubmit": [
         {
           "matcher": "",
           "hooks": [
             {
               "type": "command",
               "command": "/path/to/require-in-session.sh"
             }
           ]
         }
       ],
       "Stop": [
         {
           "matcher": "",
           "hooks": [
             {
               "type": "command",
               "command": "echo 'Session wrap-up reminder: run /in-finish to update ST docs, intent/wip.md, and prepare a clean commit.'"
             }
           ]
         }
       ]
     }
   }
   ```

   Three default hook types: `SessionStart` (context inject), `UserPromptSubmit` (strict `/in-session` gate, per decision #2), `Stop` (wrap-up reminder). **`PostToolUse` is NOT in the default stanza (decision #4 = off).** Users who want advisory critic add the stanza themselves (example shown in `pre-commit-hook.md` / `working-with-llms.md`) and flip `.intent_critic.yml post_tool_use_advisory: true`.

2. **Helper scripts** at `lib/templates/.claude/scripts/`:
   - `session-context.sh` (SessionStart) — idempotent, < 200ms. Emits: project name, git branch, short SHA, active ST, WIP summary. Becomes the system-reminder content.
   - `require-in-session.sh` (UserPromptSubmit) — hard gate. Checks for a per-session marker (e.g., `~/.claude/projects/<dir>/.in-session-ran-<session-id>` sentinel file); if absent, emits a blocking response via exit code 2 with a stderr message directing the user to run `/in-session`. Once `/in-session` runs, it writes the sentinel; subsequent prompts pass through. The sentinel is per-session so each new session re-gates.
   - `post-tool-advisory.sh` (**opt-in** PostToolUse Write|Edit|MultiEdit) — reads tool-use JSON from stdin; extracts the target file path; if it's a source file in a supported language AND `.intent_critic.yml post_tool_use_advisory: true`, runs `intent critic <lang> --files <path> --format text` and emits findings as a system-reminder. Non-blocking (exit 0 always). **Ships with the template but is not wired into the default settings.json stanza**; users who want it add the stanza themselves.

3. **Template placeholders** if any project-specific content is needed (likely none — scripts discover project details dynamically).

4. **MODULES.md registration** for `lib/templates/.claude/` and all three helper scripts.

5. **User-level opt-out documentation** in `working-with-llms.md` (WP03 cross-references this) — how to disable the `UserPromptSubmit` gate per-session in `.claude/settings.local.json` (set hook to empty or use `.intent_critic.yml post_tool_use_advisory: false` for the PostToolUse path). Strict is canon; opt-out is per-user.

## Approach

1. **Read Claude Code hooks docs** to confirm exact JSON schema and matcher regex semantics. Anthropic docs: https://docs.claude.com/en/docs/claude-code/hooks (verify URL at WP04 start).
2. **Confirm hook command behaviour** per event type:
   - `SessionStart` — stdout injected as system-reminder; exit 0.
   - `UserPromptSubmit` — stdout + non-zero exit can block the user's prompt before it reaches the model; exit 2 is the documented "block" code. Confirm syntax for blocking message.
   - `PostToolUse` — tool-use JSON on stdin; stdout injected as system-reminder; exit 0 (non-blocking).
   - `Stop` — stdout injected; exit 0.
3. **Author the four hook stanzas** in the JSON template with absolute-path `command:` entries pointing at the helper scripts (installer in WP11 substitutes the real path).
4. **Author `session-context.sh`**: detect git repo, read `intent/wip.md`, echo project name + branch + short SHA + active ST + WIP summary. Graceful degradation when git or wip.md absent.
5. **Author `require-in-session.sh`** (strict gate):
   - Sentinel file path: `~/.claude/projects/<project-hash>/.intent-in-session-<session-id>`.
   - On invocation, check for sentinel. If present: exit 0 (pass through). If absent: emit message "Intent project requires `/in-session` to run before your first prompt. Run it now." to stderr, exit 2 (block).
   - A complementary hook on PostToolUse (or inside `/in-session` itself) writes the sentinel once `/in-session` has run.
   - Test the session-id propagation — confirm Claude Code passes session ID to hooks via env var or stdin JSON.
6. **Author `post-tool-advisory.sh`**:
   - Parse the tool-use JSON from stdin (fields: `tool_name`, `tool_input.file_path`).
   - If `tool_name` ∈ {Write, Edit, MultiEdit} and file extension maps to a supported critic language → continue; else exit 0.
   - Check `.intent_critic.yml`: if `post_tool_use_advisory: false`, exit 0.
   - Run `intent critic <lang> --files <file> --severity-min warning --format text`, capture stdout.
   - Echo findings as system-reminder with a clear "advisory — does not block" prefix.
   - Exit 0 always (advisory, not blocking).
7. **Sanity-test** the template + scripts in a scratch worktree.
8. **Commit** the template files, helper scripts, and MODULES.md update.

## Acceptance Criteria

- [ ] `lib/templates/.claude/settings.json` exists and is valid JSON (`jq . lib/templates/.claude/settings.json` returns 0).
- [ ] Template has three default hook stanzas: `SessionStart`, `UserPromptSubmit`, `Stop`. PostToolUse is **not** in the default.
- [ ] `SessionStart` matcher covers `startup|resume|clear|compact`.
- [ ] `UserPromptSubmit` invokes `require-in-session.sh`.
- [ ] `Stop` fires after every assistant turn.
- [ ] `post-tool-advisory.sh` ships with the template but is not referenced by the default settings.json stanza.
- [ ] Opt-in example for PostToolUse stanza is documented in `working-with-llms.md` (WP03) and `pre-commit-hook.md` (WP06).
- [ ] `lib/templates/.claude/scripts/session-context.sh` exists, is executable, completes in < 200ms.
- [ ] `lib/templates/.claude/scripts/require-in-session.sh` exists, is executable, enforces the sentinel-file gate, emits blocking exit 2 when `/in-session` has not run.
- [ ] `lib/templates/.claude/scripts/post-tool-advisory.sh` exists, is executable, parses tool-use JSON from stdin, short-circuits on `.intent_critic.yml post_tool_use_advisory: false`, invokes `intent critic` on single file, exits 0 always.
- [ ] All three scripts degrade gracefully on missing dependencies (no git, no `intent/wip.md`, no `.intent_critic.yml`, no `intent` on PATH).
- [ ] Tested: dropping the template into a scratch project's `.claude/settings.json` produces the expected system-reminder injection on session start, blocks a first-prompt without `/in-session`, and injects advisory findings after a Write to a rule-violating file.
- [ ] `intent/llm/MODULES.md` registers `lib/templates/.claude/` and all three helper scripts.
- [ ] Opt-out mechanism documented in `working-with-llms.md` (WP03 cross-ref) — both for the strict gate and the PostToolUse advisory.
- [ ] Commit follows Intent conventions.

### Tests to add

- **BATS test for `session-context.sh`**: input: mock project with git + wip.md; assert stdout contains expected strings. Input 2: no git; assert graceful degradation. Input 3: no wip.md; assert graceful degradation.

### Tests to update

None.

## Dependencies

- **Blocked by**: WP01 (version bump).
- **Blocks**: WP11 (`intent claude upgrade` installs this template).
- **Coordinates with**: WP03 (working-with-llms.md references this template's JSON structure).

## Implementation Notes

- **Hook JSON schema is terse**: Claude Code's hook spec is simple but literal. Double-check the matcher regex syntax — some events expect regex strings, others expect empty string or glob.
- **session-context.sh must be fast**: it runs on every session start. Target < 200ms. Use `git rev-parse --short HEAD` over `git log -1`. Cache nothing — always fresh.
- **Opt-out pattern**: users can override hooks in `.claude/settings.local.json`. Don't try to prevent this; document it as a feature, not a bug.
- **No user-secret content in hook output**: hook stdout becomes part of the conversation. Don't leak anything sensitive from the project env.
- **Intent-aware but tool-agnostic**: the script should work in any Intent project regardless of language. Don't hardcode Elixir/Rust/etc. assumptions.

## Risks and Edge Cases

- **Risk**: Hook spec changes between Claude Code versions. **Mitigation**: pin schema to the version at WP04 shipment (2026-04 spec); revisit on Claude Code major bumps.
- **Risk**: session-context.sh breaks on Windows / non-POSIX shells. **Mitigation**: Intent already targets bash 3.x macOS/Linux. Document Windows as out-of-scope. If Windows support is needed later, rewrite as cross-platform script.
- **Risk**: The Stop hook fires too often and becomes annoying. **Mitigation**: matchers are permissive by default; if user feedback says it's noisy, tighten the matcher to fire only on first Stop in a session (stateful — requires session-id tracking). Defer to v2.9.2 if needed.
- **Edge**: `intent/wip.md` parsing — if it's empty or malformed, script must not fail. Echo "No active WIP" and move on.
- **Edge**: Race condition on session start with very slow git repos. Unlikely; acceptable to timeout at 200ms without crashing the hook.

## Verification Steps

1. `jq . lib/templates/.claude/settings.json` exits 0.
2. `bash lib/templates/.claude/scripts/session-context.sh` from Intent root emits expected output (project: Intent, branch: current, SHA, WIP).
3. `bash lib/templates/.claude/scripts/session-context.sh` from `/tmp` (non-git dir) degrades gracefully — no crash, sensible "no project detected" output.
4. Copy template to `/tmp/scratch/.claude/settings.json` of a git repo, open Claude Code in `/tmp/scratch`, confirm system-reminder injection (manual verification — document outcome in WP14 as-built).
5. `tests/run_tests.sh` passes, with new BATS test for `session-context.sh` green.

## Size and Estimate

- **Size**: M (Medium). 2 sessions.
- Session 1: Read hook spec; author JSON template; author session-context.sh.
- Session 2: BATS test; MODULES.md; WP03 coordination for doc example; commit.

## Exit Checklist

- [ ] Template JSON valid + matches hook spec.
- [ ] session-context.sh tested under all three scenarios (git+wip, git+no-wip, no-git).
- [ ] MODULES.md updated.
- [ ] Cross-ref in working-with-llms.md confirmed (WP03 responsibility, but flagged in this WP's commit note).
- [ ] Commit follows Intent conventions.
