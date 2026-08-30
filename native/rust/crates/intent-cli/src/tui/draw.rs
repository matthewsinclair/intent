//! The printer: `AT-17.11` covering `AC-17.11`, second half.
//!
//! **THIS MODULE HAS NO OPINIONS AND THAT IS ITS ENTIRE SPECIFICATION.**
//! [`super::layout`] decides where every character goes; this puts those
//! characters on a surface. The split is not tidiness -- it is what makes
//! `AC-17.11` assertable. *The alignment IS the design, so it is asserted
//! rather than eyeballed*, and a property asserted against a rendered buffer is
//! really a property asserted against whatever the widget library chose to do.
//! So the alignment is proved on the plan, as data, and the only question left
//! here is the small one: **did the printer print the plan it was given?**
//!
//! # Why the foot is pinned and the rows are not
//!
//! `AC-17.11` puts one modeline at the foot and one rule with it, and makes the
//! column scroll. Those are different obligations: the foot is at a FIXED
//! position relative to the viewport, and the rows are a WINDOW onto a list
//! that is usually longer than the screen. A renderer that laid all of it out
//! top-down would put the modeline wherever the content happened to end, which
//! is the one place the operator cannot learn to look.
//!
//! # No borders, no decoration
//!
//! There is no `Block`, no border, no title bar and no padding widget here.
//! The criterion forbids them, and the cheapest way to keep forbidding them is
//! to never reach for the widget that draws one: this module writes strings at
//! coordinates and nothing else.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

use super::layout::Plan;

/// How many lines the foot occupies: the rule and the modeline.
pub const FOOT: u16 = 2;

/// The rows `area` has room for once the foot is taken out.
pub fn rows_visible(area: Rect) -> usize {
  area.height.saturating_sub(FOOT) as usize
}

