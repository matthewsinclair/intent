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
//! **`Target`'s exemption was one word doing three jobs, and it now names which
//! one** (ic, 2026-08-17). The reason written under it -- 40 prose keys, 28 of
//! them on a single row, against 2 that code reads -- argues for exempting it
//! from TOTALITY, and says nothing about the two arms that bind the declaration
//! to the code. Those cost two list entries. `Target.spelling` was deserialized
//! in `ac84dc10` and is read for the retirement message, so a read key sat in no
//! list at all -- the exact state the five lost fields were in, inside the one
//! type these checks did not look at. So `target` is now in scope for the
//! declaration arms and stays exempt from `every_key_authored_on_a_leaf_is_
//! classified_exactly_once` and from the generator's `KEY_UNCLASSED`.
//!
//! **EXEMPTING A TYPE FROM TOTALITY TIGHTENS WHAT IT MAY READ, and that reads
//! backwards, which is why it is stated here rather than only in the canon.**
//! The note list is what makes "read but deliberately unclassified" expressible
//! -- `Entry.v2` is prose by classification, deserialized, and costs nothing.
//! Remove the note list and the only surviving distinction is declared-or-not,
//! so a key the type reads MUST be declared. `Target` therefore gets the
//! STRICTER half of arm 2, not the looser one. The word "exempt" invites the
//! opposite conclusion, and a later reader drawing it would relax the arm to
//! match their expectation rather than discover the reasoning (cc, 2026-08-17,
//! asking for it to sit next to the strictness).
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

/// Every key that actually appears on any object of one leaf kind, read from
/// the table itself rather than from any list describing it.
fn authored_keys(table: &serde_json::Value, kind: &str) -> BTreeSet<String> {
  let entries: Vec<&serde_json::Value> = table["families"]
    .as_array()
    .expect("families")
    .iter()
    .flat_map(|f| f["entries"].as_array().expect("entries").iter())
    .chain(table["new_surface"].as_array().expect("new_surface").iter())
    .collect();
  let objects: Vec<&serde_json::Value> = match kind {
    "entry" => entries,
    "flag" => entries
      .iter()
      .filter_map(|e| e["flags"].as_array())
      .flatten()
      .collect(),
    "arg" => entries
      .iter()
      .filter_map(|e| e["args"].as_array())
      .flatten()
      .collect(),
    other => panic!("no such leaf kind: {other}"),
  };
  objects
    .iter()
    .filter_map(|o| o.as_object())
    .flat_map(|o| o.keys().cloned())
    .collect()
}

