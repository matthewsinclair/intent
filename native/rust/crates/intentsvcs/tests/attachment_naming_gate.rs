//! AT-03.3 / ST0057 AC-03.3: **the naming gate rejects, at ingest, any
//! attachment path that cannot be given both a canon path and a URL -- and
//! rejection is NOT retroactive.**
//!
//! # The rule is DERIVED from its two consumers, and that is what is under test
//!
//! A list of forbidden characters would be a second opinion about what canon
//! storage and the addressing layer accept, and a second opinion drifts the
//! moment either changes -- silently, and in the direction that admits a name
//! one of them cannot take. So each half asks the mechanism itself: the canon
//! half builds the real path through `canon_blob_rel`, the URL half round-trips
//! through the real `address::parse`.
//!
//! **The consequence worth testing is that the gate cannot disagree with the
//! addressing layer**, so there is an arm that asserts exactly that rather than
//! asserting a list of characters somebody chose.
//!
//! # `..` is a write outside the thread, not an untidy name
//!
//! Hydration WRITES to the canon path. A name resolving outside the thread's
//! own canon directory is a write to wherever the author's `..` points, so the
//! containment test is the security half of this criterion and the URL test is
//! the addressability half.
//!
//! # Not retroactive, and it FALLS OUT rather than being arranged
//!
//! Refusal at ingest means the file is not carried. `organize` then meets a
//! `ThreadFile::Attachment` the store does not hold, reports it UNCLAIMED at
//! row five, and never removes it -- because it is the only copy. So an
//! existing violator is named on every run and left exactly where its author
//! put it. **That is asserted here, not assumed**, because "we simply won't
//! delete it" is the kind of promise that survives a refactor only if something
//! checks.

mod common;

use common::{Fixture, sample_thread};
use intentsvcs::project::{BadName, Project, attachment_name, canon_blob_rel};
use testkit::repo_root;

const ID: &str = "ST0001";

/// **The control, and it is the whole estate rather than a chosen example.**
/// Every attachment path people have actually authored must pass, or the gate
/// is a rule about the model rather than about the data.
#[test]
fn every_attachment_path_in_the_estate_is_acceptable() {
  let root = repo_root();
  let project = Project::open(&root).expect("the real project opens");
  let canon = intentsvcs::ingest::read(&project).expect("canon reads");

  let mut checked = 0usize;
  let mut refused: Vec<String> = Vec::new();
  for thread in &canon.threads {
    for att in &thread.attachments {
      checked += 1;
      if let Err(bad) = attachment_name(&thread.id, &att.path) {
        refused.push(format!("{}/{}: {bad}", thread.id, att.path));
      }
    }
  }

  eprintln!(
    "AT-03.3: {checked} authored attachment path(s) checked across {} thread(s)",
    canon.threads.len()
  );
  assert!(
    checked > 0,
    "no attachment path was checked, so this arm's population cannot contain the failure it tests \
     for"
  );
  assert!(
    refused.is_empty(),
    "{} path(s) people have already authored are refused by the gate, which makes it retroactive \
     by accident:\n  {}",
    refused.len(),
    refused.join("\n  ")
  );
}

/// **THE SECURITY HALF: a name that escapes the thread's canon directory is
/// refused.**
#[test]
fn a_path_that_would_write_outside_the_threads_canon_directory_is_refused() {
  for escape in [
    "../ST0002/stolen.md",
    "../../../../etc/passwd",
    "..",
    "a/../../b.md",
  ] {
    let verdict = attachment_name(ID, escape);
    assert!(
      matches!(verdict, Err(BadName::EscapesCanon { .. })),
      "{escape:?} was accepted, so hydration would write it to {:?} -- outside the thread, at a \
       location its author chose",
      canon_blob_rel(ID, escape)
    );
  }

  // The control: a nested path that does NOT escape is fine, so the rule is
  // about containment and not about separators.
  assert_eq!(
    attachment_name(ID, "notes/day-one/reference.md"),
    Ok(()),
    "a legitimately nested attachment was refused, so the containment test is rejecting depth \
     rather than escape"
  );
}

/// **A NAME THAT RESOLVES ONTO ANOTHER ATTACHMENT'S CANON FILE IS REFUSED, AND
/// IT IS A DIFFERENT FAULT FROM AN ESCAPE.**
///
/// `a/./b.md` is contained, addressable, and shares a canon file with the
/// attachment actually named `a/b.md`. Two paths, one sidecar, and the second
/// write destroys the first -- **found by probing the accepted set, not by
/// reading the function, which passed it and looked correct doing so.**
#[test]
fn a_name_that_is_not_already_normalised_is_refused_as_a_collision() {
  for (path, collides_with) in [
    ("a/./b.md", "a/b.md"),
    ("double//sep.md", "double/sep.md"),
    ("trailing/", "trailing"),
  ] {
    match attachment_name(ID, path) {
      Err(BadName::Collides { would_be }) => assert_eq!(
        would_be,
        canon_blob_rel(ID, collides_with),
        "{path:?} is refused, but not for landing on {collides_with:?}"
      ),
      other => panic!(
        "{path:?} gave {other:?}. It resolves onto the canon file of {collides_with:?}, so \
         carrying both would leave one sidecar holding whichever was written last"
      ),
    }
  }
}

