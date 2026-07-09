---
node: cc
name: Control Claude
role: control
session_id: 4c632d01-b80b-44a9-8178-bb8e30e36ef2
heartbeat_at: 2026-07-09T09:17Z
status: active
focus: "2.16.1 READY TO CUT (pending hv release). ST0054 (usage-rules v1.x alignment) CLOSED (3 WPs, 6/6 ACs, gate PASS) + 4 companion chores all DONE: C1 st-index markdown-persist fix; C2 localfold/globalfold terms; C3 intent todo in instructions; C4 intent<->utilz todo mutual generator-marker guard. Two critic-shell passes CLEAN. hv runs scripts/release --patch; config.json bump is the manual post-tag wrap. Utilz-side guard = separate follow-up (handoff note delivered)."
claims: [ST0054]
---

# Control Claude (cc)

## DOING

**2026-07-09 -- 2.16.1 in build (docs/skills-alignment release; anchor ST0054).** hv provisioned ST0054 off a Laksa deps-hygiene sweep (usage_rules 0.1.26 -> 1.2.6) + fed 4 companion chores. Status:

- **[x] ST0054 (3 WPs, five-step, gated) -- CLOSED.** usage-rules v1.x alignment. WP-01 rewrote `working-with-llms.md` interop (v1.x config-driven model + two-artifacts distinction + `.claude/skills` coexistence policy, Intent stays Intent-native). WP-02 added topical `deps/*/usage-rules/*.md` folders to `/in-standards` + `/in-elixir-essentials` + `/in-ash-ecto-essentials`. WP-03 `_usage-rules.md`: dropped staleable version line, added name-collision note. Verify sweep reconciled ToC + line-124 auto-gather framing. 6/6 ACs, `intent ac gate` PASS. Ground-truthed vs `../Laksa/deps/usage_rules/README.md` (1.2.6).
- **[x] C1 st-index markdown-persist fix -- DONE.** `render_table` gained a content-fit `markdown` mode (`| ... |` + `| --- |`); `st sync --write` routes through it (blank-line padded), terminal `st list`/`wp list` display unchanged. Persisted `steel_threads.md` now deterministic canonical GFM (identical at COLUMNS=200/60). Rewrote `output_width.bats:77` to the new contract; both affected bats files green; critic-shell CLEAN (0 findings, Highlander preserved).
- **[x] C2 localfold/globalfold terms -- DONE.** "Fold scopes" section in `/in-finish` (+ description frontmatter surfaces the terms) + "Fold vocabulary" cross-ref in `/in-whiteboard`; memory `feedback_localfold_vs_globalfold` updated to hv's timing-based definition.
- **[x] C3 `intent todo` in instructions -- DONE.** Rule 9 in `/in-essentials` (always-loaded) + orientation step in `/in-start` + review step in `/in-next`.
- **[x] C4 intent todo <-> utilz todo mutual guard -- DONE.** hv chose "implement guard now". `bin/intent_todo` stamps `generator: intent todo` YAML frontmatter into the generated todo.md and refuses (via `error()`) to overwrite a file whose frontmatter names a different generator, or a pre-marker utilz file (`title:`/`history:`); legacy no-frontmatter intent files regenerate + gain the marker. 4 new/updated bats (21/21 green, prettier-stable). Contract: each tool writes `generator: <tool> todo`. Utilz-side follow-up (separate repo): add `generator: utilz todo` + symmetric guard -- handoff note to hv.

## TODO

- Hand to hv for `scripts/release --patch` (2.16.1). All 4 chores ride 2.16.1; ST0054 is the only ST. matts runs the full suite + is the acceptance verifier.
- Utilz-side follow-up (hv, separate repo): add `generator: utilz todo` frontmatter + symmetric guard to utilz `todo`.
- DEFERRED (needs hv ruling): AT-name traceability -- machine-check `acceptance.md` AT ids against real bats `@test` names.

## Watch-outs

- `scripts/release` does the tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it.
- Skill edits this session (in-standards, in-elixir-essentials, in-ash-ecto-essentials, in-finish, in-whiteboard, in-essentials, in-start, in-next) were propagated to the installed `~/.claude/skills` via `intent claude skills sync`; fleet picks them up on `intent upgrade`.
- C1: `render_table` (intent_helpers) terminal mode is pipeless/width-filled by design (matts's v2.15.1 UX) -- only the FILE-persist path uses markdown mode. Don't "unify" them back.

## Decisions

- (2026-07-09) hv locked 2.16.1 scope: ST0054 is the only ST; C1/C2/C3(/C4) ride as companion chores (no new STs). Follow the five-step for ST0054's WPs.
- (2026-07-09) C1 decouple ratified by hv: persisted steel_threads.md index = deterministic canonical GFM (content-fit); terminal `st list` display stays width-filled. Reverses the v2.15.1 `output_width.bats:77` "file fills terminal width" invariant (was linter-masked).
- (2026-07-09) localfold vs globalfold (hv, authoritative): localfold = per-workstream tidy before a compact; globalfold = project-wide tidy before EOD when all workstreams close, the coordinating/validation node's job.
- (2026-07-07) hv RATIFIED ST0053 D1-D5 [shipped v2.16.0].
