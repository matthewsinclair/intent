//! **Issue 0460: `st relate` and `st unrelate` are the doors to a thread's
//! `related` links.**
//!
//! Until them the list was readable as a descent and writable by nothing: `set`
//! hands every field a string, the list wants a sequence, and the refusal named
//! a door that did not exist. Where it bit, in Lamplight, three threads named a
//! thread adopted under a new id, the estate's commit gate refused every commit
//! under `intent/`, and no command could repair the links.
//!
//! The refusals read the whole tree back, because a refusal that wrote before
//! refusing is indistinguishable from one that did not by every observation but
//! that one.

use crate::common::{Fixture, sample_thread};
use intentsvcs::facade::{Facade, FacadeError, Outcome};
use intentsvcs::model::Related;
use serde_json::json;

/// Every file in the project but the store's own.
fn files(fx: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
  let mut all = crate::common::tree(fx.root());
  all.retain(|path, _| !path.starts_with("intent/.cache/"));
  all
}

/// A sample thread with no links, since `sample_thread` carries two.
fn unlinked(id: &str) -> intentsvcs::model::Thread {
  let mut thread = sample_thread(id);
  thread.related.clear();
  thread
}

/// ST0001 and ST0002 with no links, and ST0003 carrying one link to ST0338, a
/// thread this project does not have: Lamplight's shape.
fn estate() -> (Fixture, Facade) {
  let fx = Fixture::new();
  fx.write_thread(&unlinked("ST0001"));
  fx.write_thread(&unlinked("ST0002"));
  let mut dangling = unlinked("ST0003");
  dangling.related = vec![Related {
    id: "ST0338".to_string(),
    note: Some("adopted as ST0002".to_string()),
  }];
  fx.write_thread(&dangling);
  let f = fx.facade_on_disk();
  (fx, f)
}

fn events_of(f: &Facade, op: &str) -> Vec<intentsvcs::event::Envelope> {
  f.store()
    .events()
    .expect("events")
    .into_iter()
    .filter(|e| e.op == op)
    .collect()
}

#[test]
fn relate_writes_the_link_to_the_store_canon_and_the_view_under_one_event() {
  let (fx, mut f) = estate();

  let outcome = f
    .st_relate("ST0001", "ST0002", Some("builds on its design"))
    .expect("relate");

  assert!(matches!(outcome, Outcome::Moved), "{outcome:?}");
  assert_eq!(
    f.st_show("ST0001").expect("ST0001").related,
    vec![Related {
      id: "ST0002".to_string(),
      note: Some("builds on its design".to_string()),
    }]
  );
  assert!(
    fx.read_canon("ST0001").contains("builds on its design"),
    "canon carries the link"
  );
  let cover = fx.read("intent/st/ST0001/info.md");
  assert!(
    cover.contains("- ST0002: builds on its design"),
    "the realised view lists the link:\n{cover}"
  );
  let events = events_of(&f, "st.relate");
  assert_eq!(events.len(), 1, "one act, one event");
  assert_eq!(events[0].subject.id, "ST0001");
  assert_eq!(events[0].payload["target"], "ST0002");
  assert_eq!(events[0].payload["note"], "builds on its design");

  // A store rebuilt from what is on disk agrees.
  drop(f);
  assert_eq!(
    fx.facade().st_show("ST0001").expect("ST0001").related.len(),
    1
  );
}

#[test]
fn a_link_is_a_value_so_the_same_note_writes_nothing_and_another_replaces_it() {
  let (_fx, mut f) = estate();
  f.st_relate("ST0001", "ST0002", Some("first"))
    .expect("relate");

  let again = f
    .st_relate("ST0001", "ST0002", Some("first"))
    .expect("again");
  assert_eq!(again.already(), Some("related to ST0002"));
  assert_eq!(
    events_of(&f, "st.relate").len(),
    1,
    "an unchanged link records nothing"
  );

  f.st_relate("ST0001", "ST0002", Some("second"))
    .expect("re-note");
  f.st_relate("ST0001", "ST0002", None)
    .expect("clear the note");
  assert_eq!(
    f.st_show("ST0001").expect("ST0001").related,
    vec![Related {
      id: "ST0002".to_string(),
      note: None,
    }],
    "one link, its note replaced each time and cleared by the last"
  );
  assert_eq!(events_of(&f, "st.relate").len(), 3);
}

#[test]
fn unrelate_drops_a_link_to_a_thread_the_project_no_longer_has() {
  let (fx, mut f) = estate();

  let outcome = f.st_unrelate("ST0003", "ST0338").expect("unrelate");

  assert!(matches!(outcome, Outcome::Moved), "{outcome:?}");
  assert!(f.st_show("ST0003").expect("ST0003").related.is_empty());
  assert!(
    !fx.read_canon("ST0003").contains("ST0338"),
    "canon no longer names it"
  );
  let events = events_of(&f, "st.unrelate");
  assert_eq!(events.len(), 1);
  assert_eq!(events[0].subject.id, "ST0003");
  assert_eq!(events[0].payload["target"], "ST0338");

  // Repointing is the drop and then the link: the repair Lamplight needed.
  f.st_relate("ST0003", "ST0002", Some("adopted from ST0338"))
    .expect("relate the adopted id");
  drop(f);
  assert_eq!(
    fx.facade().st_show("ST0003").expect("ST0003").related,
    vec![Related {
      id: "ST0002".to_string(),
      note: Some("adopted from ST0338".to_string()),
    }]
  );
}

#[test]
fn every_refusal_names_its_cause_and_writes_nothing() {
  let (fx, mut f) = estate();
  let before = files(&fx);

  assert!(matches!(
    f.st_relate("ST0001", "ST0338", None),
    Err(FacadeError::NoSuchRelatedTarget { .. })
  ));
  assert!(matches!(
    f.st_relate("ST0001", "ST0001", None),
    Err(FacadeError::RelatedToItself { .. })
  ));
  assert!(matches!(
    f.st_relate("ST0404", "ST0001", None),
    Err(FacadeError::NoSuchThread { .. })
  ));
  assert!(matches!(
    f.st_unrelate("ST0001", "ST0002"),
    Err(FacadeError::NoSuchRelated { .. })
  ));
  assert!(matches!(
    f.st_unrelate("ST0404", "ST0001"),
    Err(FacadeError::NoSuchThread { .. })
  ));

  assert_eq!(files(&fx), before, "a refusal moved the tree");
  assert!(events_of(&f, "st.relate").is_empty());
  assert!(events_of(&f, "st.unrelate").is_empty());
}

#[test]
fn set_refuses_related_and_names_the_two_verbs() {
  let (_fx, mut f) = estate();
  let address = intentsvcs::address::parse("intent:///threads/ST0001").expect("address");

  let refused = f
    .set(
      &address,
      "related",
      json!([{ "id": "ST0002", "note": "through the wrong door" }]),
    )
    .expect_err("set does not write related");

  let FacadeError::FieldNotWritable { field, why, .. } = &refused else {
    panic!("{refused:?}");
  };
  assert_eq!(field, "related");
  assert!(
    why.contains("intent st relate") && why.contains("intent st unrelate"),
    "the refusal names the doors that exist: {why}"
  );
  assert!(f.st_show("ST0001").expect("ST0001").related.is_empty());
}
