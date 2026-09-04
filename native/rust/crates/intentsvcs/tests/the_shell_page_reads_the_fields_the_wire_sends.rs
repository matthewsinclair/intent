//! `0241`: **the page must read the field names the wire actually SENDS.**
//!
//! # What went wrong, and why nothing caught it
//!
//! The shell page detected a daemon refusal with `answer.body.error`.
//! [`intentsvcs::wire::Response`] is `#[serde(tag = "result")]`, so a refusal
//! is `{result: "error", message, remedy}` and there is **no `error` key
//! anywhere in the enum**. The guard was therefore dead: it never fired, the
//! `status !== 200` guard could not catch it either -- refusals are 200 by
//! design -- and a refusal was handed to the SUCCESS renderer, which painted
//! `undefined` over an empty form while a precise message and its remedy were
//! discarded.
//!
//! **HALF THE GUESS TYPE-CHECKED AGAINST REALITY, WHICH IS HOW IT SURVIVED A
//! READ.** `remedy` IS a real field on that variant, so the wrong key sat
//! beside a correct sibling.
//!
//! # Why this test exists rather than a browser test
//!
//! **NOTHING OWNED THE RELATION BETWEEN THE WIRE'S FIELD NAMES AND THE PAGE'S
//! READER.** Each half was internally consistent and separately correct-looking
//! -- the daemon serialises its enum properly, and the page reads fields off an
//! object properly. The defect existed only at the seam, so no amount of
//! reading either file finds it, and the web-face tests could not: they assert
//! what the DAEMON sends and that the shell is SERVED. Nothing executes this
//! page's JavaScript, so its branch logic is untested by construction.
//!
//! # Why it lives in `intentsvcs` and not beside the page
//!
//! **IT NEEDS NO NEW DEPENDENCY HERE, AND ADDING ONE TO THE DAEMON FOR A TEST
//! IS A REAL COST.** `AC-08.10` makes every dependency `intentd` declares argue
//! for itself, and the first version of this test put `serde_json` in that
//! manifest to serialise the wire types -- which the check refused, correctly.
//! `intentsvcs` owns `wire::Response` and already has `serde_json`, so the
//! contract and its checker sit together.
//!
//! **THE PRECEDENT IS `dependency_rationale.rs`**, which lives here and reads
//! `intentd`'s own manifest at RUN time: this estate already guards `intentd`'s
//! files from this crate's suite. **A cross-crate `include_str!` is NOT the
//! established shape and cost the whole repository its commits for a few
//! minutes when this test first tried it** -- see [`page`].
//!
//! This asserts the one thing that is checkable without a JS engine and that
//! would have caught the defect: **every field the page reads off a response is
//! a field some response can carry**, and the refusal's own text field is one
//! of them.

use std::collections::BTreeSet;
use testkit::workspace_root;

