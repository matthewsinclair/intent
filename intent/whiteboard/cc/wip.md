---
node: cc
name: Control Claude
role: control
session_id: bf1f140b-ef3b-410c-bf6c-da5a3ff299de
heartbeat_at: 2026-07-08T00:40Z
status: active
focus: "ST0053 CLOSED (15/15 gate PASS, -> COMPLETED). Content/prose pack COMPLETE: IN-PR-* base (4) + IN-CO-* content (6); critic-author->critic-prose (parameterised); intent lang init content; /in-content-essentials + dispatch. docs/blog dogfood run -> findings applied (description+canonical on 7 posts, seamless reworded); overall dropped from IN-PR-STYLE-001 (v2) per hv. Also fixed a latent intent_todo fixture UTC bug (`ca13323`). index 64; all suites green. 2.16.0 READY TO CUT: hv runs `scripts/release --minor`; then cc does the post-tag config wrap (intent_version -> 2.16.0). CHANGELOG [2.16.0]-in progress + history + release notes done."
claims: []
---

# Control Claude (cc)

## DOING

**2026-07-07 -- 2.15.1 SHIPPED.** Tag `2cdb5b5` both remotes + GitHub release (matts ran `scripts/release --patch --skip-tests`; full suite green pre-cut). Post-tag wrap `0e7039d` bumped config.json intent_version -> 2.15.1 (release script doesn't stamp it -- known wart); doctor green. Contents: table-renderer Highlander (`b88cb8c`); `intent todo` enumeration Highlander (`4973a30`); `scripts/release confirm()` hardening (`06b386a`); dead `get_terminal_width` removal (`4eafb82`); CI apt hardening (`9016c3c`). Localfold done (shipped detail -> `.history/20260707/`).

**ST0053 -- web-content / prose pack (NEXT: build -> 2.16.0). D1-D5 hv-RATIFIED.** D1=`CO` code (content: book, course, etc). D2=shared `IN-PR-*` prose base pack that both author + content disciplines build on (reuse, not copy). D3=one `critic-prose` serving both, parameterised by declared language. D4=headless gate in scope. D5=(a) 2.15.1 first (DONE), pack as 2.16.0. Scope docs (info+design) written; WPs to be created on hv go-ahead.

## TODO

- **cc: build ST0053 -> 2.16.0** (awaiting hv go-ahead on the WP plan). Draft order: WP1 `IN-PR-*` base extraction + refactor author onto it; WP2 `CO` content tiers; WP3 `critic-prose` (parameterised); WP4 lang canon (`intent lang init content`); WP5 skill + dispatch; WP6 dogfood + close.
- DEFERRED (needs hv ruling): AT-name traceability -- tag `@test` with its `AT-id` / bake into `intent at` so `acceptance.md` `::names` are machine-checkable against real test names.

## Watch-outs

- `scripts/release` does the tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- that stays a manual post-tag wrap commit, so a release tag carries the PRIOR config.json version until the wrap lands. Don't skip the wrap.
- D4 evidence for ST0053's headless gate: Intent's mandated `--` house style trips the trope catalogue's dash-overuse regex, so a headless prose gate needs a confirmation/suppression layer or it drowns in house-style false positives. Reinforces the two-tier (mechanical candidate -> judgment confirm) design.

## Decisions

- (2026-07-07) hv RATIFIED ST0053 D1-D5: D1=`CO` code; D2=shared `IN-PR-*` base pack (author + content both build on it, reuse not copy); D3=one `critic-prose` parameterised by language; D4=headless gate in scope; D5=(a) 2.15.1 patch first, then the pack as 2.16.0.
- (2026-07-07) localfold defined (hv): fold ONLY this workstream's own intent docs; cf globalfold = the project-wide docs (wip.md/restart.md/done.md), usually the vc workstream's job.
