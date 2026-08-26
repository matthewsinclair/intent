---
wp_id: WP-13
title: The v2 tree survives migration and disagrees with the store: ingest bucket files as attachments, then remove the bucket
scope: S
status: Not Started
---

# WP-13: The v2 tree survives migration and disagrees with the store: ingest bucket files as attachments, then remove the bucket

## Objective

**The migration leaves the v2 tree in place, and it disagrees with the store.** Devbin tracks 54 files under `intent/st/COMPLETED/` beside 31 in the flat v3 tree; both hold an `acceptance.md` for ST0001 and they now differ (the v3 view carries AC-10.5's evidence, the v2 copy does not). The v2 path is the one that existed for a year, so it answers greps first -- devbin/vc: every grep in a two-hour investigation hit the stale copy. And the store does NOT hold the bucket's content: on a copy of Devbin, `sync --to-disk ST0001` emitted nothing and canon's attachments are `[]`, while the bucket holds `design.md`, `impl.md`, `tasks.md`, `help-migration.md` (vc, measured 2026-08-26). `organize` already knows these paths as `unclaimed` (Devbin 54, Riffle 41, Baize 196). PROPOSED, for hv to rule: the migration ingests every unclaimed bucket file as an attachment of its thread (the naming gate permitting; the v2 `acceptance.md` preamble is template boilerplate plus any authored lines, into `preamble`), then removes the bucket under the fail-forward ruling of 2026-08-19 (git holds every historical ST); for the eleven already-migrated estates, `organize --apply` gains a prune of unclaimed bucket paths that does the same ingest-then-remove. The alternative hv named through devbin/vc is to MARK the v2 copy as stale so a reader knows which is live. vc's earlier `collapse-buckets.sh` deleted bucket files whose content canon did not carry and was halted for exactly this reason; the ingest step is what makes removal a move rather than a loss.

## The target layout of a v3 project's `intent/` (what "clean" means)

- Tool-written: `.config/`, `.canon/` (`st/`, `issues/`), `.intentfiles`, `st/<open threads>/` (realised views + attachments), `steel_threads.md`, `todo.md`, `llm/`.
- Authored: `docs/`, `wip.md`, `restart.md`, `whiteboard/`.
- Per-machine, ignored, never committed: `.cache/`, `events.jsonl`.
- NOT present: `st/COMPLETED|CANCELLED|NOT-STARTED/` (v2 buckets), `issues/OPEN|CLOSED/` (v2 issues; every record is in `.canon/issues/` with its body -- Devbin 34 of 34), `.treeindex` (retired by v3, still on every estate as an ignored leftover), stale `.backup/`.

## Measured across the fleet (2026-08-26)

All 19 migrated projects carry v2 st buckets (Baize 196 unclaimed paths, Devbin 54, Riffle 41); six carry v2 `issues/OPEN|CLOSED/` (arca_cli, Baize, Conflab, Devbin, Lamplight, Utilz); all 19 carry `.treeindex`. **Devbin, by devbin/vc's content probe with a positive control: 24 of the 54 bucket files have no copy anywhere else** (ST0001: 33 bucket / 20 flat / 13 bucket-only; ST0002: 21 / 10 / 11), and canon carries no `attachments` key for either thread. **And the estate's own restart brief routes two of its five opening questions to bucket paths** (`intent/st/COMPLETED/ST0001/design.md`, `.../WP/{10,11}/*-equivalence.md`), both bucket-only -- a prune without ingest breaks the pointers that bootstrap every session there, and the other estates will each have their own.

## The predicate that survives (devbin/vc)

Every natural check is satisfiable by an ingest that did nothing: "the bucket is gone" (true after a prune that ingested nothing); "ingest returned ok" (`intent ingest --from-md` prints `ok:` at rc 0 while writing nothing -- issue 0097); "attachments is non-empty" (true if paths were ingested without content). The one that survives is a PER-FILE content probe: a distinctive phrase from EACH bucket file found in the store afterwards, and the same probe shown returning 0 for a phrase never ingested. Per file, because ST0001's `acceptance.md` migrated and its `design.md` did not, so a thread-level check passes on a half-migrated thread.

## Three layers, in order

1. Ingest: every bucket-only file becomes an attachment of its thread (naming gate permitting), verified per file as above. `organize` today reports `unclaimed` and `0 to prune` as separate columns -- unclaimed is deliberately NOT prunable; this WP changes what that column means, and that change is the risky part.
2. Prune: the buckets, the v2 issues dirs and `.treeindex` are removed -- at migration for new conversions, by `organize --apply` for the 19 already-migrated estates -- under the fail-forward rule (Intent prunes on migration; git holds history). `.intentfiles` is absent on every estate, so what "declared" resolves to when the manifest is absent decides what `--apply` does to 19 trees: WP-11's default must be written FIRST.
3. Pointers: a sparse tree has no path for a closed thread's `design.md`, so authored pointers into `intent/st/...` are wrong on every dehydrated estate whatever this WP does; the readable form is the `intent://` address (WP-07) or `intent edit <ID> <file>`, which realises on demand. The migration reports every authored file naming a bucket path; rewriting them is the estate's, with the report as the worklist.

Recommended sequencing: WP-11 in 3.0.1; this WP with ST0061 (dehydrate) and the preconditions-in-the-tool in the release after -- the ingest-then-prune is the part that must not be rushed into the same cut as the packaging fix.

## Acceptance

Acceptance Criteria for this work package are RENDERED into `ST0057/acceptance.md`, under the `WP-13` heading. THAT FILE IS A GENERATED VIEW -- a row authored there is discarded by the next sync. The contract is canon in the thread's model: change a state with the `intent ac` / `intent at` verbs, and mint or reword a row in `.canon/st/ST0057.json`, then `intent sync --to-store`. This cover never restates them.

---

_Generated by Intent v3.0.0 from `the thread canon`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
