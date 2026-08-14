---
verblock: "14 Aug 2026:v1.04: vc - globalfold; v2.19.0 built + green, cut first thing next session"
intent_version: 2.18.0
---

# Work In Progress

## Current State

**v2.19.0 BUILT, VERIFIED, HOLDING FOR THE CUT (first thing 2026-08-15).** Eleven issues closed end to end (0009-0019): the AT row grammar + `at lint`/`--fix` (0017, subsuming 0014 + 0015), four AC states with descope/withdraw verb pairs (0013 + hv's addition), one steel-thread enumerator + a voiced `organize` (0011), the whiteboard header block ruled not-YAML and enforced (0012), portable hooks with no baked home path (0016), AGENTS.md prerequisites from the declared languages (0009), the objective-placeholder warning at close (0010), the treeindex cache untracked (0018), and the canonical thread index actually indexing all 55 threads (0019). The lossy first version of `--fix` was caught by a consumer measuring a real sweep; the 87 name links it destroyed in our own estate were restored verbatim from git (`ee44f63`). **Full suite GREEN at HEAD (hv-run, 2026-08-14).** Nothing pushed; no tag; VERSION/config deliberately still 2.18.0 for `bin/release` to stamp; CHANGELOG heading `## [2.19.0] - in progress` for the script to date. Terse ledger: `intent/done.md`; comprehensive narrative: CHANGELOG `[2.19.0]`; per-issue record: `intent/issues/CLOSED/0009..0019`.

## Next Up

1. **Cut v2.19.0**: `bin/release --minor` (interactive; NEVER `--no-confirm`). Stamps all five sidecars + dates the CHANGELOG at the cut; refuses a dirty tree. Post-cut: `intent/history/v2.19.0.md` narrative, done.md entry flips to shipped + tag, push both remotes + GitHub release.
2. **Consumer sweeps — one `intent upgrade` per project** (Lamplight, Utilz, Baize). The one pass now sweeps AT grammar (`at lint --fix`; residue named, never guessed — expect BLOCKED-until-swept, which is the fix working), converges AGENTS.md + portable-hook settings.json + the gitignore entries, and PRINTS (never runs) the `git rm` for a tracked treeindex cache. Lamplight first: 314 AT rows plus its four known bad-status contracts (ST0276 `**green` x11, ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`).
3. **Push the fleet issue-normalisation commits** in their own repos: Utilz (`0171297`), Lamplight (`7058fd3a8`); Conflab already pushed. Utilz-side todo guard carry-over.
4. **hv-ruling queue** (each becomes its own issue if wanted): 0004 item 4 (`ac status` exit code — premise does not reproduce); a `javascript` pack to complete 0009's Node exception; pruning consumers' now-inert `.claude/scripts/` copies; `intent_claude_prime:212` (truncation notice on stdout with a capital prefix — voice AND stream).

## Recent

- **2026-08-13/14**: v2.19.0 built — issues 0009-0019 fixed + closed; `--fix` hardened three times on consumer evidence; home-estate name links restored (`ee44f63`); suite green.
- **2026-07-30**: v2.18.0 + v2.17.4 shipped. Earlier: `intent/history/202607-done.md`.

## Parked

_(None.)_
