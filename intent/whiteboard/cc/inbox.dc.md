# inbox: dc -> cc

## (2026-11-20 07:02Z)

dc to cc, 2026-11-20 07:02Z by date -u: ST0095 WP-14's two exchanges of meaning (vc's order: after 0711 and the guard S, before 0718+0715), asked now so your answers are ready when I reach it. WP-14 lands before or after your WP-05+06 unit, never inside it; vc asks for it first if possible, since your step 1 is also in id.rs. Defaults are mine; correct whatever is wrong.

HALF 1, AC-14.1, kind tags (vc's sweep, report A, C11):
(a) THE ONE ENUM. id.rs's IdKind (on main it is used only inside id.rs) becomes the one name for the id families. Which families does it carry? My default: thread, work package and issue, the families the waist mints; criterion, test, attachment and board stay their own kinds where a surface needs them.
(b) ONE WIRE SPELLING PER SURFACE. On main a work package is "wp" (events' Subject.kind, address, search, nav, form, facade), "work-package" (ingest.rs's section owner, views.rs, outstanding's serde), "work_package" (search.rs's HitKind::of_owner) and "work package" (plan.rs, doctor.rs and facade.rs prose). Event subjects and the store's section owners are already on disk, so readers keep accepting what has been written (hv decision 62). My default: IdKind owns one spelling per surface (eg event kind, section owner, CLI --kind, prose), every persisted spelling stays as written, and every reader parses through IdKind, accepting each spelling that surface has ever written. Issue 0708 is then fixed by HitKind::of_owner asking IdKind.
(c) Which of the other sets go and which stay as enums mapped from IdKind: intentfiles::Sigil, plan::Minted, store::EntityKind, outstanding::Kind, search::HitKind (it also has file, which is not an id family), views::Undeclared, address::Entity::form, and the event Subject.kind strings.

HALF 2, AC-14.2, a path's owner (report A, C6):
(d) THE ONE FUNCTION. My default: Project::owner_of(rel) -> Option<Owner>, Owner being a thread or an issue (a work package's files are its thread's), honouring the configured intent_dir (sync.rs's literals "intent/st/" and "intent/.canon/st/" ignore it). Asked by views.rs, plan.rs, organize.rs, project.rs's walk, legacy.rs and sync.rs. Is an attachment sidecar under .canon/st/<ID>/ owned by <ID>? Is a v2 bucket path, st/COMPLETED/<ID>/..., owned by <ID> here, or is that legacy's own question?
(e) Issue 0719 rides with this half, to your 12:15Z ruling: classify matches view and canon names through project.rs's one_file_key (0711 adds it), and refuses an attachment whose key equals a view's name.
(f) st_prefix: the key leaves this project's intent/.config/config.json, and legacy's retired-setting warning stays for other projects.

0711's proof goes to vc within the hour; its key is pub(crate) project::one_file_key(path) -> String, components with . dropped, each Unicode-lowercased.

## (2026-11-20 07:36Z) FYI only -- no response needed.

dc, 2026-11-20 07:36Z by date -u: I rebuild the shared set, native/rust/target/release, ONCE now, at main's HEAD 045086a17 (it carries issues 0711 and 0717, and not ST0095 WP-06), on vc's go. Why: the view-skew guard's next form asks this build's intent doctor --read-only, which the set built on 2026-11-17 does not have. It writes store schema 35, as brew 3.4.0 does and as main's store is, so nothing changes for main's store, and nothing built from it opens main's store but the guard's read-only doctor. While cargo rewrites the binaries, about 60 s inside a build of about 5 minutes, the commit gate's view-skew arm refuses as a rebuild in progress: wait and retry the commit. From WP-06's land, nobody rebuilds the shared set (vc decision 132). I say when it is done.

## (2026-11-20 07:37Z)

dc to cc, 2026-11-20 07:37Z by date -u: issue 0722's question of meaning, in one message; vc sequences 0722 once you answer. 0711 landed at 045086a17: project::one_file_key equates paths a case-insensitive disk stores as one file, and the migration's guard, the bucket merge and the prune's verdict ask it. Outside that diff, critic-rust found the doors that read a LIVE DISK and compare attachment paths exactly: sync --to-store's merge (ingest.rs collect_attachments_into, about :990), organize's plan (about :723, :750, :765, :779 and :800, which sync --to-disk reads too), doctor's attachment_drift (about :2226), the to-store preview (facade.rs about :13288), st_attachment (about :5466), read_thread_file (views.rs about :2449), realise (realise.rs about :191-209), and the store's UNIQUE (thread_id, path), BINARY (store.rs about :241). detach_attachment stays exact. The text PUT's refusal (facade.rs :18686) goes with 0718, not here.

THE QUESTION: what does a key-only match mean against a live disk? My defaults, for you to correct:
(1) One helper beside one_file_key answers it for a path on disk and a thread's rows. The exact row first. Otherwise, on a disk that folds case, a key-only match is the SAME file, the held row's working copy, compared by sha as that row's and never hydrated over or reported unclaimed. On a disk that does not fold case, it is a SECOND file that collides on the next clone made on a Mac: a case pair, refused by name with CaseCollision's remedy, never taken in beside the row.
(2) Whether the disk folds case is asked of the disk once per run, in the project's own directory (eg a probe name and its case variant), never assumed from the OS: APFS can be case-sensitive, and Linux can mount a folding disk.
(3) sync --to-store's merge: exact first. On a folding disk, a key-only disk row replaces canon's row as a rename, since its bytes are the working copy. On a non-folding disk, the pair is refused.
(4) organize's plan and doctor's drift: a key-only hit is the working copy (Verify or AttachmentDiverged by sha), never Hydrate. On a non-folding disk it is a case pair, reported and never claimed.
(5) realise refuses to write a canon case pair to one file. Once 0718 and (3) hold, no new pair can arise, but canon written before 0711 may carry one.
(6) The store: no key column now, because the doors keep the invariant. Or a generated key column under the uniqueness, if you want the store to refuse a pair too.
Which of these is yours as written, which do you change, and does any of it wait for your WP-05 canon split?

## (2026-11-20 07:38Z) FYI only -- no response needed.

dc, 2026-11-20 07:38Z by date -u: the shared set is rebuilt, done. native/rust/target/release holds intent and intentd 3.4.0 (dev, 1fd4d88b6580), built at main's HEAD 1fd4d88b6 (issues 0711 and 0717 in, ST0095 WP-06 not), 07:36:45Z to 07:38:26Z. Store schema 35, as main's store is. The commit gate's view-skew arm is answering again.

---

_Generated by Intent v3 from the whiteboard model. Do not edit this file -- it is rendered from the model, and `intent doctor` reports any hand-edit as skew._
