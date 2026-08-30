//! The flat aligned column: `AT-17.11` covering `AC-17.11`.
//!
//! **THIS MODULE COMPUTES THE PICTURE AND DRAWS NOTHING**, which is the whole
//! reason it can be asserted at all. `AC-17.11` says the alignment IS the
//! design, so it is asserted rather than eyeballed -- and a property asserted
//! against a rendered terminal buffer is a property asserted against whatever
//! the renderer happened to do. Here the plan is DATA: every line is a string
//! this module produced, at a width it was handed, and [`super::draw`] is a
//! printer with no opinions. The buffer test one level up then checks that the
//! printer did not lie, which is a different question and a much smaller one.
//!
//! **IT CARRIES NO `ratatui`**, for the reason [`super::mode`],
//! [`super::terminal`] and [`super::focus`] carry no terminal: the realiser is
//! what these invariants CHECK. A layout that could only be exercised through
//! the widget library would be tested by the thing it exists to constrain.
//!
//! # The two columns, and why the name column is capped
//!
//! `AC-17.11` demands the property be measured *over a form whose longest name
//! and longest value both exceed the viewport*. That requirement is what forces
//! the design: if the name column simply took the longest name, a name wider
//! than the terminal would leave zero columns for values, and "names align in
//! one column and values align in another" would be vacuously true of a screen
//! with no values on it. So the name column is capped at whatever leaves
//! [`MIN_VALUE_COLS`] for values, and names truncate too.
//!
//! # Truncated visibly, never wrapped
//!
//! A wrapped value breaks the one guarantee the layout makes, because the
//! second line's text would start at column zero where a NAME belongs. So
//! [`plan`] emits exactly one line per row -- asserted, not intended -- and an
//! over-long value loses its tail to [`TRUNCATED`] rather than its shape to a
//! second row.
//!
//! **TRUNCATION COUNTS CHARACTERS, NEVER BYTES.** This estate has already paid
//! for the other choice once, in a list renderer that cut multi-byte characters
//! in half. `AC-17.11`'s own subject makes it likely rather than theoretical:
//! criterion prose is the longest value in the model and it is full of typography.

/// Columns between the name column and the value column. Two rather than one
/// so a name truncated to its full width still reads as separate from the value
/// beside it, and rather than three because the gap is dead space on every row.
pub const GAP: usize = 2;

/// The value column never shrinks below this, however long the longest name.
/// **A value column of zero would make the alignment property vacuous** rather
/// than false, which is the failure this constant exists to prevent.
pub const MIN_VALUE_COLS: usize = 8;

/// Marks a value that lost its tail. ASCII and one column wide: this estate
/// bans decorative non-ASCII, and a multi-column marker would have to be
/// measured before it could be subtracted, which is the bug it would be
/// guarding against.
pub const TRUNCATED: char = '>';

/// One line of the form: what `AC-17.11` calls `{title, value, type}`.
///
/// **`kind` IS CARRIED AND NOT PRINTED, AND THAT IS A READING OF THE CRITERION
/// RATHER THAN AN OMISSION.** The row model is the three the criterion names;
/// the FORMATTING guarantee it states is two aligned columns, and it forbids
/// decoration in the same sentence. A third printed column would be a third
/// alignment promise the criterion never made. `kind` drives behaviour instead
/// -- which rows hand off to `$EDITOR`, which descend -- and that is where the
/// realiser reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
  pub title: String,
  pub value: String,
  pub kind: String,
}

impl Row {
  pub fn new(title: impl Into<String>, value: impl Into<String>, kind: impl Into<String>) -> Self {
    Self {
      title: title.into(),
      value: value.into(),
      kind: kind.into(),
    }
  }
}

/// A rendered form: one string per row, plus the foot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
  /// Where every name starts. Zero, always -- named rather than assumed so the
  /// assertion reads the same shape for both columns.
  pub name_col: usize,
  /// Where every value starts.
  pub value_col: usize,
  /// The width every line was planned against.
  pub width: usize,
  /// Exactly one line per input row, in input order.
  pub rows: Vec<String>,
  /// The single rule, directly above the modeline.
  pub rule: String,
  /// The modeline: the bottom line of the screen.
  pub modeline: String,
}

impl Plan {
  /// The rows visible in a window `height` tall, starting at `first`.
  ///
  /// **THE WINDOW IS A LAYOUT QUESTION AND THE SCROLL POSITION IS NOT.** Where
  /// the operator has scrolled to is application state that changes on a
  /// keystroke; which rows that position makes visible is arithmetic, and
  /// arithmetic is the thing worth asserting. Out-of-range `first` yields an
  /// empty window rather than panicking, because a form that shrinks under a
  /// held scroll position is an ordinary event and not a bug.
  pub fn visible(&self, first: usize, height: usize) -> &[String] {
    let start = first.min(self.rows.len());
    let end = start.saturating_add(height).min(self.rows.len());
    &self.rows[start..end]
  }

