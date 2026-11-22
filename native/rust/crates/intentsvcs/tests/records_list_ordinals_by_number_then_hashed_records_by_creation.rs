//! ST0095 WP-08 (AC-08.1): **every listing orders ordinals by number, then
//! hashed records by creation.** `id::record_order` is the one comparator; this
//! drives it over the two record kinds `id.rs`'s own test cannot build, issues
//! and work packages, and at the disk boundary, where a thread file written
//! before WP-08 holds its hashed packages by hash.
//!
//! Every hashed pair here is chosen so its hashes run against its creation, so
//! an order by id fails each assertion.

use crate::common::{Fixture, sample_issue, sample_thread};
use intentsvcs::id::{IssueId, ThreadId, WpId, record_order};
use intentsvcs::model::{Issue, WorkPackage};
use testkit::against_creation;

fn issue(number: IssueId, origin: Option<&str>) -> Issue {
  let mut issue = sample_issue(1);
  issue.number = number;
  issue.origin = origin.map(str::to_string);
  issue
}

fn package(seq: WpId, origin: Option<&str>) -> WorkPackage {
  let mut wp = sample_thread("ST0001").wps[0].clone();
  wp.seq = seq;
  wp.origin = origin.map(str::to_string);
  wp
}

#[test]
fn issues_list_ordinals_by_number_then_hashed_issues_by_creation() {
  let (early, late) = against_creation(IssueId::from_origin);
  let ordinal = |n| IssueId::from_ordinal(n).expect("an ordinal");
  let mut issues = [
    issue(IssueId::from_origin(late), Some(late)),
    issue(ordinal(10), None),
    issue(IssueId::from_origin(early), Some(early)),
    issue(ordinal(2), None),
  ];
  issues.sort_by(record_order);
  let order: Vec<IssueId> = issues.iter().map(|i| i.number).collect();
  assert_eq!(
    order,
    [
      ordinal(2),
      ordinal(10),
      IssueId::from_origin(early),
      IssueId::from_origin(late)
    ],
    "an issue answers to the order by its number and its origin"
  );
}

#[test]
fn packages_list_ordinals_by_number_then_hashed_packages_by_creation() {
  let (early, late) = against_creation(WpId::from_origin);
  let ordinal = |n| WpId::from_ordinal(n).expect("an ordinal");
  let mut wps = [
    package(WpId::from_origin(late), Some(late)),
    package(ordinal(10), None),
    package(WpId::from_origin(early), Some(early)),
    package(ordinal(2), None),
  ];
  wps.sort_by(record_order);
  let order: Vec<WpId> = wps.iter().map(|wp| wp.seq).collect();
  assert_eq!(
    order,
    [
      ordinal(2),
      ordinal(10),
      WpId::from_origin(early),
      WpId::from_origin(late)
    ],
    "a work package answers to the order by its seq and its origin"
  );
}

#[test]
fn a_thread_file_holding_hashed_packages_by_hash_is_read_in_the_one_order() {
  let (early, late) = against_creation(WpId::from_origin);
  let ordinal = WpId::from_ordinal(2).expect("an ordinal");
  let fx = Fixture::new();
  let mut thread = sample_thread("ST0001");
  // An order no reader may keep: the hashed packages by hash (`late`'s is the
  // lower), and the ordinal deliberately last, so the family split is driven
  // too.
  thread.wps = vec![
    package(WpId::from_origin(late), Some(late)),
    package(WpId::from_origin(early), Some(early)),
    package(ordinal, None),
  ];
  fx.write_thread(&thread);
  let canon = intentsvcs::ingest::read(&fx.project()).expect("the canon reads");
  let read: Vec<WpId> = canon
    .threads
    .iter()
    .find(|t| t.id == "ST0001")
    .expect("the thread is read")
    .wps
    .iter()
    .map(|wp| wp.seq)
    .collect();
  assert_eq!(
    read,
    [ordinal, WpId::from_origin(early), WpId::from_origin(late)],
    "the disk read holds a thread's packages in the order the store reads them back, so nothing moves under a write"
  );
}

#[test]
fn the_disk_read_lists_hashed_threads_and_issues_by_creation_not_by_their_file_names() {
  let (early_thread, late_thread) = against_creation(ThreadId::from_origin);
  let (early_issue, late_issue) = against_creation(IssueId::from_origin);
  let fx = Fixture::new();
  // Written in creation order; the directory walk finds them by name, which
  // for a hashed id is its hash, so it meets the later one first.
  for origin in [early_thread, late_thread] {
    let mut thread = sample_thread("ST0001");
    thread.id = ThreadId::from_origin(origin);
    thread.origin = Some(origin.to_string());
    fx.write_thread(&thread);
  }
  for origin in [early_issue, late_issue] {
    fx.write_issue(&issue(IssueId::from_origin(origin), Some(origin)));
  }
  let canon = intentsvcs::ingest::read(&fx.project()).expect("the canon reads");
  let threads: Vec<ThreadId> = canon.threads.iter().map(|t| t.id.clone()).collect();
  assert_eq!(
    threads,
    [
      ThreadId::from_origin(early_thread),
      ThreadId::from_origin(late_thread)
    ],
    "the disk read lists hashed threads by creation, not in the walk's name order"
  );
  let issues: Vec<IssueId> = canon.issues.iter().map(|i| i.number).collect();
  assert_eq!(
    issues,
    [
      IssueId::from_origin(early_issue),
      IssueId::from_origin(late_issue)
    ],
    "and hashed issues likewise"
  );
}

/// Issue 0743: **the two newest-first orders the views draw are the one record
/// order reversed.** `st list`, the generated thread index and the explorer's
/// /threads call `views::index_order`, and the explorer's /issues calls
/// `views::issue_index_order`. Both compared ids as text, so git-style ids
/// came out in hex order. Every record of a kind shares one status, so the
/// open/closed split does not enter, and every hashed pair runs against its creation.
#[test]
fn the_views_newest_first_orders_list_hashed_records_newest_made_first_above_the_ordinals() {
  let (early, late) = against_creation(ThreadId::from_origin);
  let hashed = |origin: &str| {
    let mut thread = sample_thread("ST0001");
    thread.id = ThreadId::from_origin(origin);
    thread.origin = Some(origin.to_string());
    thread
  };
  let threads = [
    hashed(early),
    sample_thread("ST0010"),
    hashed(late),
    sample_thread("ST0002"),
  ];
  let order: Vec<ThreadId> = intentsvcs::views::index_order(&threads)
    .iter()
    .map(|t| t.id.clone())
    .collect();
  let ordinal_thread = |id: &str| ThreadId::parse(id).expect("an ordinal thread");
  assert_eq!(
    order,
    [
      ThreadId::from_origin(late),
      ThreadId::from_origin(early),
      ordinal_thread("ST0010"),
      ordinal_thread("ST0002"),
    ],
    "threads list newest made first, hashed above ordinals, ordinals by number"
  );

  let (early, late) = against_creation(IssueId::from_origin);
  let ordinal = |n| IssueId::from_ordinal(n).expect("an ordinal");
  let issues = [
    issue(IssueId::from_origin(early), Some(early)),
    issue(ordinal(10), None),
    issue(IssueId::from_origin(late), Some(late)),
    issue(ordinal(2), None),
  ];
  let order: Vec<IssueId> = intentsvcs::views::issue_index_order(&issues)
    .iter()
    .map(|i| i.number)
    .collect();
  assert_eq!(
    order,
    [
      IssueId::from_origin(late),
      IssueId::from_origin(early),
      ordinal(10),
      ordinal(2),
    ],
    "and issues likewise"
  );
}
