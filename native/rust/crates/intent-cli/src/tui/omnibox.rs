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

/// One addressable destination, as the omnibox sees it.
///
/// **THE DOOR IS DECLARED, NOT DERIVED** -- the same rule rows follow
/// (`tui-design.md` §6). It stopped being a computed `Item` when the index
/// grew COLLECTION entries: typing `iss` jumps to the issues LIST, which
/// keeps every level of the model reachable through the one device after
/// the entities lobby stopped being the root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
  /// The id an operator would type: `ST0056`, `0171`, `thread`.
  pub id: String,
  pub title: String,
  /// The display form of the entity's status; empty for a collection.
  pub status: String,
  /// Where Enter on this entry goes.
  pub door: View,
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

/// Rank `needle` over already-built haystacks, best first, at most `cap`.
///
/// **THE ONE SCORING PATH, AND IT IS SHARED ON PURPOSE.** Two vocabularies
/// come through here -- the ENTITY index below, and the command palette in
/// [`super::commands`] -- and they must rank the same way or `/qu` and `56`
/// would feel like two different programs wearing one input. A second matcher
/// for commands was the obvious shape and it is the Highlander defect: the
/// scoring weights, the run bonus and the tie-break are one behaviour, not two.
///
/// Each haystack carries the width of its BOOSTED PREFIX -- the id column for
/// an entity, the command's own name for a command -- because "a hit in the
/// name beats a hit in the description" is the same rule in both vocabularies
/// and only the boundary moves.
pub fn rank(needle: &str, hays: &[(String, usize)], cap: usize) -> Vec<Match> {
  let mut out: Vec<Match> = hays
    .iter()
    .enumerate()
    .filter_map(|(i, (hay, boost))| {
      score(needle, hay, *boost).map(|(score, positions)| Match {
        entry: i,
        score,
        positions,
      })
    })
    .collect();
  // Stable order under equal scores: the caller's own order, which every
  // caller builds deliberately.
  out.sort_by(|a, b| b.score.cmp(&a.score).then(a.entry.cmp(&b.entry)));
  out.truncate(cap);
  out
}

/// Every entry `buffer` hits, best first, at most `cap`.
///
/// An empty buffer returns nothing: the omnibox at rest shows the model, not
/// a preemptive listing of it -- the body is already the listing. **The
/// command palette deliberately differs and says so at its own door**: a
/// palette at rest must show its vocabulary, because discovery is the whole
/// reason it exists.
pub fn matches(index: &[Entry], buffer: &str, cap: usize) -> Vec<Match> {
  let needle = buffer.trim();
  if needle.is_empty() || needle.starts_with(':') {
    return Vec::new();
  }
  let hays: Vec<(String, usize)> = index
    .iter()
    .map(|e| (haystack(e), e.id.chars().count()))
    .collect();
  rank(needle, &hays, cap)
}

/// The input's state: what has been typed, where the caret is, and which match
/// is picked.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Omnibox {
  pub buffer: String,
  /// Where the next character lands, as a CHAR index in `0..=len`.
  ///
  /// **THE BUFFER USED TO HAVE NO CARET AND THE DESIGN SAID SO.**
  /// `tui-design.md` §4 left Left and Right unbound *against a cursor the
  /// buffer does not yet have*, deliberately, so that binding them later could
  /// not contradict a meaning operators had already learned. hv asked for
  /// ordinary terminal editing on 2026-09-02; this is that reservation being
  /// spent.
  ///
  /// **CHARS, NEVER BYTES.** The same rule the clipping obeys, for the same
  /// reason: this estate has paid for the other choice once, and an id typed
  /// beside a criterion's prose is exactly where a multi-byte character turns
  /// an index into a panic.
  cursor: usize,
  /// The leftmost column the caret may reach: the palette's sigil sits below
  /// it.
  ///
  /// **THE SIGIL IS A PROMPT, NOT CONTENT.** `/` is what tells the operator
  /// which vocabulary is being searched, and a bulk act should no more eat it
  /// than `C-u` should eat a shell's `$`. Without a floor, `C-w` on `/quit`
  /// walks to zero -- there is no whitespace to stop at -- takes the sigil
  /// with it and SILENTLY CLOSES THE PALETTE, and `C-a` then puts the caret
  /// where the next character would land in front of the sigil and close it
  /// too. Found by driving `C-w` in a test rather than by reading.
  ///
  /// **BACKSPACE IS DELIBERATELY NOT FLOORED**: erasing back past the sigil is
  /// the palette's declared exit, and it is a single, visible keystroke. The
  /// rule is that BULK acts do not cross a boundary the operator cannot see;
  /// one character at a time is not a bulk act.
  floor: usize,
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

