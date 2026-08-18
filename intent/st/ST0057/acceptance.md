---
st_id: ST0057
title: Disk as a sparse projection of the store
---

# ST0057: Disk as a sparse projection of the store -- Acceptance

> Canonical acceptance contract. Acceptance Criteria (AC) are the ratified completeness boundary; Acceptance Tests (AT) are the small red-to-green tests that prove them.
>
> Done = every AC is covered by a GREEN AT, or (for a non-test AC) its named evidence is satisfied, AND the AC set is the ratified full boundary. Done is read from this map, never from a hand-ticked box.
>
> Test-backed satisfaction is COMPUTED from covering green ATs and never stored -- storing it would be double truth. An AC has four states, not two: beyond satisfied and unsatisfied, a requirement can be **descoped** to a named thread or **withdrawn** with its reason on the record. Both are non-blocking and both are reported separately, so a thread that descoped half its contract looks like one.

## Acceptance Criteria

### ST-level

- AC-00.1 **No dehydration path removes any file while any declared precondition is unmet, and the refusal names every unmet one.** The set of preconditions the gate enforces is read from the SAME single declaration `design.md`'s list renders from, so the two cannot disagree -- a gate that checks a hand-copied subset of its own list is the failure this criterion exists to prevent. Denominator printed: preconditions checked against preconditions declared. -- satisfied: no (computed)
- AC-00.2 **`THREAD_PROSE` names no filename in the classifier, and every `design.md`, `impl.md` and `tasks.md` under `intent/st/` is carried as an attachment** (D57-6). Property, not count: for every file matching the three names, the artefact is present in canon and its bytes round-trip. The enumerated total is printed as the denominator, so a carry that covers some is distinguishable from one that covers all. -- satisfied: no (computed)
- AC-00.3 (non-test) **The conservation verdict is green at full scope over a pinned subject, and the finding names the revision it was measured at** -- `HEAD` is a pointer and a conservation figure that does not name its subject cannot be re-checked. -- satisfied: no
- AC-00.4 **`ROOT_FILES` (`AGENTS.md`, `CLAUDE.md`, `usage-rules.md`) have a v3 generator before anyone reasons about them as derivable.** They are on disk, they are outside `.intentfiles` scope, and something emptied `AGENTS.md` on 2026-08-18 -- so their derivability is currently an assumption rather than a mechanism. -- satisfied: no (computed)

### WP-01 -- Canon relocation to intent/.canon/ (status: Not Started)

- AC-01.1 **Canon for every thread and every issue resolves under `intent/.canon/`, one file per artefact, and `intent/st/` holds no `thread.json`.** Both populations are printed and the canon count equals the artefact count -- the criterion is the equality, not either figure. -- satisfied: no (computed)
- AC-01.2 **No path under `intent/.canon/` is matched by any ignore rule, and a fresh clone contains canon for every artefact the source tree has** (D29). This is the criterion that makes a clone complete; it is checked by cloning, never by reading `.gitignore`, because the question is what git DOES and not what a rule appears to say. -- satisfied: no (computed)
- AC-01.3 **The relocation is lossless: every field of every artefact round-trips through the move byte-identically**, denominator printed over the full field set. This is ST0056's AC-02.6 applied to the move -- a field that does not survive the relocation is data loss at the clone boundary. -- satisfied: no (computed)
- AC-01.4 **Two threads edited concurrently produce diffs in two distinct files**, which is the property that rejected the consolidated `threads.jsonl` (D57-1 option B). Checked by editing two threads and observing the changed-path set, not by inspecting the layout. -- satisfied: no (computed)

### WP-02 -- .intentfiles: the manifest and its refusing grammar (status: Not Started)

- AC-02.1 **The grammar REFUSES rather than skips.** The parser accepts exactly `<SIGIL>:<ID>` with sigil in `STEELTHREAD | ISSUE` and an optional trailing comment. For every rejected input the run exits non-zero AND the offending line number appears in the output. No input is silently skipped -- a skipped line drops an artefact from realisation and leaves an estate indistinguishable from one that never listed it. -- satisfied: no (computed)
- AC-02.2 **Lines outside the BEGIN/END markers survive an `organize` rewrite byte for byte**, while the generated region is rewritten from status. Checked with a pin present and the generated region changing in the same run. -- satisfied: no (computed)
- AC-02.3 **A pin to a thread that later closes still realises that thread after `organize`.** This is the two-region design's whole reason: without it the pin disappears with the files and nothing in the output names the decision. -- satisfied: no (computed)
- AC-02.4 **`.intentfiles` is tracked, and a realisation change lands in the working tree together with the manifest line that caused it** -- the reviewability property the committed-manifest ruling rests on. Checked by observing the changed-path set after an `organize` that changes realisation. -- satisfied: no (computed)

