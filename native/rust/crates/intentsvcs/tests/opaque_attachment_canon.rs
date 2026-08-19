//! AT-03.1 / ST0057 AC-03.1: **an OPAQUE attachment's bytes live in canon as a
//! FILE at `intent/.canon/st/<ID>/<path>`, and reading canon back reproduces
//! them byte-identically** (D57-7).
//!
//! # The estate cannot test this and says so out loud
//!
//! The criterion asks for a denominator over every opaque attachment in the
//! estate. **That denominator is 0, measured: of 745 files under `intent/st/`,
//! exactly one is not valid UTF-8 and it is `intent/st/.DS_Store`, which D29
//! puts outside the corpus entirely.** So a probe that only walked the estate
//! would print `0 of 0` and go green having exercised nothing -- right verb,
//! right depth, a population that cannot contain the failure.
//!
//! **So the property is driven by a CONSTRUCTED fixture carrying real
//! non-UTF-8 bytes, and the estate zero is printed BESIDE it as a zero.** The
//! reader sees the denominator instead of the verdict, and the day someone
//! attaches a PNG the estate arm starts carrying weight without anyone editing
//! this file.
//!
//! # What "opaque" means, since nothing in the estate defined it
//!
//! Not an extension, and not a guess about the file. It is decided by
//! DECODING: `collect_attachments` reads the bytes, and a `String::from_utf8`
//! that fails is what makes an attachment opaque. The extension already
//! answered a different question one step earlier -- whether the file is
//! carried at all -- and reusing it here would call a `.sh` with one Latin-1
//! byte in a comment "text" and mangle exactly the file most likely to be
//! mishandled.
//!
//! # Presence is not the property
//!
//! An attachment that exists with the wrong bytes passes every check that
//! looks for it, which is the shape this thread keeps meeting. Every assertion
//! below is byte equality against the bytes that went in, and the sidecar's
//! own on-disk bytes are compared to the source file directly.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::export::{self, Bundle};
use intentsvcs::ingest;
use intentsvcs::model::Attachment;
use intentsvcs::project::Project;
use testkit::repo_root;

/// Bytes no `String` can hold: a lone `0xff`, which is not a legal UTF-8 lead
/// byte in any position.
///
/// **Spelled as a constant with a reason, because the first version of a
/// fixture like this used `\x00\x01` -- which ARE valid UTF-8 control
/// characters, so the file decoded fine, was carried inline, and the test
/// passed while proving nothing.** `ignored_paths_corpus.rs` records the same
/// trap from the other side.
const NOT_UTF8: &[u8] = b"\xff\xd8\xff\xe0\x00\x10JFIF\x00\x01\xff\xdb";

const ID: &str = "ST0001";
const REL: &str = "reference.md";

fn opaque_thread() -> intentsvcs::model::Thread {
  let mut thread = sample_thread(ID);
  thread.attachments = vec![Attachment::opaque(REL, NOT_UTF8)];
  thread
}

#[test]
fn an_opaque_attachment_is_carried_as_bytes_and_never_as_text() {
  let att = Attachment::opaque(REL, NOT_UTF8);

  assert!(
    att.is_opaque(),
    "the constructor produced an attachment that does not read as opaque, so every branch keyed \
     on it below is testing the text path"
  );
  assert_eq!(
    att.text, None,
    "an opaque attachment carries no text, and a `Some(lossy)` here would be the silent \
     corruption this form exists to prevent"
  );
  assert_eq!(
    att.as_bytes(),
    Some(NOT_UTF8),
    "the bytes came back changed by the carry"
  );
  assert_eq!(att.bytes as usize, NOT_UTF8.len());
  assert_eq!(
    att.sha256,
    intentsvcs::model::sha256_hex(NOT_UTF8),
    "`sha256` must describe the BYTES; it is the only thing that can ever say the sidecar is the \
     file the author wrote, because nobody can read the content and notice it is wrong"
  );

  // **The control.** Every assertion above would hold for a form that made
  // EVERYTHING opaque, so a text attachment at the same depth must come back
  // as text.
  let text = Attachment::new(REL, "# Reference\n");
  assert!(
    !text.is_opaque(),
    "a valid-UTF-8 attachment reads as opaque, so the discriminator is not discriminating and the \
     agreement asserted above is vacuous"
  );
  assert_eq!(text.text.as_deref(), Some("# Reference\n"));
}

