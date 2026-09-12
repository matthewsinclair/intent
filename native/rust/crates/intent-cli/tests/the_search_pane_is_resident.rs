//! **AT-21.1, AT-21.2 and AT-21.3 / AC-21.1 to AC-21.3: `/search` opens a
//! RESIDENT pane whose rows are the envelope's hits, Enter routes each hit by
//! what it is, and none of it needs a terminal.**
//!
//! The pane replaced a lend. `/search` used to hand the screen to `intent
//! search`, so the hits were printed onto the real terminal and were gone the
//! moment the explorer repainted: readable once, navigable never. What follows
//! asserts the three things that makes true.
//!
//! **EVERY TEST HERE RUNS WITHOUT A PTY, AND THAT IS AC-21.3's SECOND HALF
//! RATHER THAN A CONVENIENCE.** The pane is a view over a value: `App` is the
//! state machine, `views::search_rows` is a pure map from a `SearchAnswer`, and
//! neither can reach a store or a screen. A pane that had to be driven through a
//! terminal to be checked would be one nobody checked.

use intent_cli::tui::app::{App, Step};
use intent_cli::tui::commands;
use intent_cli::tui::nav::View;
use intent_cli::tui::views;
use intentsvcs::index::corpus::Corpus;
use intentsvcs::search::{
  CorpusState, Hit, HitKind, IndexFreshness, SearchAnswer, Span, Tier, TierGroup, corpus_key,
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn key(code: KeyCode) -> KeyEvent {
  KeyEvent::new(code, KeyModifiers::NONE)
}

/// An app whose palette is the REAL vocabulary. A bare `App` carries none --
/// the loop fills it -- so a test driving `/search` against an empty palette
/// would prove that Enter does nothing, which is true and not the question.
fn explorer() -> App {
  let mut app = App::explore();
  app.commands = commands::vocabulary(&intent_cli::spine::build(&intent_cli::dispatch::table()));
  app
}

fn typed(app: &mut App, text: &str) {
  for c in text.chars() {
    app.on_key(key(KeyCode::Char(c)), &[]);
  }
}

/// An envelope with one canon hit that has an owner and one file hit that does
/// not, so the two Enter routes below are both reachable.
fn answer() -> SearchAnswer {
  SearchAnswer {
    query: "quokka".to_string(),
    index: IndexFreshness::new(std::collections::BTreeMap::from([(
      corpus_key(&Corpus::Canon).to_string(),
      CorpusState {
        policy: "hash".to_string(),
        files: 2,
      },
    )])),
    groups: vec![TierGroup {
      tier: Tier::Lexical,
      hits: vec![
        Hit {
          kind: HitKind::Thread,
          name: "Objective".to_string(),
          owner: Some("ST0069".to_string()),
          lang: None,
          path: "intent/.canon/st/ST0069.json".to_string(),
          span: None,
          score: -1.0,
          snippet: "a quokka in the objective".to_string(),
          stale: false,
        },
        Hit {
          kind: HitKind::File,
          name: "notes.md".to_string(),
          owner: None,
          lang: None,
          path: "docs/notes.md".to_string(),
          span: Some(Span::line(12)),
          score: -0.5,
          snippet: "a quokka on line twelve".to_string(),
          stale: false,
        },
      ],
    }],
    matched: 2,
    returned: 2,
  }
}

/// AT-21.1: **the pane is a VIEW, entered by the act and remembered by the
/// stack**, and its rows are the envelope's hits.
#[test]
fn a_search_opens_a_resident_pane_carrying_the_query() {
  let mut app = explorer();
  app.on_key(key(KeyCode::Char('/')), &[]);
  typed(&mut app, "search quokka");
  let step = app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &[]);

  assert!(
    matches!(step, Step::Continue),
    "a search is a push, not a lend: {step:?}"
  );
  assert_eq!(
    app.stack.current(),
    &View::Search {
      query: "quokka".to_string()
    },
    "the view carries the query, so the stack remembers the search"
  );

  let rows = views::search_rows(&answer());
  assert_eq!(rows.len(), 2, "one row per hit: {rows:?}");
  assert_eq!(
    rows[0].title, "intent/.canon/st/ST0069.json",
    "a hit with no line is the file alone"
  );
  assert_eq!(
    rows[1].title, "docs/notes.md:12",
    "a hit with a line shows it, so the operator does not go hunting"
  );
  assert_eq!(
    rows[1].value, "file  a quokka on line twelve",
    "the hit's kind travels in what the operator reads"
  );
  assert_eq!(
    rows[1].kind, "button",
    "the row's KIND is the mode machine's widget: a hit that claimed no arm would open an editor"
  );
}

