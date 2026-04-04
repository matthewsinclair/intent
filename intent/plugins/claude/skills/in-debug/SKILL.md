---
description: "Systematic debugging: 4-phase process with 3-strike architectural review"
chains_to: ["in-verify"]
---

# Systematic Debugging

Follow this procedure when investigating any bug or unexpected behavior. Do not skip phases.

## Procedure

### Phase 1: Reproduce and gather evidence

1. Read the actual error message. Do not guess what it says.
2. Reproduce the issue. If you cannot reproduce it, you cannot fix it.
3. Check recent changes (`git log --oneline -10`, `git diff`)
4. Identify the exact file and line where the error occurs
5. Document: what was expected, what happened, exact error output

### Phase 2: Pattern analysis

1. Find working code that does something similar
2. Compare the broken code against the working code line by line
3. Check if this is a known pattern (search issues, git blame the area)
4. Look for recent changes to dependencies or config that could cause this

### Phase 3: Hypothesis testing

1. Form ONE hypothesis about the root cause
2. Test ONLY that hypothesis -- change one variable at a time
3. If the hypothesis is wrong, revert the change before trying the next one
4. Document each hypothesis and its result

NEVER change two things at once. NEVER leave a failed fix in place while trying the next one.

### Phase 4: Fix and verify

1. Write a test that reproduces the bug (if testable)
2. Implement the fix
3. Run the reproducing test -- it must pass
4. Run the broader test suite -- no regressions
5. Invoke `/in-verify` to confirm

## The 3-Strike Rule

If 3 consecutive fix attempts fail for the same issue:

1. STOP attempting fixes
2. Present the pattern of failures to the user:
   - What was tried
   - Why each attempt failed
   - What this pattern suggests
3. Ask: "Is the approach itself wrong? Should we reconsider the architecture?"
4. Do NOT attempt a 4th fix without explicit user direction

This rule prevents thrashing. Three failed fixes usually means you are solving the wrong problem.

## Red Flags

| Rationalization                      | Reality                                                         |
| ------------------------------------ | --------------------------------------------------------------- |
| "Let me just try one more thing"     | That's what you said 3 fixes ago. Stop and think.               |
| "I think I know what it is"          | Thinking is not knowing. Reproduce first.                       |
| "The fix is obvious"                 | Obvious fixes don't need 3 attempts. Something deeper is wrong. |
| "I'll clean up the debug code later" | Remove debug artifacts before committing. Now.                  |
