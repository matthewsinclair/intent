//! **`0262` AND THE FOUR IT WAS SITTING WITH.**
//!
//! `intent st attach` accepted a repo-relative path, returned `ok:` at rc=0, and
//! minted a SECOND attachment row for a file that already had one -- resolving
//! nowhere, removed by no verb, reported by no `doctor`. **The form that broke
//! it is the form the tooling invites**: the commit gate prints
//! `intent st attach <ST> <rel-path> --from <file>`, and *relative* to a reader
//! means relative to the repository.
//!
//! # The row named one path shape; the door was accepting five
//!
//! Driven on `da5919e8` before the fix, eight paths offered and eight rows
//! minted at rc=0: the repo-relative form, an empty string, an absolute path,
//! `../escape.md`, and the unnormalised `./x` and `a/../x`. **The issue is
//! written about the one somebody happened to type.** Every arm below was a
//! real acceptance, not a hypothetical.
//!
//! # Why every arm reads canon back
//!
//! Taken from `attachment_put_refuses_what_it_cannot_carry`, whose reason is
//! the good one: `st edit` once refused correctly and hydrated anyway, so **a
//! refusal that mutates is indistinguishable from one that did not, by every
//! observation except the one nobody makes.**
//!
//! # The control that makes the refusals mean something
//!
//! A suite that only refuses passes just as well when the door refuses
//! EVERYTHING. `a_thread_relative_path_is_still_written` is the arm that fails
//! if this check is too wide, and it is the reason the negative arms are
//! evidence rather than decoration.

use crate::common::{Fixture, sample_thread};
use intentsvcs::address::{Address, Entity};
use intentsvcs::model::Attachment;
use intentsvcs::remedy::Remedy;

fn fixture() -> Fixture {
  let fx = Fixture::new();
  let mut t = sample_thread("ST0001");
  t.attachments = vec![Attachment::new("design.md", "# original\n")];
  fx.write_thread(&t);
  fx
}

fn address(path: &str) -> Address {
  Address {
    authority: None,
    entity: Entity::Attachment {
      thread: "ST0001".to_string(),
      path: path.to_string(),
    },
    format: None,
  }
}

fn paths(f: &mut intentsvcs::facade::Facade) -> Vec<String> {
  f.st_show("ST0001")
    .expect("the fixture thread")
    .attachments
    .iter()
    .map(|a| a.path.clone())
    .collect()
}

/// Offer `path` to the write door and assert it was refused AND wrote nothing.
///
/// Returns `(why, remedy)`. **BOTH, because they are separate obligations**: the
/// `why` names what is wrong and the remedy hands over the corrected path, and a
/// test reading only one would pass while the other said something written for a
/// different caller -- which is exactly what the first build of this check did.
fn refused(path: &str) -> (String, String) {
  let fx = fixture();
  let mut f = fx.facade();
  let before = paths(&mut f);
  let err = f
    .put_attachment(&address(path), b"PROBE\n")
    .expect_err("this path must be refused");
  let after = paths(&mut f);
  assert_eq!(
    before, after,
    "the refusal of `{path}` minted or moved an attachment row"
  );
  (err.to_string(), Remedy::remedy(&err))
}

/// **THE POSITIVE CONTROL, AND IT IS LOAD-BEARING.** Every other arm here is a
/// refusal, and a door that refuses everything would pass all of them. This is
/// the arm that goes red if the rule is drawn too wide.
#[test]
fn a_thread_relative_path_is_still_written() {
  let fx = fixture();
  let mut f = fx.facade();
  f.put_attachment(&address("parity/data/probe.tsv"), b"PROBE\n")
    .expect("a plain thread-relative path is exactly what this verb is for");
  assert!(
    paths(&mut f).contains(&"parity/data/probe.tsv".to_string()),
    "the row must land: {:?}",
    paths(&mut f)
  );
}

/// **`0262` ITSELF.** The refusal computes the corrected spelling rather than
/// describing it, because the reader who typed the wrong one is the reader who
/// has to type the right one.
#[test]
fn a_repo_relative_path_is_refused_and_the_remedy_is_computed() {
  let (why, remedy) = refused("intent/st/ST0001/parity/probe.txt");
  assert!(
    why.contains("relative to the REPOSITORY"),
    "the refusal must name WHICH relative it got: {why}"
  );
  assert!(
    why.contains("intent/st/ST0001/intent/st/ST0001/parity/probe.txt"),
    "the refusal must show what the path WOULD have named, which is what makes it obviously wrong: {why}"
  );
  assert!(
    remedy.contains("write `parity/probe.txt`"),
    "the remedy must SPELL the corrected path rather than describe the rule: {remedy}"
  );
  assert!(
    !remedy.contains("PUT` json"),
    "the remedy must be written for someone who mistyped a PATH, not routed to another door: {remedy}"
  );
}

/// A path naming ANOTHER thread's directory is the same defect and must not
/// slip through on the id not matching.
#[test]
fn a_path_naming_another_threads_directory_is_refused_too() {
  let (why, _) = refused("intent/st/ST0099/design.md");
  assert!(
    why.contains("relative to the REPOSITORY"),
    "same refusal regardless of which thread the prefix names: {why}"
  );
}

#[test]
fn an_empty_path_names_no_file_and_is_refused() {
  let (why, remedy) = refused("");
  assert!(
    why.contains("empty"),
    "the refusal must say the path is empty: {why}"
  );
  assert!(
    remedy.contains("intent st attach ST0001"),
    "with no corrected path to offer, the remedy must show the shape instead: {remedy}"
  );
}

#[test]
fn an_absolute_path_is_refused_and_the_refusal_names_the_thread() {
  let (why, _) = refused("/tmp/absolute.md");
  assert!(why.contains("absolute"), "{why}");
  assert!(
    why.contains("intent/st/ST0001"),
    "the refusal must say where an attachment does live: {why}"
  );
}

#[test]
fn a_path_climbing_out_of_the_thread_is_refused() {
  let (why, _) = refused("../escape.md");
  assert!(
    why.contains("climbs out of the thread"),
    "the refusal must name the escape rather than call it malformed: {why}"
  );
}

/// **`sub/../normal.md` RESOLVES INSIDE THE THREAD AND IS STILL REFUSED**, and
/// that is deliberate: canon stores the STRING, so two spellings of one file are
/// two rows. That is `0262`'s defect reached by a quieter route.
#[test]
fn a_path_that_climbs_and_returns_is_refused_because_canon_stores_the_string() {
  let (why, _) = refused("sub/../normal.md");
  assert!(why.contains("climbs out of the thread"), "{why}");
}

#[test]
fn an_unnormalised_path_is_refused_and_the_remedy_is_the_plain_spelling() {
  let (why, remedy) = refused("./dotslash.md");
  assert!(
    why.contains("not written plainly"),
    "the refusal must explain the SECOND SPELLING hazard, not merely reject it: {why}"
  );
  assert!(
    remedy.contains("write `dotslash.md`"),
    "the remedy must spell the plain form: {remedy}"
  );
}

/// A `.` in the MIDDLE is invisible to a components walk -- Rust normalises it
/// away -- so it is caught by comparing the path against its own plain
/// rendering rather than by looking for a component.
#[test]
fn an_interior_dot_segment_is_refused_by_the_same_rule() {
  let (why, remedy) = refused("parity/./probe.txt");
  assert!(why.contains("not written plainly"), "{why}");
  assert!(remedy.contains("write `parity/probe.txt`"), "{remedy}");
}
