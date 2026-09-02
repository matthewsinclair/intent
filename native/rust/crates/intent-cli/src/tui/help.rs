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
use super::nav::View;

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

/// The CLI's own commands, read out of the `clap::Command` that `--help`
/// renders.
///
/// **hv's REQUIREMENT, VERBATIM: _it would be good if we didn't dupe it, but
/// rather got it from the same place that `--help` gets it from_.** This is
/// that place. `--help` is clap printing this tree; `spine::build` makes the
/// tree from `dispatch::Table`; and this walks the tree rather than the table,
/// because the tree is what an operator has actually seen. **A second list of
/// command names in a help page is the `AC-17.2` defect in its purest form** --
/// a command added to the surface would be absent here, silently, and the page
/// would go on looking complete.
///
/// `of` names a subcommand, so `/help st` is `intent st --help`. An unknown
/// spelling comes back as a REFUSAL row rather than an empty page: section 8's
/// rule -- *a spelling that names nothing is refused AS A SPELLING* -- and
/// section 8's other rule, that a view which cannot load renders an error row
/// and never an empty form.
pub fn cli_rows(root: &clap::Command, of: Option<&str>) -> Vec<Row> {
  let Some(at) = locate(root, of) else {
    let tried = of.unwrap_or_default();
    return vec![
      entry(
        format!("`{tried}` is not a command"),
        "`/help` on its own lists every command this build offers".to_string(),
      ),
      gap(),
    ];
  };

  let mut out = Vec::new();
  if let Some(about) = at.get_about() {
    out.push(heading(&about.to_string()));
    out.push(gap());
  }
  // **`get_subcommands` IS THE SAME ITERATOR CLAP'S OWN HELP WALKS**, so the
  // set here and the set on `--help` cannot differ. Hidden commands are
  // skipped for the same reason clap skips them: a surface that hides a
  // command from `--help` and lists it here would be two answers to one
  // question.
  let mut any = false;
  for sub in at.get_subcommands().filter(|c| !c.is_hide_set()) {
    any = true;
    let name = sub.get_name().to_string();
    let says = sub
      .get_about()
      .map(|a| a.to_string())
      .unwrap_or_else(String::new);
    // **A COMMAND WITH CHILDREN IS A DOOR; A LEAF IS NOT.** Descending into a
    // leaf would push a view whose only content is the line already on screen,
    // which is a door that leads back to what you were reading.
    let row = if sub.get_subcommands().next().is_some() {
      Row::named(name.clone(), name.clone(), says, "button").opening(View::Help {
        of: Some(match of {
          Some(parent) => format!("{parent} {name}"),
          None => name.clone(),
        }),
      })
    } else {
      Row::named(name.clone(), name, says, "label")
    };
    out.push(row);
  }
  if !any {
    out.push(entry(
      "(no subcommands)".to_string(),
      "this command takes arguments rather than a subcommand".to_string(),
    ));
  }
  out
}

/// The command `of` names, or the root when it names nothing.
///
/// **A SPACE-SEPARATED PATH, SO `/help st new` WORKS THE WAY IT READS.** The
/// palette's argument is already everything after the first space, so a nested
/// spelling arrives here whole rather than being a second thing to support.
fn locate<'c>(root: &'c clap::Command, of: Option<&str>) -> Option<&'c clap::Command> {
  let Some(path) = of.map(str::trim).filter(|s| !s.is_empty()) else {
    return Some(root);
  };
  let mut at = root;
  for segment in path.split_whitespace() {
    at = at.find_subcommand(segment)?;
  }
  Some(at)
}

