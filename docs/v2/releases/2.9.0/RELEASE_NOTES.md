# Intent v2.9.0 Release Notes

**Release Date**: 2026-04-23

## Overview

Intent v2.9.0 ships the Agentic Software Engineering Suite (ST0034). Three things change at once and they were designed to land together:

- **Rules become first-class citizens.** Coding standards live in `intent/plugins/claude/rules/<lang>/<category>/<slug>/RULE.md` as atomic, cite-able files with stable `IN-*` IDs, structured frontmatter, Detection heuristics, and bad/good examples.
- **Critic subagents enforce rules mechanically.** A new `critic-<lang>` family (`critic-elixir`, `critic-rust`, `critic-swift`, `critic-lua`, `critic-shell`) reads the rule library, applies each rule's Detection heuristic to target files, and emits a stable severity-grouped report.
- **A user extension system at `~/.intent/ext/`** lets you ship your own subagents, skills, or rule packs without forking Intent. Discovery is layered: canon is the default; user extensions override by name with a visible shadow warning. The reference extension is `worker-bee`, relocated from canon to demonstrate the mechanism end-to-end.

These three pieces unlock the same workflow loop: a rule file says what good looks like, a critic enforces it on real code, and an extension lets you customise either without touching upstream.

## Rules library

A rule is one atomic standard. It has a stable ID (`IN-EX-CODE-006`, `IN-AG-HIGHLANDER-001`, etc.), a Detection heuristic, bad/good examples, and required Markdown sections. Skills cite rules by ID; the rule file owns the prose.

The library ships with packs for `agnostic`, `elixir`, `rust`, `swift`, `lua`, and `shell`. The schema is intentionally compatible with [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic), so upstream rules drop into Intent's discovery unchanged.

The `intent claude rules` command surface — `list`, `show`, `validate`, `index` — operates against the library. `validate` is the canonical authoring gate; `index` regenerates a deterministic, sorted `index.json`.

Authoring guide: `intent/docs/rules.md`. Schema reference: `intent/plugins/claude/rules/_schema/rule-schema.md`.

## Critic subagents

Critics are thin orchestrators. On invocation, a critic re-reads the rule library (no caching), applies each rule's Detection heuristic to the target source files, and emits a parse-stable severity-grouped report:

```
## Critic Report: critic-elixir code lib/myapp/accounts.ex

CRITICAL
- IN-EX-CODE-005 (no-silent-failures) lib/myapp/accounts.ex:42
  rescue _ -> :ok swallows the lookup failure silently.
  Surface the error tuple or let it raise.

WARNING
- IN-EX-CODE-002 (tagged-tuple-returns) lib/myapp/accounts.ex:18
  Function returns bare nil on not-found.
  Return {:ok, value} | {:error, reason} so callers can pattern-match.

Summary: 1 critical, 1 warning, 0 recommendation, 0 style.
Rules applied: 4 agnostic, 12 language-specific.
```

Modes are `code` and `test` (`critic-shell` is `code` only). Each rule's `applies_to` glob narrows the file set further. Per-project config lives in `.intent_critic.yml` at the project root for disabling rules and adjusting severity thresholds.

Critics never autofix, never shell out to external linters, never invent rule IDs. Every finding cites a real `RULE.md` you can open directly.

The full contract — invocation, modes, ambiguity handling, report format, `.intent_critic.yml` schema, Diogenes/Socrates handoffs, and the registration-freeze operational note — lives at `intent/docs/critics.md`.

## User extensions (`~/.intent/ext/`)

Extensions are content-only directories that contribute subagents, skills, or rule packs. Each extension has an `extension.json` manifest declaring its contributions and Intent compatibility bounds. Discovery is layered: canon stays the default; user extensions override by name; extension rules override canon rules with the same ID. Every shadow is logged — no silent overrides.

Author your own:

```bash
intent ext new my-agent --subagent
intent ext new my-skill --skill
intent ext new my-rules --rule-pack
intent ext validate my-agent
```

Authoring guide: `intent/docs/writing-extensions.md`. Manifest schema: `intent/plugins/claude/ext-schema/extension.schema.json`.

The reference extension is `worker-bee`. In v2.8.x it was a canon subagent; in v2.9.0 it is the worked example for the extension mechanism. The migration seeds it from `lib/templates/ext-seeds/worker-bee/` to `~/.intent/ext/worker-bee/` on first run. If worker-bee can live as an extension end-to-end, any user-authored subagent can.

## Breaking changes

- **`elixir` subagent deleted.** Use `critic-elixir` instead. The migration prunes installed copies on upgrade.
- **`worker-bee` moved out of canon.** Re-install from the extension after upgrade: `intent claude subagents install worker-bee`.

Both are aggressive, fail-forward changes — there are no compatibility shims and no deprecation period inside the migration. The migration prunes the installed copies of both and seeds the new worker-bee extension. Run the upgrade, restart your session, re-install worker-bee from the extension if you want it.

## Upgrade

```bash
intent upgrade --apply
```

The `migrate_v2_8_2_to_v2_9_0` step (in `bin/intent_helpers`) does four things:

1. Stamps `.intent/config.json` with `intent_version: 2.9.0`.
2. Bootstraps `~/.intent/ext/` with a README on first run.
3. Seeds `~/.intent/ext/worker-bee/` from `lib/templates/ext-seeds/worker-bee/` (skipped if already present — never overwrites user state).
4. Prunes installed copies of the deleted `elixir` subagent and the relocated `worker-bee` from `~/.claude/agents/` and `~/.intent/agents/installed-agents.json`.

The migration is idempotent — running the upgrade twice is safe.

To verify post-upgrade state:

```bash
intent doctor                                   # general health check
cat .intent/config.json | jq .intent_version    # should print "2.9.0"
intent ext list                                 # should show worker-bee
intent claude subagents list | grep critic-     # should show 5 critic-* entries
```

**Restart Claude Code after the upgrade.** Mid-session subagent installs are not visible to `Task()` until the next session starts. If you try to invoke `critic-elixir` (or any other critic) without restarting, you'll get "subagent not found". This is a Claude Code constraint, not an Intent behaviour.

## Acknowledgements

The rule schema, the borrowed rule principles, and the `_attribution/elixir-test-critic.md` MIT notice all credit [`iautom8things/elixir-test-critic`](https://github.com/iautom8things/elixir-test-critic) (MIT, copyright 2026 Manuel Zubieta), pinned at commit `1d9aa40700dab7370b4abd338ce11b922e914b14`. The compatibility was deliberate: upstream rules drop into Intent's discovery unchanged, and Intent's `critic-elixir` recognises upstream's `elixir-test-critic` plugin if it is installed at `~/.claude/plugins/elixir-test-critic/`.

## Migration notes for fleet projects

- The migration is gated on `needs_v2_9_0_upgrade` (returns true for `< 2.9.0`, recognises `2.9.x`, `2.10.x`, and `3.x` as already-migrated).
- The migration chain in `bin/intent_upgrade` covers every prior starting version (16 chain-tails); a project on any v2.x release upgrades cleanly through to v2.9.0.
- Worker-bee's `intent_compat.min` in the seed manifest is `2.8.2` — projects on the new minimum or higher load the extension successfully.
