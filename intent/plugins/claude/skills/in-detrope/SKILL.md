---
description: "Detrope: diagnoses LLM writing tropes and stylometric tells, assesses in project context, shows remediation plan"
---

# Detrope

Scans documents for LLM writing tropes and stylometric tells. Assesses findings
in context of the project where it runs. Produces a diagnostic report with
severity-ranked findings and concrete rewrites.

Trope catalog: [llm-tropes](https://github.com/matthewsinclair/llm-tropes)
(44 tropes across 8 categories, vendored in data/trope-catalog.md).

## Procedure

### Step 1 -- Gather parameters

Ask the user for:

- **Target**: file path or glob pattern (required)
- **Mode**: `quick` (diagnosis only) or `full` (diagnosis + remediation rewrites). Default: `full`
- **Document type override**: `expository` | `narrative` | `technical` | `internal` (optional -- normally auto-detected from context)
- **Severity floor**: minimum severity to report: `low` | `medium` | `high`. Default: `low`
- **Save report**: yes or no. Default: no (output to conversation). If yes, save to `intent/detrope/YYYYMMDD-<slug>.md`

### Step 2 -- Establish project context

Read these files in order to understand what you are analyzing and for whom:

1. The project's `CLAUDE.md` -- project purpose, conventions, audience
2. The project's `README.md` -- what the project is
3. The target file's frontmatter and first few paragraphs -- what this specific document covers

Extract and note:

- **Project purpose**: what this project does and who uses it
- **Target audience**: developers, general public, internal team, etc.
- **Document type**: blog post, technical docs, README, internal design doc, etc.
- **Tone expectations**: formal, conversational, technical, marketing

This context drives Step 5's contextual assessment. A trope in a published blog
post is worse than the same trope in an internal design doc. A "serves as" in
API documentation may be legitimate.

### Step 3 -- Read the trope catalog

Read the vendored trope catalog:

```bash
cat "$(find ~/.claude/skills/in-detrope -name trope-catalog.md 2>/dev/null | head -1)"
```

If the file is too large to read at once, read the Table of Contents first, then
read individual trope sections as needed during analysis.

### Step 4 -- Resolve and read target files

Expand the target path or glob pattern.

```bash
ls -1 <target-pattern> 2>/dev/null | head -40
```

Rules:

- If more than 10 files match: show the list, ask the user to confirm or narrow
- If more than 30 files match: refuse, ask to narrow the pattern
- Process files one at a time to manage context

For each file, read its full contents.

### Step 5 -- Analyze each file

Perform two passes per file:

**Pass 1 -- Pattern detection**: Scan for every trope in the catalog. For each
finding, record:

- Trope slug (the canonical identifier from the catalog)
- Severity (high / medium / low)
- Exact text containing the trope, with line number or paragraph reference
- Threshold status: for density-based tropes, count occurrences and compare to
  the threshold in the catalog

**Pass 2 -- Contextual assessment**: For each finding from Pass 1, evaluate
whether it is actually problematic given the project context from Step 2.
Assign one of three verdicts:

- `flag` -- genuinely problematic in this context. Should be rewritten.
- `note` -- marginal. Worth knowing about but tolerable here. The surrounding
  text is clean, or the project's style may permit it.
- `pass` -- acceptable. The trope is used effectively, the word carries its
  literal meaning (not a trope), or the project context explicitly permits it.

When assessing context, consider:

- Technical terms used literally ("robust to failure") are not tropes
- Published blog posts have lower tolerance than internal docs
- A single low-severity trope in 2000 words of clean prose is usually `pass`
- Trope clusters (3+ different tropes in one paragraph) always flag regardless of
  individual severity

### Step 6 -- Generate remediation (full mode only)

For each finding marked `flag`:

- Rewrite the surrounding sentence or paragraph to eliminate the trope
- Preserve the original meaning
- Vary sentence rhythm -- do not replace one AI pattern with another
- Use concrete, specific language instead of abstract generalities
- The rewrite should sound like a human wrote it: imperfect, direct, grounded

For each finding marked `note`:

- Suggest an alternative phrasing, marked as optional
- Explain why it might be worth changing even though it is not a hard flag

### Step 7 -- Produce the report

Output a structured report. For quick mode, omit the Remediation Plan section
and per-finding rewrites.

```markdown
# Detrope Report: [target]

**Date**: YYYY-MM-DD
**Mode**: full | quick
**Project**: [from CLAUDE.md]
**Document type**: [inferred]
**Files analyzed**: N

## Summary

| Severity | Flags | Notes | Passed |
| -------- | ----: | ----: | -----: |
| High     |     N |     N |      N |
| Medium   |     N |     N |      N |
| Low      |     N |     N |      N |

**Trope density**: N per 1000 words (clean: <2, moderate: 2-5, heavy: >5)
**AI signal strength**: low | moderate | strong | unmistakable
**Top tropes**: [top 3 slugs by frequency]

## Findings by File

### [filename]

#### Flags (action recommended)

**1. `negative-parallelism`** (high)

- Line NN: > "original text containing the trope"
- Verdict: flag -- [one sentence: why this is problematic here]
- Rewrite: > "improved text without the trope"

#### Notes (optional improvements)

**1. `magic-adverbs`** (low, 4 occurrences)

- Verdict: note -- density is borderline, review individually
- Locations: lines NN, NN, NN, NN
- Alternative: [optional suggestion]

#### Suppressed

- `em-dash-addiction`: project style permits em dashes (per CLAUDE.md)

## Remediation Plan (full mode only)

Priority-ordered list of changes, grouped by severity:

### High Priority

1. Line NN: [original] -> [rewrite] (`slug`)
2. Line NN: [original] -> [rewrite] (`slug`)

### Medium Priority

1. Line NN: [original] -> [rewrite] (`slug`)

### Low Priority (optional)

1. Line NN: [original] -> [rewrite] (`slug`)

## Stylometric Profile

- **AI signal strength**: [low | moderate | strong | unmistakable]
- **Dominant tells**: [top 3 trope slugs with counts]
- **Densest section**: [which heading/section has highest trope concentration]
- **Assessment**: [1-2 sentence verdict on overall AI-ness of the document]
```

If the user requested a saved report, write it to:
`intent/detrope/YYYYMMDD-<slug>.md`

## Important Notes

- This skill reads files but never modifies them directly. All changes are
  proposals for the user to review and apply.
- Context assessment depends on CLAUDE.md being current. If the project lacks
  a CLAUDE.md, ask the user about the target audience and document purpose.
- The trope catalog covers English-language prose only.
- Density thresholds are calibrated for documents of 500+ words. Very short
  files may produce skewed density numbers -- note this in the report.
- For a fast automated pre-scan of the mechanically-detectable subset (~28 of
  44 tropes), use `cleanz --detrope` from [Utilz](https://github.com/matthewsinclair/Utilz)
  before running the full skill analysis.
- Trope slugs are stable identifiers from the
  [llm-tropes](https://github.com/matthewsinclair/llm-tropes) catalog.
  Use them consistently in reports and discussions.
