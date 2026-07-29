---
node: cc
name: Control Claude
role: control
session_id: dd8ff218-ab8e-4b00-8d30-0c8635f1d01d
heartbeat_at: 2026-07-29T22:40Z
status: active
focus: "Issues 0005 0006 0007 0008 all FIXED + CLOSED, staged as v2.17.4. Full unit suite 1105/1105 + both integration files green; critic clean. Uncommitted -- awaiting hv's go on commit + release."
claims: []
---

# Control Claude (cc)

## DOING

- **Four issues fixed + closed (2026-07-29), staged as v2.17.4 -- NOT yet committed.** hv's instruction was "fix it all"; work is complete and verified, waiting on hv for the commit + release step.
  - **0006 + 0007 (high, `bin/intent_acceptance`)** -- one root shape, both directions. New `extract_field` seam: match before substitute, empty + non-zero on non-match, so a failed parse stops returning the whole line as a plausible value. New `assert_written`: every mutation re-read and verified, so a writer can no longer report success having written nothing. `ac satisfy` made total over bare + tailed rows. New `warn_bad_fields` names out-of-vocabulary AT statuses and unclosed `(non-test` markers -- diagnostic only, no new block (they already fail closed; they failed closed silently). Also fixed `replace_line`'s unchecked copy-back, found by the new guard.
  - **0005 + 0008 (medium, generator-into-consumer)** -- Language Packs entry names `intent claude rules list --lang <lang>` instead of a path that only exists in the Intent install; entry-writer became an upsert so already-initialised projects heal on a re-run (they could not have, as specified); needle unified from three hard-coded copies into one. `agents sync` bash prerequisite gated on declared `languages` with no version floor; `has_project_language` new in `intent_helpers`.
  - **0009 filed** -- the structural point hv flagged inside 0008 (filesystem probes vs the declared `languages` array), deliberately NOT folded in: migrating all four probes changes every consumer's `AGENTS.md` and wants its own decision.

## TODO

- **hv go needed: commit + release.** Recommend one patch **v2.17.4** carrying all four (precedent: v2.17.2 = 0002 + 0003). Split by pair if you would rather ship the high-severity parser fix on its own. Nothing is committed yet -- on `main`, so waiting on your ask.
- **Consumer sweep after release (hv, separate repos):** re-run `intent agents sync` in Utilz / Lamplight / Baize (all three still carry `Bash 4.0+`), and `intent lang init <lang>` per declared language to heal the Language Packs entries. Lamplight + Baize will lose the shell prerequisite entirely unless they declare `shell` -- correct, neither is a shell project.
- **Lamplight contract sweep (hv, separate repo):** ST0276 (11 bolded `**green` rows), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`. The parser fix does not flip these -- emphasis is deliberately not tolerated -- but the tool now names every offending row on `ac list` / `ac gate`, so the sweep is mechanical.

- **hv ruling owed -- issue 0004 item 4 (`ac status` exit code).** hv asked for uniform non-zero on a BLOCKED `ac status`; premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if hv wants it changed.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` does tag/push/gh-release + CHANGELOG date + VERSION/AGENTS sidecar, but NOT the config.json `intent_version` bump -- manual post-tag wrap. Don't skip it. (It is interactive; if a `bin/release` run is interrupted mid-cut, finish the push/gh/wrap by hand -- it is idempotent on the tag.)
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-29) Issues 0005-0008 fixed under the issues (no ST), per the standing hv ruling. Four judgement calls made under "fix it all", each reversible and each recorded in the issue's own Resolutions: **(1)** markdown emphasis on an AT status is NOT tolerated on read -- vocabulary is documented in every contract's preamble, and tolerance would have to swallow trailing prose too, with no principled stopping point; the fix is the diagnostic, per 0007's own caution that leniency without one just moves the silence. **(2)** The new field diagnostics WARN and do not block -- unlike a malformed id (which vanishes from the count and lets the gate go vacuous, hence F1's block), an unreadable status or unclosed marker leaves its AC uncredited, so the gate already failed closed; it failed closed silently, and the voice is the whole fix. **(3)** 0005 was widened to an upsert -- as specified the fix would have healed nothing, because the entry-writer skipped any language already present, so every affected project would have kept the stale text for good. **(4)** 0008's structural half (probes vs declared languages) filed as 0009 rather than folded in.
- (2026-07-29) The 0005 minor spacing item does NOT reproduce from the tool (three inits, re-runs, remove/re-add: no blank lines emitted). Most likely a markdown formatter on save in Lamplight. Left unfixed and said so, rather than inventing a fix for a defect that cannot be reproduced -- same discipline as 0004 item 4.
- (2026-07-24) Issue 0004 fixed under the issue (no ST), per the standing hv ruling from 0002/0003. Resolution design: target resolution is a distinct FAILABLE step ahead of evaluation, shared by the whole ac/at family; the gate announces every verdict including PASS, because silence-on-success is what made the vacuous passes invisible. hv's fix item 4 (non-zero exit on a BLOCKED `ac status`) deliberately NOT actioned -- premise does not reproduce; `ac status` is the reporter (verdict on stdout, exit 0), `ac gate` is the gate (verdict in `$?`). Left for an hv ruling as its own issue.
- (2026-07-13) v2.17.2 SHIPPED: issues 0002 + 0003 fixed + closed; 0003 gate design = defer to `intent critic` exit code (prose no-op), not a `--languages` skip. Detail: `done.md` + `.history/20260713/`.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed; fleet normalised. Detail: `.history/20260710/`.
