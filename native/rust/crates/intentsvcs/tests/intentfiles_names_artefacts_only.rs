//! AT-02.5 / AC-02.5: **`.intentfiles` names ARTEFACTS and never enumerates
//! FILES; `Project::classify` remains the single answer to what a file IS.**
//!
//! The two answer different questions -- the manifest answers _which artefacts
//! are realised_, `classify` answers _what is this file_ -- and they COMPOSE:
//! `STEELTHREAD:ST0056` realises the thread and, through `classify`, whatever
//! files that thread produces. Neither may acquire a second, independent
//! enumeration of files, because two declarations of which-files-matter agree
//! for months and then quietly do not.
//!
//! # This row PINS a mechanism it did not build
//!
//! The grammar already refuses paths, and it refuses them **because no path
//! satisfies [`model::is_thread_id`] or [`model::is_issue_id`]** -- not because
//! anything looks for a `/`. There is no path check to test. So the property
//! under test is not "paths are rejected"; it is **the accepted set IS EXACTLY
//! the id set**, which is what makes a file-valued line unrepresentable rather
//! than merely discouraged.
//!
//! **A corpus of paths would be the wrong test and would pass on the wrong
//! implementation.** Write nine plausible paths, watch nine refusals, and a
//! blacklist keyed on `/` and `.` passes every one -- while `STEELTHREAD:README`
//! sails through, because it contains neither. The exemplar would have become
//! the definition, which is this estate's recurring defect and one I have
//! already committed today in a different file.
//!
//! So the sweep below is over the COMPLEMENT: strings drawn from far outside
//! any path vocabulary, asserted refused for the same reason a path is. What a
//! path corpus buys on top is only that the real estate cannot express itself,
//! and that is taken from the ACTUAL FILES ON DISK rather than from nine
//! invented ones.

use intentsvcs::intentfiles::{Sigil, parse};
use intentsvcs::model;
use intentsvcs::project::{Project, ThreadFile};
use std::path::Path;
use testkit::repo_root;

fn accepted(line: &str) -> bool {
  parse(line).is_ok_and(|m| m.entries.len() == 1)
}

/// **The mechanism, stated as an equivalence rather than as a list.**
///
/// For a `STEELTHREAD:` line the manifest accepts exactly what
/// `is_thread_id` accepts, and for `ISSUE:` exactly what `is_issue_id`
/// accepts. Any implementation that diverges -- a path blacklist, a `/` check,
/// a length guard spelled a second time -- breaks this on some input, because
/// the two sides are then two declarations of one rule.
#[test]
fn the_accepted_set_is_exactly_the_id_set() {
  let candidates = [
    // ids
    "ST0056",
    "ST0000",
    "ST9999",
    "0001",
    "0042",
    "9999",
    // near-ids, which is where a second spelling diverges first
    "ST056",
    "ST00567",
    "ST005a",
    "st0056",
    "ST 0056",
    "",
    "042",
    "00042",
    "-001",
    "0x42",
    // paths -- present as ORDINARY non-ids, not as a special class
    "intent/st/ST0056/info.md",
    "./ST0056",
    "/abs/ST0056",
    "ST0056/",
    "ST0056/info.md",
    "..",
    // things with neither slash nor dot, which a naive path guard admits
    "README",
    "Cargo",
    "main",
  ];

  for c in candidates {
    assert_eq!(
      accepted(&format!("STEELTHREAD:{c}")),
      model::is_thread_id(c),
      "STEELTHREAD:{c} -- the manifest and `is_thread_id` must agree exactly;\n       \
       a divergence here means the grammar has grown a second id rule"
    );
    assert_eq!(
      accepted(&format!("ISSUE:{c}")),
      model::is_issue_id(c),
      "ISSUE:{c} -- the manifest and `is_issue_id` must agree exactly"
    );
  }
}

/// **Whitespace is the LINE's business, and it is settled before the grammar
/// sees an id.**
///
/// Stated separately because the equivalence above cannot hold it, and the
/// first version of this file tried to make it. `STEELTHREAD:ST0056 ` is
/// accepted while `is_thread_id("ST0056 ")` is false, and that looked like the
/// manifest having a second id rule -- it is not. The line is trimmed before
/// it is split, so the id predicate never sees the space. Two rules operating
/// at two levels, which is fine; two rules operating at ONE level is the
/// defect AC-02.5 names, and this is not that.
///
/// Pinned rather than left implicit: a trailing space in a committed file is
/// invisible to a reader and shows in a diff, so refusing on one would be a
/// refusal nobody could see coming, and admitting one silently would be a
/// tolerance nobody wrote down.
#[test]
fn line_whitespace_is_settled_before_the_id_is_read() {
  for spaced in [
    "STEELTHREAD:ST0056 ",
    "  STEELTHREAD:ST0056",
    "\tSTEELTHREAD:ST0056\t",
    "STEELTHREAD: ST0056",
    "STEELTHREAD :ST0056",
  ] {
    assert!(
      accepted(spaced),
      "`{spaced}` is one artefact with whitespace around it, not a malformed id"
    );
  }
  // But whitespace INSIDE the id is not whitespace around a line, and the id
  // predicate does see it.
  assert!(!accepted("STEELTHREAD:ST 0056"));
  assert!(!accepted("ISSUE:00 42"));
}

