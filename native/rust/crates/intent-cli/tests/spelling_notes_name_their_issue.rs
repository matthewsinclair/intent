//! **A `spelling_note` recording a HELD spelling names the issue that would
//! release it, in the note's own text.**
//!
//! `help`'s note is why this file exists. It says, correctly and at length,
//! that `target.spelling: ""` is *transitional and dated*, that hv ruled a v3
//! `help` surface into the cut on 2026-08-26, and that the value is right only
//! until that lands. **Nothing read it.** So `spine.rs` went on rendering
//! *there is no v3 replacement -- remove it from any script that calls it*,
//! `intent surface retired` published the same, and on 2026-08-30 two nodes
//! diagnosed the row, authorised a fix and routed it to a third before anyone
//! opened the note that had already answered the question. A ruling recorded
//! where no instrument looks is a ruling with no gate.
//!
//! # THE LIMIT, AND IT IS LARGER THAN WHAT THIS FILE COVERS
//!
//! **This covers `spelling_note` and nothing else. It is not the class.**
//! Measured 2026-08-30 over the raw table: **137 distinct key names carry more
//! than 200 characters of prose** -- `note` appears 66 times, `notes` 10,
//! `disposition_basis` 49, `ratification` 47, down a long tail of one-offs. A
//! roster classifying all of it would be several hundred rows, which is a
//! second copy of the table rather than a test.
//!
//! **AND WIDENING THE NET IS THE WRONG MOVE RATHER THAN THE EXPENSIVE ONE.**
//! The first version of this file keyed on the `_note` SUFFIX and found eight
//! key names, believing that was the population; `note` and `notes` were
//! invisible to it. `arg_values_note` states the general form, from cc's count
//! of 2026-08-17 where three enumerators reported three, four and five on a set
//! of five: **a population reported by an enumerator is the enumerator's
//! subject, not the population.** So this file keys on a STRUCTURED field whose
//! members it can name, and says what it does not reach.
//!
//! **THE GENERAL ANSWER IS A SCHEMA AFFORDANCE, NOT A WIDER GREP.** The
//! register has no way to say *ruled and not yet built*, so a ruled-and-unbuilt
//! surface is indistinguishable from one nobody thought of. That is a change to
//! the table's shape and it is not this file's to make.

use intent_cli::dispatch;

/// What a `spelling_note` is doing, decided by a person and recorded here.
///
/// **`Scheduled` is a classification, not an exemption** -- it pins the held
/// value to the issue that would release it, so the exposure cannot be silence,
/// and it reds the moment the note stops naming that issue. The shape is
/// `declared_values_are_enforced.rs`'s, whose `Unenforced(issue)` disposition
/// solves the neighbouring problem the same way; the vocabulary differs because
/// a note is not a slot and `Unwired` / `Planned` say nothing about prose.
#[derive(Debug)]
enum Note {
  /// Explains a spelling that is settled. Nothing about it expires.
  Durable,
  /// The annotated spelling is HELD, pending a decision already taken
  /// elsewhere. Names the issue tracking the gap.
  Scheduled(&'static str),
}

/// **Every `spelling_note` in the table, and adding one without deciding about
/// it FAILS.** An undeclared entry is a failure rather than a skip, for the
/// reason `mutation_completeness.rs` gives about a `State` field with no drive
/// arm: nobody has decided about it, and a quiet pass reads as coverage.
const CLASSIFIED: [(&str, Note); 2] = [
  ("help:spelling_note", Note::Scheduled("0086")),
  ("schema:spelling_note", Note::Durable),
];

/// Collect every `spelling_note` in the raw table, keyed by the nearest
/// enclosing row that names itself.
///
/// **Raw text rather than `dispatch::table()`, and the reason is measured**:
/// the typed model drops fields it has no struct member for, so an enumerator
/// built on it is structurally unable to report what the struct does not carry.
/// `declared_values_are_enforced.rs` reads the raw text for the same reason and
/// records the instance -- a `values` array on a flag row deserialising into
/// nothing.
fn collect(node: &serde_json::Value, owner: Option<&str>, out: &mut Vec<(String, String)>) {
  match node {
    serde_json::Value::Object(map) => {
      let here = ["path", "id", "name"]
        .iter()
        .find_map(|k| map.get(*k).and_then(|v| v.as_str()))
        .or(owner);
      for (key, value) in map {
        if key == "spelling_note"
          && let Some(text) = value.as_str()
        {
          let named = match here {
            Some(owner) => format!("{owner}:{key}"),
            None => key.clone(),
          };
          out.push((named, text.to_string()));
        }
        collect(value, here, out);
      }
    }
    serde_json::Value::Array(items) => {
      for value in items {
        collect(value, owner, out);
      }
    }
    _ => {}
  }
}

#[test]
fn every_spelling_note_is_classified_and_a_held_one_names_its_issue() {
  let table: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the compiled-in table parses");

  let mut found: Vec<(String, String)> = Vec::new();
  collect(&table, None, &mut found);

  assert!(
    !found.is_empty(),
    "no `spelling_note` was found in the compiled-in table, so this file measured an empty set \
     and would pass against any table at all. Either the key was renamed -- in which case this \
     file follows it -- or the walk is broken"
  );

  for (key, _) in &found {
    assert!(
      CLASSIFIED.iter().any(|(k, _)| k == key),
      "`{key}` is a `spelling_note` nobody has classified. Decide: `Durable` if the spelling it \
       explains is settled, or `Scheduled(issue)` if the spelling is HELD pending a decision \
       taken elsewhere -- and if it is held, the note must name that issue in its own text, \
       because a reader of the TABLE must find it without reading this file"
    );
  }

  for (key, _) in CLASSIFIED {
    assert!(
      found.iter().any(|(k, _)| k == key),
      "`{key}` is classified here and is not in the table. **This is the direction a roster \
       fails silently in**: the classification outlives the note and goes on describing prose \
       that is gone. Delete the row"
    );
  }

  for (key, note) in CLASSIFIED {
    let Note::Scheduled(issue) = note else {
      continue;
    };
    let text = found
      .iter()
      .find(|(k, _)| k == key)
      .map(|(_, text)| text.as_str())
      .expect("completeness is asserted above");
    assert!(
      text.contains(issue),
      "`{key}` is classified `Scheduled({issue})` and its own text does not name that issue. \
       **The point is that the TABLE carries the pointer, not this file** -- a reader who opens \
       the note must be told what would release the held spelling, without knowing this test \
       exists. Write `intent#{issue}` into the note"
    );
  }
}
