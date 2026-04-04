---
description: "Verification gate: require fresh evidence before any completion claim"
---

# Verification Gate

Before claiming any task is complete, working, or passing, provide fresh verification evidence. No exceptions.

## Rules

### 1. Show actual output

Every completion claim must include the actual command output that proves it. Do not paraphrase, summarize, or reference a prior run.

BAD: "Tests pass."
BAD: "I verified this works."
BAD: "As shown earlier, the tests pass."

GOOD: Run the command, show the output, then state the conclusion.

### 2. Run verification in this message

Verification must happen in the current message, not reference a prior message. Context compaction may have removed prior evidence.

### 3. Verify the specific change

Run the specific test or check that covers your change. "All 462 tests pass" is less useful than "the 3 tests covering this module pass." Run both if practical: specific first, then broad.

### 4. Types of verification

| Change Type      | Minimum Verification                              |
| ---------------- | ------------------------------------------------- |
| Code change      | Run relevant tests; show output                   |
| Config change    | Show the config loads without error               |
| Build change     | Run build; show success                           |
| Doc change       | Confirm file exists and render check (if tooling) |
| Skill/agent file | Install and show in list output                   |
| Bug fix          | Show the bug is gone AND no regressions           |

### 5. What "done" requires

A task is done when:

- Specific verification command was run in this message
- Output is shown (not summarized)
- Output confirms the change works
- No new warnings or errors introduced

## Red Flags

| Rationalization                       | Reality                                               |
| ------------------------------------- | ----------------------------------------------------- |
| "I just ran the tests above"          | Context may be compacted. Run again.                  |
| "This is a doc-only change, no tests" | Confirm the file exists and has no syntax errors.     |
| "The change is too small to verify"   | Small changes break things. Verify takes seconds.     |
| "I'll verify at the end"              | Verify each step. Compound errors are harder to find. |
