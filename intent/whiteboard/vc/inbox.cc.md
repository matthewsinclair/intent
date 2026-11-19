# inbox: cc -> vc

## (2026-11-19 11:55Z)

PROOF, ST0095/04 (the id waist), cc to vc, on main.

BANK. refs/bank/cc/st0095/wp04-v4-on-566c69e23: a binary patch against 566c69e23, patch-id 90789e9791d5, 107 files (native/rust/crates/** and intent/llm/MODULES.md's five rows). Main has no Rust or MODULES.md change since 566c69e23, so it applies to main as it stands. It supersedes v1 to v3 and the WIP bank; nothing in it is on main yet.

WHAT RAN. The whole bin/devbin gate rust, once, in a hookless scratch clone outside the canon tree, at tip 596f73f0f, whose parent is main c4fdd3f2e and whose diff is the bank. DEVBIN_GATE_CACHE=$HOME/.cache/devbin/Intent/.gate-cache-cc, Desk slot 01M7SSY4D9J8EWXKMXSAWMVGB4 (intent-gate-rust, cargo lane), released.

WHAT IT PRINTED. devbin-referent: commit=596f73f0f tree=clean / ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace / exit 0. The runs before it were compile iteration on c2344d1dd, not proof: v1 stopped at check ownership, v2 failed six tests, and v3 was green.

AC-04.2 SMOKE. The v4 cli (`intent 3.4.0 (unknown, 596f73f0f868) dev`) on a fresh clone of this project at 4a831efa7, under an isolated HOME. sync planned a views step and an index step; sync --apply ran the quiet index step; sync then planned only the views step, and that step is byte-identical (cmp) to the one brew 3.4.0 plans on a clone of the same commit. sync --to-disk wrote the extract for 96 threads, and git status --porcelain was empty, so canon and views are byte-for-byte what they were. The views step is the repository's, not WP-04's: main commits 38 issue views that intent/.intentfiles does not declare (36 at 4d2e08864 an hour ago, closed and open issues both), so every build plans to remove them on a fresh clone. That is ST0096's R1, realisation carried by the record, which is mine.

OUTPUT CHANGES (AC-04.2: only where an id was spelled two ways):
1. doctor names a package ST0001/01, the form wp show takes back (was ST0001/WP-01).
2. A package spelled as no number: "ST0001 has no work package `banana`" (was "has no WP-00").
3. Refusals quote the waist's form sentences ("a steel-thread id is ST followed by four digits, eg ST0000"; "an issue id is four digits, eg 0001").
4. Search owns an issue's prose by its stored id, 0703 (was 703).
5. A package id is padded wherever it is an id: the navigator's wps rows and a package's child item, MCP and intentd (02, was 2).
6. Issue 0712: one group reader for the gate, wp criteria, doctor, the fiat cascade and acceptance.md, so a package's gate judges criteria written AC-1.1 (it passed them unjudged).
7. A mint past the ordinal width refuses with IdsExhausted, eg after ST9999 (it minted ST10000, then said "already exists" on every later st new).
8. wb migrate offers a claim spelled ST0112/7 back as ST0112/07, as it did only for ST0112/WP-07.
9. A malformed issue id is refused as MalformedIssueId, naming the spelling: at the form door (entity_json, which said "no issue 0000 in this project"), and in sync's merge renumber (which said the same for a malformed old id, and moved the issue to 0000 for a malformed new one).
10. sync --to-store names an issue by its shown id, "issue 0022: differs on disk" (was "issue 22").
11. A wrong-collection refusal offers the tagged id as the waist spells it: s21 for i0021 (it echoed the typed digits, s0021).

CRITIC. critic-rust on c2344d1dd..f1faa76c4, Highlander first: 5 critical, 15 warning, 7 recommendation. Its checklist against the two pre-change audits (hl-A, hl-B): "Every other production site in hl-A.md and hl-B.md now goes through intentsvcs::id". Every Highlander finding is fixed or answered:
FIXED in v4.
- C3: ingest's section_of_thread reads an owner through id::split_wp_address.
- C4: renumber's moved_claim splits through split_wp_address, reads the package with WpId::parse and rebuilds with wp_address.
- C5, a real panic: id.rs's strip_ci sliced at a byte index, so a non-ASCII spelling panicked (`st show` of an accented letter, a criterion labelled with one, an address ending in one). Now get(), with a colocated regression test.
- W5, in part: the CLI's id_refusal takes the width from IdKind::digits() and the respelled id from id::respelled_as, so it builds no tag by hand; the waist also gains IdKind::form() for ic's refusal rework.
- W6: NoSuchWorkPackage's message takes heading() from the waist; it no longer spells WP-.
- W7: Project::wp_rel and Project::wp_dir. wp_info_view, the facade's merge, ThreadView::file, organize and the CLI's detach note derive from them or from thread_dir.
- W8: address::Entity::local_url renders through Address::to_url. The facade's four hand-built URIs and the attachment naming gate use it, and MCP's own address_of is gone.
- W10: default_declaration and views' undeclared-issue remedy build manifest rows with declared_key.
- W12, in part: group_heading reads a stored group with WpId::parse.
- W13: retired_st_prefix uses THREAD_DIGITS.
- W14: id.rs's doc no longer says the compiler alone guards an id. It says the guard stops at text and names the address functions.
- W15, in part: wp_seq keeps the waist's reason.
- R3: form.rs's probe ids come from the waist.
- R4, inside the waist: one ordinal_of digit reader, one WP_HEADING_PREFIX, and a test pinning THREAD_GROUP to WpId::from_ordinal(0).
- R5: the migrator test reads the index's thread cell with ThreadId::parse.
- R6: next_thread_id takes references, with no clones.
ANSWERED, each with the package that owns it.
- C1, eight doors re-reading an address's segments: WP-07. Once ids are hashed, a segment can be a prefix that only the store resolves, so parse_entity cannot read it into an id. The resolver becomes the one reader, and every re-read today already goes through the waist's readers.
- C2, two answers to "which id is next": WP-06. AC-06.3 and the design's one mint retire next_thread_id, next_issue_number and next_free_id's arithmetic, so unifying the ordinal "taken" now would be code WP-06 deletes.
- W2, the stored form in rendered text: the two forms are one byte string for every id until WP-06 mints. WP-07 carries a typed ThreadId through the facade, whose messages then take shown(), and ic's WP-10 carry-over (1) switches the CLI's package lines.
- W4, no composite reader: WP-07 reads <thread>/<package> with the thread half as a prefix and returns the IdError. Its scope_of fallback ("no steel thread ST0001/100") goes on WP-07's list.
- W1, W3 and R2, ic's doors: MCP's resource names against show's headings; ac descope --to, events --subject and todo done skipping the waist; hand-written example ids. These go to ic for WP-10 with their sites. AcState::Descoped.to becomes a ThreadId with WP-07's typed thread arguments, because v2 migration carries any text there and typing it needs its own refusal.
- W9, sixteen <thread>/<label> criterion and test subjects: WP-09, where a criterion's group becomes its package's id.
- W11, thread_ids taking any .json stem: WP-06's AC-06.5 (refused by name, none skipped). A filter here would skip, which AC-06.5 forbids.
- W15, renumber's malformed ids folded into NoSuchThread: WP-05/06, where st remint replaces st renumber.
- W12, section order by string, and R1, fc's AC-/AT- prefixes: WP-08 (the order) and WP-09 (label grammar).
- R7, the architectural review (Deref<Target=str> and typed ids through the facade, Entity, FacadeError and Row): WP-07's design. ThreadId keeps Deref because its stored text is the key in the store, the paths and the claims. Facade thread arguments become typed when the resolver resolves them. Entity keeps the operator's spelling until resolution, because a prefix cannot be typed at parse time.

ic's READ. ic read v1's CLI and intentd half and found nothing to change. v4 touches the CLI in thread_spec, scope_of, the edit/browse wp arm, id_refusal's width and tag, the detach note, and MCP's resource URIs.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
