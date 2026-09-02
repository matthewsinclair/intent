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

use super::commands::{self, Act, Command};
use super::edit::Handoff;
use super::focus::Focus;
use super::keys;
use super::layout::Row;
use super::mode::{self, Mode};
use super::nav::{Stack, View};
use super::omnibox::{Entry, Go, Omnibox};

/// How many matches the omnibox offers at once. Eight: enough to show a
/// collision, few enough that the dropdown never eats the body.
pub const MATCH_CAP: usize = 8;

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
  /// The operator asked to leave: `:q` from the omnibox, or the realiser's
  /// own `Ctrl-C`. **`tui-design.md` §3: quitting is an act, never an
  /// accident** -- no key reaches this by walking.
  Quit,
  /// A spelling the omnibox could not match: hand it to the address resolver.
  ///
  /// **A `Step` BECAUSE PRESENCE IS A FACT ONLY THE STORE KNOWS.** `nav::land`
  /// needs a presence probe, the app has no facade on purpose, and guessing
  /// here would navigate to entities that do not exist.
  Land(String),
  /// Open an in-place edit: read this field's RAW value first.
  ///
  /// **THE READ GOES THROUGH THE SAME `Model::read` THE $EDITOR HANDOFF
  /// USES** -- the display value on the row may be truncated or derived, and
  /// an editor seeded from a rendering writes the rendering back. The loop
  /// reads, then [`App::begin_edit`] or [`App::abort_edit`] lands the result.
  ReadField(Handoff),
  /// Commit an in-place edit through `Model::write` -- `facade.set`'s own
  /// refusals reach the notice in the model's words.
  WriteField(Handoff, String),
  /// `AC-17.10`: hand this field to `$VISUAL`/`$EDITOR`.
  Hand(Handoff),
  /// `AC-17.8`: open one realised artefact of this entity, or refuse it.
  ///
  /// **Its own variant rather than a flag on [`Handoff`]**, because
  /// `tui-design.md` section 7 makes it a different operation and not a mode of
  /// the same one: no scratch file, no read-back, and the file the editor
  /// receives is the artefact rather than a copy of a field.
  Open {
    kind: String,
    id: String,
    name: String,
  },
}

/// Which pane the cursor is in.
///
/// **A GUARD ON OMNI'S EDGES, NOT A FIFTH MODE.** [`super::keys`] already
/// says so by refusing `Tab` a trigger: crossing panes changes where `Move` and
/// `Enter` LAND, never what the keys mean, so it is the app's state and not the
/// machine's. A `Mode::Detail` would have to duplicate every OMNI edge.
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
  /// The rest state's input: buffer and pick. See [`super::omnibox`].
  pub omnibox: Omnibox,
  /// Every addressable entity, for the omnibox's matcher. Handed in by the
  /// run loop at startup, because the app deliberately holds no facade.
  pub index: Vec<Entry>,
  /// What `/` offers. Handed in beside [`App::index`] and for the same
  /// reason: the `Go` half is DERIVED from the declared entity kinds, which
  /// is a fact about the schema and therefore not this module's to know.
  pub commands: Vec<Command>,
  /// The in-place edit in flight, while the mode is FIELD.
  pub editing: Option<FieldEdit>,
}

/// One in-place edit: where it writes, and what has been typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldEdit {
  pub handoff: Handoff,
  pub buffer: String,
}

impl App {
  /// An app rooted at `bottom`. **THE ONE CONSTRUCTOR, because the root is the
  /// only thing that differs between the entry points** -- `Stack::rooted_at`
  /// already says exactly that about the stack, and a third spelled-out copy of
  /// these seven fields arrived the day `explore` learned to take an address.
  pub fn rooted_at(bottom: View) -> Self {
    Self {
      stack: Stack::rooted_at(bottom),
      mode: mode::REST,
      scroll: 0,
      focus: None,
      wants_detail: false,
      detail_focus: None,
      notice: String::new(),
      omnibox: Omnibox::default(),
      index: Vec::new(),
      commands: Vec::new(),
      editing: None,
    }
  }

  /// The reason this app opened where it did, for the info row.
  ///
  /// **`intent explore <address>` that could not land opens at the root AND
  /// SAYS WHY** (hv's fallback, ruled 2026-08-30; vc ruled that *opens at the
  /// root* contrasts with REFUSING rather than with TELLING). The notice
  /// clears on the first view change, so it survives exactly as long as the
  /// operator is looking at the place they did not ask for.
  pub fn saying(mut self, why: impl Into<String>) -> Self {
    self.notice = why.into();
    self
  }

  /// `intent explore` -- rooted where [`Stack::explore`] roots: the threads
  /// list. One home for the root; this constructor only borrows it.
  pub fn explore() -> Self {
    Self::rooted_at(Stack::explore().current().clone())
  }

