---
description: "TCA audit: execute component audits with sub-agents, track progress, manage context"
chains_to: ["in-tca-synthesize"]
---

# TCA Audit

Executes the component audit phase (Phase 1) of a Total Codebase Audit. Manages sub-agent launches, progress tracking, context management, and commit discipline.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Check progress

Run the progress script to see current state:

```bash
bash "$(find ~/.claude/skills/in-tca-audit -name tca-progress.sh 2>/dev/null | head -1)" \
  --tca-dir intent/st/STXXXX
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

### 4. Dispatch the critic

Per WP, dispatch the language critic against the WP's file set. There is no custom prompt template — the critic enforces the rule library automatically and emits a stable severity-grouped report (see `intent/docs/critics.md`).

```
Task(subagent_type="critic-<lang>", prompt="review <file1> <file2> ...")
```

For test files in the WP's manifest, also run `test-check`:

```
Task(subagent_type="critic-<lang>", prompt="test-check <test_file1> <test_file2> ...")
```

`<lang>` is the WP's language per `info.md`. Polyglot WPs run one dispatch per language. The critic auto-loads the right rule packs (agnostic + language code/test + framework subdirs) and honours the project's `.intent_critic.yml`.

**Critic selection**:

| Project signal    | Critic to dispatch |
| ----------------- | ------------------ |
| `mix.exs`         | `critic-elixir`    |
| `Cargo.toml`      | `critic-rust`      |
| `Package.swift`   | `critic-swift`     |
| `.luarc.json`     | `critic-lua`       |
| Bash/zsh shebangs | `critic-shell`     |

This matches the `/in-review` stage-2 dispatcher.

### 4a. Capture the critic report

Write the captured critic output to `WP/{NN}/socrates.md` verbatim, with a small header for context:

```markdown
# WP-{NN} {Component Name} -- Critic Audit

**Critic**: critic-<lang> (review + test-check)
**Files**: {N} ({code count} code, {test count} test)
**Date**: {YYYY-MM-DD}

<critic report verbatim, including all severity sections and the Summary line>

## Cross-WP Highlander notes

- vs. WP-XX: {what to investigate at synthesis}
- vs. WP-YY: {what to investigate at synthesis}
```

The critic report itself owns the rule IDs (IN-\*), severities (CRITICAL/WARNING/RECOMMENDATION/STYLE), file:line citations, and one-line violation descriptions. The wrapper only adds component identity, dispatch metadata, and cross-WP Highlander handoffs that synthesis will consume.

If the critic emits zero findings, the report still includes the bare `Summary: 0 critical, 0 warning, 0 recommendation, 0 style.` line — record it. Absence of findings is a first-class outcome.

### 5. Post-WP

After each critic dispatch completes:

1. **Record metadata** at the top of `WP/{NN}/socrates.md` immediately under the H1. Format: `**Critic**: critic-<lang>; **Files**: N (code:M test:K); **Findings**: critical=A warning=B recommendation=C style=D; **FPs noted**: N`. The findings counts come straight from the critic's `Summary:` line; FPs noted is the count flagged in §4a's cross-check against pre-filter ground truth. This line is queryable across audits and feeds the "Critic Effectiveness" section of the final feedback report.
2. **Commit immediately**: `git add WP/{NN}/socrates.md && git commit -m "audit: WP-{NN} {component}"`
3. **Log the summary** in your running tally
4. **Run progress check** to update status

### 6. Repeat

Continue dispatching the critic for each WP in the current batch, then move to the next batch per the ordering in design.md. WPs within a batch can run in parallel if they don't share files; dispatch them in the same message so the critics run concurrently.

**Critic registration freeze**: if any critic was installed mid-session (`intent claude subagents install critic-<lang>`), the `Task()` dispatch will fail with "subagent not found" until the next session starts. Restart Claude Code before launching the audit. See `intent/docs/critics.md` §Operational note.

### 7. Final check

After all component WPs are done, run the progress script one final time:

```bash
bash "$(find ~/.claude/skills/in-tca-audit -name tca-progress.sh 2>/dev/null | head -1)" \
  --tca-dir intent/st/STXXXX
```

Exit code 0 confirms all WPs complete. Proceed to `/in-tca-synthesize`.

## Crash Prevention

| Risk             | Mitigation                                                         |
| ---------------- | ------------------------------------------------------------------ |
| Context overflow | `/compact` before each WP                                          |
| Lost work        | Commit after every WP (never batch)                                |
| Critic not found | Restart session before audit if critics were installed mid-session |
| Session crash    | Keep a running log outside the session                             |
| WP too large     | Split WPs with >60 files into sub-WPs                              |
| File manifest    | Verify files exist before each WP                                  |

## Important Notes

- Never batch commits -- commit after every single WP
- Capture the critic report verbatim. Critics never invent IN-\* IDs and never invent file:line citations; the report is ground truth for synthesis.
- Cross-WP Highlander notes are the wrapper's job — the critic does not see other WPs.
- If context usage exceeds 70%, start a fresh session rather than risk truncation.
