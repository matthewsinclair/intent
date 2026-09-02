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
  /// The whole reference, in the body. hv asked for it 2026-09-02.
  ///
  /// **IT IS A VIEW RATHER THAN A HANDOFF TO `$EDITOR -R`, and the reason is
  /// measured**: there is no portable read-only flag for an editor -- `vim`
  /// takes `-R`, `nano` takes `-v`, `emacs` needs an `--eval`, `code` has
  /// none -- and `$EDITOR` is whatever the operator set. See [`super::help`].
  Help,
  /// The operator's own settings: the view with no argument, one value with
  /// one. `AC-17.14`.
  ///
  /// **THE ARGUMENT IS READ FROM THE BUFFER AT RUN TIME, NOT CARRIED HERE.**
  /// [`vocabulary`] is a constant list and an act holding an argument would
  /// have to be rebuilt on every keystroke to stay in step with what is typed
  /// -- and a stale copy of it is a command that runs against an argument the
  /// operator has already edited.
  Settings,
  /// Run `intent <verb> ...` through the CLI's OWN dispatch, with the terminal
  /// lent. hv ruled this in 2026-09-02: *I should be able to run any `intent
  /// {cmd} ...` command via `/{cmd} ...` in the explorer.*
  ///
  /// **THE VERB ONLY. The arguments are read from the buffer at run time**, for
  /// the reason [`Act::Settings`] gives one line up.
  Cli(String),
}

/// The `intent` verbs the palette will run: **AN ALLOW-LIST, AND THE ONLY
/// DECISION ABOUT WHAT IS RUNNABLE.**
///
/// # Why a roster and not the dispatch table
///
/// The table is the whole surface, and reading it here would make every verb
/// added in future palette-runnable **by nobody's decision** -- which is
/// `AC-17.14`'s reason one surface over, and exactly how `intent_version` would
/// have become an editable setting. A deny-list has the same defect with an
/// extra step: it is a list of what somebody thought of.
///
/// **THE EXCLUSIONS ARE THE SUBSTANCE, SO THEY ARE WRITTEN DOWN RATHER THAN
/// IMPLIED BY ABSENCE**, and hv's ruling was *any* command, so each one is an
/// exception that has to earn itself:
///
/// - `explore` -- the TUI from inside the TUI.
/// - `mcp` -- a stdio server that never returns; the lend would never come
///   back and the operator's only exit is killing the process.
/// - `daemon` -- starts and stops the process that holds this store open,
///   underneath a running explorer.
/// - `fc` -- fiat close is the human's verb (`IN-AG-FIAT-001`), and a roster
///   is the class of thing that rule names: a script, a hook, a menu. hv can
///   still run it; it does not get to be one keystroke from every session.
/// - `init` / `bootstrap` / `upgrade` -- one-time or whole-tree setup. `init`
///   refuses inside a project by design, so offering it is a menu of errors.
/// - `graphql` / `browse` -- **nothing implements them yet.** vc measured the
///   GraphQL escape hatch as a schema document with no executor; `browse`
///   waits on cc's WP-08 stub. This module's own rule: the vocabulary grows
///   when the act behind it lands.
/// - `help` -- claimed by [`Act::Help`]. See [`vocabulary`].
pub const CLI_ROSTER: &[&str] = &[
  "st", "wp", "ac", "at", "issues", "todo", "info", "config", "doctor", "agents", "claude",
  "critic", "lang", "llm", "learn", "modules", "plugin", "ext", "version", "search", "sync",
  "schema", "export", "ingest", "backup", "organize", "edit", "events", "surface",
];

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