  /// Every line the plan puts on screen, top to bottom. The rule and the
  /// modeline are the last two, in that order.
  pub fn lines(&self) -> Vec<&str> {
    self
      .rows
      .iter()
      .map(String::as_str)
      .chain([self.rule.as_str(), self.modeline.as_str()])
      .collect()
  }
}

/// Cut `s` to `w` columns, marking the cut when one happens.
///
/// Counts `chars`, so a cut never lands inside a multi-byte character.
fn clip(s: &str, w: usize) -> String {
  if w == 0 {
    return String::new();
  }
  if s.chars().count() <= w {
    return s.to_string();
  }
  let mut out: String = s.chars().take(w - 1).collect();
  out.push(TRUNCATED);
  out
}

/// Lay `rows` out at `width`, with `mode` on the modeline.
///
/// The returned [`Plan`] holds one line per row and never a line wider than
/// `width`.
pub fn plan(rows: &[Row], width: usize, mode: &str) -> Plan {
  let longest = rows
    .iter()
    .map(|r| r.title.chars().count())
    .max()
    .unwrap_or(0);
  // The cap, not the longest name, is what keeps the value column alive.
  let room_for_names = width.saturating_sub(GAP + MIN_VALUE_COLS);
  let name_width = longest.min(room_for_names);
  let value_col = if width == 0 {
    0
  } else {
    (name_width + GAP).min(width)
  };
  let value_width = width.saturating_sub(value_col);

  let lines = rows
    .iter()
    .map(|r| {
      let name = clip(&r.title, name_width);
      let pad = name_width - name.chars().count();
      let value = clip(&r.value, value_width);
      // Trailing space is decoration; the line ends where its content does.
      let mut line = String::with_capacity(width);
      line.push_str(&name);
      if !value.is_empty() {
        line.extend(std::iter::repeat_n(' ', pad + GAP));
        line.push_str(&value);
      }
      line
    })
    .collect();

  Plan {
    name_col: 0,
    value_col,
    width,
    rows: lines,
    rule: "-".repeat(width),
    modeline: clip(mode, width),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use intentsvcs::form::Loaded;

  /// A name and a value that both exceed the viewport the tests use.
  ///
  /// **THIS FIXTURE IS THE CRITERION'S OWN POSITIVE CONTROL**, which is why it
  /// is a named constant rather than an inline literal: `AC-17.11` says the
  /// property is measured over a form whose longest name AND longest value both
  /// exceed the viewport, and `the_fixture_is_the_hard_case_the_criterion_names`
  /// below asserts that it does. Shrink the fixture and that test goes red
  /// before the alignment tests go quietly vacuous.
  /// **30 RATHER THAN 40, AND THE CONTROL BELOW IS WHY.** The first version of
  /// this fixture used 40 against a longest name of 34, so the name did NOT
  /// exceed the viewport, the name column never hit its cap, and every
  /// alignment assertion here was quietly measuring the easy case. The control
  /// caught it on its first run. At 30 both the longest name (34) and the
  /// longest value (76) exceed the viewport, and the value column lands exactly
  /// on `MIN_VALUE_COLS`, which is the boundary the cap exists to hold.
  const NARROW: usize = 30;

  fn hard_rows() -> Vec<Row> {
    vec![
      Row::new("title", "ST0056: Intent v3", "text"),
      Row::new("status", "WIP", "select"),
      Row::new(
        "parity/tools/conservation_check.sh",
        "a value long enough that it cannot fit beside a name that does not fit either",
        "button",
      ),
      Row::new("objective", "", "prose"),
    ]
  }

  #[test]
  fn the_fixture_is_the_hard_case_the_criterion_names() {
    let rows = hard_rows();
    let longest_name = rows.iter().map(|r| r.title.chars().count()).max().unwrap();
    let longest_value = rows.iter().map(|r| r.value.chars().count()).max().unwrap();
    assert!(
      longest_name > NARROW,
      "the fixture's longest name is {longest_name} at a viewport of {NARROW}, so it does not \
       exceed the viewport and every alignment assertion below is measuring the easy case"
    );
    assert!(
      longest_value > NARROW,
      "the fixture's longest value is {longest_value} at a viewport of {NARROW}, so nothing here \
       exercises truncation"
    );
  }

  /// The criterion's whole content, asserted against the STRINGS rather than
  /// against the arithmetic that made them -- a check that recomputed
  /// `value_col` and compared it to itself would pass for any layout at all.
  #[test]
  fn every_name_starts_at_one_column_and_every_value_at_another() {
    let rows = hard_rows();
    let p = plan(&rows, NARROW, "NORMAL");
    let mut examined = 0usize;
    for (i, (row, line)) in rows.iter().zip(p.rows.iter()).enumerate() {
      let chars: Vec<char> = line.chars().collect();
      assert!(
        chars[p.name_col] != ' ',
        "row {i} does not start its name at column 0: {line:?}"
      );
      if row.value.is_empty() {
        assert_eq!(
          chars.len(),
          row.title.chars().count().min(NARROW),
          "row {i} has no value, so its line must end where its name does: {line:?}"
        );
        continue;
      }
      // **A ROW WITH A VALUE MUST REACH THE VALUE COLUMN. This is an
      // ASSERTION and was once a `continue`**, which made the whole test
      // vacuous: a layout that put values immediately after each name produced
      // lines too short to reach `value_col`, every row was skipped, and the
      // check passed green over nothing. Measured -- mutating the padding to
      // zero left all seven tests passing.
      assert!(
        chars.len() > p.value_col,
        "row {i} carries a value but its line is {} columns, which does not reach the value \
         column at {}. Values must all START at that column: {line:?}",
        chars.len(),
        p.value_col
      );
      assert!(
        chars[p.value_col] != ' ',
        "row {i} does not start its value at column {}: {line:?}",
        p.value_col
      );
      for (x, c) in chars
        .iter()
        .enumerate()
        .take(p.value_col)
        .skip(p.value_col - GAP)
      {
        assert_eq!(
          *c, ' ',
          "row {i} has {c:?} at column {x}, inside the gap: {line:?}"
        );
      }
      examined += 1;
    }
    // **THE CONTROL THAT WAS MISSING.** `the_fixture_is_the_hard_case` proves
    // the FIXTURE is hard; it says nothing about whether the loop above looked
    // at anything. A count is the only thing that distinguishes "every row
    // aligned" from "no row was examined".
    assert_eq!(
      examined,
      rows.iter().filter(|r| !r.value.is_empty()).count(),
      "the alignment assertion examined {examined} rows, which is not every row that carries a \
       value. A loop that skips its subject is green for free"
    );
    assert!(
      examined > 0,
      "no row carried a value, so this test asserted nothing at all"
    );
  }

  #[test]
  fn a_value_that_does_not_fit_is_cut_and_marked_rather_than_wrapped() {
    let rows = hard_rows();
    let p = plan(&rows, NARROW, "NORMAL");
    assert_eq!(
      p.rows.len(),
      rows.len(),
      "the layout emitted a different number of lines than it was given rows, which is what \
       wrapping looks like from the outside"
    );
    let long = &p.rows[2];
    assert!(
      long.ends_with(TRUNCATED),
      "a value too long for its column must say so; it ended {long:?}"
    );
  }

  #[test]
  fn no_line_is_ever_wider_than_the_viewport() {
    for width in [0usize, 1, 2, 9, 12, NARROW, 200] {
      let p = plan(
        &hard_rows(),
        width,
        "NORMAL: a modeline longer than some of these widths",
      );
      for line in p.lines() {
        assert!(
          line.chars().count() <= width,
          "at width {width} a line was {} columns: {line:?}",
          line.chars().count()
        );
      }
    }
  }

  /// The estate has cut a multi-byte character in half before. Driven on a
  /// planted value rather than on the real forms, because whether the corpus
  /// happens to contain one today is not something this property should depend
  /// on.
  #[test]
  fn a_cut_never_lands_inside_a_multi_byte_character() {
    let value = "\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}\u{2014}";
    let rows = vec![Row::new("f", value, "text")];
    for width in 4..14 {
      let p = plan(&rows, width, "M");
      // Reaching this at all means no slice panicked mid-character.
      let line = &p.rows[0];
      assert!(line.is_char_boundary(line.len()));
      assert!(line.chars().count() <= width);
    }
  }

  #[test]
  fn the_foot_is_one_rule_and_one_modeline_with_the_modeline_last() {
    let p = plan(&hard_rows(), NARROW, "NORMAL");
    let lines = p.lines();
    assert_eq!(
      lines.len(),
      p.rows.len() + 2,
      "the foot is exactly two lines"
    );
    assert_eq!(
      lines[lines.len() - 2],
      "-".repeat(NARROW),
      "the rule is one unbroken line directly above the modeline"
    );
    assert_eq!(
      lines[lines.len() - 1],
      "NORMAL",
      "the modeline is the bottom line"
    );
    assert!(
      !p.rows.iter().any(|l| l.contains("---")),
      "a rule appeared among the rows; `AC-17.11` allows exactly one"
    );
  }

  /// **THE PROPERTY OVER THE REAL CORPUS, not only over a fixture built to
  /// exhibit it.** A fixture proves the layout can align; the shipped forms
  /// prove it does.
  #[test]
  fn every_shipped_form_lays_out_with_its_columns_aligned() {
    let loaded = Loaded::load().expect("the shipped form declaration must load");
    assert!(
      !loaded.forms().is_empty(),
      "no forms to lay out, so this test asserts nothing"
    );
    for form in loaded.forms() {
      let rows: Vec<Row> = form
        .fields
        .iter()
        .map(|f| Row::new(f.label.clone(), format!("<{}>", f.name), f.widget.clone()))
        .collect();
      assert!(!rows.is_empty(), "a shipped form declares no fields");
      let p = plan(&rows, NARROW, "NORMAL");
      assert_eq!(p.rows.len(), rows.len());
      for line in &p.rows {
        let chars: Vec<char> = line.chars().collect();
        if chars.len() > p.value_col {
          assert!(
            chars[p.value_col] != ' ',
            "a shipped form's value does not start at column {}: {line:?}",
            p.value_col
          );
        }
      }
    }
  }
}
