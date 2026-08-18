//! **v2's issue tracker, which went entirely unread until WP-10 measured it.**
//!
//! `intent/issues/{OPEN,CLOSED}/<nnnn>/<nnnn>-<slug>.md` shares no ancestor
//! directory with the threads, so a thread walk finds none of it -- and the
//! failure mode is the one `retired_settings` names: nothing recognises an
//! issue, nothing is emitted, and **every count reconciles perfectly against
//! zero.**
//!
//! # Measured at `42fb5269` before a line was written
//!
//! 61 issues, 23 OPEN and 38 CLOSED. All six frontmatter keys present on all
//! 61, so every one has a home in the model and nothing is carried as legacy.
//! `severity` is one of four -- medium 34, high 17, low 9, critical 1. The
//! directory and the `status:` field agree on all 61, and every `id` matches
//! its directory name.
//!
//! **The scanner's output was then counted independently off the canon JSON and
//! reproduces all of it exactly**: 23 open / 38 closed, the same four severity
//! counts, reporter 61 of 61, closed 0 of 61. Two derivations of one subject by
//! two code paths -- the only arrangement in which either count could have
//! falsified the other.
//!
//! # The two traps this estate actually contains
//!
//! **THE FRONTMATTER IS PARSED, NEVER GREPPED.** A line-oriented scan for
//! `^status:` over these files returns FOUR values -- CLOSED 38, OPEN 23, WIP
//! 3, Done 1, which is 65 readings over 61 files -- because issue BODIES quote
//! status lines while describing the bug. **A grep-shaped reader would have
//! invented two statuses this estate does not have**, and they would have
//! looked exactly like real ones.
//!
//! **EVERY `id` IS QUOTED**: `id: "0015"`, on all 61. Parsed without stripping
//! the quotes, every issue in the estate fails and the migration reports an
//! empty tracker with every count agreeing against zero.

mod common;

use common::Fixture;
use intentsvcs::finding::FindingClass;
use intentsvcs::legacy;
use intentsvcs::model::IssueStatus;

fn project(fixture: &Fixture) {
  fixture.write_file(
    "intent/.config/config.json",
    "{\"intent_version\":\"2.19.0\",\"project_name\":\"P\",\"author\":\"cc\",\"intent_dir\":\"intent\",\"languages\":[\"rust\"]}\n",
  );
}

/// One v2 issue, written the way v2 writes one.
fn issue(fixture: &Fixture, bucket: &str, num: &str, slug: &str, front: &str, body: &str) {
  fixture.write_file(
    &format!("intent/issues/{bucket}/{num}/{num}-{slug}.md"),
    &format!("---\n{front}---\n\n# {num}: A title\n\n{body}"),
  );
}

fn scan(fixture: &Fixture) -> legacy::Scan {
  legacy::scan(&fixture.project()).expect("scan the v2 estate")
}

fn v2_front(id: &str, status: &str) -> String {
  format!(
    "id: \"{id}\"\ntitle: a title: with a colon in it\ndate: 2026-08-05\nreporter: matts\nstatus: {status}\nseverity: medium\n"
  )
}

#[test]
fn both_buckets_are_read_and_every_key_finds_its_home() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "0007",
    "a-live-one",
    &v2_front("0007", "OPEN"),
    "## Summary\n\nx\n",
  );
  issue(
    &fixture,
    "CLOSED",
    "0015",
    "a-done-one",
    &v2_front("0015", "CLOSED"),
    "## Summary\n\ny\n",
  );
  let scan = scan(&fixture);

  assert_eq!(scan.issues.len(), 2, "{:?}", scan.issues);
  let open = &scan.issues[0];
  assert_eq!(open.number, 7, "the quoted id parses to a number");
  assert_eq!(open.slug, "a-live-one", "the slug is the filename tail");
  assert_eq!(open.status, IssueStatus::Open);
  assert_eq!(open.created, "2026-08-05", "v2's `date` is the domain date");
  assert_eq!(open.reporter.as_deref(), Some("matts"));
  assert_eq!(open.severity.as_deref(), Some("medium"));
  assert_eq!(scan.issues[1].status, IssueStatus::Closed);
}

