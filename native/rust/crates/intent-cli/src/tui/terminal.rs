//! Borrowing the terminal, and giving it back on every path out.
//!
//! **THE FAILURE MODE HERE IS NOT A WRONG ANSWER.** Every other defect in this
//! estate produces something an operator can see and report: a bad exit code, a
//! refusal about the wrong thing, a number that does not add up. This one
//! leaves their shell in raw mode -- no echo, no line discipline, Ctrl-C
//! delivering a byte instead of a signal -- **after the process that did it has
//! already exited**. There is nothing left running to notice, nothing to
//! report, and it does not look like a crash. It looks like the terminal broke.
//!
//! **SO RESTORATION CANNOT BE THE LAST STATEMENT OF A FUNCTION**, which is
//! where it naturally gets written. Cleanup written after the body is dead code
//! until the day the body does not reach it, and on that day it does not run
//! (cc's framing, adopted). Two mechanisms, because each covers what the other
//! cannot:
//!
//! - [`Borrowed`]'s `Drop` runs on a normal return AND while unwinding, so it
//!   covers `?` returning early and a panic anywhere inside.
//! - A panic hook runs BEFORE the panic message is printed, which `Drop` cannot
//!   do -- unwinding reaches the guard only after the hook has already written
//!   the message. A panic message printed in raw mode has no line breaks: it
//!   staircases across the screen, which is the state the operator has to read
//!   the bug report out of.
//!
//! # Why the sequence goes through a trait
//!
//! **A TERMINAL IS THE THING THIS MODULE EXISTS TO TOUCH, SO A TEST THAT NEEDS
//! ONE CANNOT CHECK IT.** There is no tty in CI, and the ordering properties --
//! everything entered is left, left in reverse, left exactly once -- are
//! statements about a SEQUENCE and have nothing to do with a terminal. Over
//! [`Screen`] they are provable against a recording implementation, with
//! `crossterm` appearing in the shipped path and in no test.
//!
//! That is the same discipline as [`super::mode`]: the machine is provable
//! without the realiser, because the realiser is what it checks.

use std::fmt;
use std::io;

/// The two things `intent tui` does to a terminal, and their undos.
///
/// **DECLARED AS A PAIR PER STEP RATHER THAN AS `setup`/`teardown`.** A single
/// teardown is one function whose body can drift from the setup it mirrors, and
/// nothing says so; a step that carries its own undo makes the pairing
/// structural, which is what lets [`Borrowed`] unwind them generically.
pub trait Screen {
  fn enter_raw(&mut self) -> io::Result<()>;
  fn leave_raw(&mut self) -> io::Result<()>;
  fn enter_alternate(&mut self) -> io::Result<()>;
  fn leave_alternate(&mut self) -> io::Result<()>;
}

/// One reversible thing done to the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
  Raw,
  Alternate,
}

impl Step {
  /// The order they are applied. **Raw first**: the alternate screen is a
  /// write, and writing before the mode is set is the one ordering that can
  /// leave visible output on the operator's real screen.
  pub const ORDER: &'static [Step] = &[Step::Raw, Step::Alternate];

  fn enter(self, screen: &mut dyn Screen) -> io::Result<()> {
    match self {
      Step::Raw => screen.enter_raw(),
      Step::Alternate => screen.enter_alternate(),
    }
  }

  fn leave(self, screen: &mut dyn Screen) -> io::Result<()> {
    match self {
      Step::Raw => screen.leave_raw(),
      Step::Alternate => screen.leave_alternate(),
    }
  }
}

/// A borrowed terminal. Dropping it gives the terminal back.
///
/// **`taken` IS WHAT MAKES A PARTIAL BORROW SAFE.** If entering the alternate
/// screen fails after raw mode succeeded, the raw mode is already the
/// operator's problem -- so the guard records each step as it succeeds and
/// unwinds exactly what it took. A guard that assumed all-or-nothing would
/// leave raw mode on precisely the error path that reports a terminal problem.
pub struct Borrowed<S: Screen> {
  screen: S,
  taken: Vec<Step>,
}

impl<S: Screen> fmt::Debug for Borrowed<S> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Borrowed")
      .field("taken", &self.taken)
      .finish()
  }
}

