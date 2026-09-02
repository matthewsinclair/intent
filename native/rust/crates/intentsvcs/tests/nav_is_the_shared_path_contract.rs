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

use intentsvcs::address::Entity;
use intentsvcs::form::Loaded;
use intentsvcs::nav::{Landing, Unlanded, View, descents, face_json, kinds, land};

fn loaded() -> Loaded {
  Loaded::load().expect("the shipped form declaration must load")
}

/// Every view the real declaration can produce, so the round trip below is held
/// over the corpus rather than over three hand-picked examples.
fn every_view(l: &Loaded) -> Vec<View> {
  let mut out = vec![
    View::Entities,
    View::Settings,
    View::Help { of: None },
    View::Help {
      of: Some("st".into()),
    },
  ];
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

/// **THE RESERVATION IS HELD AGAINST THE REAL DECLARATION, NOT ASSUMED.**
/// `/settings` is answered ahead of the entity namespace, so a form declared
/// with that name would become unaddressable -- silently, since `View::parse`
/// would keep returning a perfectly good `View::Settings`. This is the alarm.
#[test]
fn no_declared_entity_kind_is_reserved() {
  let l = loaded();
  let declared = kinds(&l);
  assert!(
    !intentsvcs::nav::RESERVED.is_empty(),
    "nothing is reserved, so this test asserts nothing"
  );
  for reserved in intentsvcs::nav::RESERVED {
    assert!(
      !declared.contains(&reserved.to_string()),
      "a form is declared as `{reserved}`, which `/{reserved}` reserves, so its collection is \
       now unreachable by path in both faces; rename one of them"
    );
    // The reservation is only a reservation if it actually wins.
    assert_ne!(
      View::parse(&format!("/{reserved}")),
      Some(View::Collection {
        kind: (*reserved).to_string()
      }),
      "`/{reserved}` parsed as an entity collection, so it is not reserved at all"
    );
  }
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

// ===========================================================================
// hv's RESOLUTION LADDER, 2026-08-31. These arms exist because the change that
// removed the width arm broke NOTHING in either suite -- which was not good
// news. `promote("0164")` went from silently meaning ISSUE to being ambiguous,
// and the TUI's editor, the explorer and the `edit` verb all changed behaviour
// with every test still green, because every existing test drives `ST0056`:
// tagged, and therefore unaffected either way. **A green suite over a corpus
// that cannot exhibit the change is the vacuity class, and it was measured
// here rather than argued.**
// ===========================================================================

/// The one presence rule these arms vary. `land` derives its resolver probe
/// from this same closure, so a test that says *both exist* means it for both
/// questions and cannot make them disagree.
fn only(kinds: &'static [&'static str]) -> impl Fn(&View) -> bool {
  move |v: &View| match v {
    View::Item { kind, .. } => kinds.contains(&kind.as_str()),
    _ => true,
  }
}

/// **A BARE NUMBER NAMING TWO LIVE THINGS IS REFUSED, AND THE CANDIDATES COME
/// BACK AS ADDRESSES.** Before the ladder this opened the ISSUE, chosen by
/// digit count, with nothing said.
#[test]
fn a_bare_number_that_names_two_things_lands_nowhere_and_names_both() {
  let Landing::Root(Unlanded::Ambiguous { input, candidates }) =
    land("0059", only(&["thread", "issue"]))
  else {
    panic!("two live candidates is an ambiguity, not a landing");
  };
  assert_eq!(input, "0059");
  assert_eq!(
    candidates.iter().map(|a| a.to_url()).collect::<Vec<_>>(),
    vec![
      "intent:///threads/ST0059".to_string(),
      "intent:///issues/0059".to_string()
    ],
    "the ladder's order fixes the report's order, and each candidate is rendered by the grammar"
  );
}

/// **ONE LIVE CANDIDATE RESOLVES, WHICH IS THE HALF THAT MAKES THE REFUSAL
/// ABOVE TOLERABLE.** A door that refused every bare number would be a
/// regression sold as a fix; this is what makes the ladder a resolution rather
/// than a narrowing.
#[test]
fn a_bare_number_that_names_one_live_thing_lands_on_it() {
  assert_eq!(
    land("0059", only(&["issue"])),
    Landing::At(View::Item {
      kind: "issue".to_string(),
      id: "0059".to_string()
    }),
    "only the issue is there, so the number is not ambiguous IN THIS PROJECT"
  );
  assert_eq!(
    land("0059", only(&["thread"])),
    Landing::At(View::Item {
      kind: "thread".to_string(),
      id: "ST0059".to_string()
    }),
    "and the same spelling lands on the thread when that is the one that exists"
  );
}

/// **NAMING NOTHING IS ITS OWN ANSWER AND SAYS WHERE IT LOOKED.** Reporting
/// `no such thread` here would answer about one of the two things the caller
/// might have meant -- the precedence defect one layer up.
#[test]
fn a_bare_number_that_names_nothing_reports_both_places_it_looked() {
  let Landing::Root(Unlanded::Unresolvable { searched, .. }) = land("0059", only(&[])) else {
    panic!("no live candidate is unresolvable, and is not the same fact as absent");
  };
  assert_eq!(
    searched.iter().map(|a| a.to_url()).collect::<Vec<_>>(),
    vec![
      "intent:///threads/ST0059".to_string(),
      "intent:///issues/0059".to_string()
    ]
  );
}

/// **A TAGGED SPELLING NEVER CONSULTS PRESENCE TO DECIDE WHAT IT NAMES**, so
/// `s59` still lands on the thread in a project where only the issue exists.
/// The kind came from the argument; presence only decides `At` against
/// `Absent`.
#[test]
fn a_tagged_spelling_is_not_re_decided_by_what_happens_to_exist() {
  assert!(
    matches!(
      land("s59", only(&["issue"])),
      Landing::Root(Unlanded::Absent { kind, .. }) if kind == "thread"
    ),
    "`s59` names a thread; the thread is missing, which is ABSENT rather than a resolution to the issue"
  );
}

/// **EVERY ENTITY `view_for` GIVES AN ITEM VIEW REBUILDS FROM THAT VIEW.** The
/// TUI editor holds a kind and an id and has to get back to an address; this
/// holds the two directions together so neither grows an arm the other lacks.
#[test]
fn an_item_view_round_trips_to_the_entity_it_came_from() {
  for entity in [
    Entity::Thread {
      id: "ST0059".to_string(),
    },
    Entity::Issue {
      id: "0059".to_string(),
    },
  ] {
    let view = intentsvcs::nav::view_for(&entity).expect("both forms have item views");
    assert_eq!(
      intentsvcs::nav::entity_for_item(&view).as_ref(),
      Some(&entity),
      "the inverse must return the entity the view was built from"
    );
  }

  // A view that is not an item has no entity to rebuild, and saying so is not
  // the same as failing.
  assert_eq!(
    intentsvcs::nav::entity_for_item(&View::Collection {
      kind: "thread".to_string()
    }),
    None
  );
}
