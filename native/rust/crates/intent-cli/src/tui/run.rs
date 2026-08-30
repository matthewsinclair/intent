//! Composing the screen, and the shell that drives it.
//!
//! **[`screen_for`] IS PURE AND [`run`] IS AS SMALL AS IT CAN BE MADE**, which
//! is the same split as everywhere else in this module tree: the part with a
//! property worth asserting takes no terminal, and the part that must own a
//! terminal has no decisions left in it. `run` reads events and paints; it
//! chooses nothing.
//!
//! # `AT-17.9` covers `AC-17.9`, and its other half lives here
//!
//! *The current mode is always visible, and Esc always reaches nav mode from
//! anywhere.* The Esc half is proved twice already -- [`super::mode`] proves it
//! walks toward the rest state, [`super::app`] proves holding it gets you out.
//! **The always-visible half could not be proved before there was a screen**,
//! because it is not a fact about the machine at all: it is a fact about what
//! is on the STATUS row, in every mode, at every viewport size the chrome
//! survives.
//!
//! That is asserted here over every mode rather than eyeballed on one, because
//! *a modal interface whose mode is not on screen is the classic trap, and the
//! failure is silent: keystrokes go somewhere the operator did not intend and
//! nothing says so.*

use std::io;

use crossterm::event::{self, Event};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

use super::app::{App, Pane, Step};
use super::draw;
use super::edit::{self, Handoff, Landed, Refused, Session};
use super::layout::{self, Row, Screen};
use super::nav::View;
use super::terminal::{self, Borrowed, real};
use super::views;

/// Everything the loop needs from the store.
///
/// **ONE OBJECT RATHER THAN A `fetch` CLOSURE, AND THE BORROW CHECKER IS RIGHT
/// ABOUT WHY.** The loop reads rows and, on a handoff, writes a field: two
/// closures over one facade are two mutable borrows of one resource, because
/// that is what they are. One trait with a supertrait says the same thing
/// without lying about it.
///
/// The rows still arrive through a seam rather than a call, so **the whole
/// composition above stays drivable with no store at all**.
pub trait Source: edit::Model {
  fn rows(&mut self, view: &View) -> Vec<Row>;
}

/// A [`Session`] wrapped so its child runs with the terminal given back.
///
/// **THE BRACKET LIVES HERE AND NOT IN [`edit`], BECAUSE THAT MODULE MUST NOT
/// KNOW WHAT A TERMINAL IS.** Its whole value is that the handoff sequence is
/// provable without one. A decorator keeps the two facts in the two places that
/// own them: the sequence in `edit`, the borrow in `terminal`.
struct Lending<'a, S, T: terminal::Screen> {
  inner: S,
  term: &'a mut Borrowed<T>,
}

impl<S: Session, T: terminal::Screen> Session for Lending<'_, S, T> {
  fn scratch(&mut self, h: &Handoff, value: &str) -> Result<std::path::PathBuf, Refused> {
    self.inner.scratch(h, value)
  }

  fn launch(&mut self, path: &std::path::Path) -> Result<(), Refused> {
    // Destructured so the two fields are borrowed disjointly: the closure needs
    // `inner` while `term` is already borrowed mutably by `lend`.
    let Self { inner, term } = self;
    term
      .lend(|| inner.launch(path))
      .map_err(|e| Refused::new(format!("error: the terminal would not come back -- {e}")))?
  }

  fn read_back(&mut self, path: &std::path::Path) -> Result<String, Refused> {
    self.inner.read_back(path)
  }

  fn discard(&mut self, path: &std::path::Path) {
    self.inner.discard(path);
  }
}

/// Compose the whole screen for `app` showing `rows`, at `width`.
///
/// Pure: every decision about what the operator sees is made here, where it can
/// be asserted without a terminal.
pub fn screen_for(app: &App, rows: &[Row], width: usize) -> Screen {
  Screen {
    app: app_row(app),
    body: layout::plan(rows, width),
    // **THE SELECTED ROW DECIDES, AND NOTHING ELSE DOES.** No view kind is
    // consulted here and none should be: `tui-design.md` section 6 says the
    // split is triggered by the row CARRYING detail, because a list of kinds is
    // a second place to update and *it is the half that gets forgotten*.
    detail: app
      .focused_row(rows)
      .filter(|r| r.has_detail())
      .and_then(|r| r.detail.as_ref())
      .map(|d| layout::plan(d, width)),
    status: status_row(app, rows),
    command: command_row(app),
    info: info_row(app, rows),
  }
}

