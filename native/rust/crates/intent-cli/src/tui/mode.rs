//! The TUI's mode machine, DECLARED rather than implemented.
//!
//! **A CONTROLLER WRITTEN AS `match` ARMS IS A GRAPH NOBODY CAN SEE.** The
//! edges exist, but only as control flow, so "can this mode be left" and "is
//! this mode reachable" are answerable by reading every arm and by nothing
//! else. Declaring the table makes both questions a fold, and it is
//! `transitions.rs`'s own idiom -- the estate already models entity state as
//! declared edges checked by invariants, and a controller is the same shape
//! one layer up.
//!
//! **IT CARRIES NO `ratatui` AND NO `crossterm`, AND THAT IS THE POINT RATHER
//! THAN A CONVENIENCE.** The machine is what the realiser is checked AGAINST,
//! so it has to be provable without one. A mode machine that could only be
//! exercised by drawing to a terminal would be tested by the thing it exists
//! to constrain, and the invariants below would need a pty to answer a
//! question that has nothing to do with a terminal.
//!
//! Transcribed from `tui-design.md` section 3. **The table there is the
//! ratified one and this is its transcription**, which is the `data-model.md`
//! relationship exactly -- a code-only edge means a ruling never reached the
//! table it was ratified in, and the reverse means a ruling never reached the
//! code.

/// The five modes. **Pane focus is NOT one of them** -- `tui-design.md` is
/// explicit that list-versus-detail is a GUARD on NORMAL's edges rather than a
/// sixth mode, because it changes where `Move` and `Enter` land and not what
/// the keys mean. Modelling it as a mode would double every NORMAL row here
/// and make the reachability invariant answer a question about focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
  /// The rest state. vi's word for exactly this, and vi coherence is stated.
  Normal,
  /// Editing one text or select row in place.
  Field,
  /// Composing a `:` command.
  Command,
  /// The `/` menu.
  Menu,
  /// A child process -- `$EDITOR` -- owns the terminal.
  Embed,
}

impl Mode {
  pub const ALL: &'static [Mode] = &[
    Mode::Normal,
    Mode::Field,
    Mode::Command,
    Mode::Menu,
    Mode::Embed,
  ];

  pub fn name(self) -> &'static str {
    match self {
      Mode::Normal => "NORMAL",
      Mode::Field => "FIELD",
      Mode::Command => "COMMAND",
      Mode::Menu => "MENU",
      Mode::Embed => "EMBED",
    }
  }
}

/// One declared edge. `note` is the design table's guard text, ABBREVIATED
/// where the full sentence would push the row past rustfmt's `fn_call_width`
/// and explode the table this shape exists to keep readable. `tui-design.md`
/// section 3 is the authority on the wording; nothing asserts on `note`.
#[derive(Debug, Clone, Copy)]
pub struct Edge {
  pub from: Mode,
  pub on: &'static str,
  pub to: Mode,
  pub note: &'static str,
}

impl Edge {
  /// **A `const fn` RATHER THAN A STRUCT LITERAL, WHICH IS `transitions.rs`'s
  /// IDIOM AND IS ABOUT THE FORMATTER.** rustfmt explodes a named-field literal
  /// onto six lines each, turning a readable table into 100 lines nobody reads
  /// as a graph; a call that fits stays on one line. The workspace uses
  /// `rustfmt::skip` in exactly zero places, so keeping the table readable is
  /// done by writing it in a shape the formatter already likes rather than by
  /// exempting it.
  const fn new(from: Mode, on: &'static str, to: Mode, note: &'static str) -> Self {
    Self { from, on, to, note }
  }
}

/// The rest state. Named rather than spelled `Mode::Normal` at each use, so the
/// invariants below say what they mean about the machine rather than about one
/// variant.
pub const REST: Mode = Mode::Normal;

