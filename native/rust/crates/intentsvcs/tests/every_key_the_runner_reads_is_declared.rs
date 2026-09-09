//! **THE MIRROR OF `flag_reachability`, ONE ARTEFACT OVER.**
//!
//! `AC-06.8` forbids a SURFACE declaring what nothing reads -- a flag on
//! `--help` that no renderer arm consumes. This asserts the other direction:
//! **CODE must not read what nothing declares.** A frontmatter key the critic
//! runner consults, absent from `_schema/rule-schema.md`, is configuration that
//! works and that no author has been told about.
//!
//! **THE COST IS NOT THE UNDOCUMENTED KEY, IT IS THE REPAIR IT INVITES.**
//! Measured 2026-09-09, the day `intent claude rules validate` was wired: it
//! refused five shipped rules for carrying `critic_tool`, `critic_tool_codes`
//! and `critic_tool_context`, none of which the schema declared. **From that
//! direction the cheap fix is to delete the keys from the rules** -- which would
//! have silently disabled shellcheck code-narrowing and the clippy integration,
//! with every test still green, because nothing asserted that a rule's tool
//! codes reach the runner. The rules were right and the SCHEMA was stale.
//!
//! **SO THE REFUSAL BELOW NAMES THE SCHEMA AND NEVER THE RULES.** A check whose
//! message points at the wrong artefact leads a competent person to the wrong
//! fix, and this one has a correct direction available to it.

use std::collections::BTreeSet;

/// The runner's frontmatter accessors. **DERIVED FROM THE CALL SITES, NEVER
/// FROM A ROSTER** -- a hand-maintained list of keys is precisely the artefact
/// this check exists to make unnecessary, and it would go stale in the same
/// direction and for the same reason the schema did.
const READ_CALLS: [&str; 2] = ["frontmatter_scalar(", "frontmatter_list("];

