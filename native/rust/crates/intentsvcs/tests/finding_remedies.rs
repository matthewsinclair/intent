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
/// **THE STATED GROUND WAS FALSE AND IS WITHDRAWN RATHER THAN QUIETLY
/// REPLACED (cc measured it 2026-08-19; vc verified and re-cut).** It read:
/// `sync --to-store` replaces the entire store and `event_log` is durable
/// truth no file can reconstruct, so an operator repairing one artefact could
/// lose history that exists nowhere else. **`Store::rebuild` (`store.rs:1575`)
/// deletes `tests`, `criteria`, `related`, `attachments`, `wps`, `threads`,
/// `issues` -- and NOT `events`.** The event log is not in the batch and never
/// was, so the harm this rule named cannot happen and could not happen on the
/// day the rule was written.
///
/// **THE BAN SURVIVES ON A DIFFERENT AND REAL HAZARD: the UNSCOPED form takes
/// disk for EVERY thread**, so it discards store state not yet extracted for
/// threads the operator is not repairing, and reads every peer's uncommitted
/// disk state into the store -- measured happening twice in one day. That
/// radius is wider than any single-artefact fault.
///
/// **AND THE SCOPED FORM IS *NOT* BEING PERMITTED, THOUGH IT NOW EXISTS AND ITS
/// RADIUS IS ONE ARTEFACT.** cc hit this rule on AC-03.4, declined to edit it
/// so their remedy would pass, and wrote a better remedy than the one the ban
/// forbade: **lead with _copy the working file outside the project FIRST_ --
/// the only step that cannot lose anything -- then name the disk-ward command
/// with its consequence.** An arm asserts the copy-aside instruction comes
/// BEFORE the overwriting command, because an operator acting on the first
/// sentence must not destroy what the second was about to protect.
///
/// **So the hole cc identified is real and is already closed from the other
/// side: an attachment is the one finding class with NO re-derivable side --
/// both copies are authored bytes -- and the remedy that survives the ban
/// protects it without naming any sync at all.** Loosening the rule would
/// permit remedies strictly worse than the one it forced. The cost turned out
/// to be the benefit; that is luck rather than design, and it is recorded as
/// luck.
///
/// Deleting the store is the same class with the volume up, and D36 already
/// forbids it.
///
/// `sync --to-disk` is deliberately NOT banned: it rewrites artefacts that are
/// re-creatable from the store by definition, so its radius is bounded to
/// things nothing authored depends on.
#[test]
fn no_remedy_proposes_an_operation_wider_than_the_fault() {
  const FORBIDDEN: &[(&str, &str)] = &[
    (
      "--to-store",
      "the UNSCOPED form takes disk for every thread, including peers' uncommitted state; see the withdrawn premise above -- `events` is NOT in Store::rebuild's delete batch",
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

/// **A DISPOSITION CLASS CANNOT NAME A SUBJECT** (issue 0106).
///
/// `Advisory` is not a kind of artefact; it is a kind of DISPOSITION -- reported
/// and not counted -- and its members have nothing in common but that. It held
/// "the ROW is well-formed and resolves. Rewrite it in the v3 GRAMMAR when the
/// THREAD is next touched", which is the remedy for one member (a legacy AT
/// reference) promoted to the class. The hook-carrier advisory is also in this
/// class, and a hook carrier is not a row, is not a thread, is not touched, and
/// has no grammar -- so `doctor --verbose` printed prose about acceptance-test
/// rows on a subject that has none.
///
/// **THE STRING WAS NEVER WRONG; IT WAS IN THE WRONG SCOPE.** What is
/// per-instance goes in the detail -- the rule the `UnhonourableSetting` comment
/// already states -- and the AT advisory's own detail already ended "worth
/// rewriting in the v3 grammar next time the thread is touched", so demoting it
/// lost nothing and removed a second home.
///
/// The control is IN the test: the withdrawn string is asserted to fail the same
/// check that the live one passes. Without it this arm would keep passing if
/// someone replaced the matcher with one that matches nothing.
#[test]
fn the_advisory_remedy_names_no_subject_because_its_members_share_none() {
  const WITHDRAWN: &str = "nothing is owed now: the row is well-formed and resolves. Rewrite it in the v3 grammar when the thread is next touched; a closed thread carries it as it is";
  const SUBJECT_WORDS: &[(&str, &str)] = &[
    ("row", "a hook carrier is not a row"),
    (
      "thread",
      "a hook carrier is not a thread and is never `touched`",
    ),
    (
      "grammar",
      "a hook carrier has no grammar to be rewritten in",
    ),
  ];

  let live = FindingClass::Advisory.remedy();
  let mut caught_in_withdrawn = 0;
  for (needle, why) in SUBJECT_WORDS {
    assert!(
      !names_operation(live, needle),
      "the `advisory` remedy names `{needle}` -- {why}, and it is in this class too.\n  remedy was: {live}"
    );
    if names_operation(WITHDRAWN, needle) {
      caught_in_withdrawn += 1;
    }
  }
  assert_eq!(
    caught_in_withdrawn,
    SUBJECT_WORDS.len(),
    "THE CONTROL FAILED, so the assertions above prove nothing: the withdrawn \
     string is the exact text this test exists to reject, and the matcher must \
     catch every one of its subject words. If this fires, fix the matcher before \
     trusting the arm above it"
  );
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
