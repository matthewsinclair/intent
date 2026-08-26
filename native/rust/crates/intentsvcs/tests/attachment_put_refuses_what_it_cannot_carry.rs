//! **THE THREE REFUSALS ON `put`'s ATTACHMENT ARM THAT ARE BUILD DECISIONS
//! RATHER THAN CRITERION BEHAVIOUR** (cc, 2026-08-23, arm at `62fdcdfa`).
//!
//! `design.md:271` rules the DIRECTION -- an attachment is authored on disk, so
//! text-in is correct -- and hv ruled that. **Nothing ruled what happens at the
//! three edges below.** I decided them, so they are covered HERE and not under
//! AC-08.5.
//!
//! **THE SEPARATION IS ic's AND THE REASON IS THE GOOD PART:** AC-08.5's row
//! asks whether every entity form in the population is reachable and whether
//! every exclusion cites its ruling. **Putting three unruled build decisions
//! under a criterion hv ruled would smuggle them into its authority** -- the
//! same population defect ic split out of `declared_reach` the same morning,
//! where one field carried both *not built* and *never, by ruling*.
//!
//! # Every arm asserts that NOTHING WAS WRITTEN, and that is not belt-and-braces
//!
//! `st edit` refused and hydrated anyway (`580c1038`, fixed the same week): the
//! refusal was correct, printed correctly, and the write had already happened.
//! **A refusal that mutates is indistinguishable from a refusal that did not,
//! by every observation except the one nobody makes.** So each arm here reads
//! canon back.

mod common;
use common::{Fixture, sample_thread};
use intentsvcs::address::parse;
use intentsvcs::model::Attachment;

const NOT_UTF8: [u8; 5] = [0x23, 0x21, 0xff, 0xfe, 0x0a];

fn fixture() -> Fixture {
  let fx = Fixture::new();
  let mut t = sample_thread("ST0001");
  t.attachments = vec![
    Attachment::new("design.md", "# original\n"),
    // A shell script carrying one non-UTF-8 byte. **The carry names this exact
    // file as "precisely the file that would be silently mangled"**, which is
    // why it is the fixture rather than a `.png`: an opaque attachment whose
    // extension is one canon carries is the case that actually arises.
    Attachment::opaque("mangle.sh", NOT_UTF8.to_vec()),
  ];
  fx.write_thread(&t);
  // Canon naming bytes that are not on disk is a refusal at ingest, so the
  // opaque half needs its sidecar before the fixture can be opened at all.
  let side = fx.root().join("intent/.canon/st/ST0001");
  std::fs::create_dir_all(&side).expect("mkdir sidecar dir");
  std::fs::write(side.join("mangle.sh"), NOT_UTF8).expect("write sidecar");
  fx
}

/// Every attachment on the thread, as `(path, text)`, so an arm can assert the
/// whole set is untouched rather than only the row it aimed at.
fn attachments(f: &mut intentsvcs::facade::Facade) -> Vec<(String, Option<String>)> {
  f.st_show("ST0001")
    .expect("the fixture thread")
    .attachments
    .iter()
    .map(|a| (a.path.clone(), a.text.clone()))
    .collect()
}

fn refusal(url: &str, body: &str) -> (String, Vec<(String, Option<String>)>) {
  let fx = fixture();
  let mut f = fx.facade();
  let before = attachments(&mut f);
  let addr = parse(url).unwrap_or_else(|e| panic!("{url} must PARSE for this to test `put`: {e}"));
  let err = f
    .put(&addr, body)
    .expect_err("this address must be refused");
  let after = attachments(&mut f);
  assert_eq!(before, after, "the refusal wrote something");
  (err.to_string(), after)
}

/// **`?format=json` REFUSES, BECAUSE THE ROUND-TRIP HABIT IS THE HAZARD.**
///
/// The mutation format is the interchange format: `GET ?format=json`, modify,
/// `PUT` the same shape back. Every other address teaches that. **At this one
/// address it would write the attachment's own RECORD into the file as its
/// CONTENT** -- and every other guard passes while it happens, leaving a
/// `sha256` that correctly describes the wrong thing.
#[test]
fn a_json_body_would_write_the_record_into_the_file_so_the_format_is_refused() {
  let (why, _) = refusal(
    "intent:///threads/ST0001/attachments/design.md?format=json",
    "{\"path\":\"design.md\",\"bytes\":11,\"sha256\":\"00\"}",
  );
  assert!(
    why.contains("body is its content"),
    "the refusal must say WHY json is wrong here, not merely that it is: {why}"
  );
}

/// **A BODY CANON WOULD NOT CARRY HAS NO RECORD TO WRITE, AND ON 2026-08-26 ITS
/// SUBJECT MOVED WHILE ITS PROPERTY DID NOT.**
///
/// The property is unchanged and is the whole point: writing a row here that
/// `--to-store` would never have produced puts canon out of step with the disk
/// on the very next carry -- **an artefact the owning pipeline cannot reproduce
/// is already drifting the moment it lands.**
///
/// It used to be asked of an EXTENSION (`script.py`, outside
/// `ATTACHMENT_EXTENSIONS`). The list is gone, so `.py` is carried now and is no
/// longer an example of anything. **What the carrier still refuses is SIZE**, so
/// that is what this door must refuse too -- and it did not, until this test's
/// property was read rather than its subject. `put` accepted an over-cap body
/// and wrote the row; the carrier would have refused the same bytes on the next
/// pass. **The gap was opened by removing the list and closed by keeping the
/// question the list was answering.**
#[test]
fn a_body_over_the_cap_is_refused_rather_than_recorded() {
  let over = "x".repeat(intentsvcs::project::ATTACHMENT_CAP_BYTES as usize + 1);
  let (why, after) = refusal("intent:///threads/ST0001/attachments/notes.md", &over);
  assert!(
    why.contains(&intentsvcs::project::ATTACHMENT_CAP_BYTES.to_string()),
    "the refusal must name the cap, or the operator cannot act on it: {why}"
  );
  assert!(
    !after.iter().any(|(p, _)| p == "notes.md"),
    "the over-cap path was recorded anyway"
  );
}

