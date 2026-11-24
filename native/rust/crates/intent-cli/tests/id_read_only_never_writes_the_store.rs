//! ST0095 AC-12.1's routing half, first door (hv decision 79): **`intent id
//! --read-only` writes nothing on disk.** The commit gate's scripts ask ids
//! through it, and a dev build must never write the store it reads. A store at
//! this build's schema is read as it stands and left byte for byte; no store is
//! answered from the committed canon and not made; an older store is not
//! opened, exit 2, with doctor's `store` object under `--format json`, so one
//! runner reads both doors.

use crate::common::{STORE, created, ok, run, stamp_older_schema};
use intentsvcs::store::SCHEMA_VERSION;
use testkit::{assert_checkpointed, assert_no_wal_frames};

/// `id --read-only --format json <token>` in `root`: its JSON and exit code.
fn id_read_only(root: &std::path::Path, token: &str) -> (serde_json::Value, i32) {
  let (out, err, code) = run(root, &["id", "--read-only", "--format", "json", token]);
  let json = serde_json::from_str(&out)
    .unwrap_or_else(|e| panic!("id --read-only printed no JSON ({e}):\n{out}{err}"));
  (json, code)
}

/// The id the one answer names, when it names one.
fn answered(json: &serde_json::Value) -> Option<&str> {
  json["answers"][0]["id"].as_str()
}

#[test]
fn id_read_only_answers_without_writing_and_stops_where_reading_would_migrate() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "readonly"]);
  let st = created(&ok(root, &["st", "new", "A thread the door names"]));
  let db = root.join(STORE);
  assert_checkpointed(&db);

  // A store at this build's schema: answered, and not one byte written.
  let current = std::fs::read(&db).expect("read the store");
  let (json, code) = id_read_only(root, &st);
  assert_eq!(
    (code, answered(&json), json.get("store").is_none()),
    (0, Some(st.as_str()), true),
    "a current store answers the id as usual: {json}"
  );
  assert_eq!(
    std::fs::read(&db).expect("read the store"),
    current,
    "--read-only wrote to a store it could read as it stands"
  );
  assert_no_wal_frames(&db, "id --read-only on a current store");

  // An older store: not opened, exit 2, and the JSON is doctor's object.
  let (stamped, older) = stamp_older_schema(&db);
  let (json, code) = id_read_only(root, &st);
  assert_eq!(
    (
      code,
      json["store"]["opening"].as_str(),
      json["store"]["schema"].as_i64(),
      json["store"]["build"].as_i64(),
      json.get("answers").is_none()
    ),
    (
      2,
      Some("migrate"),
      Some(i64::from(older)),
      Some(i64::from(SCHEMA_VERSION)),
      true
    ),
    "an older store is named and no token is answered: {json}"
  );
  assert_eq!(
    std::fs::read(&db).expect("read the store"),
    stamped,
    "--read-only migrated the store it refused to open"
  );
  assert_no_wal_frames(&db, "id --read-only on an older store");

  // No store: the committed canon is everything, so it answers, and makes none.
  std::fs::remove_file(&db).expect("remove the store");
  let (json, code) = id_read_only(root, &st);
  assert_eq!(
    (code, answered(&json)),
    (0, Some(st.as_str())),
    "with no store the id is answered from the committed canon: {json}"
  );
  assert!(
    !db.exists(),
    "--read-only made the store it was told not to"
  );
}
