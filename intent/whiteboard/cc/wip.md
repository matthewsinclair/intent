---
node: cc
name: Control Claude
role: control
session_id: dd8ff218-ab8e-4b00-8d30-0c8635f1d01d
heartbeat_at: 2026-07-30T00:05Z
status: active
focus: "v2.17.4 SHIPPED (tag af13633, both remotes + GitHub release). Issues 0005-0008 closed; 0009 open. bin/release then fixed so no release needs a manual post-tag wrap again -- proved by cutting a test release in a clone. Suite 1127/1127."
claims: []
---

# Control Claude (cc)

## DOING

- **v2.17.4 SHIPPED (2026-07-30).** hv cut it; tag `af13633` on both remotes + GitHub release. Wrap `9650a5e` done by hand -- the last time that will be necessary.
- **`bin/release` fixed so the wrap is no longer a manual step (`2af02d7` + `fc89d3e`).** Every published tag was internally inconsistent: `VERSION` said the new version while `config.json` and `CLAUDE.md` said the old one, because the wrap commit landed AFTER the tag. Now all five sidecars are stamped before the commit, from one `SIDECAR_FILES` list that the detect step and the stage step both read (they disagreed before, so the script could announce a commit it had not made), and it refuses to tag if anything outside that list is left dirty. `config.json` goes through the new shared `stamp_project_version`; `CLAUDE.md` is delegated to the canon engine that owns it, so `bin/release` renders no template itself. Usage block now documents authoring the heading as `## [X.Y.Z] - in progress` -- a hand-typed date is right only on the day it is typed, and v2.17.4's went stale at midnight and aborted the pre-flight.
- **Two further defects found by actually cutting a test release in a clone**, neither visible to a dry run: an `INTENT_HOME` exported in the maintainer's shell silently beat the checkout being released, so a cut from any other checkout would stamp the wrong version into the generated files; and the canon engine also rewrote the `.claude/` stack, tripping the new dirty-tree guard. Pinned and scoped respectively.

- **Four issues fixed + closed (2026-07-29), shipped in v2.17.4.**
  - **0006 + 0007 (high, `bin/intent_acceptance`)** -- one root shape, both directions. New `extract_field` seam: match before substitute, empty + non-zero on non-match, so a failed parse stops returning the whole line as a plausible value. New `assert_written`: every mutation re-read and verified, so a writer can no longer report success having written nothing. `ac satisfy` made total over bare + tailed rows. New `warn_bad_fields` names out-of-vocabulary AT statuses and unclosed `(non-test` markers -- diagnostic only, no new block (they already fail closed; they failed closed silently). Also fixed `replace_line`'s unchecked copy-back, found by the new guard.
  - **0005 + 0008 (medium, generator-into-consumer)** -- Language Packs entry names `intent claude rules list --lang <lang>` instead of a path that only exists in the Intent install; entry-writer became an upsert so already-initialised projects heal on a re-run (they could not have, as specified); needle unified from three hard-coded copies into one. `agents sync` bash prerequisite gated on declared `languages` with no version floor; `has_project_language` new in `intent_helpers`.
  - **0009 filed** -- the structural point hv flagged inside 0008 (filesystem probes vs the declared `languages` array), deliberately NOT folded in: migrating all four probes changes every consumer's `AGENTS.md` and wants its own decision.

## TODO

- **UNPUSHED: the `bin/release` fix (`2af02d7`, `fc89d3e`) + wrap (`9650a5e`) are local-only on `main`.** They land in whatever ships next; v2.17.5 will be the first cut the fixed script performs end to end, which is also the only real proof it works in anger.
- **Consumer sweep after release (hv, separate repos):** re-run `intent agents sync` in Utilz / Lamplight / Baize (all three still carry `Bash 4.0+`), and `intent lang init <lang>` per declared language to heal the Language Packs entries. Lamplight + Baize will lose the shell prerequisite entirely unless they declare `shell` -- correct, neither is a shell project.
- **Lamplight contract sweep (hv, separate repo):** ST0276 (11 bolded `**green` rows), ST0298 `GREEN`, ST0270 `BOTH`, ST0198 `BUILT`. The parser fix does not flip these -- emphasis is deliberately not tolerated -- but the tool now names every offending row on `ac list` / `ac gate`, so the sweep is mechanical.

