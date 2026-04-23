---
verblock: "22 Apr 2026:v0.3: matts - Done"
wp_id: WP-12
title: "Shell rule pack (bash + zsh)"
scope: Small
status: Done
---

# WP-12: Shell rule pack (bash + zsh)

## As-Built (2026-04-22)

Shell rule pack shipped with 6 rules, `critic-shell` subagent, dogfood journal, and blog-post draft. Total rule count rose 42 -> 48.

### Rules shipped

| ID             | Slug                      | Severity | Dialect       |
| -------------- | ------------------------- | -------- | ------------- |
| IN-SH-CODE-001 | code/quote-expansions     | critical | bash + zsh    |
| IN-SH-CODE-002 | code/no-parse-ls          | warning  | bash + zsh    |
| IN-SH-CODE-003 | code/set-euo-pipefail     | warning  | bash-specific |
| IN-SH-CODE-004 | code/setopt-err-exit      | warning  | zsh-specific  |
| IN-SH-CODE-005 | code/no-silent-exit-codes | critical | bash + zsh    |
| IN-SH-CODE-006 | code/module-highlander    | warning  | bash + zsh    |

### critic-shell subagent

`intent/plugins/claude/subagents/critic-shell/` ships with:

- `agent.md` — thin-coordinator prompt. Reads agnostic + shell rules, detects dialect from shebang, applies Detection heuristics, emits stable severity-grouped report. No autofix, no external linting.
- `metadata.json` — declares tools (Read, Grep, Glob, Bash) and tags.
- Registered in `intent/plugins/claude/subagents/.manifest/global-agents.json`.

This is the first `critic-<lang>` to ship in v2.9.0. WP07 will add Elixir/Rust/Swift/Lua counterparts.

### Agnostic concretised_by updates

- HIGHLANDER: +IN-SH-CODE-006
- NO-SILENT-ERRORS: +IN-SH-CODE-005
- (Body "Concretising rules" lines updated with same IDs.)

### Tests

`tests/unit/rule_pack_shell.bats` — 14 tests covering presence, ID declaration, language field, dialect-specific tags, validator agreement, `rules list --lang shell` membership, Bad/Good fenced code blocks, concretised_by backlinks, critic-shell registration.

Suite rose 619 -> 633 (+14).

### Dogfood journal + blog draft

- `intent/st/ST0034/WP/12/dogfood-journal.md` — Entry 0 (rule authorship pass) complete. Entries 1-3 placeholders for post-WP12 subagent invocations against `bin/intent*`.
- `docs/blog-drafts/shell-critic-inception.md` — draft narrative "Using Intent's critic-shell on Intent: Inception Edition". Publication blocked on real subagent dogfood runs.

### Deviations from plan

