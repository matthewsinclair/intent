//! `AT-17.12`, the address half: `intent explore [address]` covering
//! `AC-17.12`.
//!
//! **The ROUND-TRIP half of `AC-17.12` is not here** -- `View <-> path` over
//! every view the declaration can produce is asserted in
//! `nav_is_the_shared_path_contract.rs`, and repeating it would be a second
//! home for one property. This file asserts the door hv ruled open on
//! 2026-08-30: an operator's spelling becoming a place to open.
//!
//! # What could make every assertion below pass while the feature is broken
//!
//! `nav::land` takes its presence test as a closure. **A `land` that ignored
//! that closure entirely would satisfy the classification tests, the two
//! spellings test and the vocabulary test** -- all of them drive present
//! entities or unparseable input, and none of them can tell a consulted
//! predicate from an unread argument. That is why
//! `the_presence_test_is_actually_consulted` exists and why it is the control
//! rather than a nicety: it is the only assertion here that fails if the
//! injected predicate is dead.

use intentsvcs::form::Loaded;
use intentsvcs::nav::{self, Landing, Unlanded, View, descents};

/// Every entity form the address grammar can express, with an address that
/// produces it.
///
/// **Thirteen rows because `Entity` has thirteen forms.** The guard against a
/// fourteenth is not this list -- it is `view_for`'s match, which does not
/// compile until a new form is classified. This list says what the
/// classification IS, so a change of mind about an existing form is visible
/// here rather than only in a diff.
const FORMS: &[(&str, &str)] = &[
  ("threads", "intent:///threads"),
  ("issues", "intent:///issues"),
  ("thread", "intent:///threads/ST0056"),
  ("issue", "intent:///issues/0142"),
  ("wp-collection", "intent:///threads/ST0056/wp"),
  ("ac-collection", "intent:///threads/ST0056/ac"),
  ("wp", "intent:///threads/ST0056/wp/01"),
  ("ac", "intent:///threads/ST0056/ac/AC-17.1"),
  ("at", "intent:///threads/ST0056/at/AT-17.1"),
  (
    "attachment",
    "intent:///threads/ST0056/attachments/design.md",
  ),
  ("node", "intent:///nodes/ic"),
  ("node-inbox", "intent:///nodes/ic/inbox/vc/2026-08-30"),
  ("event", "intent:///events/1"),
];

/// The forms this surface renders. Every other form must land at the root
/// SAYING SO, never claiming the address named nothing.
const WITH_VIEWS: &[&str] = &[
  "threads",
  "issues",
  "thread",
  "issue",
  "wp-collection",
  "ac-collection",
];

/// A presence test that says yes to everything -- for the arms whose subject is
/// the MAPPING rather than the store.
fn anything(_: &View) -> bool {
  true
}

#[test]
fn every_addressable_form_either_opens_a_view_or_is_refused_by_name() {
  assert_eq!(FORMS.len(), 13, "the address grammar's form set has moved");
  for (form, url) in FORMS {
    let landing = nav::land(url, anything);
    if WITH_VIEWS.contains(form) {
      assert!(
        matches!(landing, Landing::At(_)),
        "`{url}` is a {form}, which this surface renders, and it did not open a view: {landing:?}"
      );
    } else {
      match landing {
        Landing::Root(Unlanded::NoView { form: named, .. }) => {
          assert_eq!(&named, form, "the refusal named the wrong form for `{url}`")
        }
        other => panic!("`{url}` is a {form}, which has no view, and it did not say so: {other:?}"),
      }
    }
  }
}

/// **THE CONTROL FOR THIS WHOLE FILE.** Everything else drives present
/// entities or unparseable text, so a `land` that never called its presence
/// argument would pass all of it.
#[test]
fn the_presence_test_is_actually_consulted() {
  let url = "intent:///threads/ST0056";
  let found = nav::land(url, |_| true);
  let missing = nav::land(url, |_| false);
  assert_ne!(
    found, missing,
    "the presence test is not consulted -- `land` returns the same landing whether the entity is \
     there or not, so every other assertion in this file is about a grammar rather than a project"
  );
  assert!(matches!(found, Landing::At(_)));
  assert!(matches!(missing, Landing::Root(Unlanded::Absent { .. })));
}

