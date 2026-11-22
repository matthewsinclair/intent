//! Issues 0718 and 0715: **THE ATTACHMENT DOOR WRITES ONE FILE, AND THE ONE
//! THE THREAD HOLDS.**
//!
//! 0718 (cc's ruling): no thread holds two attachments one disk stores as one
//! file, so the door refuses the second by the held one's name. 0715: an attach
//! over a held attachment replaces it in the store, the canon and the realised
//! file in the one verb, the realised file first, and refuses before any write
//! when that file holds an edit the store does not carry.

use crate::common::{Fixture, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::facade::FacadeError;

/// The sample thread's own attachment, as canon and the realised file hold it.
const HELD: &str = "reference.md";
const HELD_TEXT: &str = "# Reference\n\nA quokka.\n";

fn address(path: &str) -> intentsvcs::address::Address {
  parse(&format!("intent:///threads/ST0001/attachments/{path}")).expect("an attachment address")
}

/// The sample thread, its held attachment realised on disk as the store has it.
fn realised() -> Fixture {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread("ST0001"));
  fx.write_file(&format!("intent/st/ST0001/{HELD}"), HELD_TEXT);
  fx
}

fn held_text(facade: &intentsvcs::facade::Facade) -> Option<String> {
  facade
    .st_show("ST0001")
    .expect("the thread")
    .attachments
    .iter()
    .find(|a| a.path == HELD)
    .and_then(|a| a.text.clone())
}

#[test]
fn a_second_attachment_one_disk_stores_as_a_held_ones_file_is_refused_by_the_held_name() {
  let fx = realised();
  let mut facade = fx.facade();
  match facade.put_attachment(&address("Reference.md"), b"# shadow\n") {
    Err(FacadeError::OneFileAttachment { thread, path, held }) => {
      assert_eq!(
        (thread.as_str(), path.as_str(), held.as_str()),
        ("ST0001", "Reference.md", HELD)
      );
    }
    other => panic!("the door must refuse the case pair by the held name: {other:?}"),
  }
  let paths: Vec<&str> = facade
    .st_show("ST0001")
    .expect("the thread")
    .attachments
    .iter()
    .map(|a| a.path.as_str())
    .collect();
  assert!(
    !paths.contains(&"Reference.md"),
    "nothing of the refused name reached canon: {paths:?}"
  );
  assert_eq!(
    std::fs::read_to_string(fx.path(&format!("intent/st/ST0001/{HELD}"))).expect("the held file"),
    HELD_TEXT,
    "the held file is untouched, on a disk that folds case as on one that does not"
  );
}

#[test]
fn an_attach_over_a_held_attachment_replaces_the_realised_file_with_the_store() {
  let fx = realised();
  let mut facade = fx.facade();
  facade
    .put_attachment(&address(HELD), b"# Reference, replaced\n")
    .expect("an attach over a held attachment is taken");
  assert_eq!(
    std::fs::read_to_string(fx.path(&format!("intent/st/ST0001/{HELD}")))
      .expect("the realised file"),
    "# Reference, replaced\n",
    "the realised file holds the new bytes, so a watcher carries nothing old back"
  );
  assert_eq!(
    held_text(&facade).as_deref(),
    Some("# Reference, replaced\n")
  );
}

#[test]
fn an_attach_over_an_unsynced_edit_is_refused_before_any_write() {
  let fx = realised();
  let mut facade = fx.facade();
  // The hand edit lands after the store has read the tree, so the store does
  // not carry it.
  let realised = fx.path(&format!("intent/st/ST0001/{HELD}"));
  std::fs::write(&realised, "# Reference, edited by hand\n").expect("hand edit");
  match facade.put_attachment(&address(HELD), b"# Reference, replaced\n") {
    Err(FacadeError::UnsyncedAttachment { thread, path }) => {
      assert_eq!(
        (thread.as_str(), path.as_str()),
        ("ST0001", "intent/st/ST0001/reference.md")
      );
    }
    other => panic!("an attach over an unsynced edit must be refused: {other:?}"),
  }
  assert_eq!(
    std::fs::read_to_string(&realised).expect("the realised file"),
    "# Reference, edited by hand\n",
    "the edit is kept"
  );
  assert_eq!(
    held_text(&facade).as_deref(),
    Some(HELD_TEXT),
    "the store is kept"
  );
}

#[test]
fn a_new_attachment_over_a_file_the_store_does_not_carry_is_refused_before_any_write() {
  let fx = realised();
  let mut facade = fx.facade();
  // The author's file lands after the store has read the tree, so the store
  // does not carry it.
  let authored = fx.path("intent/st/ST0001/notes.md");
  std::fs::write(&authored, "# the author's own notes\n").expect("write notes");
  match facade.put_attachment(&address("notes.md"), b"# attached over them\n") {
    Err(FacadeError::UnsyncedAttachment { thread, path }) => {
      assert_eq!(
        (thread.as_str(), path.as_str()),
        ("ST0001", "intent/st/ST0001/notes.md")
      );
    }
    other => panic!("an attach over a file the store does not carry must be refused: {other:?}"),
  }
  assert_eq!(
    std::fs::read_to_string(&authored).expect("the author's file"),
    "# the author's own notes\n",
    "the author's file is kept"
  );
}

/// Issue I1d5a3b5: a new attachment on a realised thread gets its realised
/// file in the one verb, and the drain names it as written, so a commit of what
/// the verb printed carries the file with no `organize --apply` between.
#[test]
fn a_new_attachment_on_a_realised_thread_writes_its_file_and_names_it() {
  const NEW: &str = "notes/new.md";
  const NEW_TEXT: &str = "# New\n\nWritten by the attach.\n";
  let fx = realised();
  let mut facade = fx.facade();
  facade.take_landed();
  facade
    .put_attachment(&address(NEW), NEW_TEXT.as_bytes())
    .expect("a new attachment is taken");
  let file = format!("intent/st/ST0001/{NEW}");
  assert_eq!(
    std::fs::read_to_string(fx.path(&file)).expect("the realised file"),
    NEW_TEXT,
    "the realised thread holds the attached bytes"
  );
  let landed = facade.take_landed();
  assert!(
    landed.wrote.contains(&file),
    "the verb names the file it wrote: {:?}",
    landed.wrote
  );
}

/// Issue I1d5a3b5's other half: realised is the record's answer, not the
/// disk's. A thread whose record is not realised gets no file, though a
/// directory stands where its tree would be.
#[test]
fn a_new_attachment_on_an_unrealised_thread_writes_no_file() {
  const NEW: &str = "new.md";
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.realised = false;
  fx.write_thread(&thread);
  fx.write_file(
    "intent/st/ST0001/left-by-hand.md",
    "# a hand mkdir's file\n",
  );
  let mut facade = fx.facade();
  facade
    .put_attachment(&address(NEW), b"# New\n")
    .expect("a new attachment is taken");
  assert!(
    !fx.path(&format!("intent/st/ST0001/{NEW}")).exists(),
    "an unrealised thread's directory was written into"
  );
}
