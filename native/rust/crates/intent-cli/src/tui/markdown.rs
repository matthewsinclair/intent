//! A field's markdown as lines and role spans: the contents pane (issue 0399).
//!
//! **THE PARSER IS `pulldown-cmark` AND THE LOOK IS DECIDED HERE, IN THE PURE
//! LAYER.** [`super::layout`] composes the whole screen as strings plus [`Role`]
//! spans, so that what the operator sees can be asserted without a terminal, and
//! [`super::draw`] is a printer with no opinions. A markdown widget that painted
//! `ratatui` text directly would move the pane's layout into the printer, where
//! none of it could be asserted -- so this renders into the same [`Plan`] the
//! body is laid out in, and the pane is composed, scrolled and checked exactly
//! as the body is.
//!
//! **THE MARKUP IS READ, NOT SHOWN.** Heading markers, emphasis stars and code
//! fences are dropped, and what they meant travels as a role. Bullets, quote
//! gutters and rules are drawn, because they are structure the reader needs and
//! the source could only spell them with punctuation.
//!
//! **EVERY LINE FITS THE WIDTH IT WAS GIVEN.** Prose wraps at spaces and a word
//! is cut only when it alone is wider than the line; code and tables are cut at
//! the edge rather than reflowed, because reflowing either changes what it says.
//! Widths are counted in `chars`, the unit every span in [`super::layout`] uses.

use pulldown_cmark::{Alignment, Event, Options, Parser, Tag, TagEnd};

use super::layout::{Ink, Plan, Role};

/// What the pane shows for a field with nothing in it. **The pane still says
/// something**: an empty pane under its rule would be a rule separating nothing
/// from nothing, which the design allows nowhere.
pub const EMPTY: &str = "(empty)";

const BULLET: &str = "\u{2022} ";
const QUOTE_BAR: &str = "\u{2502} ";
const RULE: char = '\u{2500}';
const CODE_INDENT: &str = "  ";
const COLUMN_GAP: &str = "  ";

/// Render `source` as markdown at `width`.
pub fn render(source: &str, width: usize) -> Plan {
  let options = Options::ENABLE_TABLES
    | Options::ENABLE_STRIKETHROUGH
    | Options::ENABLE_TASKLISTS
    | Options::ENABLE_YAML_STYLE_METADATA_BLOCKS;
  let mut out = Writer::new(width);
  for event in Parser::new_ext(source, options) {
    out.event(event);
  }
  out.finish()
}

/// A table as the parser hands it over: collected whole, because a column's
/// width is only known once every row has been read.
struct Table {
  alignments: Vec<Alignment>,
  head: Vec<String>,
  body: Vec<Vec<String>>,
  row: Vec<String>,
  cell: String,
}

/// The line being filled, the lines already finished, and the block context a
/// new line inherits.
struct Writer {
  width: usize,
  rows: Vec<String>,
  inks: Vec<Ink>,
  line: String,
  ink: Ink,
  /// The open line's length in chars.
  len: usize,
  /// A line is open: its prefix has been written.
  open: bool,
  /// The open line carries content beyond its prefix.
  started: bool,
  /// A space separates the next word from the last one, if they share a line.
  space_owed: bool,
  /// A blank line separates the next block from the last one.
  gap_owed: bool,
  /// Inline roles in force, innermost last.
  inline: Vec<Role>,
  quotes: usize,
  /// One entry per open list: the next number, or `None` for bullets.
  lists: Vec<Option<u64>>,
  /// The marker width of each open item, which is its hanging indent.
  items: Vec<usize>,
  /// The current item's marker, until its first line has been opened.
  marker: Option<String>,
  code: Option<String>,
  table: Option<Table>,
  /// Inside a YAML front-matter block, which is read and not shown: a
  /// generated view opens with one, and it repeats the fields above the pane.
  metadata: bool,
}

impl Writer {
  fn new(width: usize) -> Self {
    Self {
      width,
      rows: Vec::new(),
      inks: Vec::new(),
      line: String::new(),
      ink: Ink::new(),
      len: 0,
      open: false,
      started: false,
      space_owed: false,
      gap_owed: false,
      inline: Vec::new(),
      quotes: 0,
      lists: Vec::new(),
      items: Vec::new(),
      marker: None,
      code: None,
      table: None,
      metadata: false,
    }
  }