/// The whole vocabulary: the TUI's own acts, then the allow-listed `intent`
/// verbs.
///
/// Order is the palette's resting order, so it is the order a reader meets the
/// tool in -- **and it is also the collision rule made structural.** The acts
/// come first and a roster name that collides with one is DROPPED, so `/help`
/// is the help view and never `intent help`. The drop is a guard rather than a
/// policy: [`the_roster_never_collides_with_an_act`] holds that it has no work
/// to do, because a silently-swallowed roster entry is the same defect as a
/// silently-ignored one.
///
/// **THE BLURB COMES FROM `clap`, NEVER FROM A STRING HERE.** It is the same
/// `get_about()` that `intent <verb> --help` prints and that [`super::help`]
/// walks -- one home, so the palette cannot describe a command differently
/// from the CLI. hv asked for exactly this: *it would be good if we didn't
/// dupe it, but rather got it from the same place that `--help` gets it from.*
///
/// A roster name the surface does not carry is dropped too, and
/// [`every_roster_name_is_a_real_command`] refuses that in a test -- **at run
/// time an unknown verb must not become a palette entry that fails on Enter**,
/// and in the suite it must not be tolerated at all.
pub fn vocabulary(cli: &clap::Command) -> Vec<Command> {
  let mut out = acts();
  for name in CLI_ROSTER {
    if out.iter().any(|c| c.name == *name) {
      continue;
    }
    let Some(sub) = cli
      .get_subcommands()
      .find(|s| s.get_name() == *name && !s.is_hide_set())
    else {
      continue;
    };
    out.push(Command {
      name: (*name).to_string(),
      blurb: sub
        .get_about()
        .map(|a| a.to_string())
        .unwrap_or_else(|| format!("run `intent {name}`")),
      act: Act::Cli((*name).to_string()),
    });
  }
  out
}

