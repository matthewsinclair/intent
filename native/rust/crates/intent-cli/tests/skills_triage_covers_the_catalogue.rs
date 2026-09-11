//! `AT-15.1` (ST0056) / `AC-15.1`: **the triage table carries a row for every
//! skill in the catalogue, and the denominator is enumerated at close.**
//!
//! # THIS ASSERTS A SET EQUALITY, NEVER A ROW COUNT, AND THE DIFFERENCE IS THE ROW
//!
//! `AC-15.1`'s own words are *the row count is asserted against the catalogue
//! enumerated at close, never against a count carried from when the WP was
//! written*. **A count-based assertion passes on a table that names 23 of the
//! wrong skills** -- the same defect `AC-04.6`'s note names over the whole
//! estate, and the same one `WP-15`'s objective records happening FOUR times to
//! this very denominator: right at authoring, stale when `in-handoff` retired,
//! corrected in a view and reverted by a regeneration, corrected in canon in a
//! field that carried the figure twice so one copy was fixed and the other left
//! contradicting it, and stale again when `in-next` and `in-start` retired.
//!
//! So the two populations are derived INDEPENDENTLY -- one by walking the
//! filesystem, one by parsing the document -- and compared as sets, in both
//! directions, naming the offending members rather than reporting a delta.
//!
//! # THE POPULATION IS ASSERTED BEFORE THE PROPERTY
//!
//! Both sides are checked non-empty and above a floor FIRST. An extractor that
//! silently matches nothing makes `{} == {}` pass, which is how a green reports
//! that a check ran when it did not -- the recurring failure this estate keeps
//! finding in its own instruments (a control that cannot exhibit the defect, a
//! pattern that cannot match the subject, a glob that returned 13 of 156).
//!
//! # WHAT THIS DOES NOT CLAIM
//!
//! It does not judge whether a verdict is CORRECT. Whether `in-plan` deserves
//! UPDATE rather than KEEP is judgement, and no test can hold it. This asserts
//! only that every skill was RULED ON and that no row names a skill that is not
//! there -- which is exactly `AC-15.1`'s subject: *an absent row and an
//! unexamined skill are the same absence*.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use testkit::repo_root;

const TRIAGE_DOC: &str = "intent/docs/skills-triage.md";
const SKILLS_DIR: &str = "intent/plugins/claude/skills";

/// A directory is a skill if and only if it carries a `SKILL.md`. That is the
/// same marker `payload.rs` uses to decide a skill is a unit, so this walk and
/// the tool's own agree by construction rather than by coincidence.
fn catalogue(skills_dir: &Path) -> BTreeSet<String> {
  let mut out = BTreeSet::new();
  let Ok(entries) = fs::read_dir(skills_dir) else {
    return out;
  };
  for e in entries.flatten() {
    if e.path().join("SKILL.md").is_file()
      && let Some(name) = e.file_name().to_str()
    {
      out.insert(name.to_string());
    }
  }
  out
}

/// A triage row is a markdown table row whose FIRST cell is a backticked skill
/// name. The verdict is returned with it so the caller can assert the
/// vocabulary -- a row saying nothing is an absent row wearing a present one.
fn triage_rows(doc: &str) -> Vec<(String, String)> {
  let mut out = Vec::new();
  for line in doc.lines() {
    let line = line.trim();
    if !line.starts_with('|') {
      continue;
    }
    let cells: Vec<&str> = line.trim_matches('|').split('|').map(str::trim).collect();
    if cells.len() < 3 {
      continue;
    }
    let name = cells[0].trim_matches('`').trim();
    // The header and its `---` separator are not rows; a skill name is the only
    // first cell that is backticked AND starts with the catalogue's prefix.
    if !cells[0].starts_with('`') || !name.starts_with("in-") {
      continue;
    }
    let verdict = cells[1].trim_matches('*').trim().to_string();
    out.push((name.to_string(), verdict));
  }
  out
}

