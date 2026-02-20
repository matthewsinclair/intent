# Implementation - ST0021: Intent Autopsy

## Implementation

### Elixir Script (`autopsy.exs`)

Six modules in a single `.exs` file with `Mix.install([{:jason, "~> 1.4"}])`:

- **Autopsy.Parser** -- JSONL parsing, message/compaction extraction, project dir resolution
- **Autopsy.Census** -- Baseline metrics (session count, compactions, messages, bytes, date range)
- **Autopsy.Corrections** -- Correction pairs (acknowledgment patterns), frustration signals, user flags
- **Autopsy.Regressions** -- Deferral detection with legitimacy filtering and meta-discussion exclusion
- **Autopsy.Rules** -- Banned pattern deep scan (conversation + tool_use inputs) with negation detection
- **Autopsy.CLI** -- Argument parsing, orchestration, JSON output

Key design choices:

- `File.stream!()` with `Stream.with_index()` for memory-efficient parsing of large sessions
- Compaction detected via `type: "system", subtype: "compact_boundary"` (discovered from real data)
- User message extraction skips tool_result-only messages to find actual user text
- Negation detection checks 60-char window before pattern match
- Default banned-words.txt auto-loaded from script directory via `__DIR__`

### Skills Infrastructure

Changed `intent_claude_skills` install and sync to use `cp -r "$source/"* "$target/"` instead of
copying just SKILL.md. Sync still checksums SKILL.md only. Uninstall already used `rm -rf`.

### Doctor Command

Added Elixir to the optional tools check in `intent_doctor`. Reports as optional since only
`intent-autopsy` requires it.

## Technical Details

### Session JSONL Format

Messages have `type` field: `user`, `assistant`, `system`, `progress`, `file-history-snapshot`, `queue-operation`.

Compaction boundaries are `type: "system"` with `subtype: "compact_boundary"` and contain:

```json
{
  "compactMetadata": {
    "trigger": "auto",
    "preTokens": 167374
  }
}
```

### Pattern Matching

Correction acknowledgments: "you're right", "I apologize", "I was wrong", "good catch", "my mistake", etc.

Frustration signals: "how many times", "I told you", "you forgot", "wrong file", "did you actually look", etc.

Deferral patterns: "you'll need to manually", "I can't access", "outside my capabilities", etc.

Legitimacy filters: browser, login, 2FA, CAPTCHA, physical, password, GUI, phone, etc.

## Challenges & Solutions

### Empty user messages in corrections

Initially, correction pairs showed empty user messages because the preceding "user" message was
a tool_result (no text content). Fixed by adding `has_text_content?/1` filter to skip
tool_result-only messages when searching for the preceding user correction.

### Self-referential banned word matches

The autopsy plan itself mentions banned words as examples (e.g., "delve", "I'd be happy to").
When running against sessions that discussed the plan, these show up as violations. The negation
detection helps somewhat, but some meta-references are expected. The skill's second-pass
instructions tell Claude to account for this.
