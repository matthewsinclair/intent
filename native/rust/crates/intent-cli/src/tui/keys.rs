//! The keymap: `tui-design.md` §4, held against [`super::mode`]'s own table.
//!
//! **A KEYMAP IS A SECOND HOME FOR THE TRIGGER VOCABULARY, AND THIS MODULE
//! EXISTS TO STOP IT BEING ONE.** The strings `"Move"`, `"Enter"`, `"Esc"` and
//! the rest are declared in `EDGES`; a keymap that spells one of them
//! differently produces a trigger no edge answers, [`super::mode::step`]
//! returns `None`, and the key is silently ignored. **Nothing fails, nothing
//! logs, and the key just does not work** -- which is the silent class this
//! estate's contract forbids, arriving through a typo.
//!
//! So the vocabulary is asserted in BOTH directions, and the two catch
//! different things:
//!
//! - **Every trigger this module can emit is declared by an edge.** Catches a
//!   misspelt or invented trigger: a key wired to nothing.
//! - **Every declared trigger is reachable from some key.** Catches the
//!   opposite: an edge added to the machine that no keystroke can ever take, a
//!   feature that reads as built and is not. The exemptions are declared
//!   rather than filtered, for the reason [`super::mode::ESC_NOT_OURS`] is
//!   declared -- a predicate that skipped unreachable triggers would also skip
//!   the one that became unreachable by accident.
//!
//! # What is deliberately NOT here
//!
//! **`Tab` produces no trigger.** *Pane focus is a GUARD on OMNI's edges, not
//! a fifth mode* -- it changes where Move and Enter land and not what the keys
//! mean, so it is the app's state and not the machine's.
//!
//! **THE BUFFER GUARD IS NOT HERE EITHER, AND THAT IS THE LOAD-BEARING
//! OMISSION.** Since `NAV` folded into the composer, three keys mean two
//! things apiece depending on whether the buffer is empty: `/` opens the menu
//! or types a slash, `Backspace` pops the view or deletes a character, and
//! arrows browse the body or pick among matches. **This module cannot see the
//! buffer**, so in every case it emits the MODE-SIGNIFICANT trigger and the
//! app downgrades it -- the rule `/` has always followed here. Spelling the
//! guard into the trigger instead (`Enter (buffer empty)`) was drafted and
//! refused for this exact reason: it would name a trigger no keystroke can
//! produce.
//!
//! **`Enter` from OMNI is ambiguous by design and stays that way here.** The
//! machine declares three edges -- descend for door rows, FIELD for editable
//! rows, EMBED for prose rows -- and `tui-design.md` §3 guards the triple on
//! the ROW, which this module cannot see. Resolving it here would put the
//! row-kind rule in the keymap, where the next reader would not look for it.
//!
//! **`Ctrl-C` produces no trigger either.** Quitting is an act of the SHELL
//! the realiser answers before the machine is consulted, the way `Tab` is --
//! `tui-design.md` §3: *quitting is now an act, never an accident*, and an
//! edge for it would put QUIT in the graph as a mode.
//!
//! **THERE ARE NO SINGLE-LETTER BINDINGS OUTSIDE `MENU`, and that is a design
//! cost the design pays on purpose** (`tui-design.md` §4): the composer always
//! holds the keyboard, which is the Claude Code affordance -- you never select
//! the input before typing. A letter bound to a verb would be a letter the
//! composer never receives.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::mode::{EDGES, Mode};

/// Triggers no keystroke can produce, with the reason.
///
/// **DECLARED, NOT FILTERED.** A predicate excluding triggers that happen to be
/// unreachable would also excuse an edge that became unreachable by accident,
/// and the two look identical to a filter. As a declared pair, a second entry
/// is an edit somebody has to justify -- and the assertion below is an
/// EQUALITY, so adding one goes red.
pub const NOT_FROM_A_KEY: &[(&str, &str)] = &[(
  "ChildExit",
  "the child editor exiting is an event from the process, not from the operator. EMBED's only \
   exit is the child ending, which is the one mode the TUI cannot get you out of -- stated in \
   `tui-design.md` §7 as the cost of embedding rather than discovered by an operator holding Esc.",
)];