#[test]
fn every_skill_in_the_catalogue_has_a_triage_row_and_no_row_invents_one() {
  let root = repo_root();
  let skills = catalogue(&root.join(SKILLS_DIR));
  let doc = fs::read_to_string(root.join(TRIAGE_DOC))
    .unwrap_or_else(|e| panic!("{TRIAGE_DOC} is not readable: {e}"));
  let rows = triage_rows(&doc);

  // POPULATION BEFORE PROPERTY. Either side collapsing to empty would make the
  // set equality below pass while proving nothing.
  assert!(
    skills.len() >= 10,
    "the catalogue walk found {} skill(s) under {SKILLS_DIR} -- the walk is broken, not the catalogue",
    skills.len()
  );
  assert!(
    rows.len() >= 10,
    "the table parse found {} row(s) in {TRIAGE_DOC} -- the parser is broken, not the table",
    rows.len()
  );

  let named: BTreeSet<String> = rows.iter().map(|(n, _)| n.clone()).collect();
  assert_eq!(
    named.len(),
    rows.len(),
    "the table names a skill twice; duplicates are how one verdict silently overwrites another"
  );

  let unexamined: Vec<&String> = skills.difference(&named).collect();
  assert!(
    unexamined.is_empty(),
    "{} skill(s) in the catalogue have NO triage row -- an absent row and an unexamined skill are the same absence: {unexamined:?}",
    unexamined.len()
  );

  let invented: Vec<&String> = named.difference(&skills).collect();
  assert!(
    invented.is_empty(),
    "the table rules on {} skill(s) that are not in the catalogue -- a verdict on something that is not there: {invented:?}",
    invented.len()
  );
}

#[test]
fn every_verdict_is_one_of_the_three_the_criterion_names() {
  let root = repo_root();
  let doc = fs::read_to_string(root.join(TRIAGE_DOC)).expect("triage doc readable");
  let rows = triage_rows(&doc);
  assert!(!rows.is_empty(), "no rows parsed -- the parser is broken");

  let bad: Vec<&(String, String)> = rows
    .iter()
    .filter(|(_, v)| !matches!(v.as_str(), "KEEP" | "UPDATE" | "RETIRE"))
    .collect();
  assert!(
    bad.is_empty(),
    "AC-15.1 names exactly three verdicts; these rows carry something else: {bad:?}"
  );
}

/// CONTROL. The check above is only worth its green if it can go red, so both
/// failure directions are driven against planted fixtures rather than argued.
/// A control that would also pass under a broken instrument is decoration.
#[test]
fn the_check_reddens_in_both_directions_on_planted_fixtures() {
  let real = "| `in-plan` | KEEP | reason |\n| `in-verify` | UPDATE | reason |\n";
  let parsed = triage_rows(real);
  assert_eq!(
    parsed.len(),
    2,
    "the fixture itself did not parse: {parsed:?}"
  );

  // Direction 1: a skill with no row. The catalogue has it, the table does not.
  let catalogue: BTreeSet<String> = ["in-plan", "in-verify", "in-session"]
    .iter()
    .map(|s| s.to_string())
    .collect();
  let named: BTreeSet<String> = parsed.iter().map(|(n, _)| n.clone()).collect();
  let unexamined: Vec<&String> = catalogue.difference(&named).collect();
  assert_eq!(
    unexamined,
    vec![&"in-session".to_string()],
    "the unexamined-skill direction did not fire on a planted absence"
  );

  // Direction 2: a row for a skill that is not there.
  let smaller: BTreeSet<String> = ["in-plan"].iter().map(|s| s.to_string()).collect();
  let invented: Vec<&String> = named.difference(&smaller).collect();
  assert_eq!(
    invented,
    vec![&"in-verify".to_string()],
    "the invented-row direction did not fire on a planted extra row"
  );

  // Direction 3: the vocabulary guard. A row whose verdict is prose is a row
  // that examined nothing, and it must not read as a verdict.
  let vague = triage_rows("| `in-plan` | probably fine | reason |\n");
  assert_eq!(vague.len(), 1);
  assert!(
    !matches!(vague[0].1.as_str(), "KEEP" | "UPDATE" | "RETIRE"),
    "a prose verdict was accepted as one of the three"
  );
}