/// **THE CRITERION: the bytes land at the declared path and come back
/// identical.**
#[test]
fn the_bytes_live_in_canon_as_a_file_and_read_back_byte_identically() {
  let fx = Fixture::new();
  let thread = opaque_thread();

  let bundle = Bundle::new("opaque", vec![thread.clone()], Vec::new(), Vec::new());
  for (rel, body) in export::canon_parts(&bundle).expect("canon serialises") {
    fx.write_file(&format!("intent/{rel}"), &body);
  }

  let blobs = export::canon_blobs(&bundle);
  assert_eq!(
    blobs.len(),
    1,
    "the exporter emitted {} sidecar(s) for one opaque attachment; a zero here would make every \
     comparison below run over a file this test wrote itself",
    blobs.len()
  );
  let (blob_rel, blob_bytes) = &blobs[0];
  assert_eq!(
    blob_rel,
    &format!(".canon/st/{ID}/{REL}"),
    "the sidecar is not at the path AC-03.1 declares, so a reader following the criterion would \
     look somewhere the bytes are not"
  );
  for (rel, bytes) in &blobs {
    let path = fx.path(&format!("intent/{rel}"));
    std::fs::create_dir_all(path.parent().expect("a sidecar has a parent")).expect("mkdir");
    std::fs::write(&path, bytes).expect("write the sidecar");
  }

  // **The canon JSON must NOT contain the bytes** -- AC-03.2's other half,
  // checked here rather than assumed from the `#[serde(skip)]`.
  // **Asked of the ATTACHMENT OBJECT, not of the file.** The first version of
  // this grepped the whole document for `"text"` and failed -- correctly --
  // because an acceptance criterion carries a `text` field of its own. A
  // whole-file grep answers a question about the document when the claim is
  // about one object in it, which is the same label-for-subject substitution
  // this estate has paid for elsewhere today.
  let json = fx.read_canon(ID);
  let parsed: serde_json::Value = serde_json::from_str(&json).expect("canon is JSON");
  let attachment = &parsed["attachments"][0];
  assert!(
    attachment.get("text").is_none(),
    "the attachment object carries a `text` field for opaque content, so the bytes went inline \
     after all: {attachment}"
  );
  assert_eq!(
    attachment["path"], REL,
    "the object being checked is not the attachment: {attachment}"
  );

  let back = ingest::read(&fx.project()).expect("canon reads back with its sidecar");
  let carried = &back.threads[0].attachments[0];

  assert_eq!(
    carried.as_bytes(),
    Some(NOT_UTF8),
    "the bytes did not survive the round trip through canon -- canon holds {} byte(s), the source \
     was {}",
    carried.bytes,
    NOT_UTF8.len()
  );
  assert!(
    carried.is_opaque(),
    "it came back as a TEXT attachment, which means something decoded it lossily on the way"
  );
  assert_eq!(
    std::fs::read(fx.path(&format!("intent/{blob_rel}"))).expect("the sidecar is on disk"),
    NOT_UTF8,
    "the file at the declared path does not hold the source bytes"
  );
  assert_eq!(blob_bytes.as_slice(), NOT_UTF8);
}

/// **A sidecar that is missing REFUSES; it does not read as an empty
/// attachment.**
///
/// Without this the green above is unfalsifiable in the direction that costs a
/// file: an attachment present with zero bytes satisfies every check that looks
/// for it, and hydration would then write emptiness over the author's copy.
#[test]
fn canon_naming_a_sidecar_that_is_not_there_refuses_and_names_it() {
  let fx = Fixture::new();
  let bundle = Bundle::new("opaque", vec![opaque_thread()], Vec::new(), Vec::new());
  for (rel, body) in export::canon_parts(&bundle).expect("canon serialises") {
    fx.write_file(&format!("intent/{rel}"), &body);
  }
  // The sidecar is deliberately NOT written.

  let err = ingest::read(&fx.project()).expect_err(
    "canon naming bytes that are not on disk was read as a valid project -- the attachment would \
     arrive empty and every check that looks for it would pass",
  );
  let rendered = format!("{err}");
  assert!(
    rendered.contains(REL),
    "the refusal does not name the attachment whose bytes are missing: {rendered}"
  );
  assert!(
    rendered.contains(ID),
    "the refusal does not name the thread: {rendered}"
  );
}