/// **A title contains colons and must survive whole.** 0015's real title is
/// `... does not exist: the citation is never resolved`; a frontmatter reader
/// splitting on the LAST colon truncates it at its own punctuation.
#[test]
fn a_title_containing_a_colon_is_not_truncated_at_it() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "0007",
    "x",
    &v2_front("0007", "OPEN"),
    "## Summary\n\nx\n",
  );

  assert_eq!(
    scan(&fixture).issues[0].title,
    "a title: with a colon in it"
  );
}

/// **`closed` is never back-filled.** v2's format has no closed date, and a
/// file's mtime is a fact about the file rather than about the world. All-NULL
/// is a readable answer -- "this came from v2" -- and a plausible date is not.
#[test]
fn a_closed_issue_gets_no_invented_closed_date() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "CLOSED",
    "0015",
    "x",
    &v2_front("0015", "CLOSED"),
    "## Summary\n\nx\n",
  );

  assert_eq!(scan(&fixture).issues[0].closed, None);
}

/// **THE GREP TRAP, driven.** The body quotes a status line while describing
/// the bug, exactly as this estate's bodies do -- 65 readings over 61 files.
/// The frontmatter is what decides.
#[test]
fn a_status_line_quoted_in_the_body_does_not_become_the_issues_status() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "0007",
    "x",
    &v2_front("0007", "OPEN"),
    "## Reproduction\n\nThe work package reads:\n\nstatus: Done\n\nand the gate believes it.\n",
  );
  let scan = scan(&fixture);

  assert_eq!(
    scan.issues[0].status,
    IssueStatus::Open,
    "the body's quoted `status: Done` was read as the issue's own status"
  );
  assert!(
    scan.residue.is_empty() && scan.carried.is_empty(),
    "and it is not a finding either -- it is prose: {:?} {:?}",
    scan.residue,
    scan.carried
  );
}

/// The strip must not be load-bearing in the other direction: an unquoted id
/// is still an id.
#[test]
fn an_unquoted_id_parses_too() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "0007",
    "x",
    "id: 0007\ntitle: t\ndate: 2026-08-05\nreporter: matts\nstatus: OPEN\nseverity: low\n",
    "## Summary\n\nx\n",
  );

  assert_eq!(scan(&fixture).issues[0].number, 7);
}

/// An id that is not a number has no identity to convert, and says so rather
/// than being skipped in silence.
#[test]
fn an_unparseable_id_is_reported_rather_than_dropped() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "banana",
    "x",
    "id: \"banana\"\ntitle: t\ndate: 2026-08-05\nreporter: matts\nstatus: OPEN\nseverity: low\n",
    "## Summary\n\nx\n",
  );
  let scan = scan(&fixture);

  assert!(scan.issues.is_empty(), "{:?}", scan.issues);
  assert!(
    scan
      .residue
      .iter()
      .any(|f| f.class == FindingClass::UnparseableRow),
    "a dropped issue with no finding is indistinguishable from an estate that \
     never had it: {:?}",
    scan.residue
  );
}

/// **The FRONTMATTER decides when the two disagree, because that is what an
/// author wrote and the directory is where a tool put it.**
///
/// **The disagreement is deliberately NOT reported**, and the reason is worth
/// the test rather than a comment alone: a finding needs a residue class, none
/// of the nine declared ones fits, and hv's moratorium names new classes. The
/// two agree on all 61 issues of this estate, so nothing is lost today --
/// but a wrong-but-declared class would put a misclassification into an
/// operator's work list, which is worse than silence.
#[test]
fn the_frontmatter_decides_when_the_bucket_disagrees_with_it() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "CLOSED",
    "0015",
    "x",
    &v2_front("0015", "OPEN"),
    "## Summary\n\nx\n",
  );
  let scan = scan(&fixture);

  assert_eq!(
    scan.issues[0].status,
    IssueStatus::Open,
    "the directory said CLOSED and the author wrote OPEN"
  );
  assert!(
    scan.residue.is_empty() && scan.carried.is_empty(),
    "and no finding is emitted, because no declared class fits it and \
     inventing one is what the moratorium names: {:?} {:?}",
    scan.residue,
    scan.carried
  );
}