/// The entity and, when nested, the trail and the key that leaves.
///
/// *A way back that is wired and unlabelled is a way back nobody finds* -- a
/// real strawman defect, where `Backspace` worked and nothing on screen said so.
fn app_row(app: &App) -> String {
  let here = views::app_line(app.stack.current());
  if app.stack.at_root() {
    here
  } else {
    format!("{here}   {}   ESC back", app.stack.trail())
  }
}

/// Mode first, then as much about the row in context as fits.
///
/// **THE MODE IS FIRST AND UNCONDITIONAL.** Everything after it is about
/// whatever is selected and may be absent; the mode never is, because a modal
/// interface whose mode is not on screen is the trap `AC-17.9` names.
fn status_row(app: &App, rows: &[Row]) -> String {
  let mut parts = vec![app.mode.name().to_string()];
  if let Some(f) = app.focus
    && let Some(row) = rows.get(f.index())
  {
    parts.push(row.title.clone());
    parts.push(row.kind.clone());
    parts.push(format!("{}/{}", f.index() + 1, f.len()));
  }
  // **THE PANE HINT IS ONLY THERE WHEN THERE IS A PANE TO CROSS TO.** A `TAB
  // detail` standing on every row would be an offer the key does not honour,
  // which teaches the operator that Tab is broken rather than that this row has
  // no detail -- the same reason `cross_panes` does nothing rather than
  // self-looping.
  if let Some(hint) = pane_hint(app, rows) {
    parts.push(hint);
  }
  parts.join("   ")
}

/// `TAB detail` from the list, `TAB list` from the detail pane, and nothing at
/// all where the row carries none.
///
/// *A way across that is wired and unlabelled is a way across nobody finds* --
/// the same defect the APP row's `ESC back` exists for, one pane down.
fn pane_hint(app: &App, rows: &[Row]) -> Option<String> {
  if !app.focused_row(rows).is_some_and(layout::Row::has_detail) {
    return None;
  }
  Some(match app.pane(rows) {
    Pane::List => "TAB detail".to_string(),
    Pane::Detail => "TAB list".to_string(),
  })
}

/// The command in play. `tui-design.md` §2: the `:` line while composing, the
/// menu in MENU, the child's name in EMBED.
fn command_row(app: &App) -> String {
  match app.mode {
    super::mode::Mode::Command => ":".to_string(),
    super::mode::Mode::Menu => {
      "Go: [<-]  Back  Threads  Issues  Packages  Criteria  [X]".to_string()
    }
    super::mode::Mode::Embed => "editor running -- returns when the child exits".to_string(),
    _ => "cmd: (none)".to_string(),
  }
}

/// Help for whatever is under the cursor -- **unless something just happened**,
/// which takes the row.
///
/// **AN EDITOR HANDOFF IS THE ONE ACTION WHOSE RESULT IS INVISIBLE.** Every
/// other keystroke changes the screen. This one gives the terminal away and
/// comes back to a form that looks identical whether the save landed, was
/// declined, or was refused -- so a silent return and a silent failure are the
/// same picture, which is the shape `AC-17.10` spends its whole text on.
fn info_row(app: &App, rows: &[Row]) -> String {
  if !app.notice.is_empty() {
    return app.notice.clone();
  }
  let Some(row) = app.focus.and_then(|f| rows.get(f.index())) else {
    return String::new();
  };
  match row.kind.as_str() {
    "prose" => "A long value. Enter opens it in $EDITOR.".to_string(),
    "button" => "Enter opens this in its own pane.".to_string(),
    "select" => "A closed choice. Enter offers the legal ones.".to_string(),
    "number" => "A number. Enter edits it in place.".to_string(),
    _ => "Enter edits this in place.".to_string(),
  }
}

