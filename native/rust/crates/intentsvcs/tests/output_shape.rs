//! **THE SHAPE OF OUTPUT IS ONE CONCERN WITH ONE DOOR.**
//!
//! Before `output.rs` it was decided four ways across eleven flags -- `--width`
//! on two rows, `--markdown` on one, `--json` on three, `--format` on two -- so
//! `issues list` was width-aware and `wp list` was not, for no reason anybody
//! had decided.
//!
//! **THE ARM WORTH READING FIRST IS `a_target_width_is_a_maximum_and_not_only_a_minimum`.**
//! v2 and v3 both carried the same rule in near-identical words -- v2's
//! `render_table` says *content-fit is the floor, so nothing is ever truncated*
//! and `views.rs` said *a narrow terminal never truncates, it just stops
//! padding*. Neither states the consequence: ONE oversized cell sets the width
//! of EVERY row. Measured 2026-08-25, `issues list` rendered 312 columns into an
//! 80-column terminal because a single title ran to 287 characters.

use intentsvcs::output::{Format, Output, OutputError};
use intentsvcs::views::{self, TableMode};

const H: &[&str] = &["ID", "Title"];

fn rows(title: &str) -> Vec<Vec<String>> {
  vec![vec!["0063".to_string(), title.to_string()]]
}

fn widest(s: &str) -> usize {
  s.lines().map(|l| l.chars().count()).max().unwrap_or(0)
}

/// **THE DEFECT, AND IT IS THE ONE BOTH IMPLEMENTATIONS SHARED.**
#[test]
fn a_target_width_is_a_maximum_and_not_only_a_minimum() {
  let long = "x".repeat(300);
  let out = views::table(H, &rows(&long), TableMode::Terminal { fill: 80 });
  assert_eq!(
    widest(&out),
    80,
    "a 300-character cell must not set the width of the table"
  );
  // And the floor half still holds: a narrow table still expands to fill.
  let out = views::table(H, &rows("hi"), TableMode::Terminal { fill: 80 });
  assert_eq!(widest(&out), 80, "a narrow table still fills the width");
}

/// Clipping is by CHARACTER. A byte slice through a multi-byte character
/// produces broken output, and the ellipsis is itself one character and three
/// bytes -- exactly the trap a byte-based truncate falls into.
#[test]
fn clipping_counts_characters_and_not_bytes() {
  let wide = "é".repeat(300);
  let out = views::table(H, &rows(&wide), TableMode::Terminal { fill: 40 });
  assert_eq!(widest(&out), 40, "chars, not bytes");
  assert!(
    out.contains('\u{2026}'),
    "a clipped cell says it was clipped"
  );
  assert!(
    out.len() > 40 * out.lines().count(),
    "and the byte length exceeds the char length, which is the whole hazard"
  );
}

/// **HEADERS ARE THE FLOOR.** A clipped header makes its column unidentifiable,
/// which is worse than a wide table: the row below it becomes unreadable rather
/// than merely shortened.
#[test]
fn a_header_is_never_clipped_and_an_impossible_width_overflows_honestly() {
  let out = views::table(H, &rows("some title"), TableMode::Terminal { fill: 4 });
  let header = out.lines().next().expect("a header");
  assert!(
    header.contains("ID") && header.contains("Title"),
    "both headers survive an impossible width: {header}"
  );
}

/// **A PERSISTED FILE MUST NOT DEPEND ON THE WINDOW THAT GENERATED IT.** Markdown
/// ignores width entirely -- otherwise every regeneration at a different window
/// size rewrites bytes and the skew check reports files nobody touched.
#[test]
fn markdown_ignores_width_in_both_directions() {
  let long = "x".repeat(300);
  let narrow = views::table(H, &rows(&long), TableMode::Markdown);
  assert!(
    widest(&narrow) > 300,
    "markdown is content-fit, never clipped"
  );
  assert_eq!(Output::new(Format::Markdown, 80).width(), 0);
  assert_eq!(Output::new(Format::Json, 80).width(), 0);
  assert_eq!(Output::new(Format::Terminal, 80).width(), 80);
}

