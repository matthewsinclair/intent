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
//! **`Tab` produces no trigger.** *Pane focus is a GUARD on NORMAL's edges, not
//! a sixth mode* -- it changes where Move and Enter land and not what the keys
//! mean, so it is the app's state and not the machine's.
//!
//! **`Enter` from NORMAL is ambiguous by design and stays that way here.** The
//! machine declares two edges -- to FIELD for editable rows, to EMBED for prose
//! rows -- and `tui-design.md` §3 guards it on the ROW, which this module
//! cannot see. Resolving it here would put the row-kind rule in the keymap,
//! where the next reader would not look for it.

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

/// The trigger `key` produces in `mode`, or `None` when the keymap says nothing.
///
/// **`None` MEANS "NOT A KEY WE BIND", and the caller must not invent a
/// self-loop for it** -- that is the same trap [`super::mode::step`] documents,
/// one layer out.
pub fn trigger(mode: Mode, key: KeyEvent) -> Option<&'static str> {
  // `tui-design.md` §4: `:` and `/` go straight from NORMAL, with no Esc-first
  // step, and `/` is the MENU key rather than a second command prefix.
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
    (Mode::Normal, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) => Some("Move"),
    (Mode::Menu, KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right) => Some("Move"),
    (_, KeyCode::Enter) => Some("Enter"),
    (Mode::Normal, KeyCode::Char(':')) => Some(":"),
    (Mode::Normal, KeyCode::Char('/')) => Some("/"),
    (Mode::Normal, KeyCode::Backspace) => Some("Back"),
    (Mode::Menu, KeyCode::Backspace) => Some("Back"),
    // A menu accelerator. Found by POSITION in the label rather than assumed to
    // be the first character, which is the menu's own rule -- but which letter
    // is live is the menu's business, so any character offers itself here and
    // the menu refuses the ones it does not bind.
    (Mode::Menu, KeyCode::Char(_)) => Some("Hotkey"),
    // Text goes to whatever is collecting it. EMBED forwards to the child.
    (Mode::Field | Mode::Command, KeyCode::Char(_) | KeyCode::Backspace) => Some("Typing"),
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
  /// mode machine proves Esc walks toward the rest state; this proves the key
  /// actually reaches the machine from every mode. Both are needed: an Esc edge
  /// nothing presses is not an escape.
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

  /// Pane focus is a guard on NORMAL's edges, not a sixth mode -- so `Tab` must
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

  /// Typing must reach the collector in every mode that collects, and must not
  /// in NORMAL, where a bare letter is a command rather than text.
  #[test]
  fn typing_reaches_the_collectors_and_not_the_rest_state() {
    let a = KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE);
    for mode in [Mode::Field, Mode::Command, Mode::Embed] {
      assert_eq!(
        trigger(mode, a),
        Some("Typing"),
        "{mode:?} must collect text"
      );
    }
    assert_ne!(
      trigger(Mode::Normal, a),
      Some("Typing"),
      "a bare letter in NORMAL must not be swallowed as text"
    );
  }

  /// **THE WHOLE KEYMAP, DRIVEN THROUGH THE MACHINE.** A trigger being declared
  /// somewhere is weaker than the machine answering it FROM THE MODE THE KEY
  /// WAS PRESSED IN: `Typing` is declared, but not by any edge out of NORMAL.
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
