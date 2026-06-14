# Claude Code Session Restart -- narrative state

## Current state (2026-06-14)

v2.11.14 is the shipped baseline (a Linux `intent organize` fix -- `((x++))` under `set -e` -- atop ST0044's v2.11.13). **ST0044 is COMPLETE + SHIPPED** (2026-06-14) -- `acceptance.md` as a default steel-thread doc + the AC/AT process that makes "done" externally verified; dogfooded on itself with matts as verifier and closed through its own close-gate (16/16). Relocated to `intent/st/COMPLETED/ST0044/` and shipped as a standalone patch (tag `5cc676a`, both remotes + GitHub release) on opt-in-by-presence grounds. Terse ledger `intent/done.md`; narrative `intent/history/v2.11.13.md`. **Active next: ST0043** (rethink `intent upgrade`, its own v2.12.0 minor) -- its own session.

## ST0044 -- acceptance.md + AC/AT process (COMPLETE)

Done 2026-06-14, dogfooded on itself with matts as verifier, closed through the close-gate it built (16/16 ACs). `acceptance.md` is a default steel-thread doc; AC = ratified coverage boundary, AT = red-to-green proof (green only from red); `intent ac` / `intent at` instrument the contract; the close-gate (`intent ac gate`, consulted by `st done` / `wp done`) is opt-in / legacy-safe. Five-step in `working-with-llms.md` D11 with pointers in `/in-plan` / `/in-verify` / `/in-finish`. Full detail: `intent/st/COMPLETED/ST0044/`; narrative: `intent/history/v2.11.13.md`.

## ST0043 -- Rethink `intent upgrade` (active next)

WIP. Architecture-B in `intent/st/ST0043/info.md`; ACs drafted in `intent/st/ST0043/acceptance.md`. Targets v2.12.0.

## Where detail lives

- `.claude/restart.md` -- next-session focus (ST0043 kickoff).
- `intent/st/COMPLETED/ST0044/` -- ST0044 docs (closed); `intent/st/ST0043/` -- ST0043 design + ACs.
- `intent/wip.md` -- current-state summary + backlog.
- `intent/done.md` + `intent/history/v2.11.*.md` -- shipped-work ledger / narratives.

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; user runs the full test suite externally (single-file bats fine); matts is the acceptance verifier this session.