/// **THE CONTROL.** Without it every test above passes against a scanner that
/// reports findings constantly, or one that invents issues from directories
/// that hold none.
#[test]
fn a_clean_estate_yields_its_issues_and_no_findings() {
  let fixture = Fixture::new();
  project(&fixture);
  issue(
    &fixture,
    "OPEN",
    "0007",
    "x",
    &v2_front("0007", "OPEN"),
    "## Summary\n\nx\n",
  );
  let scan = scan(&fixture);

  assert_eq!(scan.issues.len(), 1);
  assert!(
    scan.residue.is_empty() && scan.carried.is_empty(),
    "{:?} {:?}",
    scan.residue,
    scan.carried
  );
}

/// A project with no issue estate at all converts cleanly -- most do.
#[test]
fn an_absent_issue_estate_is_not_a_finding() {
  let fixture = Fixture::new();
  project(&fixture);
  let scan = scan(&fixture);

  assert!(scan.issues.is_empty());
  assert!(scan.residue.is_empty() && scan.carried.is_empty());
}

/// **The body is carried WHOLE and byte-for-byte, which is the property that
/// lets the disk stop being where an issue lives.**
///
/// It had no field at all until vc specced one: 40 files and 443,643 bytes on
/// this project, in the v2 estate and nowhere else. Under the old model that
/// was residue; under an index-plus-render-on-demand disk it is what the first
/// render destroys.
///
/// **Reassembling the frontmatter and this reproduces the file exactly**, so
/// the round trip needs nothing to compensate for it. An earlier cut trimmed,
/// on `Thread::preamble`'s precedent, and that was safe only while nothing
/// rendered an issue back to disk -- a normalisation that needs a future
/// component to remember something is a scheduled defect, not a tidy-up.
#[test]
fn the_body_is_carried_verbatim_including_the_blank_line_below_the_frontmatter() {
  let fixture = Fixture::new();
  project(&fixture);
  let front = v2_front("0007", "OPEN");
  let body = "## Tags\n\nshell, parsing\n\n## Summary\n\nIt printed `ok` and was not.\n";
  issue(&fixture, "OPEN", "0007", "x", &front, body);

  let scan = scan(&fixture);
  let carried = &scan.issues[0].body;

  // The fixture writes `---\n{front}---\n\n# 0007: A title\n\n{body}`, so
  // everything below the frontmatter fence is exactly this.
  assert_eq!(
    carried,
    &format!("\n# 0007: A title\n\n{body}"),
    "verbatim: the blank line, the `# ` title line and every heading the author \
     wrote, in the order they wrote them"
  );
  assert!(
    carried.ends_with('\n'),
    "the trailing newline survives -- a POSIX text file has one, and losing it \
     costs a byte on every round trip once the renderer exists"
  );
}

/// **The `# <nnnn>: <title>` line is CARRIED, not reconstructed, and that is
/// measured rather than assumed.**
///
/// It rebuilds from `number` + `title` on 37 of this project's 40 issues and
/// fails on three, whose v2 frontmatter QUOTES the title -- so a reconstruction
/// would have been correct-looking on 37 files, wrong on 3, and silent on all
/// 40. The same estate is why `id` is unquoted before parsing.
#[test]
fn a_heading_that_does_not_match_the_frontmatter_title_still_survives() {
  let fixture = Fixture::new();
  project(&fixture);
  fixture.write_file(
    "intent/issues/OPEN/0011/0011-x.md",
    "---\nid: \"0011\"\ntitle: \"a quoted title\"\ndate: 2026-08-05\nreporter: matts\nstatus: OPEN\nseverity: medium\n---\n\n# 0011: a quoted title\n\n## Summary\n\nx\n",
  );

  let scan = scan(&fixture);
  assert!(
    scan.issues[0].body.contains("# 0011: a quoted title"),
    "the heading the author wrote is in the body, whatever the frontmatter says"
  );
}
