//! The event loop's state, with no terminal in it.
//!
//! **THE LOOP IS A PURE FUNCTION OF (STATE, KEY) AND THAT IS THE WHOLE POINT.**
//! Everything a realiser usually hides inside `while let Ok(event) = read()`
//! lives here as [`App::on_key`], so the properties that make a modal interface
//! safe -- repeated Esc always terminates, an unbound key changes nothing, quit
//! happens only at the root -- are checked by calling a function rather than by
//! driving a terminal nobody can drive in CI.
//!
//! The pieces this composes are each proved on their own:
//! [`super::keys`] turns a keystroke into a declared trigger,
//! [`super::mode`] answers what that trigger does, [`super::nav`] holds the view
//! stack, [`super::layout`] computes the picture and [`super::draw`] prints it.
//! **This module adds no fifth opinion**; it is the wiring, and its tests are
//! about the wiring.
//!
//! # Repeated Esc always terminates
//!
//! `tui-design.md` §3: *NORMAL is the rest state and ESC always walks toward
//! it -- repeated ESC therefore always terminates, which is the property that
//! makes a modal UI safe to be lost in.* The mode machine proves Esc walks
//! toward NORMAL; [`super::keys`] proves the key reaches the machine from every
//! mode that owns its escape. **Neither of those is the property an operator
//! cares about**, which is that holding Esc gets you OUT -- through modes and
//! then through the whole view stack. That is asserted here, from every mode at
//! every depth, with a bounded number of presses so a loop that never
//! terminates fails rather than hangs.
//!
//! # An unbound key changes NOTHING
//!
//! Not "stays in the same mode" -- changes nothing at all, including the view
//! stack and the scroll. `super::mode::step` returning `None` means *the machine
//! says nothing*, and a realiser that treated it as a self-loop would silently
//! absorb every input the table forgot.

use crossterm::event::KeyEvent;

use super::keys;
use super::mode::{self, Mode};
use super::nav::{Stack, View};

/// What the loop should do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
  Continue,
  /// The operator left the root. **`tui-design.md` §3: at the root, ESC QUITS.**
  Quit,
}

/// The realiser's whole state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
  pub stack: Stack,
  pub mode: Mode,
  /// The first body row on screen.
  pub scroll: usize,
}

impl App {
  /// `intent explore` -- rooted at the entity kinds.
  pub fn explore() -> Self {
    Self {
      stack: Stack::explore(),
      mode: mode::REST,
      scroll: 0,
    }
  }

  /// `intent edit <kind> <id>` -- rooted at one item.
  pub fn at_item(kind: impl Into<String>, id: impl Into<String>) -> Self {
    Self {
      stack: Stack::at_item(kind, id),
      mode: mode::REST,
      scroll: 0,
    }
  }

  /// Feed one keystroke.
  pub fn on_key(&mut self, key: KeyEvent) -> Step {
    // Not a key we bind. **Nothing happens -- not a self-loop, nothing.**
    let Some(trigger) = keys::trigger(self.mode, key) else {
      return Step::Continue;
    };
    // The machine says nothing about this trigger from this mode. Same rule.
    let Some(next) = mode::step(self.mode, trigger) else {
      return Step::Continue;
    };

    // **POPPING THE VIEW STACK IS NORMAL'S JOB AND ONLY NORMAL'S.** Esc from a
    // mode means *leave the mode*; Esc in the rest state means *leave the
    // view*, and at the root that is the quit. Reading the trigger rather than
    // the resulting mode matters because both are self-loops on NORMAL: Esc and
    // Move land in the same place and mean entirely different things.
    if self.mode == mode::REST && matches!(trigger, "Esc" | "Back") {
      let left = self.stack.pop();
      self.scroll = 0;
      if !left {
        // `Back` at the root is a no-op; `Esc` at the root quits. The design
        // gives the two different jobs at exactly this one point.
        return if trigger == "Esc" {
          Step::Quit
        } else {
          Step::Continue
        };
      }
    }

    self.mode = next;
    Step::Continue
  }

