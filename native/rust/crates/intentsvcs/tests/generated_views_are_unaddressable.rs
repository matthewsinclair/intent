//! **THE OVERLAP BETWEEN TWO ENCODINGS OF "WHAT IS A GENERATED VIEW", DRIVEN
//! FROM THE ESTATE RATHER THAN FROM EITHER ENCODING.**
//!
//! `address::VIEW_NAMES` is a list of BASENAMES that `parse` refuses as a path
//! segment, so a generated view has no address. `Project::classify` answers a
//! different question over a different domain -- given a path relative to a
//! thread directory, whose file is this -- and it answers by SHAPE (depth plus
//! name), consulting no list.
//!
//! **THE TWO ARE NOT TWO HOMES FOR ONE FACT AND MUST NOT BE COLLAPSED INTO
//! ONE.** `VIEW_NAMES` carries `todo.md` and `steel_threads.md`, which are
//! project-level and belong to no thread, so `classify` never sees them; it
//! also carries the extensionless `info` and `acceptance`, which are not
//! filenames at all but the segment spellings an operator types. A single
//! list serving both would have to be wrong for one of them.
//!
//! **ONLY THEIR OVERLAP HAS TO AGREE, AND THAT DIRECTION IS ONE-WAY**: every
//! basename `classify` calls a [`ThreadFile::GeneratedView`] must appear in
//! `VIEW_NAMES`, or that view becomes addressable and `ViewAddressed` stops
//! refusing it.
//!
//! **AND SINCE ISSUE 0461 THE OTHER DIRECTION IS ASSERTED TOO**: every path
//! `classify` hands to the ATTACHMENT side must not be refused as a view when it
//! is named as an attachment. Until then only the first direction was checked,
//! and a deep `info.md` -- the author's, by `classify` -- was refused by `parse`
//! at any depth, so the file could never be carried. `parse` now refuses a view
//! basename inside an attachment path only at a view's depth (vc's ruling of
//! 2026-09-19), and the arms below hold the two answers together at both edges.
//!
//! # This test exists because the constant asked for it, in these words
//!
//! `address.rs`'s own doc comment says the gate "is an integration test needing
//! no access to this constant -- walk the estate's thread files, and for each
//! one `classify` calls a `GeneratedView`, require `parse` to refuse its
//! basename as a segment", and then says plainly: **"Nothing asserts that
//! today."** This is that test, and it deliberately does NOT import
//! `VIEW_NAMES` -- reading the list would make the assertion circular, proving
//! only that a list equals itself.
//!
//! # Its reach, stated rather than discovered
//!
//! It catches a new view type only once such a file EXISTS in the estate. A
//! `GeneratedView` shape that `classify` would recognise but that nobody has
//! written yet is invisible here. That is a real limit, it is the one the
//! constant predicted, and it is still strictly better than the nothing that
//! guarded this before.

use intentsvcs::address::{AddressError, SCHEME, parse};
use intentsvcs::project::{Project, ThreadFile, attachment_name};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use testkit::repo_root;

/// Every file under `dir`, recursively.
fn files_under(dir: &Path, out: &mut Vec<PathBuf>) {
  let Ok(entries) = std::fs::read_dir(dir) else {
    return;
  };
  for entry in entries.flatten() {
    let path = entry.path();
    match path.is_dir() {
      true => files_under(&path, out),
      false => out.push(path),
    }
  }
}

/// Ask `parse` about an address carrying `segment`, and say what came back.
///
/// The segment is planted in a path that is otherwise well formed, so a refusal
/// can only be about the segment itself.
fn parse_with_segment(segment: &str) -> Result<(), AddressError> {
  parse(&format!("{SCHEME}/threads/ST0056/{segment}")).map(|_| ())
}

#[test]
fn every_generated_view_in_the_estate_is_refused_as_an_address_segment() {
  let st_root = repo_root().join("intent").join("st");
  let mut threads = 0usize;
  let mut examined = 0usize;
  let mut basenames: BTreeSet<String> = BTreeSet::new();

  let Ok(entries) = std::fs::read_dir(&st_root) else {
    panic!("no estate at {} to walk", st_root.display());
  };
  for entry in entries.flatten() {
    let thread_dir = entry.path();
    if !thread_dir.is_dir() {
      continue;
    }
    threads += 1;
    let mut files = Vec::new();
    files_under(&thread_dir, &mut files);
    for file in &files {
      let Ok(rel) = file.strip_prefix(&thread_dir) else {
        continue;
      };
      examined += 1;
      if Project::classify(rel) != ThreadFile::GeneratedView {
        continue;
      }
      let Some(base) = rel.file_name().and_then(|n| n.to_str()) else {
        continue;
      };
      basenames.insert(base.to_string());
    }
  }

  // **THE DENOMINATOR IS ASSERTED, NOT PRINTED AND TRUSTED.** A walk that finds
  // nothing satisfies every per-item assertion below vacuously, and would go on
  // passing after a relocation moved the estate out from under it -- which is
  // the failure this project has already met more than once.
  assert!(
    threads > 0 && examined > 0,
    "the walk found {threads} thread dir(s) and {examined} file(s) under {}, so it \n       \
     proved nothing -- a green here would be about an empty population",
    st_root.display()
  );
  assert!(
    !basenames.is_empty(),
    "walked {examined} file(s) across {threads} thread(s) and `classify` called none of \n       \
     them a GeneratedView -- either the estate has no generated views, which is false, \n       \
     or this walk is not handing `classify` the path it expects (relative to the thread \n       \
     directory, so `info.md` is depth 1 and `WP/01/info.md` is depth 3)"
  );

  for base in &basenames {
    let outcome = parse_with_segment(base);
    assert!(
      matches!(outcome, Err(AddressError::ViewAddressed { ref segment }) if segment == base),
      "`classify` calls `{base}` a generated view, so `parse` must refuse it as a segment \n       \
       and `VIEW_NAMES` must carry it -- otherwise that view is addressable. Got: {outcome:?}"
    );
  }

  eprintln!(
    "generated-view basenames refused: {} of {} distinct, across {threads} thread(s), \
     {examined} file(s) examined -- {}",
    basenames.len(),
    basenames.len(),
    basenames.iter().cloned().collect::<Vec<_>>().join(", ")
  );
}