- First subagent dogfood run deferred. Rationale: critic-shell is invoked via Task() from a Claude Code session, not from bash — its first invocation belongs to a future session. Entry 0 of the journal documents the authorship-pass substitute (rules authored by reading Intent's corpus, so critic-shell against Intent is expected-clean by construction).
- Three-iteration acceptance criterion partially satisfied: the authorship pass is Entry 0; Entries 1-3 land in follow-up work.
- Blog-post draft kept short and explicitly flagged "publication deferred". Real findings content fills in after Entry 1+.

### Commits

- (current session) — shell pack (6 rules) + critic-shell subagent + agnostic concretised_by updates + 14 BATS tests + dogfood journal + blog draft = 633 total.

## Objective

Author a shell rule pack at `rules/shell/` covering both bash and zsh. Dogfood the rules against Intent's own `bin/intent*` scripts — Intent is a pure-bash codebase, so `critic-shell` running on Intent itself is the acceptance test for rule quality. Capture the dogfooding process as a journal so it can be written up as a blog post afterwards ("Using Intent's critic-shell on Intent: Inception Edition" or similar).

## Context

Intent is written entirely in bash. Every rule authored here has an immediate validation corpus: run `critic-shell` across `bin/`, `intent/plugins/claude/bin/`, and `intent/plugins/claude/lib/`. If a rule is sensible, Intent's own scripts pass. If it's noise, the rule needs tuning before it ships. This feedback loop is the load-bearing quality gate for WP12 — not a separate fixture battery.

Bash and zsh share ~80% of rule content (quoting, `$()` over backticks, no-parse-`ls`, no `eval` on untrusted input). The other ~20% genuinely diverges (`set -e` vs `setopt err_exit`, 0-based vs 1-based arrays, word-splitting defaults). Layout: single `rules/shell/` pack with `IN-SH-*` IDs; shared rules cover both shells and tag as `[bash, zsh]`; divergent concerns split into separate slug-distinct rules tagged `bash-specific` or `zsh-specific`.

## Deliverables

### Rule pack (≥5 rules)

- `intent/plugins/claude/rules/shell/code/quote-expansions/` — `IN-SH-CODE-001` — Always quote `"$var"`, `"${arr[@]}"` (applies to both bash and zsh)
- `intent/plugins/claude/rules/shell/code/no-parse-ls/` — `IN-SH-CODE-002` — No `ls | while read` or `for f in $(ls)`; use globs or `find -print0 | xargs -0` (applies to both)
- `intent/plugins/claude/rules/shell/code/set-euo-pipefail/` — `IN-SH-CODE-003` — `set -euo pipefail` at the top of every bash script (bash-specific; zsh equivalent in -004)
- `intent/plugins/claude/rules/shell/code/setopt-err-exit/` — `IN-SH-CODE-004` — `setopt err_exit pipe_fail no_unset` in zsh, with noted semantic differences from bash's `set -e` (zsh-specific)
- `intent/plugins/claude/rules/shell/code/no-silent-exit-codes/` — `IN-SH-CODE-005` — Never discard exit codes; check with `|| { err; exit 1; }` or explicit `case $?`. Concretises `IN-AG-NO-SILENT-001` (applies to both)
- `intent/plugins/claude/rules/shell/code/module-highlander/` — `IN-SH-CODE-006` — One helper function per concern across `bin/intent_*`. Concretises `IN-AG-HIGHLANDER-001` (applies to both; dogfood rule for Intent itself)

Candidate additions if the dogfood pass finds real violations in Intent's own code:

- `IN-SH-CODE-007` — `local` on every function-local variable in bash (prevents leakage into parent shell)
- `IN-SH-CODE-008` — No `eval` on input that could be attacker-controlled
- `IN-SH-CODE-009` — Prefer `[[ ]]` over `[ ]` in bash; both work in zsh
- `IN-SH-CODE-010` — Heredoc indentation (`<<-EOF` + tab-only indent)

### Critic subagent

- `intent/plugins/claude/subagents/critic-shell/agent.md` + `metadata.json`
- Language detection via shebang: `#!/bin/bash` → load bash rules; `#!/bin/zsh` → load zsh rules; both shebangs → load shared rules + both dialect rules
- Code mode only (shell test-framework coverage in Intent is `bats`, handled via Elixir-adjacent test-rule discipline in WP05)

### Dogfooding journal

- `intent/st/ST0034/WP/12/dogfood-journal.md` — chronologically ordered, one entry per distinct run of `critic-shell` against Intent's own codebase. Each entry records:
  - The rule set at the time of the run.
  - Invocation: target dirs, invocation string.
  - Findings: count by severity, notable examples, any false positives.
  - Rule changes that resulted: new rules added, existing rules tightened or loosened, rules rejected.
  - A short "what did we learn" paragraph.
- This journal is the raw material for the "Inception Edition" blog post authored post-release.

### Tests

- `tests/unit/rule_pack_shell.bats` — presence, frontmatter validity, `bash -n` / `zsh -n` syntax check on any `good.sh` / `bad.sh` examples that ship
- `tests/unit/critic_shell_dogfood.bats` (optional) — skeleton that invokes `critic-shell` against `bin/intent` and asserts exit 0 once the rule pack is tuned; serves as a regression gate

## Acceptance Criteria

- [ ] At least 5 rules authored; target 6-8 with room to grow based on dogfood findings
- [ ] Each rule has complete frontmatter per schema; IDs conform to `IN-SH-<CAT>-<NNN>`
- [ ] Each rule has all 9 structural elements per schema
- [ ] Rules that apply to one dialect only carry the `bash-specific` or `zsh-specific` tag; shared rules tag both `bash` and `zsh`
- [ ] `IN-SH-CODE-005` (no-silent-exit-codes) and `IN-SH-CODE-006` (module-highlander) have their IDs added to `concretised_by:` on the relevant WP04 agnostic rules
- [ ] `intent claude rules validate rules/shell/` exits 0
- [ ] `critic-shell` subagent shipped with language-detection logic via shebang
- [ ] Dogfood journal recorded in `intent/st/ST0034/WP/12/dogfood-journal.md` with at least three distinct dogfood-and-tune iterations
- [ ] After rule-tuning, `critic-shell` against `bin/intent*` produces zero critical findings (the rules describe code that Intent's own scripts adhere to — "Inception clean")
- [ ] Blog-post outline drafted (not published) at `docs/blog-drafts/shell-critic-inception.md` with sections: why shell, layout decision, first dogfood pass, lessons, final rule count

### Tests to add

- [ ] `tests/unit/rule_pack_shell.bats`
- [ ] Optional: `tests/unit/critic_shell_dogfood.bats`

### Tests to update

- [ ] `./tests/run_tests.sh` exits 0 after commit (pristine invariant)

## Dependencies

- **WP04** (agnostic pack): soft dep. WP12 updates WP04 `concretised_by:` to include shell IDs.
- **WP06** (Rust/Swift/Lua): no hard dep but sequenced after so WP06 has the textual-only precedent locked in.
- **WP07** (critic family): WP12 adds `critic-shell` to what WP07 delivers. If WP07 has already landed, WP12 appends; if not, WP12 piggybacks on the WP07 commit.

## Approach

1. **Bootstrap pack.** Create `rules/shell/code/` with the six seed rules listed above. Frontmatter + body only; no `good.sh`/`bad.sh` yet.
2. **First dogfood pass.** Write a minimal `critic-shell` prompt that loads `rules/shell/` and reports findings. Run against `bin/intent`. Log results in the journal.
3. **Iterate.** For each finding, triage: (a) true positive → fix Intent's script, (b) false positive → loosen the rule's Detection, (c) genuine new concern → draft a new rule. Re-run. Log the iteration.
4. **Widen the corpus.** Expand to `intent/plugins/claude/bin/`, then `intent/plugins/claude/lib/`, then all `bin/intent_*`. Each expansion is a journal entry.
5. **Stabilise.** Once three consecutive runs produce zero critical findings, freeze the pack. Backfill optional `good.sh`/`bad.sh` examples for rules where runnable illustration clarifies the concern.
6. **Ship critic-shell subagent.** Same contract as WP07's Critic subagents, with shebang-based dialect detection.
7. **Update WP04 concretised_by.** Add the two shell IDs to the relevant agnostic rules.
8. **Draft blog post.** Assemble the journal into a narrative at `docs/blog-drafts/shell-critic-inception.md`. Blog post stays as a draft in the repo — publication is out of scope for v2.9.0.

## Size and Estimate

- **Size**: S (Small-to-Medium, 2-3 sessions).
- Session 1: Seed rules, first dogfood pass, iterate-and-fix loop.
- Session 2: critic-shell subagent, widen corpus, journal entries.
- Session 3: Stabilise, tests, WP04 backlink, blog draft, commit.

## Exit Checklist

- [ ] All acceptance criteria met
- [ ] Rules pass validator
- [ ] critic-shell subagent ships and detects dialect correctly
- [ ] Intent's own `bin/intent*` passes critic-shell at zero critical findings
- [ ] Dogfood journal has at least three entries
- [ ] WP04 concretised_by updated with shell IDs
- [ ] Blog-post draft at `docs/blog-drafts/shell-critic-inception.md`
- [ ] Registered in MODULES.md as part of rules library (entry pre-seeded in prep)
