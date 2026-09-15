//! AT-07.6 / AC-07.6: **empty authority means THIS project**, and a
//! cross-project reference carries the slug, parses and round-trips -- and
//! every door that takes an address refuses it by name (hv's ruling 4,
//! 2026-09-15; issue 0338).
//!
//! **A reference that hard-codes the project name breaks on rename or fork**,
//! so the empty form is the one intra-project prose must use -- and that makes
//! `Some(slug)` the deliberate exception rather than the neutral default.
//!
//! The asymmetry is the whole content of the row: it is not that both forms
//! parse, it is that the DEFAULT is the portable one. A scheme where the local
//! form required a slug would put the breakage on the common case.

use crate::common::{Fixture, sample_thread, tree};
use intentsvcs::address::{Entity, parse};
use intentsvcs::facade::FacadeError;
use intentsvcs::remedy::Remedy;

#[test]
fn the_triple_slash_form_is_local() {
  let a = parse("intent:///threads/ST0056").expect("resolves");
  assert_eq!(a.authority, None);
  assert!(a.is_local(), "empty authority means THIS project");
}

#[test]
fn a_slug_makes_it_cross_project() {
  let a = parse("intent://lamplight/threads/ST0056").expect("resolves");
  assert_eq!(a.authority.as_deref(), Some("lamplight"));
  assert!(!a.is_local());
  assert_eq!(
    a.entity,
    Entity::Thread {
      id: "ST0056".into()
    },
    "the authority changes WHERE it resolves, never WHAT it names"
  );
}

/// **The pair that makes it a test rather than two observations.** The same
/// entity, addressed both ways, differs in exactly one field -- so nothing
/// about the authority leaks into the entity, and nothing about the entity
/// depends on being local.
#[test]
fn local_and_remote_differ_in_exactly_the_authority() {
  let local = parse("intent:///threads/ST0056/ac/AC-02.1").expect("resolves");
  let remote = parse("intent://other/threads/ST0056/ac/AC-02.1").expect("resolves");
  assert_eq!(local.entity, remote.entity);
  assert_eq!(local.format, remote.format);
  assert_ne!(local.authority, remote.authority);
}

/// Both forms round-trip, so prose that carries an address can be rewritten by
/// a tool without silently changing which project it points at.
#[test]
fn both_forms_round_trip() {
  for url in [
    "intent:///threads/ST0056",
    "intent://lamplight/threads/ST0056",
    "intent:///issues/0042?format=json",
    "intent://laksa/issues/0042?format=md",
  ] {
    assert_eq!(parse(url).expect("resolves").to_url(), url);
  }
}

/// The empty authority is the SHORTEST form, which is the mechanism behind the
/// criterion rather than a nicety: the portable spelling must also be the one
/// someone reaches for without thinking.
#[test]
fn the_portable_form_is_the_shortest_one() {
  let local = "intent:///threads/ST0056";
  let named = "intent://intent/threads/ST0056";
  assert!(local.len() < named.len());
  assert!(parse(local).expect("resolves").is_local());
  assert!(
    !parse(named).expect("resolves").is_local(),
    "naming your own project is a CROSS-project reference that happens to\n       \
     point home -- it breaks on rename exactly as the criterion says"
  );
}

/// **EVERY FACADE DOOR THAT TAKES AN ADDRESS REFUSES ANOTHER PROJECT'S, BY NAME,
/// AND NOTHING MOVES** (hv's ruling 4, 2026-09-15; issue 0338).
///
/// The tests above are about the grammar, and a grammar that parses a slug says
/// nothing about what a door then does with it. Driven as built, the doors
/// disagreed: hydrate named the slug under a remedy about artefacts, `set` named
/// the URL and promised a registry that does not exist, `edit` answered with
/// this project's not-found, and the attachment doors never looked. So every
/// door is handed the same foreign address and has to say the same three
/// things: the address it refused, the project that address names, and a remedy
/// spelling this project's own form.
#[test]
fn every_facade_door_refuses_another_projects_address_by_name() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file("intent/.intentfiles", "STEELTHREAD:ST0001\n");
  let mut facade = fx.facade();
  let before = tree(fx.root());

  let thread = parse("intent://elsewhere/threads/ST0001").expect("resolves");
  let absent = parse("intent://elsewhere/threads/ST0009").expect("resolves");
  let threads = parse("intent://elsewhere/threads").expect("resolves");
  let attachment =
    parse("intent://elsewhere/threads/ST0001/attachments/notes.md").expect("resolves");

  // **COLLECTED, NOT ASSERTED ONE AT A TIME**, so a red names every door that
  // gets it wrong rather than stopping at the first.
  let outcomes: Vec<(&str, String, Option<FacadeError>)> = vec![
    ("hydrate", thread.to_url(), facade.hydrate(&thread).err()),
    (
      "hydration",
      thread.to_url(),
      facade.hydration(&thread).err(),
    ),
    (
      "hydration_overwriting",
      thread.to_url(),
      facade
        .hydration_overwriting(&thread, true, &mut |_: &[std::path::PathBuf]| {})
        .err(),
    ),
    (
      "dehydrate",
      thread.to_url(),
      facade.dehydrate(&thread).err(),
    ),
    (
      "dehydrate_announcing",
      thread.to_url(),
      facade
        .dehydrate_announcing(
          &thread,
          &mut |_: &[std::path::PathBuf], _: &[std::path::PathBuf]| {},
        )
        .err(),
    ),
    (
      "edit, an id this project carries",
      thread.to_url(),
      facade.edit(&thread, "design").err(),
    ),
    (
      "edit, an id this project lacks",
      absent.to_url(),
      facade.edit(&absent, "design").err(),
    ),
    ("post", threads.to_url(), facade.post(&threads, "{}").err()),
    ("put", thread.to_url(), facade.put(&thread, "{}").err()),
    (
      "set",
      thread.to_url(),
      facade
        .set(
          &thread,
          "title",
          serde_json::Value::String("Renamed".to_string()),
        )
        .err(),
    ),
    (
      "put_attachment",
      attachment.to_url(),
      facade.put_attachment(&attachment, b"# Notes\n").err(),
    ),
    (
      "detach_attachment",
      attachment.to_url(),
      facade.detach_attachment(&attachment).err(),
    ),
  ];

  let mut failures = Vec::new();
  for (door, url, refusal) in &outcomes {
    let Some(err) = refusal else {
      failures.push(format!("{door}: did not refuse `{url}` at all"));
      continue;
    };
    let message = err.to_string();
    let remedy = err.remedy();
    if !message.contains(&format!("`{url}`")) {
      failures.push(format!(
        "{door}: the refusal does not name `{url}`: {message}"
      ));
    }
    if !message.contains("`elsewhere`") {
      failures.push(format!(
        "{door}: the refusal does not name the project `elsewhere`: {message}"
      ));
    }
    if !remedy.contains("intent:///") {
      failures.push(format!(
        "{door}: the remedy does not spell this project's own address: {remedy}"
      ));
    }
  }
  assert!(
    failures.is_empty(),
    "a door took another project's address without refusing it by name:\n{}",
    failures.join("\n")
  );
  assert_eq!(
    tree(fx.root()),
    before,
    "a door handed another project's address changed this project's tree"
  );
}