  /// `intent edit <kind> <id>` -- rooted at one item.
  pub fn at_item(kind: impl Into<String>, id: impl Into<String>) -> Self {
    Self::rooted_at(View::Item {
      kind: kind.into(),
      id: id.into(),
    })
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
  /// ROW CAN SETTLE.** `tui-design.md` section 3 gives `OMNI + Enter` three
  /// arms -- descend for a door row, FIELD for an editable one, EMBED for a
  /// prose one -- and [`mode::step`] picks between them with `.find()`, which
  /// is TABLE ORDER. The machine's own test says as much in its message.
  /// Passing the rows in is what lets [`mode::arm`] answer from the declared
  /// discriminator instead.
  ///
  /// **AND `self.omnibox` IS THE OTHER HALF OF THE SAME STORY.** Since `NAV`
  /// folded into the composer, the machine's edges no longer distinguish
  /// *acting on the match list* from *acting on the body*; the buffer does.
  /// Every guard below is that one fact asked at a different keystroke.
  pub fn on_key(&mut self, key: KeyEvent, rows: &[Row]) -> Step {
    // **`Tab` IS A PANE GUARD AND NOT A MODE TRIGGER**, which [`super::keys`]
    // declares by refusing it one. It is answered here, ahead of the machine,
    // because the machine has nothing to say about it -- and only while the
    // composer is EMPTY, because that is when the operator is browsing the
    // body. With a query typed the panes are behind a dropdown, and in FIELD
    // the keyboard belongs to a buffer and in EMBED to the child.
    if self.mode == Mode::Omni
      && self.omnibox.is_empty()
      && matches!(key.code, KeyCode::Tab | KeyCode::BackTab)
    {
      self.cross_panes(rows);
      return Step::Continue;
    }
    // Not a key we bind. **Nothing happens -- not a self-loop, nothing.**
    let Some(mut trigger) = keys::trigger(self.mode, key) else {
      return Step::Continue;
    };
    // **THE BUFFER GUARD, ONE RULE ASKED OF TWO KEYS** (`tui-design.md` §3).
    // Mid-query, `/` is a character -- `st/ST0056` is a legal spelling -- and
    // `Backspace` deletes one rather than walking up the model. The keymap
    // cannot see the buffer, so it offers the mode-significant trigger and
    // this downgrades it. **They are guarded TOGETHER on purpose**: two keys
    // sharing one rule should not grow two copies of it.
    if self.mode == Mode::Omni && !self.omnibox.is_empty() && matches!(trigger, "/" | "Back") {
      trigger = "Typing";
    }

    // The composer's own triggers, resolved before the generic tail because
    // each needs the keystroke or the match list, which the machine never sees.
    //
    // **`Typing` IS UNCONDITIONAL; `Move` AND `Enter` ARE GUARDED ON THE
    // BUFFER.** An empty composer means the operator is browsing, so those two
    // belong to the ROW and fall through to the tail; with a query typed they
    // belong to the match list and are answered here.
    if self.mode == Mode::Omni {
      match trigger {
        "Typing" => {
          match key.code {
            KeyCode::Char(c) => self.omnibox.type_char(c),
            KeyCode::Backspace => self.omnibox.erase(),
            _ => {}
          }
          return Step::Continue;
        }
        // **ESC CLEARS THE QUERY, AND ON AN ALREADY-EMPTY COMPOSER IT IS A
        // NO-OP BECAUSE YOU ARE ALREADY HOME.** Stated as one unconditional
        // call rather than as a branch with an empty arm: the no-op is the
        // behaviour, not a case nobody got round to. It never navigates --
        // popping the stack is `Back`'s job, and the collapse deliberately did
        // not overload Esc with it.
        // **`/` SEEDS THE COMPOSER WITH THE SIGIL AND OPENS THE PALETTE.**
        // Reached only on an empty buffer, since the guard above turned a
        // mid-query slash into a character. The sigil STAYS in the buffer:
        // it is what tells the operator which vocabulary is being searched,
        // and it gives Backspace an exit that needs no special case.
        "/" => {
          self.omnibox.type_char(commands::SIGIL);
          self.mode = Mode::Menu;
          return Step::Continue;
        }
        "Esc" => {
          self.omnibox.clear();
          return Step::Continue;
        }
        "Move" if !self.omnibox.is_empty() => {
          let n = self.match_count();
          self.omnibox.pick_move(key.code == KeyCode::Down, n);
          return Step::Continue;
        }
        "Enter" if !self.omnibox.is_empty() => {
          let m = super::omnibox::matches(&self.index, &self.omnibox.buffer, MATCH_CAP);
          let go = self.omnibox.go(&self.index, &m);
          // **THE COMPOSER IS WHERE YOU ALREADY ARE, SO GOING SOMEWHERE DOES
          // NOT LEAVE IT.** The machine's edge is `OMNI + Enter -> OMNI`; the
          // old `= Mode::Omni` here was the operator being moved off the input
          // by a successful navigation, which is exactly what the one-home
          // ruling retired. Quit is the one exception and it is not a mode.
          return match go {
            Go::Nothing => Step::Continue,
            Go::Quit => Step::Quit,
            Go::UnknownCommand(c) => {
              self.notice = format!("unknown command `:{c}` -- `/` opens the menu");
              Step::Continue
            }
            Go::Pick(view) => {
              self.omnibox.clear();
              self.push(view);
              Step::Continue
            }
            Go::Spelling(s) => Step::Land(s),
          };
        }
        _ => {}
      }
    }

    // **THE PALETTE: ONE INPUT, A SECOND VOCABULARY.** It reuses the
    // composer's own buffer and pick because it IS the composer -- the sigil
    // is what says which vocabulary is being searched, and sharing the input
    // is why `/qu` and `56` cannot drift into feeling like two programs.
    //
    // **EVERY TRIGGER MENU DECLARES IS ANSWERED HERE AND RETURNS.** That is
    // load-bearing rather than tidy: the generic `Move` handler at the tail
    // moves the BODY cursor, so a MENU trigger falling through to it scrolls
    // the list invisibly behind the palette. That is exactly what hv drove
    // into -- arrows in the menu appearing to do nothing while silently
    // moving something else.
    if self.mode == Mode::Menu {
      match trigger {
        "Typing" => {
          match key.code {
            KeyCode::Char(c) => self.omnibox.type_char(c),
            KeyCode::Backspace => self.omnibox.erase(),
            _ => {}
          }
          // **ERASING THE SIGIL LEAVES THE PALETTE**, which is why MENU needs
          // no exit key of its own and why `Back` was retired from its edges.
          if commands::query_of(&self.omnibox.buffer).is_none() {
            self.omnibox.clear();
            self.mode = Mode::Omni;
          }
          return Step::Continue;
        }
        "Move" => {
          let n = self.palette().len();
          self.omnibox.pick_move(key.code == KeyCode::Down, n);
          return Step::Continue;
        }
        "Enter" => {
          let hits = self.palette();
          let picked = self.omnibox.picked(hits.len()).map(|p| hits[p].entry);
          self.omnibox.clear();
          self.mode = Mode::Omni;
          let Some(at) = picked else {
            return Step::Continue;
          };
          return match self.commands[at].act.clone() {
            Act::Quit => Step::Quit,
            Act::Back => {
              self.pop_view();
              Step::Continue
            }
          };
        }
        "Esc" | "Cancel" | "/" => {
          self.omnibox.clear();
          self.mode = Mode::Omni;
          return Step::Continue;
        }
        _ => {}
      }
    }

    // FIELD collects into the in-flight edit; commit and discard are the two
    // declared exits and both carry their outcome with them.
    if self.mode == Mode::Field {
      match trigger {
        "Typing" => {
          if let Some(edit) = &mut self.editing {
            match key.code {
              KeyCode::Char(c) => edit.buffer.push(c),
              KeyCode::Backspace => {
                edit.buffer.pop();
              }
              _ => {}
            }
          }
          return Step::Continue;
        }
        "Enter" => {
          self.mode = Mode::Omni;
          return match self.editing.take() {
            Some(edit) => Step::WriteField(edit.handoff, edit.buffer),
            None => Step::Continue,
          };
        }
        "Esc" => {
          self.mode = Mode::Omni;
          if let Some(edit) = self.editing.take() {
            self.notice = format!("{} unchanged -- edit discarded", edit.handoff.field);
          }
          return Step::Continue;
        }
        _ => {}
      }
    }

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

    // **DESCENT IS THE DOOR'S, AND THE DOOR IS DECLARED ON THE ROW** --
    // `tui-design.md` §6: *a row's door is DECLARED on the row, not inferred
    // from its kind*. Enter resolving back to OMNI means the row claims
    // descent; a door-less button descends nowhere, visibly, rather than
    // guessing. Reached only on an empty composer, since a typed query
    // answered Enter above.
    if self.mode == Mode::Omni && trigger == "Enter" && next == Mode::Omni {
      match self.focused_row(rows).and_then(|r| r.door.clone()) {
        Some(view) => self.push(view),
        None => self.notice = "this row opens nothing yet".to_string(),
      }
      return Step::Continue;
    }

    // **AN IN-PLACE EDIT OPENS ON THE RAW VALUE, NOT THE ROW'S RENDERING** --
    // same reason, same door as the $EDITOR handoff: the display value may be
    // truncated or derived, and an editor seeded from a rendering writes the
    // rendering back. Item views only, exactly like EMBED below.
    if next == Mode::Field && self.mode == Mode::Omni && trigger == "Enter" {
      let (Some(View::Item { kind, id }), Some(row)) = (
        Some(self.stack.current().clone()).filter(|v| matches!(v, View::Item { .. })),
        self.focused_row(rows),
      ) else {
        return Step::Continue;
      };
      self.mode = Mode::Field;
      return Step::ReadField(Handoff {
        kind,
        id,
        field: row.name.clone(),
      });
    }

    // **THE HANDOFF LEAVES AS A REQUEST AND CHANGES NOTHING ELSE.** It needs a
    // field to write back to, so it can only be asked for on an ITEM view; a
    // prose row cannot occur anywhere else, and a realiser that guessed an
    // address here would write to whatever it guessed.
    if next == Mode::Embed && self.mode == Mode::Omni {
      let (Some(View::Item { kind, id }), Some(row)) = (
        Some(self.stack.current().clone()).filter(|v| matches!(v, View::Item { .. })),
        self.focused_row(rows),
      ) else {
        return Step::Continue;
      };
      let name = row.name.clone();
      let artefact = row.kind == "artefact";
      self.mode = next;
      // **THE ROW KIND DECIDES, AND IT IS THE SAME FACT `mode::arm` ALREADY
      // USED TO GET HERE.** Both kinds claim `Embed` in `BY_ROW_KIND`; what
      // differs is what the editor is handed, which is the design's own split.
      return if artefact {
        Step::Open { kind, id, name }
      } else {
        Step::Hand(Handoff {
          kind,
          id,
          field: name,
        })
      };
    }

    // **POPPING THE VIEW STACK IS `Back`'s JOB AND ONLY `Back`'s** since the
    // omnibox machine: Esc means *the other home mode* now, so Backspace is
    // the one key that walks up the model. At the root it is a no-op rather
    // than a quit -- `tui-design.md` §3 retired the accident.
    if self.mode == Mode::Omni && trigger == "Back" {
      self.pop_view();
      return Step::Continue;
    }

    // **DIRECTION IS THE APP'S BUSINESS AND THE MACHINE'S IGNORANCE IS
    // DELIBERATE.** `EDGES` says OMNI + Move stays in OMNI, and that is all
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

  /// How many entries the current buffer matches, for the pick's bounds.
  pub fn match_count(&self) -> usize {
    super::omnibox::matches(&self.index, &self.omnibox.buffer, MATCH_CAP).len()
  }

  /// The commands the current palette query hits, best first.
  ///
  /// **ONE FUNCTION, because three callers need the SAME list**: the pick's
  /// bounds, the command Enter runs, and the dropdown the operator is reading.
  /// If Enter recomputed the hits differently from the renderer, it would run
  /// whichever command the two happened to disagree about -- and the operator
  /// would have watched a correct list the whole time.
  pub fn palette(&self) -> Vec<super::omnibox::Match> {
    let query = commands::query_of(&self.omnibox.buffer).unwrap_or("");
    commands::matches(&self.commands, query, MATCH_CAP)
  }

  /// Pop the view stack. **ONE HOME for an act two doors reach** -- the `Back`
  /// trigger and the palette's own `back` command -- because a second copy is
  /// how the scroll reset gets forgotten in one of them.
  fn pop_view(&mut self) {
    if self.stack.pop() {
      self.scroll = 0;
    }
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

  /// The loop read the field: the edit is live, seeded with the RAW value.
  pub fn begin_edit(&mut self, handoff: Handoff, value: String) {
    self.editing = Some(FieldEdit {
      handoff,
      buffer: value,
    });
  }

  /// The read refused, so the edit never opened: back to the composer with
  /// the refusal standing where the operator is looking. **A field that cannot
  /// open must not present a collector**, or typing goes somewhere nothing
  /// will ever write.
  pub fn abort_edit(&mut self, why: String) {
    self.editing = None;
    self.mode = Mode::Omni;
    self.notice = why;
  }

  /// The child that owned the terminal has gone.
  ///
  /// **THE MACHINE SAYS WHERE THIS LANDS, NOT THIS FUNCTION.** `EMBED +
  /// ChildExit -> OMNI` is a declared edge with the note *read the file
  /// back*; spelling `self.mode = Mode::Omni` here would be a second copy of
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

  /// **THE PROPERTY THAT MAKES A MODAL UI SAFE TO BE LOST IN**, restated for
  /// the omnibox machine: from anywhere, one Esc lands in a HOME mode and
  /// further presses stay inside the pair -- **and no number of them ever
  /// quits**, because `tui-design.md` §3 retired the accident. The walk is
  /// bounded and driven from every mode at several depths.
  #[test]
  fn repeated_esc_reaches_home_and_never_quits() {
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
        let budget = Mode::ALL.len() + depth + 4;
        for press in 0..budget {
          assert_eq!(
            app.on_key(esc(), &[]),
            Step::Continue,
            "Esc press {press} from {started_in:?} at depth {depth} quit -- quitting is an act, \
             never an accident"
          );
          assert!(
            press == 0 || mode::HOME.contains(&app.mode),
            "Esc press {press} from {started_in:?} left home for {:?} -- one press lands home \
             and the rest stay there",
            app.mode
          );
        }
        assert!(
          mode::HOME.contains(&app.mode),
          "the walk from {started_in:?} ended outside home in {:?}",
          app.mode
        );
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

  /// **QUIT IS `:q`, TYPED, AND NOTHING ELSE THE APP SEES** -- `Ctrl-C` is
  /// the realiser's own intercept and never reaches `on_key`. Driven at depth
  /// so the act works from anywhere, not only at a root the operator may not
  /// be standing on.
  #[test]
  fn quit_is_the_typed_act_and_works_from_any_depth() {
    for depth in [0usize, 3] {
      let mut app = App::explore();
      for i in 0..depth {
        app.push(View::Collection {
          kind: format!("k{i}"),
        });
      }
      // **TYPED STRAIGHT INTO THE COMPOSER, WHICH IS THE WHOLE POINT OF ONE
      // HOME.** This used to press `:` from NAV to SEED the omnibox and then
      // assert the seed had landed -- a mode change carrying a character. With
      // the composer always holding the keyboard there is no seed step to
      // assert: the characters simply arrive.
      assert_eq!(app.on_key(key(KeyCode::Char(':')), &[]), Step::Continue);
      assert_eq!(app.mode, Mode::Omni, "typing must not move the operator");
      assert_eq!(app.on_key(key(KeyCode::Char('q')), &[]), Step::Continue);
      assert_eq!(
        app.on_key(key(KeyCode::Enter), &[]),
        Step::Quit,
        ":q at depth {depth} did not quit"
      );
    }
    // And `:q!`, the discard spelling, is the same act.
    let mut app = App::explore();
    for c in ":q!".chars() {
      assert_eq!(app.on_key(key(KeyCode::Char(c)), &[]), Step::Continue);
    }
    assert_eq!(app.on_key(key(KeyCode::Enter), &[]), Step::Quit);
  }

  /// **`Back` IS THE ONE KEY THAT WALKS UP THE MODEL** since the omnibox
  /// machine -- Esc means *the other home mode* now -- and at the root it is
  /// a no-op, never a quit.
  #[test]
  fn back_pops_the_view_and_does_not_quit_at_the_root() {
    let mut app = App::explore();
    app.mode = Mode::Omni;
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
    app.mode = Mode::Omni;
    app.scroll = 17;
    app.push(View::Collection {
      kind: "thread".into(),
    });
    assert_eq!(
      app.scroll, 0,
      "descending kept a scroll position from the view above"
    );
    app.scroll = 9;
    app.on_key(key(KeyCode::Backspace), &[]);
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
    app.mode = Mode::Omni;
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
    assert_eq!(e.on_key(esc(), &[]), Step::Continue);
    assert_eq!(i.on_key(esc(), &[]), Step::Continue);
    assert_eq!(e.mode, Mode::Omni);
    assert_eq!(
      i.mode, e.mode,
      "the same key reached a different mode from a different root"
    );
    assert_eq!(e.on_key(key(KeyCode::Char(':')), &[]), Step::Continue);
    assert_eq!(i.on_key(key(KeyCode::Char(':')), &[]), Step::Continue);
    assert_eq!(e.mode, mode::REST, "`:` seeds the omnibox from NAV");
    assert_eq!(i.mode, mode::REST);
    assert_eq!(
      e.omnibox.buffer, ":",
      "the seed carries its character into the buffer"
    );
    assert_eq!(i.omnibox.buffer, ":");
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
    // The tests here exercise NAV's Enter; the app opens in the omnibox.
    app.mode = Mode::Omni;
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

  /// **`AT-17.8`: ENTER ON AN ARTEFACT ROW OPENS THE FILE, NOT THE FIELD.**
  ///
  /// The two row kinds reach `Embed` through the SAME edge -- both claim it in
  /// `mode::BY_ROW_KIND` -- so nothing in the mode machine distinguishes them
  /// and the whole distinction lives here. **Without this the artefact would
  /// take the field path**, and `hand_off` would ask the model to read a FIELD
  /// called `design`, realise it to a scratch file, and write whatever came
  /// back into a property that does not exist. That is a silent wrong-target
  /// write rather than a visible failure, which is why it is asserted rather
  /// than left to the fact that it currently works.
  #[test]
  fn enter_on_an_artefact_row_opens_the_file_rather_than_handing_off_a_field() {
    let mut rows = item_rows();
    rows.push(Row::named("design", "design.md", "authored", "artefact"));
    let last = rows.len() - 1;
    // **`Focus` CARRIES ITS OWN LENGTH, so pointing at a row set and then
    // growing it leaves the cursor bounded by the OLD count.** The first
    // version of this test walked `Focus::forward` until it reached `last` --
    // which `on_item` had made unreachable by construction, because it points
    // at `item_rows().len()` and the pushed row sits one past the end. It spun
    // at 100% of a core with no timeout able to fire, and dc found it by
    // sampling the stack rather than by any signal the test gave. **A walk with
    // no bound is worse than an assertion that fails**: the assertion names the
    // invariant, and the walk says nothing at all while burning a core. Point
    // at the real length and address the row directly.
    let mut app = App::at_item("thread", "ST0056");
    app.mode = Mode::Omni;
    app.point_at(rows.len());
    app.focus = app.focus.and_then(|f| f.at(last));
    assert_eq!(
      app.cursor(&rows).map(|f| f.index()),
      Some(last),
      "the cursor could not be placed on the artefact row, so the keystroke below would be about \
       some other row"
    );
    let step = app.on_key(key(KeyCode::Enter), &rows);
    assert_eq!(
      step,
      Step::Open {
        kind: "thread".to_string(),
        id: "ST0056".to_string(),
        name: "design".to_string(),
      },
      "an artefact row must open the realised file, addressed by the file's own name"
    );
    assert!(
      !matches!(step, Step::Hand(_)),
      "an artefact reaching the FIELD handoff would write the file's name into a property that \
       does not exist"
    );
    assert_eq!(
      app.mode,
      Mode::Embed,
      "the child owns the terminal either way"
    );
  }

  /// **AND EVERY EDITABLE ROW EDITS IN PLACE**, opening on the RAW value the
  /// loop reads -- never the row's rendering -- and committing through the
  /// same write door the $EDITOR handoff uses. `AC-17.4`: `prose` is the
  /// handoff, `button` is the descent; both have their own tests.
  #[test]
  fn enter_on_any_other_row_edits_in_place_rather_than_handing_off() {
    for (at, row) in item_rows().iter().enumerate() {
      if row.kind == "prose" || row.kind == "button" {
        continue;
      }
      let mut app = on_item();
      app.focus = app.focus.and_then(|f| f.at(at));
      let step = app.on_key(key(KeyCode::Enter), &item_rows());
      assert_eq!(
        step,
        Step::ReadField(Handoff {
          kind: "thread".into(),
          id: "ST0056".into(),
          field: row.name.clone(),
        }),
        "the `{}` row must ask for its RAW value, addressed by the field NAME",
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

  /// The whole in-place round trip, driven: the loop seeds the RAW value,
  /// typing edits it, Enter carries the edited buffer to the write door, and
  /// Esc discards without one -- with the mode landing in NAV either way.
  #[test]
  fn an_in_place_edit_commits_the_edited_buffer_and_esc_discards_it() {
    let rows = item_rows();
    let mut app = on_item();
    let Step::ReadField(h) = app.on_key(key(KeyCode::Enter), &rows) else {
      panic!("the title row did not open an edit");
    };
    app.begin_edit(h.clone(), "raw title".into());
    for c in "!".chars() {
      assert_eq!(app.on_key(key(KeyCode::Char(c)), &rows), Step::Continue);
    }
    assert_eq!(
      app.on_key(key(KeyCode::Enter), &rows),
      Step::WriteField(h.clone(), "raw title!".into()),
      "Enter must carry the EDITED buffer to the write door"
    );
    assert_eq!(app.mode, Mode::Omni);
    assert_eq!(app.editing, None, "a committed edit must not linger");

    // Discard: the same entry, Esc out, nothing written.
    let Step::ReadField(h2) = app.on_key(key(KeyCode::Enter), &rows) else {
      panic!("re-entry did not open");
    };
    app.begin_edit(h2, "raw title".into());
    assert_eq!(app.on_key(key(KeyCode::Backspace), &rows), Step::Continue);
    assert_eq!(app.on_key(esc(), &rows), Step::Continue);
    assert_eq!(app.mode, Mode::Omni);
    assert_eq!(app.editing, None);
    assert!(
      app.notice.contains("discarded"),
      "a discard must say so: {:?}",
      app.notice
    );

    // And a refused read never presents a collector.
    let Step::ReadField(_) = app.on_key(key(KeyCode::Enter), &rows) else {
      panic!("third entry did not open");
    };
    app.abort_edit("error: not a text field".into());
    assert_eq!(
      app.mode,
      Mode::Omni,
      "a refused read must fall back to the composer"
    );
    assert_eq!(app.editing, None);
  }

  /// **A DOORED BUTTON DESCENDS, AND A DOOR-LESS ONE SAYS SO** -- the fix for
  /// the strawman's worst defect (hv drove it, 2026-08-30): Enter on `thread`
  /// flipped modes and navigated nowhere. The door is DECLARED on the row
  /// (`tui-design.md` §6), so descent is asserted against the declaration and
  /// a row nobody doored reports itself instead of guessing.
  #[test]
  fn enter_on_a_button_descends_through_its_declared_door_or_says_it_has_none() {
    let doored = vec![Row::new("thread", "", "button").opening(View::Collection {
      kind: "thread".into(),
    })];
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.point_at(doored.len());
    assert_eq!(app.on_key(key(KeyCode::Enter), &doored), Step::Continue);
    assert_eq!(
      app.stack.current(),
      &View::Collection {
        kind: "thread".into()
      },
      "Enter on a doored button did not descend through its door"
    );
    assert_eq!(
      app.mode,
      Mode::Omni,
      "descent stays in OMNI -- the composer is where you already are"
    );

    let doorless = vec![Row::new("wps", "17", "button")];
    let mut app = App::explore();
    app.mode = Mode::Omni;
    app.point_at(doorless.len());
    assert_eq!(app.on_key(key(KeyCode::Enter), &doorless), Step::Continue);
    assert!(
      app.stack.at_root(),
      "a door-less button descended into something nobody declared"
    );
    assert!(
      !app.notice.is_empty(),
      "a door-less button must SAY it opens nothing rather than silently doing nothing"
    );
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
      app.mode = Mode::Omni;
      app.stack = Stack::rooted_at(view.clone());
      app.point_at(rows.len());
      assert_eq!(
        app.on_key(key(KeyCode::Enter), &rows),
        Step::Continue,
        "{view:?} produced a handoff with no field to write back to"
      );
      assert_eq!(app.mode, Mode::Omni, "{view:?} left the mode moved");
    }
  }

  /// **THE CHILD EXITING LANDS WHERE THE MACHINE SAYS**, not where this
  /// function says. `EMBED + ChildExit -> NAV` is a declared edge: the
  /// operator comes back to the form they were on, cursor intact, not to the
  /// omnibox -- the field they just edited is what they want to see.
  #[test]
  fn the_child_exiting_lands_in_nav_by_the_declared_edge() {
    let mut app = on_item();
    app.mode = Mode::Embed;
    app.child_exited();
    assert_eq!(
      app.mode,
      mode::step(Mode::Embed, "ChildExit").expect("the machine declares this edge"),
      "the realiser and the machine disagree about where a child exit lands"
    );
    assert_eq!(app.mode, Mode::Omni);
    assert!(
      mode::HOME.contains(&app.mode),
      "a child exit must land somewhere fully operable"
    );
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
    // These tests exercise NAV's panes and arrows; the app opens in the omnibox.
    app.mode = Mode::Omni;
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

  /// **hv's THREE FINDINGS, DRIVEN AS THE OPERATOR DROVE THEM.** hv rebuilt at
  /// `a8981480`, opened the menu, and reported: the arrows do nothing, `:q` is
  /// still the only way out, and `/quit` does not work because the composer
  /// only searches entities. All three were one defect -- MENU was a painted
  /// string with no model -- and these are the three keystroke sequences hv
  /// actually typed.
  #[test]
  fn hv_can_open_the_palette_filter_it_and_quit_without_typing_a_colon() {
    let mut app = App::explore();
    app.commands = commands::vocabulary();

    // 1. `/` opens the palette, ONE press, and seeds the sigil so the operator
    //    can see which vocabulary is being searched.
    assert_eq!(app.on_key(key(KeyCode::Char('/')), &[]), Step::Continue);
    assert_eq!(app.mode, Mode::Menu, "`/` did not open the palette");
    assert_eq!(app.omnibox.buffer, "/", "the sigil must stay visible");

    // 2. The palette at REST offers its whole vocabulary -- discovery is the
    //    reason it exists, so an empty query must not mean an empty list.
    assert_eq!(
      app.palette().len(),
      app.commands.len(),
      "the palette opened empty, so it teaches the operator nothing"
    );

    // 3. Typing FILTERS commands. This is the half hv found missing: the
    //    composer searched entities and had no command vocabulary at all.
    for c in "quit".chars() {
      assert_eq!(app.on_key(key(KeyCode::Char(c)), &[]), Step::Continue);
    }
    assert_eq!(app.omnibox.buffer, "/quit");
    let hits = app.palette();
    assert!(!hits.is_empty(), "`/quit` matched no command");
    assert_eq!(
      app.commands[hits[0].entry].act,
      Act::Quit,
      "`/quit` did not rank quit first"
    );

    // 4. Enter RUNS it -- hv's finding 2, that `:q` was the only way out.
    assert_eq!(
      app.on_key(key(KeyCode::Enter), &[]),
      Step::Quit,
      "Enter on the picked command did not run it"
    );
  }

  /// **THE ARROWS MOVE THE PALETTE AND MUST NOT MOVE THE BODY.** hv reported
  /// them as doing nothing; they were doing something WORSE -- the generic
  /// `Move` handler is not mode-guarded, so in MENU they scrolled the list
  /// cursor invisibly behind the menu bar. **The body cursor is asserted
  /// UNMOVED**, because "the palette pick changed" alone would pass just as
  /// well if the body moved too.
  #[test]
  fn arrows_in_the_palette_move_the_pick_and_leave_the_body_alone() {
    let r = item_rows();
    let mut app = App::explore();
    app.commands = commands::vocabulary();
    app.point_at(r.len());
    let body_before = app.focus.map(Focus::index);
    assert!(
      app.commands.len() >= 2,
      "a one-command palette cannot exhibit a moving pick"
    );

    app.on_key(key(KeyCode::Char('/')), &r);
    let pick_before = app.omnibox.picked(app.palette().len());
    app.on_key(key(KeyCode::Down), &r);

    assert_ne!(
      app.omnibox.picked(app.palette().len()),
      pick_before,
      "the arrow did not move the palette pick"
    );
    assert_eq!(
      app.focus.map(Focus::index),
      body_before,
      "the arrow moved the BODY cursor behind the palette -- the silent defect hv drove into"
    );
    assert_eq!(app.mode, Mode::Menu, "moving the pick left the palette");
  }

  /// Erasing back past the sigil leaves the palette, which is why MENU needs
  /// no exit key of its own and why `Back` was retired from its edges.
  #[test]
  fn erasing_the_sigil_leaves_the_palette() {
    let mut app = App::explore();
    app.commands = commands::vocabulary();
    app.on_key(key(KeyCode::Char('/')), &[]);
    app.on_key(key(KeyCode::Char('q')), &[]);
    assert_eq!(app.mode, Mode::Menu);
    app.on_key(key(KeyCode::Backspace), &[]);
    assert_eq!(app.mode, Mode::Menu, "erasing the query is not leaving");
    app.on_key(key(KeyCode::Backspace), &[]);
    assert_eq!(
      app.mode,
      Mode::Omni,
      "erasing the sigil must return to the composer"
    );
    assert!(app.omnibox.buffer.is_empty());
  }

  /// **REACHABILITY INTO THE MACHINE AND REACHABILITY OUT OF IT ARE TWO
  /// PROPERTIES, AND UNTIL NOW WE TESTED ONE.**
  ///
  /// `keys::every_declared_trigger_is_reachable_from_some_key` proves a key
  /// PRODUCES each trigger.
  /// `keys::every_key_the_map_binds_moves_the_machine_from_the_mode_it_was_pressed_in`
  /// proves the machine ANSWERS it. **Neither asks whether anything ACTS**, so
  /// both passed for the whole life of `Hotkey` -- declared, emitted,
  /// reachable, answered by an edge, and consumed by no realiser. It was a
  /// dead key with a clean bill of health, and it was found by hv pressing a
  /// letter at a menu and watching the body scroll behind it. **That is not a
  /// detection mechanism.**
  ///
  /// So this closes the third side: every `(mode, trigger)` the machine
  /// declares is DRIVEN through [`App::on_key`] in a state where it is
  /// meaningful, and something observable must happen -- a returned [`Step`]
  /// other than `Continue`, or a changed `App`. **Comparing the whole `App`
  /// rather than a chosen field is what stops this from being the next test
  /// that only looks like it checks something.**
  ///
  /// The exemptions are DECLARED with reasons, for the same purpose
  /// [`mode::ESC_NOT_OURS`] is declared: a predicate that skipped pairs which
  /// happen to do nothing would have skipped `Hotkey` too.
  ///
  /// **DRIVEN TO RED BEFORE BEING TRUSTED.** Un-exempting `EMBED + Typing` --
  /// a pair known to be inert here because the run loop forwards it -- and
  /// arming it makes this fail with the message below. A test of this shape
  /// that has never been seen to fail is decoration.
  ///
  /// **WHAT IT DOES NOT CATCH, STATED SO NOBODY READS IT AS MORE:** a
  /// MIS-ROUTED key, which changes something wrong rather than nothing at all.
  /// `MENU + Move` leaking into the body cursor would still change the `App`
  /// and still pass here -- and that was a real defect, so it has a real test
  /// of its own in
  /// `arrows_in_the_palette_move_the_pick_and_leave_the_body_alone`. This one
  /// is the DEAD-key check; routing is asserted per pair, where the right
  /// answer is known.
  #[test]
  fn every_trigger_the_machine_answers_is_acted_on_by_the_realiser() {
    const NOT_THE_APPS_TO_ACT_ON: &[(Mode, &str, &str)] = &[
      (
        Mode::Embed,
        "Typing",
        "the child owns the terminal and `run` forwards the keystroke to it; acting here would \
         mean the TUI and $EDITOR both consumed one key",
      ),
      (
        Mode::Embed,
        "ChildExit",
        "no key produces it (`keys::NOT_FROM_A_KEY`) -- the run loop calls `child_exited` when \
         the process ends",
      ),
    ];

    let mut examined = 0usize;
    let mut exempted = 0usize;
    for &m in Mode::ALL {
      for trigger in mode::EDGES.iter().filter(|e| e.from == m).map(|e| e.on) {
        if let Some((_, _, why)) = NOT_THE_APPS_TO_ACT_ON
          .iter()
          .find(|(em, et, _)| *em == m && *et == trigger)
        {
          assert!(
            !why.trim().is_empty(),
            "an exemption with no reason forgives nothing"
          );
          exempted += 1;
          continue;
        }
        let Some((mut app, rows, k)) = armed(m, trigger) else {
          panic!(
            "{m:?} + {trigger:?} is declared by the machine and has no armed state here, so the \
             pair is DECLARED and UNDRIVEN -- the hole this test exists to close, reopened by \
             omission"
          );
        };
        let before = app.clone();
        let step = app.on_key(k, &rows);
        assert!(
          step != Step::Continue || app != before,
          "{m:?} + {trigger:?} is declared by the machine and the realiser does NOTHING with it: \
           bound, reaching the machine, and inert. This is the `Hotkey` shape"
        );
        examined += 1;
      }
    }
    assert!(
      examined > 0,
      "no pair was driven, so this test asserted nothing"
    );
    assert_eq!(
      exempted,
      NOT_THE_APPS_TO_ACT_ON.len(),
      "an exemption was declared and never reached, so it is excusing nothing"
    );
  }

  /// A state in which `(mode, trigger)` is MEANINGFUL, and the key that fires
  /// it. **Armed deliberately per pair**: driving `Esc` on an empty composer
  /// or `Back` at the root would find a no-op that IS the design, and a test
  /// that accepted those would accept a dead key too.
  fn armed(m: Mode, trigger: &str) -> Option<(App, Vec<Row>, KeyEvent)> {
    let mut app = App::explore();
    app.commands = commands::vocabulary();
    let rows = item_rows();
    app.point_at(rows.len());
    Some(match (m, trigger) {
      (Mode::Omni, "Typing") => (app, rows, key(KeyCode::Char('a'))),
      (Mode::Omni, "Move") => (app, rows, key(KeyCode::Down)),
      (Mode::Omni, "Enter") => {
        app.omnibox.type_char('z');
        (app, rows, key(KeyCode::Enter))
      }
      (Mode::Omni, "/") => (app, rows, key(KeyCode::Char('/'))),
      (Mode::Omni, "Esc") => {
        app.omnibox.type_char('z');
        (app, rows, key(KeyCode::Esc))
      }
      (Mode::Omni, "Back") => {
        app.push(View::Collection {
          kind: "thread".into(),
        });
        (app, rows, key(KeyCode::Backspace))
      }
      (Mode::Menu, t) => {
        app.on_key(key(KeyCode::Char('/')), &rows);
        let k = match t {
          "Typing" => key(KeyCode::Char('q')),
          "Move" => key(KeyCode::Down),
          "Enter" => key(KeyCode::Enter),
          "Esc" => key(KeyCode::Esc),
          "Cancel" => KeyEvent::new(KeyCode::Char('g'), KeyModifiers::CONTROL),
          "/" => key(KeyCode::Char('/')),
          _ => return None,
        };
        (app, rows, k)
      }
      (Mode::Field, t) => {
        app.mode = Mode::Field;
        app.begin_edit(
          Handoff {
            kind: "thread".into(),
            id: "ST0056".into(),
            field: "title".into(),
          },
          "seed".into(),
        );
        let k = match t {
          "Typing" => key(KeyCode::Char('x')),
          "Enter" => key(KeyCode::Enter),
          "Esc" => key(KeyCode::Esc),
          _ => return None,
        };
        (app, rows, k)
      }
      _ => return None,
    })
  }
}