/// Is the BEST offer drawn nearest the input, at the bottom of the dropdown?
///
/// **THE ONE HOME FOR THE OFFER LIST'S DIRECTION.** The renderer reverses on
/// it ([`super::run`]'s `dropdown`) and the motion converts through it
/// ([`Omnibox::pick_screen`]), so the arrows and the picture cannot drift
/// apart -- which they had, silently, for as long as both existed.
///
/// True because the input sits at the BOTTOM: the adjacent line is where the
/// eye rests, which is the television idiom for a bottom prompt.
pub const BEST_IS_NEAREST_THE_INPUT: bool = true;

impl Omnibox {
  /// Where the caret is, in chars.
  pub fn cursor(&self) -> usize {
    self.cursor
  }

  /// How many characters are in the buffer.
  fn len(&self) -> usize {
    self.buffer.chars().count()
  }

  /// The byte offset of char index `at`, or the buffer's end.
  fn byte_at(&self, at: usize) -> usize {
    self
      .buffer
      .char_indices()
      .nth(at)
      .map(|(b, _)| b)
      .unwrap_or(self.buffer.len())
  }

  /// Insert one typed character at the caret. `/` arrives here too when the
  /// buffer is non-empty -- the app's guard routes it -- because `st/ST0056`
  /// is a legal spelling.
  pub fn type_char(&mut self, c: char) {
    let at = self.byte_at(self.cursor);
    self.buffer.insert(at, c);
    self.cursor += 1;
    self.pick = 0;
  }

  /// Backspace: delete the character BEFORE the caret. At the start of the
  /// buffer it is a no-op, not an error.
  pub fn erase(&mut self) {
    if self.cursor == 0 {
      return;
    }
    let at = self.byte_at(self.cursor - 1);
    self.buffer.remove(at);
    self.cursor -= 1;
    self.pick = 0;
  }

  /// `C-d`: delete the character UNDER the caret. At the end, a no-op.
  pub fn delete_forward(&mut self) {
    if self.cursor >= self.len() {
      return;
    }
    let at = self.byte_at(self.cursor);
    self.buffer.remove(at);
    self.pick = 0;
  }

  /// Where the caret may not go left of. See [`Omnibox::floor`].
  pub fn set_floor(&mut self, floor: usize) {
    self.floor = floor.min(self.len());
    self.cursor = self.cursor.max(self.floor);
  }

  /// `C-a` / Home -- to the floor, which is the start unless a sigil holds it.
  pub fn home(&mut self) {
    self.cursor = self.floor;
  }

  /// `C-e` / End.
  pub fn end(&mut self) {
    self.cursor = self.len();
  }

  /// `C-b` / Left.
  pub fn left(&mut self) {
    self.cursor = self.cursor.saturating_sub(1).max(self.floor);
  }

  /// `C-f` / Right.
  pub fn right(&mut self) {
    self.cursor = (self.cursor + 1).min(self.len());
  }

  /// vi `b`: back to the start of the word before the caret.
  ///
  /// **THE MOTION AND [`Omnibox::kill_word_back`] SHARE THEIR BOUNDARY RULE AND
  /// NOTHING ELSE**, which is why the walk is one function both call: two
  /// copies of *skip whitespace, then skip word* is how `b` and `C-w` come to
  /// disagree about where a word starts, in a buffer where the operator can see
  /// both happen to the same text.
  pub fn word_back(&mut self) {
    self.cursor = self.word_start_before(self.cursor);
  }

  /// vi `w`: forward to the start of the next word.
  pub fn word_forward(&mut self) {
    let chars: Vec<char> = self.buffer.chars().collect();
    let end = chars.len();
    let mut at = self.cursor;
    while at < end && !chars[at].is_whitespace() {
      at += 1;
    }
    while at < end && chars[at].is_whitespace() {
      at += 1;
    }
    self.cursor = at;
  }

  /// The start of the word before `from`, never left of the floor.
  fn word_start_before(&self, from: usize) -> usize {
    let chars: Vec<char> = self.buffer.chars().collect();
    let mut at = from;
    while at > self.floor && chars[at - 1].is_whitespace() {
      at -= 1;
    }
    while at > self.floor && !chars[at - 1].is_whitespace() {
      at -= 1;
    }
    at
  }

  /// `C-k`: kill from the caret to the end.
  pub fn kill_to_end(&mut self) {
    let at = self.byte_at(self.cursor);
    self.buffer.truncate(at);
    self.pick = 0;
  }