/// AT-21.1's other half: **the freshness line is a NOTE beside the rows, and an
/// empty corpus is said differently from an incomplete one.**
#[test]
fn the_freshness_line_says_which_kind_of_incomplete_this_is() {
  assert_eq!(
    views::freshness_note(&answer()),
    None,
    "a complete index has nothing to warn about"
  );

  let mut stale = answer();
  stale.index.mark_stale("docs/notes.md");
  let note = views::freshness_note(&stale).expect("a stale index owes the reader a line");
  assert!(
    note.contains("changed since"),
    "the note must say what happened: {note:?}"
  );

  let mut empty = answer();
  empty.index = IndexFreshness::new(std::collections::BTreeMap::from([(
    corpus_key(&Corpus::Canon).to_string(),
    CorpusState {
      policy: "hash".to_string(),
      files: 0,
    },
  )]));
  let note = views::freshness_note(&empty).expect("an empty index is never a miss");
  assert!(
    note.contains("nothing is indexed"),
    "an empty corpus is a different fact from a stale one: {note:?}"
  );
}

/// AT-21.2: **Enter routes by what the hit IS.** An entity hit descends through
/// the navigation every other view uses; a file hit is opened, because a file is
/// not a place in the model.
#[test]
fn enter_descends_on_an_entity_hit_and_opens_a_file_hit() {
  let rows = views::search_rows(&answer());
  let mut app = App::explore();
  app.push(View::Search {
    query: "quokka".to_string(),
  });
  // **THE CURSOR IS PUT ON THE ROWS THE WAY THE LOOP PUTS IT THERE.** A bare
  // `App` points at nothing until rows arrive, and Enter with no focused row is
  // a no-op -- which would make every assertion below pass for the wrong
  // reason.
  app.refocus(rows.len());

  let step = app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &rows);
  assert!(matches!(step, Step::Continue));
  assert_eq!(
    app.stack.current(),
    &View::Item {
      kind: "thread".to_string(),
      id: "ST0069".to_string()
    },
    "an entity hit lands on its view rather than opening its canon JSON"
  );

  let mut app = App::explore();
  app.push(View::Search {
    query: "quokka".to_string(),
  });
  app.refocus(rows.len());
  app.on_key(key(KeyCode::Down), &rows);
  let step = app.on_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE), &rows);
  match step {
    Step::OpenFile(path) => assert_eq!(
      path, "docs/notes.md",
      "the path opened is the BARE path -- `docs/notes.md:12` is not a file"
    ),
    other => panic!("a file hit must open the file: {other:?}"),
  }
}

/// AT-21.3: **the rows are a pure function of the envelope**, which is what
/// makes the pane and the CLI one answer: there is no second query here to
/// disagree with the first, and no store or terminal to reach for.
#[test]
fn the_rows_are_a_pure_function_of_the_envelope() {
  let answer = answer();
  assert_eq!(
    views::search_rows(&answer),
    views::search_rows(&answer),
    "the same envelope must render the same rows"
  );

  // A tier a later package adds is a GROUP, and the pane renders it with a
  // boundary rather than blending it into the one above -- the envelope's rule,
  // honoured by the only row builder the pane has.
  let mut two = answer.clone();
  two.groups.push(TierGroup {
    tier: Tier::Structural,
    hits: vec![Hit {
      kind: HitKind::Def,
      name: "quokka".to_string(),
      owner: None,
      lang: Some("rust".to_string()),
      path: "src/lib.rs".to_string(),
      span: Some(Span::line(3)),
      score: 0.0,
      snippet: "fn quokka()".to_string(),
      stale: false,
    }],
  });
  let rows = views::search_rows(&two);
  assert!(
    rows.iter().any(|r| r.is_rule()),
    "two groups are separated by a boundary: {rows:?}"
  );
  assert_eq!(
    rows.iter().filter(|r| !r.is_rule()).count(),
    3,
    "every hit of every group is a row: {rows:?}"
  );
}
