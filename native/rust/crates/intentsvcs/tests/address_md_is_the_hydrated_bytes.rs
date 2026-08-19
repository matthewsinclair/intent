//! AT-07.4 / AC-07.4: **`?format=md` serves `View.content` from
//! `views::render_all`, so the served bytes EQUAL the file `organize` would
//! hydrate BY CONSTRUCTION rather than by test** -- `View.path` is literally
//! where it would land. One renderer, three jobs.
//!
//! # What "by construction" means, and why the obvious test is the wrong one
//!
//! The obvious test renders the view a second way and compares. **That test
//! passes for exactly as long as two renderers happen to agree**, which is the
//! same failure AC-07.1 rules out for resolvers and the same one a hand-count
//! has against a regex. It also cannot fail on the day it matters, because the
//! day it matters is the day someone changes one of the two.
//!
//! So the property here is structural: `serve_md` SELECTS from `render_all`'s
//! output by path and never renders. The behavioural cases below then have
//! nothing left to prove except that the selection picks the right member --
//! which is a real thing to get wrong, and cheap to check.
//!
//! # The estate is the subject
//!
//! Run against the real canon rather than a fixture, because the claim is
//! about the bytes `organize` would write HERE. A fixture would prove the
//! selection works on a fixture.

mod common;

use common::ctx;
use intentsvcs::address::{Entity, ServeError, parse, serve_md};
use intentsvcs::project::Project;
use intentsvcs::{ingest, views};
use testkit::repo_root;

fn estate() -> (Project, ingest::Canon) {
  let project = Project::open(&repo_root()).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads");
  (project, canon)
}

/// **The construction itself.** Every view `render_all` emits for an
/// addressable entity must be reachable at that address, byte for byte, from
/// the same call -- and the path served must be the path it would land at.
#[test]
fn served_markdown_is_the_render_all_member_at_its_own_path() {
  let (project, canon) = estate();
  let rendered = views::render_all(&project, &canon, &ctx());
  assert!(
    !rendered.is_empty(),
    "precondition: the renderer produced nothing, so this test's population\n       \
     cannot contain the failure it looks for"
  );

  let mut checked = 0usize;
  for thread in &canon.threads {
    for (url, want_path) in [
      (
        format!("intent:///threads/{}?format=md", thread.id),
        project.info_view(&thread.id),
      ),
      (
        format!("intent:///threads/{}/ac?format=md", thread.id),
        project.acceptance_view(&thread.id),
      ),
    ] {
      let address = parse(&url).unwrap_or_else(|e| panic!("{url}: {e}"));
      let served = serve_md(&project, &canon, &ctx(), &address)
        .unwrap_or_else(|e| panic!("{url} must serve: {e}"));

      assert_eq!(
        served.path, want_path,
        "{url}: the served path must be where organize would hydrate it"
      );
      let hydrated = rendered
        .iter()
        .find(|v| v.path == want_path)
        .unwrap_or_else(|| panic!("{url}: render_all emits no view at {want_path:?}"));
      assert_eq!(
        served.content, hydrated.content,
        "{url}: the served bytes and the hydrated bytes are the same member of\n       \
         the same render_all call -- if these differ, something rendered twice"
      );
      checked += 1;
    }
  }
  assert!(
    checked > 0,
    "the estate held no threads, so nothing was checked"
  );
}

/// The thread COLLECTION renders as the steel-threads view. Included because
/// it is the address whose view is not per-thread, and a selection keyed on a
/// thread id would quietly miss it.
#[test]
fn the_thread_collection_serves_the_steel_threads_view() {
  let (project, canon) = estate();
  let address = parse("intent:///threads?format=md").expect("resolves");
  assert_eq!(address.entity, Entity::Threads);
  let served = serve_md(&project, &canon, &ctx(), &address).expect("serves");
  assert_eq!(served.path, project.steel_threads_view());
}

/// **No second renderer.** `address.rs` may call `render_all` and must not
/// call the per-view renderers, because doing so would produce bytes that
/// merely equal the hydrated ones rather than being them.
///
/// Structural for the reason the header gives: a comparison of two renderings
/// cannot fail until they diverge, and by then it has already shipped.
#[test]
fn the_serve_path_selects_and_never_renders() {
  let src = repo_root().join("native/rust/crates/intentsvcs/src/address.rs");
  let text = std::fs::read_to_string(&src).expect("address.rs is readable");

  let mut offenders = Vec::new();
  for (i, line) in text.lines().enumerate() {
    let code = line.trim_start();
    if code.starts_with("//") {
      continue;
    }
    for renderer in [
      "views::info(",
      "views::acceptance(",
      "views::wp_info(",
      "views::steel_threads(",
      "views::todo(",
    ] {
      if line.contains(renderer) {
        offenders.push(format!("{}:{} calls {renderer}", src.display(), i + 1));
      }
    }
  }
  assert!(
    offenders.is_empty(),
    "the serve path must SELECT from render_all, never render:\n  {}",
    offenders.join("\n  ")
  );
  assert!(
    text.contains("views::render_all("),
    "and it must actually go through render_all -- if this line ever stops\n       \
     being true the test above passes vacuously"
  );
}

/// An entity with no markdown rendering is REFUSED with a reason, not served
/// an empty string. A silent empty body is indistinguishable from a document
/// that happens to be empty.
#[test]
fn an_entity_without_a_rendering_is_refused_with_its_reason() {
  let (project, canon) = estate();
  for url in [
    "intent:///issues/0042?format=md",
    "intent:///nodes/ic?format=md",
    "intent:///events/1234?format=md",
    "intent:///threads/ST0056/ac/AC-02.1?format=md",
  ] {
    let address = parse(url).expect("resolves");
    match serve_md(&project, &canon, &ctx(), &address) {
      Err(ServeError::NoMarkdownRendering { .. }) => {}
      Err(other) => panic!("{url}: wrong refusal -- {other}"),
      Ok(v) => panic!(
        "{url}: served {} bytes and should not have",
        v.content.len()
      ),
    }
  }
}

/// A thread the estate does not hold is NotFound rather than NoMarkdown --
/// the two are different facts and an operator acts on them differently.
#[test]
fn an_absent_thread_is_not_found_rather_than_unrenderable() {
  let (project, canon) = estate();
  let address = parse("intent:///threads/ST9999?format=md").expect("resolves");
  match serve_md(&project, &canon, &ctx(), &address) {
    Err(ServeError::NotFound { .. }) => {}
    other => panic!("expected NotFound, got {other:?}"),
  }
}
