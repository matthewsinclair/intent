---
node: cc
name: Control Claude
role: control
session_id: 347f2808-51b6-4c3c-90a5-3b43d41f5ecf
heartbeat_at: 2026-08-13T21:07Z
status: active
focus: "Executing vc's triage work order on issues 0009-0017 (7 units -> v2.19.0). Plan posted to hv; 4 ratified calls in veto window. Unit 1 (0017 AT grammar) first once hv says go."
claims: []
---

# Control Claude (cc)

## DOING

- **vc work order 0009-0017 -> v2.19.0** (inbox entry 2026-08-13 20:59; fix under the issues, no ST). hv ratified all four calls + granted per-unit commit authority, and ADDED an AC-withdrawal verb to U2. Sequence: ~~U1 0017+0014+0015 AT grammar~~ **DONE** -> U2 0013 descope + withdraw (M) -> U3 0011 enumerator (M) -> U4 0012 not-YAML (S) -> U5 0016 portable hooks (S/M) -> U6 0009 prereqs-from-langs (S) -> U7 0010 objective warn (XS).
- **U1 landed.** Two-arm anchored AT grammar + `intent at lint` L1-L5 + `--fix` + gate fold + `at green|red` citation refusal + `at_grammar` ledger step. Three holes in the issue's proposed grammar were found by RUNNING it against real rows, not reading it: the retired `path::name` (vc's delta a), the parenthetical status note the shipped template itself taught, and -- structurally -- non-test `n/a` rows, which the one-armed grammar gave no legal form to migrate to despite the status vocabulary declaring them. Our own 116 rows swept (103 mechanical, 13 by hand) and one REAL issue-0015 instance found in our own estate: ST0052 AT-03.1 green on `critic_author.bats`, renamed to `critic_prose.bats` in ST0053, citation left behind.

## TODO

- **Consumer sweep (hv, separate repos): now just `intent upgrade` per project**, shipped in v2.18.0. Utilz / Lamplight / Baize still on old canon. NOTE: after v2.19.0's Unit 1, the same upgrade also sweeps AT grammar -- fold into one pass.
- **Lamplight contract sweep (hv, separate repo):** ST0276 (11 bolded `**green` rows), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`; now also the 314-row AT-grammar sweep once v2.19.0 lands (`at lint --fix` does the mechanical part).
- **hv ruling owed -- issue 0004 item 4 (`ac status` exit code).** Premise does not reproduce; `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if hv wants it changed.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- AT-name traceability deferral: SUPERSEDED by 0017's link-by-id (L3 makes the id the traceable token, not the `@test` name) -- confirm at U1 close and retire the entry.

## Watch-outs

- `bin/release` now stamps ALL five sidecars (VERSION, CHANGELOG date, config.json `intent_version`, AGENTS.md, CLAUDE.md) before the commit -- there is no manual post-tag wrap any more. Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. It is still interactive; if a run is interrupted mid-cut, finish the push/gh by hand -- it is idempotent on the tag. If it aborts with "refusing to tag a dirty tree", something outside the sidecar list changed: read the list it prints, do not just re-run.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-30) Two tooling fixes taken directly rather than filed, on hv's "just fix it": `bin/release` (every published tag was self-inconsistent because the version wrap landed AFTER the tag) and `intent upgrade` (the convergent orchestrator did not converge the tool-managed Language Packs block, so an issue-0005-class fix only reached projects that re-ran `lang init` by hand). Both were verified by EXERCISING them, not by reading: a real test release cut in a throwaway clone against a scratch remote with a `gh` stub, and a poisoned pre-2.17.4 consumer fixture upgraded end to end. Each turned up defects a dry run could not have shown -- a stale exported `INTENT_HOME` beating the checkout being released, the canon engine reaching beyond CLAUDE.md, and a `set -e` abort in the stamper I had introduced an hour earlier. Standing lesson: for tooling that only runs at release or upgrade time, the dry-run path and the real path diverge, so test the real one in a sacrificial copy.
- (2026-07-29) Issues 0005-0008 fixed under the issues (no ST), per the standing hv ruling. Four judgement calls made under "fix it all", each reversible and each recorded in the issue's own Resolutions: **(1)** markdown emphasis on an AT status is NOT tolerated on read -- vocabulary is documented in every contract's preamble, and tolerance would have to swallow trailing prose too, with no principled stopping point; the fix is the diagnostic, per 0007's own caution that leniency without one just moves the silence. **(2)** The new field diagnostics WARN and do not block -- unlike a malformed id (which vanishes from the count and lets the gate go vacuous, hence F1's block), an unreadable status or unclosed marker leaves its AC uncredited, so the gate already failed closed; it failed closed silently, and the voice is the whole fix. **(3)** 0005 was widened to an upsert -- as specified the fix would have healed nothing, because the entry-writer skipped any language already present, so every affected project would have kept the stale text for good. **(4)** 0008's structural half (probes vs declared languages) filed as 0009 rather than folded in.
- (2026-07-29) The 0005 minor spacing item does NOT reproduce from the tool (three inits, re-runs, remove/re-add: no blank lines emitted). Most likely a markdown formatter on save in Lamplight. Left unfixed and said so, rather than inventing a fix for a defect that cannot be reproduced -- same discipline as 0004 item 4.
- (2026-07-24) Issue 0004 fixed under the issue (no ST), per the standing hv ruling from 0002/0003. Resolution design: target resolution is a distinct FAILABLE step ahead of evaluation, shared by the whole ac/at family; the gate announces every verdict including PASS, because silence-on-success is what made the vacuous passes invisible. hv's fix item 4 (non-zero exit on a BLOCKED `ac status`) deliberately NOT actioned -- premise does not reproduce; `ac status` is the reporter (verdict on stdout, exit 0), `ac gate` is the gate (verdict in `$?`). Left for an hv ruling as its own issue.
- (2026-07-13) v2.17.2 SHIPPED: issues 0002 + 0003 fixed + closed; 0003 gate design = defer to `intent critic` exit code (prose no-op), not a `--languages` skip. Detail: `done.md` + `.history/20260713/`.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed; fleet normalised. Detail: `.history/20260710/`.
