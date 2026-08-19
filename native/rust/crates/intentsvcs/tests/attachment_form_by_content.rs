//! AT-03.2 / ST0057 AC-03.2: **form follows content, BOTH WAYS -- no opaque
//! attachment is stored inline, and no text attachment is stored as a sibling
//! file.**
//!
//! # One rule, and the row says why it is one rule
//!
//! _A check that only catches one direction passes an estate that has drifted
//! the other way._ So the two halves are asserted together over the same
//! population, and each has its own positive control: a check that could only
//! ever fail one way is half a check wearing a whole one's name.
//!
//! # THE TWO DIRECTIONS HAVE VERY DIFFERENT ESTATE DENOMINATORS, AND SAYING SO
//! IS THE POINT
//!
//! Measured at the time of writing: **280 attachments across 57 threads, of
//! which 0 are opaque, and no `.canon/st/<ID>/` directory exists at all.**
//!
//! - **"no opaque stored inline" has a denominator of ZERO over the estate.**
//!   Nothing here can exercise it, and a green on that half alone would be the
//!   vacuous pass this thread keeps paying for -- right verb, right depth, a
//!   population that cannot contain the failure. It is driven by a fixture and
//!   the zero is printed as a zero.
//! - **"no text stored as a sibling" has a denominator of 280.** That half IS
//!   exercised by the real estate, and it is exercised on every run.
//!
//! **The asymmetry is the interesting part rather than a caveat.** One rule
//! stated in one sentence turns out to be two claims with populations three
//! orders of magnitude apart, and only one of them is currently falsifiable by
//! the corpus. A single count over "attachments checked" would have hidden
//! that behind a number that looks like coverage.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::ingest;
use intentsvcs::model::Attachment;
use intentsvcs::project::{Project, canon_blob_rel};
use serde_json::Value;
use testkit::repo_root;

const NOT_UTF8: &[u8] = b"\x89PNG\r\n\x1a\n\x00\xff\xfe";

/// Where an attachment's content actually IS, read from the artefacts rather
/// than from the model -- because the model is what the rule is a claim about.
#[derive(Debug, PartialEq, Eq)]
enum Stored {
  Inline,
  Sidecar,
  /// Both, which is the drift no single-direction check would catch.
  Both,
  Neither,
}

fn stored(json: &Value, index: usize, sidecar_exists: bool) -> Stored {
  let inline = json["attachments"][index].get("text").is_some();
  match (inline, sidecar_exists) {
    (true, false) => Stored::Inline,
    (false, true) => Stored::Sidecar,
    (true, true) => Stored::Both,
    (false, false) => Stored::Neither,
  }
}

/// **THE DISCRIMINATING FIXTURE: one of each, in one thread, checked both
/// ways.**
#[test]
fn a_text_attachment_goes_inline_and_an_opaque_one_goes_to_a_sidecar() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments = vec![
    Attachment::new("notes.md", "# Notes\n"),
    Attachment::opaque("logo.png", NOT_UTF8),
  ];
  fx.write_thread(&thread);

  let bundle = intentsvcs::export::Bundle::new("form", vec![thread], Vec::new(), Vec::new());
  for (rel, body) in intentsvcs::export::canon_parts(&bundle).expect("canon serialises") {
    fx.write_file(&format!("intent/{rel}"), &body);
  }
  let blobs = intentsvcs::export::canon_blobs(&bundle);
  for (rel, bytes) in &blobs {
    let path = fx.path(&format!("intent/{rel}"));
    std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
    std::fs::write(&path, bytes).expect("write");
  }

  let json: Value = serde_json::from_str(&fx.read_canon("ST0001")).expect("canon is JSON");
  let text_sidecar = fx
    .path(&format!("intent/{}", canon_blob_rel("ST0001", "notes.md")))
    .exists();
  let opaque_sidecar = fx
    .path(&format!("intent/{}", canon_blob_rel("ST0001", "logo.png")))
    .exists();

  assert_eq!(
    stored(&json, 0, text_sidecar),
    Stored::Inline,
    "the TEXT attachment is not stored inline and only inline. A text attachment given a sibling \
     file is the direction a check written for binaries would never look at"
  );
  assert_eq!(
    stored(&json, 1, opaque_sidecar),
    Stored::Sidecar,
    "the OPAQUE attachment is not stored as a sibling and only as a sibling"
  );

  // **Both halves must have been exercised by this fixture**, or one of the
  // assertions above passed over an attachment that was not there.
  assert_eq!(json["attachments"][0]["path"], "notes.md");
  assert_eq!(json["attachments"][1]["path"], "logo.png");
  assert_eq!(
    blobs.len(),
    1,
    "exactly one sidecar for two attachments -- {} emitted, so the rule is not discriminating",
    blobs.len()
  );
}

