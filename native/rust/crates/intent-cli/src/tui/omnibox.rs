//! The omnibox: the rest state's input, and the primary navigation device.
//!
//! **hv's ruling, 2026-08-30, verbatim frame:** *an omnibox style input that
//! autonavigates to entity by its address.* Typing filters an index of every
//! addressable entity; Enter goes to the selection; an exact spelling that
//! matches nothing fuzzily still lands through `nav::land`, so the fuzzy list
//! is a convenience over the addressing scheme and never a replacement for it.
//!
//! **PURE, LIKE EVERYTHING ELSE WITH A PROPERTY WORTH ASSERTING.** No
//! terminal, no store: the index is data handed in, the matcher is a function
//! of (index, buffer), and what Enter means is a function the app asks. The
//! run loop owns the one impure step -- resolving a spelling against the
//! store -- because presence is a fact only the facade knows.
//!
//! # The matcher is a SUBSEQUENCE scorer, written here rather than imported
//!
//! A dependency for forty lines would put a fuzzy-matching crate in
//! `Cargo.lock` for every node that builds this workspace. The scoring is
//! deliberately simple and deliberately stated: subsequence hits only,
//! contiguous runs beat scattered ones, hits in the ID beat hits in the
//! title, and earlier starts beat later ones. It does not do
//! Smith-Waterman and it does not need to: the corpus is a few hundred short
//! strings, and the test of "good enough" is that typing an id's own
//! characters always puts that id first -- which is asserted, not hoped.

use intentsvcs::nav::View;

/// One addressable entity, as the omnibox sees it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
  /// The entity kind -- `thread`, `issue`, `wp` -- as `View::Item` spells it.
  pub kind: String,
  /// The id an operator would type: `ST0056`, `0171`.
  pub id: String,
  pub title: String,
  /// The display form of the entity's status, already humanised by the model.
  pub status: String,
}

impl Entry {
  /// Where Enter on this entry goes.
  pub fn door(&self) -> View {
    View::Item {
      kind: self.kind.clone(),
      id: self.id.clone(),
    }
  }
}

/// One scored hit: the entry, and which character positions of its haystack
/// matched -- kept so a renderer can highlight the letters that earned the
/// placement, which is the television-style affordance that makes a fuzzy
/// list legible rather than magical.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Match {
  /// Index into the entry slice the matcher was given.
  pub entry: usize,
  pub score: i64,
  /// Positions into [`haystack`]'s output for this entry.
  pub positions: Vec<usize>,
}

/// The searchable text of one entry. **One function, used by the matcher and
/// by any renderer that highlights [`Match::positions`]** -- two spellings of
/// this concatenation would make the positions point into text nobody drew.
pub fn haystack(e: &Entry) -> String {
  format!("{} {}", e.id, e.title)
}

/// Score `needle` against `hay` as a case-insensitive subsequence.
///
/// `None` when `needle` is not a subsequence at all. The weights are stated in
/// the module doc; their exact values are tuning, and the properties the tests
/// pin are ordinal (an id's own prefix ranks its entity first), never the
/// numbers themselves.
fn score(needle: &str, hay: &str, id_len: usize) -> Option<(i64, Vec<usize>)> {
  let hay_lower: Vec<char> = hay.to_lowercase().chars().collect();
  let needle_lower: Vec<char> = needle.to_lowercase().chars().collect();
  if needle_lower.is_empty() {
    return None;
  }
  let mut positions = Vec::with_capacity(needle_lower.len());
  let mut at = 0usize;
  for &c in &needle_lower {
    let found = hay_lower[at..].iter().position(|&h| h == c)?;
    positions.push(at + found);
    at = at + found + 1;
  }
  let mut s: i64 = 0;
  for (i, &p) in positions.iter().enumerate() {
    // Hits inside the id column are worth more than hits in the title.
    s += if p < id_len { 100 } else { 10 };
    // A hit adjacent to the previous one is a run, and runs are what a human
    // meant when they typed consecutive characters.
    if i > 0 && positions[i - 1] + 1 == p {
      s += 40;
    }
  }
  // Earlier first-hits beat later ones, gently.
  s -= positions[0] as i64;
  Some((s, positions))
}

/// Every entry `buffer` hits, best first, at most `cap`.
///
/// An empty buffer returns nothing: the omnibox at rest shows the model, not
/// a preemptive listing of it -- the body is already the listing.
pub fn matches(index: &[Entry], buffer: &str, cap: usize) -> Vec<Match> {
  let needle = buffer.trim();
  if needle.is_empty() || needle.starts_with(':') {
    return Vec::new();
  }
  let mut out: Vec<Match> = index
    .iter()
    .enumerate()
    .filter_map(|(i, e)| {
      let hay = haystack(e);
      score(needle, &hay, e.id.chars().count()).map(|(score, positions)| Match {
        entry: i,
        score,
        positions,
      })
    })
    .collect();
  // Stable order under equal scores: the index's own order, which the source
  // built deliberately (threads before issues, each newest-ish first).
  out.sort_by(|a, b| b.score.cmp(&a.score).then(a.entry.cmp(&b.entry)));
  out.truncate(cap);
  out
}