  /// Descend into `view`. Cursor and scroll reset with the view, *because a row
  /// index means nothing once the row set changes*.
  pub fn push(&mut self, view: View) {
    self.stack.push(view);
    self.scroll = 0;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crossterm::event::{KeyCode, KeyModifiers};

  fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
  }

  fn esc() -> KeyEvent {
    key(KeyCode::Esc)
  }

  /// Every mode, so the walks below start from all of them rather than from the
  /// rest state that trivially satisfies everything.
  fn from_every_mode(depth: usize) -> Vec<App> {
    Mode::ALL
      .iter()
      .map(|&m| {
        let mut a = App::explore();
        for i in 0..depth {
          a.push(View::Collection {
            kind: format!("k{i}"),
          });
        }
        a.mode = m;
        a
      })
      .collect()
  }

  #[test]
  fn the_corpus_covers_every_mode_and_more_than_one_depth() {
    assert!(
      Mode::ALL.len() > 1,
      "one mode makes every walk below trivial"
    );
    assert_eq!(from_every_mode(3).len(), Mode::ALL.len());
    assert_eq!(from_every_mode(3)[0].stack.depth(), 4);
  }

  /// **THE PROPERTY THAT MAKES A MODAL UI SAFE TO BE LOST IN**, and the one an
  /// operator actually cares about: from anywhere, holding Esc gets you out.
  /// Bounded, so a state that never terminates fails instead of hanging.
  #[test]
  fn repeated_esc_terminates_from_every_mode_that_owns_its_escape() {
    // **THE EXEMPTION IS READ FROM THE MACHINE, NEVER RETYPED HERE.** EMBED is
    // exempt because a child process owns the keyboard, and that fact is
    // already declared once in `mode::ESC_NOT_OURS`. A second copy here would
    // go on exempting EMBED after the machine stopped needing it -- and would
    // exempt whatever else somebody added there without this walk noticing.
    let exempt: Vec<Mode> = mode::ESC_NOT_OURS.iter().map(|(m, _)| *m).collect();
    assert!(
      !exempt.is_empty(),
      "no mode is exempt, so the second half asserts nothing"
    );
    let mut walked = 0usize;
    for depth in 0..4 {
      for mut app in from_every_mode(depth) {
        let started_in = app.mode;
        if exempt.contains(&started_in) {
          // Load-bearing rather than skipped: the exempt mode must ACTUALLY
          // trap, or the exemption is forgiving something that is not there.
          let budget = Mode::ALL.len() + depth + 4;
          for _ in 0..budget {
            assert_eq!(
              app.on_key(esc()),
              Step::Continue,
              "{started_in:?} is declared exempt from owning its escape, and Esc left it anyway \
               -- the exemption is now forgiving a mode that does not need it"
            );
          }
          continue;
        }
        let mut presses = 0;
        let budget = Mode::ALL.len() + depth + 4;
        loop {
          assert!(
            presses < budget,
            "held Esc {presses} times from {started_in:?} at depth {depth} and never quit; the \
             machine or the stack is absorbing escapes"
          );
          presses += 1;
          if app.on_key(esc()) == Step::Quit {
            break;
          }
        }
        walked += 1;
      }
    }
    assert!(
      walked > 0,
      "every mode was exempt, so this test asserted nothing"
    );
  }

  /// EMBED is the exception the design states rather than hides: the child owns
  /// the keyboard, so Esc does NOT walk out of it. Asserted so that the walk
  /// above cannot quietly start passing for the wrong reason if EMBED ever
  /// loses its forwarding.
  #[test]
  fn esc_inside_embed_is_forwarded_and_does_not_leave_the_mode() {
    let mut app = App::explore();
    app.mode = Mode::Embed;
    assert_eq!(app.on_key(esc()), Step::Continue);
    assert_eq!(
      app.mode,
      Mode::Embed,
      "Esc left EMBED, but the child owns that key"
    );
  }