/// **The real estate cannot express itself as manifest lines.**
///
/// Every file Intent actually carries under a thread directory, tried as an
/// id under both sigils. Taken from disk rather than from a list here, for the
/// reason `openness.rs` enumerates its tables from the DDL: a hand-written
/// corpus stops covering the day someone adds a shape nobody thought of.
#[test]
fn no_file_in_the_estate_can_be_named_as_an_artefact() {
  let root = repo_root();
  let project = Project::open(&root).expect("this repository is a project");
  let st_dir = root.join("intent").join("st");

  let mut tried = 0usize;
  for entry in std::fs::read_dir(&st_dir).expect("intent/st is readable") {
    let path = entry.expect("dir entry").path();
    let Some(id) = path.file_name().and_then(|n| n.to_str()) else {
      continue;
    };
    if !model::is_thread_id(id) {
      continue;
    }
    for rel in project.thread_files(id) {
      let rel_str = rel.to_string_lossy().to_string();
      tried += 1;
      assert!(
        !accepted(&format!("STEELTHREAD:{rel_str}")),
        "a real file expressed itself as an artefact: STEELTHREAD:{rel_str}"
      );
      assert!(
        !accepted(&format!("ISSUE:{rel_str}")),
        "a real file expressed itself as an artefact: ISSUE:{rel_str}"
      );
      // The absolute form too -- a manifest read on another machine.
      let abs = path.join(&rel).to_string_lossy().to_string();
      assert!(!accepted(&format!("STEELTHREAD:{abs}")));
    }
  }

  assert!(
    tried > 0,
    "this test is vacuous unless it saw real files -- intent/st held none,\n       \
     so the sweep proved nothing and must not report a pass"
  );
}

/// **The two answers are different questions, and neither can be asked of the
/// other.**
///
/// `classify` takes a path and never an id; the manifest takes an id and never
/// a path. The composition is: the manifest names `ST0056`, and `classify`
/// then says what each of ST0056's files IS. This asserts the two vocabularies
/// do not overlap -- if an id ever became a thing `classify` recognised, or a
/// path a thing the manifest accepted, the second enumeration would exist.
#[test]
fn the_manifest_and_classify_do_not_overlap() {
  // An id is not a file, so classify has nothing useful to say about one.
  // `Unattached` is the named remainder rather than a silent gap, which is the
  // right answer for something that is not a file at all.
  assert_eq!(
    Project::classify(Path::new("ST0056")),
    ThreadFile::Unattached,
    "an artefact id is not a file and classify must not claim otherwise"
  );

  // And the files classify DOES recognise cannot be named in the manifest.
  for named in ["info.md", "acceptance.md", "design.md", "thread.json"] {
    let kind = Project::classify(Path::new(named));
    assert_ne!(
      kind,
      ThreadFile::Unattached,
      "{named} is a file classify has an answer for -- the fixture is stale if not"
    );
    assert!(
      !accepted(&format!("STEELTHREAD:{named}")),
      "{named} is classify's business and must be unrepresentable in the manifest"
    );
  }
}

/// The sigil space is closed, which is the other half of "unrepresentable".
///
/// A third sigil -- `FILE:`, `PATH:`, `GLOB:` -- is how the second enumeration
/// would actually arrive in practice: not by loosening the id rule, but by
/// adding a vocabulary term beside it. Adding one is a model change and must
/// be one.
#[test]
fn the_sigil_space_is_closed() {
  for smuggled in ["FILE", "PATH", "GLOB", "DIR", "VIEW", "ATTACHMENT"] {
    assert!(
      Sigil::parse(smuggled).is_none(),
      "{smuggled} must not be a sigil -- a file-valued vocabulary term is the\n       \
       second enumeration arriving beside the id rule rather than through it"
    );
    assert!(
      !accepted(&format!("{smuggled}:intent/st/ST0056/info.md")),
      "{smuggled}: must be refused by the grammar, not merely absent from it"
    );
  }
}