/// The carry decides form by DECODING, over a file written to disk -- the seam
/// a real project actually crosses.
#[test]
fn a_carryable_file_that_is_not_utf8_is_carried_opaque_rather_than_refused() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(ID));
  let dir = fx.path(&format!("intent/st/{ID}"));
  std::fs::create_dir_all(&dir).expect("mkdir");
  std::fs::write(dir.join("binary.txt"), NOT_UTF8).expect("write");
  std::fs::write(dir.join("plain.txt"), "hello\n").expect("write");

  let project = fx.project();
  let (carried, refused) = project.collect_attachments(ID);

  assert!(
    refused.is_empty(),
    "a carryable file that is not valid UTF-8 was REFUSED rather than carried as bytes. It is \
     then in no canon, recoverable from nothing, and pinned to disk forever: {refused:?}"
  );
  let binary = carried
    .iter()
    .find(|a| a.path == "binary.txt")
    .expect("the non-UTF-8 file is carried");
  let plain = carried
    .iter()
    .find(|a| a.path == "plain.txt")
    .expect("the UTF-8 file is carried");

  assert_eq!(binary.as_bytes(), Some(NOT_UTF8));
  assert!(binary.is_opaque(), "form must follow content");
  assert!(
    !plain.is_opaque(),
    "the TEXT file was carried as bytes -- AC-03.2 is one rule checked BOTH ways, and a form that \
     made everything opaque would pass every assertion about the binary one"
  );
  assert_eq!(plain.text.as_deref(), Some("hello\n"));
}

/// **The estate arm, and its ZERO is printed as a zero.**
///
/// It is not evidence that the mechanism works -- nothing here can be, because
/// there is nothing to carry. It is a standing measurement of the denominator,
/// so the day an opaque attachment enters the estate this arm starts asserting
/// the criterion over it without anyone remembering to come back.
#[test]
fn the_estate_denominator_is_reported_and_every_opaque_attachment_in_it_round_trips() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = ingest::read(&project).expect("canon reads from the real estate");

  let opaque: Vec<(&str, &intentsvcs::model::Attachment)> = canon
    .threads
    .iter()
    .flat_map(|t| t.attachments.iter().map(move |a| (t.id.as_str(), a)))
    .filter(|(_, a)| a.is_opaque())
    .collect();
  let total: usize = canon.threads.iter().map(|t| t.attachments.len()).sum();

  eprintln!(
    "AT-03.1: {} opaque attachment(s) of {total} carried across {} thread(s). A ZERO HERE IS A \
     FACT ABOUT THE CORPUS AND NOT A PASS -- the property is driven by the constructed fixture in \
     this file.",
    opaque.len(),
    canon.threads.len()
  );

  assert!(
    total > 0,
    "the estate carries no attachments at all, so this arm's denominator is not merely zero for \
     the opaque half -- the read itself found nothing and the measurement is of the wrong thing"
  );

  for (id, att) in &opaque {
    let rel = format!(
      "intent/{}",
      intentsvcs::project::canon_blob_rel(id, &att.path)
    );
    let on_disk = std::fs::read(root.join(&rel))
      .unwrap_or_else(|e| panic!("{id}/{}: its sidecar at {rel} is unreadable: {e}", att.path));
    assert_eq!(
      att.as_bytes(),
      Some(on_disk.as_slice()),
      "{id}/{}: canon's bytes and the sidecar's differ",
      att.path
    );
    assert_eq!(
      att.sha256,
      intentsvcs::model::sha256_hex(&on_disk),
      "{id}/{}: the recorded hash does not describe the bytes on disk",
      att.path
    );
  }
}
