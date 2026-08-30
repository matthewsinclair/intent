//! The event loop state, with no terminal in it: `AT-17.9` in part, covering the ESC half of `AC-17.9`.
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

use crossterm::event::{KeyCode, KeyEvent};

use super::edit::Handoff;
use super::focus::Focus;
use super::keys;
use super::layout::Row;
use super::mode::{self, Mode};
use super::nav::{Stack, View};

/// What the loop should do next.
///
/// **[`Step::Hand`] IS A REQUEST, NOT AN ACTION.** `on_key` is a pure function
/// of (state, key) and launching `$EDITOR` is neither pure nor a decision the
/// state machine is entitled to make on its own -- it needs a terminal to lend
/// and a store to write. So the app SAYS what the keystroke asked for and
/// [`super::run`] does it, which is what keeps every property below provable by
/// calling a function.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Step {
  Continue,
  /// The operator left the root. **`tui-design.md` §3: at the root, ESC QUITS.**
  Quit,
  /// `AC-17.10`: hand this field to `$VISUAL`/`$EDITOR`.
  Hand(Handoff),
}

/// Which pane the cursor is in.
///
/// **A GUARD ON NORMAL'S EDGES, NOT A SIXTH MODE.** [`super::keys`] already
/// says so by refusing `Tab` a trigger: crossing panes changes where `Move` and
/// `Enter` LAND, never what the keys mean, so it is the app's state and not the
/// machine's. A `Mode::Detail` would have to duplicate every NORMAL edge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Pane {
  #[default]
  List,
  Detail,
}

/// The realiser's whole state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct App {
  pub stack: Stack,
  pub mode: Mode,
  /// The first body row on screen.
  pub scroll: usize,
  /// Which row the cursor is on. `None` for a view with no rows -- an empty
  /// form has no focus rather than a focus on nothing (`AC-17.5`).
  pub focus: Option<Focus>,
  /// Where the operator has ASKED to be. **The pane they are actually in is
  /// [`App::pane`], which is derived** -- see there.
  pub wants_detail: bool,
  /// The cursor inside the detail pane.
  pub detail_focus: Option<Focus>,
  /// News from the last thing that happened, for the INFO row. Empty most of
  /// the time.
  ///
  /// **AN EDITOR HANDOFF IS THE ONE ACTION AN OPERATOR CANNOT SEE THE RESULT
  /// OF.** Every other keystroke changes the screen; this one gives the
  /// terminal away, comes back, and repaints something that looks identical
  /// whether the save landed, was declined, or was refused by the store. A
  /// silent return is indistinguishable from a silent failure.
  pub notice: String,
}

impl App {
  /// `intent explore` -- rooted at the entity kinds.
  pub fn explore() -> Self {
    Self {
      stack: Stack::explore(),
      mode: mode::REST,
      scroll: 0,
      focus: None,
      wants_detail: false,
      detail_focus: None,
      notice: String::new(),
    }
  }

  /// `intent edit <kind> <id>` -- rooted at one item.
  pub fn at_item(kind: impl Into<String>, id: impl Into<String>) -> Self {
    Self {
      stack: Stack::at_item(kind, id),
      mode: mode::REST,
      scroll: 0,
      focus: None,
      wants_detail: false,
      detail_focus: None,
      notice: String::new(),
    }
  }

