//! `/help`: the whole reference, DERIVED from the things it describes.
//!
//! **hv ASKED FOR THIS AND ASKED WHETHER `$EDITOR` IN READ-ONLY MODE WOULD DO
//! IT (2026-09-02). IT WOULD NOT, AND THE REASON IS WORTH RECORDING BECAUSE IT
//! IS NOT OBVIOUS: THERE IS NO PORTABLE READ-ONLY MODE FOR AN EDITOR.** `vim`
//! and `nvim` take `-R`, `nano` takes `-v`, `emacs` needs an `--eval`, and
//! `code` has nothing at all -- and `$EDITOR` is whatever the operator set, so
//! the flag cannot be chosen without knowing which program it is. Guessing is
//! the fallback chain `render::launch_editor` already refuses by name. The tool
//! whose entire contract IS read-only full-screen text is a PAGER, which would
//! work -- but it is not needed, because help is not prose.
//!
//! **HELP IS A TABLE, AND THE BODY IS A TABLE.** Keys, commands and settings
//! are all `{name, meaning}` pairs, which is exactly the two-column form
//! `AC-17.11` already guarantees alignment for. So `/help` is a VIEW, it
//! scrolls with the machinery every other view scrolls with, it needs no child
//! process, no terminal handoff and no read-back, and nothing can be left on
//! disk by it.
//!
//! # Nothing here is written out
//!
//! **A HAND-WRITTEN HELP PAGE IS A SECOND HOME FOR THE KEYMAP**, and it goes
//! stale exactly the way a hand-written field list does -- `AC-17.2`'s argument
//! applied to documentation. Worse than a stale field list, because **nothing
//! reads help**: a form that lost a field is noticed the first time somebody
//! looks for it, and a help page that describes last month's keys is believed.
//!
//! So every row below is derived from the declaration it describes:
//!
//! - **the key table from [`super::mode::EDGES`]** -- the same ratified table
//!   `tui-design.md` section 3 transcribes, so the help and the machine cannot
//!   disagree without the transcription test failing first;
//! - **the commands from [`super::commands::vocabulary`]**, which is already
//!   the one place a command can exist;
//! - **the chords from [`super::keys::CHORDS`] and the vi keys from
//!   [`super::keys::VI_KEYS`]** -- the only two rosters here, and both are held
//!   TWO-SIDED against the functions they describe over the whole
//!   printable-ASCII population, which is enumerable. A binding added without a
//!   roster row fails; a roster row naming an unbound key fails.
//! - **the settings from [`intentsvcs::settings::DECLARED`]**, the allow-list
//!   `AC-17.14` requires.

use super::commands;
use super::keys;
use super::layout::Row;
use super::mode;

/// A heading row. **`label` IS NOT A DOOR AND NEVER WILL BE** -- see
/// [`super::mode::BY_ROW_KIND`], where Enter on one is a declared no-op rather
/// than a key that falls through to a default arm and says nothing.
fn heading(text: &str) -> Row {
  Row::new(text.to_string(), String::new(), "label")
}

/// One reference line: what you press, and what it does.
fn entry(key: impl Into<String>, says: impl Into<String>) -> Row {
  Row::new(key.into(), says.into(), "label")
}

/// A blank line, so the sections do not run together.
fn gap() -> Row {
  Row::new(String::new(), String::new(), "label")
}

