//! When a file has to be read again, and when a stat settles it.
//!
//! **PURE. IT READS NO FILE AND OPENS NO STORE** (IN-AG-PFIC-001). Every
//! function here decides from values: a policy, what the store recorded, and
//! what a stat says now. The stat and the hash are `reconcile`'s.
//!
//! # Two corpora, two policies, and the asymmetry is deliberate
//!
//! AC-18.3, and it is the D24 interaction read in both directions.
//!
//! **CANON HASHES ALWAYS.** D24 and AC-03.3 require detecting a rewrite that
//! leaves size and mtime untouched, which no amount of stat comparison can see
//! -- that is the whole point of the case. Canon is the model's own truth on
//! its way into the store, so a missed edit there is a wrong ANSWER, not a
//! stale one.
//!
//! **SOURCE IS STAT-THEN-HASH, AND IT MISSES EXACTLY THAT CASE.** Said plainly
//! rather than left to be discovered: a same-size same-mtime edit to a source
//! file is not seen until something else moves that file's stat. The
//! justification is consequence, not convenience. A missed edit in source costs
//! ONE STALE HIT -- a line number that no longer matches, which WP-19's
//! envelope already reports as stale by comparing the indexed bytes at read
//! time -- where hashing every source file in a repository on every reconcile
//! is paid on every reconcile, by everyone, forever.
//!
//! Both policies are asserted against that same missed-edit case, in opposite
//! directions, and the pair is the measurement: one catches it, one is shown
//! not to.

use super::corpus::Corpus;

/// How freshness is decided for a corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
  /// Read and hash the bytes every time.
  HashAlways,
  /// Trust a stat that matches; read only when it does not.
  StatThenHash,
}

/// The policy this corpus is under.
///
/// **A MATCH RATHER THAN A FIELD, so a corpus added later cannot default into
/// the cheap policy by omission** -- the compiler asks the question instead.
pub fn policy_for(corpus: &Corpus) -> Policy {
  match corpus {
    Corpus::Canon => Policy::HashAlways,
    Corpus::Prose | Corpus::Code { .. } => Policy::StatThenHash,
  }
}

/// What a stat says about a file: the two facts a cheap comparison has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stamp {
  pub size: u64,
  /// RFC3339, as `sync::stamp_of` spells it. The comparison is textual and the
  /// two spellings have one home for that reason.
  pub mtime: String,
}

/// Must this file's bytes be read to answer whether it has changed since it was
/// indexed?
///
/// `seen` is what a stat says now. `recorded` is `None` when the index has
/// never held this path, which is a
/// read under either policy: there is nothing to compare against, and guessing
/// fresh would leave a file permanently unindexed with a row saying it is fine.
pub fn must_read(policy: Policy, recorded: Option<&Stamp>, seen: &Stamp) -> bool {
  let Some(recorded) = recorded else {
    return true;
  };
  match policy {
    Policy::HashAlways => true,
    Policy::StatThenHash => recorded != seen,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn stamp(size: u64, modified: &str) -> Stamp {
    Stamp {
      size,
      mtime: modified.to_string(),
    }
  }

  #[test]
  fn canon_hashes_a_file_whose_stat_did_not_move_and_source_does_not() {
    // **THE MISSED-EDIT CASE, PUT TO BOTH POLICIES.** Identical stamps, which
    // is what a same-size same-mtime rewrite leaves behind.
    let then = stamp(120, "2026-09-12T08:00:00Z");
    let seen = stamp(120, "2026-09-12T08:00:00Z");

    assert!(
      must_read(Policy::HashAlways, Some(&then), &seen),
      "canon reads the bytes anyway -- a stat cannot see the case AC-03.3 is about"
    );
    assert!(
      !must_read(Policy::StatThenHash, Some(&then), &seen),
      "and source does not, which MISSES that edit. Stated rather than hidden: \
       the cost is one stale hit, and the envelope reports staleness at read time"
    );
  }

  #[test]
  fn a_stat_that_moved_is_read_under_either_policy() {
    let then = stamp(120, "2026-09-12T08:00:00Z");
    for seen in [
      stamp(121, "2026-09-12T08:00:00Z"),
      stamp(120, "2026-09-12T09:00:00Z"),
    ] {
      assert!(must_read(Policy::StatThenHash, Some(&then), &seen));
      assert!(must_read(Policy::HashAlways, Some(&then), &seen));
    }
  }

  #[test]
  fn a_path_the_index_has_never_held_is_read() {
    let seen = stamp(120, "2026-09-12T08:00:00Z");
    assert!(must_read(Policy::StatThenHash, None, &seen));
    assert!(must_read(Policy::HashAlways, None, &seen));
  }

  #[test]
  fn the_cheap_policy_is_never_reached_by_omission() {
    assert_eq!(policy_for(&Corpus::Canon), Policy::HashAlways);
    assert_eq!(policy_for(&Corpus::Prose), Policy::StatThenHash);
    assert_eq!(
      policy_for(&Corpus::Code { lang: Some("rust") }),
      Policy::StatThenHash
    );
    assert_eq!(
      policy_for(&Corpus::Code { lang: None }),
      Policy::StatThenHash
    );
  }
}
