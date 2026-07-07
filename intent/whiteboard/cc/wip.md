---
node: cc
name: Control Claude
role: control
session_id: bf1f140b-ef3b-410c-bf6c-da5a3ff299de
heartbeat_at: 2026-07-07T22:11Z
status: active
focus: "2.15.1 patch line READY to cut. Awaiting hv to run `scripts/release --patch` (matts runs externally; cc can't -- pre-flight runs the full bats suite). After the tag: post-tag wrap (bump config.json intent_version -> 2.15.1). THEN cc builds ST0053 (web-content pack) -> 2.16.0 (D5=(a): patch first, pack second)."
claims: []
---

# Control Claude (cc)

## DOING

**2026-07-07 -- 2.15.1 prepped + folded; awaiting hv to cut.** Everything for 2.15.1 is committed + pushed both remotes; CHANGELOG carries `[2.15.1] - in progress`. Contents: table-renderer Highlander (`b88cb8c` -- one `render_table` in bin/intent_helpers drives `st list` + `st sync` + `wp list`; fills the terminal width or explicit `--width`, content-fit as the FLOOR, never truncates; st list==sync byte-identical); `intent todo` enumeration Highlander (`4973a30`); `scripts/release confirm()` hardening (`06b386a`); dead `get_terminal_width` removal (`4eafb82`); CI apt hardening (`9016c3c`). 3/4 vc v2.14.1 follow-ups closed (AT-name traceability DEFERRED -- see TODO). Localfold done this session (shipped detail -> `.history/20260707/`).

**ST0053 -- web-content / prose pack (STOOD UP, D1-D5 hv-RATIFIED).** Not yet built (waits on 2.15.1 tag). D1=`CO` code (content: book, course, etc). D2=shared `IN-PR-*` prose base pack that both author + content disciplines use (reuse, not copy). D3=one `critic-prose` serving both, parameterised by declared language. D4=in scope this release. D5=(a) cut 2.15.1 first, then build the pack as 2.16.0. Scope docs (info+design) written; NO WPs until the 2.15.1 tag lands.

## TODO

- **hv ACTION: cut 2.15.1** -- `bash scripts/release --dry-run --patch` (preview), then `bash scripts/release --patch`. After the tag, cc does the post-tag wrap (config.json intent_version -> 2.15.1; the release script does NOT bump it -- known wart).
- **THEN cc: build ST0053 -> 2.16.0.** Order: WP1 `IN-PR-*` base extraction + refactor author onto it; WP2 `CO` content tiers; WP3 `critic-prose` (parameterised); WP4 lang canon (`intent lang init content`); WP5 skill + dispatch; WP6 dogfood + close.
- DEFERRED (needs hv ruling): AT-name traceability -- tag `@test` with its `AT-id` / bake into `intent at` so `acceptance.md` `::names` are machine-checkable against real test names.

## Watch-outs

- `scripts/release` does the tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- that stays a manual post-tag wrap commit, so a release tag carries the PRIOR config.json version until the wrap lands. Don't skip the wrap.
- D4 evidence for ST0053's headless gate: Intent's mandated `--` house style trips the trope catalogue's dash-overuse regex, so a headless prose gate needs a confirmation/suppression layer or it drowns in house-style false positives. Reinforces the two-tier (mechanical candidate -> judgment confirm) design.

## Decisions

- (2026-07-07) hv RATIFIED ST0053 D1-D5: D1=`CO` code; D2=shared `IN-PR-*` base pack (author + content both build on it, reuse not copy); D3=one `critic-prose` parameterised by language; D4=headless gate in scope; D5=(a) 2.15.1 patch first, then the pack as 2.16.0.
- (2026-07-07) localfold defined (hv): fold ONLY this workstream's own intent docs; cf globalfold = the project-wide docs (wip.md/restart.md/done.md), usually the vc workstream's job.
