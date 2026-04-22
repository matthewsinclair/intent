# Shell Critic Dogfood Journal (WP12)

Chronologically ordered record of `critic-shell` runs against Intent's own `bin/intent*` and `intent/plugins/claude/**/*.sh` codebase. Each entry captures: the rule set at the time, the invocation, the findings, what changed, what we learned.

## Entry 0 -- 2026-04-22 -- Rule authorship pass

**Premise.** The shell rule pack was authored by reading Intent's existing bin/ dispatchers and shared libraries. Rather than conduct a subagent-driven first pass and then discover that 30 real findings in `bin/intent` would each demand a rule retune, the rules were authored in alignment with how Intent's shell code is actually written. This is deliberate: Intent's shell style was validated over years, 50+ releases, 16 fleet projects. It is an existence proof of a defensible shell discipline. Using it as the reference corpus means the first critic-shell run against Intent is expected to produce zero critical findings.

**Rules shipped in this iteration (6):**

- `IN-SH-CODE-001` quote-expansions (critical)
- `IN-SH-CODE-002` no-parse-ls (warning)
- `IN-SH-CODE-003` set-euo-pipefail (warning; Intent uses `set -e` alone and documents why)
- `IN-SH-CODE-004` setopt-err-exit (warning; zsh-specific)
- `IN-SH-CODE-005` no-silent-exit-codes (critical)
- `IN-SH-CODE-006` module-highlander (warning; Intent's `bin/intent_helpers` + `lib/claude_plugin_helpers.sh` are the canonical reference)

**Notable rule-authorship decisions:**

- `IN-SH-CODE-003` acknowledges Intent's own `set -e` (no `-u`, no `-o pipefail`) as a documented exception. Reason: bash 3.x on macOS has quirks with `-o pipefail` that Intent has explicitly rejected. The rule's `## When This Does Not Apply` section cites Intent's constraint.
- `IN-SH-CODE-006` names `bin/intent_helpers` and `claude_plugin_helpers.sh` as the reference patterns. This makes the Highlander Rule concrete rather than aspirational.
- No `bash-strict-mode` / `zsh-strict-mode` merge. Keeping IN-SH-CODE-003 and IN-SH-CODE-004 separate because the semantic differences (trap interactions, subshell behaviour) are real enough to warrant distinct rules.

**Lessons:** Authoring rules against a real codebase (rather than a whiteboard set of ideals) produced tighter Detection heuristics. Writing "`ls | while read` is always wrong" is easy; writing "here is Intent's real iteration pattern using `find -print0 | while IFS= read -r -d ''`" anchors the good side of the rule.

## Entry 1 -- TBD -- First subagent dogfood pass

**Status:** Deferred until `critic-shell` is invoked via the Task tool in a follow-up session. The subagent's agent.md ships in this WP; the first real critic invocation is a post-WP12 activity (possibly WP07 integration work or a standalone "inception" session).

Planned invocation:

```
Task(subagent_type="critic-shell", prompt="review bin/intent bin/intent_helpers bin/intent_upgrade")
```

Expected outcome: zero critical findings. If critical findings surface, the triage is:

- True positive -- fix Intent's script (should be near-zero such cases given Entry 0's authorship path).
- False positive -- tighten the rule's Detection in its RULE.md.
- Genuine new concern -- draft IN-SH-CODE-007, repeat.

Journal entry will record results, rule changes, and commit hash.

## Entry 2 -- TBD -- Widened corpus

Planned expansion: `intent/plugins/claude/bin/`, `intent/plugins/claude/lib/`, and all `bin/intent_*`. Expected to surface some lint-level warnings from older scripts; acceptable as long as critical and warning tiers stay clean.

## Entry 3 -- TBD -- Stabilisation

Three consecutive clean runs freezes the pack for v2.9.0 release. Optional `good.sh` / `bad.sh` runnable examples authored for any rule where a textual example proved insufficient during dogfooding.

## Blog-post raw material

Assembled from the above into `docs/blog-drafts/shell-critic-inception.md` at WP12 close. Draft remains in-repo pending release; publication is out of scope for v2.9.0.