/// **POSITIVE CONTROL FOR THE HALF THE ESTATE CANNOT EXERCISE.**
///
/// The estate holds no opaque attachment, so "no opaque stored inline" can only
/// be shown to be checkable by making one and confirming it does not go inline.
#[test]
fn an_opaque_attachment_cannot_reach_the_inline_field_at_all() {
  let att = Attachment::opaque("logo.png", NOT_UTF8);
  let json: Value = serde_json::to_value(&att).expect("an attachment serialises");

  assert!(
    json.get("text").is_none(),
    "opaque bytes reached the inline field: {json}"
  );
  assert!(
    json.get("blob").is_none(),
    "the bytes were serialised into the thread's canon under another name, which is the same \
     defect wearing a different field: {json}"
  );
  assert_eq!(
    json["bytes"],
    NOT_UTF8.len(),
    "the record still describes the content it is not carrying, or the sidecar cannot be checked \
     against anything"
  );

  // The control: a TEXT attachment at the same seam DOES reach it, so the
  // absence above is discrimination rather than a serialiser that emits
  // nothing.
  let text: Value = serde_json::to_value(Attachment::new("notes.md", "# Notes\n")).expect("ok");
  assert_eq!(
    text["text"], "# Notes\n",
    "a text attachment does not reach the inline field either, so the assertion above is vacuous"
  );
}

/// **A TEXT ATTACHMENT WITH A SIDECAR IS THE OTHER DIRECTION, AND IT IS
/// CONSTRUCTED BECAUSE NOTHING PRODUCES IT.**
///
/// No code path writes this today -- `canon_blobs` emits only when there are
/// bytes to emit. That is exactly why the check must exist: the drift it names
/// is one nobody would introduce deliberately, so nothing else would notice it.
#[test]
fn a_text_attachment_that_also_has_a_sidecar_is_detected() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments = vec![Attachment::new("notes.md", "# Notes\n")];
  fx.write_thread(&thread);

  let bundle = intentsvcs::export::Bundle::new("form", vec![thread], Vec::new(), Vec::new());
  for (rel, body) in intentsvcs::export::canon_parts(&bundle).expect("canon serialises") {
    fx.write_file(&format!("intent/{rel}"), &body);
  }

  let rel = canon_blob_rel("ST0001", "notes.md");
  let path = fx.path(&format!("intent/{rel}"));
  std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
  std::fs::write(&path, b"# Notes\n").expect("plant the sidecar");

  let json: Value = serde_json::from_str(&fx.read_canon("ST0001")).expect("canon is JSON");
  assert_eq!(
    stored(&json, 0, path.exists()),
    Stored::Both,
    "a text attachment carrying BOTH an inline body and a sibling file was not detected as drift. \
     Two copies of one file's content, and nothing says which is authoritative"
  );
}

/// What the sweep found: the two conforming populations, and the drift.
struct Tally {
  inline: usize,
  sidecar: usize,
  problems: Vec<String>,
}

/// The rule, applied to every attachment in a canon.
///
/// **A FUNCTION, so the estate arm and the planted-violation arm run THE SAME
/// CODE.** The estate contains no violation -- 280 of 280 conform -- so its
/// two problem branches are never taken there, and an arm whose failure path
/// has never executed is a green that says nothing about whether it can go
/// red. `a_planted_violation_is_reported_by_the_same_sweep` drives these exact
/// lines over an estate that does violate, which is the only reason the green
/// below means the sweep works rather than that the sweep ran.
fn tally(intent_dir: &std::path::Path, canon: &ingest::Canon) -> Tally {
  let mut out = Tally {
    inline: 0,
    sidecar: 0,
    problems: Vec::new(),
  };
  for thread in &canon.threads {
    for att in &thread.attachments {
      let rel = canon_blob_rel(&thread.id, &att.path);
      let on_disk = intent_dir.join(&rel).exists();
      match (att.is_opaque(), on_disk) {
        // Text, inline only. The 280-strong half.
        (false, false) => out.inline += 1,
        // Opaque, sidecar only.
        (true, true) => out.sidecar += 1,
        (false, true) => out.problems.push(format!(
          "{}/{}: TEXT and carried inline, and a sibling file exists at {rel}. Two copies of one \
           file, and nothing declares which is authoritative",
          thread.id, att.path
        )),
        (true, false) => out.problems.push(format!(
          "{}/{}: OPAQUE, so its bytes are only at {rel}, and that file is not there. Canon names \
           bytes no reader can obtain",
          thread.id, att.path
        )),
      }
    }
  }
  out
}

