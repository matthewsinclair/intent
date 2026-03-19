---
description: "TCA init: provision steel thread, rule set, component map, and work packages for a Total Codebase Audit"
---

# TCA Init

Provisions a Total Codebase Audit: creates the steel thread, defines the rule set, maps the codebase into components, and generates work package directories with templated info.md files.

For reference: `intent/docs/total-codebase-audit.md`

## Procedure

### 1. Gather parameters

Ask the user:

- Project name
- Ecosystem(s): Elixir, Rust, Swift, TypeScript, or polyglot combination
- Approximate file count (or let discovery determine it)
- Whether the project uses Ash Framework
- Whether it is an umbrella/monorepo

### 2. Define the rule set

Load ecosystem-appropriate defaults from the TCA reference doc:

- **Elixir**: R1-R15 (add A1-A5 if Ash)
- **Rust**: R1-R12 (validated rules with context-severity)
- **Swift**: R1-R10 (validated rules with context-sensitivity)
- **Polyglot**: separate rule sets per ecosystem + X-rules for cross-ecosystem concerns

Customize with the user:

- Remove rules that don't apply (e.g., R10 content source rules)
- Add project-specific rules
- Define rule precision boundaries (what each rule does NOT apply to)

### 3. Map codebase into components

Guided discovery:

1. List top-level source directories
2. For each directory, enumerate files by type using glob patterns
3. Calculate **effective file counts** using weight table:

| File Type         | Weight | Rationale                          |
| ----------------- | -----: | ---------------------------------- |
| Standard code     |   1.0x | Normal audit effort                |
| Ash DSL resources |  0.25x | Declarative, limited violations    |
| Emission/struct   |   0.5x | Thin files, few rules apply        |
| Dead stubs        |   0.0x | Exclude entirely                   |
| Rust code         |   1.5x | Ownership + lifetime complexity    |
| Swift AppKit      |   1.3x | Legacy framework, higher density   |
| Test files        |   0.0x | Excluded unless testing rules used |

4. Group files into WPs targeting 12-20 effective files each
5. Identify 3-4 special focus rules per WP
6. Identify cross-WP Highlander dependencies

### 4. Create steel thread

```bash
intent st new "Total Codebase Audit" --start
```

Capture the ST ID from the output.

### 5. Run tca-init.sh

The script lives alongside this SKILL.md:

```bash
bash "$(find ~/.claude/skills/in-tca-init -name tca-init.sh 2>/dev/null | head -1)" \
  --st-dir intent/st/STXXXX \
  --wp-count N \
  --project "ProjectName"
```

This creates WP directories with templated info.md files and empty socrates.md files. The last WP is always the synthesis WP.

### 6. Populate WP info.md files

For each WP, fill in:

- Scope description (1-2 sentences)
- Complete file list (every file to be audited)
- Applicable rules with special focus
- Cross-WP Highlander dependencies (2-4 other WPs that might overlap)

### 7. Write design.md

The steel thread's design.md should contain:

- Complete rule set with "What to Check" descriptions
- Component map with effective file counts
- Batch ordering for parallelization (dependency-ordered)
- Pre-filter results (Phase 0.5)

### 8. Write tasks.md

Phase checklist:

- Phase 0: Provisioning (this step)
- Phase 0.5: Pre-filtering
- Phase 1: Component audit (list all WPs)
- Phase 2: Synthesis
- Phase 3: Review
- Phase 4: Remediation

### 9. Pre-filter mechanical rules (Phase 0.5)

Run grep for mechanical rules before launching sub-agents:

```bash
# R15: Debug artifacts
grep -rn "IO\.inspect\|dbg()" lib/ --include="*.ex"

# R8: Boolean operator candidates
grep -rn " && \| || " lib/ --include="*.ex"

# R11: Missing @impl candidates
grep -rL "@impl" lib/ --include="*.ex" | xargs grep -l "def mount\|def handle_"
```

Adapt patterns for the project's ecosystem. Record results in design.md.

### 10. Verify file manifests

For every WP, confirm all listed files exist:

```bash
# Quick verification
for f in $(grep "^- \`" intent/st/STXXXX/WP/01/info.md | sed 's/.*`\(.*\)`.*/\1/'); do
  [ -f "$f" ] || echo "MISSING: $f"
done
```

Fix any stale references before proceeding to Phase 1.

## Important Notes

- The sweet spot is 12-20 effective files per WP
- Always create a synthesis WP as the last WP
- Encode cross-WP Highlander dependencies at provisioning time, not synthesis time
- Pre-filter results are ground truth for validating sub-agent findings
