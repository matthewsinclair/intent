# Using Intent's critic-shell on Intent: Inception Edition

**Status:** Draft. Publication deferred until post-v2.9.0 and after a real subagent dogfood pass has produced a findings diff against `bin/intent*`. The narrative below sketches the arc; real run outputs will land here once `critic-shell` has been invoked as a Task subagent against the repo.

## The setup

Intent v2.9.0 introduced a Critic subagent family: one agent per supported language (Elixir, Rust, Swift, Lua, shell), each reading the rule library at `intent/plugins/claude/rules/` and reporting violations in a target codebase. The agent is small by design -- its job is to apply each rule's Detection section to files, group by severity, and emit a stable report. No autofix, no external tool orchestration.

The shell variant (`critic-shell`) was added as WP12. Intent itself is ~15k lines of pure bash -- dispatchers in `bin/intent*`, plugin helpers in `intent/plugins/claude/lib/`, templates for new project scaffolding. That makes Intent both the author and the target of critic-shell. The "Inception" working title: we wrote the rules against the codebase, then we used the codebase to validate the rules.

## Layout decision: one pack, dialect-tagged

Bash and zsh share ~80% of rule content: quoting, command-substitution syntax, filename-handling, exit-code discipline. The other ~20% genuinely diverges -- `set -e` vs `setopt err_exit`, 0-based vs 1-based array indexing, word-splitting defaults.

Two packaging options were considered:

1. Two packs: `rules/bash/` and `rules/zsh/`, with roughly duplicated content.
2. One pack `rules/shell/`, with rules tagged `bash-specific` or `zsh-specific` where they differ.

Option 2 won on Highlander grounds: duplicating the quoting rule in two places would let them drift. Shared rules cover both shells; dialect-specific rules call it out in their `tags:` and `applies_when:` frontmatter. critic-shell detects the dialect from the shebang line at invocation time.

## Authorship: rules from the corpus, not a whiteboard

The honest confession: the v2.9.0 shell rules were authored by reading `bin/intent*` and asking "what does Intent already do correctly that a new contributor might get wrong?" rather than by drafting an ideal and forcing Intent to comply. That flips the usual "set ideal, measure codebase against ideal" shape: we started with a codebase that had survived years of use across 16 fleet projects, extracted the patterns that had held up, and wrote rules matching them.

Benefits:

- **Tight Detection heuristics.** The "no parse ls" rule's ## Good section isn't a textbook example; it's the pattern Intent's `intent_upgrade` actually uses. Readers see real code, not contrived.
- **Honest exceptions.** IN-SH-CODE-003 (strict mode) normally recommends `set -euo pipefail`. Intent uses `set -e` alone because of bash 3.x constraints on macOS. That exception appears verbatim in the rule's ## When This Does Not Apply section. No hypocrisy between rule and reference implementation.
- **First-run clean.** When critic-shell is eventually invoked against `bin/intent`, critical-tier findings will be scarce by construction. The rules were designed to match what's there.

Trade-off: if Intent's own shell style has flaws (it might), those flaws are now normalised in the rule library. This is mitigated by the next iteration's purpose.

## What the subagent actually does

critic-shell's agent.md ships a ~120-line prompt. The contract:

1. Read the rule library (agnostic + shell packs; also user-extension rules under `~/.intent/ext/*/rules/shell/`).
2. For each target file, detect dialect from the shebang.
3. Apply each rule's Detection section to the file. Report findings grouped by severity.
4. Never rewrite. Never call external linters. Read + report.

The report format is stable -- rule ID, file:line, snippet, one-line suggested fix -- so diffs between runs show rule pack drift or codebase drift unambiguously.

## Iterations (placeholder -- fill from journal)

Journal at `intent/st/ST0034/WP/12/dogfood-journal.md` captures each dogfood pass. This section will be filled with:

- Entry 1: first subagent-invoked run against `bin/intent`. Findings, rule tunes, fix count.
- Entry 2: widened corpus to `intent/plugins/claude/bin/` and shared libs. What drifted from Entry 1.
- Entry 3: three consecutive clean runs gate the release.

## Lessons (placeholder -- fill post-iteration)

Expected themes:

- Rule-authorship from a real corpus vs a whiteboard: trade-offs.
- Dialect handling via shebang detection: what works, what breaks.
- When rules disagree with existing code, fix the code vs fix the rule: a triage heuristic.
- Signal/noise: what severity tiers work in practice.

## Meta-lesson: the inception is the point

A critic that cannot catch bugs in its own authors' code is unlikely to catch bugs in anyone else's. Shipping critic-shell without running it against Intent would be shipping a rule-book nobody has tested against the reference implementation. The dogfood iteration -- whenever it lands post-release -- is how the rule pack earns its place on the shelf.

---

_Draft by matts, 2026-04-22. Publication blocked on a real critic-shell run._
