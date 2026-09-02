//! The command vocabulary: what `/` offers, and the palette that filters it.
//!
//! **hv RULED THE PALETTE OVER THE LOTUS BAR (2026-09-02), AFTER DRIVING THE
//! BUILD.** `tui-design.md` §5 specified a nested horizontal menu -- arrows
//! along a bar, accelerators coloured in place, `[<-]` and `[X]` as selectable
//! positions. hv tested the shipped machine and reached for `/quit`, which is
//! the other shape entirely: **`/` opens a FILTERED LIST of commands, typing
//! narrows it, the arrows pick, Enter runs.** The Lotus tree survives as the
//! GROUPING of this vocabulary and not as a widget.
//!
//! # Only what is wired is declared
//!
//! **A COMMAND THAT CANNOT RUN MUST NOT APPEAR**, and this is the module where
//! that rule costs something, so it is stated here rather than assumed. §5's
//! tree lists `Docs > Browse`, `Docs > New`, `File > Write`, `File > Reload`
//! and `Help`; none of them has a realiser. Declaring them would produce a
//! palette that ADVERTISES a menu of errors -- which is exactly the defect hv
//! drove into: a menu bar painted as a string, offering entries that did
//! nothing when chosen. **The vocabulary grows when the act behind it lands,
//! never before.**
//!
//! # `/` IS FOR THINGS TO DO; TYPING IS FOR PLACES TO GO
//!
//! **The palette holds ACTS, and navigation is deliberately not in it.** hv's
//! own frame draws the line: *`/commands` fire up the menus, and anything else
//! is omni-dispatched from the Omni.*
//!
//! §5's bar had a `Go` group listing `Threads  Issues  Packages  Criteria`,
//! and building it here was the obvious move. **It would have been redundant,
//! and the code says so**: `Live::index` already puts one entry per declared
//! kind into the omnibox, so `thread` ALREADY reaches the threads collection
//! by typing. A `Go` group would have been a second route to a destination
//! the composer reaches better -- and it would have buried the two commands
//! that have no other route under a list of ones that do.
//!
//! So the vocabulary is small on purpose, and it is the SMALL HONEST SET
//! rather than a large one with holes. It grows when an act lands.

use super::omnibox::{Match, rank};

/// The character that opens the palette and stays visible in the composer.
pub const SIGIL: char = '/';

/// What running a command actually does.
///
/// **EVERY VARIANT HAS A REALISER TODAY.** This enum is the honest boundary of
/// the vocabulary: adding a variant is how a new command becomes possible, and
/// there is deliberately no `Unimplemented` arm to park one in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Act {
  /// Leave. `tui-design.md` §3: quitting is an act, never an accident.
  Quit,
  /// Pop the view stack -- the same act as `Backspace` on an empty composer.
  Back,
  /// The operator's own settings: the view with no argument, one value with
  /// one. `AC-17.14`.
  ///
  /// **THE ARGUMENT IS READ FROM THE BUFFER AT RUN TIME, NOT CARRIED HERE.**
  /// [`vocabulary`] is a constant list and an act holding an argument would
  /// have to be rebuilt on every keystroke to stay in step with what is typed
  /// -- and a stale copy of it is a command that runs against an argument the
  /// operator has already edited.
  Settings,
}

/// One offer in the palette.
///
/// **THERE IS NO `group` FIELD, AND ITS REMOVAL IS A FINDING RATHER THAN A
/// SIMPLIFICATION.** §5 says the Lotus tree *survives as the GROUPING of this
/// vocabulary*, and it was built that way: a `group` label on every command,
/// declared, populated -- **and read by nothing, for its whole life.** That is
/// the `Hotkey` defect exactly, in the module whose own note condemns it, one
/// commit later. Rendering it would have meant putting the group at the front
/// of the haystack, where the ranker's boosted prefix has to be the NAME; so
/// the honest move is the one this module already states for offers, applied to
/// a field: **it comes back when something reads it.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Command {
  /// What the operator types. The boosted half of the haystack.
  pub name: String,
  /// What it does, in the operator's words.
  pub blurb: String,
  pub act: Act,
}

