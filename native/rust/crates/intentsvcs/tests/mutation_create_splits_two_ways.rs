//! AT-08.4 / AC-08.4: **caller-assigned ids are a `PUT` to the entity address;
//! server-assigned ids are a `POST` to the COLLECTION address returning the
//! new address.**
//!
//! **You cannot address `ST0058` before the tool has decided it is `ST0058`.**
//! That is the whole of the split: the address of a caller-assigned entity
//! exists before the entity does, so `PUT` can create it; the address of a
//! server-assigned one does not exist until the write has happened, so there
//! is nothing to `PUT` to.
//!
//! The refusal for the server-assigned side matters as much as the acceptance
//! for the other: a surface that quietly accepted `PUT intent:///threads/ST0058`
//! would either invent an id the tool did not choose, or write to a thread
//! that is not the one the caller meant.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::address::parse;

const NEW_AT: &str = r#"{"id":"AT-09.1","kind":"test",
  "file":"native/rust/crates/intentsvcs/tests/new.rs",
  "covers":["AC-03.1"],"status":"to-write",
  "note":"created through the address surface"}"#;

/// **A caller-assigned id is CREATED by `PUT`** -- and this is the gap
/// AC-08.5's fourth instance names: no verb creates an AC or an AT, so the
/// only route was a hand-edit of canon plus `sync --to-store`.
#[test]
fn put_creates_a_row_at_an_address_that_did_not_exist() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  assert!(
    !facade.canon().threads[0]
      .tests
      .iter()
      .any(|t| t.id == "AT-09.1"),
    "precondition: the row must be ABSENT, or this tests an update"
  );

  let address = parse("intent:///threads/ST0001/at/AT-09.1?format=json").expect("resolves");
  facade.put(&address, NEW_AT).expect("PUT creates it");

  let row = facade.canon().threads[0]
    .tests
    .iter()
    .find(|t| t.id == "AT-09.1")
    .expect("the row now exists");
  assert_eq!(
    row.note.as_deref(),
    Some("created through the address surface"),
    "and it carries the field no verb could set -- which is the point"
  );
}

/// The same `PUT` twice is a no-op, not a duplicate row.
#[test]
fn put_is_idempotent() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();
  let address = parse("intent:///threads/ST0001/at/AT-09.1").expect("resolves");

  facade.put(&address, NEW_AT).expect("creates");
  facade
    .put(&address, NEW_AT)
    .expect("second PUT is accepted");

  assert_eq!(
    facade.canon().threads[0]
      .tests
      .iter()
      .filter(|t| t.id == "AT-09.1")
      .count(),
    1,
    "a repeated PUT of the same shape must not duplicate the row"
  );
}

/// **The server-assigned side, refused with its reason.**
#[test]
fn put_to_a_server_assigned_id_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  for url in [
    "intent:///threads/ST0058",
    "intent:///threads",
    "intent:///issues/0099",
  ] {
    let address = parse(url).expect("resolves");
    let err = facade.put(&address, "{}").err().unwrap_or_else(|| {
      panic!("{url} names a SERVER-ASSIGNED id and must be refused, not written")
    });
    assert!(
      err.to_string().contains("server-assigned"),
      "{url}: the refusal must name the rule that sent it away -- {err}"
    );
  }
}

/// An update through the address surface replaces the row and touches nothing
/// else in the thread -- the same property AT-08.5 asserts for `at_set`, held
/// for the write-by-address path too.
#[test]
fn put_updates_without_disturbing_the_rest_of_the_thread() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  let mut facade = fx.facade();

  let before_others: Vec<String> = facade.canon().threads[0]
    .tests
    .iter()
    .filter(|t| t.id != "AT-03.1")
    .map(|t| serde_json::to_string(t).expect("serialises"))
    .collect();

  let address = parse("intent:///threads/ST0001/at/AT-03.1").expect("resolves");
  let body = r#"{"id":"AT-03.1","kind":"test","file":"native/rust/crates/intentsvcs/tests/moved.rs",
    "covers":["AC-03.1"],"status":"green","note":"rewritten whole through the address"}"#;
  facade.put(&address, body).expect("updates");

  let after_others: Vec<String> = facade.canon().threads[0]
    .tests
    .iter()
    .filter(|t| t.id != "AT-03.1")
    .map(|t| serde_json::to_string(t).expect("serialises"))
    .collect();

  assert_eq!(
    before_others, after_others,
    "a PUT addressed at one row must not move a sibling"
  );
}
