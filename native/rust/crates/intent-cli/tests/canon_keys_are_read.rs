//! **Issue 0039's CLASS, not its instance: a key the canon declares as driving
//! behaviour must be a key some Rust type actually reads.**
//!
//! Five times in three files a field was authored in `dispatch-table.json`, no
//! type deserialized it, serde dropped it in silence, and every instrument
//! reported agreement -- because a JSON file cannot say whether anyone is
//! listening. `Flag.required`/`default`/`value`, `Entry.exposed_on_mcp`,
//! `Entry.read_or_mutate`, and `Entry.aliases`, which was the one that
//! surfaced: `at done` and `at notdone` were declared `disposition: keep` and
//! did not exist in the binary.
//!
//! Five instance-fixes closed five instances and nothing else. This is the
//! check that makes the sixth impossible to ship quietly.
//!
//! **Why it could not be `deny_unknown_fields`, which is the obvious answer.**
//! `dispatch.rs` carries an explicit ruling against it, and the ruling is
//! right: the table is a REGISTER, not canon the tool writes. It holds prose,
//! provenance and measurement blocks that exist to be read by people --
//! `target` alone carries 44 authored keys against one field -- and a strict
//! type would stop the binary loading its own surface the first time someone
//! documented a decision in it. **But that exemption is also exactly how five
//! fields were lost**, because it makes an unread CONTRACT key indistinguishable
//! from an unread NOTE, and nothing mechanical separates them: not count
//! (`read_or_mutate` is 112 rows and gates agent safety; `observed` is 93 rows
//! and is a measurement), not type (both are strings).
//!
//! So the split is AUTHORED, by the person who knows which is which. ic
//! declared it in `key_classes` (`fd961437`), scoped to `Entry`, `Flag` and
//! `Arg`, with `Table` and `Target` keeping their exemption deliberately. This
//! test is the half that binds the declaration to the code: **the canon says
//! which keys must drive behaviour, and the types say which keys they read.**
//! Neither is restated here.
//!
//! The types are asked what they read by SERIALIZING them. A hand-kept list of
//! field names in this file would be a roster of the same kind that failed --
//! and it would be wrong in precisely the place the type was wrong, since
//! whoever forgot the field would forget the list entry in the same edit.

use std::collections::BTreeSet;

use intent_cli::dispatch;

/// The keys a type actually deserializes, read from the type itself.
///
/// Serialization is the only introspection serde offers, and it is exact here:
/// none of these structs uses `skip_serializing_if`, so every field it knows
/// appears, under the name serde would accept -- `Arg.kind` emits `type`,
/// which is the canon's spelling for it.
fn fields_of<T: serde::Serialize>(sample: &T) -> BTreeSet<String> {
  serde_json::to_value(sample)
    .expect("a table type serializes")
    .as_object()
    .expect("a struct serializes to an object")
    .keys()
    .cloned()
    .collect()
}

fn strings(value: &serde_json::Value, class: &str, kind: &str) -> BTreeSet<String> {
  value[class][kind]
    .as_array()
    .unwrap_or_else(|| panic!("key_classes.{class}.{kind} is a list"))
    .iter()
    .map(|v| {
      v.as_str()
        .expect("a classified key is a string")
        .to_string()
    })
    .collect()
}

/// The three leaf types, each with a real instance taken from the real table.
///
/// From the table rather than from a constructed sample, so a type that
/// deserializes a field the canon never carries has somewhere to show up.
fn leaves() -> (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>) {
  let table = dispatch::table();
  let entries: Vec<&dispatch::Entry> = table
    .families
    .iter()
    .flat_map(|f| f.entries.iter())
    .chain(table.new_surface.iter())
    .collect();
  let entry = entries.first().expect("the table has entries");
  let flag = entries
    .iter()
    .flat_map(|e| e.flags.iter())
    .next()
    .expect("the table has flags");
  let arg = entries
    .iter()
    .flat_map(|e| e.args.iter())
    .next()
    .expect("the table has args");
  (fields_of(entry), fields_of(flag), fields_of(arg))
}

/// **Every key the canon classifies as a DECLARATION is deserialized.**
///
/// The one that would have caught all five, on the day each was authored.
#[test]
fn every_declared_key_is_read_by_the_type_that_owns_it() {
  let classes: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  let classes = &classes["key_classes"];
  assert!(
    classes.is_object(),
    "the table declares no key_classes, so this test would pass by having nothing to check -- \
     the declaration/note split is what makes the check possible and its absence is the finding"
  );

  let (entry, flag, arg) = leaves();
  let mut missing = Vec::new();

  for (class, read) in [("entry", &entry), ("flag", &flag), ("arg", &arg)] {
    let declared = strings(classes, class, "declaration");
    assert!(
      !declared.is_empty(),
      "key_classes.{class}.declaration is empty, so every key below it is unchecked"
    );
    for key in declared.difference(read) {
      missing.push(format!(
        "  `{key}` is declared on `{class}` and no Rust field deserializes it"
      ));
    }
  }

  assert!(
    missing.is_empty(),
    "the canon declares keys that must drive behaviour and the types do not read them:\n{}\n\
     Each is silently dropped by serde, so the declaration reads as covered and the behaviour it \
     describes is absent. Add the field, or ask the table's author to reclassify the key as a note.",
    missing.join("\n")
  );
}

/// **And nothing is read that the canon has not classified at all.**
///
/// The other direction, and it is not symmetry for its own sake: a field
/// reading a key that appears in neither list means the type depends on
/// something the register does not know it is promising -- either a key no row
/// carries any more, or one whose classification was never made. Both are
/// answered by a person rather than by a default.
///
/// A key classified as a NOTE and still deserialized is allowed and not a
/// defect: `Entry.v2` is the v2 antecedent, prose by classification, and
/// carrying it costs nothing. What is refused is reading a key that was never
/// classified either way.
#[test]
fn nothing_is_read_that_the_canon_has_not_classified() {
  let classes: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  let classes = &classes["key_classes"];

  let (entry, flag, arg) = leaves();
  let mut unclassified = Vec::new();

  for (class, read) in [("entry", &entry), ("flag", &flag), ("arg", &arg)] {
    let known: BTreeSet<String> = strings(classes, class, "declaration")
      .union(&strings(classes, class, "note"))
      .cloned()
      .collect();
    for key in read.difference(&known) {
      unclassified.push(format!(
        "  `{class}` deserializes `{key}`, which key_classes does not list either way"
      ));
    }
  }

  assert!(
    unclassified.is_empty(),
    "the types read keys the register has not classified:\n{}\n\
     A key nothing classifies is either gone from the table or was never decided about.",
    unclassified.join("\n")
  );
}