/// **THE INSTRUMENT IS DRIVEN TO BOTH VERDICTS, OR THE GREEN ABOVE MEANS
/// NOTHING.**
///
/// If `parse_with_segment` refused everything -- a typo in the URL, a scheme
/// that stopped parsing, a path shape the grammar rejects for its own reasons
/// -- every assertion above would pass while testing nothing at all. This
/// plants a basename that is NOT a generated view and requires it THROUGH.
#[test]
fn the_probe_lets_a_non_view_segment_through() {
  let outcome = parse_with_segment("design.md");
  assert!(
    !matches!(outcome, Err(AddressError::ViewAddressed { .. })),
    "`design.md` is not a generated view, so a `ViewAddressed` refusal here means the probe \n       \
     refuses everything and the population test above is vacuous. Got: {outcome:?}"
  );
}

/// Name `rel` as an attachment of ST0056 and say whether `parse` refused it as
/// a view.
fn refused_as_a_view(rel: &str) -> bool {
  matches!(
    parse(&format!("{SCHEME}/threads/ST0056/attachments/{rel}")),
    Err(AddressError::ViewAddressed { .. })
  )
}

/// **THE DIRECTION THIS FILE USED TO SKIP, OVER THE ESTATE** (issue 0461). Every
/// file `classify` calls an attachment must be nameable as one without a view
/// refusal, or the ingest leaves it unheld and the prune withholds it forever.
#[test]
fn no_file_classify_calls_an_attachment_is_refused_as_a_view() {
  let st_root = repo_root().join("intent").join("st");
  let mut examined = 0usize;
  let mut refused: Vec<String> = Vec::new();
  let Ok(entries) = std::fs::read_dir(&st_root) else {
    panic!("no estate at {} to walk", st_root.display());
  };
  for entry in entries.flatten() {
    let thread_dir = entry.path();
    if !thread_dir.is_dir() {
      continue;
    }
    let mut files = Vec::new();
    files_under(&thread_dir, &mut files);
    for file in &files {
      let Ok(rel) = file.strip_prefix(&thread_dir) else {
        continue;
      };
      if Project::classify(rel) != ThreadFile::Attachment {
        continue;
      }
      examined += 1;
      let rel = rel.to_string_lossy();
      if refused_as_a_view(&rel) {
        refused.push(rel.into_owned());
      }
    }
  }
  assert!(
    examined > 0,
    "the walk found no attachment under {}, so it proved nothing",
    st_root.display()
  );
  assert!(
    refused.is_empty(),
    "`classify` calls these the author's and `parse` refuses them as views, so they can \n     \
     never be carried: {refused:?}"
  );
}

/// **BOTH EDGES OF THE DEPTH RULE, PLANTED, BECAUSE THE ESTATE MAY HOLD NEITHER.**
/// The shapes `classify` calls views -- the thread root's `info.md` and
/// `acceptance.md`, and `info.md` directly under any `WP/` directory -- are
/// refused as attachment names, and so is every view name at the thread root,
/// which AT-07.2 reserves; the same basenames anywhere deeper that `classify`
/// calls the author's are addressable and nameable. The view side pins that 0461's
/// fix did not open a door onto a generated file.
#[test]
fn a_view_basename_is_refused_only_at_a_views_depth() {
  for view in [
    "info.md",
    "acceptance.md",
    "todo.md",
    "steel_threads.md",
    "info",
    "WP/01/info.md",
    "WP/_superseded/info.md",
  ] {
    assert!(
      refused_as_a_view(view),
      "`{view}` is at a view's depth and must be refused as a view"
    );
  }
  for rel in [
    "WP/_superseded/01/info.md",
    "parity/tools/info.md",
    "WP/01/acceptance.md",
  ] {
    assert_eq!(
      Project::classify(std::path::Path::new(rel)),
      ThreadFile::Attachment,
      "`{rel}` is the author's by `classify`, which this arm assumes"
    );
    assert!(
      !refused_as_a_view(rel),
      "`{rel}` is the author's, so naming it as an attachment must not be a view refusal"
    );
    assert_eq!(
      attachment_name("ST0056", rel),
      Ok(()),
      "`{rel}` must round-trip as an attachment name, or the ingest leaves it unheld"
    );
  }
  for view in ["info.md", "wp/01/info.md"] {
    assert!(
      matches!(
        parse(&format!("{SCHEME}/threads/ST0056/{view}")),
        Err(AddressError::ViewAddressed { .. })
      ),
      "`/threads/ST0056/{view}` is a view outside the attachment path and stays refused"
    );
  }
}
