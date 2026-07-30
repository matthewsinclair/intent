# Claude Code Session Restart -- narrative state

## Current state (2026-07-30)

**Two releases shipped the same day. v2.18.0 is current.**

**v2.17.4** closed four issues in two pairs, each pair sharing one root cause:

- **0006 + 0007 (high, `bin/intent_acceptance`) -- a `sed` non-match is invisible, in both directions.** The field extractors were bare `sed -E 's/.../\1/'`, and sed prints the input unchanged when a substitution does not fire, so each was total BY ACCIDENT and its failure value was a plausible-looking one: `status: **green` came back as the entire AT line, compared false against `green`, and gated a green test as unsatisfied in silence. On the write side, `ac satisfy` substituted into a ` -- evidence:` segment a non-test AC need not carry, and sed exits 0 on zero substitutions -- so the tool that records verified state printed `ok:` having written nothing. Fixed with one extraction seam (`extract_field`: match before substitute, one pattern for both uses, empty on non-match) and post-state write verification (`assert_written`: verifies the RESULT, not the mechanism). New diagnostics name out-of-vocabulary AT statuses and unclosed `(non-test` markers but deliberately do NOT block -- unlike a malformed id, these fail closed already; they failed closed silently, and the voice is the fix.
- **0005 + 0008 (medium) -- a generator asserting into a consumer repo something true only where the tool lives.** `intent lang init` wrote a rule-pack path that resolves only inside the Intent install; `intent agents sync` asserted `Bash 4.0+` into every project unconditionally. Both corrected. 0005 was WIDENED beyond its report because as specified the fix would have healed nothing: the writer skipped any language already present, so every affected project would have kept the stale text permanently. Made an upsert.
- **0009 filed** for the structural half of 0008 (filesystem probes vs the declared `languages` array) rather than folded in -- it changes every consumer's `AGENTS.md`.

**v2.18.0** completes v2.17.4 by making those corrections REACH existing projects, which turned out to be the more interesting bug:

- **`intent upgrade` now converges the Language Packs block.** New `lang_packs` ledger step delegating to a new `intent lang sync [--check]`. It delegates to `sync`, deliberately NOT `lang init` -- `init` copies `RULES-<lang>.md` over whatever is on disk and projects hand-edit those files, so an unattended upgrade running it would have destroyed exactly the work that surfaced 0005. Verified a marker in `RULES-shell.md` survives.
- **`bin/release` stamps all five sidecars before the tag.** Every prior tag was internally inconsistent (v2.17.2/3/4 each carried the previous version in `config.json` + `CLAUDE.md`, because the wrap landed after the tag). One `SIDECAR_FILES` list read by both the detect and stage steps; refusal to tag a dirty tree; `INTENT_HOME` pinned to the checkout being released. **`v2.18.0` is verified as the first self-consistent tag Intent has cut, and needed no wrap.**

Tags: `v2.17.4` (`af13633`, wrap `9650a5e`), `v2.18.0` (`6cd4400`). Both remotes + GitHub releases. Suite 1132/1132 + both integration files.

## Standing lesson from this session

For tooling that only runs at release or upgrade time, the dry-run path and the real path diverge. Both fixes were proven by EXERCISING them in a sacrificial copy -- a real release cut in a throwaway clone against a scratch remote with a `gh` stub, and a poisoned pre-2.17.4 consumer fixture upgraded end to end. Each turned up defects a dry run could not have shown: a stale exported `INTENT_HOME` beating the checkout being released, the canon engine reaching beyond `CLAUDE.md`, and a `set -e` abort in the shared stamper introduced an hour earlier. A first fixture also gave the WRONG answer about `intent upgrade` healing `AGENTS.md` because it was built wrong; it was rebuilt rather than trusted.

## Open follow-ups (non-blocking)

- **Consumer sweep: now just `intent upgrade` per project** (shipped in v2.18.0). Utilz / Lamplight / Baize all still carry the old canon. Watch the first one -- the upgrade path has one fixture behind it, not a live estate.
- **Lamplight contract sweep:** ST0276 (11 bolded `**green` rows) + `ST0298` `GREEN`, `ST0270` `BOTH`, `ST0198` `BUILT`. Not flipped by the parser fix (emphasis deliberately not tolerated), but each row is now named by the tool.
- **hv ruling owed -- issue 0004 item 4:** uniform non-zero exit on a BLOCKED `ac status`. Premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if wanted.
- Issue 0009 (open, low). Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits in their own repos (Conflab pushed). Utilz-side todo guard.
- AT-name traceability (vc deferral) -- now load-bearing: two ST0043 orchestrator guards had to keep their `@test` names verbatim because ST0043's `acceptance.md` cites them. Headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Where detail lives

- `.claude/restart.md` -- next-session focus. `intent/wip.md` -- current state + backlog.
- `intent/done.md` -- shipped ledger (July; older months in `intent/history/YYYYMM-done.md`). CHANGELOG `[2.17.4]` + `[2.18.0]`; issues 0005-0008 in `intent/issues/CLOSED/`, 0009 in `OPEN/`.
- Guards worth knowing: `tests/unit/release_sidecars.bats` (the release contract), `intent_upgrade_orchestrator.bats` (ledger + stamp-once), `intent_acceptance_cli.bats` (the 0006/0007 pair).

## Conventions (carry forward)

T-shirt sizing; intent CLI for ST/WP; never manually wrap markdown; no Claude attribution (end commit bodies `(C) hello@matthewsinclair.com`); no vanity metrics; fail-forward; commit to main only when asked; matts runs the full suite externally (single-file bats fine); matts is the acceptance verifier; never `bin/release --no-confirm`. **New:** author a CHANGELOG heading as `## [X.Y.Z] - in progress` and let `bin/release` stamp the date at cut time -- a hand-typed date is right only on the day it is typed.
