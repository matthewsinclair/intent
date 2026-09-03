//! The screen: `AT-17.11` covering `AC-17.11`, against `tui-design.md` §2.
//!
//! **THIS MODULE COMPUTES THE PICTURE AND DRAWS NOTHING**, which is the whole
//! reason it can be asserted at all. `AC-17.11` says the alignment IS the
//! design, so it is asserted rather than eyeballed -- and a property asserted
//! against a rendered terminal buffer is a property asserted against whatever
//! the renderer happened to do. Here the screen is DATA: every line is a string
//! this module produced, at a size it was handed, and [`super::draw`] is a
//! printer with no opinions. The buffer test one level up then checks that the
//! printer did not lie, which is a different question and a much smaller one.
//!
//! **IT CARRIES NO `ratatui`**, for the reason [`super::mode`],
//! [`super::terminal`] and [`super::focus`] carry no terminal: the realiser is
//! what these invariants CHECK.
//!
//! # The foot, and where its shape came from
//!
//! `AC-17.11` originally said *one modeline at the foot above a single rule*.
//! `tui-design.md` §2 -- ratified with hv on 2026-08-29, a day later, by
//! driving a strawman against real ST0056 data -- says **three sections
//! separated by two rules**, with the foot carrying a STATUS row, a COMMAND row
//! and an INFO row rather than one modeline. **vc ruled the design wins and
//! reworded the criterion**: a criterion that contradicts a ratified design is
//! the criterion being stale. This module follows the design.
//!
//! The five sections, and what each is for:
//!
//! - **APP** -- the entity's id and name. When nested it carries the view trail
//!   and the key that leaves. *A way back that is wired and unlabelled is a way
//!   back nobody finds* -- a real strawman defect, where `Backspace` worked and
//!   nothing on screen said so.
//! - **BODY** -- the flat `{name, value, type}` column ([`Plan`]).
//! - **STATUS** -- mode, field, kind, editability, row position, pane hint.
//! - **COMMAND** -- the command in play, the `:` line while composing, the menu
//!   in MENU, the child's name in EMBED.
//! - **INFO** -- help for whatever is under the cursor, changing per row.
//!
//! **REVISED AGAIN 2026-09-02 (hv's O1): THE COMPOSER IS FRAMED**, and it is
//! the one bordered element on the screen. The frame is paid for out of the
//! BODY rather than the chrome, so every degradation guarantee here is
//! unchanged and a viewport that cannot afford it renders exactly what it
//! rendered before -- see [`FRAME_COST`].
//!
//! **THE CHROME SITS AT FIXED VIEWPORT POSITIONS, NEVER AT THE END OF THE
//! CONTENT.** A screen laid out top-down puts the status row wherever the rows
//! happened to stop, which is the one place an operator cannot learn to look.
//!
//! # The two columns, and why the name column is capped
//!
//! `AC-17.11` demands the property be measured *over a form whose longest name
//! and longest value both exceed the viewport*. That requirement is what forces
//! the design: if the name column simply took the longest name, a name wider
//! than the terminal would leave zero columns for values, and "names align in
//! one column and values align in another" would be vacuously true of a screen
//! with no values on it. So the name column is capped at whatever leaves
//! [`MIN_VALUE_COLS`] for values, and names clip too.
//!
//! **The gutter is COMPUTED FROM THE ROW SET, never pinned.** The strawman
//! pinned it at 13 and real data collided on the first render.
//!
//! # Clipped visibly, never wrapped
//!
//! A wrapped value breaks the one guarantee the layout makes, because the
//! second line's text would start at column zero where a NAME belongs. So
//! [`plan`] emits exactly one line per row -- asserted, not intended -- and an
//! over-long value loses its tail to [`CLIPPED`] rather than its shape to a
//! second row.
//!
//! **CLIPPING HAPPENS HERE, AT RENDER, AND NEVER IN THE MODEL.** A value
//! truncated into the model is truncated for every width forever.
//!
//! **IT COUNTS CHARACTERS, NEVER BYTES.** This estate has already paid for the
//! other choice once. `AC-17.11`'s own subject makes it likely rather than
//! theoretical: criterion prose is the longest value in the model and it is
//! full of typography.

/// Columns between the name column and the value column.
pub const GAP: usize = 2;

/// The value column never shrinks below this, however long the longest name.
/// **A value column of zero would make the alignment property vacuous** rather
/// than false, which is the failure this constant exists to prevent.
pub const MIN_VALUE_COLS: usize = 8;

/// Marks a value that lost its tail.
pub const CLIPPED: char = '\u{2026}';

/// The rule. `tui-design.md` §2 specifies unicode box-drawing. These two rules
/// were *the only chrome on the screen* until hv ratified the framed composer
/// (O1, 2026-09-02); the rule now reads **borders on the composer and nowhere
/// else** -- see [`BOX_TL`]. They remain the only FULL-WIDTH chrome, which is
/// what the two-rules assertions actually pin.
pub const RULE: char = '\u{2500}';

/// The composer's box: the ONE bordered element on the screen (hv's O1,
/// ratified 2026-09-02). `tui-design.md` §2's *two rules are the only chrome*
/// is relaxed to exactly this and no further -- **borders are allowed on the
/// composer and nowhere else**, stated as a positive rule so a later reader
/// does not read the relaxation as open season.
pub const BOX_TL: char = '\u{256d}';
pub const BOX_TR: char = '\u{256e}';
pub const BOX_BL: char = '\u{2570}';
pub const BOX_BR: char = '\u{256f}';
pub const BOX_V: char = '\u{2502}';

/// What framing the composer costs: one line above it and one below.
///
/// **THE FRAME IS ADDITIVE AND IT IS THE FIRST THING A SHORT VIEWPORT GIVES
/// UP.** It is paid for out of the BODY rather than out of the chrome, so
/// every degradation guarantee below is unchanged: a terminal that cannot
/// afford it renders the composer exactly as it did before the frame existed.
/// **A border is the least informative line on the screen** -- it carries no
/// text at all -- so it is the correct thing to lose first, and losing it
/// costs the operator nothing but the look.
pub const FRAME_COST: usize = 2;

/// Lines above the body: the APP row and the rule under it.
pub const HEAD: usize = 2;

/// Lines below the body: the rule, then the OMNIBOX line and the HINT line.
/// Was 4 (STATUS + COMMAND + INFO) -- collapsed on hv's minimal-chrome
/// ruling: one mode-chipped hint line does the work three did, and the row
/// the foot gives back goes to the body.
pub const FOOT: usize = 3;

/// Every line the screen spends on chrome rather than on rows.
pub const CHROME: usize = HEAD + FOOT;

/// The one extra line a split BODY costs: the labelled rule between the panes.
pub const SPLIT_RULE: usize = 1;

/// The chrome of a screen whose BODY is split.
pub const SPLIT_CHROME: usize = CHROME + SPLIT_RULE;

/// The label on the rule between the panes.
pub const DETAIL_LABEL: &str = " detail ";

/// Stands in for "a rule goes here" while the degraded screen is assembled, so
/// the priority order reads as a list of sections rather than as arithmetic.
/// Never rendered: [`Screen::compose`] replaces it with a rule of the right
/// width. A sentinel no value can collide with, because a section's text is
/// operator-facing prose and this is not.
const RULE_MARK: &str = "\u{0}rule";

/// One line of the form: what `AC-17.11` calls `{title, value, type}`.
///
/// **`kind` IS CARRIED AND NOT PRINTED AS A THIRD COLUMN.** The row model is
/// the three the criterion names; the FORMATTING guarantee is two aligned
/// columns, and a third printed column would be a third alignment promise
/// nothing made. `kind` drives behaviour -- which rows hand off to `$EDITOR`,
/// which descend -- and the STATUS row is where the operator reads it.
///
/// **`name` IS THE ROW IDENTITY AND `title` IS WHAT THE OPERATOR READS.** They
/// are the same string on most rows and they are NOT the same fact: a form row
/// displays its LABEL (`work pkgs`) and is acted on by its declared field NAME
/// (`wps`). Recovering the name by indexing the declaration at render time
/// would be a second derivation of an order the first one already fixed --
/// `AC-17.10`'s handoff needs the name to write the field back, and a row that
/// can be acted on has to carry what identifies it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
  pub name: String,
  pub title: String,
  pub value: String,
  pub kind: String,
  /// What this row expands into, if anything.
  ///
  /// **THE SPLIT IS TRIGGERED BY THE ROW CARRYING DETAIL, NEVER BY A LIST OF
  /// VIEW KINDS** (`tui-design.md` section 6, in those words). A list of kinds
  /// is a second place to update when a new view arrives, *and it is the half
  /// that gets forgotten* -- the new view renders, looks complete, and silently
  /// has no detail pane. As a field on the row there is nothing to remember: a
  /// row that has detail shows detail.
  ///
  /// Detail rows are ROWS, so the two panes share one renderer. Stripping
  /// markup in one place and parsing it in the other is two encodings of one
  /// fact.
  pub detail: Option<Vec<Row>>,
  /// Where Enter on this row descends, if anywhere.
  ///
  /// **DECLARED ON THE ROW, NEVER INFERRED FROM ITS KIND** -- `tui-design.md`
  /// §6, in those words: working out where `documents` goes from the fact
  /// that it looks like a pane is the same guess-from-shape that made
  /// `intent edit st 68` misparse. The builders that know the model set it;
  /// a door-less `button` visibly opens nothing rather than guessing.
  pub door: Option<super::nav::View>,
}

