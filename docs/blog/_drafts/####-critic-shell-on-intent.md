---
title: "Critic-shell on Intent's own bash"
date: "2026-04-28"
author: "Matthew Sinclair"
draft: true
word_count: 1800
---

> **Status.** Draft. The first subagent pass is captured below; continuous validation since v2.10.0 runs through the pre-commit critic gate. Move out of `_drafts/` when ready to publish.

# Critic-shell on Intent's own bash

## Setup

Intent v2.9.0 added a Critic subagent family — one agent per supported language (Elixir, Rust, Swift, Lua, shell). Each agent loads the rule library at `intent/plugins/claude/rules/` and applies each rule's Detection section against a target file. The output is a severity-grouped report. The agent never modifies code and never calls external tools.

`critic-shell` shipped as WP12 of ST0034. Intent itself is about 15k lines of bash. Most of it lives in the `bin/intent*` dispatchers and the plugin helpers under `intent/plugins/claude/lib/`; the rest is the project-template scaffolding under `lib/templates/`. The rule pack was drafted while reading those files; once it was in place we ran it back over them.

## One pack, dialect-tagged

Bash and zsh share most of what a shell rule pack has to say: quoting, command-substitution syntax, filename handling, exit-code discipline. The differences live in the defaults — different word-splitting behaviour, different strict-mode incantations (`set -e` for bash, `setopt err_exit` for zsh), and array indexing that runs from 0 in one and 1 in the other.

Two packs (`rules/bash/` and `rules/zsh/` with mostly duplicated content) was the obvious option, but lost on Highlander grounds: the duplicated-rule copies will drift independently. The pack we shipped is one `rules/shell/` directory with dialect tags on the rules that differ. `critic-shell` reads the shebang on each target at invocation time and skips dialect-tagged rules that don't apply.

## Rules from the corpus

The v2.9.0 shell rules were drafted by reading `bin/intent*` and asking which patterns the codebase already followed that a new contributor might miss. Most rule packs come the other way around: somebody picks an external ideal and the codebase has to catch up. We inverted that on the theory that Intent's shell had survived several years of use across sixteen fleet projects, which is some evidence of a defensible style.

The risk is that any latent flaws in Intent's bash got encoded into the rule library along with the strengths. Iteration 1 was where those would surface.

The `## Good` examples in the pack reference real code. The no-parse-ls rule's good case is the iteration pattern from `intent_upgrade`. `IN-SH-CODE-003` recommends `set -euo pipefail` in the general case, and then documents the bash 3.x macOS quirk that pushed Intent to `set -e` alone in its `## When This Does Not Apply` section — so the rule and the reference implementation say the same thing on the page where they would otherwise disagree by omission.

## What the subagent does

`critic-shell`'s `agent.md` is around 120 lines. The agent loads the agnostic and shell rule packs (plus any user-extension rules under `~/.intent/ext/*/rules/shell/`) and walks the target list. For each target, the dialect tag on each rule is matched against the shebang. Detection runs on the matched rules; findings accumulate under the rule ID. The report carries rule ID, file:line, snippet, and a one-line suggested fix, in a format stable enough that two runs over the same tree produce diffable output.

## Iteration 1: the first subagent run

Commit `b020fbe`. Twenty-seven targets — the four `bin/intent*` dispatchers, `intent_agents`, the four `intent_claude_*` plugin dispatchers, and `tests/run_tests.sh`. No `.intent_critic.yml` at the root, so defaults applied. All targets bash; the zsh-only rule (`IN-SH-CODE-004`) had nothing to act on.

The counts came back **7 CRITICAL, 16 WARNING, 17 RECOMMENDATION, 14 STYLE**.

Entry 0 of the dogfood journal predicted "near-zero critical findings", on the theory that the rules came from this same codebase. The seven CRITICALs say otherwise. Aligning a rule pack to the broad shape of a codebase during authorship is a different operation from running each rule's Detection over each line, and the second one isn't a side-effect of the first.

### Triage

| Tier | Severity       | Action                                                                                         | Findings                                                                                                                                                                                                                          |
| ---- | -------------- | ---------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P0   | CRITICAL       | Fix this session                                                                               | Unquoted `$(...)` returns in `intent_st` (paths-with-spaces bug, two sites); `for x in $(find ...)` swallowing `find`'s exit code in two more `intent_st` sites; unquoted `$lib_dirs` expansions in `intent_audit` (three sites). |
| P1   | WARNING        | Fix this session                                                                               | `ls \| wc -l` in `intent_doctor`; `find \| xargs` without `-print0` in `tests/run_tests.sh`; a silent shadow of the canonical `get_terminal_width` helper in `intent_plugin`.                                                     |
| P2a  | RECOMMENDATION | Partial: shipped for `tests/run_tests.sh`; rolled back for six production scripts              | Missing `set -e` in seven dispatcher/helper files. Six BATS tests went red after the rollout; see below.                                                                                                                          |
| P2b  | RECOMMENDATION | Shipped (Highlander pass): canonical `warning()` and new `info()` lifted into `intent_helpers` | Pure-shadow duplicates of `warning()` / `info()` across `intent_bootstrap`, `intent_doctor`, `intent_config`, and `tests/run_tests.sh`.                                                                                           |
| P3   | STYLE          | Deferred                                                                                       | Fourteen `[ $n -gt $m ]` bare-numeric arithmetic comparisons. Project convention has historically allowed them; revisit via `.intent_critic.yml` rather than a mechanical sweep.                                                  |

