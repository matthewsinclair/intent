//! AT-07.7 / AC-07.7: **the four COLLECTION addresses resolve**, and their
//! denominator is read out of CANON rather than hand-copied.
//!
//! # Why this is a new row and not a widening of AC-07.1
//!
//! AC-07.1's population is *every ENTITY form in D57-8's list*, and it is
//! faithful to that: against nine entity forms it is satisfied and stays
//! satisfied. **What satisfying it completely still leaves broken is four
//! addressable COLLECTION forms with no resolution coverage at all.** They
//! entered the grammar through D57-8's `POST` clause and its under-addressing
//! clause while D57-8's own enumerating fence stayed at nine, so they were
//! mandated, implemented, and unreachable by any test that enumerates the
//! grammar from the document.
//!
//! # The denominator comes from canon, and that is the whole point
//!
//! `d57_8_forms()` in `address_resolution_single_home.rs` is a HAND-COPY
//! asserted to equal nine, and **the hand-copy is the mechanism that went four
//! short**: the design grew a second fence and the roster beside it did not.
//! So this row does not copy anything. It reads
//! `intent/.canon/st/ST0057.json`, takes `design.md`'s attachment text, and
//! parses the fence out of it.
//!
//! Canon rather than `intent/st/ST0057/design.md` because **an attachment on
//! disk is a projection this very thread exists to make optional** -- a test
//! whose denominator is the disk copy stops working the day the thread
//! dehydrates. Canon carries the full text plus a `sha256`, so the disk file is
//! used only as a CROSS-CHECK when it happens to be present: that turns *canon
//! is stale* into a named failure instead of a silently short denominator, and
//! it degrades correctly when the file is gone.
//!
//! Reading canon and refusing on finding nothing is the estate's existing
//! shape, not a new one -- `preconditions.rs` reads canon, never the generated
//! view, and refuses rather than passing vacuously.
//!
//! **And not from `address.rs` either.** A denominator read out of the
//! implementation agrees with the implementation by construction, which is a
//! test that can only ever pass.
//!
//! # Singularity lives in the other file, deliberately
//!
//! `no_second_resolver_exists` (`address_resolution_single_home.rs:138`) asks
//! *does any other crate spell the scheme*, which is a **form-independent**
//! question -- so it already covers these four, and a second scheme scan here
//! would be two answers to one question. What this file adds instead is the
//! arm that file **cannot** make: an exhaustive match over `Entity`, so a
//! fourteenth variant does not compile until somebody decides whether it is a
//! collection. That is form-DEPENDENT, and it is the half a scheme scan is
//! blind to.

use intentsvcs::address::{Entity, Format, parse};
use intentsvcs::model::Thread;
use sha2::{Digest, Sha256};
use testkit::repo_root;

/// A thread id to substitute for `{stid}`. Any real-looking id does; the fence
/// is a grammar, not a claim about which threads exist.
const STID: &str = "ST0056";

/// D57-8's SECOND fence, read out of canon. **This is the denominator.**
///
/// Every failure here is a refusal naming what it could not find, never a
/// silently short list -- a denominator that can shrink without saying so is
/// the defect this row was minted for.
fn d57_8_collection_fence() -> Vec<String> {
  let canon_path = repo_root().join("intent/.canon/st/ST0057.json");
  let raw = std::fs::read_to_string(&canon_path).unwrap_or_else(|e| {
    panic!(
      "AT-07.7's denominator is canon and canon is unreadable at {}: {e}\n  \
       This row asserts a population read from the record of intent. It does not\n  \
       fall back to the implementation, because a denominator taken from the thing\n  \
       under test agrees with it by construction.",
      canon_path.display()
    )
  });

  // Typed rather than `Value`, so a schema change is a loud deserialisation
  // failure here instead of a silent `None` two lines down.
  let thread: Thread = serde_json::from_str(&raw)
    .unwrap_or_else(|e| panic!("ST0057 canon did not read as a Thread: {e}"));

  let design = thread
    .attachments
    .iter()
    .find(|a| a.path == "design.md")
    .unwrap_or_else(|| {
      panic!(
        "ST0057 canon carries no `design.md` attachment, so D57-8 has no durable\n  \
         home to read the denominator from. Attachments present: {:?}",
        thread
          .attachments
          .iter()
          .map(|a| &a.path)
          .collect::<Vec<_>>()
      )
    });

  let text = design.text.as_deref().unwrap_or_else(|| {
    panic!(
      "ST0057's `design.md` is carried OPAQUE (no `text`), so the fence cannot be\n  \
       read from canon. D57-8 is prose and must be carried as text."
    )
  });

  cross_check_against_disk(design.sha256.as_str(), design.bytes, text);
  second_fence_of_d57_8(text)
}

