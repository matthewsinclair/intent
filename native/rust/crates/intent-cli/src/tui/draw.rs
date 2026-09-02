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
//! # One border, and no decoration
//!
//! There is no `Block`, no title bar and no padding widget here. `tui-design.md`
//! §2 allowed exactly two rules and called them *the only chrome* until hv
//! ratified the framed composer (O1, 2026-09-02); the rule is now **borders on
//! the composer and nowhere else**. Even that box is drawn the same way as
//! everything else -- **strings at coordinates**, assembled by
//! [`super::layout`] -- because reaching for a bordering widget is how the
//! second border arrives.

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};

use super::layout::{Role, Screen};
use super::mode::Mode;

/// The palette: one place, role in, style out.
///
/// **ROLES COME FROM LAYOUT AND COLOURS LIVE HERE**, so a theme is a draw
/// edit and the information is a layout edit. The vocabulary follows
/// `docs/design/design-system.md`'s semantic set -- ok/warn/error from the
/// CLI's own prefix vocabulary, one accent, chrome dim -- translated to the
/// sixteen-colour terminal space so it renders everywhere.
///
/// **THE MODE CHIP IS THE HEADLINE** (hv, 2026-08-30: *the state changes
/// between modes are not obvious*): each mode gets its own colour, reversed,
/// so the chip reads as a lamp rather than a word.
fn style(role: Role) -> Style {
  let d = Style::default();
  match role {
    Role::Chrome | Role::Name | Role::Muted => d.fg(Color::DarkGray),
    // **THE COMPOSER'S BOX IS THE ONE BORDER THE DESIGN ALLOWS**, and it is
    // drawn in chrome dim rather than in the accent: the frame's job is to say
    // *this is where you type*, which the position already says. A bright box
    // would make the least informative line on the screen the loudest.
    Role::Frame => d.fg(Color::DarkGray),
    Role::Value => d,
    Role::Door => d.fg(Color::Cyan),
    Role::Ok => d.fg(Color::Green),
    Role::Warn => d.fg(Color::Yellow),
    Role::Error => d.fg(Color::Red),
    Role::Selected => d.add_modifier(Modifier::REVERSED),
    Role::Match => d.fg(Color::Cyan).add_modifier(Modifier::BOLD),
    Role::Title => d.add_modifier(Modifier::BOLD),
    Role::OmniActive => d.fg(Color::Cyan).add_modifier(Modifier::BOLD),
    // **THE COLOUR FOLLOWS THE LAMP, NOT THE MODE, and that is why `Field` and
    // `Embed` share one.** [`Mode::lamp`] shows both as `EDIT`; painting them
    // different colours would put one word on screen in two colours, which
    // reads as a distinction the operator is meant to act on and there is
    // none -- who owns the terminal is the machine's business, and `EMBED`
    // announces itself in the composer row instead.
    Role::ModeChip(mode) => {
      let fg = match mode {
        Mode::Omni => Color::Cyan,
        Mode::Menu => Color::Magenta,
        Mode::Field | Mode::Embed => Color::Yellow,
      };
      d.fg(fg).add_modifier(Modifier::REVERSED | Modifier::BOLD)
    }
  }
}

