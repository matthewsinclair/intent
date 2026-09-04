//! The window between the command and the first frame.
//!
//! **`intent explore` LOOKS HUNG BEFORE IT LOOKS SLOW.** The operator types a
//! command and gets nothing: no echo, no cursor movement, no output. On a
//! heavily-written store that silence has been measured at five seconds, and
//! silence is the one response a terminal cannot distinguish from a crash.
//!
//! # Why this is not a spinner inside the TUI
//!
//! **MEASURED 2026-09-04, ACROSS FIVE PROJECTS AND EVERY RUN: the gap between
//! the terminal being taken and the first frame landing is ONE MILLISECOND** --
//! including on a run that took 4,916 ms to get there. Almost the whole wait
//! happens in `explore()` BEFORE [`super::terminal::Borrowed::take`], while
//! opening the facade, loading the form declaration and building the dispatch
//! table.
//!
//! So a spinner painted by the TUI would be visible for about a millisecond.
//! **The indicator has to live on the command line, before the screen is
//! taken**, which is also where the operator is already looking.
//!
//! **AND MOVING THE LOAD AFTER THE TAKE WOULD COST SOMETHING DELIBERATE.**
//! `explore()` resolves its address before taking the terminal precisely so a
//! spelling it cannot read is reported on a screen the operator can read,
//! rather than behind a raw-mode switch.
//!
//! # The threshold is the whole design
//!
//! **A project that starts in 10 ms must show NOTHING.** An indicator that
//! flashes on every invocation is worse than no indicator: it trains the
//! operator to ignore it, and it makes a fast tool look busy. Baize starts in
//! ~10 ms and Conflab in ~83 ms; Laksa takes seconds. [`THRESHOLD`] is what
//! separates them, and it is the reason this module is mostly a decision
//! rather than a renderer.

use std::time::Duration;

/// How long a load may take before the operator is told anything.
///
/// **150 ms, AND THE NUMBER IS A MEASUREMENT RATHER THAN A FEEL.** Startup on
/// this estate clusters at two scales: projects that reach the first frame in
/// 10-260 ms, and projects that take over a second. The threshold sits above
/// the whole of the first cluster, so the common case shows nothing at all.
///
/// **IT IS DELIBERATELY NOT TUNED TO THE HUMAN PERCEPTION FIGURE** (~100 ms,
/// where a delay stops feeling instant). That would be the right number if the
/// question were *when does this feel slow*; the question here is *when is a
/// project actually in the slow cluster*, and answering the second one with the
/// first would put an indicator on every healthy project on the estate.
pub const THRESHOLD: Duration = Duration::from_millis(150);

/// How often the line is redrawn once it is showing.
///
/// **SLOW ON PURPOSE.** The dots exist to prove the process is alive, and four
/// redraws a second does that; anything faster spends the operator's terminal
/// bandwidth to look busy. It also bounds the damage if the line is ever drawn
/// somewhere it should not be.
pub const TICK: Duration = Duration::from_millis(250);

/// The mark. **Two columns wide in every terminal that renders it, and the
/// dots are what actually carry the liveness** -- a glyph that fails to render
/// degrades to a box and the line still reads as progress.
const MARK: &str = "\u{1F422}";

/// The most dots drawn before they wrap back to one.
///
/// **A BOUND RATHER THAN A COUNTER.** An unbounded run of dots is a five-second
/// load writing forty characters and wrapping the operator's line, and the
/// wrapped line is then impossible to erase with a single carriage return.
const MAX_DOTS: usize = 4;

/// What the operator should see after `elapsed`, or `None` for nothing at all.
///
/// **PURE, AND THAT IS THE POINT: THE DECISION IS PROVABLE WITHOUT A
/// TERMINAL.** This is the same discipline [`super::terminal`] states for the
/// borrow sequence -- the machine is checkable without the realiser, because
/// the realiser is the thing under test everywhere else.
pub fn line(elapsed: Duration) -> Option<String> {
  if elapsed < THRESHOLD {
    return None;
  }
  let ticks = (elapsed - THRESHOLD).as_millis() / TICK.as_millis();
  let dots = (ticks as usize % MAX_DOTS) + 1;
  Some(format!("{MARK} {}", ".".repeat(dots)))
}

