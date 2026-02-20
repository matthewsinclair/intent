---
description: "Session forensics: analyzes Claude Code sessions against memory rules, finds gaps and enforcement failures"
---

# Intent Autopsy

Session analysis and memory meta-learning. Compares what happened in Claude Code sessions against what MEMORY.md and CLAUDE.md say should happen. Finds the gaps. Proposes fixes.

## Procedure

### 1. Gather parameters

Ask the user:

- Time range (default: last 7 days)
- Minimum compactions filter (default: 0, use 1 to focus on compacted sessions)
- Any additional banned words beyond the defaults
- Project name filter (optional, defaults to most recent)

### 2. Run the Elixir script

The script lives alongside this SKILL.md:

```bash
elixir "$(dirname "$(find ~/.claude/skills/intent-autopsy -name autopsy.exs 2>/dev/null | head -1)")/autopsy.exs" \
  --days 7 \
  -o /tmp/autopsy-findings.json
```

Adjust flags based on user input:

- `--days N` for time range
- `--min-compactions N` for compaction filter
- `--project NAME` for project filter
- `--banned-words "word1,word2"` for additional banned words
- `--banned-file PATH` for custom banned words file

The script outputs JSON to the specified file and progress to stderr.

### 3. Read the findings

Read the JSON findings file. Also read:

- The project's MEMORY.md (check `~/.claude/projects/*/memory/MEMORY.md`)
- The project's CLAUDE.md (in project root)
- The user's global CLAUDE.md (`~/.claude/CLAUDE.md`)

### 4. Classify findings against memory

For each correction and frustration signal from the findings:

**Memory Gap**: The correction addresses something with NO matching rule in MEMORY.md or CLAUDE.md. This is a candidate for a new memory entry.

**Enforcement Failure**: A matching rule EXISTS in MEMORY.md or CLAUDE.md but was violated anyway. The rule needs strengthening, better examples, or elevation (e.g., from MEMORY.md to CLAUDE.md).

### 5. Find undocumented conventions

Look across multiple sessions for consistent patterns that are NOT captured in any memory file:

- Repeated corrections about the same topic
- Consistent tool usage patterns
- Workflow sequences that appear in every session

### 6. Find stale memory

Check each rule in MEMORY.md against the findings. Rules that were never triggered, never relevant to any correction, and never referenced in any session are candidates for pruning.

### 7. Produce the report

Save the report to `intent/autopsy/YYYYMMDD.md` (create the directory if needed).

## Report Format

```markdown
# Autopsy Report: YYYYMMDD

## Summary

- Sessions analyzed: N | Compactions: N | Date range: YYYY-MM-DD to YYYY-MM-DD
- Corrections: N (M post-compaction) | Frustration signals: N
- User flags: N | Deferrals: N (L legitimate) | Banned violations: N

## User Flags (highest confidence)

[If any user flags found, list them first -- these are explicit markers]

## Memory Gaps (corrections with no matching rule)

### Gap 1: [short description]

- Evidence: [session_id, line, user quote]
- Proposed MEMORY.md addition:
```

[Concrete text to add to MEMORY.md]

```

## Enforcement Failures (rules violated despite existing)
### Failure 1: [rule from MEMORY.md or CLAUDE.md]
- Rule: [exact text of the existing rule]
- Evidence: [session_id, line, quote]
- Recommendation: [strengthen wording / add example / elevate to CLAUDE.md]

## Undocumented Conventions
### Convention 1: [pattern description]
- Evidence: [sessions where this appeared]
- Proposed MEMORY.md addition:
```

[Concrete text]

```

## Stale Memory
- [rule text] -- not triggered in N analyzed sessions

## Correction Pairs
[Full list with session_id, line numbers, user message, assistant acknowledgment, post-compaction flag]

## Frustration Signals
[Full list with session_id, line numbers, signal types, post-compaction flag]

## Deferrals
[Full list, separated into legitimate vs suspicious]

## Banned Pattern Violations
[Full list, noting negations (anti-examples) vs actual violations, and whether found in conversation or tool_use inputs]
```

## Second Pass

After producing the initial report, do a second pass:

1. **Missed corrections**: Were there cases where Claude was wrong but the user just fixed it themselves without a correction acknowledgment? Look for user messages that re-state something with emphasis.

2. **Deep scan**: Check tool_use inputs in the findings. Banned words in Edit/Write/Bash inputs are more significant than in conversation text.

3. **Deferral legitimacy**: Review each deferral. Could Claude actually have done the thing? Browser/login/2FA deferrals are legitimate. API/file/command deferrals usually are not.

Note findings from both passes in the report.

## Important Notes

- This skill requires Elixir to be installed (`elixir --version` to check)
- Session files are at `~/.claude/projects/<project-key>/*.jsonl`
- The script reads files but never modifies them
- Large sessions (hundreds of MB) are handled by the script's streaming parser
- Reports are proposed, never auto-applied. The user decides what memory updates to make.