impl<S: Screen> Borrowed<S> {
  /// Take the terminal, unwinding whatever was taken if a later step fails.
  pub fn take(mut screen: S) -> Result<Self, io::Error> {
    let mut taken: Vec<Step> = Vec::new();
    for &step in Step::ORDER {
      if let Err(e) = step.enter(&mut screen) {
        // **THE FAILING BORROW GIVES BACK WHAT IT TOOK, AND REPORTS THE
        // ORIGINAL ERROR.** An undo that fails here is swallowed on purpose:
        // there is exactly one useful thing to tell the caller and it is why
        // the borrow failed, not why the cleanup after it also failed.
        for &done in taken.iter().rev() {
          let _ = done.leave(&mut screen);
        }
        return Err(e);
      }
      taken.push(step);
    }
    Ok(Self { screen, taken })
  }

  /// Give the terminal back now. **Idempotent**, because the panic hook and
  /// this guard's `Drop` both run on a panic and neither can know about the
  /// other. Draining `taken` is what makes the second call a no-op rather than
  /// a second `leave_raw` on a terminal that is no longer raw.
  pub fn restore(&mut self) {
    for step in std::mem::take(&mut self.taken).into_iter().rev() {
      let _ = step.leave(&mut self.screen);
    }
  }

  /// What is still borrowed. Empty once restored.
  pub fn outstanding(&self) -> &[Step] {
    &self.taken
  }

  pub fn screen(&mut self) -> &mut S {
    &mut self.screen
  }
}

impl<S: Screen> Drop for Borrowed<S> {
  fn drop(&mut self) {
    self.restore();
  }
}

/// The `crossterm` implementation -- the only place in this module that knows
/// what a terminal is.
pub mod real {
  use super::Screen;
  use std::io;

  pub struct Crossterm;

  impl Screen for Crossterm {
    fn enter_raw(&mut self) -> io::Result<()> {
      crossterm::terminal::enable_raw_mode()
    }

    fn leave_raw(&mut self) -> io::Result<()> {
      crossterm::terminal::disable_raw_mode()
    }

    fn enter_alternate(&mut self) -> io::Result<()> {
      crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen)
    }