### WP-03 -- Attachment canon: opaque as file, and the naming gate (status: Not Started)

- AC-03.1 **An opaque attachment's bytes live in canon as a file at `intent/.canon/st/<ID>/<path>`, and hydration reproduces the working copy byte-identically from it.** Denominator printed over every opaque attachment in the estate. Until this holds an opaque attachment cannot be dehydrated at all, because there is no source beside it in the commit. -- satisfied: no (computed)
- AC-03.2 **Form follows content, both ways: no opaque attachment is stored inline and no text attachment is stored as a sibling file.** One rule, checked in both directions -- a check that only catches one direction passes an estate that has drifted the other way. -- satisfied: no (computed)
- AC-03.3 **The naming gate rejects, at ingest, any attachment path that cannot be given both a canon path and a URL -- and rejection is NOT retroactive.** An existing violator (the estate already carries a `.webloc` with spaces in its name) is reported by `organize` as UNCLAIMED under its fifth row and never silently removed. -- satisfied: no (computed)
- AC-03.4 **`sha256` in canon detects a working-copy edit by COMPARISON rather than inference**: an attachment modified on disk without the store knowing is reported by `doctor` as divergence, and the report names the path. -- satisfied: no (computed)

### WP-04 -- intent organize: four answers, one refusal, one gate (status: Not Started)

- AC-04.1 **`organize` implements exactly the five rows of D57-3's table and every row is exercised**, with the fifth -- a path the renderer does not produce -- REPORTED and never removed. `organize` must not be the thing that decides an unrecognised file is rubbish. -- satisfied: no (computed)
- AC-04.2 **The dehydration gate re-renders each view into memory, compares to the bytes on disk, refuses on ANY difference, and names the path.** A view carrying a hand edit is never removed. Fail-safe by construction rather than by discipline: one render and one comparison per file, on the removal path itself. -- satisfied: no (computed)
- AC-04.3 **`organize` NEVER resolves an attachment divergence: it reports the path, names both verbs, and modifies neither side.** Authority follows authorship -- a view diverging means the file is stale, an attachment diverging means the STORE is stale, and one policy for both discards somebody's work. -- satisfied: no (computed)
- AC-04.4 **`organize` run twice changes nothing, INCLUDING mtimes**: the second run writes zero files. Measured as the count of files whose mtime moved, which must be zero -- not as a content diff, because the defect being closed is a byte-identical re-emission that moves mtime and corrupts `file_index`'s clean/changed state. -- satisfied: no (computed)
- AC-04.5 **`organize` refuses when the tree digest computed immediately before the irreversible step differs from the one it measured, and the refusal names the difference.** It locks against ANY process, not other `organize` runs -- every read verb materialises the store on access, so "the estate was quiet when I measured" is not establishable over any window. -- satisfied: no (computed)

### WP-05 -- intent edit <ID> (status: Not Started)

- AC-05.1 **`intent edit <ID>` dispatches on id shape, hydrates when absent, and prints a path that EXISTS after the call** -- for every id shape the verb accepts, denominator printed over that set. -- satisfied: no (computed)
- AC-05.2 **A hand realisation writes to the PINNED region and survives the next `organize`.** Writing it to the generated region means the next run reverts it, because that region is a function of status and the artefact opened by hand is typically closed. -- satisfied: no (computed)
- AC-05.3 (non-test) **Path-printing has ONE home: `intent edit` is `intent st edit`'s behaviour learning to hydrate first, not a second implementation of it** (Highlander). `wip` is not used as the verb -- it already means a WP status, a thread status, `intent/wip.md`, and every node's board. -- satisfied: no

### WP-06 -- The full text realisation, and the refusal that expires (status: Not Started)

- AC-06.1 **The realisation is complete and SAYS SO WITH A DENOMINATOR**: every thread, WP, issue, attachment and view appears, with the printed count equal to the canon totals. A partial export that reads as complete is worse than no export, so the denominator is part of the criterion rather than a nicety of the output. -- satisfied: no (computed)
- AC-06.2 **It is regenerable and never authoritative**: written under `.backup/text/<UTC>/`, with no import path and `classify` never seeing a path under it. Checked by asserting the absence of a read-back route, not by intending one. -- satisfied: no (computed)
- AC-06.3 **`export --format md` is accepted, and its refusal is removed in the SAME change that makes the refusal's premise false -- not before.** The refusal (`export.rs:192`, `:208`) is correct while the views are in the tree and becomes false the moment sparseness lands. Retire a refusal with the change that expires its reason. -- satisfied: no (computed)
- AC-06.4 **`intent init` creates a working project from an empty directory, and the text realisation is exercised end-to-end from one.** `init` is currently unimplemented, and this is a PRECONDITION of the assurance rather than a neighbouring gap: you cannot demonstrate a fallback from a clean directory if you cannot create a clean directory. -- satisfied: no (computed)

