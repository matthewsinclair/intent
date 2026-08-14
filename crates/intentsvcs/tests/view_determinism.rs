//! AT-03.2 / AC-03.2: view rendering is deterministic and idempotent -- the
//! same model renders the same bytes, twice, on both platforms.
//!
//! Two halves, and the second is the one that generalises.
//!
//! **Behavioural**: render twice, render from a separately-built identical
//! model, and render under a different filesystem root. All byte-identical.
//!
//! **Structural**: the renderer cannot reach a clock, an environment variable,
//! a hostname or a user -- asserted by scanning `views.rs` itself. This is the
//! vc law (2026-08-14) rather than a preference, and it is checked mechanically
//! because a behavioural test only proves the views that exist today behave.
//! v2's defect was not one bad view; it was that the blessed generated-banner
//! pattern embedded a render date, so every view nobody had written yet
//! inherited it. A guard scoped to what is already clean certifies the status
//! quo, so this one is scoped to the capability.

mod common;

use common::{Fixture, ctx, sample_thread};
use intentsvcs::ingest::Canon;
use intentsvcs::views;

fn canon() -> Canon {
  Canon {
    threads: vec![sample_thread("ST0056"), sample_thread("ST0043")],
    issues: vec![common::sample_issue(21)],
    sections: Vec::new(),
  }
}

#[test]
fn rendering_twice_yields_identical_bytes() {
  let fx = Fixture::new();
  let project = fx.project();
  let canon = canon();

  let first = views::render_all(&project, &canon, &ctx());
  let second = views::render_all(&project, &canon, &ctx());
  assert_eq!(first, second, "the same model renders the same bytes");
  assert!(!first.is_empty(), "the fixture actually rendered something");
}

#[test]
fn an_independently_built_identical_model_renders_identically() {
  let fx = Fixture::new();
  let project = fx.project();

  let a = views::render_all(&project, &canon(), &ctx());
  let b = views::render_all(&project, &canon(), &ctx());
  assert_eq!(
    a.iter().map(|v| v.content.clone()).collect::<Vec<_>>(),
    b.iter().map(|v| v.content.clone()).collect::<Vec<_>>(),
  );
}

/// Two projects at different filesystem paths render identical CONTENT. A
/// renderer that leaked an absolute path into a view would pass every
/// same-directory determinism check and fail this one.
#[test]
fn content_does_not_depend_on_where_the_project_lives() {
  let one = Fixture::new();
  let two = Fixture::new();
  assert_ne!(one.root(), two.root(), "precondition: different roots");

  let a: Vec<String> = views::render_all(&one.project(), &canon(), &ctx())
    .into_iter()
    .map(|v| v.content)
    .collect();
  let b: Vec<String> = views::render_all(&two.project(), &canon(), &ctx())
    .into_iter()
    .map(|v| v.content)
    .collect();
  assert_eq!(
    a, b,
    "view content is a function of the model, not the path"
  );
}

#[test]
fn writing_the_views_twice_leaves_the_bytes_unchanged() {
  let fx = Fixture::new();
  let project = fx.project();
  let canon = canon();

  views::write_all(&project, &canon, &ctx()).expect("first write");
  let after_first = fx.read("intent/st/steel_threads.md");
  views::write_all(&project, &canon, &ctx()).expect("second write");
  assert_eq!(
    fx.read("intent/st/steel_threads.md"),
    after_first,
    "writing is idempotent"
  );
}

/// Every rendered table is already in the formatter's canonical padded form.
///
/// Structural half of the formatter-stability contract, and it runs
/// everywhere. A table is prettier-canonical when every row of a block has the
/// same cell widths as its separator row -- which is precisely the property
/// that stops the pre-commit formatter from rewriting what we just wrote.
#[test]
fn every_rendered_table_is_in_canonical_padded_form() {
  let fx = Fixture::new();
  let mut checked = 0;
  for view in views::render_all(&fx.project(), &canon(), &ctx()) {
    for block in table_blocks(&view.content) {
      let widths: Vec<Vec<usize>> = block.iter().map(|l| cell_widths(l)).collect();
      assert!(
        widths.windows(2).all(|w| w[0] == w[1]),
        "{}: table rows disagree on column widths, so prettier will rewrite them:\n{}",
        view.path.display(),
        block.join("\n")
      );
      checked += 1;
    }
  }
  assert!(checked > 0, "the fixture rendered at least one table");
}

