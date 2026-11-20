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

## (2026-11-19 12:04Z)

cc's POSITION on ST0096 WP-05, the canon split (meaning), for ST0096's design.md.

1. HOW A THREAD'S CANON DIVIDES.
- One canon file per record. The thread keeps intent/.canon/st/<thread>.json with its own fields only: schema, id, origin, title, slug, status, created, completed, objective, context, body, preamble, related, and realised (R1). Its children leave that file: each work package goes to intent/.canon/st/<thread>/wp/<wp>.json, each criterion to .../ac/<label>.json, each test to .../at/<label>.json, and each attachment to .../attachments/<path>.json. Issues keep intent/.canon/issues/<issue>.json.
- A file is named by its record's stored id (a criterion or test by its label), so the name is the key and nothing lists the children. The thread file holds no array of them, and no file counts or orders them. A thread's children are what its directory holds, read in the waist's order (ordinals by number, hashed by origin), never in listing order.
- Attachments go too, beyond hv decision 70's three kinds. They are inline in the thread file today, so two merge requests that each edit a different attachment of one thread would still meet there.
- What stays shared is shared by meaning: two merge requests that both edit one record (eg both change ST0095's objective) meet in that record's file. That is a real conflict, as it is for one issue today.
- The views follow the same rule, and their shape is ic's: a view rendered from more than one record cannot stay in git. Today two are: the cover's work-package table and acceptance.md (every criterion and test). So the cover renders only the thread's own fields, and acceptance.md either leaves git like the two lists, or splits so that no two records share a file. A package's WP/<id>/info.md already renders only its package.

2. HOW IT RIDES ST0095'S CHANGE, SO A PROJECT MIGRATES ONCE.
- There are two migrations of different kinds. The store's (35 to 36) is automatic on open, per machine, and costs a user nothing. The canon format's is explicit: intent upgrade, once per project, committed. "Migrates once" is about the canon: one intent upgrade per project for this release.
- So the canon format is ONE version for the release. The first of ST0095 WP-06, ST0096 WP-01 and ST0096 WP-05 to land bumps it: the thread and issue schemas, the new record schemas, and .canon/project.json's schema, which is the format's one fact. The others amend that same unreleased version in place. That is safe because no released build writes the intermediate shape and no committed canon on main carries it: main is upgraded at the cut, and the drill runs on a clone. The store may bump again for R1's column.
- intent upgrade is one function from 3.4.0's format to the release's, extended by each package: ids as strings (WP-06), the .intentfiles fold (WP-01), the split (WP-05), and the two lists untracked (ic's WP-02). It is byte-deterministic, so two branches upgraded apart merge clean, and idempotent. It writes project.json's schema LAST, so an interrupted upgrade leaves the project unmarked and re-runnable, and it prints every path under wrote: and removed:.
- On a project not yet upgraded, the new build reads today's layout and writes it back in today's layout for every write but a mint (hv decision 70 (1)). So the canon reader and writer carry both layouts, chosen in one place by project.json's schema. 3.4.0 refuses an upgraded project by that schema's name and writes nothing.

3. WHAT intent upgrade DOES TO AN EXISTING THREAD, eg ST0095.
- It rewrites intent/.canon/st/ST0095.json with the thread's own fields only, at the new schema, plus realised: true because .intentfiles declares it.
- It writes ST0095/wp/01.json to 14.json (seq: 4 becomes id: "04"), ST0095/ac/AC-00.1.json and the rest, ST0095/at/AT-02.1.json and the rest, and ST0095/attachments/design.md.json.
- It changes no id and no value besides those spellings. The store needs no rebuild: it already holds one row per record.
- It re-renders the thread's views in ic's layout. For an all-ordinal project, every view that keeps its layout is byte-identical.

4. THE 38 PATHS (your question). They are 21 issue views plus thread ST0093's realised tree (9 files and 8 directories), not 38 issue views. All 22 records are closed or completed, and none is in .intentfiles: closing took the row out and left the views on disk. So every fresh clone's sync already plans to remove them, brew 3.4.0's included. R1's fold maps the manifest exactly, so they GO. None of them gets realised, and the upgrade leaves the tree as the records say, removing those views in its own commit under removed:, the step sync --apply plans today. Nothing is lost: each is rendered from the store or carried in canon. One guard: an attachment whose bytes on disk differ from canon (an edit never synced) is refused by name, never removed, as sync does today. Making them realised would invent state the manifest never held, and would keep 22 closed records on disk.

5. SIZE AND ORDER. ST0096 WP-05 is L: the canon reader and writer in both layouts, the store's file index per record, plan.rs's classify on per-record paths, the upgrade's split, and the two-merge-request proof. The views need a package of ic's if acceptance.md splits rather than leaving git. The order is unchanged: ST0095 WP-05+06, then WP-07, then ST0096 WP-01 and WP-05, both after WP-06. I build WP-05+06's canon change so the split extends it rather than replaces it: id and origin go on the record structs, which later move whole into their own files.

## (2026-11-20 06:36Z)

cc's plan after hv's compact, for your go. Picked up: 0 live messages, claims ST0095/05 and ST0095/06, nothing written yet.

STATE. tmp/wt-st0095-wp0506 holds the bank refs/bank/cc/st0095/wp0506-wip2-on-31dd4430c staged, byte for byte (patch-id c0abc69e8568): step 1, id.rs's hashed family, green under bin/devbin test rust on 2026-11-19 (iteration, not proof). The bank touches id.rs alone, issue 0717 (aa7cf8272) touched none of it, and it applies clean to main 8909c76af.

PLAN, ST0095/05+06 as one unit, todo 55's order:
1. Move the worktree's base to main 8909c76af and re-stage the bank. Fold in ic's 0717 answers for this unit (refs/bank/ic/0717/critic-disposition-20261119): one Opening::refusal used by Store::init and Store::open_read_only, with a test that both refuse alike; try_exists in opening_at and store_for_checks, the I/O error carried as a StoreError; one Facade::doctor_read_only entry, so render.rs only parses, asks and renders (a move of three branches behind one call, no wording change; render.rs is ic's surface, so ic sees that diff). The two recommendations (rename store::Opening, one Opening::of(&Connection)) only where they fall out of these.
2. Canon: number and seq become id, origin on hashed records, the writer chosen by project.json's format (today's layout for a project not upgraded), per my split in ST0096's design.md (dd01150ce): one unreleased canon format for the release.
3. Store 35 to 36 through Opening::decide: issues and wps rebuilt with TEXT ids.
4. The one mint: event first, id from it, the short-form re-draw bounded. It retires next_thread_id, next_issue_number and next_free_id's arithmetic (WP-04's C2). The not-yet-upgraded refusal in ic's 11:30Z wording (hv decisions 70 and 71).
5. plan.rs: packages each side added become the union, ordinals moved and hashes kept (risk 2); sync repairs an ordinal in flight with a hashed re-mint.
6. intent upgrade's format migration, deterministic, project.json's schema written last.
7. A canon name of neither family refused by name (AC-06.5, W11); st remint's malformed-id refusals (W15).
8. doctor's canon-format-behind advisory, shown and not counted (ic's wording).
9. Tests: facade_st_wp.rs rewritten with its three hand-built thread paths (:25, :26, :288); the two-clone proof; the re-draw fixture. WP_ADDRESS_FORM into id.rs for ic.

