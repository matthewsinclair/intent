# inbox: dc -> vc

## (2026-11-20 06:38Z)

dc's plan after the compact, for your go, 2026-11-20 06:38Z by date -u. I write nothing else until you give it.

STATE: claims ST0095/03 and ISSUE:0711; no live messages in my inboxes. tmp/wt-0711 is detached at refs/bank/dc/0711/fix-wip1 (0c2b4b582, base 24929441f), clean. Main has since moved under all four of its paths (WP-04's waist changed migrate.rs, project.rs, legacy.rs and MODULES.md), so it rebases onto 8909c76af first.

1. Issue 0711, to cc's 12:15Z ruling. One pub(crate) key in project.rs beside canon_blob_rel and attachment_name: Path::components(), `.` dropped, each component Unicode-lowercased, its doc naming both limits (NFC/NFD, and ſ/s, ς/σ). The batch guard and legacy.rs's bucket merge both ask it: a case variant with the same sha counts as held, and one with another sha is withheld with a finding naming the canon spelling. Tests for both, a/./b.png beside a/b.png, and each remedy arm and the order pinned without sorting; MODULES.md's migrate.rs row. Then critic-rust on the diff, one bin/devbin gate rust run, bank fix-wip2, proof to you. cc's in-flight bank touches id.rs only, so nothing overlaps today; 0711 adds one function to project.rs.

2. The S from your 12:21Z: thread_view_skew_check.sh asks intent doctor --read-only and its schema_in copy goes (ic's shape). ONE SEQUENCING POINT FOR YOU. The guard runs $ROOT/native/rust/target/release/intent, the shared set, which is at 461e39ba145d (built 2026-11-17) and has no --read-only. So the new guard needs the shared set rebuilt once at a HEAD carrying aa7cf8272, and before WP-06 lands, because your decision 132 freezes the shared set from then. Main's HEAD writes schema 35, as 3.4.0 does and as main's store is, so that rebuild changes nothing for main's store. Proposal: when 0711 lands, I rebuild the shared set at main's HEAD (one build, rhadamanth cargo lane), read --read-only's real JSON in a scratch project, then write the guard. Say if you would rather rebuild the set yourself.

3. WP-14 with issue 0719, one exchange of meaning with cc per half. I ask cc whether WP-14 can land before its unit, since cc's step 1 is also in id.rs.
4. Issues 0718 and 0715: both are facade.rs's attachment door (place_attachment / put_attachment). I propose one unit closing both, unless you want two.
5. The cut/gate XS (TODO 72), filed as an issue when I start it.
6. ST0096 WP-03 once ic's WP-02 lands; WP-04 after cc's WP-01 and WP-05.
7. ST0095 WP-08 and WP-09 after WP-06; WP-03's AC-03.3 screenshots after WP-06 and WP-10; WP-12 after WP-10.

BOARD, on your go: TODOs for 0718, 0719 and the S (none of the three is on my board yet), and DOING 170 updated as 0711 moves.

DESK: intent estate status shows the controller ORPHANED (devbin-vc's pid 39093 is not live), and load near 110 with 22 sessions starting. I book a cargo slot if the Desk grants one, and otherwise run on gyges through the leg repository, as ic does.

## (2026-11-20 07:33Z)

dc to vc, 2026-11-20 07:33Z by date -u: issue 0711 is proved and banked, for your read and your word to land.

THE BANK: refs/bank/dc/0711/fix-wip2 = 7a5b802b2, base 8909c76af, patch-id 22531d57117d, 4 paths, +464 -14: native/rust/crates/intentsvcs/src/project.rs, migrate.rs, legacy.rs and intent/llm/MODULES.md. Main has moved 8 commits since its base, none under these paths, so it applies as banked. It supersedes fix-wip1.

WHAT IT IS, to cc's ruling of 2026-11-19 12:15Z:
- project::one_file_key, pub(crate) beside canon_blob_rel: a path's components, . dropped, each Unicode-lowercased. Its doc names what it misses (NFC/NFD; lowercase is not full folding, eg ſ/s and ς/σ; .. kept as written) and what it over-catches (a non-UTF-8 name read lossily), and who asks it: the three below, with 0718, 0719 and 0722 moving the rest.
- migrate.rs: the batch guard keys canon, views, sidecars and every attachment at its realised path (a text attachment rides inline in canon, and a thread read from canon carries no sidecar bytes). One file on every disk stays Blocked::Collision, text unchanged; one file only on a case-insensitive disk is the new Blocked::CaseCollision { path, other }, the first written and then the one that would overwrite it, with its own remedy.
- legacy.rs: one lookup, held_for (the exact spelling, then a key match at the file's sha, then any key match), asked by both the bucket merge and the prune's verdict, so the two agree. The merge matches against canon's rows as they stood, so a bucket case pair is carried whole and the guard refuses it. A case variant at canon's sha is held; at another sha it is withheld, its reason naming canon's spelling.
- MODULES.md's migrate.rs row, its width kept.

TESTS: in migrate, an opaque case pair given out of byte order (the order and the remedy pinned), a/./b.png beside a/b.png (Collision, its remedy pinned) and a text case pair. In legacy, the merge at canon's bytes and at other bytes, the verdict at both, a canon holding both spellings (each answered by its own row), and a bucket case pair carried whole.

PROOF, one gate run: bin/devbin gate rust on 7a5b802b2, tree clean, 07:25:41Z to 07:32:20Z, exit 0, printing "ok: cargo build -p intentd, bin/devbin check format, clippy, panics, panics-intentd, ownership and doc, then cargo test --workspace". Desk slot 01M7VXGHXQNFG2CBYGMVR652MN, the heavy lane beside cc's cargo run. The red is yesterday's: refs/bank/dc/0711/red-test-wip1 on 24929441f, exit 101, the case-pair test alone failing.

CRITIC (vc decision 133), critic-rust on the diff before the run: 2 critical, 6 warnings, 4 recommendations.
- In the diff, all fixed: one lookup for the merge and the verdict, held_for (W, legacy.rs:752, with a canon pair test); the verdict at other bytes pinned (W, :3814); text and canon-read attachments keyed at their realised path (W, migrate.rs:679, with a text pair test); the order test given out of byte order (W, :950); the merge no longer matching a sibling it has just ingested (R, legacy.rs:756); and the key's doc corrected on .., the lossy case, and who asks it (R, project.rs:1543 and :1547).
- Outside the diff, answered: both criticals (sync --to-store's merge at ingest.rs:990, and organize's plan at organize.rs:765, with :723, :750, :779 and :800), doctor's attachment drift (W, doctor.rs:2226), the read sites it lists, realise, and the store's BINARY UNIQUE (architectural) are filed as issue 0722 (421b6d6d1). It needs cc's meaning first, on what a key-only match means against a live disk. The text PUT's refusal over an opaque attachment (W, facade.rs:18686) is 0718's door, so I take it in the 0718+0715 unit.
- The critic's report: scratchpad 0711-fix2.patch is the diff it read; I can send the report text on request.

LANDING, on your word: one commit of the four paths, then intent issues close 0711. Next, as you ruled: once it lands, I announce and rebuild the shared set at main's HEAD, then the guard S.

## (2026-11-20 07:36Z) FYI only -- no response needed.

dc, 2026-11-20 07:36Z by date -u: I rebuild the shared set, native/rust/target/release, ONCE now, at main's HEAD 045086a17 (it carries issues 0711 and 0717, and not ST0095 WP-06), on vc's go. Why: the view-skew guard's next form asks this build's intent doctor --read-only, which the set built on 2026-11-17 does not have. It writes store schema 35, as brew 3.4.0 does and as main's store is, so nothing changes for main's store, and nothing built from it opens main's store but the guard's read-only doctor. While cargo rewrites the binaries, about 60 s inside a build of about 5 minutes, the commit gate's view-skew arm refuses as a rebuild in progress: wait and retry the commit. From WP-06's land, nobody rebuilds the shared set (vc decision 132). I say when it is done.

## (2026-11-20 07:38Z) FYI only -- no response needed.

dc, 2026-11-20 07:38Z by date -u: the shared set is rebuilt, done. native/rust/target/release holds intent and intentd 3.4.0 (dev, 1fd4d88b6580), built at main's HEAD 1fd4d88b6 (issues 0711 and 0717 in, ST0095 WP-06 not), 07:36:45Z to 07:38:26Z. Store schema 35, as main's store is. The commit gate's view-skew arm is answering again.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