/// One editing action on the composer's buffer.
///
/// **THE COMPOSER IS AN INPUT AND OPERATORS ALREADY KNOW HOW INPUTS WORK.**
/// hv, 2026-09-02, driving the build: *we need normal terminal editing keys in
/// the omnibox.* Before this, `C-a` typed an `a`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edit {
  Insert(char),
  Backspace,
  DeleteForward,
  Home,
  End,
  Left,
  Right,
  KillToEnd,
  KillToStart,
  KillWordBack,
}

/// The composer's editing keymap: readline's emacs bindings, which are the
/// terminal default nearly everywhere.
///
/// **THESE ARE NOT MODE TRIGGERS AND MUST NOT BECOME ONE.** Every action here
/// changes the BUFFER and leaves the mode alone, so [`super::mode`] has
/// nothing to say about them -- they all arrive as `Typing`, the self-loop
/// that already means *the composer collects this keystroke*. A trigger per
/// motion would put ten self-loops into a table whose entire value is being
/// readable as a graph.
///
/// **VI MODE IS NOT HERE, AND ITS ABSENCE IS A DECISION RATHER THAN A GAP.**
/// hv asked for vi bindings under `set -o vi`; that shell setting is not
/// visible to a child process at all -- measured, not assumed: `SHELLOPTS` is
/// bash-only and absent under zsh, nothing in the environment carries it, and
/// `~/.inputrc` is readline's file which zsh never reads. So the mode has to
/// be DECLARED rather than detected, which is `ST0037`'s ruling one surface
/// over: explicit configuration beat filesystem probing for languages and it
/// beats environment probing here. hv ruled a settings file plus a
/// `/settings` command; **vi lands when that lands**, and this map is what the
/// default resolves to.
pub fn edit(key: KeyEvent) -> Option<Edit> {
  let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
  Some(match (key.code, ctrl) {
    (KeyCode::Left, false) => Edit::Left,
    (KeyCode::Right, false) => Edit::Right,
    (KeyCode::Home, _) => Edit::Home,
    (KeyCode::End, _) => Edit::End,
    (KeyCode::Backspace, false) => Edit::Backspace,
    (KeyCode::Delete, _) => Edit::DeleteForward,
    (KeyCode::Char('a'), true) => Edit::Home,
    (KeyCode::Char('e'), true) => Edit::End,
    (KeyCode::Char('b'), true) => Edit::Left,
    (KeyCode::Char('f'), true) => Edit::Right,
    (KeyCode::Char('d'), true) => Edit::DeleteForward,
    (KeyCode::Char('k'), true) => Edit::KillToEnd,
    (KeyCode::Char('u'), true) => Edit::KillToStart,
    (KeyCode::Char('w'), true) => Edit::KillWordBack,
    (KeyCode::Char('h'), true) => Edit::Backspace,
    // **A CONTROL CHORD THIS MAP DOES NOT KNOW IS SWALLOWED, NEVER TYPED.**
    // `C-x` inserting an `x` is the defect hv reported, and the arm below is
    // what stops the next unbound chord doing it again.
    (KeyCode::Char(_), true) => return None,
    (KeyCode::Char(c), false) => Edit::Insert(c),
    _ => return None,
  })
}