/// Drive the TUI until the operator quits.
///
/// **THE TERMINAL IS RESTORED ON EVERY EXIT PATH INCLUDING A PANIC**
/// (`AC-17.10`): the panic hook goes up before the borrow, because unwinding
/// reaches a `Drop` guard only AFTER the hook has printed -- by which time the
/// message is staircased across a raw-mode screen, and that is the state the
/// operator has to read the bug report out of.
///
/// `fetch` is how rows reach the screen. It is a parameter rather than a
/// facade because **this function should have no opinion about where data comes
/// from**; that is what lets the whole composition above be driven in tests
/// with no store at all.
pub fn run(app: &mut App, source: &mut impl Source, mut session: impl Session) -> io::Result<()> {
  real::restore_on_panic();
  let mut borrowed = Borrowed::take(real::Crossterm)?;
  let mut term = Terminal::new(CrosstermBackend::new(io::stdout()))?;

  let mut rows = source.rows(app.stack.current());
  app.point_at(rows.len());

  loop {
    let area = term.size()?;
    let screen = screen_for(app, &rows, area.width as usize);
    term.draw(|f| draw::render(&screen, app.scroll, f.area(), f.buffer_mut()))?;

    // Only key presses move the machine. A resize repaints on the next pass
    // because the loop re-reads the size every time rather than caching it.
    let ev = event::read()?;
    let Event::Key(key) = ev else { continue };
    if key.kind != event::KeyEventKind::Press {
      continue;
    }
    let was = app.stack.current().clone();
    match app.on_key(key, &rows) {
      Step::Quit => break,
      Step::Continue => {}
      Step::Hand(hand) => {
        let mut lending = Lending {
          inner: &mut session,
          term: &mut borrowed,
        };
        let outcome = edit::hand_off(source, &mut lending, &hand);
        app.child_exited();
        // **THE RE-READ IS UNCONDITIONAL, AND THAT IS THE CRITERION.** Not "if
        // the handoff reported a write" -- `AC-17.10` says the editor is the
        // AUTHORITY, and a return path that trusts its own report of what
        // happened is trusting the thing it just gave the terminal away to. A
        // cheap read is what stands between the operator and a repaint from a
        // model the file has moved past.
        rows = source.rows(app.stack.current());
        app.refocus(rows.len());
        app.notice = match outcome {
          Ok(Landed::Written) => format!("{} saved", hand.field),
          Ok(Landed::Unchanged) => format!("{} unchanged", hand.field),
          Err(why) => why.to_string(),
        };
        // A terminal that would not come back is not survivable: the loop is
        // about to paint into a screen it does not own.
        if borrowed.outstanding().is_empty() {
          return Err(io::Error::other(
            "the terminal was lent to the editor and could not be taken back",
          ));
        }
      }
    }
    // **THE VIEW CHANGED, SO THE ROWS MUST BE RE-READ BEFORE ANYTHING DERIVED
    // FROM THEM IS PAINTED.** Repainting a new view from the old rows is the
    // same class as `AC-17.10`'s stale-model save, one keystroke earlier.
    if *app.stack.current() != was {
      rows = source.rows(app.stack.current());
      app.point_at(rows.len());
      app.notice.clear();
    }
  }

  borrowed.restore();
  Ok(())
}

#[cfg(test)]
mod tests {
  use super::super::mode::Mode;
  use super::*;

  fn rows() -> Vec<Row> {
    vec![
      Row::new("title", "ST0056: Intent v3", "text"),
      Row::new("status", "wip", "select"),
      Row::new("objective", "673 bytes", "prose"),
      Row::new("work pkgs", "17", "button"),
    ]
  }

  /// **`AC-17.9`'s ALWAYS-VISIBLE HALF, AND IT IS NOT A FACT ABOUT THE MACHINE.**
  /// Asserted over every mode and at every viewport the chrome survives, on the
  /// COMPOSED lines rather than on the status string -- a status row that is
  /// correct and never reaches the screen is exactly the silent failure the
  /// criterion names.
  #[test]
  fn the_current_mode_is_on_screen_in_every_mode() {
    let r = rows();
    let mut checked = 0usize;
    for &mode in Mode::ALL {
      let mut app = App::explore();
      app.mode = mode;
      app.point_at(r.len());
      for height in [layout::CHROME, layout::CHROME + 4, 24] {
        let screen = screen_for(&app, &r, 60);
        let lines = screen.compose(app.scroll, height);
        assert!(
          lines.iter().any(|l| l.contains(mode.name())),
          "{mode:?} is not on screen at height {height}; a modal interface whose mode is not \
           visible sends keystrokes somewhere the operator did not intend and says nothing"
        );
        checked += 1;
      }
    }
    assert!(
      checked > 0,
      "no mode was examined, so this test asserted nothing"
    );
  }