/// The behavioural half: the real formatter, over the real output.
///
/// Skipped when prettier is absent, and the skip is LOUD -- but the structural
/// test above runs unconditionally, so a machine without prettier still checks
/// the property, just by the weaker route. A test that quietly passed on a
/// missing binary would be the vacuous green this project keeps paying for.
#[test]
fn the_formatter_leaves_rendered_views_unchanged() {
  if std::process::Command::new("prettier")
    .arg("--version")
    .output()
    .is_err()
  {
    eprintln!(
      "SKIPPED the prettier half of AT-03.2: prettier is not on PATH. The structural check (every_rendered_table_is_in_canonical_padded_form) still ran."
    );
    return;
  }

  let fx = Fixture::new();
  let project = fx.project();
  views::write_all(&project, &canon(), &ctx()).expect("write views");

  let targets = [
    "intent/st/steel_threads.md",
    "intent/todo.md",
    "intent/st/ST0056/info.md",
    "intent/st/ST0056/acceptance.md",
  ];
  for rel in targets {
    let before = fx.read(rel);
    let status = std::process::Command::new("prettier")
      .arg("--write")
      .arg(fx.path(rel))
      .output()
      .expect("run prettier");
    assert!(
      status.status.success(),
      "prettier failed on {rel}: {}",
      String::from_utf8_lossy(&status.stderr)
    );
    let after = fx.read(rel);
    assert_eq!(
      before, after,
      "{rel} is not formatter-stable: the pre-commit hook would rewrite what the renderer just wrote, so every regeneration would oscillate and the skew check would flag files nobody edited"
    );
  }
}

/// Contiguous runs of table lines.
fn table_blocks(text: &str) -> Vec<Vec<String>> {
  let mut blocks = Vec::new();
  let mut current: Vec<String> = Vec::new();
  for line in text.lines() {
    if line.starts_with('|') {
      current.push(line.to_string());
    } else if !current.is_empty() {
      blocks.push(std::mem::take(&mut current));
    }
  }
  if !current.is_empty() {
    blocks.push(current);
  }
  blocks
}

/// The width of each cell in a table line, padding included.
fn cell_widths(line: &str) -> Vec<usize> {
  line
    .trim_end()
    .split('|')
    .skip(1)
    .take_while(|_| true)
    .map(|c| c.chars().count())
    .collect::<Vec<_>>()
    .split_last()
    .map(|(_, rest)| rest.to_vec())
    .unwrap_or_default()
}

/// The law: the renderer has no clock, and no other ambient input either.
///
/// Scans the renderer's source for the APIs that could reach one. Comment
/// lines are stripped first so this file's own prose -- and views.rs's, which
/// explains at length why it must not read a clock -- cannot trip it.
#[test]
fn the_renderer_cannot_reach_a_clock_or_the_environment() {
  let source =
    std::fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/views.rs"))
      .expect("read views.rs");

  let code: String = source
    .lines()
    .filter(|l| {
      let t = l.trim_start();
      !t.starts_with("//") && !t.starts_with("///") && !t.starts_with("//!")
    })
    .collect::<Vec<_>>()
    .join("\n");

  // Every way a renderer could become non-deterministic. `now` covers
  // OffsetDateTime::now_utc, SystemTime::now and Instant::now in one needle.
  const BANNED: &[&str] = &[
    "now(",
    "SystemTime",
    "OffsetDateTime",
    "Instant",
    "std::env",
    "env::var",
    "env!(",
    "hostname",
    "current_dir",
    "random",
    "HashMap",
    "HashSet",
  ];

  let hits: Vec<&str> = BANNED
    .iter()
    .copied()
    .filter(|needle| code.contains(needle))
    .collect();
  assert!(
    hits.is_empty(),
    "views.rs reached for ambient input {hits:?} -- a generated view must be a function of the model and the tool version alone (vc law, 2026-08-14). HashMap/HashSet are banned alongside the clock because their iteration order is randomised per process, which is the same defect wearing a different hat."
  );
}