/// The whole vocabulary.
///
/// Order is the palette's resting order, so it is the order a reader meets the
/// tool in.
pub fn vocabulary() -> Vec<Command> {
  vec![
    Command {
      name: "quit".into(),
      blurb: "leave explore".into(),
      act: Act::Quit,
    },
    Command {
      name: "back".into(),
      blurb: "up one view".into(),
      act: Act::Back,
    },
    Command {
      name: "settings".into(),
      blurb: "explorer settings -- name one to read it".into(),
      act: Act::Settings,
    },
  ]
}

/// The searchable text of one command. **One function, shared with whatever
/// highlights [`Match::positions`]**, for the reason
/// [`super::omnibox::haystack`] is one function: two spellings of this
/// concatenation would make the positions point into text nobody drew.
pub fn haystack(c: &Command) -> String {
  format!("{} {}", c.name, c.blurb)
}

/// The query inside a palette buffer, or `None` when this is not one.
///
/// **THE SIGIL STAYS IN THE BUFFER AND THAT IS WHAT MAKES THE MODE VISIBLE.**
/// The operator can see `/qu` and knows why the list below is commands; a
/// palette whose sigil vanished on the keypress would leave a query indistinguishable
/// from an address. It also gives `Backspace` an exit that needs no special
/// case: erase back past the `/` and there is no palette left to be in.
pub fn query_of(buffer: &str) -> Option<&str> {
  parts_of(buffer).map(|(query, _)| query)
}

/// A palette buffer split into the command query and its argument, or `None`
/// when this is not a palette buffer at all.
///
/// **AN ARGUMENT IS EVERYTHING AFTER THE FIRST SPACE, AND THE COMMAND IS
/// MATCHED ON THE FIRST WORD ALONE.** Without the split, typing
/// `/settings editing.mode` would search the vocabulary for the whole phrase
/// and match nothing -- the palette would empty out mid-argument and Enter
/// would run nothing, with the operator watching a correct-looking prompt.
///
/// **NO COMMAND NAME CONTAINS A SPACE, and [`no_command_name_contains_a_space`]
/// holds that** -- the split is unambiguous only while that is true, and it is
/// the sort of thing a two-word command would quietly break.
pub fn parts_of(buffer: &str) -> Option<(&str, &str)> {
  let rest = buffer.strip_prefix(SIGIL)?;
  Some(match rest.split_once(' ') {
    Some((query, argument)) => (query, argument.trim()),
    None => (rest, ""),
  })
}

/// The argument typed after the command, empty when there is none.
pub fn argument_of(buffer: &str) -> &str {
  parts_of(buffer).map(|(_, arg)| arg).unwrap_or("")
}

/// Every command `query` hits, best first, at most `cap`.
///
/// **AN EMPTY QUERY RETURNS THE WHOLE VOCABULARY, WHICH IS THE OPPOSITE OF
/// [`super::omnibox::matches`] AND DELIBERATE.** The omnibox at rest shows
/// nothing because the body is already the listing of the model. The palette
/// at rest must show its vocabulary, because **discovery is the entire reason
/// it exists** -- a `/` that opened an empty box would teach the operator
/// nothing and hide every command behind knowing its name already.
pub fn matches(cmds: &[Command], query: &str, cap: usize) -> Vec<Match> {
  let needle = query.trim();
  if needle.is_empty() {
    return cmds
      .iter()
      .enumerate()
      .take(cap)
      .map(|(entry, _)| Match {
        entry,
        score: 0,
        positions: Vec::new(),
      })
      .collect();
  }
  let hays: Vec<(String, usize)> = cmds
    .iter()
    .map(|c| (haystack(c), c.name.chars().count()))
    .collect();
  rank(needle, &hays, cap)
}

#[cfg(test)]
mod tests {
  use super::*;

  /// **THE PALETTE AT REST SHOWS ITS VOCABULARY**, which is the property that
  /// makes it a discovery surface rather than a guessing game.
  #[test]
  fn an_empty_query_offers_the_whole_vocabulary_in_declared_order() {
    let v = vocabulary();
    let m = matches(&v, "", 32);
    assert_eq!(
      m.len(),
      v.len(),
      "the palette at rest hid part of its own vocabulary"
    );
    assert!(
      m.iter().enumerate().all(|(i, hit)| hit.entry == i),
      "the resting palette reordered itself; at rest there is no ranking to apply"
    );
  }

