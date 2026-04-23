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

## Entry 1 -- 2026-04-23 -- First subagent dogfood pass (post-release)

**Invocation:** `Task(subagent_type="critic-shell", prompt="review <27 paths>")` against the full `bin/intent*` quartet, both `intent/plugins/agents/bin/intent_agents` and the four `intent/plugins/claude/bin/intent_claude_*` plugin dispatchers, plus `tests/run_tests.sh`. Tree at commit `b020fbe`. No `.intent_critic.yml` at project root (defaults applied). All targets bash; IN-SH-CODE-004 (zsh) not applied.

**Counts:** 7 CRITICAL · 16 WARNING · 17 RECOMMENDATION · 14 STYLE.

Entry 0 predicted "near-zero critical findings". Actual count of seven critical findings refutes that prediction. Reading: rule authorship anchored to Intent's broad style (no `set -u`/`pipefail` exception, helper-naming conventions) but did not catch every concrete instance of IN-SH-CODE-001 / 005. Authorship-as-corpus aligns the broad shape; only a critic pass exercises the narrow Detection heuristics. Both are needed.

### Triage

| Tier | Maps to        | Action                                                                            | Findings                                                                                                                                                                                                                    |
| ---- | -------------- | --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P0   | CRITICAL true  | Fix this session                                                                  | IN-SH-CODE-001 `intent_st:378,1130` (unquoted `$()` -- paths-with-spaces bug); IN-SH-CODE-005 `intent_st:865,940` (`for x in $(find ...)` swallows find exit); IN-SH-CODE-001 `intent_audit:397,463,505` (`$lib_dirs`).     |
| P1   | WARNING        | Fix this session                                                                  | IN-SH-CODE-002 `intent_doctor:525` (`ls \| wc -l`); IN-SH-CODE-002 `tests/run_tests.sh:77` (`find \| xargs` no `-print0`); IN-SH-CODE-006 `intent_plugin:37` (silent shadow of canonical `get_terminal_width`).             |
| P2a  | RECOMMENDATION | Sweep commit (this session or follow-up ST)                                       | IN-SH-CODE-003 missing `set -e` in 7 files (`intent_doctor`, `intent_minimal`, `intent_agents`, three `intent_claude_*`, `tests/run_tests.sh`); `intent_config` separate (sourced library -- comment, not flag).            |
| P2b  | RECOMMENDATION | Lift `info()` / `warning()` into `intent_helpers`, then strip duplicates          | IN-SH-CODE-006 `intent_doctor:107,112,124`; `intent_bootstrap:59`; `intent_config:176`; `tests/run_tests.sh:13-31`. Net Highlander pass.                                                                                    |
| P3   | STYLE          | Defer; revisit if a `.intent_critic.yml` carve-out is preferred to a quoting pass | 14 unquoted `[ $n -gt $m ]` arithmetic comparisons across 7 files; project's de-facto convention has historically allowed bare numerics in test brackets.                                                                   |
| FP   | --             | Accept                                                                            | IN-SH-CODE-006 `plugin_*` clones across `intent_claude_skills` / `intent_claude_subagents` -- legitimate per-plugin callbacks, refactor only when a 3rd plugin appears (critic itself flagged "defer to author judgement"). |

### What was fixed this session

P0/P1 fixes shipped in commit `a9ee349`. All seven CRITICAL plus three of the WARNING tier resolved. Bats remained 707/707 green throughout.

### What was queued

P2a (`set -e` sweep) and P2b (lift `info`/`warning` into `intent_helpers`, strip 4 duplicate sites) folded into a follow-up ST: see `intent/st/<ID>` (or filed inline in this entry's "Follow-on" section if the ST is opened later).

### What was learned

- **Authorship-as-corpus catches the silhouette; subagent dogfooding catches the joints.** Entry 0's "zero critical findings" prediction was wrong by seven. The shape of the rules was right; the line-by-line application was always going to need a real run.
- **`for x in $(find ...)` is everywhere.** Two CRITICAL hits in `intent_st` alone, plus several RECOMMENDATION-tier echoes. The rule's bad/good examples should be promoted to a runnable `bad.sh`/`good.sh` pair so the conversion idiom is unmistakable.
- **No false positives on the rules themselves.** Every CRITICAL was a real bug; every WARNING was a defensible re-shaping. The "defer to author judgement" plugin-clone case was correctly self-flagged by the critic, not by Intent's review.
- **Time to first finding:** ~13 minutes (single subagent invocation, 27 files, 100k tokens).
- **No `.intent_critic.yml` carve-outs were warranted.** The bracket-quoting STYLE class is the only candidate; deferred until a future style-pass decision.

## Entry 2 -- TBD -- Widened corpus

Planned expansion: `intent/plugins/claude/bin/`, `intent/plugins/claude/lib/`, and all `bin/intent_*`. Expected to surface some lint-level warnings from older scripts; acceptable as long as critical and warning tiers stay clean.

## Entry 3 -- TBD -- Stabilisation

Three consecutive clean runs freezes the pack for v2.9.0 release. Optional `good.sh` / `bad.sh` runnable examples authored for any rule where a textual example proved insufficient during dogfooding.

## Blog-post raw material

Assembled from the above into `docs/blog-drafts/shell-critic-inception.md` at WP12 close. Draft remains in-repo pending release; publication is out of scope for v2.9.0.