/// The bytes that erase whatever [`line`] last wrote.
///
/// **CARRIAGE RETURN PLUS SPACES PLUS CARRIAGE RETURN, NOT A CLEAR-LINE ESCAPE.**
/// This runs before the terminal is taken, so it must not assume the terminal
/// honours any particular control sequence -- and it must leave the cursor at
/// column zero for whatever prints next, which on the error path is a refusal
/// the operator has to read.
pub fn erase(width: usize) -> String {
  format!("\r{}\r", " ".repeat(width))
}

/// The widest line [`line`] can produce, for [`erase`].
///
/// **DERIVED FROM THE SAME CONSTANTS THE RENDERER USES**, so the eraser cannot
/// drift narrower than the thing it erases and leave dots on the screen.
pub fn widest() -> usize {
  // The mark renders two columns, plus a space, plus the dots.
  2 + 1 + MAX_DOTS
}

/// What [`while_loading`] settled on.
pub enum Outcome<T> {
  Done(T),
  /// The operator asked for the load to stop before it finished.
  Cancelled,
}

/// The indicator's state across a load: what to draw next, and what to undo.
///
/// **PURE, AND IT HOLDS THE TWO PROPERTIES THE LOOP AROUND IT CANNOT BE TRUSTED
/// WITH.** A fast load must leave the terminal byte-for-byte untouched -- not
/// even an erase sequence -- and a line must not be rewritten while it says the
/// same thing. Both are decisions about what has already been shown, so they
/// belong to something that remembers rather than to a loop that ticks.
#[derive(Default)]
pub struct Ticker {
  last: Option<String>,
}

impl Ticker {
  pub fn new() -> Self {
    Self::default()
  }

  /// The bytes to write now, or `None` when the screen is already correct.
  pub fn frame(&mut self, elapsed: Duration) -> Option<String> {
    let rendered = line(elapsed)?;
    if self.last.as_deref() == Some(rendered.as_str()) {
      return None;
    }
    self.last = Some(rendered.clone());
    Some(format!("\r{rendered}"))
  }

  /// The bytes that take the indicator back off the screen, or `None` if
  /// nothing was ever put there.
  ///
  /// **`None` IS THE COMMON CASE AND THE IMPORTANT ONE.** Every healthy project
  /// on this estate finishes under [`THRESHOLD`], so the overwhelmingly usual
  /// outcome is that this whole module writes nothing at all.
  pub fn clear(&mut self) -> Option<String> {
    self.last.take()?;
    Some(erase(widest()))
  }
}

/// How often the keyboard is checked while a load is running.
///
/// **SHORTER THAN [`TICK`] BECAUSE IT ANSWERS A DIFFERENT QUESTION.** `TICK`
/// paces what the operator SEES; this paces how quickly a keypress is noticed.
/// Tying them together would make a cancel take up to a quarter second to
/// register, which reads as the key not having worked.
const POLL: Duration = Duration::from_millis(50);

/// Should this event stop the load?
///
/// **`Ctrl-C` IS HERE BECAUSE RAW MODE IS WHAT TOOK IT AWAY**, and leaving it
/// out would be a real regression rather than a missing nicety. Outside raw
/// mode the line discipline turns `Ctrl-C` into `SIGINT` and the operator can
/// always abandon a slow load; the moment this module enters raw mode to watch
/// for `Esc`, that stops happening and `Ctrl-C` arrives as an ordinary key
/// event. An indicator that made the wait UNINTERRUPTIBLE would be worse than
/// the silence it replaced.
pub fn is_cancel(event: &crossterm::event::Event) -> bool {
  use crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers};
  let Event::Key(key) = event else {
    return false;
  };
  // A release is the same physical press arriving a second time on terminals
  // that report both, so acting on it would cancel on the key going UP after a
  // press this loop had already handled.
  if key.kind == KeyEventKind::Release {
    return false;
  }
  key.code == KeyCode::Esc
    || (key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL))
}

