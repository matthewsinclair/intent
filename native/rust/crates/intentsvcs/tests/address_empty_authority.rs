//! AT-07.6 / AC-07.6: **empty authority means THIS project**, and a
//! cross-project reference carries the slug and resolves against intentd's
//! project registry.
//!
//! **A reference that hard-codes the project name breaks on rename or fork**,
//! so the empty form is the one intra-project prose must use -- and that makes
//! `Some(slug)` the deliberate exception rather than the neutral default.
//!
//! The asymmetry is the whole content of the row: it is not that both forms
//! parse, it is that the DEFAULT is the portable one. A scheme where the local
//! form required a slug would put the breakage on the common case.

use intentsvcs::address::{Entity, parse};

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
