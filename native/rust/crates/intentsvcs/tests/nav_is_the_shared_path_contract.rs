//! `AC-17.12`: the TUI's view stack and the browser's URL are the same
//! sequence, from the same declarations.
//!
//! **THESE TESTS MOVED DOWN A CRATE WITH THE CONTRACT THEY COVER**
//! (2026-08-30, vc's ruling). `View`, its path round trip, and the derivation
//! of entity kinds and descents were first written in
//! `intent-cli/src/tui/nav.rs` -- one crate too high, for the same reason the
//! form derivation was: `intentd` depends on `intentsvcs` and NOT on the CLI,
//! so the web face could not have reached them. **If it derived its own
//! segments the two faces would stop being the same sequence, and `AC-17.12`
//! would be unsatisfiable by construction.**
//!
//! The view STACK stayed in the face, because it is state rather than contract
//! -- the web's equivalent is browser history, which it already has.
//!
//! Moving code without moving its tests is how a shared home ends up less
//! covered than the private one it replaced, so this file exists at the moment
//! of the move rather than after it.

use intentsvcs::form::Loaded;
use intentsvcs::nav::{View, descents, face_json, kinds};

fn loaded() -> Loaded {
  Loaded::load().expect("the shipped form declaration must load")
}

/// Every view the real declaration can produce, so the round trip below is held
/// over the corpus rather than over three hand-picked examples.
fn every_view(l: &Loaded) -> Vec<View> {
  let mut out = vec![View::Entities];
  for kind in kinds(l) {
    out.push(View::Collection { kind: kind.clone() });
    out.push(View::Item {
      kind: kind.clone(),
      id: "ST0056".into(),
    });
    for d in descents(l, &kind) {
      out.push(View::Children {
        kind: kind.clone(),
        id: "ST0056".into(),
        field: d.field,
      });
    }
  }
  out
}

#[test]
fn the_declaration_is_not_empty_and_neither_is_the_root() {
  let l = loaded();
  assert!(
    !kinds(&l).is_empty(),
    "no entity kinds, so every walk below is over nothing"
  );
  assert!(
    every_view(&l).len() > kinds(&l).len(),
    "no view beyond the root and its collections, so the round trip asserts almost nothing"
  );
}

/// **THE PROPERTY `AC-17.12` NAMES.** Held as a round trip over every
/// constructible view, so neither face can invent a level the other cannot
/// express.
#[test]
fn every_view_round_trips_through_its_path() {
  let l = loaded();
  let views = every_view(&l);
  assert!(!views.is_empty());
  for v in &views {
    let p = v.path();
    assert_eq!(
      View::parse(&p).as_ref(),
      Some(v),
      "{v:?} rendered to {p:?} and did not parse back to itself"
    );
  }
}

/// A path that names nothing is refused AS A SPELLING rather than resolved to
/// something near it.
#[test]
fn a_path_that_names_nothing_is_refused_rather_than_guessed() {
  for bad in ["", "thread", "/thread//ST0056", "/a/b/c/d", "//"] {
    assert_eq!(
      View::parse(bad),
      None,
      "{bad:?} parsed to a view and should not have"
    );
  }
}

/// **NO SEGMENT IS INVENTED, PLURALISED OR PRETTIFIED.** A spelling rule is a
/// second home for naming, it breaks on the first kind that does not take an
/// `-s`, and it has to be inverted to route.
#[test]
fn every_path_segment_is_a_name_the_declaration_already_carries() {
  let l = loaded();
  let declared: Vec<String> = kinds(&l);
  for kind in &declared {
    let fields: Vec<String> = descents(&l, kind).into_iter().map(|d| d.field).collect();
    let v = View::Collection { kind: kind.clone() };
    assert_eq!(
      v.path(),
      format!("/{kind}"),
      "a collection path is not the declared kind"
    );
    for f in fields {
      let child = View::Children {
        kind: kind.clone(),
        id: "X".into(),
        field: f.clone(),
      };
      assert!(
        child.path().ends_with(&format!("/{f}")),
        "a child path does not end in the field the form declares"
      );
    }
  }
}

