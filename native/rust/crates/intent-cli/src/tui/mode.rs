//! The TUI mode machine, DECLARED rather than implemented: `AT-17.9` in part, covering the ESC half of `AC-17.9`.
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
//! so a machine only exercisable through a terminal library would be tested by
//! the thing it exists to constrain.
//!
//! # The omnibox machine (hv, ruled 2026-08-30, superseding the NORMAL-rest design)
//!
//! **THE OMNIBOX IS THE REST STATE.** hv's ruling, in hv's own frame: *the
//! omnibox is the starting point, but pressing ESC or `/` takes you into those
//! other modes.* The input is where a session opens and where Esc converges;
//! an address typed there autonavigates, which makes the one typed vocabulary
//! also the primary navigation device. `COMMAND` is gone -- the omnibox
//! absorbed it, because a `:` line beside an always-present input is two
//! prompts for one keyboard.
//!
//! **ESC TOGGLES BETWEEN THE TWO HOME MODES AND NEVER QUITS.** `OMNIBOX -> NAV`
//! leaves the input; `NAV -> OMNIBOX` returns to it. Repeated Esc therefore
//! converges to [`HOME`] from anywhere in one press (Embed excepted, below) and
//! then oscillates between two fully-operable states -- which keeps the
//! property that makes a modal UI safe to be lost in, without the property
//! that made it dangerous to lean on: **quitting is now an act, never an
//! accident.** `Ctrl-C` quits from anywhere and `:q` quits from the omnibox;
//! the modern agent idiom (Claude Code's own) is exactly this shape.

/// The modes. hv's word for the second one -- *edit/nav mode distinction* --
/// is the name used, in place of vi's NORMAL: the rest state moved to the
/// omnibox, so vi coherence stopped being a naming constraint the day the
/// input became home.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mode {
  /// The rest state: the input. Addresses, fuzzy picks, and `:` commands.
  Omnibox,
  /// Walking the model: the cursor is in the body.
  Nav,
  /// The `/` menu.
  Menu,
  /// Editing one text or select row in place.
  Field,
  /// A child process -- `$EDITOR` -- owns the terminal.
  Embed,
}

impl Mode {
  pub const ALL: &'static [Mode] = &[
    Mode::Omnibox,
    Mode::Nav,
    Mode::Menu,
    Mode::Field,
    Mode::Embed,
  ];

  pub fn name(self) -> &'static str {
    match self {
      Mode::Omnibox => "OMNIBOX",
      Mode::Nav => "NAV",
      Mode::Menu => "MENU",
      Mode::Field => "FIELD",
      Mode::Embed => "EMBED",
    }
  }
}

/// One declared edge. `note` is the design table's guard text, ABBREVIATED
/// where the full sentence would push the row past rustfmt's `fn_call_width`
/// and explode the table this shape exists to keep readable. `tui-design.md`
/// section 3 is the authority on the wording; nothing asserts on `note`.
#[derive(Debug)]
pub struct Edge {
  pub from: Mode,
  pub on: &'static str,
  pub to: Mode,
  pub note: &'static str,
}

impl Edge {
  const fn new(from: Mode, on: &'static str, to: Mode, note: &'static str) -> Self {
    Self { from, on, to, note }
  }
}

/// Where a session opens and where Esc converges.
pub const REST: Mode = Mode::Omnibox;

/// The two fully-operable states Esc oscillates between once it has walked
/// out of everything nested. **A pair, deliberately**: the omnibox is home for
/// the keyboard and NAV is home for the cursor, and hv's ruling makes Esc the
/// road between them rather than the way out of the building.
pub const HOME: &[Mode] = &[Mode::Omnibox, Mode::Nav];

