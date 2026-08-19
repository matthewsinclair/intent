//! AC-06.9: every finding class carries a remedy an operator can read and run.
//!
//! **`doctor --fix` was WITHDRAWN** (hv, 2026-08-15) on the ground that a
//! diagnostic which NAMES the exact remedy is strictly better than one that
//! performs it: the operator sees what will happen, decides whether it is what
//! they meant, and keeps the blast radius in their own hands. That makes the
//! remedy strings the whole of the tool's repair offer, so they have to be
//! there, they have to differ, and they have to be safe to follow.
//!
//! **The roster is DISCOVERED from the schema**, never listed here. A
//! hand-written list of classes is correct the day it is typed and silently
//! wrong at the next variant added, because the act that invalidates it
//! (adding a class) is not the act that updates it -- and this file exists
//! precisely to catch a class that forgot something.

use intentsvcs::finding::{Finding, FindingClass};
use serde_json::Value;

/// Every class the published schema declares.
///
/// **Both shapes are read, and that is not defensiveness.** schemars renders a
/// documented unit enum as `oneOf` of `const`s and an undocumented one as a
/// flat `enum` -- so the schema's shape depends on whether the variants happen
/// to carry doc comments. A reader that knew only one shape would return an
/// EMPTY roster the day someone edited a comment, and every test below passes
/// vacuously over an empty roster. The emptiness assertion downstream is what
/// makes that safe; this just stops it firing for a reason nobody cares about.
fn declared_classes() -> Vec<String> {
  let schema: Value = serde_json::to_value(schemars::schema_for!(FindingClass))
    .expect("the finding class renders as a schema");

  let from_enum = schema["enum"].as_array().map(|vs| {
    vs.iter()
      .filter_map(|v| v.as_str().map(str::to_string))
      .collect::<Vec<_>>()
  });
  let from_one_of = schema["oneOf"].as_array().map(|vs| {
    vs.iter()
      .filter_map(|v| v["const"].as_str().map(str::to_string))
      .collect::<Vec<_>>()
  });

  let classes = from_enum
    .into_iter()
    .chain(from_one_of)
    .find(|v: &Vec<String>| !v.is_empty())
    .unwrap_or_default();
  assert!(
    !classes.is_empty(),
    "the roster is discovered from the face, and the face declared nothing -- every test in this file would pass vacuously: {schema}"
  );
  classes
}

/// Round-trip a wire spelling back to the variant, so the test can reach every
/// class it discovered without naming any of them.
fn class_of(wire: &str) -> FindingClass {
  serde_json::from_value(Value::String(wire.to_string()))
    .unwrap_or_else(|e| panic!("the schema declared `{wire}` and serde will not read it back: {e}"))
}

#[test]
fn every_declared_class_carries_a_remedy() {
  let declared = declared_classes();
  assert!(
    declared.len() >= 8,
    "precondition: the roster was discovered, not empty ({declared:?})"
  );

  for wire in &declared {
    let remedy = class_of(wire).remedy();
    assert!(
      !remedy.trim().is_empty(),
      "`{wire}` reports with no remedy -- with `--fix` withdrawn this line is the only repair offer the tool makes"
    );
    // A remedy that merely restates the class tells the operator what they
    // already read one field to the left.
    assert!(
      remedy.trim() != *wire,
      "`{wire}`'s remedy restates the class instead of saying what to do"
    );
  }
}

/// Distinct per class. A copy-pasted remedy is worse than a missing one: it
/// reads as considered and sends the operator after the wrong repair.
#[test]
fn no_two_classes_share_a_remedy() {
  let declared = declared_classes();
  for (i, a) in declared.iter().enumerate() {
    for b in declared.iter().skip(i + 1) {
      assert_ne!(
        class_of(a).remedy(),
        class_of(b).remedy(),
        "`{a}` and `{b}` give the same remedy for different faults"
      );
    }
  }
}