/// **Canon is authoritative here and disk is the CROSS-CHECK, not the source.**
///
/// An attachment is authored on disk, so canon lags until `sync --to-store`.
/// If the two disagree, the denominator this row is about may be stale -- and a
/// stale denominator is exactly the failure that produced this row. So the
/// divergence is named rather than tolerated.
///
/// **Absent is not divergent.** When the thread is dehydrated there is no disk
/// file to compare, and canon alone is the correct and complete answer.
fn cross_check_against_disk(canon_sha: &str, canon_bytes: u64, canon_text: &str) {
  let disk_path = repo_root().join("intent/st/ST0057/design.md");
  let Ok(disk) = std::fs::read(&disk_path) else {
    // Dehydrated, or never realised. Canon stands alone, which is the point of
    // sourcing from canon in the first place.
    return;
  };

  let disk_sha = format!("{:x}", Sha256::digest(&disk));
  assert_eq!(
    disk_sha,
    canon_sha,
    "ST0057's `design.md` on disk does not match the copy in canon.\n  \
     canon: {canon_sha} ({canon_bytes} bytes)\n  \
     disk:  {disk_sha} ({} bytes)\n  \
     AT-07.7's denominator is read from CANON, so a disk edit that has not been\n  \
     synced means this row is measuring a stale D57-8. Run\n  \
     `intent sync --to-store ST0057` (thread-scoped -- it carries nothing of a\n  \
     peer's) and re-run, or revert the disk edit.",
    disk.len()
  );

  // Belt and braces on the one field that could disagree with the text while
  // the hash still matched -- they are written together and can only diverge
  // through a hand edit of canon.
  assert_eq!(
    canon_bytes as usize,
    canon_text.len(),
    "canon's recorded byte count disagrees with the text it carries"
  );
}

/// D57-8 carries TWO fences: the nine entity forms, then the four collections.
///
/// **Asserting there are exactly two is load-bearing.** If the document grows a
/// third, "the second one" silently stops meaning the collections and this row
/// would go on passing over the wrong list.
fn second_fence_of_d57_8(text: &str) -> Vec<String> {
  let start = text
    .find("## D57-8")
    .unwrap_or_else(|| panic!("ST0057's design carries no `## D57-8` section"));
  let section = &text[start..];
  // Bound at the next same-level heading so a later section's fences cannot be
  // counted as D57-8's.
  let section = match section[2..].find("\n## ") {
    Some(offset) => &section[..offset + 2],
    None => section,
  };

  let fences: Vec<&str> = section
    .split("```")
    .enumerate()
    .filter(|(i, _)| i % 2 == 1)
    .map(|(_, body)| body)
    .collect();

  assert_eq!(
    fences.len(),
    2,
    "D57-8 must carry exactly two fenced blocks -- the nine entity forms, then\n  \
     the four collection addresses. Found {}. If the document grew a third,\n  \
     `second_fence_of_d57_8` no longer names the collections and this row's\n  \
     denominator is wrong. Fix the selector, do not relax the count.",
    fences.len()
  );

  let forms: Vec<String> = fences[1]
    .lines()
    .skip(1) // the info string on the opening delimiter's line
    .map(str::trim)
    .filter(|l| !l.is_empty())
    .map(str::to_owned)
    .collect();

  assert!(
    !forms.is_empty(),
    "D57-8's second fence is empty, so the denominator would be vacuous"
  );
  forms
}

