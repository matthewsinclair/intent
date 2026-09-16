//! Issues 0411 and 0415: a whiteboard write leaves a trace. Every verb that
//! changes a board row records one `wb.*` event naming the acting node and the
//! verb's arguments, and every row it changes carries `updated_at` from the
//! same write -- the store kept neither, so it could say what a board row held
//! and never who wrote it or that it had been edited after insert.
//!
//! **A WRITE THAT MOVES NOTHING RECORDS NOTHING.** Claiming what is already
//! claimed and registering the same values twice both report that nothing
//! moved, and an event beside that report would contradict it.

use crate::common::Fixture;
use intentsvcs::model::WbItemKind;
use rusqlite::Connection;

/// The whiteboard ops in the log, oldest first, with their subject ids.
fn wb_events(f: &intentsvcs::facade::Facade) -> Vec<(String, String, serde_json::Value)> {
  f.store()
    .events()
    .expect("read the log")
    .into_iter()
    .filter(|e| e.op.starts_with("wb."))
    .map(|e| (e.op, e.subject.id, e.payload))
    .collect()
}

fn ops(f: &intentsvcs::facade::Facade) -> Vec<String> {
  wb_events(f).into_iter().map(|(op, _, _)| op).collect()
}

#[test]
fn every_board_verb_records_one_event_naming_the_node_and_its_arguments() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_pickup("cc", Some("session-1"), Some("bank 1"), false)
    .expect("pickup");
  f.wb_touch("cc").expect("touch");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  f.wb_decide("cc", "one door").expect("decide");
  f.wb_archive("cc", WbItemKind::Todo, 1).expect("archive");
  f.wb_claim("cc", "ST0069").expect("claim");
  f.wb_unclaim("cc", "ST0069").expect("unclaim");
  f.wb_ask("cc", "vc", "bank 1 is green", None, false)
    .expect("ask");
  f.wb_announce("vc", "a train lands").expect("announce");
  f.wb_clear("vc", "cc").expect("clear");
  f.wb_release("cc").expect("release");

  assert_eq!(
    ops(&f),
    [
      "wb.register",
      "wb.register",
      "wb.pickup",
      "wb.touch",
      "wb.add",
      "wb.decide",
      "wb.archive",
      "wb.claim",
      "wb.unclaim",
      "wb.ask",
      "wb.announce",
      "wb.clear",
      "wb.release",
    ],
    "one event per verb, in the order the verbs ran"
  );

  let events = wb_events(&f);
  let (_, subject, payload) = &events[2];
  assert_eq!(subject, "cc", "the subject is the acting node");
  assert_eq!(payload["session_id"], "session-1");
  assert_eq!(payload["focus"], "bank 1");
  let (_, subject, payload) = &events[9];
  assert_eq!(subject, "cc", "an ask is the sender's act");
  assert_eq!(payload["recipient"], "vc");
  assert_eq!(payload["body"], "bank 1 is green");
}

#[test]
fn a_verb_that_moves_nothing_records_nothing() {
  let fx = Fixture::new();
  let mut f = fx.facade();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_claim("cc", "ST0069").expect("claim");
  let before = ops(&f).len();

  assert_eq!(
    f.wb_register("cc", "Control Claude", "control")
      .expect("the same values"),
    0
  );
  assert!(!f.wb_claim("cc", "ST0069").expect("claimed already"));
  assert!(!f.wb_unclaim("cc", "ST0070").expect("never claimed"));
  assert!(
    !f.wb_archive("cc", WbItemKind::Todo, 7)
      .expect("no such item")
  );
  assert_eq!(f.wb_clear("cc", "cc").expect("an empty inbox"), 0);

  assert_eq!(
    ops(&f).len(),
    before,
    "a write that moved no row left an event: {:?}",
    ops(&f)
  );
}

/// `updated_at` moves with the write that changes the row, and never lags the
/// field that write stamped.
#[test]
fn every_row_a_verb_changes_carries_updated_at_from_that_write() {
  let fx = Fixture::new();
  let mut f = fx.facade_on_disk();
  f.wb_register("cc", "Control Claude", "control")
    .expect("register cc");
  f.wb_register("vc", "Validation Claude", "validation")
    .expect("register vc");
  f.wb_add("cc", WbItemKind::Todo, "write the door")
    .expect("add");
  f.wb_ask("vc", "cc", "go", None, false).expect("ask");
  // Past the registration's millisecond, so a stamp that never moved is seen.
  std::thread::sleep(std::time::Duration::from_millis(5));

  f.wb_pickup("cc", Some("session-1"), Some("bank 1"), false)
    .expect("pickup");
  f.wb_archive("cc", WbItemKind::Todo, 1).expect("archive");
  f.wb_clear("cc", "vc").expect("clear");

  let db = Connection::open(fx.project().db_path()).expect("open the store");
  let one = |sql: &str| -> (String, String) {
    db.query_row(sql, [], |r| Ok((r.get(0)?, r.get(1)?)))
      .expect(sql)
  };
  let (updated, heartbeat) =
    one("SELECT updated_at, heartbeat_at FROM wb_node WHERE moniker = 'cc'");
  assert!(
    updated >= heartbeat,
    "the pickup moved the heartbeat to {heartbeat} and left updated_at at {updated}"
  );
  let (updated, archived) = one("SELECT updated_at, archived_at FROM wb_item WHERE node = 'cc'");
  assert!(
    updated >= archived,
    "the archive stamped {archived} and left updated_at at {updated}"
  );
  let (updated, handled) =
    one("SELECT updated_at, handled_at FROM wb_message WHERE recipient = 'cc'");
  assert!(
    updated >= handled,
    "the clear stamped {handled} and left updated_at at {updated}"
  );
}