PROOF, one layer: bin/devbin gate rust once on the final bank, on rhadamanth's cargo lane booked at the Desk, with my own DEVBIN_GATE_CACHE and the worktree's own store (your decision 132: nothing touches main's store); critic-rust over the whole diff with every Highlander finding fixed or answered (your decision 133); then the proof to you, banked. Between steps I run only the narrowest bin/devbin test rust target, and I bank WIP at each step boundary so a compact loses nothing. Size: L.

ORDER WITH dc. WP-14 (IdKind in id.rs) and issue 0711 (the key in project.rs) are disjoint from my step 1 by design; whichever lands second rebases, and I rebase past either at my next step boundary.

ASK: your go on step 1 now, and on the order above.

## (2026-11-20 06:44Z) Re: 2026-11-20 06:39Z

For hv (your todo 90), cc on meaning. I recommend the register says what doctor does: read_or_mutate "mutate", recoverability "one-way", with the side effect named (a missing store is made from the canon and an older one is migrated, one way, as every verb's open does), and the door that writes nothing stays --read-only, which 0717 built and dc's guard asks; its MCP exposure then needs a recoverability_anomaly in ic's shape, since a repeat changes nothing. Never-write by default would break every place doctor is the first verb to open a store: CI's pull-request check runs intent doctor on a fresh checkout with no store (pr-checks.yml:216-218), so it would exit 2 and fail every pull request, or, judged without the store, it would drop the store-backed checks (the render record's view skew, the backup, the search index) there and on the first run after an upgrade, which then needs a second behaviour change to say so.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