/// Run `work`, showing the indicator if it outlasts [`THRESHOLD`], and letting
/// `Esc` or `Ctrl-C` abandon it.
///
/// **THE WORK RUNS ON THE THREAD AND THE WATCHING HAPPENS HERE**, which is the
/// way round that makes cancelling possible at all. The load is a single
/// synchronous call into the store; nothing can interrupt it from outside, so
/// the only way `Esc` can end the wait is for the waiting to be what the main
/// thread is doing.
///
/// **A CANCELLED LOAD ABANDONS ITS THREAD ON PURPOSE.** The work is read-only
/// -- opening the facade, loading the form declaration, building the dispatch
/// table -- so there is nothing to roll back and nothing a half-finished read
/// can corrupt. Waiting for it to notice would defeat the whole point, since
/// the load being slow is why the operator pressed the key.
///
/// **IF THE TERMINAL WILL NOT GO RAW, THE LOAD STILL HAPPENS.** An indicator is
/// a courtesy; refusing to open the explorer because a decoration could not be
/// drawn would turn a cosmetic problem into an outage.
pub fn while_loading<T, W>(work: W) -> Outcome<T>
where
  W: FnOnce() -> T + Send + 'static,
  T: Send + 'static,
{
  use super::terminal::{Borrowed, Step, real::Crossterm};
  use std::io::Write;
  use std::sync::mpsc::{self, TryRecvError};

  let (tx, rx) = mpsc::channel();
  let worker = std::thread::spawn(move || {
    let _ = tx.send(work());
  });

  let Ok(_borrowed) = Borrowed::take_only(Crossterm, &[Step::Raw]) else {
    return Outcome::Done(collect(worker, &rx));
  };

  // **THE POLL IS THE TIMER, BECAUSE THIS WORKSPACE HAS NO CLOCK IN IT** (D42,
  // and `intentsvcs/tests/one_clock.rs` enforces it -- `Instant::now` is on the
  // banned list by name, with an exempt list documented as having to stay
  // empty). Each timed-out poll is [`POLL`] of waiting, so counting them
  // measures the wait without asking anything what time it is.
  //
  // **IT UNDER-READS WHEN KEYS ARRIVE, AND THAT IS THE SAFE DIRECTION.** A poll
  // cut short by a keypress contributes nothing, so an operator typing during a
  // load delays the indicator rather than summoning one on a fast project. The
  // cancel check does not go through this estimate at all -- `Esc` is tested on
  // every iteration -- so the degradation is cosmetic and the responsive half
  // is exact.
  let mut waited = Duration::ZERO;
  let mut ticker = Ticker::new();
  let mut out = std::io::stderr();
  let mut show = |what: Option<String>| {
    if let Some(bytes) = what {
      let _ = out.write_all(bytes.as_bytes());
      let _ = out.flush();
    }
  };

  loop {
    match rx.try_recv() {
      Ok(loaded) => {
        show(ticker.clear());
        return Outcome::Done(loaded);
      }
      // The worker is gone without a value, so it panicked. Take the indicator
      // off the screen and give the terminal back BEFORE the panic is re-raised
      // -- a backtrace printed into raw mode staircases across the screen,
      // which is the state the operator has to read the bug report out of.
      Err(TryRecvError::Disconnected) => {
        show(ticker.clear());
        drop(_borrowed);
        return Outcome::Done(collect(worker, &rx));
      }
      Err(TryRecvError::Empty) => {}
    }

    match crossterm::event::poll(POLL) {
      Ok(true) => {
        if let Ok(event) = crossterm::event::read() {
          if is_cancel(&event) {
            show(ticker.clear());
            return Outcome::Cancelled;
          }
        }
      }
      // A poll that timed out waited the whole interval; one that failed is a
      // terminal that has stopped answering, and counting it keeps the loop
      // from becoming a busy spin that never reaches the threshold.
      _ => waited += POLL,
    }

    show(ticker.frame(waited));
  }
}