  fn event(&mut self, event: Event<'_>) {
    match event {
      Event::Start(tag) => self.start(tag),
      Event::End(tag) => self.end(tag),
      Event::Text(t) | Event::InlineMath(t) | Event::DisplayMath(t) => self.text(&t),
      Event::Code(t) => self.inline_as(Role::Code, &t),
      Event::Html(t) | Event::InlineHtml(t) => self.inline_as(Role::Muted, &t),
      Event::FootnoteReference(label) => self.inline_as(Role::Muted, &format!("[^{label}]")),
      Event::SoftBreak => self.text(" "),
      Event::HardBreak => self.close_line(),
      Event::Rule => self.rule(),
      Event::TaskListMarker(done) => self.task(done),
    }
  }

  fn start(&mut self, tag: Tag<'_>) {
    match tag {
      Tag::Paragraph => self.block(),
      Tag::Heading { .. } => {
        self.block();
        self.inline.push(Role::Heading);
      }
      Tag::BlockQuote(_) => {
        self.block();
        self.quotes += 1;
        self.inline.push(Role::Muted);
      }
      Tag::CodeBlock(_) => {
        self.block();
        self.code = Some(String::new());
      }
      Tag::HtmlBlock => {
        self.block();
        self.inline.push(Role::Muted);
      }
      Tag::List(first) => {
        // A nested list continues its item rather than starting a new block.
        if self.lists.is_empty() {
          self.block();
        } else {
          self.close_line();
        }
        self.lists.push(first);
      }
      Tag::Item => {
        self.close_line();
        let marker = match self.lists.last_mut() {
          Some(Some(n)) => {
            let marker = format!("{n}. ");
            *n += 1;
            marker
          }
          _ => BULLET.to_string(),
        };
        self.items.push(marker.chars().count());
        self.marker = Some(marker);
      }
      Tag::Emphasis => self.inline.push(Role::Emphasis),
      Tag::Strong => self.inline.push(Role::Strong),
      Tag::Strikethrough => self.inline.push(Role::Muted),
      Tag::Link { .. } => self.inline.push(Role::Link),
      Tag::Image { .. } => {
        self.inline.push(Role::Muted);
        self.text("[image: ");
      }
      Tag::FootnoteDefinition(label) => {
        self.block();
        self.inline_as(Role::Muted, &format!("[{label}]: "));
      }
      Tag::MetadataBlock(_) => self.metadata = true,
      Tag::Table(alignments) => {
        self.block();
        self.table = Some(Table {
          alignments,
          head: Vec::new(),
          body: Vec::new(),
          row: Vec::new(),
          cell: String::new(),
        });
      }
      _ => {}
    }
  }

  fn end(&mut self, tag: TagEnd) {
    match tag {
      TagEnd::Paragraph | TagEnd::FootnoteDefinition => {
        self.close_line();
        self.gap_owed = true;
      }
      TagEnd::Heading(_) | TagEnd::HtmlBlock => {
        self.close_line();
        self.inline.pop();
        self.gap_owed = true;
      }
      TagEnd::BlockQuote(_) => {
        self.close_line();
        self.inline.pop();
        self.quotes = self.quotes.saturating_sub(1);
        self.gap_owed = true;
      }
      TagEnd::CodeBlock => {
        if let Some(code) = self.code.take() {
          self.code_lines(&code);
        }
        self.gap_owed = true;
      }
      TagEnd::List(_) => {
        self.close_line();
        self.lists.pop();
        if self.lists.is_empty() {
          self.gap_owed = true;
        }
      }
      TagEnd::Item => {
        self.close_line();
        self.items.pop();
        self.marker = None;
      }
      TagEnd::Emphasis | TagEnd::Strong | TagEnd::Strikethrough | TagEnd::Link => {
        self.inline.pop();
      }
      TagEnd::Image => {
        self.text("]");
        self.inline.pop();
      }
      TagEnd::TableCell => {
        if let Some(t) = &mut self.table {
          let cell = std::mem::take(&mut t.cell);
          t.row.push(cell.trim().to_string());
        }
      }
      TagEnd::TableHead => {
        if let Some(t) = &mut self.table {
          t.head = std::mem::take(&mut t.row);
        }
      }
      TagEnd::TableRow => {
        if let Some(t) = &mut self.table {
          let row = std::mem::take(&mut t.row);
          t.body.push(row);
        }
      }
      TagEnd::MetadataBlock(_) => self.metadata = false,
      TagEnd::Table => {
        if let Some(t) = self.table.take() {
          self.table_lines(&t);
        }
        self.gap_owed = true;
      }
      _ => {}
    }
  }

