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

/// The KNOWN GAP, asserted rather than hidden.
///
/// Authored prose passes through the renderer verbatim -- migration.md forbids
/// reflowing or "improving" it, and `objective` / `context` are markdown by
/// design (D22). But a formatter normalises markdown SYNTAX: prettier rewrites
/// `*emphasis*` to `_emphasis_`. So a canon whose prose uses `*` renders a view
/// the formatter then rewrites, and the skew check flags a file nobody edited.
///
/// This is a THIRD class, distinct from the two already ruled on:
///   - layout the renderer controls (column widths, blank runs, trailing
///     space) -- fixed at `finish()` and `kv()`;
///   - markup the renderer ADDS around data that carries its own delimiters --
///     vc's ruling: never wrap a possibly-markdown value in inline markup;
///   - THIS: markup the AUTHOR wrote, which the renderer must not touch and the
///     formatter will not leave alone.
///
/// It cannot be fixed in the renderer without rewriting authored prose, which
/// is the one thing the renderer is forbidden to do. The structural fix is to
/// stop the formatter running over tool-owned files at all -- one writer per
/// file, the same principle as D02 applied to writers rather than content --
/// and that is a repo-level decision, not cc's to take unilaterally.
///
/// The test asserts the gap EXISTS so it cannot close by accident and go
/// unnoticed. If it starts failing, the class has been fixed and this test
/// should be replaced by one asserting stability.
#[test]
fn authored_prose_emphasis_is_the_one_case_the_renderer_cannot_stabilise() {
  if std::process::Command::new("prettier")
    .arg("--version")
    .output()
    .is_err()
  {
    eprintln!("SKIPPED: prettier is not on PATH");
    return;
  }

  let fx = Fixture::new();
  let mut thread = sample_thread("ST0056");
  thread.objective = "One *major* release.".to_string();
  let canon = Canon {
    threads: vec![thread],
    issues: Vec::new(),
    sections: Vec::new(),
  };
  views::write_all(&fx.project(), &canon, &ctx()).expect("write");

  let before = fx.read("intent/st/ST0056/info.md");
  assert!(
    before.contains("*major*"),
    "precondition: the renderer passed the author's `*` through verbatim, as it must"
  );
  std::process::Command::new("prettier")
    .arg("--write")
    .arg(fx.path("intent/st/ST0056/info.md"))
    .output()
    .expect("run prettier");
  let after = fx.read("intent/st/ST0056/info.md");

  assert!(
    after.contains("_major_") && !after.contains("*major*"),
    "the formatter normalises the AUTHOR's emphasis marker. If this assertion fails, the class has been closed -- replace this test with a stability assertion rather than deleting it."
  );
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

// ---------------------------------------------------------------------------
// The generated view reproduces the AUTHORED vocabulary, not the wire one
// ---------------------------------------------------------------------------

/// **A generated view must write the token a human authored, and one status did
/// not** -- issue 0056.
///
/// `test_line` rendered the AT status with `enum_str`, ie the JSON canon's tag. For
/// three of the four values that is the same string an author writes; for `Na` the
/// tag is `n-a` and the authored form is `n/a`. **Measured across this estate:
/// every authored AT row in `acceptance.md` spells it `n/a` and none spells it
/// `n-a`** -- so
/// the next projection over any thread with a non-test AT would have rewritten each
/// of those rows into a spelling v2's own linter rejects at L1. Not a preference: a
/// migration hazard, in the direction that silently damages authored files.
///
/// **This test exists because the fix was invisible to the whole suite.** Reverting
/// `views.rs` to `enum_str` left 72 legs green and cargo exit 0 -- measured, with
/// the run's own leg count checked first, because the same check run against a
/// suite that had not executed printed the identical "nothing red" verdict. A fix
/// whose green is identical either side of it has been performed rather than
/// measured.
///
/// **IT COMPARED THE RENDERER TO `display()` UNTIL vc CAUGHT IT, AND THAT IS THE
/// FUNCTION DEFINING THE RENDERER'S SPELLING.** The assertion was
/// `view.contains(&format!("status: {}", status.display()))`, so changing
/// `display()` to return `n@a` left it green: the only way to fail it was for
/// `views.rs` to bypass `display()` altogether, which is a plumbing failure, not
/// a spelling one. A self-consistency check wearing a parity name.
///
/// The reasoning that put it there is preserved because it is the instructive
/// part: "pinning four strings here would be a fifth copy of it." **`display()`
/// is not a copy of the property, it is the definition** -- and the property is
/// not "the view speaks the display vocabulary" but "the view speaks the
/// vocabulary a HUMAN AUTHORED". The only witness for that is authored bytes v3
/// did not produce.
///
/// So the four spellings below are a TRANSCRIPTION of an external authority --
/// `acceptance.md` files v2 wrote -- in the same posture as the `RATIFIED_*`
/// consts that transcribe data-model.md. Their agreeing with `display()` is the
/// claim, which means it has to be asserted rather than assumed.
///
/// **All four, and the three that coincide are the reason.** `to-write`, `red`
/// and `green` are byte-identical between the wire tag and the authored form;
/// only `n-a` against `n/a` diverges. That coincidence is the entire hiding
/// mechanism -- it is why echoing the wrong source was correct three times and
/// wrong once -- so a fixture carrying only the divergent variant would prove the
/// fix and stop proving the other three still agree.
#[test]
fn the_view_writes_every_at_status_in_the_authored_spelling() {
  use intentsvcs::model::{AtStatus, enum_str};

  // Transcribed from authored `acceptance.md` in this estate: every authored AT
  // row spells `n/a` and none spells `n-a`.
  let authored_spellings = [
    (AtStatus::ToWrite, "to-write"),
    (AtStatus::Red, "red"),
    (AtStatus::Green, "green"),
    (AtStatus::Na, "n/a"),
  ];

  for (status, authored) in authored_spellings {
    let fixture = Fixture::new();
    let mut thread = sample_thread("ST0056");
    for test in thread.tests.iter_mut() {
      test.status = status;
    }
    let canon = Canon {
      threads: vec![thread],
      issues: Vec::new(),
      sections: Vec::new(),
    };
    // Rendered directly rather than through the projection: the claim is about
    // the RENDERER's vocabulary, and reaching it through `write_all` would make a
    // plumbing failure look like a spelling one.
    let view = views::render_all(&fixture.project(), &canon, &ctx())
      .into_iter()
      .find(|v| v.path.to_string_lossy().ends_with("acceptance.md"))
      .expect("the acceptance view is rendered")
      .content;

    // The load-bearing one: the LITERAL a human wrote, not the function the
    // renderer reads.
    assert!(
      view.contains(&format!("status: {authored}")),
      "the view must write `status: {authored}` -- the spelling transcribed from authored files -- and it wrote none of them:\n{view}"
    );
    // The two enumerators agreeing IS the claim, so it is asserted. This is what
    // reds if `display()` drifts to a spelling no author uses, which the previous
    // form could not see.
    assert_eq!(
      status.display(),
      authored,
      "`display()` and the authored corpus disagree about {status:?}. One of them moved: if the corpus did, retranscribe the literal above and say what \
       changed the authored vocabulary; if `display()` did, that is a view about to rewrite authored rows"
    );
    // The wire form must never reach a view. **It only bites on `Na`, and saying
    // so is the point** -- for the other three the wire tag and the authored form
    // are the same string, so this assertion is vacuous on three quarters of the
    // population and a reader should not mistake it for four checks.
    let wire = enum_str(&status);
    if wire != authored {
      assert!(
        !view.contains(&format!("status: {wire}")),
        "`{wire}` is the WIRE form and must never reach a generated view: an authored file rewritten to it fails v2's linter at L1, and every authored row in this \
         estate carries `{authored}`"
      );
    }
  }
}