/// The input's state: what has been typed, and which match is picked.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Omnibox {
  pub buffer: String,
  /// Which of the current matches Enter takes. **Always valid by clamping at
  /// read time** ([`Omnibox::picked`]), because the match list changes under
  /// it on every keystroke and a stored index into a vanished list is the
  /// stale-cursor class the app already refuses for rows.
  pick: usize,
}

/// What Enter means, decided from the state alone.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Go {
  /// Nothing typed: just leave the input.
  Nothing,
  /// `:q` / `:q!` -- the one command the omnibox owes an answer to today.
  Quit,
  /// A `:` command this build does not know.
  UnknownCommand(String),
  /// The picked match's door.
  Pick(View),
  /// No match to pick: hand the spelling to the address resolver.
  Spelling(String),
}

impl Omnibox {
  /// Append one typed character. `/` arrives here too when the buffer is
  /// non-empty -- the app's guard routes it -- because `st/ST0056` is a legal
  /// spelling.
  pub fn type_char(&mut self, c: char) {
    self.buffer.push(c);
    self.pick = 0;
  }

  /// Backspace. Deleting from an empty buffer is a no-op, not an error.
  pub fn erase(&mut self) {
    self.buffer.pop();
    self.pick = 0;
  }

  /// Seed the buffer from NAV -- `:` or any printable lands here with its
  /// character, which is the you-just-start-typing affordance.
  pub fn seed(&mut self, c: char) {
    self.buffer.clear();
    self.buffer.push(c);
    self.pick = 0;
  }

  pub fn is_empty(&self) -> bool {
    self.buffer.is_empty()
  }

  /// The picked match index, clamped into `n` current matches.
  pub fn picked(&self, n: usize) -> Option<usize> {
    if n == 0 {
      None
    } else {
      Some(self.pick.min(n - 1))
    }
  }

  /// Move the pick. `down` walks toward worse matches; the list is best-first.
  pub fn pick_move(&mut self, down: bool, n: usize) {
    let Some(cur) = self.picked(n) else { return };
    self.pick = if down {
      (cur + 1).min(n - 1)
    } else {
      cur.saturating_sub(1)
    };
  }

