//! AT-07.2 / AC-07.2: **views get NO address.** A reference to a view resolves
//! to its entity or is rejected; no path segment names a view.
//!
//! **This is what stops the scheme becoming a path alias.** A view is
//! derivable from its entity, so a reference to a view is a reference to its
//! source. Giving views addresses would re-create, INSIDE the scheme, the
//! exact conditionality the scheme exists to remove -- `intent:///.../info.md`
//! is `intent/st/ST0056/info.md` with extra syntax, and it is conditional on
//! what `organize` last did in precisely the same way.
//!
//! The criterion offers two acceptable behaviours -- resolve to the entity, or
//! reject -- and this estate's posture picks rejection: a silent redirect from
//! a view to its entity teaches the writer that the view path works.

use intentsvcs::address::{Entity, Format, parse};

/// Every view name the renderer emits. If `render_all` grows a view, its name
/// belongs here -- an unnamed view is one the scheme would happily address.
const VIEW_SEGMENTS: &[&str] = &[
  "info.md",
  "acceptance.md",
  "todo.md",
  "steel_threads.md",
  "info",
  "acceptance",
];

#[test]
fn no_view_segment_is_addressable() {
  for v in VIEW_SEGMENTS {
    for url in [
      format!("intent:///threads/ST0056/{v}"),
      format!("intent:///threads/ST0056/attachments/{v}"),
      format!("intent:///{v}"),
    ] {
      assert!(
        parse(&url).is_err(),
        "`{url}` names a VIEW and must not resolve -- addressing it re-creates\n       \
         the conditionality the scheme exists to remove"
      );
    }
  }
}

/// The refusal says WHY, because an operator who wrote a view path has the
/// wrong model rather than a typo, and "unknown collection" would send them
/// looking for a spelling mistake.
#[test]
fn the_refusal_names_the_view_and_offers_the_entity() {
  use intentsvcs::remedy::Remedy;
  let err = parse("intent:///threads/ST0056/info.md").expect_err("refused");
  assert!(
    err.to_string().contains("VIEW"),
    "the message must say it is a view: {err}"
  );
  assert!(
    err.remedy().contains("format="),
    "the remedy must point at the representation, not at a spelling: {}",
    err.remedy()
  );
}

/// **The discriminating pair.** The entity IS addressable and its markdown IS
/// reachable -- through `?format=`, which selects a representation rather than
/// naming a separate thing. Without this half, the test above would also pass
/// against a parser that refused everything.
#[test]
fn the_entity_is_addressable_and_its_markdown_is_a_representation() {
  let a = parse("intent:///threads/ST0056?format=md").expect("the ENTITY resolves");
  assert_eq!(
    a.entity,
    Entity::Thread {
      id: "ST0056".into()
    }
  );
  assert_eq!(a.format, Some(Format::Md));
}

/// An attachment is NOT a view, and the two are distinguished by authorship
/// rather than by suffix. A `.md` attachment an author wrote is addressable;
/// a `.md` view the renderer emits is not.
#[test]
fn an_authored_md_attachment_is_still_addressable() {
  let a = parse("intent:///threads/ST0056/attachments/design.md").expect("attachments resolve");
  assert_eq!(
    a.entity,
    Entity::Attachment {
      thread: "ST0056".into(),
      path: "design.md".into()
    },
    "suffix does not decide -- `Project::classify` does, and design.md is authored"
  );
}