/// **THE SWEEP'S FAILURE PATH, EXECUTED.** Both drift directions planted in one
/// estate, both reported, and the conforming attachment beside them still
/// counted -- so the sweep discriminates rather than condemning everything once
/// anything is wrong.
#[test]
fn a_planted_violation_is_reported_by_the_same_sweep() {
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  thread.attachments = vec![
    Attachment::new("fine.md", "# Fine\n"),
    Attachment::new("doubled.md", "# Doubled\n"),
    Attachment::opaque("missing.png", NOT_UTF8),
  ];
  fx.write_thread(&thread);
  let bundle = intentsvcs::export::Bundle::new("form", vec![thread], Vec::new(), Vec::new());
  for (rel, body) in intentsvcs::export::canon_parts(&bundle).expect("canon serialises") {
    fx.write_file(&format!("intent/{rel}"), &body);
  }
  // `doubled.md` is text AND gets a sibling. `missing.png` is opaque and its
  // sidecar is deliberately never written.
  let planted = fx.path(&format!(
    "intent/{}",
    canon_blob_rel("ST0001", "doubled.md")
  ));
  std::fs::create_dir_all(planted.parent().expect("parent")).expect("mkdir");
  std::fs::write(&planted, b"# Doubled\n").expect("plant");

  let project = fx.project();
  // `ingest::read` REFUSES the missing sidecar, which is AC-03.1's own rule --
  // so the canon this sweep runs over is built from the model directly. The
  // sweep is what is under test here, not the reader.
  let canon = ingest::Canon {
    threads: bundle.threads.clone(),
    ..Default::default()
  };
  let tally = tally(&project.intent_dir(), &canon);

  assert_eq!(
    tally.inline, 1,
    "the conforming attachment was not counted, so the sweep condemns the whole thread once one \
     file is wrong: {:?}",
    tally.problems
  );
  assert_eq!(
    tally.problems.len(),
    2,
    "both drift directions were planted and {} were reported: {:?}",
    tally.problems.len(),
    tally.problems
  );
  assert!(
    tally
      .problems
      .iter()
      .any(|p| p.contains("doubled.md") && p.contains("TEXT")),
    "the text-with-a-sibling direction is not reported: {:?}",
    tally.problems
  );
  assert!(
    tally
      .problems
      .iter()
      .any(|p| p.contains("missing.png") && p.contains("OPAQUE")),
    "the opaque-with-no-sidecar direction is not reported: {:?}",
    tally.problems
  );
}

/// **THE ESTATE ARM, WITH BOTH DENOMINATORS PRINTED AND ONLY ONE OF THEM
/// MEANINGFUL.**
#[test]
fn every_attachment_in_the_estate_is_stored_in_the_form_its_content_requires() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads from the real estate");
  let intent_dir = project.intent_dir();

  let Tally {
    inline,
    sidecar,
    problems,
  } = tally(&intent_dir, &canon);

  eprintln!(
    "AT-03.2: {inline} inline of {} attachment(s) across {} thread(s); {sidecar} sidecar. THE \
     SIDECAR COUNT IS A FACT ABOUT THE CORPUS AND NOT A PASS -- if it is 0 the opaque half of this \
     rule was not exercised here, and the fixtures in this file are what exercise it.",
    inline + sidecar + problems.len(),
    canon.threads.len()
  );

  assert!(
    inline > 0,
    "not one attachment in the estate is stored inline, so the half of this rule the corpus CAN \
     falsify was not exercised either, and this arm is measuring nothing at all"
  );
  assert!(
    problems.is_empty(),
    "{} attachment(s) are stored in a form their content does not require:\n  {}",
    problems.len(),
    problems.join("\n  ")
  );
}