  /// What Enter means right now, given the matches for the current buffer.
  pub fn go(&self, index: &[Entry], m: &[Match]) -> Go {
    let typed = self.buffer.trim();
    if typed.is_empty() {
      return Go::Nothing;
    }
    if let Some(cmd) = typed.strip_prefix(':') {
      return match cmd.trim() {
        "q" | "q!" => Go::Quit,
        other => Go::UnknownCommand(other.to_string()),
      };
    }
    match self.picked(m.len()) {
      Some(p) => Go::Pick(index[m[p].entry].door()),
      None => Go::Spelling(typed.to_string()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn index() -> Vec<Entry> {
    let mk = |kind: &str, id: &str, title: &str, status: &str| Entry {
      kind: kind.into(),
      id: id.into(),
      title: title.into(),
      status: status.into(),
    };
    vec![
      mk(
        "thread",
        "ST0056",
        "Add a Rust-based CLI with a local SQLite DB",
        "wip",
      ),
      mk("thread", "ST0058", "v3 test architecture asks", "wip"),
      mk("thread", "ST0064", "SwiftUI menubar app", "not-started"),
      mk(
        "issue",
        "0171",
        "fc dispatches to four facade methods",
        "open",
      ),
      mk(
        "issue",
        "0056",
        "an issue that shares digits with a thread",
        "open",
      ),
      mk("wp", "ST0056/09", "MCP server and agent guide", "wip"),
    ]
  }

  /// **TYPING AN ID'S OWN CHARACTERS PUTS THAT ID FIRST** -- the one property
  /// that makes autonavigate-by-address real rather than roulette. Driven on
  /// the deliberately-planted collision: thread `ST0056` and issue `0056`
  /// share their digits, and the full spelling must separate them.
  #[test]
  fn an_ids_own_spelling_ranks_its_entity_first() {
    let idx = index();
    for (typed, want) in [
      ("ST0056", "ST0056"),
      ("st0058", "ST0058"),
      ("0171", "0171"),
      ("56/09", "ST0056/09"),
    ] {
      let m = matches(&idx, typed, 8);
      assert!(
        !m.is_empty(),
        "{typed:?} matched nothing in an index that contains it"
      );
      assert_eq!(
        idx[m[0].entry].id, want,
        "{typed:?} ranked {} first instead of {want}",
        idx[m[0].entry].id
      );
    }
  }

  /// `56` is genuinely ambiguous -- a thread and an issue both carry the
  /// digits -- and the CLI already reports that ambiguity. The omnibox's
  /// answer is the fuzzy list: both are IN it, so the operator sees the
  /// collision instead of being routed through it.
  #[test]
  fn an_ambiguous_spelling_offers_both_owners() {
    let idx = index();
    let m = matches(&idx, "56", 8);
    let ids: Vec<&str> = m.iter().map(|h| idx[h.entry].id.as_str()).collect();
    assert!(
      ids.contains(&"ST0056") && ids.contains(&"0056"),
      "both carriers of `56` must be offered: {ids:?}"
    );
  }

  /// Title words reach entities whose id the operator does not know -- the
  /// half of the omnibox that is a finder rather than an address bar.
  #[test]
  fn title_words_find_their_entity() {
    let idx = index();
    let m = matches(&idx, "menubar", 8);
    assert_eq!(idx[m[0].entry].id, "ST0064");
    let m = matches(&idx, "facade", 8);
    assert_eq!(idx[m[0].entry].id, "0171");
  }

  /// The positions point into [`haystack`]'s text, and every one is a real
  /// index -- the highlight contract, held here so a renderer can trust it
  /// blind.
  #[test]
  fn match_positions_index_into_the_shared_haystack() {
    let idx = index();
    for hit in matches(&idx, "rust cli", 8) {
      let hay: Vec<char> = haystack(&idx[hit.entry]).chars().collect();
      for &p in &hit.positions {
        assert!(
          p < hay.len(),
          "position {p} is outside the haystack it claims to index"
        );
      }
      let picked: String = hit.positions.iter().map(|&p| hay[p]).collect();
      assert_eq!(
        picked.to_lowercase().replace(' ', ""),
        "rustcli",
        "the highlighted characters must spell the needle"
      );
    }
  }

  /// An empty buffer offers nothing, and a `:` command is never fuzzy-matched
  /// -- `:q` quitting is a contract, and a `:` buffer that matched `criteria`
  /// by subsequence would put a destination one keystroke from a command.
  #[test]
  fn empty_and_command_buffers_match_nothing() {
    let idx = index();
    assert!(matches(&idx, "", 8).is_empty());
    assert!(matches(&idx, "   ", 8).is_empty());
    assert!(matches(&idx, ":q", 8).is_empty());
  }

  /// What Enter means, across the whole state space that decides it.
  #[test]
  fn enter_means_what_the_state_says() {
    let idx = index();
    let mut o = Omnibox::default();
    assert_eq!(
      o.go(&idx, &[]),
      Go::Nothing,
      "empty buffer leaves the input"
    );

    for c in ":q".chars() {
      o.type_char(c);
    }
    assert_eq!(o.go(&idx, &[]), Go::Quit);
    o.type_char('!');
    assert_eq!(o.go(&idx, &[]), Go::Quit, ":q! is the discard spelling");

    let mut o = Omnibox::default();
    for c in ":wq".chars() {
      o.type_char(c);
    }
    assert_eq!(o.go(&idx, &[]), Go::UnknownCommand("wq".into()));

    let mut o = Omnibox::default();
    for c in "menubar".chars() {
      o.type_char(c);
    }
    let m = matches(&idx, "menubar", 8);
    assert_eq!(
      o.go(&idx, &m),
      Go::Pick(View::Item {
        kind: "thread".into(),
        id: "ST0064".into()
      }),
      "a match under the pick is where Enter goes"
    );

    let mut o = Omnibox::default();
    for c in "zzz-no-such".chars() {
      o.type_char(c);
    }
    assert_eq!(
      o.go(&idx, &matches(&idx, "zzz-no-such", 8)),
      Go::Spelling("zzz-no-such".into()),
      "no match hands the spelling to the address resolver rather than dying here"
    );
  }

  /// The pick clamps to the live list rather than remembering a longer one.
  #[test]
  fn the_pick_survives_the_list_shrinking_under_it() {
    let mut o = Omnibox::default();
    for c in "st".chars() {
      o.type_char(c);
    }
    o.pick_move(true, 5);
    o.pick_move(true, 5);
    assert_eq!(o.picked(5), Some(2));
    assert_eq!(o.picked(1), Some(0), "a shrunk list clamps the pick");
    assert_eq!(o.picked(0), None, "an empty list has no pick at all");
    o.type_char('x');
    assert_eq!(o.picked(5), Some(0), "typing resets the pick to the top");
  }
}