  /// The mode is FIRST on the status row, so it survives clipping at a narrow
  /// viewport -- which is the width at which an operator most needs it.
  #[test]
  fn the_mode_survives_a_viewport_too_narrow_for_the_rest_of_the_status_row() {
    let r = rows();
    for &mode in Mode::ALL {
      let mut app = App::explore();
      app.mode = mode;
      app.point_at(r.len());
      let screen = screen_for(&app, &r, mode.name().len() + 2);
      let lines = screen.compose(0, 20);
      assert!(
        lines.iter().any(|l| l.contains(mode.name())),
        "{mode:?} was clipped off its own status row"
      );
    }
  }

  /// The status row names the row in context, and the INFO row changes with it
  /// -- *help for whatever is under the cursor, right now*.
  #[test]
  fn the_status_and_info_rows_follow_the_cursor() {
    let r = rows();
    let mut app = App::explore();
    app.point_at(r.len());
    let first = screen_for(&app, &r, 60);
    assert!(
      first.status.contains("title"),
      "the status row does not name the selected row"
    );
    assert!(
      first.status.contains("1/4"),
      "the status row does not say where the cursor is"
    );

    app.focus = app.focus.map(super::super::focus::Focus::forward);
    app.focus = app.focus.map(super::super::focus::Focus::forward);
    let third = screen_for(&app, &r, 60);
    assert!(third.status.contains("objective"));
    assert!(third.status.contains("3/4"));
    assert_ne!(
      first.info, third.info,
      "the INFO row did not change with the cursor"
    );
    assert!(
      third.info.contains("$EDITOR"),
      "a prose row must say where Enter takes you"
    );
  }

  /// A view with no rows still composes: the chrome is there, the mode is
  /// there, and nothing indexes into an empty list.
  #[test]
  fn an_empty_view_still_carries_its_chrome_and_its_mode() {
    let mut app = App::explore();
    app.point_at(0);
    let screen = screen_for(&app, &[], 40);
    let lines = screen.compose(0, 12);
    assert_eq!(lines.len(), 12);
    assert!(lines.iter().any(|l| l.contains(app.mode.name())));
    assert!(
      screen.info.is_empty(),
      "an empty view must not offer help about a row that is absent"
    );
  }

  /// Nested views say where they are and how to leave. The root does not need
  /// to, because there is nowhere above it.
  #[test]
  fn a_nested_view_names_its_trail_and_the_key_that_leaves() {
    let r = rows();
    let mut app = App::explore();
    app.point_at(r.len());
    let root = screen_for(&app, &r, 80);
    assert!(
      !root.app.contains("ESC back"),
      "the root offered a way back to nowhere"
    );

    app.push(View::Collection {
      kind: "thread".into(),
    });
    let nested = screen_for(&app, &r, 80);
    assert!(
      nested.app.contains("ESC back"),
      "a nested view did not say how to leave"
    );
    assert!(
      nested.app.contains("thread"),
      "a nested view did not say where it is"
    );
  }

  /// The command row is the one place four different modes each say something
  /// different, so it is asserted as four distinct strings rather than as
  /// "non-empty".
  #[test]
  fn the_command_row_says_something_different_in_every_mode_that_uses_it() {
    let mut seen: Vec<String> = Vec::new();
    for mode in [Mode::Normal, Mode::Command, Mode::Menu, Mode::Embed] {
      let mut app = App::explore();
      app.mode = mode;
      let line = screen_for(&app, &[], 80).command;
      assert!(!line.is_empty(), "{mode:?} puts nothing on the command row");
      assert!(
        !seen.contains(&line),
        "{mode:?} shares a command row with another mode: {line:?}"
      );
      seen.push(line);
    }
  }

