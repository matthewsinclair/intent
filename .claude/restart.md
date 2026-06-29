# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) **Intent now HAS a whiteboard** (`intent/whiteboard/`, hv+cc+vc) -- so `/in-session` step 5 chains `/in-whiteboard pickup`. This is a solo session unless you were launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.13.1 SHIPPED (ST0048 + ST0049)

v2.13.1 shipped (release tag `v2.13.1`, commit `d01a1b2`, both remotes + GitHub release); Intent self-upgraded 2.13.0 -> 2.13.1 clean. **ST0048** -- the acceptance close-gate is now fail-by-default: a missing `acceptance.md`, or a present one with zero in-scope ACs, BLOCKS `st done` / `wp done`; the sole escape is `acceptance: exempt` in the frontmatter (announced, never inferred); WP scope is WP-lenient (a WP with no own ACs rolls up to the ST boundary). This REVERSES the v2.12.0 F6 ruling (missing-stays-open) -- a shipped-as-broken patch, matts-ratified. **ST0049** -- comprehensive retroactive 2.13.0 MAAC release note + the 2.13.1 note; `docs/releases` resumes after the 2.9.0 lapse (NO backfill of 2.10-2.12). Both closed through their own gates (ST0048 11/11; ST0049 EXEMPT) -> `intent/st/COMPLETED/`.

### Open: push the wrap commit

Fix `baeae83` + release `d01a1b2` are pushed; the post-ship wrap `c0eeefe` (config.json/CLAUDE.md bump + ST closures + board) is LOCAL -- main is one commit ahead of both remotes, and the /in-finish doc commit sits on top. Push when matts says.

### Fleet

v2.13.0 fleet sweep is DONE (12 `~/Devel/prj` projects, 2026-06-25). Members pick up v2.13.1 on their next `intent upgrade` -- and will hit the close-gate change: any in-flight unit with no authored ACs stops closing until it authors them or marks `acceptance: exempt`. Excludes Pplr, Sites-in-Laksa, llm-tropes.

## v2.13+ backlog (from wip.md)

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab test findings (TEST-001/005/007); Homebrew tap; `scripts/release` v2 polish (config.json bump still a manual post-tag wrap); `$N`-in-SKILL.md trap audit; shell-critic-inception blog draft; skill-sync script-change blind spot; ST0040 + ST0041 deferred items.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `scripts/release --no-confirm`.