  /// **NOT "STAYS IN THE SAME MODE" -- CHANGES NOTHING.** A realiser that
  /// treated an unbound key as a self-loop would silently absorb every input the
  /// table forgot.
  #[test]
  fn an_unbound_key_changes_nothing_at_all() {
    let unbound = [
      KeyCode::F(5),
      KeyCode::Insert,
      KeyCode::PageUp,
      KeyCode::Home,
    ];
    let mut touched = 0usize;
    for mut app in from_every_mode(2) {
      for code in unbound {
        if keys::trigger(app.mode, key(code)).is_some() {
          continue; // bound here; a different test's business
        }
        let before = app.clone();
        assert_eq!(app.on_key(key(code)), Step::Continue);
        assert_eq!(
          app, before,
          "{code:?} changed the app in {:?} and binds to nothing",
          app.mode
        );
        touched += 1;
      }
    }
    assert!(
      touched > 0,
      "every key swept was bound, so this test asserted nothing"
    );
  }

  /// Quit is reachable only from the root. Anywhere deeper, Esc unwinds one
  /// level and the session continues -- otherwise a nested view would drop the
  /// operator out of the tool.
  #[test]
  fn quit_happens_at_the_root_and_never_above_it() {
    let mut app = App::explore();
    for i in 0..3 {
      app.push(View::Collection {
        kind: format!("k{i}"),
      });
    }
    while !app.stack.at_root() {
      assert_eq!(
        app.on_key(esc()),
        Step::Continue,
        "quit fired at depth {}",
        app.stack.depth()
      );
    }
    assert_eq!(app.on_key(esc()), Step::Quit, "at the root, Esc must quit");
  }

  /// `Back` and `Esc` are the same motion everywhere except the root, where the
  /// design gives them different jobs: Backspace pops history, Esc quits.
  #[test]
  fn back_pops_like_esc_but_does_not_quit_at_the_root() {
    let mut app = App::explore();
    app.push(View::Collection {
      kind: "thread".into(),
    });
    assert_eq!(app.on_key(key(KeyCode::Backspace)), Step::Continue);
    assert!(app.stack.at_root(), "Backspace did not pop the view");
    assert_eq!(
      app.on_key(key(KeyCode::Backspace)),
      Step::Continue,
      "Backspace must not quit"
    );
    assert!(app.stack.at_root());
  }

  /// Cursor and scroll reset with the view, *because a row index means nothing
  /// once the row set changes*.
  #[test]
  fn scroll_resets_when_the_view_changes_in_either_direction() {
    let mut app = App::explore();
    app.scroll = 17;
    app.push(View::Collection {
      kind: "thread".into(),
    });
    assert_eq!(
      app.scroll, 0,
      "descending kept a scroll position from the view above"
    );
    app.scroll = 9;
    app.on_key(esc());
    assert_eq!(
      app.scroll, 0,
      "popping kept a scroll position from the view below"
    );
  }

  /// `explore` and `edit` differ in their root and in nothing else the loop can
  /// see -- the same keys do the same things in both.
  #[test]
  fn explore_and_edit_run_the_same_loop() {
    let mut e = App::explore();
    let mut i = App::at_item("thread", "ST0056");
    assert_eq!(e.mode, i.mode);
    assert_eq!(e.on_key(key(KeyCode::Char(':'))), Step::Continue);
    assert_eq!(i.on_key(key(KeyCode::Char(':'))), Step::Continue);
    assert_eq!(e.mode, Mode::Command);
    assert_eq!(
      i.mode, e.mode,
      "the same key reached a different mode from a different root"
    );
    assert_eq!(e.on_key(esc()), Step::Continue);
    assert_eq!(i.on_key(esc()), Step::Continue);
    assert_eq!(e.mode, mode::REST);
    assert_eq!(i.mode, mode::REST);
    assert_eq!(e.on_key(esc()), Step::Quit, "explore quits from its root");
    assert_eq!(i.on_key(esc()), Step::Quit, "edit quits from its root");
  }
}
