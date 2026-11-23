//! **`Thread::attachments`: the authored files beside a thread that no typed
//! document holds.**
//!
//! What this file pins: that the derived fields cannot drift from the content,
//! and that the view test keys on a file's shape rather than its bare name.

use intentsvcs::model::Attachment;
use intentsvcs::project::{Project, ThreadFile};

/// **The derived fields cannot drift from the content, because there is one
/// way to make one.**
#[test]
fn bytes_and_sha256_describe_the_text_they_were_built_from() {
  let a = Attachment::new("reference.md", "# Reference\n");
  let text = a
    .text
    .as_deref()
    .expect("a text attachment carries its text");
  assert_eq!(a.bytes as usize, text.len());
  assert_eq!(a.sha256, intentsvcs::model::sha256_hex(text.as_bytes()));
  assert_ne!(
    a.sha256,
    intentsvcs::model::sha256_hex(b""),
    "and it is not the empty-string hash, which is what a field left unset \
     would carry and what 112 ratified template drops already look like"
  );
}

/// **The view test keys on SHAPE, not on a bare filename**, and this is the
/// arm that proves the difference.
///
/// `info.md` at a thread's root is the generated cover. `parity/info.md` is
/// somebody's document that happens to share a name -- matching on the name
/// alone would take an author's file, call it ours, and overwrite it on the
/// next render.
#[test]
fn a_file_named_like_a_view_but_nested_is_an_authored_file() {
  assert_eq!(
    Project::classify(std::path::Path::new("info.md")),
    ThreadFile::GeneratedView
  );
  assert_eq!(
    Project::classify(std::path::Path::new("WP/01/info.md")),
    ThreadFile::GeneratedView
  );
  assert_eq!(
    Project::classify(std::path::Path::new("parity/info.md")),
    ThreadFile::Attachment,
    "three levels down under someone's own directory, this is not our cover"
  );
  assert_eq!(
    Project::classify(std::path::Path::new("notes/design.md")),
    ThreadFile::Attachment,
    "and a typed document's name only means that at the thread's root"
  );
  assert_eq!(
    Project::classify(std::path::Path::new("design.md")),
    ThreadFile::Attachment,
    "D57-6: THREAD_PROSE is deleted, so a thread's own design.md is carried \
     verbatim like any other authored markdown rather than skipped"
  );
  // **THIS ROW CHANGED ITS ANSWER ON 2026-08-26, AND THE CHANGE IS A RULING
  // RATHER THAN A CONSEQUENCE.**
  //
  // It used to assert `Unattached`, because `ATTACHMENT_EXTENSIONS` was
  // `["md", "txt", "sh"]` and a `.tap` was not on the list. That list encoded a
  // real principle of vc's -- *no tool can make this again, versus a tool made
  // this and can again* -- under which a generated baseline stays out because
  // carrying regenerable output buys nothing.
  //
  // **The principle survives; the list did not, because a global extension list
  // cannot answer a per-project question.** `.tap` is tool output HERE and
  // Conflab's `.json` may well be authored THERE, so the same list answered one
  // right and the other wrong. Its new home is `.intentfiles`, a per-project
  // declaration of what a project does not want carried (vc, 3.0.2).
  //
  // Until that lands, the baseline IS carried, under the cap. The measured cost
  // is 328 KB in one thread on this repo, taken deliberately in preference to
  // inventing a second classifier under release pressure.
  assert_eq!(
    Project::classify(std::path::Path::new("parity/baseline.tap")),
    ThreadFile::Attachment,
    "an extension no longer decides what is carried -- size does, and \
     authorship will, per project, once `.intentfiles` carries it"
  );
}