- **hv ruling owed -- issue 0004 item 4 (`ac status` exit code).** hv asked for uniform non-zero on a BLOCKED `ac status`; premise does not reproduce (exits 0 on both BLOCKED shapes; `intent_acceptance_cli.bats:111` asserts it). `status` = reporter (stdout), `gate` = gate (`$?`). Own issue if hv wants it changed.
- **Push fleet issue-normalisation commits (hv, separate repos):** Utilz (`0171297`) + Lamplight (`7058fd3a8`) local-only; Conflab pushed.
- Carry-over (hv, separate repo): utilz-side `generator: utilz todo` marker + symmetric guard.
- DEFERRED (needs hv ruling): AT-name traceability -- `acceptance.md` AT ids grep-able to bats `@test` names.

## Watch-outs

- `bin/release` now stamps ALL five sidecars (VERSION, CHANGELOG date, config.json `intent_version`, AGENTS.md, CLAUDE.md) before the commit -- there is no manual post-tag wrap any more. Author the CHANGELOG heading as `## [X.Y.Z] - in progress` and let the script date it. It is still interactive; if a run is interrupted mid-cut, finish the push/gh by hand -- it is idempotent on the tag. If it aborts with "refusing to tag a dirty tree", something outside the sidecar list changed: read the list it prints, do not just re-run.
- New command wiring: `bin/intent_<name>` auto-dispatches via the `*)` default case in `bin/intent`. Register in MODULES.md FIRST (Highlander); single template source under `lib/templates/`.

## Decisions

- (2026-07-29) Issues 0005-0008 fixed under the issues (no ST), per the standing hv ruling. Four judgement calls made under "fix it all", each reversible and each recorded in the issue's own Resolutions: **(1)** markdown emphasis on an AT status is NOT tolerated on read -- vocabulary is documented in every contract's preamble, and tolerance would have to swallow trailing prose too, with no principled stopping point; the fix is the diagnostic, per 0007's own caution that leniency without one just moves the silence. **(2)** The new field diagnostics WARN and do not block -- unlike a malformed id (which vanishes from the count and lets the gate go vacuous, hence F1's block), an unreadable status or unclosed marker leaves its AC uncredited, so the gate already failed closed; it failed closed silently, and the voice is the whole fix. **(3)** 0005 was widened to an upsert -- as specified the fix would have healed nothing, because the entry-writer skipped any language already present, so every affected project would have kept the stale text for good. **(4)** 0008's structural half (probes vs declared languages) filed as 0009 rather than folded in.
- (2026-07-29) The 0005 minor spacing item does NOT reproduce from the tool (three inits, re-runs, remove/re-add: no blank lines emitted). Most likely a markdown formatter on save in Lamplight. Left unfixed and said so, rather than inventing a fix for a defect that cannot be reproduced -- same discipline as 0004 item 4.
- (2026-07-24) Issue 0004 fixed under the issue (no ST), per the standing hv ruling from 0002/0003. Resolution design: target resolution is a distinct FAILABLE step ahead of evaluation, shared by the whole ac/at family; the gate announces every verdict including PASS, because silence-on-success is what made the vacuous passes invisible. hv's fix item 4 (non-zero exit on a BLOCKED `ac status`) deliberately NOT actioned -- premise does not reproduce; `ac status` is the reporter (verdict on stdout, exit 0), `ac gate` is the gate (verdict in `$?`). Left for an hv ruling as its own issue.
- (2026-07-13) v2.17.2 SHIPPED: issues 0002 + 0003 fixed + closed; 0003 gate design = defer to `intent critic` exit code (prose no-op), not a `--languages` skip. Detail: `done.md` + `.history/20260713/`.
- (2026-07-10) v2.17.1 SHIPPED: ST0055 `intent issues` closed; fleet normalised. Detail: `.history/20260710/`.