/// The whole help page, for the composer keymap currently in force.
///
/// **THE vi SECTION APPEARS ONLY UNDER THE vi KEYMAP, and that is the same rule
/// the palette follows**: a page that listed normal-mode keys to an operator
/// running emacs would be offering a vocabulary that does nothing when used.
/// The SETTING is listed either way, so the route to the other keymap is always
/// on the page.
pub fn rows(keymap: keys::Keymap, cli: &clap::Command, of: Option<&str>) -> Vec<Row> {
  // **`/help <command>` IS THE CLI PAGE ALONE, AND THAT IS THE POINT OF
  // ASKING FOR ONE.** An operator who typed `/help st` wants `intent st
  // --help`; repeating the whole keyboard reference above it would bury the
  // answer under the thing they did not ask for.
  if of.is_some() {
    let mut out = vec![heading(&format!("intent {}", of.unwrap_or_default()))];
    out.push(gap());
    out.extend(cli_rows(cli, of));
    return out;
  }

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

  out.push(gap());
  out.push(heading(
    "intent commands -- /help <name>, or press enter on a row",
  ));
  out.extend(cli_rows(cli, None));

  out
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **THE REAL SHIPPED SURFACE, NOT A FIXTURE.** `intent help` is
  /// `spine::build(&dispatch::table()).render_help()`, so this is the same
  /// object -- which is what makes the assertions below claims about what an
  /// operator actually sees rather than about a tree this test invented.
  fn cli() -> clap::Command {
    crate::spine::build(&crate::dispatch::table())
  }

  /// **EVERY DECLARED EDGE REACHES THE PAGE.** The population is the machine's
  /// own table, so an edge added to it without appearing here is a key the
  /// operator cannot look up -- and this is what makes the page derived rather
  /// than merely generated once.
  #[test]
  fn every_edge_the_machine_declares_is_on_the_page() {
    let page = rows(keys::Keymap::Emacs, &cli(), None);
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
    let page = rows(keys::Keymap::Emacs, &cli(), None);
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

  /// **hv's REQUIREMENT AS A TEST: THE PAGE AND `intent help` CANNOT
  /// DISAGREE.** Not "the page lists some commands" -- every subcommand the
  /// shipped surface offers, with the about text clap would print for it. A
  /// command added to `dispatch::Table` and absent here fails, which is the
  /// whole reason the walk goes through `clap::Command` instead of a copy.
  #[test]
  fn every_command_the_cli_offers_reaches_the_page_with_its_own_about_text() {
    let root = cli();
    let page = rows(keys::Keymap::Emacs, &root, None);
    let mut checked = 0usize;
    for sub in root.get_subcommands().filter(|c| !c.is_hide_set()) {
      let name = sub.get_name();
      let row = page
        .iter()
        .find(|r| r.name == name)
        .unwrap_or_else(|| panic!("`intent {name}` is on the surface and not on the page"));
      if let Some(about) = sub.get_about() {
        assert_eq!(
          row.value,
          about.to_string(),
          "`{name}` is described differently on the page than on `--help`"
        );
      }
      checked += 1;
    }
    assert!(
      checked > 10,
      "only {checked} command(s) were examined -- the surface did not load"
    );
  }

  /// **`/help st` IS `intent st --help`, DRIVEN ON hv's OWN EXAMPLE.** The
  /// argument page carries that command's subcommands and NOT the root's, so
  /// an operator who asked about one command is not handed the whole surface.
  #[test]
  fn an_argument_narrows_the_page_to_that_commands_own_usage() {
    let root = cli();
    let st = root
      .find_subcommand("st")
      .expect("`intent st` must be on the shipped surface");
    let page = rows(keys::Keymap::Emacs, &root, Some("st"));

    for sub in st.get_subcommands().filter(|c| !c.is_hide_set()) {
      assert!(
        page.iter().any(|r| r.name == sub.get_name()),
        "`intent st {}` is missing from `/help st`",
        sub.get_name()
      );
    }
    // The narrowing is the point: a sibling of `st` must NOT be on this page.
    assert!(
      !page.iter().any(|r| r.name == "doctor"),
      "`/help st` listed a command that is not one of st's -- it did not narrow at all"
    );
    // Nor the keyboard reference, which is what the operator did not ask for.
    assert!(
      !page.iter().any(|r| r.title.starts_with("Ctrl-")),
      "`/help st` buried the answer under the whole keyboard reference"
    );
  }

  /// **AN UNKNOWN SPELLING IS REFUSED AS A SPELLING, NEVER AS AN EMPTY PAGE**
  /// -- section 8, and section 8's other rule that a view which cannot load
  /// renders an error row rather than a blank form.
  #[test]
  fn a_command_that_does_not_exist_is_refused_by_name() {
    let page = rows(keys::Keymap::Emacs, &cli(), Some("zzznotacommand"));
    assert!(
      page
        .iter()
        .any(|r| r.title.contains("zzznotacommand") || r.value.contains("zzznotacommand")),
      "the refusal does not say what was tried: {page:?}"
    );
    assert!(
      page.iter().any(|r| r.value.contains("/help")),
      "the refusal does not say how to find what IS a command"
    );
  }

  /// **THE vi SECTION FOLLOWS THE KEYMAP IN FORCE**, so an operator running
  /// emacs is never taught a vocabulary that does nothing when they use it --
  /// and one running vi always is.
  #[test]
  fn the_vi_keys_appear_under_vi_and_not_under_emacs() {
    let emacs = rows(keys::Keymap::Emacs, &cli(), None);
    let vi = rows(keys::Keymap::Vi, &cli(), None);
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
    let page = rows(keys::Keymap::Vi, &cli(), None);
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
    // **NO ROW LOOKS ACTIONABLE WITHOUT BEING ACTIONABLE**, which is
    // `AC-17.13` on this page: reference text is a `label`, and the only other
    // kind here is a `button` that CARRIES the door it advertises. A `button`
    // with no door is the exact offer-that-cannot-perform the palette refuses.
    for r in &page {
      match r.kind.as_str() {
        "label" => assert!(r.door.is_none(), "a label carries a door: {}", r.title),
        "button" => assert!(
          r.door.is_some(),
          "`{}` is drawn as a door and opens nothing",
          r.title
        ),
        other => panic!(
          "`{}` carries the kind `{other}`, which this page does not use",
          r.title
        ),
      }
    }
  }
}