/// The kind of a row that is a BOUNDARY rather than a thing.
///
/// **A SEPARATOR IS NOT AN ITEM, AND THE ROW MODEL HAD NO WAY TO SAY SO.**
/// Every other row is something the operator can select and open; this one is
/// chrome that happens to live in the body, because the body is the only place
/// a boundary BETWEEN rows can be drawn. So it is declared as a kind, the
/// cursor steps over it ([`super::app::App`]'s `Move`) and the position
/// indicator does not count it -- three places, because a separator that can be
/// selected, or that makes a list of ten report eleven, is worse than no
/// separator at all.
pub const RULE_KIND: &str = "rule";

impl Row {
  /// A horizontal rule across the body: a boundary between groups of rows.
  pub fn rule() -> Self {
    Row::new("", "", RULE_KIND)
  }

  /// Is this row a boundary rather than a thing the operator can act on?
  pub fn is_rule(&self) -> bool {
    self.kind == RULE_KIND
  }

  /// A row whose identity IS what it displays -- a thread id, an entity kind.
  pub fn new(title: impl Into<String>, value: impl Into<String>, kind: impl Into<String>) -> Self {
    let title = title.into();
    Self {
      name: title.clone(),
      title,
      value: value.into(),
      kind: kind.into(),
      detail: None,
      door: None,
    }
  }

  /// A row displayed under one string and acted on under another.
  pub fn named(
    name: impl Into<String>,
    title: impl Into<String>,
    value: impl Into<String>,
    kind: impl Into<String>,
  ) -> Self {
    Self {
      name: name.into(),
      title: title.into(),
      value: value.into(),
      kind: kind.into(),
      detail: None,
      door: None,
    }
  }

  /// The same row, carrying what it expands into.
  pub fn expanding_to(mut self, detail: Vec<Row>) -> Self {
    self.detail = Some(detail);
    self
  }

  /// Whether this row splits the BODY.
  ///
  /// **EMPTY DETAIL IS NOT DETAIL.** A `Some(vec![])` would open a pane with
  /// nothing in it, delimited by a rule separating nothing from nothing --
  /// which section 2 allows nowhere.
  /// Declare where Enter on this row descends.
  pub fn opening(mut self, view: super::nav::View) -> Self {
    self.door = Some(view);
    self
  }

  pub fn has_detail(&self) -> bool {
    self.detail.as_ref().is_some_and(|d| !d.is_empty())
  }
}

/// How the BODY divides between the list and the detail pane.
///
/// **THE DETAIL PANE TAKES WHAT IT NEEDS AND NEVER MORE THAN HALF.** A fixed
/// split wastes lines on a two-row detail and starves a long one. An UNCAPPED
/// one is worse: a criterion whose detail runs to a dozen rows would push the
/// list it was selected from off the screen, and the operator loses the thing
/// they were navigating in order to look at one item of it.
///
/// **A DETAIL PANE OF ZERO LINES IS NO SPLIT AT ALL**, which is why this
/// returns the whole body to the list rather than a rule with nothing under it.
pub fn divide(body: usize, detail_rows: usize) -> (usize, usize) {
  let for_detail = detail_rows.min(body / 2);
  (body - for_detail, for_detail)
}

/// The rule between the panes, with its label centred.
///
/// **THE LABEL IS DROPPED RATHER THAN CLIPPED ON A NARROW VIEWPORT.** A rule
/// reading `── deta` is a rule that looks broken; a plain rule looks like the
/// two the screen already carries, which is what it is.
pub fn labelled_rule(width: usize) -> String {
  if width < DETAIL_LABEL.chars().count() + 4 {
    return std::iter::repeat_n(RULE, width).collect();
  }
  let label = DETAIL_LABEL.chars().count();
  let left = (width - label) / 2;
  let right = width - label - left;
  let mut out: String = std::iter::repeat_n(RULE, left).collect();
  out.push_str(DETAIL_LABEL);
  out.extend(std::iter::repeat_n(RULE, right));
  out
}

/// What a run of characters IS, for the printer to colour.
///
/// **ROLES, NOT COLOURS, AND THE SPLIT IS THE MODULE TREE'S OWN**: this module
/// computes the picture and [`super::draw`] owns the palette, so a theme
/// change is a draw edit and an information change is a layout edit. hv's
/// ask (2026-08-30) was *a bit of colour and a small bit of flare* aimed at
/// one defect -- *the state changes between modes are not obvious* -- so the
/// vocabulary is deliberately small and the mode chip is the headline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
  /// Rules and structural furniture. Dim.
  Chrome,
  /// A field NAME. Dim -- the design brightens VALUES, not labels.
  Name,
  /// An editable value. Bright default.
  Value,
  /// A door: an id, an entity row, something Enter descends into.
  Door,
  /// A read-only or generated value.
  Muted,
  /// Semantic status: fine.
  Ok,
  /// Semantic status: in flight / attention.
  Warn,
  /// Semantic status: wrong.
  Error,
  /// The cursor row.
  Selected,
  /// A matched letter in the dropdown -- why this entry is offered.
  Match,
  /// The mode chip -- coloured PER MODE by the printer.
  ModeChip(super::mode::Mode),
  /// The composer's box: the one border the design allows.
  Frame,
  /// The omnibox line while the omnibox has the keyboard.
  OmniActive,
  /// The APP row's identity.
  Title,
}

/// The spans of one composed line: `(start, end, role)` in CHARACTERS,
/// non-overlapping except that a later span paints OVER an earlier one --
/// which is how the selection overlays a row without the row builders
/// knowing about cursors.
pub type Ink = Vec<(usize, usize, Role)>;

/// Which semantic role a rendered VALUE carries, judged from its display
/// form. **Display forms are the model's own vocabulary** (`form::field`
/// renders them), so the words matched here are the words the store shows;
/// an unknown word is simply a value, never a guess.
fn semantic(value: &str) -> Option<Role> {
  match value {
    "done" | "ok" | "green" | "satisfied" | "closed" => Some(Role::Ok),
    "wip" | "open" | "pending" | "triage" | "hold" => Some(Role::Warn),
    "red" | "blocked" | "cancelled" | "withdrawn" => Some(Role::Error),
    _ => None,
  }
}

/// The BODY: one rendered string per row, and the columns they share.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
  /// Where every name starts. Zero, always -- named rather than assumed so the
  /// assertion reads the same shape for both columns.
  pub name_col: usize,
  /// Where every value starts.
  pub value_col: usize,
  /// The width every line was planned against.
  pub width: usize,
  /// Exactly one line per input row, in input order.
  pub rows: Vec<String>,
  /// One [`Ink`] per line, same order -- computed in the same walk that
  /// builds the line, so the spans cannot describe text nobody rendered.
  pub inks: Vec<Ink>,
}

impl Plan {
  /// The rows visible in a window `height` tall, starting at `first`.
  ///
  /// **THE WINDOW IS A LAYOUT QUESTION AND THE SCROLL POSITION IS NOT.** Where
  /// the operator has scrolled to is application state that changes on a
  /// keystroke; which rows that position makes visible is arithmetic, and
  /// arithmetic is the thing worth asserting. An out-of-range `first` yields an
  /// empty window rather than panicking, because a form that shrinks under a
  /// held scroll position is an ordinary event and not a bug.
  pub fn visible(&self, first: usize, height: usize) -> &[String] {
    let start = first.min(self.rows.len());
    let end = start.saturating_add(height).min(self.rows.len());
    &self.rows[start..end]
  }
}

