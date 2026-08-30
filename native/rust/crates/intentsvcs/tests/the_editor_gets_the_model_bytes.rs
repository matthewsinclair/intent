//! `AT-17.10` / `AC-17.10`: what a prose field hands to `$EDITOR`.
//!
//! **THE ROW VALUE AND THE EDITABLE VALUE ARE TWO DIFFERENT FACTS ABOUT ONE
//! FIELD, AND ONLY ONE OF THEM IS THE OPERATOR'S TEXT.** `form::triples`
//! collapses whitespace so a value cannot become two screen lines -- correct,
//! deliberate, and the destruction of every paragraph break in an authored
//! objective the moment those bytes are what gets written back.
//!
//! `AC-17.10` spends its text on the RETURN -- *a TUI that repaints from its
//! stale in-memory model and then saves writes the old bytes over what the
//! operator just wrote*. This is the same destruction at the DEPARTURE, and no
//! test of the return path can see it: the round trip is faithful, the file is
//! re-read, the write lands. The bytes were already wrong when they left.
//!
//! **THE FUNCTION IS SHARED RATHER THAN TUI-LOCAL FOR THE REASON `triples` IS.**
//! The web face's textarea needs exactly these bytes, `intentd` cannot reach
//! into the CLI, and two answers to *what is the real value of this field* is
//! the second home the derivation was moved down a crate to avoid.

use intentsvcs::form::{self, Loaded};
use serde_json::json;

/// Authored prose: paragraph breaks, a soft-wrapped line, trailing newline.
const AUTHORED: &str = "First paragraph.\n\nSecond one, which\nwraps across lines.\n";

fn loaded() -> Loaded {
  Loaded::load().expect("the shipped form declaration must load")
}

/// **THE CONTROL.** A fixture the row render leaves alone could not tell a
/// correct handoff from one wired to the painted row.
#[test]
fn the_fixture_is_a_value_the_row_render_visibly_destroys() {
  let collapsed = AUTHORED.split_whitespace().collect::<Vec<_>>().join(" ");
  assert_ne!(
    collapsed, AUTHORED,
    "the fixture survives a one-line render unchanged, so every assertion below would hold for \
     both wirings"
  );
  assert!(
    !collapsed.contains('\n'),
    "the collapsed form still has line breaks, so it is not the render this is contrasted with"
  );
}

/// **`raw` RETURNS THE STORED BYTES, EXACTLY.** Not trimmed, not normalised,
/// not re-wrapped -- `tui-design.md` §7 deleted an earlier design that
/// hard-wrapped on the way out and unwrapped on the way back because *it was
/// only reversible for 439 of 444 real criteria*.
#[test]
fn raw_returns_the_stored_bytes_without_touching_them() {
  let entity = json!({ "objective": AUTHORED });
  assert_eq!(form::raw(&entity, "objective").as_deref(), Some(AUTHORED));
}

/// **THE TWO DISAGREE, AND THAT IS THE POINT.** Held over every `prose` field
/// of every shipped form, so a widget added to the DSL cannot quietly acquire a
/// handoff that hands over a rendering.
#[test]
fn every_prose_field_hands_over_bytes_the_row_would_have_flattened() {
  let l = loaded();
  let mut examined = 0usize;
  for f in l.forms() {
    let prose: Vec<&str> = f
      .fields
      .iter()
      .filter(|field| field.widget == "prose")
      .map(|field| field.name.as_str())
      .collect();
    for name in prose {
      let entity = json!({ name: AUTHORED });
      let handed = form::raw(&entity, name).expect("a prose field must have editable bytes");
      let painted = form::triples(f, &entity)
        .into_iter()
        .find(|t| t.name == name)
        .expect("the field is declared, so it has a row")
        .value;
      assert_eq!(
        handed, AUTHORED,
        "{}.{name} handed over altered bytes",
        f.entity
      );
      assert_ne!(
        handed, painted,
        "{}.{name} hands the editor the same string the row shows -- if these are ever equal for \
         this fixture the row render has stopped collapsing and this test has stopped proving \
         anything",
        f.entity
      );
      examined += 1;
    }
  }
  assert!(
    examined > 0,
    "no prose field was examined, so this test asserted nothing"
  );
}

/// **A FIELD WITH NO TEXT TO EDIT IS `None`, NOT `""`.** An array or an object
/// has no bytes; offering an edit would let an editor session replace a
/// collection with a string, and the empty string is exactly what an operator
/// gets if they open it and save.
#[test]
fn a_field_that_is_not_text_offers_no_edit_at_all() {
  let entity = json!({
    "objective": "text",
    "wps": [1, 2, 3],
    "fiat": { "because": "x" },
    "seq": 4,
    "absent_marker": null,
  });
  assert!(form::raw(&entity, "objective").is_some());
  for not_text in ["wps", "fiat", "seq"] {
    assert_eq!(
      form::raw(&entity, not_text),
      None,
      "`{not_text}` offered an edit that would replace it with a string"
    );
  }
  assert_eq!(
    form::raw(&entity, "nothing_declares_this"),
    None,
    "a field the entity does not carry has nothing to edit"
  );
  assert_eq!(
    form::raw(&entity, "absent_marker").as_deref(),
    Some(""),
    "an explicit null is an EMPTY optional field, which is editable -- it is how a value gets \
     written for the first time"
  );
}