/// **AN ESCAPE IS NOT REPORTED AS A COLLISION, AND THE ORDER OF THE TWO CHECKS
/// IS WHAT DECIDES IT.**
///
/// `../x.md` fails both tests -- it escapes AND it is un-normalised -- and only
/// one of them describes it. The first version of the gate answered "collision"
/// for every traversal: correct refusals, wrong reasons, which is the shape a
/// reader trusts and acts on.
#[test]
fn a_traversal_is_reported_as_an_escape_and_not_as_a_collision() {
  assert!(
    matches!(
      attachment_name(ID, "../x.md"),
      Err(BadName::EscapesCanon { .. })
    ),
    "a path traversal is reported as a collision, which sends an operator looking for a duplicate \
     that does not exist and buries the write-outside-the-thread finding under a tidiness one"
  );
}

/// **THE ADDRESSABILITY HALF, AND IT IS ASSERTED AGAINST THE REAL PARSER.**
///
/// Every accepted name must round-trip through `address::parse` to the same
/// path. This is the arm that would fail if the gate and the addressing layer
/// ever came to different views, which is the failure a hand-written character
/// list makes invisible.
#[test]
fn the_gate_and_the_addressing_layer_cannot_disagree() {
  let candidates = [
    "reference.md",
    "notes/day-one.txt",
    "a b.md",
    "weird#fragment.md",
    "query?x=1.md",
    "percent%20encoded.md",
    "trailing/",
    "double//separator.md",
    "",
  ];

  let mut accepted = 0usize;
  let mut refused = 0usize;
  for path in candidates {
    let url = format!("intent:///threads/{ID}/attachments/{path}");
    let round_trip = intentsvcs::address::parse(&url)
      .ok()
      .and_then(|a| match a.entity {
        intentsvcs::address::Entity::Attachment { path, .. } => Some(path),
        _ => None,
      });
    let gate = attachment_name(ID, path);

    match (&gate, round_trip.as_deref()) {
      // Accepted, and the URL really does read back as this path.
      (Ok(()), Some(back)) => {
        accepted += 1;
        assert_eq!(
          back, path,
          "{path:?} was ACCEPTED and its URL reads back as {back:?}. The gate and the addressing \
           layer disagree, which is exactly what deriving the rule from the parser was supposed to \
           make impossible"
        );
      }
      (Ok(()), None) => panic!(
        "{path:?} was accepted and its URL does not parse as an attachment at all -- an accepted \
         name that cannot be addressed is the criterion's own failure"
      ),
      // Refused. Fine either way: the gate may also refuse for the canon half.
      (Err(_), _) => refused += 1,
    }
  }

  assert!(
    accepted > 0 && refused > 0,
    "the candidate set produced {accepted} accepted and {refused} refused. Both must be non-zero \
     or this arm is agreeing with a gate that answers the same way to everything"
  );
}

/// **NOT RETROACTIVE: a badly-named file already on disk is REFUSED at ingest,
/// left where it is, and reported.**
#[test]
fn an_existing_violator_is_reported_and_never_removed() {
  let fx = Fixture::new();
  fx.write_thread(&sample_thread(ID));
  let dir = fx.path(&format!("intent/st/{ID}"));
  std::fs::create_dir_all(&dir).expect("mkdir");
  // **A name that is legal on the filesystem and cannot be ADDRESSED.** `?`
  // opens a query in the URL, so the address does not parse back to this path.
  //
  // **It is deliberately NOT a name with spaces, and that correction is worth
  // recording.** AC-03.3 cites the estate's spaced `.webloc` as its existing
  // violator; measured, a spaced path round-trips through `address::parse`
  // unchanged and the gate ACCEPTS it. The row's example does not fail the
  // row's own rule.
  let violator = "query?x=1.md";
  std::fs::write(dir.join(violator), "# An existing file\n").expect("write");
  std::fs::write(dir.join("fine.md"), "# A well-named one\n").expect("write");

  let project = fx.project();
  let (carried, refused) = project.collect_attachments(ID);

  assert!(
    carried.iter().any(|a| a.path == "fine.md"),
    "the well-named attachment beside the violator was not carried, so the gate refuses a whole \
     thread once one file in it is badly named"
  );
  assert!(
    !carried.iter().any(|a| a.path == violator),
    "the badly-named file was carried into canon, so nothing gates the name at the only door into \
     the model"
  );
  assert!(
    refused.iter().any(|(name, _)| name.contains(violator)),
    "the violator is neither carried nor reported, which is the silent form of this defect: \
     {refused:?}"
  );

  // **AND IT IS STILL THERE.** Refusal must not be a euphemism for removal.
  assert!(
    dir.join(violator).exists(),
    "ingest REMOVED the file it refused. It is the only copy: refusing to carry it and deleting \
     it are opposite acts"
  );
  assert_eq!(
    std::fs::read_to_string(dir.join(violator)).expect("read"),
    "# An existing file\n",
    "the refused file was rewritten"
  );
}

/// The refusal says which of the two halves failed, because the two need
/// different fixes: one is a rename, the other is a file in the wrong place.
#[test]
fn the_refusal_names_which_half_it_failed() {
  let escapes = attachment_name(ID, "../elsewhere.md").expect_err("refused");
  assert!(
    escapes.to_string().contains("outside the thread"),
    "the containment refusal does not say what is wrong: {escapes}"
  );

  let unaddressable = attachment_name(ID, "query?x=1.md").expect_err("refused");
  assert!(
    unaddressable.to_string().contains("addressed"),
    "the addressability refusal does not say what is wrong: {unaddressable}"
  );
  assert_ne!(
    escapes.to_string(),
    unaddressable.to_string(),
    "both halves give the same message, so an operator cannot tell a rename from a misplaced file"
  );
}
