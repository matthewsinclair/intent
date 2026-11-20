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

use crate::common::{APART, Fixture, SHARED_A, SHARED_B, sample_issue, sample_thread};
use intentsvcs::id::{IdKind, MIN_PREFIX_HEX, THREAD_PREFIX, ThreadId, WpId};
use intentsvcs::model::TShirt;
use serde_json::{Value, json};

#[tokio::test]
async fn the_four_reads_answer_what_the_facade_holds() {
  let fx = Fixture::new();
  // An ordinal issue, seeded, beside a minted thread: `issue(id:)` takes
  // either family's id as a string (ST0095 WP-10), as `thread(id:)` does.
  let seeded = sample_issue(21);
  fx.write_issue(&seeded);
  let mut facade = fx.facade();
  let id = facade.st_new("Hatch fixture").expect("mint a thread");

  let answer = facade
    .graphql(
      "query($id: String!, $issue: String!) { threads { id title } thread(id: $id) { id title } issues { id title } issue(id: $issue) { id } missing: thread(id: \"ST9999\") { id } }",
      Some(json!({ "id": id, "issue": seeded.number.stored() })),
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
      .is_some_and(|issues| issues.iter().any(|i| i["id"] == seeded.number.stored())),
    "issues: {data}"
  );
  assert_eq!(data["issue"]["id"], seeded.number.stored());
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
      "{ boards { node { moniker } } board(node: \"cc\") { items { kind text } handled { id } } sender: board(node: \"vc\") { sent { recipient body } } missing: board(node: \"zz\") { schema } }",
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
      "handled": [],
    }),
    "one board with its items and its marks, none yet: {data}"
  );
  assert_eq!(
    data["sender"],
    json!({ "sent": [{ "recipient": "cc", "body": "the order" }] }),
    "a message is on its sender's board (issue 0612): {data}"
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

/// The message of the one error an answer carries for the field `alias`.
fn error_at(answer: &Value, alias: &str) -> String {
  let errors = answer["errors"].as_array().cloned().unwrap_or_default();
  let at: Vec<&Value> = errors
    .iter()
    .filter(|e| e["path"] == json!([alias]))
    .collect();
  assert_eq!(at.len(), 1, "one error on `{alias}`: {answer}");
  at[0]["message"].as_str().unwrap_or_default().to_string()
}

#[tokio::test]
async fn a_typed_id_answers_as_every_door_answers_it() {
  // **cc's meaning, ST0095 WP-10**: a whole id or a unique prefix names its
  // record; a well-formed spelling that names nothing is `null`; a prefix two
  // records begin, and a spelling that is no id of the kind, are refused, each
  // in the facade's own wording, never `null`. The pair is seeded from the
  // kept origins whose ids show one short form, so the ambiguity is real.
  let fx = Fixture::new();
  let pair = [
    (
      ThreadId::from_origin(SHARED_A),
      SHARED_A,
      "the first of two",
    ),
    (
      ThreadId::from_origin(SHARED_B),
      SHARED_B,
      "the second of two",
    ),
  ];
  for (id, origin, title) in &pair {
    let mut thread = sample_thread(&id.stored());
    thread.origin = Some((*origin).to_string());
    thread.title = (*title).to_string();
    thread.related.clear();
    fx.write_thread(&thread);
  }
  let mut facade = fx.facade();
  let minted = facade
    .st_new("minted, one of a kind")
    .expect("mint a thread");
  let issue = facade
    .issue_add("an issue typed by prefix", None, None, "body")
    .expect("mint an issue");
  let thread = ThreadId::parse(&minted).expect("the mint returns a stored thread id");
  let shared = pair[0].0.shown();
  // Letters, so the spelling cannot read as an ordinal's digits.
  let short = format!("{THREAD_PREFIX}{}", "a".repeat(MIN_PREFIX_HEX - 1));

  let answer = facade
    .graphql(
      "query($shown: String!, $issue: String!, $shared: String!, $short: String!) { one: thread(id: $shown) { title } issue(id: $issue) { title } none: thread(id: \"ST9999\") { id } both: thread(id: $shared) { id } banana: issue(id: \"banana\") { id } short: thread(id: $short) { id } }",
      Some(json!({
        "shown": thread.shown(),
        "issue": issue.shown(),
        "shared": shared,
        "short": short,
      })),
    )
    .await
    .expect("the answer serialises");

  let data = &answer["data"];
  assert_eq!(data["one"]["title"], "minted, one of a kind", "{answer}");
  assert_eq!(
    data["issue"]["title"], "an issue typed by prefix",
    "{answer}"
  );
  assert_eq!(
    data["none"],
    Value::Null,
    "a well-formed id naming nothing: {answer}"
  );
  for refused in ["both", "banana", "short"] {
    assert_eq!(
      data[refused],
      Value::Null,
      "`{refused}` is refused, so its field is null: {answer}"
    );
  }
  let refused = ["both", "banana", "short"];
  assert_eq!(
    answer["errors"].as_array().map(Vec::len),
    Some(refused.len()),
    "one error per refused field, none for the answers: {answer}"
  );
  let both = error_at(&answer, "both");
  assert!(
    pair.iter().all(|(_, _, title)| both.contains(title)),
    "the ambiguity names both candidates: {both}"
  );
  let banana = error_at(&answer, "banana");
  assert!(
    banana.contains("`banana`") && banana.contains(IdKind::Issue.with_article()),
    "a spelling that is no issue's id is refused in the waist's words: {banana}"
  );
  let too_short = error_at(&answer, "short");
  assert!(
    too_short.contains(&format!("`{short}`")) && too_short.contains("too short"),
    "hex under the floor is refused as too short: {too_short}"
  );
}

#[tokio::test]
async fn a_work_package_carries_the_day_its_origin_was_written() {
  // A hashed package's `created` is the facade's answer, the day its origin's
  // event was written in this log. An ordinal package has no origin, and a
  // package whose origin this log never wrote has no day to read, so each
  // `created` is null, which is an answer and not an error (cc's meaning,
  // ST0095 WP-10; issue 0732 keeps `None` for exactly these two).
  let fx = Fixture::new();
  let mut seeded = sample_thread("ST0001");
  let mut elsewhere = seeded.wps[0].clone();
  elsewhere.seq = WpId::from_origin(APART);
  elsewhere.origin = Some(APART.to_string());
  seeded.wps.push(elsewhere.clone());
  fx.write_thread(&seeded);
  let mut facade = fx.facade();
  let minted = facade
    .st_new("a thread with a package")
    .expect("mint a thread");
  let wp = facade
    .wp_new(&minted, "a minted package", TShirt::S)
    .expect("mint a package");
  let package = facade
    .st_show(&minted)
    .expect("the minted thread")
    .wps
    .iter()
    .find(|w| w.seq == wp)
    .cloned()
    .expect("the minted package");
  let day = facade
    .wp_created(&package)
    .expect("the log reads")
    .expect("a minted package's origin is in this log");

  let answer = facade
    .graphql(
      "query($id: String!) { minted: thread(id: $id) { wps { id created } } ordinal: thread(id: \"ST0001\") { wps { id created } } }",
      Some(json!({ "id": minted })),
    )
    .await
    .expect("the answer serialises");

  assert_eq!(answer["errors"], Value::Null, "{answer}");
  assert_eq!(
    answer["data"]["minted"]["wps"],
    json!([{ "id": wp.stored(), "created": day }]),
    "{answer}"
  );
  let seeded_wps = answer["data"]["ordinal"]["wps"]
    .as_array()
    .expect("the seeded thread's packages");
  assert_eq!(
    seeded_wps.len(),
    seeded.wps.len(),
    "every seeded package is read: {answer}"
  );
  assert!(
    seeded_wps.iter().all(|w| w["created"] == Value::Null),
    "neither an ordinal package nor one whose origin this log never wrote has a day: {answer}"
  );
  assert!(
    seeded_wps
      .iter()
      .any(|w| w["id"] == elsewhere.seq.stored().as_str()),
    "the package from elsewhere is among them: {answer}"
  );
}
