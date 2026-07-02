---
node: cc
name: Control Claude
role: control
session_id: b9414b4d-2a1e-42bc-98e9-9fc6b795f865
heartbeat_at: 2026-07-02T21:45Z
status: active
focus: "v2.14.0 releasing (matts runs scripts/release --minor manually); board archived; post-release wrap pending"
claims: [ST0050, ST0051]
---

# Control Claude (cc)

## DOING

**v2.14.0 release in progress -- matts running `scripts/release --minor` (manual; its pre-flight runs the full suite).** ST0050 + ST0051 built + closed; tree clean; 11 commits ride the release push. The completed homerun detail is archived in `.history/20260702/`.

## TODO

1. **Post-release wrap** (after matts' release tag): `scripts/release` bumps `VERSION` but NOT `config.json` / `CLAUDE.md` (both still 2.13.1). Self-upgrade `intent upgrade --apply` (2.13.1 -> 2.14.0: bumps config.json + regens canon) or manual bump; commit + push the wrap.
2. **Standing backlog** (post-2.14.0): `/in-review` Elixir fleet sweep; Conflab test findings; Homebrew tap; `scripts/release` v2 polish (the config.json bump gap this wrap exercises); `$N`-in-SKILL.md audit; shell-critic-inception blog; ST0040/ST0041 deferred items.

## Watch-outs

- **WP-06 sticky-watermark flagged for matts' acceptance-verify** (open until he confirms). `## DONE:<T>` is authoritative + sticky; "swept daily" became "swept on flush". Reversible pre-release via `intent st start ST0050`; post-release it's a 2.14.1.
- **11 commits unpushed** until matts' release push (2x v2.13.1 wrap + ST0050 capture + ST0051 + 6 ST0050 build/close + 2 doc/board wrap). `scripts/release` needs a CLEAN tree -- keep it clean while it runs.
- **config.json / CLAUDE.md lag 2.13.1** until the post-release wrap (`scripts/release` bumps `VERSION` only -- known v2 gap).

## Decisions (ratified)

- (2026-07-02) hv RATIFIED D1-D4; cc RULED WP-06 as-built (verb placement, sticky `>=` watermark, UTC, prune stdout/note-stderr). Full detail archived in `.history/20260702/wip.md`; canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED. Detail in `intent/wip.md` + git history.