fn runner_source() -> String {
  let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/critic.rs");
  std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn canon_root() -> std::path::PathBuf {
  std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../intent/plugins/claude/rules")
}

/// Everything before `#[cfg(test)]`.
///
/// **THE RUNNER'S OWN TESTS READ KEYS TOO AND THEY ARE NOT THE POPULATION.**
/// `critic.rs`'s test module calls both accessors with literal keys; those are
/// assertions about the parser, not the runner consulting a rule. Counting them
/// would make the check pass or fail on test fixtures, which is the population
/// error this file is a response to.
fn production_half(source: &str) -> &str {
  match source.find("#[cfg(test)]") {
    Some(at) => &source[..at],
    None => source,
  }
}

/// The keys read by a LITERAL argument, and the number of call sites whose key
/// is not a literal.
///
/// **THE SECOND HALF OF THAT PAIR IS THE REACH, AND IT IS RETURNED RATHER THAN
/// SWALLOWED.** A call site passing a variable is a read this scan cannot
/// attribute to any key; reporting the count is what stops a green here being
/// read as *every key is declared* when it means *every key I could see is*.
fn keys_read(source: &str) -> (BTreeSet<String>, usize) {
  let mut keys = BTreeSet::new();
  let mut opaque = 0usize;
  for call in READ_CALLS {
    let mut rest = source;
    while let Some(at) = rest.find(call) {
      rest = &rest[at + call.len()..];
      let Some(close) = rest.find(')') else {
        opaque += 1;
        continue;
      };
      let args = &rest[..close];
      match args.find('"').and_then(|q| {
        args[q + 1..]
          .find('"')
          .map(|e| args[q + 1..q + 1 + e].to_string())
      }) {
        Some(key) if !key.is_empty() => {
          keys.insert(key);
        }
        _ => opaque += 1,
      }
    }
  }
  (keys, opaque)
}

fn declared() -> BTreeSet<String> {
  // **THE SHIPPED PARSER, NOT A SECOND READING OF THE SAME TABLES.** If the
  // schema reader breaks, this test and `rules validate` go red together --
  // which is correct, because a check that read the doc its own way could
  // certify a vocabulary the validator does not actually use.
  let schema = intentsvcs::rules::Schema::read(&canon_root()).expect("read the rule schema");
  schema.required.into_iter().chain(schema.optional).collect()
}

/// **THE VERDICT ITSELF, PURE, SO IT CAN BE DRIVEN BOTH WAYS WITHOUT EDITING A
/// SHARED SOURCE FILE.**
///
/// The first version of this file asserted inline and controlled the two INPUTS
/// separately -- that the scanner finds a planted key, and that the schema does
/// not declare it. **That is a compositional argument, not a drive**, and it is
/// the same shape as an arm deleted from `surface_is_declared.rs` earlier the
/// same day for being vacuous. The alternative on offer was to mutate
/// `critic.rs` in place and revert, on a checkout five nodes write. Neither is
/// necessary once the decision is a function of its arguments.
fn undeclared_keys(source: &str, declared: &BTreeSet<String>) -> Vec<String> {
  let (read, _) = keys_read(production_half(source));
  read.into_iter().filter(|k| !declared.contains(k)).collect()
}

#[test]
fn every_key_the_runner_reads_is_declared_by_the_schema() {
  let source = runner_source();
  let (read, opaque) = keys_read(production_half(&source));
  let declared = declared();

  let undeclared = undeclared_keys(&source, &declared);

  println!(
    "runner-frontmatter: {} key(s) read by literal across the runner's production half, {} declared by the schema, {} call site(s) whose key is not a literal and which this scan CANNOT attribute",
    read.len(),
    declared.len(),
    opaque
  );

  assert!(
    undeclared.is_empty(),
    "`_schema/rule-schema.md` DOES NOT DECLARE {} key(s) that `critic.rs` reads: {undeclared:?}\n\
     \n\
     THE FIX IS IN THE SCHEMA, NOT IN THE RULES. These keys are read by the\n\
     runner, so a rule carrying one is configured and working. Deleting them\n\
     from the rules would silence the check and disable the behaviour, with\n\
     every other test still green.\n\
     \n\
     Add each to the `### Required fields` or `### Optional fields` table in\n\
     `intent/plugins/claude/rules/_schema/rule-schema.md`.",
    undeclared.len()
  );
}

#[test]
fn the_extractor_finds_the_keys_that_are_demonstrably_read() {
  // THE POSITIVE CONTROL FOR THE SCANNER ITSELF. An extractor that found
  // nothing would satisfy the arm above vacuously, and it would look exactly
  // like a clean estate.
  let (read, _) = keys_read(production_half(&runner_source()));
  assert!(
    read.len() >= 4,
    "the scan found only {} key(s) -- it has stopped seeing the call sites: {read:?}",
    read.len()
  );
  for expected in ["critic_tool", "critic_tool_codes", "applies_to"] {
    assert!(
      read.contains(expected),
      "`{expected}` is read at a literal call site and the scan missed it: {read:?}"
    );
  }
}

#[test]
fn the_verdict_goes_both_ways_on_a_planted_read_site() {
  // **THE DECISION FUNCTION DRIVEN TO BOTH VERDICTS AGAINST THE REAL SCHEMA.**
  // Not the inputs checked separately -- the thing the criterion arm asserts on.
  let declared = declared();

  let planted = "fn run() { let x = frontmatter_scalar(body, \"a_key_no_schema_declares\"); }";
  let red = undeclared_keys(planted, &declared);
  assert_eq!(
    red,
    vec!["a_key_no_schema_declares".to_string()],
    "a read site for an undeclared key must redden, naming it"
  );

  // The GREEN half, and it must be green for the right reason: the same shape
  // of call site, with a key the schema really does declare.
  let ok = "fn run() { let x = frontmatter_scalar(body, \"applies_to\"); }";
  assert!(
    undeclared_keys(ok, &declared).is_empty(),
    "a read site for a DECLARED key must not redden, or the check fires on the call site rather than on the key"
  );
  assert!(
    declared.contains("applies_to"),
    "the green half's key must genuinely be in the schema, or it is green by accident"
  );
}

#[test]
fn a_declared_key_that_nothing_reads_is_not_this_checks_business() {
  // **THE DIRECTION THIS CHECK DELIBERATELY DOES NOT ASSERT.** The schema is
  // allowed to declare keys the RUNNER ignores -- `title`, `summary` and
  // `principles` are for readers and for other consumers. Asserting the reverse
  // containment would demand the runner read every documented field, which is a
  // different and false criterion.
  let (read, _) = keys_read(production_half(&runner_source()));
  let all = declared();
  let unread: Vec<&String> = all.iter().filter(|k| !read.contains(*k)).collect();
  assert!(
    !unread.is_empty(),
    "if every declared key were read, this arm would be asserting nothing -- check the fixture"
  );
}

#[test]
fn the_test_half_of_the_runner_is_excluded_and_the_exclusion_is_load_bearing() {
  // The cut must actually cut. If `#[cfg(test)]` moved or vanished,
  // `production_half` would silently return the whole file and the population
  // would quietly grow to include fixtures.
  let source = runner_source();
  let whole = keys_read(&source).0;
  let production = keys_read(production_half(&source)).0;
  assert!(
    production.len() <= whole.len(),
    "the production half cannot read more than the whole file"
  );
  assert!(
    source.contains("#[cfg(test)]"),
    "the boundary this scan cuts on is gone, so the population silently includes the runner's own tests"
  );
}