/// **`tui-design.md` section 3, transcribed.** Order follows the document.
pub const EDGES: &[Edge] = &[
  Edge::new(Mode::Normal, "Move", Mode::Normal, "in the focused pane"),
  Edge::new(Mode::Normal, "Enter", Mode::Field, "editable rows"),
  Edge::new(Mode::Normal, "Enter", Mode::Embed, "prose rows -> $EDITOR"),
  Edge::new(Mode::Normal, ":", Mode::Command, ""),
  Edge::new(Mode::Normal, "/", Mode::Menu, ""),
  Edge::new(Mode::Normal, "Back", Mode::Normal, "pop the view stack"),
  Edge::new(Mode::Normal, "Esc", Mode::Normal, "at the root it QUITS"),
  Edge::new(Mode::Field, "Typing", Mode::Field, ""),
  Edge::new(Mode::Field, "Enter", Mode::Normal, "commit"),
  Edge::new(Mode::Field, "Esc", Mode::Normal, "discard"),
  Edge::new(Mode::Command, "Typing", Mode::Command, ""),
  Edge::new(Mode::Command, "Enter", Mode::Normal, ""),
  Edge::new(Mode::Command, "Esc", Mode::Normal, ""),
  Edge::new(Mode::Menu, "Hotkey", Mode::Menu, "select or drill in"),
  Edge::new(Mode::Menu, "Move", Mode::Menu, "select or drill in"),
  Edge::new(Mode::Menu, "Enter", Mode::Normal, ""),
  Edge::new(Mode::Menu, "Back", Mode::Normal, ""),
  Edge::new(Mode::Menu, "Cancel", Mode::Normal, ""),
  Edge::new(Mode::Menu, "Esc", Mode::Normal, ""),
  Edge::new(Mode::Embed, "Typing", Mode::Embed, "forwarded to the child"),
  Edge::new(Mode::Embed, "ChildExit", Mode::Normal, "read the file back"),
];

/// The modes whose Esc key the TUI does NOT own, with the reason.
///
/// **DECLARED RATHER THAN FILTERED OUT OF THE INVARIANT.** `EMBED` has no `Esc`
/// edge because a child process holds the terminal while it runs -- an Esc goes
/// to `$EDITOR`, which is the whole point of handing it over. That is a real
/// exemption and it needs to be stated somewhere a reader meets it.
///
/// Writing it as a `!= Mode::Embed` inside the invariant would have made the
/// check pass for a mode that had LOST its Esc edge by accident, because the
/// exemption and the accident would look identical. As a declared list, adding
/// a second exempt mode is an edit somebody has to justify.
pub const ESC_NOT_OURS: &[(Mode, &str)] = &[(
  Mode::Embed,
  "a child process owns the terminal while it runs, so Esc reaches $EDITOR and not us",
)];

/// Every edge leaving `mode`.
pub fn out_of(mode: Mode) -> impl Iterator<Item = &'static Edge> {
  EDGES.iter().filter(move |e| e.from == mode)
}