/// The whole help page, for the composer keymap currently in force.
///
/// **THE vi SECTION APPEARS ONLY UNDER THE vi KEYMAP, and that is the same rule
/// the palette follows**: a page that listed normal-mode keys to an operator
/// running emacs would be offering a vocabulary that does nothing when used.
/// The SETTING is listed either way, so the route to the other keymap is always
/// on the page.
pub fn rows(keymap: keys::Keymap) -> Vec<Row> {
  let mut out = vec![heading("keys")];
  // **ONE ROW PER DECLARED EDGE, IN TABLE ORDER**, so the help page IS the
  // machine rather than a description of it. The notes column is already
  // written for an operator -- it is what `tui-design.md` section 3 puts in
  // front of a human reader -- so there is nothing here to paraphrase.
  for edge in mode::EDGES {
    out.push(entry(
      format!("{}  {}", edge.from.lamp(), edge.on),
      edge.note,
    ));
  }

  out.push(gap());
  out.push(heading("editing the composer"));
  for (c, says) in keys::CHORDS {
    out.push(entry(format!("Ctrl-{c}"), *says));
  }

  if keymap == keys::Keymap::Vi {
    out.push(gap());
    out.push(heading(
      "vi normal mode -- esc to enter, esc again to clear",
    ));
    for (c, says) in keys::VI_KEYS {
      out.push(entry(c.to_string(), *says));
    }
  }

  out.push(gap());
  out.push(heading("commands -- press / to open the palette"));
  for c in commands::vocabulary() {
    out.push(entry(format!("/{}", c.name), c.blurb));
  }

  out.push(gap());
  out.push(heading("settings -- /settings, or /settings <name>"));
  for s in intentsvcs::settings::DECLARED {
    out.push(entry(
      s.path.to_string(),
      format!("{} ({})", s.blurb, s.values.join(" | ")),
    ));
  }

  out
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **EVERY DECLARED EDGE REACHES THE PAGE.** The population is the machine's
  /// own table, so an edge added to it without appearing here is a key the
  /// operator cannot look up -- and this is what makes the page derived rather
  /// than merely generated once.
  #[test]
  fn every_edge_the_machine_declares_is_on_the_page() {
    let page = rows(keys::Keymap::Emacs);
    assert!(!mode::EDGES.is_empty(), "an empty machine asserts nothing");
    for edge in mode::EDGES {
      assert!(
        page
          .iter()
          .any(|r| r.title.contains(edge.on) && r.value == edge.note),
        "{} + {} is a declared edge and `/help` does not mention it",
        edge.from.lamp(),
        edge.on
      );
    }
  }

  /// **EVERY COMMAND AND EVERY SETTING TOO**, from the same two declarations
  /// the palette and the settings view read.
  #[test]
  fn every_command_and_every_setting_is_on_the_page() {
    let page = rows(keys::Keymap::Emacs);
    for c in commands::vocabulary() {
      assert!(
        page.iter().any(|r| r.title == format!("/{}", c.name)),
        "`/{}` is offered by the palette and `/help` does not mention it",
        c.name
      );
    }
    for s in intentsvcs::settings::DECLARED {
      assert!(
        page.iter().any(|r| r.title == s.path),
        "`{}` is a declared setting and `/help` does not mention it",
        s.path
      );
    }
  }

  /// **THE vi SECTION FOLLOWS THE KEYMAP IN FORCE**, so an operator running
  /// emacs is never taught a vocabulary that does nothing when they use it --
  /// and one running vi always is.
  #[test]
  fn the_vi_keys_appear_under_vi_and_not_under_emacs() {
    let emacs = rows(keys::Keymap::Emacs);
    let vi = rows(keys::Keymap::Vi);
    let names = |page: &[Row]| -> Vec<String> { page.iter().map(|r| r.title.clone()).collect() };

    for (c, _) in keys::VI_KEYS {
      assert!(
        names(&vi).contains(&c.to_string()),
        "vi `{c}` is bound and the vi page does not mention it"
      );
    }
    assert!(
      vi.len() > emacs.len(),
      "the vi page is no longer than the emacs one, so the section is not there at all"
    );
    // The control: the SETTING that switches keymaps is on both pages, or an
    // operator on the wrong one has no way to find the other.
    for page in [&emacs, &vi] {
      assert!(
        page.iter().any(|r| r.title == "editing.mode"),
        "the keymap setting is missing, so this page is a dead end"
      );
    }
  }

  /// A row with nothing in either column teaches nothing. Blank SPACERS are
  /// deliberate and are the one exception, so they are counted rather than
  /// exempted by a predicate that would forgive an empty entry too.
  #[test]
  fn every_row_that_is_not_a_spacer_says_something() {
    let page = rows(keys::Keymap::Vi);
    let spacers = page
      .iter()
      .filter(|r| r.title.is_empty() && r.value.is_empty())
      .count();
    assert!(
      spacers > 0 && spacers < 8,
      "spacer count looks wrong: {spacers}"
    );
    for r in &page {
      assert!(
        !(r.title.trim().is_empty() && !r.value.trim().is_empty()),
        "a row explains something it does not name: {:?}",
        r.value
      );
    }
    assert!(
      page.iter().all(|r| r.kind == "label"),
      "a help row carries a kind that makes it look actionable"
    );
  }
}
