# Implementation - ST0034: Agentic Software Engineering Suite

## Progress Tracker

Status as of: 2026-04-22 (plan accepted, WP docs drafted, coding not yet started)

| WP   | Title                              | Status      | Notes                            |
| ---- | ---------------------------------- | ----------- | -------------------------------- |
| WP01 | Architecture and rule schema       | Not started | Foundation; blocks all other WPs |
| WP02 | Extension system foundation        | Not started | Depends WP01                     |
| WP03 | Skill and subagent rationalisation | Not started | Depends WP01                     |
| WP04 | Agnostic rule pack                 | Not started | Depends WP01                     |
| WP05 | Elixir rule pack                   | Not started | Depends WP01, WP04               |
| WP06 | Rust/Swift/Lua rule packs          | Not started | Depends WP01, WP04               |
| WP07 | Critic subagent family             | Not started | Depends WP03, WP04, WP05, WP06   |
| WP08 | Worker-bee extraction              | Not started | Depends WP02                     |
| WP09 | Migration and upgrade chain        | Not started | Depends WP02, WP08               |
| WP10 | Documentation                      | Not started | Depends WP02, WP07, WP08         |
| WP11 | Release and fleet upgrade          | Not started | Depends WP01-WP10                |

## Implementation Notes

This section accumulates implementation detail, gotchas, and decisions taken during the actual work. Populate as WPs progress.

### Architectural decisions deferred to implementation

- **`rules/index.json` generation schema**: finalise during WP01 after writing archetype rule. Need to see real frontmatter shape before locking the index schema.
- **`critic-<lang>` prompt length**: target under 300 lines per agent.md. Long prompts dilute focus. Final prompt length decided empirically in WP07.
- **`.intent_critic.yml` location**: project root vs. `.intent/` subdir. Default to project root for discoverability; consider fallback to `.intent/critic.yml` if project root is crowded.
- **Rust/Swift/Lua detection heuristics** (in-review stage-2): Cargo.toml, Package.swift are reliable; Lua needs entry-file or `.luarc.json` inspection. Document ambiguity cases in WP07.

## Code Examples

### Rule schema archetype (WP01 deliverable)

```yaml
---
id: IN-EX-TEST-001
upstream_id: test-critic-strong-assertions
slug: no-shape-tests
title: Strong assertions against concrete values
language: elixir
category: test
severity: critical
applies_to: ["test/**/*_test.exs"]
references:
  - IN-AG-HIGHLANDER-001
aliases: []
version: 1
---
```

### Callback signature (WP02 deliverable)

```bash
# plugin_get_source_roots  -- echo newline-separated root directories
#                             to search, in precedence order (highest first).
#                             If undefined, defaults to a single root
#                             (the canon subagents/skills dir).
plugin_get_source_roots() {
  [ -n "${INTENT_EXT_DIR:-}" ] && echo "$INTENT_EXT_DIR"
  if [ -d "$HOME/.intent/ext" ]; then
    for e in "$HOME/.intent/ext"/*/subagents; do
      [ -d "$e" ] && echo "$e"
    done
  fi
  echo "$INTENT_HOME/intent/plugins/claude/subagents"
}
```

### Migration function skeleton (WP09 deliverable)

```bash
migrate_v2_8_2_to_v2_9_0() {
  # 1. Version stamp
  jq '.intent_version = "2.9.0"' .intent/config.json > tmp && mv tmp .intent/config.json

  # 2. Bootstrap user ext dir
  if [ ! -d "$HOME/.intent/ext" ]; then
    mkdir -p "$HOME/.intent/ext"
    generate_ext_readme "$HOME/.intent/ext/README.md"
  fi

  # 3. Seed worker-bee (idempotent)
  if [ ! -d "$HOME/.intent/ext/worker-bee" ] && [ -d "$INTENT_HOME/lib/templates/ext-seeds/worker-bee" ]; then
    cp -r "$INTENT_HOME/lib/templates/ext-seeds/worker-bee" "$HOME/.intent/ext/worker-bee"
  fi

  # 4-5. Prune installed elixir + worker-bee (fail-forward)
  for agent in elixir worker-bee; do
    if [ -f "$HOME/.claude/agents/${agent}.md" ]; then
      rm "$HOME/.claude/agents/${agent}.md"
      if [ -f "$HOME/.intent/agents/installed-agents.json" ]; then
        jq "(.installed) |= map(select(.name != \"${agent}\"))" \
          "$HOME/.intent/agents/installed-agents.json" > tmp \
          && mv tmp "$HOME/.intent/agents/installed-agents.json"
      fi
    fi
  done
}
```

## Technical Details

### Bash 3.x compatibility constraints

- No `declare -A` (associative arrays).
- No `${VAR^}` (case modification).
- No `readarray` / `mapfile`.
- No process substitution in function bodies that need POSIX-safe execution.
- Use newline-separated output + `for x in $(cmd); do` for iteration.

### Highlander enforcement during this ST

- Before creating a new module: register in `intent/llm/MODULES.md` first.
- Before creating a new rule: check for duplication with agnostic + other language packs via `grep -F`.
- Before creating a new skill: check that content doesn't duplicate rule prose.

### Fail-forward discipline

- No deprecation stubs when renaming or removing anything.
- Migration actively deletes installed copies of removed subagents.
- Risk register does not include "breaking N fleet installations" as a blocking concern.

### MIT attribution requirements

- `intent/plugins/claude/rules/_attribution/elixir-test-critic.md` must contain:
  - Full MIT license text.
  - Copyright line: "Copyright 2026 Manuel Zubieta".
  - Source URL: `https://github.com/iautom8things/elixir-test-critic`.
  - Pinned commit hash (captured at WP05 start).
  - List of Intent rules that reference upstream via `upstream_id:`.

## Challenges & Solutions

Populate as implementation proceeds. Template:

### Challenge: [description]

**Context**: What WP surfaced this.
**Solution**: What was done.
**Rationale**: Why this over alternatives.