/// The mode `on` moves to from `mode`, if the machine declares that edge.
///
/// **`None` IS "THE MACHINE SAYS NOTHING", NOT "STAY PUT".** A realiser that
/// treated an undeclared key as a self-loop would silently absorb every input
/// the table forgot, and the mode machine would agree with any realiser at all.
pub fn step(mode: Mode, on: &str) -> Option<Mode> {
  out_of(mode).find(|e| e.on == on).map(|e| e.to)
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::collections::BTreeSet;

  /// **AN UNLEAVABLE MODE IS A HANG WITH A CURSOR IN IT.** The user's only
  /// remaining move is to kill the terminal, and the terminal is left in
  /// whatever raw/alternate-screen state the TUI put it in.
  #[test]
  fn every_mode_can_be_left() {
    for &mode in Mode::ALL {
      let out: Vec<&str> = out_of(mode)
        .filter(|e| e.to != mode)
        .map(|e| e.on)
        .collect();
      assert!(
        !out.is_empty(),
        "`{}` has no edge to any other mode, so a user who reaches it cannot get out except by killing the terminal -- which leaves it in raw mode",
        mode.name()
      );
    }
  }

  /// **AN UNREACHABLE MODE IS DEAD CODE THAT READS AS A FEATURE**
  /// (`tui-design.md` section 3). It has a name, a row in the design and a
  /// realiser branch, and no sequence of keys arrives at it -- so it is
  /// documented, implemented, maintained, and never runs.
  ///
  /// Asserted by EQUALITY against every declared mode rather than by counting,
  /// so a mode added to the enum and to no edge fails here BY NAME.
  #[test]
  fn every_mode_is_reachable_from_the_rest_state() {
    let mut seen = BTreeSet::from([REST]);
    let mut frontier = vec![REST];
    while let Some(mode) = frontier.pop() {
      for edge in out_of(mode) {
        if seen.insert(edge.to) {
          frontier.push(edge.to);
        }
      }
    }
    let all: BTreeSet<Mode> = Mode::ALL.iter().copied().collect();
    let unreachable: Vec<&str> = all.difference(&seen).map(|m| m.name()).collect();
    assert!(
      unreachable.is_empty(),
      "no sequence of keys from {} reaches {:?} -- an unreachable mode is dead code that reads as a feature",
      REST.name(),
      unreachable
    );
  }

  /// **REPEATED ESC MUST TERMINATE, WHICH IS THE PROPERTY THAT MAKES A MODAL
  /// UI SAFE TO BE LOST IN** (`tui-design.md` section 3). A user who does not
  /// know where they are presses Esc until something familiar happens, and the
  /// machine owes them that this works from anywhere.
  ///
  /// The check is per-mode and names the exemption rather than skipping it, so
  /// a mode that LOST its Esc edge fails while the one that never had it
  /// passes with its reason attached.
  #[test]
  fn esc_walks_toward_the_rest_state_from_every_mode_that_owns_it() {
    for &mode in Mode::ALL {
      if let Some((_, why)) = ESC_NOT_OURS.iter().find(|(m, _)| *m == mode) {
        assert!(
          step(mode, "Esc").is_none(),
          "`{}` is declared exempt from the Esc rule because {why}, and yet it declares an Esc edge -- one of the two is wrong",
          mode.name()
        );
        assert!(
          out_of(mode).any(|e| e.to == REST),
          "`{}` does not own its Esc key, so its edge back to {} is the ONLY way out and it has none",
          mode.name(),
          REST.name()
        );
        continue;
      }
      assert_eq!(
        step(mode, "Esc"),
        Some(REST),
        "Esc from `{}` must reach {}, or repeated Esc stops being a way out of anywhere",
        mode.name(),
        REST.name()
      );
    }
  }

  /// **THE EXEMPTION LIST IS A RATCHET, NOT A PLACE TO PUT AWKWARD MODES.**
  /// Asserted by equality: adding a mode to it fails here, so the addition is
  /// an edit somebody has to make deliberately and explain.
  #[test]
  fn only_embed_is_exempt_from_owning_its_escape() {
    let exempt: Vec<&str> = ESC_NOT_OURS.iter().map(|(m, _)| m.name()).collect();
    assert_eq!(
      exempt,
      ["EMBED"],
      "the exemption list grew. The only principled reason to be on it is that another process owns the terminal"
    );
    assert!(
      ESC_NOT_OURS.iter().all(|(_, why)| !why.trim().is_empty()),
      "an exemption without a stated reason is indistinguishable from a mode somebody could not make work"
    );
  }

  /// **A DUPLICATE `(from, on)` IS A MACHINE THAT DISAGREES WITH ITSELF, AND
  /// `step` WOULD RESOLVE IT BY TABLE ORDER** -- silently, and differently
  /// from whatever the realiser did.
  ///
  /// `NORMAL + Enter` is the deliberate exception and it is NOT a
  /// contradiction: the design says an editable row goes to FIELD and a prose
  /// row goes to EMBED, so the pair is disambiguated by the ROW, which is a
  /// guard rather than an input. It is named here so the check stays exact
  /// instead of being relaxed to allow any duplicate.
  #[test]
  fn no_input_leads_two_ways_from_one_mode_except_the_one_the_design_guards() {
    let mut pairs: Vec<(&str, &str)> = EDGES.iter().map(|e| (e.from.name(), e.on)).collect();
    pairs.sort_unstable();
    let mut ambiguous: Vec<String> = Vec::new();
    for window in pairs.windows(2) {
      if window[0] == window[1] {
        ambiguous.push(format!("{} + {}", window[0].0, window[0].1));
      }
    }
    ambiguous.dedup();
    assert_eq!(
      ambiguous,
      ["NORMAL + Enter"],
      "an input leading two ways from one mode is resolved by TABLE ORDER, which no realiser can be expected to match. `NORMAL + Enter` is the design's own guarded pair -- editable rows to FIELD, prose rows to EMBED"
    );
  }

  /// The ratified table, PARSED out of `tui-design.md` section 3.
  ///
  /// Split rule: a cell combining inputs with ` / ` becomes one row per input,
  /// and the target cell either names one target for all of them or one per
  /// input. **Split on ` / ` WITH ITS SPACES, because two of the triggers ARE
  /// punctuation** -- `` `/` `` is the menu key -- and splitting on the bare
  /// character would turn that row into two empty ones and quietly SHRINK the
  /// expected set, which is the direction that fails silently.
  fn parse_ratified(text: &str) -> BTreeSet<(String, String, String)> {
    let lines: Vec<&str> = text.lines().collect();
    let start = lines
      .iter()
      .position(|l| l.starts_with("## 3. The mode machine"))
      .expect(
        "`tui-design.md` has no section 3 -- the transcription's authority was renamed or moved",
      );
    let end = lines[start + 1..]
      .iter()
      .position(|l| l.starts_with("## "))
      .map(|i| start + 1 + i)
      .unwrap_or(lines.len());

    let mut out = BTreeSet::new();
    for line in &lines[start..end] {
      let line = line.trim();
      if !line.starts_with('|') {
        continue;
      }
      let cells: Vec<String> = line
        .trim_matches('|')
        .split('|')
        .map(|c| c.trim().to_string())
        .collect();
      if cells.len() < 3 {
        continue;
      }
      let (from, trigger, to) = (&cells[0], &cells[1], &cells[2]);
      if from.eq_ignore_ascii_case("from") || from.starts_with('-') {
        continue;
      }
      let strip = |s: &str| s.trim().trim_matches('`').to_string();
      let triggers: Vec<String> = trigger.split(" / ").map(strip).collect();
      let mut targets: Vec<String> = to.split(" / ").map(strip).collect();
      if targets.len() == 1 {
        targets = vec![targets[0].clone(); triggers.len()];
      }
      assert_eq!(
        triggers.len(),
        targets.len(),
        "row `{from} | {trigger} | {to}` pairs {} inputs with {} targets and the split rule cannot match them",
        triggers.len(),
        targets.len()
      );
      for (t, d) in triggers.into_iter().zip(targets) {
        out.insert((from.clone(), t, d));
      }
    }
    out
  }

  /// The transcription is only worth having if it matches the ratified table.
  ///
  /// **THIS TEST USED TO PIN A NUMBER I HAD COUNTED BY HAND -- 17 -- AGAINST A
  /// DESIGN THAT RATIFIES 21.** Four self-loops (`FIELD` typing, `MENU` hotkey,
  /// `MENU` move, `EMBED` typing) never made it out of section 3, and the pin
  /// was taken from the NARROWED table rather than from the document, so the
  /// check agreed with the drift and its own message asserted the design agreed
  /// too. Its doc comment said it existed to catch exactly that. **A test over
  /// two authored values can only fire after somebody writes the second one.**
  ///
  /// So the expected set is now DERIVED from the document, and compared as
  /// TRIPLES rather than counted -- which also catches a row transcribed to the
  /// wrong target, something a length never could.
  #[test]
  fn the_transcription_carries_every_row_the_design_ratifies() {
    let path = testkit::repo_root().join("intent/st/ST0056/tui-design.md");
    let text =
      std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let ratified = parse_ratified(&text);
    assert!(
      !ratified.is_empty(),
      "parsed zero rows out of `tui-design.md` section 3. A parser that finds its section and reads nothing agrees with ANY transcription, which is the shape of a green for free"
    );

    let transcribed: BTreeSet<(String, String, String)> = EDGES
      .iter()
      .map(|e| {
        (
          e.from.name().to_string(),
          e.on.to_string(),
          e.to.name().to_string(),
        )
      })
      .collect();

    let missing: Vec<&(String, String, String)> = ratified.difference(&transcribed).collect();
    let extra: Vec<&(String, String, String)> = transcribed.difference(&ratified).collect();
    assert!(
      missing.is_empty() && extra.is_empty(),
      "the transcription and `tui-design.md` section 3 disagree.\n  ratified, NOT transcribed -- a ruling that never reached the code: {missing:?}\n  transcribed, NOT ratified -- a ruling that never reached the table it was ratified in: {extra:?}"
    );
  }

  /// **THE PARSER IS DRIVEN ON A PLANTED TABLE, because a parser checked only
  /// against a corpus it already matches is green for free** -- the class this
  /// estate keeps meeting, and the one that let the count above stand.
  ///
  /// The plant exercises all three shapes at once: a trigger that IS a slash,
  /// a combined trigger against ONE target, and a combined trigger against one
  /// target EACH. It also puts a `## 4.` after the table, so a parser that ran
  /// past its section would over-read and fail here.
  #[test]
  fn the_parse_is_driven_on_a_planted_table_rather_than_only_the_real_one() {
    let planted = r#"## 3. The mode machine

| from    | trigger              | to                    | notes |
| ------- | -------------------- | --------------------- | ----- |
| NORMAL  | `/`                  | MENU                  |       |
| MENU    | Hotkey / Move        | MENU                  | both  |
| COMMAND | Typing / Enter / Esc | COMMAND / NORMAL / NORMAL |   |

## 4. Keys

| NOTAROW | Move | NOWHERE | should not be read |
"#;
    let got = parse_ratified(planted);
    let want: BTreeSet<(String, String, String)> = [
      ("NORMAL", "/", "MENU"),
      ("MENU", "Hotkey", "MENU"),
      ("MENU", "Move", "MENU"),
      ("COMMAND", "Typing", "COMMAND"),
      ("COMMAND", "Enter", "NORMAL"),
      ("COMMAND", "Esc", "NORMAL"),
    ]
    .iter()
    .map(|(f, o, t)| (f.to_string(), o.to_string(), t.to_string()))
    .collect();

    assert_eq!(
      got, want,
      "the split rule must survive a slash-as-trigger, a shared target, a per-input target, and a table in the NEXT section"
    );
  }
}