/// The TUI's own acts -- the closed vocabulary that wins every collision.
fn acts() -> Vec<Command> {
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
      name: "help".into(),
      blurb: "every key, command and setting".into(),
      act: Act::Help,
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

  /// Is every act on the resting palette, at this cap?
  ///
  /// **A PREDICATE RATHER THAN AN INLINE ASSERTION, SO IT CAN BE RUN AGAINST A
  /// VOCABULARY WHERE THE ANSWER IS NO.** An assertion can only be observed
  /// passing; a predicate can be driven to both answers in one test, which is
  /// the only way to show it discriminates.
  fn every_act_is_offered(v: &[Command], cap: usize) -> bool {
    let offered = matches(v, "", cap);
    acts()
      .iter()
      .all(|a| offered.iter().any(|hit| v[hit.entry].name == a.name))
  }

  /// **THE PALETTE AT REST SHOWS THE TOP OF ITS VOCABULARY, IN DECLARED ORDER,
  /// AND EVERY ACT IS IN IT.**
  ///
  /// This test read *the whole vocabulary* until `/{cmd} ...` landed, and the
  /// change is a real consequence rather than a test bent to fit. The
  /// vocabulary was four acts, so *all of it* and *the first
  /// [`super::app::MATCH_CAP`]* were the same set and the test could not tell
  /// which one it was asserting. **With the CLI roster on it the vocabulary is
  /// dozens, and a cap that showed all of them would be the dropdown eating
  /// the body** -- the defect the cap exists to prevent.
  ///
  /// So the discovery guarantee moves rather than weakens. What an operator is
  /// owed at rest is the CLOSED vocabulary -- the acts, which are the commands
  /// that exist nowhere else -- and the full CLI list is `/help`, which lists
  /// every command with clap's own text. **The acts come first in
  /// [`vocabulary`] and that is what makes this true by construction**, which
  /// is why the assertion is worth keeping: reorder the vocabulary so an act
  /// falls past the cap and it fails here rather than in front of hv.
  #[test]
  fn an_empty_query_offers_the_top_of_the_vocabulary_in_declared_order() {
    let v = vocabulary(&crate::spine::surface());
    let cap = super::super::app::MATCH_CAP;
    let m = matches(&v, "", cap);
    assert_eq!(
      m.len(),
      v.len().min(cap),
      "the palette at rest offered neither its whole vocabulary nor a full page of it"
    );
    assert!(
      m.iter().enumerate().all(|(i, hit)| hit.entry == i),
      "the resting palette reordered itself; at rest there is no ranking to apply"
    );
    assert!(
      every_act_is_offered(&v, cap),
      "an act is not on the resting palette -- it has been pushed past the cap"
    );
    // **AND THE CHECK ABOVE IS SHOWN TO HAVE TEETH, IN THE SAME TEST.**
    // Against the shipped vocabulary it passes because acts sort first, which
    // is the very thing that makes it pass rather than something it proves --
    // vc's finding, and the same undecided-assertion shape this test was
    // rewritten to remove. So the predicate is also run against a vocabulary
    // that pushes the acts past the cap, where it MUST fail. Without this arm,
    // `every_act_is_offered` returning `true` unconditionally would satisfy
    // everything above.
    let mut buried = vec![
      Command {
        name: "filler".into(),
        blurb: String::new(),
        act: Act::Cli("filler".into()),
      };
      cap
    ];
    buried.extend(acts());
    assert!(
      !every_act_is_offered(&buried, cap),
      "the visibility check passes even when every act is past the cap, so it \
       is not checking visibility"
    );
  }

  /// **THE CONSTRUCTION THE PROPERTY ABOVE RESTS ON: EVERY ACT SORTS AHEAD OF
  /// EVERY CLI VERB.**
  ///
  /// *Every act is visible at rest* is a CONSEQUENCE of this and of the cap,
  /// and asserting a consequence that holds by construction is an assertion
  /// that cannot go red. This one can: [`vocabulary`] happens to start from
  /// [`acts`] and append, which is a single line anybody could write the other
  /// way round without seeing what it was holding up. **It is cheap to pin now
  /// and archaeology later.**
  #[test]
  fn every_act_sorts_ahead_of_every_cli_verb() {
    let v = vocabulary(&crate::spine::surface());
    let last_act = v
      .iter()
      .rposition(|c| !matches!(c.act, Act::Cli(_)))
      .expect("the vocabulary carries no acts at all");
    let first_cli = v
      .iter()
      .position(|c| matches!(c.act, Act::Cli(_)))
      .expect("the vocabulary carries no CLI verbs, so this asserts nothing");
    assert!(
      last_act < first_cli,
      "`{}` is an act sitting behind the CLI verb `{}` -- the resting palette's \
       act visibility rests on acts coming first, and it no longer does",
      v[last_act].name,
      v[first_cli].name
    );
  }

  /// hv's own test, as a test: `/quit` must reach quit. hv drove the shipped
  /// build, reached for exactly this, and had to fall back to `:q`.
  #[test]
  fn the_spelling_hv_reached_for_finds_the_command_hv_wanted() {
    let v = vocabulary(&crate::spine::surface());
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
    let v = vocabulary(&crate::spine::surface());
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
    let v = vocabulary(&crate::spine::surface());
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
    let v = vocabulary(&crate::spine::surface());
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

    let v = vocabulary(&crate::spine::surface());
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
    for c in vocabulary(&crate::spine::surface()) {
      assert!(
        !c.name.contains(' '),
        "`{}` has a space in its name, so the argument split cuts it in half",
        c.name
      );
    }
  }

  /// **EVERY ROSTER NAME IS A REAL COMMAND ON THE SHIPPED SURFACE.**
  ///
  /// [`vocabulary`] drops an unknown one rather than offering a palette entry
  /// that fails on Enter -- **and a drop is exactly the silence this suite
  /// exists to break.** A verb renamed in the dispatch table would otherwise
  /// leave the palette quietly one command shorter, which is the `Hotkey`
  /// class read from the other end: not a thing declared and never read, but a
  /// thing named and never resolved.
  #[test]
  fn every_roster_name_is_a_real_command() {
    let cli = crate::spine::surface();
    let offered: Vec<String> = cli
      .get_subcommands()
      .filter(|s| !s.is_hide_set())
      .map(|s| s.get_name().to_string())
      .collect();
    assert!(!offered.is_empty(), "an empty surface asserts nothing");
    for name in CLI_ROSTER {
      assert!(
        offered.iter().any(|s| s == name),
        "`{name}` is on the palette roster and `intent --help` does not offer it"
      );
    }
  }

  /// **THE ROSTER IS AN ALLOW-LIST, AND THIS IS THE ASSERTION THAT SAYS SO.**
  ///
  /// The first half is the property; **the second half is the positive control
  /// on the instrument**, and it is the half worth having. A roster that had
  /// been "simplified" into *every subcommand* would satisfy every other test
  /// in this file -- each name resolves, no name collides, every blurb matches
  /// -- and would have quietly become the deny-list-by-omission that `AC-17.14`
  /// refuses. Only a strict-subset assertion can tell the two apart.
  ///
  /// The named exclusions are spelled out rather than left to the count,
  /// because a count is satisfied by excluding anything at all: `explore`
  /// recurses, `mcp` never returns, `daemon` moves the store underneath a
  /// running explorer, and `fc` is the human's verb by rule.
  #[test]
  fn the_roster_is_a_strict_subset_of_the_surface_and_omits_the_hazardous_verbs() {
    let cli = crate::spine::surface();
    let offered = cli.get_subcommands().filter(|s| !s.is_hide_set()).count();
    assert!(
      CLI_ROSTER.len() < offered,
      "the roster carries {} of {offered} commands -- a roster that is the whole \
       surface is not an allow-list, it is the absence of one",
      CLI_ROSTER.len()
    );
    for hazard in [
      "explore",
      "mcp",
      "daemon",
      "fc",
      "init",
      "bootstrap",
      "upgrade",
    ] {
      assert!(
        !CLI_ROSTER.contains(&hazard),
        "`{hazard}` reached the palette roster -- see the exclusions on CLI_ROSTER \
         for why it is not a verb an operator should be one keystroke from"
      );
    }
  }

  /// **THE TUI's VOCABULARY WINS A COLLISION, AND THE GUARD THAT MAKES IT WIN
  /// HAS NO WORK TO DO.**
  ///
  /// Two assertions with different jobs. The first is the operator-facing
  /// property: one `/help`, and it is the view. The second holds that the
  /// roster does not contain a colliding name in the first place -- because
  /// [`vocabulary`]'s drop would otherwise be swallowing a roster entry in
  /// silence, and a roster whose entries can vanish without a word is a roster
  /// nobody can read to find out what runs.
  #[test]
  fn the_roster_never_collides_with_an_act() {
    let v = vocabulary(&crate::spine::surface());
    let act_names: Vec<String> = acts().into_iter().map(|c| c.name).collect();
    for name in &act_names {
      let carrying: Vec<&Command> = v.iter().filter(|c| &c.name == name).collect();
      assert_eq!(
        carrying.len(),
        1,
        "`/{name}` is offered {} times -- the palette is ambiguous about what it runs",
        carrying.len()
      );
      assert!(
        !matches!(carrying[0].act, Act::Cli(_)),
        "`/{name}` runs the CLI verb -- the TUI's own act lost its spelling"
      );
      assert!(
        !CLI_ROSTER.contains(&name.as_str()),
        "`{name}` is on the roster AND is an act, so `vocabulary` is dropping it \
         silently -- remove it from the roster rather than relying on the drop"
      );
    }
  }

  /// **hv's OWN REQUIREMENT AS A TEST: THE PALETTE DOES NOT DUPE `--help`.**
  ///
  /// *It would be good if we didn't dupe it, but rather got it from the same
  /// place that `--help` gets it from.* So every CLI offer's blurb IS the
  /// `about` clap prints, character for character -- not merely similar, and
  /// not a hand-written paraphrase that starts true.
  #[test]
  fn every_cli_offer_carries_claps_own_about_text() {
    let cli = crate::spine::surface();
    let mut checked = 0usize;
    for c in vocabulary(&cli) {
      let Act::Cli(verb) = &c.act else { continue };
      let sub = cli
        .get_subcommands()
        .find(|s| s.get_name() == verb)
        .unwrap_or_else(|| panic!("`{verb}` is offered and is not on the surface"));
      if let Some(about) = sub.get_about() {
        assert_eq!(
          c.blurb,
          about.to_string(),
          "`/{verb}` is described differently in the palette than on `--help`"
        );
      }
      checked += 1;
    }
    assert!(
      checked > 0,
      "no CLI offer was examined, so this test asserted nothing"
    );
  }
}