### WP-07 -- intent:// addressing and read resolution (status: Not Started)

- AC-07.1 **Every entity form in D57-8's list resolves by address, and resolution is implemented ONCE**: the CLI and intentd call the same function in `intentsvcs`. Two resolvers is the failure -- they agree exactly until one moves, with nothing watching. Checked by asserting a single implementation, not by comparing two outputs. -- satisfied: no (computed)
- AC-07.2 **Views get NO address.** A reference to a view resolves to its entity or is rejected; no path segment names a view. This is what stops the scheme becoming a path alias and re-creating, inside the scheme, the conditionality it exists to remove. -- satisfied: no (computed)
- AC-07.3 **`?format=` accepts exactly `json` and `md` and nothing else, and an entity with more than one rendering is split into distinct addresses rather than gaining a `?view=`.** If an entity has more than one rendering it is UNDER-ADDRESSED. -- satisfied: no (computed)
- AC-07.4 **`?format=md` serves `View.content` from `views::render_all`, so the served bytes EQUAL the file `organize` would hydrate by construction rather than by test** -- `View.path` is literally where it would land. One renderer, three jobs. -- satisfied: no (computed)
- AC-07.5 **NO DAEMON IS REQUIRED TO READ YOUR OWN PROJECT.** With intentd stopped, uninstalled and never started, every address in a FULLY DEHYDRATED estate resolves through the CLI. If reading a dehydrated thread came to need a running daemon, the disk model would have made the estate less accessible than a pile of markdown -- which inverts D57-5's whole reason for existing. -- satisfied: no (computed)
- AC-07.6 **Empty authority means THIS project**, and a cross-project reference carries the slug and resolves against intentd's project registry. A reference that hard-codes the project name breaks on rename or fork, so the empty form is the one intra-project prose must use. -- satisfied: no (computed)

### WP-08 -- The mutation surface: write-by-address and the missing verbs (status: Not Started)

- AC-08.1 **The write path is DB first, canon ALWAYS, views IF MARKED.** After any write, canon contains the change whether or not the artefact is realised. Not "disk if marked" -- that collapses canon into views and leaves a dehydrated artefact inside a gitignored database, absent from a fresh clone. -- satisfied: no (computed)
- AC-08.2 **`GET ?format=json`, modify, `PUT` the same shape back is lossless for every field of every entity** -- the interchange format IS the mutation format, denominator printed over the field set. This gives ST0056's AC-02.6 its second job: a field that does not round-trip is now a field that cannot be WRITTEN. -- satisfied: no (computed)
- AC-08.3 **`PUT` accepts json only and rejects md -- except for attachments, where text-in is correct.** The exception is not an exception: an attachment is authored on disk, so authority runs the other way. Authorship decides direction, and `Project::classify` is the single answer to what a file is. -- satisfied: no (computed)
- AC-08.4 **Caller-assigned ids are a `PUT` to the entity address; server-assigned ids are a `POST` to the COLLECTION address returning the new address.** You cannot address `ST0058` before the tool has decided it is `ST0058`. -- satisfied: no (computed)
- AC-08.5 **Every writable field of every entity is settable through the mutation surface, and a field that cannot be written is reported BY NAME.** The burning case is ST0011's `completed`, which is NULL on the estate's one genuinely wrong row and has no field-setter verb today -- the criterion is the completeness of the surface, with the unsettable set as the printed output. -- satisfied: no (computed)

## Acceptance Tests

### ST-level

- AT-00.1 `intent/st/ST0057/parity/tools/dehydration_gate_check.sh` -- covers AC-00.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-00.2 `native/rust/crates/intentsvcs/tests/thread_prose_carried.rs` -- covers AC-00.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-00.3 (non-test) `conservation_check.sh` run at full scope over a named revision; vc reads the verdict and records the revision in the finding -- covers AC-00.3 -- status: n/a
- AT-00.4 `native/rust/crates/intentsvcs/tests/root_files_generated.rs` -- covers AC-00.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-01 -- Canon relocation to intent/.canon/ (status: Not Started)

- AT-01.1 `native/rust/crates/intentsvcs/tests/canon_relocation.rs` -- covers AC-01.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-01.2 `intent/st/ST0057/parity/tools/canon_clone_completeness.sh` -- covers AC-01.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-01.3 `native/rust/crates/intentsvcs/tests/canon_relocation_roundtrip.rs` -- covers AC-01.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-01.4 `intent/st/ST0057/parity/tools/canon_concurrent_diff.sh` -- covers AC-01.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-02 -- .intentfiles: the manifest and its refusing grammar (status: Not Started)