/// **THE COUNTER-ARM, without which the arm above passes for a door that
/// refuses everything.**
///
/// A `.py` -- the exact path the old list rejected -- is carried now, because
/// an extension decides nothing.
#[test]
fn a_body_under_the_cap_with_any_extension_is_recorded() {
  let fx = fixture();
  let mut f = fx.facade();
  let addr = parse("intent:///threads/ST0001/attachments/script.py").expect("parses");
  f.put(&addr, "print()\n")
    .expect("a small text file is carried whatever it is called");
  assert!(
    attachments(&mut f).iter().any(|(p, _)| p == "script.py"),
    "an extension no list ever held is an attachment like any other"
  );
}

/// **AN OPAQUE ATTACHMENT IS NOT OVERWRITTEN THROUGH A TEXT DOOR.**
///
/// `text: None` is the ONLY marker that the content is bytes, and this door
/// cannot express bytes. Accepting the write would replace a sidecar nobody can
/// read with a string and report success. **A refusal is a one-line change
/// later; a converted file is gone.**
#[test]
fn an_opaque_attachment_is_not_overwritten_through_a_text_door() {
  let (why, after) = refusal(
    "intent:///threads/ST0001/attachments/mangle.sh",
    "#!/bin/sh\necho harmless\n",
  );
  assert!(
    why.contains("carried as bytes"),
    "the refusal must name the FORM as the reason: {why}"
  );
  let (_, text) = after
    .iter()
    .find(|(p, _)| p == "mangle.sh")
    .expect("the opaque row survives");
  assert!(
    text.is_none(),
    "the row stopped being opaque, which is the conversion this refuses"
  );
}

/// **THE STRAY-CANON LIMB OF THE CLASSIFY GUARD, WHICH IS THE ONLY ONE THAT
/// FIRES -- and this test exists to say WHICH LAYER refused.**
///
/// **ic named the trap before I could fall into it.** An arm asserting *a
/// generated view cannot be written as an attachment* passes on
/// [`the_view_names_never_reach_this_guard`] below, **never reaches the guard
/// at all, and is indistinguishable from a green that measured the right
/// layer.** So the guard is driven through the one input that does arrive: a
/// `thread.json` from a v2 tree wearing an attachment's address.
#[test]
fn a_stray_canon_file_is_refused_by_the_classify_guard_and_told_where_to_go() {
  let (why, _) = refusal("intent:///threads/ST0001/attachments/thread.json", "{}\n");
  assert!(
    why.contains("generated from the model"),
    "the classify guard must say what kind of file it refused: {why}"
  );
}

/// **THE REMEDY MUST COMPLETE THE SENTENCE IT IS INTERPOLATED INTO.**
///
/// `EditDisposition::author_with`'s contract is a phrase finishing `author it
/// with ...`, and the `Canon` value was a CLAUSE -- so this guard's only
/// reachable message read *author it with canon is written by the verbs*.
///
/// **SPLIT FROM THE TEST ABOVE SO THE KILL-SETS ARE DISTINCT.** Both assertions
/// lived in one test first, and reverting the guard and reverting the grammar
/// then failed the same single name -- **two different defects with one
/// symptom, which is the thing a mutation proof is supposed to tell apart.**
///
/// **AND NOTHING CAUGHT THE ORIGINAL BECAUSE EACH CONSUMER REACHES EXACTLY ONE
/// OF THE TWO VALUES.** `st edit` appends `.md` to its argument, so it can never
/// classify a file as `Canon` and only ever prints the view arm, which composes.
/// It took a THIRD consumer arriving. This is the pin that stops it recurring.
#[test]
fn the_refusals_remedy_completes_the_sentence_it_is_interpolated_into() {
  let (why, _) = refusal("intent:///threads/ST0001/attachments/thread.json", "{}\n");
  assert!(
    why.contains("author it with the verbs that write canon"),
    "the remedy no longer composes with `author it with`: {why}"
  );
}

/// **A VIEW'S NAME NEVER REACHES `put`, AND THE DEFENCE IS A LAYER LOWER.**
///
/// Asserted as its own fact rather than folded into a refusal test, because the
/// two are not interchangeable: this one would stay green **if `put`'s classify
/// guard were deleted entirely.** Stating the layer is what stops a later
/// reader taking it as evidence about the guard.
#[test]
fn the_view_names_never_reach_this_guard_because_the_parser_refuses_them_first() {
  for view in ["acceptance.md", "info.md"] {
    let url = format!("intent:///threads/ST0001/attachments/{view}");
    let err = parse(&url).expect_err("a view name must not parse as an attachment address");
    let why = err.to_string();
    assert!(
      why.contains("VIEW") && why.contains("no address"),
      "the parser must refuse it AS a view, so the reason survives the layer: {why}"
    );
  }
}
