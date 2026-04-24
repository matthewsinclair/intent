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

From design D7: soft reminder is the default (not a hard-gating `UserPromptSubmit` hook). Open decision #2 lets the user flip to strict.

WP04 ships the template only — installation into projects is WP11.

## Deliverables

1. **Template file** at `lib/templates/.claude/settings.json`. JSON shape:

   ```json
   {
     "hooks": {
       "SessionStart": [
         {
           "matcher": "startup|resume|clear|compact",
           "hooks": [
             {
               "type": "command",
               "command": "echo 'Intent project detected. Run /in-session before your first response to load coding standards, rules, and session discipline.'"
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

   (Final shape verified against Claude Code hook spec during WP04.)

2. **Intent-aware context**: the SessionStart hook command should be richer than `echo` — pipe in:
   - Current git branch + short SHA.
   - Current `intent wp` WIP (if any) from `intent/wip.md`.
   - Active ST ID.
   - Project name.

   This becomes a helper script at `lib/templates/.claude/scripts/session-context.sh` that the hook `command:` invokes. The script must be idempotent and fast (<200ms) — it runs on every session start.

3. **Template placeholders** if any project-specific content is needed (likely none — session-context.sh discovers project details dynamically).

4. **MODULES.md registration** for the new template dir `lib/templates/.claude/` and the helper script.

5. **User-level opt-out documentation** in `working-with-llms.md` (WP03 cross-references this) — how to disable hooks per-session in `.claude/settings.local.json` for users who don't want them.

## Approach

1. **Read Claude Code hooks docs** to confirm exact JSON schema and matcher regex semantics. Anthropic docs: https://docs.claude.com/en/docs/claude-code/hooks (verify URL at WP04 start).
2. **Confirm hook command behaviour**:
   - Stdout → injected as system-reminder.
   - Exit 0 → allow; exit 2 → block (don't block on SessionStart).
3. **Author the template** with the JSON structure above. Keep permissive matchers so hooks fire on all session-start variations.
4. **Author session-context.sh**:
   - Detect git repo (`git rev-parse --is-inside-work-tree`); if no git, skip git-context injection gracefully.
   - Read `intent/wip.md` if present; echo the first non-empty line or "No active WIP".
   - Echo current branch, short SHA, project name (from `.intent/config.json`).
   - All output goes to stdout — becomes the system-reminder content.
5. **Reasonable defaults for local.json precedence**: user overrides in `.claude/settings.local.json` win. Document the precedence in the template's comments / header (JSON with comments isn't valid, so comment in the `/*` section of adjacent MODULES entry and in working-with-llms.md).
6. **Sanity-test** the template by copying it to Intent's own `.claude/settings.json` in a scratch worktree and confirming hooks fire — but don't commit Intent's settings yet (that's WP14's dogfood).
7. **Commit** the template files and MODULES.md update.

## Acceptance Criteria

- [ ] `lib/templates/.claude/settings.json` exists and is valid JSON (`jq . lib/templates/.claude/settings.json` returns 0).
- [ ] Template has `hooks.SessionStart` with matcher covering `startup|resume|clear|compact`.
- [ ] Template has `hooks.Stop` with a matcher that fires after every assistant turn.
- [ ] Template's SessionStart command invokes `session-context.sh` (or inlines a reasonable subset).
- [ ] `lib/templates/.claude/scripts/session-context.sh` exists, is executable, and completes in < 200ms on a typical project.
- [ ] `session-context.sh` emits: project name, git branch, short SHA, and WIP summary (when available).
- [ ] `session-context.sh` degrades gracefully when not in a git repo or when `intent/wip.md` is absent.
- [ ] Tested: dropping the template into Intent's `.claude/settings.json` (in a scratch worktree) produces the expected system-reminder injection on session start. (Documented observation in WP14 when self-apply runs.)
- [ ] `intent/llm/MODULES.md` registers `lib/templates/.claude/` and `lib/templates/.claude/scripts/session-context.sh`.
- [ ] Template uses permissive matchers — no missed session-start events.
- [ ] Opt-out mechanism documented in `working-with-llms.md` (WP03 cross-ref).
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
