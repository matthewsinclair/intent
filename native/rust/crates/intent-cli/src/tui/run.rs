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

use crossterm::event::{self, Event, KeyCode};
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

  /// Resolve an operator's spelling to a view, or say why not.
  ///
  /// **ON THE SOURCE BECAUSE PRESENCE IS A FACT ONLY THE STORE KNOWS** --
  /// `nav::land` needs a presence probe and the app deliberately has no
  /// facade. The default refuses, which is the honest answer for a test
  /// source that never claimed to resolve anything.
  ///
  /// [`Refused`] rather than a bare string, and rather than a new enum: the
  /// one consumer is the info row, which is exactly what `Refused` already
  /// means on this surface -- a refusal whose content is its sentence, with
  /// the refusing module's own words carried whole (`AC-06.12`'s resolver
  /// writes them; a variant enum here would be a second author).
  fn locate(&mut self, spelling: &str) -> Result<View, Refused> {
    Err(Refused::new(format!(
      "`{spelling}` resolves nothing -- this source has no resolver"
    )))
  }

  /// Every addressable entity, for the omnibox. Default: none, honestly.
  fn index(&mut self) -> Vec<super::omnibox::Entry> {
    Vec::new()
  }
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
  // **THE EDIT IS DRAWN IN THE VALUE COLUMN, WHERE THE FIELD LIVES**
  // (`tui-design.md` section 7: inline, not in a footer). The substitution is
  // display-only: the rows the caller owns are untouched, and the buffer
  // replaces the value on exactly the row whose name the handoff carries.
  let edited: Vec<Row>;
  let rows = match (&app.editing, app.mode) {
    (Some(edit), super::mode::Mode::Field) => {
      edited = rows
        .iter()
        .map(|r| {
          if r.name == edit.handoff.field {
            let mut shown = r.clone();
            shown.value = format!("{}\u{258f}", edit.buffer);
            shown
          } else {
            r.clone()
          }
        })
        .collect();
      edited.as_slice()
    }
    _ => rows,
  };
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
    omnibox: omnibox_row(app),
    hint: hint_row(app, rows),
    dropdown: dropdown(app),
    mode: app.mode,
    // The overlay follows the LIST cursor only: the detail pane keeps its own
    // focus and its own (future) treatment.
    selected: matches!(app.pane(rows), super::app::Pane::List)
      .then(|| app.focus.map(|f| f.index()))
      .flatten(),
    noticed: !app.notice.is_empty(),
  }
}

/// The entity and, when nested, the trail and the key that leaves.
///
/// *A way back that is wired and unlabelled is a way back nobody finds* -- a
/// real strawman defect, where `Backspace` worked and nothing on screen said so.
///
/// **AND IT SAID `ESC back` UNTIL THE COLLAPSE, WHICH IS THE SAME DEFECT
/// INVERTED.** Esc used to leave the omnibox for NAV, which read as *back*
/// from inside the input; it now clears the query and never navigates, so the
/// label named a key that no longer does the thing. **A way back labelled with
/// the WRONG key is worse than an unlabelled one** -- the operator presses it,
/// something else happens, and the screen told them to.
fn app_row(app: &App) -> String {
  let here = views::app_line(app.stack.current());
  if app.stack.at_root() {
    here
  } else {
    format!("{here}   {}   \u{232b} back", app.stack.trail())
  }
}

