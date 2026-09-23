//! **0535: a fresh clone's store holds its whiteboard, and no remedy empties a
//! board.**
//!
//! A store that an Intent before 0535 warmed carried every thread and issue and
//! no board, because only an unscoped `sync --to-store` carried `board.json` in.
//! The remedies it then printed led, at rc 0, to an empty board written over
//! every node's file: `wb register` followed by `sync --to-disk`, or `wb
//! migrate`, which read the rendered `wip.md` as though it were hand-authored.
//! Driven on a clone of Intent's own estate, that deleted 10258 lines across
//! five boards.
//!
//! The fix has four parts, and each arm below pins one:
//! - (a) the cold warm carries the boards, and a store holding a board is warm;
//! - (b) every write that would render over such a file refuses first;
//! - (c) the reads name `sync --to-store`;
//! - (d) doctor names the state before anyone runs a write.

use crate::common::{Fixture, ctx, sample_thread, tree};
use intentsvcs::facade::FacadeError;
use intentsvcs::finding::FindingClass;
use intentsvcs::model::{Board, WbItemKind};
use intentsvcs::remedy::Remedy;
use intentsvcs::store::Store;
use intentsvcs::sync::Scope;

/// An estate with a thread and two migrated boards, as its committed extract
/// carries them: cc holds a live item, an archived one and a message from vc,
/// so its inbox view exists; vc holds one item.
fn an_estate_with_boards() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  f.wb_add("cc", WbItemKind::Doing, "carry the boards")
    .expect("add");
  f.wb_archive("cc", WbItemKind::Todo, 1).expect("archive");
  f.wb_add("vc", WbItemKind::Todo, "judge the run")
    .expect("add");
  f.wb_ask("vc", "cc", "go", None, false).expect("ask");
  fx
}

/// The boards as the estate's own store holds them.
fn boards_of(fx: &Fixture) -> Vec<Board> {
  fx.facade_on_disk().boards().expect("the estate's boards")
}

/// A clone whose store an Intent before 0535 warmed: every thread and issue the
/// extract carries, and no board. `Store::rebuild` is that warm's own write,
/// the rebuild that took threads and issues only.
fn a_clone_warmed_before_0535(fx: &Fixture) -> Fixture {
  let clone = fx.clone_extract();
  let canon = intentsvcs::ingest::read(&clone.project()).expect("the extract reads");
  Store::open(&clone.project().db_path())
    .expect("the clone's store")
    .rebuild(&canon.threads, &canon.issues)
    .expect("the older warm");
  clone
}

/// Every file under the whiteboard, by path and bytes.
fn whiteboard(fx: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
  tree(&fx.root().join("intent").join("whiteboard"))
}

fn lacking(err: &FacadeError) -> Option<&[String]> {
  match err {
    FacadeError::WbBoardsNotInTheStore { nodes } => Some(nodes),
    _ => None,
  }
}

#[test]
fn a_fresh_clone_opens_holding_every_board_its_extract_carries() {
  let fx = an_estate_with_boards();
  let clone = fx.clone_extract();
  assert!(
    !clone.project().db_path().exists(),
    "precondition: a clone carries no store"
  );

  let mut f = clone.facade_on_disk();
  assert_eq!(
    f.wb_status().expect("the clone's roster"),
    boards_of(&fx),
    "the first open took every board, archived items and messages included"
  );
  let files = whiteboard(&clone);
  f.sync_to_disk(&Scope::All)
    .expect("the warmed store egests");
  assert_eq!(
    whiteboard(&clone),
    files,
    "the store renders the files it was warmed from, byte for byte"
  );
}

#[test]
fn a_store_holding_a_board_and_no_thread_is_not_warmed_again_from_an_older_file() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_add("cc", WbItemKind::Todo, "held in the store")
    .expect("add");
  drop(f);

  // The file falls behind the store, as it does while a board write's extract
  // has not landed. A store holding only boards that read as cold would restore
  // this file over its own row at the next open.
  let path = fx.project().board_json("cc");
  let mut behind: Board =
    serde_json::from_str(&std::fs::read_to_string(&path).expect("read")).expect("a board");
  behind.items.clear();
  std::fs::write(&path, serde_json::to_string_pretty(&behind).expect("json")).expect("write");

  let f = fx.facade_on_disk();
  assert_eq!(
    f.board("cc").expect("cc's board").items.len(),
    1,
    "the store's item survived an open over an older board.json"
  );
}

#[test]
fn a_cold_warm_under_the_lock_leaves_a_board_a_peer_registered_first() {
  let fx = an_estate_with_boards();
  let clone = fx.clone_extract();
  let mut peer = Store::open(&clone.project().db_path()).expect("the peer's handle");
  let mut warmer = Store::open(&clone.project().db_path()).expect("the warmer's handle");
  let peers = boards_of(&fx);
  peer
    .replace_boards(&peers[..1])
    .expect("the peer lands its board first");

  let warmed = warmer
    .warm_if_cold(&[], &[], &peers)
    .expect("the warm runs against a disk the peer's store has moved past");
  assert!(
    !warmed,
    "a store holding a node is not cold, and the warm must say it did nothing"
  );
  assert_eq!(
    warmer.hydrate_boards().expect("read the store"),
    peers[..1].to_vec(),
    "the warm restored the disk's boards over a store a peer had written"
  );
}