/// The page, read from disk at RUN time rather than embedded at COMPILE time.
///
/// **THE EMBED BLOCKED EVERY COMMIT IN THE REPOSITORY, ON EVERY PATH, AND THE
/// GUARD'S AUTHOR PREDICTED IT (dc found it).** `include_str!` from
/// `crates/intentsvcs/tests/` reaching `../../intentd/src/shell.html` climbs to
/// `crates/` -- a directory BETWEEN `native/rust` and the repository root. The
/// shared-artefact guard's arm 6b decides coverage by stripping `../` prefixes
/// and treating the remainder as repo-relative, and its own comment states the
/// residual in as many words: *this assumes the climb reaches the root and not
/// some directory between.* This was the first embed in the tree to land
/// between, so the arm reported a real file inside the declared scope as
/// uncovered, and the guard reads the WORKTREE -- so an UNTRACKED file refused
/// every node's commits regardless of what they were committing.
///
/// **A RUNTIME READ REMOVES THE EMBED, SO THE ARM NEVER SEES IT.** That is also
/// the pattern this file's header cites: `dependency_rationale.rs` reads
/// `intentd`'s manifest through `workspace_root()` and `fs::read_to_string`.
/// The first version of this test named that precedent and then did not follow
/// it.
///
/// **IT PANICS ON A MISSING FILE, DELIBERATELY.** Neither absence nor an
/// unreadable path may arrive at the assertions below as a clean sheet.
fn page() -> String {
  let path = workspace_root()
    .join("crates")
    .join("intentd")
    .join("src")
    .join("shell.html");
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// Every identifier the page reads off a response body, **in CODE and never in
/// a COMMENT**.
///
/// Scanned rather than parsed, deliberately: a JS parser here would be a second
/// thing to be wrong, and the read is spelled one way in this file.
///
/// **COMMENT LINES ARE STRIPPED FIRST, AND THAT IS NOT TIDINESS -- THE FIRST
/// VERSION OF THIS TEST FAILED ON THE COMMENT EXPLAINING THE BUG.** The fix for
/// `0241` documents the dead branch by quoting the field it used to read, so a
/// scanner that reads prose reports the defect as still present and **makes
/// writing the explanation an offence.** The whiteboard protocol's own header
/// guard refuses to scan prose for exactly this reason: nodes report these
/// defects to each other by quoting them.
fn fields_the_page_reads() -> BTreeSet<String> {
  const NEEDLE: &str = "answer.body.";
  let mut found = BTreeSet::new();
  let html = page();
  let code: String = html
    .lines()
    .filter(|line| !line.trim_start().starts_with("//"))
    .collect::<Vec<_>>()
    .join("\n");
  let mut rest = code.as_str();
  while let Some(at) = rest.find(NEEDLE) {
    rest = &rest[at + NEEDLE.len()..];
    let ident: String = rest
      .chars()
      .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
      .collect();
    if !ident.is_empty() {
      found.insert(ident);
    }
  }
  found
}

/// Every field name a response THIS PAGE can receive actually carries.
///
/// The page sends only `Op::Form`, and any op can be refused, so those are the
/// two variants it can meet. Taken by SERIALISING the real types rather than by
/// listing names, so this cannot drift from the wire it is checking -- a hand
/// list would be a second home for the contract and would agree with the bug.
fn fields_the_wire_sends() -> BTreeSet<String> {
  use intentsvcs::wire::Response;
  let mut keys = BTreeSet::new();
  let answers = [
    serde_json::to_value(Response::Error {
      message: "m".to_string(),
      remedy: "r".to_string(),
    })
    .expect("a refusal serialises"),
    serde_json::to_value(Response::Form {
      entity: "e".to_string(),
      title: "t".to_string(),
      fields: Vec::new(),
    })
    .expect("a form serialises"),
  ];
  for answer in answers {
    for key in answer
      .as_object()
      .expect("a tagged response serialises to an object")
      .keys()
    {
      keys.insert(key.clone());
    }
  }
  keys
}

/// **THE PAGE READS NO FIELD THE WIRE NEVER SENDS.** This is `0241`.
#[test]
fn the_shell_page_reads_no_field_the_wire_never_sends() {
  let reads = fields_the_page_reads();

  // **VACUITY GUARD, AND IT IS NOT DECORATION.** The scan keys on one spelling.
  // If the page is refactored to bind the body to another name, this finds
  // nothing and the assertion below passes over an empty set -- reporting the
  // seam as sound precisely when it has stopped being checked.
  assert!(
    !reads.is_empty(),
    "found no response fields read by the page at all -- the scan looks for \
     `answer.body.<field>` and the page no longer spells it that way, so this \
     test is measuring nothing"
  );

  let sends = fields_the_wire_sends();
  let phantom: Vec<&String> = reads.difference(&sends).collect();
  assert!(
    phantom.is_empty(),
    "the page reads {phantom:?}, which no response carries. A field that is \
     never sent reads as `undefined`, so a branch guarded on it is DEAD and \
     whatever it was protecting silently does not happen. The wire sends \
     {sends:?}."
  );
}

/// **A REFUSAL'S TEXT IS ACTUALLY READ, OR THE PAGE CANNOT SURFACE ONE.**
///
/// The test above rules out reading a field that does not exist. It does not on
/// its own require the page to read the refusal at all -- a page that read only
/// `result` would pass it while still discarding every message. This is the
/// other half, and it is the arm that fails on the original defect: before the
/// fix the page read `error` and `remedy` and never `message`.
#[test]
fn the_refusals_own_message_is_one_of_the_fields_the_page_reads() {
  let reads = fields_the_page_reads();
  assert!(
    reads.contains("message"),
    "the page never reads `message`, which is where a refusal's text lives, so \
     a refusal cannot reach the reader however well the daemon words it. Reads: \
     {reads:?}"
  );
}