/// The totality check, over a table given as data so a mutated copy can drive
/// it -- which is how the canary below adds a junk key without editing a file
/// that belongs to another node.
fn unclassified(table: &serde_json::Value) -> Vec<String> {
  let classes = &table["key_classes"];
  let mut out = Vec::new();
  for kind in ["entry", "flag", "arg"] {
    let declared = strings(classes, kind, "declaration");
    let noted = strings(classes, kind, "note");
    let known: BTreeSet<String> = declared.union(&noted).cloned().collect();
    for key in authored_keys(table, kind).difference(&known) {
      out.push(format!(
        "`{kind}` carries `{key}` and key_classes lists it neither way"
      ));
    }
    for key in declared.intersection(&noted) {
      out.push(format!(
        "`{kind}` lists `{key}` BOTH ways, so it means nothing"
      ));
    }
  }
  out
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

/// The three leaf types plus `Target`, each with a real instance taken from the
/// real table.
///
/// From the table rather than from a constructed sample, so a type that
/// deserializes a field the canon never carries has somewhere to show up.
///
/// `Target` comes off the same entry, because `Entry.target` is a plain field
/// rather than an `Option` -- every row has one, so the first row's is as good
/// as any and there is no absent case to handle.
fn leaves() -> (
  BTreeSet<String>,
  BTreeSet<String>,
  BTreeSet<String>,
  BTreeSet<String>,
) {
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
  (
    fields_of(entry),
    fields_of(flag),
    fields_of(arg),
    fields_of(&entry.target),
  )
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

  let (entry, flag, arg, target) = leaves();
  let mut missing = Vec::new();

  for (class, read) in [
    ("entry", &entry),
    ("flag", &flag),
    ("arg", &arg),
    ("target", &target),
  ] {
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

  // **The scope of `target`'s exemption is asserted, not described** (cc's ask,
  // 2026-08-17). Everything above passes equally well if someone deletes
  // `spelling` from the declaration AND stops reading it, which is silent --
  // the list would shrink to `state` and every arm would stay green while a
  // shipped behaviour lost its only canon witness. `spelling` builds the
  // remedy line of the retirement refusal (`spine.rs::retired_refusal`), and
  // `retired_commands.rs` asserts that refusal names the replacement. So the
  // fact has two witnesses at opposite ends -- the behaviour at one, the
  // declaration at the other -- and neither can go quiet alone.
  assert!(
    strings(classes, "target", "declaration").contains("spelling"),
    "`key_classes.target.declaration` no longer lists `spelling`. It is read by \
     `retired_refusal` to build the `use X instead` remedy, so dropping the declaration \
     either means the retirement message lost its replacement half, or a read key went \
     back to being undeclared inside the one type the totality arm does not cover."
  );

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

  let (entry, flag, arg, target) = leaves();
  let mut unclassified = Vec::new();

  for (class, read) in [
    ("entry", &entry),
    ("flag", &flag),
    ("arg", &arg),
    ("target", &target),
  ] {
    // **`target` is STRICTER here, and it follows from the exemption rather
    // than being an extra rule.** A note list is what makes "read but
    // deliberately unclassified" expressible. `target` is exempt from totality
    // and so deliberately has none -- its 40 prose keys, 28 of them on one row,
    // are the reason -- which leaves declared-or-not as the only distinction
    // available. With no note list there is no way to tell a note key from an
    // unclassified one, so on this type every key read must be declared.
    let known: BTreeSet<String> = if class == "target" {
      strings(classes, class, "declaration")
    } else {
      strings(classes, class, "declaration")
        .union(&strings(classes, class, "note"))
        .cloned()
        .collect()
    };
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

/// **Every authored key is classified -- the arm that makes a junk key RED.**
///
/// dc's condition, relayed by vc as non-negotiable: *"add a junk key to the
/// canon and watch the check go RED. Every one of the four instances passed a
/// checker that existed."* They were right to insist, because the two checks
/// above do not close it. Both start from `key_classes` and ask what the types
/// do with it, so **a key authored on a row and listed NOWHERE is invisible to
/// both** -- which is the state every one of the lost fields was in.
///
/// `gen_dispatch_table.sh` already refuses on an unclassified key, and that is
/// the wrong place for the only witness: it is a shell tool nobody runs on a
/// push, and a property whose sole witness lives outside the suite regresses on
/// the next refactor.
#[test]
fn every_key_authored_on_a_leaf_is_classified_exactly_once() {
  let table: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  let problems = unclassified(&table);
  assert!(
    problems.is_empty(),
    "keys are authored on the table's leaves that the register does not classify:\n  {}\n\
     Every one of the five lost fields sat in exactly this state. Classify it as a declaration \
     (and give it a Rust field) or as a note.",
    problems.join("\n  ")
  );
}

/// **The canary, run as a test rather than as a ritual.**
///
/// A junk key added to a real entry must make the check above red. Driven over
/// a mutated COPY of the table, so the canon itself is never edited -- it
/// belongs to another node, and a canary that requires touching a peer's file
/// is one nobody runs twice.
#[test]
fn a_junk_key_added_to_the_canon_is_caught() {
  let mut table: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  assert!(
    unclassified(&table).is_empty(),
    "the unmutated table must be clean, or this canary proves nothing"
  );

  table["families"][0]["entries"][0]["a_key_nobody_declared"] = serde_json::json!("hello");
  let problems = unclassified(&table);
  assert_eq!(
    problems.len(),
    1,
    "exactly the junk key, and nothing else: {problems:?}"
  );
  assert!(
    problems[0].contains("a_key_nobody_declared"),
    "and it must NAME the key, because a count sends someone hunting: {}",
    problems[0]
  );
}

/// And a key classified BOTH ways is caught too -- the other half of totality,
/// which the register's own note names and nothing was checking.
#[test]
fn a_key_classified_both_ways_is_caught() {
  let mut table: serde_json::Value =
    serde_json::from_str(dispatch::TABLE).expect("the table parses as JSON");
  table["key_classes"]["entry"]["note"]
    .as_array_mut()
    .expect("the note list")
    .push(serde_json::json!("path"));

  let problems = unclassified(&table);
  assert!(
    problems.iter().any(|p| p.contains("BOTH ways")),
    "`path` is a declaration and was just also called a note: {problems:?}"
  );
}