pub const EDGES: &[Edge] = &[
  Edge::new(Mode::Omnibox, "Typing", Mode::Omnibox, "the address buffer"),
  Edge::new(Mode::Omnibox, "Move", Mode::Omnibox, "pick among matches"),
  Edge::new(
    Mode::Omnibox,
    "Enter",
    Mode::Nav,
    "go -- address, pick or :cmd",
  ),
  Edge::new(Mode::Omnibox, "Esc", Mode::Nav, "leave the input as it is"),
  Edge::new(Mode::Omnibox, "/", Mode::Menu, "empty buffer only"),
  Edge::new(Mode::Nav, "Move", Mode::Nav, "in the focused pane"),
  Edge::new(Mode::Nav, "Enter", Mode::Nav, "descend -- rows with a door"),
  Edge::new(Mode::Nav, "Enter", Mode::Field, "editable rows"),
  Edge::new(Mode::Nav, "Enter", Mode::Embed, "prose rows -> $EDITOR"),
  Edge::new(Mode::Nav, "Back", Mode::Nav, "pop the view stack"),
  Edge::new(Mode::Nav, "Esc", Mode::Omnibox, "home to the input"),
  Edge::new(Mode::Nav, ":", Mode::Omnibox, "seed the buffer with `:`"),
  Edge::new(
    Mode::Nav,
    "Typing",
    Mode::Omnibox,
    "a printable seeds the buffer",
  ),
  Edge::new(Mode::Nav, "/", Mode::Menu, ""),
  Edge::new(Mode::Menu, "Hotkey", Mode::Menu, "select or drill in"),
  Edge::new(Mode::Menu, "Move", Mode::Menu, "select or drill in"),
  Edge::new(Mode::Menu, "Enter", Mode::Nav, ""),
  Edge::new(Mode::Menu, "Back", Mode::Nav, ""),
  Edge::new(Mode::Menu, "Cancel", Mode::Nav, ""),
  Edge::new(Mode::Menu, "Esc", Mode::Nav, ""),
  Edge::new(Mode::Field, "Typing", Mode::Field, ""),
  Edge::new(Mode::Field, "Enter", Mode::Nav, "commit"),
  Edge::new(Mode::Field, "Esc", Mode::Nav, "discard"),
  Edge::new(Mode::Embed, "Typing", Mode::Embed, "forwarded to the child"),
  Edge::new(Mode::Embed, "ChildExit", Mode::Nav, "read the file back"),
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

/// **THE ONE AMBIGUITY THE DESIGN GUARDS, AND WHAT RESOLVES IT.**
///
/// `tui-design.md` section 3 declares THREE arms of `NAV + Enter` -- to NAV for
/// *rows with a door* (descend), to FIELD for *editable rows*, to EMBED for
/// *prose rows*. The machine is right to carry all three and cannot choose
/// among them: **which arm a keystroke takes is a fact about the ROW, and the
/// machine has never seen a row.**
///
/// So the discriminator is DECLARED here, beside [`ESC_NOT_OURS`] and for the
/// same reason. `AC-17.4` is the sentence the EMBED half encodes: *`prose` IS
/// NOT AN INLINE MULTI-LINE WIDGET -- it is a HANDOFF to the external editor*.
/// The NAV half is the omnibox design's own fix for the strawman's worst
/// defect: Enter on a `button` row used to reach FIELD, so the one navigation
/// verb on screen edited a row nobody could edit and descended into nothing.
pub const BY_ROW_KIND: &[(&str, Mode)] = &[
  ("prose", Mode::Embed),
  ("artefact", Mode::Embed),
  ("button", Mode::Nav),
];

/// Every edge `on` offers from `mode`.
///
/// **[`step`] ANSWERS AN AMBIGUOUS PAIR WITH TABLE ORDER, WHICH IS NOT AN
/// ANSWER** -- the machine's own test says so in as many words. A realiser that
/// can see a row asks for all the arms and resolves them with [`arm`].
pub fn steps(mode: Mode, on: &str) -> Vec<&'static Edge> {
  out_of(mode).filter(|e| e.on == on).collect()
}

