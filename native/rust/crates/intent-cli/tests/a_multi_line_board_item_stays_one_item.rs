//! Issue 0532: a board item's text can run to several lines, and every line
//! after its first printed at column 0 -- in the board view, where it left the
//! list item, and under `wb show` and `wb pickup`, where it read as text
//! attached to nothing.
//!
//! **THE VIEW IS JUDGED BY A MARKDOWN PARSER, AS ic FOUND IT.** ic's instrument
//! was `pandoc -f gfm`: Conflab's `dc` board wrote five `## ` headings and the
//! parser read two, because an item holding a fenced block swallowed the rest
//! of the page. `pulldown-cmark`, which the TUI already renders with, is the
//! same kind of instrument in process. Each arm reads the older shape first
//! and must see the defect there, so a green is never a parser that sees
//! nothing.

use std::path::Path;

fn run(cwd: &Path, args: &[&str]) -> (String, i32) {
  let out = crate::common::intent()
    .args(args)
    .current_dir(cwd)
    .env("HOME", testkit::fixture_home())
    .stdin(testkit::lifeline_for(args))
    .output()
    .expect("run intent");
  (
    format!(
      "{}{}",
      String::from_utf8_lossy(&out.stdout),
      String::from_utf8_lossy(&out.stderr)
    ),
    out.status.code().unwrap_or(-1),
  )
}

/// An item whose lines after the first are a list.
const LISTED: &str = "two steps:\n- first\n- second";
/// An item that IS a fenced block, the shape of Conflab's dc doing 10.
const FENCED: &str = "```\ngit log -1\n```";
/// A one-line item, which prints exactly as it always did.
const ONE_LINE: &str = "held until the pair is rebuilt";

/// `cc`'s board: the list first, the fence after it, a one-line hold below.
fn seeded() -> tempfile::TempDir {
  let dir = tempfile::tempdir().expect("tempdir");
  let steps: [&[&str]; 5] = [
    &["init", "wbmultiline"],
    &[
      "wb", "register", "cc", "--name", "Control", "--role", "control",
    ],
    &["wb", "add", "doing", LISTED, "--node", "cc"],
    &["wb", "add", "doing", FENCED, "--node", "cc"],
    &["wb", "add", "hold", ONE_LINE, "--node", "cc"],
  ];
  for args in steps {
    let (text, code) = run(dir.path(), args);
    assert_eq!(code, 0, "{args:?}: {text}");
  }
  dir
}

fn board_view(dir: &Path) -> String {
  std::fs::read_to_string(dir.join("intent/whiteboard/cc/wip.md")).expect("the board view")
}

/// The headings `view` writes that a markdown parser does not read as headings.
fn unread_headings(view: &str) -> Vec<String> {
  use pulldown_cmark::{Event, Parser, Tag, TagEnd};
  let mut read = Vec::new();
  let mut current: Option<String> = None;
  for event in Parser::new(view) {
    match event {
      Event::Start(Tag::Heading { .. }) => current = Some(String::new()),
      Event::Text(t) | Event::Code(t) => {
        if let Some(heading) = current.as_mut() {
          heading.push_str(&t);
        }
      }
      Event::End(TagEnd::Heading(_)) => read.extend(current.take()),
      _ => {}
    }
  }
  view
    .lines()
    .filter_map(|l| l.strip_prefix("## ").or_else(|| l.strip_prefix("# ")))
    .filter(|written| !read.iter().any(|r| r == written))
    .map(str::to_string)
    .collect()
}

/// How deep the parser nests lists in `view`: 2 when a sub-bullet stays
/// inside its item, 1 when it reads as a sibling.
fn deepest_list(view: &str) -> usize {
  use pulldown_cmark::{Event, Parser, Tag, TagEnd};
  let (mut depth, mut deepest) = (0usize, 0usize);
  for event in Parser::new(view) {
    match event {
      Event::Start(Tag::List(_)) => {
        depth += 1;
        deepest = deepest.max(depth);
      }
      Event::End(TagEnd::List(_)) => depth = depth.saturating_sub(1),
      _ => {}
    }
  }
  deepest
}

#[test]
fn a_fenced_block_in_an_item_leaves_every_heading_of_the_board_a_heading() {
  let dir = seeded();
  let view = board_view(dir.path());
  let older = intentsvcs::views::board_before_0532(&view);
  assert!(
    !unread_headings(&older).is_empty(),
    "the instrument must see the fence swallow the board in the older shape, or a green below \
     proves nothing:\n{older}"
  );
  let unread = unread_headings(&view);
  assert!(
    unread.is_empty(),
    "every heading the board writes is a heading to the parser, unread: {unread:?}\n{view}"
  );
}

#[test]
fn a_sub_bullet_stays_inside_its_item() {
  let dir = seeded();
  let view = board_view(dir.path());
  assert_eq!(
    deepest_list(&intentsvcs::views::board_before_0532(&view)),
    1,
    "the instrument must read the older shape's sub-bullets as siblings"
  );
  assert_eq!(
    deepest_list(&view),
    2,
    "the sub-bullets are a list inside their item:\n{view}"
  );
}

#[test]
fn wb_show_and_wb_pickup_print_an_items_lines_under_it() {
  let dir = seeded();
  for verb in [
    &["wb", "show", "cc"][..],
    &["wb", "pickup", "--node", "cc"][..],
  ] {
    let (text, code) = run(dir.path(), verb);
    assert_eq!(code, 0, "{verb:?}: {text}");
    assert!(
      text.contains("  [doing] 1 two steps:\n    - first\n    - second\n"),
      "{verb:?} prints the item's lines under it:\n{text}"
    );
    assert!(
      text.contains(&format!("  [hold] 1 {ONE_LINE}\n")),
      "{verb:?} prints a one-line item exactly as it did:\n{text}"
    );
  }
}

/// Issue 0539: a thread write over a board an older Intent wrote names no lost
/// edit, and a hand edit in that board is still named. Before 0539 the write's
/// foreign-bytes check asked only about text the renderer owns, so `st new`
/// warned that an edit to the board was gone when nobody had made one.
#[test]
fn a_thread_write_over_an_older_board_warns_only_of_a_hand_edit() {
  const WARNING: &str = "overwrote bytes that were not the store's render";
  let write_over = |shape: &dyn Fn(&str) -> String| -> (String, String) {
    let dir = seeded();
    let view = dir.path().join("intent/whiteboard/cc/wip.md");
    let now = board_view(dir.path());
    std::fs::write(&view, shape(&now)).expect("write the board as it was left");
    let (text, code) = run(dir.path(), &["st", "new", "a thread"]);
    assert_eq!(code, 0, "st new: {text}");
    (text, board_view(dir.path()))
  };

  let (text, after) = write_over(&|now: &str| intentsvcs::views::board_before_0532(now));
  assert!(
    !text.contains(WARNING),
    "a board an older Intent wrote holds no edit to lose:\n{text}"
  );
  assert!(
    !after.contains("\n- first"),
    "the write re-rendered the board in today's shape:\n{after}"
  );

  let (text, _) = write_over(&|now: &str| {
    intentsvcs::views::board_before_0532(now).replacen("git log -1", "git log -2", 1)
  });
  assert!(
    text.contains(WARNING) && text.contains("intent/whiteboard/cc/wip.md"),
    "control: a hand edit in the older shape is still named:\n{text}"
  );
}