  /// A block begins: finish the line before it and pay any blank line owed.
  fn block(&mut self) {
    self.close_line();
    if self.gap_owed && !self.rows.is_empty() {
      self.gap();
    }
    self.gap_owed = false;
  }

  /// A blank line that keeps the quote gutters but never spends an item's
  /// marker, with its trailing spaces removed.
  fn gap(&mut self) {
    let marker = self.marker.take();
    self.open_line();
    let kept = self.line.trim_end().chars().count();
    self.line = self.line.chars().take(kept).collect();
    self.len = kept;
    self.close_line();
    self.marker = marker;
  }

  /// Open a line: quote gutters, the hanging indent, and the item marker once.
  fn open_line(&mut self) {
    if self.open {
      return;
    }
    self.open = true;
    for _ in 0..self.quotes {
      self.put(QUOTE_BAR, Some(Role::Chrome));
    }
    let outer = if self.marker.is_some() {
      self.items.len().saturating_sub(1)
    } else {
      self.items.len()
    };
    let hang: usize = self.items[..outer].iter().sum();
    if hang > 0 {
      self.put(&" ".repeat(hang), None);
    }
    if let Some(marker) = self.marker.take() {
      self.put(&marker, Some(Role::Chrome));
    }
  }

  fn close_line(&mut self) {
    if !self.open {
      return;
    }
    self.rows.push(std::mem::take(&mut self.line));
    self.inks.push(std::mem::take(&mut self.ink));
    self.len = 0;
    self.open = false;
    self.started = false;
    self.space_owed = false;
  }

  /// Append `s` to the open line, in `role` if one is given.
  fn put(&mut self, s: &str, role: Option<Role>) {
    let start = self.len;
    self.line.push_str(s);
    self.len += s.chars().count();
    if let Some(role) = role {
      self.ink.push((start, self.len, role));
    }
  }

  /// Append `s` in every inline role in force, innermost painted last.
  fn styled(&mut self, s: &str) {
    let start = self.len;
    self.put(s, None);
    let end = self.len;
    self
      .ink
      .extend(self.inline.iter().map(|&role| (start, end, role)));
  }

  fn text(&mut self, t: &str) {
    if self.metadata {
      return;
    }
    if let Some(code) = &mut self.code {
      code.push_str(t);
      return;
    }
    if let Some(table) = &mut self.table {
      table.cell.push_str(t);
      return;
    }
    if t.starts_with(char::is_whitespace) {
      self.space_owed = true;
    }
    let mut words = t.split_whitespace().peekable();
    while let Some(word) = words.next() {
      self.word(word);
      if words.peek().is_some() {
        self.space_owed = true;
      }
    }
    if t.ends_with(char::is_whitespace) {
      self.space_owed = true;
    }
  }

  fn inline_as(&mut self, role: Role, t: &str) {
    self.inline.push(role);
    self.text(t);
    self.inline.pop();
  }

  /// One word, on this line if it fits beside what is there, else on the next,
  /// and cut only when it is wider than a line on its own.
  fn word(&mut self, word: &str) {
    let mut rest: Vec<char> = word.chars().collect();
    while !rest.is_empty() {
      self.open_line();
      let space = usize::from(self.started && self.space_owed);
      let room = self.width.saturating_sub(self.len + space);
      if rest.len() <= room {
        if space == 1 {
          self.put(" ", None);
        }
        let s: String = rest.drain(..).collect();
        self.styled(&s);
        self.started = true;
        self.space_owed = false;
      } else if self.started {
        self.close_line();
      } else {
        let take = room.max(1).min(rest.len());
        let s: String = rest.drain(..take).collect();
        self.styled(&s);
        self.started = true;
        self.close_line();
      }
    }
  }

