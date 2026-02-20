# Design - ST0021: Intent Autopsy

## Architecture

Three components work together:

1. **Elixir script** (`autopsy.exs`) -- pre-processes JSONL session files. Extracts messages,
   compaction boundaries, tool uses. Runs pattern matching for corrections, frustration signals,
   deferrals, banned patterns. Outputs structured JSON findings.

2. **Skill** (`intent-autopsy/SKILL.md`) -- user-invocable via `/intent-autopsy`. Contains the
   analysis methodology, categories, output format, and instructions for running the helper
   script and interpreting results.

3. **Claude analysis** -- reads script output + MEMORY.md + CLAUDE.md. Compares findings against
   stated rules. Identifies memory gaps, enforcement failures, undocumented conventions, stale
   memory. Proposes concrete memory file updates.

### Data flow

```
~/.claude/projects/<key>/*.jsonl
        |
        v
  autopsy.exs (Elixir)
  - parse JSONL lines
  - extract messages, compactions, tool uses
  - pattern match: corrections, frustration, deferrals, banned words
  - output: findings.json
        |
        v
  Claude (guided by SKILL.md)
  - read findings.json
  - read MEMORY.md, CLAUDE.md
  - compare findings against rules
  - identify: gaps, enforcement failures, conventions, stale memory
  - output: intent/autopsy/YYYYMMDD.md
```

## Analysis Categories

### Script categories (handled by autopsy.exs)

1. **Census** -- session count, compaction count, message counts, data volume
2. **Correction pairs** -- user corrected Claude, tracked pre/post-compaction
3. **Frustration signals** -- "how many times", "I told you", "did you actually look"
4. **Capability regression** -- Claude deferred when it could have acted
5. **Banned pattern violations** -- AI-isms in conversation AND tool_use inputs

### Memory-aware categories (Claude analysis, not script)

6. **Memory gaps** -- corrections with no matching rule in MEMORY.md/CLAUDE.md
7. **Enforcement failures** -- rules that exist but were violated
8. **Undocumented conventions** -- consistent patterns not captured in memory
9. **Stale memory** -- rules never triggered in analyzed sessions

## Design Decisions

### Full directory install

Extended `intent claude skills install` to copy entire skill directory (not just SKILL.md).
Scripts at source get installed alongside SKILL.md. Future-proofs for other skills with
supporting files. Sync still checksums SKILL.md only. Uninstall already uses `rm -rf`.

### Elixir script (not Python)

The reference implementation uses Python. We use Elixir because: (a) Intent already has Elixir
subagent and skills, (b) pattern matching is ideal for JSONL parsing, (c) standalone .exs with
Mix.install needs no project setup.

### Banned words file

Ships a default `banned-words.txt` with common AI-isms. Labels: ai_ism, hedging, deferral,
identity_break. Users can extend or replace.

### Report persistence

Reports saved to `intent/autopsy/YYYYMMDD.md` for historical reference. Directory created on
first run.

## Alternatives Considered

### Python script (rejected)

Bill's reference uses Python. Elixir is a better fit for this project's ecosystem.

### Skill-only approach (rejected)

Having Claude parse raw JSONL directly would consume too much context. Sessions can be hundreds
of MB. The Elixir script solves the volume problem by extracting only relevant findings.

### Separate CLI command (rejected)

Could have been `intent autopsy` instead of a skill. But the analysis requires Claude's
reasoning for memory comparison, so a skill (which guides Claude's behavior) is the right
abstraction.