/// Wait for the worker's value, re-raising its panic if that is what it left.
///
/// **THE PANIC IS RE-RAISED RATHER THAN REPORTED**, so moving the load onto a
/// thread does not change what a bug in it looks like. A load that used to
/// abort the process with a backtrace must still do that; swallowing it here
/// would turn every future panic in `open()` into a silent wrong answer.
fn collect<T>(worker: std::thread::JoinHandle<()>, rx: &std::sync::mpsc::Receiver<T>) -> T {
  match rx.recv() {
    Ok(loaded) => loaded,
    Err(_) => match worker.join() {
      Err(panic) => std::panic::resume_unwind(panic),
      Ok(()) => unreachable!("the worker returned without sending and without panicking"),
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn nothing_is_shown_before_the_threshold() {
    assert_eq!(line(Duration::ZERO), None);
    assert_eq!(
      line(Duration::from_millis(10)),
      None,
      "Baize starts in ~10ms"
    );
    assert_eq!(
      line(Duration::from_millis(83)),
      None,
      "Conflab starts in ~83ms"
    );
    assert_eq!(
      line(Duration::from_millis(149)),
      None,
      "one millisecond under the threshold is still silence"
    );
  }

  #[test]
  fn the_indicator_appears_exactly_at_the_threshold() {
    assert_eq!(line(THRESHOLD), Some(format!("{MARK} .")));
  }

  #[test]
  fn the_dots_advance_with_time_and_wrap_rather_than_growing_without_bound() {
    let at = |ms: u64| line(Duration::from_millis(ms)).expect("past the threshold");
    assert_eq!(at(150), format!("{MARK} ."));
    assert_eq!(at(400), format!("{MARK} .."));
    assert_eq!(at(650), format!("{MARK} ..."));
    assert_eq!(at(900), format!("{MARK} ...."));
    // **THE WRAP IS THE ASSERTION THAT MATTERS.** A five-second load must not
    // write twenty dots across the operator's line.
    assert_eq!(at(1150), format!("{MARK} ."), "the run must wrap, not grow");
    assert_eq!(at(5000), format!("{MARK} ...."));
  }

  #[test]
  fn no_line_is_ever_wider_than_the_eraser() {
    // Driven across the whole first ten seconds rather than at a few points,
    // because `erase` leaving one dot behind is exactly the defect that would
    // survive a spot check.
    for ms in (0..10_000).step_by(7) {
      if let Some(rendered) = line(Duration::from_millis(ms)) {
        // The mark occupies two columns and every other character one, so the
        // width is the char count plus one for the mark's second column.
        let columns = rendered.chars().count() + 1;
        assert!(
          columns <= widest(),
          "at {ms}ms the line is {columns} columns and the eraser clears {}",
          widest()
        );
      }
    }
  }

  /// **THE LITERAL, BECAUSE THE VERSION WRITTEN IN TERMS OF `THRESHOLD` WAS
  /// DECORATION AND A MUTATION PROVED IT.** The first form asserted
  /// `line(THRESHOLD - 1ms)` is `None` and `line(THRESHOLD)` is `Some`, which
  /// is true for EVERY value the constant could hold -- so when the threshold
  /// was mutated from 150 ms to 50 ms it passed, while the test that named a
  /// real number caught it. **A test phrased in the terms of the thing it
  /// checks cannot detect a change to it.**
  ///
  /// The number is here rather than only in the constant because the constant
  /// is a MEASUREMENT (the gap between this estate's fast and slow startup
  /// clusters). Changing it should require changing a test that says so.
  #[test]
  fn the_threshold_is_150ms_and_a_change_to_it_must_break_a_test() {
    assert_eq!(THRESHOLD, Duration::from_millis(150));
    assert_eq!(line(Duration::from_millis(149)), None);
    assert!(line(Duration::from_millis(150)).is_some());
  }
}

#[cfg(test)]
mod ticker_tests {
  use super::*;
  use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};

  /// **THE PROPERTY THE WHOLE MODULE EXISTS TO PRESERVE.** After the store fix
  /// every project on this estate loads in tens of milliseconds, so this is not
  /// an edge case -- it is what happens on essentially every invocation. A
  /// single stray carriage return here would put a cosmetic defect on every
  /// `intent explore` on every machine.
  #[test]
  fn a_load_that_beats_the_threshold_writes_nothing_at_all() {
    let mut ticker = Ticker::new();
    for ms in [0, 1, 10, 54, 83, 149] {
      assert_eq!(
        ticker.frame(Duration::from_millis(ms)),
        None,
        "{ms}ms is inside the threshold and must draw nothing"
      );
    }
    assert_eq!(
      ticker.clear(),
      None,
      "nothing was drawn, so nothing may be erased -- not even an empty erase"
    );
  }

  /// A 50 ms poll against a 250 ms tick means the loop asks five times per
  /// visible change. Redrawing on every ask is five times the terminal traffic
  /// for the same picture.
  #[test]
  fn the_line_is_written_only_when_it_changes() {
    let mut ticker = Ticker::new();
    assert!(
      ticker.frame(Duration::from_millis(150)).is_some(),
      "the first frame past the threshold has to be drawn"
    );
    for ms in [160, 200, 250, 300, 399] {
      assert_eq!(
        ticker.frame(Duration::from_millis(ms)),
        None,
        "at {ms}ms the line still reads the same, so it must not be rewritten"
      );
    }
    assert!(
      ticker.frame(Duration::from_millis(400)).is_some(),
      "the second dot is a real change and has to be drawn"
    );
  }

  #[test]
  fn a_frame_is_written_from_column_zero_and_erased_exactly_once() {
    let mut ticker = Ticker::new();
    let drawn = ticker.frame(Duration::from_millis(150)).expect("drawn");
    assert!(
      drawn.starts_with('\r'),
      "the line has to start at column zero or it walks across the prompt: {drawn:?}"
    );
    assert_eq!(ticker.clear(), Some(erase(widest())));
    assert_eq!(
      ticker.clear(),
      None,
      "clearing twice must be a no-op -- the loop and a later error path can both reach it"
    );
  }

  fn press(code: KeyCode, modifiers: KeyModifiers) -> Event {
    Event::Key(KeyEvent::new(code, modifiers))
  }

  #[test]
  fn escape_stops_the_load() {
    assert!(is_cancel(&press(KeyCode::Esc, KeyModifiers::NONE)));
  }

  /// **THE REGRESSION GUARD, NOT A NICETY.** Entering raw mode to watch for
  /// `Esc` is what takes `SIGINT` away; without this arm the indicator would
  /// make a slow load impossible to abandon, which is strictly worse than the
  /// silence it replaced.
  #[test]
  fn ctrl_c_stops_the_load_because_raw_mode_took_sigint_away() {
    assert!(is_cancel(&press(KeyCode::Char('c'), KeyModifiers::CONTROL)));
  }

  #[test]
  fn ordinary_typing_does_not_stop_the_load() {
    assert!(!is_cancel(&press(KeyCode::Char('c'), KeyModifiers::NONE)));
    assert!(!is_cancel(&press(KeyCode::Char('q'), KeyModifiers::NONE)));
    assert!(!is_cancel(&press(KeyCode::Enter, KeyModifiers::NONE)));
    assert!(!is_cancel(&Event::Resize(80, 24)));
  }

  /// Terminals that report releases send the same physical press twice. Acting
  /// on the second one cancels on the key coming back UP, after this loop has
  /// already handled it going down.
  #[test]
  fn a_key_release_is_not_a_second_press() {
    let release = Event::Key(KeyEvent {
      code: KeyCode::Esc,
      modifiers: KeyModifiers::NONE,
      kind: KeyEventKind::Release,
      state: KeyEventState::NONE,
    });
    assert!(!is_cancel(&release));
  }
}