#[test]
fn every_write_that_would_render_over_a_board_the_store_lacks_refuses_first() {
  let fx = an_estate_with_boards();
  let clone = a_clone_warmed_before_0535(&fx);
  let files = whiteboard(&clone);
  let mut f = clone.facade_on_disk();
  let want = ["cc".to_string(), "vc".to_string()];

  let register = f
    .wb_register("cc", "Control Claude", "control")
    .expect_err("registering over a board the store lacks");
  assert_eq!(
    lacking(&register),
    Some(&want[..]),
    "wb register cc: {register:?}"
  );
  let roster = f
    .register_roster()
    .expect_err("registering a roster from rendered headers");
  assert_eq!(lacking(&roster), Some(&want[..]), "wb register: {roster:?}");
  let migrate = f
    .wb_migrate("cc", false)
    .expect_err("migrating a rendered board");
  assert_eq!(
    lacking(&migrate),
    Some(&want[..]),
    "wb migrate cc: {migrate:?}"
  );
  let to_disk = f
    .sync_to_disk(&Scope::All)
    .expect_err("rendering boards the store lacks");
  assert_eq!(
    lacking(&to_disk),
    Some(&want[..]),
    "sync --to-disk: {to_disk:?}"
  );
  assert!(
    to_disk.remedy().contains("intent sync --to-store"),
    "the refusal names the door that carries the boards in: {}",
    to_disk.remedy()
  );

  assert_eq!(
    whiteboard(&clone),
    files,
    "no refused door wrote a whiteboard file"
  );
}

#[test]
fn the_reads_name_sync_to_store_and_it_carries_every_board_in() {
  let fx = an_estate_with_boards();
  let clone = a_clone_warmed_before_0535(&fx);
  let mut f = clone.facade_on_disk();

  let status = f
    .wb_status()
    .expect_err("an empty roster beside migrated boards");
  assert!(lacking(&status).is_some(), "wb status: {status:?}");
  assert!(
    status.remedy().contains("intent sync --to-store"),
    "wb status names sync --to-store, not wb register: {}",
    status.remedy()
  );
  let show = f.board("cc").expect_err("a board the store lacks");
  assert!(lacking(&show).is_some(), "wb show cc: {show:?}");
  assert!(
    matches!(f.board("zz"), Err(FacadeError::WbNodeNotRegistered { .. })),
    "a moniker with no board on disk is still only unregistered"
  );

  let files = whiteboard(&clone);
  f.sync_from_disk(&Scope::All)
    .expect("sync --to-store carries the boards in");
  assert_eq!(
    f.wb_status().expect("the roster after the restore"),
    boards_of(&fx),
    "the restore took every board, archived items and messages included"
  );
  f.sync_to_disk(&Scope::All)
    .expect("the restored store egests");
  assert_eq!(
    whiteboard(&clone),
    files,
    "the restored store renders the files it took, byte for byte"
  );
}

#[test]
fn doctor_names_a_store_that_holds_the_estate_and_not_its_boards() {
  let about_the_boards = |fx: &Fixture| -> Vec<String> {
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
      .findings
      .into_iter()
      .filter(|f| f.class == FindingClass::StoreStale && f.detail.contains("migrated board"))
      .map(|f| f.detail)
      .collect()
  };
  let fx = an_estate_with_boards();
  assert_eq!(
    about_the_boards(&fx),
    Vec::<String>::new(),
    "control: the estate's own store holds its boards"
  );
  assert_eq!(
    about_the_boards(&fx.clone_extract()),
    Vec::<String>::new(),
    "control: a clone with no store is cold, and its first open warms the boards"
  );

  let clone = a_clone_warmed_before_0535(&fx);
  let found = about_the_boards(&clone);
  assert_eq!(found.len(), 1, "one finding names the state: {found:?}");
  assert!(
    found[0].contains("cc, vc") && found[0].contains("intent sync --to-store"),
    "it names the nodes and the refusal's own remedy: {found:?}"
  );
  assert!(
    !FindingClass::StoreStale.is_actionable(),
    "reported in its sibling's class, and never counted"
  );

  clone
    .facade_on_disk()
    .sync_from_disk(&Scope::All)
    .expect("sync --to-store");
  assert_eq!(
    about_the_boards(&clone),
    Vec::<String>::new(),
    "the restore clears it"
  );
}

#[test]
fn a_skewed_inbox_on_a_cold_clone_is_rendered_back_by_sync_to_disk() {
  const INBOX: &str = "intent/whiteboard/cc/inbox.vc.md";
  let skew_on_the_inbox = |fx: &Fixture| -> usize {
    intentsvcs::doctor::diagnose(&fx.project(), &ctx(), None, intentsvcs::doctor::Scope::All)
      .findings
      .iter()
      .filter(|f| f.class == FindingClass::ViewSkew && f.to_string().contains("inbox.vc.md"))
      .count()
  };
  let fx = an_estate_with_boards();
  let clone = fx.clone_extract();
  let rendered = clone.read(INBOX);
  clone.write_file(INBOX, &format!("{rendered}a line no renderer wrote\n"));
  assert_eq!(
    skew_on_the_inbox(&clone),
    1,
    "control: doctor sees the hand edit as skew"
  );

  clone
    .facade_on_disk()
    .sync_to_disk(&Scope::All)
    .expect("the first open warms the boards, so the egest has an inbox to render");
  assert_eq!(
    clone.read(INBOX),
    rendered,
    "sync --to-disk, the remedy doctor printed, rendered the inbox back"
  );
  assert_eq!(skew_on_the_inbox(&clone), 0, "and doctor reads it clean");
}
