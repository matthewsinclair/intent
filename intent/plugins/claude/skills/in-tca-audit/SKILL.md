---
description: "TCA audit: execute component audits with sub-agents, track progress, manage context"
---

# TCA Audit

Executes the component audit phase (Phase 1) of a Total Codebase Audit. Manages sub-agent launches, progress tracking, context management, and commit discipline.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Check progress

Run the progress script to see current state:

```bash
bash "$(find ~/.claude/skills/in-tca-audit -name tca-progress.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX
```

This shows which WPs are complete, pending, or in-progress, with violation counts.

### 2. Select next WP batch

From the batch ordering in `design.md`, identify the next batch of WPs to audit. WPs within a batch can run in parallel if they don't share files.

### 3. Pre-flight checklist (per WP)

Before launching each sub-agent:

1. **Run `/compact`** to reclaim context window space
2. **Verify file manifest** -- confirm all files in the WP's `info.md` exist:
   ```bash
   for f in $(grep "^- \`" WP/NN/info.md | sed 's/.*`\(.*\)`.*/\1/'); do
     [ -f "$f" ] || echo "MISSING: $f"
   done
   ```
3. **Check context usage** -- if above 70%, consider fresh session
4. **Verify previous WP committed** -- no uncommitted audit files

### 4. Launch sub-agent

Use the prompt template with these required elements:

```
You are performing a forensic code audit of the **{Component Name}** subsystem
in the {Project} application.

### Audit Rules

| #   | Rule                    | What to Check                              |
| --- | ----------------------- | ------------------------------------------ |
| R1  | {rule name}             | {brief description of what to look for}    |
...

### Files to Audit

1. `{path/to/file1}`
2. `{path/to/file2}`
...

### Special Focus

{R-numbers and why they matter for this component}

### Cross-WP Highlander Check

- vs. WP-XX: {what logic might be duplicated and why}
- vs. WP-YY: {what logic might be duplicated and why}

### Instructions

Read EVERY file listed above. For each file, check every function against
all rules. Report violations in this exact format:

#### V{N}: {short title}

- **File**: `{path}:{line(s)}`
- **Rule**: R{N} -- {rule name}
- **Severity**: High | Medium | Low
- **Confidence**: HIGH | MEDIUM | LOW
- **Description**: {what the violation is}
- **Remedy**: {how to fix it}

After all violations, add a summary table:

### Summary

| Severity  | Count |
| --------- | ----- |
| High      | X     |
| Medium    | Y     |
| Low       | Z     |
| **Total** | **T** |

Write the complete audit to `{path/to/WP/NN/socrates.md}`.

Be thorough. Do NOT skip files. Do NOT invent violations -- only report
what you actually see in the code. If a file has no violations, say so
explicitly.
```

**Agent selection**:

| Agent Type        | Use For                                                  |
| ----------------- | -------------------------------------------------------- |
| `diogenes`        | Primary choice -- Socratic dialog, structured output     |
| `general-purpose` | Large polyglot WPs where no specialized agent matches    |
| `Explore`         | Pre-audit reconnaissance if component boundaries unclear |
| `elixir`          | Elixir-specific deep dives (post-audit)                  |

The prompt is the quality driver, not the agent type.

### 5. Post-WP

After each sub-agent completes:

1. **Commit immediately**: `git add WP/{NN}/socrates.md && git commit -m "audit: WP-{NN} {component}"`
2. **Log the summary** in your running tally
3. **Run progress check** to update status

### 6. Repeat

Continue launching sub-agents for each WP in the current batch, then move to the next batch per the ordering in design.md.

### 7. Final check

After all component WPs are done, run the progress script one final time:

```bash
bash "$(find ~/.claude/skills/in-tca-audit -name tca-progress.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX
```

Exit code 0 confirms all WPs complete. Proceed to `/in-tca-synthesize`.

## Crash Prevention

| Risk             | Mitigation                             |
| ---------------- | -------------------------------------- |
| Context overflow | `/compact` before each WP              |
| Lost work        | Commit after every WP (never batch)    |
| Sub-agent stall  | Set max_turns limit on large WPs       |
| Session crash    | Keep a running log outside the session |
| WP too large     | Split WPs with >60 files into sub-WPs  |
| File manifest    | Verify files exist before each WP      |

## Important Notes

- Never batch commits -- commit after every single WP
- The anti-hallucination instruction is not optional
- Include the confidence field in every violation
- If context usage exceeds 70%, start a fresh session rather than risk truncation
