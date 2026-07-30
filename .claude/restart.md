# Claude Code Session Restart

## First actions after `/compact` or new session

1. **Invoke `/in-session`.** Loads `/in-essentials` + `/in-standards`, releases the gate. (Languages: shell only.) Whiteboard present (`intent/whiteboard/`, hv+cc+vc) -- `/in-session` chains `/in-whiteboard pickup`. Solo unless launched as a node via `intent claude start <ws>`.
2. **Read this file + `intent/wip.md`.**

## State: v2.18.0 SHIPPED

**Two releases the same day (2026-07-30).**

**v2.17.4** -- four issues closed in two pairs. **0006 + 0007** (high, `bin/intent_acceptance`): a `sed` non-match is invisible, so the extractors were total BY ACCIDENT (a failed parse returned the whole line as a plausible value -- `status: **green` gated a green test as unsatisfied in silence) and `ac satisfy` printed `ok:` having written nothing. Fixed with one extraction seam (`extract_field`) + post-state write verification (`assert_written`). New diagnostics name bad AT statuses and unclosed `(non-test` markers but do NOT block -- these already fail closed; they failed closed silently. **0005 + 0008** (medium): a generator asserting into a consumer repo something true only where the tool lives (a dangling rule-pack path; an unconditional `Bash 4.0+`). 0005 was widened to an upsert because as specified it would have healed nothing. **0009 filed** (probes vs declared `languages`), deliberately not folded in.

**v2.18.0** -- makes those corrections REACH existing projects, which was the more interesting bug. `intent upgrade` now converges the Language Packs block (`lang_packs` ledger step -> new `intent lang sync [--check]`); it delegates to `sync`, NOT `lang init`, because `init` copies `RULES-<lang>.md` over hand-edited files. `bin/release` now stamps all five sidecars before the tag: **`v2.18.0` is the first self-consistent tag Intent has cut** (v2.17.2/3/4 each carried the previous version in `config.json` + `CLAUDE.md`), and needed no wrap.

Tags `v2.17.4` (`af13633`), `v2.18.0` (`6cd4400`); both remotes + GitHub releases. Suite 1132/1132. Detail: `intent/done.md`, `intent/restart.md`, CHANGELOG `[2.17.4]`/`[2.18.0]`, `intent/issues/CLOSED/000{5,6,7,8}/`.

## Standing lesson

For tooling that only runs at release or upgrade time, the dry-run path and the real path diverge -- exercise the real one in a sacrificial copy. Both fixes were proven that way (a real release cut in a throwaway clone with a `gh` stub; a poisoned consumer fixture upgraded end to end), and each turned up defects a dry run could not show. One fixture was built wrong and gave the WRONG answer; it was rebuilt rather than trusted.

## Open follow-ups (non-blocking)

- **Consumer sweep: `intent upgrade` per project** -- one command now (v2.18.0). Utilz / Lamplight / Baize still on old canon. Watch the first: one fixture behind it, not a live estate.
- **Lamplight contract sweep:** ST0276 (11 bolded `**green` rows) + `ST0298` `GREEN`, `ST0270` `BOTH`, `ST0198` `BUILT`. Not flipped by the fix (emphasis deliberately not tolerated); each row is now named by the tool.
- **hv ruling owed -- issue 0004 item 4:** non-zero exit on a BLOCKED `ac status`; premise does not reproduce. `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if wanted.
- Issue 0009 (open). Push Utilz (`0171297`) + Lamplight (`7058fd3a8`) issue-normalisation commits (Conflab pushed). Utilz-side todo guard.
- AT-name traceability (vc deferral) -- now load-bearing: two ST0043 guards keep their `@test` names verbatim because ST0043's contract cites them. `bin/release` v2 polish is DONE. Headless `intent critic prose` gate (D4); `docs/blog/README.md` dead link 0007.

## Backlog

`/in-review` Elixir fleet sweep (Anvil, Lamplight, MeetZaya, MicroGPTEx, Conflab); Conflab TEST-001/005/007; Homebrew tap; `$N`-in-SKILL.md audit; shell-critic-inception blog; skill-sync blind spot; ST0040/ST0041 deferred.

## Fleet

Members pick up v2.18.0 on next `intent upgrade`. Excludes Pplr, Sites-in-Laksa, llm-tropes. NOTE: v2.18.0 makes `intent upgrade` REWRITE `intent/llm/RULES.md` (the Language Packs block) on every member where it previously did not -- entries are upserted to canon once. That is the fix working. Hand edits to `RULES-<lang>.md` are untouched; only the tool-managed block inside the `intent-lang-packs` markers changes. Carried forward from v2.17.4: contracts with out-of-vocabulary AT statuses now emit warnings naming those rows -- the rows were already failing to satisfy their criteria, so the warning reports what was previously silent, not a new restriction.

## Conventions

T-shirt sizing only. ALWAYS use the intent CLI for ST/WP. NEVER manually wrap markdown. NO Claude attribution in commits; end bodies with `(C) hello@matthewsinclair.com`. No vanity metrics. Fail-forward. Commit to `main` only when matts asks. matts runs the full suite externally. matts is the acceptance verifier. NEVER `bin/release --no-confirm`. Author CHANGELOG headings as `## [X.Y.Z] - in progress` and let `bin/release` date them at cut time.