  /// `C-u`: kill from the start to the caret.
  ///
  /// **THE READLINE MEANING, WHICH IS NOT "CLEAR THE LINE".** `C-u` in emacs
  /// mode kills backwards to the start and leaves the tail; a version that
  /// emptied the buffer would be the shell's `kill-whole-line` wearing the
  /// same key, and an operator who knows the binding would lose text they
  /// expected to keep.
  pub fn kill_to_start(&mut self) {
    let from = self.byte_at(self.floor);
    let to = self.byte_at(self.cursor);
    self.buffer.replace_range(from..to, "");
    self.cursor = self.floor;
    self.pick = 0;
  }

  /// `C-w`: kill the word before the caret, whitespace-delimited.
  ///
  /// **`C-w` WAS RETIRED FROM THE IN-PLACE FIELD KEYMAP AND IS BACK HERE ON
  /// PURPOSE.** hv retired it on 2026-08-30 with the vi/emacs field keymaps --
  /// *we're handing the text off to a dedicated editor, not trying to recreate
  /// it inside* -- and that reasoning is about FIELD, which has `$EDITOR` one
  /// keystroke away. **The composer has no such escape**: it is the primary
  /// input and cannot be handed to anything, so the argument that retired it
  /// there does not reach here.
  pub fn kill_word_back(&mut self) {
    let at = self.word_start_before(self.cursor);
    let from = self.byte_at(at);
    let to = self.byte_at(self.cursor);
    self.buffer.replace_range(from..to, "");
    self.cursor = at;
    self.pick = 0;
  }