  /// **THE SPLIT FOLLOWS THE CURSOR WITHIN ONE VIEW, WHICH IS WHAT MAKES IT
  /// ROW-TRIGGERED RATHER THAN KIND-TRIGGERED.** Same app, same view, same row
  /// set -- only the cursor moves, and the pane opens and closes with it.
  ///
  /// **A SPLIT KEYED OFF THE VIEW KIND IS CORRECT ON EVERY SCREENSHOT OF A
  /// CRITERIA LIST AND FAILS HERE**, which is why the control is moving the
  /// cursor rather than changing the view. `tui-design.md` section 6 names this
  /// exactly: a list of kinds is a second place to update when a new view
  /// arrives, *and it is the half that gets forgotten*.
  #[test]
  fn the_split_follows_the_cursor_rather_than_the_view() {
    let with_detail = vec![
      Row::new("title", "ST0056", "text"),
      Row::new("status", "wip", "select").expanding_to(vec![
        Row::new("legal", "done, cancelled", "text"),
        Row::new("owed", "a reason", "text"),
      ]),
      Row::new("slug", "add-a-rust-based-cli", "text"),
    ];
    assert_eq!(
      with_detail.iter().filter(|r| r.has_detail()).count(),
      1,
      "the fixture must carry detail on exactly one row, or the walk below cannot tell the two \
       wirings apart"
    );

    let mut app = App::explore();
    app.point_at(with_detail.len());
    let before = app.stack.current().clone();

    let mut opened_on = Vec::new();
    for at in 0..with_detail.len() {
      app.focus = app.focus.and_then(|f| f.at(at));
      let screen = screen_for(&app, &with_detail, 60);
      if screen.detail.is_some() {
        opened_on.push(at);
      }
      assert_eq!(
        app.stack.current(),
        &before,
        "the view changed during the walk, so the split could still be keyed off it"
      );
    }
    assert_eq!(
      opened_on,
      vec![1usize],
      "the detail pane opened on rows {opened_on:?}, and exactly one row carries detail"
    );
  }

  /// The detail pane reaches the screen, with its own rows on it. A `Screen`
  /// that carried detail nothing composed would satisfy the trigger test above
  /// and show the operator nothing.
  #[test]
  fn the_detail_the_row_carries_is_what_reaches_the_screen() {
    let rows = vec![
      Row::new("title", "ST0056", "text"),
      Row::new("status", "wip", "select").expanding_to(vec![
        Row::new("legal", "done, cancelled", "text"),
        Row::new("owed", "a reason", "text"),
      ]),
    ];
    let mut app = App::explore();
    app.point_at(rows.len());
    app.focus = app.focus.and_then(|f| f.at(1));
    let lines = screen_for(&app, &rows, 60).compose(0, 24);
    for expected in ["legal", "done, cancelled", "owed", "a reason"] {
      assert!(
        lines.iter().any(|l| l.contains(expected)),
        "{expected:?} is in the row's detail and never reached the screen"
      );
    }
    assert!(
      lines
        .iter()
        .any(|l| l.contains(layout::DETAIL_LABEL.trim())),
      "the detail pane has no rule naming it"
    );
  }

  /// **THE PANE HINT IS ON THE STATUS ROW ONLY WHERE THERE IS A PANE TO CROSS
  /// TO, AND IT NAMES WHICH WAY THE KEY GOES.** A `TAB detail` standing on
  /// every row is an offer the key does not honour, which teaches the operator
  /// that Tab is broken rather than that this row has no detail; and *a way
  /// across that is wired and unlabelled is a way across nobody finds*, which
  /// was a real defect on the APP row one level up.
  #[test]
  fn the_status_row_offers_the_crossing_only_where_there_is_one() {
    let rows = vec![
      Row::new("title", "ST0056", "text"),
      Row::new("status", "wip", "select").expanding_to(vec![Row::new("legal", "done", "text")]),
    ];
    let mut app = App::explore();
    app.point_at(rows.len());

    let plain = screen_for(&app, &rows, 80).status;
    assert!(
      !plain.contains("TAB"),
      "a row with no detail offered a crossing: {plain:?}"
    );

    app.focus = app.focus.and_then(|f| f.at(1));
    let offered = screen_for(&app, &rows, 80).status;
    assert!(
      offered.contains("TAB detail"),
      "a row carrying detail did not say how to reach it: {offered:?}"
    );

    app.wants_detail = true;
    let inside = screen_for(&app, &rows, 80).status;
    assert!(
      inside.contains("TAB list"),
      "the detail pane did not say how to get back: {inside:?}"
    );
    assert!(
      inside.contains(app.mode.name()),
      "the hint displaced the mode, which is the one thing the status row must always carry"
    );
  }
}