/// **THE DISCRIMINATOR, DRIVEN ON THE PAIR THAT MOTIVATES IT.** `wps` and
/// `fiat` are both `button` rows on the thread form; one is a collection and one
/// is a verb. A check that only asserted `wps` is present would pass for a rule
/// that admitted everything.
#[test]
fn the_discriminator_separates_a_real_descent_from_a_real_action() {
  let l = loaded();
  let form = l.form("thread").expect("the thread form must be declared");
  let buttons: Vec<&str> = form
    .fields
    .iter()
    .filter(|f| f.widget == "button")
    .map(|f| f.name.as_str())
    .collect();
  assert!(
    buttons.contains(&"wps"),
    "the fixture assumes `wps` is a button row"
  );
  assert!(
    buttons.contains(&"fiat"),
    "the fixture assumes `fiat` is a button row -- without it this test cannot show the \
     discriminator refusing anything"
  );

  let declared = descents(&l, "thread");
  let found: Vec<&str> = declared.iter().map(|d| d.field.as_str()).collect();
  assert!(
    found.contains(&"wps"),
    "`wps` is an array of a $ref and must be a descent"
  );
  assert!(
    !found.contains(&"fiat"),
    "`fiat` is an action, not a collection; descending into it would descend into a verb"
  );
}

/// **THE FRAGMENT IS RESOLVED, AND THIS IS THE ONLY THING THAT SAYS SO.** `wp`
/// resolves to `thread.schema.json#/$defs/WorkPackage`. Ignoring the fragment
/// yields the THREAD schema, and `descents(l, "wp")` still comes back empty --
/// the right answer from the wrong lookup -- because the wp form's only button
/// row misses at both levels. Asserted on the properties themselves so the
/// resolution is checked rather than its consequence.
#[test]
fn a_face_that_is_a_definition_resolves_through_its_fragment() {
  let wp = face_json("wp").expect("wp must resolve to a face");
  let props = wp
    .get("properties")
    .expect("the WorkPackage definition must carry properties");
  assert!(
    props.get("seq").is_some(),
    "the resolved face is missing WorkPackage's own `seq`"
  );
  assert!(
    props.get("wps").is_none(),
    "the resolved face carries `wps`, which belongs to THREAD -- the fragment was ignored and \
     every wp descent would be read off the wrong schema"
  );
  let thread = face_json("thread").expect("thread must resolve to a face");
  assert!(
    thread
      .get("properties")
      .and_then(|p| p.get("wps"))
      .is_some(),
    "the thread face is missing `wps`, so the contrast above proves nothing"
  );
}

/// A descent must name the kind it opens onto, or the level below it cannot be
/// built.
#[test]
fn every_descent_names_the_kind_it_opens_onto() {
  let l = loaded();
  let mut total = 0usize;
  for kind in kinds(&l) {
    for d in descents(&l, &kind) {
      assert!(
        !d.child.is_empty(),
        "{kind}.{} declares no child kind",
        d.field
      );
      assert!(!d.label.is_empty(), "{kind}.{} declares no label", d.field);
      total += 1;
    }
  }
  assert!(
    total > 0,
    "no descent was found anywhere, so this test asserted nothing"
  );
}

/// **THE POINT OF THE MOVE, ASSERTED RATHER THAN ASSUMED.** If this passes from
/// a crate that knows nothing about any renderer, the TUI, the daemon and the
/// menubar app can all reach it -- which is what makes `AC-17.12`'s agreement
/// structural. A copy back up into a face would pass its own tests and quietly
/// reintroduce the second home.
#[test]
fn the_contract_needs_no_renderer_and_no_terminal() {
  let v = View::Children {
    kind: "thread".into(),
    id: "ST0056".into(),
    field: "wps".into(),
  };
  assert_eq!(v.path(), "/thread/ST0056/wps");
  assert_eq!(View::parse("/thread/ST0056/wps"), Some(v));
}