/// **The blast-radius rule, applied to every remedy at once** (vc,
/// 2026-08-15): a remedy must not propose an operation whose blast radius
/// exceeds the fault it repairs.
///
/// `sync --to-store` replaces the ENTIRE store from the extract, and
/// `event_log` is the one table that is durable truth and not reconstructible
/// from the files -- so an operator following it to repair a single artefact
/// could lose history that exists nowhere else. Deleting the store is the same
/// class with the volume up, and D36 already forbids it.
///
/// `sync --to-disk` is deliberately NOT banned: it rewrites artefacts that are
/// re-creatable from the store by definition, so its radius is bounded to
/// things nothing authored depends on.
#[test]
fn no_remedy_proposes_an_operation_wider_than_the_fault() {
  const FORBIDDEN: &[(&str, &str)] = &[
    (
      "--to-store",
      "replaces the whole store; event_log is durable truth no file can reconstruct",
    ),
    ("rm", "D36: the store is the source of truth, not a cache"),
    ("delete the store", "D36"),
    ("intent.db", "no remedy sends an operator at the store file"),
  ];

  for wire in declared_classes() {
    let remedy = class_of(&wire).remedy();
    for (needle, why) in FORBIDDEN {
      assert!(
        !names_operation(remedy, needle),
        "`{wire}`'s remedy names `{needle}` -- {why}\n  remedy was: {remedy}"
      );
    }
  }
}

/// Whether `remedy` names `needle` AS A COMMAND rather than inside a word.
///
/// **`rm` was matched with a bare `contains("rm ")` and it fired on the word
/// "form"** -- and on "confirm", "perform" and "term", none of which anyone
/// had written yet. A substring needle for a two-letter command is a trap set
/// for whoever next uses an ordinary English word, and the failure it produces
/// blames their remedy for a defect in the check.
///
/// Matching on token boundaries instead: the needle must start the string or
/// follow a non-word character, and must end the string or be followed by one.
/// That still catches every real spelling -- `rm intent.db`, "run `rm`",
/// "(rm)" -- and stops reading the middle of words.
fn names_operation(remedy: &str, needle: &str) -> bool {
  let boundary = |c: char| !c.is_alphanumeric() && c != '_' && c != '-';
  let mut from = 0;
  while let Some(hit) = remedy[from..].find(needle) {
    let at = from + hit;
    let end = at + needle.len();
    let before_ok = at == 0 || remedy[..at].chars().next_back().is_some_and(boundary);
    let after_ok = end == remedy.len() || remedy[end..].chars().next().is_some_and(boundary);
    if before_ok && after_ok {
      return true;
    }
    from = at + 1;
  }
  false
}

/// **The boundary matcher itself, driven both ways.** Loosening a check is how
/// a check stops checking, so the loosening gets its own proof: the words that
/// caused the false positive must pass, and every real spelling of the command
/// must still be caught.
#[test]
fn the_operation_matcher_reads_commands_and_not_the_middles_of_words() {
  for innocent in [
    "rename the artefacts to the fixed form before migrating",
    "confirm the change",
    "perform the migration",
    "the term is retired",
    "reformat it",
  ] {
    assert!(
      !names_operation(innocent, "rm"),
      "a word containing `rm` is not the command `rm`: {innocent}"
    );
  }
  for real in [
    "rm intent.db",
    "run `rm` on it",
    "do not rm",
    "(rm) the file",
    "rm",
  ] {
    assert!(
      names_operation(real, "rm"),
      "this names the command and must still be caught: {real}"
    );
  }
}

/// The remedy reaches the rendered line, not just the API.
///
/// A remedy no operator ever sees is the same as no remedy, and the two are
/// indistinguishable from inside the type.
#[test]
fn the_rendered_finding_carries_its_remedy() {
  let rendered = Finding::new(
    "intent/.canon/st/ST0001.json",
    FindingClass::SchemaInvalid,
    "at /status: \"wip \" is not one of the declared values",
  )
  .to_string();

  assert!(
    rendered.contains("residue: intent/.canon/st/ST0001.json"),
    "{rendered}"
  );
  assert!(
    rendered.contains("\n  remedy: "),
    "the two-line refusal grammar the rest of the estate uses: {rendered}"
  );
  assert!(
    rendered.contains(FindingClass::SchemaInvalid.remedy()),
    "and it is THIS class's remedy: {rendered}"
  );
}