  /// The row under the cursor, if there is one.
  pub fn focused_row<'r>(&self, rows: &'r [Row]) -> Option<&'r Row> {
    self.focus.and_then(|f| rows.get(f.index()))
  }

  /// The pane the cursor is in.
  ///
  /// **DERIVED, NEVER STORED, AND THAT IS THE WHOLE INVARIANT.** The detail
  /// pane exists only while the selected row carries detail, so a STORED pane
  /// would go stale the instant the cursor moved to a row without any -- and
  /// the operator would be typing into a pane that is not on the screen, with
  /// nothing saying so. `wants_detail` records what they ASKED for; this
  /// answers what they GOT, and the two can only disagree in the safe
  /// direction.
  pub fn pane(&self, rows: &[Row]) -> Pane {
    if self.wants_detail && self.focused_row(rows).is_some_and(Row::has_detail) {
      Pane::Detail
    } else {
      Pane::List
    }
  }

  /// The cursor of whichever pane is active.
  pub fn cursor(&self, rows: &[Row]) -> Option<Focus> {
    match self.pane(rows) {
      Pane::List => self.focus,
      Pane::Detail => self.detail_focus,
    }
  }

  /// `Tab`: cross to the other pane, if there is one.
  ///
  /// **A TAB WITH NOWHERE TO GO CHANGES NOTHING** -- the same rule as an
  /// unbound key, and for the same reason: a self-loop that silently absorbs
  /// the keystroke teaches the operator the key does not work rather than that
  /// this row has no detail.
  fn cross_panes(&mut self, rows: &[Row]) {
    let Some(detail) = self
      .focused_row(rows)
      .and_then(|r| r.detail.as_ref())
      .filter(|d| !d.is_empty())
    else {
      return;
    };
    if self.wants_detail {
      self.wants_detail = false;
    } else {
      self.wants_detail = true;
      self.detail_focus = Focus::first(detail.len());
    }
  }

  /// Feed one keystroke.
  ///
  /// **`rows` IS AN ARGUMENT BECAUSE THE MACHINE DECLARES AN AMBIGUITY ONLY A
  /// ROW CAN SETTLE.** `tui-design.md` section 3 gives `NORMAL + Enter` two
  /// arms -- FIELD for an editable row, EMBED for a prose one -- and
  /// [`mode::step`] picks between them with `.find()`, which is TABLE ORDER.
  /// The machine's own test says as much in its message. Passing the rows in is
  /// what lets [`mode::arm`] answer from the declared discriminator instead.
  pub fn on_key(&mut self, key: KeyEvent, rows: &[Row]) -> Step {
    // **`Tab` IS A PANE GUARD AND NOT A MODE TRIGGER**, which [`super::keys`]
    // declares by refusing it one. It is answered here, ahead of the machine,
    // because the machine has nothing to say about it -- and only from the rest
    // state, because in FIELD and COMMAND the key belongs to the text being
    // typed and in EMBED it belongs to the child.
    if self.mode == mode::REST && matches!(key.code, KeyCode::Tab | KeyCode::BackTab) {
      self.cross_panes(rows);
      return Step::Continue;
    }
    // Not a key we bind. **Nothing happens -- not a self-loop, nothing.**
    let Some(trigger) = keys::trigger(self.mode, key) else {
      return Step::Continue;
    };
    // The machine says nothing about this trigger from this mode. Same rule --
    // and `arm` says nothing for an ambiguity no row kind resolves, which is
    // the same answer for the same reason.
    let row_kind = self
      .focused_row(rows)
      .map(|r| r.kind.as_str())
      .unwrap_or("");
    let Some(next) = mode::arm(&mode::steps(self.mode, trigger), row_kind) else {
      return Step::Continue;
    };

    // **THE HANDOFF LEAVES AS A REQUEST AND CHANGES NOTHING ELSE.** It needs a
    // field to write back to, so it can only be asked for on an ITEM view; a
    // prose row cannot occur anywhere else, and a realiser that guessed an
    // address here would write to whatever it guessed.
    if next == Mode::Embed && self.mode == mode::REST {
      let (Some(View::Item { kind, id }), Some(row)) = (
        Some(self.stack.current().clone()).filter(|v| matches!(v, View::Item { .. })),
        self.focused_row(rows),
      ) else {
        return Step::Continue;
      };
      let hand = Handoff {
        kind,
        id,
        field: row.name.clone(),
      };
      self.mode = next;
      return Step::Hand(hand);
    }

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

    // **DIRECTION IS THE APP'S BUSINESS AND THE MACHINE'S IGNORANCE IS
    // DELIBERATE.** `EDGES` says NORMAL + Move stays in NORMAL, and that is all
    // it should say: up and down are the same MODE transition and different
    // motions, so folding direction into the trigger vocabulary would put four
    // near-identical self-loops in a table whose whole value is being readable
    // as a graph. The app has the keystroke and reads it here.
    if trigger == "Move" {
      let step: fn(Focus) -> Focus = match key.code {
        KeyCode::Up | KeyCode::Left => Focus::back,
        KeyCode::Down | KeyCode::Right => Focus::forward,
        _ => return Step::Continue,
      };
      // **THE ARROWS MOVE THE PANE THE CURSOR IS IN**, which is the whole
      // meaning of section 4's *move within the focused pane*.
      match self.pane(rows) {
        Pane::Detail => self.detail_focus = self.detail_focus.map(step),
        Pane::List => {
          self.focus = self.focus.map(step);
          // **LEAVING THE ROW LEAVES ITS PANE.** The detail belonged to the row
          // that opened it, so carrying the request onto the next row would
          // reopen a pane over somebody else's detail -- or over none.
          self.wants_detail = false;
          self.detail_focus = None;
        }
      }
    }

    self.mode = next;
    Step::Continue
  }

  /// Point the cursor at a view of `n` rows.
  ///
  /// **A ROW COUNT CHANGE RESETS THE CURSOR RATHER THAN CLAMPING IT**, for the
  /// reason the design gives for the scroll: *a row index means nothing once
  /// the row set changes*. Clamping keeps a number that no longer refers to
  /// anything the operator chose.
  pub fn point_at(&mut self, n: usize) {
    self.focus = Focus::first(n);
    self.scroll = 0;
    self.wants_detail = false;
    self.detail_focus = None;
  }

  /// Re-read the same view: keep the cursor where the operator left it.
  ///
  /// **THE DESIGN RESETS THE CURSOR WITH THE VIEW, AND THIS IS NOT A NEW VIEW.**
  /// `tui-design.md` section 6: *a row index means nothing once the row set
  /// changes* -- which is a statement about the row SET, not about the model
  /// behind it. Saving a field and being returned to the top of the form is the
  /// sort of thing that makes an operator stop using a form.
  pub fn refocus(&mut self, n: usize) {
    match self
      .focus
      .map(Focus::index)
      .and_then(|at| Focus::first(n)?.at(at))
    {
      Some(kept) => self.focus = Some(kept),
      None => self.point_at(n),
    }
  }

  /// The child that owned the terminal has gone.
  ///
  /// **THE MACHINE SAYS WHERE THIS LANDS, NOT THIS FUNCTION.** `EMBED +
  /// ChildExit -> NORMAL` is a declared edge with the note *read the file
  /// back*; spelling `self.mode = Mode::Normal` here would be a second copy of
  /// a transition the table already owns.
  pub fn child_exited(&mut self) {
    if let Some(next) = mode::step(self.mode, "ChildExit") {
      self.mode = next;
    }
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
  use super::super::nav::Stack;
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
              app.on_key(esc(), &[]),
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
          if app.on_key(esc(), &[]) == Step::Quit {
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
    assert_eq!(app.on_key(esc(), &[]), Step::Continue);
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
        assert_eq!(app.on_key(key(code), &[]), Step::Continue);
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
        app.on_key(esc(), &[]),
        Step::Continue,
        "quit fired at depth {}",
        app.stack.depth()
      );
    }
    assert_eq!(
      app.on_key(esc(), &[]),
      Step::Quit,
      "at the root, Esc must quit"
    );
  }

  /// `Back` and `Esc` are the same motion everywhere except the root, where the
  /// design gives them different jobs: Backspace pops history, Esc quits.
  #[test]
  fn back_pops_like_esc_but_does_not_quit_at_the_root() {
    let mut app = App::explore();
    app.push(View::Collection {
      kind: "thread".into(),
    });
    assert_eq!(app.on_key(key(KeyCode::Backspace), &[]), Step::Continue);
    assert!(app.stack.at_root(), "Backspace did not pop the view");
    assert_eq!(
      app.on_key(key(KeyCode::Backspace), &[]),
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
    app.on_key(esc(), &[]);
    assert_eq!(
      app.scroll, 0,
      "popping kept a scroll position from the view below"
    );
  }

  /// **THE CURSOR MOVES BOTH WAYS AND WRAPS**, which is `AC-17.5` reaching the
  /// loop: the focus module proves the walk is total and reversible, and this
  /// proves the ARROW KEYS drive it. A realiser can satisfy the first and fail
  /// the second by wiring both arrows to `forward`.
  #[test]
  fn the_arrows_move_the_cursor_in_opposite_directions_and_wrap() {
    let mut app = App::explore();
    app.point_at(3);
    assert_eq!(app.focus.map(Focus::index), Some(0));
    app.on_key(key(KeyCode::Down), &[]);
    assert_eq!(
      app.focus.map(Focus::index),
      Some(1),
      "Down did not advance the cursor"
    );
    app.on_key(key(KeyCode::Up), &[]);
    assert_eq!(app.focus.map(Focus::index), Some(0), "Up did not undo Down");
    app.on_key(key(KeyCode::Up), &[]);
    assert_eq!(
      app.focus.map(Focus::index),
      Some(2),
      "the cursor did not wrap backwards"
    );
    app.on_key(key(KeyCode::Down), &[]);
    assert_eq!(
      app.focus.map(Focus::index),
      Some(0),
      "the cursor did not wrap forwards"
    );
  }

  /// A view with no rows has no cursor -- not a cursor on row zero of nothing.
  #[test]
  fn an_empty_view_has_no_cursor_and_moving_does_not_invent_one() {
    let mut app = App::explore();
    app.point_at(0);
    assert_eq!(app.focus, None);
    app.on_key(key(KeyCode::Down), &[]);
    assert_eq!(app.focus, None, "moving in an empty view invented a cursor");
  }

  /// Arrows must not move the cursor from a mode that is collecting text --
  /// there, they belong to the field editor.
  #[test]
  fn arrows_do_not_move_the_cursor_while_a_field_is_collecting_text() {
    let mut app = App::explore();
    app.point_at(3);
    app.mode = Mode::Field;
    let before = app.focus;
    app.on_key(key(KeyCode::Down), &[]);
    assert_eq!(
      app.focus, before,
      "an arrow moved the row cursor while editing a field"
    );
  }

  /// `explore` and `edit` differ in their root and in nothing else the loop can
  /// see -- the same keys do the same things in both.
  #[test]
  fn explore_and_edit_run_the_same_loop() {
    let mut e = App::explore();
    let mut i = App::at_item("thread", "ST0056");
    assert_eq!(e.mode, i.mode);
    assert_eq!(e.on_key(key(KeyCode::Char(':')), &[]), Step::Continue);
    assert_eq!(i.on_key(key(KeyCode::Char(':')), &[]), Step::Continue);
    assert_eq!(e.mode, Mode::Command);
    assert_eq!(
      i.mode, e.mode,
      "the same key reached a different mode from a different root"
    );
    assert_eq!(e.on_key(esc(), &[]), Step::Continue);
    assert_eq!(i.on_key(esc(), &[]), Step::Continue);
    assert_eq!(e.mode, mode::REST);
    assert_eq!(i.mode, mode::REST);
    assert_eq!(
      e.on_key(esc(), &[]),
      Step::Quit,
      "explore quits from its root"
    );
    assert_eq!(i.on_key(esc(), &[]), Step::Quit, "edit quits from its root");
  }

  /// The rows an ITEM view shows: a label that is NOT the field name on the row
  /// the handoff cares about, which is what makes the assertion below able to
  /// fail.
  fn item_rows() -> Vec<Row> {
    vec![
      Row::named("title", "title", "ST0056: Intent v3", "text"),
      Row::named("status", "status", "wip", "select"),
      Row::named("objective", "the objective", "673 bytes", "prose"),
      Row::named("wps", "work pkgs", "17", "button"),
    ]
  }

  fn on_item() -> App {
    let mut app = App::at_item("thread", "ST0056");
    app.point_at(item_rows().len());
    app
  }

  #[test]
  fn the_fixture_labels_and_names_differ_on_the_row_that_matters() {
    let prose = &item_rows()[2];
    assert_eq!(prose.kind, "prose");
    assert_ne!(
      prose.name, prose.title,
      "the prose row's label and field name are the same string, so no test below could tell a \
       handoff addressed by NAME from one addressed by what is on screen"
    );
  }

  /// **`AT-17.10` / `AC-17.10`: ENTER ON A PROSE ROW ASKS FOR THE EDITOR, AND
  /// ADDRESSES THE FIELD BY ITS DECLARED NAME.** Addressing it by the label
  /// would write `the objective` to a thread, which the store refuses -- so the
  /// failure is loud, once, and only for fields whose label happens to differ.
  #[test]
  fn enter_on_a_prose_row_asks_for_the_editor_and_names_the_field_not_the_label() {
    let mut app = on_item();
    app.focus = app.focus.map(Focus::forward);
    app.focus = app.focus.map(Focus::forward);
    let step = app.on_key(key(KeyCode::Enter), &item_rows());
    assert_eq!(
      step,
      Step::Hand(Handoff {
        kind: "thread".to_string(),
        id: "ST0056".to_string(),
        field: "objective".to_string(),
      }),
      "a prose row must hand off, addressed by the declared field name"
    );
    assert_eq!(
      app.mode,
      Mode::Embed,
      "the child owns the terminal, and the mode has to say so"
    );
  }

  /// **AND EVERY OTHER ROW EDITS IN PLACE.** The same keystroke, one row over.
  /// `AC-17.4`: `prose` is the handoff and the rest of the vocabulary is not.
  #[test]
  fn enter_on_any_other_row_edits_in_place_rather_than_handing_off() {
    for (at, row) in item_rows().iter().enumerate() {
      if row.kind == "prose" {
        continue;
      }
      let mut app = on_item();
      app.focus = app.focus.and_then(|f| f.at(at));
      let step = app.on_key(key(KeyCode::Enter), &item_rows());
      assert_eq!(
        step,
        Step::Continue,
        "the `{}` row handed off to an editor",
        row.kind
      );
      assert_eq!(
        app.mode,
        Mode::Field,
        "the `{}` row did not open an in-place edit",
        row.kind
      );
    }
  }

  /// **A HANDOFF NEEDS AN ADDRESS, SO IT CANNOT BE ASKED FOR WHERE THERE IS
  /// NONE.** A collection row is a thread, not a field of one; a realiser that
  /// guessed an address here would write the operator's prose to whatever it
  /// guessed.
  #[test]
  fn a_prose_row_outside_an_item_view_cannot_hand_off() {
    let rows = vec![Row::named("objective", "the objective", "x", "prose")];
    for view in [
      View::Entities,
      View::Collection {
        kind: "thread".to_string(),
      },
      View::Children {
        kind: "thread".to_string(),
        id: "ST0056".to_string(),
        field: "wps".to_string(),
      },
    ] {
      let mut app = App::explore();
      app.stack = Stack::rooted_at(view.clone());
      app.point_at(rows.len());
      assert_eq!(
        app.on_key(key(KeyCode::Enter), &rows),
        Step::Continue,
        "{view:?} produced a handoff with no field to write back to"
      );
      assert_eq!(app.mode, mode::REST, "{view:?} left the mode moved");
    }
  }

  /// **THE CHILD EXITING LANDS WHERE THE MACHINE SAYS**, not where this
  /// function says. `EMBED + ChildExit -> NORMAL` is a declared edge.
  #[test]
  fn the_child_exiting_returns_to_the_rest_state_by_the_declared_edge() {
    let mut app = on_item();
    app.mode = Mode::Embed;
    app.child_exited();
    assert_eq!(
      app.mode,
      mode::step(Mode::Embed, "ChildExit").expect("the machine declares this edge"),
      "the realiser and the machine disagree about where a child exit lands"
    );
    assert_eq!(app.mode, mode::REST);
  }

  /// **RE-READING THE SAME VIEW KEEPS THE CURSOR; CHANGING VIEW RESETS IT.**
  /// Saving a field and being returned to the top of the form is the sort of
  /// thing that makes an operator stop using a form -- and the design's reset
  /// rule is about the row SET, not about the model behind it.
  #[test]
  fn a_re_read_keeps_the_cursor_and_a_new_view_resets_it() {
    let mut app = on_item();
    app.focus = app.focus.map(Focus::forward);
    app.focus = app.focus.map(Focus::forward);
    assert_eq!(app.focus.map(Focus::index), Some(2));

    app.refocus(item_rows().len());
    assert_eq!(
      app.focus.map(Focus::index),
      Some(2),
      "a re-read of the same view moved the operator's cursor"
    );

    app.point_at(item_rows().len());
    assert_eq!(
      app.focus.map(Focus::index),
      Some(0),
      "a new view must reset the cursor: a row index means nothing once the row set changes"
    );
  }

  /// A re-read that finds fewer rows than the cursor names falls back to the
  /// reset rather than keeping an index that refers to nothing.
  #[test]
  fn a_re_read_that_loses_rows_resets_rather_than_keeping_a_dangling_index() {
    let mut app = on_item();
    app.focus = app.focus.map(Focus::forward);
    app.focus = app.focus.map(Focus::forward);
    app.refocus(2);
    assert_eq!(app.focus.map(Focus::index), Some(0));
    app.refocus(0);
    assert_eq!(app.focus, None, "a view with no rows has no focus at all");
  }

  fn detailed_rows() -> Vec<Row> {
    vec![
      Row::named("title", "title", "ST0056", "text"),
      Row::named("status", "status", "wip", "select").expanding_to(vec![
        Row::new("legal", "done, cancelled", "text"),
        Row::new("owed", "a reason", "text"),
        Row::new("from", "wip", "text"),
      ]),
      Row::named("slug", "slug", "add-a-rust-based-cli", "text"),
    ]
  }

  fn tab() -> KeyEvent {
    key(KeyCode::Tab)
  }

  fn on_rows(n: usize) -> App {
    let mut app = App::explore();
    app.point_at(n);
    app
  }

  #[test]
  fn the_pane_fixture_has_detail_on_exactly_one_row() {
    let with: Vec<usize> = detailed_rows()
      .iter()
      .enumerate()
      .filter(|(_, r)| r.has_detail())
      .map(|(i, _)| i)
      .collect();
    assert_eq!(
      with,
      vec![1usize],
      "the fixture must have detail on exactly one row, or the walks below cannot show the pane \
       following it"
    );
  }

  /// **THE OPERATOR CAN NEVER BE IN A PANE THAT IS NOT ON THE SCREEN.** The
  /// pane is DERIVED from the row, so a request made on a row that has detail
  /// cannot survive onto one that has not -- asserted by walking the whole row
  /// set with the request left standing.
  #[test]
  fn a_pane_request_never_survives_onto_a_row_with_no_detail() {
    let rows = detailed_rows();
    let mut app = on_rows(rows.len());
    let mut in_detail = 0usize;
    for at in 0..rows.len() {
      app.focus = app.focus.and_then(|f| f.at(at));
      app.wants_detail = true;
      match app.pane(&rows) {
        Pane::Detail => {
          assert!(
            rows[at].has_detail(),
            "row {at} carries no detail and the cursor is in its detail pane"
          );
          in_detail += 1;
        }
        Pane::List => assert!(
          !rows[at].has_detail(),
          "row {at} carries detail and a standing request did not reach it"
        ),
      }
    }
    assert_eq!(
      in_detail, 1,
      "the walk found {in_detail} rows in the detail pane and the fixture has one"
    );
  }

  /// **TAB CROSSES BOTH WAYS, AND A PANE CANNOT BE ENTERED THAT CANNOT BE
  /// LEFT** -- `no_state_can_be_entered_and_not_left` applied to panes.
  #[test]
  fn tab_crosses_into_the_detail_pane_and_back_out_of_it() {
    let rows = detailed_rows();
    let mut app = on_rows(rows.len());
    app.focus = app.focus.and_then(|f| f.at(1));
    assert_eq!(app.pane(&rows), Pane::List);

    assert_eq!(app.on_key(tab(), &rows), Step::Continue);
    assert_eq!(app.pane(&rows), Pane::Detail, "Tab did not cross");
    assert_eq!(
      app.detail_focus.map(Focus::len),
      Some(3),
      "crossing did not point the cursor at the detail rows"
    );

    assert_eq!(app.on_key(tab(), &rows), Step::Continue);
    assert_eq!(app.pane(&rows), Pane::List, "Tab could not cross back");
  }

  /// **A TAB WITH NOWHERE TO GO CHANGES NOTHING AT ALL** -- not the pane, not
  /// the cursor, not the mode. The same rule as an unbound key: a self-loop
  /// that absorbs the keystroke teaches the operator that Tab is broken.
  #[test]
  fn tab_on_a_row_with_no_detail_changes_nothing() {
    let rows = detailed_rows();
    for at in [0usize, 2] {
      let mut app = on_rows(rows.len());
      app.focus = app.focus.and_then(|f| f.at(at));
      let before = app.clone();
      assert_eq!(app.on_key(tab(), &rows), Step::Continue);
      assert_eq!(app, before, "Tab on row {at} changed the app");
    }
  }

  /// **THE ARROWS MOVE THE PANE THE CURSOR IS IN**, which is what section 4's
  /// *move within the focused pane* means. Driven on BOTH panes, because a
  /// realiser that always moved the list passes every list-only assertion.
  #[test]
  fn the_arrows_move_whichever_pane_the_cursor_is_in() {
    let rows = detailed_rows();
    let mut app = on_rows(rows.len());
    app.focus = app.focus.and_then(|f| f.at(1));
    app.on_key(tab(), &rows);
    assert_eq!(app.pane(&rows), Pane::Detail);

    let list_before = app.focus.map(Focus::index);
    app.on_key(key(KeyCode::Down), &rows);
    assert_eq!(
      app.detail_focus.map(Focus::index),
      Some(1),
      "Down did not move the detail cursor"
    );
    assert_eq!(
      app.focus.map(Focus::index),
      list_before,
      "Down moved the LIST cursor while the detail pane had focus"
    );
    assert_eq!(app.pane(&rows), Pane::Detail, "moving left the pane");
  }

  /// **LEAVING THE ROW LEAVES ITS PANE.** The detail belonged to the row that
  /// opened it; carrying the request onto the next row would reopen a pane over
  /// somebody else's detail.
  #[test]
  fn moving_the_list_cursor_closes_the_pane_the_previous_row_opened() {
    let rows = detailed_rows();
    let mut app = on_rows(rows.len());
    app.focus = app.focus.and_then(|f| f.at(1));
    app.on_key(tab(), &rows);
    assert_eq!(app.pane(&rows), Pane::Detail);

    // Back to the list first, then move: the arrow that moves the LIST is the
    // one that has to clear the request.
    app.on_key(tab(), &rows);
    app.on_key(key(KeyCode::Down), &rows);
    assert!(
      !app.wants_detail && app.detail_focus.is_none(),
      "the pane request outlived the row that made it"
    );
    assert_eq!(app.pane(&rows), Pane::List);
  }

  /// A re-read that changes the row set drops the pane with the cursor, for the
  /// same reason it drops the cursor: the detail belonged to a row that may no
  /// longer be there.
  #[test]
  fn pointing_at_a_new_row_set_closes_any_open_pane() {
    let rows = detailed_rows();
    let mut app = on_rows(rows.len());
    app.focus = app.focus.and_then(|f| f.at(1));
    app.on_key(tab(), &rows);
    assert_eq!(app.pane(&rows), Pane::Detail);
    app.point_at(rows.len());
    assert_eq!(
      app.pane(&rows),
      Pane::List,
      "a new row set kept the old pane"
    );
    assert!(app.detail_focus.is_none());
  }
}