/// The trigger `key` produces in `mode`, or `None` when the keymap says nothing.
///
/// **`None` MEANS "NOT A KEY WE BIND", and the caller must not invent a
/// self-loop for it** -- that is the same trap [`super::mode::step`] documents,
/// one layer out.
pub fn trigger(mode: Mode, key: KeyEvent) -> Option<&'static str> {
  // **EMBED SWALLOWS EVERYTHING, INCLUDING Esc, AND IT IS FIRST FOR THAT
  // REASON.** `mode::ESC_NOT_OURS` declares EMBED the one mode whose escape is
  // not ours: a child process owns the terminal while it runs, so Esc reaches
  // `$EDITOR`. A blanket Esc arm above this one emitted `"Esc"` here and the
  // machine had no edge to answer it -- a bound, inert key, found by
  // `every_key_the_map_binds_moves_the_machine_from_the_mode_it_was_pressed_in`
  // rather than by an operator holding Esc in a full-screen editor.
  if mode == Mode::Embed {
    return Some("Typing");
  }
  let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
  match (mode, key.code) {
    // **`C-g` CANCELS THE MENU, RULED BY ic UNDER AC-05.1 RATHER THAN
    // INVENTED.** `tui-design.md` §3 lists `Cancel` as one of MENU's exits and
    // names no key for it, and §7 resolves the default keymap to EMACS -- where
    // `C-g` IS cancel. Without a binding the edge was unreachable, which
    // `every_declared_trigger_is_reachable_from_some_key` caught: a menu exit
    // that reads as built and no operator can take.
    (Mode::Menu, KeyCode::Char('g')) if ctrl => Some("Cancel"),
    (_, KeyCode::Esc) => Some("Esc"),
    (_, KeyCode::Enter) => Some("Enter"),
    // **THE COMPOSER'S EDITING KEYS ALL ARRIVE AS `Typing`**, because they
    // change the buffer and not the mode -- see [`edit`]. Left and Right were
    // held unbound by `tui-design.md` §4 *against a cursor the buffer does not
    // yet have*; the buffer has one now, so the reservation is spent on the
    // meaning it was reserved for.
    (
      Mode::Omni | Mode::Menu,
      KeyCode::Left | KeyCode::Right | KeyCode::Home | KeyCode::End | KeyCode::Delete,
    ) => Some("Typing"),
    // **ONLY THE VERTICAL PAIR IS BOUND, and it now does both jobs**: on an
    // empty buffer the arrows browse the body, with a query typed they pick
    // among the matches. One trigger covers both because both are `OMNI Move
    // -> OMNI` -- the difference is the app's guard, not the machine's edge.
    // Left and Right stay reserved against a cursor the buffer does not yet
    // have; binding them today would teach a meaning tomorrow's cursor
    // contradicts.
    (Mode::Omni, KeyCode::Up | KeyCode::Down) => Some("Move"),
    // `/` is the MENU key ONLY on an empty buffer -- `st/ST0056` is a legal
    // address (`tui-design.md` §3). The guard is the app's, the way the pane
    // guard is: this map cannot see the buffer, so it offers the trigger and
    // the app reroutes a mid-address `/` to `Typing`.
    (Mode::Omni, KeyCode::Char('/')) => Some("/"),
    // **Backspace FOLLOWS `/`'s RULE, and it has to.** With NAV folded in, one
    // key must both pop the view stack and delete a character. The
    // mode-significant reading is offered here and the app downgrades it to
    // `Typing` while the buffer holds anything -- the same shape as `/`,
    // deliberately, so there is ONE rule for guarded keys rather than one per
    // key.
    (Mode::Omni, KeyCode::Backspace) => Some("Back"),
    (Mode::Omni, KeyCode::Char(_)) => Some("Typing"),
    // **ONLY THE VERTICAL PAIR, because the palette is a LIST and not a bar.**
    // The Lotus design moved along a horizontal menu, so it bound all four;
    // hv ruled the filtered palette in its place on 2026-09-02 and a list has
    // no left or right.
    (Mode::Menu, KeyCode::Up | KeyCode::Down) => Some("Move"),
    // **`/` CLOSES THE PALETTE**, which is the whole of its meaning now that
    // the three-way ring is retired: one key, one job, from either side.
    // Bound before the text arm so the sigil never lands in its own query.
    (Mode::Menu, KeyCode::Char('/')) => Some("/"),
    // **A LETTER FILTERS; IT IS NOT AN ACCELERATOR.** The retired bar bound
    // letters to `Hotkey`, which nothing ever consumed -- bound, reaching the
    // machine, and inert, because the invariant that guards this only asks
    // whether the MACHINE has an edge and not whether the app does anything.
    // In a palette the letter has an obvious job, so the dead trigger is
    // RETIRED rather than given a handler. Backspace erases, and erasing back
    // past the sigil is how you leave -- no separate exit key to declare.
    (Mode::Menu, KeyCode::Char(_) | KeyCode::Backspace) => Some("Typing"),
    (Mode::Field, KeyCode::Char(_) | KeyCode::Backspace) => Some("Typing"),
    _ => None,
  }
}