  fn rule(&mut self) {
    self.block();
    self.open_line();
    let n = self.width.saturating_sub(self.len);
    self.put(&RULE.to_string().repeat(n), Some(Role::Chrome));
    self.close_line();
    self.gap_owed = true;
  }

  fn task(&mut self, done: bool) {
    self.open_line();
    self.put(if done { "[x] " } else { "[ ] " }, Some(Role::Chrome));
    self.started = true;
    self.space_owed = false;
  }

  /// A code block, line for line and indented, cut at the edge rather than
  /// wrapped at spaces: reflowing code changes what it says.
  fn code_lines(&mut self, code: &str) {
    for source in code.lines() {
      let mut rest: Vec<char> = source.chars().collect();
      loop {
        self.open_line();
        self.put(CODE_INDENT, None);
        let room = self.width.saturating_sub(self.len).max(1);
        let s: String = rest.drain(..room.min(rest.len())).collect();
        if !s.is_empty() {
          self.put(&s, Some(Role::Code));
        }
        self.close_line();
        if rest.is_empty() {
          break;
        }
      }
    }
  }

  /// A table with its columns lined up: the header strong and ruled off, each
  /// column as wide as its widest cell, aligned the way the source asked.
  fn table_lines(&mut self, t: &Table) {
    let columns = t
      .body
      .iter()
      .map(Vec::len)
      .chain([t.head.len(), t.alignments.len()])
      .max()
      .unwrap_or(0);
    let mut widths = vec![0usize; columns];
    for row in std::iter::once(&t.head).chain(t.body.iter()) {
      for (c, cell) in row.iter().enumerate() {
        widths[c] = widths[c].max(cell.chars().count());
      }
    }
    if !t.head.is_empty() {
      self.table_row(&t.head, &widths, &t.alignments, Some(Role::Strong));
      self.open_line();
      for (c, w) in widths.iter().enumerate() {
        if c > 0 {
          self.put(COLUMN_GAP, None);
        }
        self.put(&RULE.to_string().repeat(*w), Some(Role::Chrome));
      }
      self.close_line();
    }
    for row in &t.body {
      self.table_row(row, &widths, &t.alignments, None);
    }
  }

  fn table_row(
    &mut self,
    row: &[String],
    widths: &[usize],
    alignments: &[Alignment],
    role: Option<Role>,
  ) {
    self.open_line();
    for (c, w) in widths.iter().enumerate() {
      if c > 0 {
        self.put(COLUMN_GAP, None);
      }
      let cell = row.get(c).map_or("", String::as_str);
      let align = alignments.get(c).copied().unwrap_or(Alignment::None);
      self.put(&pad(cell, *w, align), role);
    }
    self.close_line();
  }

  /// The finished pane. **The width is enforced here, once, over every line**,
  /// so no construct above can hand the layout a line wider than the pane.
  fn finish(mut self) -> Plan {
    self.close_line();
    if self.rows.is_empty() {
      self.open_line();
      self.put(EMPTY, Some(Role::Muted));
      self.close_line();
    }
    let width = self.width;
    for (row, ink) in self.rows.iter_mut().zip(self.inks.iter_mut()) {
      if row.chars().count() > width {
        *row = row.chars().take(width).collect();
      }
      let n = row.chars().count();
      ink.retain_mut(|(start, end, _)| {
        *end = (*end).min(n);
        *start < *end
      });
    }
    Plan {
      name_col: 0,
      value_col: 0,
      width,
      rows: self.rows,
      inks: self.inks,
    }
  }
}