### The P2a rollback

The plan for P2a was to add `set -e` to all seven dispatcher and helper files. After the commit, six BATS tests went red. The pattern was the same in each one: scripts use `[ -d "$path" ] || error "not found"` to surface a missing-input error, and under `set -e` the `[ -d ... ]` returning 1 is itself enough to abort. So the `||` branch never fires and the error message never prints; the test that asserted the error then gets a clean exit instead of the failure it was looking for.

`tests/run_tests.sh` kept `set -e` because its only non-zero-tolerant call was `bats`, which got rewritten to `bats ... || EXIT_STATUS=$?`. The other six scripts now carry an inline comment marking the deferral. The next revision of `IN-SH-CODE-003` will need to spell out the absence-check idiom (`if ! [ -d ... ]; then error ...; fi`) in its `## Bad` and `## Good` examples; the current form of the rule doesn't make the interaction visible, and an author writing a new dispatcher will keep falling into it. BATS stayed 707/707 at every commit boundary.

### Highlander pass

Four files carried local copies of `warning()` and `info()` that had drifted independently. `intent_bootstrap`'s `warning` printed to stdout where `intent_config`'s went to stderr. `intent_doctor` honoured `QUIET=1` and the others didn't. ANSI colour was inconsistent. Lifting the canonical pair into `intent_helpers` and stripping the duplicates collapsed the divergence to one definition.

`intent_doctor`'s `show_error` and `show_warning` stayed where they were. They bump the `ERRORS` and `WARNINGS` counters in addition to printing — that's a different concern from emitting a message, and the names already reflected the difference. The Highlander pass came out at three duplicates removed and one named-shadow kept.

In the same commit, postfix `((var++))` got converted to prefix `((++var))` in `intent_doctor`, `intent_agents`, and `intent_claude_subagents`. Under `set -e`, postfix increment evaluates to zero on the first call — which is non-zero-exit territory — and the loop aborts on what looks like a routine counter bump. The conversion was free and removed a foreseeable obstacle to the next strict-mode attempt.

## After Iteration 1

Entries 2 and 3 of the dogfood journal — "widened corpus" and "three consecutive clean runs" — never landed as discrete subagent passes. v2.10.0 introduced a pre-commit critic gate that runs the same rule pack against staged shell files on every commit (`bin/intent_critic` invoked from `.git/hooks/pre-commit`). Periodic dogfood passes turned into continuous validation. Every shell change since v2.10.0 has gone through the gate, and the rule pack hasn't drifted from the codebase since.

By v2.10.1 the pre-commit gate had become the default form for shell-rule validation. The new `scripts/release` orchestrator that v2.10.1 introduced ran through the gate alongside every other shell-touching diff during its development. Continuous validation has turned out to need much less ceremony than scheduled audits would have; the rule library exercises itself at commit time, on a diff small enough to keep findings actionable.

## Reflections

Authoring rules from a corpus only does part of the work. The broad style alignment held during Iteration 1 — none of the seven CRITICALs prompted an argument about whether the rule was right. They were instances the rule should have caught and didn't, until the subagent pointed at the line. The implication for the next language pack is that skipping the dogfood pass on the strength of corpus-derived rules will catch up with the author sooner or later.

The most prolific pattern in the findings was `for x in $(find ...)`. Two CRITICAL hits in `intent_st` alone, with softer echoes elsewhere. The conversion idiom (`while IFS= read -r -d '' x; do ... done < <(find ... -print0)`) is awkward enough that the broken form keeps drifting back into new code, since at a glance it looks fine. Shortly after Iteration 1 the rule's textual examples got promoted to a runnable `bad.sh` that mishandles a filename containing spaces and a `good.sh` that doesn't. Once you can run the bad case and see it misbehave, the trap stops feeling subtle.

The triage tiers held up during the cleanup. P0 and P1 were unambiguous. P2a bit because of a script-level interaction between absence-check idioms and `set -e` that lived between rules — no individual rule had the full picture, and the combined hazard surfaced only when somebody tried to apply the recommendation to a real script. P2b was the case where a rule pointed at a code-organisation problem; the appropriate response was a refactor (per-site patching would have missed the point). P3 was a project-style decision that belongs in `.intent_critic.yml`, since the rule itself shouldn't have a position on a project's bracket-quoting convention. One RECOMMENDATION-tier finding (the `plugin_*` callback clones across `intent_claude_skills` and `intent_claude_subagents`) the critic itself flagged with a "defer to author judgement" annotation — the right behaviour when two implementations are distinct on purpose, pending a third use site.

Time to first finding was about thirteen minutes — one subagent invocation reading 27 files of bash, around 100k tokens in. An external linter would beat that on some rules and lose on others, but the wall-clock it claws back tends to get spent on reconciling its output against project-specific carve-outs. The subagent has a narrower job — read the pack and walk the targets — and that accounts for most of the speed.

Each P0/P1 fix in the session also touched the rule that had caught it; usually a clarifying line in `## Bad`, occasionally a refinement to the Detection section. Without that loop, rule packs decay against the codebases they're meant to govern, and the drift stays invisible until somebody runs the rules against the code again. From v2.10.0 onwards the pre-commit gate carries that loop: every shell-touching commit runs the pack.

---

_Draft by matts; original sketch 2026-04-22, expanded with Iteration 1 findings 2026-04-28._