/// The v2 spellings are parity obligations and resolve to the same place.
#[test]
fn the_v2_alias_flags_resolve_into_the_one_vocabulary() {
  let t = |f, w, j, m| Output::resolve(f, w, j, m, 100).map(|o| o.format());
  assert_eq!(t(None, None, true, false), Ok(Format::Json));
  assert_eq!(t(None, None, false, true), Ok(Format::Markdown));
  assert_eq!(t(Some("json"), None, false, false), Ok(Format::Json));
  assert_eq!(
    t(Some("markdown"), None, false, false),
    Ok(Format::Markdown)
  );
  assert_eq!(t(Some("md"), None, false, false), Ok(Format::Markdown));
  assert_eq!(t(Some("text"), None, false, false), Ok(Format::Terminal));
  assert_eq!(t(Some("JSON"), None, false, false), Ok(Format::Json));
  assert_eq!(t(None, None, false, false), Ok(Format::Terminal));
  // An AGREEING pair is not a conflict.
  assert_eq!(t(Some("json"), None, true, false), Ok(Format::Json));
}

/// **A CONFLICT REFUSES RATHER THAN PICKING A WINNER.** A silent precedence
/// would make the surface unlearnable in the one case where the caller has
/// already shown they believe two things.
#[test]
fn two_flags_asking_for_different_formats_refuse() {
  let e = Output::resolve(Some("md"), None, true, false, 100).expect_err("md and json disagree");
  assert!(
    matches!(e, OutputError::ConflictingFormat { .. }),
    "got {e:?}"
  );
  let e = Output::resolve(None, None, true, true, 100).expect_err("json and markdown disagree");
  assert!(
    matches!(e, OutputError::ConflictingFormat { .. }),
    "got {e:?}"
  );
}

/// **`0` MEANS THE DEFAULT, WHICH IS THE DECLARED CONTRACT.** `st list --width`'s
/// surface row has said *0 or absent means terminal width* since it shipped, and
/// refusing it would put the only row documenting the flag in disagreement with
/// the only implementation of it.
#[test]
fn a_zero_width_means_the_default_and_a_non_number_refuses() {
  let w = |raw| Output::resolve(None, raw, false, false, 100).map(|o| o.width());
  assert_eq!(w(Some("0")), Ok(100));
  assert_eq!(w(None), Ok(100));
  assert_eq!(w(Some("80")), Ok(80));
  assert_eq!(w(Some(" 80 ")), Ok(80));
  assert!(matches!(w(Some("nope")), Err(OutputError::BadWidth { .. })));
  assert!(matches!(w(Some("-1")), Err(OutputError::BadWidth { .. })));
}

/// An unknown format is refused by name, and the refusal names the set -- a
/// remedy that cannot be acted on costs a search first.
#[test]
fn an_unknown_format_is_refused_by_name() {
  let e = Output::resolve(Some("yaml"), None, false, false, 100).expect_err("yaml is not a format");
  assert!(matches!(e, OutputError::UnknownFormat { .. }));
  let said = format!("{e}");
  for spelling in Format::SPELLINGS {
    assert!(
      said.contains(spelling),
      "the refusal names {spelling}: {said}"
    );
  }
}

/// **JSON IS REFUSED BY THE TABLE DOOR RATHER THAN RENDERED BADLY.** A
/// list-of-lists is a shape nobody asked for wearing the name of one they did.
#[test]
fn the_table_door_has_no_json_projection() {
  let o = Output::new(Format::Json, 80);
  assert_eq!(o.table(H, &rows("t")), None);
  assert!(
    Output::new(Format::Terminal, 80)
      .table(H, &rows("t"))
      .is_some()
  );
  assert!(
    Output::new(Format::Markdown, 0)
      .table(H, &rows("t"))
      .is_some()
  );
}