/// The tool's name, top-right of the APP row. hv's ask, 2026-09-02.
///
/// **THE TURTLE IS THE SAME MARK THE MENUBAR APP WEARS** (`AC-01.8`), so the
/// two faces of Intent are recognisable as one thing. It is the only emoji on
/// the screen and it is here rather than anywhere else for that reason.
pub const BRAND: &str = "\u{1f422} Intent";

/// What [`BRAND`] costs in COLUMNS, which is not what it costs in characters.
///
/// **THIS MODULE COUNTS CHARACTERS EVERYWHERE ELSE AND SAYS SO IN ITS OWN
/// HEADER**, which is right for text and wrong for an emoji: `🐢` is ONE char
/// and TWO columns. Every other string here is prose the operator or the model
/// wrote, and none of it has ever contained one. So the cost is DECLARED for
/// this one string rather than the module learning a general width model it
/// would then have to be right about everywhere.
///
/// **IT IS A RULE, NOT A MAGIC NUMBER**, and
/// [`the_brand_costs_what_it_says_it_costs`] holds it: ASCII is one column,
/// anything else is two. **The caveat is real and stated: a terminal that
/// renders the turtle single-width shifts the brand one column right.** That
/// is a display wart, not a correctness failure -- nothing is clipped by it,
/// because the brand is the LAST thing composed and the first thing dropped.
pub fn brand_cols() -> usize {
  cols(BRAND)
}

/// What a string costs in COLUMNS under the declared rule: ASCII is one,
/// anything else is two.
///
/// **THE RULE WAS DECLARED FOR ONE STRING AND NOW SERVES TWO**, because the
/// APP row carries the project's directory name beside the brand and a
/// directory name is the operator's, not ours. The rule stays a rule rather
/// than becoming a width model: a name in a script this over-counts reserves a
/// column too many and shifts the group one left, which is the same display
/// wart [`BRAND`] already documents for a single-width turtle, and it errs
/// toward reserving rather than clipping.
pub fn cols(s: &str) -> usize {
  s.chars().map(|c| if c.is_ascii() { 1 } else { 2 }).sum()
}

/// What separates the project from the brand on the APP row.
pub const BRAND_SEP: &str = " | ";

/// The APP row with the project and [`BRAND`] against its right edge:
/// `Utilz | \u{1f422} Intent`.
///
/// **THE RIGHT GROUP DEGRADES RIGHT-TO-LEFT, AND THE ORDER IS THE WHOLE
/// POINT** (hv, 2026-09-03). This said the brand *is the least informative
/// thing on the screen -- the operator knows which program they are running*,
/// which is true of the brand and FALSE of the project name: that one answers
/// *which checkout am I in*, and it is the question that bites hardest on a
/// machine carrying two of them. So the turtle goes first and the project name
/// is held longest; a narrowing viewport must never silently take away the one
/// fact this row was widened to carry.
///
/// **THE TEXT STILL OUTRANKS BOTH.** Where even the project name cannot be
/// afforded, the row says where you ARE and the group goes entirely -- the
/// same degradation order this module applies to the composer's frame.
///
/// An empty `project` composes the brand alone, which is what every caller
/// without a store on the other end gets.
fn branded(left: &str, project: &str, w: usize) -> (String, Ink) {
  let text = clip(left, w);
  let used = text.chars().count();
  // One column of air between the text and the group, minimum, or they read as
  // one string. Right-to-left: the fullest group that still fits wins.
  let full = format!("{project}{BRAND_SEP}{BRAND}");
  let Some(right) = [full.as_str(), project, BRAND]
    .into_iter()
    .filter(|c| !c.is_empty())
    .find(|c| used + 1 + cols(c) <= w)
  else {
    return (text, vec![(0, used, Role::Title)]);
  };
  let pad = w - used - cols(right);
  let line = format!("{text}{}{right}", " ".repeat(pad));
  // **THE INK IS IN CHARACTERS BECAUSE THE PRINTER INDEXES CHARACTERS**, so
  // the brand's span is its char count and not its column cost. The two differ
  // by exactly the emoji, and using the wrong one here would colour one
  // character too many and bleed the brand's role into nothing.
  let start = used + pad;
  (
    line,
    vec![
      (0, used, Role::Title),
      (start, start + right.chars().count(), Role::Door),
    ],
  )
}

/// The first body row to paint so that `selected` is on screen.
///
/// **THE SCROLL IS DERIVED, NEVER STORED, AND THAT IS A DEFECT FIX RATHER THAN
/// A REFACTOR.** `App` carried a `scroll` field that was assigned `0` in three
/// places and incremented in NONE -- so a body taller than the viewport could
/// not be reached past the fold, and the cursor walked off the bottom with the
/// selection painted nowhere. It is the `Command::group` class one field over:
/// declared, plausible, and read by something that could never see it move.
///
/// Derived, the reset rule the design states -- *cursor and scroll reset with
/// the view* -- holds by construction, because the cursor resets and the
/// scroll is a function of it.
pub fn scroll_to(selected: Option<usize>, height: usize) -> usize {
  match selected {
    // **NO CURSOR MEANS THE TOP, NOT WHEREVER IT WAS.** An empty view has no
    // focus (`AC-17.5`), and a remembered offset into rows that are gone is an
    // offset into nothing.
    None => 0,
    Some(at) if height == 0 => at,
    // The window is the last `height` rows ending at the cursor, or the top.
    Some(at) => at.saturating_sub(height - 1),
  }
}

/// Add the one-cell cursor overlay to a composer line's ink.
///
/// **LAST IN THE INK, BECAUSE THE PRINTER APPLIES SPANS IN ORDER** -- the
/// composer's own role already covers this cell, and a caret painted before it
/// would be overwritten by the thing it sits on top of. That is the same rule
/// the dropdown's `Role::Selected` follows, and it is one line away from being
/// silently wrong.
///
/// **A CARET PAST `limit` IS DROPPED RATHER THAN CLAMPED.** The line is
/// clipped to the viewport, so a cursor scrolled off the right-hand end has no
/// cell to sit on; clamping would park it on the last visible character and
/// claim the operator is typing there.
fn caret_ink(ink: &mut Ink, at: Option<usize>, limit: usize) {
  if let Some(at) = at
    && at < limit
  {
    ink.push((at, at + 1, Role::Selected));
  }
}

/// The whole screen: five sections, composed at a fixed size.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Screen {
  pub app: String,
  /// The project's DIRECTORY name, shown left of the brand on the APP row.
  ///
  /// **THE DIRECTORY, NOT `config.project_name`**, ruled by hv 2026-09-03 and
  /// it is the discriminating one: `~/Devel/prj/Intentv2` declares
  /// `project_name: "Intent"`, so the configured name cannot tell two checkouts
  /// of one project apart and the directory can. Empty where there is no
  /// project on the other end, which composes the brand alone.
  pub project: String,
  pub body: Plan,
  /// The lower pane, when the selected row carries detail.
  ///
  /// **ITS OWN COLUMNS, NOT THE LIST'S.** `AC-17.11` guarantees that names
  /// align in one column and values in another *across the whole form*, and the
  /// detail pane is a different pane with a different row set -- forcing it to
  /// the list's gutter would indent four short names past a column computed for
  /// `attachments`. Each pane keeps the guarantee within itself, which is what
  /// the guarantee is about; a shared gutter would be a promise neither pane
  /// made.
  pub detail: Option<Plan>,
  /// The omnibox line: prompt + buffer. **The caret is NOT in it** -- see
  /// [`Screen::caret`].
  pub omnibox: String,
  /// Which cell of [`Screen::omnibox`] the text cursor is on, in characters
  /// from the start of the line.
  ///
  /// **A CURSOR IS A PROPERTY OF A CELL, NOT A CHARACTER IN THE LINE**, and
  /// this field is a defect hv drove into. The caret used to be a glyph
  /// spliced into the buffer at the cursor: correct-looking at the END of the
  /// line, where it lands after the last character, and wrong everywhere else
  /// -- the glyph takes a column, so everything after the cursor shifts one
  /// right, and `C-a` appeared to insert a space in front of the text.
  ///
  /// Painted as an overlay, the way the dropdown's pick already is. `None`
  /// where the composer does not hold the keyboard.
  pub caret: Option<usize>,
  /// The hint line: mode chip + whatever helps RIGHT NOW. A notice takes it.
  pub hint: String,
  /// Dropdown lines composed by the caller, painted ABOVE the bottom rule,
  /// eating body rows -- present only while the omnibox has matches to show.
  pub dropdown: Vec<(String, Ink)>,
  /// The live mode, for the chip's per-mode colour and the omnibox line's
  /// active/inactive brightness.
  pub mode: super::mode::Mode,
  /// The cursor's body-row index (list pane), for the selection overlay.
  pub selected: Option<usize>,
  /// The hint line is a NOTICE -- something happened -- rather than help.
  pub noticed: bool,
}