  /// Abandon the query.
  ///
  /// **THE PICK RESETS WITH THE BUFFER, and this exists so that it does.**
  /// Every other mutator here resets `pick` for the same reason -- an index
  /// into a match list that no longer exists points at nothing -- but the two
  /// callers that abandon a query were both clearing `buffer` directly and
  /// leaving `pick` behind. [`Omnibox::picked`] clamps, so nothing was visibly
  /// wrong, which is precisely why it would have stayed wrong.
  ///
  /// **IT REPLACES `seed`, WHICH RETIRED WITH `NAV`.** Seeding carried a
  /// keystroke from a mode that no longer exists into an input that is now
  /// always focused; there is nothing to carry a character FROM.
  pub fn clear(&mut self) {
    self.buffer.clear();
    self.cursor = 0;
    self.floor = 0;
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

  /// Move the pick by a SCREEN direction, which is the only direction an
  /// operator has.
  ///
  /// **THIS EXISTS BECAUSE THE PICK AND THE PICTURE COUNT IN OPPOSITE
  /// DIRECTIONS, AND NOTHING USED TO RECONCILE THEM.** `pick_move` walks an
  /// index over a best-FIRST list; the dropdown is drawn best-LAST
  /// ([`BEST_IS_NEAREST_THE_INPUT`]). Both were correct about their own half
  /// and no code owned the relationship, so every `Down` walked the caret up
  /// the screen -- in the palette AND in the omnibox, because one renderer
  /// serves both. hv reported it in the palette on 2026-09-03.
  ///
  /// **THE CONVERSION IS DERIVED FROM THE RENDER ORDER RATHER THAN WRITTEN
  /// OUT**, so flipping [`BEST_IS_NEAREST_THE_INPUT`] moves the arrows with
  /// the picture and cannot leave them disagreeing again. A hand-inverted
  /// boolean at each call site would have fixed the symptom and rebuilt the
  /// defect: two places obliged to agree, with nothing holding them to it.
  pub fn pick_screen(&mut self, screen_down: bool, n: usize) {
    self.pick_move(screen_down != BEST_IS_NEAREST_THE_INPUT, n);
  }

  /// Move the pick. `down` walks toward worse matches; the list is best-first.
  ///
  /// **THIS IS INDEX SPACE, NOT SCREEN SPACE.** Callers holding a keystroke
  /// want [`pick_screen`](Self::pick_screen).
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
      Some(p) => Go::Pick(index[m[p].entry].door.clone()),
      None => Go::Spelling(typed.to_string()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn index() -> Vec<Entry> {
    let mk = |kind: &str, id: &str, title: &str, status: &str| Entry {
      id: id.into(),
      title: title.into(),
      status: status.into(),
      door: View::Item {
        kind: kind.into(),
        id: id.into(),
      },
    };
    vec![
      Entry {
        id: "issue".into(),
        title: "all issues".into(),
        status: String::new(),
        door: View::Collection {
          kind: "issue".into(),
        },
      },
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
    // And a COLLECTION is a destination like any other: the issues list is
    // one word away, which is what replaced the entities lobby as the way in.
    let m = matches(&idx, "issue", 8);
    assert_eq!(
      idx[m[0].entry].door,
      View::Collection {
        kind: "issue".into()
      },
      "typing a kind must offer its collection first"
    );
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

  /// **THE CARET IS WHERE THE NEXT CHARACTER LANDS**, which is the whole
  /// property the buffer gained. Driven as an operator drives it: type, go
  /// home, type again.
  #[test]
  fn typing_lands_at_the_caret_and_not_at_the_end() {
    let mut o = Omnibox::default();
    for c in "56".chars() {
      o.type_char(c);
    }
    assert_eq!((o.buffer.as_str(), o.cursor()), ("56", 2));
    o.home();
    o.type_char('S');
    assert_eq!(
      (o.buffer.as_str(), o.cursor()),
      ("S56", 1),
      "typing at the caret must insert there, not append"
    );
    o.end();
    o.type_char('!');
    assert_eq!(o.buffer, "S56!");
  }

  /// Backspace deletes BEFORE the caret and `C-d` deletes UNDER it -- the two
  /// are different keys because they are different acts, and an input that
  /// conflated them would eat the wrong character.
  #[test]
  fn backspace_deletes_behind_and_delete_forward_deletes_under() {
    let mut o = Omnibox::default();
    for c in "abc".chars() {
      o.type_char(c);
    }
    o.left();
    o.erase();
    assert_eq!(
      (o.buffer.as_str(), o.cursor()),
      ("ac", 1),
      "backspace ate the wrong side"
    );
    o.delete_forward();
    assert_eq!(
      (o.buffer.as_str(), o.cursor()),
      ("a", 1),
      "C-d must delete under the caret"
    );
    o.delete_forward();
    assert_eq!(o.buffer, "a", "C-d at the end is a no-op, not a panic");
    o.home();
    o.erase();
    assert_eq!(
      o.buffer, "a",
      "backspace at the start is a no-op, not a panic"
    );
  }

  /// **`C-u` IS READLINE'S KILL-TO-START, NOT CLEAR-THE-LINE.** An operator who
  /// knows the binding expects the tail to survive; emptying the buffer would
  /// be a different command wearing the same key.
  #[test]
  fn the_kill_keys_do_what_readline_does() {
    let mut o = Omnibox::default();
    for c in "one two three".chars() {
      o.type_char(c);
    }
    o.kill_word_back();
    assert_eq!(o.buffer, "one two ", "C-w must kill one word, not the line");
    o.kill_word_back();
    assert_eq!(o.buffer, "one ");

    let mut o = Omnibox::default();
    for c in "abcdef".chars() {
      o.type_char(c);
    }
    o.home();
    o.right();
    o.right();
    o.kill_to_end();
    assert_eq!(
      (o.buffer.as_str(), o.cursor()),
      ("ab", 2),
      "C-k kills forward from the caret"
    );

    let mut o = Omnibox::default();
    for c in "abcdef".chars() {
      o.type_char(c);
    }
    o.home();
    o.right();
    o.right();
    o.kill_to_start();
    assert_eq!(
      (o.buffer.as_str(), o.cursor()),
      ("cdef", 0),
      "C-u kills BACK to the start and keeps the tail -- it is not clear-the-line"
    );
  }

  /// **CHARS, NEVER BYTES.** The estate has paid for the other choice once,
  /// and criterion prose is full of typography.
  #[test]
  fn the_caret_walks_characters_and_never_splits_one() {
    let mut o = Omnibox::default();
    for c in "aé中z".chars() {
      o.type_char(c);
    }
    assert_eq!(o.cursor(), 4);
    o.home();
    o.right();
    o.delete_forward();
    assert_eq!(o.buffer, "a中z", "a multi-byte character must delete whole");
    o.end();
    o.erase();
    assert_eq!(o.buffer, "a中");
    o.erase();
    assert_eq!(
      o.buffer, "a",
      "backspacing a multi-byte character must not split it"
    );
  }

  /// Motions do not disturb the pick: moving the caret does not change the
  /// query, so it does not change what Enter would take.
  #[test]
  fn moving_the_caret_leaves_the_pick_alone() {
    let mut o = Omnibox::default();
    for c in "st".chars() {
      o.type_char(c);
    }
    o.pick_move(true, 5);
    let picked = o.picked(5);
    o.home();
    o.end();
    o.left();
    assert_eq!(o.picked(5), picked, "a caret motion moved the pick");
  }
}