- AT-02.1 `native/rust/crates/intentsvcs/tests/intentfiles_grammar.rs` -- covers AC-02.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-02.2 `native/rust/crates/intentsvcs/tests/intentfiles_two_regions.rs` -- covers AC-02.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-02.3 `native/rust/crates/intentsvcs/tests/intentfiles_pin_survives_close.rs` -- covers AC-02.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-02.4 `intent/st/ST0057/parity/tools/intentfiles_reviewable.sh` -- covers AC-02.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-03 -- Attachment canon: opaque as file, and the naming gate (status: Not Started)

- AT-03.1 `native/rust/crates/intentsvcs/tests/opaque_attachment_canon.rs` -- covers AC-03.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-03.2 `native/rust/crates/intentsvcs/tests/attachment_form_by_content.rs` -- covers AC-03.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-03.3 `native/rust/crates/intentsvcs/tests/attachment_naming_gate.rs` -- covers AC-03.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-03.4 `native/rust/crates/intentsvcs/tests/attachment_drift_detected.rs` -- covers AC-03.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-04 -- intent organize: four answers, one refusal, one gate (status: Not Started)

- AT-04.1 `native/rust/crates/intentsvcs/tests/organize_five_rows.rs` -- covers AC-04.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-04.2 `native/rust/crates/intentsvcs/tests/organize_dehydration_gate.rs` -- covers AC-04.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-04.3 `native/rust/crates/intentsvcs/tests/organize_attachment_divergence.rs` -- covers AC-04.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-04.4 `native/rust/crates/intentsvcs/tests/organize_idempotent_mtime.rs` -- covers AC-04.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-04.5 `native/rust/crates/intentsvcs/tests/organize_moment_of_act_digest.rs` -- covers AC-04.5 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-05 -- intent edit <ID> (status: Not Started)

- AT-05.1 `native/rust/crates/intent-cli/tests/edit_hydrates.rs` -- covers AC-05.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-05.2 `native/rust/crates/intentsvcs/tests/edit_writes_pinned_region.rs` -- covers AC-05.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-05.3 (non-test) vc reads the dispatch and confirms a single path-printing implementation and no `wip` verb -- covers AC-05.3 -- status: n/a

### WP-06 -- The full text realisation, and the refusal that expires (status: Not Started)

- AT-06.1 `native/rust/crates/intentsvcs/tests/text_realisation_complete.rs` -- covers AC-06.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-06.2 `native/rust/crates/intentsvcs/tests/text_realisation_never_read_back.rs` -- covers AC-06.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-06.3 `native/rust/crates/intent-cli/tests/export_md_accepted.rs` -- covers AC-06.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-06.4 `native/rust/crates/intent-cli/tests/init_from_empty_dir.rs` -- covers AC-06.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-07 -- intent:// addressing and read resolution (status: Not Started)

- AT-07.1 `native/rust/crates/intentsvcs/tests/address_resolution_single_home.rs` -- covers AC-07.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-07.2 `native/rust/crates/intentsvcs/tests/address_views_have_no_url.rs` -- covers AC-07.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-07.3 `native/rust/crates/intentsvcs/tests/address_format_set.rs` -- covers AC-07.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-07.4 `native/rust/crates/intentsvcs/tests/address_md_is_the_hydrated_bytes.rs` -- covers AC-07.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-07.5 `intent/st/ST0057/parity/tools/no_daemon_required.sh` -- covers AC-07.5 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-07.6 `native/rust/crates/intentsvcs/tests/address_empty_authority.rs` -- covers AC-07.6 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

### WP-08 -- The mutation surface: write-by-address and the missing verbs (status: Not Started)

- AT-08.1 `native/rust/crates/intentsvcs/tests/write_path_canon_always.rs` -- covers AC-08.1 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-08.2 `native/rust/crates/intentsvcs/tests/mutation_roundtrip_complete.rs` -- covers AC-08.2 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-08.3 `native/rust/crates/intentsvcs/tests/mutation_put_format_by_authorship.rs` -- covers AC-08.3 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-08.4 `native/rust/crates/intentsvcs/tests/mutation_create_splits_two_ways.rs` -- covers AC-08.4 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.
- AT-08.5 `native/rust/crates/intentsvcs/tests/mutation_every_writable_field.rs` -- covers AC-08.5 -- status: to-write -- unwritten -- ST0057 is 0 of 8 WPs. `to-write` is the honest state: the test does not exist, which is not the same as existing and failing.

---

_Generated by Intent v3.0.0-dev from `thread.json`. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