  /// hv's own test, as a test: `/quit` must reach quit. hv drove the shipped
  /// build, reached for exactly this, and had to fall back to `:q`.
  #[test]
  fn the_spelling_hv_reached_for_finds_the_command_hv_wanted() {
    let v = vocabulary();
    for typed in ["quit", "qui", "qu", "q"] {
      let m = matches(&v, typed, 8);
      assert!(!m.is_empty(), "`/{typed}` matched nothing at all");
      assert_eq!(
        v[m[0].entry].act,
        Act::Quit,
        "`/{typed}` did not rank quit first; hv drove exactly this spelling"
      );
    }
  }

  /// **A HIT IN THE NAME BEATS A HIT IN THE BLURB**, which is the shared
  /// ranker's boosted-prefix rule doing its job over a second vocabulary.
  #[test]
  fn a_name_hit_outranks_a_blurb_hit() {
    let v = vocabulary();
    let m = matches(&v, "back", 8);
    assert_eq!(
      v[m[0].entry].name, "back",
      "the command NAMED back lost to one that merely mentions it"
    );
  }

  /// **NOTHING IS OFFERED THAT CANNOT RUN.** Asserted structurally: every
  /// command's act is a variant the realiser matches exhaustively, so this
  /// pins the other half -- that the vocabulary is not empty and every entry
  /// carries a name and a reason a human can read.
  #[test]
  fn every_offer_is_nameable_and_explains_itself() {
    let v = vocabulary();
    assert!(!v.is_empty(), "a palette with no commands is a dead key");
    for c in &v {
      assert!(
        !c.name.trim().is_empty(),
        "a command with no name cannot be typed"
      );
      assert!(
        !c.blurb.trim().is_empty(),
        "`{}` offers no description, so the palette cannot teach it",
        c.name
      );
    }
  }

  /// A query that hits nothing returns nothing, rather than falling back to
  /// the whole list -- **an empty result is the honest answer**, and showing
  /// everything would read as "these all match".
  #[test]
  fn a_query_that_hits_nothing_offers_nothing() {
    let v = vocabulary();
    assert!(
      matches(&v, "zzzznotacommand", 8).is_empty(),
      "a miss fell back to the full vocabulary, which reads as a list of matches"
    );
  }

  /// The sigil stays in the buffer, and erasing it is the exit.
  #[test]
  fn the_query_is_what_follows_the_sigil_and_a_bare_buffer_is_not_a_palette() {
    assert_eq!(query_of("/qu"), Some("qu"));
    assert_eq!(query_of("/"), Some(""));
    assert_eq!(
      query_of("ST0056"),
      None,
      "an address must never be read as a command query"
    );
  }

  /// **THE COMMAND KEEPS MATCHING WHILE ITS ARGUMENT IS BEING TYPED.** Driven
  /// through `matches` rather than through the splitter alone: the splitter
  /// being right is no evidence that the ranker was given its output.
  #[test]
  fn an_argument_does_not_stop_the_command_from_matching() {
    assert_eq!(
      parts_of("/settings editing.mode"),
      Some(("settings", "editing.mode"))
    );
    assert_eq!(parts_of("/settings"), Some(("settings", "")));
    assert_eq!(parts_of("/settings   "), Some(("settings", "")));
    assert_eq!(argument_of("/settings editing.mode"), "editing.mode");
    assert_eq!(argument_of("ST0056"), "");

    let v = vocabulary();
    let with_arg = matches(
      &v,
      query_of("/settings editing.mode").expect("a palette buffer"),
      8,
    );
    assert!(
      !with_arg.is_empty(),
      "the palette emptied out while an argument was being typed"
    );
    assert_eq!(
      v[with_arg[0].entry].act,
      Act::Settings,
      "the argument outranked the command it belongs to"
    );
  }

  /// The split is unambiguous only while no command name carries a space.
  #[test]
  fn no_command_name_contains_a_space() {
    for c in vocabulary() {
      assert!(
        !c.name.contains(' '),
        "`{}` has a space in its name, so the argument split cuts it in half",
        c.name
      );
    }
  }
}