/// The composer line: the caret and the buffer, in OMNI and in MENU alike.
///
/// **THE PALETTE DOES NOT BORROW THIS LINE; IT IS THIS LINE.** Until hv ruled
/// the filtered palette (2026-09-02) this returned a HARDCODED Lotus menu bar
/// -- `Go: [<-] Back Threads ...` -- a string with no model behind it, whose
/// entries could not be selected and did nothing when chosen. The palette
/// types into the composer like everything else; the leading `/` is what says
/// which vocabulary the dropdown below is showing.
fn omnibox_row(app: &App) -> String {
  match app.mode {
    super::mode::Mode::Embed => "editor running -- returns when the child exits".to_string(),
    // **ALWAYS THERE, AND NOW ALWAYS LIT** -- the one home's whole point: the
    // composer holds the keyboard in every state the TUI owns, so it carries
    // the cursor rather than standing dim waiting to be selected. **MENU
    // carries it too, because the palette COLLECTS**: the sigil in the buffer
    // is what tells the two apart, not the presence of a cursor.
    super::mode::Mode::Omni | super::mode::Mode::Menu => {
      // **THE CARET IS DRAWN WHERE IT IS, NOT AT THE END.** It was appended
      // while the buffer had no cursor; with one, appending would paint the
      // caret in a place the next keystroke does not land -- an input that
      // lies about where you are typing.
      let buffer = &app.omnibox.buffer;
      let at = buffer
        .char_indices()
        .nth(app.omnibox.cursor())
        .map(|(i, _)| i)
        .unwrap_or(buffer.len());
      format!("\u{276f} {}\u{258f}{}", &buffer[..at], &buffer[at..])
    }
    _ => format!("\u{276f} {}", app.omnibox.buffer),
  }
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

/// The hint line: mode chip first and unconditional (`AC-17.9` -- a modal
/// interface whose mode is not on screen is the trap), then a notice if one
/// is standing, else what the keys do RIGHT NOW -- position, and the one
/// verb Enter means on this row.
fn hint_row(app: &App, rows: &[Row]) -> String {
  // **THE LAMP, NOT THE MACHINE'S NAME.** `AC-17.9` asks that the mode be
  // visible to the OPERATOR, and `FIELD`/`EMBED` is a distinction they cannot
  // act on -- see [`super::mode::Mode::lamp`]. It stays FIRST on the row so it
  // survives clipping at any width.
  let mut parts = vec![app.mode.lamp().to_string()];
  if !app.notice.is_empty() {
    parts.push(app.notice.clone());
    return parts.join("  ");
  }
  match app.mode {
    // **ONE MODE, TWO HINT SETS, GUARDED ON THE BUFFER** -- the same guard
    // that routes the keys. Browsing and querying were two MODES with two hint
    // rows before the collapse; they are now two states of one row, and the
    // hints have to follow the keys or the foot lies about what Enter does.
    super::mode::Mode::Omni if !app.omnibox.is_empty() => {
      parts.push(
        "\u{23ce} go \u{b7} \u{2191}\u{2193} pick \u{b7} esc clear \u{b7} \u{232b} delete".into(),
      );
    }
    super::mode::Mode::Omni => {
      if let Some(f) = app.focus
        && let Some(row) = rows.get(f.index())
      {
        parts.push(format!("{}/{}", f.index() + 1, f.len()));
        let verb = match row.kind.as_str() {
          "prose" => "\u{23ce} $EDITOR",
          "artefact" => "\u{23ce} open file",
          "button" if row.door.is_some() => "\u{23ce} open",
          "button" => "",
          "select" => "\u{23ce} choose",
          _ => "\u{23ce} edit",
        };
        if !verb.is_empty() {
          parts.push(verb.into());
        }
      }
      parts.push(
        "\u{2191}\u{2193} browse \u{b7} / menu \u{b7} \u{232b} back \u{b7} type to find".into(),
      );
      if let Some(hint) = pane_hint(app, rows) {
        parts.push(hint);
      }
    }
    super::mode::Mode::Menu => parts.push(
      "type to filter \u{b7} \u{2191}\u{2193} pick \u{b7} \u{23ce} run \u{b7} esc close".into(),
    ),
    super::mode::Mode::Field => parts.push("\u{23ce} commit \u{b7} esc discard".into()),
    super::mode::Mode::Embed => {}
  }
  parts.join("  ")
}

/// The dropdown: one line per match, best first, the pick marked and the
/// matched letters carrying [`Role::Match`] -- the television affordance
/// that makes a fuzzy list legible rather than magical.
///
/// **THE LINE IS THE HAYSTACK, VERBATIM, TWO COLUMNS IN** -- built from
/// `omnibox::haystack` so `Match::positions` map by a constant offset and
/// cannot drift from the text they highlight.
/// **ONE RENDERER, TWO VOCABULARIES.** The entity index and the command
/// palette differ only in what the haystack says and where its boosted prefix
/// ends -- exactly the two things [`super::omnibox::rank`] already takes. A
/// second dropdown for commands would have been the obvious shape and it is
/// the Highlander defect: the pick marker, the reversed order, the
/// matched-letter highlighting and the offset arithmetic are one behaviour.
fn dropdown(app: &App) -> Vec<(String, layout::Ink)> {
  use super::layout::Role;
  // (hit, haystack, boosted-prefix length) for whichever vocabulary is live.
  let listed: Vec<(super::omnibox::Match, String, usize)> = match app.mode {
    super::mode::Mode::Omni => {
      super::omnibox::matches(&app.index, &app.omnibox.buffer, super::app::MATCH_CAP)
        .into_iter()
        .map(|hit| {
          let e = &app.index[hit.entry];
          let hay = super::omnibox::haystack(e);
          let boost = e.id.chars().count();
          (hit, hay, boost)
        })
        .collect()
    }
    super::mode::Mode::Menu => app
      .palette()
      .into_iter()
      .map(|hit| {
        let c = &app.commands[hit.entry];
        let hay = super::commands::haystack(c);
        let boost = c.name.chars().count();
        (hit, hay, boost)
      })
      .collect(),
    _ => return Vec::new(),
  };
  let picked = app.omnibox.picked(listed.len());
  // **RENDER ORDER IS REVERSED: BEST LAST, NEAREST THE INPUT.** The input
  // sits at the bottom, so the adjacent line is where the eye rests -- the
  // television idiom for a bottom prompt. `Up` walking toward worse matches
  // is then also literally up the screen.
  listed
    .iter()
    .enumerate()
    .rev()
    .map(|(i, (hit, hay, boost))| {
      let mark = if picked == Some(i) { "\u{276f} " } else { "  " };
      let line = format!("{mark}{hay}");
      let offset = 2;
      let mut ink: layout::Ink = vec![(0, line.chars().count(), Role::Muted)];
      ink.push((offset, offset + boost, Role::Door));
      for &p in &hit.positions {
        ink.push((offset + p, offset + p + 1, Role::Match));
      }
      if picked == Some(i) {
        ink.push((0, line.chars().count(), Role::Selected));
      }
      (line, ink)
    })
    .collect()
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
  app.index = source.index();
  app.commands = super::commands::vocabulary();

  loop {
    let area = term.size()?;
    let screen = screen_for(app, &rows, area.width as usize);
    term.draw(|f| draw::render(&screen, app.scroll, f.area(), f.buffer_mut()))?;
    let mut lent_the_terminal = false;

    // Only key presses move the machine. A resize repaints on the next pass
    // because the loop re-reads the size every time rather than caching it.
    let ev = event::read()?;
    let Event::Key(key) = ev else { continue };
    if key.kind != event::KeyEventKind::Press {
      continue;
    }
    // **QUITTING IS AN ACT, NEVER AN ACCIDENT** (`tui-design.md` §3): Ctrl-C
    // from anywhere, `:q` from the omnibox, and no key reaches quit by
    // walking. Answered ahead of the machine the way `Tab` is, because QUIT
    // is not a mode and an edge for it would put one in the graph.
    if key.code == KeyCode::Char('c')
      && key
        .modifiers
        .contains(crossterm::event::KeyModifiers::CONTROL)
    {
      break;
    }
    let was = app.stack.current().clone();
    match app.on_key(key, &rows) {
      Step::Quit => break,
      Step::Continue => {}
      // **THE SPELLING LANDS THROUGH THE ONE RESOLVER** (`AC-06.12` -- `56`,
      // `ST0056`, `st56` all name one thread). Failure reaches the info row
      // in the resolver's own words, never a guess about what was meant.
      Step::Land(spelling) => match source.locate(&spelling) {
        Ok(view) => {
          app.omnibox.buffer.clear();
          app.push(view);
        }
        Err(why) => app.notice = why.to_string(),
      },
      // The edit opens on the RAW value or not at all.
      Step::ReadField(h) => match source.read(&h) {
        Ok(value) => app.begin_edit(h, value),
        Err(why) => app.abort_edit(why.to_string()),
      },
      // **THE RE-READ IS UNCONDITIONAL** -- same rule as the handoff: the
      // store is the authority on what the write did, and a repaint from the
      // in-memory rows is a repaint from before it.
      Step::WriteField(h, value) => {
        app.notice = match source.write(&h, &value) {
          Ok(()) => format!("{} saved", h.field),
          Err(why) => why.to_string(),
        };
        rows = source.rows(app.stack.current());
        app.refocus(rows.len());
      }
      // **`AC-17.8`: THE ARTEFACT IS OPENED IN PLACE, AND A GENERATED VIEW IS
      // REFUSED BY NAME.** No scratch file and no read-back -- the editor
      // writes the artefact directly, so there are no bytes of the operator's
      // held anywhere that a failure could strand. The re-read afterwards is
      // still unconditional, for `AC-17.10`'s reason: the editor is the
      // authority on what happened and its own report is not evidence.
      Step::Open { kind, id, name } => {
        match source.artefact(&kind, &id, &name) {
          Err(why) => app.notice = why.to_string(),
          Ok(path) => {
            let lent = borrowed.lend(|| session.launch(&path));
            app.notice = match lent {
              Ok(Ok(())) => format!("{name} closed"),
              Ok(Err(why)) => why.to_string(),
              Err(e) => format!("the terminal would not come back: {e}"),
            };
          }
        }
        app.child_exited();
        lent_the_terminal = true;
        rows = source.rows(app.stack.current());
        app.refocus(rows.len());
      }
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
        lent_the_terminal = true;
      }
    }

    // **COMING BACK FROM A CHILD IS ONE SITUATION, SO IT IS HANDLED IN ONE
    // PLACE.** Both handoff paths lend the terminal and both owe the same two
    // things on return; they were carrying one of them as a copy apiece.
    if lent_the_terminal {
      // A terminal that would not come back is not survivable: the loop is
      // about to paint into a screen it does not own.
      if borrowed.outstanding().is_empty() {
        return Err(io::Error::other(
          "the terminal was lent to the editor and could not be taken back",
        ));
      }
      // **THE SCREEN COMES BACK BLANK AND `ratatui` DOES NOT KNOW IT.** The
      // editor drew over the alternate screen and `lend` re-entered it, so the
      // cells are empty -- but the terminal still holds the buffer it painted
      // BEFORE the handoff, and the next draw is a diff against that. Every
      // cell matches, almost nothing is emitted, and the operator gets an
      // unpainted page. `clear` drops the remembered buffer so the next draw
      // paints in full. **Found by hv driving it, and invisible to every test
      // here** -- the whole module is built to be provable without a terminal,
      // and this is a defect that only exists inside one.
      term.clear()?;
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
  ///
  /// **IT LOOKS FOR THE LAMP, NOT THE MACHINE'S NAME, because the lamp is what
  /// the operator can see.** `FIELD` and `EMBED` both show `EDIT`; asserting
  /// `name()` here would demand the screen print a word it deliberately does
  /// not, and the criterion is about what is VISIBLE.
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
          lines.iter().any(|l| l.contains(mode.lamp())),
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
      let screen = screen_for(&app, &r, mode.lamp().len() + 2);
      let lines = screen.compose(0, 20);
      assert!(
        lines.iter().any(|l| l.contains(mode.lamp())),
        "{mode:?} was clipped off its own status row"
      );
    }
  }

  /// The hint line follows the cursor -- position, and the one verb Enter
  /// means on THIS row -- which is the collapsed foot doing the work the
  /// STATUS and INFO rows used to split between them.
  #[test]
  fn the_hint_line_follows_the_cursor() {
    let r = rows();
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.point_at(r.len());
    let first = screen_for(&app, &r, 60);
    assert!(
      first.hint.contains("1/4"),
      "the hint line does not say where the cursor is: {:?}",
      first.hint
    );

    app.focus = app.focus.map(super::super::focus::Focus::forward);
    app.focus = app.focus.map(super::super::focus::Focus::forward);
    let third = screen_for(&app, &r, 60);
    assert!(third.hint.contains("3/4"));
    assert_ne!(
      first.hint, third.hint,
      "the hint line did not change with the cursor"
    );
    assert!(
      third.hint.contains("$EDITOR"),
      "a prose row must say where Enter takes you: {:?}",
      third.hint
    );
  }

  /// A view with no rows still composes: the chrome is there, the mode is
  /// there, and nothing indexes into an empty list.
  #[test]
  fn an_empty_view_still_carries_its_chrome_and_its_mode() {
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.point_at(0);
    let screen = screen_for(&app, &[], 40);
    let lines = screen.compose(0, 12);
    assert_eq!(lines.len(), 12);
    assert!(lines.iter().any(|l| l.contains(app.mode.lamp())));
    assert!(
      !screen.hint.contains("\u{23ce}"),
      "an empty view must not offer a verb about a row that is absent: {:?}",
      screen.hint
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
      !root.app.contains("\u{232b} back"),
      "the root offered a way back to nowhere"
    );

    app.push(View::Collection {
      kind: "thread".into(),
    });
    let nested = screen_for(&app, &r, 80);
    assert!(
      nested.app.contains("\u{232b} back"),
      "a nested view did not say how to leave"
    );
    assert!(
      nested.app.contains("thread"),
      "a nested view did not say where it is"
    );
  }

  /// The composer line is the one place four different modes each say
  /// something different, so it is asserted as four distinct strings rather
  /// than as "non-empty".
  ///
  /// **MENU IS REACHED BY PRESSING THE KEY, NOT BY ASSIGNING THE MODE, and
  /// that is the point rather than the ceremony.** This test used to set
  /// `app.mode = Menu` with an empty buffer -- a state the machine cannot
  /// produce, since the only door into MENU types the sigil on the way in. It
  /// then failed when MENU stopped having a hardcoded line of its own, which
  /// read as a collision and was really the fixture asserting about a screen
  /// no operator can ever be looking at. **A test that builds an unreachable
  /// state can only tell you about a program that does not exist.**
  #[test]
  fn the_composer_line_says_something_different_in_every_mode_that_uses_it() {
    let mut seen: Vec<String> = Vec::new();
    for mode in [Mode::Omni, Mode::Menu, Mode::Field, Mode::Embed] {
      let mut app = App::explore();
      if mode == Mode::Menu {
        app.commands = super::super::commands::vocabulary();
        assert_eq!(
          app.on_key(key(KeyCode::Char('/')), &[]),
          Step::Continue,
          "`/` did not open the palette"
        );
        assert_eq!(app.mode, Mode::Menu, "`/` did not reach MENU");
      } else {
        app.mode = mode;
      }
      let line = screen_for(&app, &[], 80).omnibox;
      assert!(
        !line.is_empty(),
        "{mode:?} puts nothing on the omnibox line"
      );
      assert!(
        !seen.contains(&line),
        "{mode:?} shares an omnibox line with another mode: {line:?}"
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
  fn the_hint_line_offers_the_crossing_only_where_there_is_one() {
    let rows = vec![
      Row::new("title", "ST0056", "text"),
      Row::new("status", "wip", "select").expanding_to(vec![Row::new("legal", "done", "text")]),
    ];
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.point_at(rows.len());

    let plain = screen_for(&app, &rows, 80).hint;
    assert!(
      !plain.contains("TAB"),
      "a row with no detail offered a crossing: {plain:?}"
    );

    app.focus = app.focus.and_then(|f| f.at(1));
    let offered = screen_for(&app, &rows, 80).hint;
    assert!(
      offered.contains("TAB detail"),
      "a row carrying detail did not say how to reach it: {offered:?}"
    );

    app.wants_detail = true;
    let inside = screen_for(&app, &rows, 80).hint;
    assert!(
      inside.contains("TAB list"),
      "the detail pane did not say how to get back: {inside:?}"
    );
    assert!(
      inside.contains(app.mode.name()),
      "the hint displaced the mode, which is the one thing the hint line must always carry"
    );
  }
  fn key(code: KeyCode) -> crossterm::event::KeyEvent {
    crossterm::event::KeyEvent::new(code, crossterm::event::KeyModifiers::NONE)
  }

  fn esc() -> crossterm::event::KeyEvent {
    key(KeyCode::Esc)
  }

  /// **THE DROPDOWN IS THE OMNIBOX'S HALF OF THE SCREEN CONTRACT**: typing
  /// puts the matches above the input, best first, the pick wearing the
  /// caret and the Selected overlay, matched letters inked [`Role::Match`],
  /// and every line vanishing the moment the mode leaves OMNIBOX -- a
  /// dropdown that outlives its input is a menu nobody opened.
  #[test]
  fn typing_in_the_omnibox_offers_matches_above_the_input_and_nav_clears_them() {
    use super::super::layout::Role;
    use super::super::omnibox::Entry;
    let mut app = App::explore();
    app.index = vec![
      Entry {
        id: "ST0056".into(),
        title: "Add a Rust-based CLI".into(),
        status: "wip".into(),
        door: View::Item {
          kind: "thread".into(),
          id: "ST0056".into(),
        },
      },
      Entry {
        id: "0056".into(),
        title: "an issue sharing digits with a thread".into(),
        status: "open".into(),
        door: View::Item {
          kind: "issue".into(),
          id: "0056".into(),
        },
      },
    ];
    for c in "56".chars() {
      app.on_key(key(KeyCode::Char(c)), &[]);
    }
    let screen = screen_for(&app, &[], 80);
    assert_eq!(
      screen.dropdown.len(),
      2,
      "both carriers of `56` are offered"
    );
    // Best LAST -- nearest the bottom input -- and the best is `0056`, whose
    // earlier first-hit outranks `ST0056` under the stated weights.
    let (best, best_ink) = &screen.dropdown[1];
    assert!(
      best.starts_with("\u{276f} "),
      "the pick wears the caret: {best:?}"
    );
    assert!(best.contains("0056"), "{best:?}");
    assert!(
      best_ink.iter().any(|&(_, _, r)| r == Role::Match),
      "the matched letters are inked so the list is legible: {best_ink:?}"
    );
    assert!(
      best_ink.last().map(|&(_, _, r)| r) == Some(Role::Selected),
      "the pick is overlaid last so it wins the row's own ink"
    );
    assert!(
      screen.dropdown[0].0.contains("ST0056"),
      "the worse match sits farther from the input: {:?}",
      screen.dropdown[0].0
    );

    // The lines reach the composed screen directly above the bottom rule.
    let lines = screen.compose(0, 20);
    // Derived from the declared foot rather than counted back by hand: the
    // bottom rule sits at `20 - foot`, and the dropdown fills the body rows
    // directly above it, best match nearest the input.
    let foot = layout::FOOT + layout::FRAME_COST;
    assert!(
      lines[20 - foot - 1].contains("0056") && lines[20 - foot - 2].contains("ST0056"),
      "the dropdown must sit above the bottom rule, best match nearest the input:\n{lines:#?}"
    );
    // **A RULE DELIMITS THE OFFERS FROM THE BODY** (hv, 2026-09-02, driving
    // it): without one they sit flush against rows they are not part of and
    // read as more body. Two matches here, so the rule is the line above them.
    let rule: String = std::iter::repeat_n(layout::RULE, 80).collect();
    assert_eq!(
      lines[20 - foot - 3],
      rule,
      "a rule must separate the dropdown from the body:\n{lines:#?}"
    );

    // And Esc to NAV clears it.
    let mut app = app;
    app.on_key(esc(), &[]);
    let screen = screen_for(&app, &[], 80);
    assert!(
      screen.dropdown.is_empty(),
      "the dropdown outlived the omnibox"
    );
  }
  /// **A NOTICE THAT NEVER REACHES THE SCREEN IS A SILENT FAILURE WEARING A
  /// GREEN'S CLOTHES** -- driven after a pty run showed three write-path
  /// notices vanishing. The hint line must carry a standing notice in every
  /// mode that can stand one.
  #[test]
  fn a_standing_notice_reaches_the_hint_line() {
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.notice = "title saved".into();
    let hint = screen_for(&app, &[], 80).hint;
    assert!(
      hint.contains("title saved"),
      "the notice did not reach the hint line: {hint:?}"
    );
  }
}