fn pad(cell: &str, width: usize, align: Alignment) -> String {
  let room = width.saturating_sub(cell.chars().count());
  match align {
    Alignment::Right => format!("{}{cell}", " ".repeat(room)),
    Alignment::Center => format!(
      "{}{cell}{}",
      " ".repeat(room / 2),
      " ".repeat(room - room / 2)
    ),
    Alignment::None | Alignment::Left => format!("{cell}{}", " ".repeat(room)),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn has(plan: &Plan, row: usize, span: (usize, usize, Role)) -> bool {
    plan.inks[row].contains(&span)
  }

  /// **THE MARKUP IS READ AND NOT SHOWN**: the heading's marker, the stars, the
  /// backticks and the fence are gone from every line, and what each meant is on
  /// the span instead.
  #[test]
  fn the_markup_is_read_and_not_shown() {
    let source =
      "# Title\n\nSome **bold** and `code` here.\n\n- one\n- two\n\n```\nlet x = 1;\n```\n";
    let plan = render(source, 40);
    assert_eq!(
      plan.rows,
      vec![
        "Title",
        "",
        "Some bold and code here.",
        "",
        "\u{2022} one",
        "\u{2022} two",
        "",
        "  let x = 1;",
      ]
    );
    assert!(
      has(&plan, 0, (0, 5, Role::Heading)),
      "the heading lost its role"
    );
    assert!(
      has(&plan, 2, (5, 9, Role::Strong)),
      "the bold word lost its role"
    );
    assert!(
      has(&plan, 2, (14, 18, Role::Code)),
      "the inline code lost its role"
    );
    assert!(
      has(&plan, 4, (0, 2, Role::Chrome)),
      "the bullet is not drawn as structure"
    );
    assert!(
      has(&plan, 7, (2, 12, Role::Code)),
      "the code block lost its role"
    );
  }

  /// A paragraph wraps at spaces, and a list item's second line hangs under
  /// the item's text rather than under its bullet.
  #[test]
  fn prose_wraps_at_spaces_and_an_item_hangs_under_its_own_text() {
    let plan = render("- alpha beta gamma delta", 14);
    assert_eq!(plan.rows, vec!["\u{2022} alpha beta", "  gamma delta"]);
  }

  /// A table's columns line up, its header is ruled off, and a column aligned
  /// right in the source is aligned right on screen.
  #[test]
  fn a_table_lines_its_columns_up() {
    let plan = render("| a | bb |\n|---|---:|\n| ccc | d |\n", 40);
    assert_eq!(
      plan.rows,
      vec![
        "a    bb",
        "\u{2500}\u{2500}\u{2500}  \u{2500}\u{2500}",
        "ccc   d"
      ]
    );
    assert!(
      has(&plan, 0, (0, 3, Role::Strong)),
      "the header is not strong"
    );
  }

  /// **NO LINE IS WIDER THAN THE PANE AND NO SPAN LEAVES ITS LINE**, swept over
  /// every width a pane can plausibly have and every construct this renders.
  #[test]
  fn no_line_is_wider_than_its_width_and_no_span_leaves_its_line() {
    let source = "# A heading long enough to wrap\n\n> quoted **text** that goes on and on\n\n\
                  1. first item with averylongwordthatcannotfitanywhere\n   - nested bullet\n2. second\n\n\
                  ```\nfn main() { println!(\"a line of code wider than most panes\"); }\n```\n\n\
                  | column | other |\n|---|---|\n| x | y |\n\n---\n\n- [x] done\n\nend";
    for width in 1..=60usize {
      let plan = render(source, width);
      assert!(
        plan.rows.len() > 5,
        "almost nothing rendered at width {width}"
      );
      assert_eq!(plan.rows.len(), plan.inks.len());
      for (row, ink) in plan.rows.iter().zip(&plan.inks) {
        let n = row.chars().count();
        assert!(n <= width, "a {n}-char line at width {width}: {row:?}");
        for &(start, end, role) in ink {
          assert!(
            start < end && end <= n,
            "span ({start},{end},{role:?}) outside a {n}-char line: {row:?}"
          );
        }
      }
    }
  }

  /// A generated view's front matter is read and not shown.
  #[test]
  fn front_matter_is_not_shown() {
    let plan = render("---\nst_id: ST0075\nstatus: Triage\n---\n\n# Heading\n", 40);
    assert_eq!(plan.rows, vec!["Heading"]);
  }

  /// An empty field still says what it holds.
  #[test]
  fn an_empty_field_says_so() {
    for source in ["", "   \n\n"] {
      assert_eq!(render(source, 20).rows, vec![EMPTY], "{source:?}");
    }
  }
}
