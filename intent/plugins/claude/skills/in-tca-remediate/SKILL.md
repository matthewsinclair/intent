---
description: "TCA remediate: execute prioritized fix batches in main conversation with compile/test gates"
---

# TCA Remediate

Executes the remediation phase (Phase 4) of a Total Codebase Audit. All remediation happens in the main conversation with compile/test gates between batches.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Read synthesis output

Read the synthesis WP's `socrates.md` from `/in-tca-synthesize`. This contains the prioritized remediation backlog with fix batches.

### 2. Confirm remediation scope

Ask the user which batches to tackle this session:

- All batches (A through H)?
- Just P0 and P1 (Batches A-D)?
- Specific batches?

Set expectations: P0+P1 typically takes 1-2 hours. Full remediation may take 2-4 hours depending on scope.

### 3. Execute remediation in main conversation

**DO ALL REMEDIATION IN THE MAIN CONVERSATION.** Sub-agents cannot coordinate compile+test cycles and hit permission walls on file edits.

For each batch:

1. **Apply fixes** for each item in the batch
2. **After every 5-8 edits**: run compile+test gate

   ```bash
   # Elixir
   mix compile --warnings-as-errors && mix test --failed

   # Rust
   cargo check && cargo test

   # Swift
   swift build && swift test
   ```

3. **Track completion** at individual item level, not batch level
4. **Mark false positives** explicitly -- do not just skip them, record why
5. **Commit at natural stopping points** (end of each batch or every 10-15 fixes)

### 4. Batch ordering

Execute in priority order:

```
Batch A: P0 critical bugs        -- test after each fix
Batch B: P1 top Highlander fix   -- test after batch
Batch C: P1 second Highlander    -- test after batch
Batch D: P1 domain-scoped dedup  -- test after batch
Batch E: P2a mechanical fixes    -- test after batch
Batch F: P2b refactoring         -- test after each extraction
Batch G: P2b multi-head          -- test after batch
Batch H: P3 style fixes          -- test after batch
```

**Interleaving**: P2a mechanical fixes (Batch E) can be interleaved with P1 work since they touch different files. P2b refactoring should wait until P1 shared modules exist.

### 5. Parallel extraction (exception only)

For P1/P2b extraction work ONLY, when file scopes are completely disjoint:

1. Sub-agents may be used for extraction
2. **Create target modules FIRST**, then update callers (creation-before-migration)
3. Main session does verification pass after all agents complete
4. Ensure file scopes are truly disjoint before launching parallel agents

### 6. Test optimization

Between individual edits (fast feedback):

```bash
mix test --failed
mix test test/specific_file_test.exs
```

At batch boundaries (full verification):

```bash
# Elixir
mix compile --warnings-as-errors && mix test && mix credo --strict

# Rust
cargo check && cargo test && cargo clippy -- -D warnings

# Swift
swift build && swift test
```

### 7. Handle false positives

If a fix breaks tests, investigate the data type before forcing the fix:

- `Map.get` on a plain map is correct -- only flag on known defstructs (R7)
- `&&` on a truthy non-boolean value may be intentional (R8)
- `String.to_atom` on controlled inputs may be correct (not user input)

Revert broken fixes immediately. Mark as false positive with explanation in the synthesis doc.

### 8. Final verification

After all batches are complete:

```bash
# Full verification pass
mix compile --warnings-as-errors && mix test && mix credo --strict
```

Commit the final state. Proceed to `/in-tca-finish`.

## Important Notes

- NEVER do remediation in sub-agents (except parallel extraction with disjoint files)
- Track completion at individual item level, not batch level
- Mark false positives explicitly with reasons
- Create target modules FIRST, then update callers
- Run mix test --failed between edits; full suite at batch boundaries
- Commit at natural stopping points, not at the very end
