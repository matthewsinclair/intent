//! The printer: `AT-17.11` covering `AC-17.11`, second half.
//!
//! **THIS MODULE HAS NO OPINIONS AND THAT IS ITS ENTIRE SPECIFICATION.**
//! [`super::layout`] decides where every character goes; this puts those
//! characters on a surface. The split is not tidiness -- it is what makes
//! `AC-17.11` assertable. *The alignment IS the design, so it is asserted
//! rather than eyeballed*, and a property asserted against a rendered buffer is
//! really a property asserted against whatever the widget library chose to do.
//! So the whole screen is composed on the plan, as data, and the only question
//! left here is the small one: **did the printer print the screen it was
//! given?**
//!
//! The function below is four lines for that reason. Every decision about
//! where the chrome sits, what clips, and what a short viewport keeps lives in
//! [`super::layout::Screen::compose`], where it can be asserted without a
//! terminal.
//!
//! # No borders, no decoration
//!
//! There is no `Block`, no border, no title bar and no padding widget here.
//! `tui-design.md` §2 allows exactly two rules and calls them *the only
//! chrome*; the cheapest way to keep it that way is to never reach for a widget
//! that draws one. This module writes strings at coordinates and nothing else.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

use super::layout::Screen;

/// Paint `screen` into `area`, scrolled so body row `first` is at the top of
/// the body.
pub fn render(screen: &Screen, first: usize, area: Rect, buf: &mut Buffer) {
  if area.height == 0 || area.width == 0 {
    return;
  }
  for (i, line) in screen
    .compose(first, area.height as usize)
    .iter()
    .enumerate()
  {
    buf.set_string(area.x, area.y + i as u16, line, Style::default());
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tui::layout::{self, CHROME, RULE, Row, Screen, plan};
  use ratatui::Terminal;
  use ratatui::backend::TestBackend;

  const W: u16 = 30;
  const H: u16 = 12;

  fn rows() -> Vec<Row> {
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

  fn screen() -> Screen {
    Screen {
      detail: None,
      app: "ST0056   Add a Rust-based CLI".into(),
      body: plan(&rows(), W as usize),
      status: "NORMAL   title   text   1/4".into(),
      command: "cmd: (none)".into(),
      info: "What this thread is called.".into(),
    }
  }

  /// The same screen with the selected row expanded. **FOUR detail rows against
  /// a body that can afford them**, so the split actually happens rather than
  /// degrading to the unsplit case this is contrasted with.
  fn split_screen() -> Screen {
    let detail = vec![
      Row::new("kind", "test", "text"),
      Row::new("state", "computed", "text"),
      Row::new("evidence", "layout.rs", "text"),
      Row::new("text", "A CHECKER VERIFIES MEMBERSHIP", "prose"),
    ];
    Screen {
      detail: Some(plan(&detail, W as usize)),
      ..screen()
    }
  }

  /// Draw through a real `ratatui` terminal and read the cells back.
  fn painted(s: &Screen, first: usize, h: u16) -> Vec<String> {
    let mut term = Terminal::new(TestBackend::new(W, h)).expect("TestBackend must build");
    term
      .draw(|f| {
        let area = f.area();
        render(s, first, area, f.buffer_mut());
      })
      .expect("a draw into an in-memory backend cannot fail");
    let buf = term.backend().buffer().clone();
    (0..h)
      .map(|y| {
        (0..W)
          .map(|x| buf[(x, y)].symbol().to_string())
          .collect::<String>()
          .trim_end()
          .to_string()
      })
      .collect()
  }

  /// **THE ONLY QUESTION THIS MODULE OWES AN ANSWER TO.** The layout is proved
  /// on the plan; here the plan's own strings must be the ones that reached the
  /// screen, on the rows the plan put them on.
  #[test]
  fn the_printer_prints_the_screen_it_was_given() {
    let s = screen();
    let composed = s.compose(0, H as usize);
    let screen_lines = painted(&s, 0, H);
    assert_eq!(screen_lines.len(), composed.len());
    for (i, expected) in composed.iter().enumerate() {
      assert_eq!(
        &screen_lines[i],
        expected.trim_end(),
        "line {i} on screen is not the line the layout composed"
      );
    }
  }

  /// `tui-design.md` §2's shape, verified on the painted cells rather than on
  /// the composed strings -- a printer that dropped the foot would still have
  /// been handed a correct plan.
  #[test]
  fn the_five_sections_reach_the_screen_in_their_declared_positions() {
    let s = screen();
    let painted_lines = painted(&s, 0, H);
    let rule: String = std::iter::repeat_n(RULE, W as usize).collect();
    assert_eq!(painted_lines[0], s.app, "the APP row must be painted first");
    assert_eq!(
      painted_lines[1], rule,
      "a rule must be painted under the APP row"
    );
    assert_eq!(
      painted_lines[H as usize - 4],
      rule,
      "a rule must be painted above the foot"
    );
    assert_eq!(painted_lines[H as usize - 3], s.status);
    assert_eq!(painted_lines[H as usize - 2], s.command);
    assert_eq!(painted_lines[H as usize - 1], s.info);
  }

  /// Scrolling moves the WINDOW and leaves the chrome alone -- the two
  /// obligations are independent, and a renderer laying everything out top-down
  /// would satisfy neither once the content grew.
  #[test]
  fn scrolling_moves_the_rows_and_never_the_chrome() {
    let s = screen();
    let top = painted(&s, 0, H);
    let down = painted(&s, 2, H);
    assert_ne!(
      top[2], down[2],
      "scrolling by two did not change the first body row"
    );
    assert_eq!(
      down[2], s.body.rows[2],
      "the first body row after scrolling by two is wrong"
    );
    for i in [
      0,
      1,
      H as usize - 4,
      H as usize - 3,
      H as usize - 2,
      H as usize - 1,
    ] {
      assert_eq!(top[i], down[i], "chrome line {i} moved when the rows did");
    }
  }

  /// **NOTHING DECORATIVE REACHES THE SCREEN.** Asserted over the painted cells
  /// rather than over this module's source, because the thing forbidden is a
  /// border on the screen and a source grep would pass just as happily against
  /// a widget that drew one under a different name.
  ///
  /// The horizontal rule is the one box-drawing character allowed, and only on
  /// the two lines the design puts it on -- so the check is by POSITION, not by
  /// **ONLY THE DECLARED RULES ARE EVER PAINTED, AND NO BORDER IS.** Section 2:
  /// *there are no borders anywhere; those rules are the only chrome.*
  ///
  /// **DRIVEN ON BOTH SHAPES, BECAUSE THE COUNT IS NOT A CONSTANT.** An unsplit
  /// screen carries two rules and a split one carries three -- and the third is
  /// LABELLED, so a check that recognised a rule by "every character is the
  /// box-drawing horizontal" would read the label as a border and report the
  /// split as a defect. Asserting only the unsplit shape would leave that check
  /// looking correct until the first detail pane.
  #[test]
  fn only_the_declared_rules_are_ever_painted_and_no_border_is() {
    let plain: String = std::iter::repeat_n(RULE, W as usize).collect();
    let labelled = layout::labelled_rule(W as usize);
    assert_ne!(
      plain, labelled,
      "the labelled rule is indistinguishable from a plain one, so the split case below proves \
       nothing this test could not already see"
    );

    for (what, s, rule_lines) in [
      ("unsplit", screen(), vec![1usize, H as usize - 4]),
      ("split", split_screen(), vec![1usize, 1 + 4, H as usize - 4]),
    ] {
      let painted_lines = painted(&s, 0, H);
      for (y, line) in painted_lines.iter().enumerate() {
        if rule_lines.contains(&y) {
          assert!(
            *line == plain || *line == labelled,
            "{what}: line {y} is declared a rule and is not one: {line:?}"
          );
          continue;
        }
        for ch in line.chars() {
          assert!(
            !"|+\u{2502}\u{250c}\u{2510}\u{2514}\u{2518}\u{251c}\u{2524}\u{252c}\u{2534}\u{253c}"
              .contains(ch),
            "{what}: a border character reached the screen on line {y}: {line:?}"
          );
          assert!(
            ch != RULE,
            "{what}: a box-drawing horizontal appeared off a rule line, on line {y}: {line:?}"
          );
        }
      }
      assert_eq!(
        painted_lines
          .iter()
          .filter(|l| **l == plain || **l == labelled)
          .count(),
        rule_lines.len(),
        "{what}: the screen painted a number of rules it did not declare"
      );
    }
  }

  /// A terminal too short for the chrome must not panic and must still show the
  /// way out. Panes get dragged shut.
  #[test]
  fn a_viewport_shorter_than_the_chrome_still_paints_the_foot() {
    let s = screen();
    for h in 1..CHROME as u16 {
      let painted_lines = painted(&s, 0, h);
      assert_eq!(painted_lines.len(), h as usize);
      assert_eq!(
        *painted_lines.last().unwrap(),
        s.info,
        "the INFO row must survive to the last line at height {h}"
      );
    }
  }
}