/// Which arm a row of kind `row_kind` takes, given every edge on offer.
///
/// **THE DEFAULT ARM IS DEFINED BY EXCLUSION, NEVER BY POSITION.** It is the
/// one no row kind claims. Taking "the first edge" would make the answer depend
/// on the order rows appear in [`EDGES`], and that order is a reading
/// convenience following the design document -- nothing about it is a decision
/// anyone made about behaviour.
///
/// `None` where the machine genuinely says nothing: no edges at all, or two
/// unclaimed arms, which is undecidable from a row kind and must not be
/// guessed.
pub fn arm(edges: &[&'static Edge], row_kind: &str) -> Option<Mode> {
  match edges {
    [] => return None,
    [only] => return Some(only.to),
    _ => {}
  }
  let claimed_by_this_row = BY_ROW_KIND
    .iter()
    .find(|(kind, _)| *kind == row_kind)
    .map(|(_, mode)| *mode);
  if let Some(mode) = claimed_by_this_row
    && edges.iter().any(|e| e.to == mode)
  {
    return Some(mode);
  }
  let mut unclaimed = edges
    .iter()
    .filter(|e| !BY_ROW_KIND.iter().any(|(_, mode)| *mode == e.to));
  let only = unclaimed.next()?;
  if unclaimed.next().is_some() {
    return None;
  }
  Some(only.to)
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

  /// **ONE ESC FROM ANYWHERE LANDS IN A HOME MODE, WHICH IS THE PROPERTY THAT
  /// MAKES A MODAL UI SAFE TO BE LOST IN** -- restated for the omnibox
  /// machine, where the old form (*repeated Esc terminates*) was retired
  /// deliberately: **quitting became an act rather than a convergence point**,
  /// so what Esc owes the lost operator is a fully-operable state, not an
  /// exit. Between the two home modes Esc toggles, and both directions are
  /// asserted so home stays a pair rather than quietly becoming a trap with
  /// two rooms.
  ///
  /// The check is per-mode and names the exemption rather than skipping it, so
  /// a mode that LOST its Esc edge fails while the one that never had it
  /// passes with its reason attached.
  #[test]
  fn esc_lands_in_a_home_mode_from_every_mode_that_owns_it() {
    for &mode in Mode::ALL {
      if let Some((_, why)) = ESC_NOT_OURS.iter().find(|(m, _)| *m == mode) {
        assert!(
          step(mode, "Esc").is_none(),
          "`{}` is declared exempt from the Esc rule because {why}, and yet it declares an Esc edge -- one of the two is wrong",
          mode.name()
        );
        assert!(
          out_of(mode).any(|e| HOME.contains(&e.to)),
          "`{}` does not own its Esc key, so its edge back to a home mode is the ONLY way out and it has none",
          mode.name()
        );
        continue;
      }
      let to = step(mode, "Esc");
      assert!(
        to.is_some_and(|t| HOME.contains(&t)),
        "Esc from `{}` reaches {to:?}, not a home mode -- one press must land somewhere fully operable",
        mode.name()
      );
    }
    assert_eq!(
      (step(Mode::Omnibox, "Esc"), step(Mode::Nav, "Esc")),
      (Some(Mode::Nav), Some(Mode::Omnibox)),
      "the two home modes must toggle -- hv's ruling: the omnibox is the starting point and ESC takes you into the other modes"
    );
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
  /// `NAV + Enter` is the deliberate exception and it is NOT a contradiction:
  /// the design says a row with a door descends, an editable row goes to
  /// FIELD, and a prose row goes to EMBED, so the triple is disambiguated by
  /// the ROW, which is a guard rather than an input. It is named here so the
  /// check stays exact instead of being relaxed to allow any duplicate.
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
      ["NAV + Enter"],
      "an input leading two ways from one mode is resolved by TABLE ORDER, which no realiser can be expected to match. `NAV + Enter` is the design's own guarded triple -- door rows descend, editable rows to FIELD, prose rows to EMBED"
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
  /// past its section would over-read and fail here. The mode names in the
  /// plant are the RETIRED machine's deliberately, so the parser is proved
  /// independent of the current vocabulary.
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

  /// **THE GUARDED AMBIGUITY IS RESOLVED BY THE ROW, AND THE PROOF IS THAT THE
  /// ANSWER DOES NOT MOVE WHEN THE EDGES DO.** `AT-17.10` / `AC-17.10`.
  ///
  /// The sibling test above records that `NAV + Enter` leads three ways and
  /// says table order is *not an answer a realiser can be expected to match*.
  /// This is the other half: given the arms in EITHER order, the row kind
  /// picks the same one. Reversing is the whole control -- an implementation
  /// that took `edges[0]` passes every assertion below in one direction.
  #[test]
  fn the_guarded_ambiguity_is_resolved_by_the_row_and_never_by_table_order() {
    let mut edges = steps(Mode::Nav, "Enter");
    assert_eq!(
      edges.len(),
      3,
      "the fixture is not the ambiguous triple, so nothing below could distinguish a row-driven \
       answer from a positional one"
    );
    for pass in 0..2 {
      assert_eq!(
        arm(&edges, "prose"),
        Some(Mode::Embed),
        "pass {pass}: a prose row must reach the editor -- `AC-17.4` says prose is a HANDOFF and \
         not an inline multi-line widget"
      );
      assert_eq!(
        arm(&edges, "button"),
        Some(Mode::Nav),
        "pass {pass}: a door row must DESCEND -- Enter routed to FIELD on a button is the \
         strawman defect hv drove: the one navigation verb on screen navigated nowhere"
      );
      for editable in ["text", "select", "number"] {
        assert_eq!(
          arm(&edges, editable),
          Some(Mode::Field),
          "pass {pass}: a `{editable}` row must edit in place"
        );
      }
      assert_eq!(
        arm(&edges, ""),
        Some(Mode::Field),
        "pass {pass}: with no row at all the default arm is the unclaimed one"
      );
      edges.reverse();
    }
  }

  /// **EVERY AMBIGUITY THE TABLE DECLARES IS RESOLVABLE.** An arm added to the
  /// design without a row kind claiming it would make [`arm`] return `None` for
  /// every row -- a key that is declared, bound, reachable, and does nothing.
  ///
  /// **THE CLAIMED SET IS DEDUPLICATED, AND IT HAS TO BE.** The count is of
  /// ARMS, not of entries: [`BY_ROW_KIND`] is many-to-one and legitimately so,
  /// because `prose` and `artefact` both mean *hand the terminal to `$EDITOR`*
  /// and differ only in what is handed over -- which is [`super::app::Step`]'s
  /// business, not the machine's. Counting entries was indistinguishable from
  /// counting arms while exactly one entry existed, and it reported the second
  /// one as having eaten the default arm.
  #[test]
  fn every_ambiguity_the_table_declares_can_be_resolved_from_some_row_kind() {
    let mut examined = 0usize;
    for &mode in Mode::ALL {
      for &trigger in &declared_triggers_here() {
        let edges = steps(mode, trigger);
        if edges.len() < 2 {
          continue;
        }
        examined += 1;
        let mut claimed: Vec<Mode> = BY_ROW_KIND
          .iter()
          .filter(|(_, m)| edges.iter().any(|e| e.to == *m))
          .map(|(_, m)| *m)
          .collect();
        claimed.sort_by_key(|m| m.name());
        claimed.dedup();
        assert_eq!(
          edges.len() - claimed.len(),
          1,
          "{} + {trigger} offers {} arms and row kinds claim {} of them; exactly one must be left \
           unclaimed or there is no default",
          mode.name(),
          edges.len(),
          claimed.len()
        );
      }
    }
    assert!(
      examined > 0,
      "no ambiguous pair was examined, so this test asserted nothing"
    );
  }

  /// **TWO UNCLAIMED ARMS IS `None`, NOT A GUESS.** Driven on a planted pair
  /// built from real edges, because the shipped table has no such case -- and a
  /// property that only holds because the corpus cannot exhibit it is not a
  /// property.
  ///
  /// The planted pair moved when `button` claimed `NAV`: the old pair's `MENU
  /// Enter -> NORMAL` arm became a claimed target, so the plant would have
  /// stopped exhibiting the case it names. `NAV`'s `:` and `/` edges target
  /// `OMNIBOX` and `MENU`, which no row kind claims.
  #[test]
  fn two_unclaimed_arms_are_refused_rather_than_guessed() {
    let planted: Vec<&'static Edge> = EDGES
      .iter()
      .filter(|e| e.from == Mode::Nav && (e.on == ":" || e.on == "/"))
      .collect();
    assert_eq!(
      planted.len(),
      2,
      "the planted pair is not two edges, so this test is not driving the case it names"
    );
    assert!(
      planted
        .iter()
        .all(|e| !BY_ROW_KIND.iter().any(|(_, m)| *m == e.to)),
      "a planted arm is claimed by a row kind, so this is not the unclaimed case"
    );
    assert_eq!(
      arm(&planted, "prose"),
      None,
      "two arms no row kind claims must say nothing rather than take the first"
    );
  }

  /// Every trigger the table names, for the sweep above.
  fn declared_triggers_here() -> Vec<&'static str> {
    let mut out: Vec<&'static str> = EDGES.iter().map(|e| e.on).collect();
    out.sort_unstable();
    out.dedup();
    out
  }

  /// **`AC-17.10`'s LAST CLAUSE, MADE CHECKABLE: THE FATE OF AN UNSAVED FORM AT
  /// HANDOFF IS STATED, AND IT IS STATED STRUCTURALLY.**
  ///
  /// The criterion asks that it be *stated rather than discovered*. The machine
  /// states it by having no edge at all from `FIELD` to `EMBED`: a handoff
  /// cannot be reached from inside an in-place edit, so there is no
  /// interleaving to define. `FIELD` leaves by `Enter` (commit) or `Esc`
  /// (discard) and the operator is in `NAV` before any editor exists.
  ///
  /// **A PROSE HANDOFF SHARES ITS TRIGGER WITH THE IN-PLACE EDIT**, so this is
  /// exactly the pair somebody would be tempted to wire straight through -- and
  /// wiring it would hand `$EDITOR` a field whose in-memory buffer holds
  /// characters the store has never seen. Asserted rather than left to the
  /// prose above, because a note nobody can run is a note that goes stale.
  #[test]
  fn an_in_place_edit_cannot_reach_the_editor_without_first_committing_or_discarding() {
    assert!(
      out_of(Mode::Field).all(|e| e.to != Mode::Embed),
      "FIELD leads straight to EMBED, so an operator can hand $EDITOR a buffer the store has \
       never seen and the fate of those characters is undefined"
    );
    let out: Vec<(&str, Mode)> = out_of(Mode::Field).map(|e| (e.on, e.to)).collect();
    assert!(
      out.contains(&("Enter", Mode::Nav)) && out.contains(&("Esc", Mode::Nav)),
      "FIELD must offer both a commit and a discard, or the fate above is unreachable rather \
       than stated: {out:?}"
    );
    assert!(
      out.iter().any(|(on, _)| *on == "Typing"),
      "FIELD collects no text, so it is not the in-place edit this is a claim about"
    );
  }
}