impl Screen {
  /// How many body rows fit at `height`. Zero when the chrome alone does not
  /// fit, which is a small terminal rather than an error.
  pub fn body_height(height: usize) -> usize {
    height.saturating_sub(CHROME)
  }

  /// The first body row to paint at `height`, so the cursor stays on screen.
  ///
  /// **THE SCREEN ANSWERS THIS, NOT THE APP**, because the answer needs the
  /// viewport height and the app has never had one -- which is exactly why the
  /// stored `scroll` could not move. See [`scroll_to`].
  pub fn first_row(&self, height: usize) -> usize {
    scroll_to(self.selected, Self::body_height(height))
  }

  /// Compose exactly `height` lines, scrolled so body row `first` is at the top
  /// of the body.
  ///
  /// **EVERY LINE IS PLACED BY POSITION, NOT BY APPENDING**, which is what
  /// makes the chrome's location a property of the viewport rather than of the
  /// content. Padding rows are empty strings, so nothing is painted over them.
  pub fn compose(&self, first: usize, height: usize) -> Vec<String> {
    self
      .painted(first, height)
      .into_iter()
      .map(|(line, _)| line)
      .collect()
  }

  /// [`Screen::compose`] with each line's [`Ink`]: THE one walk -- `compose`
  /// derives from this, so the coloured picture and the plain one cannot
  /// disagree about what is on a line.
  pub fn painted(&self, first: usize, height: usize) -> Vec<(String, Ink)> {
    let w = self.body.width;
    if height == 0 {
      return Vec::new();
    }
    // **A VIEWPORT TOO SHORT FOR THE CHROME DEGRADES IN A DECLARED ORDER**,
    // and the order is the point: body rows go first, then the APP row, and
    // the foot is kept longest. Mode and the command line are what an operator
    // needs to get OUT of a pane that has been dragged shut; rows they cannot
    // read are not.
    //
    // **THE SECOND RULE GOES WITH THE BODY IT DELIMITED.** A rule separating
    // nothing from nothing is decoration, and the design allows none -- so the
    // degraded screen carries ONE rule, under the APP row, for as long as the
    // APP row survives.
    if height < CHROME {
      let degraded = [
        self.app.as_str(),
        RULE_MARK,
        self.omnibox.as_str(),
        self.hint.as_str(),
      ];
      return degraded
        .iter()
        .rev()
        .take(height)
        .rev()
        .map(|s| {
          if *s == RULE_MARK {
            let line: String = std::iter::repeat_n(RULE, w).collect();
            let n = line.chars().count();
            (line, vec![(0, n, Role::Chrome)])
          } else {
            (clip(s, w), Ink::new())
          }
        })
        .collect();
    }

    // **THE FRAME IS AFFORDED OUT OF THE BODY, AND ONLY IF A BODY ROW
    // SURVIVES IT.** A screen that spent its last two lines on a border would
    // have traded the content for the decoration, which is the opposite of the
    // degradation order this module already declares.
    let framed = w > 4 && height >= CHROME + FRAME_COST + 1;
    let frame_cost = if framed { FRAME_COST } else { 0 };

    let rule: String = std::iter::repeat_n(RULE, w).collect();
    let rule_ink: Ink = vec![(0, rule.chars().count(), Role::Chrome)];
    let whole = |line: &str, role: Role| -> Ink { vec![(0, line.chars().count(), role)] };
    let mut out: Vec<(String, Ink)> = Vec::with_capacity(height);
    let (app, app_ink) = branded(&self.app, &self.project, w);
    out.push((app, app_ink));
    out.push((rule.clone(), rule_ink.clone()));

    // **THE SPLIT IS THE LAST THING TO SURVIVE A SHRINKING VIEWPORT, NOT THE
    // FIRST.** A detail pane needs the labelled rule AND at least one line
    // under it; where the body cannot pay for both, the LIST is what stays,
    // because it is what the operator navigates with and the detail is one
    // keystroke away again. `divide` returning zero says the same thing, and
    // both routes land on the unsplit body below rather than on a rule with
    // nothing beneath it.
    let split = match &self.detail {
      Some(d) if height >= SPLIT_CHROME + frame_cost => {
        let body_h = height - SPLIT_CHROME - frame_cost;
        let (list_h, detail_h) = divide(body_h, d.rows.len());
        (detail_h > 0).then_some((list_h, detail_h, d))
      }
      _ => None,
    };

    // **THE SELECTION IS AN OVERLAY, PUSHED LAST**, so the row builders know
    // nothing about cursors and the printer resolves overlap by order.
    let body_rows =
      |out: &mut Vec<(String, Ink)>, plan: &Plan, from: usize, h: usize, sel: bool| {
        let lines = plan.visible(from, h);
        for (i, line) in lines.iter().enumerate() {
          let mut ink = plan.inks.get(from + i).cloned().unwrap_or_default();
          if sel && self.selected == Some(from + i) {
            ink.push((0, line.chars().count(), Role::Selected));
          }
          out.push((line.clone(), ink));
        }
        for _ in lines.len()..h {
          out.push((String::new(), Ink::new()));
        }
      };

    match split {
      Some((list_h, detail_h, detail)) => {
        body_rows(&mut out, &self.body, first, list_h, true);
        let labelled = labelled_rule(w);
        let ink = whole(&labelled, Role::Chrome);
        out.push((labelled, ink));
        body_rows(&mut out, detail, 0, detail_h, false);
      }
      None => {
        let body_h = height - CHROME - frame_cost;
        body_rows(&mut out, &self.body, first, body_h, true);
      }
    }

    // **THE DROPDOWN EATS THE BODY FROM THE BOTTOM, NEVER THE CHROME** --
    // it slots in above the bottom rule, replacing the last body rows, so
    // the omnibox line it belongs to stays exactly where the operator's eye
    // already is. Clipped to the body it can actually eat.
    if !self.dropdown.is_empty() {
      let body_len = out.len().saturating_sub(HEAD);
      // **THE DROPDOWN IS DELIMITED FROM THE BODY BY A RULE** (hv, 2026-09-02,
      // driving it): without one the offers sit flush against the rows they
      // are not part of, and the eye reads them as more body. It is the SAME
      // rule the composer's box sits under -- one chrome vocabulary, not a
      // second separator invented for this list.
      //
      // **THE RULE IS THE FIRST THING THE BLOCK GIVES UP.** Where the body can
      // pay for offers but not for offers-plus-a-separator, the OFFERS win: a
      // separator with nothing to separate is the chrome-delimiting-nothing
      // the design already refuses, and the match is the thing the operator
      // asked for.
      let take = (self.dropdown.len() + 1).min(body_len);
      let (rows, ruled) = if take > 1 {
        (take - 1, true)
      } else {
        (take, false)
      };
      out.truncate(out.len() - take);
      if ruled {
        out.push((rule.clone(), rule_ink.clone()));
      }
      for (line, ink) in self.dropdown.iter().take(rows) {
        out.push((clip(line, w), ink.clone()));
      }
    }

    out.push((rule, rule_ink));
    let omnibox = clip(&self.omnibox, w);
    let omnibox_role = if self.mode == super::mode::Mode::Omni {
      Role::OmniActive
    } else {
      Role::Muted
    };
    if framed {
      // `╭──╮` / `│ text │` / `╰──╯`, each exactly `w` columns so the printer
      // never has to reason about width.
      let bar: String = std::iter::repeat_n(RULE, w - 2).collect();
      let top: String = format!("{BOX_TL}{bar}{BOX_TR}");
      let bottom: String = format!("{BOX_BL}{bar}{BOX_BR}");
      let edge: Ink = vec![(0, w, Role::Frame)];
      let inner = w - 4;
      let text = clip(&omnibox, inner);
      let pad = inner.saturating_sub(text.chars().count());
      let line = format!("{BOX_V} {text}{} {BOX_V}", " ".repeat(pad));
      let mut ink: Ink = vec![
        (0, 2, Role::Frame),
        (2, 2 + text.chars().count(), omnibox_role),
        (w - 2, w, Role::Frame),
      ];
      // The frame and its space push the text two columns in, so the caret's
      // offset within the LINE is two more than its offset within the text.
      caret_ink(&mut ink, self.caret.map(|c| c + 2), 2 + inner);
      out.push((top, edge.clone()));
      out.push((line, ink));
      out.push((bottom, edge));
    } else {
      let mut omnibox_ink = whole(&omnibox, omnibox_role);
      caret_ink(&mut omnibox_ink, self.caret, omnibox.chars().count());
      out.push((omnibox, omnibox_ink));
    }
    // The mode chip leads the hint line and is coloured PER MODE -- hv's
    // "the state changes between modes are not obvious", answered where the
    // state is written.
    let hint = clip(&self.hint, w);
    // **THE CHIP SPANS THE LAMP, NOT THE MACHINE'S NAME**, because the lamp is
    // what `run::hint_row` actually writes there. They agree for OMNI and MENU
    // and differ for FIELD/EMBED, both of which show `EDIT` -- so measuring the
    // name would have coloured five characters of a four-character word and
    // bled the chip into the hint after it.
    let chip = self.mode.lamp().chars().count().min(hint.chars().count());
    let mut hint_ink: Ink = vec![(0, chip, Role::ModeChip(self.mode))];
    let tail_role = if self.noticed {
      Role::Warn
    } else {
      Role::Muted
    };
    hint_ink.push((chip, hint.chars().count(), tail_role));
    out.push((hint, hint_ink));
    out
  }
}

