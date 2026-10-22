//! **Issue 0612: a message that predates the 4.0 board takes the same id in
//! every store that converts it.**
//!
//! A mark names its message by id, so two clones that converted the old shape
//! with different ids would never join a mark written in one to the message as
//! the other extracts it. The id is therefore derived from the old row alone,
//! by one function, at both doors: the read of a 3.0 `board.json` and the
//! store's rung 32. This arm builds two fresh stores from the same 3.0 files
//! and converts a store at schema 31 holding the same rows, and every id, every
//! join and every mark agrees across the three.

use crate::common::Fixture;
use intentsvcs::store::Store;

const R1: &str = "2026-10-01T09:00:00.100Z";
const R2: &str = "2026-10-01T09:05:00.200Z";
const R3: &str = "2026-10-01T09:10:00.300Z";
const HANDLED: &str = "2026-10-01T10:00:00.000Z";

/// Every message as (sender, recipient, recorded_at, body, mid) and every mark
/// as (mid, handled_at), each sorted, as a store holds them.
type Held = (
  Vec<(String, String, String, String, String)>,
  Vec<(String, String)>,
);

fn held(db: &std::path::Path) -> Held {
  let conn = rusqlite::Connection::open(db).expect("open the store");
  let mut messages: Vec<(String, String, String, String, String)> = conn
    .prepare("SELECT sender, recipient, recorded_at, body, mid FROM wb_message")
    .expect("prepare")
    .query_map([], |r| {
      Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?))
    })
    .expect("query")
    .collect::<Result<_, _>>()
    .expect("rows");
  messages.sort();
  let mut marks: Vec<(String, String)> = conn
    .prepare("SELECT mid, handled_at FROM wb_mark")
    .expect("prepare")
    .query_map([], |r| Ok((r.get(0)?, r.get(1)?)))
    .expect("query")
    .collect::<Result<_, _>>()
    .expect("rows");
  marks.sort();
  (messages, marks)
}

/// One node's 3.0 `board.json`: migrated, and carrying the messages addressed
/// to it in its old order.
fn old_board(moniker: &str, messages: serde_json::Value) -> String {
  serde_json::to_string_pretty(&serde_json::json!({
    "schema": "intent/board@3.0",
    "node": {
      "moniker": moniker, "name": format!("Node {moniker}"), "role": "worker",
      "heartbeat_at": R1, "status": "active", "focus": "", "claims": [],
      "recorded_at": R1, "migrated_at": R1,
    },
    "items": [],
    "messages": messages,
  }))
  .expect("json")
}

fn message(
  sender: &str,
  recipient: &str,
  at: &str,
  body: &str,
  handled: bool,
) -> serde_json::Value {
  match handled {
    true => serde_json::json!({
      "sender": sender, "recipient": recipient, "body": body, "fyi": false,
      "state": "handled", "handled_at": HANDLED, "recorded_at": at,
    }),
    false => serde_json::json!({
      "sender": sender, "recipient": recipient, "body": body, "fyi": false,
      "state": "live", "recorded_at": at,
    }),
  }
}

/// An estate whose boards are 3.0 files and which has no store: bb holds a
/// handled message from aa, then twins (same sender, recipient and stamp), and
/// aa holds one live message from bb.
fn an_old_estate() -> Fixture {
  let fx = Fixture::new();
  let project = fx.project();
  let write = |node: &str, text: String| {
    let path = project.board_json(node);
    std::fs::create_dir_all(path.parent().expect("node dir")).expect("mkdir");
    std::fs::write(path, text).expect("write board");
  };
  write(
    "aa",
    old_board(
      "aa",
      serde_json::json!([message("bb", "aa", R3, "from bb", false)]),
    ),
  );
  write(
    "bb",
    old_board(
      "bb",
      serde_json::json!([
        message("aa", "bb", R1, "handled first", true),
        message("aa", "bb", R2, "twin one", false),
        message("aa", "bb", R2, "twin two", false),
      ]),
    ),
  );
  fx
}

/// A store at schema 31 holding the same rows in the same order, rung 32 not
/// yet run: the shape rung 30 left, with the handled state on the row.
fn a_store_at_31(fx: &Fixture) {
  let path = fx.project().db_path();
  drop(Store::open(&path).expect("create the store"));
  let conn = rusqlite::Connection::open(&path).expect("open the store");
  crate::common::wb_messages_as_at_31(&conn);
  for (sender, recipient, at, body, state) in [
    ("aa", "bb", R1, "handled first", "handled"),
    ("aa", "bb", R2, "twin one", "live"),
    ("aa", "bb", R2, "twin two", "live"),
    ("bb", "aa", R3, "from bb", "live"),
  ] {
    let handled_at = (state == "handled").then_some(HANDLED);
    conn
      .execute(
        "INSERT INTO wb_message (sender, recipient, body, fyi, state, handled_at, recorded_at) \
         VALUES (?1, ?2, ?3, 0, ?4, ?5, ?6)",
        rusqlite::params![sender, recipient, body, state, handled_at, at],
      )
      .expect("an old row");
  }
  conn
    .pragma_update(None, "user_version", 31)
    .expect("stamp the version before the rung");
}

#[test]
fn two_fresh_stores_and_the_rung_give_every_old_message_one_id() {
  let first = an_old_estate();
  let second = first.clone_extract();
  drop(first.facade_on_disk());
  drop(second.facade_on_disk());
  let converted = Fixture::new();
  a_store_at_31(&converted);
  drop(Store::open(&converted.project().db_path()).expect("the store migrates"));

  let (messages, marks) = held(&first.project().db_path());
  assert_eq!(
    messages.len(),
    4,
    "precondition: every old message is carried"
  );
  let ids: std::collections::BTreeSet<&String> = messages.iter().map(|m| &m.4).collect();
  assert_eq!(ids.len(), 4, "twins take two ids: {messages:?}");
  let handled_first = messages
    .iter()
    .find(|m| m.3 == "handled first")
    .map(|m| m.4.clone());
  assert_eq!(
    marks,
    vec![(
      handled_first.expect("the handled message"),
      HANDLED.to_string()
    )],
    "the one handled message keeps its mark and its stamp"
  );
  assert_eq!(
    held(&second.project().db_path()),
    (messages.clone(), marks.clone()),
    "a second fresh store from the same files holds the same ids"
  );
  assert_eq!(
    held(&converted.project().db_path()),
    (messages, marks),
    "a store converted at rung 32 holds the same ids as the stores built from the files"
  );
}
