//! Issue `0717`: **`doctor --read-only` never creates or migrates the store.**
//! The commit gate's view-skew guard kept a second copy of `Store::init`'s
//! open-or-migrate decision, comparing SQLite headers, because no door said
//! whether opening a store would write it. Under the flag doctor asks the
//! store's one decision first: a store at this build's schema is judged and
//! left byte for byte, and one that opening would migrate or make is not
//! opened at all, exit 2, with `--format json` naming why.
//!
//! An older store is made by stamping the header's `user_version` (bytes 60
//! to 63, big-endian) one below this build's, because this crate never links
//! SQLite (design D06). The stamp is what the decision reads, and the store is
//! checkpointed first, so no write-ahead log holds a page 1 that shadows it.

use crate::common::{ok, run};
use intentsvcs::store::SCHEMA_VERSION;

const STORE: &str = "intent/.cache/intent.db";

/// `doctor --read-only --format json` in `root`: its JSON and its exit code.
fn doctor_read_only(root: &std::path::Path) -> (serde_json::Value, i32) {
  let (out, err, code) = run(root, &["doctor", "--read-only", "--format", "json"]);
  let json = serde_json::from_str(&out)
    .unwrap_or_else(|e| panic!("doctor --read-only printed no JSON ({e}):\n{out}{err}"));
  (json, code)
}

#[test]
fn a_current_store_is_judged_untouched_and_one_it_would_write_is_not_opened() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "readonly"]);
  // A verb that opens the store, so there is one at this build's schema.
  ok(root, &["st", "list"]);
  let db = root.join(STORE);
  assert!(
    db.is_file() && !root.join(format!("{STORE}-wal")).exists(),
    "the verbs leave a checkpointed store, so its header is its stamp"
  );

  // A store at this build's schema: a verdict, and not one byte written.
  let current = std::fs::read(&db).expect("read the store");
  let (json, code) = doctor_read_only(root);
  assert!(
    code <= 1
      && json["healthy"].is_boolean()
      && json.get("store").is_none()
      && !json["search_index"].is_null(),
    "a current store is judged as usual, its store-backed checks included, with a verdict and no refusal (exit {code}): {json}"
  );
  assert_eq!(
    std::fs::read(&db).expect("read the store"),
    current,
    "--read-only wrote to a store it could open as it stands"
  );

  // An older store: not opened, exit 2, and the JSON says it would migrate.
  let older = SCHEMA_VERSION - 1;
  let mut stamped = current;
  stamped[60..64].copy_from_slice(&older.to_be_bytes());
  std::fs::write(&db, &stamped).expect("stamp an older schema");
  let (json, code) = doctor_read_only(root);
  assert_eq!(
    (
      code,
      json["store"]["opening"].as_str(),
      json["store"]["schema"].as_i64(),
      json["store"]["build"].as_i64()
    ),
    (
      2,
      Some("migrate"),
      Some(i64::from(older)),
      Some(i64::from(SCHEMA_VERSION))
    ),
    "an older store is named and not judged: {json}"
  );
  assert_eq!(
    std::fs::read(&db).expect("read the store"),
    stamped,
    "--read-only migrated the store it refused to open"
  );

  // No store: not made, exit 2, and the JSON says opening would create it.
  std::fs::remove_file(&db).expect("remove the store");
  let (json, code) = doctor_read_only(root);
  assert_eq!(
    (
      code,
      json["store"]["opening"].as_str(),
      json["store"]["schema"].is_null()
    ),
    (2, Some("create"), true),
    "a missing store is named and not made: {json}"
  );
  assert!(
    !db.exists(),
    "--read-only made the store it was told not to"
  );
}