/// Cut `s` to `w` columns, marking the cut when one happens.
///
/// Counts `chars`, so a cut never lands inside a multi-byte character.
fn clip(s: &str, w: usize) -> String {
  if w == 0 {
    return String::new();
  }
  if s.chars().count() <= w {
    return s.to_string();
  }
  let mut out: String = s.chars().take(w - 1).collect();
  out.push(CLIPPED);
  out
}

/// Lay `rows` out at `width` into the BODY column.
pub fn plan(rows: &[Row], width: usize) -> Plan {
  let longest = rows
    .iter()
    .map(|r| r.title.chars().count())
    .max()
    .unwrap_or(0);
  // The cap, not the longest name, is what keeps the value column alive.
  let room_for_names = width.saturating_sub(GAP + MIN_VALUE_COLS);
  let name_width = longest.min(room_for_names);
  let value_col = if width == 0 {
    0
  } else {
    (name_width + GAP).min(width)
  };
  let value_width = width.saturating_sub(value_col);

  let mut lines = Vec::with_capacity(rows.len());
  let mut inks = Vec::with_capacity(rows.len());
  for r in rows {
    // **A RULE ROW IS ONE LINE LIKE ANY OTHER**, which is what keeps the row
    // index and the line index the same number -- the selected overlay and the
    // scroll window both address rows by position, so a row rendering as two
    // lines (or none) would silently move the highlight off the row it names.
    if r.is_rule() {
      let line: String = std::iter::repeat_n(RULE, width).collect();
      let n = line.chars().count();
      inks.push(vec![(0, n, Role::Chrome)]);
      lines.push(line);
      continue;
    }
    let name = clip(&r.title, name_width);
    let pad = name_width - name.chars().count();
    let value = clip(&r.value, value_width);
    // Trailing space is decoration; the line ends where its content does.
    let mut line = String::with_capacity(width);
    let mut ink: Ink = Vec::with_capacity(2);
    if !name.is_empty() {
      // A row that IS a door -- an entity in a collection, an entity kind --
      // wears the door colour on its NAME, which is the column that carries
      // its identity; a field row's name is a label and stays dim.
      let name_role = if r.door.is_some() {
        Role::Door
      } else {
        Role::Name
      };
      ink.push((0, name.chars().count(), name_role));
    }
    line.push_str(&name);
    if !value.is_empty() {
      line.extend(std::iter::repeat_n(' ', pad + GAP));
      let start = line.chars().count();
      line.push_str(&value);
      let role = semantic(&value).unwrap_or(match r.kind.as_str() {
        "artefact" => Role::Muted,
        "button" if r.door.is_none() => Role::Muted,
        "button" => Role::Value,
        _ => Role::Value,
      });
      ink.push((start, line.chars().count(), role));
    }
    lines.push(line);
    inks.push(ink);
  }

  Plan {
    name_col: 0,
    value_col,
    width,
    rows: lines,
    inks,
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use intentsvcs::form::Loaded;

  /// **THIS FIXTURE IS THE CRITERION'S OWN POSITIVE CONTROL**, which is why the
  /// width is a named constant with a test behind it: `AC-17.11` measures the
  /// property over a form whose longest name AND longest value both exceed the
  /// viewport, and `the_fixture_is_the_hard_case_the_criterion_names` asserts
  /// that it does.
  ///
  /// **30 RATHER THAN 40, AND A FAILING RUN IS WHY.** The first version used 40
  /// against a longest name of 34, so the name did NOT exceed the viewport, the
  /// name column never hit its cap, and every alignment assertion was quietly
  /// measuring the easy case. At 30 both the longest name (34) and the longest
  /// value (76) exceed it, and the value column lands exactly on
  /// [`MIN_VALUE_COLS`], which is the boundary the cap exists to hold.
  const NARROW: usize = 30;

  fn hard_rows() -> Vec<Row> {
    vec![
      Row::new("title", "ST0056: Intent v3", "text"),
      Row::new("status", "WIP", "select"),
      Row::new(
        "parity/tools/conservation_check.sh",
        "a value long enough that it cannot fit beside a name that does not fit either",
        "button",
      ),
      Row::new("objective", "", "prose"),
    ]
  }

  fn screen() -> Screen {
    Screen {
      detail: None,
      app: "ST0056   Add a Rust-based CLI".into(),
      project: String::new(),
      body: plan(&hard_rows(), NARROW),
      omnibox: "\u{276f}".into(),
      caret: Some(1),
      hint: "NAV  1/4  \u{23ce} edit".into(),
      dropdown: Vec::new(),
      mode: super::super::mode::Mode::Omni,
      selected: None,
      noticed: false,
    }
  }

  /// **THE BRAND'S COLUMN COST IS A RULE, NOT A NUMBER**, and this is the
  /// rule: ASCII is one column, anything else is two. Asserted because the
  /// module counts CHARACTERS everywhere else and would be off by one here --
  /// `🐢` is one char and two columns.
  #[test]
  fn the_brand_costs_what_it_says_it_costs() {
    let chars = BRAND.chars().count();
    let cols = brand_cols();
    assert!(
      cols > chars,
      "the brand is pure ASCII, so nothing here needed a column model at all -- either the \
       turtle went missing or this rule is now dead weight"
    );
    assert_eq!(
      cols - chars,
      BRAND.chars().filter(|c| !c.is_ascii()).count(),
      "the column cost and the character count disagree by something other than the emoji"
    );
  }

  /// hv's ask: the tool names itself, top-right, above the rule.
  #[test]
  fn the_app_row_carries_the_brand_against_its_right_edge() {
    let w = 60;
    let (line, ink) = branded("thread", "", w);
    assert!(
      line.ends_with(BRAND),
      "the brand is not against the right edge: {line:?}"
    );
    assert!(
      line.starts_with("thread"),
      "the brand displaced the view name"
    );
    assert_eq!(
      line.chars().count() + brand_cols() - BRAND.chars().count(),
      w,
      "the row does not fill the width once the emoji's second column is counted"
    );
    assert!(
      ink.iter().any(|&(_, _, r)| r == Role::Door),
      "the brand carries no ink of its own"
    );
  }

  /// **THE BRAND IS DROPPED, NEVER CLIPPED**, and never at the cost of the
  /// text that says where you are.
  #[test]
  fn a_viewport_too_narrow_for_both_keeps_the_view_name_and_drops_the_brand() {
    for w in 1..=(brand_cols() + 6) {
      let (line, _) = branded("thread", "", w);
      assert!(
        !line.contains('\u{1f422}') || line.ends_with(BRAND),
        "at width {w} the brand was clipped rather than dropped: {line:?}"
      );
      assert!(
        line.chars().count() <= w,
        "at width {w} the row overflowed: {line:?}"
      );
    }
    let (line, _) = branded("thread", "", 8);
    assert!(
      line.starts_with("thread") && !line.contains('\u{1f422}'),
      "a narrow row lost the view name rather than the brand: {line:?}"
    );
  }

  /// **THE PROJECT SITS LEFT OF THE BRAND AND OUTLIVES IT WHEN SPACE RUNS
  /// OUT** -- hv's ask, 2026-09-03, and the ORDER is the half worth testing.
  ///
  /// Dropping right-to-left is what makes the row worth widening: the brand is
  /// decoration the operator can infer, and the project name answers *which
  /// checkout am I in*. A degradation that took them in the other order, or
  /// together, would remove the new fact first and leave the old decoration --
  /// and it would look completely fine at full width, which is where a screen
  /// is always read when it is being checked.
  #[test]
  fn the_project_sits_beside_the_brand_and_is_the_last_of_the_two_to_go() {
    let w = 60;
    let (line, ink) = branded("thread", "Utilz", w);
    assert!(
      line.ends_with(&format!("Utilz{BRAND_SEP}{BRAND}")),
      "the project must sit immediately left of the brand: {line:?}"
    );
    assert!(
      line.starts_with("thread"),
      "the group displaced the view name"
    );
    assert!(
      ink.iter().any(|&(_, _, r)| r == Role::Door),
      "the right-hand group carries no ink of its own"
    );

    // Narrow until each piece goes, and assert the ORDER rather than the widths
    // -- the widths are arithmetic, the order is the ruling.
    let full = cols("Utilz") + cols(BRAND_SEP) + cols(BRAND);
    let mut seen_project_alone = false;
    for w in 1..=(full + "thread".len() + 2) {
      let (line, _) = branded("thread", "Utilz", w);
      assert!(
        line.chars().count() <= w,
        "at width {w} the row overflowed: {line:?}"
      );
      let has_brand = line.contains('\u{1f422}');
      let has_project = line.contains("Utilz");
      assert!(
        !has_brand || has_project,
        "at width {w} the BRAND survived and the project did not -- the drop ran \
         left-to-right: {line:?}"
      );
      if has_project && !has_brand {
        seen_project_alone = true;
      }
      assert!(
        line.starts_with("thread") || w < "thread".len(),
        "at width {w} the row lost the view name before the group: {line:?}"
      );
    }
    assert!(
      seen_project_alone,
      "no width dropped the brand and kept the project, so the ordering this test \
       exists for was never exercised"
    );
  }

  /// The window follows the cursor, and at the top it does not move.
  #[test]
  fn the_window_holds_the_cursor_wherever_the_cursor_is() {
    let height = 10;
    for at in 0..40usize {
      let first = scroll_to(Some(at), height);
      assert!(
        at >= first && at < first + height,
        "row {at} is outside the window {first}..{}",
        first + height
      );
      if at < height {
        assert_eq!(
          first, 0,
          "the window scrolled while the cursor was on screen"
        );
      }
    }
    assert_eq!(
      scroll_to(None, height),
      0,
      "a view with no cursor did not start at the top"
    );
  }

  #[test]
  fn the_fixture_is_the_hard_case_the_criterion_names() {
    let rows = hard_rows();
    let longest_name = rows.iter().map(|r| r.title.chars().count()).max().unwrap();
    let longest_value = rows.iter().map(|r| r.value.chars().count()).max().unwrap();
    assert!(
      longest_name > NARROW,
      "the fixture's longest name is {longest_name} at a viewport of {NARROW}, so it does not \
       exceed the viewport and every alignment assertion below is measuring the easy case"
    );
    assert!(
      longest_value > NARROW,
      "the fixture's longest value is {longest_value} at a viewport of {NARROW}, so nothing here \
       exercises clipping"
    );
  }

  /// The criterion's whole content, asserted against the STRINGS rather than
  /// against the arithmetic that made them -- a check that recomputed
  /// `value_col` and compared it to itself would pass for any layout at all.
  #[test]
  fn every_name_starts_at_one_column_and_every_value_at_another() {
    let rows = hard_rows();
    let p = plan(&rows, NARROW);
    let mut examined = 0usize;
    for (i, (row, line)) in rows.iter().zip(p.rows.iter()).enumerate() {
      let chars: Vec<char> = line.chars().collect();
      assert!(
        chars[p.name_col] != ' ',
        "row {i} does not start its name at column 0: {line:?}"
      );
      if row.value.is_empty() {
        assert_eq!(
          chars.len(),
          row.title.chars().count().min(NARROW),
          "row {i} has no value, so its line must end where its name does: {line:?}"
        );
        continue;
      }
      // **A ROW WITH A VALUE MUST REACH THE VALUE COLUMN. This is an ASSERTION
      // and was once a `continue`**, which made the whole test vacuous: a
      // layout that put values immediately after each name produced lines too
      // short to reach `value_col`, every row was skipped, and the check passed
      // green over nothing. Measured -- mutating the padding to zero left all
      // seven tests passing.
      assert!(
        chars.len() > p.value_col,
        "row {i} carries a value but its line is {} columns, which does not reach the value \
         column at {}. Values must all START at that column: {line:?}",
        chars.len(),
        p.value_col
      );
      assert!(
        chars[p.value_col] != ' ',
        "row {i} does not start its value at column {}: {line:?}",
        p.value_col
      );
      for (x, c) in chars
        .iter()
        .enumerate()
        .take(p.value_col)
        .skip(p.value_col - GAP)
      {
        assert_eq!(
          *c, ' ',
          "row {i} has {c:?} at column {x}, inside the gap: {line:?}"
        );
      }
      examined += 1;
    }
    // **THE CONTROL THAT WAS MISSING.** `the_fixture_is_the_hard_case` proves
    // the FIXTURE is hard; it says nothing about whether the loop above looked
    // at anything. A count is the only thing that distinguishes "every row
    // aligned" from "no row was examined".
    assert_eq!(
      examined,
      rows.iter().filter(|r| !r.value.is_empty()).count(),
      "the alignment assertion examined {examined} rows, which is not every row that carries a \
       value. A loop that skips its subject is green for free"
    );
    assert!(
      examined > 0,
      "no row carried a value, so this test asserted nothing at all"
    );
  }

  #[test]
  fn a_value_that_does_not_fit_is_clipped_and_marked_rather_than_wrapped() {
    let rows = hard_rows();
    let p = plan(&rows, NARROW);
    assert_eq!(
      p.rows.len(),
      rows.len(),
      "the layout emitted a different number of lines than it was given rows, which is what \
       wrapping looks like from the outside"
    );
    assert!(
      p.rows[2].ends_with(CLIPPED),
      "a value too long for its column must say so; it ended {:?}",
      p.rows[2]
    );
  }

  /// The estate has cut a multi-byte character in half before.
  #[test]
  fn a_cut_never_lands_inside_a_multi_byte_character() {
    let value = "\u{2014}".repeat(10);
    let rows = vec![Row::new("f", value, "text")];
    for width in 4..14 {
      let p = plan(&rows, width);
      let line = &p.rows[0];
      assert!(line.is_char_boundary(line.len()));
      assert!(line.chars().count() <= width);
    }
  }

  /// **THE SHAPE `tui-design.md` §2 SPECIFIES**, asserted by position because
  /// that is the whole claim: the chrome is where the viewport says, not where
  /// the content stopped.
  #[test]
  fn the_screen_is_five_sections_separated_by_two_rules() {
    let s = screen();
    let rule: String = std::iter::repeat_n(RULE, NARROW).collect();
    for height in [CHROME, CHROME + 1, 12, 40] {
      let lines = s.compose(0, height);
      assert_eq!(
        lines.len(),
        height,
        "compose must fill exactly the height it was given"
      );
      assert_eq!(
        lines[0],
        clip(&s.app, NARROW),
        "the APP row is the first line"
      );
      assert_eq!(lines[1], rule, "a rule sits directly under the APP row");
      // **POSITIONS ARE DERIVED FROM THE DECLARED CONSTANTS, NEVER COUNTED
      // BACK FROM THE BOTTOM BY HAND.** The foot grew when hv's framed
      // composer landed and it will grow again when the status segments do;
      // a test that hardcodes `height - 3` has to be rewritten on each of
      // those, and rewriting an assertion to match what the code now does is
      // how a check stops being one.
      let framed = NARROW > 4 && height >= CHROME + FRAME_COST + 1;
      let foot = if framed { FOOT + FRAME_COST } else { FOOT };
      assert_eq!(
        lines[height - foot],
        rule,
        "a rule sits directly above the foot (framed: {framed})"
      );
      if framed {
        assert!(
          lines[height - foot + 1].starts_with(BOX_TL),
          "the composer's box opens under the rule: {:?}",
          lines[height - foot + 1]
        );
        assert!(
          lines[height - 3].contains(s.omnibox.trim()),
          "the composer's text sits inside its box: {:?}",
          lines[height - 3]
        );
        assert!(
          lines[height - 2].starts_with(BOX_BL),
          "the composer's box closes above the hint: {:?}",
          lines[height - 2]
        );
      } else {
        assert_eq!(
          lines[height - 2],
          s.omnibox,
          "unframed, the composer line is second from the bottom"
        );
      }
      assert_eq!(lines[height - 1], s.hint, "the HINT line is the last line");
      // **STILL EXACTLY TWO FULL-WIDTH RULES.** The box's own horizontals are
      // not rules: they open and close with corners, so they never equal this
      // string. The relaxation `tui-design.md` §2 takes is BORDERS ON THE
      // COMPOSER AND NOWHERE ELSE, and this is the half of that which is
      // checkable here.
      assert_eq!(
        lines.iter().filter(|l| **l == rule).count(),
        2,
        "exactly two rules; the composer's box is a border and not a third rule"
      );
    }
  }

  /// The chrome does not move when the content does. A form of one row in a
  /// twenty-line terminal finds its status row where a form of four hundred does.
  #[test]
  fn the_chrome_holds_its_position_whatever_the_content_does() {
    let short = Screen {
      body: plan(&hard_rows()[..1], NARROW),
      ..screen()
    };
    let long_rows: Vec<Row> = (0..400)
      .map(|i| Row::new(format!("row{i}"), format!("value {i}"), "text"))
      .collect();
    let long = Screen {
      body: plan(&long_rows, NARROW),
      ..screen()
    };
    let a = short.compose(0, 20);
    let b = long.compose(0, 20);
    for i in [0, 1, 17, 18, 19] {
      assert_eq!(
        a[i], b[i],
        "chrome line {i} moved when the row count changed"
      );
    }
    assert_ne!(a[2], b[2], "the body did not change when the rows did");
  }

  #[test]
  fn no_line_is_ever_wider_than_the_viewport() {
    for width in [0usize, 1, 2, 9, 12, NARROW, 200] {
      let s = Screen {
        body: plan(&hard_rows(), width),
        ..screen()
      };
      for height in [0usize, 1, 3, CHROME, 20] {
        for line in s.compose(0, height) {
          assert!(
            line.chars().count() <= width,
            "at {width}x{height} a line was {} columns: {line:?}",
            line.chars().count()
          );
        }
      }
    }
  }

  /// **A PANE DRAGGED SHUT MUST NOT PANIC, AND MUST SHOW THE WAY OUT.** With
  /// less room than the chrome needs, the foot survives and the rows go: mode
  /// and the command line are what an operator needs to leave, and rows they
  /// cannot read are not.
  #[test]
  fn a_viewport_too_short_for_the_chrome_keeps_the_foot() {
    let s = screen();
    assert!(
      s.compose(0, 0).is_empty(),
      "a zero-height viewport composes nothing"
    );
    for height in 1..CHROME {
      let lines = s.compose(0, height);
      assert_eq!(lines.len(), height);
      assert_eq!(
        *lines.last().unwrap(),
        s.hint,
        "the HINT row survives to the last line"
      );
    }
  }

  /// **THE PROPERTY OVER THE REAL CORPUS, not only over a fixture built to
  /// exhibit it.** A fixture proves the layout can align; the shipped forms
  /// prove it does.
  #[test]
  fn every_shipped_form_lays_out_with_its_columns_aligned() {
    let loaded = Loaded::load().expect("the shipped form declaration must load");
    assert!(
      !loaded.forms().is_empty(),
      "no forms to lay out, so this test asserts nothing"
    );
    let mut checked = 0usize;
    for form in loaded.forms() {
      let rows: Vec<Row> = form
        .fields
        .iter()
        .map(|f| Row::new(f.label.clone(), format!("<{}>", f.name), f.widget.clone()))
        .collect();
      assert!(!rows.is_empty(), "a shipped form declares no fields");
      let p = plan(&rows, NARROW);
      assert_eq!(p.rows.len(), rows.len());
      for line in &p.rows {
        let chars: Vec<char> = line.chars().collect();
        assert!(
          chars.len() > p.value_col,
          "a shipped form's row does not reach the value column: {line:?}"
        );
        assert!(
          chars[p.value_col] != ' ',
          "a shipped form's value does not start at column {}: {line:?}",
          p.value_col
        );
        checked += 1;
      }
    }
    assert!(
      checked > 0,
      "no shipped row was examined, so this test asserted nothing"
    );
  }

  fn detail_rows() -> Vec<Row> {
    vec![
      Row::new("kind", "test", "text"),
      Row::new("state", "computed", "text"),
      Row::new("evidence", "layout.rs", "text"),
      Row::new(
        "text",
        "A CHECKER VERIFIES MEMBERSHIP IN A VOCABULARY",
        "prose",
      ),
    ]
  }

  fn split() -> Screen {
    Screen {
      detail: Some(plan(&detail_rows(), NARROW)),
      ..screen()
    }
  }

  /// **NEVER MORE THAN HALF, AND NEVER LESS THAN THE LIST NEEDS.** An uncapped
  /// detail pane pushes the list it was selected from off the screen, so the
  /// operator loses the thing they were navigating in order to look at one item
  /// of it.
  #[test]
  fn the_detail_pane_takes_what_it_needs_and_never_more_than_half() {
    let mut examined = 0usize;
    for body in 0..40usize {
      for wanted in 0..40usize {
        let (list, detail) = divide(body, wanted);
        assert_eq!(list + detail, body, "divide({body}, {wanted}) lost a line");
        assert!(
          detail <= body / 2,
          "divide({body}, {wanted}) gave the detail more than half"
        );
        assert!(
          detail <= wanted,
          "divide({body}, {wanted}) gave the detail more lines than it has rows"
        );
        assert!(
          list >= detail,
          "divide({body}, {wanted}) left the list smaller than the pane it opened"
        );
        examined += 1;
      }
    }
    assert!(examined > 0, "no division was examined");
    assert_eq!(divide(1, 9), (1, 0), "a body of one line cannot be split");
    assert_eq!(divide(0, 9), (0, 0));
  }

  /// The rule between the panes is exactly as wide as everything else, and it
  /// **drops its label rather than clipping it** -- `-- deta` reads as broken
  /// where a plain rule reads as the two the screen already carries.
  #[test]
  fn the_labelled_rule_fills_its_width_and_drops_the_label_before_clipping_it() {
    let mut carried = 0usize;
    let mut plain = 0usize;
    for w in 0..60usize {
      let r = labelled_rule(w);
      assert_eq!(
        r.chars().count(),
        w,
        "the rule at width {w} is not {w} wide"
      );
      if r.contains(DETAIL_LABEL.trim()) {
        carried += 1;
      } else {
        assert!(
          r.chars().all(|c| c == RULE),
          "a rule without its label must be a plain rule: {r:?}"
        );
        plain += 1;
      }
    }
    assert!(
      carried > 0 && plain > 0,
      "only one of the two arms was driven"
    );
  }

  /// **THE CHROME HOLDS ITS POSITION WHETHER THE BODY IS SPLIT OR NOT.** The
  /// APP row, the rules and the three foot lines are where an operator looks;
  /// a split that shifted them would move the mode indicator every time the
  /// cursor crossed a row that happened to carry detail.
  #[test]
  fn a_split_body_leaves_the_app_row_and_the_foot_exactly_where_they_were() {
    for height in [SPLIT_CHROME, SPLIT_CHROME + 3, 24usize] {
      let flat = screen().compose(0, height);
      let cut = split().compose(0, height);
      assert_eq!(flat.len(), height);
      assert_eq!(cut.len(), height, "a split screen is not {height} lines");
      assert_eq!(cut[0], flat[0], "the APP row moved at height {height}");
      for back in 1..=3usize {
        assert_eq!(
          cut[height - back],
          flat[height - back],
          "foot line -{back} moved at height {height}"
        );
      }
    }
  }

  /// **THE SPLIT IS THE FIRST THING TO GO AND THE LIST IS WHAT STAYS.** A body
  /// that cannot pay for the labelled rule AND a line under it renders unsplit
  /// rather than showing a rule with nothing beneath it -- section 2 allows no
  /// chrome that delimits nothing.
  #[test]
  fn a_body_too_short_for_both_panes_keeps_the_list_and_drops_the_detail() {
    let label = DETAIL_LABEL.trim();
    let mut split_at = 0usize;
    let mut flat_at = 0usize;
    // **THE SWEEP WIDENS WITH THE FOOT.** The framed composer takes two lines
    // out of the body, so the height at which a split becomes affordable moved
    // up by exactly that -- a sweep pinned to the old range stops straddling
    // the boundary and silently tests one side twice.
    for height in CHROME..=(SPLIT_CHROME + FRAME_COST + 2) {
      let lines = split().compose(0, height);
      assert_eq!(lines.len(), height);
      if lines.iter().any(|l| l.contains(label)) {
        split_at += 1;
      } else {
        flat_at += 1;
      }
    }
    assert!(
      split_at > 0 && flat_at > 0,
      "the sweep never saw both outcomes, so it is not driving the degradation it names"
    );
    assert!(
      !split().compose(0, CHROME).iter().any(|l| l.contains(label)),
      "a body with no room for a detail pane still opened one"
    );
  }

  /// **EMPTY DETAIL IS NOT DETAIL.** `Some(vec![])` would open a pane with
  /// nothing in it under a rule separating nothing from nothing.
  #[test]
  fn an_empty_detail_row_set_does_not_split_anything() {
    let s = Screen {
      detail: Some(plan(&[], NARROW)),
      ..screen()
    };
    assert!(
      !s.compose(0, 24)
        .iter()
        .any(|l| l.contains(DETAIL_LABEL.trim())),
      "an empty detail pane was opened"
    );
  }

  /// **EACH PANE KEEPS `AC-17.11`'s ALIGNMENT WITHIN ITSELF.** They do not
  /// share a gutter, and they must not have to: the guarantee is that names
  /// align in one column and values in another, and a shared gutter computed
  /// for `attachments` would indent four short detail names past it.
  #[test]
  fn both_panes_are_internally_aligned_without_sharing_a_gutter() {
    let list = plan(&hard_rows(), NARROW);
    let detail = plan(&detail_rows(), NARROW);
    for (what, p, rows) in [
      ("list", &list, hard_rows()),
      ("detail", &detail, detail_rows()),
    ] {
      let mut checked = 0usize;
      for (line, row) in p.rows.iter().zip(rows.iter()) {
        if row.value.is_empty() {
          continue;
        }
        let chars: Vec<char> = line.chars().collect();
        assert!(
          chars.len() > p.value_col,
          "{what}: a row carrying a value is shorter than its own value column"
        );
        assert_ne!(
          chars[p.value_col], ' ',
          "{what}: the value column does not start where the plan says: {line:?}"
        );
        checked += 1;
      }
      assert_eq!(
        checked,
        rows.iter().filter(|r| !r.value.is_empty()).count(),
        "{what}: some row carrying a value was never examined"
      );
    }
    assert_ne!(
      list.value_col, detail.value_col,
      "the two panes happen to share a gutter here, so this test cannot show they compute their \
       own -- pick fixtures whose longest names differ"
    );
  }

  /// No line is ever wider than the viewport, **the labelled rule included** --
  /// it is the one chrome line built from a width rather than clipped to one.
  #[test]
  fn no_line_of_a_split_screen_is_wider_than_the_viewport() {
    for width in [8usize, NARROW, 100] {
      let s = Screen {
        detail: Some(plan(&detail_rows(), width)),
        body: plan(&hard_rows(), width),
        ..screen()
      };
      for (i, line) in s.compose(0, 24).iter().enumerate() {
        assert!(
          line.chars().count() <= width,
          "line {i} is {} wide at viewport {width}: {line:?}",
          line.chars().count()
        );
      }
    }
  }
  /// **THE COLOUR LAYER'S CONTRACT, PINNED WHERE THE SPANS ARE MADE.** Roles,
  /// not colours -- the palette is draw's -- so what this asserts is the
  /// INFORMATION: the mode chip leads the status line and names the live
  /// mode, the omnibox line brightens exactly when the omnibox holds the
  /// keyboard, the selection overlays the cursor row and only that row, and
  /// a status value wears its semantic role. hv's ask, 2026-08-30: *the
  /// state changes between modes are not obvious. Maybe a bit of colour?*
  #[test]
  fn the_mode_chip_and_omnibox_line_change_ink_with_the_mode() {
    use super::super::mode::Mode;
    for mode in [Mode::Omni, Mode::Menu, Mode::Field] {
      let mut sc = screen();
      sc.mode = mode;
      sc.hint = format!("{}   title", mode.lamp());
      let painted = sc.painted(0, 24);
      let (hint, hint_ink) = &painted[painted.len() - 1];
      assert_eq!(
        hint_ink.first(),
        Some(&(0, mode.lamp().chars().count(), Role::ModeChip(mode))),
        "the hint line must LEAD with a chip naming {mode:?}: {hint:?} {hint_ink:?}"
      );
      // **THE COMPOSER IS INSIDE ITS BOX NOW, so its ink is the span AFTER the
      // left border rather than the first on the line.** At 24 rows the frame
      // is affordable, so the last three lines are box-bottom, hint -- and the
      // composer sits above them.
      let (_, command_ink) = &painted[painted.len() - 3];
      let want = if mode == Mode::Omni {
        Role::OmniActive
      } else {
        Role::Muted
      };
      assert_eq!(
        command_ink.first().map(|&(_, _, r)| r),
        Some(Role::Frame),
        "the composer line must OPEN with its border in {mode:?}: {command_ink:?}"
      );
      assert_eq!(
        command_ink.get(1).map(|&(_, _, r)| r),
        Some(want),
        "the composer's TEXT must read {want:?} in {mode:?} -- brightness IS the focus signal"
      );
    }
  }

  #[test]
  fn the_selection_overlays_the_cursor_row_and_only_that_row() {
    let mut sc = screen();
    sc.selected = Some(1);
    let painted = sc.painted(0, 24);
    let body_first = 2; // app row + rule
    for (i, (line, ink)) in painted.iter().enumerate() {
      let selected_here = ink
        .iter()
        .any(|&(s, e, r)| r == Role::Selected && s == 0 && e == line.chars().count());
      assert_eq!(
        selected_here,
        i == body_first + 1,
        "Selected must cover exactly the cursor row (line {i}): {line:?} {ink:?}"
      );
    }
    // And it is pushed LAST, so the printer paints it over the row's own ink.
    let (_, ink) = &painted[body_first + 1];
    assert_eq!(
      ink.last().map(|&(_, _, r)| r),
      Some(Role::Selected),
      "the overlay must be the LAST span or the row's own colours win"
    );
  }

  #[test]
  fn a_status_value_wears_its_semantic_role_and_a_door_wears_the_doors() {
    let rows = vec![
      Row::new("status", "wip", "select"),
      Row::new("state", "done", "select"),
      Row::new("verdict", "blocked", "select"),
      Row::new("ST0056", "Add a Rust-based CLI", "button").opening(super::super::nav::View::Item {
        kind: "thread".into(),
        id: "ST0056".into(),
      }),
    ];
    let p = plan(&rows, 60);
    let role_of_value = |i: usize| {
      p.inks[i]
        .iter()
        .find(|&&(s, _, _)| s >= p.value_col)
        .map(|&(_, _, r)| r)
    };
    assert_eq!(role_of_value(0), Some(Role::Warn), "wip is in-flight");
    assert_eq!(role_of_value(1), Some(Role::Ok), "done is fine");
    assert_eq!(role_of_value(2), Some(Role::Error), "blocked is wrong");
    assert_eq!(
      p.inks[3].first().map(|&(_, _, r)| r),
      Some(Role::Door),
      "a doored row's NAME is the door -- its identity column, not its value"
    );
    assert_eq!(
      p.inks[0].first().map(|&(_, _, r)| r),
      Some(Role::Name),
      "a field row's name is a label and stays dim"
    );
  }

  /// Every span indexes characters its line actually has -- the contract the
  /// printer leans on, asserted across a whole composed screen including the
  /// clipped hard-case rows.
  #[test]
  fn every_span_stays_inside_its_line() {
    let mut sc = screen();
    sc.selected = Some(0);
    sc.detail = Some(plan(&detail_rows(), NARROW));
    let mut spans = 0usize;
    for (line, ink) in sc.painted(0, 24) {
      let n = line.chars().count();
      for &(start, end, role) in &ink {
        assert!(
          start <= end && end <= n,
          "span ({start},{end},{role:?}) exceeds a {n}-char line: {line:?}"
        );
        spans += 1;
      }
    }
    assert!(
      spans > 8,
      "almost nothing was painted, so this asserted almost nothing"
    );
  }
}
