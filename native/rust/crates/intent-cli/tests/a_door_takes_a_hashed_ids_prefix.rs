//! ST0095 WP-07 (AC-07.1), driven through the binary: the CLI's thread, issue
//! and package doors, and `edit`'s, take the shown form of a hashed id, a prefix of it, and
//! refuse one too short to be a prefix in the waist's words. The ids come from
//! the mint, as `st new`, `wp new` and `issues add` print them.

use crate::common::{ok, run};
use intentsvcs::id::{IssueId, MIN_PREFIX_HEX, THREAD_PREFIX, ThreadId, WpId};

// ST0095 AT-10.1: the arm below.
#[test]
fn st_show_issues_show_and_wp_show_take_a_prefix() {
  let dir = tempfile::tempdir().expect("tempdir");
  let root = dir.path();
  ok(root, &["init", "prefixproj"]);

  let st = crate::common::created(&ok(root, &["st", "new", "A thread typed by prefix"]));
  let wp = crate::common::created(&ok(
    root,
    &["wp", "new", st.as_str(), "A package typed by prefix"],
  ));
  let issue = crate::common::created(&ok(root, &["issues", "add", "An issue typed by prefix"]));

  let thread = ThreadId::parse(&st).expect("the mint prints a stored thread id");
  let package = WpId::parse(
    intentsvcs::id::split_wp_address(&wp)
      .expect("a package's id is its address")
      .1,
  )
  .expect("a stored package id");
  let issue = IssueId::parse(&issue).expect("the mint prints a stored issue id");
  // A shown form that is the stored id would pass every assertion below
  // without completing anything.
  assert!(
    thread.is_hashed() && package.is_hashed() && issue.is_hashed(),
    "the mint makes hashed ids, whose shown form is a prefix"
  );

  assert!(
    ok(root, &["st", "show", &thread.shown()]).contains("A thread typed by prefix"),
    "`st show` takes the thread's shown form"
  );
  assert!(
    ok(root, &["issues", "show", &issue.shown()]).contains("An issue typed by prefix"),
    "`issues show` takes the issue's shown form"
  );
  let typed = format!("{}/{}", thread.shown(), package.shown());
  assert!(
    ok(root, &["wp", "show", &typed]).contains("A package typed by prefix"),
    "`wp show` takes `{typed}`, both halves prefixes"
  );
  assert_eq!(
    ok(root, &["edit", "st", &thread.shown(), "--path"]),
    ok(root, &["edit", "st", &thread.stored(), "--path"]),
    "`edit st` opens by the shown form what it opens by the whole id"
  );

  // `edit issue` is refused by kind before any id is read (issue 0334), so the
  // issue arm of `edit`'s door is reached through `browse`, which reads its id
  // there before it looks for a daemon. With no daemon answering, an id the
  // door takes reaches that refusal, and one it refuses never does.
  let home = tempfile::tempdir().expect("an isolated home, where no intentd answers");
  let browse = |id: &str| {
    let argv = ["browse", "issue", id];
    let out = crate::common::intent()
      .args(argv)
      .current_dir(root)
      .env("HOME", home.path())
      .stdin(testkit::lifeline_for(&argv))
      .output()
      .expect("the intent binary runs");
    (
      String::from_utf8_lossy(&out.stderr).into_owned(),
      out.status.code(),
    )
  };
  let whole = browse(&issue.stored());
  assert!(
    whole.0.contains("no `intentd` is answering"),
    "control: the whole id passes the door and reaches the daemon check: {}",
    whole.0
  );
  assert_eq!(
    browse(&issue.shown()),
    whole,
    "`browse issue` passes the door by the shown form as by the whole id"
  );

  // A hex letter, so the spelling cannot read as an ordinal: digits alone
  // after the marker are the ordinal (design, "Typing an id"), and the front
  // of a minted id is all digits in about one run in four.
  let short = format!("{THREAD_PREFIX}{}", "f".repeat(MIN_PREFIX_HEX - 1));
  let (_, stderr, code) = run(root, &["st", "show", &short]);
  assert_ne!(code, 0, "`{short}` is too short to be a prefix");
  assert!(
    stderr.contains("too short"),
    "the refusal says why, in the waist's words: {stderr}"
  );
}