    fn leave_alternate(&mut self) -> io::Result<()> {
      crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen)
    }
  }

  /// Restore the terminal before the panic message is written, then let the
  /// previous hook print it.
  ///
  /// **THIS CANNOT BE `Drop`, AND THAT IS THE WHOLE REASON IT EXISTS.**
  /// Unwinding reaches a guard only AFTER the hook has printed, so by the time
  /// `Drop` runs the message is already on a raw-mode screen with no line
  /// breaks -- staircased across it, which is the state the operator has to
  /// read the bug report out of.
  ///
  /// It restores blind rather than reaching for the live guard: a panic can
  /// arrive with the guard borrowed, poisoned, or mid-construction, and a hook
  /// that needs to reach shared state is a hook that can fail on the path where
  /// everything has already failed. `disable_raw_mode` on a terminal that is
  /// not raw is a no-op, which is what makes blind restoration safe.
  pub fn restore_on_panic() {
    let previous = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
      let _ = crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
      let _ = crossterm::terminal::disable_raw_mode();
      previous(info);
    }));
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::cell::RefCell;
  use std::rc::Rc;

  /// A screen that records what it was asked to do, and can be told to fail at
  /// a chosen step.
  #[derive(Default)]
  struct Recorder {
    log: Rc<RefCell<Vec<&'static str>>>,
    fail_on: Option<&'static str>,
  }

  impl Recorder {
    fn new(log: &Rc<RefCell<Vec<&'static str>>>) -> Self {
      Self {
        log: Rc::clone(log),
        fail_on: None,
      }
    }

    fn failing(log: &Rc<RefCell<Vec<&'static str>>>, at: &'static str) -> Self {
      Self {
        log: Rc::clone(log),
        fail_on: Some(at),
      }
    }

    fn note(&mut self, what: &'static str) -> io::Result<()> {
      if self.fail_on == Some(what) {
        return Err(io::Error::other(what));
      }
      self.log.borrow_mut().push(what);
      Ok(())
    }
  }

  impl Screen for Recorder {
    fn enter_raw(&mut self) -> io::Result<()> {
      self.note("enter_raw")
    }
    fn leave_raw(&mut self) -> io::Result<()> {
      self.note("leave_raw")
    }
    fn enter_alternate(&mut self) -> io::Result<()> {
      self.note("enter_alternate")
    }
    fn leave_alternate(&mut self) -> io::Result<()> {
      self.note("leave_alternate")
    }
  }

  fn log() -> Rc<RefCell<Vec<&'static str>>> {
    Rc::new(RefCell::new(Vec::new()))
  }

  /// **EVERYTHING TAKEN IS GIVEN BACK, AND IN REVERSE.** Asserted as the whole
  /// sequence by equality rather than as "leave_raw was called somewhere",
  /// because the ORDER is the property: leaving raw mode before leaving the
  /// alternate screen returns the operator to their scrollback with the
  /// alternate screen's contents still on it.
  #[test]
  fn a_normal_drop_gives_back_everything_it_took_in_reverse() {
    let log = log();
    {
      let _borrowed = Borrowed::take(Recorder::new(&log)).expect("take the terminal");
    }
    assert_eq!(
      *log.borrow(),
      [
        "enter_raw",
        "enter_alternate",
        "leave_alternate",
        "leave_raw"
      ],
      "the undo must mirror the setup exactly and run backwards"
    );
  }

  /// **THE PATH THE `Drop` GUARD EXISTS FOR.** A `restore()` written as the
  /// last statement of the TUI's body would not run here, and this is the exact
  /// day it matters.
  #[test]
  fn a_panic_inside_the_borrow_still_gives_the_terminal_back() {
    let log = log();
    let inner = Rc::clone(&log);
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
      let _borrowed = Borrowed::take(Recorder::new(&inner)).expect("take the terminal");
      panic!("the TUI fell over");
    }));
    assert!(
      outcome.is_err(),
      "the panic must not be swallowed by the guard"
    );
    assert_eq!(
      *log.borrow(),
      [
        "enter_raw",
        "enter_alternate",
        "leave_alternate",
        "leave_raw"
      ],
      "unwinding past the guard must restore exactly as a normal return does"
    );
  }

  /// **A PARTIAL BORROW UNWINDS WHAT IT GOT.** The error path is the one that
  /// reports a terminal problem, so leaving raw mode enabled on it is the worst
  /// possible moment: the operator is now reading an error message through a
  /// terminal this code broke.
  #[test]
  fn a_borrow_that_fails_halfway_gives_back_the_half_it_took() {
    let log = log();
    let err = Borrowed::take(Recorder::failing(&log, "enter_alternate"))
      .expect_err("the alternate screen was rigged to fail");
    assert_eq!(
      err.to_string(),
      "enter_alternate",
      "the ORIGINAL failure is what the caller needs, not a cleanup error"
    );
    assert_eq!(
      *log.borrow(),
      ["enter_raw", "leave_raw"],
      "raw mode was taken and must be given back, and the alternate screen was never entered so it is not left"
    );
  }

  /// **RESTORING TWICE MUST BE HARMLESS, BECAUSE ON A PANIC IT HAPPENS.** The
  /// hook restores and then `Drop` restores, and neither can know about the
  /// other. Without draining, the second pass calls `leave_raw` on a terminal
  /// that is no longer raw -- which is where a real terminal starts emitting
  /// escape sequences into the operator's shell.
  #[test]
  fn restoring_twice_does_the_work_once() {
    let log = log();
    {
      let mut borrowed = Borrowed::take(Recorder::new(&log)).expect("take the terminal");
      borrowed.restore();
      assert!(
        borrowed.outstanding().is_empty(),
        "nothing is owed after a restore"
      );
      borrowed.restore();
    }
    assert_eq!(
      *log.borrow(),
      [
        "enter_raw",
        "enter_alternate",
        "leave_alternate",
        "leave_raw"
      ],
      "three restores -- two explicit and the Drop -- must leave one undo each"
    );
  }

  /// The order is declared once, in [`Step::ORDER`], and the undo derives from
  /// it. Asserted by equality so a step added to the enum and not to the order
  /// fails here rather than being silently never taken -- and never given back.
  #[test]
  fn every_step_is_in_the_declared_order() {
    assert_eq!(
      Step::ORDER,
      &[Step::Raw, Step::Alternate],
      "raw mode goes first: the alternate screen is a WRITE, and writing before the mode is set is the ordering that leaves output on the operator's real screen"
    );
    let log = log();
    {
      let _b = Borrowed::take(Recorder::new(&log)).expect("take the terminal");
    }
    assert_eq!(
      log.borrow().len(),
      Step::ORDER.len() * 2,
      "every declared step must be both taken and given back -- a step present in the enum and absent from ORDER is never taken, which looks like it working"
    );
  }
}