/// **Exhaustive ON PURPOSE: a fourteenth `Entity` variant does not compile
/// until it is named here.**
///
/// This is the arm `no_second_resolver_exists` cannot make. A scheme scan asks
/// whether a second parser exists and is blind to the population growing; this
/// asks whether every form has been classified, and a new variant fails to
/// COMPILE rather than quietly joining neither set.
///
/// The wildcard is banned here for the same reason `Entity::form()` bans it and
/// for the reason `view_path_of` was wrong to allow it: a hand-kept roster
/// beside an enum is the shape that shrinks.
fn is_collection(entity: &Entity) -> bool {
  match entity {
    Entity::Threads
    | Entity::Issues
    | Entity::WpCollection { .. }
    | Entity::AcCollection { .. } => true,
    Entity::Thread { .. }
    | Entity::Wp { .. }
    | Entity::Ac { .. }
    | Entity::At { .. }
    | Entity::Attachment { .. }
    | Entity::Issue { .. }
    | Entity::Node { .. }
    | Entity::NodeInbox { .. }
    | Entity::Event { .. } => false,
  }
}

/// Every collection form in D57-8's second fence parses and resolves.
///
/// **The population is asserted, not assumed.** The fence's own length is the
/// denominator and it is stated in the assertion, so a form added to the design
/// without a parser arm fails here rather than going silently unaddressable --
/// and a form REMOVED fails too.
#[test]
fn every_d57_8_collection_form_resolves() {
  let fence = d57_8_collection_fence();
  assert_eq!(
    fence.len(),
    4,
    "D57-8's second fence declares the collection addresses and this row's\n  \
     population is exactly that fence. Found {}: {fence:?}\n  \
     If the design gained or lost a collection form, this row must move with it.",
    fence.len()
  );

  let mut resolved: Vec<Entity> = Vec::new();

  for template in &fence {
    let url = template.replace("{stid}", STID);
    let address =
      parse(&url).unwrap_or_else(|e| panic!("{url} is declared in D57-8 and must resolve: {e}"));

    assert!(
      is_collection(&address.entity),
      "{url} is in D57-8's COLLECTION fence but resolved to {}, which is not a\n  \
       collection. The fence and the parser disagree about what this address is.",
      address.entity.form()
    );
    assert!(address.is_local(), "{url} has the empty authority");
    assert_eq!(address.format, None, "no `?format=` was asked for");
    assert_eq!(
      address.to_url(),
      url,
      "{url} did not survive the round trip, so the parse recognised it without\n  \
       resolving it"
    );

    for f in [Format::Json, Format::Md] {
      let with_format = format!("{url}?format={}", f.as_str());
      let b = parse(&with_format).unwrap_or_else(|e| panic!("{with_format} must resolve: {e}"));
      assert_eq!(b.format, Some(f));
      assert_eq!(b.to_url(), with_format);
      assert_eq!(
        b.entity, address.entity,
        "a format must not change WHAT is addressed, only its representation"
      );
    }

    assert!(
      !resolved.contains(&address.entity),
      "{url} resolved to {}, which another form in the fence already produced.\n  \
       Four declared addresses must reach four distinct entities; two collapsing\n  \
       into one is an address that cannot be a POST target.",
      address.entity.form()
    );
    resolved.push(address.entity);
  }

  assert_eq!(resolved.len(), 4, "four forms must reach four entities");
}

/// **The dependency this row rests on, made visible.**
///
/// Half of AC-07.7's contract -- *resolution is implemented ONCE for these as
/// for the entity forms* -- is discharged by `no_second_resolver_exists`, which
/// is form-independent and therefore already covers these four. Writing a
/// second scheme scan here would be the Highlander defect.
///
/// **But a contract half-held in another file can be deleted without this row
/// noticing**, so the dependency is pinned rather than left as a comment. This
/// checks a NAME, which would be the wrong instrument for a capability
/// question; it is the right one here because the failure direction is safe --
/// a rename fails this loudly and is fixed in one line, where the hazard being
/// guarded is silent removal.
#[test]
fn the_singularity_arm_this_row_depends_on_still_exists() {
  let sibling =
    repo_root().join("native/rust/crates/intentsvcs/tests/address_resolution_single_home.rs");
  let text = std::fs::read_to_string(&sibling).unwrap_or_else(|e| {
    panic!(
      "AT-07.7 leans on the singularity arm in {} and it is unreadable: {e}",
      sibling.display()
    )
  });

  assert!(
    text.contains("fn no_second_resolver_exists"),
    "`no_second_resolver_exists` is where AC-07.7's singularity half is\n  \
     discharged -- it asks whether any other crate spells the scheme, which is\n  \
     form-independent and so already covers the four collection forms.\n  \
     It is gone or renamed. Either restore it, or give this row its own\n  \
     structural singularity arm; do not leave the half unheld."
  );
}