/// Every trigger `EDGES` declares, deduplicated, in first-declared order.
pub fn declared_triggers() -> Vec<&'static str> {
  let mut out: Vec<&'static str> = Vec::new();
  for e in EDGES {
    if !out.contains(&e.on) {
      out.push(e.on);
    }
  }
  out
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeSet;

  /// Every key this estate binds or might plausibly meet, so the sweeps below
  /// are over a corpus rather than over the cases that happen to work.
  fn every_key() -> Vec<KeyEvent> {
    let mut keys: Vec<KeyCode> = vec![
      KeyCode::Esc,
      KeyCode::Enter,
      KeyCode::Tab,
      KeyCode::BackTab,
      KeyCode::Backspace,
      KeyCode::Delete,
      KeyCode::Up,
      KeyCode::Down,
      KeyCode::Left,
      KeyCode::Right,
      KeyCode::Home,
      KeyCode::End,
      KeyCode::PageUp,
      KeyCode::PageDown,
      KeyCode::Insert,
      KeyCode::F(1),
    ];
    // **`g` IS IN THIS LIST BECAUSE ITS ABSENCE HID A BINDING.** The first
    // version swept `"abeqxzAZ:/?01 -_."`, which contains no `g`, so `C-g`
    // was never pressed and `Cancel` read as unreachable AFTER it had been
    // bound. A sweep is only as good as its alphabet: a corpus that cannot
    // press a key cannot see what that key does.
    for c in "abcdefghijklmnopqrstuvwxyzAZ:/?01 -_.".chars() {
      keys.push(KeyCode::Char(c));
    }
    let mut out = Vec::new();
    for code in keys {
      for m in [
        KeyModifiers::NONE,
        KeyModifiers::CONTROL,
        KeyModifiers::SHIFT,
      ] {
        out.push(KeyEvent::new(code, m));
      }
    }
    out
  }

  #[test]
  fn the_corpus_is_not_empty_and_neither_is_the_vocabulary() {
    assert!(
      every_key().len() > 40,
      "too few keys swept for the sweeps below to mean anything"
    );
    assert!(
      !declared_triggers().is_empty(),
      "the machine declares no triggers at all"
    );
  }

  /// **A KEY WIRED TO A TRIGGER NO EDGE ANSWERS IS A KEY THAT SILENTLY DOES
  /// NOTHING.** `step` returns `None`, nothing fails, nothing logs, and the key
  /// just does not work.
  #[test]
  fn every_trigger_this_keymap_emits_is_declared_by_an_edge() {
    let declared: BTreeSet<&str> = declared_triggers().into_iter().collect();
    let mut emitted = 0usize;
    for &mode in Mode::ALL {
      for key in every_key() {
        if let Some(t) = trigger(mode, key) {
          assert!(
            declared.contains(t),
            "{mode:?} + {:?} emits {t:?}, which no edge declares. The machine would say nothing \
             and the key would silently do nothing",
            key.code
          );
          emitted += 1;
        }
      }
    }
    assert!(
      emitted > 0,
      "the keymap emitted no trigger at all, so this test asserted nothing"
    );
  }

  /// The converse, which catches the opposite defect: an edge nothing can take.
  /// Asserted as an EQUALITY against the declared exemptions, so a newly
  /// unreachable trigger goes red rather than joining a silent tail.
  #[test]
  fn every_declared_trigger_is_reachable_from_some_key() {
    let mut reachable: BTreeSet<&str> = BTreeSet::new();
    for &mode in Mode::ALL {
      for key in every_key() {
        if let Some(t) = trigger(mode, key) {
          reachable.insert(t);
        }
      }
    }
    let unreachable: BTreeSet<&str> = declared_triggers()
      .into_iter()
      .filter(|t| !reachable.contains(t))
      .collect();
    let exempt: BTreeSet<&str> = NOT_FROM_A_KEY.iter().map(|(t, _)| *t).collect();
    assert_eq!(
      unreachable, exempt,
      "the set of declared triggers no key can produce changed. An edge nothing can take is a \
       feature that reads as built and is not; add it to NOT_FROM_A_KEY with its reason, or bind \
       a key to it"
    );
  }

  /// **AN EXEMPTION THAT NAMES NOTHING FORGIVES NOTHING**, and the check would
  /// then pass for a reason that is no longer true.
  #[test]
  fn every_exemption_names_a_real_trigger_and_states_why() {
    let declared: BTreeSet<&str> = declared_triggers().into_iter().collect();
    for (t, why) in NOT_FROM_A_KEY {
      assert!(
        declared.contains(t),
        "NOT_FROM_A_KEY names {t:?}, which no edge declares"
      );
      assert!(
        !why.trim().is_empty(),
        "{t:?} is exempted with no stated reason"
      );
    }
  }

  /// **ESC IS TOTAL, WHICH IS `AC-17.9`'s HALF THAT LIVES IN THE KEYMAP.** The
  /// mode machine proves Esc lands in a home mode; this proves the key
  /// actually reaches the machine from every mode. Both are needed: an Esc
  /// edge nothing presses is not an escape.
  #[test]
  fn esc_reaches_the_machine_from_every_mode_that_owns_its_escape() {
    let exempt: BTreeSet<&str> = super::super::mode::ESC_NOT_OURS
      .iter()
      .map(|(m, _)| m.name())
      .collect();
    let mut checked = 0usize;
    for &mode in Mode::ALL {
      if exempt.contains(mode.name()) {
        assert_eq!(
          trigger(mode, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)),
          Some("Typing"),
          "{mode:?} does not own its escape, so Esc must FORWARD rather than reach the machine"
        );
        continue;
      }
      for m in [KeyModifiers::NONE, KeyModifiers::SHIFT] {
        assert_eq!(
          trigger(mode, KeyEvent::new(KeyCode::Esc, m)),
          Some("Esc"),
          "Esc produces no trigger in {mode:?}, so the operator cannot leave it by pressing it"
        );
      }
      checked += 1;
    }
    assert!(
      checked > 0,
      "every mode was exempt, so this test asserted nothing"
    );
    assert!(
      !exempt.is_empty(),
      "no mode is exempt, so the forwarding half asserted nothing"
    );
  }

  /// Pane focus is a guard on OMNI's edges, not a fifth mode -- so `Tab` must
  /// not reach the machine at all.
  #[test]
  fn tab_is_not_a_mode_trigger_anywhere_the_tui_owns_the_keyboard() {
    let mut checked = 0usize;
    for &mode in Mode::ALL {
      let got = trigger(mode, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
      if mode == Mode::Embed {
        // The child owns every key here, Tab included -- forwarding it is the
        // point rather than an exception to it.
        assert_eq!(
          got,
          Some("Typing"),
          "EMBED must forward Tab to the child like anything else"
        );
        continue;
      }
      assert_eq!(
        got, None,
        "Tab produced a mode trigger in {mode:?}; pane focus is a guard"
      );
      checked += 1;
    }
    assert!(
      checked > 0,
      "every mode was skipped, so this test asserted nothing"
    );
  }

  /// Typing must reach the collector in **every** mode that collects -- and
  /// since hv ruled the filtered palette (2026-09-02) that includes `MENU`,
  /// which used to be the one exception.
  ///
  /// **THE `NAV` SEED IS GONE BECAUSE WHAT IT COMPENSATED FOR IS GONE.** While
  /// the cursor could live outside the input, a printable had to be CARRIED
  /// from NAV into the omnibox -- hv's you-just-start-typing affordance,
  /// implemented as a mode change with a character in flight. With the
  /// composer permanently holding the keyboard there is nothing to seed FROM:
  /// typing lands where the cursor already is. **The affordance survives; the
  /// machinery under it does not, which is the point of the collapse.**
  ///
  /// **AND `MENU` STOPPED BEING AN EXCEPTION FOR A BETTER REASON THAN
  /// CONVENIENCE.** Its letters used to emit `Hotkey`, a trigger the machine
  /// declared and no realiser consumed -- a bound, reachable, inert key. The
  /// palette gives every letter an obvious job, so the exception is gone and
  /// so is the dead trigger.
  #[test]
  fn typing_reaches_every_collector_including_the_palette() {
    let a = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    for mode in Mode::ALL {
      assert_eq!(
        trigger(*mode, a),
        Some("Typing"),
        "{mode:?} must collect text -- every mode is a collector now that the palette filters"
      );
    }
    assert_eq!(
      super::super::mode::step(Mode::Omni, "Typing"),
      Some(Mode::Omni),
      "Typing in the composer must SELF-LOOP -- an edge that left OMNI would move the operator \
       off the one home on every keystroke"
    );
    assert_eq!(
      super::super::mode::step(Mode::Menu, "Typing"),
      Some(Mode::Menu),
      "Typing in the palette must SELF-LOOP -- filtering is not leaving"
    );
  }

  /// **NO TRIGGER MAY BE DECLARED THAT NOTHING CONSUMES, and `Hotkey` is the
  /// proof this was worth asserting.** It was emitted by this keymap, declared
  /// as an edge, and handled by no realiser: every existing invariant passed
  /// while the key did nothing, because they all ask whether the MACHINE
  /// answers and none asks whether anything ACTS. hv found it by pressing a
  /// letter at a menu and watching the body scroll behind it.
  ///
  /// A full realiser-side check is not available from here -- this module
  /// cannot see `app.rs` -- so this pins the narrow, checkable half: the
  /// retired trigger is gone from the vocabulary entirely, in both directions.
  #[test]
  fn the_retired_accelerator_trigger_is_gone_from_the_vocabulary() {
    assert!(
      !declared_triggers().contains(&"Hotkey"),
      "`Hotkey` is declared again. It had no consumer for the whole life of the Lotus bar; if it \
       is back, something must ACT on it and not merely answer it"
    );
    for &mode in Mode::ALL {
      for key in every_key() {
        assert_ne!(
          trigger(mode, key),
          Some("Hotkey"),
          "{mode:?} still emits the retired accelerator trigger"
        );
      }
    }
  }

  /// **THE WHOLE KEYMAP, DRIVEN THROUGH THE MACHINE.** A trigger being declared
  /// somewhere is weaker than the machine answering it FROM THE MODE THE KEY
  /// WAS PRESSED IN: `Typing` is declared, but a mode could emit it with no
  /// edge of its own to answer it.
  #[test]
  fn every_key_the_map_binds_moves_the_machine_from_the_mode_it_was_pressed_in() {
    let mut moved = 0usize;
    for &mode in Mode::ALL {
      for key in every_key() {
        let Some(t) = trigger(mode, key) else {
          continue;
        };
        assert!(
          super::super::mode::step(mode, t).is_some(),
          "{mode:?} + {:?} emits {t:?} and the machine declares no edge out of {mode:?} for it, \
           so the key is bound and inert",
          key.code
        );
        moved += 1;
      }
    }
    assert!(
      moved > 0,
      "no key moved the machine, so this test asserted nothing"
    );
  }
}