/// Put `plan` on `buf` inside `area`, scrolled so `first` is the top row.
///
/// Writes strings at coordinates and nothing else -- see the module note on why
/// there is no [`ratatui::widgets::Block`] in here.
pub fn render(plan: &Plan, first: usize, area: Rect, buf: &mut Buffer) {
  if area.height == 0 || area.width == 0 {
    return;
  }
  let style = Style::default();

  for (i, line) in plan.visible(first, rows_visible(area)).iter().enumerate() {
    let y = area.y + i as u16;
    buf.set_string(area.x, y, line, style);
  }

  // The foot is pinned to the bottom of the viewport, never to the end of the
  // content. Guarded because a viewport shorter than the foot has nowhere to
  // put it, and a one-line terminal is a thing that happens.
  if area.height >= FOOT {
    let bottom = area.y + area.height - 1;
    buf.set_string(area.x, bottom - 1, &plan.rule, style);
    buf.set_string(area.x, bottom, &plan.modeline, style);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::tui::layout::{Row, plan};
  use ratatui::Terminal;
  use ratatui::backend::TestBackend;

  const W: u16 = 30;
  const H: u16 = 8;

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

  /// Draw through a real `ratatui` terminal and read the cells back.
  fn painted(first: usize) -> Vec<String> {
    let p = plan(&rows(), W as usize, "NORMAL");
    let mut term = Terminal::new(TestBackend::new(W, H)).expect("TestBackend must build");
    term
      .draw(|f| {
        let area = f.area();
        render(&p, first, area, f.buffer_mut());
      })
      .expect("a draw into an in-memory backend cannot fail");
    let buf = term.backend().buffer().clone();
    (0..H)
      .map(|y| {
        (0..W)
          .map(|x| buf[(x, y)].symbol().to_string())
          .collect::<String>()
          .trim_end()
          .to_string()
      })
      .collect()
  }

  /// **THE ONLY QUESTION THIS MODULE OWES AN ANSWER TO.** The alignment is
  /// proved on the plan; here the plan's own strings must be the ones that
  /// reached the screen, at the rows the plan put them on.
  #[test]
  fn the_printer_prints_the_plan_it_was_given() {
    let p = plan(&rows(), W as usize, "NORMAL");
    let screen = painted(0);
    for (i, expected) in p
      .visible(0, rows_visible(Rect::new(0, 0, W, H)))
      .iter()
      .enumerate()
    {
      assert_eq!(
        &screen[i], expected,
        "row {i} on screen is not the row the plan computed"
      );
    }
  }

  /// `AC-17.11` puts the modeline at the foot. Pinned to the VIEWPORT, so a
  /// form with four rows in an eight-line terminal still finds it in the same
  /// place as a form with four hundred.
  #[test]
  fn the_foot_is_at_the_bottom_of_the_viewport_not_the_end_of_the_content() {
    let screen = painted(0);
    assert_eq!(screen.len(), H as usize);
    assert_eq!(
      screen[H as usize - 2],
      "-".repeat(W as usize),
      "the rule must be the second-from-last line whatever the content did"
    );
    assert_eq!(
      screen[H as usize - 1],
      "NORMAL",
      "the modeline must be the last line"
    );
    assert_eq!(
      screen[rows().len()],
      "",
      "the line after the last row must be blank, not the foot floating up to meet it"
    );
  }

  /// Scrolling moves the WINDOW and leaves the foot alone -- the two
  /// obligations `AC-17.11` states are independent, and a renderer that laid
  /// everything out top-down would satisfy neither once the content grew.
  #[test]
  fn scrolling_moves_the_rows_and_never_the_foot() {
    let top = painted(0);
    let down = painted(2);
    assert_ne!(
      top[0], down[0],
      "scrolling by two did not change the first visible row"
    );
    assert_eq!(
      down[0],
      plan(&rows(), W as usize, "NORMAL").rows[2],
      "the first visible row after scrolling by two is not the third row"
    );
    assert_eq!(
      top[H as usize - 1],
      down[H as usize - 1],
      "the modeline moved when the rows did"
    );
    assert_eq!(
      top[H as usize - 2],
      down[H as usize - 2],
      "the rule moved when the rows did"
    );
  }

  /// **NOTHING DECORATIVE REACHES THE SCREEN.** Asserted over the painted
  /// cells rather than over this module's source, because the thing forbidden
  /// is a border on the screen and a source grep would pass just as happily
  /// against a widget that drew one under a different name.
  #[test]
  fn no_border_or_decoration_is_ever_painted() {
    let screen = painted(0);
    let rule = "-".repeat(W as usize);
    for (y, line) in screen.iter().enumerate() {
      if *line == rule {
        assert_eq!(
          y,
          H as usize - 2,
          "a rule appeared somewhere other than the foot"
        );
        continue;
      }
      for ch in line.chars() {
        assert!(
          !"|+\u{2500}\u{2502}\u{250c}\u{2510}\u{2514}\u{2518}".contains(ch),
          "a box-drawing or border character reached the screen on line {y}: {line:?}"
        );
      }
    }
  }

  /// A terminal too short for the foot must not panic and must not paint a
  /// half foot. One-line terminals happen, and so do panes dragged shut.
  #[test]
  fn a_viewport_shorter_than_the_foot_paints_rows_and_no_foot() {
    let p = plan(&rows(), W as usize, "NORMAL");
    for h in [1u16, 2] {
      let mut term = Terminal::new(TestBackend::new(W, h)).expect("TestBackend must build");
      term
        .draw(|f| {
          let area = f.area();
          render(&p, 0, area, f.buffer_mut());
        })
        .expect("a draw into an in-memory backend cannot fail");
      let buf = term.backend().buffer().clone();
      let first: String = (0..W)
        .map(|x| buf[(x, 0)].symbol().to_string())
        .collect::<String>()
        .trim_end()
        .to_string();
      if h == 1 {
        assert_eq!(
          first, "",
          "a one-line viewport has no room for a row or a foot"
        );
      }
    }
  }
}