/// Paint `screen` into `area`, scrolled so body row `first` is at the top of
/// the body.
///
/// Base line first, then each span over it IN ORDER -- a later span wins an
/// overlap, which is how the selection overlays a row's own colours.
pub fn render(screen: &Screen, first: usize, area: Rect, buf: &mut Buffer) {
  if area.height == 0 || area.width == 0 {
    return;
  }
  for (i, (line, ink)) in screen
    .painted(first, area.height as usize)
    .iter()
    .enumerate()
  {
    let y = area.y + i as u16;
    buf.set_string(area.x, y, line, Style::default());
    let chars: Vec<char> = line.chars().collect();
    for &(start, end, role) in ink {
      let end = end.min(chars.len());
      if start >= end {
        continue;
      }
      let segment: String = chars[start..end].iter().collect();
      buf.set_string(area.x + start as u16, y, &segment, style(role));
    }
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
      omnibox: "\u{276f}".into(),
      caret: Some(1),
      hint: "OMNI  1/4  \u{23ce} edit".into(),
      dropdown: Vec::new(),
      mode: crate::tui::mode::Mode::Omni,
      selected: None,
      noticed: false,
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
  ///
  /// **POSITIONS DERIVE FROM THE DECLARED CONSTANTS.** The foot grew when hv's
  /// framed composer landed and will grow again with the status segments;
  /// hardcoded offsets would have to be edited on each, and editing an
  /// assertion to match what the code now does is how a check stops being one.
  #[test]
  fn the_sections_reach_the_screen_in_their_declared_positions() {
    let s = screen();
    let painted_lines = painted(&s, 0, H);
    let h = H as usize;
    let rule: String = std::iter::repeat_n(RULE, W as usize).collect();
    let framed = (W as usize) > 4 && h >= CHROME + layout::FRAME_COST + 1;
    assert!(framed, "this fixture is meant to exercise the FRAMED foot");
    let foot = layout::FOOT + layout::FRAME_COST;
    assert_eq!(painted_lines[0], s.app, "the APP row must be painted first");
    assert_eq!(
      painted_lines[1], rule,
      "a rule must be painted under the APP row"
    );
    assert_eq!(
      painted_lines[h - foot],
      rule,
      "a rule must be painted above the foot"
    );
    assert!(
      painted_lines[h - 4].starts_with(layout::BOX_TL),
      "the composer's box must open under the rule: {:?}",
      painted_lines[h - 4]
    );
    assert!(
      painted_lines[h - 3].contains(s.omnibox.trim()),
      "the composer's text must be painted inside its box: {:?}",
      painted_lines[h - 3]
    );
    assert!(
      painted_lines[h - 2].starts_with(layout::BOX_BL),
      "the composer's box must close above the hint: {:?}",
      painted_lines[h - 2]
    );
    assert_eq!(painted_lines[h - 1], s.hint);
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

    let h = H as usize;
    let foot = layout::FOOT + layout::FRAME_COST;
    // The composer's three lines: the ONLY place a border may appear.
    let framed_lines = [h - 4, h - 3, h - 2];
    for (what, s) in [("unsplit", screen()), ("split", split_screen())] {
      let painted_lines = painted(&s, 0, H);
      // **THE LABELLED RULE IS LOCATED, NOT COUNTED.** Its line depends on how
      // the body divides, which the framed composer changed by taking two rows
      // -- so a hardcoded index here reports "a border appeared" when all that
      // moved was the split.
      let mut rule_lines = vec![1usize, h - foot];
      if let Some(at) = painted_lines.iter().position(|l| *l == labelled) {
        rule_lines.push(at);
      }
      for (y, line) in painted_lines.iter().enumerate() {
        if rule_lines.contains(&y) {
          assert!(
            *line == plain || *line == labelled,
            "{what}: line {y} is declared a rule and is not one: {line:?}"
          );
          continue;
        }
        if framed_lines.contains(&y) {
          // **THE ONE RELAXATION, AND IT IS BOUNDED BY POSITION.** hv ruled
          // the framed composer in on 2026-09-02, so `no borders anywhere`
          // becomes BORDERS ON THE COMPOSER AND NOWHERE ELSE. Checking it by
          // POSITION rather than by exempting the characters globally is what
          // keeps the rest of the screen under the original rule -- an
          // exemption written as "these glyphs are allowed" would have
          // retired the property everywhere at once.
          continue;
        }
        for ch in line.chars() {
          assert!(
            !"|+\u{2502}\u{250c}\u{2510}\u{2514}\u{2518}\u{251c}\u{2524}\u{252c}\u{2534}\u{253c}\u{256d}\u{256e}\u{2570}\u{256f}"
              .contains(ch),
            "{what}: a border character reached the screen OFF the composer, on line {y}: {line:?}"
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
        s.hint,
        "the HINT row must survive to the last line at height {h}"
      );
    }
  }
  /// **THE PRINTER PAINTS THE ROLES IT WAS GIVEN** -- one cell per claim,
  /// read back through a real ratatui buffer, because a palette that never
  /// reaches a cell is a colour scheme in prose. The three headline claims:
  /// the mode chip is reversed and mode-coloured, a `wip` value is yellow,
  /// and dim chrome is actually dim.
  #[test]
  fn painted_roles_reach_the_cells() {
    use ratatui::style::{Color, Modifier};
    let mut sc = Screen {
      body: layout::plan(&[Row::new("status", "wip", "select")], W as usize),
      ..screen()
    };
    sc.mode = crate::tui::mode::Mode::Omni;
    sc.hint = "OMNI   status".into();
    let mut term = Terminal::new(TestBackend::new(W, H)).expect("TestBackend must build");
    term
      .draw(|f| {
        let area = f.area();
        render(&sc, 0, area, f.buffer_mut());
      })
      .expect("a draw into an in-memory backend cannot fail");
    let buf = term.backend().buffer().clone();

    // The chip leads the HINT line -- the last row. OMNI in the composer's
    // cyan, reversed.
    let chip = &buf[(0, H - 1)];
    assert_eq!(chip.symbol(), "O");
    assert_eq!(
      chip.style().fg,
      Some(Color::Cyan),
      "the chip must wear the mode's colour"
    );
    assert!(
      chip.style().add_modifier.contains(Modifier::REVERSED),
      "the chip must be reversed -- a lamp, not a word"
    );

    // The wip value: row 2 (app, rule, first body row), at the value column.
    let vx = sc.body.value_col as u16;
    let wip = &buf[(vx, 2)];
    assert_eq!(
      wip.symbol(),
      "w",
      "the fixture moved; this cell is not the value"
    );
    assert_eq!(
      wip.style().fg,
      Some(Color::Yellow),
      "wip must read as in-flight"
    );

    // The rule under the app row is chrome, and chrome is dim.
    let rule = &buf[(0, 1)];
    assert_eq!(rule.style().fg, Some(Color::DarkGray), "chrome must be dim");
  }
}
