//! The escape hatch's resolvers answer what the facade holds, and the schema
//! itself refuses what the bound excludes (`AC-09.2`, reads only -- vc under
//! hv's pen, 2026-08-31).
//!
//! **DRIVEN THROUGH `Facade::graphql`, THE ONE DOOR, AGAINST A REAL STORE.** A
//! test that built the schema and attached its own data would prove the
//! resolvers read a struct, not that they read the facade -- which is the
//! whole of the bound. The one place the schema IS driven bare, below, exists
//! to prove that door is the only one.
//!
//! ST0069 AT-14.7 cites this file: any node's board is read over GraphQL through
//! that one door, `boards` and `board(node)`.

use crate::common::Fixture;
use serde_json::{Value, json};

#[tokio::test]
async fn the_four_reads_answer_what_the_facade_holds() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  let id = facade.st_new("Hatch fixture").expect("mint a thread");
  let number = facade
    .issue_add("A hatch issue", None, None, "reported by the hatch test")
    .expect("add an issue");

  let answer = facade
    .graphql(
      &format!(
        "query($id: String!) {{ threads {{ id title }} thread(id: $id) {{ id title }} issues {{ number title }} issue(number: {number}) {{ number }} missing: thread(id: \"ST9999\") {{ id }} }}"
      ),
      Some(json!({ "id": id })),
    )
    .await
    .expect("the answer serialises");

  assert_eq!(
    answer["errors"],
    Value::Null,
    "a valid read carries no errors: {answer}"
  );
  let data = &answer["data"];
  assert!(
    data["threads"]
      .as_array()
      .is_some_and(|threads| threads.iter().any(|t| t["id"] == id.as_str())),
    "threads: {data}"
  );
  assert_eq!(data["thread"]["title"], "Hatch fixture");
  assert!(
    data["issues"]
      .as_array()
      .is_some_and(|issues| issues.iter().any(|i| i["number"] == number)),
    "issues: {data}"
  );
  assert_eq!(data["issue"]["number"], number);
  // An id nobody minted is `null`, which is an answer and not an error --
  // the spec's shape for an absent object, and the reason the resolver's
  // return type is `Option`.
  assert_eq!(data["missing"], Value::Null);
}

#[tokio::test]
async fn any_node_reads_every_board_and_one_board_by_moniker() {
  let fx = Fixture::new();
  let mut facade = fx.facade();
  for (node, name, role) in [
    ("cc", "Control Claude", "control"),
    ("vc", "Validation Claude", "validation"),
  ] {
    facade
      .wb_register(node, name, role)
      .expect("register the node");
  }
  facade
    .wb_add(
      "cc",
      intentsvcs::model::WbItemKind::Hold,
      "held until the pair is rebuilt",
    )
    .expect("a hold");
  facade
    .wb_ask("vc", "cc", "the order", None, false)
    .expect("a message");

  let answer = facade
    .graphql(
      "{ boards { node { moniker } } board(node: \"cc\") { items { kind text } messages { sender body } } missing: board(node: \"zz\") { schema } }",
      None,
    )
    .await
    .expect("the answer serialises");

  assert_eq!(
    answer["errors"],
    Value::Null,
    "a valid read carries no errors: {answer}"
  );
  let data = &answer["data"];
  assert_eq!(
    data["boards"],
    json!([{ "node": { "moniker": "cc" } }, { "node": { "moniker": "vc" } }]),
    "every registered board: {data}"
  );
  assert_eq!(
    data["board"],
    json!({
      "items": [{ "kind": "HOLD", "text": "held until the pair is rebuilt" }],
      "messages": [{ "sender": "vc", "body": "the order" }],
    }),
    "one board with its items and the messages addressed to it: {data}"
  );
  assert_eq!(
    data["missing"],
    Value::Null,
    "an unregistered moniker is null, not an error"
  );
}

#[tokio::test]
async fn a_mutation_document_is_refused_by_the_schema_inside_the_answer() {
  // **THE BOUND IS ENFORCED BY WHAT SHIPS, NOT BY A CHECK IN FRONT OF IT.**
  // `EmptyMutation` is the schema's mutation root, so a mutation document fails
  // validation and the refusal comes back in `errors` -- the spec's own channel
  // -- with `data` null. Nothing in the hatch reads the document to decide.
  let fx = Fixture::new();
  let facade = fx.facade();

  let refused = facade
    .graphql("mutation { anything }", None)
    .await
    .expect("even a refusal serialises");

  assert!(
    refused["data"].is_null(),
    "a refused mutation has no data: {refused}"
  );
  assert!(
    refused["errors"]
      .as_array()
      .is_some_and(|errors| !errors.is_empty()),
    "the refusal is in `errors`: {refused}"
  );
  // The positive control on the mechanism: the published face carries no
  // mutation root at all, so the refusal above is structural.
  assert!(
    !intentsvcs::graphql::sdl().contains("type Mutation"),
    "the SDL face must publish no mutation root while the bound stands"
  );
}

#[tokio::test]
async fn a_request_built_without_the_snapshot_is_refused_by_name() {
  // The schema driven bare, with no facade behind it: the one way to reach the
  // resolvers without `Facade::graphql`, and it must not answer empty.
  let answer = intentsvcs::graphql::schema()
    .execute("{ threads { id } }")
    .await;
  let text = serde_json::to_value(answer).expect("serialisable");
  assert!(text["data"].is_null(), "no data without a snapshot: {text}");
  let message = text["errors"][0]["message"]
    .as_str()
    .expect("an error with a message");
  assert!(
    message.contains("Facade::graphql"),
    "the refusal names the only door: {message}"
  );
}
