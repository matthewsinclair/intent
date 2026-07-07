---
archived: 2026-07-07
node: cc
---

# cc archive -- 2026-07-07 localfold

Folded out of the live board when 2.15.1 was prepped and ST0053 stood up. Shipped detail retained here for provenance.

## From DOING (shipped)

**Post-ship fix (2026-07-04, `d2abfc7`): `intent wp list` column width (matts-flagged Highlander).** The WP table hardcoded `%-30s` on Title, truncating every title regardless of terminal width (matts hit it as `intent wp list 270` in the control/NLP project -- global `intent` symlinks this repo, so the same `bin/intent_wp` served it). Fixed to derive columns from `get_terminal_width` (bin/intent_helpers) and flex Title to fill the terminal, mirroring `intent st list`. Guard test `wp_commands.bats` "sizes to terminal width (not a hardcoded 30)" -- 30/30 green. **Follow-up audit (matts: "any other examples?"):** swept every stdout table renderer -- no other truncating-middle-column bug. Found + removed two DEAD `get_terminal_width` calls -- `ext list` + `plugin list` computed width and never used it. `chore` `4eafb82`.

**Shared table renderer (`aa15a66` content-fit -> `b88cb8c` terminal-fill, FINAL).** matts flagged `st list` != `st sync` (a real Highlander violation: same codepath, divergent width param). Fixed by extracting ONE `render_table` (bin/intent_helpers) that `st list` + `st sync` + `wp list` all render through. Intermediate `aa15a66` made both content-fit -- WRONG; matts wanted terminal-width KEPT. `b88cb8c` is the correct terminal-fill: fills the terminal width (or explicit `--width`) with content-fit as the FLOOR (never truncates); st list==sync byte-identical (sync composes list, forwards --width). Caught + fixed an IFS-leak in the dash generator (now `printf -v`, no word-splitting). Full 10-suite sweep green.

**ST0052 -- author project-type pack (SHIPPED in v2.15.0, tag `33e5d57`).** First non-code discipline on the `languages` axis. `AU` code; nine `IN-AU-*` rules (style/craft tiers); `critic-author` subagent (two-form detrope off the single `in-detrope` catalogue); `intent lang init author` canon; `/in-author-essentials` + `author -> critic-author` dispatch. Six WPs, closed 21/21. Headless prose gate deferred (D4). Post-tag wrap `425fa59` + docs-lean `01f0875`; Intent self-upgraded 2.14.0 -> 2.15.0.

## From Decisions (shipped-ST era)

- (2026-07-02) matts ACCEPTED the WP-06 sticky-watermark model (ST0050): DONE = "completed since the last flush"; acceptance-verify flag closed.
- (2026-07-02) hv RATIFIED D1-D4 (ST0050); cc RULED WP-06 as-built. Canon in `intent/st/COMPLETED/ST0050/design.md`.
- (earlier) v2.13.1 SHIPPED.