/// **A WELL-FORMED ADDRESS FOR SOMETHING THAT IS NOT THERE IS THE CASE THE
/// GRAMMAR CANNOT SEE.** `promote` never reads the store, so `ST9999` resolves
/// perfectly; and the realiser deliberately renders a form that will not load
/// with its field names intact, so an absent thread and an empty one paint the
/// same screen.
#[test]
fn a_well_formed_address_for_something_absent_opens_the_root_and_says_so() {
  let landing = nav::land("ST9999", |_| false);
  let Landing::Root(Unlanded::Absent { kind, view, .. }) = &landing else {
    panic!("a well-formed id for an absent thread did not report absence: {landing:?}");
  };
  assert_eq!(kind, "thread");
  assert_eq!(
    view,
    &View::Item {
      kind: "thread".to_string(),
      id: "ST9999".to_string()
    },
    "the landing must carry the view it WOULD have opened, so the message can name it"
  );
  assert!(
    landing.to_string_lossy().contains("ST9999"),
    "the reason on screen must name what the operator typed"
  );
}

/// **NAMING TWO THINGS IS NOT NAMING NOTHING.** `56` is `IdError::Ambiguous`,
/// which is the ambiguity `AC-17.6` exists to record -- and it is hv's own
/// natural spelling, so the message has to be the useful one.
#[test]
fn a_bare_short_number_names_two_things_and_the_message_says_which() {
  let landing = nav::land("56", anything);
  let Landing::Root(Unlanded::Unreadable { why, .. }) = &landing else {
    panic!("`56` is ambiguous and was not reported as unreadable: {landing:?}");
  };
  assert!(
    why.contains("56"),
    "the parser's own words must reach the operator, and they did not mention the input: {why}"
  );
  let nothing = nav::land("banana", anything);
  let Landing::Root(Unlanded::Unreadable { why: other, .. }) = &nothing else {
    panic!("`banana` names nothing and was not reported as unreadable: {nothing:?}");
  };
  assert_ne!(
    why, other,
    "naming two things and naming nothing share a variant BECAUSE the parser tells them apart in \
     words -- if the words are the same, the variant is hiding a distinction rather than \
     delegating it"
  );
}

/// **A REAL ADDRESS MUST NOT BE REPORTED AS NAMING NOTHING.** Telling an
/// operator that `intent:///nodes/vc` named nothing sends them hunting for a
/// thing they already have.
#[test]
fn a_real_address_with_no_view_is_not_reported_as_naming_nothing() {
  let landing = nav::land("intent:///nodes/vc", anything);
  let Landing::Root(Unlanded::NoView { form, .. }) = &landing else {
    panic!("a node address was not reported as a viewless form: {landing:?}");
  };
  assert_eq!(
    form, "node",
    "the message must name the form it cannot show"
  );
  let said = landing.to_string_lossy();
  assert!(
    said.contains("node"),
    "the operator is told nothing useful: {said}"
  );
}

/// Both spellings hv named reach ONE door, so they must reach one answer.
#[test]
fn the_two_spellings_hv_named_reach_the_same_view() {
  let bare = nav::land("ST0056", anything);
  let url = nav::land("intent:///threads/ST0056", anything);
  assert_eq!(
    bare, url,
    "`ST0056` and its URL form landed in different places"
  );
  assert!(matches!(bare, Landing::At(View::Item { .. })));
}

/// **THE TRANSLATION IS HELD AGAINST THE DECLARATION, BECAUSE NOTHING DERIVES
/// IT.** An address says `/threads/ST0056/ac`; a view path says
/// `/thread/ST0056/criteria`. Both are ratified and neither produces the other,
/// so the mapping is authored -- and an authored mapping beside a declaration
/// is precisely the shape that goes stale in silence.
#[test]
fn every_child_view_this_maps_to_is_a_descent_the_declaration_carries() {
  let loaded = Loaded::load().expect("the form declaration must load");
  let declared: Vec<String> = descents(&loaded, "thread")
    .into_iter()
    .map(|d| d.field)
    .collect();
  assert!(
    !declared.is_empty(),
    "the declaration reports no descents for `thread`, so this test cannot fail and proves nothing"
  );
  for url in ["intent:///threads/ST0056/wp", "intent:///threads/ST0056/ac"] {
    let Landing::At(View::Children { field, kind, .. }) = nav::land(url, anything) else {
      panic!("`{url}` did not open a child view");
    };
    assert_eq!(kind, "thread");
    assert!(
      declared.contains(&field),
      "`{url}` maps to `/thread/../{field}`, which the form declaration does not declare as a \
       descent -- the address vocabulary and the view vocabulary have drifted. Declared: {declared:?}"
    );
  }
}

/// Small helper so the assertions above can read the operator-facing text
/// without every one of them importing `std::error::Error`.
trait Says {
  fn to_string_lossy(&self) -> String;
}

impl Says for Landing {
  fn to_string_lossy(&self) -> String {
    match self {
      Landing::At(v) => v.path(),
      Landing::Root(why) => why.to_string(),
    }
  }
}
