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
//!
//! # The row was RED for evidencing one limb of two, and this is the other
//!
//! vc ruled it red on a measured gap rather than on pending evidence: `fn post`
//! had zero hits in `facade.rs`, so the criterion's second clause -- *a POST to
//! the COLLECTION address RETURNING THE NEW ADDRESS* -- had no implementation
//! at all. **ic had offered the PUT refusal for the server-assigned side as
//! though it were coverage of the POST side.** It is not: refusing the wrong
//! door is not the same as opening the right one, and a criterion naming two
//! capabilities is not satisfied by one of them plus a refusal.

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

// ---------------------------------------------------------------------------
// The POST half -- the limb this row was red for.
// ---------------------------------------------------------------------------

/// **THE PROPERTY.** A POST to the collection creates and hands back the
/// address the tool assigned.
#[test]
fn post_to_the_collection_creates_and_returns_the_new_address() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let made = facade
    .post(
      &parse("intent:///threads").expect("the collection resolves"),
      r#"{"title":"a thread the caller could not have named"}"#,
    )
    .expect("posting to the collection creates");

  // **THE RETURN IS AN ADDRESS, NOT AN ID, AND THAT IS THE CRITERION'S WORD.**
  // A caller handed a bare id has to build the address itself, which is a
  // second spelling of the scheme at every call site.
  let url = made.to_url();
  assert!(
    url.starts_with("intent:///threads/ST"),
    "the new address names the thread the tool assigned: {url}"
  );
  assert_ne!(
    url, "intent:///threads/ST0056",
    "and it is a NEW id, not the one already there"
  );

  // And the address it handed back actually resolves to the thing it made.
  let id = url.rsplit('/').next().expect("an id");
  assert_eq!(
    facade.st_show(id).expect("the posted thread exists").title,
    "a thread the caller could not have named",
    "the returned address is usable -- an address naming nothing is not a create"
  );
}

/// **THE CONTROL.** Two POSTs make two threads, so the id is genuinely assigned
/// per call rather than derived from the body.
///
/// Without it the case above passes on an implementation that returns a
/// constant address and writes once.
#[test]
fn two_posts_make_two_threads() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  let coll = parse("intent:///threads").expect("resolves");

  let a = facade.post(&coll, r#"{"title":"first"}"#).expect("creates");
  let b = facade.post(&coll, r#"{"title":"second"}"#).expect("creates");
  assert_ne!(
    a.to_url(),
    b.to_url(),
    "the tool assigns a fresh id per POST -- an id derived from the body would\n       \
     collide the moment two callers posted the same title"
  );
}

/// **POST to an ENTITY address is refused, which is the mirror of
/// `put_to_a_server_assigned_id_is_refused`.**
///
/// The two refusals together are what make the split a rule rather than two
/// independent behaviours: each verb refuses exactly where the other works.
#[test]
fn post_to_an_entity_address_is_refused_and_names_the_verb_that_works() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();

  let err = facade
    .post(
      &parse("intent:///threads/ST0056/ac/AC-01.1").expect("resolves"),
      r#"{"title":"x"}"#,
    )
    .expect_err("its id is already known, so POST is the wrong door");
  let said = err.to_string();
  assert!(
    said.contains("PUT"),
    "the refusal names the door that DOES open -- sending a caller away without\n       \
     one is how a surface acquires a reputation for arbitrary refusals: {said}"
  );
  // **`said.contains("ac")` WAS THE FIRST VERSION AND IT WAS VACUOUS.** The
  // error renders the URL, and the URL is `.../ac/AC-01.1` -- so the assertion
  // passed on the address being echoed back rather than on the form being
  // named, and a mutant that dropped the form name entirely survived it.
  //
  // A `node-inbox` discriminates because its FORM NAME is not a substring of
  // its own URL: the address spells `nodes` and `inbox` separately, so only a
  // message that actually names the form can contain `node-inbox`.
  let err = facade
    .post(
      &parse("intent:///nodes/ic/inbox/vc/2026-08-19T11:41Z").expect("resolves"),
      r#"{"title":"x"}"#,
    )
    .expect_err("a node inbox is not a collection this tool assigns ids in");
  let said = err.to_string();
  assert!(
    said.contains("node-inbox"),
    "the refusal names the FORM it refused, so it is countable rather than\n       \
     generic -- and the form name is not spelled anywhere in the address, so\n       \
     this cannot pass on the URL being echoed: {said}"
  );
}

/// A posted thread still has to be a legal thread: a blank title is refused
/// rather than defaulted.
///
/// **The door being new is not a reason for it to be laxer than `st new`.** A
/// create arriving through a different surface must not be able to make an
/// entity the verb refuses to make.
#[test]
fn a_posted_thread_without_a_title_is_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0056"));
  let mut facade = fx.facade();
  let coll = parse("intent:///threads").expect("resolves");

  for bad in [r#"{}"#, r#"{"title":""}"#, r#"{"title":"   "}"#, "not json"] {
    assert!(
      facade.post(&coll, bad).is_err(),
      "`{bad}` must be refused -- a thread with no title is unfindable in every\n       \
       view that lists it"
    );
  }
}
